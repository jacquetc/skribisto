// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **How far in each level of a stream is set**: the margin that shows a Full Chapter,
//! Part or Book as an outline, a chapter further in than its part and a scene further
//! in than its chapter.
//!
//! Three numbers decide it, and [`level_inset`] is the whole rule:
//!
//! ```text
//! s   = the step Settings asks for: None 0, Small 16, Medium 32, Large 48 dp
//! R   = min(width the row is placed in, Settings ▸ Text width,
//!           that width less the comment gutter the page reserves)
//! D   = the deepest level among the rows shown (0: nothing to indent)
//! s'  = min(s, R / (3·D))      all the levels together take at most a third of R
//! I_d = d · s'                 for a row d levels deep (1 ≤ d ≤ D)
//! ```
//!
//! **Why the cap, and why on every level at once.** A pane can be narrow (a laptop with
//! the binder and the inspector open), and a Book three levels deep at the large step
//! would take 144 dp off its scenes. Clamping each row on its own would keep the levels
//! apart until the deepest ones hit the clamp together and merged; scaling the step for
//! all of them keeps every level distinct, a scene still one step further in than its
//! chapter, however narrow the pane. The deepest row then keeps two thirds of the
//! column. The step shrinks continuously with the pane, so widening or narrowing the
//! window never makes a row jump.
//!
//! **Why `R` is the reference, for every column of a row.** A row has up to three
//! columns (its header, its prose, its synopsis), and they do not share a width: the
//! synopsis is narrower, and in distraction-free mode the prose follows its own width.
//! Each is set in by the same amount, computed once from the row's place and the text
//! width, so the header and the box under it keep exactly the offset they have with no
//! indent at all. How that amount moves a column is
//! [`crate::shared::column_inset::inset_frame`]'s business.
//!
//! **What the text width does.** It caps `R`. Widening the column in Settings widens
//! every row and leaves the step alone, until a narrow column makes the cap bind; a pane
//! narrower than the column is what usually does.
//!
//! **What comments do.** Once any row on the page has comment cards, the page reserves a
//! gutter for them, and a row whose cards do not fit beside a centred column has its text
//! squeezed into the width left beside them. `R` is never wider than that width, for
//! every row and for every column of a row: so a row's title and its text, which the
//! squeezing pushes against the same leading edge, are set in by the same amount and stay
//! aligned, the step is the same on every row of the page, and the deepest text keeps two
//! thirds of the width it is squeezed to. With no comments on the page the gutter is zero
//! and this changes nothing.

use serde::{Deserialize, Serialize};
use teksilo::core::BindingLevel;
use teksilo::prelude::*;

use crate::shared::column_inset::ColumnInset;

/// How much the levels of a stream are set in: Settings ▸ Editor ▸ Editor Behavior ▸
/// Indent by level. Stored by variant name under [`crate::STREAM_INDENT_KEY`].
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamIndent {
    /// Every row on one line, as before the indent existed.
    None,
    /// A hint of the outline.
    Small,
    /// The outline at a glance. The default.
    #[default]
    Medium,
    /// The outline first.
    Large,
}

impl StreamIndent {
    /// Every setting, from none to the largest.
    pub fn all() -> [Self; 4] {
        [Self::None, Self::Small, Self::Medium, Self::Large]
    }

    /// This setting's slot in the `SegmentedControl`. Paired with
    /// [`from_index`](Self::from_index).
    pub fn to_index(self) -> usize {
        match self {
            Self::None => 0,
            Self::Small => 1,
            Self::Medium => 2,
            Self::Large => 3,
        }
    }

    /// The setting at a control slot; out of range reads as the default.
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::None,
            1 => Self::Small,
            2 => Self::Medium,
            3 => Self::Large,
            _ => Self::default(),
        }
    }

    /// How far in each level is set, in logical pixels, before [`level_inset`] fits
    /// the levels to the room there is.
    pub fn step(self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Small => 16.0,
            Self::Medium => 32.0,
            Self::Large => 48.0,
        }
    }
}

/// The most of the reference width all the levels together may take.
pub const INDENT_SHARE: f32 = 1.0 / 3.0;

/// How far in a row `depth` levels deep is set, when the deepest row shown is `deepest`
/// levels deep, each level asks for `step`, and the row's reference width is
/// `reference`. See the [module docs](self) for the rule and its reasons.
///
/// Zero for the container's own surfaces (depth 0), for a stream with no rows, and for a
/// step of zero.
pub fn level_inset(step: f32, reference: f32, depth: u8, deepest: u8) -> f32 {
    if depth == 0 || deepest == 0 || step <= 0.0 {
        return 0.0;
    }
    let per_level = step.min(reference.max(0.0) * INDENT_SHARE / f32::from(deepest));
    f32::from(depth.min(deepest)) * per_level
}

/// The width a row's levels are measured against, `R` in the [module docs](self): the
/// narrower of the width the row is placed in and the text width, and never wider than
/// what is left beside the comment `gutter` the page reserves (zero with no comments).
///
/// What is left beside the gutter is floored at the column's own floor, so a pane too
/// narrow for any column beside the cards still measures something.
pub fn reference_width(available: f32, text_width: f32, gutter: f32) -> f32 {
    let beside = if gutter > 0.0 {
        (available - gutter).max(crate::tabs::shared::MIN_COLUMN_WIDTH)
    } else {
        available
    };
    available.min(text_width).min(beside)
}

/// The inset every column of one stream row shares: its depth, the deepest level the
/// page shows, the setting, the text width and the page's comment gutter it is measured
/// against (see [`reference_width`]), all live.
///
/// Every column of the row (its header, its prose, its synopsis) asks with the width of
/// the row, so they all get the same number, comments or not.
///
/// The five signals are bound at `Relayout` on each widget that places one of the row's
/// columns, so a new setting, a part added above the row, a filter that changes the
/// deepest level or a first comment on the page moves the row without rebuilding its
/// editors.
pub fn row_inset(
    depth: Signal<u8>,
    deepest: Signal<u8>,
    scale: Signal<StreamIndent>,
    column_width: Signal<f32>,
    gutter: Signal<f32>,
) -> ColumnInset {
    let bind = {
        let (depth, deepest, scale, column_width, gutter) = (
            depth.clone(),
            deepest.clone(),
            scale.clone(),
            column_width.clone(),
            gutter.clone(),
        );
        move |ctx: &mut BuildContext| {
            let me = ctx.self_id();
            depth.bind_to(me, ctx.binding_registry(), BindingLevel::Relayout);
            deepest.bind_to(me, ctx.binding_registry(), BindingLevel::Relayout);
            scale.bind_to(me, ctx.binding_registry(), BindingLevel::Relayout);
            column_width.bind_to(me, ctx.binding_registry(), BindingLevel::Relayout);
            gutter.bind_to(me, ctx.binding_registry(), BindingLevel::Relayout);
        }
    };
    ColumnInset::new(
        move |available| {
            level_inset(
                scale.get().step(),
                reference_width(available, column_width.get(), gutter.get()),
                depth.get(),
                deepest.get(),
            )
        },
        bind,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const LARGE: f32 = 48.0;
    const MEDIUM: f32 = 32.0;

    /// A wide pane: every level takes the full step.
    #[test]
    fn with_room_every_level_takes_the_full_step() {
        let at = |d| level_inset(LARGE, 700.0, d, 3);
        assert_eq!([at(1), at(2), at(3)], [48.0, 96.0, 144.0]);
    }

    /// A narrow pane: the step shrinks for every level at once, so the levels stay apart
    /// and the deepest row keeps two thirds of the reference.
    #[test]
    fn a_narrow_pane_scales_every_level_together() {
        let at = |d| level_inset(LARGE, 360.0, d, 3);
        assert_eq!([at(1), at(2), at(3)], [40.0, 80.0, 120.0]);
        assert_eq!(360.0 - at(3), 240.0, "two thirds of the reference are left");
        // The medium step fits in the same pane untouched.
        assert_eq!(level_inset(MEDIUM, 360.0, 3, 3), 96.0);
    }

    /// The cap follows how deep the page actually goes: a Part's two levels have more
    /// room each than a Book's three.
    #[test]
    fn a_shallower_page_gives_each_level_more_room() {
        assert_eq!(level_inset(LARGE, 240.0, 2, 3), 2.0 * 240.0 / 9.0);
        assert_eq!(level_inset(LARGE, 240.0, 2, 2), 2.0 * 40.0);
    }

    /// The step shrinks continuously with the pane: no width makes it jump.
    #[test]
    fn the_inset_never_jumps_as_the_pane_narrows() {
        let mut last = level_inset(LARGE, 1200.0, 3, 3);
        let mut width = 1200.0;
        while width > 0.0 {
            width -= 1.0;
            let now = level_inset(LARGE, width, 3, 3);
            assert!(
                now <= last && last - now <= 1.0 + f32::EPSILON,
                "{last} -> {now} at {width}"
            );
            last = now;
        }
        assert_eq!(last, 0.0);
    }

    /// Nothing to indent: the container's own surfaces, an empty page, the None setting.
    #[test]
    fn nothing_is_set_in_without_a_depth_a_row_or_a_step() {
        assert_eq!(level_inset(LARGE, 700.0, 0, 3), 0.0);
        assert_eq!(level_inset(LARGE, 700.0, 1, 0), 0.0);
        assert_eq!(level_inset(StreamIndent::None.step(), 700.0, 3, 3), 0.0);
        assert_eq!(level_inset(LARGE, -10.0, 2, 3), 0.0);
    }

    /// A row deeper than the page reports (it cannot be, by construction) is held at
    /// the deepest level rather than pushed past it.
    #[test]
    fn a_row_never_goes_past_the_deepest_level() {
        assert_eq!(
            level_inset(LARGE, 700.0, 5, 3),
            level_inset(LARGE, 700.0, 3, 3)
        );
    }

    /// The control's slots round-trip, and an unknown slot reads as the default.
    #[test]
    fn every_setting_has_its_slot() {
        for indent in StreamIndent::all() {
            assert_eq!(StreamIndent::from_index(indent.to_index()), indent);
        }
        assert_eq!(StreamIndent::from_index(9), StreamIndent::default());
        assert_eq!(StreamIndent::default(), StreamIndent::Medium);
        let steps: Vec<f32> = StreamIndent::all().iter().map(|i| i.step()).collect();
        assert!(
            steps.windows(2).all(|w| w[0] < w[1]),
            "steps grow: {steps:?}"
        );
    }

    /// The reference is the narrowest of the row, the text width and what the page's
    /// comment gutter leaves; with no gutter, the gutter changes nothing.
    #[test]
    fn the_reference_is_never_wider_than_what_the_comments_leave() {
        assert_eq!(reference_width(1400.0, 700.0, 0.0), 700.0);
        assert_eq!(reference_width(512.0, 700.0, 0.0), 512.0);
        // A wide pane: the cards fit beside a full column, the reference is the column.
        assert_eq!(reference_width(1400.0, 700.0, 328.0), 700.0);
        // A tight one: the text is squeezed to what the cards leave, and so is `R`.
        assert_eq!(reference_width(512.0, 700.0, 328.0), 184.0);
        // Narrower still: floored at the column's own floor.
        assert_eq!(
            reference_width(380.0, 700.0, 328.0),
            crate::tabs::shared::MIN_COLUMN_WIDTH
        );
    }

    /// The row's inset reads its signals live, against the narrowest of the pane, the
    /// text width and what the page's comment gutter leaves.
    #[test]
    fn a_row_inset_follows_its_signals() {
        let depth = Signal::new(2u8);
        let deepest = Signal::new(3u8);
        let scale = Signal::new(StreamIndent::Large);
        let width = Signal::new(700.0f32);
        let gutter = Signal::new(0.0f32);
        let inset = row_inset(
            depth.clone(),
            deepest.clone(),
            scale.clone(),
            width.clone(),
            gutter.clone(),
        );
        assert_eq!(inset.leading(1400.0), 96.0);
        // The pane narrower than the text width: the pane is the reference.
        assert_eq!(inset.leading(360.0), 80.0);
        // The text width narrower than the pane: the text width is.
        width.set(360.0);
        assert_eq!(inset.leading(1400.0), 80.0);
        width.set(700.0);
        scale.set(StreamIndent::Small);
        assert_eq!(inset.leading(1400.0), 32.0);
        depth.set(1);
        assert_eq!(inset.leading(1400.0), 16.0);
        scale.set(StreamIndent::None);
        assert_eq!(inset.leading(1400.0), 0.0);
        deepest.set(0);
        scale.set(StreamIndent::Large);
        assert_eq!(inset.leading(1400.0), 0.0, "an empty page indents nothing");
        // A first comment on the page in a tight pane: every row measures against what
        // the cards leave, 184 of 512.
        deepest.set(3);
        depth.set(3);
        gutter.set(328.0);
        assert!((inset.leading(512.0) - 3.0 * 184.0 / 9.0).abs() < 0.001);
    }
}
