// Custom implementation: soft-delete (trash) a whole Binder. The binder and all
// its items flip `activated` to false; a single TrashInfo (TrashedBinder) is
// indexed under System.trash_infos. Undoable via whole-store snapshot/restore.
use crate::TrashBinderDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::system::SystemRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::entities::{Binder, BinderItem, System, TrashInfo};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;

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
#[macros::uow_action(entity = "Binder", action = "Get")]
#[macros::uow_action(entity = "Binder", action = "Update")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "Snapshot")]
#[macros::uow_action(entity = "Binder", action = "Restore")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
pub trait TrashBinderUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_trash_binder_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct TrashBinderUseCase {
    uow_factory: Box<dyn TrashBinderUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl TrashBinderUseCase {
    pub fn new(uow_factory: Box<dyn TrashBinderUnitOfWorkFactoryTrait>) -> Self {
        TrashBinderUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self, dto: &TrashBinderDto) -> Result<()> {
        let binder_id = dto.binder_id as EntityId;

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let snap_before = uow.snapshot_binder(&[])?;

        let mut binder = uow
            .get_binder(&binder_id)?
            .ok_or_else(|| anyhow!("trash_binder: binder {binder_id} not found"))?;
        binder.activated = false;
        uow.update_binder(&binder)?;

        let item_ids =
            uow.get_binder_relationship(&binder_id, &BinderRelationshipField::BinderItems)?;
        let mut updated: Vec<BinderItem> = Vec::new();
        for it in uow.get_binder_item_multi(&item_ids)?.into_iter().flatten() {
            let mut it = it;
            it.activated = false;
            updated.push(it);
        }
        if !updated.is_empty() {
            uow.update_binder_item_multi(&updated)?;
        }

        let system = system_singleton(uow.as_ref())?;
        let mut trash_infos =
            uow.get_system_relationship(&system.id, &SystemRelationshipField::TrashInfos)?;
        let now = chrono::Utc::now();
        let info = uow.create_orphan_trash_info(&TrashInfo {
            created_at: now,
            updated_at: now,
            trashed_at: now,
            origin_binder_id: 0, // a whole binder has no parent binder
            ..Default::default()
        })?;
        uow.set_trash_info_relationship(
            &info.id,
            &TrashInfoRelationshipField::TrashedBinder,
            &[binder_id],
        )?;
        trash_infos.push(info.id);
        uow.set_system_relationship(
            &system.id,
            &SystemRelationshipField::TrashInfos,
            &trash_infos,
        )?;

        let snap_after = uow.snapshot_binder(&[])?;
        uow.commit()?;
        uow.publish_trash_binder_event(vec![binder_id], None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(())
    }
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
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("trash_binder: nothing to undo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_binder(snap)?;
        uow.commit()?;
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let snap = self
            .snap_after
            .as_ref()
            .ok_or_else(|| anyhow!("trash_binder: nothing to redo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_binder(snap)?;
        uow.commit()?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
