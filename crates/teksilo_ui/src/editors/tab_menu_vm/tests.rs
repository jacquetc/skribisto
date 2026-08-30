// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The menu's policy, tested as **data**.
//!
//! No widget tree, no locale, no event loop: `rows()` returns a `Vec<TabMenuItem>`,
//! so every enablement rule here is a plain equality assertion. That is the point
//! of publishing the shape rather than building the widgets — a rule proved here
//! cannot be broken by the renderer, and the renderer's own tests only have to
//! prove it renders what it was handed.

use super::*;
use crate::editors::EditorsViewModel;
use crate::editors::test_support::{
    app_ctx_of, editors as detached_editors, ids_of, push_scene_tab,
};

/// Build a detached editors view-model plus the menu view-model over it, sharing
/// one `AppContext` — the same shape `App` wires, minus the window.
fn fixture() -> (EditorsViewModel, Rc<TabMenuViewModel>) {
    let editors = detached_editors();
    let vm = Rc::new(TabMenuViewModel::new(
        app_ctx_of(&editors),
        ids_of(&editors),
    ));
    vm.set_editors(editors.clone());
    (editors, vm)
}

fn rows_of(vm: &Rc<TabMenuViewModel>, side: Side, tab: TabId) -> Vec<TabMenuRow> {
    vm.rows(side, tab, true)
        .into_iter()
        .map(|i| i.row)
        .collect()
}

fn enabled(vm: &Rc<TabMenuViewModel>, side: Side, tab: TabId, row: TabMenuRow) -> bool {
    vm.rows(side, tab, true)
        .into_iter()
        .find(|i| i.row == row)
        .map(|i| i.enabled)
        .unwrap_or(false)
}

/// The whole menu, in its fixed order. A row appearing out of place is a
/// behaviour change, not a cosmetic one: the separators are what group "close
/// things", "put it somewhere else" and "keep it".
#[test]
fn the_menu_offers_every_row_in_a_fixed_order() {
    let (editors, vm) = fixture();
    let tab = push_scene_tab(&editors, Side::Primary, 1);

    assert_eq!(
        rows_of(&vm, Side::Primary, tab),
        vec![
            TabMenuRow::Close,
            TabMenuRow::CloseOthers,
            TabMenuRow::CloseAll,
            TabMenuRow::Separator,
            TabMenuRow::OpenOtherSide,
            TabMenuRow::MoveOtherSide,
            TabMenuRow::MoveToNewWindow,
            TabMenuRow::Separator,
            TabMenuRow::Pin,
        ]
    );
}

/// Close is never gated. A pinned tab has no cross, no middle-click close and no
/// Delete key, so this row is the only way to close one — disabling it would
/// make a pinned tab unclosable.
#[test]
fn close_is_always_offered_even_for_a_pinned_tab() {
    let (editors, vm) = fixture();
    let tab = push_scene_tab(&editors, Side::Primary, 1);
    editors.pin(Side::Primary, tab);

    assert!(enabled(&vm, Side::Primary, tab, TabMenuRow::Close));
}

/// The two bulk closes are offered only when they would close something.
#[test]
fn close_others_and_close_all_are_disabled_when_they_would_close_nothing() {
    let (editors, vm) = fixture();
    let only = push_scene_tab(&editors, Side::Primary, 1);

    // A lone tab: "others" is empty, but "all" would still close this one.
    assert!(!enabled(&vm, Side::Primary, only, TabMenuRow::CloseOthers));
    assert!(enabled(&vm, Side::Primary, only, TabMenuRow::CloseAll));

    // Every tab pinned: neither would close anything at all.
    editors.pin(Side::Primary, only);
    assert!(!enabled(&vm, Side::Primary, only, TabMenuRow::CloseOthers));
    assert!(!enabled(&vm, Side::Primary, only, TabMenuRow::CloseAll));

    // A second, unpinned tab brings both back.
    let other = push_scene_tab(&editors, Side::Primary, 2);
    assert!(enabled(&vm, Side::Primary, only, TabMenuRow::CloseOthers));
    assert!(enabled(&vm, Side::Primary, other, TabMenuRow::CloseAll));
}

/// "Close others" counts the *clicked* tab as spared, so a pane of one pinned
/// plus one clicked tab offers nothing to close.
#[test]
fn close_others_ignores_the_clicked_tab_when_counting() {
    let (editors, vm) = fixture();
    let clicked = push_scene_tab(&editors, Side::Primary, 1);
    let pinned = push_scene_tab(&editors, Side::Primary, 2);
    editors.pin(Side::Primary, pinned);

    assert!(
        !enabled(&vm, Side::Primary, clicked, TabMenuRow::CloseOthers),
        "the only 'other' is pinned, so there is nothing to close"
    );
}

/// A pinned tab is offered Unpin and never Pin, and vice versa — the two are
/// mutually exclusive rows rather than one checkable row, so the menu never has
/// to reflect a `bool` that lives in a plain `HashSet` with no signal behind it.
#[test]
fn a_pinned_tab_is_offered_unpin_and_never_pin() {
    let (editors, vm) = fixture();
    let tab = push_scene_tab(&editors, Side::Primary, 1);

    let rows = rows_of(&vm, Side::Primary, tab);
    assert!(rows.contains(&TabMenuRow::Pin) && !rows.contains(&TabMenuRow::Unpin));

    editors.pin(Side::Primary, tab);

    let rows = rows_of(&vm, Side::Primary, tab);
    assert!(rows.contains(&TabMenuRow::Unpin) && !rows.contains(&TabMenuRow::Pin));
}

/// "Move into a new window" is gated on the caller's verdict — a project that
/// has never been saved has no file on disk for a second window to open onto.
#[test]
fn move_into_a_new_window_is_disabled_without_a_resolvable_project() {
    let (editors, vm) = fixture();
    let tab = push_scene_tab(&editors, Side::Primary, 1);

    let gated = vm
        .rows(Side::Primary, tab, false)
        .into_iter()
        .find(|i| i.row == TabMenuRow::MoveToNewWindow)
        .expect("the row is present, only disabled");
    assert!(!gated.enabled);
    assert!(
        vm.rows(Side::Primary, tab, false)
            .iter()
            .any(|i| i.row == TabMenuRow::MoveToNewWindow),
        "the row must stay visible so its tooltip can explain why"
    );
}

/// With no editors injected, the menu is empty rather than wrong — the graceful
/// degradation that lets a tab exist before the two-phase wiring completes.
#[test]
fn a_menu_with_no_editors_offers_nothing() {
    let editors = detached_editors();
    let vm = Rc::new(TabMenuViewModel::new(
        app_ctx_of(&editors),
        ids_of(&editors),
    ));
    assert!(vm.rows(Side::Primary, TabId::fresh(), true).is_empty());
}
