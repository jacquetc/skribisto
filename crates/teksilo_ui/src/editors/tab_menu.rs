// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The editor tab strip's context menu — the view over
//! [`TabMenuViewModel`].
//!
//! Thin by construction: [`tab_context_menu`] renders
//! [`TabMenuViewModel::rows`](super::TabMenuViewModel::rows) one row for one row
//! and holds no policy of its own, so every enablement rule is testable as data
//! and a missing arm here is a missing *row*, not a wrong behaviour.
//!
//! `pub`, not `pub(crate)`: a `pub` item inside a `pub(crate)` module is
//! unreachable, and `App` installs [`installer`] from outside this module.

use std::rc::{Rc, Weak};

use teksilo::prelude::*;
use teksilo::widgets::{GroupHeader, MenuItem, MenuList};

use super::{Side, TabMenuInstaller, TabMenuRow, TabMenuViewModel};
use teksilo::widgets::TabId;

/// The installer baked into every editor tab's `TabInfo`.
///
/// **Captures a `Weak`, never a strong handle.** `TabInfo.context_menu` is an
/// `Rc<dyn Fn>` stored inside a `TabHandle` inside the pane's
/// `ListModel<TabHandle>`, itself an `Rc<RefCell<..>>` field of
/// `EditorsViewModel` — so capturing an `EditorsViewModel` (or a strong
/// `TabMenuViewModel`, which holds one) closes the cycle
/// `ListModel → TabHandle → TabInfo → closure → view-model → the same ListModel`.
/// Every tab, every `ContentTab` and every `Rc<OpenDoc>` of a closed window would
/// then survive for the life of the process. Nothing panics and nothing fails;
/// the memory simply never comes back.
///
/// The `Weak` also buys the live reads this menu needs: unlike a factory that
/// captured only `Copy` ids, it can resolve the pane, the caption and each row's
/// enablement at the moment the menu opens — which is the only moment any of
/// them is known to be true.
pub fn installer(vm: &Rc<TabMenuViewModel>) -> TabMenuInstaller {
    let weak = Rc::downgrade(vm);
    Rc::new(move |id: TabId, info: teksilo::widgets::TabInfo| {
        let weak: Weak<TabMenuViewModel> = weak.clone();
        info.context_menu(move |_pos, ctx| {
            // The window this tab belonged to is gone: decline, and let the
            // click fall through to an ancestor.
            let vm = weak.upgrade()?;
            // Resolved now, never captured — a tab dragged across panes keeps
            // this very closure.
            let side = vm.side_of(id)?;
            let can_open_window = vm.can_open_new_window(ctx);
            Some(Box::new(tab_context_menu(&vm, side, id, can_open_window)) as Box<dyn Widget>)
        })
    })
}

/// Build the menu for one right-clicked tab.
///
/// The `_pos` teksilo hands the factory is deliberately ignored: it is in world
/// coordinates despite what its doc comment says, so anything computed from it
/// would be wrong by the tab's origin. The overlay anchors itself.
pub fn tab_context_menu(
    vm: &Rc<TabMenuViewModel>,
    side: Side,
    tab_id: TabId,
    can_open_window: bool,
) -> MenuList {
    let mut menu = MenuList::new();

    // Which tab is this menu about?
    //
    // Not decoration: teksilo returns early on a successful Secondary
    // `PointerDown`, so a right-click neither selects nor focuses the tab it
    // lands on. Without this row, a menu opened on an *unselected* tab looks
    // exactly like one opened on the selected one — and "Close others" would
    // read as having closed the wrong set.
    //
    // A `GroupHeader` is a `TextWidget`, not a `MenuItem`, so it parses no
    // mnemonic: a chapter called "Cast & Crew" arrives intact instead of
    // rendering as "Cast  Crew" with a stolen access key.
    if let Some(caption) = vm.tab_caption(side, tab_id) {
        menu = menu.header(GroupHeader::new(lit!(caption)));
    }

    for item in vm.rows(side, tab_id, can_open_window) {
        menu = match item.row {
            TabMenuRow::Separator => menu.separator(),
            row => {
                let (label, action) = row_label_and_action(vm, row, side, tab_id);
                let mut entry = MenuItem::new(label).on_activate_fn(action);
                if !item.enabled {
                    entry = entry.enabled(false);
                    if row == TabMenuRow::MoveToNewWindow {
                        // The one disabled row whose reason is not obvious from
                        // the strip: a project that has never been saved has no
                        // file on disk for a second window to open onto.
                        entry = entry.tooltip(tr!(ctx_tab_move_window_unsaved()));
                    }
                }
                menu.item(entry)
            }
        };
    }
    menu
}

/// What activating one menu row does. Boxed because each arm closes over a
/// different capture set, so they have no common concrete type.
type RowAction = Box<dyn Fn(&mut EventContext)>;

/// One row's label and what activating it does.
///
/// The four directional rows read differently depending on which pane the menu
/// was opened in — "Open to the Side" from the main pane, "Open in the main
/// pane" from the side pane — because a single "Split editor" label is simply
/// wrong from the side: the split is already open, and what the row does is show
/// the document on the left as well.
fn row_label_and_action(
    vm: &Rc<TabMenuViewModel>,
    row: TabMenuRow,
    side: Side,
    tab_id: TabId,
) -> (LocalizedString, RowAction) {
    let vm = vm.clone();
    match row {
        TabMenuRow::Close => (
            tr!(ctx_tab_close()),
            Box::new(move |_ctx: &mut EventContext| vm.close(side, tab_id)),
        ),
        TabMenuRow::CloseOthers => (
            tr!(ctx_tab_close_others()),
            Box::new(move |_ctx: &mut EventContext| vm.close_others(side, tab_id)),
        ),
        TabMenuRow::CloseAll => (
            tr!(ctx_tab_close_all()),
            Box::new(move |_ctx: &mut EventContext| vm.close_all(side)),
        ),
        TabMenuRow::OpenOtherSide => (
            match side {
                Side::Primary => tr!(ctx_tab_open_to_side()),
                Side::Secondary => tr!(ctx_tab_open_in_main()),
            },
            Box::new(move |ctx: &mut EventContext| vm.open_other_side(side, tab_id, ctx)),
        ),
        TabMenuRow::MoveOtherSide => (
            match side {
                Side::Primary => tr!(ctx_tab_move_to_side()),
                Side::Secondary => tr!(ctx_tab_move_to_main()),
            },
            Box::new(move |_ctx: &mut EventContext| vm.move_other_side(side, tab_id)),
        ),
        TabMenuRow::MoveToNewWindow => (
            tr!(ctx_tab_move_to_new_window()),
            Box::new(move |ctx: &mut EventContext| vm.move_to_new_window(side, tab_id, ctx)),
        ),
        TabMenuRow::Pin => (
            tr!(ctx_tab_pin()),
            Box::new(move |_ctx: &mut EventContext| vm.toggle_pin(side, tab_id)),
        ),
        TabMenuRow::Unpin => (
            tr!(ctx_tab_unpin()),
            Box::new(move |_ctx: &mut EventContext| vm.toggle_pin(side, tab_id)),
        ),
        // Handled by the caller, which never reaches here.
        TabMenuRow::Separator => (
            lit!(""),
            Box::new(|_ctx: &mut EventContext| {}) as RowAction,
        ),
    }
}

#[cfg(test)]
mod tests;
