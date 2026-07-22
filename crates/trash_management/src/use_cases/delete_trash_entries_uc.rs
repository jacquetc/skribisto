// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: permanently delete a caller-chosen SUBSET of trash
// entries (per-entry "Delete forever"), sharing the purge core with empty_trash.
// A stale id (one no longer in Work.trash_infos, e.g. from an outdated UI
// snapshot) is ignored, not an error. The shared plan folds in any collateral
// TrashInfo whose target is swept up by the requested purge (see purge.rs), so a
// subset delete never leaves a dangling trash row. Undoable via a Work-scoped
// snapshot/restore; the UI, not the backend, decides whether to clear the undo
// stack after a grace period.
use crate::DeleteTrashEntriesDto;
use crate::purge;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{BinderItem, Work};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use std::collections::HashSet;

pub trait DeleteTrashEntriesUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn DeleteTrashEntriesUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/delete_trash_entries_uow.rs. It matches empty_trash's set:
// both drive the shared purge core (crate::purge::PurgeAccess).
#[macros::uow_action(entity = "TrashInfo", action = "GetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "RemoveMulti")]
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "Work", action = "Snapshot")]
#[macros::uow_action(entity = "Work", action = "Restore")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Binder", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "RemoveMulti")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "RemoveMulti")]
#[macros::uow_action(entity = "Content", action = "RemoveMulti")]
pub trait DeleteTrashEntriesUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_delete_trash_entries_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct DeleteTrashEntriesUseCase {
    uow_factory: Box<dyn DeleteTrashEntriesUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl DeleteTrashEntriesUseCase {
    pub fn new(uow_factory: Box<dyn DeleteTrashEntriesUnitOfWorkFactoryTrait>) -> Self {
        DeleteTrashEntriesUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self, dto: &DeleteTrashEntriesDto) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // Work resolution: dto.work_id, validated against the open Works --
        // Phase 0.6: this used to pick `get_all_work().next()`, which had no
        // defined subject once a second Work was open. This is the sneakiest
        // variant of the bug: filtering dto.trash_info_ids against the WRONG
        // Work's index leaves `ids` empty (TrashInfo ids never coincide
        // across Works), and the existing "stale id -> no-op, not error"
        // tolerance below then swallows it with zero error -- "Delete
        // forever" reports success and deletes nothing. Validating here
        // turns a closed/wrong work_id into a real error instead.
        let work_id = work_id(uow.as_ref(), dto.work_id as EntityId)?;

        // Keep only ids still in the index (stale ids → no-op, not error).
        let indexed: HashSet<EntityId> = uow
            .get_work_relationship(&work_id, &WorkRelationshipField::TrashInfos)?
            .into_iter()
            .collect();
        let missing: Vec<EntityId> = dto
            .trash_info_ids
            .iter()
            .copied()
            .filter(|id| !indexed.contains(id))
            .collect();
        if !missing.is_empty() {
            // A missing id is one of two very different things: a genuinely
            // stale row (already purged elsewhere -- tolerate, per this file's
            // documented "stale id -> no-op" contract) or a row that is very
            // much alive under a DIFFERENT open Work (the caller passed the
            // wrong work_id alongside a real id it doesn't own). The plain
            // `indexed` filter above cannot tell these apart -- both look like
            // "not in `work_id`'s own list". Only a reverse lookup on the
            // missing ids does: this is the "sneakiest variant" this file's
            // header warns about, where the tolerant filter would otherwise
            // swallow a real cross-Work mismatch as a silent, successful no-op.
            let owners = uow.get_work_relationships_from_right_ids(
                &WorkRelationshipField::TrashInfos,
                &missing,
            )?;
            let foreign: Vec<EntityId> = owners
                .into_iter()
                .filter(|(w, _)| *w != work_id)
                .flat_map(|(_, ids)| ids)
                .collect();
            if !foreign.is_empty() {
                return Err(anyhow!(
                    "delete_trash_entries: trash entries {foreign:?} do not belong to work {work_id}"
                ));
            }
        }
        let ids: Vec<EntityId> = dto
            .trash_info_ids
            .iter()
            .copied()
            .filter(|id| indexed.contains(id))
            .collect();

        // Snapshot unconditionally so undo/redo work even for a no-op delete.
        let snap_before = uow.snapshot_work(&[work_id])?;
        let removed_items = if ids.is_empty() {
            Vec::new()
        } else {
            let plan = purge::plan_purge(uow.as_ref(), work_id, &ids)?;
            purge::apply_purge(uow.as_ref(), work_id, &ids, &plan)?
        };
        let snap_after = uow.snapshot_work(&[work_id])?;
        uow.commit()?;
        uow.publish_delete_trash_entries_event(removed_items, None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(())
    }
}

// `get_work_relationship` does not validate that `id` is a real, open Work --
// a junction lookup against an unknown id just comes back empty, which would
// silently no-op instead of reporting the caller's mistake. So the id from
// `dto.work_id` is checked against the open Works first, exactly like
// `empty_trash_uc.rs` does.
fn work_id(uow: &dyn DeleteTrashEntriesUnitOfWorkTrait, requested: EntityId) -> Result<EntityId> {
    uow.get_all_work()?
        .into_iter()
        .find(|w| w.id == requested)
        .map(|w| w.id)
        .ok_or_else(|| anyhow!("work {requested} is not open"))
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for DeleteTrashEntriesUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("delete_trash_entries: nothing to undo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_work(snap)?;
        uow.commit()?;
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let snap = self
            .snap_after
            .as_ref()
            .ok_or_else(|| anyhow!("delete_trash_entries: nothing to redo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_work(snap)?;
        uow.commit()?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
