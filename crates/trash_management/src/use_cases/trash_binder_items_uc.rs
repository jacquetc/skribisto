// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: soft-delete (trash) a set of BinderItems and their
// subtrees. Trashed items stay in place in the binder; `activated` flips to
// false and one TrashInfo per requested root is indexed under Work.trash_infos.
//
// Undoable via a targeted inverse (this is a high-frequency op): undo reactivates
// exactly the cascade it trashed and removes exactly the TrashInfo rows it
// created. Post-reparent TrashInfo lives in the Work trunk alongside the items,
// so no cross-trunk snapshot is needed.
//
// dto.work_id must be validated against the open Works before use.
//
// The rules themselves -- `activated = !trashed`, one TrashInfo per requested
// root, the open-Work and ownership checks -- live in `crate::trash_ops`, shared
// with `trash_binder` and `trash_selection`. No use case calls another; they
// call the same module.
use crate::TrashBinderItemsDto;
use crate::trash_ops::{
    self, ItemStore, TrashIndex, assert_work_owns, file_trash_infos, resolve_work, set_activated,
    unfile_trash_infos,
};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{BinderItem, TrashInfo, Work};
use common::types::EntityId;
use std::collections::HashSet;

pub trait TrashBinderItemsUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn TrashBinderItemsUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/trash_binder_items_uow.rs.
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "CreateOrphan")]
#[macros::uow_action(entity = "TrashInfo", action = "SetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "RemoveMulti")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
pub trait TrashBinderItemsUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_trash_binder_items_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

impl TrashIndex for dyn TrashBinderItemsUnitOfWorkTrait + '_ {
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

impl ItemStore for dyn TrashBinderItemsUnitOfWorkTrait + '_ {
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

pub struct TrashBinderItemsUseCase {
    uow_factory: Box<dyn TrashBinderItemsUnitOfWorkFactoryTrait>,
    // Undo/redo state — targeted inverse.
    origin_binder: EntityId,
    work_id: EntityId,
    trashed_at: DateTime<Utc>,
    roots: Vec<EntityId>,
    cascade: Vec<EntityId>,
    created_trash: Vec<EntityId>,
}

impl TrashBinderItemsUseCase {
    pub fn new(uow_factory: Box<dyn TrashBinderItemsUnitOfWorkFactoryTrait>) -> Self {
        TrashBinderItemsUseCase {
            uow_factory,
            origin_binder: 0,
            work_id: 0,
            trashed_at: Utc::now(),
            roots: Vec::new(),
            cascade: Vec::new(),
            created_trash: Vec::new(),
        }
    }

    pub fn execute(&mut self, dto: &TrashBinderItemsDto) -> Result<()> {
        if dto.binder_item_ids.is_empty() {
            return Err(anyhow!("trash_binder_items: no items"));
        }
        let origin_binder = dto.origin_binder_id as EntityId;
        let requested: HashSet<EntityId> =
            dto.binder_item_ids.iter().map(|&x| x as EntityId).collect();

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let store: &dyn TrashBinderItemsUnitOfWorkTrait = uow.as_ref();

        // Resolve the requested roots + their contiguous cascade from binder order.
        let order = store.binder_items(origin_binder)?;
        let indent = trash_ops::indents(store, &order)?;
        let (roots, cascade) = trash_ops::roots_and_cascade(&order, &indent, &requested);
        if cascade.is_empty() {
            return Err(anyhow!(
                "trash_binder_items: no matching items in binder {origin_binder}"
            ));
        }

        self.origin_binder = origin_binder;
        self.work_id = resolve_work(store, dto.work_id as EntityId)?;

        // Ownership check: origin_binder_id must be one of THIS Work's own
        // binders. The roots/cascade above are resolved purely from
        // `origin_binder`'s own order with no cross-check against the Work,
        // so without this a caller pairing Work B's origin_binder_id with
        // Work A's work_id would trash Work B's cascade while the new
        // TrashInfo landed under Work A's index.
        assert_work_owns(store, self.work_id, &[origin_binder])?;

        self.trashed_at = Utc::now();
        self.roots = roots.clone();
        self.cascade = cascade;
        self.apply(store)?;

        uow.commit()?;
        uow.publish_trash_binder_items_event(roots, None);
        Ok(())
    }

    /// Forward (execute + redo): flip the cascade to trashed and index one
    /// TrashInfo per root under Work.trash_infos, recording the new ids.
    fn apply(&mut self, uow: &dyn TrashBinderItemsUnitOfWorkTrait) -> Result<()> {
        set_activated(uow, &self.cascade, false)?;
        let origin = self.origin_binder as i64;
        self.created_trash = file_trash_infos(
            uow,
            self.work_id,
            &self.roots,
            &TrashInfoRelationshipField::TrashedBinderItem,
            &|_| origin,
            self.trashed_at,
        )?;
        Ok(())
    }

    /// Inverse (undo): reactivate the cascade and drop this command's TrashInfo
    /// rows, unlinking them from Work.trash_infos.
    fn revert(&self, uow: &dyn TrashBinderItemsUnitOfWorkTrait) -> Result<()> {
        set_activated(uow, &self.cascade, true)?;
        unfile_trash_infos(uow, self.work_id, &self.created_trash)
    }
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for TrashBinderItemsUseCase {
    fn undo(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        self.revert(uow.as_ref())?;
        uow.commit()?;
        uow.publish_trash_binder_items_event(self.cascade.clone(), None);
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        self.apply(uow.as_ref())?;
        uow.commit()?;
        uow.publish_trash_binder_items_event(self.roots.clone(), None);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
