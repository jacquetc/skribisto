// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The menu's rendering. Its *policy* is tested as data next door in
//! `tab_menu_vm/tests.rs`; what is left to prove here is that every row the
//! policy hands over really becomes a `MenuItem`, that the labels follow the
//! pane, and that the header names the tab the writer actually right-clicked.

use super::*;
use crate::editors::test_support::{
    app_ctx_of, editors as detached_editors, ids_of, push_scene_tab,
};
use crate::editors::{EditorsViewModel, TabMenuViewModel};
use teksilo::core::widget_tree::WidgetTree;
use teksilo::prelude::{SizeProposal, WidgetId};

fn fixture() -> (EditorsViewModel, Rc<TabMenuViewModel>) {
    let editors = detached_editors();
    let vm = Rc::new(TabMenuViewModel::new(
        app_ctx_of(&editors),
        ids_of(&editors),
    ));
    vm.set_editors(editors.clone());
    (editors, vm)
}

/// Every `MenuItem` label in a built menu, in tree order.
fn labels_of(menu: MenuList) -> Vec<String> {
    fn walk(tree: &WidgetTree, id: WidgetId, out: &mut Vec<String>) {
        if let Some(item) = tree
            .widget_as_any(id)
            .and_then(|a| a.downcast_ref::<MenuItem>())
        {
            out.push(item.label());
        }
        for child in tree.children(id) {
            walk(tree, child, out);
        }
    }
    let mut tree = WidgetTree::new();
    let root = tree.add(menu);
    tree.layout(SizeProposal::exact(400.0, 400.0));
    let mut out = Vec::new();
    walk(&tree, root, &mut out);
    out
}

/// Every accessible name in a built menu.
///
/// The header has to be read through the accessibility tree rather than by
/// downcasting: `MenuList` mounts it with `ctx.add_boxed`, so `widget_as_any`
/// hands back the box rather than the `GroupHeader` inside it. That is not a
/// workaround — the AT tree is where a section caption is *supposed* to show
/// up, and `GroupHeader`'s own `accessibility()` is what puts it there, so this
/// asserts the property that actually matters to a screen-reader user.
fn names_of(menu: MenuList) -> Vec<String> {
    let mut tree = WidgetTree::new().with_theme(teksilo::presets::intui::light());
    let _root = tree.add(menu);
    tree.layout(SizeProposal::exact(400.0, 400.0));
    let _ = tree.render();
    let update = tree.sync_accessibility();
    update
        .nodes
        .iter()
        .filter_map(|(_, node)| {
            node.value()
                .map(str::to_string)
                .or_else(|| node.label().map(|l| l.to_string()))
        })
        .collect()
}

/// The rendered menu carries one `MenuItem` per non-separator row the policy
/// offered — the guard on a missing `match` arm in `row_label_and_action`, which
/// would otherwise drop a row silently.
#[test]
fn the_rendered_menu_matches_the_row_list_one_for_one() {
    let (editors, vm) = fixture();
    let tab = push_scene_tab(&editors, Side::Primary, 1);

    let expected = vm
        .rows(Side::Primary, tab, true)
        .into_iter()
        .filter(|i| i.row != TabMenuRow::Separator)
        .count();

    let labels = labels_of(tab_context_menu(&vm, Side::Primary, tab, true));
    assert_eq!(labels.len(), expected, "rendered rows: {labels:?}");
    assert!(labels.iter().all(|l| !l.is_empty()));
}

/// From the main pane the destination rows name the side; from the side pane
/// they name the main pane. A single "Split editor" label would be wrong from
/// the side — the split is already open there, and what the row does is show the
/// document on the left as well.
#[test]
fn the_destination_rows_name_the_pane_they_would_reach() {
    let (editors, vm) = fixture();
    let main = push_scene_tab(&editors, Side::Primary, 1);
    editors.set_split(true);
    let side = push_scene_tab(&editors, Side::Secondary, 2);

    let from_main = labels_of(tab_context_menu(&vm, Side::Primary, main, true));
    let from_side = labels_of(tab_context_menu(&vm, Side::Secondary, side, true));

    assert_ne!(
        from_main, from_side,
        "the two panes offered identical labels, so one of them names the wrong destination"
    );
}

/// The header names the right-clicked tab.
///
/// Teksilo returns early on a successful Secondary `PointerDown`, so a
/// right-click neither selects nor focuses the tab it lands on. Without this row
/// a menu opened on an unselected tab is indistinguishable from one opened on
/// the selected one — and "Close others" reads as having closed the wrong set.
///
/// Needs a real store to have a title to name, so it does not run under
/// `--features mocks`; the rendering it proves is feature-independent.
#[cfg(not(feature = "mocks"))]
#[test]
fn the_header_row_names_the_tab_the_menu_was_opened_on() {
    use crate::editors::test_support::{seed_item, seed_work};
    use frontend::common::entities::BinderItemSubRole;

    let (editors, vm) = fixture();
    let binder = seed_work(&editors);
    let storm = seed_item(&editors, binder, "The Storm", BinderItemSubRole::Scene);
    let calm = seed_item(&editors, binder, "The Calm", BinderItemSubRole::Scene);
    editors.open_in(Side::Primary, storm, "The Storm");
    editors.open_in(Side::Primary, calm, "The Calm");
    // Select the *first* tab, then open the menu on the second.
    editors.select_item(Side::Primary, storm);
    let second = editors
        .tab_id_of_item(Side::Primary, calm)
        .expect("the second tab is open");

    let names = names_of(tab_context_menu(&vm, Side::Primary, second, true));

    assert!(
        names.contains(&"The Calm".to_string()),
        "the header must name the right-clicked tab, not the selected one; got {names:?}"
    );
    assert!(
        !names.contains(&"The Storm".to_string()),
        "the header named the selected tab instead of the clicked one; got {names:?}"
    );
}

/// A writer's own ampersand reaches the header intact.
///
/// `MenuItem` runs its mnemonic parser over every label it is handed, `lit!`
/// data included — which is why the header is a `GroupHeader` (a `TextWidget`)
/// and not a `MenuItem`. A chapter called "Cast & Crew" would otherwise render
/// as "Cast  Crew" with the C of "Crew" silently bound as an access key.
#[cfg(not(feature = "mocks"))]
#[test]
fn a_title_with_an_ampersand_reaches_the_header_intact() {
    use crate::editors::test_support::{seed_item, seed_work};
    use frontend::common::entities::BinderItemSubRole;

    let (editors, vm) = fixture();
    let binder = seed_work(&editors);
    let id = seed_item(&editors, binder, "Cast & Crew", BinderItemSubRole::Note);
    editors.open_in(Side::Primary, id, "Cast & Crew");
    let tab = editors
        .tab_id_of_item(Side::Primary, id)
        .expect("the tab is open");

    let names = names_of(tab_context_menu(&vm, Side::Primary, tab, true));

    assert!(
        names.contains(&"Cast & Crew".to_string()),
        "the ampersand was eaten by a mnemonic parser the header must not have; got {names:?}"
    );
}

/// A disabled row stays *visible*, so its tooltip can say why.
///
/// The app's context-menu convention is to omit an inapplicable row rather than
/// grey it — but "Move into a new window" is the exception on purpose: a writer
/// whose project has never been saved needs to be told that, not to watch the
/// row disappear.
#[test]
fn move_into_a_new_window_stays_visible_when_it_is_unavailable() {
    let (editors, vm) = fixture();
    let tab = push_scene_tab(&editors, Side::Primary, 1);

    let with_window = labels_of(tab_context_menu(&vm, Side::Primary, tab, true));
    let without = labels_of(tab_context_menu(&vm, Side::Primary, tab, false));

    assert_eq!(
        with_window.len(),
        without.len(),
        "the unavailable row was removed instead of disabled"
    );
}

/// A pinned tab is offered Unpin, and the label really changes in the render —
/// not just in the row list.
#[test]
fn a_pinned_tabs_menu_offers_unpin() {
    let (editors, vm) = fixture();
    let tab = push_scene_tab(&editors, Side::Primary, 1);

    let before = labels_of(tab_context_menu(&vm, Side::Primary, tab, true));
    editors.pin(Side::Primary, tab);
    let after = labels_of(tab_context_menu(&vm, Side::Primary, tab, true));

    assert_ne!(
        before.last(),
        after.last(),
        "pinning did not change the last row's label"
    );
}

/// Activating "Close others" closes exactly the others.
///
/// Fired through the built `MenuItem`'s own action, so the wiring from label to
/// verb is under test and not just the verb. No `MenuActionHost` stand-in is
/// needed — unlike the binder's rows, these call the view-model directly rather
/// than dispatching a global action, which is a real ergonomic win of putting
/// the policy in a view-model.
#[test]
fn activating_close_others_closes_exactly_the_others() {
    let (editors, vm) = fixture();
    let keep = push_scene_tab(&editors, Side::Primary, 1);
    let _b = push_scene_tab(&editors, Side::Primary, 2);
    let _c = push_scene_tab(&editors, Side::Primary, 3);

    let action = {
        let (_label, action) =
            super::row_label_and_action(&vm, TabMenuRow::CloseOthers, Side::Primary, keep);
        action
    };
    crate::editors::test_support::with_event_context(move |ctx| action(ctx));

    assert_eq!(editors.tab_item_ids(Side::Primary), vec![1]);
}

/// The menu acts on the tab that was right-clicked, **not** on the selected one.
#[test]
fn the_menu_acts_on_the_tab_that_was_right_clicked_not_the_selected_one() {
    let (editors, vm) = fixture();
    let a = push_scene_tab(&editors, Side::Primary, 1);
    let b = push_scene_tab(&editors, Side::Primary, 2);
    let _c = push_scene_tab(&editors, Side::Primary, 3);
    // Select A, then open the menu on B.
    editors.select_item(Side::Primary, 1);
    assert_eq!(editors.selected_item(Side::Primary), Some(1));

    let (_label, action) =
        super::row_label_and_action(&vm, TabMenuRow::CloseOthers, Side::Primary, b);
    crate::editors::test_support::with_event_context(move |ctx| action(ctx));

    assert_eq!(
        editors.tab_item_ids(Side::Primary),
        vec![2],
        "the menu closed the wrong set — it followed the selection instead of the click"
    );
    let _ = a;
}
