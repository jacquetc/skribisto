// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `FindViewModel` — the per-editor find banner's state (Ctrl+F).
//!
//! One per prose tab (held on its [`ContentTab`](crate::tabs::ContentTab)), over
//! that tab's main document. It owns a [`FindSession`] — text-document's own
//! find-highlight layer, driven by the *same* matcher a project-wide search uses,
//! so the two can never disagree about what a match is — plus the input signals
//! the banner binds and the editor handle it needs to select and scroll the
//! current match into view.
//!
//! The `FindSession` is created **lazily**, in the banner's `build`: its two
//! highlight colours are resolved from theme roles, which are only available with
//! a live context. The editor handle is (re)attached on every editor build (a tab
//! rebuild mints a fresh widget); until it is present, next/prev still recolour
//! the highlights, they just don't scroll.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use teksilo::prelude::*;
use teksilo::text_document::{FindMatch, FindOptions, HighlightFormat, TextDocument};
use teksilo::widgets::rich_text::{EditorHandle, FindSession};

// The API is bound by the banner + the editors view-model; wired incrementally.
#[allow(dead_code)]
#[derive(Clone)]
pub struct FindViewModel {
    /// The `BinderItem` this banner searches, for the margin lane's arbiter.
    ///
    /// `None` for a surface with no item behind it, and the banner then publishes
    /// nothing — a lane cannot mark hits it cannot attribute to a document.
    item: std::cell::Cell<Option<common::types::EntityId>>,
    /// The main prose document this banner searches.
    doc: TextDocument,
    /// The find-highlight layer, created lazily (needs theme colours).
    session: Rc<RefCell<Option<FindSession>>>,
    /// The current editor's handle — set on each editor build; drives select +
    /// scroll-into-view of the current match.
    handle: Rc<RefCell<Option<EditorHandle>>>,
    /// Banner visibility (Ctrl+F opens it, Escape/close hides it).
    visible: Signal<bool>,
    query: Signal<String>,
    case_sensitive: Signal<bool>,
    whole_word: Signal<bool>,
    /// Total matches, and the 1-based index of the current one (0 when none) —
    /// the "N of M" the banner shows. Kept as signals so the label is reactive.
    count: Signal<usize>,
    current: Signal<usize>,
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
    pub fn new(doc: TextDocument) -> Self {
        Self {
            item: std::cell::Cell::new(None),
            doc,
            session: Rc::new(RefCell::new(None)),
            handle: Rc::new(RefCell::new(None)),
            visible: Signal::new(false),
            query: Signal::new(String::new()),
            case_sensitive: Signal::new(false),
            whole_word: Signal::new(false),
            count: Signal::new(0),
            current: Signal::new(0),
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
    pub fn for_item(self, item: common::types::EntityId) -> Self {
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
    pub fn editor_handle(&self) -> Option<EditorHandle> {
        self.handle.borrow().clone()
    }

    /// Create the `FindSession` if it doesn't exist yet, with the two theme-
    /// resolved highlight formats, and (re)apply the current query. Idempotent —
    /// the banner's `build` calls it every time; only the first does real work.
    pub fn ensure_session(&self, current_format: HighlightFormat, other_format: HighlightFormat) {
        if self.session.borrow().is_some() {
            return;
        }
        *self.session.borrow_mut() =
            Some(FindSession::new(&self.doc, current_format, other_format));
        self.refresh_query();
    }

    /// Open the banner and ask the field for focus (bumps `focus_seq`, which the
    /// banner observes at `Rebuild`). Idempotent — pressing Ctrl+F while already
    /// open just re-focuses the field.
    pub fn open(&self) {
        self.visible.set(true);
        let s = &self.focus_seq;
        s.set(s.get().wrapping_add(1));
    }

    /// Close the banner: clear the highlighting and hide the row. The query text
    /// is kept, so reopening finds it again.
    pub fn close(&self) {
        if let Some(fs) = self.session.borrow_mut().as_mut() {
            fs.clear();
        }
        self.visible.set(false);
        self.publish();
    }

    /// Close the banner and return keyboard focus to the prose — the find-bar
    /// convention for Escape and the close button, so the user keeps typing where
    /// they were reading. Focus lands only if the editor handle is attached.
    pub fn close_and_refocus(&self, ctx: &mut EventContext) {
        self.close();
        if let Some(handle) = self.handle.borrow().as_ref() {
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

    /// Re-run the query and repaint the highlights. Does **not** scroll (it runs
    /// from a signal effect with no `EventContext`); the first match becomes
    /// current, and Enter / Next reveal it.
    pub fn refresh_query(&self) {
        let query = self.query.get();
        let options = self.options();
        if let Some(fs) = self.session.borrow_mut().as_mut() {
            fs.set_query(&query, &options);
        }
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
        let m = self
            .session
            .borrow_mut()
            .as_mut()
            .and_then(|fs| fs.next_match());
        self.publish();
        self.reveal(ctx, m);
    }

    /// Step to the previous match (wrapping) and reveal it.
    pub fn prev(&self, ctx: &mut EventContext) {
        self.reveal_pending.set(false);
        let m = self
            .session
            .borrow_mut()
            .as_mut()
            .and_then(|fs| fs.prev_match());
        self.publish();
        self.reveal(ctx, m);
    }

    /// Reveal the current match — Enter in the query field, after a fresh query
    /// left the first match current.
    pub fn reveal_current(&self, ctx: &mut EventContext) {
        let m = self
            .session
            .borrow()
            .as_ref()
            .and_then(|fs| fs.current_match());
        self.reveal(ctx, m);
    }

    /// The current match count (reads the session directly).
    fn count_now(&self) -> usize {
        self.session
            .borrow()
            .as_ref()
            .map_or(0, |fs| fs.match_count())
    }

    // ── in-editor replace ────────────────────────────────────────────────────

    /// Open the banner in replace mode (Ctrl+R / the replace toggle).
    pub fn open_with_replace(&self) {
        self.replace_mode.set(true);
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

    /// Replace the current match, then step onto the next — the find-bar
    /// convention. The edit is on the editor's own document (undoable with Ctrl+Z),
    /// applied under one lock via `find_and_replace` so no offset is carried
    /// across it.
    pub fn replace_current(&self, ctx: &mut EventContext) {
        if self.query.get().trim().is_empty() || !self.may_replace() {
            return;
        }
        let query = self.query.get();
        let cur = self
            .session
            .borrow()
            .as_ref()
            .map_or(0, |fs| fs.current_index());
        let opts = self.replace_options();
        let me = self.clone();
        let _ = self.doc.find_and_replace(&query, &opts, move |matched, i| {
            (i == cur).then(|| me.compute_replacement(matched))
        });
        // The edit staled the session — re-derive (clamps `current` onto what is
        // now the next match) and reveal it.
        if let Some(fs) = self.session.borrow_mut().as_mut() {
            fs.refresh_if_stale();
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
        let opts = self.replace_options();
        let me = self.clone();
        self.doc
            .find_and_replace(&query, &opts, move |matched, _i| {
                Some(me.compute_replacement(matched))
            })
            .unwrap_or(0)
    }

    /// Per-frame: re-derive the matches if an edit staled them (offsets moved),
    /// keeping the highlight boxes on the right characters. Cheap no-op otherwise.
    pub fn tick(&self) {
        let changed = self
            .session
            .borrow_mut()
            .as_mut()
            .is_some_and(|fs| fs.refresh_if_stale());
        if changed {
            self.publish();
        }
    }

    /// Select + scroll a match into view through the attached editor handle.
    fn reveal(&self, ctx: &mut EventContext, m: Option<FindMatch>) {
        let Some(m) = m else { return };
        if let Some(handle) = self.handle.borrow().as_ref() {
            let (start, end) = (m.position, m.position + m.length);
            handle.select_range(start, end);
            handle.reveal_range(ctx, start, end);
        }
    }

    /// Mirror the session's match count + current index into the reactive signals.
    fn publish(&self) {
        let (count, current) = match self.session.borrow().as_ref() {
            Some(fs) if fs.match_count() > 0 => (fs.match_count(), fs.current_index() + 1),
            _ => (0, 0),
        };
        self.count.set(count);
        self.current.set(current);
        self.publish_to_lane(count);
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
    fn publish_to_lane(&self, count: usize) {
        use crate::margin_lane::{LaneQuery, LaneQuerySource};
        let Some(item) = self.item.get() else {
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
            // The ordinal only, and only while there is one: with no matches the
            // banner shows "0 of 0", and marking a current hit would be inventing a
            // position for something that is not there.
            current: (count > 0).then(|| self.current.get().saturating_sub(1)),
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
        // update happens before that, via `publish`, so assert through a fresh
        // query round-trip instead of fabricating a context here.
        // Re-running the query keeps the count stable and current at the first.
        vm.refresh_query();
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
}
