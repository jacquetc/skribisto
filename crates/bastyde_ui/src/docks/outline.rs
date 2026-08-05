// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The binder **outline** dock: the leading-side tree of the open work's binder
//! items, fronted by the switcher + search header, with per-row context menus
//! and key handling. [`outline_dock`] packages it as a `DockWidget` for `App` to
//! mount; everything below is the dock's private content builder.
//!
//! The dock stays decoupled from `EditorsViewModel`: `App` supplies an
//! [`OpenItemFn`] callback so activating a row opens its editor tab without the
//! tree importing the editors.

use std::rc::Rc;

use bastyde::core::widget::WidgetPlacement;
use bastyde::data::TreeDataSource;
use bastyde::prelude::*;
use bastyde::widgets::tooltip::TooltipContent;
use bastyde::widgets::{
    ActivateOn, Button, ButtonVariant, DockOpenLocation, DockSide, DockWidget, DragTransferMode,
    Expand, FocusScope, HStack, MenuItem, MenuList, MessageBox, MessageBoxButtons, Padding,
    ScrollBarMode, Spacer, StandardTreeItem, TextWidget, ToolbarItem, TraversalScopePolicy,
    TreeRow, TreeView, VStack,
};

use frontend::AppContext;

use skribisto_model::PromoteTarget;

use crate::binder::create_labels::{
    recommendation_label, recommendation_placement, recommendation_tooltip_key,
};
use crate::binder::switcher_button::{BinderSwitcherButton, binder_search_button};
use crate::docks::create_split_button::CreateSplitButton;
use crate::intents::AppIntent;
use crate::models::{BinderTreeKey, TreeNode};
use crate::view_models::OutlineViewModel;

/// Callback App supplies to the binder tree to open (or focus) an item's editor
/// tab on activation — keeps the tree decoupled from `EditorsViewModel`.
pub type OpenItemFn = Rc<dyn Fn(u64, String)>;

/// Id of the outline's **scoped** "Open to the Side" shortcut (Ctrl+Enter). Scoped
/// (not global) so it never shadows `RichTextEditor`'s own Ctrl+Enter (insert
/// block); the menu reads the translatable accelerator from it via `for_shortcut`.
const OPEN_TO_SIDE_SHORTCUT: &str = "outline.open_to_side";

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
    // A clone for the framework header's Create button (the content closure below
    // moves `outline`).
    let header_outline = outline.clone();
    DockWidget::new(dock_id, tr!(binder()), move |_id| {
        // Group the dock's Tab order: a Continue scope keeps the binder's
        // tab_index numbering from colliding with other docks/regions while
        // still letting Tab flow out at the ends. `OutlineKeys` wraps it as the
        // dock-content root so its scoped Ctrl+Enter shortcut covers the tree's
        // focus (and nothing outside it).
        OutlineKeys::new(
            outline.clone(),
            FocusScope::new(TraversalScopePolicy::Continue).child(binder_tree(
                outline.clone(),
                app_ctx.clone(),
                on_open.clone(),
                active_item.clone(),
            )),
        )
    })
    .icon(crate::icons::activity::outline_icon)
    // Show the sole-pane dock's header bar (title + actions) and pin the
    // context-dependent "Create" SplitButton into it as a custom toolbar item.
    .show_header(true)
    .header_actions(move |_id| {
        vec![ToolbarItem::custom(CreateSplitButton::new(
            header_outline.clone(),
        ))]
    })
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
            // A row whose title has been cleared is named by its ordinal instead, exactly
            // as the export names it — and then the badge is dropped, because "3. Chapter 3"
            // is the duplication this whole feature exists to remove.
            let (label, badge) = crate::models::label_and_badge(
                &node.title,
                node.fallback_label.as_deref(),
                node.number,
            );
            let mut item = StandardTreeItem::new(lit!(label))
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
                crate::binder::icons::binder_icon()
            } else {
                crate::binder::icons::kind_sub_role_icon(&node.kind, &node.sub_role)
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
            // The chapter's ordinal, between the icon and the title — which is exactly what
            // `center_slot` is for ("leading is the row's icon identity, center is
            // label-adjacent decoration"). A separate slot, never spliced into the title
            // string: the text filter below matches `node.title`, and every inline rename
            // seeds its buffer from it.
            if badge.is_some() {
                item = item.center_slot(crate::widgets::StructureNumber::new(badge));
            }
            let cm = menu_outline.clone();
            // Middle-click opens the item to the side (only for item rows).
            let mid = node.item_id.map(|id| (id, node.title.clone()));
            Box::new(
                item.context_menu(move |_pos, _ctx| {
                    // Operate on the right-clicked row directly — do NOT mutate the
                    // selection here: selecting rebuilds this row, destroying the
                    // menu's anchor (the overlay would fall back to the corner).
                    Some(Box::new(binder_context_menu(cm.clone(), key)) as Box<dyn Widget>)
                })
                // `on_pointer_event` (not `accept_tap_buttons`) so this never
                // builds a tap gesture arena that would swallow the tree's own
                // primary-click activation; we consume only the middle button.
                .on_pointer_event(move |ev, ctx| {
                    if let WidgetEvent::PointerDown {
                        button: PointerButton::Middle,
                        ..
                    } = ev
                        && let Some((item_id, title)) = &mid
                    {
                        ctx.send_intent(AppIntent::OpenItemToSide {
                            item_id: *item_id,
                            title: title.clone(),
                        });
                        return EventResponse::Handled;
                    }
                    EventResponse::Ignored
                }),
            ) as Box<dyn Widget>
        },
    )
    // Adaptive row heights: each row measures to its content, so title-only
    // rows collapse to the single-line minimum (28) while rows carrying a
    // subtitle take the two-line height (44) — instead of every row paying the
    // uniform two-line cost. (A flat `item_height(40.0)` also clipped the 44px
    // subtitled rows.) The estimate seeds unrealized rows for scroll extent.
    .auto_item_height(28.0)
    // The dock is narrow, so the default `Permanent` bar (a 12px gutter column
    // stolen from every row) is too costly here: `Overlay` floats a thin resting
    // indicator over the content and expands it to the full track on hover.
    .scroll_bar_style(ScrollBarMode::Overlay)
    .row_click_expands(false)
    .reorderable(true)
    // Rows are also draggable OUT of the tree onto an editor pane (which opens
    // the item). `Copy` leaves the row in place; in-tree reorder still works —
    // one drag can drop inside the tree (reorder) or onto a pane (open).
    .exportable(DragTransferMode::Copy)
    // Single-click to open (Scrivener convention) — arrow-key navigation only
    // moves the highlight, so stepping through the binder never spawns tabs.
    .activate_on(ActivateOn::SingleClick)
    .on_activate(move |idx, _ctx| {
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
        // Says what the filter is doing, and offers the way out. Only rendered while a
        // filter is active; see `BinderFilterBar`.
        .child(BinderFilterBar::new(outline.clone()))
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
            // Ctrl+Enter ("Open to the Side") is a scoped `Shortcut` registered by
            // `OutlineKeys`, not handled here — so the menu can show its
            // translatable accelerator via `for_shortcut`.
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

/// Reconstruct a row's `BinderTreeKey` from its `TreeNode` (the `from_source` delegate
/// gives the node + flat metadata, not the key).
///
/// The node carries its own `uid` precisely so this stays a field read rather than a tree
/// lookup — the delegate runs for every realized row on every rebuild.
fn key_of(node: &TreeNode) -> BinderTreeKey {
    if node.kind == "binder" {
        BinderTreeKey::Binder(node.uid)
    } else {
        BinderTreeKey::Item(node.uid)
    }
}

/// The per-row context menu: ("Open to the Side" for item rows) / Add ▸ / rename
/// / duplicate / trash. The "Add ▸" submenu holds the recommended new-item types
/// for the row (see [`add_recommendations_menu`]).
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
    // Item rows (not binder roots) can open to the side.
    let open_side = outline
        .node_item(key)
        .and_then(|(id, title)| id.map(|item_id| (item_id, title)));

    let add_outline = outline.clone();
    let rename = outline.clone();
    let duplicate = outline.clone();
    let dup_batch = batch.clone();
    let trash = outline.clone();
    let trash_batch = batch;
    let mut menu = MenuList::new();
    if let Some((item_id, title)) = open_side {
        menu = menu
            .item(
                MenuItem::new(tr!(ctx_open_to_side()))
                    .for_shortcut(OPEN_TO_SIDE_SHORTCUT)
                    .on_activate_fn(move |ctx| {
                        ctx.send_intent(AppIntent::OpenItemToSide {
                            item_id,
                            title: title.clone(),
                        })
                    }),
            )
            .separator();
    }
    menu = menu
        // Context-dependent "Add ▸" submenu: the recommended new-item types for
        // this row, in recommended order, each with a rich tooltip. Mirrors the
        // header "Create" SplitButton but anchored on the right-clicked row.
        // (Replaces the old generic New Item / New Folder entries.)
        .item(MenuItem::submenu(tr!(ctx_add()), move || {
            Box::new(add_recommendations_menu(add_outline.clone(), key)) as Box<dyn Widget>
        }));

    // "Convert to ▸" — every type this item may become. A folder can become any other
    // kind of folder, so this is a submenu, not a single paired toggle.
    let targets = outline.promote_targets_of(key);
    if !targets.is_empty() {
        let promote_vm = outline.clone();
        menu = menu
            .separator()
            .item(MenuItem::submenu(tr!(ctx_promote()), move || {
                Box::new(promote_menu(promote_vm.clone(), key)) as Box<dyn Widget>
            }));
    }

    // The prologue lever, on the row itself. It is in the Inspector too, but a writer with
    // a prologue has no reason to go looking under Numbering — and renaming a chapter
    // "Prologue" pointedly does *not* un-number it, a trap other tools document.
    //
    // Labelled by what activating it does, rather than shown checked: `MenuItem` has no
    // check affordance, and "Number this chapter" / "Do not number this chapter" says the
    // outcome without needing one.
    if let Some(is_numbered) = outline.numbering_state(key) {
        let vm = outline.clone();
        menu = menu.separator().item(
            MenuItem::new(if is_numbered {
                tr!(ctx_unnumber())
            } else {
                tr!(ctx_number())
            })
            .rich_tooltip_content(
                TooltipContent::new("inspector.numbered", tr!(inspector_numbered_tip()))
                    .with_more(tr!(inspector_numbered_tip_more())),
            )
            .on_activate_fn(move |_| vm.set_numbered(key, !is_numbered)),
        );
    }

    menu.separator()
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

/// The "Convert to ▸" submenu: every type `key` may become, each behind the guards in
/// [`promote_with_guard`]. Shared by the outline context menu and the Inspector.
pub fn promote_menu(outline: OutlineViewModel, key: BinderTreeKey) -> MenuList {
    let mut menu = MenuList::new();
    for target in outline.promote_targets_of(key) {
        let (_, sub_role) = target.combo();
        let vm = outline.clone();
        menu = menu.item(
            MenuItem::new(crate::binder::create_labels::promote_target_label(target))
                .icon(crate::binder::icons::sub_role_icon(&sub_role))
                .rich_tooltip(crate::binder::create_labels::promote_target_tooltip_key(
                    target,
                ))
                .on_activate_fn(move |ctx| promote_with_guard(&vm, key, target, ctx)),
        );
    }
    menu
}

/// Convert `key` into `target`, behind two guards:
///
/// * a container becoming a leaf must be **empty** (a chapter folder still holding
///   scenes cannot collapse into a single flat chapter);
/// * the target must have somewhere to keep the item's text (a chapter holding prose
///   cannot become a Part, which has no prose at all).
///
/// Both are refused with an explanation rather than silently discarding anything. The
/// use case enforces the same invariants, so this is presentation, not protection.
pub fn promote_with_guard(
    outline: &OutlineViewModel,
    key: BinderTreeKey,
    target: PromoteTarget,
    ctx: &mut EventContext,
) {
    let blocked = outline.demote_blocked_children(key, target);
    if blocked > 0 {
        MessageBox::warning(tr!(promote_blocked_title()))
            .text(tr!(promote_blocked_text(count = blocked.to_string())))
            .buttons(MessageBoxButtons::Ok)
            .present(ctx);
        return;
    }
    let lost = outline.promote_content_loss(key, target);
    if !lost.is_empty() {
        let kinds = lost
            .iter()
            .map(|r| crate::binder::create_labels::content_role_label(r).resolve_now())
            .collect::<Vec<_>>()
            .join(", ");
        MessageBox::warning(tr!(promote_lossy_title()))
            .text(tr!(promote_lossy_text(
                target = crate::binder::create_labels::promote_target_label(target).resolve_now(),
                kinds = kinds
            )))
            .buttons(MessageBoxButtons::Ok)
            .present(ctx);
        return;
    }
    outline.promote(key, target);
}

/// The "Add ▸" submenu content: the recommended new-item types for `key`, in
/// recommended order, each with a rich tooltip. Fires `add_recommended` on the
/// outline directly (row-anchored — mirrors `new_item_at`, not via an intent).
fn add_recommendations_menu(outline: OutlineViewModel, key: BinderTreeKey) -> MenuList {
    let recs = outline.recommendations_for_key(Some(key));
    // Anchor title for the tooltips — `Some` only for a real item row.
    let anchor_title = outline
        .node_item(key)
        .and_then(|(item_id, title)| item_id.map(|_| title));
    let mut menu = MenuList::new();
    for rec in &recs {
        let vm = outline.clone();
        let rec_owned = *rec;
        // Trailing "where it lands" hint + the shared registered type explainer
        // (identical to the header SplitButton row).
        let placement = recommendation_placement(anchor_title.as_deref(), rec.relation);
        menu = menu.item(
            MenuItem::new(recommendation_label(rec.create_type))
                .icon(crate::binder::icons::create_type_icon(rec.create_type))
                .trailing_hint(placement)
                .rich_tooltip(recommendation_tooltip_key(rec.create_type))
                .on_activate_fn(move |_| vm.add_recommended(Some(key), &rec_owned)),
        );
    }
    menu
}

/// Dock-content root that registers the outline's **scoped** Ctrl+Enter shortcut
/// ("Open to the Side") on build. Because it's an ancestor of the tree, the
/// shortcut fires only while the tree has focus — never shadowing the editor's
/// own Ctrl+Enter (insert block). Otherwise a transparent single-child
/// pass-through (fills its bounds with the wrapped content).
struct OutlineKeys {
    outline: OutlineViewModel,
    child_id: Option<WidgetId>,
    pending: Option<Box<dyn Widget>>,
}

impl OutlineKeys {
    fn new(outline: OutlineViewModel, child: impl Widget + 'static) -> Self {
        Self {
            outline,
            child_id: None,
            pending: Some(Box::new(child)),
        }
    }
}

impl std::fmt::Debug for OutlineKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OutlineKeys").finish()
    }
}

impl Widget for OutlineKeys {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        ctx.register_shortcut(
            Shortcut::new(OPEN_TO_SIDE_SHORTCUT)
                .name("Open to the Side")
                .primary(KeyStroke::new(Key::Enter, Modifiers::CTRL))
                .build(),
        );
        let vm = self.outline.clone();
        ctx.register_action(
            Action::new(OPEN_TO_SIDE_SHORTCUT).on_invoke(move |_i, ctx| {
                if let Some((item_id, title)) = vm.selected_item() {
                    ctx.send_intent(AppIntent::OpenItemToSide { item_id, title });
                }
            }),
        );
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

/// The binder's filter feedback: how much the search is hiding, and a way to stop.
///
/// The search field lives in a popover, so dismissing it leaves a live filter with nothing
/// on screen to show for it. A writer who searched for something that matches nothing sees
/// an empty binder and an idle-looking magnifier — indistinguishable from a project with no
/// rows in it, and there is no affordance to undo it because the field is gone.
///
/// So the dock says it itself: a count while the filter is narrowing anything, and — when it
/// has hidden *everything* — the query it is filtering by, so the writer can see what to
/// clear. Both collapse to nothing when no filter is active, which is the normal case.
struct BinderFilterBar {
    outline: OutlineViewModel,
    root: Option<WidgetId>,
}

impl BinderFilterBar {
    fn new(outline: OutlineViewModel) -> Self {
        Self {
            outline,
            root: None,
        }
    }
}

impl std::fmt::Debug for BinderFilterBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinderFilterBar").finish_non_exhaustive()
    }
}

impl Widget for BinderFilterBar {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        use bastyde::core::BindingLevel;

        // Both inputs rebuild this: the query decides whether the bar exists at all, the
        // counts decide what it says.
        self.outline.search_query_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.outline.match_counts_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        let query = self.outline.search_query_signal().get();
        if query.trim().is_empty() {
            self.root = None;
            return Vec::new();
        }

        let (shown, total) = self.outline.match_counts_signal().get();
        let clearer = self.outline.clone();
        let mut col = VStack::new().spacing(2.0).child(
            HStack::new()
                .spacing(6.0)
                .child(
                    TextWidget::new(tr!(binder_filter_count(
                        shown = shown as i64,
                        total = total as i64
                    )))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
                )
                .child(Spacer::new())
                .child(
                    Button::new(tr!(binder_filter_clear()))
                        .variant(ButtonVariant::Plain)
                        .on_activate_fn(move |_| clearer.clear_search()),
                ),
        );

        // Naming the query matters most in exactly the case where the tree is blank: it is
        // the only thing left on screen that explains the blankness.
        if shown == 0 {
            col = col.child(
                TextWidget::new(tr!(binder_filter_none(query = query.clone())))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            );
        }

        let id = ctx.add(Padding::symmetric(8.0, 4.0).child(col));
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // No filter, no bar — and no height, so the tree keeps the whole dock.
        self.root
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
            child.origin = Point::new(bounds.x, bounds.y);
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}
