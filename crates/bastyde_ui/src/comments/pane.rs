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

use bastyde::canvas::{Point, Rect, Size};
use bastyde::core::widget::{LayoutContext, LayoutResponse, Widget, WidgetPlacement};
use bastyde::core::widget_id::WidgetId;
use bastyde::prelude::*;

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
        }
    }

    /// The width the column is proposed, floored so it bottoms out rather than
    /// collapsing toward zero (which would wrap the prose one word per line and
    /// make the page absurdly tall).
    fn floor(w: f32) -> f32 {
        w.max(crate::tabs::shared::editor::MIN_COLUMN_WIDTH)
    }

    /// Measure both children and decide where each goes.
    ///
    /// Returns the placement plus the column's resolved size, so the caller can
    /// report a height without measuring a third time.
    fn resolve(&self, available: f32, ctx: &LayoutContext) -> (PanePlacement, Size) {
        // The margin reports zero width until this document has a comment, which
        // is what makes it appear and disappear rather than permanently reserving
        // a column the majority of documents never use.
        // Asked with an *unspecified* proposal, because the question is what the
        // margin wants, not what it would accept: its width is a fixed column or
        // nothing at all, never a share of what is going.
        let margin_w = self
            .margin
            .and_then(|id| ctx.child_size(id, SizeProposal::unspecified()))
            .map(|s| s.width)
            .filter(|w| *w > 0.0);

        let wants = Self::floor(available.min(self.cap.get()));
        let Some(id) = self.column else {
            return (place_pane(available, wants, margin_w), Size::new(wants, 0.0));
        };

        let desired = ctx
            .child_size(id, SizeProposal::with_width(wants))
            .unwrap_or(Size::new(wants, 0.0));
        let first = place_pane(available, desired.width, margin_w);
        if first.column_width >= desired.width {
            return (first, desired);
        }

        // It has to give up width: re-measure so the reported height is the height
        // of the column the writer will actually see.
        let squeezed = ctx
            .child_size(id, SizeProposal::with_width(Self::floor(first.column_width)))
            .unwrap_or(Size::new(first.column_width, desired.height));
        (
            place_pane(available, squeezed.width, margin_w),
            squeezed,
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
        let (_, column) = self.resolve(available, ctx);
        // The column's intrinsic height is the page's height: this sits inside the
        // tab's outer `ScrollArea`, and filling the proposal would make the page
        // exactly one screen tall however long the scene is.
        Size::new(available, column.height).into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        ctx: &LayoutContext,
    ) {
        let (p, column) = self.resolve(bounds.width, ctx);
        for child in children.iter_mut() {
            if Some(child.id) == self.column {
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
    use bastyde::core::widget_tree::WidgetTree;
    use bastyde::widgets::{FixedSize, MaxSize};

    /// A stand-in column with the real one's shape: a `MaxSize` cap over an
    /// `Expand` that fills whatever width it is proposed, so it resolves to
    /// `min(cap, proposed)` exactly as the writing column does.
    fn column(cap: f32) -> impl Widget {
        MaxSize::width(cap).child(
            bastyde::widgets::Expand::horizontal().child(
                FixedSize::new()
                    .height(200.0)
                    .child(bastyde::widgets::Spacer::new()),
            ),
        )
    }

    /// A stand-in margin of a fixed width, or nothing at all.
    fn margin(width: f32) -> impl Widget {
        FixedSize::new()
            .width(width)
            .child(bastyde::widgets::Spacer::new())
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
