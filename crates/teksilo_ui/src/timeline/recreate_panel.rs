// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **Where should it go?** — picking a place in the binder for a row being
//! brought back from a backup.
//!
//! The tree is [`crate::widgets::DestinationPicker`], the same one document
//! import and trash-restore raise, because all three ask the identical question.
//! What is here is what is about to happen: the chrome, the confirm, and the
//! sentence naming the row.
//!
//! ## Why a picker at all, and not the recorded place
//!
//! A version records the row's `indent`, and it is tempting to treat that as
//! where it goes. It is not: a depth means something only against the tree it was
//! measured in, and that tree is exactly what the row is no longer part of. The
//! chapter it sat under may itself have been cut, the binder reorganised, the
//! whole part rewritten. Every other feature that puts a row somewhere —
//! restoring out of the trash, importing a document — asks, and so does this.
//!
//! The panel carries no knowledge of *how* a row is recreated. It is handed a
//! picker and a closure, so the peers that operation needs (the open-documents
//! store, this Work's undo stack, the binder) stay where they are assembled, in
//! `app::project_shell`.

use std::rc::Rc;

use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::styles::PanelVariant;
use teksilo::prelude::TextStyleRole;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, Padding, Panel, Spacer,
    TextWidget, VStack,
};

use crate::widgets::DestinationPicker;
use crate::widgets::destination_picker::BinderDestination;

const CARD_W: f32 = 560.0;
const CARD_H: f32 = 520.0;

/// What the confirm button hands back.
pub type ChoseDestination = Rc<dyn Fn(&mut EventContext, BinderDestination)>;

/// Present the destination picker for a row about to be brought back.
pub fn present_recreate_target(
    ctx: &mut EventContext,
    picker: DestinationPicker,
    item_title: String,
    on_confirm: ChoseDestination,
) {
    // The tree fills from backend events, and this modal is raised long after
    // those have landed for the open project — so nothing would arrive to
    // trigger a first load. The same reason `DestinationPicker::reload` exists.
    picker.reload();
    ctx.present_modal(
        ModalRequest::deferred(move |t| {
            t.add(RecreateTargetPanel::new(
                picker.clone(),
                item_title.clone(),
                on_confirm.clone(),
            ))
        })
        .presentation(ModalPresentation::InTree)
        // Esc and click-outside are fine here, unlike the trash picker's queue:
        // dismissing this costs nothing and leaves nothing half-done.
        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
        .title(tr!(versions_recreate_picker_title()))
        .size(CARD_W as u32, CARD_H as u32),
    );
}

pub struct RecreateTargetPanel {
    picker: DestinationPicker,
    item_title: String,
    on_confirm: ChoseDestination,
    root_child: Option<WidgetId>,
}

impl RecreateTargetPanel {
    pub fn new(
        picker: DestinationPicker,
        item_title: String,
        on_confirm: ChoseDestination,
    ) -> Self {
        Self {
            picker,
            item_title,
            on_confirm,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for RecreateTargetPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RecreateTargetPanel")
            .field("item_title", &self.item_title)
            .finish()
    }
}

impl Widget for RecreateTargetPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let body = self.picker.view(tr!(versions_recreate_picker_empty()));
        let confirm_enabled = self.picker.has_selection();

        let confirm = {
            let picker = self.picker.clone();
            let on_confirm = self.on_confirm.clone();
            move |ctx: &mut EventContext| {
                let Some(destination) = picker.selected() else {
                    return;
                };
                // Dismissed before the handler runs: it raises a confirmation of
                // its own, and a `MessageBox` presented under a still-open modal
                // is the arrangement that put a dialog behind an overlay.
                ctx.dismiss_modal();
                on_confirm(ctx, destination);
            }
        };

        // Named, not "this row": a writer who opened three readers in a row has
        // to be able to tell which one this question is about.
        let heading = tr!(versions_recreate_confirm_title(
            item = self.item_title.clone()
        ));

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
                                        TextWidget::new(heading) {
                                            style: TextStyleRole::Small
                                            color: TextRole::Secondary
                                        }
                                    }
                                    IconButton::clear() {
                                        tooltip: tr!(versions_recreate_picker_cancel())
                                        on_activate_fn: |ctx| ctx.dismiss_modal()
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
                                    Button::new(tr!(versions_recreate_picker_cancel())) {
                                        variant: ButtonVariant::Plain
                                        on_activate_fn: |ctx| ctx.dismiss_modal()
                                    }
                                    Button::new(tr!(versions_recreate_picker_confirm())) {
                                        variant: ButtonVariant::Filled
                                        enabled: confirm_enabled
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

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_ids::AppIds;
    use frontend::AppContext;
    use teksilo::prelude::SizeProposal;

    /// The modal composes with a real tree under it. An extraction that left the
    /// picker out would compile perfectly well and open an empty card.
    #[test]
    fn the_recreate_modal_mounts_its_destination_tree() {
        let app_ctx = std::rc::Rc::new(AppContext::new());
        let ids = AppIds::default();
        let panel = RecreateTargetPanel::new(
            DestinationPicker::new(app_ctx.clone(), ids.work_id.clone()),
            "A cut chapter".to_string(),
            std::rc::Rc::new(|_: &mut EventContext, _: BinderDestination| {}),
        );

        // `tree_with_events`, not a bare `WidgetTree`: the picker's tree model
        // subscribes to backend events, and `ctx.subscribe_event` panics with no
        // source registered.
        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        let id = tree.add_boxed(Box::new(panel));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));

        fn first_containing(
            tree: &teksilo::core::widget_tree::WidgetTree,
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
