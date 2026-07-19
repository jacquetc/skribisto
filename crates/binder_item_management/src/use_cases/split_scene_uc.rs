// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: split scene A (source) at the caret into two. A keeps
// the before-caret half; a new Scene is created immediately after A carrying the
// after-caret half. Both writing roles are reassigned: `SceneText` from
// `before_text`/`after_text` and `SynopsisText` from `before_synopsis`/
// `after_synopsis`, so the split works from *either* editor — the caller cuts the
// role it is editing at the caret and passes the other role whole to the source
// (empty to the new scene). The Djot-aware text split is done UI-side; this use
// case does only the atomic structural change.
//
// The writes are authoritative reassignments, not appends (contrast
// `merge_two_scenes`, which skips an empty source text because it is appending
// onto an already-correct row): an empty half must genuinely empty its row, or a
// split from the synopsis would leave the new scene carrying a copy of the
// source's prose.
//
// Undoable via a scoped snapshot/restore of the source's binder subtree.
use crate::SplitSceneDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::entities::{BinderItem, BinderItemRole, BinderItemSubRole, Content, ContentRole};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use skribisto_model::RoleExt;

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
        // The scoped snapshot is taken below, once the source's binder is known.

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
        // Any prose-bearing row can be split — the constraint matrix decides which those
        // are, not a hardcoded sub_role list. That is what makes a chapter *folder*
        // splittable: `Folder/ChapterScene` carries a `SceneText` exactly like the flat
        // `Item/ChapterScene` it promotes to/from, so refusing to split its prose would
        // contradict the model. Title-only rows (a Part, a Book) carry no prose and are
        // rejected here.
        if !skribisto_model::content_allowed(&src.role, &src.sub_role, &ContentRole::SceneText) {
            return Err(anyhow!(
                "split_scene: {:?}/{:?} carries no scene prose",
                src.role,
                src.sub_role
            ));
        }

        // Scoped snapshot of the source's binder subtree, before the first mutation.
        let snap_before = uow.snapshot_binder(&[binder])?;

        let now = chrono::Utc::now();

        // 1. Reassign the source's writing roles to the before-caret halves.
        let mut src_content_ids =
            uow.get_binder_item_relationship(&source, &BinderItemRelationshipField::Contents)?;
        let src_rows: Vec<Content> = uow
            .get_content_multi(&src_content_ids)?
            .into_iter()
            .flatten()
            .collect();
        let mut src_ids_changed = false;
        for (role, text) in [
            (ContentRole::SceneText, &dto.before_text),
            (ContentRole::SynopsisText, &dto.before_synopsis),
        ] {
            match src_rows.iter().find(|c| c.role == role).cloned() {
                Some(mut row) => {
                    row.data = text.clone();
                    row.updated_at = now;
                    uow.update_content(&row)?;
                }
                None => {
                    // Don't materialise an empty row for a role the source never had.
                    if text.is_empty() {
                        continue;
                    }
                    let created = uow.create_orphan_content(&Content {
                        created_at: now,
                        updated_at: now,
                        activated: true,
                        role,
                        data: text.clone(),
                        ..Default::default()
                    })?;
                    src_content_ids.push(created.id);
                    src_ids_changed = true;
                }
            }
        }
        if src_ids_changed {
            uow.set_binder_item_relationship(
                &source,
                &BinderItemRelationshipField::Contents,
                &src_content_ids,
            )?;
        }

        // 2. Create the new scene carrying the after-caret halves. `Item/Scene`
        //    allows both SceneText and SynopsisText (skribisto_model), so both
        //    roles are legal on it by construction.
        //
        //    Indent: splitting a leaf yields a *sibling*, but splitting a container
        //    (a chapter folder's own prose) must yield a *child* — the cut-off
        //    prose becomes that chapter's first scene, not a scene after the chapter.
        //    Indent is UI-only, but getting it wrong would visibly eject the new
        //    scene from the chapter it came out of.
        let indent = if src.role.is_container() {
            src.indent + 1
        } else {
            src.indent
        };
        let new_item = uow.create_orphan_binder_item(&BinderItem {
            // The second half of a split is a NEW row with its own identity.
            uid: common::uid::new_uid(),
            created_at: now,
            updated_at: now,
            title: dto.new_title.clone(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
            activated: true,
            is_exportable: true,
            indent,
            // Both halves are the same prose in the same language: without this the new
            // half falls back to the Work language, silently losing a per-item override
            // (a French passage inside an English project reverts to English for
            // spell-checking and search folding).
            dict_language: src.dict_language.clone(),
            // Everything else is deliberately NOT carried from `src`, unlike `duplicate`:
            // duplicating makes a second copy of the same thing, whereas splitting makes a
            // genuinely new scene that happens to start with the source's back half.
            //   * `aliases` — two scenes answering to one entity's name would be wrong.
            //   * `word_count_goal`/`char_count_goal` — per-scene targets; inheriting them
            //     would silently double the project's total goal on every split.
            //   * `is_favorite`, `label`, `sub_title` — the author's annotations about the
            //     *source* scene, not facts about the prose that moved.
            // `dict_language` above is the sole exception because it describes the prose
            // itself, and the prose is the one thing genuinely shared between the halves.
            ..Default::default()
        })?;
        let mut new_content_ids = Vec::new();
        for (role, text) in [
            (ContentRole::SceneText, &dto.after_text),
            (ContentRole::SynopsisText, &dto.after_synopsis),
        ] {
            if text.is_empty() {
                continue; // the split left this role entirely on the source
            }
            let created = uow.create_orphan_content(&Content {
                created_at: now,
                updated_at: now,
                activated: true,
                role,
                data: text.clone(),
                ..Default::default()
            })?;
            new_content_ids.push(created.id);
        }
        uow.set_binder_item_relationship(
            &new_item.id,
            &BinderItemRelationshipField::Contents,
            &new_content_ids,
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

        let snap_after = uow.snapshot_binder(&[binder])?;
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
