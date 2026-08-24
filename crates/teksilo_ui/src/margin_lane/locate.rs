// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **A document character offset to a fraction of the lane.**
//!
//! The one conversion every provider depends on and none of them performs. It is
//! here rather than in each provider because getting it wrong is invisible: marks
//! still appear, still move as the writer types, and are simply in the wrong
//! places — worst exactly where a reader looks to check.
//!
//! ## Why not `offset / character_count`
//!
//! Because it is not proportional to vertical position. A scene break, a heading,
//! a blank line and an inline image each occupy a handful of characters and a
//! whole line or more of height; a long paragraph occupies hundreds of characters
//! and, wrapped, several lines. The character fraction and the pixel fraction
//! agree only for uniform monospaced text, which prose never is.
//!
//! So the conversion goes through the editor's **own laid-out geometry** —
//! [`EditorHandle::offset_content_rect`] over
//! [`EditorHandle::content_height`], both of which are the scroll-free content
//! space added upstream for this.
//!
//! ## Why content space and not window space
//!
//! The window-space [`range_rect`](teksilo::widgets::rich_text::EditorHandle::range_rect)
//! answers "where is this on screen right now", which changes on every scroll tick
//! and is meaningless for an offset the writer has scrolled a thousand lines past
//! — and those are most of the offsets a lane draws. The lane is the fixed map the
//! viewport moves over; mapping window coordinates would slide every mark up the
//! strip as the writer scrolled down.
//!
//! ## The editor's text, not its box
//!
//! The one place this conversion is deliberately approximate. An editor laid out
//! taller than its content — a three-line scene in a pane with a ten-line minimum —
//! reports the *text's* height, so its marks spread across its slot rather than
//! crowding into the top fifth of it and leaving the rest blank.
//!
//! For a tab that is invisible: there is nothing else in the extent and nothing to
//! scroll. For a stream row it means a short row's marks are spread over its slot
//! rather than aligned to the prose inside it. Aligning them exactly would mean
//! threading each editor's content padding and placed box through every provider
//! call, to move a mark by a few pixels on a strip whose smallest mark is three. The
//! spread is the better trade, and it is a choice rather than an oversight.

use crate::widgets::LaneSpan;
use teksilo::widgets::rich_text::EditorHandle;

/// Where one editor's text sits within the whole extent a lane maps.
///
/// A tab maps a single editor and uses [`LaneExtent::WHOLE`]. A **stream** maps
/// many documents down one scroll area, so each row's editor occupies a slice of
/// the lane and its own `0.0..=1.0` has to be placed inside that slice — which is
/// the only thing this type does.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LaneExtent {
    /// Fraction of the lane at which this editor's text begins.
    pub offset: f32,
    /// Fraction of the lane its text spans.
    pub scale: f32,
}

impl LaneExtent {
    /// The editor is the whole extent — one document on one surface.
    pub const WHOLE: Self = Self {
        offset: 0.0,
        scale: 1.0,
    };

    /// A row occupying `[top, top + height)` of a `total`-tall scroll content.
    ///
    /// `None` for a `total` of zero, which is what "not laid out yet" looks like
    /// from here: every row would claim the whole lane, and marks from a hundred
    /// rows would stack on top of each other at the top of the strip.
    pub fn slice(top: f32, height: f32, total: f32) -> Option<Self> {
        (total > 0.0 && height > 0.0).then_some(Self {
            offset: top / total,
            scale: height / total,
        })
    }

    /// Place a `0.0..=1.0` position within this editor onto the whole lane.
    pub fn place(self, local: f32) -> f32 {
        (self.offset + local.clamp(0.0, 1.0) * self.scale).clamp(0.0, 1.0)
    }
}

/// A document character offset as a fraction of the lane.
///
/// The **middle** of the line the offset sits on, not its top: a point mark is
/// grown to a minimum height around the position it is given, and anchoring it to
/// the top of a line would push a mark on the first line half off the strip.
///
/// `None` before the first layout, which is also what a provider gets on the very
/// first frame — and why every provider must handle it rather than substitute a
/// zero. A mark at fraction 0 is not "unknown", it is "the top of the document",
/// stated with total confidence.
pub fn locate_offset(handle: &EditorHandle, extent: LaneExtent, offset: usize) -> Option<f32> {
    let height = handle.content_height()?;
    if height <= 0.0 {
        return None;
    }
    let rect = handle.offset_content_rect(offset)?;
    Some(extent.place((rect.y + rect.height / 2.0) / height))
}

/// A character range as a span of the lane — the top of the first line it touches
/// to the bottom of the last.
///
/// Not two [`locate_offset`] calls: those would each report a line's middle, so a
/// comment on a single line would come out as a zero-height span sitting inside
/// the line rather than covering it.
pub fn locate_span(
    handle: &EditorHandle,
    extent: LaneExtent,
    start: usize,
    end: usize,
) -> Option<LaneSpan> {
    let height = handle.content_height()?;
    if height <= 0.0 {
        return None;
    }
    let rect = handle.range_content_rect(start, end)?;
    Some(LaneSpan::new(
        extent.place(rect.y / height),
        extent.place((rect.y + rect.height) / height),
    ))
}

/// The closure [`LaneContext::locate`](super::LaneContext::locate) is built from.
///
/// Takes the handle by value: the host resolves marks well after the editor it
/// belongs to has gone into the widget tree, and a borrow could not outlive that.
pub fn locator(handle: EditorHandle, extent: LaneExtent) -> impl Fn(usize) -> Option<f32> {
    move |offset| locate_offset(&handle, extent, offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_whole_extent_is_the_identity() {
        assert_eq!(LaneExtent::WHOLE.place(0.0), 0.0);
        assert_eq!(LaneExtent::WHOLE.place(0.5), 0.5);
        assert_eq!(LaneExtent::WHOLE.place(1.0), 1.0);
    }

    /// The stream case: three rows of different weights each map their own text
    /// onto their own slice, and the slices tile the lane without overlapping.
    ///
    /// `slice` is arithmetic and does not care what the numbers mean. What they
    /// mean is decided in [`LaneRows::resolve`](super::super::surface), and it is
    /// deliberately **not** pixel heights — see the reasoning there.
    #[test]
    fn rows_tile_the_lane_in_proportion_to_their_weights() {
        let total = 1000.0;
        let a = LaneExtent::slice(0.0, 200.0, total).unwrap();
        let b = LaneExtent::slice(200.0, 500.0, total).unwrap();
        let c = LaneExtent::slice(700.0, 300.0, total).unwrap();

        assert!(
            (a.place(1.0) - b.place(0.0)).abs() < 1e-6,
            "a's end is b's start"
        );
        assert!(
            (b.place(1.0) - c.place(0.0)).abs() < 1e-6,
            "b's end is c's start"
        );
        assert!(
            (c.place(1.0) - 1.0).abs() < 1e-6,
            "and the last row reaches the bottom"
        );
        assert!(
            (b.place(0.5) - 0.45).abs() < 1e-6,
            "halfway down b is 45% down the lane"
        );
    }

    /// Before layout the total height is zero, and every row would otherwise claim
    /// the whole lane — a hundred rows' marks stacked at the top of the strip.
    #[test]
    fn an_unlaid_out_stream_yields_no_extent_rather_than_the_whole_lane() {
        assert!(LaneExtent::slice(0.0, 100.0, 0.0).is_none());
        assert!(LaneExtent::slice(0.0, 0.0, 1000.0).is_none());
    }

    /// A row whose editor reports a position slightly outside its own content —
    /// which rounding at the very last character does — must not paint outside the
    /// row's slice, or a mark from the last scene lands under the next one.
    #[test]
    fn a_position_outside_the_editor_is_clamped_into_its_own_slice() {
        let row = LaneExtent::slice(200.0, 500.0, 1000.0).unwrap();
        assert!((row.place(1.5) - 0.7).abs() < 1e-6);
        assert!((row.place(-0.5) - 0.2).abs() < 1e-6);
    }
}
