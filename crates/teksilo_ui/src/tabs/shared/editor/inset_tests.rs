// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A column set in from its leading edge by [`CenterColumnFlowing`]: the geometry
//! [`crate::shared::column_inset`] states, measured on real layouts, in both directions.

use super::*;
use crate::shared::column_inset::ColumnInset;
use crate::test_support::WrapsLikeProse;
use teksilo::canvas::Rect;
use teksilo::core::environment::LayoutDirection;
use teksilo::core::widget_tree::WidgetTree;

/// What a stand-in column of this area is as tall as at `width`.
const AREA: f32 = 120_000.0;

/// A column with the real one's shape (a cap over a fill) whose text wraps.
fn column(cap: f32) -> impl Widget {
    MaxSize::width(cap).child(Expand::horizontal().child(WrapsLikeProse { area: AREA }))
}

/// The column placed in a pane `available` wide, and the height the pane reported.
fn place(available: f32, cap: f32, inset: Option<f32>, rtl: bool) -> (Rect, f32) {
    let mut tree = WidgetTree::new();
    if rtl {
        tree.set_layout_direction(LayoutDirection::RightToLeft);
    }
    let mut flowing = CenterColumnFlowing::new(column(cap));
    if let Some(inset) = inset {
        flowing = flowing.indent(ColumnInset::fixed(inset));
    }
    let root = tree.add(flowing);
    tree.layout(SizeProposal::with_width(available));
    let placed = tree.bounds(tree.children(root)[0]);
    (placed, tree.bounds(root).height)
}

/// A wide pane: the column is centred as it always was, then its left edge moves in and
/// its right edge stays.
#[test]
fn an_inset_column_keeps_its_right_edge() {
    let (plain, _) = place(1400.0, 700.0, None, false);
    assert_eq!((plain.x, plain.width), (350.0, 700.0));
    let (inset, _) = place(1400.0, 700.0, Some(96.0), false);
    assert_eq!((inset.x, inset.width), (446.0, 604.0));
    assert_eq!(
        inset.x + inset.width,
        plain.x + plain.width,
        "the right edge did not move"
    );
}

/// Right to left, the leading edge is the right one.
#[test]
fn right_to_left_the_right_edge_moves_in() {
    let (inset, _) = place(1400.0, 700.0, Some(96.0), true);
    assert_eq!((inset.x, inset.width), (350.0, 604.0));
}

/// A pane narrower than the column: the column fills it, and the inset still comes off
/// the leading edge alone.
#[test]
fn in_a_narrow_pane_the_inset_comes_off_the_leading_edge() {
    let (inset, _) = place(400.0, 700.0, Some(96.0), false);
    assert_eq!((inset.x, inset.width), (96.0, 304.0));
}

/// **The height is the narrowed column's.** Narrower text wraps taller, and a widget that
/// reported the wide measurement would cut the last lines of every indented row out of
/// the page's scrollable extent.
#[test]
fn the_reported_height_is_the_narrowed_columns() {
    let (inset, height) = place(1400.0, 700.0, Some(96.0), false);
    let expected = AREA / 604.0;
    assert!(
        (inset.height - expected).abs() < 0.01,
        "placed {}",
        inset.height
    );
    assert!((height - expected).abs() < 0.01, "reported {height}");
    let (_, plain_height) = place(1400.0, 700.0, None, false);
    assert!(height > plain_height, "and taller than the unindented one");
}

/// No inset, or a zero one, is the unindented placement to the bit.
#[test]
fn no_inset_places_the_column_exactly_as_before() {
    for rtl in [false, true] {
        for available in [1400.0, 400.0, 90.0] {
            let plain = place(available, 700.0, None, rtl);
            assert_eq!(place(available, 700.0, Some(0.0), rtl), plain);
        }
    }
}

/// The guard: an inset larger than the column can give stops at the column's floor.
#[test]
fn an_inset_never_takes_the_column_below_its_floor() {
    let (inset, _) = place(160.0, 700.0, Some(144.0), false);
    assert_eq!(inset.width, MIN_COLUMN_WIDTH);
    assert_eq!(inset.x + inset.width, 160.0);
}
