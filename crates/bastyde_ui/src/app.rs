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
use bastyde::data::{FlatEntry, KeyedSelectionModel, NodeId, TreeModel};
use bastyde::prelude::*;
use bastyde::settings::SettingsExt;
use bastyde::tokens::SurfaceRole::Hover;
use bastyde::widgets::{
    DockOpenLocation, DockRail, DockSide, DockWidget, DockingLayout, Divider, Expand, HStack,
    IconButtonSize, NotificationArchiveModel, NotificationCenterButton, Spacer, StandardTreeItem,
    StatusBar, TabBarVisibility, TabWidget, TreeView, VStack,
};

use frontend::AppContext;
use frontend::common::event::{Event, Origin, WorkManagementEvent};

use crate::editor_tab::{EditorTab, editor_pane};
use crate::intents::AppIntent;
use crate::models::TreeNode;
use crate::view_models::{EditorsViewModel, OutlineViewModel, SettingsViewModel};

pub struct App {
    app_ctx: Rc<AppContext>,
    /// The outline view-model is created in `main` (the title-bar menu needs a
    /// handle to it for the reactive checkmark) and shared with `App`.
    outline: OutlineViewModel,
    /// Created once on first build (its column-width signal needs `ctx.settings()`).
    editors: Option<EditorsViewModel>,
    root_child: Option<WidgetId>,
}

impl App {
    pub fn new(app_ctx: Rc<AppContext>, outline: OutlineViewModel) -> Self {
        Self {
            app_ctx,
            outline,
            editors: None,
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
        // ── Layer-B view-models: created once, then shared by clone ──────────
        let settings = SettingsViewModel::new(ctx.settings());

        let app_ctx = self.app_ctx.clone();
        let column_width = settings.column_width();
        let editors = self
            .editors
            .get_or_insert_with(|| EditorsViewModel::new(app_ctx, column_width))
            .clone();

        let outline = self.outline.clone();

        // ── App-global commands (the scriptable surface) ─────────────────────
        // Registered with `register_action_global` so they're reachable as a
        // dispatch fallback regardless of where the intent originates — the
        // title-bar menu (which renders in an overlay, NOT under `App`), a global
        // shortcut anchored at the root, or any content handler. A plain
        // `register_action` would only fire on `App`'s own source→root path,
        // which the chrome-fired menu never touches.
        ctx.register_shortcut_global(
            Shortcut::new("outline.toggle")
                .name("Toggle Outline")
                .primary(KeyStroke::ctrl(Key::B))
                .build(),
        );
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("outline.toggle").on_invoke(move |_i, _c| outline.toggle()),
            );
        }
        {
            let editors = editors.clone();
            ctx.register_action_global(Action::new("editor.open_item").on_invoke(move |i, _c| {
                if let Some(AppIntent::OpenItem { item_id, title }) = AppIntent::from_intent(i) {
                    editors.open_or_focus(*item_id, title);
                }
            }));
        }

        // On project load: rebuild the tree and drop now-stale editor tabs.
        {
            let outline = outline.clone();
            let editors = editors.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |_event: &Event| {
                    outline.reload();
                    editors.close_all();
                },
            );
        }

        // App mediates the two peer view-models: selecting a binder item opens
        // (or focuses) its editor tab. Neither view-model imports the other.
        {
            let outline_sel = outline.clone();
            let editors = editors.clone();
            ctx.effect(&outline.selection_signal(), move |sel: &HashSet<NodeId>| {
                for node in sel.iter() {
                    if let Some((Some(item_id), title)) = outline_sel.node_item(*node) {
                        editors.open_or_focus(item_id, &title);
                    }
                }
            });
        }

        // ── Center: dynamic editor tabs ──────────────────────────────────────
        let center = TabWidget::new(editors.selected_tab())
            .dynamic_tab::<EditorTab>("editor", |_handle, state| editor_pane(state))
            .dynamic_model(editors.tabs())
            .bar_visibility(TabBarVisibility::Always)
            .compact_bar()
            .selected_tab_background(SurfaceRole::Content)
            .hover_tab_background(Hover)
            .tab_dividers()
            .active_indicator(bastyde::widgets::TabIndicatorPosition::InnerEdge);

        // ── Leading dock: the binder tree, fronted by a VS Code-style activity
        //    bar (icon rail). The OutlineViewModel owns the DockingModel. ──────
        let docking = outline.docking();
        let dock_outline = outline.clone();
        let layout = DockingLayout::new(docking.clone())
            .rail(DockRail::new(DockSide::Leading).background(SurfaceRole::Main).divider())
            .center(center)
            .dock(
                DockWidget::new(outline.dock_id(), lit!("Binder"), move |_id| {
                    binder_tree(dock_outline.tree(), dock_outline.selection())
                })
                .closable(false)
                .default_location(DockOpenLocation::side(DockSide::Leading)),
            );
        outline.open_in_layout();

        // ── Status bar (thin) with the notification bell ─────────────────────
        let archive = ctx
            .app_state::<Rc<NotificationArchiveModel>>()
            .cloned()
            .expect("install_toast_default registers the notification archive");
        let status = StatusBar::new().background(SurfaceRole::Main).child(
            HStack::new().spacing(8.0).child(Spacer::new()).child(
                NotificationCenterButton::new(archive).size(IconButtonSize::Compact),
            ),
        );

        let root = ctx.add(
            VStack::new()
                .spacing(0.0)
                .child(Divider::new())
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

