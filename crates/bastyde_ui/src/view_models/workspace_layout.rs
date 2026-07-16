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
//! ## Why ordinals
//!
//! A tab is persisted by its **position** in the work's ordered binder-item stream,
//! not by its `BinderItem` store id: store ids are remapped on every load and are
//! not stable across a save→load cycle. See
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
use frontend::commands::{binder_commands, binder_item_commands, work_commands};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;

use crate::app_ids::AppIds;
use crate::models::{PaneLayout, PerProjectLayout, WorkspaceLayoutService, uid_is_usable};
use crate::singles::{SingleWork, SingleWorkInfo};
use crate::view_models::{EditorsViewModel, Side};

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
            default_docks: Rc::new(RefCell::new(None)),
        }
    }

    /// Hand the editors view-model over, once `App::build` has created it.
    pub fn set_editors(&self, editors: EditorsViewModel) {
        *self.editors.borrow_mut() = Some(editors);
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
    /// Tabs are persisted as ordinals into the on-disk item stream, so a capture is
    /// only meaningful when the store matches disk. Capturing while dirty (e.g. a
    /// discard-close still holding an unsaved reorder) would record ordinals a
    /// reload — which reads the *saved* file — resolves against a different stream.
    /// So capture runs only when clean; the on-save trigger then keeps the persisted
    /// desk fresh and disk-aligned after every write (surviving a later hard exit),
    /// and a clean graceful close captures the final tab set.
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

        // item id → ordinal (position in the flat, binder-major item stream).
        let order = self.ordered_items();
        let index_of: HashMap<u64, usize> =
            order.iter().enumerate().map(|(i, (id, _))| (*id, i)).collect();
        let to_ordinals = |ids: Vec<u64>| -> Vec<usize> {
            ids.iter().filter_map(|id| index_of.get(id).copied()).collect()
        };
        let ordinal_of = |id: Option<u64>| -> Option<usize> {
            id.and_then(|i| index_of.get(&i).copied())
        };

        let primary = PaneLayout {
            tabs: to_ordinals(editors.tab_item_ids(Side::Primary)),
            selected: ordinal_of(editors.selected_item(Side::Primary)),
        };
        let secondary = PaneLayout {
            tabs: to_ordinals(editors.tab_item_ids(Side::Secondary)),
            selected: ordinal_of(editors.selected_item(Side::Secondary)),
        };
        let split_active = editors.split_active().get();

        let record = PerProjectLayout {
            work_uid: uid,
            last_path: self.single_work_info.file_name().get().unwrap_or_default(),
            primary,
            secondary,
            split_active,
            focus_secondary: editors.focused_side() == Side::Secondary,
            editor_splitter: split_active.then(|| editors.splitter().export_state()),
            docks: Some(self.docking.export_state()),
        };
        if let Err(e) = self.service.set(record) {
            eprintln!("skribisto: workspace layout capture failed: {e}");
        }
    }

    // ── Restore ───────────────────────────────────────────────────────────────

    /// Apply the open project's saved desk (or the defaults). Call after the
    /// id-seeding subscriber has run for this `LoadWork` / `NewWork`.
    pub fn restore(&self) {
        let uid = self.single_work.unique_id().get();
        // A backup file shares its source project's `unique_id` (retention
        // correlates on it), so its saved layout is the *source's*. Never import it
        // into a backup view: that would show the live project's docks + tabs
        // (resolved against the backup's own, possibly older, item stream) behind
        // the backup-choice modal. Give a backup a clean default desk instead —
        // symmetric with `capture` being inert in backup mode. Sniffed here from the
        // path because `backup_mode` is only set by a *later* `LoadWork` subscriber.
        let path = self.single_work_info.file_name().get().unwrap_or_default();
        let is_backup =
            !path.trim().is_empty() && crate::backup::backup_context_for(&path).is_some();
        let saved = if !is_backup && uid_is_usable(&uid) {
            self.service.get(&uid)
        } else {
            None
        };

        // Docks first: this project's saved arrangement, or reset to the default so
        // an unconfigured project (or a backup view) never inherits the previous
        // one's docks.
        match saved.as_ref().and_then(|r| r.docks.as_ref()) {
            Some(docks) => self.docking.import_state(docks),
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

        // ordinal → (item id, title) in the freshly-loaded project. Resolve BOTH
        // panes up front: a saved ordinal can be stale (its item removed between
        // capture and this load — the "persist by position" tradeoff), and the
        // split / focus decisions below must key off what actually resolved, not the
        // persisted list length, or an all-stale side pane would show up empty.
        let order = self.ordered_items();
        let resolve = |ord: usize| -> Option<(u64, String)> { order.get(ord).cloned() };
        let primary_tabs: Vec<(u64, String)> =
            rec.primary.tabs.iter().filter_map(|&o| resolve(o)).collect();
        let secondary_tabs: Vec<(u64, String)> =
            rec.secondary.tabs.iter().filter_map(|&o| resolve(o)).collect();

        for (id, title) in &primary_tabs {
            editors.open_in(Side::Primary, *id, title);
        }
        if secondary_tabs.is_empty() {
            // Nothing resolved for the side pane — never show an empty split, even if
            // one was saved active.
            editors.set_split(false);
        } else {
            editors.set_split(true);
            for (id, title) in &secondary_tabs {
                editors.open_in(Side::Secondary, *id, title);
            }
            // The two-pane splitter ratio (only meaningful while split).
            if let Some(sp) = &rec.editor_splitter {
                editors.splitter().import_state(sp);
            }
        }

        // Re-select the tabs that were selected (open_in leaves the last-opened one
        // selected; override to the persisted choice).
        if let Some(ord) = rec.primary.selected
            && let Some((id, _)) = resolve(ord)
        {
            editors.select_item(Side::Primary, id);
        }
        if !secondary_tabs.is_empty()
            && let Some(ord) = rec.secondary.selected
            && let Some((id, _)) = resolve(ord)
        {
            editors.select_item(Side::Secondary, id);
        }

        // Re-mark the focused pane (drives the binder's open-item accent) — the side
        // pane only if it actually restored tabs.
        let focus = if rec.focus_secondary && !secondary_tabs.is_empty() {
            Side::Secondary
        } else {
            Side::Primary
        };
        editors.set_focused(focus);
    }

    /// Reset the docks to the pristine default arrangement (first-build snapshot).
    fn apply_default_docks(&self) {
        if let Some(def) = self.default_docks.borrow().clone() {
            self.docking.import_state(&def);
        }
    }

    // ── Backend enumeration ─────────────────────────────────────────────────────

    /// The open project's binder items as `(item_id, title)`, in the flat,
    /// binder-major stream order — the authoritative relationship order that a save
    /// writes and a load reproduces. Empty when no project is open.
    fn ordered_items(&self) -> Vec<(u64, String)> {
        let ctx = &self.app_ctx;
        let Some(work_id) = self.ids.work_id.get() else {
            return Vec::new();
        };
        let mut out = Vec::new();
        let binder_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default();
        for binder_id in binder_ids {
            let item_ids = binder_commands::get_binder_relationship(
                ctx,
                &binder_id,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            // `get_binder_item_multi` returns `Vec<Option<..>>` in db-key order (not
            // request order), so index by id and walk `item_ids` (the authoritative
            // relationship order) to build the stream.
            let title_of: HashMap<u64, String> =
                binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                    .map(|it| (it.id, it.title))
                    .collect();
            for id in item_ids {
                let title = title_of.get(&id).cloned().unwrap_or_default();
                out.push((id, title));
            }
        }
        out
    }
}
