// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: push a spellcheck language onto every item beneath
// one, as a single undo step. `set_descendants_exportable`'s walk (shared via
// `crate::subtree`) over a different field.
//
// This is the *explicit* half of the language model, and the only half there
// is. Nothing propagates a language implicitly:
// `skribisto_model::language::tags_in_binder` resolves an item's own tag, else
// the Work's -- there is no container scope in between. So a chapter written in
// another language is not expressed by tagging the chapter; it is expressed by
// tagging every row inside it, which is what this does in one gesture. What the
// Inspector shows on an item is then exactly what that item is checked against.
//
// An **empty** `tags` is a legitimate value to push, not a missing argument: it
// clears the descendants back to inheriting the Work's language, and given the
// absence of container scope it is the only way to walk back an over-broad
// apply without visiting every child by hand.
//
// The root is not touched -- the pill field beside this button owns it.
//
// Undoable via a targeted inverse. Unlike its boolean sibling the prior value
// cannot be derived from the new one, so each written row's own previous tag
// list is kept and restored.
use crate::SetDescendantsDictLanguageDto;
use crate::SetDescendantsDictLanguageResultDto;
use crate::subtree::{BinderWalk, descendants_of};
use anyhow::Result;
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::entities::BinderItem;
use common::types::EntityId;
use std::collections::HashMap;

pub trait SetDescendantsDictLanguageUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn SetDescendantsDictLanguageUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/set_descendants_dict_language_uow.rs.
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
pub trait SetDescendantsDictLanguageUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_set_descendants_dict_language_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

/// The three reads [`crate::subtree::descendants_of`] needs, forwarded to this
/// use case's own generated unit of work.
impl BinderWalk for dyn SetDescendantsDictLanguageUnitOfWorkTrait + '_ {
    fn owning_binder(&self, item: EntityId) -> Result<Option<EntityId>> {
        Ok(self
            .get_binder_relationships_from_right_ids(
                &BinderRelationshipField::BinderItems,
                &[item],
            )?
            .into_iter()
            .next()
            .map(|(binder, _)| binder))
    }

    fn ordered_items(&self, binder: EntityId) -> Result<Vec<EntityId>> {
        self.get_binder_relationship(&binder, &BinderRelationshipField::BinderItems)
    }

    fn indents(&self, ids: &[EntityId]) -> Result<HashMap<EntityId, i64>> {
        Ok(self
            .get_binder_item_multi(ids)?
            .into_iter()
            .flatten()
            .map(|it| (it.id, it.indent))
            .collect())
    }
}

pub struct SetDescendantsDictLanguageUseCase {
    uow_factory: Box<dyn SetDescendantsDictLanguageUnitOfWorkFactoryTrait>,
    /// What each written row held before, in document order — the inverse.
    previous: Vec<(EntityId, Vec<String>)>,
    /// What the forward pass wrote, for redo.
    tags: Vec<String>,
}

impl SetDescendantsDictLanguageUseCase {
    pub fn new(uow_factory: Box<dyn SetDescendantsDictLanguageUnitOfWorkFactoryTrait>) -> Self {
        SetDescendantsDictLanguageUseCase {
            uow_factory,
            previous: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn execute(
        &mut self,
        dto: &SetDescendantsDictLanguageDto,
    ) -> Result<SetDescendantsDictLanguageResultDto> {
        let item_id = dto.item_id as EntityId;

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        let targets: Vec<(EntityId, Vec<String>)> = descendants_of(uow.as_ref(), item_id)?
            .into_iter()
            .map(|id| (id, dto.tags.clone()))
            .collect();
        self.tags = dto.tags.clone();
        self.previous = apply_tags(uow.as_ref(), &targets)?;

        let changed_ids: Vec<EntityId> = self.previous.iter().map(|(id, _)| *id).collect();
        uow.commit()?;
        uow.publish_set_descendants_dict_language_event(changed_ids.clone(), None);
        Ok(SetDescendantsDictLanguageResultDto { changed_ids })
    }

    /// Write `targets` in one batch and record what they replaced.
    ///
    /// Both directions go through here, which is what keeps them symmetric: the
    /// forward pass points every descendant at the same list, the inverse points
    /// each row at its own former one, and each call hands back the inverse of
    /// the one just performed.
    ///
    /// Keyed to the rows the forward pass wrote, never a fresh subtree walk: a
    /// row moved out from under the root since then must still be reverted, and
    /// one moved in must not be touched.
    ///
    /// Cannot raise once committed, for the reason spelled out on
    /// `set_descendants_exportable_uc`'s `flip`: `UndoRedoManager::undo`
    /// re-pushes a failed command, so an error after the commit would let the
    /// next undo apply the same inverse twice.
    fn step(&mut self, targets: Vec<(EntityId, Vec<String>)>) -> Result<()> {
        if targets.is_empty() {
            return Ok(());
        }
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let ids: Vec<EntityId> = targets.iter().map(|(id, _)| *id).collect();
        let undone = apply_tags(uow.as_ref(), &targets)?;
        // Refreshed rather than assumed: if something outside this command
        // changed a row while the step was undone, the next reversal must put
        // back what was actually there, not what was there originally. An empty
        // result means every row already agreed, and the existing list stays.
        if !undone.is_empty() {
            self.previous = undone;
        }
        uow.commit()?;
        uow.publish_set_descendants_dict_language_event(ids, None);
        Ok(())
    }
}

/// Point each `(id, tags)` at its list, writing back only rows that differ.
/// Returns `(id, previous_tags)` for those rows, in the order given — the
/// inverse of the write just performed.
fn apply_tags(
    uow: &dyn SetDescendantsDictLanguageUnitOfWorkTrait,
    targets: &[(EntityId, Vec<String>)],
) -> Result<Vec<(EntityId, Vec<String>)>> {
    if targets.is_empty() {
        return Ok(Vec::new());
    }
    let now = chrono::Utc::now();
    let ids: Vec<EntityId> = targets.iter().map(|(id, _)| *id).collect();
    // Indexed then walked in `targets` order: `get_binder_item_multi` promises
    // no order, and document order is what the result reports back.
    let mut by_id: HashMap<EntityId, BinderItem> = uow
        .get_binder_item_multi(&ids)?
        .into_iter()
        .flatten()
        .map(|it| (it.id, it))
        .collect();

    let mut updated: Vec<BinderItem> = Vec::new();
    let mut previous: Vec<(EntityId, Vec<String>)> = Vec::new();
    for (id, tags) in targets {
        let Some(mut it) = by_id.remove(id) else {
            // A row deleted since the write cannot be restored, and saying so
            // would turn a partial revert into a failed one. Skipped, as in
            // `trash_binder_uc`'s reverts.
            continue;
        };
        if &it.dict_language == tags {
            continue;
        }
        previous.push((
            it.id,
            std::mem::replace(&mut it.dict_language, tags.clone()),
        ));
        // Bumped, and not restored by the inverse: `skrib_format::fingerprint`
        // neutralises every timestamp before comparing, so a bump cannot make
        // an unchanged project look dirty.
        it.updated_at = now;
        updated.push(it);
    }
    if !updated.is_empty() {
        uow.update_binder_item_multi(&updated)?;
    }
    Ok(previous)
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for SetDescendantsDictLanguageUseCase {
    fn undo(&mut self) -> Result<()> {
        let targets = self.previous.clone();
        self.step(targets)
    }

    fn redo(&mut self) -> Result<()> {
        let tags = self.tags.clone();
        let targets: Vec<(EntityId, Vec<String>)> = self
            .previous
            .iter()
            .map(|(id, _)| (*id, tags.clone()))
            .collect();
        self.step(targets)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
