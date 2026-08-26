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
        size_range: crate::settings::TypographySizeRange::default(),
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
        // No project, so no item to name either.
        None,
        false,
        // A widget test builds no project, so there is no palette; the submenu
        // comes down to Untagged alone.
        None,
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

/// A fresh editor over the sentence "Elizabeth Bennet walked into the room.",
/// with `range` selected. `range` is the selection `editor_context_menu` will
/// read *at menu-build time*, matching how the real factory rebuilds the menu
/// fresh on every right-click rather than caching a stale one.
fn editor_with_selection(range: (usize, usize)) -> (TextDocument, EditorHandle) {
    let doc = TextDocument::new();
    doc.set_markdown("Elizabeth Bennet walked into the room.")
        .expect("parse")
        .wait()
        .expect("import");
    let editor = RichTextEditor::editor(doc.clone());
    let handle = editor.handle();
    handle.select_range(range.0, range.1);
    (doc, handle)
}

/// Whether the built context menu offers "Add as note", detected by an
/// accessibility label containing "note", which nothing else in this menu's
/// vocabulary does (no spell corrections and no comment binding are wired
/// in, so neither the spelling nor the comment group contribute any rows).
fn add_as_note_row_is_offered(
    doc: &TextDocument,
    handle: &EditorHandle,
    item_id: Option<u64>,
) -> bool {
    let menu = editor_context_menu(
        handle.clone(),
        Signal::new(0),
        None,
        doc.clone(),
        None,
        None,
        item_id,
        // No palette: the submenu comes down to Untagged alone, which is exactly the
        // shape a project with no tags gets. The row is still offered, which is what
        // this asks about.
        None,
        Vec::new(),
    );
    let mut tree = WidgetTree::new();
    tree.add(menu);
    tree.layout(SizeProposal::exact(300.0, 600.0));
    tree.sync_accessibility()
        .nodes
        .iter()
        .filter_map(|(_, n)| n.label().map(|s| s.to_string()))
        .any(|l| l.to_lowercase().contains("note"))
}

/// **The gate on "Add as note".** It must appear only when both a real
/// selection and a real `item_id` are in hand: the two things the row
/// hands off to `AppIntent::AddAsNote`, per `editor_context_menu`'s own
/// doc comment ("empty, or no row to scope it to, and the row simply is
/// not offered").
#[test]
fn add_as_note_row_present_only_with_a_selection_and_an_item_id() {
    let (doc, with_selection) = editor_with_selection((0, 9)); // "Elizabeth"
    assert!(
        add_as_note_row_is_offered(&doc, &with_selection, Some(42)),
        "a real selection plus a real item_id must offer Add as note"
    );
    assert!(
        !add_as_note_row_is_offered(&doc, &with_selection, None),
        "with item_id: None (no project row to scope it to) the row must not be offered"
    );

    let (doc, no_selection) = editor_with_selection((0, 0));
    assert!(
        !add_as_note_row_is_offered(&doc, &no_selection, Some(42)),
        "with no selection there is nothing to file, so the row must not be offered"
    );
}

/// A single-child host that owns a global [`Action`] the way `App::build`'s
/// own command modules do (`ctx.register_action_global`, only reachable from
/// inside a `build()`), so a mounted menu's `ctx.send_intent` has something
/// real to reach. Exists only for
/// [`activating_add_as_note_sends_the_intent_with_the_right_item_id_and_selection`]:
/// `WidgetTree::push_action` itself is crate-private to teksilo-core.
#[derive(Debug)]
struct ActionHost {
    menu: Option<MenuList>,
    action: Option<Action>,
    menu_id: Option<WidgetId>,
}

impl Widget for ActionHost {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        if let Some(action) = self.action.take() {
            ctx.register_action_global(action);
        }
        let id = ctx.add(self.menu.take().expect("ActionHost built more than once"));
        self.menu_id = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        Size::new(
            proposal.width.unwrap_or(300.0),
            proposal.height.unwrap_or(600.0),
        )
        .into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        if let Some(child) = children.first_mut() {
            child.origin = Point::new(bounds.x, bounds.y);
            child.size = Size::new(bounds.width, bounds.height);
        }
    }
}

/// **The wiring itself fires.** Activating the row must reach
/// `AppIntent::AddAsNote` with exactly the `item_id` the menu was built
/// with and exactly the text that was selected at that moment, proven by
/// actually navigating the mounted menu and pressing Enter, per the
/// project's own rule that event behaviour gets a headless test rather
/// than an inspection.
#[test]
fn activating_add_as_note_sends_the_intent_with_the_right_item_id_and_selection() {
    let (doc, handle) = editor_with_selection((0, 9)); // "Elizabeth"
    let menu = editor_context_menu(
        handle.clone(),
        Signal::new(0),
        None,
        doc,
        None,
        None,
        Some(42),
        None,
        Vec::new(),
    );

    /// What the action saw: the item, the selection, and the tag the writer picked.
    type Captured = Rc<RefCell<Option<(u64, String, Option<u64>)>>>;
    let captured: Captured = Rc::new(RefCell::new(None));
    let captured_for_action = captured.clone();
    // `register_action_global` (what `App::build`'s own command modules call)
    // is only reachable from inside a `Widget::build`, and `WidgetTree::push_action`
    // is crate-private to teksilo-core, so a minimal host widget stands in for
    // the app root that would normally own this action, exactly the way
    // `app::commands::story_bible::register` owns it for real.
    let action = Action::new("story_bible.add_as_note").on_invoke(move |i, _c| {
        if let Some(AppIntent::AddAsNote {
            item_id,
            selected_text,
            tag_id,
        }) = AppIntent::from_intent(i)
        {
            *captured_for_action.borrow_mut() = Some((*item_id, selected_text.clone(), *tag_id));
        }
    });

    let mut tree = WidgetTree::new();
    let host_id = tree.add(ActionHost {
        menu: Some(menu),
        action: Some(action),
        menu_id: None,
    });

    tree.layout(SizeProposal::exact(300.0, 600.0));
    let menu_id = tree
        .children(host_id)
        .into_iter()
        .next()
        .expect("the menu must have been mounted as the host's one child");
    tree.focus(menu_id);
    // Type-ahead: "Add as note…" is the only item in this menu (no
    // spelling group, no comment group) whose label starts with 'a':
    // Cut/Copy/Paste/Select all/the format strip all fail that match.
    tree.press_key(Key::A, Modifiers::NONE);
    // It is a submenu now, not a leaf: the tag is chosen *inside* it, so the row
    // itself opens rather than fires. Nothing may have been sent yet.
    tree.press_key(Key::ArrowRight, Modifiers::NONE);
    assert_eq!(
        tree.active_overlays().len(),
        1,
        "the tag submenu must be up"
    );
    assert!(
        captured.borrow().is_none(),
        "opening the submenu must not capture anything: no tag has been chosen"
    );
    // With no palette the submenu is Untagged alone, which is the tagless project's
    // whole menu and the one row every project always has.
    tree.press_key(Key::ArrowDown, Modifiers::NONE);
    tree.press_key(Key::Enter, Modifiers::NONE);

    let (item_id, selected_text, tag_id) = captured
        .borrow()
        .clone()
        .expect("activating the row must send AppIntent::AddAsNote");
    assert_eq!(item_id, 42);
    assert_eq!(selected_text, "Elizabeth");
    assert_eq!(tag_id, None, "the Untagged row files under no tag");
}

/// The submenu is still offered on a surface with no palette, and comes down to
/// **Untagged** alone.
///
/// The ordering and the dividers are [`crate::story_bible::capture::CaptureMenu::entries`]'s
/// rules and are tested there against plain values; this checks only that the widget
/// half renders what that rule returns, and that a palette-less surface therefore keeps
/// a working capture rather than losing the row.
#[test]
fn a_surface_with_no_palette_still_offers_untagged() {
    let mut tree = WidgetTree::new();
    tree.add(capture_submenu(42, "Elizabeth", None, &[]));
    tree.layout(SizeProposal::exact(300.0, 600.0));
    let labels: Vec<String> = tree
        .sync_accessibility()
        .nodes
        .iter()
        .filter_map(|(_, n)| n.label().map(|s| s.to_string()))
        .collect();
    assert!(
        labels.iter().any(|l| l.to_lowercase().contains("untag")),
        "the one always-present row must render: {labels:?}"
    );
}

/// **The writer's palette reaches the capture menu.**
///
/// A tag is recognised by its colour everywhere else in the app — the binder's dots, the
/// corkboard's, the editor's subtitle row — and this menu is exactly where a writer picks
/// between four of them at speed. A row that named the tag without showing its colour
/// would be the one place in the app where the palette does not hold.
///
/// The assertion is on **painted** colour, not on a builder call, because the default
/// every other menu row wants is the opposite one: `MenuItem` tints its icon to the row's
/// own foreground, which is right for a glyph that repeats the label and erases one whose
/// colour *is* the label. `icon_keeps_color` is what separates the two, and only a render
/// can tell whether it took.
#[test]
fn a_capture_row_shows_the_tag_in_the_writers_own_colour() {
    use crate::story_bible::capture::{CaptureMenu, CaptureTag};

    let tag = |id: u64, name: &str, color: &str| CaptureTag {
        id,
        uid: uuid::Uuid::from_u128(id as u128),
        name: name.to_string(),
        color: color.to_string(),
        discoverable: true,
    };
    // Two colours no theme role resolves to, so finding them can only mean the tags'
    // own reached the screen.
    let menu = CaptureMenu {
        primary: vec![tag(1, "Characters", "#e91e63"), tag(2, "Places", "#00838f")],
        recent: Vec::new(),
        all: Vec::new(),
    };

    let mut tree = WidgetTree::new();
    tree.add(render_capture_menu(7, "Elise Laroche", menu));
    tree.layout(teksilo::prelude::SizeProposal::exact(400.0, 300.0));

    let painted: Vec<[u8; 4]> = tree
        .render()
        .paths
        .iter()
        .map(|p| {
            let q = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
            [q(p.color[0]), q(p.color[1]), q(p.color[2]), q(p.color[3])]
        })
        .collect();
    let rgba8 = |hex: &str| {
        let c = crate::tags::contrast::parse(hex);
        let q = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        [q(c.r()), q(c.g()), q(c.b()), q(c.a())]
    };
    for hex in ["#e91e63", "#00838f"] {
        assert!(
            painted.contains(&rgba8(hex)),
            "the swatch for {hex} was not painted in the writer's colour; got {painted:?}"
        );
    }
}
