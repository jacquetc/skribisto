// Custom implementation: split scene A (source) at the caret into two. A keeps
// the text before the caret (`before_text`); a new Scene is created immediately
// after A carrying the text after the caret (`after_text`). The Djot-aware text
// split is done UI-side; this use case does only the atomic structural change.
// Undoable via whole-binder snapshot/restore.
use crate::SplitSceneDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::entities::{
    BinderItem, BinderItemRole, BinderItemSubRole, Content, ContentRole,
};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;

pub trait SplitSceneUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn SplitSceneUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/split_scene_uow.rs.
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Binder", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "Snapshot")]
#[macros::uow_action(entity = "Binder", action = "Restore")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "CreateOrphan")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "SetRelationship")]
#[macros::uow_action(entity = "Content", action = "GetMulti")]
#[macros::uow_action(entity = "Content", action = "Update")]
#[macros::uow_action(entity = "Content", action = "CreateOrphan")]
pub trait SplitSceneUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_split_scene_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct SplitSceneUseCase {
    uow_factory: Box<dyn SplitSceneUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl SplitSceneUseCase {
    pub fn new(uow_factory: Box<dyn SplitSceneUnitOfWorkFactoryTrait>) -> Self {
        SplitSceneUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self, dto: &SplitSceneDto) -> Result<()> {
        let source = dto.source_id as EntityId;

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let snap_before = uow.snapshot_binder(&[])?;

        // Locate the source's binder.
        let groups = uow.get_binder_relationships_from_right_ids(
            &BinderRelationshipField::BinderItems,
            &[source],
        )?;
        let (binder, _) = groups
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("split_scene: source is in no binder"))?;

        let src = uow
            .get_binder_item_multi(&[source])?
            .into_iter()
            .next()
            .flatten()
            .ok_or_else(|| anyhow!("split_scene: source vanished"))?;
        if !matches!(
            src.sub_role,
            BinderItemSubRole::Scene | BinderItemSubRole::ChapterScene
        ) {
            return Err(anyhow!("split_scene: source is not a scene"));
        }

        let now = chrono::Utc::now();

        // 1. Overwrite the source's SceneText with the before-caret text.
        let mut src_content_ids =
            uow.get_binder_item_relationship(&source, &BinderItemRelationshipField::Contents)?;
        let src_rows: Vec<Content> = uow
            .get_content_multi(&src_content_ids)?
            .into_iter()
            .flatten()
            .collect();
        match src_rows
            .iter()
            .find(|c| c.role == ContentRole::SceneText)
            .cloned()
        {
            Some(mut row) => {
                row.data = dto.before_text.clone();
                row.updated_at = now;
                uow.update_content(&row)?;
            }
            None => {
                let created = uow.create_orphan_content(&Content {
                    created_at: now,
                    updated_at: now,
                    activated: true,
                    role: ContentRole::SceneText,
                    data: dto.before_text.clone(),
                    ..Default::default()
                })?;
                src_content_ids.push(created.id);
                uow.set_binder_item_relationship(
                    &source,
                    &BinderItemRelationshipField::Contents,
                    &src_content_ids,
                )?;
            }
        }

        // 2. Create the new scene carrying the after-caret text.
        let new_item = uow.create_orphan_binder_item(&BinderItem {
            created_at: now,
            updated_at: now,
            title: dto.new_title.clone(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
            activated: true,
            is_printable: true,
            indent: src.indent,
            ..Default::default()
        })?;
        let new_content = uow.create_orphan_content(&Content {
            created_at: now,
            updated_at: now,
            activated: true,
            role: ContentRole::SceneText,
            data: dto.after_text.clone(),
            ..Default::default()
        })?;
        uow.set_binder_item_relationship(
            &new_item.id,
            &BinderItemRelationshipField::Contents,
            &[new_content.id],
        )?;

        // 3. Splice the new scene into the binder immediately after the source.
        let order = uow.get_binder_relationship(&binder, &BinderRelationshipField::BinderItems)?;
        let pos = order
            .iter()
            .position(|&id| id == source)
            .ok_or_else(|| anyhow!("split_scene: source not in binder order"))?;
        let mut new_order = Vec::with_capacity(order.len() + 1);
        new_order.extend_from_slice(&order[..=pos]);
        new_order.push(new_item.id);
        new_order.extend_from_slice(&order[pos + 1..]);
        uow.set_binder_relationship(&binder, &BinderRelationshipField::BinderItems, &new_order)?;

        let snap_after = uow.snapshot_binder(&[])?;
        uow.commit()?;
        uow.publish_split_scene_event(vec![source, new_item.id], None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(())
    }
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for SplitSceneUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("split_scene: nothing to undo"))?;
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
            .ok_or_else(|| anyhow!("split_scene: nothing to redo"))?;
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
