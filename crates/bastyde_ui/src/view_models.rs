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
};

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands, content_commands,
    trash_management_commands, undo_redo_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::{CreateBinderItemDto, UpdateBinderDto, UpdateBinderItemDto};

use frontend::binder_item_management::{DuplicateDto, MoveDto, MovePlace};
use frontend::trash_management::{TrashBinderDto, TrashBinderItemsDto};

use crate::editor_tab::EditorTab;
use crate::models::{BinderBinderItemsTreeModel, BinderTreeKey, CommitMove};
use crate::{DARK_KEY, EDITOR_WIDTH_DEFAULT, EDITOR_WIDTH_KEY, LOCALE_KEY};

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
}

impl EditorsViewModel {
    pub fn new(app_ctx: Rc<AppContext>, column_width: Signal<f32>) -> Self {
        Self {
            app_ctx,
            tabs: ListModel::from_vec(Vec::new()),
            selected_tab: Signal::new(None),
            active_item: Signal::new(None),
            column_width,
        }
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
                    h.payload.downcast_ref::<EditorTab>().map(|e| e.item_id)
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

    /// Open the editor tab for `item_id`, or focus it if already open.
    pub fn open_or_focus(&self, item_id: u64, title: &str) {
        if let Some(tid) = self.find_open(item_id) {
            self.selected_tab.set(Some(tid));
            return;
        }
        let (main_md, synopsis_md) = self.load_markdown(item_id);
        let label = if title.is_empty() { "Untitled" } else { title };
        let id = TabId::fresh();
        self.tabs.push(TabHandle::dynamic(
            id,
            "editor",
            TabInfo::new().title(lit!(label.to_string())).closable(true),
            EditorTab::new(item_id, &main_md, &synopsis_md, self.column_width.clone()),
        ));
        self.selected_tab.set(Some(id));
    }

    /// Close every open tab (e.g. on project load).
    pub fn close_all(&self) {
        while self.tabs.len() > 0 {
            self.tabs.remove(0);
        }
        self.selected_tab.set(None);
    }

    /// `Some(tab id)` if an editor for `item_id` is already open.
    fn find_open(&self, item_id: u64) -> Option<TabId> {
        for i in 0..self.tabs.len() {
            let hit = self.tabs.with_item(i, |h| {
                h.payload
                    .downcast_ref::<EditorTab>()
                    .filter(|e| e.item_id == item_id)
                    .map(|_| h.id)
            });
            if let Some(Some(tid)) = hit {
                return Some(tid);
            }
        }
        None
    }

    /// Pull an item's main + synopsis Markdown out of its `Content` rows.
    fn load_markdown(&self, item_id: u64) -> (String, String) {
        let ctx = &*self.app_ctx;
        let content_ids = binder_item_commands::get_binder_item_relationship(
            ctx,
            &item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default();
        let contents = content_commands::get_content_multi(ctx, &content_ids).unwrap_or_default();

        let (mut main_md, mut synopsis_md) = (String::new(), String::new());
        for c in contents.into_iter().flatten() {
            match c.role {
                ContentRole::SceneText | ContentRole::NoteText => main_md = c.data,
                ContentRole::SynopsisText => synopsis_md = c.data,
                _ => {}
            }
        }
        (main_md, synopsis_md)
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
    /// Per-`Work` undo stack id, created on `LoadWork`. Every binder-tree
    /// mutation runs on this stack so one Ctrl+Z history reverses them all.
    stack_id: Signal<Option<u64>>,
}

// The visibility / reveal / action methods are the feature's public API
// (callable from menus, shortcuts, other view-models); wired incrementally, so
// not every one has a caller yet.
#[allow(dead_code)]
impl OutlineViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        model: BinderBinderItemsTreeModel,
        docking: DockingModel,
        dock_id: DockWidgetId,
    ) -> Self {
        let vm = Self {
            app_ctx,
            model,
            selection: KeyedSelectionModel::new(SelectionMode::Single),
            docking,
            dock_id,
            stack_id: Signal::new(None),
        };
        vm.install_reorder();
        vm
    }

    /// Build the outline view-model with its standard leading-dock presentation
    /// (280 px side + 48 px activity rail). The single place that knows the
    /// outline's dock geometry — so `App` and the title-bar menu share one
    /// handle without either re-stating layout constants.
    pub fn new_default(app_ctx: Rc<AppContext>) -> Self {
        let model = BinderBinderItemsTreeModel::new(app_ctx.clone());
        let docking = DockingModel::new();
        docking.set_side_size(DockSide::Leading, 280.0);
        // A non-zero rail thickness switches the leading side to Rail
        // presentation (a `DockActivityBar` icon rail); the layout sizes the
        // rail from the `DockRail` config.
        docking.set_side_rail(DockSide::Leading, 48.0);
        Self::new(app_ctx, model, docking, DockWidgetId::fresh())
    }

    /// Inject the model's drag-reorder closure. **Cycle-safety:** it captures
    /// only `app_ctx` + the `stack_id` signal and calls the free `apply_move`
    /// helper — NOT `self` (capturing the vm would form model → closure → vm →
    /// model, the `Rc` cycle the designer deliberately avoids).
    fn install_reorder(&self) {
        let app_ctx = self.app_ctx.clone();
        let stack_id = self.stack_id.clone();
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
        let id = undo_redo_commands::create_new_stack(&self.app_ctx);
        self.stack_id.set(Some(id));
    }
    fn stack(&self) -> Option<u64> {
        self.stack_id.get()
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
        let current = self
            .model
            .node_of(&key)
            .map(|(_, t)| t)
            .unwrap_or_default();
        let vm = self.clone();
        InputDialog::new(lit!("Rename"))
            .default_text(current)
            .on_result(move |result, _ctx| {
                if let Some(name) = result {
                    if !name.trim().is_empty() {
                        vm.rename(key, &name);
                    }
                }
            })
            .present(ctx);
    }

    /// Apply a rename to a binder (name) or item (title).
    pub fn rename(&self, key: BinderTreeKey, title: &str) {
        let ctx = &*self.app_ctx;
        match key {
            BinderTreeKey::Binder(b) => {
                if let Ok(Some(binder)) = binder_commands::get_binder(ctx, &b) {
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
                if let Ok(Some(it)) = binder_item_commands::get_binder_item(ctx, &i) {
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
                    &TrashBinderDto { binder_id: *b as i64 },
                );
            }
        }
        // Items, grouped by their origin binder.
        let mut by_binder: HashMap<u64, Vec<i64>> = HashMap::new();
        for key in sel {
            if let BinderTreeKey::Item(i) = key {
                if let Some(b) = self.model.binder_of(key) {
                    by_binder.entry(b).or_default().push(*i as i64);
                }
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
                let it = binder_item_commands::get_binder_item(ctx, &i).ok()??;
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
        let work = work_commands::get_all_work(ctx).ok()?.into_iter().next()?;
        work_commands::get_work_relationship(ctx, &work.id, &WorkRelationshipField::Binders)
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
            let Ok(Some(it)) = binder_item_commands::get_binder_item(ctx, &i) else {
                continue;
            };
            let new_indent = if delta > 0 {
                // Indent ≤ predecessor sibling's indent + 1 (can't indent the
                // first child of a level).
                let pred = if pos == 0 {
                    -1
                } else {
                    binder_item_commands::get_binder_item(ctx, &order[pos - 1])
                        .ok()
                        .flatten()
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
}

// Accessors/setters are the feature's public API; bound to widgets incrementally.
#[allow(dead_code)]
impl SettingsViewModel {
    pub fn new(store: &SettingsStore) -> Self {
        Self {
            dark: store.signal(DARK_KEY, false),
            locale: store.signal(LOCALE_KEY, "en-US".to_string()),
            column_width: store.signal(EDITOR_WIDTH_KEY, EDITOR_WIDTH_DEFAULT),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn editors() -> EditorsViewModel {
        EditorsViewModel::new(Rc::new(AppContext::new()), Signal::new(700.0))
    }

    /// Push an editor tab directly (bypassing the backend) so tab-management
    /// logic can be tested without a loaded project.
    fn push_tab(p: &EditorsViewModel, item_id: u64) -> TabId {
        let id = TabId::fresh();
        p.tabs.push(TabHandle::dynamic(
            id,
            "editor",
            TabInfo::new().closable(true),
            EditorTab::new(item_id, "", "", p.column_width.clone()),
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
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()));
        assert!(outline.stack_id.get().is_none());
        outline.init_stack();
        assert!(
            outline.stack_id.get().is_some(),
            "init_stack opens the per-Work undo stack"
        );
    }

    #[test]
    fn outline_actions_on_empty_selection_are_noops() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()));
        // No selection, no loaded Work — these must not panic and must do nothing.
        outline.trash_selected();
        outline.duplicate_selected();
        outline.indent_selected();
        outline.outdent_selected();
    }

    #[test]
    fn outline_show_hide_toggle_track_side_visibility() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()));
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
