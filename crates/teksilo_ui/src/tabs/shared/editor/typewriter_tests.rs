// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use teksilo::core::widget_tree::WidgetTree;

use crate::shared::{TypewriterAnchor, TypewriterSettings};

fn typo() -> EditorTypography {
    EditorTypography {
        font_family: Signal::new("Literata".to_string()),
        size: Signal::new(1.0),
        line_height: Signal::new(1.5),
        first_line_indent: Signal::new(0.0),
        para_spacing_before: Signal::new(0.0),
        para_spacing_after: Signal::new(0.0),
        size_range: crate::settings::TypographySizeRange::default(),
    }
}

/// A real writing column, reaching the editor's handle the way the rest of
/// the app does — through the find view-model `writing_column` attaches it
/// to. A detached editor over the same document would share the text but
/// not the state under test.
fn column_with(typewriter: Option<TypewriterSettings>) -> (EditorHandle, WidgetTree) {
    column_with_band(typewriter, None)
}

/// As [`column_with`], with an explicit caret band — the shape the band tests need.
fn column_with_band(
    typewriter: Option<TypewriterSettings>,
    caret: Option<crate::shared::CaretBand>,
) -> (EditorHandle, WidgetTree) {
    let (_doc, handle, tree) = column_with_document_and(typewriter, caret);
    (handle, tree)
}

/// As [`column_with_band`], handing back the document too — the caret-band tests assert
/// on its paint spans, which is where the whole chain ends up.
pub(super) fn column_with_document(
    caret: Option<crate::shared::CaretBand>,
) -> (TextDocument, EditorHandle, WidgetTree) {
    column_with_document_and(None, caret)
}

fn column_with_document_and(
    typewriter: Option<TypewriterSettings>,
    caret: Option<crate::shared::CaretBand>,
) -> (TextDocument, EditorHandle, WidgetTree) {
    let doc = TextDocument::new();
    doc.set_plain_text("Some prose to write in.").unwrap();
    let find = crate::search::FindViewModel::new(doc.clone());
    let col = writing_column(
        &doc,
        &Signal::new(700.0),
        &typo(),
        MAIN_MIN_LINES,
        || {},
        None,
        Some(find.clone()),
        None,
        None,
        None,
        typewriter,
        caret,
        // No app around this tree, so no writing game either.
        None,
        None,
        None,
        // No project around this tree, so no footnote binding, no image
        // source, and no project to tally typing against.
        None,
        None,
        None,
        // Editable: this fixture is a normal, untrashed surface.
        false,
        // No project around this tree, so no item to name.
        None,
        false,
        // No project behind this probe, so no palette and no capture menu.
        None,
        // Nothing private on this probe's editor: the default `all()`.
        None,
    );
    let mut tree = WidgetTree::new();
    tree.add(col);
    tree.layout(SizeProposal::exact(900.0, 600.0));
    let handle = find
        .editor_handle()
        .expect("writing_column attached its handle");
    (doc, handle, tree)
}

#[test]
fn the_setting_reaches_the_editor_at_build_time() {
    // Pushed up front, not only on the next settings change — an editor
    // opened while the feature is already on must pin from its first
    // keystroke.
    let tw = TypewriterSettings::new(
        Signal::new(true),
        Signal::new(Some(TypewriterAnchor::BottomQuarter)),
    );
    let (handle, _tree) = column_with(Some(tw));
    assert_eq!(handle.get_typewriter(), Some(0.75));
}

#[test]
fn an_editor_built_with_the_feature_off_does_not_pin() {
    let tw = TypewriterSettings::new(Signal::new(false), Signal::new(None));
    let (handle, _tree) = column_with(Some(tw));
    assert_eq!(handle.get_typewriter(), None);
}

#[test]
fn a_surface_that_never_pins_passes_no_setting_at_all() {
    let (handle, _tree) = column_with(None);
    assert_eq!(handle.get_typewriter(), None);
}

#[test]
fn toggling_the_setting_reaches_an_already_open_editor() {
    // The live path: Settings ▸ Editor Behavior flips the signal while tabs
    // are open. Both source signals must drive it — a combined derived
    // signal would panic on observe, which is why the wiring registers one
    // effect per field.
    let enabled = Signal::new(false);
    let preset = Signal::new(Some(TypewriterAnchor::Middle));
    let tw = TypewriterSettings::new(enabled.clone(), preset.clone());
    let (handle, _tree) = column_with(Some(tw));
    assert_eq!(handle.get_typewriter(), None);

    enabled.set(true);
    assert_eq!(
        handle.get_typewriter(),
        Some(0.5),
        "turning the feature on must reach an editor that is already open"
    );

    preset.set(Some(TypewriterAnchor::TopThird));
    assert!(
        (handle.get_typewriter().unwrap() - 1.0 / 3.0).abs() < 1e-6,
        "changing the preset must move the pin on an already-open editor"
    );

    enabled.set(false);
    assert_eq!(
        handle.get_typewriter(),
        None,
        "and turning it off must stop it"
    );
}
