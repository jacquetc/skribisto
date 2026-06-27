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

use std::rc::Rc;

use bastyde::core::widget::WidgetPlacement;
use bastyde::data::TreeDataSource;
use bastyde::prelude::*;
use bastyde::settings::SettingsExt;
use bastyde::tokens::SurfaceRole::Hover;
use bastyde::widgets::{
    ActivateOn, DockOpenLocation, DockRail, DockSide, DockWidget, DockingLayout, Divider, Expand,
    FocusScope, HStack, IconButtonSize, MenuItem, MenuList, NotificationArchiveModel,
    NotificationCenterButton, Spacer, StandardTreeItem, StatusBar, TabBarVisibility, TabWidget,
    TraversalScopePolicy, TreeRow, TreeView, VStack,
};

use frontend::AppContext;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use frontend::common::event::{Event, Origin, WorkManagementEvent};

use crate::editor_tab::{EditorTab, editor_pane};
use crate::intents::AppIntent;
use crate::models::{BinderTreeKey, TreeNode};
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

        // ── Binder-tree commands (the scriptable surface for the outline). ───
        // Each drives an `OutlineViewModel` method; the context menu and key
        // handlers below also call these methods directly.
        {
            let outline = outline.clone();
            ctx.register_action_global(Action::new("binder.new_item").on_invoke(move |i, _c| {
                if let Some(AppIntent::NewItem { role, sub_role }) = AppIntent::from_intent(i) {
                    outline.new_item(role.clone(), sub_role.clone());
                }
            }));
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.rename").on_invoke(move |_i, c| outline.rename_selected(c)),
            );
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.duplicate")
                    .on_invoke(move |_i, _c| outline.duplicate_selected()),
            );
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.trash_selected")
                    .on_invoke(move |_i, _c| outline.trash_selected()),
            );
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.indent").on_invoke(move |_i, _c| outline.indent_selected()),
            );
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.outdent").on_invoke(move |_i, _c| outline.outdent_selected()),
            );
        }
        ctx.register_shortcut_global(
            Shortcut::new("binder.duplicate")
                .name("Duplicate")
                .primary(KeyStroke::ctrl(Key::D))
                .build(),
        );

        // On project load: open the per-Work undo stack, rebuild the tree and
        // drop now-stale editor tabs.
        {
            let outline = outline.clone();
            let editors = editors.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |_event: &Event| {
                    outline.init_stack();
                    outline.reload();
                    editors.close_all();
                },
            );
        }

        // App mediates the two peer view-models: *activating* a binder item
        // (click or Enter — NOT arrow navigation, which only moves the selection)
        // opens (or focuses) its editor tab. The tree fires this via
        // `TreeView::on_activate`; App supplies the open callback so neither
        // view-model imports the other.
        let on_open: OpenItemFn = {
            let editors = editors.clone();
            Rc::new(move |item_id, title| editors.open_or_focus(item_id, &title))
        };

        // Keep the "open document" id in sync with the active tab (open, close,
        // or a tab-bar click), so the binder's open-item marker tracks it.
        {
            let editors = editors.clone();
            ctx.effect(&editors.selected_tab(), move |_| editors.sync_active_item());
        }
        let active_item = editors.active_item();

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
                    // Group the dock's Tab order: a Continue scope keeps the
                    // binder's tab_index numbering from colliding with other
                    // docks/regions while still letting Tab flow out at the ends.
                    FocusScope::new(TraversalScopePolicy::Continue).child(binder_tree(
                        dock_outline.clone(),
                        on_open.clone(),
                        active_item.clone(),
                    ))
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

/// The binder-item tree shown in the leading dock, backed by the
/// `OutlineViewModel`'s `TreeDataSource` (so it drag-reorders). `StandardTreeItem`
/// gives the expand chevron and renders the user-note `label` as the subtitle.
/// Rows select (not expand) on click; selection is keyed by `BinderTreeKey`.
/// Each row carries a right-click context menu; the wrapping column handles
/// Delete / F2 / Tab / Shift-Tab.
fn binder_tree(
    outline: OutlineViewModel,
    on_open: OpenItemFn,
    active_item: Signal<Option<u64>>,
) -> impl Widget {
    let menu_outline = outline.clone();
    // Open on row *activation* (click or Enter), resolved from the flat index via
    // the source — NOT on selection, so arrow-key navigation only moves the
    // highlight and never spawns a tab.
    let activate_model = outline.model();
    let tree = TreeView::from_source_keyed(
        outline.model(),
        outline.selection(),
        move |node: &TreeNode, row: &TreeRow, selected: bool| {
            let key = key_of(node);
            let mut item = StandardTreeItem::new(lit!(node.title.clone()))
                .depth(row.depth)
                .has_children(row.has_children)
                .is_expanded(row.is_expanded)
                .selected(selected)
                .on_toggle_rc(row.toggle_callback());
            if !node.label.is_empty() {
                item = item.subtitle(lit!(node.label.clone()));
            }
            // Persistent "open document" marker: the row whose item is the
            // active editor tab shows an accent title — independent of selection
            // and focus, so you can always see what's open. Reactive (no rebuild).
            if let Some(item_id) = node.item_id {
                let title_color = active_item.map(move |a| {
                    if *a == Some(item_id) {
                        TextRole::Accent
                    } else {
                        TextRole::Primary
                    }
                });
                item = item.label_color(title_color);
            }
            let cm = menu_outline.clone();
            Box::new(item.context_menu(move |_pos, _ctx| {
                // Operate on the right-clicked row directly — do NOT mutate the
                // selection here: selecting rebuilds this row, destroying the
                // menu's anchor (the overlay would fall back to the corner).
                Some(Box::new(binder_context_menu(cm.clone(), key)) as Box<dyn Widget>)
            })) as Box<dyn Widget>
        },
    )
    .item_height(40.0)
    .row_click_expands(false)
    .reorderable(true)
    // Single-click to open (Scrivener convention) — arrow-key navigation only
    // moves the highlight, so stepping through the binder never spawns tabs.
    .activate_on(ActivateOn::SingleClick)
    .on_activate(move |idx| {
        if let Some(key) = activate_model.key_at(idx) {
            // Binder rows have `item_id == None` and don't open an editor.
            if let Some((Some(item_id), title)) = activate_model.node_of(&key) {
                on_open(item_id, title);
            }
        }
    });

    let keys = outline.clone();
    VStack::new()
        .spacing(0.0)
        .child(Expand::new().child(tree))
        .on_key(move |ev, ctx| match ev {
            WidgetEvent::KeyDown { key: Key::Delete, .. } => {
                keys.trash_selected();
                EventResponse::Handled
            }
            WidgetEvent::KeyDown { key: Key::F2, .. } => {
                keys.rename_selected(ctx);
                EventResponse::Handled
            }
            // Indent / outdent via Ctrl+] / Ctrl+[ (the macOS Notes / outliner
            // convention). Tab is deliberately NOT bound — it stays free for
            // focus traversal out of the tree, so the keyboard isn't trapped.
            WidgetEvent::KeyDown { key: Key::Character(']'), modifiers, .. }
                if modifiers.ctrl() =>
            {
                keys.indent_selected();
                EventResponse::Handled
            }
            WidgetEvent::KeyDown { key: Key::Character('['), modifiers, .. }
                if modifiers.ctrl() =>
            {
                keys.outdent_selected();
                EventResponse::Handled
            }
            _ => EventResponse::Ignored,
        })
}

/// Callback App supplies to the binder tree to open (or focus) an item's editor
/// tab on activation — keeps the tree decoupled from `EditorsViewModel`.
type OpenItemFn = Rc<dyn Fn(u64, String)>;

/// Reconstruct a row's `BinderTreeKey` from its `TreeNode` (the `from_source`
/// delegate gives the node + flat metadata, not the key).
fn key_of(node: &TreeNode) -> BinderTreeKey {
    if node.kind == "binder" {
        BinderTreeKey::Binder(node.binder_id.unwrap_or(0))
    } else {
        BinderTreeKey::Item(node.item_id.unwrap_or(0))
    }
}

/// The per-row context menu: create / rename / duplicate / trash. *New Folder*
/// is just `new_item(Folder, None)` — there is no separate folder command.
fn binder_context_menu(outline: OutlineViewModel, key: BinderTreeKey) -> MenuList {
    let new_item = outline.clone();
    let new_folder = outline.clone();
    let rename = outline.clone();
    let duplicate = outline.clone();
    let trash = outline;
    MenuList::new()
        .item(MenuItem::new(lit!("New Item")).on_activate_fn(move |_| {
            new_item.new_item_at(key, BinderItemRole::Item, BinderItemSubRole::Text)
        }))
        .item(MenuItem::new(lit!("New Folder")).on_activate_fn(move |_| {
            new_folder.new_item_at(key, BinderItemRole::Folder, BinderItemSubRole::None)
        }))
        .separator()
        .item(
            MenuItem::new(lit!("Rename"))
                .on_activate_fn(move |ctx| rename.begin_rename(key, ctx)),
        )
        .item(
            MenuItem::new(lit!("Duplicate"))
                .on_activate_fn(move |_| duplicate.duplicate_keys(&[key])),
        )
        .separator()
        .item(
            MenuItem::new(lit!("Move to Trash"))
                .on_activate_fn(move |_| trash.trash_keys(&[key])),
        )
}

