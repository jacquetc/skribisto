// Custom implementation: "promote" toggles a binder item to its paired type
// (flat Chapter <-> Chapter folder, Scene <-> Note, Folder <-> Note folder). The
// target is derived from the item's current (role, sub_role); its content roles
// are remapped into the target's vocabulary so prose survives (SceneText <->
// NoteText). The item keeps its place in the binder. Undoable via a scoped
// snapshot/restore of the item's own subtree (item + its Content rows). The
// caller (UI) enforces the "empty folder before demoting a Chapter folder to a
// flat Chapter" rule.
use crate::PromoteDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::entities::{BinderItem, Content};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;

pub trait PromoteUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn PromoteUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/promote_uow.rs.
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "Update")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "Snapshot")]
#[macros::uow_action(entity = "BinderItem", action = "Restore")]
#[macros::uow_action(entity = "Content", action = "GetMulti")]
#[macros::uow_action(entity = "Content", action = "Update")]
pub trait PromoteUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_promote_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct PromoteUseCase {
    uow_factory: Box<dyn PromoteUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl PromoteUseCase {
    pub fn new(uow_factory: Box<dyn PromoteUnitOfWorkFactoryTrait>) -> Self {
        PromoteUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self, dto: &PromoteDto) -> Result<()> {
        let item_id = dto.item_id as EntityId;

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let snap_before = uow.snapshot_binder_item(&[item_id])?;

        let item = uow
            .get_binder_item_multi(&[item_id])?
            .into_iter()
            .next()
            .flatten()
            .ok_or_else(|| anyhow!("promote: item not found"))?;

        // The paired target type (a bidirectional toggle).
        let (target_role, target_sub_role) =
            skribisto_model::promote_target(&item.role, &item.sub_role)
                .ok_or_else(|| anyhow!("promote: item type has no promote pair"))?;

        let now = chrono::Utc::now();

        // 1. Flip the item's type (scalar update — relationships untouched).
        let mut updated = item.clone();
        updated.role = target_role.clone();
        updated.sub_role = target_sub_role.clone();
        updated.updated_at = now;
        uow.update_binder_item(&updated)?;

        // 2. Remap the item's content roles into the target's vocabulary so prose
        //    survives (SceneText <-> NoteText for Scene<->Note; others unchanged).
        let content_ids =
            uow.get_binder_item_relationship(&item_id, &BinderItemRelationshipField::Contents)?;
        let rows: Vec<Content> = uow
            .get_content_multi(&content_ids)?
            .into_iter()
            .flatten()
            .collect();
        for mut row in rows {
            if let Some(new_role) =
                skribisto_model::remap_content(&target_role, &target_sub_role, &row.role)
                && new_role != row.role
            {
                row.role = new_role;
                row.updated_at = now;
                uow.update_content(&row)?;
            }
        }

        let snap_after = uow.snapshot_binder_item(&[item_id])?;
        uow.commit()?;
        uow.publish_promote_event(vec![item_id], None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(())
    }
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for PromoteUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("promote: nothing to undo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_binder_item(snap)?;
        uow.commit()?;
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let snap = self
            .snap_after
            .as_ref()
            .ok_or_else(|| anyhow!("promote: nothing to redo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_binder_item(snap)?;
        uow.commit()?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
