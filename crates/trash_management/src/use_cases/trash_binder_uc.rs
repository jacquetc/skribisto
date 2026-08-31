// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: soft-delete (trash) a whole Binder. The binder and all
// its items flip `activated` to false; a single TrashInfo (TrashedBinder) is
// indexed under Work.trash_infos.
//
// Undoable via a targeted inverse (high-frequency op): undo reactivates the
// binder + its items and removes the TrashInfo it created. TrashInfo lives in
// the Work trunk (post-reparent).
// The rules themselves -- `activated = !trashed`, one TrashInfo per requested
// root, the open-Work and ownership checks -- live in `crate::trash_ops`, shared
// with `trash_binder_items` and `trash_selection`. No use case calls another;
// they call the same module.
use crate::TrashBinderDto;
use crate::trash_ops::{
    BinderStore, ItemStore, TrashIndex, assert_work_owns, file_trash_infos, resolve_work,
    set_activated, set_binder_activated, unfile_trash_infos,
};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{Binder, BinderItem, TrashInfo, Work};
use common::types::EntityId;

pub trait TrashBinderUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn TrashBinderUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/trash_binder_uow.rs.
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "CreateOrphan")]
#[macros::uow_action(entity = "TrashInfo", action = "SetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "RemoveMulti")]
#[macros::uow_action(entity = "Binder", action = "Get")]
#[macros::uow_action(entity = "Binder", action = "Update")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
pub trait TrashBinderUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_trash_binder_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

impl TrashIndex for dyn TrashBinderUnitOfWorkTrait + '_ {
    fn open_works(&self) -> Result<Vec<EntityId>> {
        Ok(self.get_all_work()?.into_iter().map(|w| w.id).collect())
    }
    fn work_binders(&self, work: EntityId) -> Result<Vec<EntityId>> {
        self.get_work_relationship(&work, &WorkRelationshipField::Binders)
    }
    fn trash_index(&self, work: EntityId) -> Result<Vec<EntityId>> {
        self.get_work_relationship(&work, &WorkRelationshipField::TrashInfos)
    }
    fn set_trash_index(&self, work: EntityId, ids: &[EntityId]) -> Result<()> {
        self.set_work_relationship(&work, &WorkRelationshipField::TrashInfos, ids)?;
        Ok(())
    }
    fn new_trash_info(&self, info: &TrashInfo) -> Result<EntityId> {
        Ok(self.create_orphan_trash_info(info)?.id)
    }
    fn link_trash_info(
        &self,
        info: EntityId,
        field: &TrashInfoRelationshipField,
        right: &[EntityId],
    ) -> Result<()> {
        self.set_trash_info_relationship(&info, field, right)?;
        Ok(())
    }
    fn drop_trash_infos(&self, ids: &[EntityId]) -> Result<()> {
        self.remove_trash_info_multi(ids)?;
        Ok(())
    }
}

impl ItemStore for dyn TrashBinderUnitOfWorkTrait + '_ {
    fn binder_items(&self, binder: EntityId) -> Result<Vec<EntityId>> {
        self.get_binder_relationship(&binder, &BinderRelationshipField::BinderItems)
    }
    fn items(&self, ids: &[EntityId]) -> Result<Vec<BinderItem>> {
        Ok(self
            .get_binder_item_multi(ids)?
            .into_iter()
            .flatten()
            .collect())
    }
    fn save_items(&self, items: &[BinderItem]) -> Result<()> {
        self.update_binder_item_multi(items)?;
        Ok(())
    }
}

impl BinderStore for dyn TrashBinderUnitOfWorkTrait + '_ {
    fn binder(&self, id: EntityId) -> Result<Option<Binder>> {
        self.get_binder(&id)
    }
    fn save_binder(&self, binder: &Binder) -> Result<()> {
        self.update_binder(binder)?;
        Ok(())
    }
}

pub struct TrashBinderUseCase {
    uow_factory: Box<dyn TrashBinderUnitOfWorkFactoryTrait>,
    binder_id: EntityId,
    work_id: EntityId,
    item_ids: Vec<EntityId>,
    trashed_at: DateTime<Utc>,
    created_trash: Vec<EntityId>,
}

impl TrashBinderUseCase {
    pub fn new(uow_factory: Box<dyn TrashBinderUnitOfWorkFactoryTrait>) -> Self {
        TrashBinderUseCase {
            uow_factory,
            binder_id: 0,
            work_id: 0,
            item_ids: Vec::new(),
            trashed_at: Utc::now(),
            created_trash: Vec::new(),
        }
    }

    pub fn execute(&mut self, dto: &TrashBinderDto) -> Result<()> {
        let binder_id = dto.binder_id as EntityId;

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let store: &dyn TrashBinderUnitOfWorkTrait = uow.as_ref();

        if store.binder(binder_id)?.is_none() {
            return Err(anyhow!("trash_binder: binder {binder_id} not found"));
        }
        let item_ids = store.binder_items(binder_id)?;

        self.binder_id = binder_id;
        // dto.work_id must be validated against the open Works.
        self.work_id = resolve_work(store, dto.work_id as EntityId)?;

        // Ownership check: binder_id must be one of THIS Work's own binders.
        // Work.binders is a strong one_to_many so this forward lookup is
        // exact. Without it, a caller passing Work B's binder_id alongside
        // Work A's work_id would trash Work B's binder while the new
        // TrashInfo landed under Work A's index, with no undo record on
        // Work B's side.
        assert_work_owns(store, self.work_id, &[binder_id])?;

        self.item_ids = item_ids;
        self.trashed_at = Utc::now();
        self.apply(store)?;

        uow.commit()?;
        uow.publish_trash_binder_event(vec![binder_id], None);
        Ok(())
    }

    fn apply(&mut self, uow: &dyn TrashBinderUnitOfWorkTrait) -> Result<()> {
        set_binder_activated(uow, self.binder_id, false)?;
        set_activated(uow, &self.item_ids, false)?;
        self.created_trash = file_trash_infos(
            uow,
            self.work_id,
            &[self.binder_id],
            &TrashInfoRelationshipField::TrashedBinder,
            // A whole binder has no parent binder to be restored into.
            &|_| 0,
            self.trashed_at,
        )?;
        Ok(())
    }

    fn revert(&self, uow: &dyn TrashBinderUnitOfWorkTrait) -> Result<()> {
        set_binder_activated(uow, self.binder_id, true)?;
        set_activated(uow, &self.item_ids, true)?;
        unfile_trash_infos(uow, self.work_id, &self.created_trash)
    }
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for TrashBinderUseCase {
    fn undo(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        self.revert(uow.as_ref())?;
        uow.commit()?;
        uow.publish_trash_binder_event(vec![self.binder_id], None);
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        self.apply(uow.as_ref())?;
        uow.commit()?;
        uow.publish_trash_binder_event(vec![self.binder_id], None);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
