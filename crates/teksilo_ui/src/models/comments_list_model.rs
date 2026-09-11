// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reactive list model over the open Work's comment threads.
//!
//! **One model serves both docks.** The leading, project-wide dock binds the whole
//! list; the trailing, current-document dock binds the same handle through a
//! `SortFilterListModel` narrowed to the focused item. Two models over one entity
//! set would drift the moment a comment was resolved in one and not the other.
//!
//! Each row carries the *owning item* as well as the annotated `Content`. That is
//! not redundancy: `Content` has no back-reference to its `BinderItem` (the
//! relationship only points downward), so "which scene is this comment in" — the
//! project dock's breadcrumb and its grouping key — can only be answered by walking
//! the binder tree once and inverting it, exactly as `overview_rows_model` does for
//! its own item↔content map.
//!
//! Two `#[cfg]`-gated `mod imp` variants share one public surface, per the house
//! model/mock convention: the real one reads through `work_commands` /
//! `comment_commands` and stays live on `Comment`/`CommentReply` events plus
//! project switches; the mock one holds a fabricated list, since `--features mocks`
//! has no backend to own a `Comment`.

use frontend::common::entities::{CommentAnchorKind, CommentOrphanReason};

/// One reply in a thread.
///
/// A reply is a first-class conversation turn, not a string. It carries its own
/// id precisely so it can be edited, deleted and replied to exactly like the
/// comment that opened the thread — which is what makes the margin card a
/// conversation rather than a note with a footnote.
#[derive(Clone, Debug, PartialEq)]
pub struct ReplyRow {
    pub id: u64,
    pub author_name: String,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Default for ReplyRow {
    fn default() -> Self {
        Self {
            id: 0,
            author_name: String::new(),
            body: String::new(),
            created_at: chrono::DateTime::UNIX_EPOCH,
        }
    }
}

/// One comment thread, flattened for display.
///
/// `content_id` is `None` for an orphan whose anchored `Content` is gone — the
/// state the bundle-root orphanage preserves across a save. `item_id` is likewise
/// `None` for those, which is what puts them in the docks' "no home" bucket rather
/// than under some unrelated chapter.
#[derive(Clone, Debug, PartialEq)]
pub struct CommentRow {
    pub id: u64,
    pub content_id: Option<u64>,
    pub item_id: Option<u64>,
    pub item_title: String,
    pub kind: CommentAnchorKind,
    pub author_name: String,
    pub body: String,
    pub resolved: bool,
    pub orphaned: bool,
    pub orphan_reason: CommentOrphanReason,
    /// Document-absolute CHARACTER offsets — the space cursors and `FindMatch`
    /// speak. Never bytes, never a block id.
    pub range_start: u64,
    pub range_length: u64,
    pub quote_prefix: String,
    pub quote_exact: String,
    pub quote_exact_truncated: bool,
    pub quote_suffix: String,
    pub block_ordinal_hint: u64,
    /// The thread's replies, oldest first — the whole conversation, not a
    /// summary of it.
    ///
    /// Carried in full because the margin card renders every turn, and the two
    /// summary values the docks want (a count and the last body) are cheap
    /// derivations of it. Storing those *instead* would mean the docks and the
    /// card disagreed about what a thread contains.
    pub replies: Vec<ReplyRow>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Default for CommentRow {
    fn default() -> Self {
        Self {
            id: 0,
            content_id: None,
            item_id: None,
            item_title: String::new(),
            kind: CommentAnchorKind::Range,
            author_name: String::new(),
            body: String::new(),
            resolved: false,
            orphaned: false,
            orphan_reason: CommentOrphanReason::NotOrphaned,
            range_start: 0,
            range_length: 0,
            quote_prefix: String::new(),
            quote_exact: String::new(),
            quote_exact_truncated: false,
            quote_suffix: String::new(),
            block_ordinal_hint: 0,
            replies: Vec::new(),
            created_at: chrono::DateTime::UNIX_EPOCH,
        }
    }
}

impl CommentRow {
    /// Whether this row still points at live text you can seek the caret to and
    /// highlight. An orphan is *shown*, always — it just cannot be navigated to.
    ///
    /// `range_length > 0` is as load-bearing here as `orphaned` and `content_id`:
    /// a comment can resolve *successfully* to an empty range.
    /// `CommentAnchorKind::Document` — the kind the importer minted for a comment
    /// on a heading, a blank paragraph or a table before it stopped being able to
    /// point at prose at all (see `document_ingest::plan`'s module doc, and
    /// `sources::rich`'s for the two cases that still mint it) — resolves to
    /// exactly `Anchor::default()`: a genuine zero-length range, not a missing
    /// one, so `orphaned` stays `false` for it. Before this guard checked
    /// `range_length`, such a row reported itself anchored and a click seeked to
    /// a fabricated `(0, 0)` instead of nowhere. See [`Self::is_unplaced`].
    pub fn is_anchored(&self) -> bool {
        !self.orphaned && self.content_id.is_some() && self.range_length > 0
    }

    /// True for a comment that resolved successfully yet has nowhere to point:
    /// not orphaned — its `Content` is alive and its anchor "resolved" — but its
    /// range is empty. This is `CommentAnchorKind::Document`'s only surviving
    /// shape (see [`Self::is_anchored`]'s doc for why). The docks give it its own
    /// badge and snippet rather than the blank quote a `range_length: 0` row
    /// would otherwise render, and refuse to seek it anywhere.
    pub fn is_unplaced(&self) -> bool {
        !self.orphaned && self.content_id.is_some() && self.range_length == 0
    }

    /// Open threads are the actionable ones, and the number the binder-tree badge
    /// and the Overview rollup report.
    pub fn is_open(&self) -> bool {
        !self.resolved
    }

    pub fn reply_count(&self) -> usize {
        self.replies.len()
    }

    /// The most recent turn, which is what a one-line dock summary shows: the
    /// state of the conversation is its last word, not its first.
    pub fn latest_reply(&self) -> Option<&ReplyRow> {
        self.replies.last()
    }
}

/// A fingerprint of everything about the comment set **except body text**.
///
/// The margin rebuilds on this rather than on every change, and that distinction
/// is load bearing: a card's body editor writes back on each keystroke, which
/// refreshes the model, which would rebuild the margin, which would re-mint the
/// very editor being typed into — resetting the caret to 0 on every character.
///
/// Ids, order, resolved/orphaned state and anchor positions all belong here,
/// because each of them genuinely changes what the margin must draw. The body
/// does not: it only changes what is *inside* a card that already exists.
fn structure_key(rows: &[CommentRow]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    for r in rows {
        r.id.hash(&mut h);
        r.content_id.hash(&mut h);
        r.resolved.hash(&mut h);
        r.orphaned.hash(&mut h);
        r.range_start.hash(&mut h);
        r.range_length.hash(&mut h);
        // Every reply id, not just the count: deleting one and adding another in
        // the same refresh leaves the count untouched while changing what the
        // card must draw.
        for reply in &r.replies {
            reply.id.hash(&mut h);
        }
    }
    h.finish()
}

/// Document order within one item, then by creation time.
///
/// Comments sort by where they sit in the prose because that is the order a writer
/// scrolls past them; recency is only the tie-break (and the explicit alternative
/// sort the dock header offers). Orphans have no meaningful position, so they sort
/// last within their group rather than pretending to be at offset 0.
fn sort_rows(rows: &mut [CommentRow]) {
    rows.sort_by(|a, b| {
        a.item_id
            .is_none()
            .cmp(&b.item_id.is_none())
            .then_with(|| a.item_id.cmp(&b.item_id))
            .then_with(|| a.orphaned.cmp(&b.orphaned))
            .then_with(|| a.block_ordinal_hint.cmp(&b.block_ordinal_hint))
            .then_with(|| a.range_start.cmp(&b.range_start))
            .then_with(|| a.created_at.cmp(&b.created_at))
            .then_with(|| a.id.cmp(&b.id))
    });
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::collections::HashMap;
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{
        binder_commands, binder_item_commands, comment_commands, comment_reply_commands,
        work_commands,
    };
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::direct_access::comment::CommentRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::entities::{CommentAnchorKind, CommentOrphanReason};
    use frontend::common::event::{
        DirectAccessEntity, EntityEvent, Event, Origin, WorkManagementEvent,
    };
    use frontend::direct_access::{
        CommentRelationshipDto, CreateCommentDto, CreateCommentReplyDto, UpdateCommentDto,
        UpdateCommentReplyDto, WorkRelationshipDto,
    };

    use crate::app_ids::AppIds;
    use crate::comments::signature::Signature;

    use super::{CommentRow, sort_rows};

    struct Inner {
        model: ListModel<CommentRow>,
        version: Signal<u64>,
        /// Bumped only when the set's *shape* changes — see [`structure_key`](super::structure_key).
        structure: Signal<u64>,
        last_structure: Cell<u64>,
        ctx: Rc<AppContext>,
        ids: AppIds,
    }

    #[derive(Clone)]
    pub struct CommentsListModel {
        inner: Rc<Inner>,
    }

    impl CommentsListModel {
        pub fn new(ctx: Rc<AppContext>, ids: AppIds) -> Self {
            let rows = ids
                .work_id
                .get()
                .map(|id| load_rows(&ctx, id))
                .unwrap_or_default();
            let key = super::structure_key(&rows);
            let model = ListModel::from_vec(rows);
            Self {
                inner: Rc::new(Inner {
                    model,
                    version: Signal::new(0),
                    structure: Signal::new(0),
                    last_structure: Cell::new(key),
                    ctx,
                    ids,
                }),
            }
        }

        /// Subscribe (once) so both docks stay live.
        ///
        /// `CommentReply` events matter as much as `Comment` ones: a reply changes a
        /// row's summary line and its reply badge, and a dock that only watched
        /// `Comment` would show a stale "2 replies" until something else forced a
        /// refresh. Entity events carry no `work_id`, but `refresh` always re-derives
        /// from this model's own `ids.work_id`, so a sibling Work's event costs a
        /// harmless re-read rather than a wrong one — the same posture
        /// `DictWordListModel` documents.
        ///
        /// **Re-subscribes on every call.** A `BuildContext` subscription is scoped to
        /// the current build and dropped on the next, so the one-shot guard this used to
        /// carry left the model deaf after any rebuild while still reporting itself
        /// wired, and skipped the catch-up below with it.
        pub fn wire(&self, ctx: &mut BuildContext) {
            for ev in [
                EntityEvent::Created,
                EntityEvent::Updated,
                EntityEvent::Removed,
            ] {
                for origin in [
                    Origin::DirectAccess(DirectAccessEntity::Comment(ev.clone())),
                    Origin::DirectAccess(DirectAccessEntity::CommentReply(ev.clone())),
                ] {
                    let me = self.clone();
                    ctx.subscribe_event(origin, move |_event: &Event| me.refresh());
                }
            }
            // A comment's *owning item* is part of its row, so a binder-item change
            // (rename, trash) can change what this list should show even though no
            // comment was touched.
            for ev in [EntityEvent::Updated, EntityEvent::Removed] {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::BinderItem(ev)),
                    move |_event: &Event| me.refresh(),
                );
            }
            for wev in [WorkManagementEvent::LoadWork, WorkManagementEvent::NewWork] {
                let me = self.clone();
                ctx.subscribe_event(Origin::WorkManagement(wev), move |event: &Event| {
                    if !me.inner.ids.is_bootstrap_or_own(&event.ids) {
                        return;
                    }
                    // Prefer the seeded id; fall back to the one the event carries.
                    // See `refresh_for`: this model subscribes before the lifecycle
                    // seed runs, so the signal is still `None` here.
                    let work_id = me
                        .inner
                        .ids
                        .work_id
                        .get()
                        .or_else(|| event.ids.first().copied());
                    me.refresh_for(work_id);
                });
            }
            {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::WorkManagement(WorkManagementEvent::CloseWork),
                    move |event: &Event| {
                        if me.inner.ids.is_event_for_my_work(&event.ids) {
                            me.refresh_for(None)
                        }
                    },
                );
            }
            // Catch up when this window is already seeded — a rebuild, or a wire
            // that ran after the project was open.
            self.refresh();
        }

        /// The reactive handle both docks bind (the per-document dock through a
        /// `SortFilterListModel` narrowed to its focused item).
        pub fn list_model(&self) -> ListModel<CommentRow> {
            self.inner.model.clone()
        }

        /// Bumped on each refresh, for consumers that observe rather than bind
        /// (empty states, the aggregation rollup).
        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        /// Bumped only when the comment set's shape changes, never on a body edit.
        /// What the margin binds — see [`structure_key`](super::structure_key).
        pub fn structure_signal(&self) -> Signal<u64> {
            self.inner.structure.clone()
        }

        pub fn rows(&self) -> Vec<CommentRow> {
            snapshot(&self.inner.model)
        }

        pub fn len(&self) -> usize {
            self.inner.model.len()
        }

        pub fn is_empty(&self) -> bool {
            self.inner.model.len() == 0
        }

        /// Every comment on `item_id`, across all of that item's `Content` rows —
        /// a `BinderItem` owns up to three (body / synopsis / title), and a comment
        /// on the synopsis must not vanish because the writer is looking at the body.
        pub fn rows_for_item(&self, item_id: u64) -> Vec<CommentRow> {
            self.rows()
                .into_iter()
                .filter(|r| r.item_id == Some(item_id))
                .collect()
        }

        /// Comments anchored to one specific `Content` — what the editor's highlight
        /// session paints.
        pub fn rows_for_content(&self, content_id: u64) -> Vec<CommentRow> {
            self.rows()
                .into_iter()
                .filter(|r| r.content_id == Some(content_id))
                .collect()
        }

        /// Orphans, in one place: the docks' "no home" bucket and its always-visible
        /// count chip both read this.
        pub fn orphans(&self) -> Vec<CommentRow> {
            self.rows().into_iter().filter(|r| r.orphaned).collect()
        }

        /// Create a thread anchored to `content_id`, and wire it onto the Work.
        ///
        /// Returns the new id, or `None` if no project is open. Writes go through
        /// the model (not a view-model) for the same reason `DictWordListModel`'s
        /// do: it is the one place already gated by the real/mock seam, so no
        /// consumer needs a `#[cfg]`.
        pub fn create(
            &self,
            content_id: u64,
            kind: CommentAnchorKind,
            signature: &Signature,
            body: &str,
            anchor: &crate::comments::anchor::Anchor,
            stack_id: Option<u64>,
        ) -> Option<u64> {
            let work_id = self.inner.ids.work_id.get()?;
            let now = chrono::Utc::now();
            let created = comment_commands::create_orphan_comment(
                &self.inner.ctx,
                stack_id,
                &CreateCommentDto {
                    // Minted here, not left nil. `uid` is what an editorial round trip
                    // matches a returning comment on: exported into the `.docx`/`.odt`
                    // and read back to recognise *this* remark rather than create a
                    // second copy of it. A nil uid makes every round trip duplicate
                    // every comment, and the failure only shows up on the second one.
                    uid: common::uid::new_uid(),
                    created_at: now,
                    updated_at: now,
                    content: Some(content_id),
                    kind,
                    author_name: signature.name.clone(),
                    // Stored once, from the signature resolved at this instant, so a
                    // comment we export carries a margin label in Word instead of an
                    // anonymous one. Never re-derived on read or on export: an editor's
                    // own initials arrive inside the file they send back and are theirs
                    // — see `crate::comments::signature`.
                    author_initials: signature.initials.clone(),
                    body: body.to_string(),
                    resolved: false,
                    orphaned: false,
                    orphan_reason: CommentOrphanReason::NotOrphaned,
                    range_start: anchor.start as u64,
                    range_length: anchor.length as u64,
                    quote_prefix: anchor.prefix.clone(),
                    quote_exact: anchor.exact.clone(),
                    quote_exact_truncated: anchor.exact_truncated,
                    quote_suffix: anchor.suffix.clone(),
                    block_ordinal_hint: anchor.block_ordinal as u64,
                    replies: Vec::new(),
                },
            )
            .map_err(|e| eprintln!("comments: create failed: {e}"))
            .ok()?;

            // `create_orphan_*` leaves the row unparented; both links are wired
            // explicitly, exactly as the load materialiser does.
            if let Err(e) = comment_commands::set_comment_relationship(
                &self.inner.ctx,
                stack_id,
                &CommentRelationshipDto {
                    id: created.id,
                    field: CommentRelationshipField::Content,
                    right_ids: vec![content_id],
                },
            ) {
                eprintln!("comments: anchor wiring failed: {e}");
            }
            let mut ids = work_comment_ids(&self.inner.ctx, work_id);
            ids.push(created.id);
            if let Err(e) = work_commands::set_work_relationship(
                &self.inner.ctx,
                stack_id,
                &WorkRelationshipDto {
                    id: work_id,
                    field: WorkRelationshipField::Comments,
                    right_ids: ids,
                },
            ) {
                eprintln!("comments: work wiring failed: {e}");
            }
            // Re-read now rather than waiting for the backend event to come back
            // round. The caller's very next act is to re-anchor and paint, and the
            // event has not landed yet — without this the new thread is invisible
            // until something else forces a refresh, which is why "Add comment"
            // appeared to need pressing twice.
            self.refresh();
            Some(created.id)
        }

        /// Append a reply to `comment_id`.
        pub fn reply(
            &self,
            comment_id: u64,
            signature: &Signature,
            body: &str,
            stack_id: Option<u64>,
        ) -> Option<u64> {
            let now = chrono::Utc::now();
            let created = comment_reply_commands::create_orphan_comment_reply(
                &self.inner.ctx,
                stack_id,
                &CreateCommentReplyDto {
                    // A reply needs its own identity for the same reason its thread does,
                    // one level down: matching replies by position would re-import every
                    // reply after one the editor inserted mid-conversation.
                    uid: common::uid::new_uid(),
                    created_at: now,
                    updated_at: now,
                    // A reply carries its OWN signature, not the thread's — it is its own
                    // `<w:comment>` on the way out, and a conversation between two people
                    // that came back signed by one of them would be a lie about who said
                    // what.
                    author_name: signature.name.clone(),
                    author_initials: signature.initials.clone(),
                    body: body.to_string(),
                },
            )
            .map_err(|e| eprintln!("comments: reply failed: {e}"))
            .ok()?;

            let mut ids = comment_commands::get_comment_relationship(
                &self.inner.ctx,
                &comment_id,
                &CommentRelationshipField::Replies,
            )
            .unwrap_or_default();
            ids.push(created.id);
            if let Err(e) = comment_commands::set_comment_relationship(
                &self.inner.ctx,
                stack_id,
                &CommentRelationshipDto {
                    id: comment_id,
                    field: CommentRelationshipField::Replies,
                    right_ids: ids,
                },
            ) {
                eprintln!("comments: reply wiring failed: {e}");
            }
            // Same reason `create` refreshes: the relationship write does not
            // emit a `Comment` event, so without this the new turn would not
            // appear until something unrelated forced a reload.
            self.refresh();
            Some(created.id)
        }

        /// Replace one reply's body — the in-place edit the card's own editor does.
        pub fn set_reply_body(&self, reply_id: u64, body: &str, stack_id: Option<u64>) {
            let Ok(Some(cur)) =
                comment_reply_commands::get_comment_reply(&self.inner.ctx, &reply_id)
            else {
                return;
            };
            if let Err(e) = comment_reply_commands::update_comment_reply(
                &self.inner.ctx,
                stack_id,
                &UpdateCommentReplyDto {
                    id: cur.id,
                    // Carried through unchanged, exactly as `update_with` does for a
                    // comment: a nil here would overwrite the row's durable identity on
                    // every keystroke-committed edit.
                    uid: cur.uid,
                    created_at: cur.created_at,
                    updated_at: chrono::Utc::now(),
                    author_name: cur.author_name,
                    author_initials: cur.author_initials,
                    body: body.to_string(),
                },
            ) {
                eprintln!("comments: reply edit failed: {e}");
            }
        }

        /// Delete one reply, leaving the rest of the thread standing.
        ///
        /// The junction is reconciled by the generated remove — the same cascade
        /// that takes every reply down with its comment — so this does not need to
        /// rewrite the parent's `Replies` list, and must not: doing both would race
        /// the reconcile and could drop a sibling.
        pub fn delete_reply(&self, reply_id: u64, stack_id: Option<u64>) {
            if let Err(e) =
                comment_reply_commands::remove_comment_reply(&self.inner.ctx, stack_id, &reply_id)
            {
                eprintln!("comments: reply delete failed: {e}");
            }
            self.refresh();
        }

        /// Set (or clear) a thread's resolved flag.
        pub fn set_resolved(&self, comment_id: u64, resolved: bool, stack_id: Option<u64>) {
            self.update_with(comment_id, stack_id, |dto| dto.resolved = resolved);
        }

        /// Replace a thread's body text.
        pub fn set_body(&self, comment_id: u64, body: &str, stack_id: Option<u64>) {
            self.update_with(comment_id, stack_id, |dto| dto.body = body.to_string());
        }

        /// Write back a re-anchored position, and the orphan verdict that came with
        /// it. Called by the re-anchor pass at open, and by live tracking at flush.
        pub fn set_anchor(
            &self,
            comment_id: u64,
            resolution: &crate::comments::anchor::Resolution,
            stack_id: Option<u64>,
        ) {
            use crate::comments::anchor::Resolution;
            self.update_with(comment_id, stack_id, |dto| match resolution {
                Resolution::Anchored { start, length } => {
                    dto.range_start = *start as u64;
                    dto.range_length = *length as u64;
                    dto.orphaned = false;
                    dto.orphan_reason = CommentOrphanReason::NotOrphaned;
                }
                Resolution::Orphan(reason) => {
                    dto.orphaned = true;
                    dto.orphan_reason = reason.clone();
                }
            });
        }

        /// Delete a thread. Its replies are strong children and cascade with it;
        /// the Work's list is reconciled by the generated remove.
        pub fn delete(&self, comment_id: u64, stack_id: Option<u64>) {
            if let Err(e) = comment_commands::remove_comment(&self.inner.ctx, stack_id, &comment_id)
            {
                eprintln!("comments: delete failed: {e}");
            }
        }

        /// Read-modify-write one comment through the generated update command.
        /// `UpdateCommentDto` carries no relationship fields, so this can never
        /// re-point an anchor as a side effect of, say, resolving a thread.
        fn update_with(
            &self,
            comment_id: u64,
            stack_id: Option<u64>,
            edit: impl FnOnce(&mut UpdateCommentDto),
        ) {
            let Ok(Some(cur)) = comment_commands::get_comment(&self.inner.ctx, &comment_id) else {
                return;
            };
            let mut dto = UpdateCommentDto {
                // Carried through unchanged: a nil here would write over the row's durable
                // identity on every edit, orphaning anything that references it.
                uid: cur.uid,
                id: cur.id,
                created_at: cur.created_at,
                updated_at: chrono::Utc::now(),
                kind: cur.kind,
                author_name: cur.author_name,
                // Carried through for the same reason `uid` is: an edit to a thread's
                // resolved state must not silently blank the label its author chose.
                author_initials: cur.author_initials,
                body: cur.body,
                resolved: cur.resolved,
                orphaned: cur.orphaned,
                orphan_reason: cur.orphan_reason,
                range_start: cur.range_start,
                range_length: cur.range_length,
                quote_prefix: cur.quote_prefix,
                quote_exact: cur.quote_exact,
                quote_exact_truncated: cur.quote_exact_truncated,
                quote_suffix: cur.quote_suffix,
                block_ordinal_hint: cur.block_ordinal_hint,
            };
            edit(&mut dto);
            if let Err(e) = comment_commands::update_comment(&self.inner.ctx, stack_id, &dto) {
                eprintln!("comments: update failed: {e}");
            }
        }

        fn refresh(&self) {
            self.refresh_for(self.inner.ids.work_id.get());
        }

        /// Reload against an explicit Work, rather than whatever `ids.work_id` says
        /// right now.
        ///
        /// The distinction is load-bearing on `LoadWork`: this model is wired in
        /// `App::build` *before* the lifecycle seed that writes `ids.work_id`, so a
        /// handler that read the signal would read `None`, load nothing, and leave
        /// both docks empty until the next `Comment` entity event — which for a
        /// project the writer merely *opened* never comes. Exactly the bug
        /// `WorkTagsListModel::wire` records having had, fixed the same way.
        pub(crate) fn refresh_for(&self, work_id: Option<u64>) {
            let rows = work_id
                .map(|id| load_rows(&self.inner.ctx, id))
                .unwrap_or_default();
            let key = super::structure_key(&rows);
            // Conditional, and that is what lets `wire` end in a catch-up without
            // re-subscribing being a hazard: the catch-up runs inside `build()`, and a
            // bump on an unchanged re-read is a write a widget's own build made, which
            // dirties it, which rebuilds, which builds, which bumps. Measured on the
            // identical bug in `WorkStatusesListModel`: 205 rebuilds of an idle window
            // in six seconds. It also means this signal says "the rows changed" rather
            // than "somebody re-read them", which is what every consumer wanted anyway.
            let changed = snapshot(&self.inner.model) != rows;
            self.inner.model.reconcile_by_key(rows, |r| r.id);
            if changed {
                let v = &self.inner.version;
                v.set(v.get().wrapping_add(1));
            }
            if self.inner.last_structure.replace(key) != key {
                let s = &self.inner.structure;
                s.set(s.get().wrapping_add(1));
            }
        }
    }

    /// Content id -> (owning item id, item title), built by walking this Work's
    /// binder tree once.
    ///
    /// The relationship only runs `BinderItem -> Content`, so the inverse has to be
    /// materialised. Doing it once per refresh (rather than per comment) keeps this
    /// linear in the tree rather than quadratic.
    fn content_owner_map(ctx: &AppContext, work_id: u64) -> HashMap<u64, (u64, String)> {
        let mut out = HashMap::new();
        let binder_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default();
        for binder_id in binder_ids {
            let item_ids = binder_commands::get_binder_relationship(
                ctx,
                &binder_id,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            let titles: HashMap<u64, String> =
                binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                    .map(|it| (it.id, it.title))
                    .collect();
            for item_id in item_ids {
                let contents = binder_item_commands::get_binder_item_relationship(
                    ctx,
                    &item_id,
                    &BinderItemRelationshipField::Contents,
                )
                .unwrap_or_default();
                let title = titles.get(&item_id).cloned().unwrap_or_default();
                for cid in contents {
                    out.insert(cid, (item_id, title.clone()));
                }
            }
        }
        out
    }

    /// Read this window's own Work's comments — via `Work.comments`, never
    /// `get_all_comment`, which would merge a second simultaneously-open Work's
    /// threads into this one's docks.
    fn load_rows(ctx: &AppContext, work_id: u64) -> Vec<CommentRow> {
        let comment_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Comments)
                .unwrap_or_default();
        if comment_ids.is_empty() {
            return Vec::new();
        }
        let owners = content_owner_map(ctx, work_id);

        let mut rows: Vec<CommentRow> = Vec::with_capacity(comment_ids.len());
        for dto in comment_commands::get_comment_multi(ctx, &comment_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
        {
            // The weak `content` relationship is the authority, not the DTO field:
            // once the target is removed the junction is reconciled away, which is
            // precisely the signal that this comment has lost its anchor.
            let content_id = comment_commands::get_comment_relationship(
                ctx,
                &dto.id,
                &CommentRelationshipField::Content,
            )
            .unwrap_or_default()
            .into_iter()
            .next();

            let reply_ids = comment_commands::get_comment_relationship(
                ctx,
                &dto.id,
                &CommentRelationshipField::Replies,
            )
            .unwrap_or_default();
            // Read in one batch and re-ordered to the relationship's own order:
            // `get_comment_reply_multi` answers in id order, which is *usually*
            // creation order and would silently stop being it the moment a reply
            // is ever re-parented or restored.
            let fetched: HashMap<u64, super::ReplyRow> =
                comment_reply_commands::get_comment_reply_multi(ctx, &reply_ids)
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                    .map(|r| {
                        (
                            r.id,
                            super::ReplyRow {
                                id: r.id,
                                author_name: r.author_name,
                                body: r.body,
                                created_at: r.created_at,
                            },
                        )
                    })
                    .collect();
            let replies: Vec<super::ReplyRow> = reply_ids
                .iter()
                .filter_map(|rid| fetched.get(rid).cloned())
                .collect();

            let (item_id, item_title) = content_id
                .and_then(|cid| owners.get(&cid).cloned())
                .map(|(i, t)| (Some(i), t))
                .unwrap_or((None, String::new()));

            rows.push(CommentRow {
                id: dto.id,
                content_id,
                item_id,
                item_title,
                kind: dto.kind,
                author_name: dto.author_name,
                body: dto.body,
                resolved: dto.resolved,
                // An anchor whose content no longer resolves is orphaned regardless
                // of what the stored flag says — the store is the live truth here.
                orphaned: dto.orphaned || content_id.is_none(),
                orphan_reason: dto.orphan_reason,
                range_start: dto.range_start,
                range_length: dto.range_length,
                quote_prefix: dto.quote_prefix,
                quote_exact: dto.quote_exact,
                quote_exact_truncated: dto.quote_exact_truncated,
                quote_suffix: dto.quote_suffix,
                block_ordinal_hint: dto.block_ordinal_hint,
                replies,
                created_at: dto.created_at,
            });
        }
        sort_rows(&mut rows);
        rows
    }

    /// This Work's current comment id list — read before an append so wiring a new
    /// thread never drops the existing ones.
    fn work_comment_ids(ctx: &AppContext, work_id: u64) -> Vec<u64> {
        work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Comments)
            .unwrap_or_default()
    }

    fn snapshot(model: &ListModel<CommentRow>) -> Vec<CommentRow> {
        (0..model.len())
            .filter_map(|i| model.with_item(i, |r| r.clone()))
            .collect()
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::common::entities::{CommentAnchorKind, CommentOrphanReason};

    use crate::app_ids::AppIds;
    use crate::comments::signature::Signature;

    use super::{CommentRow, sort_rows};

    struct Inner {
        model: ListModel<CommentRow>,
        version: Signal<u64>,
        structure: Signal<u64>,
        last_structure: Cell<u64>,
        next_id: Cell<u64>,
    }

    #[derive(Clone)]
    pub struct CommentsListModel {
        inner: Rc<Inner>,
    }

    impl CommentsListModel {
        pub fn new(_ctx: Rc<AppContext>, _ids: AppIds) -> Self {
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::from_vec(fabricated()),
                    version: Signal::new(0),
                    structure: Signal::new(0),
                    last_structure: Cell::new(super::structure_key(&fabricated())),
                    next_id: Cell::new(100),
                }),
            }
        }

        /// Inert: a mock build has no backend to emit entity events.
        pub fn wire(&self, _ctx: &mut BuildContext) {}

        pub fn list_model(&self) -> ListModel<CommentRow> {
            self.inner.model.clone()
        }

        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        pub fn structure_signal(&self) -> Signal<u64> {
            self.inner.structure.clone()
        }

        pub fn rows(&self) -> Vec<CommentRow> {
            (0..self.inner.model.len())
                .filter_map(|i| self.inner.model.with_item(i, |r| r.clone()))
                .collect()
        }

        pub fn len(&self) -> usize {
            self.inner.model.len()
        }

        pub fn is_empty(&self) -> bool {
            self.inner.model.len() == 0
        }

        pub fn rows_for_item(&self, item_id: u64) -> Vec<CommentRow> {
            self.rows()
                .into_iter()
                .filter(|r| r.item_id == Some(item_id))
                .collect()
        }

        pub fn rows_for_content(&self, content_id: u64) -> Vec<CommentRow> {
            self.rows()
                .into_iter()
                .filter(|r| r.content_id == Some(content_id))
                .collect()
        }

        pub fn orphans(&self) -> Vec<CommentRow> {
            self.rows().into_iter().filter(|r| r.orphaned).collect()
        }

        // ── Writes. Same signatures as the real impl so no consumer needs a
        // `#[cfg]`; they mutate the fabricated list in place, since a mock build
        // has no backend to own a `Comment`.

        pub fn create(
            &self,
            content_id: u64,
            kind: CommentAnchorKind,
            signature: &Signature,
            body: &str,
            anchor: &crate::comments::anchor::Anchor,
            _stack_id: Option<u64>,
        ) -> Option<u64> {
            let id = self.inner.next_id.get();
            self.inner.next_id.set(id + 1);
            let mut rows = self.rows();
            rows.push(CommentRow {
                id,
                content_id: Some(content_id),
                item_id: Some(301),
                item_title: "The lamp".into(),
                kind,
                // Only the name: `CommentRow` carries no initials, because nothing in
                // the UI renders them — they exist for the `w:initials` a `.docx`
                // export writes, and a mock build has no export to feed.
                author_name: signature.name.clone(),
                body: body.to_string(),
                range_start: anchor.start as u64,
                range_length: anchor.length as u64,
                quote_prefix: anchor.prefix.clone(),
                quote_exact: anchor.exact.clone(),
                quote_exact_truncated: anchor.exact_truncated,
                quote_suffix: anchor.suffix.clone(),
                block_ordinal_hint: anchor.block_ordinal as u64,
                ..Default::default()
            });
            self.replace(rows);
            Some(id)
        }

        pub fn reply(
            &self,
            comment_id: u64,
            signature: &Signature,
            body: &str,
            _stack_id: Option<u64>,
        ) -> Option<u64> {
            let id = self.inner.next_id.get();
            self.inner.next_id.set(id + 1);
            let mut rows = self.rows();
            let row = rows.iter_mut().find(|r| r.id == comment_id)?;
            row.replies.push(super::ReplyRow {
                id,
                author_name: signature.name.clone(),
                body: body.to_string(),
                created_at: chrono::Utc::now(),
            });
            self.replace(rows);
            Some(id)
        }

        pub fn set_reply_body(&self, reply_id: u64, body: &str, _stack_id: Option<u64>) {
            let mut rows = self.rows();
            for row in rows.iter_mut() {
                if let Some(r) = row.replies.iter_mut().find(|r| r.id == reply_id) {
                    r.body = body.to_string();
                }
            }
            self.replace(rows);
        }

        pub fn delete_reply(&self, reply_id: u64, _stack_id: Option<u64>) {
            let mut rows = self.rows();
            for row in rows.iter_mut() {
                row.replies.retain(|r| r.id != reply_id);
            }
            self.replace(rows);
        }

        pub fn set_resolved(&self, comment_id: u64, resolved: bool, _stack_id: Option<u64>) {
            let mut rows = self.rows();
            if let Some(r) = rows.iter_mut().find(|r| r.id == comment_id) {
                r.resolved = resolved;
            }
            self.replace(rows);
        }

        pub fn set_body(&self, comment_id: u64, body: &str, _stack_id: Option<u64>) {
            let mut rows = self.rows();
            if let Some(r) = rows.iter_mut().find(|r| r.id == comment_id) {
                r.body = body.to_string();
            }
            self.replace(rows);
        }

        pub fn set_anchor(
            &self,
            comment_id: u64,
            resolution: &crate::comments::anchor::Resolution,
            _stack_id: Option<u64>,
        ) {
            use crate::comments::anchor::Resolution;
            let mut rows = self.rows();
            if let Some(r) = rows.iter_mut().find(|r| r.id == comment_id) {
                match resolution {
                    Resolution::Anchored { start, length } => {
                        r.range_start = *start as u64;
                        r.range_length = *length as u64;
                        r.orphaned = false;
                        r.orphan_reason = CommentOrphanReason::NotOrphaned;
                    }
                    Resolution::Orphan(reason) => {
                        r.orphaned = true;
                        r.orphan_reason = reason.clone();
                    }
                }
            }
            self.replace(rows);
        }

        pub fn delete(&self, comment_id: u64, _stack_id: Option<u64>) {
            let rows: Vec<CommentRow> = self
                .rows()
                .into_iter()
                .filter(|r| r.id != comment_id)
                .collect();
            self.replace(rows);
        }

        fn replace(&self, mut rows: Vec<CommentRow>) {
            sort_rows(&mut rows);
            let key = super::structure_key(&rows);
            self.inner.model.reconcile_by_key(rows, |r| r.id);
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
            if self.inner.last_structure.replace(key) != key {
                let s = &self.inner.structure;
                s.set(s.get().wrapping_add(1));
            }
        }
    }

    /// The binder item the fabricated comments hang off — "Scene 1", the mock
    /// binder tree's first scene under Chapter Two.
    ///
    /// It has to be a **real row of the mock binder** whose prose is a **real
    /// fabricated document**, or none of this is visible: an invented item id puts
    /// every thread in the docks' "no home" bucket, and an invented content id
    /// leaves the editor margin permanently empty. The first version of this
    /// fixture did both, and the whole margin was dead in the mocks build.
    const MOCK_ITEM: u64 = 201;
    const MOCK_ITEM_TITLE: &str = "Scene 1";

    /// The `Content` row the fabricated comments anchor to: `MOCK_ITEM`'s prose,
    /// derived through the very function the mock `SingleContent` uses, so the two
    /// cannot drift apart.
    fn mock_scene_content() -> u64 {
        crate::singles::mock_content_id(
            MOCK_ITEM,
            &frontend::common::entities::ContentRole::SceneText,
        )
    }

    /// A fabricated set covering every shape a dock has to render: an open range
    /// comment with a thread, a resolved paragraph comment, an orphan, and an
    /// *unplaced* comment — so the mock build exercises the "no home" bucket, the
    /// resolved styling, and `is_unplaced`'s own badge too, rather than only the
    /// happy row.
    ///
    /// The two anchored ones are positioned against the real fabricated prose of
    /// `MOCK_ITEM` ("This is the fabricated body of scene 201. The morning light
    /// crept over the ridgeline…"): a range over *the morning light* in the first
    /// paragraph, and a paragraph comment on the second.
    fn fabricated() -> Vec<CommentRow> {
        let now = chrono::DateTime::UNIX_EPOCH;
        let content = mock_scene_content();
        let mut rows = vec![
            CommentRow {
                id: 1,
                content_id: Some(content),
                item_id: Some(MOCK_ITEM),
                item_title: MOCK_ITEM_TITLE.into(),
                kind: CommentAnchorKind::Range,
                author_name: "Jane".into(),
                // Real Djot markup, deliberately — a body is Djot now (M-S4), and
                // a mocks fixture with none would let a card, a dock preview line
                // or the AccessKit summary silently regress to showing an
                // editor's `*emphasis*` as literal asterisks without the visual
                // QA path (`--features mocks`, no backend needed) ever catching
                // it.
                body: "Is this *too* on-the-nose?".into(),
                replies: vec![
                    super::ReplyRow {
                        id: 11,
                        author_name: "Marc".into(),
                        body: "A little. But it is the _title image_.".into(),
                        created_at: now,
                    },
                    super::ReplyRow {
                        id: 12,
                        author_name: "Jane".into(),
                        body: "Keep it — it lands.".into(),
                        created_at: now,
                    },
                ],
                // "The morning light" — offsets into the fabricated prose above.
                range_start: 42,
                range_length: 17,
                quote_prefix: "ated body of scene 201. ".into(),
                quote_exact: "The morning light".into(),
                quote_suffix: " crept over the ridgelin".into(),
                created_at: now,
                ..Default::default()
            },
            CommentRow {
                id: 2,
                content_id: Some(content),
                item_id: Some(MOCK_ITEM),
                item_title: MOCK_ITEM_TITLE.into(),
                kind: CommentAnchorKind::Paragraph,
                author_name: "Jane".into(),
                body: "This whole paragraph drags.".into(),
                // The second paragraph, whole. Left **open** so the margin has one
                // of each mark to draw — a range triangle and a paragraph bracket.
                // A fixture where every anchored thread was resolved would render
                // an empty margin and prove nothing.
                block_ordinal_hint: 1,
                range_start: 178,
                range_length: 86,
                quote_exact: "A second paragraph follows, so the manuscript streams show real \
flowing prose per row."
                    .into(),
                created_at: now,
                ..Default::default()
            },
            CommentRow {
                // A settled thread: still listed in the docks, deliberately absent
                // from the margin. Resolved threads are what the "Resolved" filter
                // chip is for, and a fixture without one would leave both the chip
                // and the margin's own exclusion untested by eye.
                id: 4,
                content_id: Some(content),
                item_id: Some(MOCK_ITEM),
                item_title: MOCK_ITEM_TITLE.into(),
                kind: CommentAnchorKind::Range,
                author_name: "Marc".into(),
                body: "Fixed — cut the adverb.".into(),
                resolved: true,
                range_start: 60,
                range_length: 5,
                quote_prefix: "morning light ".into(),
                quote_exact: "crept".into(),
                quote_suffix: " over".into(),
                created_at: now,
                ..Default::default()
            },
            CommentRow {
                id: 3,
                content_id: None,
                item_id: None,
                kind: CommentAnchorKind::Range,
                author_name: "Marc".into(),
                body: "Whatever this was about, it is gone now.".into(),
                orphaned: true,
                orphan_reason: CommentOrphanReason::TargetDeleted,
                quote_exact: "vanished".into(),
                created_at: now,
                ..Default::default()
            },
            CommentRow {
                // `CommentAnchorKind::Document`'s only surviving shape: a comment
                // the import pipeline could not point at any prose (a heading, in
                // the pre-fix importer; a blank paragraph or a table today — see
                // `document_ingest::plan`'s module doc). Its `Content` is alive
                // and it is *not* orphaned — `range_length: 0` is the only tell —
                // so without `is_unplaced` this row would render a blank quote
                // and fabricate a seek to (0, 0) on click.
                id: 5,
                content_id: Some(content),
                item_id: Some(MOCK_ITEM),
                item_title: MOCK_ITEM_TITLE.into(),
                kind: CommentAnchorKind::Document,
                author_name: "Editor".into(),
                body: "Left as a note on the whole scene by whoever wrote it \
in Word."
                    .into(),
                range_start: 0,
                range_length: 0,
                created_at: now,
                ..Default::default()
            },
        ];
        sort_rows(&mut rows);
        rows
    }
}

pub use imp::CommentsListModel;

#[cfg(test)]
mod tests {
    use super::*;

    fn row(id: u64, item: Option<u64>, block: u64, start: u64, orphaned: bool) -> CommentRow {
        CommentRow {
            id,
            item_id: item,
            content_id: if orphaned { None } else { Some(1) },
            block_ordinal_hint: block,
            range_start: start,
            // A real (non-zero) length, matching every ordinary quote. This
            // helper is about position and orphan state, not `is_unplaced`'s own
            // zero-length case — the tests that need that build a `CommentRow`
            // by hand instead (see `a_resolved_but_empty_anchor_is_unplaced`).
            range_length: if orphaned { 0 } else { 4 },
            orphaned,
            ..Default::default()
        }
    }

    #[test]
    fn rows_sort_by_document_position_within_an_item() {
        let mut rows = vec![
            row(1, Some(10), 5, 100, false),
            row(2, Some(10), 1, 900, false),
            row(3, Some(10), 1, 20, false),
        ];
        sort_rows(&mut rows);
        assert_eq!(
            rows.iter().map(|r| r.id).collect::<Vec<_>>(),
            vec![3, 2, 1],
            "earlier block first, then earlier offset within the block"
        );
    }

    #[test]
    fn orphans_sort_last_rather_than_pretending_to_be_at_offset_zero() {
        let mut rows = vec![row(1, None, 0, 0, true), row(2, Some(10), 9, 900, false)];
        sort_rows(&mut rows);
        assert_eq!(
            rows.iter().map(|r| r.id).collect::<Vec<_>>(),
            vec![2, 1],
            "an anchorless comment has no position and must not lead the list"
        );
    }

    #[test]
    fn an_orphan_is_never_reported_as_anchored() {
        let o = row(1, None, 0, 0, true);
        assert!(!o.is_anchored());
        assert!(row(2, Some(3), 0, 0, false).is_anchored());
    }

    /// `CommentAnchorKind::Document`'s only surviving shape: a comment whose
    /// anchor *resolved* — its `Content` is alive, it is not `orphaned` — but
    /// resolved to a zero-length range. Before `is_anchored` checked
    /// `range_length`, this exact shape reported itself anchored and a click
    /// against it seeked to a fabricated `(0, 0)`.
    #[test]
    fn a_resolved_but_empty_anchor_is_unplaced_not_anchored() {
        let r = CommentRow {
            content_id: Some(7),
            orphaned: false,
            range_start: 0,
            range_length: 0,
            ..Default::default()
        };
        assert!(
            r.is_unplaced(),
            "a live Content with an empty range is exactly what is_unplaced means"
        );
        assert!(
            !r.is_anchored(),
            "an empty range must never be treated as a live position"
        );
    }

    /// Orphaned takes priority over unplaced: a comment with no home at all is
    /// described by `orphaned`, not by `is_unplaced` — the two states are
    /// mutually exclusive even though neither has anywhere to seek.
    #[test]
    fn an_orphan_is_never_reported_as_unplaced_either() {
        let o = row(1, None, 0, 0, true);
        assert!(!o.is_unplaced());
    }

    /// A comment with a real, positive-length range is neither unplaced nor
    /// orphaned — the ordinary case `is_unplaced` must not misclassify.
    #[test]
    fn an_ordinary_anchored_comment_is_not_unplaced() {
        assert!(!row(2, Some(3), 0, 0, false).is_unplaced());
    }

    #[test]
    fn open_is_the_inverse_of_resolved() {
        let mut r = row(1, Some(3), 0, 0, false);
        assert!(r.is_open());
        r.resolved = true;
        assert!(!r.is_open());
    }
}

/// Against the real backend, because the bug this pins is entirely about *when*
/// `AppIds::work_id` is written relative to when this model reads it.
#[cfg(all(test, not(feature = "mocks")))]
mod real_backend_tests {
    use std::rc::Rc;

    use frontend::AppContext;
    use frontend::commands::{
        binder_item_commands, comment_commands, comment_reply_commands, content_commands,
        work_commands, work_management_commands,
    };
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::entities::CommentAnchorKind;
    use frontend::work_management::LoadWorkDto;

    use crate::app_ids::AppIds;

    use super::CommentsListModel;

    /// Load the shared fixture and return `(unseeded ids, work id)`.
    fn loaded(app_ctx: &Rc<AppContext>) -> (AppIds, u64) {
        work_management_commands::load_work(
            app_ctx,
            &LoadWorkDto {
                media_root: crate::media_paths::media_root_string(),
                file_name: format!(
                    "{}/../../resources/test/skribisto_test_project.skrib",
                    env!("CARGO_MANIFEST_DIR")
                ),
            },
        )
        .expect("load fixture");
        let work_id = work_commands::get_all_work(app_ctx)
            .expect("work")
            .first()
            .expect("one work")
            .id;
        (AppIds::new(), work_id)
    }

    /// The lowest activated `Content` in the fixture — a stable pick over a
    /// `HashMap` store, and never a trashed row.
    fn a_content(app_ctx: &Rc<AppContext>) -> u64 {
        let mut ids: Vec<u64> = binder_item_commands::get_all_binder_item(app_ctx)
            .expect("items")
            .into_iter()
            .filter(|it| it.activated)
            .flat_map(|it| {
                binder_item_commands::get_binder_item_relationship(
                    app_ctx,
                    &it.id,
                    &BinderItemRelationshipField::Contents,
                )
                .unwrap_or_default()
            })
            .filter(|cid| {
                content_commands::get_content(app_ctx, cid)
                    .ok()
                    .flatten()
                    .is_some_and(|c| c.activated)
            })
            .collect();
        ids.sort_unstable();
        ids.into_iter().next().expect("a live content row")
    }

    /// **The rebuild loop, as a unit test.** `wire` ends in a catch-up `refresh_for`
    /// that runs inside `build()`, and both docks bind this version: a bump on a re-read
    /// that changed nothing is a write a widget's own build made, which dirties it, which
    /// rebuilds, which builds, which bumps, at frame rate. Measured on the identical bug
    /// in `WorkStatusesListModel`: 205 rebuilds of an idle window in six seconds.
    ///
    /// This is what makes dropping the one-shot `subscribed` guard safe, and re-wiring on
    /// every build is what the framework asks for: a `BuildContext` subscription dies with
    /// its build.
    #[test]
    fn re_reading_unchanged_comments_does_not_move_the_version() {
        let app_ctx = Rc::new(AppContext::new());
        let (ids, work_id) = loaded(&app_ctx);
        ids.seed(&app_ctx, work_id);
        let model = CommentsListModel::new(app_ctx.clone(), ids.clone());
        model.refresh_for(Some(work_id));
        let settled = model.version_signal().get();
        let rows = model.len();

        for _ in 0..20 {
            model.refresh_for(Some(work_id));
        }

        assert_eq!(
            model.version_signal().get(),
            settled,
            "a re-read that changes nothing must not ask anyone to repaint"
        );
        assert_eq!(model.len(), rows, "and it must not lose the rows either");
    }

    /// **Regression.** A project opened with comments already in it showed two
    /// empty docks.
    ///
    /// `App::build` wires this model well before the lifecycle subscriber that
    /// seeds `AppIds::work_id`, so both subscribe to the same `LoadWork` and this
    /// one runs first — reading `work_id` as `None`, loading nothing, and never
    /// being asked again, because a project the writer merely *opened* produces no
    /// further `Comment` event. The comments were in the store the whole time.
    ///
    /// The fix is the one `WorkTagsListModel::wire` already records making for the
    /// identical bug: take the id from the event when the signal has not caught up.
    /// This pins the capability that makes that possible — loading a Work this
    /// model's own ids do not yet know about.
    ///
    /// The event itself cannot be driven here: `test_support`'s event source is
    /// real but never fires (nothing mutates the store off-thread in a headless
    /// test), which is also why no unit test caught this in the first place.
    #[test]
    fn a_dock_whose_ids_are_not_seeded_yet_can_still_load_the_work_the_event_names() {
        let app_ctx = Rc::new(AppContext::new());
        let (ids, work_id) = loaded(&app_ctx);

        // A comment in the store, exactly as a saved project's would arrive.
        let seeded = AppIds::new();
        seeded.seed(&app_ctx, work_id);
        let writer = CommentsListModel::new(app_ctx.clone(), seeded);
        writer
            .create(
                a_content(&app_ctx),
                CommentAnchorKind::Range,
                &crate::comments::signature::resolve("Editor", "", ""),
                "Is this the right word?",
                &crate::comments::anchor::Anchor {
                    start: 0,
                    length: 3,
                    exact: "The".into(),
                    block_span: 1,
                    ..Default::default()
                },
                None,
            )
            .expect("the comment is created");

        // The startup shape: this model's ids have not been seeded.
        let model = CommentsListModel::new(app_ctx.clone(), ids.clone());
        assert!(ids.work_id.get().is_none());
        assert!(
            model.is_empty(),
            "nothing is knowable before the ids are seeded — this is the state the \
             dock used to be stuck in forever"
        );

        // What the `LoadWork` handler now does: use the id the event carried.
        model.refresh_for(Some(work_id));
        assert_eq!(
            model.rows().len(),
            1,
            "the dock must fill from the event's own work id"
        );
        assert_eq!(model.rows()[0].body, "Is this the right word?");

        // And closing empties it again rather than leaving a stale project's notes.
        model.refresh_for(None);
        assert!(model.is_empty());
    }

    /// **Regression.** Every comment this app has ever written reached disk with
    /// `author_name: ""`, in projects whose author name was set years earlier.
    ///
    /// The signature was seeded by a one-shot `Signal::get` during `App::build`,
    /// which registers no dependency and ran before `load_work` had populated
    /// `SingleWork` — so the captured name was the empty string for the life of
    /// the window. This pins the half that can be tested headlessly: the name and
    /// the initials handed to `create` are what the stored row ends up carrying,
    /// rather than being recomputed, dropped, or read from somewhere else.
    ///
    /// The other half — that `App::build`'s effect keeps that value current — has
    /// no headless test: it needs a real window built before a real `load_work`,
    /// which is the exact shape of race a `WidgetTree` cannot reproduce.
    #[test]
    fn a_created_comment_stores_the_signature_it_was_handed() {
        let app_ctx = Rc::new(AppContext::new());
        let (_ids, work_id) = loaded(&app_ctx);
        let seeded = AppIds::new();
        seeded.seed(&app_ctx, work_id);
        let model = CommentsListModel::new(app_ctx.clone(), seeded);

        // Explicit initials that the derivation would NOT produce, so a row
        // carrying "MJO" would prove something re-derived them behind our back.
        let signature = crate::comments::signature::resolve("Mary-Jane O'Brien", "MO", "");
        let id = model
            .create(
                a_content(&app_ctx),
                CommentAnchorKind::Range,
                &signature,
                "Cut this?",
                &crate::comments::anchor::Anchor {
                    start: 0,
                    length: 3,
                    exact: "The".into(),
                    block_span: 1,
                    ..Default::default()
                },
                None,
            )
            .expect("the comment is created");

        let stored = comment_commands::get_comment(&app_ctx, &id)
            .expect("read comment")
            .expect("comment exists");
        assert_eq!(stored.author_name, "Mary-Jane O'Brien");
        assert_eq!(
            stored.author_initials, "MO",
            "the typed initials must be stored verbatim, not re-derived to MJO"
        );

        // A reply is signed in its own right — it is its own `<w:comment>` on the
        // way out, so it must carry the signature rather than inherit the thread's.
        let reply_signature = crate::comments::signature::resolve("Rae Okafor", "", "");
        let reply_id = model
            .reply(id, &reply_signature, "Keep it.", None)
            .expect("the reply is created");
        let stored_reply = comment_reply_commands::get_comment_reply(&app_ctx, &reply_id)
            .expect("read reply")
            .expect("reply exists");
        assert_eq!(stored_reply.author_name, "Rae Okafor");
        assert_eq!(stored_reply.author_initials, "RO");
    }
}
