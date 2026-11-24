// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The fade around the caret, from the settings bundle to a real writing column. What the
//! framework then draws is teksilo's to test (`rich_text::caret_dim`); these pin that every
//! change to the preference reaches the editor, and that a torn-down editor lets go of it.

use super::caret_band_tests::{click_into, pump};
use super::typewriter_tests::*;
use super::*;
use teksilo::widgets::rich_text::caret_highlight::CaretHighlightScope;

use crate::shared::{CaretBand, CaretDimSettings, CaretHighlightSettings, DimScope, DimStrength};

/// A caret bundle with no band and a fade over the given live signals.
fn faded(enabled: bool, scope: DimScope) -> (CaretBand, CaretDimSettings) {
    let dim = CaretDimSettings::new(
        Signal::new(enabled),
        Signal::new(scope),
        Signal::new(DimStrength::Medium),
    );
    (
        CaretBand::new(
            CaretHighlightSettings::off(),
            dim.clone(),
            Some("en".into()),
        ),
        dim,
    )
}

/// Turning the setting on reaches a real writing column, scope, strength and language
/// together, with no test double between the preference and the editor.
#[test]
fn the_setting_reaches_a_real_writing_column() {
    let (band, _) = faded(true, DimScope::Sentence);
    let (_doc, handle, mut tree) = column_with_document(Some(band));
    click_into(&mut tree);

    let dim = handle
        .get_caret_dim()
        .expect("the column was asked to fade");
    assert_eq!(dim.scope, CaretHighlightScope::Sentence);
    assert_eq!(dim.opacity, DimStrength::Medium.opacity());
    assert_eq!(dim.content_locale.as_deref(), Some("en"));
}

/// The quick toggle (F8, the title bar, the menu, the strip) flips the switch while editors
/// are open; each change must reach them, which is what the per-signal effects in
/// `TypographyBoundEditor::build` are for. Scope and strength survive the round trip.
#[test]
fn toggling_and_retuning_reach_an_already_open_editor() {
    let (band, dim) = faded(false, DimScope::Paragraph);
    let (_doc, handle, mut tree) = column_with_document(Some(band));
    click_into(&mut tree);
    assert!(handle.get_caret_dim().is_none(), "off to begin with");

    dim.enabled.set(true);
    pump(&mut tree);
    assert_eq!(
        handle.get_caret_dim().map(|d| d.scope),
        Some(CaretHighlightScope::Paragraph)
    );

    dim.strength.set(DimStrength::Strong);
    pump(&mut tree);
    assert_eq!(
        handle.get_caret_dim().map(|d| d.opacity),
        Some(DimStrength::Strong.opacity())
    );

    dim.scope.set(DimScope::Sentence);
    pump(&mut tree);
    assert_eq!(
        handle.get_caret_dim().map(|d| d.scope),
        Some(CaretHighlightScope::Sentence)
    );

    dim.enabled.set(false);
    pump(&mut tree);
    assert!(handle.get_caret_dim().is_none(), "and off again");

    dim.enabled.set(true);
    pump(&mut tree);
    assert_eq!(
        handle.get_caret_dim().map(|d| (d.scope, d.opacity)),
        Some((CaretHighlightScope::Sentence, DimStrength::Strong.opacity())),
        "turning it back on keeps the unit and the strength chosen before"
    );
}

/// **A destroyed editor lets go of its fade.** The fade never reaches the document, so a
/// leftover could not be seen anywhere else; but its resolver holds a subscription to the
/// document, and an editor state can outlive its widget (this test holds the handle across
/// the teardown, as a stale tab-rebuilt editor does). `TypographyBoundEditor::drop` releases it.
#[test]
fn destroying_an_editor_releases_its_fade() {
    let (band, _) = faded(true, DimScope::Sentence);
    let (_doc, handle, mut tree) = column_with_document(Some(band));
    click_into(&mut tree);
    assert!(handle.get_caret_dim().is_some(), "faded to begin with");

    // `handle` is deliberately still alive.
    drop(tree);
    assert!(
        handle.get_caret_dim().is_none(),
        "the torn-down editor still holds its fade and its subscription"
    );
}

/// A surface built with no settings behind it fades nothing.
#[test]
fn a_column_without_settings_fades_nothing() {
    let (_doc, handle, mut tree) = column_with_document(None);
    click_into(&mut tree);
    assert!(handle.get_caret_dim().is_none());
}

/// The band and the fade are two preferences in one bundle and must not leak into each other:
/// a fade alone asks for no band.
#[test]
fn a_fade_alone_asks_for_no_band() {
    let (band, _) = faded(true, DimScope::Paragraph);
    let (_doc, handle, mut tree) = column_with_document(Some(band));
    click_into(&mut tree);
    assert!(handle.get_caret_dim().is_some());
    assert!(handle.get_caret_highlight().is_none());
}

/// A band on a page hands that page to its editor, which is how a tab's editors learn that
/// they fade together.
#[test]
fn the_page_reaches_the_editor() {
    let (band, _) = faded(true, DimScope::Paragraph);
    let page = teksilo::widgets::rich_text::caret_dim::CaretDimPage::new();
    let (_doc, handle, mut tree) = column_with_document(Some(band.on_page(page.clone())));
    click_into(&mut tree);
    assert_eq!(handle.get_caret_dim().and_then(|d| d.page), Some(page));
}
