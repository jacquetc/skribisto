// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: merge row B (source) into row A (target). A survives and
// receives B's SceneText (after a blank line) and its SynopsisText (concatenated);
// B is then sent to Trash (`activated = false` + one TrashInfo under Work).
//
// Who may take part is decided by the constraint matrix, not a hardcoded sub_role
// list. The *target* need only be prose-bearing — which includes a chapter *folder*
// (`Folder/ChapterScene` carries its own SceneText), making merge the exact inverse of
// `split_scene`:
// a scene cut out of a chapter folder merges straight back into it. The *source* is
// trashed, so it must additionally be structurally inert: merging away a
// flat chapter would delete a chapter boundary, and merging away a chapter folder
// would orphan its child scenes. Both are rejected. The two rows
// must also be adjacent in the flat order — merge is only ever an adjacent merge,
// and adjacency is what guarantees no boundary sits between them.
//
// Undoable via a Work-scoped snapshot/restore: both A's Content and the new
// TrashInfo live in the Work trunk (post-reparent), so the affected Work is
// the undo scope. Work resolution: dto.work_id, validated against the open
// Works -- Phase 0.6: this used to pick `get_all_work().next()`, which had no
// defined subject once a second Work was open (undo could silently roll back
// an unrelated Work's tree while leaving the actual merge un-undone).
use crate::MergeTwoScenesDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{BinderItem, Content, ContentRole, TrashInfo, Work};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use skribisto_model::SubRoleExt;
use std::collections::HashSet;

pub trait MergeTwoScenesUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn MergeTwoScenesUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/merge_two_scenes_uow.rs.
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "Snapshot")]
#[macros::uow_action(entity = "Work", action = "Restore")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "TrashInfo", action = "CreateOrphan")]
#[macros::uow_action(entity = "TrashInfo", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
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

        // Load both items (in [target, source] order) and validate them.
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

        // The **target** absorbs prose, so it need only be a prose-bearing row per the
        // constraint matrix — which includes a chapter folder (it carries its own
        // SceneText). That makes merge the exact inverse of split: a scene cut out of a
        // chapter folder can be merged straight back into it.
        if !skribisto_model::content_allowed(&a.role, &a.sub_role, &ContentRole::SceneText) {
            return Err(anyhow!(
                "merge_two_scenes: target {:?}/{:?} carries no scene prose",
                a.role,
                a.sub_role
            ));
        }
        // The **source** is trashed by the merge, so it must be a row whose
        // disappearance destroys nothing: prose-bearing, and *not* a structural
        // opener. Merging away a chapter (either encoding) would silently delete a
        // chapter boundary, and merging away a chapter folder would orphan its child
        // scenes. The UI already refuses both, but the invariant belongs here — the Full
        // Part / Full Book streams are the first views whose rows span several
        // chapters, so a stale row list must not be able to corrupt the structure.
        if !skribisto_model::content_allowed(&b.role, &b.sub_role, &ContentRole::SceneText) {
            return Err(anyhow!(
                "merge_two_scenes: source {:?}/{:?} carries no scene prose",
                b.role,
                b.sub_role
            ));
        }
        if b.sub_role.opens_chapter() || b.sub_role.opens_part() || b.sub_role.opens_book() {
            return Err(anyhow!(
                "merge_two_scenes: source {:?}/{:?} opens a structural section and cannot be merged away",
                b.role,
                b.sub_role
            ));
        }

        // Merge is only ever an *adjacent*-scene merge. Enforce it here, not just
        // in the caller: the Full Part / Full Book streams are the first views
        // whose row list legitimately spans several chapters, so without this a
        // stale row list (or any future caller) could silently concatenate prose
        // across a chapter/part/book boundary. Adjacency in the flat order implies
        // no boundary marker sits between them.
        let order = uow.get_binder_relationship(&binder, &BinderRelationshipField::BinderItems)?;
        let pos_of = |id: EntityId| -> Result<usize> {
            order
                .iter()
                .position(|&x| x == id)
                .ok_or_else(|| anyhow!("merge_two_scenes: scene {id} not in the binder order"))
        };
        if pos_of(target)?.abs_diff(pos_of(source)?) != 1 {
            return Err(anyhow!(
                "merge_two_scenes: target and source are not adjacent"
            ));
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

        // Work-scoped snapshot, after the read-only validation above and before
        // the first mutation below.
        let work_id = work_id(uow.as_ref(), dto.work_id as EntityId)?;
        let snap_before = uow.snapshot_work(&[work_id])?;

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
                    let c = uow.create_orphan_content(&Content {
                        created_at: now,
                        updated_at: now,
                        activated: true,
                        role: role.clone(),
                        data: b_text,
                        ..Default::default()
                    })?;
                    a_content_ids.push(c.id);
                    uow.set_binder_item_relationship(
                        &target,
                        &BinderItemRelationshipField::Contents,
                        &a_content_ids,
                    )?;
                }
            }
        }

        // Trash B: flip `activated` and index one TrashInfo under Work.
        let mut b_off = b.clone();
        b_off.activated = false;
        uow.update_binder_item_multi(&[b_off])?;

        let mut trash_infos =
            uow.get_work_relationship(&work_id, &WorkRelationshipField::TrashInfos)?;
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
        uow.set_work_relationship(&work_id, &WorkRelationshipField::TrashInfos, &trash_infos)?;

        let snap_after = uow.snapshot_work(&[work_id])?;
        uow.commit()?;
        uow.publish_merge_two_scenes_event(vec![target, source], None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(())
    }
}

// `get_work_relationship` does not validate that `id` is a real, open Work --
// a junction lookup against an unknown id just comes back empty, which would
// silently no-op instead of reporting the caller's mistake. So the id from
// `dto.work_id` is checked against the open Works first, exactly like
// `empty_trash_uc.rs` does.
fn work_id(uow: &dyn MergeTwoScenesUnitOfWorkTrait, requested: EntityId) -> Result<EntityId> {
    uow.get_all_work()?
        .into_iter()
        .find(|w| w.id == requested)
        .map(|w| w.id)
        .ok_or_else(|| anyhow!("work {requested} is not open"))
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
        uow.restore_work(snap)?;
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
        uow.restore_work(snap)?;
        uow.commit()?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
