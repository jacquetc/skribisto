// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: soft-delete (trash) a whole Binder. The binder and all
// its items flip `activated` to false; a single TrashInfo (TrashedBinder) is
// indexed under Work.trash_infos.
//
// Undoable via a targeted inverse (high-frequency op): undo reactivates the
// binder + its items and removes the TrashInfo it created. TrashInfo lives in
// the Work trunk (post-reparent).
use crate::TrashBinderDto;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{Binder, BinderItem, TrashInfo, Work};
use common::types::EntityId;
use std::collections::HashSet;

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

        if uow.get_binder(&binder_id)?.is_none() {
            return Err(anyhow!("trash_binder: binder {binder_id} not found"));
        }
        let item_ids =
            uow.get_binder_relationship(&binder_id, &BinderRelationshipField::BinderItems)?;

        self.binder_id = binder_id;
        // Work resolution: dto.work_id, validated against the open Works --
        // Phase 0.6: this used to pick `get_all_work().next()`, which had no
        // defined subject once a second Work was open (the trash entry could
        // land under an unrelated Work while the caller's own Work lost the
        // binder with no trash-bin trace of it).
        self.work_id = work_id(uow.as_ref(), dto.work_id as EntityId)?;
        self.item_ids = item_ids;
        self.trashed_at = Utc::now();
        self.apply(uow.as_ref())?;

        uow.commit()?;
        uow.publish_trash_binder_event(vec![binder_id], None);
        Ok(())
    }

    fn apply(&mut self, uow: &dyn TrashBinderUnitOfWorkTrait) -> Result<()> {
        set_binder_activated(uow, self.binder_id, false)?;
        set_items_activated(uow, &self.item_ids, false)?;

        let mut index =
            uow.get_work_relationship(&self.work_id, &WorkRelationshipField::TrashInfos)?;
        let info = uow.create_orphan_trash_info(&TrashInfo {
            created_at: self.trashed_at,
            updated_at: self.trashed_at,
            trashed_at: self.trashed_at,
            origin_binder_id: 0, // a whole binder has no parent binder
            ..Default::default()
        })?;
        uow.set_trash_info_relationship(
            &info.id,
            &TrashInfoRelationshipField::TrashedBinder,
            &[self.binder_id],
        )?;
        index.push(info.id);
        uow.set_work_relationship(&self.work_id, &WorkRelationshipField::TrashInfos, &index)?;
        self.created_trash = vec![info.id];
        Ok(())
    }

    fn revert(&self, uow: &dyn TrashBinderUnitOfWorkTrait) -> Result<()> {
        set_binder_activated(uow, self.binder_id, true)?;
        set_items_activated(uow, &self.item_ids, true)?;

        if !self.created_trash.is_empty() {
            let drop: HashSet<EntityId> = self.created_trash.iter().copied().collect();
            let remaining: Vec<EntityId> = uow
                .get_work_relationship(&self.work_id, &WorkRelationshipField::TrashInfos)?
                .into_iter()
                .filter(|id| !drop.contains(id))
                .collect();
            uow.set_work_relationship(
                &self.work_id,
                &WorkRelationshipField::TrashInfos,
                &remaining,
            )?;
            uow.remove_trash_info_multi(&self.created_trash)?;
        }
        Ok(())
    }
}

fn set_binder_activated(
    uow: &dyn TrashBinderUnitOfWorkTrait,
    id: EntityId,
    value: bool,
) -> Result<()> {
    if let Some(mut binder) = uow.get_binder(&id)? {
        binder.activated = value;
        uow.update_binder(&binder)?;
    }
    Ok(())
}

fn set_items_activated(
    uow: &dyn TrashBinderUnitOfWorkTrait,
    ids: &[EntityId],
    value: bool,
) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }
    let mut updated: Vec<BinderItem> = Vec::new();
    for it in uow.get_binder_item_multi(ids)?.into_iter().flatten() {
        let mut it = it;
        it.activated = value;
        updated.push(it);
    }
    if !updated.is_empty() {
        uow.update_binder_item_multi(&updated)?;
    }
    Ok(())
}

// `get_work_relationship` does not validate that `id` is a real, open Work --
// a junction lookup against an unknown id just comes back empty, which would
// silently no-op instead of reporting the caller's mistake. So the id from
// `dto.work_id` is checked against the open Works first, exactly like
// `empty_trash_uc.rs` does.
fn work_id(uow: &dyn TrashBinderUnitOfWorkTrait, requested: EntityId) -> Result<EntityId> {
    uow.get_all_work()?
        .into_iter()
        .find(|w| w.id == requested)
        .map(|w| w.id)
        .ok_or_else(|| anyhow!("work {requested} is not open"))
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
