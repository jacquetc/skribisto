// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The in-editor comment highlight layer: one host-driven **range session** per
//! document, plus the live anchor tracking that keeps it correct as the writer types.
//!
//! ## Where this lives, and why it matters
//!
//! The session belongs to the **`OpenDoc`**, not to a tab or an editor widget —
//! the same placement `FindSession`, `CaretHighlightSession` and `SpellSession`
//! all use, for the same reason: an `OpenDoc` is shared by every simultaneous view
//! of one `Content` (its own tab, a split pane, a stream row, a corkboard card).
//! A session owned by a widget would paint in whichever view happened to create it
//! and nowhere else.
//!
//! ## An ochre wash, following LibreOffice
//!
//! A commented range is painted with an **ochre background**, the convention every
//! word processor with margin comments uses (LibreOffice, Word, Google Docs). It
//! reads as "this text is annotated" at a glance in a way an underline does not —
//! an underline competes with the spell squiggle and with links, and a writer
//! scanning a page for their own notes should not have to tell three underlines
//! apart.
//!
//! `HighlightFormat` merges **field by field, higher priority winning per field**,
//! and `CaretHighlightSession` also claims `background_color` for its ambient
//! "you are writing here" band. That is a real contention, resolved by priority
//! rather than by dodging the field: comments register **above** the caret band
//! (which sits at `-1000`), so a commented span keeps its ochre even while the
//! caret rests in that sentence. The band is ambient and the comment is a fact
//! about the text, so the fact wins.
//!
//! Paragraph comments paint **nothing in the text**. Their marker is a bracket in
//! the margin (`comments/margin.rs`) spanning the block — washing a whole paragraph
//! ochre would drown the page, and the bracket says "this block, as a unit" more
//! precisely than a wash ever did.
//!
//! ## Overlapping comments
//!
//! Two overlapping range comments cannot be told apart by colour, and this does
//! not try: `HighlightFormat` is a closed struct with no notion of "N things are
//! here". Instead the host flattens overlapping anchors into **disjoint painted
//! segments** before pushing them, and a segment covered by more than one thread
//! gets a deeper wash. The wash's job is reduced to "something is annotated here";
//! disambiguation is the margin's job, where each thread has its own card.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use bastyde::text_document::{
    Color, DocumentEvent, HighlightFormat, RangeHighlight, SessionId, Subscription, TextDocument,
};

use crate::comments::anchor;

/// Comment highlights sit **below** find matches and spell squiggles (both
/// actionable-right-now feedback, and both on other fields anyway) but **above**
/// the caret band at `-1000`, with which they genuinely contend for
/// `background_color`. A commented span must keep its ochre while the caret rests
/// in that sentence: the band is ambient, the comment is a fact about the text.
pub const COMMENT_HIGHLIGHT_PRIORITY: i32 = -500;

/// One comment's live extent within a document, as the session tracks it.
///
/// This is the *live* half of an anchor. The persisted half (the quote selector)
/// is deliberately not touched while typing — it stays frozen as the ground truth
/// for the next load's re-anchor pass, so an edit cannot corrupt the very thing
/// that would rescue the comment later.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveAnchor {
    pub comment_id: u64,
    pub start: usize,
    pub end: usize,
    pub is_paragraph: bool,
    pub resolved: bool,
}

impl LiveAnchor {
    fn is_empty(&self) -> bool {
        self.end <= self.start
    }
}

/// Flatten possibly-overlapping anchors into disjoint `[start, end)` segments,
/// each carrying how many threads cover it.
///
/// A sweep over the distinct boundaries: every anchor edge starts a new segment,
/// so no output segment is ever partially covered. Resolved threads are excluded
/// upstream, not here.
pub fn flatten(anchors: &[LiveAnchor]) -> Vec<(usize, usize, usize)> {
    let mut edges: Vec<usize> = Vec::with_capacity(anchors.len() * 2);
    for a in anchors {
        if a.is_empty() {
            continue;
        }
        edges.push(a.start);
        edges.push(a.end);
    }
    edges.sort_unstable();
    edges.dedup();

    let mut out = Vec::new();
    for pair in edges.windows(2) {
        let (s, e) = (pair[0], pair[1]);
        let depth = anchors
            .iter()
            .filter(|a| !a.is_empty() && a.start <= s && a.end >= e)
            .count();
        if depth > 0 {
            out.push((s, e, depth));
        }
    }
    out
}

/// Paint for one flattened segment: the ochre wash, deepened where threads stack.
fn format_for(depth: usize, base: Color, stacked: Color) -> HighlightFormat {
    HighlightFormat {
        background_color: Some(if depth > 1 { stacked } else { base }),
        ..HighlightFormat::default()
    }
}

/// A per-document comment highlight layer.
///
/// Mirrors `SpellSession`'s lifecycle exactly — a held [`Subscription`], a `dirty`
/// flag drained by a per-frame [`tick`](Self::tick), and a [`Drop`] that retires
/// the layer (dropping the subscription stops callbacks but does **not** remove
/// the session).
pub struct CommentHighlightSession {
    doc: TextDocument,
    session: SessionId,
    /// Set from the document's `on_change` callback, which is `Send + Sync` —
    /// hence an `Arc<AtomicBool>` rather than a `Cell`, exactly like `FindSession`.
    dirty: Arc<AtomicBool>,
    /// Pending edits to fold into the live anchors, queued by the change callback
    /// and drained on `tick` so a burst of keystrokes costs one recompute a frame.
    pending: Arc<std::sync::Mutex<Vec<(usize, usize, usize)>>>,
    anchors: RefCell<Vec<LiveAnchor>>,
    last: RefCell<Vec<RangeHighlight>>,
    /// Comments whose text was deleted outright while typing. Reported so the UI
    /// can mark them orphaned *immediately* rather than at the next load.
    newly_orphaned: RefCell<Vec<u64>>,
    base: Cell<Color>,
    stacked: Cell<Color>,
    active: Cell<bool>,
    _sub: Subscription,
}

impl CommentHighlightSession {
    pub fn new(doc: &TextDocument) -> Rc<Self> {
        let session = doc.add_range_session_with_priority(COMMENT_HIGHLIGHT_PRIORITY);
        let dirty = Arc::new(AtomicBool::new(false));
        let pending: Arc<std::sync::Mutex<Vec<(usize, usize, usize)>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));

        let sub = {
            let dirty = dirty.clone();
            let pending = pending.clone();
            doc.on_change(move |event| match event {
                // The exact delta the shift rule needs. text-document emits this on
                // undo and redo too (computed as a real text diff), so undo/redo
                // tracking comes along for free rather than needing its own path.
                DocumentEvent::ContentsChanged {
                    position,
                    chars_removed,
                    chars_added,
                    ..
                } => {
                    if let Ok(mut q) = pending.lock() {
                        q.push((position, chars_removed, chars_added));
                    }
                    dirty.store(true, Ordering::Relaxed);
                }
                // A wholesale replacement invalidates every live offset; the owner
                // re-seeds from a fresh re-anchor pass.
                DocumentEvent::DocumentReset => dirty.store(true, Ordering::Relaxed),
                // Never react to `HighlightPaintChanged`: our own `set_session_ranges`
                // emits it, and reacting would self-loop. Same filter SpellSession uses.
                _ => {}
            })
        };

        Rc::new(Self {
            doc: doc.clone(),
            session,
            dirty,
            pending,
            anchors: RefCell::new(Vec::new()),
            last: RefCell::new(Vec::new()),
            newly_orphaned: RefCell::new(Vec::new()),
            // Overwritten by `CommentBinding::push_live` from the one shared
            // ochre before anything paints. These defaults exist only so a
            // session built with no view-model behind it (a widget test) still
            // has a valid colour rather than a garbage one.
            base: Cell::new(Color::rgb(199, 155, 61)),
            stacked: Cell::new(Color::rgb(158, 115, 33)),
            active: Cell::new(true),
            _sub: sub,
        })
    }

    /// Theme colours for the underline (and the paragraph wash). Resolved from
    /// semantic theme roles by the caller — never hardcoded here.
    pub fn set_colors(&self, base: Color, stacked: Color) {
        self.base.set(base);
        self.stacked.set(stacked);
        self.repaint();
    }

    /// Replace the tracked set — called after a re-anchor pass, and whenever the
    /// comment list itself changes (create / delete / resolve).
    pub fn set_anchors(&self, anchors: Vec<LiveAnchor>) {
        *self.anchors.borrow_mut() = anchors;
        self.refresh_paragraph_extents();
        self.repaint();
    }

    pub fn anchors(&self) -> Vec<LiveAnchor> {
        self.anchors.borrow().clone()
    }

    /// Hide the layer without dropping it — a pane that is not currently shown, or
    /// Tools ▸ Comments switched off.
    ///
    /// **Not** a drop: `Drop` retires the highlight layer and takes the live-anchor
    /// tracking with it, so a hidden-then-shown document would come back with anchors
    /// that never saw the edits made in between — and would surface orphans that are
    /// not orphaned. The layer stays, keeps folding edits into its anchors, and simply
    /// stops painting.
    ///
    /// Repaints on **either** edge. Going inactive has to push the empty set now, or
    /// the last ranges it drew stay on screen until the next keystroke happens to
    /// repaint them — a "hide" that only takes effect once you type is not a hide.
    pub fn set_active(&self, active: bool) {
        let was = self.active.replace(active);
        if active == was {
            return;
        }
        if active {
            self.dirty.store(true, Ordering::Relaxed);
        }
        self.repaint();
    }

    /// Whether this layer is currently painting.
    pub fn is_active(&self) -> bool {
        self.active.get()
    }

    /// Drain queued edits and repaint. Called once per frame by the owner.
    ///
    /// Returns the ids of comments whose text was deleted outright by those edits,
    /// so the caller can flag them orphaned on the spot — the live half of the
    /// orphan contract, which otherwise would not surface until the next load.
    pub fn tick(&self) -> Vec<u64> {
        if !self.active.get() {
            // Leave `dirty` set so a hidden pane still owes its catch-up.
            return Vec::new();
        }
        if !self.dirty.swap(false, Ordering::Relaxed) {
            return Vec::new();
        }
        let edits: Vec<(usize, usize, usize)> = match self.pending.lock() {
            Ok(mut q) => std::mem::take(&mut *q),
            Err(_) => Vec::new(),
        };
        if !edits.is_empty() {
            self.apply_edits(&edits);
        }
        self.repaint();
        std::mem::take(&mut *self.newly_orphaned.borrow_mut())
    }

    /// Fold a batch of edits into every live anchor, keeping both edges exclusive.
    ///
    /// A **paragraph** anchor is then re-derived from the document's current block
    /// boundaries rather than left where the shift put it. The two rules answer
    /// different questions: a range comment marks *these characters*, so it must
    /// shrink when they are deleted; a paragraph comment marks *this block*, so it
    /// has to grow and shrink with the block. Shifting it like a range leaves the
    /// bracket frozen at yesterday's height while the writer adds sentences to the
    /// paragraph it is supposed to be marking.
    fn apply_edits(&self, edits: &[(usize, usize, usize)]) {
        let mut anchors = self.anchors.borrow_mut();
        let mut orphaned = Vec::new();
        for a in anchors.iter_mut() {
            let was_live = !a.is_empty();
            for &(pos, removed, added) in edits {
                let (s, e) = anchor::shift_range(a.start, a.end, pos, removed, added);
                a.start = s;
                a.end = e;
            }
            if was_live && a.is_empty() {
                orphaned.push(a.comment_id);
            }
        }
        drop(anchors);
        self.refresh_paragraph_extents();
        self.newly_orphaned.borrow_mut().extend(orphaned);
    }

    /// Re-derive every paragraph anchor's extent from the live block boundaries,
    /// preserving how many blocks it covers.
    fn refresh_paragraph_extents(&self) {
        let mut anchors = self.anchors.borrow_mut();
        if !anchors.iter().any(|a| a.is_paragraph) {
            return;
        }
        let starts: Vec<usize> = self.doc.blocks().into_iter().map(|b| b.position()).collect();
        if starts.is_empty() {
            return;
        }
        let total = self.doc.to_plain_text().map(|t| t.chars().count()).unwrap_or(0);
        for a in anchors.iter_mut().filter(|a| a.is_paragraph) {
            if a.is_empty() {
                continue;
            }
            let first = anchor::block_of(&starts, a.start);
            let last = anchor::block_of(&starts, a.end.saturating_sub(1)).max(first);
            let (s, e) = anchor::block_extent(&starts, total, first, last);
            a.start = s;
            a.end = e;
        }
    }

    /// Rebuild the pushed ranges from the current anchors.
    ///
    /// An inactive layer paints nothing at all — the anchors are still tracked, they
    /// are simply not shown. Filtering here rather than at every call site is what
    /// makes "hidden" hold for `set_anchors` and `set_colors` too, which would
    /// otherwise repaint a hidden layer back into view.
    fn repaint(&self) {
        if !self.active.get() {
            if !self.last.borrow().is_empty() {
                self.doc.set_session_ranges(self.session, Vec::new());
                self.last.borrow_mut().clear();
            }
            return;
        }
        let anchors = self.anchors.borrow();
        // Resolved threads stop painting: the writer dealt with them, and leaving
        // the underline up makes a tidied manuscript look unresolved.
        let visible: Vec<LiveAnchor> = anchors.iter().filter(|a| !a.resolved).cloned().collect();
        drop(anchors);

        // Paragraph comments paint nothing in the text — their marker is the
        // margin bracket. Washing a whole block ochre would drown the page.
        let ranges: Vec<LiveAnchor> = visible.into_iter().filter(|a| !a.is_paragraph).collect();

        let mut next: Vec<RangeHighlight> = Vec::new();
        for (s, e, depth) in flatten(&ranges) {
            next.push(RangeHighlight {
                start: s,
                length: e - s,
                format: format_for(depth, self.base.get(), self.stacked.get()),
            });
        }
        next.sort_by_key(|r| (r.start, r.length));

        // Only push on a real change: `set_session_ranges` fires a repaint event,
        // and pushing an identical set every frame would churn the layout cache.
        if *self.last.borrow() != next {
            self.doc.set_session_ranges(self.session, next.clone());
            *self.last.borrow_mut() = next;
        }
    }
}

impl Drop for CommentHighlightSession {
    fn drop(&mut self) {
        // The subscription's own drop stops callbacks but does NOT retire the
        // highlight layer — remove it explicitly, exactly as FindSession does.
        self.doc.remove_session(self.session);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a(id: u64, start: usize, end: usize) -> LiveAnchor {
        LiveAnchor {
            comment_id: id,
            start,
            end,
            is_paragraph: false,
            resolved: false,
        }
    }

    #[test]
    fn disjoint_anchors_flatten_to_themselves() {
        let got = flatten(&[a(1, 0, 5), a(2, 10, 15)]);
        assert_eq!(got, vec![(0, 5, 1), (10, 15, 1)]);
    }

    #[test]
    fn overlapping_anchors_flatten_into_disjoint_segments_with_depth() {
        // 1: [0,10)  2: [5,15)  ->  [0,5)x1  [5,10)x2  [10,15)x1
        let got = flatten(&[a(1, 0, 10), a(2, 5, 15)]);
        assert_eq!(got, vec![(0, 5, 1), (5, 10, 2), (10, 15, 1)]);
    }

    #[test]
    fn a_nested_anchor_raises_depth_only_where_it_actually_covers() {
        let got = flatten(&[a(1, 0, 20), a(2, 8, 12)]);
        assert_eq!(got, vec![(0, 8, 1), (8, 12, 2), (12, 20, 1)]);
    }

    #[test]
    fn identical_anchors_collapse_to_one_segment_of_depth_two() {
        let got = flatten(&[a(1, 3, 9), a(2, 3, 9)]);
        assert_eq!(got, vec![(3, 9, 2)]);
    }

    #[test]
    fn empty_anchors_are_skipped_rather_than_painted() {
        // A collapsed anchor is an orphan-in-waiting, not a zero-width highlight.
        let got = flatten(&[a(1, 5, 5), a(2, 0, 4)]);
        assert_eq!(got, vec![(0, 4, 1)]);
    }

    #[test]
    fn flattened_segments_never_overlap() {
        let anchors = vec![a(1, 0, 30), a(2, 5, 12), a(3, 11, 25), a(4, 24, 40)];
        let got = flatten(&anchors);
        for w in got.windows(2) {
            assert!(w[0].1 <= w[1].0, "segments overlap: {:?} then {:?}", w[0], w[1]);
        }
    }

    #[test]
    fn a_range_comment_paints_an_ochre_wash_and_nothing_else() {
        // The LibreOffice convention. Deliberately NOT an underline: that competes
        // with the spell squiggle and with links, and a writer scanning for their
        // own notes should not have to tell three underlines apart.
        let base = Color::rgb(1, 2, 3);
        let f = format_for(1, base, base);
        assert_eq!(f.background_color, Some(base));
        assert!(
            f.underline_color.is_none() && f.underline_style.is_none(),
            "the wash is the whole signal — an underline here would collide with \
             the spell squiggle"
        );
    }

    #[test]
    fn a_stacked_segment_paints_deeper_than_a_single_one() {
        let base = Color::rgb(1, 2, 3);
        let stacked = Color::rgb(9, 9, 9);
        assert_ne!(
            format_for(1, base, stacked).background_color,
            format_for(2, base, stacked).background_color,
            "overlap must be visible as *something*, even though which threads \
             overlap is the margin's job to say"
        );
    }

    /// The fallback colour must be an ochre, not whatever was convenient.
    ///
    /// Regression: this defaulted to `rgb(120, 120, 200)` — violet — and because
    /// nothing ever called `set_colors`, that default *was* what shipped. The
    /// wash came out violet while the margin drew ochre, from two separate colour
    /// sources. Red-dominant-over-blue is the cheap invariant that catches it.
    #[test]
    fn the_fallback_wash_colour_is_an_ochre() {
        let doc = bastyde::text_document::TextDocument::new();
        let session = CommentHighlightSession::new(&doc);
        let c = session.base.get();
        let (r, g, b) = (c.red, c.green, c.blue);
        assert!(
            r > b && g > b,
            "ochre is warm: expected red and green above blue, got ({r}, {g}, {b})"
        );
    }

    /// Hiding takes effect **now**, not at the next keystroke.
    ///
    /// The trap: `set_active(false)` used only to stop *future* repaints, leaving
    /// whatever was last pushed on screen. A "hide" you have to type to see is not
    /// a hide, and Tools ▸ Comments is exactly the surface that would show it.
    #[test]
    fn going_inactive_clears_the_wash_immediately() {
        let doc = bastyde::text_document::TextDocument::new();
        doc.set_plain_text("one two three").unwrap();
        let session = CommentHighlightSession::new(&doc);
        session.set_anchors(vec![a(1, 0, 3)]);
        assert!(!session.last.borrow().is_empty(), "it should be painting");

        session.set_active(false);
        assert!(
            session.last.borrow().is_empty(),
            "the last-pushed ranges survived the hide"
        );
    }

    /// And showing again paints the same anchors back, without a re-anchor pass.
    #[test]
    fn coming_back_active_repaints_from_the_anchors_it_kept() {
        let doc = bastyde::text_document::TextDocument::new();
        doc.set_plain_text("one two three").unwrap();
        let session = CommentHighlightSession::new(&doc);
        session.set_anchors(vec![a(1, 0, 3)]);
        let painted = session.last.borrow().clone();

        session.set_active(false);
        session.set_active(true);
        assert_eq!(
            *session.last.borrow(),
            painted,
            "showing again must restore exactly what hiding took away"
        );
        assert_eq!(session.anchors().len(), 1, "the anchors were never dropped");
    }

    /// A hidden layer still tracks the anchors, so it does not come back stale.
    ///
    /// This is why hiding deactivates rather than drops: a dropped layer retires
    /// its highlight session and stops folding edits into its anchors, so showing
    /// again would resurrect yesterday's offsets — and flag orphans that are not.
    #[test]
    fn a_hidden_layer_still_follows_the_text_it_is_not_painting() {
        let doc = bastyde::text_document::TextDocument::new();
        doc.set_plain_text("one two three").unwrap();
        let session = CommentHighlightSession::new(&doc);
        session.set_anchors(vec![a(1, 8, 13)]);
        session.set_active(false);

        // Four characters inserted ahead of the anchor.
        session.apply_edits(&[(0, 0, 4)]);
        let moved = session.anchors();
        assert_eq!(
            (moved[0].start, moved[0].end),
            (12, 17),
            "a hidden anchor must still shift with the prose"
        );
        assert!(
            session.last.borrow().is_empty(),
            "and must still not be painting"
        );
    }

    /// Nothing repaints a hidden layer back into view behind the toggle's back —
    /// not a re-anchor pass, not a theme change.
    #[test]
    fn neither_new_anchors_nor_new_colours_wake_a_hidden_layer() {
        let doc = bastyde::text_document::TextDocument::new();
        doc.set_plain_text("one two three").unwrap();
        let session = CommentHighlightSession::new(&doc);
        session.set_active(false);

        session.set_anchors(vec![a(1, 0, 3)]);
        assert!(session.last.borrow().is_empty(), "set_anchors repainted it");
        session.set_colors(Color::rgb(1, 2, 3), Color::rgb(4, 5, 6));
        assert!(session.last.borrow().is_empty(), "set_colors repainted it");
    }

    #[test]
    fn a_paragraph_comment_paints_nothing_in_the_text() {
        // Its marker is the margin bracket; a full-block wash would drown the page.
        let doc = bastyde::text_document::TextDocument::new();
        doc.set_plain_text("one two three").unwrap();
        let session = CommentHighlightSession::new(&doc);
        session.set_anchors(vec![LiveAnchor {
            comment_id: 1,
            start: 0,
            end: 13,
            is_paragraph: true,
            resolved: false,
        }]);
        assert!(
            session.last.borrow().is_empty(),
            "a paragraph comment must push no text highlight at all"
        );
    }
}
