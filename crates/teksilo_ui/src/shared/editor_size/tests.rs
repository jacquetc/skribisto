// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The size policy on its own — no widget tree, because clamping, snapping and
//! notch accumulation are plain arithmetic and deserve to fail loudly rather
//! than through a rendered editor.
//!
//! The wiring these back — that a real writing column's wheel reaches them — is
//! pinned by `tabs::shared::editor::text_size_tests`.

use super::*;
use crate::settings::TypographySizeRange;

fn typo(start: f32) -> EditorTypography {
    EditorTypography {
        font_family: Signal::new("Literata".to_string()),
        size: Signal::new(start),
        line_height: Signal::new(1.5),
        first_line_indent: Signal::new(0.0),
        para_spacing_before: Signal::new(0.0),
        para_spacing_after: Signal::new(0.0),
        size_range: TypographySizeRange::standard(TypographyKind::Scene, 1.0),
    }
}

#[test]
fn a_step_moves_by_exactly_one_grid_step() {
    let t = typo(1.0);
    assert_eq!(step(&t, 1), 1.05);
    assert_eq!(step(&t, -1), 1.0);
    assert_eq!(step(&t, 3), 1.15);
}

#[test]
fn stepping_clamps_at_both_ends() {
    let t = typo(1.0);
    assert_eq!(step(&t, 100), crate::EDITOR_TYPO_SIZE_MAX);
    assert_eq!(step(&t, -100), crate::EDITOR_TYPO_SIZE_MIN);
}

/// Repeated `f32` addition drifts off the grid (`1.0 + 0.05×3` is not `1.15`),
/// and the Settings slider can only land on grid values — so an un-snapped
/// wheel would set sizes the slider could never show or return to.
#[test]
fn every_stepped_value_lands_on_the_grid() {
    let t = typo(crate::EDITOR_TYPO_SIZE_MIN);
    for _ in 0..18 {
        let v = step(&t, 1);
        let steps = (v - crate::EDITOR_TYPO_SIZE_MIN) / crate::EDITOR_TYPO_SIZE_STEP;
        assert!(
            (steps - steps.round()).abs() < 1e-4,
            "{v} is not a whole number of steps off the floor"
        );
    }
}

#[test]
fn reset_restores_the_bundles_own_default() {
    let t = typo(1.45);
    assert_eq!(reset(&t), 1.0);
    assert_eq!(t.size.get(), 1.0);
}

/// A bundle's default need not be the middle of its range — the synopsis starts
/// at 0.85, the corkboard at 0.8, distraction-free at 1.15 — so reset reads the
/// bundle rather than assuming 100 %.
#[test]
fn reset_is_per_bundle_not_a_universal_one() {
    let t = EditorTypography {
        size_range: TypographySizeRange::standard(TypographyKind::Synopsis, 0.85),
        ..typo(1.3)
    };
    assert_eq!(reset(&t), 0.85);
}

/// **The sign test.** teksilo's `ScrollDelta` is a scroll-*offset* delta: the
/// platform layer negates winit so positive `y` grows an offset, i.e. positive
/// `y` is the wheel turning down. The accumulator undoes that once, here, so
/// both call sites inherit it.
#[test]
fn wheel_up_is_positive_notches() {
    let mut acc = WheelAccumulator::default();
    assert_eq!(
        acc.feed(ScrollDelta::Lines { x: 0.0, y: -3.0 }),
        Some(1),
        "wheel up (negative y) must be a positive, growing notch"
    );

    let mut acc = WheelAccumulator::default();
    assert_eq!(acc.feed(ScrollDelta::Lines { x: 0.0, y: 3.0 }), Some(-1));
}

/// One physical detent arrives as three lines, not one — the platform layer
/// multiplies by `LINES_PER_NOTCH` before any widget sees it.
#[test]
fn three_lines_is_one_notch() {
    let mut acc = WheelAccumulator::default();
    assert_eq!(acc.feed(ScrollDelta::Lines { x: 0.0, y: -1.0 }), None);
    assert_eq!(acc.feed(ScrollDelta::Lines { x: 0.0, y: -1.0 }), None);
    assert_eq!(acc.feed(ScrollDelta::Lines { x: 0.0, y: -1.0 }), Some(1));
}

/// Wayland's own delivery path: detents arrive as pixels.
#[test]
fn pixels_accumulate_to_whole_notches() {
    let mut acc = WheelAccumulator::default();
    for _ in 0..3 {
        assert_eq!(acc.feed(ScrollDelta::Pixels { x: 0.0, y: -12.0 }), None);
    }
    assert_eq!(acc.feed(ScrollDelta::Pixels { x: 0.0, y: -12.0 }), Some(1));
}

/// A fast flick delivers several notches in one event; none may be dropped.
#[test]
fn a_flick_spends_every_notch_it_carries() {
    let mut acc = WheelAccumulator::default();
    assert_eq!(acc.feed(ScrollDelta::Lines { x: 0.0, y: -9.0 }), Some(3));
}

/// Reversing direction must not leave a stale remainder pushing the next notch
/// the wrong way.
#[test]
fn the_remainder_survives_a_direction_change() {
    let mut acc = WheelAccumulator::default();
    assert_eq!(acc.feed(ScrollDelta::Pixels { x: 0.0, y: -24.0 }), None);
    // Half a notch up, then half a notch down: back to nothing owed.
    assert_eq!(acc.feed(ScrollDelta::Pixels { x: 0.0, y: 24.0 }), None);
    // …so a full notch down now steps down, not up.
    assert_eq!(acc.feed(ScrollDelta::Pixels { x: 0.0, y: 48.0 }), Some(-1));
}

/// The range travels with the bundle, so the corkboard's expanded editor can
/// reach a size no other surface may.
#[test]
fn the_expanded_card_editor_has_its_own_ceiling() {
    let t = EditorTypography {
        size_range: TypographySizeRange {
            min: crate::CORKBOARD_MODAL_SIZE_MIN,
            max: crate::CORKBOARD_MODAL_SIZE_MAX,
            step: crate::EDITOR_TYPO_SIZE_STEP,
            default: crate::CORKBOARD_MODAL_SIZE_DEFAULT,
            kind: TypographyKind::CorkboardExpanded,
        },
        ..typo(1.0)
    };
    assert_eq!(step(&t, 100), crate::CORKBOARD_MODAL_SIZE_MAX);
}
