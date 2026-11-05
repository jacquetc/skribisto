// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `GoToButton` — the "jump to any item" popover and its trigger.
//!
//! A search field over the binder tree: empty query shows the collapsible
//! tree, typing narrows it in place while keeping each match's parent chain
//! (`BinderBinderItemsTreeModel`'s own filter is ancestor-preserving, so
//! structure never disappears out from under a match). Enter jumps.
//!
//! **One popover, three ways in.** The trigger opens it; so do the Go menu's
//! "Go to…" entry and Ctrl+G, through `PopoverWidget::open_action("go.to")` —
//! which registers a *global* action running the very same toggle closure the
//! trigger runs, so the keyboard and the pointer cannot drift apart.
//! `open_signal()` is a read-back mirror only; writing it opens nothing.
//!
//! **Escape is scoped to the popover.** The popover's own dismiss consumes it,
//! so pressing Escape here closes the list and leaves you exactly where you
//! were — it must not fall through to `App`'s contextless-Escape handler and
//! drop the writer out of distraction-free mode as well. Two things at once
//! from one keystroke is the surprise this avoids.
//!
//! Keyboard shape: focus lands in the search field (the popover focuses its
//! first focusable descendant), Down/Up move the tree's selection without
//! leaving the field, Enter activates the selected row. That is the palette
//! convention — a writer types, glances, and commits without ever reaching for
//! the mouse or Tab.
//!
//! **A screen reader hears the results from the field.** `SearchField::drives_listbox`
//! is the framework's pattern for a list its host owns, but it needs the highlighted
//! row's node and a `TreeView` keeps that to itself (`current_row_widget`), and a tree
//! row cannot say its place among the matches anyway, since the ancestors kept around a
//! match are rows too. So the line under the tree is a live region, and each arrow
//! announces the row it reached.

use teksilo::core::accesskit::Live;
use teksilo::core::overlay::OverlayPlacement;
use teksilo::data::TreeDataSource;
use teksilo::prelude::*;
use teksilo::widgets::{
    ActivateOn, Expand, IconButton, MinSize, Padding, PopoverIconButton, ScrollBarMode,
    SearchField, StandardTreeItem, TextWidget, TreeRow, TreeView, VStack,
};

use crate::go::GoToViewModel;
use crate::models::TreeNode;

/// The two per-instance action names. One `GoToButton` lives in the status bar
/// and one in the distraction-free strip, and only ever one is on screen — so
/// they must not share a name. `app/commands/go.rs` owns the public `go.to` and
/// forwards it to whichever is currently visible.
pub const GO_TO_MAIN: &str = "go.to.main";
pub const GO_TO_FOCUS: &str = "go.to.focus";

/// Popover geometry. Wide enough for a chapter title plus its label, tall
/// enough for a dozen rows — past that the tree scrolls rather than the
/// popover growing to the height of a 31-chapter binder.
const POPUP_WIDTH: f32 = 380.0;
const POPUP_HEIGHT: f32 = 360.0;

/// The popover body: search field above, filtered binder tree below.
struct GoToPalette {
    vm: GoToViewModel,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for GoToPalette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoToPalette").finish()
    }
}

impl Widget for GoToPalette {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let vm = self.vm.clone();
        let model = vm.model();

        // Enter jumps without ever touching the arrows — the common path is
        // "type three letters, press Enter", and requiring a Down first to give
        // Enter a target would make the fast case slower than the slow one.
        // `activate_current` takes the highlight if there is one and the first
        // match otherwise.
        let submit_vm = vm.clone();
        let field = SearchField::new(vm.query())
            // A placeholder is gone once the writer types and is never a name.
            .label(tr!(go_to_placeholder()))
            .placeholder(tr!(go_to_placeholder()))
            .on_submit_fn(move |ctx| {
                if submit_vm.activate_current(ctx) {
                    // See the tree's `on_activate` below for why this is
                    // `dismiss_top_overlay` and not one of the scoped variants.
                    ctx.dismiss_top_overlay();
                }
            });

        let activate_model = model.clone();
        let activate_vm = vm.clone();
        let tree = TreeView::from_source_keyed(
            model.clone(),
            vm.selection(),
            move |node: &TreeNode, row: &TreeRow, selected: bool| {
                let (label, badge) = crate::models::label_and_badge(
                    &node.title,
                    node.fallback_label.as_deref(),
                    node.number,
                );
                let mut item = StandardTreeItem::new(lit!(label))
                    .depth(row.depth)
                    .has_children(row.has_children)
                    .is_expanded(row.is_expanded)
                    // A row title is as long as the writer made it; without this the label
                    // reports its full intrinsic width and paints out of the panel. See
                    // `binder::dock`, where this was found.
                    .label_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
                    .subtitle_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
                    .selected(selected)
                    .on_chevron_toggle_rc(row.toggle_callback());
                if !node.label.is_empty() {
                    item = item.subtitle(lit!(node.label.clone()));
                }
                let icon = if node.kind == "binder" {
                    crate::binder::icons::binder_icon()
                } else {
                    crate::binder::icons::kind_sub_role_icon(&node.kind, &node.sub_role)
                };
                item = item.leading_slot(icon);
                // The chapter's ordinal, between the icon and the title — jumping by
                // number is a real workflow ("take me to chapter 19"). A separate slot,
                // never spliced into the title: the tree model's filter reads `title`,
                // `label` and `fallback_label` as three fields, and a numeral folded into
                // the first would reach every rename that seeds from it.
                if badge.is_some() {
                    item = item.center_slot(crate::widgets::StructureNumber::new(badge));
                }
                Box::new(item) as Box<dyn Widget>
            },
        )
        .auto_item_height(28.0)
        .scroll_bar_style(ScrollBarMode::Overlay)
        // Single click jumps, matching the binder tree's own convention —
        // arrow keys only move the highlight, so stepping down the list never
        // fires a jump you did not ask for.
        .activate_on(ActivateOn::SingleClick)
        .on_activate(move |idx, ctx| {
            if let Some(key) = activate_model.key_at(idx)
                && let Some((item_id, title)) = activate_model.node_of(&key)
            {
                // `node_of` yields `(Option<item_id>, title)`; rebuild the
                // shape `activate` wants and let the view-model decide. It is
                // the one that knows a binder row is not a destination.
                let jumped = activate_vm.activate(
                    &TreeNode {
                        title,
                        item_id,
                        kind: if item_id.is_some() { "item" } else { "binder" }.to_string(),
                        ..Default::default()
                    },
                    ctx,
                );
                // Dismiss the overlay itself, not merely the view-model's flag:
                // `GoToViewModel::hide` owns "is the popup logically open", but
                // the popover is presented by the framework and only an
                // `EventContext` can take it down. Without this the writer
                // jumps and the list stays sitting over the manuscript.
                //
                // `dismiss_top_overlay`, **not** `dismiss_self_overlay_chain`
                // or `dismiss_all_except_hosts`: both of those deliberately
                // preserve dialog-role surfaces, which is what a popover panel
                // is, so from inside one they are silent no-ops. This popover is
                // the topmost overlay whenever a row is activated in it.
                if jumped {
                    ctx.dismiss_top_overlay();
                }
            }
        });

        // How many rows match and the first of them, once a query has been typed.
        // Zipping the model's version signal is what makes it reactive: the row set
        // changes when the model re-sources, which the query signal alone does not
        // say. A live region, because keyboard focus stays in the field the whole
        // time: a reader says the line as it appears and whenever its words change.
        let results_vm = vm.clone();
        let results = vm
            .query()
            .zip(&model.version_signal())
            .map(move |_| results_vm.results_text());
        let status = TextWidget::new(lit!(""))
            .text(results.map(|text| text.clone().unwrap_or_default()))
            .color(TextRole::Secondary)
            .overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
            .access_live(Live::Polite)
            .visible_when(results.map(Option::is_some));

        // Down/Up move the highlight through the list **while focus stays in the
        // field**, so a writer never has to Tab out of what they are typing to
        // choose a result. Handled on the wrapping column so it applies whether
        // focus is still in the field or has moved into the tree.
        let keys_vm = vm.clone();
        let tree_handle = tree.handle();
        let step = move |delta: isize, ctx: &mut EventContext| {
            ctx.announce(keys_vm.step_and_describe(delta));
            // The tree follows its own cursor and not a selection moved from outside.
            if let Some(index) = keys_vm
                .selection()
                .selected_keys()
                .first()
                .and_then(|key| keys_vm.visible_index_of(key))
            {
                tree_handle.ensure_index_visible(index);
            }
        };
        let body = MinSize::new(POPUP_WIDTH, 0.0)
            .child(
                VStack::new()
                    .spacing(6.0)
                    .child(Padding::symmetric(0.0, 2.0).child(field))
                    .child(MinSize::new(POPUP_WIDTH, POPUP_HEIGHT).child(Expand::new().child(tree)))
                    .child(status),
            )
            .on_key(move |ev, ctx| match ev {
                WidgetEvent::KeyDown {
                    key: Key::ArrowDown,
                    ..
                } => {
                    step(1, ctx);
                    EventResponse::Handled
                }
                WidgetEvent::KeyDown {
                    key: Key::ArrowUp, ..
                } => {
                    step(-1, ctx);
                    EventResponse::Handled
                }
                _ => EventResponse::Ignored,
            });
        let id = ctx.add_boxed(Box::new(body));
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// The status-bar / distraction-free-strip trigger plus its popover.
pub struct GoToButton {
    vm: GoToViewModel,
    /// The named action that opens *this* instance.
    ///
    /// Two instances exist per window — one in the status bar, one in the
    /// distraction-free strip — and only ever one of them is on screen. They
    /// must therefore not share a name: a single `go.to` registered twice is
    /// answered by whichever registration wins, and if that is the hidden one
    /// its trigger has no live bounds, so the popover opens anchored to
    /// nothing — in the top-left corner of the screen. `app/commands/go.rs`
    /// owns the public `go.to` and forwards it to whichever of these is
    /// currently visible.
    action: &'static str,
    root: Option<WidgetId>,
}

impl GoToButton {
    pub fn new(vm: GoToViewModel, action: &'static str) -> Self {
        Self {
            vm,
            action,
            root: None,
        }
    }
}

impl std::fmt::Debug for GoToButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoToButton").finish()
    }
}

impl Widget for GoToButton {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let vm = self.vm.clone();
        let content = GoToPalette {
            vm: vm.clone(),
            root: None,
        };
        // `on_open` clears the stale query (see `GoToViewModel::show`'s doc):
        // reopening pre-filtered to a search you cannot see the cause of reads
        // as a binder that lost most of its rows.
        let opening = vm.clone();
        let closing = vm.clone();
        let root = ctx.add(
            // A flat icon trigger, matching the strip's own ‹ › pair and the
            // status bar's other controls — a text button was the odd one out in
            // both places, and in the strip it competed with Exit, the one
            // control there that must read as the primary action.
            PopoverIconButton::new(
                IconButton::new(crate::icons::go::go_to_icon())
                    .toolbar()
                    .tooltip(tr!(tooltip_go_to())),
            )
            .content(content)
            // `IconButton` triggers default the disclosure caret ON; off here,
            // because the strip is a row of bare glyphs and one carrying a
            // chevron would read as a different kind of control.
            .show_disclosure_caret(false)
            // The trigger sits at the very bottom of the window (status bar, or
            // the distraction-free strip on a fullscreen screen), so the list
            // opens upward. Anchored below would land off-screen.
            .placement(OverlayPlacement::Above)
            .open_action(self.action)
            .on_open(move || opening.show())
            .on_close(move || closing.hide()),
        );
        self.root = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// Against the real store: the mock rows are fixed, and these tests need a row with no
/// title, a folder kept only for what it holds, and more than one match.
#[cfg(all(test, not(feature = "mocks")))]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;
    use std::time::Duration;

    use frontend::AppContext;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};
    use teksilo::core::widget_tree::WidgetTree;

    use crate::app_ids::AppIds;
    use crate::models::BinderTreeKey;
    use crate::test_support::with_real_messages;

    const TOOLTIP: &str = "Jump to any item in the binder (Ctrl+G)";

    /// A book holding two scenes whose names share a word, one that does not, and an
    /// untitled chapter, which the binder names by its number.
    fn seeded() -> (GoToViewModel, Rc<AppContext>) {
        let app_ctx = Rc::new(AppContext::new());
        let work = work_commands::create_orphan_work(
            &app_ctx,
            None,
            &CreateWorkDto {
                statuses: Vec::new(),
                number_chapters: true,
                dict_language: vec!["en".into()],
                ..Default::default()
            },
        )
        .expect("the in-memory store always accepts a Work");
        let binder = binder_commands::create_binder(
            &app_ctx,
            None,
            &CreateBinderDto {
                name: "Manuscript".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            0,
        )
        .expect("the in-memory store always accepts a binder");
        for (title, role, sub_role, indent) in [
            (
                "The Book",
                BinderItemRole::Folder,
                BinderItemSubRole::Book,
                0,
            ),
            ("Arrival", BinderItemRole::Item, BinderItemSubRole::Scene, 1),
            (
                "Second arrival",
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
            ),
            (
                "Departure",
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
            ),
            ("", BinderItemRole::Item, BinderItemSubRole::ChapterScene, 1),
        ] {
            binder_item_commands::create_binder_item(
                &app_ctx,
                None,
                &CreateBinderItemDto {
                    status: None,
                    title: title.into(),
                    role,
                    sub_role,
                    activated: true,
                    is_exportable: true,
                    indent,
                    ..Default::default()
                },
                binder.id,
                -1,
            )
            .expect("the in-memory store always accepts an item");
        }
        let ids = AppIds::default();
        ids.work_id.set(Some(work.id));
        let vm = GoToViewModel::new(app_ctx.clone(), ids);
        vm.model().reload();
        (vm, app_ctx)
    }

    /// The count is of rows the arrows and Enter reach. The book is on screen for what it
    /// holds, and counting it would promise a row nobody can go to.
    #[test]
    fn the_results_say_how_many_match_and_which_comes_first() {
        with_real_messages(|| {
            let (vm, _ctx) = seeded();
            assert_eq!(vm.results_text(), None, "no query, nothing to say");

            vm.query().set("arriv".into());
            assert_eq!(
                vm.results_text().as_deref(),
                Some("Matches: 2. First: Arrival")
            );

            vm.query().set("zzz".into());
            assert_eq!(
                vm.results_text().as_deref(),
                Some("No item matches that search.")
            );
        });
    }

    /// Typing what the row reads has to reach it. The filter shows an untitled chapter for
    /// its generated name; a destination that read the title alone then skipped it, and
    /// Enter did nothing on a list showing the very row asked for.
    #[test]
    fn a_row_found_by_its_generated_name_is_a_destination() {
        with_real_messages(|| {
            let (vm, _ctx) = seeded();
            let chapter = vm.model().node(&visible_key(&vm, "")).expect("the chapter");
            let name = chapter.fallback_label.clone().expect("a generated name");
            vm.query().set(name.clone());
            let results = vm.results_text().expect("a query was typed");
            assert!(
                results.contains(&name) && results.starts_with("Matches: 1."),
                "got {results:?}"
            );
            assert!(
                vm.first_openable().is_some(),
                "Enter must have a row to open"
            );
        });
    }

    fn visible_key(vm: &GoToViewModel, title: &str) -> BinderTreeKey {
        let model = vm.model();
        (0..model.visible_count())
            .filter_map(|i| model.key_at(i))
            .find(|k| {
                model
                    .node(k)
                    .is_some_and(|n| n.kind != "binder" && n.title == title)
            })
            .expect("the row is in the list")
    }

    /// The arrows say the row and its place among the matches, and an arrow at the end of
    /// the list says the same row again instead of saying nothing.
    #[test]
    fn the_arrows_describe_the_row_they_reach() {
        with_real_messages(|| {
            let (vm, _ctx) = seeded();
            vm.query().set("arriv".into());
            assert_eq!(vm.step_and_describe(1), "Arrival, Scene, 1 of 2");
            assert_eq!(vm.step_and_describe(1), "Second arrival, Scene, 2 of 2");
            assert_eq!(vm.step_and_describe(1), "Second arrival, Scene, 2 of 2");
            assert_eq!(vm.step_and_describe(-1), "Arrival, Scene, 1 of 2");

            vm.query().set("zzz".into());
            assert_eq!(vm.step_and_describe(1), "No item matches that search.");
        });
    }

    fn settle(tree: &mut WidgetTree) {
        let size = SizeProposal::exact(900.0, 700.0);
        for _ in 0..3 {
            tree.layout(size);
            tree.tick_animations(Duration::from_millis(400));
            tree.layout(size);
            let _ = tree.sync_accessibility();
        }
    }

    fn heard_since(tree: &WidgetTree, mark: u64) -> Vec<String> {
        tree.announcements_since(mark)
            .into_iter()
            .map(|a| a.text)
            .collect()
    }

    fn mark(tree: &WidgetTree) -> u64 {
        tree.announcements_since(0).last().map_or(0, |a| a.seq)
    }

    /// The palette itself, heard the way an adapter is handed it: focus stays in the field,
    /// so the results have to reach a reader from a live region and from the arrows.
    #[test]
    fn a_reader_hears_the_results_while_the_focus_stays_in_the_field() {
        with_real_messages(|| {
            let (vm, app_ctx) = seeded();
            let mut tree = crate::test_support::tree_with_events(&app_ctx);
            tree.set_records_announcements(true);
            tree.add(GoToPalette {
                vm: vm.clone(),
                root: None,
            });
            settle(&mut tree);
            let field = tree
                .find_by_label("Search the binder")
                .expect("the search field");
            tree.focus(field);
            settle(&mut tree);
            let focused = tree.focused();

            let m = mark(&tree);
            vm.query().set("arriv".into());
            settle(&mut tree);
            let heard = heard_since(&tree, m);
            assert!(
                heard.iter().any(|t| t == "Matches: 2. First: Arrival"),
                "typing must be answered, heard {heard:?}"
            );

            let m = mark(&tree);
            tree.press_key(Key::ArrowDown, Modifiers::NONE);
            settle(&mut tree);
            let heard = heard_since(&tree, m);
            assert!(
                heard.iter().any(|t| t == "Arrival, Scene, 1 of 2"),
                "Down must be answered, heard {heard:?}"
            );
            assert_eq!(
                tree.focused(),
                focused,
                "the arrows leave focus in the field"
            );
        });
    }

    /// Enter closes the popup, and the popup giving the keyboard back must not undo the
    /// jump: the focus the open edge asked for is the one that stands.
    #[test]
    fn enter_leaves_the_focus_where_the_jump_put_it() {
        with_real_messages(|| {
            let (vm, app_ctx) = seeded();
            let mut tree = crate::test_support::tree_with_events(&app_ctx);
            let target = Rc::new(Cell::new(None));
            let opened = Rc::new(Cell::new(false));
            {
                let (target, opened) = (target.clone(), opened.clone());
                vm.set_open_fn(Rc::new(move |_id, _title, ctx| {
                    opened.set(true);
                    if let Some(id) = target.get() {
                        ctx.request_focus(id);
                    }
                }));
            }
            let root = tree.add(
                VStack::new()
                    .child(teksilo::widgets::Button::new(lit!("Where the writer was")))
                    .child(teksilo::widgets::Button::new(lit!("Where the jump lands")))
                    .child(GoToButton::new(vm.clone(), GO_TO_MAIN)),
            );
            settle(&mut tree);
            let _ = root;
            target.set(tree.find_by_label("Where the jump lands"));
            let was = tree
                .find_by_label("Where the writer was")
                .expect("a control");
            tree.focus(was);
            settle(&mut tree);

            let trigger = tree.find_by_label(TOOLTIP).expect("the trigger");
            tree.click(trigger);
            settle(&mut tree);
            assert!(vm.is_open(), "the popup opened");
            vm.query().set("arriv".into());
            settle(&mut tree);

            tree.press_key(Key::Enter, Modifiers::NONE);
            settle(&mut tree);

            assert!(opened.get(), "Enter opened the first match");
            assert!(!vm.is_open(), "the popup closed");
            assert_eq!(tree.focused(), target.get(), "the jump's focus stands");
        });
    }
}
