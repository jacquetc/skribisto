// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A writer with only a keyboard, on a board of real cards.
//!
//! The board is mounted under one button, which stands for the title bar's menu button:
//! the first Tab stop of the window, where `Tab` used to send a writer inside a card.
//!
//! The synopsis tests are real-backend only: the mock `OpenDocsStore` gives a card no
//! synopsis field, so a mock board has no editor to go to. The rest run on both.

use teksilo::core::NoopWindowOps;
use teksilo::core::accessibility::widget_id_to_node_id;
use teksilo::core::accesskit::Role;
use teksilo::core::widget_tree::WidgetTree;
use teksilo::widgets::Button;

use super::*;
use crate::app_ids::AppIds;
use crate::models::OpenDocsStore;

const PROPOSAL: SizeProposal = SizeProposal {
    width: Some(1000.0),
    height: Some(700.0),
};

struct Board {
    tree: WidgetTree,
    vm: CorkboardViewModel,
    /// The window's first Tab stop.
    before: WidgetId,
    grid: WidgetId,
    /// Set when something asked for a card's item to open in the editor.
    opened: Rc<std::cell::Cell<bool>>,
    #[cfg(not(feature = "mocks"))]
    _project: crate::test_support::RealProject,
}

fn vm_over(app_ctx: Rc<AppContext>, ids: AppIds, container: u64) -> CorkboardViewModel {
    let docs = OpenDocsStore::new(app_ctx.clone());
    let tags = crate::tags::TagsViewModel::detached(app_ctx.clone(), ids.clone());
    CorkboardViewModel::new(
        app_ctx,
        ids,
        docs,
        container,
        Signal::new(true),
        Signal::new(200.0),
        Signal::new(true),
        Signal::new(false),
        Signal::new(1.0),
        Signal::new(Default::default()),
        EditorTypography {
            font_family: Signal::new(String::new()),
            size: Signal::new(1.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
            size_range: crate::settings::TypographySizeRange::default(),
        },
        crate::shared::CaretBand::off(),
        crate::writing_session::WritingGamesViewModel::detached(),
        crate::format::FormatViewModel::detached(),
        tags,
    )
}

/// Takes what the window's `editor.open_item` action takes.
#[derive(Debug)]
struct OpenProbe(Rc<std::cell::Cell<bool>>);

impl Widget for OpenProbe {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let opened = self.0.clone();
        ctx.register_action_global(
            Action::new("editor.open_item").on_invoke(move |_i, _c| opened.set(true)),
        );
        Vec::new()
    }
    fn layout_response(&self, _p: SizeProposal, _c: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }
}

fn settle(tree: &mut WidgetTree) {
    tree.layout(PROPOSAL);
    tree.run_mount_actions(&mut NoopWindowOps);
    tree.layout(PROPOSAL);
    tree.run_mount_actions(&mut NoopWindowOps);
    tree.layout(PROPOSAL);
}

struct Mounted {
    tree: WidgetTree,
    before: WidgetId,
    grid: WidgetId,
    opened: Rc<std::cell::Cell<bool>>,
}

fn mount(app_ctx: &Rc<AppContext>, vm: &CorkboardViewModel) -> Mounted {
    let opened = Rc::new(std::cell::Cell::new(false));
    let mut tree = crate::test_support::tree_with_events(app_ctx);
    tree.shortcut_registry_mut()
        .register(crate::corkboard::edit_synopsis_shortcut());
    let root = tree.add(
        VStack::new()
            .spacing(0.0)
            .child(Button::new(lit!("Before")))
            .child(OpenProbe(opened.clone()))
            .child(WireCorkboard { vm: vm.clone() })
            .child(Expand::new().child(CorkboardGrid {
                vm: vm.clone(),
                root: None,
            })),
    );
    settle(&mut tree);
    let before = tree.focusables_in_tab_order(root)[0];
    let grid = vm.grid_id().get().expect("the grid published its id");
    Mounted {
        tree,
        before,
        grid,
        opened,
    }
}

/// A board of `scenes` + 1 scene cards, under a real project's chapter.
#[cfg(not(feature = "mocks"))]
fn board_of(scenes: usize) -> Board {
    use frontend::commands::binder_item_commands::get_binder_item;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

    let project = crate::test_support::RealProject::empty_novel();
    let (first, _) = project.scenes()[0];
    let mut last = first;
    for n in 0..scenes {
        last = project.add_scene_after(last, &format!("Scene {n}")).0;
    }
    let chapter =
        crate::models::binder_stream::ordered_binder_items(&project.app_ctx, project.work_id)
            .into_iter()
            .find_map(|row| {
                let item = get_binder_item(&project.app_ctx, &row.id).ok().flatten()?;
                (item.role == BinderItemRole::Folder
                    && item.sub_role == BinderItemSubRole::ChapterScene)
                    .then_some(item.id)
            })
            .expect("the novel has a chapter folder");
    let vm = vm_over(project.app_ctx.clone(), project.ids.clone(), chapter);
    let mounted = mount(&project.app_ctx, &vm);
    Board {
        tree: mounted.tree,
        vm,
        before: mounted.before,
        grid: mounted.grid,
        opened: mounted.opened,
        _project: project,
    }
}

#[cfg(feature = "mocks")]
fn board_of(_scenes: usize) -> Board {
    let app_ctx = Rc::new(AppContext::new());
    // The mock chapter folder, whose three scenes are the only board this needs.
    let vm = vm_over(app_ctx.clone(), AppIds::new(), 104);
    let mounted = mount(&app_ctx, &vm);
    Board {
        tree: mounted.tree,
        vm,
        before: mounted.before,
        grid: mounted.grid,
        opened: mounted.opened,
    }
}

impl Board {
    fn press(&mut self, key: Key, modifiers: Modifiers) {
        self.tree.press_key(key, modifiers);
        settle(&mut self.tree);
    }

    /// Focus on the grid, the cursor on the card at `index`, by the arrow keys.
    fn on_card(&mut self, index: usize) {
        self.tree.focus(self.grid);
        settle(&mut self.tree);
        // With no cursor yet, the first arrow lands on the first card.
        self.press(Key::ArrowRight, Modifiers::NONE);
        for _ in 0..index {
            self.press(Key::ArrowRight, Modifiers::NONE);
        }
        assert_eq!(self.vm.grid_cursor(), Some(index));
    }

    /// The realized tile holding `id`.
    fn tile_of(&self, id: WidgetId) -> Option<WidgetId> {
        let mut current = Some(id);
        while let Some(node) = current {
            if self
                .tree
                .widget_type_name(node)
                .is_some_and(|name| name.contains("TileA11y"))
            {
                return Some(node);
            }
            current = self.tree.parent(node);
        }
        None
    }

    /// The realized tiles, in model order.
    fn tiles(&self) -> Vec<WidgetId> {
        fn walk(tree: &WidgetTree, id: WidgetId, out: &mut Vec<WidgetId>) {
            if tree
                .widget_type_name(id)
                .is_some_and(|name| name.contains("TileA11y"))
            {
                out.push(id);
                return;
            }
            for child in tree.children(id) {
                walk(tree, child, out);
            }
        }
        let mut out = Vec::new();
        walk(&self.tree, self.grid, &mut out);
        out
    }

    /// The first control of the card at `index`, whatever its kind.
    fn control_of(&self, index: usize) -> WidgetId {
        let tile = self.tiles()[index];
        self.tree
            .focusables_in_tab_order(tile)
            .first()
            .copied()
            .expect("a card holds controls")
    }

    fn focused(&self) -> WidgetId {
        self.tree.focused().expect("focus is somewhere")
    }
}

/// The report: from inside a card, `Tab` went to the window's first control. It now goes
/// round the card, and never out of it, in either direction.
#[test]
fn tab_inside_a_card_stays_in_that_card() {
    let mut board = board_of(3);
    let tile = board.tiles()[1];
    let start = board.control_of(1);
    board.tree.focus(start);
    settle(&mut board.tree);

    let mut seen = Vec::new();
    for _ in 0..8 {
        board.press(Key::Tab, Modifiers::NONE);
        let now = board.focused();
        assert_ne!(now, board.before, "Tab left the card for the first control");
        assert_eq!(board.tile_of(now), Some(tile), "Tab left the card");
        seen.push(now);
    }
    seen.sort();
    seen.dedup();
    assert!(
        seen.len() > 1,
        "Tab found only one stop in a card holding several controls"
    );

    board.press(Key::Tab, Modifiers::SHIFT);
    assert_eq!(
        board.tile_of(board.focused()),
        Some(tile),
        "Shift+Tab left the card"
    );
}

/// While the writer is on the grid, the grid is one Tab stop: a card's controls appear in
/// the Tab order only with focus inside that card.
#[test]
fn on_the_grid_no_card_is_a_tab_stop() {
    let mut board = board_of(3);
    board.on_card(0);
    let tile = board.tiles()[0];
    assert!(
        board.tree.tab_stops_within(tile).is_empty(),
        "a card's controls are Tab stops while the keyboard is on the grid"
    );
    board.press(Key::Tab, Modifiers::NONE);
    assert_eq!(
        board.tile_of(board.focused()),
        None,
        "Tab from the grid walked into a card"
    );
}

/// `Escape` comes back to the card it left, with the cursor on it, wherever focus was in
/// the card. It used to reach the grid's own `Escape`, which cleared the cursor and left
/// focus where it was.
#[test]
fn escape_from_inside_a_card_comes_back_to_that_card() {
    let mut board = board_of(3);
    board.on_card(1);
    let control = board.control_of(1);
    board.tree.focus(control);
    settle(&mut board.tree);
    board.press(Key::Tab, Modifiers::NONE);

    board.press(Key::Escape, Modifiers::NONE);

    assert_eq!(board.focused(), board.grid, "Escape left focus in the card");
    assert_eq!(board.vm.grid_cursor(), Some(1), "Escape moved the cursor");
    let tile = board.tiles()[1];
    let update = board.tree.sync_accessibility();
    assert_eq!(update.focus, widget_id_to_node_id(board.grid));
    let grid = update
        .nodes
        .iter()
        .find(|(_, node)| node.role() == Role::Grid)
        .map(|(_, node)| node.clone())
        .expect("the grid is in the accessibility tree");
    assert_eq!(
        grid.active_descendant(),
        Some(widget_id_to_node_id(tile)),
        "a screen reader is not told which card it is back on"
    );

    // ...and the next arrow steps from it.
    board.press(Key::ArrowRight, Modifiers::NONE);
    assert_eq!(board.vm.grid_cursor(), Some(2));
}

/// Focus arriving in a card some other way than the chord (a click, a screen reader's
/// focus) enters it too: `Escape` comes back to *that* card, not to where the cursor was.
#[test]
fn focus_arriving_in_a_card_brings_the_cursor_with_it() {
    let mut board = board_of(3);
    board.on_card(0);
    let control = board.control_of(2);
    board.tree.focus(control);
    settle(&mut board.tree);
    assert_eq!(board.vm.grid_cursor(), Some(2));
    board.press(Key::Tab, Modifiers::NONE);
    assert_eq!(
        board.tile_of(board.focused()),
        Some(board.tiles()[2]),
        "focus by another route did not enter the card"
    );
    board.press(Key::Escape, Modifiers::NONE);
    assert_eq!(board.vm.grid_cursor(), Some(2));
}

/// The board's own keys do nothing from inside a card: no walking the cursor away behind
/// the writer's back, no jumping by type-ahead and, above all, no trash.
#[test]
fn the_boards_keys_stay_out_of_a_card() {
    let mut board = board_of(3);
    board.on_card(1);
    let cards = board.vm.ordered_cards().len();
    let selected = board.vm.selected_item_ids();
    // A control that uses none of these keys itself: the last in the card's cycle is a
    // button, whatever else the card holds.
    let tile = board.tiles()[1];
    let last = *board
        .tree
        .focusables_in_tab_order(tile)
        .last()
        .expect("a card holds controls");
    board.tree.focus(last);
    settle(&mut board.tree);
    for (key, modifiers) in [
        (Key::ArrowRight, Modifiers::NONE),
        (Key::ArrowDown, Modifiers::NONE),
        (Key::End, Modifiers::NONE),
        (Key::PageDown, Modifiers::NONE),
        (Key::ArrowRight, Modifiers::ALT),
        (Key::Q, Modifiers::NONE),
        (Key::Delete, Modifiers::NONE),
        (Key::Backspace, Modifiers::NONE),
    ] {
        board.press(key, modifiers);
        assert_eq!(board.vm.grid_cursor(), Some(1), "{key:?} moved the cursor");
        assert_eq!(
            board.vm.selected_item_ids(),
            selected,
            "{key:?} moved the selection"
        );
        assert_eq!(
            board.vm.ordered_cards().len(),
            cards,
            "{key:?} removed a card"
        );
    }
}

/// `F2` from inside a card renames that card, not the selection's first.
#[test]
fn f2_inside_a_card_renames_that_card() {
    let mut board = board_of(3);
    board.on_card(0);
    let target = board.vm.ordered_cards()[2].item_id;
    let control = board.control_of(2);
    board.tree.focus(control);
    settle(&mut board.tree);
    board.press(Key::F2, Modifiers::NONE);
    assert_eq!(board.vm.editing_item().get(), Some(target));
}

/// The title's field: `F2` on the grid, then `Tab`. The field took focus from `build`,
/// which leaves the card's `focus_within` off, so its `Tab` went to the title bar.
#[test]
fn tab_from_the_title_field_stays_in_the_card() {
    let mut board = board_of(3);
    board.on_card(1);
    board.press(Key::F2, Modifiers::NONE);
    let tile = board.tiles()[1];
    let field = board.focused();
    assert_ne!(
        field, board.grid,
        "F2 did not move focus into the title's field"
    );
    assert_eq!(board.tile_of(field), Some(tile));

    board.press(Key::Tab, Modifiers::NONE);
    assert_eq!(
        board.tile_of(board.focused()),
        Some(tile),
        "Tab left the card"
    );
    assert_eq!(
        board.vm.editing_item().get(),
        None,
        "the field did not commit"
    );
}

/// The chord is one row for the whole window, and the board's action answers it only
/// with focus on the board: anywhere else the key goes on to whatever has focus.
#[test]
fn the_chord_means_nothing_outside_a_board() {
    let mut board = board_of(3);
    board.tree.focus(board.before);
    settle(&mut board.tree);
    board.press(Key::F2, Modifiers::SHIFT);
    assert_eq!(board.focused(), board.before);
}

/// The other two keys a card already has keep their meaning.
#[test]
fn enter_and_f2_keep_their_meaning_on_a_card() {
    let mut board = board_of(3);
    board.on_card(1);
    let item = board.vm.ordered_cards()[1].item_id;
    board.press(Key::F2, Modifiers::NONE);
    assert_eq!(
        board.vm.editing_item().get(),
        Some(item),
        "F2 no longer renames"
    );
    board.press(Key::Escape, Modifiers::NONE);
    assert_eq!(board.focused(), board.grid);
    assert_eq!(board.vm.editing_item().get(), None);

    // `Enter` opens the card's item, and stays on the grid: the app opens the tab.
    board.press(Key::Enter, Modifiers::NONE);
    assert!(board.opened.get(), "Enter no longer opens the card");
}

#[cfg(not(feature = "mocks"))]
mod synopsis {
    use super::*;

    /// The name a screen reader has for the focused node.
    fn focused_node(board: &mut Board) -> (Role, String) {
        let update = board.tree.sync_accessibility();
        update
            .nodes
            .iter()
            .find(|(id, _)| *id == update.focus)
            .map(|(_, node)| (node.role(), node.label().unwrap_or_default().to_string()))
            .expect("the focused node is in the accessibility tree")
    }

    /// The whole trip, as the writer makes it: to the card, into its synopsis, a few
    /// words, back to the card, on to the next.
    #[test]
    fn the_keyboard_goes_into_a_cards_synopsis_and_comes_back() {
        crate::test_support::with_real_messages(|| {
            let mut board = board_of(3);
            board.on_card(1);
            let tile = board.tiles()[1];
            let item = board.vm.ordered_cards()[1].item_id;

            board.press(Key::F2, Modifiers::SHIFT);

            assert_eq!(
                board.tile_of(board.focused()),
                Some(tile),
                "the chord did not go into the card the cursor is on"
            );
            assert_eq!(
                focused_node(&mut board),
                (Role::MultilineTextInput, "Synopsis".to_string())
            );

            let editor = board.focused();
            board.tree.type_text(editor, "Fogg wagers");
            settle(&mut board.tree);
            let doc = board
                .vm
                .synopsis_doc_for(item)
                .expect("the card's document");
            let synopsis = doc.synopsis.as_ref().expect("the card's synopsis");
            assert_eq!(
                synopsis.doc.to_plain_text().ok().as_deref(),
                Some("Fogg wagers")
            );

            board.press(Key::Escape, Modifiers::NONE);
            assert_eq!(board.focused(), board.grid);
            assert_eq!(board.vm.grid_cursor(), Some(1));

            board.press(Key::ArrowRight, Modifiers::NONE);
            assert_eq!(board.vm.grid_cursor(), Some(2));
        });
    }

    /// `Tab` from the synopsis: the report's own case. The editor is a Tab stop inside the
    /// card's cycle, so going round comes back to it.
    #[test]
    fn tab_from_a_cards_synopsis_goes_round_the_card() {
        let mut board = board_of(3);
        board.on_card(0);
        let tile = board.tiles()[0];
        board.press(Key::F2, Modifiers::SHIFT);
        let editor = board.focused();
        let mut back = false;
        for _ in 0..12 {
            board.press(Key::Tab, Modifiers::NONE);
            let now = board.focused();
            assert_ne!(
                now, board.before,
                "Tab jumped to the window's first control"
            );
            assert_eq!(board.tile_of(now), Some(tile));
            if now == editor {
                back = true;
                break;
            }
        }
        assert!(back, "Tab never came back round to the synopsis");
    }

    /// The cursor can be far from the view: the wheel scrolls without moving it. The
    /// chord brings the card back and goes in, once the card is realized.
    #[test]
    fn the_chord_reaches_a_card_scrolled_out_of_the_realized_window() {
        let mut board = board_of(60);
        let last = board.vm.ordered_cards().len() - 1;
        board.tree.focus(board.grid);
        settle(&mut board.tree);
        // The cursor goes to the far end, then the view is scrolled back.
        board.press(Key::End, Modifiers::NONE);
        assert_eq!(board.vm.grid_cursor(), Some(last));
        board.vm.scroll_grid_to_top();
        settle(&mut board.tree);
        let item = board.vm.ordered_cards()[last].item_id;
        assert!(
            !board.vm.card_editor_is_mounted(item),
            "the test needs the card out of the realized window"
        );

        board.press(Key::F2, Modifiers::SHIFT);

        assert!(board.vm.card_editor_is_mounted(item));
        assert_eq!(
            focused_node(&mut board),
            (Role::MultilineTextInput, "Synopsis".to_string()),
            "the chord did not go into the far card"
        );
    }

    /// What a screen reader is told, on the grid and in the editor, in the writer's words.
    #[test]
    fn a_screen_reader_is_told_the_way_in_and_the_way_back() {
        crate::test_support::with_real_messages(|| {
            let mut board = board_of(3);
            let update = board.tree.sync_accessibility();
            let grid = update
                .nodes
                .iter()
                .find(|(_, node)| node.role() == Role::Grid)
                .map(|(_, node)| node.clone())
                .expect("the grid");
            // The chord is written the platform's way (`⇧F2` on macOS), the sentence is ours.
            let chord = teksilo::widgets::keystroke_format::format_keystroke(
                crate::corkboard::edit_synopsis_default_key(),
            );
            assert_eq!(
                grid.description(),
                Some(format!("Press {chord} to edit the card’s synopsis.").as_str())
            );
            let editor = update
                .nodes
                .iter()
                .find(|(_, node)| node.role() == Role::MultilineTextInput)
                .map(|(_, node)| node.clone())
                .expect("a card's editor");
            // The key's name is teksilo's (`⎋` on macOS), the rest of the sentence is ours.
            let exit = teksilo::widgets::keystroke_format::format_keystroke(KeyStroke::new(
                Key::Escape,
                Modifiers::NONE,
            ));
            assert_eq!(
                editor.description(),
                Some(format!("{exit} returns to the card.").as_str())
            );
        });
    }
}
