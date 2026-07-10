// Custom implementation: soft-delete (trash) a whole Binder. The binder and all
// its items flip `activated` to false; a single TrashInfo (TrashedBinder) is
// indexed under System.trash_infos.
//
// Undoable via a *targeted inverse* (not a whole-store snapshot): the command
// records the binder + its items and the TrashInfo id it created, so undo
// reactivates exactly those and removes exactly that TrashInfo.
use crate::TrashBinderDto;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::system::SystemRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::entities::{Binder, BinderItem, System, TrashInfo};
use common::types::EntityId;
use std::collections::HashSet;

pub trait TrashBinderUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn TrashBinderUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/trash_binder_uow.rs.
#[macros::uow_action(entity = "System", action = "GetAll")]
#[macros::uow_action(entity = "System", action = "GetRelationship")]
#[macros::uow_action(entity = "System", action = "SetRelationship")]
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
    // Undo/redo state — targeted inverse, no whole-store snapshot.
    binder_id: EntityId,
    item_ids: Vec<EntityId>,
    trashed_at: DateTime<Utc>,
    created_trash: Vec<EntityId>,
}

impl TrashBinderUseCase {
    pub fn new(uow_factory: Box<dyn TrashBinderUnitOfWorkFactoryTrait>) -> Self {
        TrashBinderUseCase {
            uow_factory,
            binder_id: 0,
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
        self.item_ids = item_ids;
        self.trashed_at = Utc::now();
        self.apply(uow.as_ref())?;

        uow.commit()?;
        uow.publish_trash_binder_event(vec![binder_id], None);
        Ok(())
    }

    /// Forward direction (execute + redo): flip the binder + its items to trashed
    /// and index one TrashInfo (TrashedBinder) under System.trash_infos.
    fn apply(&mut self, uow: &dyn TrashBinderUnitOfWorkTrait) -> Result<()> {
        set_binder_activated(uow, self.binder_id, false)?;
        set_items_activated(uow, &self.item_ids, false)?;

        let system = system_singleton(uow)?;
        let mut index =
            uow.get_system_relationship(&system.id, &SystemRelationshipField::TrashInfos)?;
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
        uow.set_system_relationship(&system.id, &SystemRelationshipField::TrashInfos, &index)?;
        self.created_trash = vec![info.id];
        Ok(())
    }

    /// Inverse (undo): reactivate the binder + its items and drop the TrashInfo
    /// this command created, unlinking it from System.trash_infos.
    fn revert(&self, uow: &dyn TrashBinderUnitOfWorkTrait) -> Result<()> {
        set_binder_activated(uow, self.binder_id, true)?;
        set_items_activated(uow, &self.item_ids, true)?;

        if !self.created_trash.is_empty() {
            let system = system_singleton(uow)?;
            let drop: HashSet<EntityId> = self.created_trash.iter().copied().collect();
            let remaining: Vec<EntityId> = uow
                .get_system_relationship(&system.id, &SystemRelationshipField::TrashInfos)?
                .into_iter()
                .filter(|id| !drop.contains(id))
                .collect();
            uow.set_system_relationship(
                &system.id,
                &SystemRelationshipField::TrashInfos,
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

fn system_singleton(uow: &dyn TrashBinderUnitOfWorkTrait) -> Result<System> {
    uow.get_all_system()?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("trash: no System entity in store"))
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
