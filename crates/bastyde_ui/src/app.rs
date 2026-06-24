//! The application body: a `DockingLayout` whose leading dock is the binder
//! tree and whose center is a `TabWidget` of editor tabs (Phase 3), with a thin
//! status bar underneath. The window chrome (custom `TitleBar` + hamburger menu)
//! lives at the window root in `main.rs`.
//!
//! Clicking a binder item opens (or focuses) its editor tab via the tree's
//! `KeyedSelectionModel<NodeId>` selection signal.
//!
//! Plain builder calls rather than `bati!`: the docking/tab/editor widgets are
//! generic over closures, which the DSL doesn't express cleanly. See
//! `settings_panel.rs` for the `bati!` style.

use std::collections::HashSet;
use std::rc::Rc;

use bastyde::core::widget::WidgetPlacement;
use bastyde::data::{FlatEntry, KeyedSelectionModel, ListModel, NodeId, SelectionMode, TreeModel};
use bastyde::prelude::*;
use bastyde::settings::SettingsExt;
use bastyde::widgets::{
    DockOpenLocation, DockRail, DockSide, DockWidget, DockWidgetId, DockingLayout, DockingModel,
    Expand, HStack, IconButtonSize, NotificationArchiveModel, NotificationCenterButton, Spacer,
    StandardTreeItem, StatusBar, TabBarVisibility, TabHandle, TabId, TabInfo, TabWidget, TreeView,
    VStack,
};

use frontend::AppContext;
use frontend::commands::{binder_item_commands, content_commands};
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::entities::ContentRole;
use frontend::common::event::{Event, Origin, WorkManagementEvent};

use crate::editor_tab::{EditorTab, editor_pane};
use crate::models::{BinderBinderItemsTreeModel, TreeNode};
use crate::{EDITOR_WIDTH_DEFAULT, EDITOR_WIDTH_KEY};

pub struct App {
    app_ctx: Rc<AppContext>,
    model: BinderBinderItemsTreeModel,
    tabs: ListModel<TabHandle>,
    selected_tab: Signal<Option<TabId>>,
    tree_selection: KeyedSelectionModel<NodeId>,
    root_child: Option<WidgetId>,
}

impl App {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        let model = BinderBinderItemsTreeModel::new(app_ctx.clone());
        Self {
            app_ctx,
            model,
            tabs: ListModel::from_vec(Vec::new()),
            selected_tab: Signal::new(None),
            tree_selection: KeyedSelectionModel::new(SelectionMode::Single),
            root_child: None,
        }
    }
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").finish()
    }
}

impl Widget for App {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // On project load: rebuild the tree and drop now-stale editor tabs.
        {
            let model = self.model.clone();
            let tabs = self.tabs.clone();
            let selected = self.selected_tab.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |_event: &Event| {
                    model.reload();
                    while tabs.len() > 0 {
                        tabs.remove(0);
                    }
                    selected.set(None);
                },
            );
        }

        // Centered-editor column width — a persisted, shared settings signal.
        let column_width = ctx.settings().signal(EDITOR_WIDTH_KEY, EDITOR_WIDTH_DEFAULT);

        // Selecting a binder item opens (or focuses) its editor tab.
        {
            let app_ctx = self.app_ctx.clone();
            let tabs = self.tabs.clone();
            let selected_tab = self.selected_tab.clone();
            let tree = self.model.tree().clone();
            let column_width = column_width.clone();
            ctx.effect(&self.tree_selection.selection_signal(), move |sel: &HashSet<NodeId>| {
                for node in sel.iter() {
                    if let Some((Some(item_id), title)) =
                        tree.with_item(*node, |n| (n.item_id, n.title.clone()))
                    {
                        open_item_tab(&app_ctx, &tabs, &selected_tab, item_id, &title, &column_width);
                    }
                }
            });
        }

        // ── Center: dynamic editor tabs ──────────────────────────────────────
        let center = TabWidget::new(self.selected_tab.clone())
            .dynamic_tab::<EditorTab>("editor", |_handle, state| editor_pane(state))
            .dynamic_model(self.tabs.clone())
            .bar_visibility(TabBarVisibility::Always);

        // ── Leading dock: the binder tree, fronted by a VS Code-style activity
        //    bar (icon rail) ──────────────────────────────────────────────────
        let docking = DockingModel::new();
        docking.set_side_size(DockSide::Leading, 280.0);
        // A non-zero rail thickness switches the leading side to Rail
        // presentation, so the side's tabs render as a `DockActivityBar` icon
        // rail; the layout sizes the rail itself from the `DockRail` config.
        docking.set_side_rail(DockSide::Leading, 48.0);
        let binder_dock = DockWidgetId::fresh();
        let tree_model = self.model.tree().clone();
        let tree_selection = self.tree_selection.clone();

        let layout = DockingLayout::new(docking.clone())
            .rail(DockRail::new(DockSide::Leading))
            .center(center)
            .dock(
                DockWidget::new(binder_dock, lit!("Binder"), move |_id| {
                    binder_tree(tree_model.clone(), tree_selection.clone())
                })
                .closable(false)
                .default_location(DockOpenLocation::side(DockSide::Leading)),
            );
        docking.open_dock(binder_dock, DockOpenLocation::side(DockSide::Leading));

        // ── Status bar (thin) with the notification bell ─────────────────────
        let archive = ctx
            .app_state::<Rc<NotificationArchiveModel>>()
            .cloned()
            .expect("install_toast_default registers the notification archive");
        let status = StatusBar::new().child(
            HStack::new().spacing(8.0).child(Spacer::new()).child(
                NotificationCenterButton::new(archive).size(IconButtonSize::Compact),
            ),
        );

        let root = ctx.add(
            VStack::new()
                .spacing(0.0)
                .child(Expand::new().child(layout))
                .child(status),
        );
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// The binder-item `TreeView` shown in the leading dock. `StandardTreeItem` gives
/// the expand chevron and renders the user-note `label` as the subtitle. Rows are
/// selected (not expanded) on click; selection is keyed by `NodeId` so the app
/// can resolve a clicked row back to its item.
fn binder_tree(
    tree_model: TreeModel<TreeNode>,
    selection: KeyedSelectionModel<NodeId>,
) -> TreeView<TreeNode> {
    TreeView::new_with_context(
        tree_model,
        |node: &TreeNode, entry: &FlatEntry, selected: bool, ctx| {
            let mut row = StandardTreeItem::new(lit!(node.title.clone()))
                .from_entry(entry)
                .selected(selected)
                .on_toggle_rc(ctx.toggle_callback());
            if !node.label.is_empty() {
                row = row.subtitle(lit!(node.label.clone()));
            }
            Box::new(row) as Box<dyn Widget>
        },
    )
    .item_height(40.0)
    .keyed_selection(selection)
    .row_click_expands(false)
}

/// Open the editor tab for `item_id`, or focus it if already open.
fn open_item_tab(
    ctx: &AppContext,
    tabs: &ListModel<TabHandle>,
    selected_tab: &Signal<Option<TabId>>,
    item_id: u64,
    title: &str,
    column_width: &Signal<f32>,
) {
    // Already open? Focus it.
    for i in 0..tabs.len() {
        let hit = tabs.with_item(i, |h| {
            h.payload
                .downcast_ref::<EditorTab>()
                .filter(|e| e.item_id == item_id)
                .map(|_| h.id)
        });
        if let Some(Some(tid)) = hit {
            selected_tab.set(Some(tid));
            return;
        }
    }

    // Otherwise load its content and open a new tab.
    let content_ids = binder_item_commands::get_binder_item_relationship(
        ctx,
        &item_id,
        &BinderItemRelationshipField::Contents,
    )
    .unwrap_or_default();
    let contents = content_commands::get_content_multi(ctx, &content_ids).unwrap_or_default();

    let mut main_md = String::new();
    let mut synopsis_md = String::new();
    for c in contents.into_iter().flatten() {
        match c.role {
            ContentRole::SceneText | ContentRole::NoteText => main_md = c.data,
            ContentRole::SynopsisText => synopsis_md = c.data,
            _ => {}
        }
    }

    let label = if title.is_empty() { "Untitled" } else { title };
    let id = TabId::fresh();
    tabs.push(TabHandle::dynamic(
        id,
        "editor",
        TabInfo::new().title(lit!(label.to_string())).closable(true),
        EditorTab::new(item_id, &main_md, &synopsis_md, column_width.clone()),
    ));
    selected_tab.set(Some(id));
}
