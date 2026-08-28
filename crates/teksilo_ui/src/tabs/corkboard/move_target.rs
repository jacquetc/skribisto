// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Move to…** modal: pick a destination container for the cards a corkboard
//! action is acting on.
//!
//! The board can only reorder *within* the container it shows and drill through
//! folders — moving a scene somewhere else in the project otherwise means leaving
//! for the outline. This is that door.
//!
//! Deliberately a sibling of [`crate::trash::restore_target_panel`] rather than a
//! generalisation of it: that panel's confirm calls the **trash** use case
//! (`restore_items_to`, which peels a *trashed* row back to a live position), and
//! this one calls `move_items` on live rows. They share a shape, not a behaviour —
//! folding them together would put a runtime branch on the one line that decides
//! which use case runs. The tree wiring, the folder-vs-leaf target resolution and
//! the chrome are the same, and intentionally read the same.

use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::styles::PanelVariant;
use teksilo::data::{KeyedSelectionModel, SelectionMode, TreeDataSource};
use teksilo::prelude::TextStyleRole;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, Padding, Panel,
    ScrollBarMode, Spacer, StandardTreeItem, Switcher, TextWidget, Toast, TreeRow, TreeView,
    VStack,
};

use frontend::binder_item_management::MovePlace;

use crate::corkboard::CorkboardViewModel;
use crate::models::{BinderBinderItemsTreeModel, BinderTreeKey, TreeFilters, TreeNode};
use crate::toast_scope::ToastWorkExt;

const CARD_W: f32 = 560.0;
const CARD_H: f32 = 520.0;

/// Present the destination picker for `item_ids` over the current window.
pub(super) fn present_move_target(
    ctx: &mut EventContext,
    vm: CorkboardViewModel,
    item_ids: Vec<u64>,
) {
    if item_ids.is_empty() {
        return;
    }
    ctx.present_modal(
        ModalRequest::deferred(move |t| t.add(MoveTargetPanel::new(vm.clone(), item_ids.clone())))
            .presentation(ModalPresentation::InTree)
            .close_behavior(ModalCloseBehavior::Manual)
            .title(tr!(corkboard_move_picker_title()))
            .size(CARD_W as u32, CARD_H as u32),
    );
}

pub(super) struct MoveTargetPanel {
    vm: CorkboardViewModel,
    item_ids: Vec<u64>,
    picker_model: BinderBinderItemsTreeModel,
    picker_selection: KeyedSelectionModel<BinderTreeKey>,
    root_child: Option<WidgetId>,
}

impl MoveTargetPanel {
    fn new(vm: CorkboardViewModel, item_ids: Vec<u64>) -> Self {
        let filters = TreeFilters {
            binder: Signal::new(None),
            query: Signal::new(String::new()),
            match_counts: Signal::new((0, 0)),
            all_binders: Signal::new(false),
        };
        // The model already excludes trashed rows, so a live card can never be
        // moved into the trash by way of this picker.
        let picker_model = BinderBinderItemsTreeModel::new(vm.app_ctx(), vm.work_id(), filters);
        Self {
            vm,
            item_ids,
            picker_model,
            picker_selection: KeyedSelectionModel::new(SelectionMode::Single),
            root_child: None,
        }
    }
}

impl std::fmt::Debug for MoveTargetPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MoveTargetPanel")
            .field("item_ids", &self.item_ids)
            .finish()
    }
}

impl Widget for MoveTargetPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.picker_model.wire(ctx);

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
                    // A row title is as long as the writer made it; without this the label
                    // reports its full intrinsic width and paints out of the panel. See
                    // `binder::dock`, where this was found.
                    .label_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
                    .subtitle_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
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
                // The chapter's ordinal — the same landmarks the outline gives, so a
                // destination is as recognisable here as it is there.
                if badge.is_some() {
                    item = item.center_slot(crate::widgets::StructureNumber::new(badge));
                }
                Box::new(item) as Box<dyn Widget>
            },
        )
        .auto_item_height(28.0)
        .scroll_bar_style(ScrollBarMode::Overlay)
        .row_click_expands(false);

        let empty_model = self.picker_model.clone();
        let switch = self
            .picker_model
            .version_signal()
            .map(move |_| usize::from(empty_model.visible_count() != 0));
        let body = Switcher::new(switch)
            .child(
                Padding::symmetric(24.0, 40.0).child(
                    TextWidget::new(tr!(corkboard_move_picker_empty()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
            )
            .child(tree);

        let move_enabled = self
            .picker_selection
            .selection_signal()
            .map(|s| !s.is_empty());

        let confirm = {
            let vm = self.vm.clone();
            let model = self.picker_model.clone();
            let selection = self.picker_selection.clone();
            let item_ids = self.item_ids.clone();
            move |ctx: &mut EventContext| {
                let Some(key) = selection.selected_keys().first().copied() else {
                    return;
                };
                let (target, is_binder, place) = resolve_target(&model, key);
                let work = vm.work_id().get();
                // Report what actually moved, not what was asked: dropping the
                // selection onto one of its own members silently filters that one
                // out, and "3 cards moved" when 2 moved is a lie the writer has no
                // way to check.
                let moved = vm.move_many_to(&item_ids, target, is_binder, place);
                if moved > 0 {
                    ctx.dismiss_top_overlay();
                    ctx.show_toast(
                        Toast::success(tr!(corkboard_moved_ok(count = moved as i64)))
                            .target_work(work),
                    );
                } else {
                    // The only way a resolved move is refused: the destination sits
                    // inside the very subtree being moved. Say so, and leave the
                    // picker open so another destination can be chosen.
                    ctx.show_toast(Toast::error(tr!(corkboard_move_into_self())).target_work(work));
                }
            }
        };

        let root = teksu!(ctx => FixedSize {
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
                                        TextWidget::new(tr!(corkboard_move_picker_title())) {
                                            style: TextStyleRole::Small
                                            color: TextRole::Secondary
                                        }
                                    }
                                    IconButton::clear() {
                                        tooltip: tr!(corkboard_move_picker_cancel())
                                        on_activate_fn: move |ctx| { ctx.dismiss_modal(); }
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
                                    Button::new(tr!(corkboard_move_picker_cancel())) {
                                        variant: ButtonVariant::Plain
                                        on_activate_fn: move |ctx| { ctx.dismiss_modal(); }
                                    }
                                    Button::new(tr!(corkboard_move_here())) {
                                        variant: ButtonVariant::Filled
                                        enabled: move_enabled
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

/// A picked row → the `(target, target_is_binder, place)` triple `move_items` wants.
///
/// The same folder-vs-leaf reading `TrashRestoreTargetResolver` uses, so "Move to…"
/// and "Restore to…" land a row in the same place for the same click: a binder or a
/// folder means *inside* it, a leaf means *after* it (a leaf has no inside).
fn resolve_target(
    model: &BinderBinderItemsTreeModel,
    key: BinderTreeKey,
) -> (u64, bool, MovePlace) {
    match model.item_id_of(&key) {
        // A binder row (or one that vanished under us): move into the binder itself.
        None => (model.binder_of(&key).unwrap_or(0), true, MovePlace::Into),
        Some(i) if model.node_is_folder(&key) => (i, false, MovePlace::Into),
        Some(i) => (i, false, MovePlace::After),
    }
}
