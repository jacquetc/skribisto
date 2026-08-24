// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Dock widgets placed on the app's `DockingLayout`. Each dock owns its own
//! content builder and packages it as a `DockWidget` for `App` to mount on a
//! side; `App` only wires the cross-view-model effects around them.
//!
//! The roster: [`crate::binder::dock`] (binder tree), [`crate::search::dock`], [`crate::trash::dock`],
//! [`crate::comments::dock`] (project-wide) on the leading rail; [`inspector`],
//! [`crate::format::dock`], a per-document comments dock, [`crate::footnotes::dock`] and
//! [`crate::versions::dock`] on the trailing rail; [`crate::search::preview_dock`] and
//! [`crate::timeline::dock`] on the bottom. See [`APP_DOCKS`] for the authoritative
//! list and mount order.
//!
//! ## Stable dock ids
//!
//! `DockLayoutState` (teksilo's serialisable dock layout) keys panes by raw
//! `DockWidgetId` (`u64`). For the per-work layout restore
//! ([`crate::workspace_layout::WorkspaceLayoutViewModel`]) to match a saved layout to
//! this run's docks, each logical dock must carry the **same** id every launch —
//! a `DockWidgetId::fresh()` (a per-process atomic counter) would mint a
//! different id each run and `import_state` would drop the whole saved tree as
//! "unknown". So the four app docks use these fixed ids instead of `fresh()`.
//!
//! The base is deliberately high so it can never collide with a `fresh()` id
//! (the framework mints those from `1`, e.g. for a user-dragged dock split).

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::prelude::{LocalizedString, Widget};
use teksilo::widgets::{DockOpenLocation, DockSide, DockWidget, DockWidgetId, IconWidget};

use frontend::AppContext;

use crate::app_ids::AppIds;

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
/// The writer's self-imposed drafting constraints (leading rail).
pub const GAMES_DOCK_ID: u64 = DOCK_ID_BASE + 12;

/// One app dock's declared home: its stable id plus where it mounts on a desk
/// nobody has arranged yet.
#[derive(Clone, Copy, Debug)]
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
///    ([`WorkspaceLayoutViewModel::restore`](crate::workspace_layout::WorkspaceLayoutViewModel::restore)),
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
    // The writing games, last on the leading rail: the one activity here that is
    // about the writer rather than about the manuscript.
    AppDock {
        id: GAMES_DOCK_ID,
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

/// What the app hands an extension so its dock can build a panel.
///
/// Deliberately the *ids-only* state and the backend handle, and nothing else.
/// The app's own docks are passed their view-models directly because they are
/// built in the same function that owns those view-models; an extension is not,
/// and handing it `EditorsViewModel` or `OutlineViewModel` would make every
/// internal refactor of those a breaking change for everything installed.
///
/// It is enough: `app_ctx` reaches the store and the event hub, and `ids` carries
/// the open `Work` as a `Signal`, so an extension builds its own reactive models
/// from the two exactly as [`crate::models`] does.
///
/// **Per window, not per process** — `ids` is Tier 2 (per open `Work`), so a
/// second window on a second project gets its own context. Anything cached off
/// this must be keyed accordingly; see [`crate::sessions::WorkSession`].
#[derive(Clone)]
pub struct DockContext {
    pub app_ctx: Rc<AppContext>,
    pub ids: AppIds,
    /// This Work's save state, narrowed to what an extension may touch: mark a
    /// change so it joins the same unsaved-changes guard a manuscript edit does.
    ///
    /// Without it a dock that edits its own state left the project reading clean,
    /// and Close/Quit proceeded with no save issued — see `WorkHandle` for the
    /// full account.
    pub work: crate::save::WorkHandle,
    /// What the writer is looking at in **this** window — see
    /// [`crate::active_context`]. Docks get it and tabs do not: a tab is already
    /// scoped to one container, a dock sits outside every tab.
    pub active: crate::active_context::ActiveContext,
    /// **The prose of a row the writer has open, as it stands right now.**
    ///
    /// The ordinary read commands answer with the *stored* text, and typing does not reach
    /// the store: an edit marks its document dirty, and the prose reaches `Content` on a
    /// flush, which is to say on a save. A dock reporting on the scene the writer is in and
    /// reading the store therefore shows them the text as of their last save, silently,
    /// while a margin lane two inches away is already following the same keystrokes.
    ///
    /// `None` when the row has no mounted prose editor: it is not open, or it is below the
    /// fold of a stream whose rows build lazily. A caller falls back to the stored text for
    /// that rather than treating it as an empty scene.
    ///
    /// A closure over **this window's** editors, for the same reason `active` is: a second
    /// window on the same Work has different rows open, and `ctx.app_state` cannot tell them
    /// apart. It also cannot be re-pointed after the builder runs, so the seeded
    /// `FormatViewModel` there is permanently detached and its registry permanently empty.
    ///
    /// ⚠ **Call it per build; never hold what it returns.** `RichTextEditor::construct`
    /// mints a fresh handle, so a cached [`LiveProse`] addresses an editor that may be gone,
    /// and its `version` stops moving with nothing to say that it has.
    pub live_prose: LiveProseFn,
}

/// Reads [`LiveProse`] for one row. See [`DockContext::live_prose`].
pub type LiveProseFn = Rc<dyn Fn(common::types::EntityId) -> Option<LiveProse>>;

/// A row's prose as the editor holds it, and the counter that moves as it is typed.
pub struct LiveProse {
    /// The row's prose, plain.
    pub text: String,
    /// Bumps on every document change. Bind it to rebuild as the writer types; without it a
    /// reader gets one snapshot and never hears about the next keystroke.
    pub version: teksilo::prelude::Signal<u64>,
}

/// An extension's dock: where it sits, what it is called, and what it draws.
///
/// [`AppDock`] carries only placement, because it has to be `const`-constructible
/// for [`APP_DOCKS`]. A registration additionally needs the content factory —
/// without it a registered dock would take a slot on the rail and open onto
/// nothing, which is exactly what this type exists to make impossible.
/// Resolves a slot's label per build, so a runtime locale switch reaches it.
///
/// Named because all three UI slots share the shape and clippy asks for it —
/// [`ExtensionDock::title`], [`crate::tabs::shared::segments::ContainerSegmentSpec::label`]
/// and [`crate::tabs::analysis::AnalysisCategorySpec::label`].
pub type LabelFn = Rc<dyn Fn() -> LocalizedString>;

/// Builds an extension dock's panel from the app handles it is given.
pub type DockBuildFn = Rc<dyn Fn(&DockContext) -> Box<dyn Widget>>;

/// Builds an extension dock's rail glyph.
pub type IconFn = Rc<dyn Fn() -> IconWidget>;

#[derive(Clone)]
pub struct ExtensionDock {
    /// Its id and where it mounts on a desk nobody has arranged yet.
    pub placement: AppDock,
    /// Resolved per build, so a runtime locale switch reaches the tab label —
    /// the same reason [`crate::tabs::shared::segments::ContainerSegmentSpec`]
    /// stores a closure rather than a `LocalizedString`.
    pub title: LabelFn,
    /// The rail glyph. A rail dock with no icon is reachable only by its tooltip,
    /// so this is worth supplying, but the framework does not require it.
    pub icon: Option<IconFn>,
    /// Builds the panel, once per placement (and again after a close/re-open).
    pub build: DockBuildFn,
}

struct Registered {
    namespace: String,
    dock: ExtensionDock,
}

// Thread-local rather than a `static RwLock`: an `ExtensionDock` holds `Rc`
// closures that build widgets, and `Rc` is not `Send`. Same shape as the
// container-segment and analysis-category registries. Every reader below runs on
// the UI thread — `project_shell` builds the layout, and
// `WorkspaceLayoutViewModel` reconciles a restored desk — so a per-thread
// registry is the same registry in every case that matters.
thread_local! {
    static EXTENSION_DOCKS: RefCell<Vec<Registered>> = const { RefCell::new(Vec::new()) };
}

/// Add an extension's dock to the roster.
///
/// Registering is all that is required: [`all_docks`] feeds the first-run mount,
/// [`app_dock_ids`] feeds the `known_docks` set, and [`registered_dock_widgets`]
/// feeds the content — so an extension dock reaches a desk saved before the
/// extension existed through the very mechanism that already carries a newly added
/// app dock there.
///
/// Returns `Err` when the id is below [`EXTENSION_DOCK_ID_FLOOR`] or already
/// taken. Both are refusals rather than warnings: a colliding id does not fail
/// visibly, it makes a saved layout mount one dock's geometry for another panel —
/// and it would do so only for writers who already had a desk saved, which is the
/// hardest kind of report to act on.
///
/// ⚠ Like every registry in the extension seam this is read as a **snapshot**, by
/// `project_shell` when it builds a window's docking layout. Register at startup,
/// before any project window exists.
///
/// The returned handle unregisters on drop. Registering the same namespace twice
/// replaces the earlier entry rather than stacking a second copy.
pub fn register_dock(
    namespace: impl Into<String>,
    dock: ExtensionDock,
) -> Result<DockHandle, String> {
    let id = dock.placement.id;
    if id < EXTENSION_DOCK_ID_FLOOR {
        return Err(format!(
            "dock id {id:#x} is inside the application's reserved range; extension ids start at \
             {EXTENSION_DOCK_ID_FLOOR:#x}"
        ));
    }
    let namespace = namespace.into();
    EXTENSION_DOCKS.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(other) = reg
            .iter()
            .find(|r| r.dock.placement.id == id && r.namespace != namespace)
        {
            return Err(format!(
                "dock id {id:#x} is already registered by '{}'",
                other.namespace
            ));
        }
        reg.retain(|r| r.namespace != namespace);
        reg.push(Registered {
            namespace: namespace.clone(),
            dock,
        });
        Ok(DockHandle {
            namespace: namespace.clone(),
        })
    })
}

/// Unregisters its dock when dropped.
#[derive(Debug)]
pub struct DockHandle {
    namespace: String,
}

impl Drop for DockHandle {
    fn drop(&mut self) {
        // `try_with`: a handle released during thread teardown must not panic.
        let _ = EXTENSION_DOCKS.try_with(|reg| {
            reg.borrow_mut().retain(|r| r.namespace != self.namespace);
        });
    }
}

/// The full roster: this application's docks, then any an extension registered.
///
/// Every consumer reads the roster through here rather than through [`APP_DOCKS`],
/// so "adding a dock is the whole job" (see the table's own doc) stays true for an
/// extension's dock as well as for one of ours. Built-ins come first so their
/// first-run rail order is unaffected by what is installed.
pub fn all_docks() -> Vec<AppDock> {
    let registered = EXTENSION_DOCKS.with(|reg| {
        reg.borrow()
            .iter()
            .map(|r| r.dock.placement.clone())
            .collect::<Vec<_>>()
    });
    APP_DOCKS.iter().cloned().chain(registered).collect()
}

/// Every roster id, for the `known_docks` set persisted with a captured desk.
pub fn app_dock_ids() -> Vec<u64> {
    all_docks().iter().map(|d| d.id).collect()
}

/// The registered docks as real [`DockWidget`]s, for `project_shell` to chain onto
/// its `DockingLayout` beside the app's own.
///
/// Separate from [`all_docks`] because the two answer different questions and have
/// different callers: the roster is *placement* and is also read by the restore-time
/// reconcile, which has no `DockContext` and needs none. This is *content*, and
/// exists only where a layout is being built.
pub fn registered_dock_widgets(cx: &DockContext) -> Vec<DockWidget> {
    EXTENSION_DOCKS.with(|reg| {
        reg.borrow()
            .iter()
            .map(|r| {
                let dock = r.dock.clone();
                let cx = cx.clone();
                let build = dock.build.clone();
                let widget =
                    DockWidget::new(dock.placement.widget_id(), (dock.title)(), move |_id| {
                        crate::tabs::Boxed::new(build(&cx))
                    })
                    .show_header(true)
                    .default_location(dock.placement.location());
                match &dock.icon {
                    Some(icon) => {
                        let icon = icon.clone();
                        widget.icon(move || icon())
                    }
                    None => widget,
                }
            })
            .collect()
    })
}

pub mod create_split_button;
pub mod inspector;
pub mod inspector_sections;
pub mod outline_card;

#[cfg(test)]
mod extension_roster_tests {
    use super::*;

    // The registry is process-wide and tests run in parallel, so **no assertion
    // here may depend on `all_docks().len()`**: a sibling test's handle being
    // alive (or dropping) moves that number under this one's feet. Every check
    // below is scoped to ids this test owns. Offsets are unique per test for the
    // same reason.
    fn placement(offset: u64) -> AppDock {
        AppDock {
            id: EXTENSION_DOCK_ID_FLOOR + offset,
            side: DockSide::Trailing,
            own_tab: true,
        }
    }

    fn ext_dock(offset: u64) -> ExtensionDock {
        ExtensionDock {
            placement: placement(offset),
            title: Rc::new(|| teksilo::prelude::lit!("Demo".to_string())),
            icon: None,
            build: Rc::new(|_| {
                Box::new(teksilo::widgets::TextWidget::new(teksilo::prelude::lit!(
                    "body".to_string()
                )))
            }),
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
        let mut clash = ext_dock(0);
        clash.placement = AppDock {
            id: OUTLINE_DOCK_ID,
            side: DockSide::Leading,
            own_tab: true,
        };
        let err = register_dock("test.collide", clash).expect_err("must refuse a built-in id");
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

    /// **The gap this type closed.** Registration used to carry placement alone,
    /// so a registered dock took a rail slot, was mounted by the first-run walk
    /// over [`all_docks`] — and opened onto nothing, because dock *content* is a
    /// separate hand-chained list in `project_shell` that a registration had no
    /// way into. It failed silently, as an empty panel.
    ///
    /// So this asserts the whole path: a registration yields a real `DockWidget`
    /// carrying the extension's own id, and the widget it builds lays out.
    #[test]
    fn a_registered_dock_yields_a_dock_widget_that_actually_draws() {
        use teksilo::core::widget_tree::WidgetTree;
        use teksilo::prelude::SizeProposal;
        use teksilo::widgets::{DockingLayout, DockingModel, Spacer};

        let id = EXTENSION_DOCK_ID_FLOOR + 6;
        let _h = register_dock("test.content", ext_dock(6)).expect("register");

        let app_ctx = Rc::new(AppContext::new());
        let cx = DockContext {
            app_ctx: app_ctx.clone(),
            ids: AppIds::new(),
            work: crate::save::WorkHandle::detached(app_ctx, AppIds::new()),
            active: crate::active_context::ActiveContext::detached(),
            // Nothing mounted in a test tree, so no row has live prose.
            live_prose: Rc::new(|_| None),
        };
        let widgets = registered_dock_widgets(&cx);
        assert_eq!(
            widgets.len(),
            1,
            "the registration must produce exactly one DockWidget"
        );

        // Handed to a `DockingLayout` exactly as `project_shell` hands it one.
        // `dock()` registers with the model immediately, so the model itself is
        // the witness that the widget carries the id the roster mounts — the one
        // linkage that, broken, mounts a pane the layout was never given.
        let model = DockingModel::new();
        let mut layout = DockingLayout::new(model.clone()).center(Spacer::new());
        for w in widgets {
            layout = layout.dock(w);
        }
        assert!(
            model.is_registered(DockWidgetId::from_raw(id)),
            "the DockWidget must carry the extension's own id"
        );
        assert!(
            all_docks().iter().any(|d| d.id == id),
            "…and the roster must mount that same id"
        );

        // And the content is real, not an empty box: the failure being guarded is
        // a dock that mounts and draws nothing.
        let mut tree = WidgetTree::new();
        let wid = tree.add_boxed((ext_dock(6).build)(&cx));
        tree.layout(SizeProposal::exact(300.0, 400.0));
        assert!(
            tree.bounds(wid).width > 0.0,
            "the registered dock's panel laid out to zero width"
        );
    }

    /// **The second gap this context closed.** A dock could draw, but an edit it
    /// made was invisible to the unsaved-changes guard: `dirty_seq` is bumped
    /// only by editor typing and a whitelist of named entity events, and an
    /// extension's state is in neither list. Close/Quit read `unsaved == false`
    /// and proceeded with no save issued.
    ///
    /// So this asserts the path a dock actually walks: build the panel from a
    /// `DockContext` made out of a real `SaveStateViewModel`, mark a change
    /// through it, and see the *view-model the app's guard reads* go dirty.
    #[test]
    fn a_dock_that_marks_a_change_reaches_the_unsaved_guard() {
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let save_state = crate::save::SaveStateViewModel::new(app_ctx.clone(), ids.clone());
        assert!(!save_state.is_unsaved(), "a fresh Work starts clean");

        let cx = DockContext {
            app_ctx,
            ids,
            work: save_state.handle(),
            active: crate::active_context::ActiveContext::detached(),
            // Nothing mounted in a test tree, so no row has live prose.
            live_prose: Rc::new(|_| None),
        };

        // Exactly what a dock's build closure does with its context.
        cx.work.mark_changed(true);

        assert!(
            save_state.is_unsaved(),
            "an extension's edit must gate Close/Quit like any manuscript edit"
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
            assert!(
                !has(old),
                "…and the earlier one gone, not stacked beside it"
            );
        }
        assert!(!has(new), "a dropped handle must leave no dock behind");
    }
}
