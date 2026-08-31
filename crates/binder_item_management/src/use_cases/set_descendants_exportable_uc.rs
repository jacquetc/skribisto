// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: push one item's `is_exportable` down onto every item
// beneath it, as a single undo step.
//
// The **root is deliberately untouched**. This is the "apply to children"
// affordance that sits beside the item's own toggle, and the two are separate
// gestures: a writer excluding a folder from the export has not thereby said
// anything about its scenes, and a writer excluding the scenes has not excluded
// the folder. Fusing them would make one of the two unreachable.
//
// "Beneath" is the binder's own notion of containment, which is positional, not
// a parent/child graph: the subtree of X is X plus every following item with a
// strictly greater indent (`binder_ordering::subtree_of`). That is the whole
// reason this lives here rather than at a call site -- computing it needs the
// binder's ordered item list *and* every item's indent, which a caller holding
// nothing but an item id does not have.
//
// Trashed descendants are included, matching every other subtree write in this
// crate: `activated` gates what the manuscript compiles, not what a flag may be
// written onto, and a row restored later carries the flag its neighbours got.
//
// Undoable via a targeted inverse rather than a snapshot. The forward pass
// records exactly the ids whose value actually changed, and since the field is
// a boolean every one of them held `!exportable` before -- so the inverse is
// total and needs no stored prior state.
use crate::SetDescendantsExportableDto;
use crate::SetDescendantsExportableResultDto;
use crate::subtree::{BinderWalk, descendants_of};
use anyhow::Result;
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::entities::BinderItem;
use common::types::EntityId;
use std::collections::HashMap;

pub trait SetDescendantsExportableUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn SetDescendantsExportableUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/set_descendants_exportable_uow.rs.
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
pub trait SetDescendantsExportableUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_set_descendants_exportable_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct SetDescendantsExportableUseCase {
    uow_factory: Box<dyn SetDescendantsExportableUnitOfWorkFactoryTrait>,
    // Undo/redo state — a targeted inverse (see the header).
    changed: Vec<EntityId>,
    exportable: bool,
}

impl SetDescendantsExportableUseCase {
    pub fn new(uow_factory: Box<dyn SetDescendantsExportableUnitOfWorkFactoryTrait>) -> Self {
        SetDescendantsExportableUseCase {
            uow_factory,
            changed: Vec::new(),
            exportable: false,
        }
    }

    pub fn execute(
        &mut self,
        dto: &SetDescendantsExportableDto,
    ) -> Result<SetDescendantsExportableResultDto> {
        let item_id = dto.item_id as EntityId;

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        let descendants = descendants_of(uow.as_ref(), item_id)?;
        self.exportable = dto.exportable;
        self.changed = write_flag(uow.as_ref(), &descendants, dto.exportable)?;

        uow.commit()?;
        uow.publish_set_descendants_exportable_event(self.changed.clone(), None);
        Ok(SetDescendantsExportableResultDto {
            changed_ids: self.changed.clone(),
        })
    }
}

/// The three reads [`crate::subtree::descendants_of`] needs, forwarded to this
/// use case's own generated unit of work.
impl BinderWalk for dyn SetDescendantsExportableUnitOfWorkTrait + '_ {
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

/// Set `is_exportable` on `ids`, writing back only the rows that actually differ.
/// Returns those rows' ids, in the order given.
fn write_flag(
    uow: &dyn SetDescendantsExportableUnitOfWorkTrait,
    ids: &[EntityId],
    value: bool,
) -> Result<Vec<EntityId>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let now = chrono::Utc::now();
    let mut updated: Vec<BinderItem> = Vec::new();
    // Ordered by `ids`, not by the fetch: the caller's document order is what
    // the result reports back, and `get_binder_item_multi` makes no promise.
    let mut by_id: HashMap<EntityId, BinderItem> = uow
        .get_binder_item_multi(ids)?
        .into_iter()
        .flatten()
        .map(|it| (it.id, it))
        .collect();
    for id in ids {
        let Some(mut it) = by_id.remove(id) else {
            continue;
        };
        if it.is_exportable == value {
            continue;
        }
        it.is_exportable = value;
        // Bumped, and deliberately not restored by the inverse below:
        // `skrib_format::fingerprint` neutralises every timestamp before
        // comparing, so a bump cannot make an unchanged project look dirty.
        it.updated_at = now;
        updated.push(it);
    }
    let changed: Vec<EntityId> = updated.iter().map(|it| it.id).collect();
    if !updated.is_empty() {
        uow.update_binder_item_multi(&updated)?;
    }
    Ok(changed)
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for SetDescendantsExportableUseCase {
    fn undo(&mut self) -> Result<()> {
        self.flip(!self.exportable)
    }

    fn redo(&mut self) -> Result<()> {
        self.flip(self.exportable)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl SetDescendantsExportableUseCase {
    /// Write `value` onto exactly the rows the forward pass changed.
    ///
    /// Re-resolving the subtree here would be wrong: an item moved out from
    /// under the root between the write and the undo must still be reverted,
    /// and one moved *in* must not be touched.
    ///
    /// A row that no longer exists is skipped rather than raised, matching
    /// `trash_binder_uc`'s reverts. It also **must not** raise: a failed `undo`
    /// is re-pushed onto the undo stack by `UndoRedoManager`, so erroring after
    /// this transaction has committed would let the next undo apply the same
    /// inverse a second time.
    fn flip(&mut self, value: bool) -> Result<()> {
        if self.changed.is_empty() {
            return Ok(());
        }
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let ids = self.changed.clone();
        write_flag(uow.as_ref(), &ids, value)?;
        uow.commit()?;
        uow.publish_set_descendants_exportable_event(ids, None);
        Ok(())
    }
}
