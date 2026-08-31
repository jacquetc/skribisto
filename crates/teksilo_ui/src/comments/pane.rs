// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The writing column and its comment margin, side by side.
//!
//! A plain `HStack` cannot express this arrangement. It would pin the margin to
//! the far edge of the pane and let the column centre itself in whatever is left,
//! so a wide window puts a long empty run of leader between each comment and the
//! text it belongs to — the cards marooned against the window edge while the prose
//! sits in the middle. What a word processor actually does is keep the cards
//! *flush* against the text column and move the whole pair only when it must.
//!
//! The rules are [`crate::comments::layout::place_pane`], where they are pure and
//! table-tested; this widget is only the wiring that measures the two children and
//! applies them.
//!
//! ## Two measurements, not one
//!
//! The column is measured twice in the worst case. The first pass asks what it
//! wants when nothing competes — that width is what decides whether the pair still
//! fits centred. Only if the answer is "no" is it measured again at the reduced
//! width, because its *height* depends on where the prose wraps, and reporting the
//! first pass's height for a narrower column would leave the page's last lines
//! outside the scrollable extent.

use teksilo::canvas::{Point, Rect, Size};
use teksilo::core::widget::{LayoutContext, LayoutResponse, Widget, WidgetPlacement};
use teksilo::core::widget_id::WidgetId;
use teksilo::prelude::*;

use crate::comments::layout::{PanePlacement, place_pane};

/// Places a writing column and a comment margin across the pane.
pub struct ColumnWithMargin {
    pending_column: Option<Box<dyn Widget>>,
    pending_margin: Option<Box<dyn Widget>>,
    column: Option<WidgetId>,
    margin: Option<WidgetId>,
    /// The column's max-width preference, which is what it *wants* before any
    /// margin is taken into account.
    cap: Signal<f32>,
    /// A gutter this column reserves **even with no comments of its own**.
    ///
    /// Zero for a single-item tab, where the one margin on the page speaks for
    /// itself. A **stream** sets it for every row at once, because the reservation
    /// has to be a property of the page rather than of each row: the margin claims
    /// its width only once a document has a comment, so in a pane too tight to fit
    /// the pair centred (see `place_pane`'s rule 3) the commented rows would shift
    /// left while the others stayed centred, and the manuscript would zigzag down
    /// the page. All rows reserve, or none do.
    reserve: Signal<f32>,
}

impl std::fmt::Debug for ColumnWithMargin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ColumnWithMargin").finish()
    }
}

impl ColumnWithMargin {
    pub fn new(
        column: impl Widget + 'static,
        margin: impl Widget + 'static,
        cap: Signal<f32>,
    ) -> Self {
        Self {
            pending_column: Some(Box::new(column)),
            pending_margin: Some(Box::new(margin)),
            column: None,
            margin: None,
            cap,
            reserve: Signal::new(0.0),
        }
    }

    /// Reserve `width` for the margin whatever this document holds — see
    /// [`Self::reserve`]. A stream hands every row the same signal.
    pub fn reserve(mut self, width: Signal<f32>) -> Self {
        self.reserve = width;
        self
    }

    /// The width the column is proposed, floored so it bottoms out rather than
    /// collapsing toward zero (which would wrap the prose one word per line and
    /// make the page absurdly tall).
    fn floor(w: f32) -> f32 {
        w.max(crate::tabs::shared::editor::MIN_COLUMN_WIDTH)
    }

    /// Measure both children and decide where each goes.
    ///
    /// Returns the placement, the column's resolved size, and the height the
    /// margin's cards want — so the caller can report a size without measuring a
    /// third time.
    fn resolve(&self, available: f32, ctx: &LayoutContext) -> (PanePlacement, Size, f32) {
        // Asked with an *unspecified* proposal, because the question is what the
        // margin wants, not what it would accept: its width is a fixed column or
        // nothing at all, never a share of what is going — and its height is what
        // its card stack needs rather than whatever it is offered.
        let margin = self
            .margin
            .and_then(|id| ctx.child_size(id, SizeProposal::unspecified()));
        let margin_h = margin.map(|s| s.height).unwrap_or(0.0);
        let wants = Self::floor(available.min(self.cap.get()));

        // The margin reports zero width until this document has a comment, which
        // is what makes it appear and disappear rather than permanently reserving
        // a column the majority of documents never use. A page-level `reserve`
        // overrides that downwards-only: a row with comments never gives up its
        // gutter, it is the empty rows that take one to match.
        //
        // **But only where the reservation is free.** A stream pane is often far
        // narrower than a window — 512 dp with the binder and inspector open, next
        // to a 328 dp margin — and there the reservation does not merely shift the
        // column, it shrinks it: measured at 184 dp, four words to a line, for every
        // row on the page including the ones with nothing to show. Consistency is
        // not worth an unreadable manuscript. Below the threshold each row falls
        // back to deciding for itself, which is exactly what a single-scene tab has
        // always done in a pane this tight.
        let reserved = self.reserve.get();
        let reserved = if available - reserved >= wants {
            reserved
        } else {
            0.0
        };
        let margin_w = match margin.map(|s| s.width).filter(|w| *w > 0.0) {
            Some(w) => Some(w.max(reserved)),
            None if reserved > 0.0 => Some(reserved),
            None => None,
        };
        let Some(id) = self.column else {
            return (
                place_pane(available, wants, margin_w),
                Size::new(wants, 0.0),
                margin_h,
            );
        };

        let desired = ctx
            .child_size(id, SizeProposal::with_width(wants))
            .unwrap_or(Size::new(wants, 0.0));
        let first = place_pane(available, desired.width, margin_w);
        if first.column_width >= desired.width {
            return (first, desired, margin_h);
        }

        // It has to give up width: re-measure so the reported height is the height
        // of the column the writer will actually see.
        let squeezed = ctx
            .child_size(
                id,
                SizeProposal::with_width(Self::floor(first.column_width)),
            )
            .unwrap_or(Size::new(first.column_width, desired.height));
        (
            place_pane(available, squeezed.width, margin_w),
            squeezed,
            margin_h,
        )
    }
}

impl Widget for ColumnWithMargin {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        if let Some(w) = self.pending_column.take() {
            self.column = Some(ctx.add_boxed(w));
        }
        if let Some(w) = self.pending_margin.take() {
            self.margin = Some(ctx.add_boxed(w));
        }
        self.children()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        let available = proposal.resolve(0.0, 0.0).width;
        let (_, column, margin_h) = self.resolve(available, ctx);
        // The column's intrinsic height is the page's height: this sits inside the
        // tab's outer `ScrollArea`, and filling the proposal would make the page
        // exactly one screen tall however long the scene is.
        //
        // …but never shorter than the cards beside it. On a single-item tab the
        // prose always wins this `max` and nothing changes. On a **stream row** it
        // is the other way round often enough to matter: a two-line scene carrying
        // a thread with replies has to grow, or its cards stack over the row below.
        // Growing is the honest option — the alternative, clipping, hides the text
        // the comment exists to show (the same reasoning `layout::stack` gives for
        // pushing cards down rather than shrinking them).
        Size::new(available, column.height.max(margin_h)).into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        ctx: &LayoutContext,
    ) {
        let (p, column, _) = self.resolve(bounds.width, ctx);
        for child in children.iter_mut() {
            if Some(child.id) == self.column {
                // Its own height, not the row's: when the cards made the row taller
                // the extra belongs to the margin, and stretching the editor into it
                // would put a click target for the prose beside a comment card.
                child.origin = Point::new(bounds.x + p.column_x, bounds.y);
                child.size = Size::new(p.column_width, column.height);
            } else if Some(child.id) == self.margin {
                // Full height, so the cards can stack anywhere down the page and
                // the leaders can be painted the whole way.
                let x = p.margin_x.unwrap_or(bounds.width);
                child.origin = Point::new(bounds.x + x, bounds.y);
                child.size = Size::new((bounds.width - x).max(0.0), bounds.height);
            }
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.column.into_iter().chain(self.margin).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::widgets::{FixedSize, MaxSize};

    /// A stand-in column with the real one's shape: a `MaxSize` cap over an
    /// `Expand` that fills whatever width it is proposed, so it resolves to
    /// `min(cap, proposed)` exactly as the writing column does.
    fn column(cap: f32) -> impl Widget {
        MaxSize::width(cap).child(
            teksilo::widgets::Expand::horizontal().child(
                FixedSize::new()
                    .height(200.0)
                    .child(teksilo::widgets::Spacer::new()),
            ),
        )
    }

    /// A stand-in margin of a fixed width, or nothing at all.
    fn margin(width: f32) -> impl Widget {
        FixedSize::new()
            .width(width)
            .child(teksilo::widgets::Spacer::new())
    }

    /// A stand-in margin that behaves like the real one in the way that matters
    /// here: it *asks* for a height (its card stack) rather than accepting whatever
    /// it is given.
    fn tall_margin(width: f32, height: f32) -> impl Widget {
        FixedSize::new()
            .width(width)
            .height(height)
            .child(teksilo::widgets::Spacer::new())
    }

    fn place(available: f32, cap: f32, margin_w: f32) -> (Rect, Rect) {
        let mut tree = WidgetTree::new();
        let root = tree.add(ColumnWithMargin::new(
            column(cap),
            margin(margin_w),
            Signal::new(cap),
        ));
        tree.layout(SizeProposal::exact(available, 600.0));
        let kids = tree.children(root);
        (tree.bounds(kids[0]), tree.bounds(kids[1]))
    }

    /// **The stream-row fix.** A row is only as tall as its own few lines, so a
    /// margin whose cards need more must make the row grow — otherwise the stack
    /// runs on over the row below.
    ///
    /// The prose here is 200 tall and the cards want 340; the pair must report 340.
    #[test]
    fn the_row_grows_to_fit_a_card_stack_taller_than_its_prose() {
        let mut tree = WidgetTree::new();
        let root = tree.add(ColumnWithMargin::new(
            column(600.0),
            tall_margin(300.0, 340.0),
            Signal::new(600.0),
        ));
        // Unspecified height: this is how the pane's scrolling `VStack` asks a row
        // what it wants, and the only case in which the answer is the row's own.
        tree.layout(SizeProposal::with_width(1400.0));
        assert_eq!(
            tree.bounds(root).height,
            340.0,
            "the row must make room for its comments, not clip them"
        );
    }

    /// The other direction, which is every single-item tab: the prose is taller
    /// than the cards and nothing about the page changes.
    #[test]
    fn prose_taller_than_its_cards_still_sets_the_height() {
        let mut tree = WidgetTree::new();
        let root = tree.add(ColumnWithMargin::new(
            column(600.0),
            tall_margin(300.0, 80.0),
            Signal::new(600.0),
        ));
        tree.layout(SizeProposal::with_width(1400.0));
        assert_eq!(tree.bounds(root).height, 200.0);
    }

    /// A row that grew for its cards does **not** stretch the editor into the extra
    /// space — that would put a click target for the prose beside a comment card.
    #[test]
    fn the_editor_keeps_its_own_height_when_the_row_grows() {
        let mut tree = WidgetTree::new();
        let root = tree.add(ColumnWithMargin::new(
            column(600.0),
            tall_margin(300.0, 340.0),
            Signal::new(600.0),
        ));
        tree.layout(SizeProposal::with_width(1400.0));
        let kids = tree.children(root);
        assert_eq!(tree.bounds(kids[0]).height, 200.0, "the prose column");
        assert_eq!(
            tree.bounds(kids[1]).height,
            340.0,
            "the margin gets the rest"
        );
    }

    /// **The page-level reservation.** A row with no comments of its own still
    /// reserves the gutter when the page does, so every row in a stream keeps the
    /// same measure.
    ///
    /// Checked in a *tight* pane, because that is the only place the difference
    /// shows: with room to spare `place_pane` centres the column either way.
    #[test]
    fn a_reserved_gutter_moves_an_uncommented_row_exactly_like_a_commented_one() {
        let lay = |reserve: f32, margin_w: f32| {
            let mut tree = WidgetTree::new();
            let root = tree.add(
                ColumnWithMargin::new(column(600.0), margin(margin_w), Signal::new(600.0))
                    .reserve(Signal::new(reserve)),
            );
            tree.layout(SizeProposal::exact(1000.0, 600.0));
            tree.bounds(tree.children(root)[0]).x
        };
        let commented = lay(300.0, 300.0);
        let bare = lay(300.0, 0.0);
        assert_eq!(
            bare, commented,
            "a row without comments must sit on the same measure as one with them"
        );
        assert!(
            lay(0.0, 0.0) > bare,
            "and without the reservation it would sit somewhere else entirely — \
             the zigzag this exists to prevent"
        );
    }

    /// **The reservation never costs the prose its width.** In a pane too tight to
    /// hold the column *and* the gutter, an uncommented row keeps its full measure
    /// rather than being shrunk to match a neighbour.
    ///
    /// The numbers are the ones measured in the app: a 512 dp stream pane beside a
    /// 328 dp margin left 184 dp of prose — four words to a line, on every row of
    /// the manuscript. A consistent measure is not worth that.
    #[test]
    fn a_pane_too_tight_for_the_gutter_ignores_the_reservation() {
        let mut tree = WidgetTree::new();
        let root = tree.add(
            ColumnWithMargin::new(column(600.0), margin(0.0), Signal::new(600.0))
                .reserve(Signal::new(328.0)),
        );
        tree.layout(SizeProposal::exact(512.0, 600.0));
        assert_eq!(
            tree.bounds(tree.children(root)[0]).width,
            512.0,
            "an uncommented row must keep the whole pane, not 184 dp of it"
        );
    }

    /// …and it still applies in the band where it only *shifts* the column, which
    /// is the whole zigzag case: 1000 wide holds a 600 column beside a 300 margin,
    /// it just cannot centre it.
    #[test]
    fn the_reservation_still_applies_when_it_only_shifts() {
        let mut tree = WidgetTree::new();
        let root = tree.add(
            ColumnWithMargin::new(column(600.0), margin(0.0), Signal::new(600.0))
                .reserve(Signal::new(300.0)),
        );
        tree.layout(SizeProposal::exact(1000.0, 600.0));
        let col = tree.bounds(tree.children(root)[0]);
        assert_eq!(col.width, 600.0, "no width is given up");
        assert_eq!(col.x, 100.0, "but it sits where a commented row sits");
    }

    /// The reservation never *takes* width from a row that has its own cards: it is
    /// a floor, not an override.
    #[test]
    fn a_reservation_narrower_than_the_real_margin_does_not_shrink_it() {
        let mut tree = WidgetTree::new();
        let root = tree.add(
            ColumnWithMargin::new(column(600.0), margin(300.0), Signal::new(600.0))
                .reserve(Signal::new(50.0)),
        );
        tree.layout(SizeProposal::exact(1000.0, 600.0));
        let kids = tree.children(root);
        assert_eq!(tree.bounds(kids[1]).width, 300.0);
    }

    /// The cards sit against the prose, not against the window.
    #[test]
    fn the_margin_is_flush_with_the_column() {
        let (col, margin) = place(1400.0, 600.0, 300.0);
        assert!(
            (margin.x - (col.x + col.width)).abs() < 0.5,
            "gap between column (ends {}) and margin (starts {})",
            col.x + col.width,
            margin.x
        );
    }

    /// With room to spare the page does not move at all — a comment appearing must
    /// not yank the prose sideways.
    #[test]
    fn the_column_stays_centred_in_the_whole_pane_while_the_margin_fits() {
        let (col, _) = place(1400.0, 600.0, 300.0);
        assert!(
            (col.x - 400.0).abs() < 0.5,
            "column at {} rather than centred at 400",
            col.x
        );
    }

    /// A margin with nothing in it takes no width, and the column is simply centred.
    #[test]
    fn a_zero_width_margin_leaves_the_column_centred() {
        let (col, margin) = place(1000.0, 600.0, 0.0);
        assert_eq!(margin.width, 0.0);
        assert!(
            (col.x - 200.0).abs() < 0.5,
            "column at {} rather than centred at 200",
            col.x
        );
    }

    /// When the pair no longer fits centred, the column shifts rather than shrinks.
    #[test]
    fn a_tight_pane_shifts_the_column_before_shrinking_it() {
        let (col, margin) = place(1000.0, 600.0, 300.0);
        assert!(
            (col.width - 600.0).abs() < 0.5,
            "column shrank to {} when shifting would have done",
            col.width
        );
        assert!(col.x < 200.0, "it should have moved left of centre");
        assert!(
            margin.x + margin.width <= 1000.5,
            "the margin ran off the pane"
        );
    }

    /// Only once it is hard against the left edge does the column give up width.
    #[test]
    fn a_very_narrow_pane_shrinks_the_column() {
        let (col, _) = place(700.0, 600.0, 300.0);
        assert!(col.x < 1.0, "column at {} rather than hard left", col.x);
        assert!(
            col.width < 600.0,
            "column kept its full width in a pane too narrow for it"
        );
    }
}
