// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `WorkspaceLayoutViewModel` — captures and restores a project's **desk**: the
//! open editor tabs (both panes, their selection, the split) and the dock layout,
//! persisted per work via [`WorkspaceLayoutService`].
//!
//! ## When it runs
//!
//! * **Restore** — on `LoadWork` / `NewWork`, after the id-seeding subscriber has
//!   pointed the singles at the new project and `EditorsViewModel::close_all` has
//!   cleared the outgoing tabs. Reads this project's saved layout (by
//!   `Work.unique_id`) and re-opens the tabs + imports the dock arrangement; a
//!   project with no saved layout gets the default docks and an empty desk.
//! * **Capture** — at the two UI "doors" that leave a project while its store is
//!   still alive: the exit guard (Ctrl+W / Ctrl+Q / window close) and
//!   `ProjectSwitchViewModel` (New / Open / switcher / import-toast). It must run
//!   *there*, not in a `CloseWork` subscriber: `close_work` tears the Work subtree
//!   out of the store **before** publishing `CloseWork`, and the in-place switches
//!   fire no `CloseWork` at all — so by the time any teardown event lands there is
//!   no store left to translate a tab's item id into its persistable ordinal.
//!
//! ## Why uids
//!
//! A tab is persisted by its `BinderItem`'s durable **uid** — not its store id
//! (remapped on every load, so unstable across a save→load cycle) and not its
//! stream position either (an insertion ahead of an open tab would shift every
//! later ordinal, reopening a neighbour). See
//! [`workspace_layout_file`](crate::models::WorkspaceLayoutService)'s module docs.
//!
//! ## The bottom band
//!
//! The bottom dock is the transient search-preview band. Per the product decision
//! it is **always hidden at start** and its visibility is never persisted — so
//! every restore (saved or default) ends by forcing it hidden.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use bastyde::prelude::Signal;
use bastyde::widgets::{DockLayoutState, DockSide, DockingModel};

use frontend::AppContext;

use uuid::Uuid;

use crate::app_ids::AppIds;
use crate::models::{
    BinderItemRef, PaneLayout, PerProjectLayout, TabViewState, WorkspaceLayoutService,
    ordered_binder_items, uid_is_usable,
};
use crate::singles::{SingleWork, SingleWorkInfo};
use crate::view_models::{EditorsViewModel, OutlineViewModel, Side, TreeExpansionViewModel};

/// Per-work desk persistence (editor tabs + dock layout). Cloneable handle —
/// registered as `app_state` so the close/switch doors and `App::build` reach the
/// one instance.
#[derive(Clone)]
pub struct WorkspaceLayoutViewModel {
    app_ctx: Rc<AppContext>,
    service: WorkspaceLayoutService,
    docking: DockingModel,
    single_work: SingleWork,
    single_work_info: SingleWorkInfo,
    ids: AppIds,
    /// `true` while a *backup file* is open here — capture is inert then (like disk
    /// saves), so a backup-viewing session never overwrites the real project's
    /// saved desk (both share the project's `unique_id`).
    backup_mode: Signal<bool>,
    /// Injected once `App::build` has created the editors (they need
    /// `ctx.settings()`); `None` until then.
    editors: Rc<RefCell<Option<EditorsViewModel>>>,
    /// Injected once `App::build` has the window's own `OutlineViewModel` at hand
    /// (mirrors `editors`'s own injection — see [`Self::set_outline`]). Used by
    /// [`Self::capture_tree_expansion`] to persist the outline's own chevron state
    /// alongside every open container tab's Overview.
    outline: Rc<RefCell<Option<OutlineViewModel>>>,
    /// This Work's own tree-expansion service (Tier 2 — see its own module doc),
    /// bundled in here so [`Self::capture_tree_expansion`] can be reached from
    /// every "leaving the project" door with a single `WorkspaceLayoutViewModel`
    /// argument, exactly like [`Self::capture`] already is — never resolved via
    /// `ctx.app_state::<TreeExpansionViewModel>()`, which would silently answer
    /// with whichever Work's session registered first (see
    /// `capture_tree_expansion`'s own doc).
    tree_expansion: TreeExpansionViewModel,
    /// The pristine dock arrangement, snapshotted once on first build — the state a
    /// project with no saved layout is reset to (so switching to an unconfigured
    /// project doesn't inherit the previous one's docks).
    default_docks: Rc<RefCell<Option<DockLayoutState>>>,
}

impl WorkspaceLayoutViewModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        service: WorkspaceLayoutService,
        docking: DockingModel,
        single_work: SingleWork,
        single_work_info: SingleWorkInfo,
        ids: AppIds,
        backup_mode: Signal<bool>,
        tree_expansion: TreeExpansionViewModel,
    ) -> Self {
        Self {
            app_ctx,
            service,
            docking,
            single_work,
            single_work_info,
            ids,
            backup_mode,
            editors: Rc::new(RefCell::new(None)),
            outline: Rc::new(RefCell::new(None)),
            tree_expansion,
            default_docks: Rc::new(RefCell::new(None)),
        }
    }

    /// Hand the editors view-model over, once `App::build` has created it.
    pub fn set_editors(&self, editors: EditorsViewModel) {
        *self.editors.borrow_mut() = Some(editors);
    }

    /// Hand this window's own `OutlineViewModel` over — mirrors [`Self::set_editors`].
    /// Called once from `App::build`, alongside it.
    pub fn set_outline(&self, outline: OutlineViewModel) {
        *self.outline.borrow_mut() = Some(outline);
    }

    /// Record the pristine dock arrangement (first build only) — the reset target
    /// for a project with no saved layout.
    pub fn set_default_docks(&self, state: DockLayoutState) {
        let mut slot = self.default_docks.borrow_mut();
        if slot.is_none() {
            *slot = Some(state);
        }
    }

    // ── Capture ───────────────────────────────────────────────────────────────

    /// Persist the current desk for the open project. A no-op with no usable uid
    /// (a brand-new unsaved project), in backup mode, before the editors are wired,
    /// or while there are **unsaved edits**.
    ///
    /// Tabs are persisted by durable uid, but a capture is only meaningful when
    /// the store matches disk: a tab left open on an item created since the last
    /// save would persist a uid the next load's file doesn't have yet. So
    /// capture runs only when clean; the on-save trigger then keeps the
    /// persisted desk fresh and disk-aligned after every write (surviving a
    /// later hard exit), and a clean graceful close captures the final tab set.
    pub fn capture(&self) {
        if self.backup_mode.get() {
            return;
        }
        let uid = self.single_work.unique_id().get();
        if !uid_is_usable(&uid) {
            return;
        }
        let Some(editors) = self.editors.borrow().clone() else {
            return;
        };
        if editors.is_unsaved() {
            return;
        }

        // item id → durable uid. Persisting the *uid* means a restored tab reopens the
        // item the writer actually had open, whatever moved in the binder meanwhile.
        let order = self.ordered_items();
        let uid_of: HashMap<u64, Uuid> = order.iter().map(|r| (r.id, r.uid)).collect();
        let pane = |side| PaneLayout {
            tabs: to_uids(&uid_of, &editors.tab_item_ids(side)),
            selected: editors
                .selected_item(side)
                .and_then(|id| uid_of.get(&id).copied()),
            // Where the writer was in each tab. Captured from the live editors,
            // falling back to whatever a never-visited tab was seeded with, so a
            // save right after a restore does not erase the positions it just
            // restored (see `ViewStatePorts::capture`).
            view_states: editors
                .tab_item_ids(side)
                .into_iter()
                .filter_map(|id| {
                    let uid = uid_of.get(&id).copied()?;
                    let s = editors.view_state_of(id)?;
                    Some(TabViewState {
                        uid,
                        caret: s.caret,
                        scroll: s.scroll,
                    })
                })
                .collect(),
        };
        let split_active = editors.split_active().get();

        let record = PerProjectLayout {
            work_uid: uid,
            last_path: self.single_work_info.file_name().get().unwrap_or_default(),
            primary: pane(Side::Primary),
            secondary: pane(Side::Secondary),
            focus_secondary: editors.focused_side() == Side::Secondary,
            editor_splitter: split_active.then(|| editors.splitter().export_state()),
            docks: Some(self.docking.export_state()),
            // Stamp the desk with the roster *this* build knows, so a later build
            // can tell a dock it added from one the writer closed. Deliberately the
            // whole roster, not the currently-open subset: a closed dock is exactly
            // what has to stay distinguishable from a not-yet-invented one.
            known_docks: crate::docks::app_dock_ids(),
        };
        if let Err(e) = self.service.set(record) {
            eprintln!("skribisto: workspace layout capture failed: {e}");
        }
    }

    /// Persist every open container tab's Overview expand state, plus the outline's
    /// own, in one batched write. Shares this Work's "leaving the project" doors
    /// with [`Self::capture`] rather than having its own — both need the store
    /// alive to read, and a second set of call sites would be a second set of
    /// places to forget (see `app::capture_workspace_layout`, which calls both
    /// together). A no-op before `App::build` has injected `editors`/`outline`
    /// (see [`Self::set_editors`]/[`Self::set_outline`]) — the same guard
    /// [`Self::capture`] already applies to `editors` alone.
    ///
    /// **Never resolved via `ctx.app_state::<T>()`.** `TreeExpansionViewModel`/
    /// `EditorsViewModel`/`OutlineViewModel` are all Tier 2 (per-open-Work), so
    /// that lookup would silently answer with whichever Work's session
    /// registered first. This method reaches the right Work's data through the
    /// injected `editors`/`outline` cells instead, exactly like `capture` does.
    pub fn capture_tree_expansion(&self) {
        let Some(editors) = self.editors.borrow().clone() else {
            return;
        };
        let Some(outline) = self.outline.borrow().clone() else {
            return;
        };
        // The outline is one tree over the whole project, so it is captured on its
        // own rather than per container.
        self.tree_expansion
            .capture_outline(&outline.model().expanded_keys());
        let mut folders: Vec<(Uuid, Vec<Uuid>)> = Vec::new();
        for side in [Side::Primary, Side::Secondary] {
            let tabs = editors.tabs(side);
            for i in 0..tabs.len() {
                let snapshot = tabs.with_item(i, |h| {
                    h.payload
                        .downcast_ref::<crate::tabs::ContentTab>()
                        .and_then(|t| t.overview())
                        .and_then(|o| o.expansion_snapshot())
                });
                // The same container can be open in both panes; its two Overviews
                // share a container uid, so keep the first and let the write stay
                // idempotent.
                if let Some(Some((container, expanded))) = snapshot
                    && !folders.iter().any(|(c, _)| *c == container)
                {
                    folders.push((container, expanded));
                }
            }
        }
        self.tree_expansion.capture(&folders);
    }

    // ── Restore ───────────────────────────────────────────────────────────────

    /// Apply the open project's saved desk (or the defaults). Call after the
    /// id-seeding subscriber has run for this `LoadWork` / `NewWork`.
    ///
    /// `is_backup` — the just-loaded file is a *backup*. A backup shares its source
    /// project's `unique_id` (retention correlates on it), so its saved layout is
    /// the *source's*; importing it would show the live project's docks + tabs
    /// (resolved against the backup's own, possibly older, item stream) behind the
    /// backup-choice modal. A backup gets a clean default desk instead — symmetric
    /// with `capture` being inert in backup mode. The flag is supplied by the caller
    /// (the backup-detection subscriber, which has already sniffed the manifest), so
    /// restore doesn't sniff it a second time.
    pub fn restore(&self, is_backup: bool) {
        let uid = self.single_work.unique_id().get();
        let saved = if !is_backup && uid_is_usable(&uid) {
            self.service.get(&uid)
        } else {
            None
        };

        // Docks first: this project's saved arrangement, or reset to the default so
        // an unconfigured project (or a backup view) never inherits the previous
        // one's docks.
        match saved
            .as_ref()
            .and_then(|r| r.docks.as_ref().map(|d| (d, &r.known_docks)))
        {
            Some((docks, known)) => {
                self.docking.import_state(docks);
                self.mount_docks_added_since(known);
            }
            None => self.apply_default_docks(),
        }
        // Always hidden at start — the bottom band is the transient search-preview;
        // its visibility is never persisted (product decision).
        self.docking
            .set_side_visible_immediate(DockSide::Bottom, false);

        // Tabs.
        let Some(editors) = self.editors.borrow().clone() else {
            return;
        };
        let Some(rec) = saved else {
            return; // no saved layout: default docks (above) + the empty desk close_all left
        };

        // uid → (item id, title) in the freshly-loaded project. Resolve BOTH panes up
        // front: a saved uid can still be stale (its item was trashed or deleted between
        // capture and this load), and the split / focus decisions below must key off what
        // actually resolved, not the persisted list length, or an all-stale side pane
        // would show up empty.
        let order = self.ordered_items();
        let primary_tabs = resolve_uids(&order, &rec.primary.tabs);
        let secondary_tabs = resolve_uids(&order, &rec.secondary.tabs);

        for (id, title) in &primary_tabs {
            editors.open_in(Side::Primary, *id, title);
        }
        let (split, focus) = split_and_focus(secondary_tabs.len(), rec.focus_secondary);
        editors.set_split(split); // never an empty split (side collapses if nothing resolved)
        if split {
            for (id, title) in &secondary_tabs {
                editors.open_in(Side::Secondary, *id, title);
            }
            // The two-pane splitter ratio (only meaningful while split).
            if let Some(sp) = &rec.editor_splitter {
                editors.splitter().import_state(sp);
            }
        }

        // Put each tab's caret and page scroll back. `open_in` only pushes a
        // `TabHandle` — the pane widget has not built yet — so this *seeds* the
        // position, to be read once when it does. Doing it after both `open_in`
        // loops rather than inside them keeps the split decision above the only
        // thing that decides whether the secondary side exists at all.
        let seed = |side: Side, tabs: &[(u64, String)], states: &[TabViewState]| {
            for (id, _) in tabs {
                if let Some(uid) = order.iter().find(|r| r.id == *id).map(|r| r.uid)
                    && let Some(s) = states.iter().find(|s| s.uid == uid)
                {
                    editors.seed_view_state(
                        side,
                        *id,
                        crate::view_models::ViewState {
                            caret: s.caret,
                            scroll: s.scroll,
                        },
                    );
                }
            }
        };
        seed(Side::Primary, &primary_tabs, &rec.primary.view_states);
        if split {
            seed(Side::Secondary, &secondary_tabs, &rec.secondary.view_states);
        }

        // Re-select the tabs that were selected (open_in leaves the last-opened one
        // selected; override to the persisted choice).
        let resolve_one = |uid: Uuid| order.iter().find(|r| r.uid == uid).map(|r| r.id);
        if let Some(uid) = rec.primary.selected
            && let Some(id) = resolve_one(uid)
        {
            editors.select_item(Side::Primary, id);
        }
        if split
            && let Some(uid) = rec.secondary.selected
            && let Some(id) = resolve_one(uid)
        {
            editors.select_item(Side::Secondary, id);
        }

        // Re-mark the focused pane (drives the binder's open-item accent).
        editors.set_focused(focus);
    }

    /// Reset the docks to the pristine default arrangement (first-build snapshot).
    fn apply_default_docks(&self) {
        if let Some(def) = self.default_docks.borrow().clone() {
            self.docking.import_state(&def);
        }
    }

    /// Mount every roster dock the desk we just imported had never heard of.
    ///
    /// A saved `DockLayoutState` is a **closed list**: `import_state` rebuilds the
    /// rail purely from the snapshot, so a dock that is registered but unmentioned
    /// is silently never mounted. That is the whole bug — the two comments docks
    /// disappeared from every project captured before they shipped, exactly as the
    /// Format dock had. `known_docks` is what makes "unmentioned" readable: a dock
    /// the saved desk *listed* and still omitted was closed by the user and stays
    /// closed; one it never listed did not exist yet, and belongs on the rail.
    fn mount_docks_added_since(&self, known: &[u64]) {
        let unknown = unknown_dock_ids(&crate::docks::app_dock_ids(), known);
        if unknown.is_empty() {
            return;
        }
        // `open_dock` reveals the target side and, for an own-tab placement, selects
        // the tab it just created — both would overrule the desk that was just
        // imported, so a project left showing the binder would come back showing
        // Comments, on a side the writer had collapsed. Snapshot the two, mount, put
        // them back. Tab *indices* survive this: an own-tab open appends, and a
        // stacked one only adds a pane to an existing tab.
        let before: Vec<(DockSide, bool, usize)> = DOCK_SIDES
            .iter()
            .map(|&s| {
                (
                    s,
                    self.docking.is_side_visible(s),
                    self.docking.side_selected_tab(s),
                )
            })
            .collect();

        for dock in crate::docks::APP_DOCKS
            .iter()
            .filter(|d| unknown.contains(&d.id))
        {
            self.docking.open_dock(dock.widget_id(), dock.location());
        }

        for (side, visible, selected) in before {
            self.docking.select_tab(side, selected);
            self.docking.set_side_visible_immediate(side, visible);
        }
    }

    // ── Backend enumeration ─────────────────────────────────────────────────────

    /// The open project's binder items in the flat, binder-major stream order (through
    /// Layer A). Empty when no project is open.
    fn ordered_items(&self) -> Vec<BinderItemRef> {
        match self.ids.work_id.get() {
            Some(work_id) => ordered_binder_items(&self.app_ctx, work_id),
            None => Vec::new(),
        }
    }
}

// ── Pure uid translation (headless-testable) ────────────────────────────────────
//
// This used to persist **ordinals** — an item's position in the flat stream — because
// nothing about a `BinderItem` was durable enough to key by. That made a restore
// positional: anything inserted, removed or moved in the binder between capture and the
// next open shifted every later ordinal, so the desk reopened a *neighbour* of each tab
// the writer had left open, silently and with no way to tell. `BinderItem.uid` (`.skrib`
// v3) is stable across a save → load, so a restore now reopens the item itself.

/// Every dock side, for the snapshot-mount-restore in [`WorkspaceLayoutViewModel::mount_docks_added_since`].
const DOCK_SIDES: [DockSide; 4] = [
    DockSide::Leading,
    DockSide::Trailing,
    DockSide::Top,
    DockSide::Bottom,
];

/// Roster ids that `known` does not list — the docks a saved desk had never heard
/// of, in roster order.
///
/// The subtraction is the whole mechanism, so it is a free function rather than a
/// closure inside the mount: "absent because it did not exist yet" (mount it) versus
/// "absent because the user closed it" (leave it closed) is the distinction the
/// `known_docks` field exists to make, and it is worth testing on its own.
fn unknown_dock_ids(roster: &[u64], known: &[u64]) -> Vec<u64> {
    roster
        .iter()
        .copied()
        .filter(|id| !known.contains(id))
        .collect()
}

/// Item ids → their durable uids, dropping any not present (a tab whose item left the
/// binder since capture) and any still nil-identified (a pre-v3 item not yet healed).
fn to_uids(uid_of: &HashMap<u64, Uuid>, ids: &[u64]) -> Vec<Uuid> {
    ids.iter()
        .filter_map(|id| uid_of.get(id).copied())
        .filter(|uid| !uid.is_nil())
        .collect()
}

/// Uids → their `(id, title)` in the stream, dropping any that no longer resolve (the
/// item was trashed or deleted since capture).
fn resolve_uids(order: &[BinderItemRef], uids: &[Uuid]) -> Vec<(u64, String)> {
    uids.iter()
        .filter_map(|uid| {
            order
                .iter()
                .find(|r| r.uid == *uid)
                .map(|r| (r.id, r.title.clone()))
        })
        .collect()
}

/// Decide the side pane's visibility + the focused pane from what actually
/// resolved — never a shown-but-empty side pane, and focus the side only when it
/// has tabs. Keyed on the resolved count, not the persisted list length.
fn split_and_focus(secondary_count: usize, want_focus_secondary: bool) -> (bool, Side) {
    let split = secondary_count > 0;
    let focus = if want_focus_secondary && split {
        Side::Secondary
    } else {
        Side::Primary
    };
    (split, focus)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::docks::{COMMENTS_DOCK_ID, DOC_COMMENTS_DOCK_ID, OUTLINE_DOCK_ID, TRASH_DOCK_ID};
    use bastyde::prelude::*;
    use bastyde::widgets::{DockWidget, DockWidgetId, DockingLayout, RectWidget};

    /// The roster a build that could write a `workspace.toml` **v4** knew — the six
    /// docks that predate comments. The same list the v4 → v5 migration stamps.
    const V4_ROSTER: [u64; 6] = [
        0xD0C_0001, 0xD0C_0002, 0xD0C_0003, 0xD0C_0004, 0xD0C_0005, 0xD0C_0006,
    ];

    fn u(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    /// A `DockingModel` with every roster dock **registered** (but none mounted).
    /// `DockingLayout::dock` registers eagerly at builder time, so the builder can
    /// be dropped straight away — no widget tree needed.
    fn registered_model() -> DockingModel {
        let model = DockingModel::new();
        let mut layout = DockingLayout::new(model.clone());
        for dock in crate::docks::APP_DOCKS {
            layout = layout.dock(DockWidget::new(dock.widget_id(), lit!("Dock"), |_| {
                RectWidget::new()
            }));
        }
        drop(layout);
        model
    }

    fn vm_with(docking: DockingModel) -> WorkspaceLayoutViewModel {
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        WorkspaceLayoutViewModel::new(
            app_ctx.clone(),
            crate::models::WorkspaceLayoutService::in_memory_default(),
            docking,
            SingleWork::new(app_ctx.clone()),
            SingleWorkInfo::new(app_ctx.clone()),
            ids.clone(),
            Signal::new(false),
            TreeExpansionViewModel::new(
                app_ctx,
                ids,
                crate::models::TreeExpansionService::in_memory_default(),
            ),
        )
    }

    /// Mount only the docks a v4-era build knew, in roster order — i.e. reproduce
    /// the desk a pre-comments Skribisto would have captured.
    fn desk_as_of_v4(model: &DockingModel) {
        for dock in crate::docks::APP_DOCKS
            .iter()
            .filter(|d| V4_ROSTER.contains(&d.id))
        {
            model.open_dock(dock.widget_id(), dock.location());
        }
    }

    fn is_open(model: &DockingModel, id: u64) -> bool {
        model.dock_location(DockWidgetId::from_raw(id)).is_some()
    }

    /// **The bug, pinned.** A desk captured before the comments docks existed
    /// restores without them: `import_state` rebuilds the rail purely from the
    /// snapshot, so a registered-but-unmentioned dock is silently dropped. This is
    /// what the user saw opening Starforgers — and it fails *before* the reconcile
    /// exists, which is what makes the next test meaningful.
    #[test]
    fn a_pre_comments_desk_restores_without_the_comments_docks() {
        let model = registered_model();
        desk_as_of_v4(&model);
        let v4_snapshot = model.export_state();

        // A fresh process, this build: every dock registered, then the old desk imported.
        let fresh = registered_model();
        fresh.import_state(&v4_snapshot);

        assert!(is_open(&fresh, OUTLINE_DOCK_ID), "the old docks come back");
        assert!(
            !is_open(&fresh, COMMENTS_DOCK_ID) && !is_open(&fresh, DOC_COMMENTS_DOCK_ID),
            "…but a dock the snapshot never mentions is dropped, however registered it is"
        );
    }

    /// The fix: the reconcile mounts what the saved desk had never heard of, on the
    /// side the roster declares — **without** disturbing what the import restored.
    #[test]
    fn the_reconcile_mounts_docks_added_since_the_desk_was_captured() {
        let model = registered_model();
        desk_as_of_v4(&model);
        // A desk left mid-arrangement: the binder selected (not the last leading
        // tab), and the trailing side collapsed.
        model.select_tab(DockSide::Leading, 0);
        model.set_side_visible_immediate(DockSide::Trailing, false);
        let v4_snapshot = model.export_state();

        let fresh = registered_model();
        let vm = vm_with(fresh.clone());
        fresh.import_state(&v4_snapshot);
        vm.mount_docks_added_since(&V4_ROSTER);

        assert!(
            is_open(&fresh, COMMENTS_DOCK_ID) && is_open(&fresh, DOC_COMMENTS_DOCK_ID),
            "both comments docks are back on the rail"
        );
        assert_eq!(
            fresh
                .dock_location(DockWidgetId::from_raw(COMMENTS_DOCK_ID))
                .map(|l| l.side),
            Some(DockSide::Leading),
            "project-wide comments joins the leading rail"
        );
        assert_eq!(
            fresh
                .dock_location(DockWidgetId::from_raw(DOC_COMMENTS_DOCK_ID))
                .map(|l| l.side),
            Some(DockSide::Trailing),
            "this document's comments joins the trailing rail"
        );
        // …and the desk the writer left is untouched. `open_dock` selects the tab it
        // creates and reveals the side, so without the snapshot-restore in
        // `mount_docks_added_since` this project would reopen showing Comments on a
        // side the writer had collapsed.
        assert_eq!(
            fresh.side_selected_tab(DockSide::Leading),
            0,
            "the binder is still the selected leading activity"
        );
        assert!(
            !fresh.is_side_visible(DockSide::Trailing),
            "a collapsed side stays collapsed"
        );
    }

    /// The distinction the whole `known_docks` field exists to make: a dock the
    /// writer **closed** is also absent from the snapshot, and must stay closed. Only
    /// a dock the saved desk never listed is treated as new.
    #[test]
    fn a_dock_the_writer_closed_is_not_re_mounted() {
        let model = registered_model();
        desk_as_of_v4(&model);
        model.close_dock(DockWidgetId::from_raw(TRASH_DOCK_ID));
        // This build knows the whole roster, so *that* is what a capture stamps —
        // trash included, even though it is closed.
        let known = crate::docks::app_dock_ids();
        let snapshot = model.export_state();

        let fresh = registered_model();
        let vm = vm_with(fresh.clone());
        fresh.import_state(&snapshot);
        vm.mount_docks_added_since(&known);

        assert!(
            !is_open(&fresh, TRASH_DOCK_ID),
            "closed by the writer, and listed as known — it stays closed"
        );
    }

    /// A desk captured by *this* build needs no reconcile at all — the common path,
    /// and it must not churn the layout.
    #[test]
    fn a_current_desk_is_left_exactly_alone() {
        let roster = crate::docks::app_dock_ids();
        assert!(
            unknown_dock_ids(&roster, &roster).is_empty(),
            "nothing to mount when the saved desk knew every dock"
        );
    }

    /// The subtraction itself, including the case that actually shipped: today's
    /// roster minus a v4 stamp is exactly the two comments docks. A dock added to
    /// `project_shell` but not to `APP_DOCKS` would leave this list short — which is
    /// the failure mode the roster's own doc warns about.
    #[test]
    fn a_v4_stamp_leaves_exactly_the_two_comments_docks_unknown() {
        assert_eq!(
            unknown_dock_ids(&crate::docks::app_dock_ids(), &V4_ROSTER),
            vec![COMMENTS_DOCK_ID, DOC_COMMENTS_DOCK_ID],
            "these two, and only these two, postdate a v4 desk"
        );
        // Order follows the roster, not the known set.
        assert_eq!(unknown_dock_ids(&[3, 1, 2], &[2]), vec![3, 1]);
        assert!(unknown_dock_ids(&[], &[1, 2]).is_empty());
    }

    fn stream() -> Vec<BinderItemRef> {
        vec![
            BinderItemRef {
                id: 10,
                uid: u(10),
                title: "A".into(),
            },
            BinderItemRef {
                id: 20,
                uid: u(20),
                title: "B".into(),
            },
            BinderItemRef {
                id: 30,
                uid: u(30),
                title: "C".into(),
            },
            BinderItemRef {
                id: 40,
                uid: u(40),
                title: "D".into(),
            },
        ]
    }

    fn uid_map(order: &[BinderItemRef]) -> HashMap<u64, Uuid> {
        order.iter().map(|r| (r.id, r.uid)).collect()
    }

    /// Scope C regression guard: `capture_tree_expansion` must never panic when
    /// called before `App::build` has injected `editors`/`outline` (every "leaving
    /// the project" door calls `capture_workspace_layout`, which calls both
    /// `capture()` and `capture_tree_expansion()` unconditionally) — it must stay
    /// a safe, silent no-op, exactly as `capture()` already is before `editors` is
    /// injected.
    #[test]
    fn capture_tree_expansion_is_a_safe_no_op_before_editors_and_outline_are_injected() {
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let layout = WorkspaceLayoutViewModel::new(
            app_ctx.clone(),
            crate::models::WorkspaceLayoutService::in_memory_default(),
            bastyde::widgets::DockingModel::new(),
            SingleWork::new(app_ctx.clone()),
            SingleWorkInfo::new(app_ctx.clone()),
            ids.clone(),
            Signal::new(false),
            TreeExpansionViewModel::new(
                app_ctx,
                ids,
                crate::models::TreeExpansionService::in_memory_default(),
            ),
        );
        // Neither `set_editors` nor `set_outline` has been called yet.
        layout.capture_tree_expansion();
    }

    #[test]
    fn uids_round_trip_through_the_stream() {
        let order = stream();
        // capture: item ids → uids (tab order preserved).
        let uids = to_uids(&uid_map(&order), &[30, 10, 40]);
        assert_eq!(uids, vec![u(30), u(10), u(40)]);
        // restore: uids → the same ids, same order.
        let resolved: Vec<u64> = resolve_uids(&order, &uids)
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        assert_eq!(resolved, vec![30, 10, 40]);
    }

    /// A tab whose item left the binder drops at capture; a saved uid that no longer
    /// resolves drops at restore. Neither shifts the surviving entries.
    #[test]
    fn a_removed_item_drops_from_capture_and_a_stale_uid_drops_from_restore() {
        let order = stream();
        assert_eq!(
            to_uids(&uid_map(&order), &[20, 99, 40]),
            vec![u(20), u(40)],
            "item 99 is not in the binder"
        );
        let resolved: Vec<u64> = resolve_uids(&order, &[u(20), u(999), u(40)])
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        assert_eq!(resolved, vec![20, 40], "uid 999 no longer exists");
    }

    /// A nil uid is never captured: a pre-v3 item that has not been healed yet has no
    /// identity, and persisting one would make every such item look like the same tab.
    #[test]
    fn a_nil_uid_is_not_captured() {
        let order = vec![
            BinderItemRef {
                id: 10,
                uid: Uuid::nil(),
                title: "unhealed".into(),
            },
            BinderItemRef {
                id: 20,
                uid: u(20),
                title: "B".into(),
            },
        ];
        assert_eq!(to_uids(&uid_map(&order), &[10, 20]), vec![u(20)]);
    }

    /// **The reason for the whole re-key.** Inserting an item ahead of the open tabs
    /// shifts every later *ordinal*, so the ordinal scheme restored a neighbour of each
    /// tab. Uids are unaffected: the same items reopen, whatever moved.
    #[test]
    fn an_insertion_before_the_open_tabs_does_not_shift_what_reopens() {
        let captured = to_uids(&uid_map(&stream()), &[30, 40]);

        // Next session: a new item was inserted at the front of the stream.
        let mut shifted = vec![BinderItemRef {
            id: 5,
            uid: u(5),
            title: "new".into(),
        }];
        shifted.extend(stream());

        let reopened: Vec<u64> = resolve_uids(&shifted, &captured)
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        assert_eq!(
            reopened,
            vec![30, 40],
            "the same items reopen; under ordinals these would have been 20 and 30"
        );
    }

    #[test]
    fn split_never_shows_an_empty_side_pane() {
        // Nothing resolved for the side → collapse + focus primary, even if the side
        // was saved focused (the finding-driven "no empty split" rule).
        assert_eq!(split_and_focus(0, true), (false, Side::Primary));
        assert_eq!(split_and_focus(0, false), (false, Side::Primary));
        // Side has tabs → show it; focus it only when it was the focused pane.
        assert_eq!(split_and_focus(2, true), (true, Side::Secondary));
        assert_eq!(split_and_focus(2, false), (true, Side::Primary));
    }
}
