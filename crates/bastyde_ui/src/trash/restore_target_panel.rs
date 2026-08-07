// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Restore to…** modal: pick a destination for a single trashed item.
//!
//! Used whenever a trashed item must land somewhere other than where it sat — a
//! descendant peeled out of a still-trashed subtree, a child of a wholly-trashed
//! binder, or a plain restore that came back `orphaned`.
//!
//! The tree itself is [`crate::widgets::DestinationPicker`], shared with document
//! import since both features ask the same question. What stays here is what is
//! actually about restoring: the modal chrome, the confirmation, and the
//! orphan-queue continuation that must run on every close path.

use std::rc::Rc;

use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::core::styles::PanelVariant;
use bastyde::prelude::TextStyleRole;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, MessageBox,
    MessageBoxButtons, Padding, Panel, Spacer, StandardButton, TextWidget, Toast, VStack,
};

use crate::toast_scope::ToastWorkExt;
use crate::view_models::TrashViewModel;
use crate::widgets::DestinationPicker;

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
    picker: DestinationPicker,
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
        let picker = DestinationPicker::new(trash.app_ctx(), trash.work_id());
        Self {
            trash,
            item_id,
            entry_title,
            picker,
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
        let body = self.picker.view(tr!(trash_restore_picker_empty()));
        let restore_enabled = self.picker.has_selection();

        // Confirm handler for "Restore Here".
        let confirm = {
            let panel_trash = self.trash.clone();
            let picker = self.picker.clone();
            let item_id = self.item_id;
            let entry_title = self.entry_title.clone();
            let on_done = self.on_done.clone();
            move |ctx: &mut EventContext| {
                let Some(destination) = picker.selected() else {
                    return;
                };
                let trash = panel_trash.clone();
                let entry_title = entry_title.clone();
                let on_done = on_done.clone();
                MessageBox::question(tr!(trash_restore_to_confirm_title()))
                    .text(tr!(trash_restore_to_confirm_text(
                        item = entry_title,
                        destination = destination.title.clone()
                    )))
                    .buttons(MessageBoxButtons::OkCancel)
                    .on_result(move |r, ctx2| {
                        if r.button != StandardButton::Ok {
                            return;
                        }
                        match trash.restore_to(
                            item_id,
                            destination.binder_id,
                            destination.anchor_item_id,
                            destination.position.clone(),
                        ) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_ids::AppIds;
    use crate::docks::TRASH_DOCK_ID;
    use crate::models::TrashTreeModel;
    use bastyde::prelude::SizeProposal;
    use bastyde::widgets::{DockWidgetId, DockingModel};
    use frontend::AppContext;

    fn trash_vm(app_ctx: &std::rc::Rc<AppContext>) -> TrashViewModel {
        let ids = AppIds::default();
        let model = TrashTreeModel::new(app_ctx.clone(), ids.work_id.clone());
        TrashViewModel::new(
            app_ctx.clone(),
            ids,
            model,
            DockingModel::new(),
            DockWidgetId::from_raw(TRASH_DOCK_ID),
        )
    }

    /// The panel still composes after the tree moved out into
    /// `widgets::DestinationPicker`. Chrome plus a real tree, laid out — an
    /// extraction that left the modal empty would compile perfectly well.
    #[test]
    fn the_restore_modal_still_mounts_its_destination_tree() {
        let app_ctx = std::rc::Rc::new(AppContext::new());
        let panel = TrashRestoreTargetPanel::new(
            trash_vm(&app_ctx),
            42,
            "A trashed scene".to_string(),
            std::rc::Rc::new(|_: &mut EventContext| {}),
        );

        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        let id = tree.add_boxed(Box::new(panel));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));

        fn first_containing(
            tree: &bastyde::core::widget_tree::WidgetTree,
            root: WidgetId,
            needle: &str,
        ) -> Option<WidgetId> {
            if tree
                .widget_type_name(root)
                .is_some_and(|n| n.contains(needle))
            {
                return Some(root);
            }
            tree.children(root)
                .into_iter()
                .find_map(|c| first_containing(tree, c, needle))
        }

        assert!(
            first_containing(&tree, id, "DestinationPickerView").is_some(),
            "the modal lost its destination picker"
        );
        let bounds = tree.bounds(id);
        assert!(bounds.width > 0.0 && bounds.height > 0.0, "{bounds:?}");
    }
}
