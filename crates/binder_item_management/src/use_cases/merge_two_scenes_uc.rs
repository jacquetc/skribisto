// Custom implementation: merge scene B (source) into scene A (target). A survives
// and receives B's SceneText (after a blank line) and its SynopsisText
// (concatenated); B is then sent to Trash (`activated = false` + one TrashInfo
// under System.trash_infos). Undoable via whole-binder snapshot/restore — the
// TrashInfo is non-undoable and stays (filtered out of Trash once B is
// re-activated on undo), exactly like `trash_binder_items`.
use crate::MergeTwoScenesDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::system::SystemRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::entities::{BinderItem, BinderItemSubRole, Content, ContentRole, System, TrashInfo};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use std::collections::HashSet;

pub trait MergeTwoScenesUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn MergeTwoScenesUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/merge_two_scenes_uow.rs.
#[macros::uow_action(entity = "System", action = "GetAll")]
#[macros::uow_action(entity = "System", action = "GetRelationship")]
#[macros::uow_action(entity = "System", action = "SetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "CreateOrphan")]
#[macros::uow_action(entity = "TrashInfo", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Binder", action = "Snapshot")]
#[macros::uow_action(entity = "Binder", action = "Restore")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "SetRelationship")]
#[macros::uow_action(entity = "Content", action = "GetMulti")]
#[macros::uow_action(entity = "Content", action = "Update")]
#[macros::uow_action(entity = "Content", action = "CreateOrphan")]
pub trait MergeTwoScenesUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_merge_two_scenes_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct MergeTwoScenesUseCase {
    uow_factory: Box<dyn MergeTwoScenesUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

impl MergeTwoScenesUseCase {
    pub fn new(uow_factory: Box<dyn MergeTwoScenesUnitOfWorkFactoryTrait>) -> Self {
        MergeTwoScenesUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self, dto: &MergeTwoScenesDto) -> Result<()> {
        let target = dto.target_id as EntityId; // A — survives
        let source = dto.source_id as EntityId; // B — absorbed then trashed
        if target == source {
            return Err(anyhow!("merge_two_scenes: target and source are the same"));
        }

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let snap_before = uow.snapshot_binder(&[])?;

        // Both scenes must live in the same binder.
        let groups = uow.get_binder_relationships_from_right_ids(
            &BinderRelationshipField::BinderItems,
            &[target, source],
        )?;
        if groups.len() != 1 {
            return Err(anyhow!(
                "merge_two_scenes: scenes span {} binders",
                groups.len()
            ));
        }
        let (binder, found) = groups.into_iter().next().unwrap();
        let found: HashSet<EntityId> = found.into_iter().collect();
        if !found.contains(&target) || !found.contains(&source) {
            return Err(anyhow!("merge_two_scenes: a scene is not in the binder"));
        }

        // Load both items (in [target, source] order) and validate they are scenes.
        let items = uow.get_binder_item_multi(&[target, source])?;
        let a = items
            .first()
            .cloned()
            .flatten()
            .ok_or_else(|| anyhow!("merge_two_scenes: target vanished"))?;
        let b = items
            .get(1)
            .cloned()
            .flatten()
            .ok_or_else(|| anyhow!("merge_two_scenes: source vanished"))?;
        if !is_scene(&a.sub_role) || !is_scene(&b.sub_role) {
            return Err(anyhow!("merge_two_scenes: both items must be scenes"));
        }

        // Read A's content rows (keep their ids) and B's rows (source text).
        let mut a_content_ids =
            uow.get_binder_item_relationship(&target, &BinderItemRelationshipField::Contents)?;
        let a_rows: Vec<Content> = uow
            .get_content_multi(&a_content_ids)?
            .into_iter()
            .flatten()
            .collect();
        let b_content_ids =
            uow.get_binder_item_relationship(&source, &BinderItemRelationshipField::Contents)?;
        let b_rows: Vec<Content> = uow
            .get_content_multi(&b_content_ids)?
            .into_iter()
            .flatten()
            .collect();

        let now = chrono::Utc::now();
        for role in [ContentRole::SceneText, ContentRole::SynopsisText] {
            let b_text = b_rows
                .iter()
                .find(|c| c.role == role)
                .map(|c| c.data.clone())
                .unwrap_or_default();
            if b_text.trim().is_empty() {
                continue; // nothing to append for this role
            }
            match a_rows.iter().find(|c| c.role == role).cloned() {
                Some(mut row) => {
                    row.data = join_text(&row.data, &b_text);
                    row.updated_at = now;
                    uow.update_content(&row)?;
                }
                None => {
                    let created = uow.create_orphan_content(&Content {
                        created_at: now,
                        updated_at: now,
                        activated: true,
                        role: role.clone(),
                        data: b_text,
                        ..Default::default()
                    })?;
                    a_content_ids.push(created.id);
                    uow.set_binder_item_relationship(
                        &target,
                        &BinderItemRelationshipField::Contents,
                        &a_content_ids,
                    )?;
                }
            }
        }

        // Trash B: flip `activated` and index one TrashInfo under System.
        let mut b_off = b.clone();
        b_off.activated = false;
        uow.update_binder_item_multi(&[b_off])?;

        let system = uow
            .get_all_system()?
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("merge_two_scenes: no System entity"))?;
        let mut trash_infos =
            uow.get_system_relationship(&system.id, &SystemRelationshipField::TrashInfos)?;
        let info = uow.create_orphan_trash_info(&TrashInfo {
            created_at: now,
            updated_at: now,
            trashed_at: now,
            origin_binder_id: binder as i64,
            ..Default::default()
        })?;
        uow.set_trash_info_relationship(
            &info.id,
            &TrashInfoRelationshipField::TrashedBinderItem,
            &[source],
        )?;
        trash_infos.push(info.id);
        uow.set_system_relationship(
            &system.id,
            &SystemRelationshipField::TrashInfos,
            &trash_infos,
        )?;

        let snap_after = uow.snapshot_binder(&[])?;
        uow.commit()?;
        uow.publish_merge_two_scenes_event(vec![target, source], None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(())
    }
}

fn is_scene(sr: &BinderItemSubRole) -> bool {
    matches!(
        sr,
        BinderItemSubRole::Scene | BinderItemSubRole::ChapterScene
    )
}

/// Append `b` onto `a` with a blank-line separator (paragraph break in Djot).
fn join_text(a: &str, b: &str) -> String {
    if a.trim().is_empty() {
        b.trim_start().to_string()
    } else {
        format!("{}\n\n{}", a.trim_end(), b.trim_start())
    }
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for MergeTwoScenesUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("merge_two_scenes: nothing to undo"))?;
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
            .ok_or_else(|| anyhow!("merge_two_scenes: nothing to redo"))?;
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
