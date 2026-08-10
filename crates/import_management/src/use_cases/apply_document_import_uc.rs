// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation (hand-maintained — do NOT blanket-regenerate): the write
// half of document import. It takes the plan the writer reviewed and accepted and
// creates it, as one undo entry.
//
// Synchronous, and deliberately so. Qleany drops `undoable` under
// `long_operation`, and the expensive part — reading and converting files —
// already happened in `analyze_document_import`. Rows arrive with their Djot
// already built, so this opens a transaction, creates, splices and commits
// without parsing anything.
//
// Undo is `snapshot_binder` / `restore_binder`, the same pair `duplicate_uc`
// uses. It costs a walk of the whole destination binder rather than of what was
// imported; that is the accepted trade for reusing a proven primitive instead of
// building a new one.
//
// Three traps this has to step around, none of which the backend will catch:
//
//  1. `is_exportable` and `activated` both default to `false`, because their DTO
//     derives `Default` and `bool::default()` is false. A row created without
//     setting them is present in the binder and invisible to export and to
//     chapter numbering — with no gap left behind, since chapters renumber to
//     fill it. They are set explicitly on every row below.
//  2. `validate_item` is called only by the UI. Nothing at this layer rejects an
//     illegal `(role, sub_role)`, so this checks the combination itself.
//  3. `content_allowed` is enforced only at save time, where it *drops the row
//     silently*. Prose that its type cannot hold is refused here, while the
//     writer can still see why.
//
// Imported comments are created here, in the same transaction and therefore the
// same undo entry as the rows they annotate. That is not a convenience: a comment
// belongs to a passage, and an undo that took back the passage while leaving the
// note behind would leave a comment pointing into a manuscript that no longer has
// the sentence it was about.
//
// Their anchors are copied across field for field and never reinterpreted. The
// plan already proved every quote against the very Djot being stored here (see
// `document_ingest::plan::anchor_comments`), so second-guessing it at this layer
// could only make it worse.
//
// ── M-S7: recognition on re-import ──────────────────────────────────────────
//
// A `.docx`/`.odt` Skribisto itself exported carries a `uid` on every comment and
// reply it wrote (a private-namespace attribute neither format's own vocabulary
// has — see `document_ingest`'s DOCX/ODT scanners). `ImportComment::uid` /
// `ImportReply::uid` are `None` for a comment the plan never traced a uid for —
// an editor's own remark typed straight into Word or LibreOffice — and `Some` for
// one Skribisto minted on a previous export. That distinction is the whole of the
// recognition rule: a `Some` uid found among `Work.comments` means "the writer
// already has this note, keep it and refresh it"; anything else means "this is
// new," and gets a freshly minted uid exactly as before M-S7. Without it, running
// the same returned file through this use case twice (or once against a file the
// writer had already imported once) created a second copy of every comment and
// reply, because nothing before M-S7 ever looked at what already existed.
//
// **What "update" overwrites, and what it never touches** — because a returning
// file is a sync source, not a full replacement:
//   * File-authoritative (copied across on every match, same as a fresh create):
//     body, resolved state, orphan state, the anchor (kind + every quote/range
//     field), author name + initials, and the reply thread's membership. This is
//     the editor's current say on the passage, and the whole point of reading
//     their file back is to bring it in.
//   * Locally-kept (never touched once assigned): `id` (obviously), `uid` (the
//     very identity the match was found by — a uid a file "changes" is a
//     different comment, not an edit to this one) and `created_at` (the moment
//     the note was first authored is a historical fact a later re-read of the
//     same file must not revise, even though `updated_at` bumps every time).
//   * A **reply's** thread membership is rebuilt from the file's own order on
//     every match: a reply whose uid is recognised is updated in place (same
//     split as the comment itself), a reply with no uid or an unrecognised one is
//     newly created, and the resulting id list — in the file's order — becomes
//     `Comment.replies`. Matching is by **uid, never position**, which is what
//     lets an editor insert a brand-new reply in the middle of a thread without
//     the replies after it being mistaken for new ones too (they still carry
//     their own uids, wherever they now sit in the list).
//   * **Deliberately out of scope**: a reply whose uid existed in a previous
//     import but is absent from this one (the editor deleted it in their own
//     app) is *detached* from `Comment.replies` — it stops showing, because
//     nothing renders a `CommentReply` except through that list — but the row
//     itself is not deleted from the store. Building actual reply deletion needs
//     its own `Remove` action and its own undo story; this milestone's stated
//     job is recognition (no duplication on re-import), not garbage collection,
//     and an unreachable row here is inert data, not a correctness bug.
//   * A matched comment's `content` link is always repointed at *this* import's
//     freshly created `Content` row, never left on whatever row a previous import
//     attached it to — the returning file describes the current pass at the
//     passage, and the old row a previous import made is that pass's own
//     business, not this one's.
//
// **Undo has to put an updated row's PREVIOUS state back, not just detach it.**
// The existing `Binder`-scoped snapshot (see the note above) does not cover
// `Comment`/`CommentReply` at all, and the existing `set_comments_attached`
// detach/reattach trick is only correct for a row this transaction *created* —
// detaching a row that already existed and was visible before this import ran
// would make an unrelated, pre-existing comment vanish on undo. So every matched
// row's pre-transaction state is captured before it is touched
// (`updated_comments`/`updated_replies`, both `(before, after)` pairs) and undo
// writes `before` back — including its relationships, since `Update` itself is
// scalar-only (see the trait's own doc below) — while redo re-applies `after`.
//
// The `#[macros::uow_action(...)]` list below is hand-trimmed to what this
// actually does, and must stay in lockstep with the identical list in
// ../units_of_work/apply_document_import_uow.rs.

use crate::ApplyDocumentImportDto;
use crate::ApplyDocumentImportResultDto;
use crate::dtos::{
    ApplyImportRow, ApplyImportRows, DropPosition, ImportComment, ImportCommentKind,
    ImportOrphanReason, ImportReply,
};
use crate::kind_mapping::kind_to_create_type;
use anyhow::{Result, anyhow};
use binder_ordering::{DropPlace, anchor_for_binder_target, insert_block, resolve_item_target};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::comment::CommentRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{
    BinderItem, Comment, CommentAnchorKind, CommentOrphanReason, CommentReply, Content,
    ContentRole, Work,
};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use skribisto_model::{allowed_content, content_allowed, is_valid_combination};
use std::collections::{HashMap, HashSet};

pub trait ApplyDocumentImportUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn ApplyDocumentImportUnitOfWorkTrait>;
}

// `Work` for `chapter_mode` (how a Chapter is encoded, with no per-item
// override), and for its `comments` collection, which is where an imported comment
// hangs. `Binder` to read the destination order, splice into it, and snapshot it
// for undo. `BinderItem` + `Content` to create the rows themselves. `Comment` +
// `CommentReply` for the editor's notes that came with them.
//
// The undo snapshot is still `Binder`-scoped, and that is deliberate rather than an
// oversight: `Comment` hangs off `Work`, so a comment created here is *not* inside
// the snapshot. `undo` therefore removes it from `Work.comments` explicitly — see
// the note there.
#[macros::uow_action(entity = "Work", action = "Get")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "Snapshot")]
#[macros::uow_action(entity = "Binder", action = "Restore")]
#[macros::uow_action(entity = "BinderItem", action = "CreateOrphan")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "SetRelationship")]
#[macros::uow_action(entity = "Content", action = "CreateOrphan")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "Content", action = "GetMulti")]
#[macros::uow_action(entity = "Content", action = "Update")]
#[macros::uow_action(entity = "Comment", action = "CreateOrphan")]
#[macros::uow_action(entity = "Comment", action = "SetRelationship")]
// M-S7 (recognition on re-import): `GetMulti` reads `Work.comments` up front so
// every incoming uid can be checked against what already exists, and `Update`
// rewrites a matched row's scalar fields in place — see `execute`'s own doc for
// why this is scalar-only and `Content`/`Replies` still go through the existing
// `SetRelationship` action below.
#[macros::uow_action(entity = "Comment", action = "GetMulti")]
#[macros::uow_action(entity = "Comment", action = "Update")]
#[macros::uow_action(entity = "CommentReply", action = "CreateOrphan")]
#[macros::uow_action(entity = "CommentReply", action = "GetMulti")]
#[macros::uow_action(entity = "CommentReply", action = "Update")]
pub trait ApplyDocumentImportUnitOfWorkTrait: CommandUnitOfWork {
    fn publish_apply_document_import_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct ApplyDocumentImportUseCase {
    uow_factory: Box<dyn ApplyDocumentImportUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
    /// The `Work` the import landed in, and the comments it created there.
    ///
    /// Kept because `Comment` hangs off `Work` and the undo snapshot is
    /// `Binder`-scoped, so restoring the binder does not unlink them. Undo detaches
    /// these ids by hand; redo puts them back.
    work_id: EntityId,
    created_comment_ids: Vec<EntityId>,
    /// Existing `Comment` rows this import *recognised* (a matching uid) and
    /// updated in place, each paired with the exact state it was in just before
    /// this transaction touched it. See `execute`'s own module doc ("Undo has to
    /// put an updated row's PREVIOUS state back") for why a detach/reattach trick
    /// — correct for `created_comment_ids` — is not correct here: these rows
    /// existed and were visible before this import ran.
    updated_comments: Vec<(Comment, Comment)>,
    /// Same as `updated_comments`, for a reply recognised (matched uid) within a
    /// recognised or freshly created comment's thread.
    updated_replies: Vec<(CommentReply, CommentReply)>,
}

impl ApplyDocumentImportUseCase {
    pub fn new(uow_factory: Box<dyn ApplyDocumentImportUnitOfWorkFactoryTrait>) -> Self {
        ApplyDocumentImportUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
            work_id: 0,
            created_comment_ids: Vec::new(),
            updated_comments: Vec::new(),
            updated_replies: Vec::new(),
        }
    }

    pub fn execute(
        &mut self,
        dto: &ApplyDocumentImportDto,
    ) -> Result<ApplyDocumentImportResultDto> {
        let ApplyImportRows::Create(rows) = &dto.rows else {
            return Err(anyhow!("apply_document_import: no rows to create"));
        };
        let rows: Vec<&ApplyImportRow> = rows
            .iter()
            .filter(|r| !matches!(r, ApplyImportRow::Empty))
            .collect();
        if rows.is_empty() {
            return Err(anyhow!("apply_document_import: no rows to create"));
        }

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        let work = uow
            .get_work(&dto.work_id)?
            .ok_or_else(|| anyhow!("apply_document_import: work {} not found", dto.work_id))?;
        let chapter_mode = work.chapter_mode.clone();

        let snap_before = uow.snapshot_binder(&[dto.binder_id])?;
        let order =
            uow.get_binder_relationship(&dto.binder_id, &BinderRelationshipField::BinderItems)?;

        // Every existing comment (and every reply nested under one) this Work
        // already has, read once up front so each incoming uid can be checked
        // against what is already there — the whole mechanism M-S7 adds. Still
        // read-only, so it belongs here, before the first mutation below, the
        // same discipline `snap_before` itself already follows.
        let existing_comment_ids =
            uow.get_work_relationship(&dto.work_id, &WorkRelationshipField::Comments)?;
        let existing_comments: Vec<Comment> = uow
            .get_comment_multi(&existing_comment_ids)?
            .into_iter()
            .flatten()
            .collect();
        let existing_reply_ids: Vec<EntityId> = existing_comments
            .iter()
            .flat_map(|c| c.replies.iter().copied())
            .collect();
        // Which thread each existing reply currently sits in, keyed by reply id.
        //
        // The uid lookup below is Work-wide, and on its own that is not safe: a returning file
        // naming a reply uid belonging to a *different* comment — a hand-edited file, a
        // corrupted one, or one produced by some later feature that duplicates a thread — would
        // have that reply's text overwritten and its id appended to this comment's `replies`,
        // while the original comment still lists it too. One row in two threads, with only the
        // last write visible under both. `create_or_update_comment` therefore requires a
        // recognised reply to already belong to the thread being updated.
        let reply_owner_by_id: HashMap<EntityId, EntityId> = existing_comments
            .iter()
            .flat_map(|c| c.replies.iter().map(|r| (*r, c.id)))
            .collect();
        let existing_replies_by_uid: HashMap<uuid::Uuid, CommentReply> = uow
            .get_comment_reply_multi(&existing_reply_ids)?
            .into_iter()
            .flatten()
            .filter(|r| !r.uid.is_nil())
            .map(|r| (r.uid, r))
            .collect();
        let reply_owner_by_uid: HashMap<uuid::Uuid, EntityId> = existing_replies_by_uid
            .values()
            .filter_map(|r| reply_owner_by_id.get(&r.id).map(|owner| (r.uid, *owner)))
            .collect();
        // A nil uid is a pre-M-S1 legacy row (or a struct literal that never set
        // one) — never a value an incoming comment's own uid should be matched
        // against, since every genuinely minted uid is a real, non-nil UUID and a
        // coincidental nil-vs-nil "match" would silently merge two unrelated rows.
        let existing_comments_by_uid: HashMap<uuid::Uuid, Comment> = existing_comments
            .into_iter()
            .filter(|c| !c.uid.is_nil())
            .map(|c| (c.uid, c))
            .collect();

        // The same rows, indexed by the tag a round-trip mark spells — see
        // `ExistingComments::by_tag` for why this, and not the uid map above, is what a real
        // returning file is recognised through.
        let existing_comments_by_tag: HashMap<String, uuid::Uuid> = existing_comments_by_uid
            .keys()
            .map(|uid| (skribisto_model::round_trip::uid_tag(uid), *uid))
            .collect();

        // Everything the import creates gets one timestamp, the way every other
        // creation path in this codebase does — a row's `created_at` records the
        // import, not how long the parsing took.
        // Where the block actually goes, resolved here rather than trusted from the
        // plan: the binder can move between the writer reviewing an import and
        // accepting it, and the plan's own indent was only ever a preview.
        let (base_indent, insertion_anchor) = resolve_destination(uow.as_mut(), dto, &order)?;
        // The plan's rows sit at whatever depth the preview assumed. Shift the whole
        // block so its shallowest row lands at the depth just resolved — one offset for
        // all of them, so the tree the writer reviewed keeps its shape.
        let shift = indent_shift(&rows, base_indent);

        let now = chrono::Utc::now();
        let mut created_ids: Vec<EntityId> = Vec::with_capacity(rows.len());

        let mut created_comment_ids: Vec<EntityId> = Vec::new();
        // (before, after) pairs for every existing Comment/CommentReply this
        // import recognised and updated in place — see the module doc's "Undo
        // has to put an updated row's PREVIOUS state back" for why these are
        // tracked separately from `created_comment_ids`.
        let mut updated_comments: Vec<(Comment, Comment)> = Vec::new();
        let mut updated_replies: Vec<(CommentReply, CommentReply)> = Vec::new();

        // ── Rows the returning file brings home to ones already here ──────────────────
        //
        // Run before the creations, and touching nothing they touch: an update names a row
        // that is already in the binder, at its own depth, and takes no part in the splice
        // below. The two passes are disjoint by construction — a row is either one the mark
        // matched or one it did not.
        let by_tag = items_by_uid_tag(uow.as_mut(), &order)?;
        for row in &rows {
            let ApplyImportRow::Update {
                target_uid_tag,
                replace_prose,
                djot,
                comments,
            } = row
            else {
                continue;
            };

            // A tag naming no row in this binder is not an error to abort the whole import
            // over: the writer may have deleted the chapter locally between exporting and
            // reading the file back, which is an ordinary thing to do. The row is skipped and
            // the rest of the import lands.
            let Some(&item_id) = by_tag.get(target_uid_tag.as_str()) else {
                continue;
            };

            let content_id = prose_content_of(uow.as_mut(), item_id)?;
            let Some(content_id) = content_id else {
                // The row holds no prose row to write into or anchor against. Creating one
                // here would be a different operation than the writer asked for.
                continue;
            };

            if *replace_prose {
                let mut content = uow
                    .get_content_multi(&[content_id])?
                    .into_iter()
                    .flatten()
                    .next()
                    .ok_or_else(|| {
                        anyhow!("apply_document_import: content {content_id} vanished mid-import")
                    })?;
                content.data = djot.clone();
                content.updated_at = now;
                uow.update_content(&content)?;
            }

            for comment in comments {
                let outcome = create_or_update_comment(
                    uow.as_mut(),
                    comment,
                    content_id,
                    now,
                    &ExistingComments {
                        comments: &existing_comments_by_uid,
                        replies: &existing_replies_by_uid,
                        reply_owner: &reply_owner_by_uid,
                        by_tag: &existing_comments_by_tag,
                    },
                    &mut updated_comments,
                    &mut updated_replies,
                )?;
                if outcome.newly_created {
                    created_comment_ids.push(outcome.id);
                }
            }
        }

        for row in &rows {
            let ApplyImportRow::Create {
                indent,
                kind,
                title,
                djot,
                comments,
                // Carried on the wire and not yet read here: this milestone teaches the
                // *importer* to recover a row's identity, and the next one teaches this use
                // case to act on it (update the row it names instead of creating a second
                // copy beside it). Named rather than `..` so that adding the action arm is a
                // change to one line here, not a hunt for where the identity went.
                source_uid_tag: _,
            } = row
            else {
                continue;
            };

            let (role, sub_role) = kind_to_create_type(kind).combo(chapter_mode.clone());

            // Trap 2: nothing below this layer checks the combination.
            if !is_valid_combination(&role, &sub_role) {
                return Err(anyhow!(
                    "apply_document_import: '{title}' resolves to an invalid {role:?}/{sub_role:?}"
                ));
            }

            // Trap 3: prose whose row cannot hold it would be dropped silently at
            // the next save. Refuse it here, where the writer can still be told.
            let content_id = if djot.trim().is_empty() {
                None
            } else {
                let content_role = prose_role_for(&role, &sub_role).ok_or_else(|| {
                    anyhow!(
                        "apply_document_import: '{title}' is a {role:?}/{sub_role:?}, \
                         which holds no prose"
                    )
                })?;
                let created = uow.create_orphan_content(&Content {
                    created_at: now,
                    updated_at: now,
                    activated: true,
                    role: content_role,
                    data: djot.clone(),
                    ..Default::default()
                })?;
                Some(created.id)
            };

            let item = uow.create_orphan_binder_item(&BinderItem {
                // A created row mints its own durable identity. Everything
                // persisted about an item keys on this — including, since the
                // prose-filename fix, its own file on disk.
                uid: common::uid::new_uid(),
                created_at: now,
                updated_at: now,
                title: title.clone(),
                role,
                sub_role,
                // Trap 1: both default to false, and a row that is neither
                // exportable nor activated is present but invisible to export and
                // to chapter numbering.
                activated: true,
                is_exportable: true,
                indent: *indent + shift,
                ..Default::default()
            })?;

            if let Some(content_id) = content_id {
                uow.set_binder_item_relationship(
                    &item.id,
                    &BinderItemRelationshipField::Contents,
                    &[content_id],
                )?;
            }

            // A comment points into prose, so a row that stored none can hold none.
            // That is not a loss to hide: the analyse half only ever attaches
            // comments to rows whose prose it anchored them against, so reaching
            // here with comments and no content would mean the two halves disagree.
            if !comments.is_empty() {
                let Some(content_id) = content_id else {
                    return Err(anyhow!(
                        "apply_document_import: '{title}' carries {} comment(s) but no prose \
                         to anchor them to",
                        comments.len()
                    ));
                };
                for comment in comments {
                    let outcome = create_or_update_comment(
                        uow.as_mut(),
                        comment,
                        content_id,
                        now,
                        &ExistingComments {
                            comments: &existing_comments_by_uid,
                            replies: &existing_replies_by_uid,
                            reply_owner: &reply_owner_by_uid,
                            by_tag: &existing_comments_by_tag,
                        },
                        &mut updated_comments,
                        &mut updated_replies,
                    )?;
                    if outcome.newly_created {
                        created_comment_ids.push(outcome.id);
                    }
                }
            }

            created_ids.push(item.id);
        }

        // Comments hang off the Work, not off the row they annotate — appended to
        // whatever is already there rather than replacing it, since an import is not
        // the only thing that ever made a comment in this project.
        if !created_comment_ids.is_empty() {
            let mut all =
                uow.get_work_relationship(&dto.work_id, &WorkRelationshipField::Comments)?;
            all.extend_from_slice(&created_comment_ids);
            uow.set_work_relationship(&dto.work_id, &WorkRelationshipField::Comments, &all)?;
        }

        // Splice the new block in as one run, before the resolved insertion anchor
        // (or at the end when there is none). Keeping it contiguous is what makes the
        // import one movable, one undoable thing rather than rows scattered through
        // the binder.
        let new_order = insert_block(&order, &created_ids, insertion_anchor);
        uow.set_binder_relationship(
            &dto.binder_id,
            &BinderRelationshipField::BinderItems,
            &new_order,
        )?;

        let snap_after = uow.snapshot_binder(&[dto.binder_id])?;
        uow.commit()?;
        uow.publish_apply_document_import_event(created_ids.clone(), None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        self.work_id = dto.work_id;
        self.created_comment_ids = created_comment_ids;
        self.updated_comments = updated_comments;
        self.updated_replies = updated_replies;
        Ok(ApplyDocumentImportResultDto { created_ids })
    }
}

impl ApplyDocumentImportUseCase {
    /// Add or remove this import's comments from `Work.comments`.
    ///
    /// `Comment` hangs off `Work`, and the undo snapshot is `Binder`-scoped — so
    /// restoring the binder puts the rows back exactly and leaves the comments
    /// dangling in the Work, attached to `Content` rows that no longer exist. That
    /// is the shape of the bug the trash feature's own notes warn about: an index
    /// that outlives what it indexes.
    ///
    /// Detaching by id rather than by snapshot is deliberate too. A `Work`-wide
    /// snapshot would take back comments the writer made *after* the import, which
    /// an undo of the import has no business touching.
    ///
    /// **Only for `created_comment_ids`** — a row this import minted fresh. A row
    /// it *recognised* (matched by uid) already existed and was already reachable
    /// through `Work.comments` before this import ran; detaching it here would
    /// make an unrelated, pre-existing comment vanish on undo. Those go through
    /// `updated_comments`/`updated_replies` instead — see the module doc.
    fn set_comments_attached(
        &self,
        uow: &mut dyn ApplyDocumentImportUnitOfWorkTrait,
        attached: bool,
    ) -> Result<()> {
        if self.created_comment_ids.is_empty() || self.work_id == 0 {
            return Ok(());
        }
        let current = uow.get_work_relationship(&self.work_id, &WorkRelationshipField::Comments)?;
        let mut next: Vec<EntityId> = current
            .into_iter()
            .filter(|id| !self.created_comment_ids.contains(id))
            .collect();
        if attached {
            next.extend_from_slice(&self.created_comment_ids);
        }
        uow.set_work_relationship(&self.work_id, &WorkRelationshipField::Comments, &next)?;
        Ok(())
    }

    /// Put every recognised row this import updated back to `before` (undo) or
    /// forward to `after` (redo).
    ///
    /// `update_comment`/`update_comment_reply` are scalar-only (see the trait's
    /// own `#[macros::uow_action]` doc) — `Comment.content`/`Comment.replies` are
    /// relationships, so they are re-applied through the same `SetRelationship`
    /// action `create_or_update_comment` used the first time, not folded into the
    /// scalar update.
    fn reapply_comment_edits(
        &self,
        uow: &mut dyn ApplyDocumentImportUnitOfWorkTrait,
        forward: bool,
    ) -> Result<()> {
        for (before, after) in &self.updated_comments {
            let target = if forward { after } else { before };
            uow.update_comment(target)?;
            // `target.content` may legitimately be `None` on the vanishingly rare
            // row that predates a `content` link at all — the zero-or-one-element
            // slice this collects into is `CommentRelationshipField::Content`'s
            // (a `one_to_one`) own shape, the same one `create_or_update_comment`
            // sends via its literal `&[content_id]`.
            let content_ids: Vec<EntityId> = target.content.iter().copied().collect();
            uow.set_comment_relationship(
                &target.id,
                &CommentRelationshipField::Content,
                &content_ids,
            )?;
            uow.set_comment_relationship(
                &target.id,
                &CommentRelationshipField::Replies,
                &target.replies,
            )?;
        }
        for (before, after) in &self.updated_replies {
            let target = if forward { after } else { before };
            uow.update_comment_reply(target)?;
        }
        Ok(())
    }
}

/// The outcome of resolving one imported comment against what the Work already
/// has — see `create_or_update_comment`.
struct CommentOutcome {
    id: EntityId,
    /// True for a genuinely new `Comment` row (no local uid match). Only a new
    /// row belongs in `created_comment_ids` — a recognised one was already
    /// reachable through `Work.comments` before this import ran.
    newly_created: bool,
}

/// Create or update one imported comment and its replies, and return its id.
///
/// **Recognition (M-S7):** `comment.uid` (and each reply's own uid) is looked up
/// in `existing_comments`/`existing_replies` — populated once, up front, from
/// `Work.comments` — before deciding whether to create or update. See the module
/// doc at the top of this file for exactly which fields the returning file gets
/// to overwrite on a match (body, resolved state, the anchor, author display
/// fields, thread membership) and which a local row keeps regardless (`id`,
/// `uid`, `created_at`). Every anchor field is otherwise copied straight across
/// unexamined either way: `document_ingest::plan` already resolved the quote
/// against this exact Djot with the same matcher the editor uses, so
/// second-guessing it at this layer could only make it worse.
/// What the Work already holds, indexed the way recognition asks about it.
///
/// Grouped rather than passed as three loose maps: they are read together, built together, and
/// meaningless apart — `reply_owner` in particular only makes sense alongside the replies it
/// scopes.
struct ExistingComments<'a> {
    comments: &'a HashMap<uuid::Uuid, Comment>,
    replies: &'a HashMap<uuid::Uuid, CommentReply>,
    /// Reply uid → the comment that currently owns it. A recognised reply is updated in place
    /// only when it already belongs to the thread being updated; see the map's own
    /// construction for what reparenting one would corrupt.
    reply_owner: &'a HashMap<uuid::Uuid, EntityId>,
    /// `round_trip::uid_tag(comment.uid)` → that uid, for every comment the Work holds.
    ///
    /// The index recognition actually runs on. `ImportComment::uid` arrives populated only from
    /// a file no editor has saved — Word and LibreOffice both delete the private attribute it
    /// comes from — so on a real returning file the only identity present is the bookmark's
    /// tag, and a tag is a one-way hash that cannot be turned back into a uid. Hashing what the
    /// project already has is how the two are brought together.
    by_tag: &'a HashMap<String, uuid::Uuid>,
}

fn create_or_update_comment(
    uow: &mut dyn ApplyDocumentImportUnitOfWorkTrait,
    comment: &ImportComment,
    content_id: EntityId,
    now: chrono::DateTime<chrono::Utc>,
    existing: &ExistingComments<'_>,
    updated_comments: &mut Vec<(Comment, Comment)>,
    updated_replies: &mut Vec<(CommentReply, CommentReply)>,
) -> Result<CommentOutcome> {
    let ExistingComments {
        comments: existing_comments,
        replies: existing_replies,
        reply_owner,
        by_tag,
    } = existing;
    let ImportComment::Found {
        kind,
        uid,
        uid_tag,
        author_name,
        author_initials,
        created_at,
        body,
        resolved,
        orphaned,
        orphan_reason,
        range_start,
        range_length,
        quote_prefix,
        quote_exact,
        quote_exact_truncated,
        quote_suffix,
        block_ordinal_hint,
        replies,
    } = comment
    else {
        return Err(anyhow!("apply_document_import: an empty comment row"));
    };

    // The author's own date, when the source carried a readable one. A comment
    // stamped with the moment of import would tell the writer nothing they did not
    // already know, and would lose the one fact the file actually recorded.
    let written = parse_created_at(created_at).unwrap_or(now);

    // Every reply, resolved the same way as the comment itself below: a matching
    // uid updates the existing row in place, anything else is new. Rebuilt from
    // the file's own order and matched by **uid, never position** — the whole
    // reason an editor inserting a brand-new reply mid-thread does not make every
    // reply after it look new too (they still carry their own uids, wherever they
    // now sit in this list).
    // Resolved before the replies, because whether an incoming reply may be updated in place
    // depends on whether it already belongs to *this* thread — and that question needs the
    // thread's own id. `None` while creating a brand-new comment, where no incoming reply can
    // legitimately have an existing owner at all.
    // Which comment this *is*, resolved from whichever carrier the file still has.
    //
    // The uid first, because it is exact when present — a file this app wrote that no editor
    // has opened. Then the mark's tag, which is the case that actually happens: both Word and
    // LibreOffice delete the private attribute the uid rides on, so a manuscript coming back
    // from a real editor carries only the bookmark. Without this second step, recognition
    // works in tests and never once in practice, and every returning file duplicates every
    // comment it brought.
    let resolved_uid: Option<uuid::Uuid> = uid
        .filter(|u| existing_comments.contains_key(u))
        .or_else(|| Some(*by_tag.get(uid_tag.as_str())?).filter(|_| !uid_tag.is_empty()));
    let existing_comment = resolved_uid.and_then(|u| existing_comments.get(&u));
    let existing_id: Option<EntityId> = existing_comment.map(|c| c.id);

    // Existing replies of *this* thread already claimed by an incoming one, so two incoming
    // replies cannot both match the same row — see the natural-key fallback below.
    let mut claimed: HashSet<uuid::Uuid> = HashSet::new();

    let mut reply_ids: Vec<EntityId> = Vec::with_capacity(replies.len());
    for reply in replies {
        let ImportReply::Found {
            uid: reply_uid,
            author_name,
            author_initials,
            created_at,
            body,
        } = reply
        else {
            continue;
        };
        let at = parse_created_at(created_at).unwrap_or(written);

        // Recognised only when the reply already belongs to *this* thread. A uid naming a reply
        // that currently sits under a different comment is treated as unrecognised and creates
        // a new row, rather than lifting someone else's reply out of their conversation — see
        // `reply_owner`'s construction. `existing_id` is the comment being updated, and is
        // `None` while creating a brand-new one, where no incoming reply can legitimately
        // already have an owner.
        let recognised = reply_uid
            .filter(|u| reply_owner.get(u).copied() == existing_id)
            .or_else(|| {
                // **A reply carries no mark of its own.** Both formats anchor a reply to the
                // same range as the thread it answers, so there is no span for a bookmark to
                // bracket and nothing to name it with — and the uid above is gone the moment
                // an editor saves. Left there, the *second* round trip re-creates every reply
                // the first one brought home, and a thread grows a duplicate of itself on
                // every exchange.
                //
                // So a reply falls back to its natural key: the author who wrote it and the
                // moment they wrote it, both carried natively by ODF (`dc:creator`/`dc:date`)
                // and OOXML (`w:author`/`w:date`). Two replies by one author in the same
                // second is not a thing that happens; two replies by *different* authors, or
                // the same author at different times, never collide. Scoped to this thread's
                // own replies, and each existing row can be claimed only once.
                existing_id?;
                let hit = existing_replies.values().find(|r| {
                    reply_owner.get(&r.uid).copied() == existing_id
                        && !claimed.contains(&r.uid)
                        && r.author_name == *author_name
                        && r.created_at == at
                })?;
                Some(hit.uid)
            })
            .inspect(|u| {
                claimed.insert(*u);
            });
        let reply_id = match recognised.and_then(|u| existing_replies.get(&u)) {
            Some(existing) => {
                let updated = CommentReply {
                    id: existing.id,
                    // Kept: identity, and the moment this reply was first
                    // authored — a later re-read of the same file must not
                    // revise history, even though `updated_at` bumps every time.
                    created_at: existing.created_at,
                    uid: existing.uid,
                    // File-authoritative: everything describing what it says.
                    updated_at: at,
                    author_name: author_name.clone(),
                    author_initials: author_initials.clone(),
                    body: body.clone(),
                };
                uow.update_comment_reply(&updated)?;
                updated_replies.push((existing.clone(), updated));
                existing.id
            }
            None => {
                let created = uow.create_orphan_comment_reply(&CommentReply {
                    // A reply the file carried no recognisable uid for is new —
                    // mint one exactly as the pre-M-S7 path always did (or reuse
                    // the file's own uid when it named one this Work simply
                    // never had — e.g. the local row it once matched was
                    // deleted), so it can itself be recognised on the *next*
                    // round trip.
                    uid: reply_uid.unwrap_or_else(common::uid::new_uid),
                    created_at: at,
                    updated_at: at,
                    author_name: author_name.clone(),
                    author_initials: author_initials.clone(),
                    body: body.clone(),
                    ..Default::default()
                })?;
                created.id
            }
        };
        reply_ids.push(reply_id);
    }

    let (id, newly_created) = match existing_comment {
        Some(existing) => {
            let updated = Comment {
                id: existing.id,
                // Kept, for the same reason a reply's are above.
                created_at: existing.created_at,
                uid: existing.uid,
                // File-authoritative: everything describing what the note says
                // and where it points, including which `Content` row it is
                // anchored to — always *this* import's freshly created row,
                // never whatever an earlier import last attached it to.
                updated_at: written,
                content: Some(content_id),
                kind: anchor_kind(kind),
                author_name: author_name.clone(),
                author_initials: author_initials.clone(),
                body: body.clone(),
                resolved: *resolved,
                orphaned: *orphaned,
                orphan_reason: orphan_reason_of(orphan_reason),
                range_start: (*range_start).max(0) as u64,
                range_length: (*range_length).max(0) as u64,
                quote_prefix: quote_prefix.clone(),
                quote_exact: quote_exact.clone(),
                quote_exact_truncated: *quote_exact_truncated,
                quote_suffix: quote_suffix.clone(),
                block_ordinal_hint: (*block_ordinal_hint).max(0) as u64,
                replies: reply_ids.clone(),
            };
            // `Update` is scalar-only (see the trait's own doc); `content` and
            // `replies` are relationships and go through the same
            // `SetRelationship` action the create branch below uses.
            uow.update_comment(&updated)?;
            uow.set_comment_relationship(
                &updated.id,
                &CommentRelationshipField::Content,
                &[content_id],
            )?;
            uow.set_comment_relationship(
                &updated.id,
                &CommentRelationshipField::Replies,
                &reply_ids,
            )?;
            updated_comments.push((existing.clone(), updated));
            (existing.id, false)
        }
        None => {
            let created = uow.create_orphan_comment(&Comment {
                // No local uid match — either the file names none at all (an
                // editor's own remark, typed straight into Word or LibreOffice)
                // or names one this Work does not currently have (e.g. the row
                // it once matched was deleted locally). Either way this is a
                // fresh row: reuse the file's own uid when it offered one —
                // preserving that identity is strictly better than discarding it
                // — and mint a new one otherwise, exactly as the pre-M-S7 path
                // always did.
                uid: uid.unwrap_or_else(common::uid::new_uid),
                created_at: written,
                updated_at: written,
                kind: anchor_kind(kind),
                author_name: author_name.clone(),
                author_initials: author_initials.clone(),
                body: body.clone(),
                resolved: *resolved,
                orphaned: *orphaned,
                orphan_reason: orphan_reason_of(orphan_reason),
                range_start: (*range_start).max(0) as u64,
                range_length: (*range_length).max(0) as u64,
                quote_prefix: quote_prefix.clone(),
                quote_exact: quote_exact.clone(),
                quote_exact_truncated: *quote_exact_truncated,
                quote_suffix: quote_suffix.clone(),
                block_ordinal_hint: (*block_ordinal_hint).max(0) as u64,
                ..Default::default()
            })?;

            uow.set_comment_relationship(
                &created.id,
                &CommentRelationshipField::Content,
                &[content_id],
            )?;
            if !reply_ids.is_empty() {
                uow.set_comment_relationship(
                    &created.id,
                    &CommentRelationshipField::Replies,
                    &reply_ids,
                )?;
            }
            (created.id, true)
        }
    };

    Ok(CommentOutcome { id, newly_created })
}

/// RFC 3339, and the two shapes producers write without an offset.
fn parse_created_at(value: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(value) {
        return Some(dt.with_timezone(&chrono::Utc));
    }
    for format in ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%dT%H:%M"] {
        if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(value, format) {
            return Some(naive.and_utc());
        }
    }
    None
}

fn anchor_kind(kind: &ImportCommentKind) -> CommentAnchorKind {
    match kind {
        ImportCommentKind::Range => CommentAnchorKind::Range,
        ImportCommentKind::Paragraph => CommentAnchorKind::Paragraph,
        ImportCommentKind::Document => CommentAnchorKind::Document,
    }
}

fn orphan_reason_of(reason: &ImportOrphanReason) -> CommentOrphanReason {
    match reason {
        ImportOrphanReason::NotOrphaned => CommentOrphanReason::NotOrphaned,
        ImportOrphanReason::TextNotFound => CommentOrphanReason::TextNotFound,
        ImportOrphanReason::Ambiguous => CommentOrphanReason::Ambiguous,
        ImportOrphanReason::TargetDeleted => CommentOrphanReason::TargetDeleted,
    }
}

/// The `ContentRole` a row of this type stores its prose under, or `None` when
/// the type holds no prose at all.
///
/// Asks `skribisto_model` rather than deciding: the constraint matrix is the one
/// authority on what a combination may carry, and a second copy of that judgement
/// here would drift.
fn prose_role_for(
    role: &common::entities::BinderItemRole,
    sub_role: &common::entities::BinderItemSubRole,
) -> Option<ContentRole> {
    for candidate in [
        ContentRole::SceneText,
        ContentRole::NoteText,
        ContentRole::ParatextText,
    ] {
        if content_allowed(role, sub_role, &candidate) {
            return Some(candidate);
        }
    }
    // Nothing prose-shaped is allowed here. `allowed_content` is consulted so the
    // reason is visible in a debugger rather than inferred.
    let _ = allowed_content(role, sub_role);
    None
}

/// Every row of this binder, indexed by the tag a round-trip mark spells.
///
/// Built once per import rather than per row: a manuscript has hundreds of rows and a
/// returning file updates many of them, and hashing every uid once is the difference between
/// one pass over the binder and one pass per update.
///
/// A nil uid is skipped. It means a row created before identity was minted, which no mark can
/// name — and hashing it would give every such row the same tag, so the first would answer for
/// all of them.
fn items_by_uid_tag(
    uow: &mut dyn ApplyDocumentImportUnitOfWorkTrait,
    order: &[EntityId],
) -> Result<HashMap<String, EntityId>> {
    Ok(uow
        .get_binder_item_multi(order)?
        .into_iter()
        .flatten()
        .filter(|item| !item.uid.is_nil())
        .map(|item| (skribisto_model::round_trip::uid_tag(&item.uid), item.id))
        .collect())
}

/// The `Content` row holding this item's prose, if it has one.
///
/// "Prose" is the same three roles [`prose_role_for`] can create — scene text, note text,
/// paratext — never a title or a synopsis. A returning file's chapter is the chapter's *prose*,
/// and writing it into the row that holds the title would put a manuscript where a heading goes.
///
/// `None` when the item has no prose row at all, which is an ordinary shape (an empty chapter
/// folder) and not an error.
fn prose_content_of(
    uow: &mut dyn ApplyDocumentImportUnitOfWorkTrait,
    item_id: EntityId,
) -> Result<Option<EntityId>> {
    let ids = uow.get_binder_item_relationship(&item_id, &BinderItemRelationshipField::Contents)?;
    Ok(uow
        .get_content_multi(&ids)?
        .into_iter()
        .flatten()
        .find(|c| {
            matches!(
                c.role,
                ContentRole::SceneText | ContentRole::NoteText | ContentRole::ParatextText
            )
        })
        .map(|c| c.id))
}

/// How far to move the whole accepted block so its shallowest row sits at
/// `base_indent`.
///
/// One offset for every row, never a per-row rewrite: the tree the writer reviewed
/// has to arrive with the shape they reviewed, and only its depth may move. The plan's
/// own indents were a preview computed against the destination as it looked during
/// analysis; this re-bases them onto the destination as it is at the moment of writing.
fn indent_shift(rows: &[&ApplyImportRow], base_indent: i64) -> i64 {
    let shallowest = rows
        .iter()
        .filter_map(|row| match row {
            ApplyImportRow::Create { indent, .. } => Some(*indent),
            // An update touches a row already in the binder at its own depth; it takes part
            // in no splice and must not drag the created block up or down.
            ApplyImportRow::Update { .. } | ApplyImportRow::Empty => None,
        })
        .min()
        .unwrap_or(0);
    base_indent - shallowest
}

/// Where the accepted block lands: `(base indent, the row to insert before)`.
///
/// **The reason this is not a one-line splice.** It used to be: find the anchor in
/// the order, insert immediately after it. But a container's children *follow* it in
/// the same flat list, so inserting after the anchor row put the import between a
/// chapter and its own scenes — and since parentage is derived purely from indent
/// ("the nearest preceding row with a smaller indent"), those scenes silently
/// re-parented onto the imported book. `binder_ordering::resolve_item_target` steps
/// over the anchor's whole subtree, which is the same answer `restore_items_to`
/// already resolves for the identical question.
///
/// An anchor that is not in this binder is an error rather than a silent append: the
/// writer chose a destination, and quietly putting a hundred rows somewhere else is
/// the "it worked, just not where you said" outcome this whole feature exists to
/// avoid.
fn resolve_destination(
    uow: &mut dyn ApplyDocumentImportUnitOfWorkTrait,
    dto: &ApplyDocumentImportDto,
    order: &[EntityId],
) -> Result<(i64, Option<EntityId>)> {
    let place = match dto.drop_position {
        DropPosition::Before => DropPlace::Before,
        DropPosition::After => DropPlace::After,
        DropPosition::Into => DropPlace::Into,
    };
    let nothing_moving = std::collections::HashSet::new();

    if dto.anchor_item_id == 0 {
        return Ok((0, anchor_for_binder_target(order, place, &nothing_moving)));
    }
    if !order.contains(&dto.anchor_item_id) {
        return Err(anyhow!(
            "apply_document_import: anchor {} is not in this binder",
            dto.anchor_item_id
        ));
    }

    // Indents for the whole destination order — `resolve_item_target` needs them to
    // find where the anchor's subtree ends.
    let items = uow.get_binder_item_multi(order)?;
    let indent: std::collections::HashMap<EntityId, i64> = items
        .iter()
        .flatten()
        .map(|item| (item.id, item.indent))
        .collect();
    let anchor = items
        .iter()
        .flatten()
        .find(|item| item.id == dto.anchor_item_id)
        .ok_or_else(|| {
            anyhow!(
                "apply_document_import: anchor {} could not be read",
                dto.anchor_item_id
            )
        })?;

    resolve_item_target(
        order,
        &indent,
        anchor.id,
        anchor.indent,
        anchor.role == common::entities::BinderItemRole::Folder,
        place,
        &nothing_moving,
    )
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for ApplyDocumentImportUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("apply_document_import: nothing to undo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        // The binder first — this removes the `BinderItem`/`Content` rows this
        // import created, which is what makes it safe for the next step to point
        // a recognised comment's `content` back at whatever *older* row it was
        // attached to before this import ran: that row lives outside the Binder
        // snapshot (an earlier, already-committed transaction), so it is
        // untouched by this restore and still there to point at.
        uow.restore_binder(snap)?;
        self.set_comments_attached(uow.as_mut(), false)?;
        self.reapply_comment_edits(uow.as_mut(), false)?;
        uow.commit()?;
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let snap = self
            .snap_after
            .as_ref()
            .ok_or_else(|| anyhow!("apply_document_import: nothing to redo"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        uow.restore_binder(snap)?;
        self.set_comments_attached(uow.as_mut(), true)?;
        self.reapply_comment_edits(uow.as_mut(), true)?;
        uow.commit()?;
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create(indent: i64) -> ApplyImportRow {
        ApplyImportRow::Create {
            indent,
            kind: crate::dtos::ImportRowKind::Scene,
            title: "row".into(),
            djot: String::new(),
            comments: Vec::new(),
            source_uid_tag: String::new(),
        }
    }

    /// The block moves as one piece: its shallowest row lands at the resolved depth
    /// and every other row keeps its distance from it.
    ///
    /// (There is no `splice_after` to test any more. It inserted immediately after the
    /// anchor *row*, which put an import between a container and its own children —
    /// `binder_ordering::resolve_item_target` steps over the anchor's subtree instead,
    /// and is tested in that crate. The end-to-end consequence is pinned by
    /// `importing_into_a_chapter_never_re_parents_what_was_already_there`.)
    #[test]
    fn the_block_is_rebased_onto_the_destination_as_one_piece() {
        let rows = [create(0), create(1), create(2), create(1)];
        let refs: Vec<&ApplyImportRow> = rows.iter().collect();

        assert_eq!(indent_shift(&refs, 0), 0, "already at the top level");
        assert_eq!(
            indent_shift(&refs, 3),
            3,
            "a nested destination pushes it down"
        );
    }

    /// A plan that was previewed against a deep destination and applied to a shallow
    /// one moves *up*, rather than keeping a depth nothing under it justifies.
    #[test]
    fn a_block_previewed_deeper_than_it_lands_moves_up() {
        let rows = [create(2), create(3)];
        let refs: Vec<&ApplyImportRow> = rows.iter().collect();
        assert_eq!(indent_shift(&refs, 0), -2);
    }

    #[test]
    fn an_empty_block_needs_no_shift() {
        assert_eq!(indent_shift(&[], 4), 4);
    }

    #[test]
    fn a_scene_stores_its_prose_as_scene_text() {
        use common::entities::{BinderItemRole, BinderItemSubRole};
        assert_eq!(
            prose_role_for(&BinderItemRole::Item, &BinderItemSubRole::Scene),
            Some(ContentRole::SceneText)
        );
    }

    #[test]
    fn a_book_holds_no_prose_and_says_so() {
        use common::entities::{BinderItemRole, BinderItemSubRole};
        assert_eq!(
            prose_role_for(&BinderItemRole::Folder, &BinderItemSubRole::Book),
            None
        );
    }
}
