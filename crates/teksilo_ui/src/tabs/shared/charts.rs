// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Sizing for the container tabs' charts — Pace's writing plan and Analysis's Shape.
//!
//! ## Why a chart is sized from its data, not from its viewport
//!
//! `teksilo_charts::BarChart` derives bar width from the plot width it is handed:
//! `((plot.width - gaps) / n).max(BAR_MIN_WIDTH)`, and that floor is 4 px. Once a series is
//! long enough to hit it — a 150-scene book, or a project that has been going for a year —
//! the bars stop fitting and simply keep advancing past the plot's right edge. The chart
//! does not merely look cramped at that point; it draws outside itself.
//!
//! So a chart gets [`BAR_PITCH`] of horizontal room *per datum* and scrolls when that
//! exceeds the viewport. A bar stays a bar whether the book has thirty scenes or three
//! hundred, and a line chart keeps a readable slope instead of compressing a year into a
//! thumbnail.
//!
//! It is also what makes the framework's tilt-and-thin axis labels useful. Rotation
//! multiplies how many labels fit; it cannot create room that is not there. Real width per
//! datum is what creates the room.
//!
//! ## Why it is shared
//!
//! Pace and Analysis chart the same manuscript. When they sized themselves differently the
//! same book looked like two different shapes, which is worse than either choice on its own.

use teksilo::prelude::*;
use teksilo::widgets::{FixedSize, ScrollArea, ScrollBarPolicy};

/// Height of a primary chart body.
pub const CHART_HEIGHT: f32 = 360.0;

/// Height of a secondary strip beneath a primary chart.
pub const STRIP_HEIGHT: f32 = 260.0;

/// Horizontal room per datum — the bar plus its gap, or one step along a line.
///
/// Wide enough that a bar reads as a bar and a category label has a slot to sit in, rather
/// than the 4 px the chart falls back to when it is squeezed.
pub const BAR_PITCH: f32 = 28.0;

/// A chart sized to `n` data points, scrolling horizontally when that exceeds the viewport.
///
/// The vertical scroll bar is switched off: the height is fixed by the caller, so a vertical
/// bar here would only ever be a second, redundant control inside the page's own scroll.
///
/// `preferred_height` is what makes the chart the height it asked for. The page around it is
/// a vertical `ScrollArea`, so the column laying these out proposes an *unbounded* height and
/// asks each child how tall it would like to be — and a `ScrollArea` with nothing configured
/// answers with a 200 px constant, which is neither the chart's height nor anything to do
/// with this book. The chart inside was correctly 360 px tall the whole time; the viewport
/// around it collapsed to 200 and clipped it.
pub fn wide_chart(n: usize, height: f32, chart: impl Widget + 'static) -> impl Widget {
    ScrollArea::new()
        .vertical_scroll_bar_policy(ScrollBarPolicy::AlwaysOff)
        .preferred_height(height)
        .child(
            FixedSize::new()
                .width(content_width(n))
                .height(height)
                .child(chart),
        )
}

/// The width `n` data points need. Never narrower than a comfortable viewport, so a short
/// series still fills the pane instead of huddling at the left.
pub fn content_width(n: usize) -> f32 {
    (n as f32 * BAR_PITCH).max(MIN_CONTENT_WIDTH)
}

/// Below this a chart looks lost rather than compact — a three-day project should still
/// span the pane.
const MIN_CONTENT_WIDTH: f32 = 480.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_long_series_gets_room_rather_than_thinner_bars() {
        // The case the 4px floor broke: 150 scenes.
        assert!(
            content_width(150) >= 150.0 * BAR_PITCH,
            "every datum keeps its pitch however long the series is"
        );
    }

    #[test]
    fn a_short_series_still_fills_the_pane() {
        assert_eq!(content_width(1), MIN_CONTENT_WIDTH);
        assert_eq!(content_width(0), MIN_CONTENT_WIDTH);
    }

    /// The width has to grow strictly with the data, or the floor reappears at some size.
    #[test]
    fn width_grows_with_the_series() {
        assert!(content_width(300) > content_width(150));
        assert!(content_width(150) > content_width(30));
    }

    /// A bar's share of the width is what the chart divides down to; it must clear the
    /// framework's 4px floor by a wide margin at every size, or nothing above is true.
    #[test]
    fn every_datum_clears_the_frameworks_minimum_bar_width() {
        for n in [1usize, 30, 150, 400] {
            let per_datum = content_width(n) / n as f32;
            assert!(per_datum >= 8.0, "n={n} gives only {per_datum}px per datum");
        }
    }
}
