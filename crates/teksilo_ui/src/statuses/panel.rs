// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The completion readout as its own panel, opened on demand.
//!
//! The **content** is [`super::completion::readout`] and nothing here duplicates it: this
//! module is the modal chrome and the "ask for it now" door, exactly as `pace::panel` is
//! the chrome for the same readout when a project opens. Two renderings of one number that
//! could drift apart is the thing the split exists to prevent.
//!
//! Unlike the Pace summary this one is **always available**: it is asked for rather than
//! offered, so it has no "worth showing?" gate and no once-per-session latch. It does still
//! decline to invent news — a project with no prose rows gets the readout's own empty line
//! rather than a table of zeros.
//!
//! The chrome is `pace::panel`'s, for the same reason and by the same means: an in-tree
//! modal that draws no surface of its own has none, because `present_modal` wraps a
//! hand-drawn panel in nothing and honours `.title(..)` only in a native window. Both
//! doors to this readout sit one row apart in the Work menu, so they carry the same shell.

use std::rc::Rc;

use frontend::AppContext;
use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::styles::PanelVariant;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, Padding, Panel,
    ScrollArea, Spacer, TextWidget, VStack,
};

use crate::app_ids::AppIds;
use crate::statuses::StatusesViewModel;

const CARD_W: f32 = 420.0;
const CARD_H: f32 = 380.0;
/// The same header and footer measures the Pace summary and the backups browser use.
const HEADER_H: f32 = 44.0;
const FOOTER_H: f32 = 52.0;

/// Present the readout.
pub fn present(
    ctx: &mut EventContext,
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    statuses: StatusesViewModel,
) {
    ctx.present_modal(
        ModalRequest::deferred(move |t| {
            t.add(CompletionPanel {
                app_ctx: app_ctx.clone(),
                ids: ids.clone(),
                statuses: statuses.clone(),
                root_child: None,
                close_button: None,
            })
        })
        .presentation(ModalPresentation::InTree)
        .title(tr!(status_completion_title()))
        .size(CARD_W as u32, CARD_H as u32)
        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

struct CompletionPanel {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    statuses: StatusesViewModel,
    root_child: Option<WidgetId>,
    /// The footer's Close, captured for [`Widget::initial_focus_hint`].
    close_button: Option<WidgetId>,
}

impl std::fmt::Debug for CompletionPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompletionPanel").finish()
    }
}

impl Widget for CompletionPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Measured at build, not bound to a signal: this is a *reading*, taken when the
        // writer asked for it. A panel whose numbers shifted under them while they read it
        // would be worse, not better — and the Overview beside it is the live surface.
        let readout =
            crate::statuses::completion::readout_for(&self.app_ctx, &self.ids, &self.statuses);
        // The same card the Pace summary and the Book's planner draw their blocks in, so
        // one reading does not look like two different features depending on which row of
        // the Work menu opened it.
        let body = ctx.add(
            ScrollArea::new().child(
                Padding::symmetric(crate::pace::CARD_PADDING, 16.0).child(
                    VStack::new()
                        .spacing(12.0)
                        .child(crate::pace::panel_card(crate::tabs::Boxed::new(readout))),
                ),
            ),
        );
        // Built by hand rather than inside the `teksu!` shell so its id can be captured for
        // `initial_focus_hint` — see that method, and `pace::panel`'s own copy of it.
        let close_button = ctx.add(
            Button::new(tr!(pace_summary_close()))
                .variant(ButtonVariant::Filled)
                .on_activate_fn(|c| c.dismiss_modal()),
        );
        self.close_button = Some(close_button);
        let footer = ctx.add(
            Padding::symmetric(10.0, 16.0)
                .child(HStack::new().child(Spacer::new()).add_child(close_button)),
        );
        // `Expand::horizontal` around each bar: a height-only `FixedSize` is placed at its
        // child's intrinsic width, which starves the header title and leaves the footer's
        // `Spacer` nothing to push against (`backup::list_panel` documents the trap).
        let id = teksu!(ctx => FixedSize {
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
                            height: HEADER_H
                            Padding::symmetric(8.0, 16.0) {
                                HStack {
                                    spacing: 8.0
                                    Expand::horizontal {
                                        TextWidget::new(tr!(status_completion_title())) {
                                            style: TextStyleRole::Small
                                            color: TextRole::Secondary
                                            single_line
                                        }
                                    }
                                    IconButton::clear() {
                                        tooltip: tr!(pace_summary_close())
                                        on_activate_fn: |c| c.dismiss_modal()
                                    }
                                }
                            }
                        }
                    }
                    Expand::horizontal {
                        Divider
                    }
                    Expand::vertical {
                        child_id: body
                    }
                    Expand::horizontal {
                        Divider
                    }
                    Expand::horizontal {
                        FixedSize {
                            height: FOOTER_H
                            child_id: footer
                        }
                    }
                }
            }
        });
        self.root_child = Some(id);
        vec![id]
    }

    /// Open with the footer's **Close** focused, not the header's ✕ — the modal
    /// pipeline's `first_focusable_descendant` fallback would take the glyph, and a card
    /// that opens with a ring around its dismiss control points the eye at the way out.
    fn initial_focus_hint(&self) -> Option<WidgetId> {
        self.close_button
    }

    /// Announce the card as a named dialog — `present_modal` wraps a hand-drawn panel in
    /// no container, so nothing else here would emit a `Role::Dialog` node.
    fn accessibility(&self, builder: &mut teksilo::core::accessibility::AccessNodeBuilder) {
        builder.set_role(teksilo::core::accesskit::Role::Dialog);
        builder.set_name(tr!(status_completion_title()).resolve_now());
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
    use teksilo::core::widget_tree::WidgetTree;

    /// The same guard `pace::panel` carries, for the same bug: an in-tree modal that
    /// draws no surface renders its text straight onto the dimmed project behind it.
    /// Both doors to this readout sit one row apart in the Work menu, so a fix to one
    /// that skipped the other would just move the complaint.
    #[test]
    fn the_card_is_backed_by_a_panel_covering_all_of_it() {
        let app_ctx = Rc::new(AppContext::new());
        let mut tree = crate::test_support::tree_with_events(&app_ctx)
            .with_theme(teksilo::presets::intui::light());
        let id = tree.add(CompletionPanel {
            app_ctx: app_ctx.clone(),
            ids: AppIds::new(),
            statuses: StatusesViewModel::new(app_ctx, AppIds::new()),
            root_child: None,
            close_button: None,
        });
        // `unspecified`, so the card reports what it wants rather than being placed at
        // the proposal — which also asserts it does not swell to fill the window.
        tree.layout(SizeProposal::unspecified());
        assert_eq!(
            (tree.bounds(id).size().width, tree.bounds(id).size().height),
            (CARD_W, CARD_H)
        );

        fn walk(tree: &WidgetTree, id: WidgetId, out: &mut bool) {
            for child in tree.children(id) {
                let s = tree.bounds(child).size();
                if tree
                    .widget_type_name(child)
                    .is_some_and(|n| n.ends_with("::Panel"))
                    && s.width >= CARD_W
                    && s.height >= CARD_H
                {
                    *out = true;
                }
                walk(tree, child, out);
            }
        }
        let mut covered = false;
        walk(&tree, id, &mut covered);
        assert!(
            covered,
            "the readout must sit on a surface of its own, not on the dimmed project"
        );

        // …and focus opens on the footer's Close, not the header's ✕ — the same pin
        // `pace::panel` carries, for the same reason.
        let hinted = tree
            .widget_initial_focus_hint(id)
            .expect("the card pins its own initial focus");
        let first = tree
            .first_focusable_descendant(id)
            .expect("the card has focus stops");
        assert_ne!(
            hinted, first,
            "the hint must not be the tree-order first focusable — that is the ✕"
        );
    }
}
