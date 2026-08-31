// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `TabMenuViewModel` — the policy behind the editor tab strip's context menu.
//!
//! Sits *above* [`EditorsViewModel`]: it decides
//! which rows a given tab is offered and performs the one verb that crosses a
//! window boundary, and it reaches every other verb by calling down. Nothing in
//! `editors_vm.rs` names this module — the edge runs one way, and the menu is
//! installed into the tabs through an injected
//! [`TabMenuInstaller`](crate::editors::TabMenuInstaller).
//!
//! Tier 3 (per window), held by `App` as an `Rc` so the `Weak`s baked into every
//! tab's menu factory stay valid across widget rebuilds.
//!
//! The menu's shape is published as **data** ([`TabMenuItem`]), not as a widget
//! tree: every enablement rule is then a plain unit test with no widget tree, no
//! locale and no event loop, and [`tab_menu`](crate::editors::tab_menu) renders
//! the list one row for one row.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::prelude::EventContext;
use teksilo::widgets::TabId;

use frontend::AppContext;

use crate::app_ids::AppIds;
use crate::editors::{EditorsViewModel, Side};

/// One row the tab menu can offer. Ordering is fixed by [`TabMenuViewModel::rows`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TabMenuRow {
    Close,
    CloseOthers,
    CloseAll,
    Separator,
    /// Open this tab's item in the *other* pane as well, leaving this one open.
    OpenOtherSide,
    /// Move this tab to the other pane.
    MoveOtherSide,
    /// Tear this tab off into a second window on the same project.
    MoveToNewWindow,
    Pin,
    Unpin,
}

/// A row plus whether it is offered as active. A row the writer must not be able
/// to reach at all is *absent* from the list rather than present-and-disabled —
/// the app's context-menu convention, opposite to the menu bar's.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TabMenuItem {
    pub row: TabMenuRow,
    pub enabled: bool,
}

impl TabMenuItem {
    fn on(row: TabMenuRow) -> Self {
        Self { row, enabled: true }
    }

    fn gated(row: TabMenuRow, enabled: bool) -> Self {
        Self { row, enabled }
    }
}

/// The tab strip's context-menu policy and its one cross-window verb.
pub struct TabMenuViewModel {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    /// Injected after construction, for the same reason `EditorsViewModel` takes
    /// its own installer that way: this view-model must exist *before* the
    /// editors (so its installer can be baked into every tab as it is created)
    /// and must point *back* at them to act on one.
    editors: RefCell<Option<EditorsViewModel>>,
}

impl TabMenuViewModel {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        Self {
            app_ctx,
            ids,
            editors: RefCell::new(None),
        }
    }

    /// Point this view-model at the window's editors. Idempotent, so a rebuild is
    /// free.
    pub fn set_editors(&self, editors: EditorsViewModel) {
        *self.editors.borrow_mut() = Some(editors);
    }

    fn editors(&self) -> Option<EditorsViewModel> {
        self.editors.borrow().clone()
    }

    /// Which pane holds this tab, resolved **now**.
    ///
    /// Never captured when the tab was built: a tab dragged to the other pane
    /// keeps the very same handle, menu factory included, so a captured `Side`
    /// would make "Close others" empty the pane the tab came from.
    pub fn side_of(&self, tab_id: TabId) -> Option<Side> {
        self.editors()?.side_of(tab_id)
    }

    /// The caption to show in the menu's header row.
    pub fn tab_caption(&self, side: Side, tab_id: TabId) -> Option<String> {
        self.editors()?.tab_caption(side, tab_id)
    }

    pub fn is_pinned(&self, side: Side, tab_id: TabId) -> bool {
        self.editors().is_some_and(|e| e.is_pinned(side, tab_id))
    }

    /// The whole menu, in order, for one right-clicked tab.
    ///
    /// `can_open_window` is passed in rather than read here because answering it
    /// needs an `EventContext` (the process-wide `ProjectWindowFactory` lives in
    /// app state), and this method is deliberately callable from a plain unit
    /// test with no event loop at all.
    pub fn rows(&self, side: Side, tab_id: TabId, can_open_window: bool) -> Vec<TabMenuItem> {
        let Some(editors) = self.editors() else {
            return Vec::new();
        };
        let pinned = editors.is_pinned(side, tab_id);
        let tabs = editors.tab_item_ids(side).len();
        let pins = editors.pinned_item_ids(side).len();
        // A bulk close is offered only when it would actually close something.
        // "Close others" spares the clicked tab and every pinned tab; if the
        // clicked tab is itself pinned it is spared twice over, which is why the
        // arithmetic differs between the two rows.
        // Saturating throughout. In-tree the clicked tab is always one of this
        // pane's tabs, so `tabs - pins >= 1` whenever it is unpinned and the
        // subtraction is safe — but this is a `pub` method on a view-model the
        // extension seam can reach, and a caller passing a `TabId` from the other
        // pane would otherwise underflow into a debug-build panic.
        let all_closable = tabs.saturating_sub(pins);
        let others_closable = all_closable.saturating_sub(usize::from(!pinned));

        let mut rows = vec![
            // Never gated and never omitted: a pinned tab has no cross, no
            // middle-click close and no Delete key, so this row is the only way
            // to close one.
            TabMenuItem::on(TabMenuRow::Close),
            TabMenuItem::gated(TabMenuRow::CloseOthers, others_closable > 0),
            TabMenuItem::gated(TabMenuRow::CloseAll, all_closable > 0),
            TabMenuItem::on(TabMenuRow::Separator),
            // Always available: if the other pane already shows this item, the
            // row focuses that tab, which is exactly what it promises.
            TabMenuItem::on(TabMenuRow::OpenOtherSide),
            TabMenuItem::on(TabMenuRow::MoveOtherSide),
            TabMenuItem::gated(TabMenuRow::MoveToNewWindow, can_open_window),
            TabMenuItem::on(TabMenuRow::Separator),
        ];
        rows.push(TabMenuItem::on(if pinned {
            TabMenuRow::Unpin
        } else {
            TabMenuRow::Pin
        }));
        rows
    }

    // ── The verbs ───────────────────────────────────────────────────────────

    pub fn close(&self, side: Side, tab_id: TabId) {
        if let Some(e) = self.editors() {
            e.close_in(side, tab_id);
        }
    }

    pub fn close_others(&self, side: Side, tab_id: TabId) {
        if let Some(e) = self.editors() {
            e.close_others_in(side, tab_id);
        }
    }

    pub fn close_all(&self, side: Side) {
        if let Some(e) = self.editors() {
            e.close_all_in(side);
        }
    }

    pub fn open_other_side(&self, side: Side, tab_id: TabId, ctx: &mut EventContext) {
        if let Some(e) = self.editors() {
            e.open_other_side(side, tab_id, ctx);
        }
    }

    pub fn move_other_side(&self, side: Side, tab_id: TabId) {
        if let Some(e) = self.editors() {
            e.move_tab_to(side, side.other(), tab_id);
        }
    }

    pub fn toggle_pin(&self, side: Side, tab_id: TabId) {
        if let Some(e) = self.editors() {
            e.toggle_pin(side, tab_id);
        }
    }

    /// Can this window hand a tab to a second window right now?
    ///
    /// Three things must hold, and all three are genuinely refusable: a `Work`
    /// must be open, its path must resolve (a project that has never been saved
    /// has no file for a second window to open onto), and the process-wide
    /// factory must be registered. `ProjectWindowFactory` is one of the two
    /// lookups that are legitimately `app_state` — a Tier-1 service, not
    /// per-window state.
    pub fn can_open_new_window(&self, ctx: &mut EventContext) -> bool {
        self.new_window_target(ctx).is_some()
    }

    /// The three things a tear-off needs, or `None` if any is missing.
    fn new_window_target(
        &self,
        ctx: &mut EventContext,
    ) -> Option<(u64, String, crate::shell::windows::ProjectWindowFactory)> {
        let work_id = self.ids.work_id.get()?;
        // Resolved through *this* window's `work_info_id`, never
        // `get_all_work_info`'s first entry, which with several Works open
        // answers about somebody else's project.
        let path = crate::current_project_path(&self.app_ctx, &self.ids)?;
        if path.is_empty() {
            return None;
        }
        let factory = ctx
            .app_state::<crate::shell::windows::ProjectWindowFactory>()
            .cloned()?;
        Some((work_id, path, factory))
    }

    /// Tear this tab off into a second window on the same project.
    ///
    /// The order is load-bearing and is the whole reason this lives here rather
    /// than on `EditorsViewModel`:
    ///
    /// 1. **Write down where the writer was**, while the tab still exists — the
    ///    new window seeds its own tab from that same per-project roster.
    /// 2. **Open the window**, which is synchronous: its root builder, hence
    ///    `App::build` and the attach seed that opens the item, all run before
    ///    this call returns.
    /// 3. **Only then close the source tab.** Reversed, the close would drop the
    ///    last `OpenDoc` reference, the store would evict and flush the document,
    ///    and the new window would rebuild it from disk — losing any in-memory
    ///    editor state that had not been written back.
    ///
    /// The tab is re-*opened* in the new window rather than handed over as a
    /// `TabHandle`, deliberately: a `ContentTab` holds the **source** window's
    /// `FormatViewModel`, so a migrated handle would register its editors with
    /// the wrong window's format registry and the Format dock over there would
    /// target nothing.
    ///
    /// `ctx.open_window` panics outside a dispatch; a menu row's
    /// `on_activate_fn` is a real one, which is why this may not be called from
    /// an effect.
    pub fn move_to_new_window(&self, side: Side, tab_id: TabId, ctx: &mut EventContext) {
        let Some(editors) = self.editors() else {
            return;
        };
        let Some(item_id) = editors.item_of(side, tab_id) else {
            return;
        };
        let Some((work_id, path, factory)) = self.new_window_target(ctx) else {
            return;
        };
        editors.remember_position_of(side, tab_id);
        // `attached_window_config` bumps the Work's session refcount and consumes
        // a window ordinal before the window exists, and the only release is the
        // window's own `on_removed` — so a config that is built must always be
        // opened. Nothing fallible sits between here and `open_window`.
        let Some((config, _state)) = factory.attached_window_config(work_id, &path, Some(item_id))
        else {
            return;
        };
        ctx.open_window(config);
        editors.close_in(side, tab_id);
    }
}

#[cfg(test)]
mod tests;
