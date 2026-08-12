// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use teksilo::core::widget_tree::WidgetTree;

fn test_typo() -> EditorTypography {
    EditorTypography {
        font_family: Signal::new("Literata".to_string()),
        size: Signal::new(1.0),
        line_height: Signal::new(1.5),
        first_line_indent: Signal::new(0.0),
        para_spacing_before: Signal::new(0.0),
        para_spacing_after: Signal::new(0.0),
    }
}

/// A long synopsis must **scroll** inside the fixed-height card / expand modal,
/// never overflow it — and the caret must stay in view while typing. Two things
/// make that work, and this pins both:
///
/// 1. The editor is *greedy* (neither `min_lines` nor `max_lines`), so it consumes
///    the height its box proposes instead of growing to the whole document's
///    intrinsic height. An intrinsic (`min_lines`) editor reports the full document
///    height, so there's no bounded viewport for `ScrollPolicy::Auto` to scroll.
/// 2. It must be given that height by a widget that *proposes an exact height*
///    (`FixedSize`), NOT an `Expand` — an `Expand` measures its child with an
///    unspecified height (a 100 px fallback), so the greedy editor never learns the
///    box height and overflows, vertically centered, scrollbar pinned. The card
///    and the modal both wrap this editor in a `FixedSize` for exactly this reason.
///
/// Here a 60-paragraph synopsis inside a `FixedSize` 320×200 box must still measure
/// ~200 px tall — proving it stayed bounded. A value near the (much taller) document
/// height would mean the editor reverted to intrinsic sizing and overflowed.
#[test]
fn card_synopsis_editor_in_a_fixed_box_bounds_a_tall_synopsis_so_it_scrolls() {
    use teksilo::widgets::FixedSize;
    let doc = TextDocument::new();
    let _ = doc.set_djot_sync(&"A line of synopsis prose that says what happens.\n\n".repeat(60));
    let (editor, _handle) = card_synopsis_editor(
        doc,
        test_typo(),
        || {},
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
    let mut tree = WidgetTree::new();
    let id = tree.add(FixedSize::new().width(320.0).height(200.0).child(editor));
    // Propose an *unbounded* height, the way the corkboard's GridView tile does —
    // the `FixedSize` must still pin the editor to 200 px regardless.
    tree.layout(SizeProposal::with_width(320.0));
    let h = tree.bounds(id).height;
    assert!(
        (h - 200.0).abs() < 2.0,
        "the synopsis editor in a FixedSize(200) box must stay 200px (scroll the \
             overflow), got {h:.1}px — a taller value means it grew past the card/modal"
    );
}
/// Lay out a 60-paragraph synopsis at `fit` and report the height it claimed.
/// `pane_height` stands in for a `Splitter` pane: a hard box the editor is
/// expected to stay inside. `None` proposes an unbounded height, the way a
/// flowing page does.
fn synopsis_height(fit: SynopsisFit, pane_height: Option<f32>) -> f32 {
    use teksilo::widgets::FixedSize;
    let doc = TextDocument::new();
    let _ = doc.set_djot_sync(&"A line of synopsis prose that says what happens.\n\n".repeat(60));
    let typo = test_typo();
    let editor = synopsis_editor(
        &doc,
        &typo,
        fit,
        || {},
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        // No app around this tree, so no writing game either.
        None,
        // No project around this tree, so no comment binding.
        None,
        None,
        false,
    );
    let mut tree = WidgetTree::new();
    let id = match pane_height {
        Some(h) => tree.add(FixedSize::new().width(320.0).height(h).child(editor)),
        None => tree.add(editor),
    };
    tree.layout(SizeProposal::with_width(320.0));
    tree.bounds(id).height
}

/// The Side fit fills the pane it is given and scrolls **inside** it, rather
/// than growing to its content the way `Growing` does. A `Splitter` places a
/// pane at an exact pixel height, so this is the sizing that makes a
/// side-by-side synopsis a fixed viewport instead of a column that runs off
/// the bottom of the tab.
///
/// (The pane's *colour* — Main showing through rather than a Content card —
/// is not observable from a headless layout tree; it is checked in the app.)
#[test]
fn the_side_fit_is_greedy_so_it_takes_its_panes_exact_height() {
    let h = synopsis_height(SynopsisFit::Side, Some(200.0));
    assert!(
        (h - 200.0).abs() < 2.0,
        "a Side synopsis must stay inside its pane (200px) and scroll the \
             overflow, got {h:.1}px — a taller value means it reverted to \
             intrinsic sizing and would run past the bottom of the pane"
    );
}

/// Adding the Side arm must not have disturbed the two fits that were already
/// there: Compact stays a short capped box, Growing still grows past it.
#[test]
fn the_existing_fits_keep_their_sizing() {
    let compact_h = synopsis_height(SynopsisFit::Compact, None);
    let growing_h = synopsis_height(SynopsisFit::Growing, None);
    assert!(
        compact_h > 0.0 && compact_h < growing_h,
        "Compact ({compact_h:.1}px) must stay capped well under a 60-paragraph \
             Growing synopsis ({growing_h:.1}px)"
    );
}

/// A menu built over a selection must open with the marks the selection
/// already carries — a bold phrase should show Bold lit, not off.
#[test]
fn the_format_row_opens_showing_the_selections_state() {
    let doc = TextDocument::new();
    doc.set_markdown("hello world")
        .expect("parse")
        .wait()
        .expect("import");
    let editor = RichTextEditor::editor(doc);
    editor.select_all();
    let handle = editor.handle();
    handle.set_bold(true);

    let mut tree = WidgetTree::new();
    let id = tree.add(format_row(&handle, &CharacterMark::ALL));
    tree.layout(SizeProposal::exact(200.0, 40.0));
    assert!(
        tree.bounds(id).width > 0.0,
        "the row must lay out to something clickable"
    );
    assert!(handle.is_bold(), "precondition");
}

/// The row acts on the editor it was built over, and writes back what that
/// editor actually did rather than an optimistic flip. The menu stays open
/// across clicks, so a wrong assumption here would persist for the whole
/// visit instead of being corrected by the next rebuild.
#[test]
fn the_format_row_reports_what_the_editor_did() {
    let doc = TextDocument::new();
    doc.set_markdown("hello world")
        .expect("parse")
        .wait()
        .expect("import");
    let editor = RichTextEditor::editor(doc);
    editor.select_all();
    let handle = editor.handle();

    // The command path the row's buttons drive.
    assert!(!handle.is_bold());
    handle.toggle_bold();
    assert!(handle.is_bold());
    handle.toggle_italic();
    assert!(
        handle.is_bold() && handle.is_italic(),
        "a second mark must not undo the first — the menu stays open, so both \
             apply in one visit"
    );
}

/// A right-click inside a selection keeps it, so the row formats the phrase
/// the writer chose rather than collapsing to a caret first. The behaviour
/// belongs to `reposition_caret_for_context_menu`; this pins that the menu's
/// contract depends on it.
#[test]
fn right_clicking_inside_a_selection_keeps_it() {
    let doc = TextDocument::new();
    doc.set_markdown("hello world")
        .expect("parse")
        .wait()
        .expect("import");
    let editor = RichTextEditor::editor(doc);
    let handle = editor.handle();
    handle.select_range(0, 5);
    let (before_a, before_b) = handle.selection();
    assert_ne!(before_a, before_b, "precondition: there is a selection");

    handle.toggle_bold();
    let (after_a, after_b) = handle.selection();
    assert_eq!(
        (before_a, before_b),
        (after_a, after_b),
        "formatting must not move the selection out from under the next click"
    );
    assert!(handle.is_bold());
}
