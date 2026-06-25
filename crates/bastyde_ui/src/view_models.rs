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

use std::collections::HashSet;
use std::rc::Rc;

use bastyde::data::{KeyedSelectionModel, ListModel, NodeId, SelectionMode, TreeModel};
use bastyde::prelude::*; // EventContext, Signal, intui, lit!
use bastyde::settings::SettingsStore;
use bastyde::widgets::{
    DockOpenLocation, DockSide, DockWidgetId, DockingModel, TabHandle, TabId, TabInfo,
};

use frontend::AppContext;
use frontend::commands::{binder_item_commands, content_commands};
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::entities::ContentRole;

use crate::editor_tab::EditorTab;
use crate::models::{BinderBinderItemsTreeModel, TreeNode};
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
    column_width: Signal<f32>,
}

impl EditorsViewModel {
    pub fn new(app_ctx: Rc<AppContext>, column_width: Signal<f32>) -> Self {
        Self {
            app_ctx,
            tabs: ListModel::from_vec(Vec::new()),
            selected_tab: Signal::new(None),
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
    model: BinderBinderItemsTreeModel,
    selection: KeyedSelectionModel<NodeId>,
    docking: DockingModel,
    dock_id: DockWidgetId,
}

// The visibility / reveal methods are the feature's public API (callable from
// menus, shortcuts, other view-models); they are wired to affordances
// incrementally, so not every one has a caller yet.
#[allow(dead_code)]
impl OutlineViewModel {
    pub fn new(
        model: BinderBinderItemsTreeModel,
        docking: DockingModel,
        dock_id: DockWidgetId,
    ) -> Self {
        Self {
            model,
            selection: KeyedSelectionModel::new(SelectionMode::Single),
            docking,
            dock_id,
        }
    }

    /// Build the outline view-model with its standard leading-dock presentation
    /// (280 px side + 48 px activity rail). The single place that knows the
    /// outline's dock geometry — so `App` and the title-bar menu share one
    /// handle without either re-stating layout constants.
    pub fn new_default(app_ctx: Rc<AppContext>) -> Self {
        let model = BinderBinderItemsTreeModel::new(app_ctx);
        let docking = DockingModel::new();
        docking.set_side_size(DockSide::Leading, 280.0);
        // A non-zero rail thickness switches the leading side to Rail
        // presentation (a `DockActivityBar` icon rail); the layout sizes the
        // rail from the `DockRail` config.
        docking.set_side_rail(DockSide::Leading, 48.0);
        Self::new(model, docking, DockWidgetId::fresh())
    }

    // ── handles for wiring the widgets / layout ──
    pub fn tree(&self) -> TreeModel<TreeNode> {
        self.model.tree().clone()
    }
    pub fn selection(&self) -> KeyedSelectionModel<NodeId> {
        self.selection.clone()
    }
    pub fn selection_signal(&self) -> Signal<HashSet<NodeId>> {
        self.selection.selection_signal()
    }
    pub fn docking(&self) -> DockingModel {
        self.docking.clone()
    }
    pub fn dock_id(&self) -> DockWidgetId {
        self.dock_id
    }

    /// Resolve a tree node to its `(item_id, title)`.
    pub fn node_item(&self, node: NodeId) -> Option<(Option<u64>, String)> {
        self.model
            .tree()
            .with_item(node, |n| (n.item_id, n.title.clone()))
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

    /// Rebuild the tree from the backend (e.g. on project load).
    pub fn reload(&self) {
        self.model.reload();
    }

    // ── domain op composed from mechanism + state ──

    /// Bring the outline forward AND select a row ("reveal in outline").
    pub fn reveal_item(&self, node: NodeId) {
        self.show();
        self.selection.select(node);
    }
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
