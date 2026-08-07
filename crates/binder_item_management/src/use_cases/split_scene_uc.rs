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
// Undoable via a scoped snapshot/restore of the source's binder subtree, plus — only
// when a footnote's citation was cut into the new scene — a second, independent scoped
// snapshot/restore of that footnote alone (`Footnote` hangs off `Work`, not `Binder`,
// so it falls outside the binder-subtree scope; see `reanchor_split_footnotes`).
use crate::SplitSceneDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::footnote::FootnoteRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{
    BinderItem, BinderItemRole, BinderItemSubRole, Content, ContentRole, Footnote,
};
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
// Only for reanchoring a footnote whose reference moved into the new scene (see
// `crate::footnote_reanchor`) — `Footnote` hangs off `Work`, not `Binder`, so finding
// "which footnotes does this split's source Work own" needs its own small lookup
// chain (`Work::GetRelationshipsFromRightIds` on `Binders` to resolve the owning Work
// from `binder`, then `Work::GetRelationship` on `Footnotes`), and reparenting one
// needs its own scoped `Snapshot`/`Restore` pair — `Footnote` is not reached by
// `snap_before`/`snap_after`'s binder-subtree scope, so without this an undo would
// revert the split's structural change but leave a repointed anchor dangling.
#[macros::uow_action(entity = "Work", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Footnote", action = "GetMulti")]
#[macros::uow_action(entity = "Footnote", action = "SetRelationship")]
#[macros::uow_action(entity = "Footnote", action = "Snapshot")]
#[macros::uow_action(entity = "Footnote", action = "Restore")]
pub trait SplitSceneUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_split_scene_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct SplitSceneUseCase {
    uow_factory: Box<dyn SplitSceneUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
    // Set only when the split moved a footnote's citation into the new scene (the
    // common case has none). Kept apart from `snap_before`/`snap_after` above because
    // `Footnote` is not part of the binder subtree those snapshot — see the trait's
    // doc comment on the `Footnote` actions.
    footnote_snap_before: Option<EntityTreeSnapshot>,
    footnote_snap_after: Option<EntityTreeSnapshot>,
}

impl SplitSceneUseCase {
    pub fn new(uow_factory: Box<dyn SplitSceneUnitOfWorkFactoryTrait>) -> Self {
        SplitSceneUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
            footnote_snap_before: None,
            footnote_snap_after: None,
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
        // Per-role id of the row the after-caret half landed in, if any — needed below
        // to reanchor a footnote whose citation moved there (`None` when that role's
        // after-text was empty, so nothing was created for it).
        let mut new_scene_content_id: Option<EntityId> = None;
        let mut new_synopsis_content_id: Option<EntityId> = None;
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
                role: role.clone(),
                data: text.clone(),
                ..Default::default()
            })?;
            new_content_ids.push(created.id);
            match role {
                ContentRole::SceneText => new_scene_content_id = Some(created.id),
                ContentRole::SynopsisText => new_synopsis_content_id = Some(created.id),
                _ => {}
            }
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

        // 4. Reanchor any footnote whose `[^label]` citation was cut into the new
        // scene — see `crate::footnote_reanchor` for why this cannot be left to
        // `run_search`/`count_words` to work around: `Footnote.content` is a static
        // pointer nobody else updates, so without this a split silently misattributes
        // (search) or misattributes-and-sometimes-drops (word count) the note.
        let (footnote_snap_before, footnote_snap_after) = self.reanchor_split_footnotes(
            uow.as_mut(),
            binder,
            &src_rows,
            &dto.before_text,
            &dto.after_text,
            new_scene_content_id,
            &dto.before_synopsis,
            &dto.after_synopsis,
            new_synopsis_content_id,
        )?;

        uow.commit()?;
        uow.publish_split_scene_event(vec![source, new_item.id], None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        self.footnote_snap_before = footnote_snap_before;
        self.footnote_snap_after = footnote_snap_after;
        Ok(())
    }

    /// Reparent every footnote whose citation was cut into the new scene, for both
    /// writing roles. Returns a scoped before/after snapshot pair of exactly the
    /// footnotes touched — `None` when nothing moved, which is the common case and
    /// keeps a plain split (no footnote near the caret) from paying for a Work lookup
    /// it does not need beyond resolving the owning Work once.
    #[allow(clippy::too_many_arguments)]
    fn reanchor_split_footnotes(
        &self,
        uow: &mut dyn SplitSceneUnitOfWorkTrait,
        binder: EntityId,
        src_rows: &[Content],
        before_text: &str,
        after_text: &str,
        new_scene_content_id: Option<EntityId>,
        before_synopsis: &str,
        after_synopsis: &str,
        new_synopsis_content_id: Option<EntityId>,
    ) -> Result<(Option<EntityTreeSnapshot>, Option<EntityTreeSnapshot>)> {
        let old_scene_id = src_rows
            .iter()
            .find(|c| c.role == ContentRole::SceneText)
            .map(|c| c.id);
        let old_synopsis_id = src_rows
            .iter()
            .find(|c| c.role == ContentRole::SynopsisText)
            .map(|c| c.id);
        // Nothing existed to anchor onto before this split for either role — no
        // footnote can be involved, so skip the Work/Footnote lookups entirely.
        if old_scene_id.is_none() && old_synopsis_id.is_none() {
            return Ok((None, None));
        }

        let owners =
            uow.get_work_relationships_from_right_ids(&WorkRelationshipField::Binders, &[binder])?;
        let Some((work_id, _)) = owners.into_iter().next() else {
            // The binder's Work vanished between the earlier lookup and here — not
            // this use case's problem to diagnose further; simply nothing to reanchor.
            return Ok((None, None));
        };
        let footnote_ids =
            uow.get_work_relationship(&work_id, &WorkRelationshipField::Footnotes)?;
        if footnote_ids.is_empty() {
            return Ok((None, None));
        }
        let anchors: Vec<crate::footnote_reanchor::FootnoteAnchor> = uow
            .get_footnote_multi(&footnote_ids)?
            .into_iter()
            .flatten()
            .map(|f: Footnote| (f.id, f.content, f.label))
            .collect();

        let mut reparents = Vec::new();
        if let Some(old_id) = old_scene_id {
            reparents.extend(crate::footnote_reanchor::reanchor_on_split(
                &anchors,
                old_id,
                before_text,
                after_text,
                new_scene_content_id,
            ));
        }
        if let Some(old_id) = old_synopsis_id {
            reparents.extend(crate::footnote_reanchor::reanchor_on_split(
                &anchors,
                old_id,
                before_synopsis,
                after_synopsis,
                new_synopsis_content_id,
            ));
        }
        if reparents.is_empty() {
            return Ok((None, None));
        }

        let touched: Vec<EntityId> = reparents.iter().map(|(id, _)| *id).collect();
        let before = uow.snapshot_footnote(&touched)?;
        for (footnote_id, new_content_id) in &reparents {
            uow.set_footnote_relationship(
                footnote_id,
                &FootnoteRelationshipField::Content,
                &[*new_content_id],
            )?;
        }
        let after = uow.snapshot_footnote(&touched)?;
        Ok((Some(before), Some(after)))
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
        // Only set when this split reanchored a footnote (see
        // `reanchor_split_footnotes`) — restores its `content` pointer alongside the
        // structural undo above, in the same transaction, so the two can never drift
        // apart (a footnote left pointing at a row the binder-subtree undo just made
        // vanish).
        if let Some(fsnap) = self.footnote_snap_before.as_ref() {
            uow.restore_footnote(fsnap)?;
        }
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
        if let Some(fsnap) = self.footnote_snap_after.as_ref() {
            uow.restore_footnote(fsnap)?;
        }
        uow.commit()?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
