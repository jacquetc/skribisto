// Custom implementation: merge scene B (source) into scene A (target). A survives
// and receives B's SceneText (after a blank line) and its SynopsisText
// (concatenated); B is then sent to Trash (`activated = false` + one TrashInfo
// under System.trash_infos).
//
// Undoable via a *targeted inverse* (not a whole-store snapshot): the command
// records the A content rows it modified (old data) and created, plus the
// trashed source and the TrashInfo it created — so undo restores A's content
// exactly, reactivates B, and removes that TrashInfo, nothing else.
use crate::MergeTwoScenesDto;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::system::SystemRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::entities::{BinderItem, BinderItemSubRole, Content, ContentRole, System, TrashInfo};
use common::types::EntityId;
use std::collections::{HashMap, HashSet};

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
#[macros::uow_action(entity = "TrashInfo", action = "RemoveMulti")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "UpdateMulti")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "SetRelationship")]
#[macros::uow_action(entity = "Content", action = "GetMulti")]
#[macros::uow_action(entity = "Content", action = "Update")]
#[macros::uow_action(entity = "Content", action = "CreateOrphan")]
#[macros::uow_action(entity = "Content", action = "RemoveMulti")]
pub trait MergeTwoScenesUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_merge_two_scenes_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct MergeTwoScenesUseCase {
    uow_factory: Box<dyn MergeTwoScenesUnitOfWorkFactoryTrait>,
    target: EntityId, // A — survives
    source: EntityId, // B — absorbed then trashed
    // Undo state — (re)populated by apply().
    modified_contents: Vec<(EntityId, String, DateTime<Utc>)>, // (id, old_data, old_updated_at)
    created_contents: Vec<EntityId>,
    a_contents_before: Vec<EntityId>,
    created_trash: Vec<EntityId>,
}

impl MergeTwoScenesUseCase {
    pub fn new(uow_factory: Box<dyn MergeTwoScenesUnitOfWorkFactoryTrait>) -> Self {
        MergeTwoScenesUseCase {
            uow_factory,
            target: 0,
            source: 0,
            modified_contents: Vec::new(),
            created_contents: Vec::new(),
            a_contents_before: Vec::new(),
            created_trash: Vec::new(),
        }
    }

    pub fn execute(&mut self, dto: &MergeTwoScenesDto) -> Result<()> {
        let target = dto.target_id as EntityId;
        let source = dto.source_id as EntityId;
        if target == source {
            return Err(anyhow!("merge_two_scenes: target and source are the same"));
        }
        self.target = target;
        self.source = source;

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        self.apply(uow.as_ref())?;
        uow.commit()?;
        uow.publish_merge_two_scenes_event(vec![target, source], None);
        Ok(())
    }

    /// Forward direction (execute + redo): append B's text into A, then trash B.
    /// Records everything undo needs.
    fn apply(&mut self, uow: &dyn MergeTwoScenesUnitOfWorkTrait) -> Result<()> {
        let target = self.target;
        let source = self.source;

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
        let a_contents_before = a_content_ids.clone();
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

        let now = Utc::now();
        let mut modified: Vec<(EntityId, String, DateTime<Utc>)> = Vec::new();
        let mut created: Vec<EntityId> = Vec::new();
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
                    modified.push((row.id, row.data.clone(), row.updated_at));
                    row.data = join_text(&row.data, &b_text);
                    row.updated_at = now;
                    uow.update_content(&row)?;
                }
                None => {
                    let c = uow.create_orphan_content(&Content {
                        created_at: now,
                        updated_at: now,
                        activated: true,
                        role: role.clone(),
                        data: b_text,
                        ..Default::default()
                    })?;
                    a_content_ids.push(c.id);
                    created.push(c.id);
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

        let system = system_singleton(uow)?;
        let mut trash_infos =
            uow.get_system_relationship(&system, &SystemRelationshipField::TrashInfos)?;
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
        uow.set_system_relationship(&system, &SystemRelationshipField::TrashInfos, &trash_infos)?;

        // Record undo state.
        self.modified_contents = modified;
        self.created_contents = created;
        self.a_contents_before = a_contents_before;
        self.created_trash = vec![info.id];
        Ok(())
    }

    /// Inverse (undo): restore A's content, reactivate B, remove the TrashInfo.
    fn revert(&self, uow: &dyn MergeTwoScenesUnitOfWorkTrait) -> Result<()> {
        // 1. Restore A's modified content rows to their prior data.
        if !self.modified_contents.is_empty() {
            let ids: Vec<EntityId> = self.modified_contents.iter().map(|(id, _, _)| *id).collect();
            let mut rows: HashMap<EntityId, Content> = uow
                .get_content_multi(&ids)?
                .into_iter()
                .flatten()
                .map(|c| (c.id, c))
                .collect();
            for (id, old_data, old_updated) in &self.modified_contents {
                if let Some(mut row) = rows.remove(id) {
                    row.data = old_data.clone();
                    row.updated_at = *old_updated;
                    uow.update_content(&row)?;
                }
            }
        }

        // 2. Remove any A content rows we created, restoring A.contents.
        if !self.created_contents.is_empty() {
            uow.set_binder_item_relationship(
                &self.target,
                &BinderItemRelationshipField::Contents,
                &self.a_contents_before,
            )?;
            uow.remove_content_multi(&self.created_contents)?;
        }

        // 3. Reactivate B.
        if let Some(mut b) = uow
            .get_binder_item_multi(&[self.source])?
            .into_iter()
            .next()
            .flatten()
        {
            b.activated = true;
            uow.update_binder_item_multi(&[b])?;
        }

        // 4. Remove the TrashInfo we created and unlink it from System.
        if !self.created_trash.is_empty() {
            let system = system_singleton(uow)?;
            let drop: HashSet<EntityId> = self.created_trash.iter().copied().collect();
            let remaining: Vec<EntityId> = uow
                .get_system_relationship(&system, &SystemRelationshipField::TrashInfos)?
                .into_iter()
                .filter(|id| !drop.contains(id))
                .collect();
            uow.set_system_relationship(&system, &SystemRelationshipField::TrashInfos, &remaining)?;
            uow.remove_trash_info_multi(&self.created_trash)?;
        }
        Ok(())
    }
}

fn is_scene(sr: &BinderItemSubRole) -> bool {
    matches!(
        sr,
        BinderItemSubRole::Scene | BinderItemSubRole::ChapterScene
    )
}

fn system_singleton(uow: &dyn MergeTwoScenesUnitOfWorkTrait) -> Result<EntityId> {
    uow.get_all_system()?
        .into_iter()
        .next()
        .map(|s| s.id)
        .ok_or_else(|| anyhow!("merge_two_scenes: no System entity"))
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
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        self.revert(uow.as_ref())?;
        uow.commit()?;
        uow.publish_merge_two_scenes_event(vec![self.target, self.source], None);
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        self.apply(uow.as_ref())?;
        uow.commit()?;
        uow.publish_merge_two_scenes_event(vec![self.target, self.source], None);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
