// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Restore to…** modal: pick a destination for a single trashed item.
//!
//! Used whenever a trashed item must land somewhere other than where it sat — a
//! descendant peeled out of a still-trashed subtree, a child of a wholly-trashed
//! binder, or a plain restore that came back `orphaned`. It shows the live binder
//! tree (a fresh [`BinderBinderItemsTreeModel`], single-select) — which already
//! excludes trashed rows, so the item can never pick its own trashed context —
//! and, on confirm, calls [`TrashViewModel::restore_to`].
//!
//! Chrome mirrors [`crate::backup::list_panel`]; the tree wiring mirrors
//! [`crate::docks::outline`], minus everything interactive (no reorder, no
//! context menu, no editor-open on activation).

use std::rc::Rc;

use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::core::styles::PanelVariant;
use bastyde::data::{KeyedSelectionModel, SelectionMode, TreeDataSource};
use bastyde::prelude::TextStyleRole;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, MessageBox,
    MessageBoxButtons, Padding, Panel, ScrollBarMode, Spacer, StandardButton, StandardTreeItem,
    Switcher, TextWidget, Toast, TreeRow, TreeView, VStack,
};

use frontend::trash_management::DropPosition;

use crate::models::{BinderBinderItemsTreeModel, BinderTreeKey, TreeFilters, TreeNode};
use crate::toast_scope::ToastWorkExt;
use crate::view_models::TrashViewModel;

const CARD_W: f32 = 560.0;
const CARD_H: f32 = 520.0;

/// Present the destination picker for `item_id` over the current window.
/// `on_done` fires after the modal closes (restore *or* cancel) — the orphan
/// chain uses it to advance to the next item.
pub fn present_trash_restore_target(
    ctx: &mut EventContext,
    trash: TrashViewModel,
    item_id: u64,
    entry_title: String,
    on_done: Rc<dyn Fn(&mut EventContext)>,
) {
    ctx.present_modal(
        ModalRequest::deferred(move |t| {
            t.add(TrashRestoreTargetPanel::new(
                trash.clone(),
                item_id,
                entry_title.clone(),
                on_done.clone(),
            ))
        })
        .presentation(ModalPresentation::InTree)
        // Manual close (buttons only): an Esc/click-outside dismissal would not run
        // the Cancel/Restore handlers and would strand the orphan-restore queue.
        .close_behavior(ModalCloseBehavior::Manual)
        .title(tr!(trash_restore_picker_title()))
        .size(CARD_W as u32, CARD_H as u32),
    );
}

pub struct TrashRestoreTargetPanel {
    trash: TrashViewModel,
    item_id: u64,
    entry_title: String,
    picker_model: BinderBinderItemsTreeModel,
    picker_selection: KeyedSelectionModel<BinderTreeKey>,
    on_done: Rc<dyn Fn(&mut EventContext)>,
    root_child: Option<WidgetId>,
}

impl TrashRestoreTargetPanel {
    pub fn new(
        trash: TrashViewModel,
        item_id: u64,
        entry_title: String,
        on_done: Rc<dyn Fn(&mut EventContext)>,
    ) -> Self {
        let filters = TreeFilters {
            binder: Signal::new(None),
            query: Signal::new(String::new()),
            match_counts: Signal::new((0, 0)),
            all_binders: Signal::new(false),
        };
        let picker_model =
            BinderBinderItemsTreeModel::new(trash.app_ctx(), trash.work_id(), filters);
        Self {
            trash,
            item_id,
            entry_title,
            picker_model,
            picker_selection: KeyedSelectionModel::new(SelectionMode::Single),
            on_done,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for TrashRestoreTargetPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TrashRestoreTargetPanel")
            .field("item_id", &self.item_id)
            .finish()
    }
}

impl Widget for TrashRestoreTargetPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Keep the tree live while the modal is open.
        self.picker_model.wire(ctx);

        // The destination tree — read-only, single-select (no reorder / context
        // menu / editor-open).
        let tree = TreeView::from_source_keyed(
            self.picker_model.clone(),
            self.picker_selection.clone(),
            move |node: &TreeNode, row: &TreeRow, selected: bool| {
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
                let icon = if node.kind == "binder" {
                    crate::binder::icons::binder_icon()
                } else {
                    crate::binder::icons::kind_sub_role_icon(&node.kind, &node.sub_role)
                };
                item = item.leading_slot(icon);
            // These are the *live* rows a restore lands in, so their numbers are
            // current and meaningful (unlike the trashed rows in the dock itself).
            if badge.is_some() {
                item = item.center_slot(crate::widgets::StructureNumber::new(badge));
            }
                Box::new(item) as Box<dyn Widget>
            },
        )
        .auto_item_height(28.0)
        .scroll_bar_style(ScrollBarMode::Overlay)
        .row_click_expands(false);

        // Empty-state ↔ tree.
        let empty_model = self.picker_model.clone();
        let switch = self
            .picker_model
            .version_signal()
            .map(move |_| usize::from(empty_model.visible_count() != 0));
        let body = Switcher::new(switch)
            .child(
                Padding::symmetric(24.0, 40.0).child(
                    TextWidget::new(tr!(trash_restore_picker_empty()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
            )
            .child(tree);

        let restore_enabled = self
            .picker_selection
            .selection_signal()
            .map(|s| !s.is_empty());

        // Confirm handler for "Restore Here".
        let confirm = {
            let panel_trash = self.trash.clone();
            let model = self.picker_model.clone();
            let selection = self.picker_selection.clone();
            let item_id = self.item_id;
            let entry_title = self.entry_title.clone();
            let on_done = self.on_done.clone();
            let resolve_self = TrashRestoreTargetResolver {
                model: model.clone(),
            };
            move |ctx: &mut EventContext| {
                let Some(key) = selection.selected_keys().first().copied() else {
                    return;
                };
                let (dest, anchor, pos) = resolve_self.resolve(key);
                let dest_title = model.node_of(&key).map(|(_, t)| t).unwrap_or_default();
                let trash = panel_trash.clone();
                let entry_title = entry_title.clone();
                let on_done = on_done.clone();
                MessageBox::question(tr!(trash_restore_to_confirm_title()))
                    .text(tr!(trash_restore_to_confirm_text(
                        item = entry_title,
                        destination = dest_title
                    )))
                    .buttons(MessageBoxButtons::OkCancel)
                    .on_result(move |r, ctx2| {
                        if r.button != StandardButton::Ok {
                            return;
                        }
                        match trash.restore_to(item_id, dest, anchor, pos.clone()) {
                            Ok(_) => {
                                // Dismiss the picker FIRST, then toast: `on_result`'s
                                // ctx is root-anchored, so `dismiss_top_overlay`
                                // targets the topmost overlay — showing the toast
                                // first would make it that overlay and leave the
                                // modal open (matches the export/import ordering).
                                ctx2.dismiss_top_overlay();
                                ctx2.show_toast(
                                    Toast::success(tr!(trash_restored_ok(count = 1)))
                                        .target_work(trash.work_id().get()),
                                );
                                (on_done)(ctx2);
                            }
                            Err(e) => {
                                ctx2.show_toast(
                                    Toast::error(tr!(trash_restore_error(error = e.to_string())))
                                        .target_work(trash.work_id().get()),
                                );
                            }
                        }
                    })
                    .present(ctx);
            }
        };

        // Every close path (header ✕, footer Cancel) advances the orphan queue,
        // so a multi-item restore never stalls. (The modal is Manual-close — see
        // `present_trash_restore_target` — so there is no Esc/click-outside path that
        // could skip these.)
        let cancel_done = self.on_done.clone();
        let cancel_done2 = self.on_done.clone();

        let root = bati!(ctx => FixedSize {
            width: CARD_W
            height: CARD_H
            Panel {
                variant: PanelVariant::Raised
                corner_radius: 10.0
                padding: 0.0
                VStack {
                    spacing: 0.0
                    Expand::horizontal {
                        FixedSize {
                            height: 44.0
                            Padding::symmetric(8.0, 14.0) {
                                HStack {
                                    spacing: 8.0
                                    Expand::horizontal {
                                        TextWidget::new(tr!(trash_restore_picker_title())) {
                                            style: TextStyleRole::Small
                                            color: TextRole::Secondary
                                        }
                                    }
                                    IconButton::clear() {
                                        tooltip: tr!(trash_restore_picker_cancel())
                                        on_activate_fn: move |ctx| { ctx.dismiss_modal(); (cancel_done)(ctx); }
                                    }
                                }
                            }
                        }
                    }
                    Expand::horizontal {
                        Divider
                    }
                    Expand::vertical {
                        child: body
                    }
                    Expand::horizontal {
                        Divider
                    }
                    Expand::horizontal {
                        FixedSize {
                            height: 52.0
                            Padding::symmetric(10.0, 22.0) {
                                HStack {
                                    spacing: 8.0
                                    Spacer
                                    Button::new(tr!(trash_restore_picker_cancel())) {
                                        variant: ButtonVariant::Plain
                                        on_activate_fn: move |ctx| { ctx.dismiss_modal(); (cancel_done2)(ctx); }
                                    }
                                    Button::new(tr!(trash_restore_picker_restore_here())) {
                                        variant: ButtonVariant::Filled
                                        enabled: restore_enabled
                                        on_activate_fn: confirm
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// Tiny helper so the confirm closure can resolve a key without borrowing the
/// panel (which `build` has mutably).
struct TrashRestoreTargetResolver {
    model: BinderBinderItemsTreeModel,
}

impl TrashRestoreTargetResolver {
    fn resolve(&self, key: BinderTreeKey) -> (u64, Option<u64>, DropPosition) {
        // Both arms resolve through the tree now: the key names a row by durable uid, so
        // the live store ids live on the node rather than in the key itself.
        let binder = self.model.binder_of(&key).unwrap_or(0);
        match self.model.item_id_of(&key) {
            None => (binder, None, DropPosition::Into), // a binder row (or a vanished one)
            Some(i) => {
                let pos = if self.model.node_is_folder(&key) {
                    DropPosition::Into
                } else {
                    DropPosition::After
                };
                (binder, Some(i), pos)
            }
        }
    }
}
