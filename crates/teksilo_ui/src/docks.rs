// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Dock widgets placed on the app's `DockingLayout`. Each dock owns its own
//! content builder and packages it as a `DockWidget` for `App` to mount on a
//! side; `App` only wires the cross-view-model effects around them.
//!
//! The roster: [`outline`] (binder tree), [`search`], [`trash`], [`comments`]
//! (project-wide) on the leading rail; [`inspector`], [`mod@format`], a
//! per-document comments dock, [`mod@footnotes`] and [`versions`] on the trailing
//! rail; [`search_preview`] and [`mod@timeline`] on the bottom. See [`APP_DOCKS`]
//! for the authoritative list and mount order.
//!
//! ## Stable dock ids
//!
//! `DockLayoutState` (teksilo's serialisable dock layout) keys panes by raw
//! `DockWidgetId` (`u64`). For the per-work layout restore
//! ([`crate::view_models::WorkspaceLayoutViewModel`]) to match a saved layout to
//! this run's docks, each logical dock must carry the **same** id every launch —
//! a `DockWidgetId::fresh()` (a per-process atomic counter) would mint a
//! different id each run and `import_state` would drop the whole saved tree as
//! "unknown". So the four app docks use these fixed ids instead of `fresh()`.
//!
//! The base is deliberately high so it can never collide with a `fresh()` id
//! (the framework mints those from `1`, e.g. for a user-dragged dock split).

use std::sync::{LazyLock, RwLock};

use teksilo::widgets::{DockOpenLocation, DockSide, DockWidgetId};

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
/// The manuscript's footnotes (trailing rail, fourth tab).
pub const FOOTNOTES_DOCK_ID: u64 = DOCK_ID_BASE + 9;
/// This row's recorded past (trailing rail).
pub const VERSIONS_DOCK_ID: u64 = DOCK_ID_BASE + 10;
/// The whole project's past (bottom band, its own activity).
pub const TIMELINE_DOCK_ID: u64 = DOCK_ID_BASE + 11;

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

    /// Where [`DockingModel::open_dock`](teksilo::widgets::DockingModel::open_dock)
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
    AppDock {
        id: FOOTNOTES_DOCK_ID,
        side: DockSide::Trailing,
        own_tab: true,
    },
    AppDock {
        id: VERSIONS_DOCK_ID,
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
    // The project-wide timeline, in **its own** bottom activity rather than
    // stacked with the preview. A sole-pane dock *is* its activity, so it gets
    // the whole band; the two share only the side's height, which the writer can
    // drag and `WorkspaceLayoutService` persists. Sharing the band would leave a
    // slider, a sparkline and a change list in half of 180 dp.
    AppDock {
        id: TIMELINE_DOCK_ID,
        side: DockSide::Bottom,
        own_tab: true,
    },
];

/// The highest id this application will ever mint for a dock of its own.
///
/// Everything above it is reserved for extensions, which is what lets
/// [`register_dock`] reject a collision instead of discovering one as a saved desk
/// that silently mounts the wrong panel. Built-in ids start at [`DOCK_ID_BASE`] and
/// there are a dozen of them, so the ceiling is set far above any plausible growth
/// and still far below where an extension is asked to start.
pub const APP_DOCK_ID_CEILING: u64 = DOCK_ID_BASE + 0xFFFF;

/// Where an extension's own dock ids must begin.
///
/// An extension picks a base of its own above this and mints ids from it the same
/// way this module does — fixed, never `DockWidgetId::fresh()`, for exactly the
/// reason in the module docs: a per-process counter yields a different id every
/// launch and `import_state` drops the whole saved tree as unknown.
pub const EXTENSION_DOCK_ID_FLOOR: u64 = APP_DOCK_ID_CEILING + 1;

struct Registered {
    namespace: String,
    dock: AppDock,
}

static EXTENSION_DOCKS: LazyLock<RwLock<Vec<Registered>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));

/// Add an extension's dock to the roster.
///
/// Registering is all that is required: [`all_docks`] feeds the first-run mount,
/// and [`all_dock_ids`] feeds the `known_docks` set, so an extension dock reaches a
/// desk saved before the extension existed through the very mechanism that already
/// carries a newly added app dock there.
///
/// Returns `Err` when `dock.id` is below [`EXTENSION_DOCK_ID_FLOOR`] or already
/// taken. Both are refusals rather than warnings: a colliding id does not fail
/// visibly, it makes a saved layout mount one dock's geometry for another panel —
/// and it would do so only for writers who already had a desk saved, which is the
/// hardest kind of report to act on.
///
/// The returned handle unregisters on drop. Registering the same namespace twice
/// replaces the earlier entry rather than stacking a second copy.
pub fn register_dock(namespace: impl Into<String>, dock: AppDock) -> Result<DockHandle, String> {
    if dock.id < EXTENSION_DOCK_ID_FLOOR {
        return Err(format!(
            "dock id {:#x} is inside the application's reserved range; extension ids start at {:#x}",
            dock.id, EXTENSION_DOCK_ID_FLOOR
        ));
    }
    let namespace = namespace.into();
    let mut reg = EXTENSION_DOCKS.write().unwrap_or_else(|e| e.into_inner());
    if let Some(other) = reg
        .iter()
        .find(|r| r.dock.id == dock.id && r.namespace != namespace)
    {
        return Err(format!(
            "dock id {:#x} is already registered by '{}'",
            dock.id, other.namespace
        ));
    }
    reg.retain(|r| r.namespace != namespace);
    reg.push(Registered {
        namespace: namespace.clone(),
        dock,
    });
    Ok(DockHandle { namespace })
}

/// Unregisters its dock when dropped.
#[derive(Debug)]
pub struct DockHandle {
    namespace: String,
}

impl Drop for DockHandle {
    fn drop(&mut self) {
        let mut reg = EXTENSION_DOCKS.write().unwrap_or_else(|e| e.into_inner());
        reg.retain(|r| r.namespace != self.namespace);
    }
}

/// The full roster: this application's docks, then any an extension registered.
///
/// Every consumer reads the roster through here rather than through [`APP_DOCKS`],
/// so "adding a dock is the whole job" (see the table's own doc) stays true for an
/// extension's dock as well as for one of ours. Built-ins come first so their
/// first-run rail order is unaffected by what is installed.
pub fn all_docks() -> Vec<AppDock> {
    let reg = EXTENSION_DOCKS.read().unwrap_or_else(|e| e.into_inner());
    APP_DOCKS
        .iter()
        .map(|d| AppDock {
            id: d.id,
            side: d.side,
            own_tab: d.own_tab,
        })
        .chain(reg.iter().map(|r| AppDock {
            id: r.dock.id,
            side: r.dock.side,
            own_tab: r.dock.own_tab,
        }))
        .collect()
}

/// Every roster id, for the `known_docks` set persisted with a captured desk.
pub fn app_dock_ids() -> Vec<u64> {
    all_docks().iter().map(|d| d.id).collect()
}

pub mod comments;
pub mod create_split_button;
pub mod footnotes;
pub mod format;
pub mod inspector;
pub mod outline;
pub mod outline_card;
pub mod search;
pub mod search_preview;
pub mod search_replace_flow;
pub mod timeline;
pub mod trash;
pub mod versions;

#[cfg(test)]
mod extension_roster_tests {
    use super::*;

    // The registry is process-wide and tests run in parallel, so **no assertion
    // here may depend on `all_docks().len()`**: a sibling test's handle being
    // alive (or dropping) moves that number under this one's feet. Every check
    // below is scoped to ids this test owns. Offsets are unique per test for the
    // same reason.
    fn ext_dock(offset: u64) -> AppDock {
        AppDock {
            id: EXTENSION_DOCK_ID_FLOOR + offset,
            side: DockSide::Trailing,
            own_tab: true,
        }
    }

    fn has(id: u64) -> bool {
        all_docks().iter().any(|d| d.id == id)
    }

    /// A registered dock joins the roster **and** the `known_docks` set.
    ///
    /// Both matter, for different reasons: the roster drives the first-run mount,
    /// and `known_docks` is what lets a desk saved before the extension existed
    /// tell "this dock is new, mount it" from "the user closed it". Landing in one
    /// but not the other is the failure `APP_DOCKS`' own doc describes — registered
    /// but never mounted, invisible to every existing project.
    #[test]
    fn a_registered_dock_reaches_both_the_roster_and_the_known_set() {
        let id = EXTENSION_DOCK_ID_FLOOR + 1;
        assert!(!has(id), "id must be free before the test registers it");
        let _h = register_dock("test.roster", ext_dock(1)).expect("register");

        assert!(has(id), "the extension dock must be in the mount roster");
        assert!(
            app_dock_ids().contains(&id),
            "…and in the known_docks set, or restore will never mount it"
        );
    }

    /// Built-ins keep their first-run rail order regardless of what is installed.
    #[test]
    fn built_ins_come_before_anything_registered() {
        let _h = register_dock("test.order", ext_dock(2)).expect("register");
        let all = all_docks();
        assert_eq!(
            all[0].id, OUTLINE_DOCK_ID,
            "an extension must not displace the outline from the rail's head"
        );
        let pos = all
            .iter()
            .position(|d| d.id == EXTENSION_DOCK_ID_FLOOR + 2)
            .expect("registered dock present");
        assert!(
            pos >= APP_DOCKS.len(),
            "every built-in must precede every registered dock"
        );
    }

    /// An id inside the application's range is refused outright.
    ///
    /// A collision does not fail visibly: it makes a *saved* layout mount one
    /// dock's geometry under another panel, and only for writers who already had a
    /// desk captured. Refusing at registration turns the hardest possible bug
    /// report into an error at startup.
    #[test]
    fn an_id_in_the_apps_reserved_range_is_refused() {
        let err = register_dock(
            "test.collide",
            AppDock {
                id: OUTLINE_DOCK_ID,
                side: DockSide::Leading,
                own_tab: true,
            },
        )
        .expect_err("must refuse a built-in id");
        assert!(err.contains("reserved range"), "unhelpful message: {err}");
    }

    /// Two extensions cannot claim the same id.
    #[test]
    fn a_second_extension_cannot_take_a_taken_id() {
        let _first = register_dock("test.first", ext_dock(3)).expect("register");
        let err = register_dock("test.second", ext_dock(3)).expect_err("must refuse");
        assert!(
            err.contains("test.first"),
            "the error must name the holder, or the clash is unfixable: {err}"
        );
    }

    /// Re-registering one namespace replaces rather than stacks, and dropping the
    /// handle leaves nothing behind.
    #[test]
    fn re_registration_replaces_and_drop_unregisters() {
        let (old, new) = (EXTENSION_DOCK_ID_FLOOR + 4, EXTENSION_DOCK_ID_FLOOR + 5);
        {
            let _a = register_dock("test.same", ext_dock(4)).expect("first");
            assert!(has(old));
            let _b = register_dock("test.same", ext_dock(5)).expect("re-register");
            assert!(has(new), "the later registration must be live");
            assert!(!has(old), "…and the earlier one gone, not stacked beside it");
        }
        assert!(!has(new), "a dropped handle must leave no dock behind");
    }
}
