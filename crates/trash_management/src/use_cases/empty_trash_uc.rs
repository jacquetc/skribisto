// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: permanently delete everything indexed by
// Work.trash_infos. Trashed binders (with their items + contents) are removed
// and dropped from their Work; trashed item subtrees (with their contents) are
// removed and dropped from their binder's order. All TrashInfos are removed and
// the index cleared. Undoable via a Work-scoped snapshot/restore: post-reparent
// TrashInfo lives in the Work trunk alongside the items/binders, so the whole
// Work is the undo scope.
use crate::EmptyTrashDto;
use crate::purge;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{BinderItem, Work};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;

pub trait EmptyTrashUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn EmptyTrashUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/empty_trash_uow.rs.
#[macros::uow_action(entity = "TrashInfo", action = "GetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "RemoveMulti")]
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
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
pub trait EmptyTrashUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_empty_trash_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct EmptyTrashUseCase {
    uow_factory: Box<dyn EmptyTrashUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl EmptyTrashUseCase {
    pub fn new(uow_factory: Box<dyn EmptyTrashUnitOfWorkFactoryTrait>) -> Self {
        EmptyTrashUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self, dto: &EmptyTrashDto) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // Purge everything indexed by Work.trash_infos, for the caller-named
        // Work only. Planning is read-only; the Work-scoped snapshot is taken
        // right before the first mutation. The shared purge core is also used
        // by delete_trash_entries for a caller-chosen subset.
        let work_id = work_id(uow.as_ref(), dto.work_id as EntityId)?;
        let all_ids = uow.get_work_relationship(&work_id, &WorkRelationshipField::TrashInfos)?;
        let plan = purge::plan_purge(uow.as_ref(), work_id, &all_ids)?;
        let snap_before = uow.snapshot_work(&[work_id])?;
        let removed_items = purge::apply_purge(uow.as_ref(), work_id, &all_ids, &plan)?;
        let snap_after = uow.snapshot_work(&[work_id])?;
        uow.commit()?;
        uow.publish_empty_trash_event(removed_items, None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(())
    }
}

// `get_work_relationship` doesn't validate that `id` is a real, open Work — an
// unknown id just comes back empty, which would silently no-op instead of
// reporting the caller's mistake. So check `dto.work_id` against the open
// Works first.
fn work_id(uow: &dyn EmptyTrashUnitOfWorkTrait, requested: EntityId) -> Result<EntityId> {
    uow.get_all_work()?
        .into_iter()
        .find(|w| w.id == requested)
        .map(|w| w.id)
        .ok_or_else(|| anyhow!("work {requested} is not open"))
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for EmptyTrashUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("empty_trash: nothing to undo"))?;
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
            .ok_or_else(|| anyhow!("empty_trash: nothing to redo"))?;
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
