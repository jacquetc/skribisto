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

use teksilo::prelude::Signal;
use teksilo::widgets::{DockLayoutState, DockSide, DockingModel};

use frontend::AppContext;

use uuid::Uuid;

use crate::app_ids::AppIds;
use crate::binder::OutlineViewModel;
use crate::editors::{EditorsViewModel, Side};
use crate::models::{
    BinderItemRef, CorkboardTabState, PaneLayout, PerProjectLayout, TabViewState,
    WorkspaceLayoutService, ordered_binder_items, uid_is_usable,
};
use crate::settings::TreeExpansionViewModel;
use crate::singles::{SingleWork, SingleWorkInfo};

/// Per-work desk persistence (editor tabs + dock layout). Cloneable handle —
/// registered as `app_state` so the close/switch doors and `App::build` reach the
/// one instance.
#[derive(Clone)]
pub struct WorkspaceLayoutViewModel {
    app_ctx: Rc<AppContext>,
    service: WorkspaceLayoutService,
    /// Where the writer was in each item, whether or not a tab is open on it. The
    /// per-pane `view_states` below can only ever speak for tabs that *are* open,
    /// so this is what lets a closed item be reopened where it was left. Tier 2,
    /// shared with `EditorsViewModel` rather than owned by either.
    item_view_states: crate::shared::ItemViewStates,
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
        item_view_states: crate::shared::ItemViewStates,
    ) -> Self {
        Self {
            app_ctx,
            service,
            item_view_states,
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
                    // The Corkboard trail is persisted as **uids**, like the tab
                    // list itself: an `EntityId` is re-minted on every `load_work`,
                    // so a stored id would name a different item next launch. A
                    // crumb whose id no longer resolves truncates the trail there.
                    let corkboard = editors
                        .corkboard_state_of(id)
                        .map(|(trail_ids, query)| CorkboardTabState {
                            trail: trail_ids
                                .iter()
                                .map_while(|cid| uid_of.get(cid).copied())
                                .collect(),
                            query,
                        })
                        .unwrap_or_default();
                    Some(TabViewState {
                        uid,
                        caret: s.caret,
                        scroll: s.scroll,
                        corkboard,
                        // Which page a container tab was showing. Without it a
                        // restored Book comes back on whichever page the last Book
                        // of the session was left on, and the scroll offset beside
                        // it was measured on a different page entirely.
                        segment: editors.segment_of(id).unwrap_or_default(),
                    })
                })
                .collect(),
        };
        let split_active = editors.split_active().get();

        // Fold every open tab into the per-item roster, then drop the rows whose
        // item is no longer in the project. This is the one place that already has
        // the live uid set in hand, and nothing else prunes: no cascade reaches a
        // side table like this, so a scene deleted three sessions ago would sit in
        // it forever, holding a slot a live item needs.
        for side in [Side::Primary, Side::Secondary] {
            for id in editors.tab_item_ids(side) {
                let (Some(&uid), Some(s)) = (uid_of.get(&id), editors.view_state_of(id)) else {
                    continue;
                };
                self.item_view_states.record(TabViewState {
                    uid,
                    caret: s.caret,
                    scroll: s.scroll,
                    // Per pane, and only there: see `EditorsViewModel::remember_position`.
                    corkboard: CorkboardTabState::default(),
                    segment: editors.segment_of(id).unwrap_or_default(),
                });
            }
        }
        self.item_view_states
            .prune(&order.iter().map(|r| r.uid).collect());

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
            item_view_states: self.item_view_states.snapshot(),
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
            // No saved layout: default docks (above) plus the empty desk `close_all`
            // left. Clear the roster too, or a project opened after another one in
            // the same session would seed its tabs from that one's positions.
            self.item_view_states.clear();
            return;
        };

        // uid → (item id, title) in the freshly-loaded project. Resolve BOTH panes up
        // front: a saved uid can still be stale (its item was trashed or deleted between
        // capture and this load), and the split / focus decisions below must key off what
        // actually resolved, not the persisted list length, or an all-stale side pane
        // would show up empty.
        // Before a single tab is opened: `open_in` consults this to seed a tab it
        // creates, and the explicit per-pane seeding further down then overrides it
        // for the tabs that were actually open. Loading it later would leave the
        // first pass reading an empty roster.
        self.item_view_states.load(rec.item_view_states.clone());

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
                        crate::shared::ViewState {
                            caret: s.caret,
                            scroll: s.scroll,
                        },
                    );
                    // And the page it was showing. Empty for every combination with
                    // no segmented bar, which `seed_segment` ignores.
                    editors.seed_segment(side, *id, &s.segment);
                    // And the Corkboard's own navigation. `map_while` stops at the
                    // first crumb whose uid no longer names a live item: everything
                    // deeper described a path through a container that is gone, and
                    // a breadcrumb with a hole in it would offer to navigate to an
                    // ancestor that is no longer one.
                    if !s.corkboard.is_empty() {
                        let crumbs: Vec<(u64, String)> = s
                            .corkboard
                            .trail
                            .iter()
                            .map_while(|cuid| {
                                order
                                    .iter()
                                    .find(|r| r.uid == *cuid)
                                    .map(|r| (r.id, r.title.clone()))
                            })
                            .collect();
                        let (ids, titles): (Vec<u64>, Vec<String>) = crumbs.into_iter().unzip();
                        editors.seed_corkboard_state(side, *id, &ids, &titles, &s.corkboard.query);
                    }
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

        for dock in crate::docks::all_docks()
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
    use crate::docks::{
        COMMENTS_DOCK_ID, DOC_COMMENTS_DOCK_ID, FOOTNOTES_DOCK_ID, GAMES_DOCK_ID, OUTLINE_DOCK_ID,
        TIMELINE_DOCK_ID, TRASH_DOCK_ID, VERSIONS_DOCK_ID,
    };
    use teksilo::prelude::*;
    use teksilo::widgets::{DockWidget, DockWidgetId, DockingLayout, RectWidget};

    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole, GoalUnit};
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};

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
        for dock in &crate::docks::all_docks() {
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
            crate::shared::ItemViewStates::new(),
        )
    }

    /// Mount only the docks a v4-era build knew, in roster order — i.e. reproduce
    /// the desk a pre-comments Skribisto would have captured.
    fn desk_as_of_v4(model: &DockingModel) {
        for dock in crate::docks::all_docks()
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
    /// roster minus a v4 stamp is the two comments docks, the footnotes dock, the
    /// versions dock, the timeline band and the writing-games panel. A dock added
    /// to `project_shell` but not
    /// to `APP_DOCKS` would leave this list short — which is the failure mode the
    /// roster's own doc warns about, and is why every new dock has to appear here
    /// as well as there.
    ///
    /// This list grows by one every time a dock ships, and that is the point: it is
    /// the assertion that catches a dock which would otherwise be invisible on
    /// every desk saved before it existed.
    #[test]
    fn a_v4_stamp_leaves_exactly_the_docks_that_postdate_it_unknown() {
        assert_eq!(
            unknown_dock_ids(&crate::docks::app_dock_ids(), &V4_ROSTER),
            vec![
                COMMENTS_DOCK_ID,
                GAMES_DOCK_ID,
                DOC_COMMENTS_DOCK_ID,
                FOOTNOTES_DOCK_ID,
                VERSIONS_DOCK_ID,
                TIMELINE_DOCK_ID,
            ],
            "these five, and only these five, postdate a v4 desk"
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
            teksilo::widgets::DockingModel::new(),
            SingleWork::new(app_ctx.clone()),
            SingleWorkInfo::new(app_ctx.clone()),
            ids.clone(),
            Signal::new(false),
            TreeExpansionViewModel::new(
                app_ctx,
                ids,
                crate::models::TreeExpansionService::in_memory_default(),
            ),
            crate::shared::ItemViewStates::new(),
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

    // ── Live-store fixtures ──────────────────────────────────────────────────
    //
    // The precedence between `PaneLayout::view_states` and the per-item roster,
    // the roster load-before-open ordering, the fold-and-prune in `capture`, the
    // segment round trip, and the backup guards are all properties of `capture`
    // and `restore` *themselves*, not of the pure helpers above, so they need a
    // real seeded `Work` (for `ordered_binder_items` to resolve real uids) and a
    // real `EditorsViewModel` (for `open_in`/`seed_view_state`/`segment_of` to do
    // anything). `--features mocks` answers `ordered_binder_items` with an empty
    // stream (see that function's own doc), which would make `restore` a no-op
    // for tabs, so this crate's default (non-mocks) test run is what exercises
    // them; there is no lighter fixture that could.

    fn test_typography() -> crate::settings::EditorTypographySet {
        let bundle = |family: &str| crate::settings::EditorTypography {
            font_family: Signal::new(family.to_string()),
            size: Signal::new(1.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
            size_range: crate::settings::TypographySizeRange::default(),
        };
        crate::settings::EditorTypographySet {
            scene: bundle("Literata"),
            synopsis: bundle("Literata"),
            notes: bundle("Inter"),
            corkboard: bundle("Literata"),
            distraction_free: bundle("Literata"),
        }
    }

    /// One window's `(WorkspaceLayoutViewModel, EditorsViewModel)` pair over an
    /// already-seeded Work, sharing `service` and wired exactly as `App::build`
    /// wires them (editors and the roster injected after construction, mirroring
    /// `WorkspaceLayoutViewModel::set_editors` / `EditorsViewModel::set_item_view_states`).
    ///
    /// Factored out from [`live_project`] so a capture → restore round trip can
    /// build a **second**, independent window over the same backend Work rather
    /// than reusing the one that captured, which would already hold every tab
    /// open, proving nothing about whether `restore` can open one from scratch.
    fn wire_window(
        app_ctx: Rc<AppContext>,
        work_id: u64,
        service: WorkspaceLayoutService,
    ) -> (
        WorkspaceLayoutViewModel,
        EditorsViewModel,
        crate::shared::ItemViewStates,
        SingleWork,
        Signal<bool>,
    ) {
        let ids = AppIds::new();
        ids.work_id.set(Some(work_id));

        let item_view_states = crate::shared::ItemViewStates::new();
        let docs = crate::models::OpenDocsStore::new(app_ctx.clone());
        let save_state = crate::save::SaveStateViewModel::new(app_ctx.clone(), ids.clone());
        let tree_expansion = TreeExpansionViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            crate::models::TreeExpansionService::in_memory_default(),
        );
        let editors = EditorsViewModel::new(
            app_ctx.clone(),
            Signal::new(700.0),
            Signal::new(true),
            Signal::new(crate::shared::SynopsisPlacement::default()),
            Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
            test_typography(),
            crate::shared::TypewriterSettings::off(),
            crate::shared::CaretHighlightSettings::off(),
            crate::settings::EditorViewMemory::detached(false),
            crate::settings::CorkboardDefaults::detached(),
            ids.clone(),
            docs,
            Signal::new(false),
            save_state,
            Signal::new(false),
            tree_expansion.clone(),
            Signal::new(false),
            Signal::new(620.0),
            crate::go::GoAvailability::new(),
            crate::format::FormatViewModel::detached(),
            crate::writing_session::WritingGamesViewModel::detached(),
            Signal::new(GoalUnit::default()),
            crate::tags::TagsViewModel::detached(app_ctx.clone(), ids.clone()),
            crate::mentions::MentionIndex::new(app_ctx.clone(), ids.clone()),
        );
        editors.set_item_view_states(item_view_states.clone());

        let single_work = SingleWork::new(app_ctx.clone());
        let single_work_info = SingleWorkInfo::new(app_ctx.clone());
        let backup_mode = Signal::new(false);

        let vm = WorkspaceLayoutViewModel::new(
            app_ctx,
            service,
            DockingModel::new(),
            single_work.clone(),
            single_work_info,
            ids,
            backup_mode.clone(),
            tree_expansion,
            item_view_states.clone(),
        );
        vm.set_editors(editors.clone());

        (vm, editors, item_view_states, single_work, backup_mode)
    }

    /// A live fixture: a real Work with one Binder holding `n` Scene items (so
    /// [`ordered_binder_items`] resolves real, restorable uids), plus one window
    /// wired onto it via [`wire_window`].
    struct LiveProject {
        app_ctx: Rc<AppContext>,
        work_id: u64,
        vm: WorkspaceLayoutViewModel,
        editors: EditorsViewModel,
        item_view_states: crate::shared::ItemViewStates,
        service: WorkspaceLayoutService,
        single_work: SingleWork,
        backup_mode: Signal<bool>,
        items: Vec<BinderItemRef>,
    }

    fn live_project(n: usize) -> LiveProject {
        let app_ctx = Rc::new(AppContext::new());
        let work = work_commands::create_orphan_work(&app_ctx, None, &CreateWorkDto::default())
            .expect("seed a Work");
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
        .expect("seed a Binder");
        for i in 0..n {
            binder_item_commands::create_binder_item(
                &app_ctx,
                None,
                &CreateBinderItemDto {
                    title: format!("Scene {i}"),
                    role: BinderItemRole::Item,
                    sub_role: BinderItemSubRole::Scene,
                    activated: true,
                    is_exportable: true,
                    indent: 0,
                    ..Default::default()
                },
                binder.id,
                i as i32,
            )
            .expect("seed a BinderItem");
        }
        let items = ordered_binder_items(&app_ctx, work.id);
        assert_eq!(items.len(), n, "every seeded item must resolve back");

        let service = WorkspaceLayoutService::in_memory_default();
        let (vm, editors, item_view_states, single_work, backup_mode) =
            wire_window(app_ctx.clone(), work.id, service.clone());

        LiveProject {
            app_ctx,
            work_id: work.id,
            vm,
            editors,
            item_view_states,
            service,
            single_work,
            backup_mode,
            items,
        }
    }

    /// A tab's live `view_state` seed. Never `None` for a tab that is actually
    /// open: `ContentTab::capture_view_state` falls back to this seed on an
    /// unmounted pane (there is no headless widget tree anywhere in this file's
    /// fixtures), so it is exactly what `restore`/`capture` themselves read and
    /// write.
    fn tab_view_state(
        editors: &EditorsViewModel,
        side: Side,
        item_id: u64,
    ) -> Option<crate::shared::ViewState> {
        let tabs = editors.tabs(side);
        (0..tabs.len()).find_map(|i| {
            tabs.with_item(i, |h| {
                h.payload
                    .downcast_ref::<crate::tabs::ContentTab>()
                    .filter(|t| t.item_id() == item_id)
                    .map(|t| t.view_state().get())
            })
            .flatten()
        })
    }

    /// The segment a tab is currently seeded to open on, what `restore`'s
    /// `seed_segment` call leaves waiting, before any page exists to consume it.
    fn tab_segment_seed(editors: &EditorsViewModel, side: Side, item_id: u64) -> Option<String> {
        let tabs = editors.tabs(side);
        (0..tabs.len()).find_map(|i| {
            tabs.with_item(i, |h| {
                h.payload
                    .downcast_ref::<crate::tabs::ContentTab>()
                    .filter(|t| t.item_id() == item_id)
                    .and_then(|t| t.peek_segment_seed())
            })
            .flatten()
        })
    }

    /// Write directly into a tab's "segment on screen" slot, what `RememberSegment`
    /// does once a page actually mounts. Setting it here bypasses the whole
    /// container-page machinery, which is out of scope for this view-model's own
    /// tests; the sink is `pub(crate)` for exactly this reason (its own doc names
    /// "a capture to read" as the purpose).
    fn set_tab_segment_shown(editors: &EditorsViewModel, side: Side, item_id: u64, segment: &str) {
        let tabs = editors.tabs(side);
        for i in 0..tabs.len() {
            tabs.with_item(i, |h| {
                if let Some(t) = h.payload.downcast_ref::<crate::tabs::ContentTab>()
                    && t.item_id() == item_id
                {
                    *t.segment_shown_sink().borrow_mut() = segment.to_string();
                }
            });
        }
    }

    /// **Precedence.** A uid remembered in both the per-pane record and the
    /// per-item roster, at two different positions, must restore from the
    /// **pane** record. This is what keeps two split panes on the same item from
    /// collapsing onto one caret: only the pane list can tell them apart, and it
    /// only wins if it is applied *after* whatever the roster seeded, which is
    /// exactly the order `restore` uses (`open_in`'s internal roster seed, then
    /// `restore`'s own explicit per-pane seed).
    #[test]
    fn a_restored_tab_prefers_its_own_panes_view_state_over_the_roster() {
        let p = live_project(1);
        let uid = p.items[0].uid;
        let id = p.items[0].id;

        p.single_work.unique_id().set("precedence".to_string());
        p.service
            .set(PerProjectLayout {
                work_uid: "precedence".to_string(),
                primary: PaneLayout {
                    tabs: vec![uid],
                    selected: Some(uid),
                    view_states: vec![TabViewState {
                        uid,
                        caret: 42,
                        scroll: 4.2,
                        ..Default::default()
                    }],
                },
                // Disagrees with the pane record on purpose: a stale roster
                // position that must lose once a more specific one exists.
                item_view_states: vec![TabViewState {
                    uid,
                    caret: 999,
                    scroll: 99.0,
                    ..Default::default()
                }],
                ..Default::default()
            })
            .unwrap();

        p.vm.restore(false);

        assert_eq!(
            tab_view_state(&p.editors, Side::Primary, id),
            Some(crate::shared::ViewState {
                caret: 42,
                scroll: 4.2,
            }),
            "the pane's own recorded position must win over the roster's disagreeing one"
        );
    }

    /// **Load-before-open.** With nothing in the per-pane record for this tab (no
    /// entry in `primary.view_states` at all), the position it restores to can
    /// only have come from the roster via `open_in`'s internal seed, which is
    /// only possible if `restore` loaded the roster *before* opening the tab.
    /// Without that ordering `seed_from_memory` would find an empty roster and
    /// the tab would open at the top of the document instead.
    #[test]
    fn restore_loads_the_roster_before_opening_any_tab_so_open_in_can_seed_from_it() {
        let p = live_project(1);
        let uid = p.items[0].uid;
        let id = p.items[0].id;

        p.single_work.unique_id().set("roster-seed".to_string());
        p.service
            .set(PerProjectLayout {
                work_uid: "roster-seed".to_string(),
                primary: PaneLayout {
                    tabs: vec![uid],
                    selected: Some(uid),
                    view_states: vec![],
                },
                item_view_states: vec![TabViewState {
                    uid,
                    caret: 55,
                    scroll: 5.5,
                    segment: "roster-seg".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            })
            .unwrap();

        p.vm.restore(false);

        assert_eq!(
            tab_view_state(&p.editors, Side::Primary, id),
            Some(crate::shared::ViewState {
                caret: 55,
                scroll: 5.5,
            }),
            "with no per-pane record to apply, this position can only have come \
             from the roster, which open_in can only see if restore loaded it first"
        );
        assert_eq!(
            tab_segment_seed(&p.editors, Side::Primary, id),
            Some("roster-seg".to_string()),
            "same proof, on the segment half"
        );
    }

    /// **No inherited roster.** A project with no saved desk must clear the
    /// roster rather than keep whatever a previously open project in the same
    /// session left in it, or a freshly opened, never-before-seen project would
    /// seed its tabs from an unrelated one's remembered positions.
    #[test]
    fn restore_with_no_saved_desk_clears_the_roster_rather_than_keeping_a_previous_projects() {
        let p = live_project(1);
        p.item_view_states.load(vec![TabViewState {
            uid: u(9999),
            caret: 1,
            ..Default::default()
        }]);
        p.single_work.unique_id().set("never-saved".to_string());
        // Nothing written to `p.service` for "never-saved".

        p.vm.restore(false);

        assert!(
            p.item_view_states.snapshot().is_empty(),
            "a project with no saved desk must not inherit the previous project's roster"
        );
    }

    /// **Fold and prune.** `capture` folds every open tab's live position into
    /// the roster, and prunes any roster entry whose uid the live item stream no
    /// longer has, an item trashed or deleted since it was last recorded, which
    /// otherwise would sit in the roster forever, taking up a slot a live item
    /// needs (the roster is capped).
    #[test]
    fn capture_folds_open_tabs_into_the_roster_and_prunes_a_uid_that_left_the_binder() {
        let p = live_project(2);
        let uid_p = p.items[0].uid;
        let id_p = p.items[0].id;
        let uid_q = p.items[1].uid;
        let gone = u(0xDEAD);

        p.item_view_states.load(vec![
            TabViewState {
                uid: gone,
                caret: 1,
                ..Default::default()
            },
            TabViewState {
                uid: uid_q,
                caret: 2,
                ..Default::default()
            },
        ]);

        p.editors.open_in(Side::Primary, id_p, "P");
        p.editors.seed_view_state(
            Side::Primary,
            id_p,
            crate::shared::ViewState {
                caret: 99,
                scroll: 9.9,
            },
        );
        set_tab_segment_shown(&p.editors, Side::Primary, id_p, "streamseg");

        p.single_work.unique_id().set("prune".to_string());
        p.vm.capture();

        let snapshot = p.item_view_states.snapshot();
        assert!(
            !snapshot.iter().any(|s| s.uid == gone),
            "a uid no longer in the binder must be pruned from the roster"
        );
        let q = snapshot
            .iter()
            .find(|s| s.uid == uid_q)
            .expect("an item that is still live, but not open right now, must survive capture");
        assert_eq!(q.caret, 2, "…untouched, since its tab was never open");

        let folded = snapshot
            .iter()
            .find(|s| s.uid == uid_p)
            .expect("the open tab's live position must be folded into the roster");
        assert_eq!(folded.caret, 99);
        assert_eq!(folded.scroll, 9.9);
        assert_eq!(folded.segment, "streamseg");
    }

    /// **The segment round trip.** `capture` writes the segment a tab was
    /// showing into both the per-pane record and the roster; `restore` seeds it
    /// back onto a freshly opened tab in a second window over the same project ,
    /// proving the string travels through `PerProjectLayout`, not just through
    /// the live `EditorsViewModel` the capturing window already held open.
    #[test]
    fn capture_writes_a_tabs_segment_and_restore_seeds_it_back_onto_a_fresh_tab() {
        let p = live_project(1);
        let uid = p.items[0].uid;
        let id = p.items[0].id;

        p.single_work
            .unique_id()
            .set("segment-roundtrip".to_string());
        p.editors.open_in(Side::Primary, id, "S");
        set_tab_segment_shown(&p.editors, Side::Primary, id, "corkboard");

        p.vm.capture();

        let saved = p
            .service
            .get("segment-roundtrip")
            .expect("capture must have written a row");
        assert_eq!(
            saved
                .primary
                .view_states
                .iter()
                .find(|s| s.uid == uid)
                .map(|s| s.segment.as_str()),
            Some("corkboard"),
            "the pane record must carry the segment the tab was showing"
        );
        assert_eq!(
            saved
                .item_view_states
                .iter()
                .find(|s| s.uid == uid)
                .map(|s| s.segment.as_str()),
            Some("corkboard"),
            "so must the per-item roster"
        );

        // A second window on the same project, reopening from scratch.
        let (vm2, editors2, _states2, single_work2, _backup2) =
            wire_window(p.app_ctx.clone(), p.work_id, p.service.clone());
        single_work2
            .unique_id()
            .set("segment-roundtrip".to_string());
        vm2.restore(false);

        assert_eq!(
            tab_segment_seed(&editors2, Side::Primary, id),
            Some("corkboard".to_string()),
            "restore must seed the freshly opened tab with the segment capture recorded"
        );
    }

    /// **Backup mode, the capture half.** A window viewing a backup file must
    /// never overwrite the real project's saved desk, neither the on-disk row
    /// nor the live roster, which `capture` must return before ever touching.
    #[test]
    fn capture_is_a_no_op_in_backup_mode() {
        let p = live_project(1);
        let uid = p.items[0].uid;
        let id = p.items[0].id;

        p.single_work.unique_id().set("backup-capture".to_string());
        p.editors.open_in(Side::Primary, id, "S");
        p.item_view_states.record(TabViewState {
            uid,
            caret: 3,
            ..Default::default()
        });
        p.backup_mode.set(true);

        p.vm.capture();

        assert!(
            p.service.get("backup-capture").is_none(),
            "a window viewing a backup must never write the real project's saved desk"
        );
        let snapshot = p.item_view_states.snapshot();
        assert_eq!(
            snapshot.len(),
            1,
            "the live roster itself must also be untouched by a capture in backup mode"
        );
        assert_eq!(
            snapshot[0].caret, 3,
            "…not just the same length by coincidence"
        );
    }

    /// **Backup mode, the restore half.** A backup shares its source project's
    /// `unique_id` (retention correlates on it), so a saved desk can genuinely
    /// exist under the same uid a backup viewer opens with. `restore(true)` must
    /// not apply it: no tab reopens, and the roster is not populated from the
    /// source project's, a backup starts from nothing, symmetric with `capture`
    /// being inert in the same mode.
    #[test]
    fn restore_is_inert_for_a_backup_even_though_the_source_projects_desk_is_saved_under_the_same_uid()
     {
        let p = live_project(1);
        let uid = p.items[0].uid;

        p.single_work.unique_id().set("shared-uid".to_string());
        p.service
            .set(PerProjectLayout {
                work_uid: "shared-uid".to_string(),
                primary: PaneLayout {
                    tabs: vec![uid],
                    selected: Some(uid),
                    view_states: vec![TabViewState {
                        uid,
                        caret: 10,
                        ..Default::default()
                    }],
                },
                item_view_states: vec![TabViewState {
                    uid,
                    caret: 10,
                    ..Default::default()
                }],
                ..Default::default()
            })
            .unwrap();

        // A backup viewer for the same source project.
        p.vm.restore(true);

        assert!(
            p.editors.tab_item_ids(Side::Primary).is_empty(),
            "a backup must never reopen the source project's tabs"
        );
        assert!(
            p.item_view_states.snapshot().is_empty(),
            "and must not inherit the source project's per-item roster either"
        );
    }
}
