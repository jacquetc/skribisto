//! The binder **outline** dock: the leading-side tree of the open work's binder
//! items, fronted by the switcher + search header, with per-row context menus
//! and key handling. [`outline_dock`] packages it as a `DockWidget` for `App` to
//! mount; everything below is the dock's private content builder.
//!
//! The dock stays decoupled from `EditorsViewModel`: `App` supplies an
//! [`OpenItemFn`] callback so activating a row opens its editor tab without the
//! tree importing the editors.

use std::rc::Rc;

use bastyde::data::TreeDataSource;
use bastyde::prelude::*;
use bastyde::widgets::{
    ActivateOn, DockOpenLocation, DockSide, DockWidget, Expand, FocusScope, HStack, MenuItem,
    MenuList, Padding, StandardTreeItem, TraversalScopePolicy, TreeRow, TreeView, VStack,
};

use frontend::AppContext;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

use crate::binder_switcher_button::{BinderSwitcherButton, binder_search_button};
use crate::models::{BinderTreeKey, TreeNode};
use crate::view_models::OutlineViewModel;

/// Callback App supplies to the binder tree to open (or focus) an item's editor
/// tab on activation — keeps the tree decoupled from `EditorsViewModel`.
pub type OpenItemFn = Rc<dyn Fn(u64, String)>;

/// Build the binder outline as a `DockWidget` for the leading side. `App` passes
/// in the shared `OutlineViewModel`, the app context, the open callback, and the
/// active-item signal that drives the "open document" marker.
pub fn outline_dock(
    outline: OutlineViewModel,
    app_ctx: Rc<AppContext>,
    on_open: OpenItemFn,
    active_item: Signal<Option<u64>>,
) -> DockWidget {
    let dock_id = outline.dock_id();
    DockWidget::new(dock_id, tr!(binder()), move |_id| {
        // Group the dock's Tab order: a Continue scope keeps the binder's
        // tab_index numbering from colliding with other docks/regions while
        // still letting Tab flow out at the ends.
        FocusScope::new(TraversalScopePolicy::Continue).child(binder_tree(
            outline.clone(),
            app_ctx.clone(),
            on_open.clone(),
            active_item.clone(),
        ))
    })
    .icon(crate::activity_icons::outline_icon)
    .default_location(DockOpenLocation::side(DockSide::Leading))
}

/// The binder-item tree shown in the leading dock, backed by the
/// `OutlineViewModel`'s `TreeDataSource` (so it drag-reorders). `StandardTreeItem`
/// gives the expand chevron and renders the user-note `label` as the subtitle.
/// Rows select (not expand) on click; selection is keyed by `BinderTreeKey`.
/// Each row carries a right-click context menu; the wrapping column handles
/// Delete / F2 / Tab / Shift-Tab.
fn binder_tree(
    outline: OutlineViewModel,
    app_ctx: Rc<AppContext>,
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
            // Leading icon chosen purely by sub_role (binder rows get the binder
            // glyph); tint follows the theme via `TextRole::Primary`.
            let mut icon = if node.kind == "binder" {
                crate::binder_icons::binder_icon()
            } else {
                crate::binder_icons::sub_role_icon(&node.sub_role)
            };
            // Persistent "open document" marker: the row whose item is the
            // active editor tab shows an accent title + icon — independent of
            // selection and focus, so you can always see what's open. Reactive
            // (no rebuild); the same signal drives both title and icon color.
            if let Some(item_id) = node.item_id {
                let title_color = active_item.map(move |a| {
                    if *a == Some(item_id) {
                        TextRole::Accent
                    } else {
                        TextRole::Primary
                    }
                });
                item = item.label_color(title_color.clone());
                icon = icon.color(title_color);
            }
            item = item.leading_slot(icon);
            let cm = menu_outline.clone();
            Box::new(item.context_menu(move |_pos, _ctx| {
                // Operate on the right-clicked row directly — do NOT mutate the
                // selection here: selecting rebuilds this row, destroying the
                // menu's anchor (the overlay would fall back to the corner).
                Some(Box::new(binder_context_menu(cm.clone(), key)) as Box<dyn Widget>)
            })) as Box<dyn Widget>
        },
    )
    // Adaptive row heights: each row measures to its content, so title-only
    // rows collapse to the single-line minimum (28) while rows carrying a
    // subtitle take the two-line height (44) — instead of every row paying the
    // uniform two-line cost. (A flat `item_height(40.0)` also clipped the 44px
    // subtitled rows.) The estimate seeds unrealized rows for scroll extent.
    .auto_item_height(28.0)
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

    // Header row above the tree: the binder switcher (fills) + the search
    // button. Both drive the OutlineViewModel's filter signals; the tree model
    // re-sources reactively.
    let header = Padding::symmetric(8.0, 6.0).child(
        HStack::new()
            .spacing(4.0)
            .child(Expand::horizontal().child(BinderSwitcherButton::new(outline.clone(), app_ctx)))
            .child(binder_search_button(outline.clone())),
    );

    let keys = outline.clone();
    VStack::new()
        .spacing(0.0)
        .child(header)
        .child(Expand::new().child(tree))
        .on_key(move |ev, ctx| match ev {
            WidgetEvent::KeyDown {
                key: Key::Delete, ..
            } => {
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
            WidgetEvent::KeyDown {
                key: Key::Character(']'),
                modifiers,
                ..
            } if modifiers.ctrl() => {
                keys.indent_selected();
                EventResponse::Handled
            }
            WidgetEvent::KeyDown {
                key: Key::Character('['),
                modifiers,
                ..
            } if modifiers.ctrl() => {
                keys.outdent_selected();
                EventResponse::Handled
            }
            _ => EventResponse::Ignored,
        })
}

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
///
/// Multi-select convention for the **batch** actions (duplicate / trash): a
/// right-click *inside* the current selection acts on the whole selection; a
/// right-click on a row *outside* it acts on just that row (and, per the call
/// site, without disturbing the selection). The single-target actions (new /
/// rename) always anchor on the clicked row.
fn binder_context_menu(outline: OutlineViewModel, key: BinderTreeKey) -> MenuList {
    let selected = outline.selection().selected_keys();
    let batch: Vec<BinderTreeKey> = if selected.contains(&key) {
        selected
    } else {
        vec![key]
    };

    let new_item = outline.clone();
    let new_folder = outline.clone();
    let rename = outline.clone();
    let duplicate = outline.clone();
    let dup_batch = batch.clone();
    let trash = outline;
    let trash_batch = batch;
    MenuList::new()
        .item(MenuItem::new(tr!(ctx_new_item())).on_activate_fn(move |_| {
            new_item.new_item_at(key, BinderItemRole::Item, BinderItemSubRole::Text)
        }))
        .item(
            MenuItem::new(tr!(ctx_new_folder())).on_activate_fn(move |_| {
                new_folder.new_item_at(key, BinderItemRole::Folder, BinderItemSubRole::None)
            }),
        )
        .separator()
        .item(
            MenuItem::new(tr!(ctx_rename()))
                .on_activate_fn(move |ctx| rename.begin_rename(key, ctx)),
        )
        .item(
            MenuItem::new(tr!(ctx_duplicate()))
                .on_activate_fn(move |_| duplicate.duplicate_keys(&dup_batch)),
        )
        .separator()
        .item(
            MenuItem::new(tr!(ctx_trash())).on_activate_fn(move |_| trash.trash_keys(&trash_batch)),
        )
}
