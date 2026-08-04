// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Dock widgets placed on the app's `DockingLayout`. Each dock owns its own
//! content builder and packages it as a `DockWidget` for `App` to mount on a
//! side; `App` only wires the cross-view-model effects around them.
//!
//! The roster: [`outline`] (binder tree), [`search`], [`trash`], [`comments`]
//! (project-wide) on the leading rail; [`inspector`], [`mod@format`], and a
//! per-document comments dock on the trailing rail; [`search_preview`] on the
//! bottom. See [`APP_DOCKS`] for the authoritative list and mount order.
//!
//! ## Stable dock ids
//!
//! `DockLayoutState` (bastyde's serialisable dock layout) keys panes by raw
//! `DockWidgetId` (`u64`). For the per-work layout restore
//! ([`crate::view_models::WorkspaceLayoutViewModel`]) to match a saved layout to
//! this run's docks, each logical dock must carry the **same** id every launch —
//! a `DockWidgetId::fresh()` (a per-process atomic counter) would mint a
//! different id each run and `import_state` would drop the whole saved tree as
//! "unknown". So the four app docks use these fixed ids instead of `fresh()`.
//!
//! The base is deliberately high so it can never collide with a `fresh()` id
//! (the framework mints those from `1`, e.g. for a user-dragged dock split).

use bastyde::widgets::{DockOpenLocation, DockSide, DockWidgetId};

/// Base for the fixed app-dock ids (see the module docs). Chosen well above any
/// `DockWidgetId::fresh()` value the process could reach.
const DOCK_ID_BASE: u64 = 0xD0C_0000;
/// The binder outline (leading rail).
pub const OUTLINE_DOCK_ID: u64 = DOCK_ID_BASE + 1;
/// Search & replace (leading rail, second tab).
pub const SEARCH_DOCK_ID: u64 = DOCK_ID_BASE + 2;
/// The context inspector (trailing rail).
pub const INSPECTOR_DOCK_ID: u64 = DOCK_ID_BASE + 3;
/// The bottom search-preview band.
pub const PREVIEW_DOCK_ID: u64 = DOCK_ID_BASE + 4;
/// The trash panel (leading rail, third tab).
pub const TRASH_DOCK_ID: u64 = DOCK_ID_BASE + 5;
/// The trailing Format dock — the manuscript's formatting controls.
pub const FORMAT_DOCK_ID: u64 = DOCK_ID_BASE + 6;
/// Project-wide comments (leading rail, fourth tab).
pub const COMMENTS_DOCK_ID: u64 = DOCK_ID_BASE + 7;
/// This document's comments (trailing rail, third tab).
pub const DOC_COMMENTS_DOCK_ID: u64 = DOCK_ID_BASE + 8;

/// One app dock's declared home: its stable id plus where it mounts on a desk
/// nobody has arranged yet.
pub struct AppDock {
    pub id: u64,
    pub side: DockSide,
    /// `true` → its own rail tab (one panel visible at a time); `false` → stacked
    /// into the side's active tab (a vertical split showing both at once).
    pub own_tab: bool,
}

impl AppDock {
    pub fn widget_id(&self) -> DockWidgetId {
        DockWidgetId::from_raw(self.id)
    }

    /// Where [`DockingModel::open_dock`](bastyde::widgets::DockingModel::open_dock)
    /// should put it.
    pub fn location(&self) -> DockOpenLocation {
        let loc = DockOpenLocation::side(self.side);
        if self.own_tab { loc.new_tab() } else { loc }
    }
}

/// **The app's dock roster**, in first-run mount order.
///
/// This is a table rather than just the call sequence in
/// `project_shell` because it has two consumers that
/// must never disagree:
///
/// 1. the **first-run mount**, which walks it to arrange a fresh desk; and
/// 2. the **restore-time reconcile**
///    ([`WorkspaceLayoutViewModel::restore`](crate::view_models::WorkspaceLayoutViewModel::restore)),
///    which mounts any dock a saved desk has never heard of.
///
/// Consumer 2 is why adding a dock here is the *whole* job. A saved
/// `DockLayoutState` is a closed list: `import_state` rebuilds the rail purely
/// from the snapshot, so a dock that is registered but unmentioned is silently
/// never mounted on a desk saved before it existed. One added straight to
/// `project_shell` and not to this table would be invisible to every existing
/// project.
pub const APP_DOCKS: &[AppDock] = &[
    // Leading rail — "where am I in the project": four switchable activities.
    AppDock {
        id: OUTLINE_DOCK_ID,
        side: DockSide::Leading,
        own_tab: true,
    },
    AppDock {
        id: SEARCH_DOCK_ID,
        side: DockSide::Leading,
        own_tab: true,
    },
    AppDock {
        id: TRASH_DOCK_ID,
        side: DockSide::Leading,
        own_tab: true,
    },
    AppDock {
        id: COMMENTS_DOCK_ID,
        side: DockSide::Leading,
        own_tab: true,
    },
    // Trailing rail — "what is in front of me right now". The inspector is the
    // side's first tab (stacked, since the side starts empty); the other two join
    // it as sibling tabs rather than starving it in a split.
    AppDock {
        id: INSPECTOR_DOCK_ID,
        side: DockSide::Trailing,
        own_tab: false,
    },
    AppDock {
        id: FORMAT_DOCK_ID,
        side: DockSide::Trailing,
        own_tab: true,
    },
    AppDock {
        id: DOC_COMMENTS_DOCK_ID,
        side: DockSide::Trailing,
        own_tab: true,
    },
    // The transient search-preview band. Mounted, then hidden — see
    // `project_shell` and `WorkspaceLayoutViewModel`'s module docs.
    AppDock {
        id: PREVIEW_DOCK_ID,
        side: DockSide::Bottom,
        own_tab: false,
    },
];

/// Every roster id, for the `known_docks` set persisted with a captured desk.
pub fn app_dock_ids() -> Vec<u64> {
    APP_DOCKS.iter().map(|d| d.id).collect()
}

pub mod comments;
pub mod create_split_button;
pub mod format;
pub mod inspector;
pub mod outline;
pub mod search;
pub mod search_preview;
pub mod search_replace_flow;
pub mod trash;
