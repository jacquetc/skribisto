// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: push one item's declared `books` (its Book filing)
// down onto every item beneath it, as a single undo step. The same subtree
// walk as `set_descendants_exportable`/`set_descendants_dict_language`
// (`crate::subtree`), over the `Books` relationship instead of a scalar field.
//
// It **overwrites**, matching `set_descendants_dict_language`'s own documented
// behaviour: an **empty** `book_ids` is a legitimate instruction to push down,
// not a missing argument, and a child that already carried its own filing
// loses it to the parent's current value. A writer who wants one child to
// differ edits that child afterward.
//
// The root is not touched -- the Inspector's own Books section owns that;
// this is the "apply to children" affordance beside it.
//
// Trashed descendants are included, matching every other subtree write in
// this crate: `activated` gates what the manuscript compiles, not what a
// relationship may be written onto, and a row restored later carries the
// filing its neighbours got.
//
// Undoable via a targeted inverse. Unlike its boolean sibling
// (`set_descendants_exportable`) the prior value cannot be derived from the
// new one, so each written row's own previous `book_ids` is kept and
// restored -- the same shape as `set_descendants_dict_language`'s inverse.
//
// `Books` is a relationship (a junction table), not a scalar column, so the
// read/write pair is `get_binder_item_relationship`/
// `set_binder_item_relationship_multi`, not `GetMulti`/`UpdateMulti`:
// `BinderItemTable::update_multi` is documented scalar-only and does not
// touch junction tables at all, so writing `books` through it would silently
// no-op -- confirmed by `BinderItemHashMapTable::hydrate`, which always
// overwrites the relationship fields on read from the junction store,
// regardless of what a scalar `update` call was given.
use crate::SetDescendantsBooksDto;
use crate::SetDescendantsBooksResultDto;
use crate::subtree::{BinderWalk, descendants_of};
use anyhow::Result;
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::entities::BinderItem;
use common::types::EntityId;
use std::collections::HashMap;

pub trait SetDescendantsBooksUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn SetDescendantsBooksUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/set_descendants_books_uow.rs.
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "SetRelationshipMulti")]
pub trait SetDescendantsBooksUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_set_descendants_books_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

/// The three reads [`crate::subtree::descendants_of`] needs, forwarded to
/// this use case's own generated unit of work.
impl BinderWalk for dyn SetDescendantsBooksUnitOfWorkTrait + '_ {
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

pub struct SetDescendantsBooksUseCase {
    uow_factory: Box<dyn SetDescendantsBooksUnitOfWorkFactoryTrait>,
    /// What each written row held before, in document order -- the inverse.
    previous: Vec<(EntityId, Vec<EntityId>)>,
    /// What the forward pass wrote, for redo.
    book_ids: Vec<EntityId>,
}

impl SetDescendantsBooksUseCase {
    pub fn new(uow_factory: Box<dyn SetDescendantsBooksUnitOfWorkFactoryTrait>) -> Self {
        SetDescendantsBooksUseCase {
            uow_factory,
            previous: Vec::new(),
            book_ids: Vec::new(),
        }
    }

    pub fn execute(
        &mut self,
        dto: &SetDescendantsBooksDto,
    ) -> Result<SetDescendantsBooksResultDto> {
        let item_id = dto.item_id as EntityId;

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        let book_ids: Vec<EntityId> = dto.book_ids.iter().map(|&id| id as EntityId).collect();
        let targets: Vec<(EntityId, Vec<EntityId>)> = descendants_of(uow.as_ref(), item_id)?
            .into_iter()
            .map(|id| (id, book_ids.clone()))
            .collect();
        self.book_ids = book_ids;
        self.previous = apply_books(uow.as_ref(), &targets)?;

        let changed_ids: Vec<EntityId> = self.previous.iter().map(|(id, _)| *id).collect();
        uow.commit()?;
        uow.publish_set_descendants_books_event(changed_ids.clone(), None);
        Ok(SetDescendantsBooksResultDto { changed_ids })
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
    fn step(&mut self, targets: Vec<(EntityId, Vec<EntityId>)>) -> Result<()> {
        if targets.is_empty() {
            return Ok(());
        }
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let ids: Vec<EntityId> = targets.iter().map(|(id, _)| *id).collect();
        let undone = apply_books(uow.as_ref(), &targets)?;
        // Refreshed rather than assumed: if something outside this command
        // changed a row while the step was undone, the next reversal must put
        // back what was actually there, not what was there originally. An empty
        // result means every row already agreed, and the existing list stays.
        if !undone.is_empty() {
            self.previous = undone;
        }
        uow.commit()?;
        uow.publish_set_descendants_books_event(ids, None);
        Ok(())
    }
}

/// Point each `(id, book_ids)` at its list, writing back only rows that
/// differ. Returns `(id, previous_book_ids)` for those rows, in the order
/// given -- the inverse of the write just performed.
///
/// Reads `books` one row at a time (`GetRelationship` has no batched form on
/// this unit of work -- see the header), but writes every changed row in one
/// `set_binder_item_relationship_multi` call, so the subtree still lands as a
/// single junction-table write rather than one per descendant.
fn apply_books(
    uow: &dyn SetDescendantsBooksUnitOfWorkTrait,
    targets: &[(EntityId, Vec<EntityId>)],
) -> Result<Vec<(EntityId, Vec<EntityId>)>> {
    if targets.is_empty() {
        return Ok(Vec::new());
    }
    let mut previous: Vec<(EntityId, Vec<EntityId>)> = Vec::new();
    let mut writes: Vec<(EntityId, Vec<EntityId>)> = Vec::new();
    for (id, book_ids) in targets {
        let current = uow.get_binder_item_relationship(id, &BinderItemRelationshipField::Books)?;
        if &current == book_ids {
            continue;
        }
        previous.push((*id, current));
        writes.push((*id, book_ids.clone()));
    }
    if !writes.is_empty() {
        uow.set_binder_item_relationship_multi(&BinderItemRelationshipField::Books, writes)?;
    }
    Ok(previous)
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for SetDescendantsBooksUseCase {
    fn undo(&mut self) -> Result<()> {
        let targets = self.previous.clone();
        self.step(targets)
    }

    fn redo(&mut self) -> Result<()> {
        let book_ids = self.book_ids.clone();
        let targets: Vec<(EntityId, Vec<EntityId>)> = self
            .previous
            .iter()
            .map(|(id, _)| (*id, book_ids.clone()))
            .collect();
        self.step(targets)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
