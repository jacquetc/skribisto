// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `CommentsViewModel` — the comment feature's business logic: create a thread
//! from a selection or a caret, reply, resolve/reopen, delete, relink an orphan,
//! and keep every open document's live anchors correct as the writer types.
//!
//! Single-instance live state, created once in `App::build` and shared by
//! `.clone()`. It owns the [`CommentsListModel`] both docks bind and the filter
//! state their header chips drive; it holds **no peer view-model** (a dock never
//! reaches `EditorsViewModel` directly — `App` mediates, per the DAG rule).
//!
//! ## The two halves of an anchor
//!
//! * The **persisted** half is a quote selector, written once at creation and
//!   deliberately never rewritten while typing — it is the ground truth the next
//!   load's re-anchor pass depends on, so an edit must not be able to corrupt it.
//! * The **live** half is a `(start, end)` pair held by the document's
//!   [`CommentHighlightSession`] and folded forward on every `ContentsChanged`
//!   with exclusive edges.
//!
//! They meet twice: at **open**, when [`reanchor`] re-derives live offsets from
//! the quote; and at **flush**, when [`persist_live_anchors`] writes the tracked
//! offsets back so the next session starts from a good hint.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use std::time::Duration;

use bastyde::prelude::*;
use bastyde::text_document::TextDocument;
use bastyde::widgets::{Toast, ToastAction};

use crate::toast_scope::ToastWorkExt;

use frontend::AppContext;
use frontend::commands::undo_redo_commands;

/// How long the "deleted — Undo" snackbar stays up.
const UNDO_GRACE: Duration = Duration::from_secs(6);

use frontend::common::entities::{CommentAnchorKind, CommentOrphanReason};

use crate::comments::anchor::{self, Anchor, Resolution};
use crate::comments::session::LiveAnchor;
use crate::models::{CommentRow, CommentsListModel};

/// Which threads a dock shows. Always-visible chips, including `Orphaned` at zero
/// — a filter that only appears once something is wrong lets a lone orphan slip
/// past someone who never thought to look for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CommentFilter {
    #[default]
    All,
    Open,
    Resolved,
    Orphaned,
}

impl CommentFilter {
    pub fn admits(&self, row: &CommentRow) -> bool {
        match self {
            CommentFilter::All => true,
            CommentFilter::Open => !row.resolved && !row.orphaned,
            CommentFilter::Resolved => row.resolved,
            CommentFilter::Orphaned => row.orphaned,
        }
    }
}

/// How a dock orders its rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CommentSort {
    /// Top to bottom as the writer would scroll past them. The default, because
    /// that is how a manuscript is read.
    #[default]
    DocumentOrder,
    /// "What did I just leave myself" — the review-pass order.
    NewestFirst,
}

/// One turn in a thread: the comment that opened it, or one of its replies.
///
/// Replies and comments are separate entities with independent id spaces, so a
/// bare `u64` cannot say which of the two it names — and the two things keyed by
/// it here (the live body document, and "focus this when it appears") would
/// silently cross-talk the first time a reply's id collided with a comment's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ThreadEntry {
    Comment(u64),
    Reply(u64),
}

#[derive(Clone)]
pub struct CommentsViewModel {
    model: CommentsListModel,
    filter: Signal<CommentFilter>,
    sort: Signal<CommentSort>,
    /// Free-text author filter; empty means everyone. Populated from the distinct
    /// names actually present, never a fixed roster — this app has no identity
    /// system and this does not invent one.
    author: Signal<String>,
    /// The default author for new threads: the book's byline, which is the only
    /// name the app knows.
    default_author: Signal<String>,
    /// A seek the editor for `content_id` should perform the moment it next
    /// attaches: `(content_id, start, end)`.
    ///
    /// Needed because opening an item from a dock does not produce a live
    /// `EditorHandle` synchronously — the tab is built during the frame that
    /// follows. The caller tries an immediate seek first (which succeeds when the
    /// item was already open) and parks this as the fallback for the fresh-open
    /// case; whichever lands first consumes it.
    pending_seek: Signal<Option<(u64, usize, usize)>>,
    /// One live `TextDocument` per comment body, kept for as long as the project is
    /// open.
    ///
    /// A card is rebuilt whenever the model changes — a reply, a resolve, an edit
    /// elsewhere — and a document minted per build would lose the caret and the
    /// undo history mid-sentence. Same reasoning that puts prose documents on a
    /// shared `OpenDoc` rather than on the tab that shows them.
    body_docs: Rc<RefCell<HashMap<ThreadEntry, TextDocument>>>,
    /// The turn whose editor should take keyboard focus the moment it is built.
    ///
    /// Deliberately a plain `Cell`, not a `Signal`: nothing needs to *react* to
    /// it. The rebuild that mints the card is already driven by the comment set's
    /// structure changing, and a signal read during `build` would make the card
    /// re-run its own build every time the flag was consumed.
    pending_focus: Rc<Cell<Option<ThreadEntry>>>,
    /// Whether anchored comments are **drawn** — Tools ▸ Comments.
    ///
    /// Presentation only. The model keeps loading, the docks keep listing, the
    /// screen reader keeps announcing; what this switches off is the ochre marks,
    /// the leaders and the margin. Hiding the marks is a way to read the prose
    /// cleanly, not a way to stop having comments.
    ///
    /// A `Signal`, unlike [`pending_focus`](Self::pending_focus) beside it, because
    /// the margin has to *rebuild* when it changes — that is the whole point.
    visible: Signal<bool>,
    /// The backend handle, for the undo the delete snackbar offers.
    app_ctx: Rc<AppContext>,
    /// This Work's undo stack.
    stack: Signal<Option<u64>>,
    /// Which document the margin is currently showing, so "delete all" from a card
    /// can be scoped to it rather than to the whole project.
    margin_content: Signal<Option<u64>>,
    /// The comment palette: the wash behind commented text, the ink for the marks
    /// (triangle, bracket, leaders) and the card fill.
    ///
    /// Three values, not one — the wash is deliberately paler than the ink so text
    /// stays readable through it while the marks stay legible against the page.
    /// Held together in one place so they cannot drift apart, which is exactly what
    /// happened when the session and the margin each carried their own default.
    palette: Signal<CommentPalette>,
}

/// The comment feature's fixed palette.
///
/// These are **literal colours, not theme roles**, and that is a deliberate
/// exception to the house rule.
///
/// The precedent is *tag colours* (`tags/contrast.rs`), documented there as "the
/// one sanctioned exception to the semantic-colour rule". It is emphatically NOT
/// the spell squiggle, which derives from `theme.colors.text_error` and is the
/// model of *following* the rule — that mistake is worth naming, because it is
/// the one a reader is most likely to make here.
///
/// The justification is the same as for tags: anchored-comment ochre is a
/// cross-application convention a writer already recognises from LibreOffice and
/// Word, so it is closer to data than to chrome. Keeping the three in one named
/// struct is the concession — one place to change them, no raw hex anywhere else,
/// and everything derived (the stacked wash, the dark-mode variants) computed
/// rather than also hardcoded, exactly as `contrast.rs` splits it.
///
/// They are tuned for a light page. [`CommentPalette::for_theme`] darkens them for
/// a dark one rather than blinding the reader with a pale-yellow slab.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CommentPalette {
    /// Behind commented text.
    pub wash: Color,
    /// The triangle, the paragraph bracket and the dotted leaders.
    pub ink: Color,
    /// The card's fill — and the fill behind each turn's body editor, focused or
    /// not, so a note reads as written text rather than as a form to fill in.
    pub card: Color,
}

impl Default for CommentPalette {
    fn default() -> Self {
        Self {
            wash: Color::from_hex("#F1E4BF"),
            ink: Color::from_hex("#C69200"),
            card: Color::from_hex("#FFFFC1"),
        }
    }
}

impl CommentPalette {
    /// The palette for the current theme.
    ///
    /// On a dark page the light values are dropped toward the surface: a
    /// `#FFFFC1` card on a dark background is a glare, and a `#F1E4BF` wash under
    /// light text is unreadable. The ink survives largely intact — it is a line,
    /// not a field, and it has to stay visible either way.
    pub fn for_theme(dark: bool) -> Self {
        let base = Self::default();
        if !dark {
            return base;
        }
        Self {
            wash: base.wash.darken(0.62),
            ink: base.ink.lighten(0.08),
            card: base.card.darken(0.68),
        }
    }
}

impl CommentsViewModel {
    pub fn new(model: CommentsListModel, app_ctx: Rc<AppContext>, stack: Signal<Option<u64>>) -> Self {
        Self {
            model,
            app_ctx,
            stack,
            margin_content: Signal::new(None),
            filter: Signal::new(CommentFilter::All),
            sort: Signal::new(CommentSort::DocumentOrder),
            author: Signal::new(String::new()),
            default_author: Signal::new(String::new()),
            pending_seek: Signal::new(None),
            body_docs: Rc::new(RefCell::new(HashMap::new())),
            pending_focus: Rc::new(Cell::new(None)),
            visible: Signal::new(true),
            palette: Signal::new(CommentPalette::default()),
        }
    }

    // ── visibility ──────────────────────────────────────────────────────────

    /// Whether comment marks and the margin are drawn. Bound by the margin at
    /// `BindingLevel::Rebuild`, so flipping it rebuilds every card column.
    pub fn visible_signal(&self) -> Signal<bool> {
        self.visible.clone()
    }

    pub fn is_visible(&self) -> bool {
        self.visible.get()
    }

    /// Show or hide the marks. Driven by the `editor.comments` setting through the
    /// effect in `App::build` — the same one-setting-many-surfaces shape the master
    /// spell-check switch uses.
    pub fn set_visible(&self, visible: bool) {
        self.visible.set(visible);
    }

    /// Record which document the margin is showing — set by the binding as it
    /// pushes live anchors, so a card's "delete all" knows its own scope.
    pub fn set_margin_content(&self, content_id: u64) {
        self.margin_content.set(Some(content_id));
    }

    /// The shared palette, for the margin, the cards and the highlight session.
    pub fn palette_signal(&self) -> Signal<CommentPalette> {
        self.palette.clone()
    }

    /// Re-resolve the palette for the current theme. Called by `App` on theme change.
    pub fn set_dark(&self, dark: bool) {
        self.palette.set(CommentPalette::for_theme(dark));
    }

    /// The live document behind one comment's body, created on first use and
    /// seeded from the stored text.
    ///
    /// Seeded **only** on creation: re-seeding on every call would fight the writer
    /// mid-keystroke, since the store is updated from this very document.
    pub fn body_doc(&self, entry: ThreadEntry, initial: &str) -> TextDocument {
        let mut docs = self.body_docs.borrow_mut();
        docs.entry(entry)
            .or_insert_with(|| {
                let doc = TextDocument::new();
                let _ = doc.set_plain_text(initial);
                doc
            })
            .clone()
    }

    /// Drop a comment's cached document — called when the thread is deleted, so a
    /// long session does not accumulate documents for notes that no longer exist.
    fn forget_body_doc(&self, entry: ThreadEntry) {
        self.body_docs.borrow_mut().remove(&entry);
    }

    // ── focus ───────────────────────────────────────────────────────────────

    /// Ask the card for `entry` to take keyboard focus as soon as it exists.
    ///
    /// Parked rather than applied, because at the moment a comment is created its
    /// card does not exist yet: creating it changes the comment set's structure,
    /// the margin rebuilds on the next frame, and only then is there an editor to
    /// focus. Every create path goes through here so a new note is always ready
    /// to be typed into — the whole point of a composer that *is* the card.
    pub fn request_entry_focus(&self, entry: ThreadEntry) {
        self.pending_focus.set(Some(entry));
    }

    /// Claim the parked focus if it names `entry`. One-shot: leaving it set would
    /// yank the caret back into this card on every later rebuild.
    pub fn take_entry_focus(&self, entry: ThreadEntry) -> bool {
        if self.pending_focus.get() == Some(entry) {
            self.pending_focus.set(None);
            true
        } else {
            false
        }
    }

    /// Ask the editor showing `content_id` to select `[start, end)` when it next
    /// attaches.
    pub fn request_seek(&self, content_id: u64, start: usize, end: usize) {
        self.pending_seek.set(Some((content_id, start, end)));
    }

    /// Consume a parked seek if it targets `content_id`. Consuming rather than
    /// peeking is deliberate: a seek is a one-shot navigation, and leaving it set
    /// would drag the caret back on every subsequent rebuild of that editor.
    pub fn take_seek(&self, content_id: u64) -> Option<(usize, usize)> {
        match self.pending_seek.get() {
            Some((cid, s, e)) if cid == content_id => {
                self.pending_seek.set(None);
                Some((s, e))
            }
            _ => None,
        }
    }

    pub fn model(&self) -> CommentsListModel {
        self.model.clone()
    }

    pub fn filter_signal(&self) -> Signal<CommentFilter> {
        self.filter.clone()
    }

    pub fn sort_signal(&self) -> Signal<CommentSort> {
        self.sort.clone()
    }

    pub fn author_signal(&self) -> Signal<String> {
        self.author.clone()
    }

    pub fn set_filter(&self, f: CommentFilter) {
        self.filter.set(f);
    }

    pub fn set_sort(&self, s: CommentSort) {
        self.sort.set(s);
    }

    pub fn set_default_author(&self, name: &str) {
        self.default_author.set(name.to_string());
    }

    /// Every distinct author present, for the dock's author chip.
    pub fn authors(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .model
            .rows()
            .into_iter()
            .map(|r| r.author_name)
            .filter(|n| !n.is_empty())
            .collect();
        names.sort();
        names.dedup();
        names
    }

    /// Rows a dock should show, after filtering and sorting.
    ///
    /// `scope_item` narrows to one binder item (the trailing, per-document dock);
    /// `None` is the whole project (the leading dock).
    pub fn visible_rows(&self, scope_item: Option<u64>) -> Vec<CommentRow> {
        let filter = self.filter.get();
        let author = self.author.get();
        let mut rows: Vec<CommentRow> = self
            .model
            .rows()
            .into_iter()
            .filter(|r| scope_item.is_none_or(|i| r.item_id == Some(i)))
            .filter(|r| filter.admits(r))
            .filter(|r| author.is_empty() || r.author_name == author)
            .collect();
        if self.sort.get() == CommentSort::NewestFirst {
            rows.sort_by(|a, b| b.created_at.cmp(&a.created_at).then_with(|| b.id.cmp(&a.id)));
        }
        // DocumentOrder is the model's own sort, already applied.
        rows
    }

    /// How many open threads a dock's badge should show for `scope_item`
    /// (`None` = whole project). Resolved and orphaned threads are excluded: the
    /// badge answers "what still needs me".
    pub fn open_count(&self, scope_item: Option<u64>) -> usize {
        self.model
            .rows()
            .into_iter()
            .filter(|r| scope_item.is_none_or(|i| r.item_id == Some(i)))
            .filter(|r| r.is_open() && !r.orphaned)
            .count()
    }

    pub fn orphan_count(&self) -> usize {
        self.model.orphans().len()
    }

    // ── creating ────────────────────────────────────────────────────────────

    /// Comment on a selected range of `content_id`.
    ///
    /// `text` is the document's plain text and `block_starts` every block's start
    /// offset, both in the document-absolute char space. Returns `None` when the
    /// selection is empty — there is nothing to anchor to, and a zero-width
    /// comment would be born orphaned.
    #[allow(clippy::too_many_arguments)]
    pub fn add_range_comment(
        &self,
        content_id: u64,
        text: &str,
        start: usize,
        end: usize,
        body: &str,
        stack_id: Option<u64>,
    ) -> Option<u64> {
        if end <= start {
            return None;
        }
        let a = anchor::capture(text, start, end, 0);
        let id = self.model.create(
            content_id,
            CommentAnchorKind::Range,
            &self.default_author.get(),
            body,
            &a,
            stack_id,
        )?;
        self.request_entry_focus(ThreadEntry::Comment(id));
        Some(id)
    }

    /// Comment on the paragraph containing `caret`.
    ///
    /// The stored extent is the block's current bounds, but a paragraph comment
    /// re-derives them on every resolve, so a paragraph that grows later stays
    /// wholly covered rather than keeping this frozen length.
    /// Comment on the paragraph(s) covered by `[sel_start, sel_end]`.
    ///
    /// A bare caret (`sel_start == sel_end`) comments the one block it sits in; a
    /// selection spanning several comments **all** of them, as one thread. That is
    /// the case a caret-only version cannot express, and the reason the anchor
    /// carries a block *span* rather than a single ordinal.
    #[allow(clippy::too_many_arguments)]
    pub fn add_paragraph_comment(
        &self,
        content_id: u64,
        text: &str,
        block_starts: &[usize],
        sel_start: usize,
        sel_end: usize,
        body: &str,
        stack_id: Option<u64>,
    ) -> Option<u64> {
        let first = anchor::block_of(block_starts, sel_start.min(sel_end));
        let last = anchor::block_of(block_starts, sel_start.max(sel_end));
        let total = text.chars().count();
        let (s, e) = anchor::block_extent(block_starts, total, first, last);
        let mut a = anchor::capture(text, s, e, first);
        a.block_ordinal = first;
        a.block_span = (last - first) + 1;
        let id = self.model.create(
            content_id,
            CommentAnchorKind::Paragraph,
            &self.default_author.get(),
            body,
            &a,
            stack_id,
        )?;
        self.request_entry_focus(ThreadEntry::Comment(id));
        Some(id)
    }

    // ── thread actions ──────────────────────────────────────────────────────

    /// Append a turn to `comment_id`'s conversation and focus it.
    ///
    /// An empty body is allowed and is in fact the normal case: "Reply" from the
    /// menu opens an empty turn whose editor is where it gets written, exactly
    /// like the comment that started the thread. A dedicated compose-then-send
    /// field would be a second, differently-behaved composer for the same thing.
    pub fn reply(&self, comment_id: u64, body: &str, stack_id: Option<u64>) -> Option<u64> {
        let id = self
            .model
            .reply(comment_id, &self.default_author.get(), body, stack_id)?;
        self.request_entry_focus(ThreadEntry::Reply(id));
        Some(id)
    }

    /// Replace one reply's body — the in-place edit its own card row commits.
    pub fn set_reply_body(&self, reply_id: u64, body: &str, stack_id: Option<u64>) {
        self.model.set_reply_body(reply_id, body, stack_id);
    }

    /// Delete a single turn, leaving the rest of the conversation standing.
    pub fn delete_reply(&self, reply_id: u64, stack_id: Option<u64>) {
        self.model.delete_reply(reply_id, stack_id);
        self.forget_body_doc(ThreadEntry::Reply(reply_id));
    }

    /// Delete one turn with the same few-seconds undo offer a thread delete gets.
    pub fn delete_reply_with_undo(&self, reply_id: u64, ctx: &mut EventContext) {
        let stack = self.stack.get();
        self.delete_reply(reply_id, stack);
        self.offer_undo(ctx, tr!(comments_reply_deleted_toast()), stack);
    }

    /// Replace a thread's body — what the margin card's editor commits.
    ///
    /// This is the feature's only composer: `add_*_comment` creates a thread with
    /// an empty body and the card is where it gets written.
    pub fn set_body(&self, comment_id: u64, body: &str, stack_id: Option<u64>) {
        self.model.set_body(comment_id, body, stack_id);
    }

    pub fn resolve(&self, comment_id: u64, stack_id: Option<u64>) {
        self.model.set_resolved(comment_id, true, stack_id);
    }

    pub fn reopen(&self, comment_id: u64, stack_id: Option<u64>) {
        self.model.set_resolved(comment_id, false, stack_id);
    }

    /// Delete a thread. **Always available**, including for an orphan — the
    /// Confluence failure mode this design exists to avoid is a comment the UI
    /// will neither reopen nor remove.
    pub fn delete(&self, comment_id: u64, stack_id: Option<u64>) {
        // Forget the replies' documents too: they are strong children and go with
        // the comment, so leaving their editors cached would keep documents alive
        // for turns that no longer exist.
        for reply in self
            .model
            .rows()
            .into_iter()
            .find(|r| r.id == comment_id)
            .map(|r| r.replies)
            .unwrap_or_default()
        {
            self.forget_body_doc(ThreadEntry::Reply(reply.id));
        }
        self.model.delete(comment_id, stack_id);
        self.forget_body_doc(ThreadEntry::Comment(comment_id));
    }

    /// Delete one thread and offer a few seconds to take it back.
    ///
    /// Unlike Empty Trash, this does **not** clear the undo history when the
    /// toast expires. A comment delete stays undoable like any other edit; the
    /// snackbar is a convenience for the moment right after the click, not a
    /// point of no return. Treating it as one would mean a stray click on a
    /// margin card could silently discard the project's whole undo stack.
    pub fn delete_with_undo(&self, comment_id: u64, ctx: &mut EventContext) {
        let stack = self.stack.get();
        self.delete(comment_id, stack);
        self.offer_undo(ctx, tr!(comments_deleted_toast()), stack);
    }

    /// Delete every thread on the document the given one belongs to.
    ///
    /// Scoped to this document rather than the project: the card is a per-document
    /// surface, and "delete all" from it meaning *every comment in the book* would
    /// be a genuinely destructive surprise.
    pub fn delete_all_on_content(&self, content_id: u64, stack_id: Option<u64>) {
        for row in self.model.rows_for_content(content_id) {
            self.delete(row.id, stack_id);
        }
    }

    /// `delete_all_on_content` for whichever document currently has comments in
    /// the margin, with the same undo offer.
    pub fn delete_all_here_with_undo(&self, ctx: &mut EventContext) {
        let Some(content_id) = self.margin_content.get() else {
            return;
        };
        let stack = self.stack.get();
        let n = self.model.rows_for_content(content_id).len();
        if n == 0 {
            return;
        }
        self.delete_all_on_content(content_id, stack);
        self.offer_undo(ctx, tr!(comments_deleted_all_toast(count = n as i64)), stack);
    }

    /// The shared "deleted — Undo" snackbar.
    fn offer_undo(&self, ctx: &mut EventContext, message: LocalizedString, stack: Option<u64>) {
        let app_ctx = self.app_ctx.clone();
        ctx.show_toast(
            Toast::info(message)
                // One toast per feature: a burst of deletes replaces its own
                // snackbar rather than stacking a tower of them.
                .scoped_id("comments.deleted", 0)
                .auto_dismiss_after(UNDO_GRACE)
                .action(ToastAction::primary(tr!(comments_undo()), move |_c| {
                    let _ = undo_redo_commands::undo(&app_ctx, stack);
                })),
        );
    }

    /// Re-point an orphan at freshly chosen text, clearing its orphan state.
    pub fn relink(
        &self,
        comment_id: u64,
        text: &str,
        start: usize,
        end: usize,
        stack_id: Option<u64>,
    ) {
        if end <= start {
            return;
        }
        let a = anchor::capture(text, start, end, 0);
        self.model.set_anchor(
            comment_id,
            &Resolution::Anchored {
                start: a.start,
                length: a.length,
            },
            stack_id,
        );
    }

    // ── anchoring ───────────────────────────────────────────────────────────

    /// Re-anchor every comment on `content_id` against the document's current
    /// text, and return the live anchors the highlight session should track.
    ///
    /// Runs at open. Tier 1 (the stored offset still holds) is the overwhelming
    /// common case and is guaranteed to hit when the prose is unedited, because
    /// the Djot round-trip preserves plain text exactly.
    pub fn reanchor(
        &self,
        content_id: u64,
        text: &str,
        block_starts: &[usize],
        stack_id: Option<u64>,
    ) -> Vec<LiveAnchor> {
        let mut live = Vec::new();
        for row in self.model.rows_for_content(content_id) {
            let is_paragraph = row.kind == CommentAnchorKind::Paragraph;
            let a = Anchor {
                start: row.range_start as usize,
                length: row.range_length as usize,
                prefix: row.quote_prefix.clone(),
                exact: row.quote_exact.clone(),
                exact_truncated: row.quote_exact_truncated,
                suffix: row.quote_suffix.clone(),
                block_ordinal: row.block_ordinal_hint as usize,
                // Derived from the stored extent rather than persisted: how many
                // blocks a paragraph comment covers is a *fact about the stored
                // range*, so re-deriving it costs one lookup, needs no schema
                // field, and cannot drift out of step with the range it describes.
                block_span: span_of(block_starts, row.range_start as usize, {
                    (row.range_start + row.range_length) as usize
                }),
            };
            let resolution = anchor::resolve(text, &a, is_paragraph, block_starts);

            // Only write back when the verdict actually changed something. A
            // no-op update would bump `updated_at`, dirty the project and trigger
            // a backup on every single open.
            let changed = match &resolution {
                Resolution::Anchored { start, length } => {
                    row.orphaned
                        || *start as u64 != row.range_start
                        || *length as u64 != row.range_length
                }
                Resolution::Orphan(reason) => !row.orphaned || *reason != row.orphan_reason,
            };
            if changed {
                self.model.set_anchor(row.id, &resolution, stack_id);
            }

            if let Resolution::Anchored { start, length } = resolution {
                live.push(LiveAnchor {
                    comment_id: row.id,
                    start,
                    end: start + length,
                    is_paragraph,
                    resolved: row.resolved,
                });
            }
        }
        live
    }

    /// Write tracked live offsets back to storage.
    ///
    /// Called at flush, not per keystroke: the backend only observes prose at
    /// flush granularity anyway, and a write per keystroke would flood the undo
    /// stack with anchor updates.
    pub fn persist_live_anchors(&self, live: &[LiveAnchor], stack_id: Option<u64>) {
        for a in live {
            let resolution = if a.end > a.start {
                Resolution::Anchored {
                    start: a.start,
                    length: a.end - a.start,
                }
            } else {
                Resolution::Orphan(CommentOrphanReason::TextNotFound)
            };
            self.model.set_anchor(a.comment_id, &resolution, stack_id);
        }
    }

    /// Mark comments the live session saw collapse to nothing as orphaned, now.
    pub fn mark_orphaned(&self, ids: &[u64], stack_id: Option<u64>) {
        for id in ids {
            self.model.set_anchor(
                *id,
                &Resolution::Orphan(CommentOrphanReason::TextNotFound),
                stack_id,
            );
        }
    }

    /// Every thread covering `offset` in `content_id` — what a click on commented
    /// text opens. Returns *all* of them, which is how the panel disambiguates
    /// overlapping comments that the underline deliberately cannot.
    pub fn threads_at(&self, live: &[LiveAnchor], offset: usize) -> Vec<CommentRow> {
        let hit: Vec<u64> = live
            .iter()
            .filter(|a| a.start <= offset && offset < a.end)
            .map(|a| a.comment_id)
            .collect();
        self.model
            .rows()
            .into_iter()
            .filter(|r| hit.contains(&r.id))
            .collect()
    }
}

/// How many blocks the extent `[start, end)` touches — at least one.
fn span_of(block_starts: &[usize], start: usize, end: usize) -> usize {
    if block_starts.is_empty() || end <= start {
        return 1;
    }
    let first = anchor::block_of(block_starts, start);
    let last = anchor::block_of(block_starts, end.saturating_sub(1));
    (last.saturating_sub(first)) + 1
}

/// `App` builds this once and shares it by `.clone()`; the `Rc` keeps the
/// single-instance shape explicit at call sites that store it.
pub type SharedCommentsViewModel = Rc<CommentsViewModel>;

#[cfg(test)]
mod tests {
    use super::*;

    fn row(id: u64, item: Option<u64>, resolved: bool, orphaned: bool, author: &str) -> CommentRow {
        CommentRow {
            id,
            item_id: item,
            content_id: if orphaned { None } else { Some(1) },
            resolved,
            orphaned,
            author_name: author.into(),
            ..Default::default()
        }
    }

    #[test]
    fn the_open_filter_excludes_both_resolved_and_orphaned() {
        let f = CommentFilter::Open;
        assert!(f.admits(&row(1, Some(1), false, false, "j")));
        assert!(!f.admits(&row(2, Some(1), true, false, "j")));
        assert!(
            !f.admits(&row(3, None, false, true, "j")),
            "an orphan is not actionable in place, so it is not 'open'"
        );
    }

    #[test]
    fn the_orphaned_filter_shows_orphans_regardless_of_resolved_state() {
        let f = CommentFilter::Orphaned;
        assert!(f.admits(&row(1, None, false, true, "j")));
        assert!(f.admits(&row(2, None, true, true, "j")));
        assert!(!f.admits(&row(3, Some(1), false, false, "j")));
    }

    #[test]
    fn a_parked_seek_is_delivered_once_and_only_to_its_own_document() {
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        let vm = CommentsViewModel::new(
            CommentsListModel::new(ctx.clone(), crate::app_ids::AppIds::new()),
            ctx,
            Signal::new(None),
        );
        vm.request_seek(42, 10, 20);
        assert_eq!(
            vm.take_seek(7),
            None,
            "another document must not swallow a seek meant for 42"
        );
        assert_eq!(vm.take_seek(42), Some((10, 20)));
        assert_eq!(
            vm.take_seek(42),
            None,
            "a seek is one-shot: leaving it set would drag the caret back on \
             every later rebuild of that editor"
        );
    }

    #[test]
    fn the_all_filter_hides_nothing() {
        let f = CommentFilter::All;
        for r in [
            row(1, Some(1), false, false, "j"),
            row(2, Some(1), true, false, "j"),
            row(3, None, false, true, "j"),
        ] {
            assert!(f.admits(&r));
        }
    }
}
