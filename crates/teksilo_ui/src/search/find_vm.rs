// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `FindViewModel` — a find banner's state (Ctrl+F).
//!
//! One per prose tab (held on its [`ContentTab`](crate::tabs::ContentTab)). It owns a
//! [`FindSession`] per document — text-document's own find-highlight layer, driven by
//! the *same* matcher a project-wide search uses, so the two can never disagree about
//! what a match is — plus the input signals the banner binds and the editor handle it
//! needs to select and scroll the current match into view.
//!
//! ## One banner, one document — or a whole page of them
//!
//! The original shape was one document: a Scene, a Note, a chapter's own prose. A
//! **stream** is not that. A Full Chapter, Part or Book is one scrolling page of many
//! editors, one per row, and the writer reading it sees a manuscript rather than a
//! stack of documents. A find that stopped at the first row's last line would be
//! answering a question nobody asked.
//!
//! So a banner is over an ordered **list** of documents, and the single-document tab is
//! the list of length one. Next and previous walk it end to end, crossing into the
//! following row when the current one runs out, and exactly one match on the whole page
//! is current at a time — which is why [`FindSession`] has to be able to hold matches
//! and stand on none of them.
//!
//! Two things follow from a page being a list that can change under a banner built
//! once. The documents come from a **closure**, re-asked whenever the query changes and
//! once a frame while the banner is open, so a split, a merge or a segment switch is
//! picked up rather than searched around. And the editor a match is revealed in is
//! resolved **by item**, through the editor registry, because a page of forty editors
//! has no single "this tab's editor" to attach.
//!
//! ## Replacing is deliberately not part of it
//!
//! The replace row appears only while the banner is over exactly one document. Rewriting
//! text across a page of them is Search & Replace's job: it has the preview, the scope
//! picker and the per-document accounting, and an in-editor Replace All that quietly
//! edited forty scenes would leave a writer with forty separate things to undo.
//!
//! ## Lifetimes
//!
//! The sessions are created **lazily**, in the banner's `build`: their two highlight
//! colours are resolved from theme roles, which are only available with a live context.
//! The editor handle is (re)attached on every editor build (a tab rebuild mints a fresh
//! widget); until it is present, next/prev still recolour the highlights, they just
//! don't scroll.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use common::types::EntityId;
use teksilo::prelude::*;
use teksilo::text_document::{FindMatch, FindOptions, HighlightFormat, TextDocument};
use teksilo::widgets::rich_text::{EditorHandle, FindSession};

/// The documents a page is showing, in the order it shows them.
///
/// A closure rather than a list because the answer moves: rows are split and merged
/// while the banner is open, and a container tab's segment bar re-points the whole page
/// under a banner that was built once.
pub type PageDocuments = Rc<dyn Fn() -> Vec<(EntityId, TextDocument)>>;

/// How one of those documents is turned back into **every** editor showing it.
///
/// The registry answers this (see [`crate::format::FormatViewModel::handles_by_item`]);
/// nothing else can, because a page builds one editor per row and none of them is "the
/// tab's editor".
///
/// Every one, not the first, and that is not defensive: the same scene can be a row on
/// this page *and* a tab of its own, and a tab that is not on screen is parked dormant
/// with no layout at all. Revealing through that one requests nothing and reports nothing
/// — the match is selected, the counter moves, and the page does not budge. The caller
/// tries each until one says it could.
pub type ResolveEditor = Rc<dyn Fn(EntityId) -> Vec<EditorHandle>>;

/// What a banner searches.
enum FindSubject {
    /// This tab's own single document.
    Own(TextDocument),
    /// A page of documents, in reading order.
    Page(PageDocuments),
}

/// One searched document: its find-highlight layer, and the item whose editor shows it.
struct Slice {
    /// `None` only for the single-document banner of a surface with no item behind it
    /// (the widget tests, and the preview band) — a page always names its rows.
    item: Option<EntityId>,
    doc: TextDocument,
    session: FindSession,
}

// The API is bound by the banner + the editors view-model; wired incrementally.
#[allow(dead_code)]
#[derive(Clone)]
pub struct FindViewModel {
    /// The `BinderItem` this banner searches, for the margin lane's arbiter.
    ///
    /// `None` for a surface with no item behind it, and the banner then publishes
    /// nothing — a lane cannot mark hits it cannot attribute to a document. Meaningful
    /// for [`FindSubject::Own`] only; a page names each row instead.
    item: std::cell::Cell<Option<EntityId>>,
    /// The document, or the page of documents, this banner is over.
    subject: Rc<FindSubject>,
    /// How to reach the editor showing a given row. `None` for the single-document
    /// banner, which is handed its editor's handle directly by that editor's build.
    resolve_editor: Option<ResolveEditor>,
    /// One find-highlight layer per searched document, in reading order. Empty until
    /// the banner's first `build` resolves the theme colours.
    slices: Rc<RefCell<Vec<Slice>>>,
    /// Which slice holds the current match. The invariant every path here maintains:
    /// **this slice's session has a current match and no other session does.**
    cursor: Rc<Cell<usize>>,
    /// The two theme-resolved highlight formats, remembered so a slice minted later —
    /// a row split while the banner is open — is dressed like its siblings.
    formats: Rc<RefCell<Option<(HighlightFormat, HighlightFormat)>>>,
    /// The current editor's handle — set on each editor build, since a tab rebuild
    /// mints a fresh editor widget and thus a fresh handle. Drives select +
    /// scroll-into-view of the current match on a single-document banner.
    handle: Rc<RefCell<Option<EditorHandle>>>,
    /// Banner visibility (Ctrl+F opens it, Escape/close hides it).
    visible: Signal<bool>,
    query: Signal<String>,
    case_sensitive: Signal<bool>,
    whole_word: Signal<bool>,
    /// Total matches across every searched document, and the 1-based ordinal of the
    /// current one within that whole (0 when none) — the "N of M" the banner shows.
    /// Kept as signals so the label is reactive.
    count: Signal<usize>,
    current: Signal<usize>,
    /// Whether this banner is over exactly one document. The replace row's gate — see
    /// the module note — and reactive, because a page's row count changes under it.
    single: Signal<bool>,
    /// Bumped on every [`open`](Self::open). The banner binds this at
    /// `BindingLevel::Rebuild`, so opening the banner rebuilds it — the moment it
    /// can grab keyboard focus for the query field (the banner is otherwise a
    /// dormant, build-once child of a visibility gate, so a plain focus-in-build
    /// would never line up with opening).
    focus_seq: Signal<u64>,
    /// After a query change the first match is current but not yet scrolled to;
    /// the first Enter reveals it, and only subsequent Enters advance. Matches the
    /// find-bar convention (type → highlight; Enter → jump to the match).
    reveal_pending: Rc<Cell<bool>>,

    // ── in-editor replace (the banner's second row) ──
    /// Whether the replace row is disclosed (Ctrl+R / the banner's replace toggle).
    replace_mode: Signal<bool>,
    replacement: Signal<String>,
    /// Preserve the case of each replaced occurrence (AURÉLIEN → AURÉLIAN, not
    /// aurélian). On by default, like the project-wide replace.
    preserve_case: Signal<bool>,
}

impl FindViewModel {
    /// A banner over one document — a Scene, a Note, a chapter's own prose.
    pub fn new(doc: TextDocument) -> Self {
        Self::over(FindSubject::Own(doc), None, true)
    }

    /// A banner over **a page of documents**: a manuscript stream's rows, a note's
    /// In-prose reading. `documents` is re-asked rather than captured, and
    /// `resolve_editor` is how a match in row 31 reaches row 31's editor.
    pub fn over_page(documents: PageDocuments, resolve_editor: ResolveEditor) -> Self {
        Self::over(FindSubject::Page(documents), Some(resolve_editor), false)
    }

    fn over(subject: FindSubject, resolve_editor: Option<ResolveEditor>, single: bool) -> Self {
        Self {
            item: std::cell::Cell::new(None),
            subject: Rc::new(subject),
            resolve_editor,
            slices: Rc::new(RefCell::new(Vec::new())),
            cursor: Rc::new(Cell::new(0)),
            formats: Rc::new(RefCell::new(None)),
            handle: Rc::new(RefCell::new(None)),
            visible: Signal::new(false),
            query: Signal::new(String::new()),
            case_sensitive: Signal::new(false),
            whole_word: Signal::new(false),
            count: Signal::new(0),
            current: Signal::new(0),
            single: Signal::new(single),
            focus_seq: Signal::new(0),
            reveal_pending: Rc::new(Cell::new(false)),
            replace_mode: Signal::new(false),
            replacement: Signal::new(String::new()),
            preserve_case: Signal::new(true),
        }
    }

    // ── view handles ────────────────────────────────────────────────────────
    /// Name the item this banner is searching, so its hits can reach the margin
    /// lane. Set by the tab that builds it; a banner with no item publishes nothing.
    pub fn for_item(self, item: EntityId) -> Self {
        self.item.set(Some(item));
        self
    }

    pub fn visible_signal(&self) -> Signal<bool> {
        self.visible.clone()
    }
    pub fn query_signal(&self) -> Signal<String> {
        self.query.clone()
    }
    pub fn case_sensitive_signal(&self) -> Signal<bool> {
        self.case_sensitive.clone()
    }
    pub fn whole_word_signal(&self) -> Signal<bool> {
        self.whole_word.clone()
    }
    pub fn count_signal(&self) -> Signal<usize> {
        self.count.clone()
    }
    pub fn current_signal(&self) -> Signal<usize> {
        self.current.clone()
    }
    /// Whether the banner is over exactly one document — what the replace row is
    /// disclosed by. See the module note on why replacing across a page is withheld.
    pub fn single_document_signal(&self) -> Signal<bool> {
        self.single.clone()
    }
    /// Bumped on `open` — bind at `BindingLevel::Rebuild` to autofocus the field.
    pub fn focus_seq_signal(&self) -> Signal<u64> {
        self.focus_seq.clone()
    }
    pub fn replace_mode_signal(&self) -> Signal<bool> {
        self.replace_mode.clone()
    }
    pub fn replacement_signal(&self) -> Signal<String> {
        self.replacement.clone()
    }
    pub fn preserve_case_signal(&self) -> Signal<bool> {
        self.preserve_case.clone()
    }

    /// Attach (or replace) the editor handle — called on each editor build, since
    /// a tab rebuild mints a fresh editor widget and thus a fresh handle.
    pub fn attach_handle(&self, handle: EditorHandle) {
        *self.handle.borrow_mut() = Some(handle);
    }

    /// This tab's **main prose** editor handle, if an editor is currently built.
    ///
    /// The handle is not conceptually find's — it is "the prose editor of this
    /// tab", which this view-model happens to own because it was the first
    /// feature to need it, and which it keeps correctly re-pointed across tab
    /// rebuilds. Other prose-editing commands read it through
    /// [`crate::editors::EditorsViewModel::focused_prose_handle`] rather than re-plumbing the
    /// same attachment through every `writing_column` call site.
    ///
    /// Always `None` on a page banner: forty editors, none of them "this tab's".
    pub fn editor_handle(&self) -> Option<EditorHandle> {
        self.handle.borrow().clone()
    }

    /// Is there anything here to search at all?
    ///
    /// The gate on opening: a container tab's banner is mounted over the manuscript
    /// stream and the full-synopsis stream, and Ctrl+F reaches it from the Corkboard
    /// and the Overview too, where the page is showing no prose whatsoever. Opening
    /// an empty banner there would be a strip that can only ever say "0 of 0".
    pub fn has_documents(&self) -> bool {
        match self.subject.as_ref() {
            FindSubject::Own(_) => true,
            FindSubject::Page(documents) => !documents().is_empty(),
        }
    }

    // ── the searched documents ──────────────────────────────────────────────

    /// The documents to search, in reading order, as of right now.
    fn documents(&self) -> Vec<(Option<EntityId>, TextDocument)> {
        match self.subject.as_ref() {
            FindSubject::Own(doc) => vec![(self.item.get(), doc.clone())],
            FindSubject::Page(documents) => documents()
                .into_iter()
                .map(|(item, doc)| (Some(item), doc))
                .collect(),
        }
    }

    /// Re-resolve what this banner is over, rebuilding the highlight layers if the set
    /// has moved. Returns whether it did — the caller then re-runs the query.
    ///
    /// Compared **by item**, not by document: a row's document is stable while the row
    /// is on the page, and a `TextDocument` is a shared handle with no identity to
    /// compare. The set moves when a row is split or merged, when the reader switches
    /// segment, and on the very first build, when there is nothing here yet.
    fn sync_documents(&self) -> bool {
        let Some((current_format, other_format)) = self.formats.borrow().clone() else {
            // No theme yet: the banner has never been built, so there is nothing to
            // dress a session in. The first `ensure_session` will do this.
            return false;
        };
        let wanted = self.documents();
        {
            let slices = self.slices.borrow();
            if slices.len() == wanted.len()
                && slices
                    .iter()
                    .zip(&wanted)
                    .all(|(had, want)| had.item == want.0)
            {
                return false;
            }
        }
        let rebuilt: Vec<Slice> = wanted
            .into_iter()
            .map(|(item, doc)| Slice {
                session: FindSession::new(&doc, current_format.clone(), other_format.clone()),
                item,
                doc,
            })
            .collect();
        self.single.set(rebuilt.len() == 1);
        *self.slices.borrow_mut() = rebuilt;
        self.cursor.set(0);
        true
    }

    /// Create the highlight layers if they don't exist yet, with the two theme-
    /// resolved formats, and (re)apply the current query. Idempotent — the banner's
    /// `build` calls it every time; only a first build, or one where the page under it
    /// changed, does real work.
    pub fn ensure_session(&self, current_format: HighlightFormat, other_format: HighlightFormat) {
        if self.formats.borrow().is_none() {
            *self.formats.borrow_mut() = Some((current_format, other_format));
        }
        if self.sync_documents() {
            self.apply_query();
            self.publish();
        }
    }

    /// Open the banner and ask the field for focus (bumps `focus_seq`, which the
    /// banner observes at `Rebuild`). Idempotent — pressing Ctrl+F while already
    /// open just re-focuses the field.
    ///
    /// Refuses on a page with nothing to search — see [`has_documents`](Self::has_documents).
    pub fn open(&self) {
        if !self.has_documents() {
            return;
        }
        self.visible.set(true);
        let s = &self.focus_seq;
        s.set(s.get().wrapping_add(1));
    }

    /// Close the banner: clear the highlighting and hide the row. The query text
    /// is kept, so reopening finds it again.
    pub fn close(&self) {
        for slice in self.slices.borrow_mut().iter_mut() {
            slice.session.clear();
        }
        self.visible.set(false);
        self.publish();
    }

    /// Close the banner and return keyboard focus to the prose — the find-bar
    /// convention for Escape and the close button, so the user keeps typing where
    /// they were reading. On a page that is the editor holding the match the reader
    /// walked to, not the tab's own; focus lands only if that editor is built.
    pub fn close_and_refocus(&self, ctx: &mut EventContext) {
        let landed = self.cursor_item();
        self.close();
        if let Some(handle) = self.handles_for(landed).into_iter().next() {
            handle.focus(ctx);
        }
    }

    /// The matcher options built from the toggles.
    fn options(&self) -> FindOptions {
        FindOptions {
            case_sensitive: self.case_sensitive.get(),
            whole_word: self.whole_word.get(),
            ..Default::default()
        }
    }

    /// Run the query over every searched document and put the cursor on the first
    /// document that has a hit — the only one left standing on a match of its own.
    fn apply_query(&self) {
        let query = self.query.get();
        let options = self.options();
        let mut slices = self.slices.borrow_mut();
        for slice in slices.iter_mut() {
            slice.session.set_query(&query, &options);
        }
        let first = slices
            .iter()
            .position(|s| s.session.match_count() > 0)
            .unwrap_or(0);
        for (i, slice) in slices.iter_mut().enumerate() {
            if i != first {
                slice.session.set_current(None);
            }
        }
        drop(slices);
        self.cursor.set(first);
    }

    /// Re-run the query and repaint the highlights. Does **not** scroll (it runs
    /// from a signal effect with no `EventContext`); the first match becomes
    /// current, and Enter / Next reveal it.
    pub fn refresh_query(&self) {
        self.sync_documents();
        self.apply_query();
        // The first match is current but not scrolled to yet — the first Enter
        // should jump to it (see `submit`).
        self.reveal_pending.set(self.count_now() > 0);
        self.publish();
    }

    /// Enter in the query field: jump to the current (first) match the first time,
    /// then advance on subsequent presses — the find-bar convention.
    pub fn submit(&self, ctx: &mut EventContext) {
        if self.reveal_pending.replace(false) {
            self.reveal_current(ctx);
        } else {
            self.next(ctx);
        }
    }

    /// Advance to the next match (wrapping) and reveal it.
    pub fn next(&self, ctx: &mut EventContext) {
        self.reveal_pending.set(false);
        let found = self.step(true);
        self.publish();
        self.reveal(ctx, found);
    }

    /// Step to the previous match (wrapping) and reveal it.
    pub fn prev(&self, ctx: &mut EventContext) {
        self.reveal_pending.set(false);
        let found = self.step(false);
        self.publish();
        self.reveal(ctx, found);
    }

    /// Move one match forward (or back), **crossing into another document** when this
    /// one runs out, and wrapping round the whole page.
    ///
    /// The two halves are the whole of the multi-document walk. Inside a document it is
    /// the session's own next/prev. Leaving one, the session being left is told it holds
    /// no current match — otherwise every row the reader had passed through would keep a
    /// match dressed as current, and a page would end up with as many "current" matches
    /// as rows visited.
    ///
    /// Entering forwards lands on the new document's first match and backwards on its
    /// last, which is why the index handed to `set_current` going backwards is
    /// deliberately out of range: only the session knows how many matches it has.
    fn step(&self, forward: bool) -> Option<(Option<EntityId>, FindMatch)> {
        let mut slices = self.slices.borrow_mut();
        let n = slices.len();
        if n == 0 {
            return None;
        }
        let cursor = self.cursor.get().min(n - 1);

        // Still inside the document the reader is standing in?
        let here = &mut slices[cursor];
        let count = here.session.match_count();
        let at = here.session.current_index();
        let inside = count > 0
            && here.session.current_match().is_some()
            && if forward { at + 1 < count } else { at > 0 };
        if inside {
            let m = if forward {
                here.session.next_match()
            } else {
                here.session.prev_match()
            };
            let item = here.item;
            self.cursor.set(cursor);
            return m.map(|m| (item, m));
        }

        // Walk on to the next document that has a match, all the way round to this one
        // — which is how a single-document banner still wraps.
        for hop in 1..=n {
            let i = if forward {
                (cursor + hop) % n
            } else {
                (cursor + n - hop) % n
            };
            if slices[i].session.match_count() == 0 {
                continue;
            }
            if i != cursor {
                slices[cursor].session.set_current(None);
            }
            let entry = if forward { 0 } else { usize::MAX };
            let m = slices[i].session.set_current(Some(entry));
            let item = slices[i].item;
            self.cursor.set(i);
            return m.map(|m| (item, m));
        }
        None
    }

    /// Reveal the current match — Enter in the query field, after a fresh query
    /// left the first match current.
    pub fn reveal_current(&self, ctx: &mut EventContext) {
        let found = {
            let slices = self.slices.borrow();
            slices
                .get(self.cursor.get())
                .and_then(|s| s.session.current_match().map(|m| (s.item, m)))
        };
        self.reveal(ctx, found);
    }

    /// The total match count across every searched document.
    fn count_now(&self) -> usize {
        self.slices
            .borrow()
            .iter()
            .map(|s| s.session.match_count())
            .sum()
    }

    /// The item whose document holds the current match.
    fn cursor_item(&self) -> Option<EntityId> {
        let slices = self.slices.borrow();
        slices
            .get(self.cursor.get())
            .or_else(|| slices.first())
            .and_then(|s| s.item)
    }

    /// Put the cursor back on a document that still has a match, after an edit took
    /// the one the reader was standing on away.
    ///
    /// Restores the invariant the walk depends on: exactly one current match on the
    /// page. Without it a deletion could leave every session holding `None`, and the
    /// next Enter would re-enter from the top of the page rather than from here.
    fn normalise_cursor(&self) {
        let mut slices = self.slices.borrow_mut();
        if slices
            .get(self.cursor.get())
            .is_some_and(|s| s.session.current_match().is_some())
        {
            return;
        }
        let first = slices.iter().position(|s| s.session.match_count() > 0);
        for (i, slice) in slices.iter_mut().enumerate() {
            slice.session.set_current((Some(i) == first).then_some(0));
        }
        drop(slices);
        self.cursor.set(first.unwrap_or(0));
    }

    // ── in-editor replace ────────────────────────────────────────────────────

    /// Open the banner in **replace** mode (Ctrl+R / the replace toggle).
    ///
    /// Over a page of documents this opens a plain find: see the module note on why
    /// replacing across many documents is Search & Replace's job and not this one's.
    pub fn open_with_replace(&self) {
        if self.single.get() {
            self.replace_mode.set(true);
        }
        self.open();
    }

    /// The document-replace options (the current find options; default format policy).
    fn replace_options(&self) -> teksilo::text_document::ReplaceOptions {
        teksilo::text_document::ReplaceOptions::new(self.options())
    }

    /// How one occurrence is rewritten — verbatim, or preserving the case it found.
    /// The banner uses the untailored locale (the per-scene language is the
    /// project-wide replace's concern, not this in-place one).
    fn compute_replacement(&self, matched: &str) -> String {
        let replacement = self.replacement.get();
        if self.preserve_case.get() {
            teksilo::text_document::matching::preserve_case(
                matched,
                &replacement,
                teksilo::text_document::matching::FoldLocale::Root,
            )
        } else {
            replacement
        }
    }

    /// May this bar rewrite the document it is searching?
    ///
    /// Replacing goes through `TextDocument::find_and_replace`, which edits the
    /// document directly and never passes the editor's keyboard layer — so a
    /// writing game that has frozen this surface has to be asked about here, or
    /// Ctrl+R would be a way to delete prose that Backspace refuses to.
    ///
    /// **Finding is untouched.** Only the replace half is withheld: re-reading,
    /// stepping through matches and highlighting take nothing away, and a writer
    /// drafting forward still needs to find where they were.
    fn may_replace(&self) -> bool {
        self.handle.borrow().as_ref().is_none_or(|h| {
            h.command_filter()
                .accepts(teksilo::widgets::rich_text::EditCommandKind::Cut)
        })
    }

    /// The one document this banner is over, when it is over exactly one. `None` on a
    /// page — the gate every replace path below shares.
    fn only_doc(&self) -> Option<TextDocument> {
        let slices = self.slices.borrow();
        (slices.len() == 1).then(|| slices[0].doc.clone())
    }

    /// Replace the current match, then step onto the next — the find-bar
    /// convention. The edit is on the editor's own document (undoable with Ctrl+Z),
    /// applied under one lock via `find_and_replace` so no offset is carried
    /// across it.
    pub fn replace_current(&self, ctx: &mut EventContext) {
        if self.query.get().trim().is_empty() || !self.may_replace() {
            return;
        }
        let Some(doc) = self.only_doc() else { return };
        let query = self.query.get();
        let cur = self
            .slices
            .borrow()
            .first()
            .map_or(0, |s| s.session.current_index());
        let opts = self.replace_options();
        let me = self.clone();
        let _ = doc.find_and_replace(&query, &opts, move |matched, i| {
            (i == cur).then(|| me.compute_replacement(matched))
        });
        // The edit staled the session — re-derive (clamps `current` onto what is
        // now the next match) and reveal it.
        if let Some(slice) = self.slices.borrow_mut().first_mut() {
            slice.session.refresh_if_stale();
        }
        self.publish();
        self.reveal_current(ctx);
        ctx.request_frame();
    }

    /// Replace every match in this document at once (undoable as one action).
    pub fn replace_all(&self, ctx: &mut EventContext) {
        self.replace_all_now();
        // The matches are gone — re-run clears the highlights and the count.
        self.refresh_query();
        ctx.request_frame();
    }

    /// The document edit behind [`replace_all`](Self::replace_all), split out so it
    /// is testable without an `EventContext`. Returns how many occurrences changed.
    fn replace_all_now(&self) -> usize {
        let query = self.query.get();
        if query.trim().is_empty() || !self.may_replace() {
            return 0;
        }
        let Some(doc) = self.only_doc() else { return 0 };
        let opts = self.replace_options();
        let me = self.clone();
        doc.find_and_replace(&query, &opts, move |matched, _i| {
            Some(me.compute_replacement(matched))
        })
        .unwrap_or(0)
    }

    /// Per-frame: pick up a page whose rows have changed, and re-derive the matches
    /// of any document an edit staled (offsets moved), keeping the highlight boxes on
    /// the right characters. Cheap no-op otherwise.
    pub fn tick(&self) {
        if self.sync_documents() {
            self.apply_query();
            self.publish();
            return;
        }
        // Every document, not the first that answers: `any` would short-circuit and
        // leave the rest of the page's highlights on offsets an edit has moved.
        let mut changed = false;
        for slice in self.slices.borrow_mut().iter_mut() {
            changed |= slice.session.refresh_if_stale();
        }
        if changed {
            self.normalise_cursor();
            self.publish();
        }
    }

    /// Select + scroll a match into view through the editor showing it.
    ///
    /// Every editor over that document is selected in, and the *first that can* does the
    /// scrolling — see [`ResolveEditor`] for why "the first one" is the wrong answer.
    fn reveal(&self, ctx: &mut EventContext, found: Option<(Option<EntityId>, FindMatch)>) {
        let Some((item, m)) = found else { return };
        let (start, end) = (m.position, m.position + m.length);
        let mut revealed = false;
        for handle in self.handles_for(item) {
            handle.select_range(start, end);
            revealed |= !revealed && handle.reveal_range(ctx, start, end);
        }
    }

    /// Every editor showing `item`'s text.
    ///
    /// The single-document banner is handed its editor's handle by that editor's own
    /// build. A page cannot be: it builds one editor per row and none of them is "the
    /// tab's", so it asks the registry, which is the only thing that knows which
    /// mounted editor is showing which item.
    fn handles_for(&self, item: Option<EntityId>) -> Vec<EditorHandle> {
        match (&self.resolve_editor, item) {
            (Some(resolve), Some(item)) => resolve(item),
            _ => self.handle.borrow().clone().into_iter().collect(),
        }
    }

    /// Mirror the match count + the current match's ordinal on the page into the
    /// reactive signals.
    fn publish(&self) {
        let (count, current, item) = {
            let slices = self.slices.borrow();
            let count: usize = slices.iter().map(|s| s.session.match_count()).sum();
            let cursor = self.cursor.get();
            let standing = slices
                .get(cursor)
                .is_some_and(|s| s.session.current_match().is_some());
            let current = if count > 0 && standing {
                let before: usize = slices
                    .iter()
                    .take(cursor)
                    .map(|s| s.session.match_count())
                    .sum();
                before + slices[cursor].session.current_index() + 1
            } else {
                0
            };
            let item = slices
                .get(cursor)
                .or_else(|| slices.first())
                .and_then(|s| s.item);
            (count, current, item)
        };
        self.count.set(count);
        self.current.set(current);
        self.publish_to_lane(item, current);
    }

    /// Tell the margin lane what the writer is looking for.
    ///
    /// Through the arbiter rather than by the lane reading this view-model, because
    /// a `FindViewModel` lives on the tab that owns it and is reachable only as "the
    /// focused pane's active tab" — a lane on a stream row or a search preview has
    /// no route to it at all. Publishing here also makes "last search used wins"
    /// fall out: whichever of the two searches wrote most recently is what shows.
    ///
    /// This is the single funnel, called from [`publish`](Self::publish), so every
    /// path that changes what is being searched for reaches it: a keystroke in the
    /// field, a toggle, Next, Prev, the per-frame `tick` after an edit.
    ///
    /// `item` is the row the reader is standing **in**, not the tab: on a stream every
    /// row's hits are marked (the provider re-runs the query per document), and naming
    /// the cursor's row is what makes exactly one of them read as current.
    fn publish_to_lane(&self, item: Option<EntityId>, current: usize) {
        use crate::margin_lane::{LaneQuery, LaneQuerySource};
        let Some(item) = item else {
            return;
        };
        let source = LaneQuerySource::Editor(item);
        let text = self.query.get();
        if !self.visible.get() || text.is_empty() {
            crate::margin_lane::clear_active_query_from(source);
            return;
        }
        crate::margin_lane::set_active_query(Some(LaneQuery {
            text,
            case_sensitive: self.case_sensitive.get(),
            whole_word: self.whole_word.get(),
            // The banner has no folding switch, and leaving it off is what it
            // already does: `FindViewModel::options` folds too.
            diacritic_sensitive: false,
            source,
            // The ordinal **within the document the reader is standing in**, and only
            // while there is one: with no matches the banner shows "0 of 0", and
            // marking a current hit would be inventing a position.
            current: (current > 0).then(|| {
                self.slices
                    .borrow()
                    .get(self.cursor.get())
                    .map_or(0, |s| s.session.current_index())
            }),
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(text: &str) -> TextDocument {
        let d = TextDocument::new();
        d.set_plain_text(text).unwrap();
        d
    }

    fn fmt() -> HighlightFormat {
        HighlightFormat::default()
    }

    /// The rows a test page is showing, editable so a test can add one under a banner
    /// that is already open — which is what a split does.
    type PageRows = Rc<RefCell<Vec<(EntityId, TextDocument)>>>;

    /// A banner over a page of rows, each named by its item id.
    fn page(rows: &[(EntityId, &str)]) -> (FindViewModel, PageRows) {
        let docs: PageRows = Rc::new(RefCell::new(
            rows.iter().map(|(id, text)| (*id, doc(text))).collect(),
        ));
        let source = docs.clone();
        let vm = FindViewModel::over_page(
            Rc::new(move || source.borrow().clone()),
            Rc::new(|_| Vec::new()),
        );
        vm.ensure_session(fmt(), fmt());
        (vm, docs)
    }

    /// Which row the reader is standing in, and where in it — the pair every assertion
    /// about the walk is really about.
    fn standing(vm: &FindViewModel) -> Option<(EntityId, usize)> {
        let slices = vm.slices.borrow();
        let slice = slices.get(vm.cursor.get())?;
        slice.session.current_match()?;
        Some((slice.item?, slice.session.current_index()))
    }

    /// How many of the page's documents claim to be standing on a match. The invariant
    /// the whole walk rests on: never more than one.
    fn currents(vm: &FindViewModel) -> usize {
        vm.slices
            .borrow()
            .iter()
            .filter(|s| s.session.current_match().is_some())
            .count()
    }

    /// **The banner's hits reach the margin lane**, which cannot reach the banner.
    ///
    /// A `FindViewModel` lives on the tab that owns it and is findable only as "the
    /// focused pane's active tab"; a lane on a stream row or a preview is not that
    /// tab. So the query travels the other way, through the arbiter, and every path
    /// that changes it has to publish — which is why the publication hangs off
    /// `publish` rather than off `refresh_query` alone.
    #[test]
    fn what_the_banner_searches_for_reaches_the_lanes_arbiter() {
        use crate::margin_lane::{LaneQuerySource, active_query, set_active_query};
        set_active_query(None);

        let vm = FindViewModel::new(doc("the cat and the hat and the mat")).for_item(7);
        vm.ensure_session(fmt(), fmt());
        vm.open();
        vm.query_signal().set("the".into());
        vm.refresh_query();

        let q = active_query().get().expect("the lane was told");
        assert_eq!(q.text, "the");
        assert_eq!(q.source, LaneQuerySource::Editor(7));
        assert_eq!(
            q.current_in(7),
            Some(0),
            "the first match is the one the writer is on"
        );

        // Closing takes its own marks off the strip.
        vm.close();
        assert!(
            active_query().get().is_none(),
            "a closed banner must not leave its hits on the lane"
        );
        set_active_query(None);
    }

    /// A banner with no item behind it publishes nothing: a lane cannot attribute
    /// hits to a document that was never named.
    #[test]
    fn an_unnamed_banner_publishes_nothing() {
        use crate::margin_lane::{LaneQuery, LaneQuerySource, active_query, set_active_query};
        set_active_query(Some(LaneQuery {
            text: "ferry".into(),
            case_sensitive: false,
            whole_word: false,
            diacritic_sensitive: false,
            source: LaneQuerySource::Project,
            current: None,
        }));

        let vm = FindViewModel::new(doc("the cat"));
        vm.ensure_session(fmt(), fmt());
        vm.open();
        vm.query_signal().set("cat".into());
        vm.refresh_query();

        assert!(
            active_query().get().is_some_and(|q| q.text == "ferry"),
            "an unnamed banner must not overwrite what is on the lane"
        );
        set_active_query(None);
    }

    #[test]
    fn a_query_reports_its_match_count_and_current_index() {
        let vm = FindViewModel::new(doc("the cat and the hat and the mat"));
        vm.ensure_session(fmt(), fmt());
        vm.query_signal().set("the".into());
        vm.refresh_query();
        assert_eq!(vm.count_signal().get(), 3, "three `the`s");
        assert_eq!(
            vm.current_signal().get(),
            1,
            "first match is current (1-based)"
        );
    }

    #[test]
    fn a_missing_handle_does_not_stop_next_from_advancing() {
        // No editor handle attached (the banner opened before the editor built):
        // next/prev must still move the current match — they just won't scroll.
        let vm = FindViewModel::new(doc("a b a b a"));
        vm.ensure_session(fmt(), fmt());
        vm.query_signal().set("a".into());
        vm.refresh_query();
        assert_eq!(vm.current_signal().get(), 1);
        // `next` needs an EventContext for the (skipped) reveal; the count/index
        // update happens before that, in `step`, so drive that half directly rather
        // than fabricating a context here.
        vm.step(true);
        vm.publish();
        assert_eq!(vm.current_signal().get(), 2);
        assert_eq!(vm.count_signal().get(), 3);
    }

    #[test]
    fn closing_clears_the_count() {
        let vm = FindViewModel::new(doc("hello hello"));
        vm.ensure_session(fmt(), fmt());
        vm.query_signal().set("hello".into());
        vm.refresh_query();
        assert_eq!(vm.count_signal().get(), 2);
        vm.close();
        assert!(!vm.visible_signal().get(), "closing hides the banner");
        assert_eq!(
            vm.count_signal().get(),
            0,
            "closing clears the highlighting + count"
        );
    }

    #[test]
    fn replace_all_rewrites_every_match_in_the_document() {
        let vm = FindViewModel::new(doc("the cat and the hat and the mat"));
        vm.ensure_session(fmt(), fmt());
        vm.query_signal().set("the".into());
        vm.refresh_query();
        assert_eq!(vm.count_signal().get(), 3);

        vm.replacement_signal().set("THE".into()); // distinct so we can re-find it
        vm.preserve_case_signal().set(false);
        assert_eq!(vm.replace_all_now(), 3, "all three replaced");

        // No more lowercase "the" (case-sensitive off means "THE" still matches
        // "the"… so verify against a case-sensitive re-scan instead).
        vm.case_sensitive_signal().set(true);
        vm.query_signal().set("the".into());
        vm.refresh_query();
        assert_eq!(vm.count_signal().get(), 0, "the original text is gone");
        vm.query_signal().set("THE".into());
        vm.refresh_query();
        assert_eq!(vm.count_signal().get(), 3, "and the replacement is there");
    }

    #[test]
    fn preserve_case_keeps_each_occurrences_case() {
        let vm = FindViewModel::new(doc("elena and ELENA and Elena"));
        vm.ensure_session(fmt(), fmt());
        vm.query_signal().set("elena".into()); // case-insensitive → all three
        vm.replacement_signal().set("maria".into());
        vm.preserve_case_signal().set(true);
        assert_eq!(vm.replace_all_now(), 3);

        // Each kept its own case: maria / MARIA / Maria — verify by case-sensitive find.
        vm.case_sensitive_signal().set(true);
        for (form, n) in [("maria", 1), ("MARIA", 1), ("Maria", 1)] {
            vm.query_signal().set(form.into());
            vm.refresh_query();
            assert_eq!(vm.count_signal().get(), n, "expected {form}");
        }
    }

    #[test]
    fn whole_word_narrows_the_matches() {
        let vm = FindViewModel::new(doc("cat category cat"));
        vm.ensure_session(fmt(), fmt());
        vm.query_signal().set("cat".into());
        vm.refresh_query();
        assert_eq!(
            vm.count_signal().get(),
            3,
            "substring: matches inside `category` too"
        );
        vm.whole_word_signal().set(true);
        vm.refresh_query();
        assert_eq!(
            vm.count_signal().get(),
            2,
            "whole-word drops the one inside `category`"
        );
    }

    // ── a page of documents ─────────────────────────────────────────────────

    /// **A stream reads as one manuscript, so it counts as one.** "N of M" spans every
    /// row of the page, not the row the reader happens to be in.
    #[test]
    fn a_page_counts_every_rows_matches_as_one_run() {
        let (vm, _) = page(&[(1, "a ferry"), (2, "no boats here"), (3, "ferry, ferry")]);
        vm.query_signal().set("ferry".into());
        vm.refresh_query();

        assert_eq!(vm.count_signal().get(), 3, "one in row 1, two in row 3");
        assert_eq!(vm.current_signal().get(), 1, "standing on the first");
        assert_eq!(standing(&vm), Some((1, 0)));
        assert!(!vm.single_document_signal().get(), "three documents");
    }

    /// Next walks off the end of a row into the next one that has a hit — skipping the
    /// row with none — and wraps round the page. Exactly one row is standing on a match
    /// at every step, which is the thing that goes wrong if a row is left behind with a
    /// current match of its own.
    #[test]
    fn next_crosses_into_the_following_row_and_wraps() {
        let (vm, _) = page(&[(1, "a ferry"), (2, "no boats here"), (3, "ferry, ferry")]);
        vm.query_signal().set("ferry".into());
        vm.refresh_query();

        vm.step(true);
        vm.publish();
        assert_eq!(standing(&vm), Some((3, 0)), "row 2 has no hit to stop at");
        assert_eq!(vm.current_signal().get(), 2, "second of three on the page");
        assert_eq!(currents(&vm), 1, "row 1 gave up its current match");

        vm.step(true);
        vm.publish();
        assert_eq!(standing(&vm), Some((3, 1)), "still inside row 3");
        assert_eq!(vm.current_signal().get(), 3);

        vm.step(true);
        vm.publish();
        assert_eq!(
            standing(&vm),
            Some((1, 0)),
            "wrapped to the top of the page"
        );
        assert_eq!(vm.current_signal().get(), 1);
        assert_eq!(currents(&vm), 1);
    }

    /// Backwards is the mirror: stepping out of a row's first match arrives at the
    /// **last** match of the row above, not its first.
    #[test]
    fn prev_enters_the_row_above_at_its_last_match() {
        let (vm, _) = page(&[(1, "ferry, ferry"), (2, "one ferry")]);
        vm.query_signal().set("ferry".into());
        vm.refresh_query();
        assert_eq!(standing(&vm), Some((1, 0)));

        vm.step(false);
        vm.publish();
        assert_eq!(standing(&vm), Some((2, 0)), "wrapped back to the last row");
        assert_eq!(vm.current_signal().get(), 3, "third of three");

        vm.step(false);
        vm.publish();
        assert_eq!(standing(&vm), Some((1, 1)), "row 1's *last* match");
        assert_eq!(vm.current_signal().get(), 2);
        assert_eq!(currents(&vm), 1);
    }

    /// The lane is told **which row** the reader is standing in, and the ordinal within
    /// that row. That pairing is what makes one mark on the whole strip read as current
    /// while every other row still shows its hits: the provider re-runs the query per
    /// document and asks `current_in` for its own item.
    #[test]
    fn the_lane_is_told_the_row_the_reader_is_standing_in() {
        use crate::margin_lane::{active_query, set_active_query};
        set_active_query(None);

        let (vm, _) = page(&[(1, "a ferry"), (2, "ferry, ferry")]);
        vm.open();
        vm.query_signal().set("ferry".into());
        vm.refresh_query();

        let q = active_query().get().expect("published");
        assert_eq!(q.current_in(1), Some(0), "standing in row 1");
        assert_eq!(
            q.current_in(2),
            None,
            "row 2 shows hits, none of them current"
        );

        vm.step(true);
        vm.publish();
        let q = active_query().get().expect("published");
        assert_eq!(q.current_in(1), None, "row 1 no longer holds the cursor");
        assert_eq!(q.current_in(2), Some(0), "row 2 does, at its first hit");

        vm.close();
        set_active_query(None);
    }

    /// A row split while the banner is open joins the search without the writer having
    /// to close and reopen it — the reason the documents are a closure rather than a
    /// list captured at build.
    #[test]
    fn a_row_added_under_an_open_banner_joins_the_search() {
        let (vm, rows) = page(&[(1, "a ferry")]);
        vm.open();
        vm.query_signal().set("ferry".into());
        vm.refresh_query();
        assert_eq!(vm.count_signal().get(), 1);
        assert!(vm.single_document_signal().get(), "one row so far");

        rows.borrow_mut().push((2, doc("the ferry again")));
        vm.tick();

        assert_eq!(vm.count_signal().get(), 2, "the new row was searched too");
        assert!(!vm.single_document_signal().get());
    }

    /// **Replacing is withheld over a page.** Rewriting across many documents is Search
    /// & Replace's job, with its preview and its scope; an in-editor Replace All that
    /// quietly edited every row would leave one thing to undo per row.
    #[test]
    fn replacing_is_withheld_while_the_banner_is_over_a_page() {
        let (vm, rows) = page(&[(1, "a ferry"), (2, "another ferry")]);
        vm.query_signal().set("ferry".into());
        vm.refresh_query();
        vm.replacement_signal().set("barge".into());

        assert_eq!(vm.replace_all_now(), 0, "nothing rewritten");
        for (_, doc) in rows.borrow().iter() {
            assert!(
                doc.to_plain_text().unwrap().contains("ferry"),
                "the prose is untouched"
            );
        }

        vm.open_with_replace();
        assert!(
            !vm.replace_mode_signal().get(),
            "and the replace row is not even disclosed"
        );
    }

    /// Ctrl+F reaches a container tab's banner from the Corkboard and the Overview too,
    /// where the page in front of the reader is showing no prose at all. An empty
    /// banner there could only ever say "0 of 0", so it does not open.
    #[test]
    fn a_banner_over_a_page_with_nothing_on_it_does_not_open() {
        let (vm, _) = page(&[]);
        assert!(!vm.has_documents());
        vm.open();
        assert!(
            !vm.visible_signal().get(),
            "nothing to search, nothing to show"
        );

        let single = FindViewModel::new(doc("prose"));
        single.open();
        assert!(
            single.visible_signal().get(),
            "a single-document banner always has its document"
        );
    }

    /// An edit that deletes the match the reader was standing on must leave the page
    /// with a current match somewhere, not none — otherwise the next step re-enters
    /// from the top rather than carrying on.
    #[test]
    fn an_edit_that_removes_the_current_match_moves_the_cursor_rather_than_losing_it() {
        let (vm, rows) = page(&[(1, "a ferry"), (2, "ferry, ferry")]);
        vm.query_signal().set("ferry".into());
        vm.refresh_query();
        assert_eq!(standing(&vm), Some((1, 0)));

        rows.borrow()[0].1.set_plain_text("a boat").unwrap();
        vm.tick();

        assert_eq!(vm.count_signal().get(), 2, "only row 2's hits are left");
        assert_eq!(standing(&vm), Some((2, 0)), "the cursor moved to them");
        assert_eq!(currents(&vm), 1);
    }
}
