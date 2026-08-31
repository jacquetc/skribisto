// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: merge row B (source) into row A (target). A survives and
// receives B's SceneText (after a blank line) and its SynopsisText (concatenated);
// B is then sent to Trash (`activated = false` + one TrashInfo under Work). Any
// footnote anchored to a row of B's that got folded in reparents onto A's resulting
// row (see `crate::footnote_reanchor`) — otherwise its citation would render live and
// numbered in A while `run_search`/`count_words` kept attributing it to B, which the
// trashing below then drops out of every default view.
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
// TrashInfo live in the Work trunk, so the affected Work is the undo scope.
// dto.work_id must be validated against the open Works before use.
use crate::MergeTwoScenesDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::footnote::FootnoteRelationshipField;
use common::direct_access::trash_info::TrashInfoRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{BinderItem, Content, ContentRole, Footnote, TrashInfo, Work};
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
// A footnote anchored to the source's now-merged-away content must move with its
// text — see `crate::footnote_reanchor`. No `Snapshot`/`Restore` pair of its own is
// needed here (unlike `split_scene`): `Footnote` hangs directly off `Work`, which this
// use case already snapshots/restores wholesale via `Work::Snapshot`/`Work::Restore`
// above, so a `content` change made between `snap_before` and `snap_after` is already
// covered.
#[macros::uow_action(entity = "Footnote", action = "GetMulti")]
#[macros::uow_action(entity = "Footnote", action = "SetRelationship")]
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

        // Ownership check: `binder` (the row target and source were just
        // confirmed to share) must belong to the named Work. Every check above
        // only cross-references target/source against `binder`, never against
        // the Work, so without this two scenes sharing a binder from Work B
        // would sail through under Work A's work_id: the merge would land on
        // Work B while the undo/redo snapshot stayed scoped to Work A.
        if !uow
            .get_work_relationship(&work_id, &WorkRelationshipField::Binders)?
            .contains(&binder)
        {
            return Err(anyhow!(
                "merge_two_scenes: binder {binder} does not belong to work {work_id}"
            ));
        }

        let snap_before = uow.snapshot_work(&[work_id])?;

        // ── The merged row demotes to the LOWER of the two statuses ──────────────────
        //
        // A survives, so without this the target simply keeps its own rung — and merging a
        // "Final" scene with a "Draft" one would leave the result marked Final while it now
        // contains undrafted prose. That is a quiet lie about the manuscript, and exactly
        // the sort a writer sorts and filters on.
        //
        // "Lower" is position in `Work.statuses`, which is an `ordered_one_to_many`: that
        // order IS the ladder, and it is the only thing that defines which of two rungs is
        // less finished. A rung that no longer resolves (the reference is weak — a status
        // can be deleted out from under an item) sorts as unknown and loses to a known one,
        // and "no status" loses to everything, since an unmarked half is the least
        // finished thing there is.
        if a.status != b.status {
            let ladder = uow.get_work_relationship(&work_id, &WorkRelationshipField::Statuses)?;
            let rank = |st: Option<EntityId>| -> Option<usize> {
                st.and_then(|id| ladder.iter().position(|r| *r == id))
            };
            let lower = match (a.status, b.status) {
                // Either half unmarked ⇒ the merge is unmarked.
                (None, _) | (_, None) => None,
                (Some(x), Some(y)) => match (rank(a.status), rank(b.status)) {
                    (Some(rx), Some(ry)) => Some(if rx <= ry { x } else { y }),
                    // One of them is no longer on the ladder at all: keep the one that is,
                    // rather than inventing an order between a live rung and a dead id.
                    (Some(_), None) => Some(x),
                    (None, Some(_)) => Some(y),
                    (None, None) => None,
                },
            };
            if lower != a.status {
                uow.set_binder_item_relationship(
                    &target,
                    &BinderItemRelationshipField::Status,
                    &lower.map(|id| vec![id]).unwrap_or_default(),
                )?;
            }
        }

        let now = chrono::Utc::now();
        // (source row id, target row id) for every role whose text actually moved —
        // fed to the footnote reanchor pass below, once every role has been folded in.
        let mut moved_role_content: Vec<(EntityId, EntityId)> = Vec::new();
        for role in [ContentRole::SceneText, ContentRole::SynopsisText] {
            let b_row = b_rows.iter().find(|c| c.role == role);
            let b_text = b_row.map(|c| c.data.clone()).unwrap_or_default();
            if b_text.trim().is_empty() {
                continue; // nothing to append for this role
            }
            // `b_row` is `Some` here: a non-empty `b_text` can only have come from a
            // row that exists.
            let b_content_id = b_row.expect("non-empty b_text implies a source row").id;
            match a_rows.iter().find(|c| c.role == role).cloned() {
                Some(mut row) => {
                    row.data = join_text(&row.data, &b_text);
                    row.updated_at = now;
                    uow.update_content(&row)?;
                    moved_role_content.push((b_content_id, row.id));
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
                    moved_role_content.push((b_content_id, c.id));
                }
            }
        }

        // Reanchor every footnote whose citation was folded into the target's row —
        // `Footnote.content` is a static pointer nobody else updates (see
        // `crate::footnote_reanchor`), so without this the note would still resolve
        // (search/word-count) to the source's row, which the trashing below removes
        // from every default (non-`include_trashed`) view — silently losing the note
        // from search and from every word count, even though its text and citation are
        // now live in the surviving scene.
        if !moved_role_content.is_empty() {
            let footnote_ids =
                uow.get_work_relationship(&work_id, &WorkRelationshipField::Footnotes)?;
            if !footnote_ids.is_empty() {
                let anchors: Vec<crate::footnote_reanchor::FootnoteAnchor> = uow
                    .get_footnote_multi(&footnote_ids)?
                    .into_iter()
                    .flatten()
                    .map(|f: Footnote| (f.id, f.content, f.label))
                    .collect();
                for (old_content_id, new_content_id) in &moved_role_content {
                    for (footnote_id, new_id) in crate::footnote_reanchor::reanchor_on_merge(
                        &anchors,
                        *old_content_id,
                        *new_content_id,
                    ) {
                        uow.set_footnote_relationship(
                            &footnote_id,
                            &FootnoteRelationshipField::Content,
                            &[new_id],
                        )?;
                    }
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

// `get_work_relationship` doesn't validate that `id` is a real, open Work, so
// check `dto.work_id` against the open Works first (see `empty_trash_uc.rs`).
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
