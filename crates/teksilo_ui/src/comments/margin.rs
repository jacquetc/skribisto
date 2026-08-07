// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The comment margin: the ochre marks beside the prose, the dotted leaders, and
//! the editable cards they lead to.
//!
//! This is the LibreOffice presentation, and the shape every word processor with
//! anchored comments has converged on:
//!
//! * A **range** comment washes its text ochre (that half is the highlight
//!   session's), drops a small **triangle** at the tail of the span, and runs a
//!   **dotted leader** out through the gap between lines to its card.
//! * A **paragraph** comment paints nothing in the text. It gets an ochre
//!   **`]` bracket** — a vertical rule with a tooth at each end — spanning the
//!   block's full height, and the same dotted leader from the bracket's middle.
//!
//! ## Why the marks live here and not in the editor
//!
//! The wash is genuinely *inside* the text: it is per-character paint, which is
//! what a `HighlightFormat` is for. The triangle, the bracket and the leader are
//! not — they sit in the gaps between lines and in the margin beyond the text
//! column, where no character exists to carry them. Trying to express them as
//! character formatting would mean inventing glyphs the document does not contain,
//! which would then leak into export, word counts and the accessibility tree.
//!
//! So they are painted by this widget, which overlays the whole pane and positions
//! itself from [`EditorHandle::range_rect`] — the geometry API added upstream for
//! exactly this. That also means the marks track scrolling for free, because the
//! rects it reports already account for scroll.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::canvas::{Canvas, Point, Rect};
use teksilo::core::binding::BindingLevel;
use teksilo::core::widget::{LayoutContext, LayoutResponse, PaintContext, Widget, WidgetPlacement};
use teksilo::core::widget_id::WidgetId;
use teksilo::prelude::*;
use teksilo::widgets::rich_text::EditorHandle;

use crate::comments::binding::CommentBinding;
use crate::comments::layout::{self, CardRequest};

/// Width of the card column.
///
/// Matches the trailing dock's 300 px so a writer moving between the margin and
/// the per-document dock reads the same measure in both.
pub const MARGIN_WIDTH: f32 = 300.0;
/// Space between the text column and the card column, which the leaders cross.
pub const LEADER_GUTTER: f32 = 28.0;

/// The full width a margin occupies once it has a card to show: the card column
/// plus the gutter the leaders cross.
///
/// The one place this sum is written. A page reserving a gutter
/// (`page_gutter`) and the margin reporting its own
/// width ([`CommentMargin::layout_response`]) have to agree to the pixel — when they
/// disagree the commented rows sit on a different measure from the rest and the
/// manuscript zigzags down the page. Computing the sum separately at each site
/// still compiles after only one of them is updated, which is exactly how that
/// mismatch gets reintroduced.
pub const fn reserved_width() -> f32 {
    MARGIN_WIDTH + LEADER_GUTTER
}

const TRIANGLE: f32 = 5.0;
const BRACKET_TOOTH: f32 = 6.0;
const DASH: f32 = 3.0;
const DASH_GAP: f32 = 3.0;

/// One mark to draw, resolved against the live layout.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Mark {
    comment_id: u64,
    /// x where the leader leaves the text.
    ///
    /// `None` for a paragraph comment, whose bracket belongs at the **column
    /// edge** rather than anywhere derived from the block's own box. A multi-line
    /// paragraph's `range_rect` is the union of the caret rects at its two ends,
    /// so its right edge is wherever the *last line happens to stop* — which for a
    /// short final line is far to the left, and is why the bracket first appeared
    /// on the wrong side of the text entirely.
    anchor_x: Option<f32>,
    y: f32,
    /// A paragraph comment's block extent (top, bottom), for the bracket. `None`
    /// for a range comment, which gets a triangle instead.
    bracket: Option<(f32, f32)>,
}

/// The dashes making up one leader run, as paintable rects.
///
/// Split out as a pure function so the pattern is table-testable: "is this a
/// dashed line or a solid one" and "does it stop at the endpoint" are exactly the
/// properties that are invisible in a screenshot until they are wrong.
fn dash_segments(from: (f32, f32), to: (f32, f32)) -> Vec<Rect> {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let len = (dx * dx + dy * dy).sqrt();
    if len <= 0.0 {
        return Vec::new();
    }
    let (ux, uy) = (dx / len, dy / len);
    let mut out = Vec::new();
    let mut travelled = 0.0;
    while travelled < len {
        let seg = DASH.min(len - travelled);
        out.push(Rect::new(
            from.0 + ux * travelled,
            from.1 + uy * travelled - 0.5,
            seg.max(1.0),
            1.0,
        ));
        travelled += seg + DASH_GAP;
    }
    out
}

/// The margin overlay: paints the marks and leaders, hosts the cards.
pub struct CommentMargin {
    binding: Option<CommentBinding>,
    /// The editor whose geometry the marks are resolved against. Read on every
    /// layout rather than cached, because a tab rebuild mints a fresh editor and a
    /// stored handle would report the old one's coordinates.
    editor: Rc<RefCell<Option<EditorHandle>>>,
    cards: Vec<(u64, WidgetId)>,
    /// Resolved during layout, consumed by paint. Painting cannot query the editor
    /// itself — `paint` has no `LayoutContext` — so the geometry is computed once
    /// where it is available and stashed.
    marks: RefCell<Vec<Mark>>,
    placements: RefCell<Vec<layout::Placement>>,
    palette: Signal<crate::view_models::CommentPalette>,
}

impl std::fmt::Debug for CommentMargin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommentMargin").finish()
    }
}

impl CommentMargin {
    pub fn new(
        binding: Option<CommentBinding>,
        editor: Rc<RefCell<Option<EditorHandle>>>,
        palette: Signal<crate::view_models::CommentPalette>,
    ) -> Self {
        Self {
            binding,
            editor,
            cards: Vec::new(),
            marks: RefCell::new(Vec::new()),
            placements: RefCell::new(Vec::new()),
            palette,
        }
    }

    /// Resolve every live thread's mark against the editor's current layout.
    ///
    /// A thread whose range reports no rect is skipped rather than drawn at the
    /// origin: before the first layout, and for a range scrolled out of the laid
    /// out region, there is genuinely nowhere correct to put it, and a mark at
    /// (0, 0) would point at the wrong line with total confidence.
    fn resolve_marks(&self) -> Vec<Mark> {
        let Some(binding) = &self.binding else {
            return Vec::new();
        };
        if !binding.view_model().is_visible() {
            // Nothing would be drawn anyway (`paint` skips a mark with no placed
            // card), but resolving every range against the editor runs on each
            // layout pass, and a hidden layer should not be paying for it.
            return Vec::new();
        }
        let editor = self.editor.borrow();
        let Some(handle) = editor.as_ref() else {
            return Vec::new();
        };
        binding
            .live()
            .into_iter()
            .filter(|a| !a.resolved && a.end > a.start)
            .filter_map(|a| {
                let span = handle.range_rect(a.start, a.end)?;
                if a.is_paragraph {
                    // Vertical extent only — the x is resolved at paint time from
                    // the margin's own left edge, so the bracket always lands in
                    // the gutter beside the column whatever shape the block is.
                    Some(Mark {
                        comment_id: a.comment_id,
                        anchor_x: None,
                        y: span.y + span.height / 2.0,
                        bracket: Some((span.y, span.y + span.height)),
                    })
                } else {
                    // The triangle sits at the tail of the span, on its baseline —
                    // the end, not the start, so it never covers the first word of
                    // the thing being commented on.
                    let tail = handle.offset_rect(a.end)?;
                    Some(Mark {
                        comment_id: a.comment_id,
                        anchor_x: Some(tail.x),
                        y: tail.y + tail.height,
                        bracket: None,
                    })
                }
            })
            .collect()
    }

    /// The card requests for this margin, with anchors re-based on `base`.
    ///
    /// Shared by the measure and the place passes so the two can never disagree
    /// about how tall the stack is — the failure that would leave a card hanging
    /// over the row below by exactly the amount the two computations differed.
    fn card_requests(&self, marks: &[Mark], base: f32, ctx: &LayoutContext) -> Vec<CardRequest> {
        self.cards
            .iter()
            .map(|(id, wid)| CardRequest {
                comment_id: *id,
                // A card whose thread resolved no mark this pass sits at the top
                // rather than vanishing — the same fallback `place_children` uses.
                anchor_y: marks
                    .iter()
                    .find(|m| m.comment_id == *id)
                    .map(|m| m.y - base)
                    .unwrap_or(0.0),
                height: ctx
                    .child_size(*wid, SizeProposal::exact(MARGIN_WIDTH, f32::INFINITY))
                    .map(|s| s.height)
                    .unwrap_or(72.0),
            })
            .collect()
    }

    /// How much vertical room this margin's cards need, measured from the top of
    /// the text they annotate.
    ///
    /// A single-item tab never had to ask. There the writing column *is* the page,
    /// so the margin was handed the page's height and the cards had all of it to
    /// stack in — which is what [`Widget::place_children`]'s "full height" comment
    /// means. A **stream row** is only as tall as its own few lines, and a thread
    /// with two replies is easily taller than a two-line scene, so a margin that
    /// silently accepted the row's height would stack its cards straight over the
    /// row below.
    ///
    /// Reported through `layout_response` rather than read directly, because the
    /// only thing that needs it — [`ColumnWithMargin`](crate::comments::pane::ColumnWithMargin)
    /// — holds this margin as a `WidgetId` and can reach it only by measuring it.
    fn wanted_height(&self, ctx: &LayoutContext) -> f32 {
        if self.cards.is_empty() {
            return 0.0;
        }
        let marks = self.resolve_marks();
        // Every anchor is in the editor's own coordinate space, which is a position
        // the caller cannot interpret. Re-basing on the first line turns the answer
        // into an *extent* it can compare against a prose height. The first line
        // rather than the widget top because that is the only reference the editor
        // handle actually exposes; it is short by the editor's top padding, which
        // the trailing `GAP` below more than covers.
        // `0.0` when the editor has no laid-out rect yet — which is the same condition
        // under which `resolve_marks` above already returned nothing, so this only ever
        // pairs with an empty `marks` and the stack below collapses to `GAP`. (An
        // earlier version derived a base from the marks here; it read as meaningful
        // per-mark math but could not run, because `offset_rect` and `range_rect` share
        // one underlying range window and so fail together before layout.)
        let base = self
            .editor
            .borrow()
            .as_ref()
            .and_then(|h| h.offset_rect(0))
            .map(|r| r.y)
            .unwrap_or(0.0);
        layout::stack(&self.card_requests(&marks, base, ctx))
            .iter()
            .map(|p| p.y + p.height)
            .fold(0.0, f32::max)
            // Keep the last card off whatever comes next, with the same breathing
            // room the stack already puts between two cards.
            + layout::GAP
    }

    /// A dotted run, painted from [`dash_segments`].
    ///
    /// Discrete dashes rather than a styled stroke because the canvas has no dash
    /// pattern — and a solid rule would read as a table border rather than as a
    /// leader.
    fn dotted(canvas: &mut Canvas, from: (f32, f32), to: (f32, f32), color: Color) {
        for r in dash_segments(from, to) {
            canvas.fill_rect(r, color);
        }
    }
}

impl Widget for CommentMargin {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.cards.clear();
        let Some(binding) = self.binding.clone() else {
            return Vec::new();
        };
        // Rebuild when the comment set's SHAPE changes — a thread created,
        // deleted, resolved or re-anchored — but deliberately not on a body edit.
        //
        // Binding the full version signal instead would re-mint the very editor
        // the writer is typing into, once per keystroke, resetting their caret to
        // the start of the comment every time.
        binding.view_model().model().structure_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let vm = binding.view_model();
        // Tools ▸ Comments. Bound at `Rebuild` too, because hiding has to *remove* the
        // cards — a repaint-level binding would leave them mounted and merely stop
        // drawing the marks, which is the state where a card sits in the margin with
        // nothing in the prose to point at.
        vm.visible_signal()
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        // Re-derive the anchors *before* reading them below, so the wash, the marks
        // and the cards are all built from one snapshot of the comment set.
        //
        // Without this the highlight session kept whatever anchors it was last
        // given — refreshed only on open and on create — so a deleted thread left
        // its ochre on the text and a resolved one did too. It was invisible from
        // here because `live()` reads that same stale list, and a card whose id no
        // longer resolves against the model is silently skipped: the card went, the
        // wash stayed. See `CommentBinding::refresh_wash`.
        binding.refresh_wash();

        let rows = vm.model().rows_for_content(binding.content_id());
        // Hidden: no cards. Everything downstream is derived from `self.cards` — the
        // width, the stacking, the marks and the leaders — so starving this one list is
        // the whole hide, and the margin gives its column back to the page for free.
        let live: Vec<u64> = if !vm.is_visible() {
            Vec::new()
        } else {
            binding
                .live()
                .into_iter()
                .filter(|a| !a.resolved && a.end > a.start)
                .map(|a| a.comment_id)
                .collect()
        };

        let mut ids = Vec::new();
        for id in live {
            let Some(row) = rows.iter().find(|r| r.id == id) else {
                continue;
            };
            let card = super::card::comment_card(vm.clone(), row.clone(), self.palette.get());
            let wid = ctx.add(card);
            self.cards.push((id, wid));
            ids.push(wid);
        }
        ids
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Zero width until this document actually has a comment.
        //
        // A margin that always reserved its column would shove the writing page
        // permanently off-centre for the majority of documents, which have no
        // comments at all — paying the whole cost of the feature for none of its
        // benefit. It claims its width the moment the first thread appears, and
        // gives it back when the last one goes.
        let width = if self.cards.is_empty() {
            0.0
        } else {
            reserved_width()
        };
        // **Its own width, never the proposal's.** `SizeProposal::resolve` hands
        // back whatever it was proposed and only falls through to the default when
        // the proposal is unspecified — so resolving here made the margin report
        // the full pane width whenever a parent measured it with a definite one.
        // It then covered the prose entirely: invisible (it paints nothing without
        // marks), but swallowing every click meant for the text underneath.
        //
        // The **height** is the opposite case: a definite proposal is the parent
        // saying how tall this margin is, and it is honoured. Only when asked
        // without one — which is exactly how `ColumnWithMargin` measures — does the
        // margin state what its cards actually need. See [`Self::wanted_height`].
        teksilo::canvas::Size::new(
            width,
            proposal.height.unwrap_or_else(|| self.wanted_height(ctx)),
        )
        .into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        ctx: &LayoutContext,
    ) {
        let marks = self.resolve_marks();
        let card_x = bounds.x + bounds.width - MARGIN_WIDTH;

        // Ask each card how tall it wants to be, then stack them so none overlaps.
        // Anchors are re-based on this margin's own top, so a card with no resolved
        // mark lands at the top of the margin exactly as before — and so the stack
        // is the *same* arithmetic `wanted_height` measured with.
        // …then shifted back into the absolute space the child origins and the
        // painted leaders both speak. With `base = bounds.y` this is exactly the
        // arithmetic this pass did before the split, markless-card fallback and all.
        let placed: Vec<layout::Placement> =
            layout::stack(&self.card_requests(&marks, bounds.y, ctx))
                .into_iter()
                .map(|mut p| {
                    p.y += bounds.y;
                    p.anchor_y += bounds.y;
                    p
                })
                .collect();

        for (i, (id, _)) in self.cards.iter().enumerate() {
            if i >= children.len() {
                break;
            }
            let Some(p) = placed.iter().find(|p| p.comment_id == *id) else {
                continue;
            };
            children[i].origin = Point::new(card_x, p.y);
            children[i].size = teksilo::canvas::Size::new(MARGIN_WIDTH, p.height);
        }

        *self.marks.borrow_mut() = marks;
        *self.placements.borrow_mut() = placed;
    }

    fn paint(&self, bounds: Rect, canvas: &mut Canvas, _ctx: &PaintContext) {
        let color = self.palette.get().ink;
        let marks = self.marks.borrow();
        let placements = self.placements.borrow();
        let card_x = bounds.x + bounds.width - MARGIN_WIDTH;

        for mark in marks.iter() {
            let Some(p) = placements.iter().find(|p| p.comment_id == mark.comment_id) else {
                continue;
            };

            // The gutter runs from the margin's left edge to the card column; a
            // paragraph bracket lives at its left, hard against the text.
            let bracket_x = bounds.x + BRACKET_TOOTH;

            match mark.bracket {
                // A paragraph comment: `]` — a vertical rule with a tooth at each
                // end pointing back at the text, so the extent it claims is
                // unambiguous. A bare rule would read as a change bar; the teeth
                // are what say "this block".
                Some((top, bottom)) => {
                    canvas.fill_rect(
                        Rect::new(bracket_x, top, 1.5, (bottom - top).max(1.0)),
                        color,
                    );
                    canvas.fill_rect(
                        Rect::new(bracket_x - BRACKET_TOOTH, top, BRACKET_TOOTH, 1.5),
                        color,
                    );
                    canvas.fill_rect(
                        Rect::new(bracket_x - BRACKET_TOOTH, bottom - 1.5, BRACKET_TOOTH, 1.5),
                        color,
                    );
                }
                // A range comment: a small triangle at the tail of the span,
                // pointing back up at the text it belongs to.
                None => {
                    let tx = mark.anchor_x.unwrap_or(bracket_x);
                    let ty = mark.y;
                    for i in 0..=(TRIANGLE as i32) {
                        let w = (TRIANGLE - i as f32) * 2.0;
                        canvas.fill_rect(
                            Rect::new(tx - w / 2.0, ty + i as f32, w.max(1.0), 1.0),
                            color,
                        );
                    }
                }
            }

            // The leader, from the mark out to the card.
            let start = (mark.anchor_x.unwrap_or(bracket_x), mark.y);
            // The elbow sits in the middle of the gutter, in clear space between
            // the text and the cards.
            let elbow_x = card_x - LEADER_GUTTER / 2.0;
            let [a, b, c] = layout::leader(start.0, start.1, elbow_x, card_x, p.y + 10.0);
            Self::dotted(canvas, a, b, color);
            Self::dotted(canvas, b, c, color);
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.cards.iter().map(|(_, id)| *id).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::widget_tree::WidgetTree;

    /// With no binding the margin is inert, takes no children, and claims **no
    /// width at all** — the state every editor built without a project around it
    /// (and every widget test) is in.
    ///
    /// The proposal here is deliberately a definite one: a margin that resolved
    /// against it would report the full 300 px and cover the prose it sits beside,
    /// paint nothing, and silently swallow every click meant for the text.
    #[test]
    fn a_margin_with_no_binding_claims_no_width() {
        let m = CommentMargin::new(
            None,
            Rc::new(RefCell::new(None)),
            Signal::new(crate::view_models::CommentPalette::default()),
        );
        let mut tree = WidgetTree::new();
        // Under a parent, not at the root: a root widget is simply given the
        // window, so its own `layout_response` is never what decides its bounds.
        let root = tree.add(teksilo::widgets::HStack::new().child(m));
        tree.layout(SizeProposal::exact(MARGIN_WIDTH, 400.0));
        let id = tree.children(root)[0];
        assert_eq!(
            tree.bounds(id).width,
            0.0,
            "an empty margin must give its column back to the page"
        );
    }

    /// An empty margin asks for **no height either**, which is what keeps a stream
    /// row with no comments exactly as tall as its prose.
    ///
    /// The width half of this is above; the height half is new, and is the one a
    /// careless `unwrap_or(1.0)`-style default would break silently — every row on
    /// every page would gain a few stray pixels and nothing would point at why.
    #[test]
    fn a_margin_with_no_cards_asks_for_no_height() {
        let m = CommentMargin::new(
            None,
            Rc::new(RefCell::new(None)),
            Signal::new(crate::view_models::CommentPalette::default()),
        );
        let mut tree = WidgetTree::new();
        let root = tree.add(teksilo::widgets::HStack::new().child(m));
        // Width-only: the proposal `ColumnWithMargin` measures with, and the only
        // one under which the margin states what it wants rather than accepting.
        tree.layout(SizeProposal::with_width(MARGIN_WIDTH));
        assert_eq!(tree.bounds(tree.children(root)[0]).height, 0.0);
    }

    /// Marks resolve to nothing before the editor has laid out.
    ///
    /// The alternative — falling back to the origin — would draw every triangle and
    /// leader in the top-left corner, pointing confidently at the wrong line.
    #[test]
    fn marks_are_empty_without_a_live_editor_handle() {
        let m = CommentMargin::new(
            None,
            Rc::new(RefCell::new(None)),
            Signal::new(crate::view_models::CommentPalette::default()),
        );
        assert!(m.resolve_marks().is_empty());
    }

    /// The leader is genuinely dashed — several separated segments, not one rule.
    #[test]
    fn a_leader_run_is_dashed_not_solid() {
        let segs = dash_segments((0.0, 10.0), (30.0, 10.0));
        assert!(segs.len() > 1, "a solid rule would read as a table border");
        // Consecutive dashes are separated by a gap, which is what makes it dotted.
        let a = &segs[0];
        let b = &segs[1];
        assert!(
            b.x > a.x + a.width,
            "dashes must not touch: {a:?} then {b:?}"
        );
    }

    /// It stops at the endpoint rather than overshooting into the card.
    #[test]
    fn a_leader_run_stops_at_its_endpoint() {
        let segs = dash_segments((0.0, 0.0), (20.0, 0.0));
        let end = segs.last().expect("some dashes");
        assert!(
            end.x + end.width <= 20.0 + DASH,
            "the run overshot its endpoint: {end:?}"
        );
    }

    /// A zero-length run paints nothing rather than looping forever.
    #[test]
    fn a_zero_length_leader_paints_nothing() {
        assert!(dash_segments((5.0, 5.0), (5.0, 5.0)).is_empty());
    }

    /// Tools ▸ Comments off: no cards, no column, nothing painted.
    ///
    /// Needs a real binding — a view-model, a document, a live anchor and a row that
    /// matches it — which only the mocks build can supply without a project on disk.
    /// That is also the build where the margin is demonstrable, so this is where the
    /// toggle is worth proving end to end rather than at the flag.
    #[cfg(feature = "mocks")]
    mod visibility {
        use super::*;
        use crate::app_ids::AppIds;
        use crate::comments::binding::CommentBinding;
        use crate::comments::session::{CommentHighlightSession, LiveAnchor};
        use crate::models::CommentsListModel;
        use crate::view_models::{CommentPalette, CommentsViewModel};
        use teksilo::text_document::TextDocument;
        use frontend::common::entities::ContentRole;

        /// A margin over the mock fixture's first thread, anchored to real text.
        fn margin() -> (CommentsViewModel, CommentMargin) {
            let ctx = std::rc::Rc::new(frontend::AppContext::new());
            let vm = CommentsViewModel::new(
                CommentsListModel::new(ctx.clone(), AppIds::new()),
                ctx,
                Signal::new(None),
            );
            let content = crate::singles::mock_content_id(201, &ContentRole::SceneText);
            let doc = TextDocument::new();
            doc.set_plain_text("The morning light crept over the ridgeline.")
                .unwrap();
            let session = CommentHighlightSession::new(&doc);
            // Comment 1 of the fabricated set, anchored by hand so the test does not
            // depend on the exact offsets of the mock prose.
            session.set_anchors(vec![LiveAnchor {
                comment_id: 1,
                start: 0,
                end: 17,
                is_paragraph: false,
                resolved: false,
            }]);
            let binding = CommentBinding::new(vm.clone(), doc, session, content);
            let palette = Signal::new(CommentPalette::default());
            (
                vm,
                CommentMargin::new(Some(binding), Rc::new(RefCell::new(None)), palette),
            )
        }

        /// The same fixture, but with an extra anchor for a thread the model does
        /// **not** have — exactly the state a delete leaves behind. Hands back the
        /// session so the test can read what survived.
        fn margin_with_a_ghost() -> (
            CommentsViewModel,
            CommentMargin,
            std::rc::Rc<CommentHighlightSession>,
        ) {
            let ctx = std::rc::Rc::new(frontend::AppContext::new());
            let vm = CommentsViewModel::new(
                CommentsListModel::new(ctx.clone(), AppIds::new()),
                ctx,
                Signal::new(None),
            );
            let content = crate::singles::mock_content_id(201, &ContentRole::SceneText);
            let doc = TextDocument::new();
            doc.set_plain_text("The morning light crept over the ridgeline.")
                .unwrap();
            let session = CommentHighlightSession::new(&doc);
            session.set_anchors(vec![LiveAnchor {
                // A thread that no longer exists — deleted since the anchors were
                // last pushed. Id far outside the fixture's range so it cannot
                // accidentally match a real row.
                comment_id: 9_999,
                start: 0,
                end: 17,
                is_paragraph: false,
                resolved: false,
            }]);
            let binding = CommentBinding::new(vm.clone(), doc, session.clone(), content);
            let palette = Signal::new(CommentPalette::default());
            (
                vm,
                CommentMargin::new(Some(binding), Rc::new(RefCell::new(None)), palette),
                session,
            )
        }

        /// **The delete bug.** A thread that is gone from the model must take its
        /// wash off the text.
        ///
        /// The highlight session paints from its own anchor list, and that list was
        /// only ever refreshed when a document opened or a comment was *created* —
        /// never on delete, resolve or reopen. What made it look like a paint bug
        /// rather than a stale-model one is that the card disappeared on cue:
        /// `build` drops a card whose id no longer resolves against the model, while
        /// `repaint` needs no such lookup and kept washing the text underneath.
        ///
        /// Laying the margin out is the whole point — the assertion is that
        /// *mounting* it re-derives the anchors, not that a direct call to
        /// `refresh_wash` would.
        #[test]
        fn a_deleted_thread_stops_washing_its_text() {
            let (_vm, m, session) = margin_with_a_ghost();
            assert_eq!(session.anchors().len(), 1, "the stale anchor is seeded");

            let mut tree = WidgetTree::new();
            tree.add(teksilo::widgets::HStack::new().child(m));
            tree.layout(SizeProposal::exact(900.0, 600.0));

            assert!(
                session.anchors().iter().all(|a| a.comment_id != 9_999),
                "a thread the model no longer has kept its wash on the text"
            );
        }

        /// The width the margin claims with comments shown or hidden. Under a
        /// parent, not at the root: a root widget is simply given the window.
        fn width(visible: bool) -> f32 {
            let (vm, m) = margin();
            vm.set_visible(visible);
            let mut tree = WidgetTree::new();
            let root = tree.add(teksilo::widgets::HStack::new().child(m));
            tree.layout(SizeProposal::exact(900.0, 600.0));
            tree.bounds(tree.children(root)[0]).width
        }

        #[test]
        fn a_visible_margin_claims_its_column() {
            assert!(
                width(true) > 0.0,
                "the fixture never mounted a card — the rest of this module proves nothing"
            );
        }

        #[test]
        fn hiding_comments_gives_the_column_back_to_the_page() {
            assert_eq!(
                width(false),
                0.0,
                "a hidden margin must take no width, or the prose stays shoved aside \
                 with nothing beside it"
            );
        }

        #[test]
        fn a_hidden_margin_resolves_no_marks_to_paint() {
            let (vm, m) = margin();
            vm.set_visible(false);
            assert!(
                m.resolve_marks().is_empty(),
                "a hidden margin still resolved marks against the editor"
            );
            vm.set_visible(true);
            // (With no live editor handle there is still nothing to resolve — the
            // point here is only that hiding short-circuits before asking.)
        }
    }

    /// A vertical run dashes too — the elbow's finishing segment when a card was
    /// pushed down away from its anchor.
    #[test]
    fn a_vertical_leader_run_also_dashes() {
        let segs = dash_segments((10.0, 0.0), (10.0, 40.0));
        assert!(segs.len() > 1);
        assert!(segs[1].y > segs[0].y, "it advances downwards");
    }
}
