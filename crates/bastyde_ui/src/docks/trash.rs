// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Trash** dock (leading side, third rail tab): a tree of trashed roots —
//! one row per `TrashInfo` — each disclosing a read-only preview of its cascaded
//! descendants. Rows are **openable** (a trashed item opens in an editor with a
//! warning banner + accent), and carry a context menu (Restore / Restore to… /
//! Delete Forever). A header button empties the whole trash.
//!
//! [`trash_dock`] packages it as a `DockWidget` for `App` to mount; business
//! logic lives in [`TrashViewModel`]. `App` supplies the same [`OpenItemFn`] the
//! outline uses, so activation opens the item without the dock importing the
//! editors.

use bastyde::core::widget::WidgetPlacement;
use bastyde::data::TreeDataSource;
use bastyde::prelude::*;
use bastyde::widgets::{
    ActivateOn, Button, ButtonVariant, Divider, DockOpenLocation, DockSide, DockWidget,
    DockWidgetId, Expand, FocusScope, MenuItem, MenuList, Padding, ScrollBarMode, StandardTreeItem,
    Switcher, TextWidget, TraversalScopePolicy, TreeRow, TreeView, VStack,
};

use crate::docks::outline::OpenItemFn;
use crate::models::{TrashNode, TrashTreeKey};
use crate::view_models::TrashViewModel;

/// Build the trash panel as a leading-side `DockWidget`.
pub fn trash_dock(trash: TrashViewModel, dock_id: DockWidgetId, on_open: OpenItemFn) -> DockWidget {
    DockWidget::new(dock_id, tr!(trash_title()), move |_id| {
        TrashDockRoot::new(
            trash.clone(),
            FocusScope::new(TraversalScopePolicy::Continue)
                .child(trash_panel(trash.clone(), on_open.clone())),
        )
    })
    .icon(crate::activity_icons::trash_icon)
    .show_header(true)
    .default_location(DockOpenLocation::side(DockSide::Leading))
}

/// The panel: a header with the "Empty Trash…" button over the trash tree (or an
/// empty-state placeholder).
fn trash_panel(trash: TrashViewModel, on_open: OpenItemFn) -> impl Widget {
    let has_entries = trash.model().has_entries_signal();

    let header = {
        let vm = trash.clone();
        Padding::symmetric(8.0, 8.0).child(
            Button::new(tr!(trash_empty_button()))
                .variant(ButtonVariant::Plain)
                .enabled(has_entries.clone())
                .on_activate_fn(move |ctx| vm.confirm_empty_trash(ctx)),
        )
    };

    let tree = trash_tree(trash.clone(), on_open);

    // Empty-state ↔ tree.
    let body = Switcher::new(has_entries.map(|b| usize::from(*b)))
        .child(
            Padding::symmetric(16.0, 24.0).child(
                TextWidget::new(tr!(trash_empty_state()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            ),
        )
        .child(tree);

    VStack::new()
        .spacing(0.0)
        .child(header)
        .child(Divider::new())
        .child(Expand::new().child(body))
}

fn trash_tree(trash: TrashViewModel, on_open: OpenItemFn) -> impl Widget {
    let menu_vm = trash.clone();
    let activate_model = trash.model();
    TreeView::from_source_keyed(
        trash.model(),
        trash.selection(),
        move |node: &TrashNode, row: &TreeRow, selected: bool| {
            let mut item = StandardTreeItem::new(lit!(node.title.clone()))
                .depth(row.depth)
                .has_children(row.has_children)
                .is_expanded(row.is_expanded)
                .selected(selected)
                .on_toggle_rc(row.toggle_callback());
            if !node.label.is_empty() {
                item = item.subtitle(lit!(node.label.clone()));
            }
            let icon = if node.is_root && node.item_id.is_none() {
                crate::binder_icons::binder_icon() // whole-binder root
            } else {
                crate::binder_icons::sub_role_icon(&node.sub_role)
            };
            item = item.leading_slot(icon);
            // Cascade (descendant) rows read as a dimmed, non-actionable preview.
            if !node.is_root {
                item = item.label_color(TextRole::Secondary);
            }
            let cm = menu_vm.clone();
            let key = key_of(node);
            Box::new(item.context_menu(move |_pos, _ctx| {
                Some(Box::new(trash_context_menu(cm.clone(), key)) as Box<dyn Widget>)
            })) as Box<dyn Widget>
        },
    )
    .auto_item_height(28.0)
    .scroll_bar_style(ScrollBarMode::Overlay)
    .row_click_expands(false)
    .activate_on(ActivateOn::SingleClick)
    .on_activate(move |idx| {
        if let Some(key) = activate_model.key_at(idx)
            && let Some(item_id) = activate_model.item_id_of(key)
        {
            let title = activate_model.title_of(key).unwrap_or_default();
            on_open(item_id, title);
        }
    })
}

/// Reconstruct a row's [`TrashTreeKey`] from its node (the delegate gives the node
/// + flat metadata, not the key): a root is keyed by its `TrashInfo` id, a
/// descendant by its `BinderItem` id.
fn key_of(node: &TrashNode) -> TrashTreeKey {
    if node.is_root {
        TrashTreeKey::Root(node.trash_info_id.unwrap_or(0))
    } else {
        TrashTreeKey::Descendant(node.item_id.unwrap_or(0))
    }
}

/// Per-row context menu. Root rows: Restore / Restore to… (item roots only) /
/// Delete Forever. Descendant rows: Restore to… (peel it out).
fn trash_context_menu(trash: TrashViewModel, key: TrashTreeKey) -> MenuList {
    match key {
        TrashTreeKey::Root(id) => {
            let selected = trash.selected_roots();
            let batch: Vec<u64> = if selected.contains(&id) {
                selected
            } else {
                vec![id]
            };
            let mut menu = MenuList::new().item({
                let t = trash.clone();
                let b = batch.clone();
                MenuItem::new(tr!(trash_restore())).on_activate_fn(move |ctx| t.restore(ctx, &b))
            });
            if batch.len() == 1
                && trash.is_item_root(id)
                && let Some(item_id) = trash.model().item_id_of(TrashTreeKey::Root(id))
            {
                let t = trash.clone();
                menu = menu.item(
                    MenuItem::new(tr!(trash_restore_to()))
                        .on_activate_fn(move |ctx| t.restore_item(ctx, item_id)),
                );
            }
            menu.separator().item({
                let t = trash.clone();
                MenuItem::new(tr!(trash_delete_forever()))
                    .on_activate_fn(move |ctx| t.confirm_delete_forever(ctx, &batch))
            })
        }
        TrashTreeKey::Descendant(item_id) => MenuList::new().item({
            let t = trash.clone();
            MenuItem::new(tr!(trash_restore_to()))
                .on_activate_fn(move |ctx| t.restore_item(ctx, item_id))
        }),
    }
}

/// Dock-content root: wires the trash tree model on build (it needs a
/// `BuildContext`). Otherwise a transparent single-child pass-through.
struct TrashDockRoot {
    trash: TrashViewModel,
    child_id: Option<WidgetId>,
    pending: Option<Box<dyn Widget>>,
}

impl TrashDockRoot {
    fn new(trash: TrashViewModel, child: impl Widget + 'static) -> Self {
        Self {
            trash,
            child_id: None,
            pending: Some(Box::new(child)),
        }
    }
}

impl std::fmt::Debug for TrashDockRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TrashDockRoot").finish()
    }
}

impl Widget for TrashDockRoot {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.trash.wire(ctx);
        if let Some(w) = self.pending.take() {
            self.child_id = Some(ctx.add_boxed(w));
        }
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        match self.child_id.and_then(|id| ctx.child_size(id, proposal)) {
            Some(size) => size.into(),
            None => proposal.resolve(0.0, 0.0).into(),
        }
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
        self.child_id.into_iter().collect()
    }
}
