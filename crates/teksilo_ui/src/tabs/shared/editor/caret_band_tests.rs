// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::typewriter_tests::*;
use super::*;
use teksilo::core::widget_tree::WidgetTree;
use teksilo::text_document::{Color, FlowElementSnapshot, HighlightMask};

use crate::shared::{CaretBand, CaretHighlightSettings, HighlightScope};

const BAND: Color = Color {
    red: 255,
    green: 254,
    blue: 235,
    alpha: 255,
};

fn band(scope: HighlightScope) -> (CaretBand, Signal<HighlightScope>, Signal<Color>) {
    let scope_sig = Signal::new(scope);
    let color_sig = Signal::new(BAND);
    let settings = CaretHighlightSettings::new(scope_sig.clone(), color_sig.clone());
    (
        CaretBand::new(settings, Some("en".into())),
        scope_sig,
        color_sig,
    )
}

/// The banded extents on the document's first block, as `(start, length)`.
fn banded(doc: &TextDocument, color: Color) -> Vec<(usize, usize)> {
    match &doc.snapshot_flow_masked(&HighlightMask::all()).elements[0] {
        FlowElementSnapshot::Block(b) => b
            .paint_highlights
            .iter()
            .filter(|s| s.background_color == Some(color))
            .map(|s| (s.start, s.length))
            .collect(),
        _ => panic!("expected a block"),
    }
}

fn pump(tree: &mut WidgetTree) {
    tree.request_frame();
    tree.tick_animations(std::time::Duration::from_millis(16));
    tree.layout(SizeProposal::exact(900.0, 600.0));
}

/// Click into the column so the editor takes focus. The band deliberately shows only in
/// the **focused** view (that is what keeps a split banding once, not twice), so a test
/// that never focuses would see nothing however well the wiring works.
fn click_into(tree: &mut WidgetTree) {
    let _ = tree.render();
    tree.dispatch_event(teksilo::core::WidgetEvent::PointerDown {
        position: teksilo::canvas::Point::new(450.0, 20.0),
        button: teksilo::core::PointerButton::Primary,
        modifiers: teksilo::core::Modifiers::NONE,
    });
    pump(tree);
    assert!(tree.focused().is_some(), "the click must focus the editor");
}

/// Turning the setting on bands the caret's sentence in a real writing column — no test
/// double anywhere between the preference and the paint span.
#[test]
fn the_setting_reaches_a_real_writing_column() {
    let (band, _scope, _color) = band(HighlightScope::Sentence);
    let (doc, handle, mut tree) = column_with_document(Some(band));

    click_into(&mut tree);
    handle.select_range(2, 2);
    pump(&mut tree);

    assert_eq!(
        banded(&doc, BAND),
        [(0, 23)],
        "the caret's sentence is banded"
    );
}

/// Changing the preference on an already-open editor must reach it, which is what the
/// per-signal effects in `TypographyBoundEditor::build` are for.
#[test]
fn changing_the_scope_reaches_an_already_open_editor() {
    let (band, scope, _color) = band(HighlightScope::None);
    let (doc, handle, mut tree) = column_with_document(Some(band));
    click_into(&mut tree);
    handle.select_range(2, 2);
    pump(&mut tree);
    assert!(banded(&doc, BAND).is_empty(), "off by default here");

    scope.set(HighlightScope::Sentence);
    pump(&mut tree);
    assert_eq!(banded(&doc, BAND), [(0, 23)], "the band appeared");

    scope.set(HighlightScope::None);
    pump(&mut tree);
    assert!(banded(&doc, BAND).is_empty(), "and went away again");
}

/// A live theme switch drives the colour signal, and the band must follow it — otherwise
/// going dark would leave every open editor banded in the light theme's shade.
#[test]
fn changing_the_colour_repaints_an_open_band() {
    const DARK: Color = Color {
        red: 38,
        green: 40,
        blue: 46,
        alpha: 255,
    };
    let (band, _scope, color) = band(HighlightScope::Sentence);
    let (doc, handle, mut tree) = column_with_document(Some(band));
    click_into(&mut tree);
    handle.select_range(2, 2);
    pump(&mut tree);
    assert_eq!(banded(&doc, BAND), [(0, 23)]);

    color.set(DARK);
    pump(&mut tree);
    assert!(banded(&doc, BAND).is_empty(), "the old shade is gone");
    assert_eq!(banded(&doc, DARK), [(0, 23)], "repainted in the new one");
}

/// **A destroyed editor leaves no band on the document.**
///
/// The band is a range session on the *shared* document, so one left behind is still
/// painted by every other view of it — a split pane, a stream row, the docked editor
/// under the distraction-free surface. `CaretHighlightSession`'s own `Drop` retires it,
/// but only once the editor **state** is dropped, and that state is an `Rc` that can
/// outlive the widget: this test holds a handle across the teardown, which is exactly
/// what a stale tab-rebuilt editor amounts to. Without the explicit retire in
/// `TypographyBoundEditor::drop`, the band survives here — and nothing can ever reach
/// it afterwards, because the effects that push it die with the widget.
#[test]
fn destroying_an_editor_takes_its_band_off_the_shared_document() {
    let (band, _scope, _color) = band(HighlightScope::Sentence);
    let (doc, handle, mut tree) = column_with_document(Some(band));
    click_into(&mut tree);
    handle.select_range(2, 2);
    pump(&mut tree);
    assert_eq!(banded(&doc, BAND), [(0, 23)], "banded to begin with");

    // `handle` is deliberately still alive — that is the whole point.
    drop(tree);
    assert!(
        banded(&doc, BAND).is_empty(),
        "the torn-down editor's band is still painted on a document its \
             siblings are showing"
    );
}

/// A surface built with no settings behind it draws nothing — the contract every widget
/// test in this file relies on.
#[test]
fn a_column_without_a_band_draws_none() {
    let (doc, handle, mut tree) = column_with_document(None);
    click_into(&mut tree);
    handle.select_range(2, 2);
    pump(&mut tree);
    assert!(banded(&doc, BAND).is_empty());
    assert!(handle.get_caret_highlight().is_none());
}
