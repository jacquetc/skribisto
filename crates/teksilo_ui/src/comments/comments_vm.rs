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
//!   [`crate::comments::session::CommentHighlightSession`] and folded forward on every `ContentsChanged`
//!   with exclusive edges.
//!
//! They meet twice: at **open**, when [`CommentsViewModel::reanchor`] re-derives live offsets from
//! the quote; and at **flush**, when [`CommentsViewModel::persist_live_anchors`] writes the tracked
//! offsets back so the next session starts from a good hint.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use std::time::Duration;

use teksilo::prelude::*;
use teksilo::text_document::TextDocument;
use teksilo::widgets::Toast;

use crate::toast_scope::ToastWorkExt;

use frontend::AppContext;

/// How long the "deleted — Undo" snackbar stays up.
const UNDO_GRACE: Duration = Duration::from_secs(6);

use frontend::common::entities::{CommentAnchorKind, CommentOrphanReason};

use crate::comments::anchor::{self, Anchor, Resolution};
use crate::comments::session::LiveAnchor;
use crate::comments::signature::Signature;
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
    /// Every reader's remarks together, each reader's own in document order.
    ///
    /// The order for reading a manuscript *back*: four beta readers' returns
    /// interleaved by position are four voices talking over each other, and the
    /// question a writer has at that point is "what did Marc think", not "what is
    /// the next remark on page 12". The author filter answers the same question
    /// one reader at a time; this answers it for all of them at once, which is what
    /// a writer wants before deciding whose notes to act on.
    ByReader,
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
    /// Who a thread or reply created *now* would be signed by.
    ///
    /// Resolved by [`crate::comments::signature::resolve`] from the app-level
    /// identity and the project's byline, and kept current by an effect in
    /// `App::build` over all three sources. It was a one-shot `.get()` of the
    /// byline until 2026-08: `Signal::get` registers no dependency, so the value
    /// captured at the first build — before `load_work` had populated
    /// `SingleWork` — was the empty string for the life of the window, and every
    /// comment in every project was stored unsigned.
    signature: Signal<Signature>,
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
    /// The turn whose body is live under a caret right now, if any.
    ///
    /// Set by the card's own body editor on every *real* focus change (see
    /// `comments::card::TurnBodyStyle`, the same shape
    /// `crate::footnotes::dock::NoteBodyStyle` uses for `FootnotesViewModel::editing`),
    /// and consulted by [`body_doc`](Self::body_doc) before ever re-syncing an
    /// already-cached document from a body change that landed elsewhere — see
    /// that function's own doc for why "is someone typing into this exact turn
    /// right now" is the one thing that must gate a re-sync.
    editing: Signal<Option<ThreadEntry>>,
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
    /// Secondary text **on the card**: the author and the timestamp.
    ///
    /// Derived from [`Self::card`] rather than taken from `TextRole::Secondary`,
    /// which is what it used to be and is why it is now here. The card is a raw
    /// colour the theme knows nothing about, so a role tuned against the app's own
    /// surface lands on it by accident: in the dark theme `#9198A0` — a cool grey
    /// chosen against `#1E1F22`, where it clears 7:1 — sat on the warm olive card
    /// at **2.73:1**, measured off the rendered pixels. Well under AA, and at
    /// `TextStyleRole::Tiny`.
    ///
    /// The light theme got away with it at 5.81:1, which is exactly what made it
    /// easy to miss: the pairing was never wrong on purpose, it was never checked.
    pub meta: Color,
}

/// WCAG AA for normal-size text. The metadata is `Tiny`, so the large-text 3:1
/// allowance does not apply to it.
const META_MIN_CONTRAST: f32 = 4.5;

/// The card's secondary-text colour: muted toward the card's own best-contrast
/// extreme, and only as far as AA requires.
///
/// Deriving it — instead of picking two hex values that happen to work for today's
/// two cards — is the same move `tags::contrast` makes for tag text, and for the
/// same reason: it turns a heuristic into a guarantee. At `t = 1.0` this is
/// [`Color::best_contrast_text`], which `contrast.rs` proves clears AA for *every*
/// possible fill, so the search always terminates on something legible whatever the
/// card is later tuned to. Stopping at the first step that clears the bar is what
/// keeps it reading as metadata rather than as a second body line.
fn meta_on(card: Color) -> Color {
    let extreme = card.best_contrast_text();
    let mut t = 0.0;
    while t < 1.0 {
        let candidate = card.mix(extreme, t);
        if candidate.contrast_ratio(card) >= META_MIN_CONTRAST {
            return candidate;
        }
        t += 0.02;
    }
    extreme
}

impl Default for CommentPalette {
    fn default() -> Self {
        let card = Color::from_hex("#FFFFC1");
        Self {
            wash: Color::from_hex("#F1E4BF"),
            ink: Color::from_hex("#C69200"),
            card,
            meta: meta_on(card),
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
        let card = base.card.darken(0.68);
        Self {
            wash: base.wash.darken(0.62),
            ink: base.ink.lighten(0.08),
            card,
            // Re-derived from the *darkened* card, never carried over from the light
            // one — the metadata's whole problem was a colour that had not been
            // checked against the surface it was actually painted on.
            meta: meta_on(card),
        }
    }
}

impl CommentsViewModel {
    pub fn new(
        model: CommentsListModel,
        app_ctx: Rc<AppContext>,
        stack: Signal<Option<u64>>,
    ) -> Self {
        Self {
            model,
            app_ctx,
            stack,
            margin_content: Signal::new(None),
            filter: Signal::new(CommentFilter::All),
            sort: Signal::new(CommentSort::DocumentOrder),
            author: Signal::new(String::new()),
            signature: Signal::new(Signature::default()),
            pending_seek: Signal::new(None),
            body_docs: Rc::new(RefCell::new(HashMap::new())),
            pending_focus: Rc::new(Cell::new(None)),
            editing: Signal::new(None),
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

    /// The live document behind one turn's body, created on first use and seeded
    /// from the stored Djot.
    ///
    /// Cached, and not rebuilt from `initial` on every pass, for the reason the
    /// footnote dock's own `body_doc` records: a card is rebuilt whenever the
    /// comment set changes — a reply, a resolve, an edit elsewhere — and
    /// re-minting the document under a writer's caret drops it mid-word.
    ///
    /// **But "cached" must not mean "frozen forever".** A body can change out
    /// from under this cache without ever going through it — Search & Replace, an
    /// Undo, or a second window on the same `Work` all write `Comment.body`/
    /// `CommentReply.body` directly, and nothing downstream of that pushes it
    /// into a document that already exists. Left alone, the stale text would sit
    /// invisibly in the card until the writer's next keystroke here committed
    /// `doc.to_djot()` right back over whatever had just landed — silently
    /// discarding it.
    ///
    /// So this re-syncs whenever `initial` disagrees with what the cached
    /// document currently holds — but **only** when `entry` is not the turn
    /// [`editing`](Self::editing) says is live under a caret right now (kept
    /// current by the body editor's own real focus signal — see
    /// `comments::card::TurnBodyStyle`). That gate is what keeps a self-typed
    /// edit from tripping this at all: `on_change` commits synchronously to the
    /// backend on every keystroke, and event dispatch here is synchronous too, so
    /// by the time this function runs again for a turn the writer is actively in,
    /// `initial` already equals what they just typed — no disagreement, no
    /// re-sync attempted. It is only a genuinely external rewrite that
    /// disagrees, and gating on focus is what stops handling *that* from
    /// re-introducing the exact clobbered-caret bug this cache exists to
    /// prevent — the fix must not trade one bug for the other.
    pub fn body_doc(&self, entry: ThreadEntry, initial: &str) -> TextDocument {
        let mut docs = self.body_docs.borrow_mut();
        if let Some(doc) = docs.get(&entry) {
            let stale = doc.to_djot().unwrap_or_default() != initial;
            let live_under_a_caret = self.editing.get() == Some(entry);
            if stale && !live_under_a_caret {
                let _ = doc.set_djot_sync(initial);
            }
            return doc.clone();
        }
        let doc = TextDocument::new();
        let _ = doc.set_djot_sync(initial);
        docs.insert(entry, doc.clone());
        doc
    }

    /// Drop a comment's cached document — called when the thread is deleted, so a
    /// long session does not accumulate documents for notes that no longer exist.
    fn forget_body_doc(&self, entry: ThreadEntry) {
        self.body_docs.borrow_mut().remove(&entry);
    }

    /// Which turn's body is live under a caret right now — the dock's own
    /// no-op-for-nothing-focused answer is `None`. See [`body_doc`](Self::body_doc)'s
    /// doc for why this matters beyond bookkeeping.
    pub fn editing(&self) -> Signal<Option<ThreadEntry>> {
        self.editing.clone()
    }

    /// Record which turn's body is live under a caret, or that none is. Called
    /// by `comments::card::TurnBodyStyle` on every real focus change of that
    /// turn's editor.
    pub fn set_editing(&self, entry: Option<ThreadEntry>) {
        self.editing.set(entry);
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

    /// Re-point the signature new comments are stored with.
    ///
    /// Called from an effect over every source it is derived from, never once at
    /// build time — see the field's own doc for the bug that distinction fixed.
    ///
    /// `set_if_changed`, because there are three such effects (one per source)
    /// and each fires on registration: a plain `set` would notify three times per
    /// build for one unchanged value. The resolve itself is a few string trims,
    /// but the notify is what would fan out.
    pub fn set_signature(&self, signature: Signature) {
        self.signature.set_if_changed(signature);
    }

    /// The signature a comment created now would carry.
    ///
    /// Public so the "your comments are unsigned" nudge can ask before deciding
    /// to warn, rather than keeping a second, drifting copy of the same rule.
    pub fn signature(&self) -> Signature {
        self.signature.get()
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
        sort_rows(&mut rows, self.sort.get());
        rows
    }

    /// Everyone whose thread covers the same passage as `row`, `row`'s own author
    /// included — so the answer is never empty for an anchored thread, and a
    /// length of one means "only this reader said anything here".
    ///
    /// **The signal a writer actually wants back from several readers.** Twelve
    /// unrelated remarks are twelve things to weigh one at a time; three readers
    /// stumbling on the same paragraph is one thing to fix, and it is invisible in
    /// a list sorted by position because the three sit next to each other looking
    /// like three separate problems.
    ///
    /// # Overlap, not equality, and not the paragraph ordinal
    ///
    /// Two readers never select the same span — one marks the clause, another the
    /// sentence around it — so equality would find nothing. Overlap of the live
    /// ranges is the right relation, and it is comparable here for a reason worth
    /// stating: both threads have already been re-anchored against **the same
    /// stored prose** (`reanchor` runs per `content_id`), so their offsets are in
    /// one coordinate space even though each arrived measured against its own
    /// reader's copy.
    ///
    /// `Anchor::block_ordinal_hint` would have been the obvious key and is not
    /// usable: `add_range_comment` passes `0` for every comment this app creates,
    /// so it is a fallback hint for paragraph re-anchoring and not a paragraph
    /// *identity*. Grouping on it would put every locally written comment in one
    /// bucket.
    ///
    /// Overlap is deliberately not transitive and this does not pretend it is.
    /// The question is asked per row — "who else is on *this* passage" — which is
    /// well defined; equivalence classes over a chain of overlaps would merge two
    /// remarks that share no words at all.
    ///
    /// Empty for an orphaned or unplaced thread: it has nowhere to point, so
    /// nothing can be said to point at the same place.
    pub fn agreement(&self, row: &CommentRow) -> Vec<String> {
        agreement_in(&self.model.rows(), row)
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
            &self.signature.get(),
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
            &self.signature.get(),
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
            .reply(comment_id, &self.signature.get(), body, stack_id)?;
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
        self.offer_undo(
            ctx,
            tr!(comments_deleted_all_toast(count = n as i64)),
            stack,
        );
    }

    /// The shared "deleted — Undo" snackbar.
    ///
    /// **Call it immediately after the delete.** The sequence is stamped here,
    /// and it names the command that has just run — anything pushed in between
    /// would be what the button reversed instead. Every caller does the delete
    /// on the line above.
    fn offer_undo(&self, ctx: &mut EventContext, message: LocalizedString, stack: Option<u64>) {
        let seq = crate::shared::undo_toast::stamp(&self.app_ctx);
        let app_ctx = self.app_ctx.clone();
        ctx.show_toast(
            Toast::info(message)
                // One toast per feature: a burst of deletes replaces its own
                // snackbar rather than stacking a tower of them.
                .scoped_id("comments.deleted", 0)
                .auto_dismiss_after(UNDO_GRACE)
                .action(crate::shared::undo_toast::undo_action(
                    app_ctx,
                    stack,
                    seq,
                    tr!(comments_undo()),
                    |_c| {},
                )),
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
    ///
    /// Only anchors that actually moved are written — the same guard
    /// [`Self::reanchor`] carries, and here it is load-bearing beyond tidiness:
    /// `Comment(Updated)` is one of `app::mutation_origins`'s dirty-marking
    /// events, so an unconditional write on every flush would mark the project
    /// unsaved *during its own save*, re-arm the autosave debounce, and loop —
    /// the exact failure mode `Content` events are excluded from that list for.
    pub fn persist_live_anchors(&self, live: &[LiveAnchor], stack_id: Option<u64>) {
        let rows = self.model.rows();
        for a in live {
            let resolution = if a.end > a.start {
                Resolution::Anchored {
                    start: a.start,
                    length: a.end - a.start,
                }
            } else {
                Resolution::Orphan(CommentOrphanReason::TextNotFound)
            };
            let Some(row) = rows.iter().find(|r| r.id == a.comment_id) else {
                continue; // deleted under the tracker — nothing to write to
            };
            let changed = match &resolution {
                Resolution::Anchored { start, length } => {
                    row.orphaned
                        || *start as u64 != row.range_start
                        || *length as u64 != row.range_length
                }
                Resolution::Orphan(reason) => !row.orphaned || *reason != row.orphan_reason,
            };
            if changed {
                self.model.set_anchor(a.comment_id, &resolution, stack_id);
            }
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

#[cfg(test)]
mod palette_tests {
    use super::*;

    /// **The bug this fixes.** The card's author and timestamp must clear WCAG AA
    /// against the card they are painted on — in *both* themes, which is the half
    /// that was never checked.
    #[test]
    fn card_metadata_clears_aa_on_the_card_in_both_themes() {
        for dark in [false, true] {
            let p = CommentPalette::for_theme(dark);
            let ratio = p.meta.contrast_ratio(p.card);
            assert!(
                ratio >= META_MIN_CONTRAST,
                "dark={dark}: metadata {:?} on card {:?} is {ratio:.2}:1",
                p.meta,
                p.card
            );
        }
    }

    /// The role it replaced, named so the regression is unmistakable.
    ///
    /// `TextRole::Secondary` resolves to `#9198A0` in the dark theme — a cool grey
    /// picked against the app surface `#1E1F22`, where it is fine. On the warm olive
    /// card it measured 2.73:1 off the rendered pixels. If someone ever puts a theme
    /// role back on this surface, this is the number they will be choosing.
    #[test]
    fn the_theme_role_it_replaced_really_did_fail_on_the_dark_card() {
        let card = CommentPalette::for_theme(true).card;
        let was = Color::from_hex("#9198A0");
        let ratio = was.contrast_ratio(card);
        assert!(
            ratio < META_MIN_CONTRAST,
            "the dark theme's text_secondary now clears AA on the card at {ratio:.2}:1 — \
             if the card was re-tuned, re-check whether `meta` is still needed"
        );
        assert!(
            CommentPalette::for_theme(true).meta.contrast_ratio(card) > ratio,
            "the derived colour must actually be an improvement on it"
        );
    }

    /// Muted, not maximal. Metadata that reached full contrast would read as a
    /// second line of body text and pull the eye off the comment itself — the
    /// reason `meta_on` stops at the first step that clears the bar.
    #[test]
    fn the_metadata_stays_quieter_than_the_body() {
        for dark in [false, true] {
            let p = CommentPalette::for_theme(dark);
            let body = p.card.best_contrast_text();
            assert!(
                p.meta.contrast_ratio(p.card) < body.contrast_ratio(p.card),
                "dark={dark}: metadata is as loud as the body text"
            );
        }
    }

    /// The derivation is a **guarantee**, not a pair of values tuned for today's two
    /// cards: whatever the card is later changed to, its metadata still clears AA.
    ///
    /// Same shape as `tags::contrast`'s own cube walk, and for the same reason — it
    /// is what makes deriving preferable to hardcoding.
    #[test]
    fn metadata_clears_aa_for_any_card_colour() {
        for r in (0..=255).step_by(17) {
            for g in (0..=255).step_by(17) {
                for b in (0..=255).step_by(17) {
                    let card = Color::from_hex(&format!("#{r:02x}{g:02x}{b:02x}"));
                    let ratio = meta_on(card).contrast_ratio(card);
                    assert!(
                        ratio >= META_MIN_CONTRAST,
                        "card #{r:02x}{g:02x}{b:02x} got {ratio:.2}:1"
                    );
                }
            }
        }
    }
}

/// Order `rows` for `sort`, in place.
///
/// A free function so the two orders that are not the model's own are provable
/// against hand-built rows — `visible_rows` needs a loaded project to reach the
/// model at all, which is a lot of scaffolding to assert a comparator with.
fn sort_rows(rows: &mut [CommentRow], sort: CommentSort) {
    match sort {
        CommentSort::NewestFirst => rows.sort_by(|a, b| {
            b.created_at
                .cmp(&a.created_at)
                .then_with(|| b.id.cmp(&a.id))
        }),
        // **Stable**, and load-bearing: each reader's own remarks keep the document
        // order the model already put them in, so the sort groups without reordering
        // within a group. `sort_by` is guaranteed stable in Rust; a switch to
        // `sort_unstable_by` here would silently shuffle one reader's notes.
        //
        // An unsigned thread sorts under the empty name, which puts it first: the
        // writer's own unattributed notes are the ones they are least likely to be
        // hunting for by name, and burying them under every named reader is worse
        // than opening with them.
        CommentSort::ByReader => rows.sort_by(|a, b| a.author_name.cmp(&b.author_name)),
        // DocumentOrder is the model's own sort, already applied.
        CommentSort::DocumentOrder => {}
    }
}

/// The rule behind [`CommentsViewModel::agreement`], over a plain slice.
///
/// Separated from the view-model for the same reason [`sort_rows`] is: the
/// property worth pinning ("who else is on this passage") is arithmetic over
/// ranges, and proving it should not need a project on disk.
fn agreement_in(rows: &[CommentRow], row: &CommentRow) -> Vec<String> {
    if !row.is_anchored() {
        return Vec::new();
    }
    let Some(content_id) = row.content_id else {
        return Vec::new();
    };
    let start = row.range_start;
    let end = row.range_start + row.range_length;
    let mut names: Vec<String> = rows
        .iter()
        .filter(|other| other.content_id == Some(content_id) && other.is_anchored())
        // Half-open: two threads that merely touch end to start are about different
        // words, and a writer reading "2 readers" on a pair that shares no character
        // at all would stop trusting the number.
        .filter(|other| other.range_start < end && start < other.range_start + other.range_length)
        .map(|other| other.author_name.clone())
        .collect();
    names.sort();
    names.dedup();
    names
}

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

    /// An anchored row on content 1, `[start, start+len)`, by `author`.
    fn ranged(id: u64, author: &str, start: u64, len: u64) -> CommentRow {
        CommentRow {
            id,
            content_id: Some(1),
            item_id: Some(1),
            author_name: author.into(),
            range_start: start,
            range_length: len,
            ..Default::default()
        }
    }

    /// Three readers on the same sentence is one problem, not three — and every
    /// one of the three cards has to say so.
    #[test]
    fn readers_on_the_same_passage_agree_with_each_other() {
        let rows = vec![
            ranged(1, "Jane", 10, 20),
            ranged(2, "Marc", 15, 5),
            ranged(3, "Ada", 12, 4),
        ];
        for row in &rows {
            assert_eq!(
                agreement_in(&rows, row),
                vec!["Ada".to_string(), "Jane".to_string(), "Marc".to_string()],
                "row {} disagrees about who is on its passage",
                row.id
            );
        }
    }

    /// Overlap does not chain, and the count must not pretend it does.
    ///
    /// Jane's long selection covers both Marc's and Ada's, but those two share no
    /// character with each other. Treating this as one group would tell Marc's card
    /// that Ada is "also flagging here" when Ada marked a different sentence — the
    /// number would grow along a chain of neighbours until a densely marked chapter
    /// reported every reader on every card, which is the failure that makes an
    /// agreement count worth nothing.
    #[test]
    fn agreement_does_not_chain_through_a_shared_neighbour() {
        let rows = vec![
            ranged(1, "Jane", 10, 30),
            ranged(2, "Marc", 12, 4),
            ranged(3, "Ada", 30, 8),
        ];
        assert_eq!(
            agreement_in(&rows, &rows[0]),
            vec!["Ada".to_string(), "Jane".to_string(), "Marc".to_string()],
            "Jane's own selection really does cover both"
        );
        assert_eq!(
            agreement_in(&rows, &rows[1]),
            vec!["Jane".to_string(), "Marc".to_string()],
            "Marc is on Jane's passage, not Ada's"
        );
        assert_eq!(
            agreement_in(&rows, &rows[2]),
            vec!["Ada".to_string(), "Jane".to_string()],
            "and Ada is on Jane's, not Marc's"
        );
    }

    /// Ranges that merely touch end to start share no character, so they are not
    /// about the same words. Half-open, and worth pinning: an off-by-one here
    /// makes every consecutive pair in a densely marked chapter read as agreement.
    #[test]
    fn threads_that_only_touch_are_not_on_the_same_passage() {
        let rows = vec![ranged(1, "Jane", 0, 10), ranged(2, "Marc", 10, 10)];
        assert_eq!(agreement_in(&rows, &rows[0]), vec!["Jane".to_string()]);
        assert_eq!(agreement_in(&rows, &rows[1]), vec!["Marc".to_string()]);
    }

    /// One reader marking the same passage twice is one reader.
    #[test]
    fn a_reader_who_commented_twice_is_counted_once() {
        let rows = vec![ranged(1, "Jane", 0, 10), ranged(2, "Jane", 2, 3)];
        assert_eq!(agreement_in(&rows, &rows[0]), vec!["Jane".to_string()]);
    }

    /// Two documents are two coordinate spaces. Offsets only became comparable
    /// once both threads were re-anchored against the same stored prose, which is
    /// per `content_id` — so a row from another scene overlapping numerically is
    /// not overlapping at all.
    #[test]
    fn overlap_is_never_read_across_two_documents() {
        let mut elsewhere = ranged(2, "Marc", 0, 10);
        elsewhere.content_id = Some(2);
        let rows = vec![ranged(1, "Jane", 0, 10), elsewhere];
        assert_eq!(agreement_in(&rows, &rows[0]), vec!["Jane".to_string()]);
    }

    /// A thread with nowhere to point cannot be said to point at the same place
    /// as anything else — including one that resolved to an empty range.
    #[test]
    fn an_orphan_or_an_unplaced_thread_agrees_with_nobody() {
        let orphan = CommentRow {
            id: 1,
            content_id: None,
            orphaned: true,
            author_name: "Jane".into(),
            ..Default::default()
        };
        let unplaced = CommentRow {
            id: 2,
            content_id: Some(1),
            range_start: 4,
            range_length: 0,
            author_name: "Marc".into(),
            ..Default::default()
        };
        let rows = vec![orphan.clone(), unplaced.clone(), ranged(3, "Ada", 0, 10)];
        assert!(agreement_in(&rows, &orphan).is_empty());
        assert!(agreement_in(&rows, &unplaced).is_empty());
        assert_eq!(
            agreement_in(&rows, &rows[2]),
            vec!["Ada".to_string()],
            "and neither of them counts towards a thread that IS placed"
        );
    }

    /// Grouping by reader must not reorder one reader's own remarks: they arrive
    /// in document order and that is the order to read them in.
    #[test]
    fn grouping_by_reader_keeps_each_readers_own_document_order() {
        let mut rows = vec![
            ranged(1, "Marc", 0, 5),
            ranged(2, "Jane", 10, 5),
            ranged(3, "Marc", 20, 5),
            ranged(4, "Jane", 30, 5),
        ];
        sort_rows(&mut rows, CommentSort::ByReader);
        assert_eq!(
            rows.iter().map(|r| r.id).collect::<Vec<_>>(),
            vec![2, 4, 1, 3],
            "Jane's two in order, then Marc's two in order"
        );
    }

    /// An unsigned thread opens the list rather than being buried under every
    /// named reader.
    #[test]
    fn unsigned_threads_sort_first_when_grouped_by_reader() {
        let mut rows = vec![ranged(1, "Jane", 0, 5), ranged(2, "", 10, 5)];
        sort_rows(&mut rows, CommentSort::ByReader);
        assert_eq!(rows[0].id, 2);
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

    fn vm() -> CommentsViewModel {
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        CommentsViewModel::new(
            CommentsListModel::new(ctx.clone(), crate::app_ids::AppIds::new()),
            ctx,
            Signal::new(None),
        )
    }

    // ── `body_doc`: seeded as Djot, cached, and re-synced only when safe ─────
    //
    // Mirrors `FootnotesViewModel::body_doc`'s own tests exactly — the two
    // caches share the same clobbered-caret hazard and the same fix.

    #[test]
    fn body_doc_seeds_from_djot_not_plain_text() {
        let vm = vm();
        let entry = ThreadEntry::Comment(1);
        // `set_plain_text` would have rendered the emphasis markers literally;
        // `set_djot_sync` parses them, and `to_djot()` round-trips them back —
        // proof the seed actually went through the Djot importer.
        let doc = vm.body_doc(entry, "Is this *really* the word?");
        assert_eq!(doc.to_djot().unwrap(), "Is this *really* the word?");
        assert_eq!(
            doc.to_plain_text().unwrap(),
            "Is this really the word?",
            "the parsed document must know the emphasis is markup, not literal \
             asterisks — which only happens if the seed went through the Djot \
             importer, not set_plain_text"
        );
    }

    /// A body's cache is not rebuilt from `initial` while the turn is live
    /// under a caret — re-syncing there would be the exact clobbered-caret bug
    /// the cache exists to prevent, traded for the disappearing-edit bug this
    /// fixes.
    #[test]
    fn body_doc_does_not_resync_a_turn_that_is_being_typed_into() {
        let vm = vm();
        let entry = ThreadEntry::Comment(1);
        let first = vm.body_doc(entry, "first draft");
        assert_eq!(first.to_djot().unwrap(), "first draft");

        vm.set_editing(Some(entry));
        let still_cached = vm.body_doc(entry, "an external rewrite landed here");
        assert_eq!(
            still_cached.to_djot().unwrap(),
            "first draft",
            "a turn live under a caret must not be overwritten out from under the writer"
        );
    }

    /// A body's cache **is** refreshed once the turn is no longer the one
    /// being typed into — an external rewrite (Search & Replace, an Undo, a
    /// second window) must not stay invisible in the card forever, only while
    /// a caret actually sits in that exact turn.
    #[test]
    fn body_doc_resyncs_a_turn_that_is_not_being_typed_into() {
        let vm = vm();
        let entry = ThreadEntry::Reply(9);
        let first = vm.body_doc(entry, "first draft");
        assert_eq!(first.to_djot().unwrap(), "first draft");

        // Not editing this turn (nor any turn) — the default state whenever
        // the writer's caret is elsewhere.
        let resynced = vm.body_doc(entry, "an external rewrite landed here");
        assert_eq!(
            resynced.to_djot().unwrap(),
            "an external rewrite landed here",
            "a body changed elsewhere must reach an already-cached document"
        );
        // The SAME cached `TextDocument` was updated in place, not replaced —
        // the turn's live handle (already held by a mounted editor, if any)
        // must see the new text too.
        assert_eq!(first.to_djot().unwrap(), "an external rewrite landed here");
    }

    /// A comment's turn and a reply's turn are different `ThreadEntry` keys
    /// even when — as here — nothing else distinguishes them; `editing` must
    /// not confuse the two, or a caret in a reply would block a resync of the
    /// comment it replies to.
    #[test]
    fn editing_gates_only_the_exact_entry_it_names() {
        let vm = vm();
        let comment = ThreadEntry::Comment(1);
        let reply = ThreadEntry::Reply(1);
        vm.body_doc(comment, "comment draft");
        vm.body_doc(reply, "reply draft");

        vm.set_editing(Some(reply));
        let comment_doc = vm.body_doc(comment, "an external rewrite of the comment");
        assert_eq!(
            comment_doc.to_djot().unwrap(),
            "an external rewrite of the comment",
            "the comment is not the turn being edited, so it must resync freely"
        );
        let reply_doc = vm.body_doc(reply, "an external rewrite of the reply");
        assert_eq!(
            reply_doc.to_djot().unwrap(),
            "reply draft",
            "the reply IS the turn being edited, so it must not be clobbered"
        );
    }
}

/// [`CommentsViewModel::persist_live_anchors`]' no-op guard, against the real
/// store: `Comment(Updated)` is a dirty-marking event (`app::mutation_origins`),
/// so a flush that writes unmoved anchors would mark the project unsaved during
/// its own save and loop the autosave debounce. The guard is only provable
/// through the real write path — the write it must skip is a backend command.
#[cfg(all(test, not(feature = "mocks")))]
mod persist_tests {
    use super::*;
    use frontend::commands::{comment_commands, content_commands, work_commands};
    use frontend::common::entities::ContentRole;
    use frontend::direct_access::{CreateContentDto, CreateWorkDto};

    fn now() -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }

    /// A VM over a real Work with one range comment on one prose row.
    fn vm_with_comment() -> (CommentsViewModel, u64) {
        let ctx = Rc::new(AppContext::new());
        let ids = crate::app_ids::AppIds::new();
        let work = work_commands::create_orphan_work(
            &ctx,
            None,
            &CreateWorkDto {
                statuses: Vec::new(),
                created_at: now(),
                updated_at: now(),
                title: "W".into(),
                ..Default::default()
            },
        )
        .expect("create work")
        .id;
        ids.work_id.set(Some(work));
        let content = content_commands::create_orphan_content(
            &ctx,
            None,
            &CreateContentDto {
                uid: Default::default(),
                created_at: now(),
                updated_at: now(),
                activated: true,
                role: ContentRole::SceneText,
                data: "The lamp guttered.".into(),
            },
        )
        .expect("create content")
        .id;

        let vm = CommentsViewModel::new(
            CommentsListModel::new(ctx.clone(), ids),
            ctx,
            Signal::new(None),
        );
        let text = "The lamp guttered.";
        let a = anchor::capture(text, 4, 8, 0); // "lamp"
        let id = vm
            .model()
            .create(
                content,
                CommentAnchorKind::Range,
                &crate::comments::signature::resolve("Jane", "", ""),
                "note",
                &a,
                None,
            )
            .expect("create comment through the model");
        (vm, id)
    }

    #[test]
    fn an_unmoved_anchor_is_not_written_back_at_flush() {
        let (vm, id) = vm_with_comment();
        let before = comment_commands::get_comment(&vm.app_ctx, &id)
            .expect("read comment")
            .expect("comment exists");

        vm.persist_live_anchors(
            &[LiveAnchor {
                comment_id: id,
                start: before.range_start as usize,
                end: (before.range_start + before.range_length) as usize,
                is_paragraph: false,
                resolved: false,
            }],
            None,
        );

        let after = comment_commands::get_comment(&vm.app_ctx, &id)
            .expect("read comment")
            .expect("comment exists");
        assert_eq!(
            after.updated_at, before.updated_at,
            "an unmoved anchor must not be rewritten — the write raises \
             Comment(Updated), which re-dirties the project mid-save"
        );
    }

    #[test]
    fn a_moved_anchor_is_still_persisted() {
        let (vm, id) = vm_with_comment();
        let before = comment_commands::get_comment(&vm.app_ctx, &id)
            .expect("read comment")
            .expect("comment exists");

        vm.persist_live_anchors(
            &[LiveAnchor {
                comment_id: id,
                start: before.range_start as usize + 2,
                end: (before.range_start + before.range_length) as usize + 2,
                is_paragraph: false,
                resolved: false,
            }],
            None,
        );

        let after = comment_commands::get_comment(&vm.app_ctx, &id)
            .expect("read comment")
            .expect("comment exists");
        assert_eq!(
            after.range_start,
            before.range_start + 2,
            "a genuinely moved anchor must still reach the store — the guard \
             skips no-ops, not the write-back itself"
        );
    }
}
