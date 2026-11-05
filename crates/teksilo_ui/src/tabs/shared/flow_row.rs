// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A header row that breaks to a second line rather than squeezing what it holds.
//!
//! An `HStack` with one `Expand` child gives that child whatever is left, down to nothing.
//! In a pane's header, that child is the filter, and the centre column is as narrow as the
//! docks on either side leave it: with a binder and an Inspector open, the corkboard's and
//! the Overview's filters were laid out with a text area of no width at all. They stayed Tab
//! stops, which is how a writer tabbing lands on a field they cannot see and a screen reader
//! announces one with an empty rectangle. The row also reported the sum of its children, so
//! the whole pane grew past the column and the cards below it were cut at the right edge.
//!
//! [`Wrap`](teksilo::widgets::Wrap) answers half of that: it breaks lines, but no child can
//! take up the rest of a line and each line's children sit at its top. `FlowRow` breaks lines
//! the same way, with one child that takes what is left of its line and never gets less than
//! its minimum, and with every line's children centred across it, as an `HStack`'s are.
//!
//! The widgets are the same whichever arrangement a width resolves to, so crossing the
//! breakpoint on a window drag loses neither focus nor what is typed in the filter.

use teksilo::canvas::{Point, Rect};
use teksilo::core::IntoTeksiChild;
use teksilo::core::widget::{PendingChild, WidgetPlacement};
use teksilo::prelude::*;

/// The width a filter keeps before its row breaks: its magnifier and clear button, and enough
/// text between them to see what was typed.
pub(crate) const FILTER_MIN_WIDTH: f32 = 160.0;

/// The gap between the lines of a broken row.
pub(crate) const ROW_LINE_GAP: f32 = 6.0;

/// Children left to right in reading order, breaking to a new line when the next does not
/// fit, with one child taking whatever its line has left. See the [module docs](self).
pub(crate) struct FlowRow {
    spacing: f32,
    line_gap: f32,
    /// In reading order.
    pending: Vec<(PendingChild, Share)>,
    children: Vec<(WidgetId, Share)>,
}

/// How a child claims width on its line.
#[derive(Clone, Copy)]
enum Share {
    /// What it asks for, and not a point more.
    Natural,
    /// What the line has left, down to this.
    Fill(f32),
    /// What the line has left, down to what it asks for.
    FillFromNatural,
}

/// Where one child goes, with `x` from the row's leading edge.
struct Cell {
    id: WidgetId,
    x: f32,
    y: f32,
    size: Size,
}

impl FlowRow {
    pub(crate) fn new(spacing: f32, line_gap: f32) -> Self {
        Self {
            spacing,
            line_gap,
            pending: Vec::new(),
            children: Vec::new(),
        }
    }

    /// A child at its natural width, or the row's width if that is less.
    pub(crate) fn child(mut self, widget: impl IntoTeksiChild) -> Self {
        self.pending.push((widget.into_pending(), Share::Natural));
        self
    }

    /// A child taking what its line has left, and never given less than `min_width` while the
    /// row itself has that much. For a control whose natural width means nothing: a filter.
    pub(crate) fn fill(mut self, widget: impl IntoTeksiChild, min_width: f32) -> Self {
        self.pending
            .push((widget.into_pending(), Share::Fill(min_width)));
        self
    }

    /// A child taking what its line has left, and never given less than it asks for while the
    /// row itself has that much. For text: a breadcrumb holding a long title goes on a line of
    /// its own rather than painting over the count beside it.
    pub(crate) fn fill_from_natural(mut self, widget: impl IntoTeksiChild) -> Self {
        self.pending
            .push((widget.into_pending(), Share::FillFromNatural));
        self
    }

    /// The narrowest this row can honour: its widest rigid child on a line of its own.
    fn min_content_width(&self, ctx: &LayoutContext) -> f32 {
        self.children
            .iter()
            .map(|(id, share)| match share {
                Share::Natural => natural(*id, ctx).width,
                Share::Fill(min) => *min,
                Share::FillFromNatural => 0.0,
            })
            .fold(0.0, f32::max)
    }

    /// Break the children into lines at `width` and place them; the total height comes back
    /// beside the cells.
    fn arrange(&self, width: f32, ctx: &LayoutContext) -> (Vec<Cell>, f32) {
        // Greedy, in reading order. A child that draws nothing takes no gap either, so a
        // label with nothing to say does not leave a hole or cause a break.
        let mut lines: Vec<Vec<(WidgetId, bool, Size)>> = vec![Vec::new()];
        let mut used = 0.0_f32;
        for &(id, share) in &self.children {
            let asked = natural(id, ctx);
            let floor = match share {
                Share::Fill(min) => min,
                Share::Natural | Share::FillFromNatural => asked.width,
            };
            let fills = !matches!(share, Share::Natural);
            let size = Size::new(floor.min(width), asked.height);
            let gap = if used > 0.0 { self.spacing } else { 0.0 };
            if used > 0.0 && size.width > 0.0 && used + gap + size.width > width {
                lines.push(Vec::new());
                used = size.width;
            } else if size.width > 0.0 {
                used += gap + size.width;
            }
            if let Some(line) = lines.last_mut() {
                line.push((id, fills, size));
            }
        }

        let mut cells = Vec::with_capacity(self.children.len());
        let mut y = 0.0_f32;
        for (index, line) in lines.iter().enumerate() {
            let drawn = line.iter().filter(|(_, _, size)| size.width > 0.0).count();
            let widths: f32 = line.iter().map(|(_, _, size)| size.width).sum();
            let gaps = self.spacing * drawn.saturating_sub(1) as f32;
            let spare = (width - widths - gaps).max(0.0);
            // Measured at the width each will really get, for its height: only now is the
            // filling child's width known, and a child cut to the row's width may wrap.
            let sized: Vec<(WidgetId, Size)> = line
                .iter()
                .map(|(id, fills, size)| {
                    let width = size.width + if *fills { spare } else { 0.0 };
                    let height = ctx
                        .child_size(*id, SizeProposal::with_width(width))
                        .map_or(size.height, |measured| measured.height);
                    (*id, Size::new(width, height))
                })
                .collect();
            let height = sized.iter().map(|(_, s)| s.height).fold(0.0, f32::max);
            if index > 0 {
                y += self.line_gap;
            }
            let mut x = 0.0_f32;
            for (id, size) in sized {
                cells.push(Cell {
                    id,
                    x,
                    y: y + (height - size.height) / 2.0,
                    size,
                });
                if size.width > 0.0 {
                    x += size.width + self.spacing;
                }
            }
            y += height;
        }
        (cells, y)
    }
}

/// `id` at the size it asks for, or nothing when it is dormant.
fn natural(id: WidgetId, ctx: &LayoutContext) -> Size {
    ctx.child_size(id, SizeProposal::unspecified())
        .unwrap_or(Size::ZERO)
}

impl std::fmt::Debug for FlowRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FlowRow").finish_non_exhaustive()
    }
}

impl Widget for FlowRow {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        for (child, share) in std::mem::take(&mut self.pending) {
            let id = match child {
                PendingChild::Id(id) => id,
                PendingChild::Deferred(widget) => ctx.add_boxed(widget),
            };
            self.children.push((id, share));
        }
        self.children()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Asked with no width (a `ZStack` around it, a `Panel`, takes its width that way), it
        // answers its narrowest. The sum of its children is exactly the answer that used to
        // push the whole pane past its column.
        let width = proposal
            .width
            .unwrap_or_else(|| self.min_content_width(ctx));
        let (_, height) = self.arrange(width, ctx);
        Size::new(width, height).into()
    }

    fn children(&self) -> Vec<WidgetId> {
        self.children.iter().map(|(id, _)| *id).collect()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        ctx: &LayoutContext,
    ) {
        let (cells, _) = self.arrange(bounds.width, ctx);
        let rtl = ctx.is_rtl();
        for child in children.iter_mut() {
            let Some(cell) = cells.iter().find(|cell| cell.id == child.id) else {
                continue;
            };
            let x = if rtl {
                bounds.x + bounds.width - cell.x - cell.size.width
            } else {
                bounds.x + cell.x
            };
            child.origin = Point::new(x, bounds.y + cell.y);
            child.size = cell.size;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::environment::LayoutDirection;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::widgets::FixedSize;

    /// A row of a filter and two 100-wide neighbours, 30 and 20 tall, with spacing 10.
    fn row(tree: &mut WidgetTree) -> (WidgetId, WidgetId, WidgetId, WidgetId) {
        let filter = tree.add(FixedSize::new().height(20.0));
        let a = tree.add(FixedSize::new().width(100.0).height(30.0));
        let b = tree.add(FixedSize::new().width(100.0).height(20.0));
        let row = tree.add(
            FlowRow::new(10.0, 6.0)
                .fill(filter, 160.0)
                .child(a)
                .child(b),
        );
        (row, filter, a, b)
    }

    fn laid_out(width: f32, direction: LayoutDirection) -> (WidgetTree, [WidgetId; 3]) {
        let mut tree = WidgetTree::new();
        tree.set_layout_direction(direction);
        let (_row, filter, a, b) = row(&mut tree);
        tree.layout(SizeProposal::exact(width, 200.0));
        (tree, [filter, a, b])
    }

    #[test]
    fn with_room_the_filter_takes_what_the_line_leaves() {
        let (tree, [filter, a, b]) = laid_out(600.0, LayoutDirection::LeftToRight);
        assert_eq!(tree.bounds(filter).width, 380.0);
        assert_eq!(tree.bounds(a).x, 390.0);
        assert_eq!(tree.bounds(b).x, 500.0);
        assert_eq!(tree.bounds(a).y, tree.bounds(filter).y - 5.0);
    }

    /// The bug: with 300 to share, an `HStack` left the filter 80 and, narrower still,
    /// nothing. Here it never drops below its minimum, and what no longer fits goes down.
    #[test]
    fn short_of_room_the_line_breaks_and_the_filter_keeps_its_minimum() {
        let (tree, [filter, a, b]) = laid_out(300.0, LayoutDirection::LeftToRight);
        assert_eq!(
            tree.bounds(filter).width,
            190.0,
            "the filter takes what its own line leaves"
        );
        assert_eq!(tree.bounds(a).y, tree.bounds(filter).y - 5.0, "same line");
        assert!(tree.bounds(b).y > tree.bounds(a).y, "the next line");
        assert_eq!(tree.bounds(b).x, 0.0, "from the leading edge");
    }

    #[test]
    fn narrower_than_the_minimum_the_filter_takes_the_whole_row() {
        let (tree, [filter, a, _]) = laid_out(120.0, LayoutDirection::LeftToRight);
        assert_eq!(tree.bounds(filter).width, 120.0);
        assert!(tree.bounds(a).y > tree.bounds(filter).y);
    }

    /// The reading order runs from the right, on every line.
    #[test]
    fn a_right_to_left_row_mirrors_each_line() {
        let (tree, [filter, a, b]) = laid_out(300.0, LayoutDirection::RightToLeft);
        assert_eq!(
            tree.bounds(filter).x,
            110.0,
            "the filter leads, at the right"
        );
        assert_eq!(tree.bounds(a).x, 0.0, "its neighbour on the left of it");
        assert_eq!(
            tree.bounds(b).x,
            200.0,
            "the second line starts at the right"
        );
    }

    /// A child that fills from its natural width goes down a line rather than under its
    /// neighbour, and gets the row to itself there.
    #[test]
    fn a_child_filling_from_its_natural_width_breaks_before_it_is_squeezed() {
        let mut tree = WidgetTree::new();
        let title = tree.add(FixedSize::new().width(250.0).height(20.0));
        let count = tree.add(FixedSize::new().width(100.0).height(20.0));
        tree.add(
            FlowRow::new(10.0, 6.0)
                .fill_from_natural(title)
                .child(count),
        );
        tree.layout(SizeProposal::exact(400.0, 200.0));
        assert_eq!(tree.bounds(title).width, 290.0, "room for both");
        assert_eq!(tree.bounds(count).y, tree.bounds(title).y);

        tree.layout(SizeProposal::exact(300.0, 200.0));
        assert_eq!(tree.bounds(title).width, 300.0, "on a line of its own");
        assert!(tree.bounds(count).y > tree.bounds(title).y);
    }

    /// Asked with no width it answers its narrowest. It used to answer the sum, and a `Panel`
    /// takes its width from that question, which pushed the pane past its column.
    #[test]
    fn an_unbounded_question_gets_the_narrowest_answer() {
        let mut tree = WidgetTree::new();
        let (row, ..) = row(&mut tree);
        tree.layout(SizeProposal::unspecified());
        assert_eq!(tree.bounds(row).width, 160.0);
    }
}
