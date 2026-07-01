//! Layer B — UI **view-models** (the VM in MVVM).
//!
//! A *view-model* is a cloneable handle that owns one UI feature's **state**
//! (`Signal`s, `bastyde::data` models, framework model handles) and exposes its
//! **business API** as plain methods. Widgets bind to a view-model's signals and
//! forward events to its methods; no business logic lives in `build()`.
//!
//! The three MVVM layers in `bastyde_ui`:
//!   * **Model** — `models/` (reactive `bastyde::data` adapters over the Qleany
//!     backend) + the Qleany controllers/use-cases below them.
//!   * **View** — the widgets (`app.rs`, `editor_tab.rs`, `settings_panel.rs`).
//!   * **ViewModel** — this file. Sits between the two; plain Rust, so it
//!     unit-tests headless with no `WidgetTree`/GPU.
//!
//! "Controller" is Qleany's (backend: UI → Controllers → Use Cases); view-models
//! sit *above* that line.
//!
//! Two ownership shapes recur:
//!   * **Single-instance live state** (`EditorsViewModel`, `OutlineViewModel`):
//!     owns real mutable handles (a tab list, a `DockingModel`). Created once by
//!     `App`, shared by `.clone()`. Must have exactly one owner.
//!   * **Store-backed facade** (`SettingsViewModel`): owns only cached settings
//!     `Signal`s. Because `SettingsStore` returns the *same* signal per key,
//!     every instance is a view over the same live state — so it can be rebuilt
//!     anywhere a store is reachable (`SettingsViewModel::new(ctx.settings())`).
//!
//! Cross-view-model rules (keep the dependency graph a DAG):
//!   * A view-model may hold framework model handles and call *down* into them.
//!   * Peer view-models do **not** import each other; `App` mediates them (see the
//!     outline-selection → editor-open effect in `app.rs`).
//!   * Many-to-one / distant links graduate to the intent bus.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use bastyde::data::{DropPosition, KeyedSelectionModel, ListModel, SelectionMode};
use bastyde::prelude::*; // EventContext, Signal, intui, lit!
use bastyde::settings::SettingsStore;
use bastyde::widgets::{
    DockOpenLocation, DockSide, DockWidgetId, DockingModel, InputDialog, TabHandle, TabId, TabInfo,
    Toast,
};

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands, content_commands,
    trash_management_commands, undo_redo_commands, work_commands, work_management_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use frontend::direct_access::{
    ContentDto, CreateBinderItemDto, UpdateBinderDto, UpdateBinderItemDto,
};
use frontend::work_management::{LoadWorkDto, NewWorkDto};

use frontend::binder_item_management::{DuplicateDto, MoveDto, MovePlace};
use frontend::trash_management::{TrashBinderDto, TrashBinderItemsDto};
use frontend::work_management::SaveWorkDto;

use crate::app_ids::AppIds;
use crate::models::{BinderBinderItemsTreeModel, BinderTreeKey, CommitMove};
use crate::singles::{SingleBinder, SingleBinderItem};
use crate::tabs::{self, ContentTab};
use crate::{
    AUTOSAVE_KEY, DARK_KEY, EDITOR_WIDTH_DEFAULT, EDITOR_WIDTH_KEY, LOCALE_KEY, SHOW_WELCOME_KEY,
};

// ─────────────────────────────────────────────────────────────────────────────
// EditorsViewModel — the open-editor tab set and the open/focus/close logic.
//
// Single-instance live state: owns the tab `ListModel` and the selection signal,
// so `App` creates exactly one and shares it by clone.
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Clone)]
pub struct EditorsViewModel {
    app_ctx: Rc<AppContext>,
    tabs: ListModel<TabHandle>,
    selected_tab: Signal<Option<TabId>>,
    /// The `BinderItem` of the currently-active editor tab — the "open
    /// document". Drives the binder's persistent open-item marker (independent
    /// of selection/focus). Kept in sync with `selected_tab`.
    active_item: Signal<Option<u64>>,
    column_width: Signal<f32>,
    /// The per-`Work` undo stack id — shared with `OutlineViewModel` so editor
    /// write-back lands on the same Ctrl+Z history as tree edits. `App` wires it.
    stack_id: Signal<Option<u64>>,
    /// Reactive read handle re-pointed at an item when opening its tab — supplies
    /// the `(role, sub_role)` that selects the tab layout (Layer A single).
    item_probe: SingleBinderItem,
    /// Bumped by every open tab's editor `on_change` — the edit signal the
    /// debounced autosave timer (in `App`) observes.
    edited: Signal<u64>,
}

impl EditorsViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        column_width: Signal<f32>,
        stack_id: Signal<Option<u64>>,
    ) -> Self {
        Self {
            item_probe: SingleBinderItem::new(app_ctx.clone()),
            app_ctx,
            tabs: ListModel::from_vec(Vec::new()),
            selected_tab: Signal::new(None),
            active_item: Signal::new(None),
            column_width,
            stack_id,
            edited: Signal::new(0),
        }
    }

    /// The "an edit happened" signal — bind the debounced autosave to it.
    pub fn edited_signal(&self) -> Signal<u64> {
        self.edited.clone()
    }

    /// The dynamic-tab model to hand to `TabWidget::dynamic_model`.
    pub fn tabs(&self) -> ListModel<TabHandle> {
        self.tabs.clone()
    }

    /// The selected-tab signal to hand to `TabWidget::new`.
    pub fn selected_tab(&self) -> Signal<Option<TabId>> {
        self.selected_tab.clone()
    }

    /// The currently-open item id (active editor tab). Bind a binder row's
    /// "open document" accent to this.
    pub fn active_item(&self) -> Signal<Option<u64>> {
        self.active_item.clone()
    }

    /// Recompute `active_item` from the currently-selected tab. Call whenever
    /// `selected_tab` changes (open, close, or a tab-bar click).
    pub fn sync_active_item(&self) {
        let active = self
            .selected_tab
            .get()
            .and_then(|tab| self.item_of_tab(tab));
        if self.active_item.get() != active {
            self.active_item.set(active);
        }
    }

    /// The `BinderItem` id behind a tab, if it's an editor tab.
    fn item_of_tab(&self, tab: TabId) -> Option<u64> {
        for i in 0..self.tabs.len() {
            let hit = self.tabs.with_item(i, |h| {
                if h.id == tab {
                    h.payload.downcast_ref::<ContentTab>().map(|e| e.item_id)
                } else {
                    None
                }
            });
            if let Some(Some(id)) = hit {
                return Some(id);
            }
        }
        None
    }

    /// Open the editor tab for `item_id`, or focus it if already open. The view
    /// is chosen per `(role, sub_role)` — not every row is the prose editor.
    pub fn open_or_focus(&self, item_id: u64, title: &str) {
        if let Some(tid) = self.find_open(item_id) {
            self.selected_tab.set(Some(tid));
            return;
        }
        // Read the item's `(role, sub_role)` through the reactive single rather
        // than an ad-hoc `get_binder_item` (Layer A).
        self.item_probe.set_id(Some(item_id));
        let Some(item) = self.item_probe.dto() else {
            return;
        };
        let contents = self.load_contents(item_id, &item.role, &item.sub_role);
        let mut tab = tabs::tab_for(
            &self.app_ctx,
            item_id,
            &item.role,
            &item.sub_role,
            &contents,
            self.column_width.clone(),
        );
        // Every tab bumps the shared edit signal, so the autosave timer sees edits
        // from whichever tab is active.
        tab.edited = Some(self.edited.clone());
        // The item's title (data), or a translated "Untitled" fallback for
        // empty ones — locale-reactive so a language switch re-labels the tab.
        let tab_title = if title.is_empty() {
            tr!(untitled())
        } else {
            lit!(title.to_string())
        };
        let id = TabId::fresh();
        self.tabs.push(TabHandle::dynamic(
            id,
            "editor",
            TabInfo::new().title(tab_title).closable(true),
            tab,
        ));
        self.selected_tab.set(Some(id));
    }

    /// Persist every open tab's edits back to its `Content` rows (changed fields
    /// only), through the per-Work undo stack.
    pub fn flush_all(&self) {
        let stack = self.stack_id.get();
        for i in 0..self.tabs.len() {
            self.tabs.with_item(i, |h| {
                if let Some(t) = h.payload.downcast_ref::<ContentTab>() {
                    let _ = t.flush(stack);
                }
            });
        }
    }

    /// Save the tab with `tab_id` (if any) then remove it — the `TabWidget`'s
    /// `on_close` hook, so closing never drops unsaved edits.
    pub fn flush_and_close(&self, tab_id: TabId) {
        let stack = self.stack_id.get();
        let mut pos = None;
        for i in 0..self.tabs.len() {
            let hit = self.tabs.with_item(i, |h| {
                if h.id == tab_id {
                    if let Some(t) = h.payload.downcast_ref::<ContentTab>() {
                        let _ = t.flush(stack);
                    }
                    true
                } else {
                    false
                }
            });
            if hit == Some(true) {
                pos = Some(i);
                break;
            }
        }
        if let Some(p) = pos {
            self.tabs.remove(p);
        }
        if self.selected_tab.get() == Some(tab_id) {
            let next = (0..self.tabs.len()).find_map(|i| self.tabs.with_item(i, |h| h.id));
            self.selected_tab.set(next);
        }
    }

    /// Flush all editors to the store, then write the project to disk
    /// (`save_work`). The save is a long operation; we kick it and let the
    /// long-operation manager run it.
    pub fn save_to_disk(&self) {
        self.flush_all();
        let _ = work_management_commands::save_work(
            &self.app_ctx,
            &SaveWorkDto {
                file_name: String::new(),
                overwrite: true,
            },
        );
    }

    /// Close every open tab (e.g. on project load).
    pub fn close_all(&self) {
        while !self.tabs.is_empty() {
            self.tabs.remove(0);
        }
        self.selected_tab.set(None);
    }

    /// `Some(tab id)` if an editor for `item_id` is already open.
    fn find_open(&self, item_id: u64) -> Option<TabId> {
        for i in 0..self.tabs.len() {
            let hit = self.tabs.with_item(i, |h| {
                h.payload
                    .downcast_ref::<ContentTab>()
                    .filter(|e| e.item_id == item_id)
                    .map(|_| h.id)
            });
            if let Some(Some(tid)) = hit {
                return Some(tid);
            }
        }
        None
    }

    /// Read an item's content rows, keeping only the roles the constraint
    /// matrix allows for its `(role, sub_role)`. `Content.data` is Djot (the
    /// canonical store format).
    fn load_contents(
        &self,
        item_id: u64,
        role: &BinderItemRole,
        sub_role: &BinderItemSubRole,
    ) -> Vec<ContentDto> {
        let ctx = &*self.app_ctx;
        let allowed = skribisto_model::allowed_content(role, sub_role);
        let content_ids = binder_item_commands::get_binder_item_relationship(
            ctx,
            &item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default();
        content_commands::get_content_multi(ctx, &content_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .filter(|c| allowed.contains(&c.role))
            .collect()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// OutlineViewModel — the binder/outline dock: its tree, its selection, and its
// visibility. Visibility is *delegated* to the `DockingModel` (which is itself a
// cloneable model handle); the view-model adds no visibility state of its own —
// it only binds the dock id + side and speaks Skribisto vocabulary.
//
// Single-instance live state: owns the `DockingModel` and the tree model.
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Clone)]
pub struct OutlineViewModel {
    app_ctx: Rc<AppContext>,
    model: BinderBinderItemsTreeModel,
    selection: KeyedSelectionModel<BinderTreeKey>,
    docking: DockingModel,
    dock_id: DockWidgetId,
    /// The app's id-only global state (root/work/work-info/undo-stack ids).
    /// Binder-tree mutations run on `ids.stack_id` so one Ctrl+Z history reverses
    /// them all; shared by clone with `EditorsViewModel` and the singles.
    ids: AppIds,
    /// Reactive read handles (Layer A) re-pointed to fetch the current state of
    /// the item/binder a mutation is about to update — replacing ad-hoc `get_*`.
    item_probe: SingleBinderItem,
    binder_probe: SingleBinder,
}

// The visibility / reveal / action methods are the feature's public API
// (callable from menus, shortcuts, other view-models); wired incrementally, so
// not every one has a caller yet.
#[allow(dead_code)]
impl OutlineViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        model: BinderBinderItemsTreeModel,
        docking: DockingModel,
        dock_id: DockWidgetId,
    ) -> Self {
        let vm = Self {
            item_probe: SingleBinderItem::new(app_ctx.clone()),
            binder_probe: SingleBinder::new(app_ctx.clone()),
            app_ctx,
            model,
            selection: KeyedSelectionModel::new(SelectionMode::Single),
            docking,
            dock_id,
            ids,
        };
        vm.install_reorder();
        vm
    }

    /// The current `BinderItem` DTO, read through the reactive single (Layer A) —
    /// the source of truth when a tree mutation builds its update.
    fn item_dto(&self, id: u64) -> Option<frontend::direct_access::BinderItemDto> {
        self.item_probe.set_id(Some(id));
        self.item_probe.dto()
    }

    /// The current `Binder` DTO, read through the reactive single (Layer A).
    fn binder_dto(&self, id: u64) -> Option<frontend::direct_access::BinderDto> {
        self.binder_probe.set_id(Some(id));
        self.binder_probe.dto()
    }

    /// Build the outline view-model with its standard leading-dock presentation
    /// (280 px side + 48 px activity rail). The single place that knows the
    /// outline's dock geometry — so `App` and the title-bar menu share one
    /// handle without either re-stating layout constants.
    pub fn new_default(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        let model = BinderBinderItemsTreeModel::new(app_ctx.clone(), ids.work_id.clone());
        let docking = DockingModel::new();
        docking.set_side_size(DockSide::Leading, 280.0);
        // A non-zero rail thickness switches the leading side to Rail
        // presentation (a `DockActivityBar` icon rail); the layout sizes the
        // rail from the `DockRail` config.
        docking.set_side_rail(DockSide::Leading, 48.0);
        Self::new(app_ctx, ids, model, docking, DockWidgetId::fresh())
    }

    /// Inject the model's drag-reorder closure. **Cycle-safety:** it captures
    /// only `app_ctx` + the `stack_id` signal and calls the free `apply_move`
    /// helper — NOT `self` (capturing the vm would form model → closure → vm →
    /// model, the `Rc` cycle the designer deliberately avoids).
    fn install_reorder(&self) {
        let app_ctx = self.app_ctx.clone();
        let stack_id = self.ids.stack_id.clone();
        let commit: CommitMove = Rc::new(move |dragged, target, place| {
            apply_move(&app_ctx, stack_id.get(), dragged, target, place).is_ok()
        });
        self.model.set_reorder(commit);
    }

    // ── handles for wiring the widgets / layout ──
    /// The `TreeDataSource` to hand to `TreeView::from_source_keyed`.
    pub fn model(&self) -> BinderBinderItemsTreeModel {
        self.model.clone()
    }
    pub fn selection(&self) -> KeyedSelectionModel<BinderTreeKey> {
        self.selection.clone()
    }
    pub fn selection_signal(&self) -> Signal<HashSet<BinderTreeKey>> {
        self.selection.selection_signal()
    }
    pub fn docking(&self) -> DockingModel {
        self.docking.clone()
    }
    pub fn dock_id(&self) -> DockWidgetId {
        self.dock_id
    }

    /// Resolve a tree key to its `(item_id, title)` (binder rows: `item_id` None).
    pub fn node_item(&self, key: BinderTreeKey) -> Option<(Option<u64>, String)> {
        self.model.node_of(&key)
    }

    // ── visibility: pure delegation to the DockingModel ──
    //
    // The outline lives on a *rail* side, so "visible" means the side panel is
    // shown (the rail itself persists) — i.e. side visibility, the same concept
    // the activity-rail's click-to-hide drives. We toggle/reflect side
    // visibility, NOT dock open/close (which would add/remove the rail icon).

    /// Show the outline panel.
    pub fn show(&self) {
        self.docking.set_side_visible(DockSide::Leading, true);
    }
    /// Hide the outline panel (the rail stays).
    pub fn hide(&self) {
        self.docking.set_side_visible(DockSide::Leading, false);
    }
    /// Flip the outline panel's visibility.
    pub fn toggle(&self) {
        let visible = self.docking.is_side_visible(DockSide::Leading);
        self.docking.set_side_visible(DockSide::Leading, !visible);
    }
    /// Reactive shown/hidden state — bind a menu check or toolbar toggle to this.
    /// Reflects the side's visibility (also flips when the rail hides the side).
    pub fn is_visible(&self) -> Signal<bool> {
        self.docking.side_visible_signal(DockSide::Leading)
    }

    /// Register the outline dock on its side (called once, at layout build).
    pub fn open_in_layout(&self) {
        self.docking
            .open_dock(self.dock_id, DockOpenLocation::side(DockSide::Leading));
    }

    /// Rebuild the tree from the backend (e.g. on project load), pruning a
    /// now-stale selection (a trashed item drops out of the tree).
    pub fn reload(&self) {
        self.model.reload();
        let model = self.model.clone();
        self.selection.prune_missing(move |k| model.contains(k));
    }

    /// Bring the outline forward AND select a row ("reveal in outline").
    pub fn reveal_item(&self, key: BinderTreeKey) {
        self.show();
        self.selection.select(key);
    }

    // ── undo stack (one per loaded Work) ──

    /// Create the per-`Work` undo stack. Call on `LoadWork`.
    pub fn init_stack(&self) {
        self.ids.open_stack(&self.app_ctx);
    }
    fn stack(&self) -> Option<u64> {
        self.ids.stack_id.get()
    }
    /// The shared undo-stack signal, handed to `EditorsViewModel` so editor
    /// write-back lands on the same Ctrl+Z history as tree edits.
    pub fn stack_id_signal(&self) -> Signal<Option<u64>> {
        self.ids.stack_id.clone()
    }

    // ── actions (each: backend command on the undo stack, then reload) ──

    /// Create a new item/folder at the insertion point derived from selection.
    /// A "folder" is just `role = Folder`; there is no separate `new_folder`.
    pub fn new_item(&self, role: BinderItemRole, sub_role: BinderItemSubRole) {
        let anchor = self.selection.selected_keys().first().copied();
        self.create_item(anchor, role, sub_role);
    }

    /// Create a new item/folder anchored at a specific row (context-menu entry —
    /// does not touch the selection, so it never opens an editor).
    pub fn new_item_at(
        &self,
        anchor: BinderTreeKey,
        role: BinderItemRole,
        sub_role: BinderItemSubRole,
    ) {
        self.create_item(Some(anchor), role, sub_role);
    }

    fn create_item(
        &self,
        anchor: Option<BinderTreeKey>,
        role: BinderItemRole,
        sub_role: BinderItemSubRole,
    ) {
        if skribisto_model::validate_item(&role, &sub_role, &[]).is_err() {
            return;
        }
        let Some((binder, index, indent)) = self.insertion_point(anchor) else {
            return;
        };
        let title = match role {
            BinderItemRole::Folder => "New Folder",
            BinderItemRole::Item => "New Item",
        };
        let dto = CreateBinderItemDto {
            title: title.to_string(),
            role,
            sub_role,
            activated: true,
            is_printable: true,
            indent,
            ..Default::default()
        };
        if binder_item_commands::create_binder_item(
            &self.app_ctx,
            self.stack(),
            &dto,
            binder,
            index as i32,
        )
        .is_ok()
        {
            self.reload();
        }
    }

    /// Begin a rename: present a modal `InputDialog`, applying `rename` on OK.
    pub fn begin_rename(&self, key: BinderTreeKey, ctx: &mut EventContext) {
        let current = self.model.node_of(&key).map(|(_, t)| t).unwrap_or_default();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_rename()))
            .default_text(current)
            .on_result(move |result, _ctx| {
                if let Some(name) = result
                    && !name.trim().is_empty()
                {
                    vm.rename(key, &name);
                }
            })
            .present(ctx);
    }

    /// Apply a rename to a binder (name) or item (title).
    pub fn rename(&self, key: BinderTreeKey, title: &str) {
        let ctx = &*self.app_ctx;
        match key {
            BinderTreeKey::Binder(b) => {
                if let Some(binder) = self.binder_dto(b) {
                    let dto = UpdateBinderDto {
                        id: b,
                        created_at: binder.created_at,
                        updated_at: binder.updated_at,
                        name: title.to_string(),
                        activated: binder.activated,
                    };
                    let _ = binder_commands::update_binder(ctx, self.stack(), &dto);
                }
            }
            BinderTreeKey::Item(i) => {
                if let Some(it) = self.item_dto(i) {
                    let mut dto = update_item_dto(&it);
                    dto.title = title.to_string();
                    let _ = binder_item_commands::update_binder_item(ctx, self.stack(), &dto);
                }
            }
        }
        self.reload();
    }

    /// Rename the first selected row (the `binder.rename` command entry point).
    pub fn rename_selected(&self, ctx: &mut EventContext) {
        if let Some(key) = self.selection.selected_keys().first().copied() {
            self.begin_rename(key, ctx);
        }
    }

    /// Move the selected items to trash (the `binder.trash_selected` command).
    pub fn trash_selected(&self) {
        let sel = self.selection.selected_keys();
        self.trash_keys(&sel);
    }

    /// Move the given keys to trash (binders and items both supported). Used by
    /// the context menu (operates on the right-clicked row, not the selection).
    pub fn trash_keys(&self, sel: &[BinderTreeKey]) {
        let ctx = &*self.app_ctx;
        if sel.is_empty() {
            return;
        }
        let stack = self.stack();
        let composite = sel.len() > 1;
        if composite {
            let _ = undo_redo_commands::begin_composite(ctx, stack);
        }
        // Whole binders.
        for key in sel {
            if let BinderTreeKey::Binder(b) = key {
                let _ = trash_management_commands::trash_binder(
                    ctx,
                    stack,
                    &TrashBinderDto {
                        binder_id: *b as i64,
                    },
                );
            }
        }
        // Items, grouped by their origin binder.
        let mut by_binder: HashMap<u64, Vec<i64>> = HashMap::new();
        for key in sel {
            if let BinderTreeKey::Item(i) = key
                && let Some(b) = self.model.binder_of(key)
            {
                by_binder.entry(b).or_default().push(*i as i64);
            }
        }
        for (binder, ids) in by_binder {
            let _ = trash_management_commands::trash_binder_items(
                ctx,
                stack,
                &TrashBinderItemsDto {
                    binder_item_ids: ids,
                    origin_binder_id: binder as i64,
                },
            );
        }
        if composite {
            undo_redo_commands::end_composite(ctx);
        }
        self.reload();
    }

    /// Duplicate the selected items (the `binder.duplicate` command).
    pub fn duplicate_selected(&self) {
        let sel = self.selection.selected_keys();
        self.duplicate_keys(&sel);
    }

    /// Duplicate the given keys' item subtrees (binder keys ignored). Used by the
    /// context menu (operates on the right-clicked row, not the selection).
    pub fn duplicate_keys(&self, keys: &[BinderTreeKey]) {
        let item_ids: Vec<u64> = keys
            .iter()
            .filter_map(|k| match k {
                BinderTreeKey::Item(i) => Some(*i),
                _ => None,
            })
            .collect();
        if item_ids.is_empty() {
            return;
        }
        let _ = binder_item_management_commands::duplicate(
            &self.app_ctx,
            self.stack(),
            &DuplicateDto { item_ids },
        );
        self.reload();
    }

    /// Move one dragged item relative to a target (drag-drop and keyboard moves
    /// share the same `apply_move` helper).
    pub fn move_item(&self, dragged: BinderTreeKey, target: BinderTreeKey, place: DropPosition) {
        if apply_move(&self.app_ctx, self.stack(), dragged, target, place).is_ok() {
            self.reload();
        }
    }

    pub fn indent_selected(&self) {
        self.reindent(1);
    }
    pub fn outdent_selected(&self) {
        self.reindent(-1);
    }

    // ── private helpers ──

    /// `(binder, insert_index, indent)` for a new item, relative to `anchor`.
    fn insertion_point(&self, anchor: Option<BinderTreeKey>) -> Option<(u64, usize, i64)> {
        let ctx = &*self.app_ctx;
        match anchor {
            Some(BinderTreeKey::Binder(b)) => Some((b, 0, 0)),
            Some(BinderTreeKey::Item(i)) => {
                let binder = self.model.binder_of(&BinderTreeKey::Item(i))?;
                let order = binder_commands::get_binder_relationship(
                    ctx,
                    &binder,
                    &BinderRelationshipField::BinderItems,
                )
                .ok()?;
                let pos = order.iter().position(|&x| x == i)?;
                let it = self.item_dto(i)?;
                let indent = if it.role == BinderItemRole::Folder {
                    it.indent + 1 // first child of the folder
                } else {
                    it.indent // following sibling
                };
                Some((binder, pos + 1, indent))
            }
            None => Some((self.first_binder()?, 0, 0)),
        }
    }

    fn first_binder(&self) -> Option<u64> {
        let ctx = &*self.app_ctx;
        // ids-only global state: the open Work's id is known; no `get_all_work`.
        let work_id = self.ids.work_id.get()?;
        work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
            .ok()?
            .into_iter()
            .next()
    }

    fn reindent(&self, delta: i64) {
        let ctx = &*self.app_ctx;
        let items: Vec<u64> = self
            .selection
            .selected_keys()
            .iter()
            .filter_map(|k| match k {
                BinderTreeKey::Item(i) => Some(*i),
                _ => None,
            })
            .collect();
        if items.is_empty() {
            return;
        }
        let stack = self.stack();
        let composite = items.len() > 1;
        if composite {
            let _ = undo_redo_commands::begin_composite(ctx, stack);
        }
        for i in items {
            let Some(binder) = self.model.binder_of(&BinderTreeKey::Item(i)) else {
                continue;
            };
            let Ok(order) = binder_commands::get_binder_relationship(
                ctx,
                &binder,
                &BinderRelationshipField::BinderItems,
            ) else {
                continue;
            };
            let Some(pos) = order.iter().position(|&x| x == i) else {
                continue;
            };
            let Some(it) = self.item_dto(i) else {
                continue;
            };
            let new_indent = if delta > 0 {
                // Indent ≤ predecessor sibling's indent + 1 (can't indent the
                // first child of a level).
                let pred = if pos == 0 {
                    -1
                } else {
                    self.item_dto(order[pos - 1])
                        .map(|p| p.indent)
                        .unwrap_or(-1)
                };
                (it.indent + 1).min(pred + 1).max(0)
            } else {
                (it.indent - 1).max(0)
            };
            if new_indent != it.indent {
                let mut dto = update_item_dto(&it);
                dto.indent = new_indent;
                let _ = binder_item_commands::update_binder_item(ctx, stack, &dto);
            }
        }
        if composite {
            undo_redo_commands::end_composite(ctx);
        }
        self.reload();
    }
}

/// Build a scalar-only `UpdateBinderItemDto` from a fetched item.
fn update_item_dto(it: &frontend::direct_access::BinderItemDto) -> UpdateBinderItemDto {
    UpdateBinderItemDto {
        id: it.id,
        created_at: it.created_at,
        updated_at: it.updated_at,
        title: it.title.clone(),
        sub_title: it.sub_title.clone(),
        role: it.role.clone(),
        sub_role: it.sub_role.clone(),
        label: it.label.clone(),
        activated: it.activated,
        is_favorite: it.is_favorite,
        is_printable: it.is_printable,
        indent: it.indent,
        word_count_goal: it.word_count_goal,
        char_count_goal: it.char_count_goal,
        dict_language: it.dict_language.clone(),
    }
}

/// Apply a single-item move through the backend (undoable). Free function so the
/// model's reorder closure can call it without capturing the view-model (which
/// would create an `Rc` cycle model → closure → vm → model).
pub(crate) fn apply_move(
    ctx: &AppContext,
    stack: Option<u64>,
    dragged: BinderTreeKey,
    target: BinderTreeKey,
    place: DropPosition,
) -> anyhow::Result<()> {
    let item_id = match dragged {
        BinderTreeKey::Item(i) => i,
        BinderTreeKey::Binder(_) => anyhow::bail!("only items can be moved"),
    };
    let (target_id, target_is_binder) = match target {
        BinderTreeKey::Binder(b) => (b, true),
        BinderTreeKey::Item(i) => (i, false),
    };
    let move_place = match place {
        DropPosition::Before => MovePlace::Before,
        DropPosition::After => MovePlace::After,
        DropPosition::Into => MovePlace::Into,
    };
    binder_item_management_commands::move_items(
        ctx,
        stack,
        &MoveDto {
            item_ids: vec![item_id],
            target_id: Some(target_id),
            target_is_binder,
            move_place,
        },
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// SettingsViewModel — a facade over the persisted UI settings.
//
// Store-backed: holds only cached settings `Signal`s, so every instance is a view
// over the same live state. Rebuild it anywhere via `SettingsViewModel::new
// (ctx.settings())`. Ambient app mutations (theme/locale) reach the live app
// through an `EventContext`; pure-state ops (column width) do not.
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Clone)]
pub struct SettingsViewModel {
    dark: Signal<bool>,
    locale: Signal<String>,
    column_width: Signal<f32>,
    autosave: Signal<bool>,
    show_welcome: Signal<bool>,
}

// Accessors/setters are the feature's public API; bound to widgets incrementally.
#[allow(dead_code)]
impl SettingsViewModel {
    pub fn new(store: &SettingsStore) -> Self {
        Self {
            dark: store.signal(DARK_KEY, false),
            locale: store.signal(LOCALE_KEY, "en-US".to_string()),
            column_width: store.signal(EDITOR_WIDTH_KEY, EDITOR_WIDTH_DEFAULT),
            autosave: store.signal(AUTOSAVE_KEY, false),
            show_welcome: store.signal(SHOW_WELCOME_KEY, true),
        }
    }

    /// Whether to autosave to disk (hides the manual Save affordances when on).
    /// Store-backed, so toggling it persists.
    pub fn autosave(&self) -> Signal<bool> {
        self.autosave.clone()
    }

    /// Whether to show the Welcome modal at startup (default on). Same cached
    /// `SHOW_WELCOME_KEY` signal the Welcome dialog's inline checkbox binds.
    pub fn show_welcome(&self) -> Signal<bool> {
        self.show_welcome.clone()
    }

    // ── reactive accessors for binding ──
    pub fn column_width(&self) -> Signal<f32> {
        self.column_width.clone()
    }
    pub fn dark(&self) -> Signal<bool> {
        self.dark.clone()
    }
    pub fn locale(&self) -> Signal<String> {
        self.locale.clone()
    }

    // ── business API ──

    /// Switch theme live and persist the choice.
    pub fn set_dark(&self, ctx: &mut EventContext, dark: bool) {
        ctx.set_theme(if dark { intui::dark() } else { intui::light() });
        self.dark.set(dark); // same cached signal → persisted
    }

    /// Switch locale live and persist the choice.
    pub fn set_locale(&self, ctx: &mut EventContext, locale: &str) {
        ctx.set_locale(locale);
        self.locale.set(locale.to_string());
    }

    /// Set the centered writing-column width (persisted; every open editor
    /// resizes live because they share this signal).
    pub fn set_column_width(&self, w: f32) {
        self.column_width.set(w);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// WelcomeViewModel — facade for the Welcome start screen.
//
// Store-backed like `SettingsViewModel` (owns only the `show_welcome` signal +
// the app handle), so the Welcome panel rebuilds it anywhere from
// `WelcomeViewModel::new(ctx.settings(), app_ctx)`. The business actions (open a
// recent/example work, pick a file, create a new work) live here, not in the
// view's `build()`.
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Clone)]
pub struct WelcomeViewModel {
    show_welcome: Signal<bool>,
    app_ctx: Rc<AppContext>,
}

#[allow(dead_code)]
impl WelcomeViewModel {
    pub fn new(store: &SettingsStore, app_ctx: Rc<AppContext>) -> Self {
        Self {
            show_welcome: store.signal(SHOW_WELCOME_KEY, true),
            app_ctx,
        }
    }

    /// The persisted "show at startup" signal — bound by the dialog's inline
    /// checkbox and the Settings toggle (same cached `SHOW_WELCOME_KEY` signal).
    pub fn show_welcome(&self) -> Signal<bool> {
        self.show_welcome.clone()
    }

    /// Open a recent/known work by path. Dismisses the modal first so the loaded
    /// work is revealed behind it (mirrors `RecentProjectsButton`'s row click).
    pub fn open_work(&self, path: String, ctx: &mut EventContext) {
        ctx.dismiss_modal();
        if let Err(e) =
            work_management_commands::load_work(&self.app_ctx, &LoadWorkDto { file_name: path })
        {
            ctx.show_toast(Toast::error(tr!(could_not_open_work(error = e.to_string()))));
        }
    }

    /// Open a bundled example. Its bytes are embedded in the binary; write them
    /// to a per-user temp copy (so the read-only repo original is never mutated
    /// or saved over) and load that.
    pub fn open_example(&self, file_name: &str, bytes: &[u8], ctx: &mut EventContext) {
        match write_temp_example(file_name, bytes) {
            Ok(path) => self.open_work(path, ctx),
            Err(e) => {
                ctx.show_toast(Toast::error(tr!(could_not_open_example(error = e.to_string()))));
            }
        }
    }

    /// "Open" button — native picker for an existing `.skrib`, then load.
    pub fn pick_open(&self, ctx: &mut EventContext) {
        let app_ctx = self.app_ctx.clone();
        let req = FileDialogRequest::pick_file()
            .title("Open Skribisto work")
            .add_filter("Skribisto work", &["skrib"]);
        let _ = ctx.pick_file(req, move |res, ectx| {
            if let FileDialogResult::File(Some(path)) = res {
                ectx.dismiss_modal();
                let file = path.to_string_lossy().into_owned();
                if let Err(e) =
                    work_management_commands::load_work(&app_ctx, &LoadWorkDto { file_name: file })
                {
                    ectx.show_toast(Toast::error(tr!(could_not_open_work(error = e.to_string()))));
                }
            }
        });
    }

    /// "New Work" button — native save picker for the target `.skrib`, then create.
    pub fn new_work(&self, ctx: &mut EventContext) {
        let app_ctx = self.app_ctx.clone();
        let req = FileDialogRequest::save_file()
            .title("Create a new Skribisto work")
            .default_file_name("Untitled.skrib")
            .add_filter("Skribisto work", &["skrib"]);
        let _ = ctx.save_file(req, move |res, ectx| {
            if let FileDialogResult::Saved(Some(path)) = res {
                ectx.dismiss_modal();
                let file = path.to_string_lossy().into_owned();
                if let Err(e) =
                    work_management_commands::new_work(&app_ctx, &NewWorkDto { file_name: file })
                {
                    ectx.show_toast(Toast::error(tr!(could_not_create_work(error = e.to_string()))));
                }
            }
        });
    }
}

/// Write an embedded example's bytes to a per-user temp dir (always rewritten,
/// so a stale/partial copy never blocks a fresh open) and return its path.
fn write_temp_example(file_name: &str, bytes: &[u8]) -> std::io::Result<String> {
    let mut dir = std::env::temp_dir();
    dir.push("skribisto-examples");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(file_name);
    std::fs::write(&path, bytes)?;
    Ok(path.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_show_default_on_and_persists() {
        // A real TOML store at a unique temp path (no in-memory store exists).
        let path = std::env::temp_dir()
            .join(format!("skribisto-welcome-test-{}.toml", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let store = SettingsStore::open(path.clone()).expect("open settings store");
        let app_ctx = Rc::new(AppContext::new());

        let vm = WelcomeViewModel::new(&store, app_ctx.clone());
        assert!(vm.show_welcome().get(), "defaults to on");

        vm.show_welcome().set(false);
        // A second facade over the same store observes the change (same cached
        // signal per key) — the store-backed-facade invariant.
        let vm2 = WelcomeViewModel::new(&store, app_ctx);
        assert!(!vm2.show_welcome().get(), "toggle persists across instances");

        let _ = std::fs::remove_file(&path);
    }

    fn editors() -> EditorsViewModel {
        EditorsViewModel::new(
            Rc::new(AppContext::new()),
            Signal::new(700.0),
            Signal::new(None),
        )
    }

    /// Push an editor tab directly (bypassing the backend) so tab-management
    /// logic can be tested without a loaded project.
    fn push_tab(p: &EditorsViewModel, item_id: u64) -> TabId {
        let id = TabId::fresh();
        let tab = tabs::tab_for(
            &p.app_ctx,
            item_id,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            p.column_width.clone(),
        );
        p.tabs.push(TabHandle::dynamic(
            id,
            "editor",
            TabInfo::new().closable(true),
            tab,
        ));
        id
    }

    #[test]
    fn open_or_focus_dedupes_an_already_open_tab() {
        let vm = editors();
        let id = push_tab(&vm, 42);
        assert_eq!(vm.tabs().len(), 1);
        // Already open → focuses it, no backend hit, no new tab.
        vm.open_or_focus(42, "Scene");
        assert_eq!(vm.tabs().len(), 1);
        assert_eq!(vm.selected_tab().get(), Some(id));
    }

    #[test]
    fn close_all_empties_and_clears_selection() {
        let vm = editors();
        push_tab(&vm, 1);
        push_tab(&vm, 2);
        vm.selected_tab().set(Some(TabId::fresh()));
        vm.close_all();
        assert_eq!(vm.tabs().len(), 0);
        assert_eq!(vm.selected_tab().get(), None);
    }

    #[test]
    fn outline_init_stack_creates_a_stack_id() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        assert!(outline.stack_id_signal().get().is_none());
        outline.init_stack();
        assert!(
            outline.stack_id_signal().get().is_some(),
            "init_stack opens the per-Work undo stack"
        );
    }

    #[test]
    fn outline_actions_on_empty_selection_are_noops() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        // No selection, no loaded Work — these must not panic and must do nothing.
        outline.trash_selected();
        outline.duplicate_selected();
        outline.indent_selected();
        outline.outdent_selected();
    }

    #[test]
    fn outline_show_hide_toggle_track_side_visibility() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        let visible = outline.is_visible();

        // Sides start hidden.
        assert!(!visible.get());

        outline.show();
        assert!(visible.get(), "show() makes the side visible");

        outline.toggle();
        assert!(!visible.get(), "toggle() hides a visible side");

        outline.toggle();
        assert!(visible.get(), "toggle() re-shows a hidden side");

        outline.hide();
        assert!(!visible.get(), "hide() hides the side");
    }
}
