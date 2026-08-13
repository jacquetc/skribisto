// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Ctrl+Wheel over a real writing column, through the whole chain: the wheel
//! handler `TypographyBoundEditor` attaches, the shared step/clamp/snap policy,
//! and the settings `Signal` that fans the result back out.
//!
//! No test double between the gesture and the preference — which is the point,
//! because the two hazards here are both invisible to inspection: the sign of
//! teksilo's `ScrollDelta` (positive `y` is wheel-**down**), and whether the
//! handler runs early enough to stop the page scrolling underneath it.

use super::*;
use teksilo::core::widget_tree::WidgetTree;
use teksilo::core::{Modifiers, ScrollDelta, WidgetEvent};

use crate::settings::{TypographyKind, TypographySizeRange};

/// A bundle whose size can actually travel, unlike the tests' default fixture.
/// The kind travels inside `range`, which is where the size policy reads it.
fn typo_for(start: f32, range: TypographySizeRange) -> EditorTypography {
    EditorTypography {
        font_family: Signal::new("Literata".to_string()),
        size: Signal::new(start),
        line_height: Signal::new(1.5),
        first_line_indent: Signal::new(0.0),
        para_spacing_before: Signal::new(0.0),
        para_spacing_after: Signal::new(0.0),
        size_range: range,
    }
}

fn scene_typo(start: f32) -> EditorTypography {
    typo_for(
        start,
        TypographySizeRange::standard(TypographyKind::Scene, crate::SCENE_SIZE_DEFAULT),
    )
}

/// A real writing column over `typo`, laid out and rendered so the tree can hit-test.
fn column_over(typo: &EditorTypography) -> WidgetTree {
    let doc = TextDocument::new();
    doc.set_plain_text("Some prose to write in.").unwrap();
    let col = writing_column(
        &doc,
        &Signal::new(700.0),
        typo,
        MAIN_MIN_LINES,
        || {},
        None,
        Some(crate::search::FindViewModel::new(doc.clone())),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        // No project around this tree to tally typing against.
        None,
        false,
    );
    let mut tree = WidgetTree::new();
    tree.add(col);
    tree.layout(SizeProposal::exact(900.0, 600.0));
    let _ = tree.render();
    tree
}

/// Put the pointer over the prose, so `Scroll` has a hovered target to walk up
/// from — the dispatcher sends a wheel to `hovered.or(focused)`, and a headless
/// tree has hovered nothing until it is told.
fn hover_prose(tree: &mut WidgetTree) {
    tree.dispatch_event(WidgetEvent::PointerMove {
        position: teksilo::canvas::Point::new(450.0, 20.0),
    });
}

fn wheel(tree: &mut WidgetTree, delta: ScrollDelta, modifiers: Modifiers) {
    tree.dispatch_event(WidgetEvent::Scroll { delta, modifiers });
}

/// One physical notch, as the platform layer delivers it (3 lines per detent).
fn notch(down: bool) -> ScrollDelta {
    ScrollDelta::Lines {
        x: 0.0,
        y: if down { 3.0 } else { -3.0 },
    }
}

/// Wheel **up** grows the text.
///
/// This is the test that earns its keep: teksilo negates winit's wheel so that
/// positive `y` grows a *scroll offset* — i.e. positive is wheel-down. Every
/// consumer mapping a notch to a value rather than an offset has to undo that,
/// and teksilo's own `SpinBox` shipped with it backwards. Reading the handler
/// cannot tell you which way it goes; this can.
#[test]
fn ctrl_wheel_up_grows_the_text() {
    let typo = scene_typo(1.0);
    let mut tree = column_over(&typo);
    hover_prose(&mut tree);

    wheel(&mut tree, notch(false), Modifiers::CTRL);

    assert_eq!(typo.size.get(), 1.05, "wheel up must grow the text");
}

#[test]
fn ctrl_wheel_down_shrinks_the_text() {
    let typo = scene_typo(1.0);
    let mut tree = column_over(&typo);
    hover_prose(&mut tree);

    wheel(&mut tree, notch(true), Modifiers::CTRL);

    assert_eq!(typo.size.get(), 0.95, "wheel down must shrink the text");
}

/// An unmodified wheel is not ours: it must fall through to the page.
#[test]
fn a_plain_wheel_does_not_resize() {
    let typo = scene_typo(1.0);
    let mut tree = column_over(&typo);
    hover_prose(&mut tree);

    wheel(&mut tree, notch(true), Modifiers::NONE);

    assert_eq!(typo.size.get(), 1.0, "an unmodified wheel must only scroll");
}

/// ⌘+Wheel is the same gesture on macOS.
#[test]
fn super_wheel_resizes_too() {
    let typo = scene_typo(1.0);
    let mut tree = column_over(&typo);
    hover_prose(&mut tree);

    wheel(&mut tree, notch(false), Modifiers::SUPER);

    assert_eq!(typo.size.get(), 1.05);
}

/// The wheel stops at the bundle's ceiling and floor — the same ones the
/// Settings slider offers, so a gesture can never set a size the slider cannot
/// show.
#[test]
fn the_wheel_clamps_to_the_bundles_own_range() {
    let typo = scene_typo(1.0);
    let mut tree = column_over(&typo);
    hover_prose(&mut tree);

    for _ in 0..40 {
        wheel(&mut tree, notch(false), Modifiers::CTRL);
    }
    assert_eq!(typo.size.get(), crate::EDITOR_TYPO_SIZE_MAX);

    for _ in 0..80 {
        wheel(&mut tree, notch(true), Modifiers::CTRL);
    }
    assert_eq!(typo.size.get(), crate::EDITOR_TYPO_SIZE_MIN);
}

/// The corkboard's expanded editor has a *higher* ceiling than every other
/// bundle, and reaches it — the regression test for the whole reason the range
/// travels with the bundle. `modal_typo` builds it with struct-update syntax,
/// so an inherited range would be invisible in the source.
#[test]
fn the_expanded_card_editor_climbs_past_the_shared_ceiling() {
    let typo = typo_for(
        1.0,
        TypographySizeRange {
            min: crate::CORKBOARD_MODAL_SIZE_MIN,
            max: crate::CORKBOARD_MODAL_SIZE_MAX,
            step: crate::EDITOR_TYPO_SIZE_STEP,
            default: crate::CORKBOARD_MODAL_SIZE_DEFAULT,
            kind: TypographyKind::CorkboardExpanded,
        },
    );
    let mut tree = column_over(&typo);
    hover_prose(&mut tree);

    for _ in 0..40 {
        wheel(&mut tree, notch(false), Modifiers::CTRL);
    }

    assert_eq!(
        typo.size.get(),
        crate::CORKBOARD_MODAL_SIZE_MAX,
        "the expanded editor must reach 2.0, not stop at the shared 1.6"
    );
}

/// Wayland delivers detents as *pixels*, and a trackpad as a continuous dribble
/// of them. A step per event would make the gesture unusable there; the
/// accumulator must carry the remainder and spend it a whole notch at a time.
#[test]
fn a_pixel_dribble_steps_once_per_notch_not_once_per_event() {
    let typo = scene_typo(1.0);
    let mut tree = column_over(&typo);
    hover_prose(&mut tree);

    // A notch is 48 px; four 12 px events make exactly one.
    for _ in 0..3 {
        wheel(
            &mut tree,
            ScrollDelta::Pixels { x: 0.0, y: -12.0 },
            Modifiers::CTRL,
        );
    }
    assert_eq!(
        typo.size.get(),
        1.0,
        "three quarters of a notch must not step"
    );

    wheel(
        &mut tree,
        ScrollDelta::Pixels { x: 0.0, y: -12.0 },
        Modifiers::CTRL,
    );
    assert_eq!(typo.size.get(), 1.05, "the fourth completes the notch");
}

/// Every value the wheel writes must be one the slider can also produce.
/// Repeated `+= 0.05` in `f32` drifts (`1.1500001`), and the two surfaces would
/// quietly stop agreeing about what the size is.
#[test]
fn wheeled_values_stay_on_the_sliders_grid() {
    let typo = scene_typo(crate::EDITOR_TYPO_SIZE_MIN);
    let mut tree = column_over(&typo);
    hover_prose(&mut tree);

    for i in 1..=9 {
        wheel(&mut tree, notch(false), Modifiers::CTRL);
        let expected = crate::EDITOR_TYPO_SIZE_MIN + i as f32 * crate::EDITOR_TYPO_SIZE_STEP;
        let actual = typo.size.get();
        assert!(
            (actual - expected).abs() < 1e-6,
            "step {i}: expected {expected}, got {actual}"
        );
        // …and it must be an exact multiple of the step off the floor, not
        // merely close to one.
        let steps = (actual - crate::EDITOR_TYPO_SIZE_MIN) / crate::EDITOR_TYPO_SIZE_STEP;
        assert!(
            (steps - steps.round()).abs() < 1e-4,
            "step {i}: {actual} is off the grid"
        );
    }
}
