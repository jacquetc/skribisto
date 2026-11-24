// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A centred writing column **set in from its leading edge**, and where that puts it.
//!
//! Two widgets centre a column in this application:
//! [`CenterColumnFlowing`](crate::tabs::shared::CenterColumnFlowing), for a column alone,
//! and [`ColumnWithMargin`](crate::comments::pane::ColumnWithMargin), for a column with
//! its comment cards beside it. A Full Chapter, Part or Book sets each row in by how deep
//! it sits in the outline, and the row's header, its prose and its synopsis are three
//! columns placed by those two widgets. So the inset is a property of **placement**,
//! handed to both, and its geometry is one function, [`inset_frame`], that both call.
//!
//! ## The geometry
//!
//! The column is first placed exactly as it would be with no inset: centred, capped at
//! its width setting, floored at
//! [`MIN_COLUMN_WIDTH`], and, beside comments,
//! wherever the margin rules put it. Only then does the inset apply, and it moves the
//! **leading** edge alone:
//!
//! ```text
//! left to right:  x = x0 + inset,  width = w0 - inset
//! right to left:  x = x0,          width = w0 - inset
//! ```
//!
//! The trailing edge never moves, which is what keeps three things true down a page of
//! rows at different depths: the status glyphs and menus at the end of each header
//! still line up in one column, a row's comment cards stay flush against its text, and
//! no row can reach past the edge the unindented one stopped at. The column is narrower,
//! so its text wraps again; the widgets measure it again at the new width, and report
//! and place that one measurement, so the height laid out is the height shown.
//!
//! The inset is clamped so a column never goes below `MIN_COLUMN_WIDTH`, the floor the
//! column already has. That is a last guard, not the policy: how far in a row may go is
//! decided before it gets here, by whoever builds the [`ColumnInset`] (for a stream,
//! [`crate::stream::level_inset`], which keeps every level visible however narrow the
//! pane).
//!
//! With no inset (`0.0`), nothing here runs: the placement is the unindented one,
//! measured once, exactly as it was before insets existed.

use std::rc::Rc;

use teksilo::prelude::*;

use crate::tabs::shared::MIN_COLUMN_WIDTH;

/// How far in from its leading edge a column is set, as a function of the width it is
/// placed in, and the signals that move it.
///
/// A function of the width rather than a number, because the policy that decides it may
/// depend on the room there is (see [`crate::stream::level_inset`]). The signals are
/// bound at `Relayout` on the widget that places the column, so a change to any of them
/// moves the column without rebuilding what is inside it: an editor keeps its caret,
/// its selection and its undo history when the setting changes or a part is added above
/// its row.
#[derive(Clone)]
pub struct ColumnInset {
    leading: Rc<dyn Fn(f32) -> f32>,
    bind: Rc<dyn Fn(&mut BuildContext)>,
}

impl std::fmt::Debug for ColumnInset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ColumnInset").finish_non_exhaustive()
    }
}

impl ColumnInset {
    /// An inset computed by `leading` from the width the column is placed in.
    ///
    /// `bind` binds every signal `leading` reads to the widget being built, at
    /// `BindingLevel::Relayout`. A signal read but left unbound would leave the column
    /// where it was when that signal moved, until something unrelated laid the page out
    /// again.
    pub fn new(
        leading: impl Fn(f32) -> f32 + 'static,
        bind: impl Fn(&mut BuildContext) + 'static,
    ) -> Self {
        Self {
            leading: Rc::new(leading),
            bind: Rc::new(bind),
        }
    }

    /// A fixed inset, moved by nothing. For tests, and for a column whose inset is
    /// decided once.
    pub fn fixed(inset: f32) -> Self {
        Self::new(move |_| inset, |_| {})
    }

    /// The inset for a column placed in `available` pixels. Never negative.
    pub fn leading(&self, available: f32) -> f32 {
        (self.leading)(available).max(0.0)
    }

    /// Bind the signals this inset reads to the widget being built. Called from that
    /// widget's own `build`.
    pub fn bind(&self, ctx: &mut BuildContext) {
        (self.bind)(ctx);
    }
}

/// Where a column placed at `x0`, `w0` wide, lands once set in by `inset` from its
/// leading edge: the new `(x, width)`.
///
/// The leading edge is the left one, or the right one when `rtl`; the other edge stays
/// where it was. The inset is clamped so the column keeps at least `MIN_COLUMN_WIDTH`,
/// or keeps its width when it is already narrower than that.
pub fn inset_frame(x0: f32, w0: f32, inset: f32, rtl: bool) -> (f32, f32) {
    let inset = inset.clamp(0.0, (w0 - MIN_COLUMN_WIDTH).max(0.0));
    if rtl {
        (x0, w0 - inset)
    } else {
        (x0 + inset, w0 - inset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Left to right, the left edge moves in and the right edge stays.
    #[test]
    fn an_inset_moves_the_leading_edge_and_keeps_the_trailing_one() {
        let (x, w) = inset_frame(350.0, 700.0, 96.0, false);
        assert_eq!((x, w), (446.0, 604.0));
        assert_eq!(x + w, 350.0 + 700.0, "the right edge did not move");
    }

    /// Right to left, the leading edge is the right one: it moves in, the left stays.
    #[test]
    fn right_to_left_moves_the_right_edge() {
        let (x, w) = inset_frame(350.0, 700.0, 96.0, true);
        assert_eq!((x, w), (350.0, 604.0));
    }

    /// No inset is no change, to the bit.
    #[test]
    fn no_inset_is_the_unindented_frame() {
        for rtl in [false, true] {
            assert_eq!(inset_frame(12.5, 433.0, 0.0, rtl), (12.5, 433.0));
            assert_eq!(inset_frame(12.5, 433.0, -5.0, rtl), (12.5, 433.0));
        }
    }

    /// The guard: an inset larger than the column can give stops at the floor, and a
    /// column already at or below the floor is not narrowed at all.
    #[test]
    fn the_inset_never_takes_a_column_below_its_floor() {
        let (x, w) = inset_frame(0.0, 160.0, 144.0, false);
        assert_eq!(w, MIN_COLUMN_WIDTH);
        assert_eq!(x, 160.0 - MIN_COLUMN_WIDTH);
        assert_eq!(
            inset_frame(0.0, MIN_COLUMN_WIDTH, 48.0, false),
            (0.0, MIN_COLUMN_WIDTH)
        );
        assert_eq!(inset_frame(4.0, 60.0, 48.0, true), (4.0, 60.0));
    }

    /// The width handed to the inset is the width the column is placed in.
    #[test]
    fn a_column_inset_is_computed_from_the_width_it_is_placed_in() {
        let inset = ColumnInset::new(|available| available / 10.0, |_| {});
        assert_eq!(inset.leading(800.0), 80.0);
        let negative = ColumnInset::new(|_| -3.0, |_| {});
        assert_eq!(negative.leading(800.0), 0.0, "never negative");
        assert_eq!(ColumnInset::fixed(32.0).leading(10.0), 32.0);
    }
}
