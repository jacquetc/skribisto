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

use std::rc::Rc;

use frontend::AppContext;
use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::prelude::*;
use teksilo::widgets::{Padding, ScrollArea, VStack};

use crate::app_ids::AppIds;
use crate::statuses::StatusesViewModel;

const CARD_W: f32 = 420.0;
const CARD_H: f32 = 380.0;

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
        let body =
            crate::statuses::completion::readout_for(&self.app_ctx, &self.ids, &self.statuses);
        let id = ctx.add(
            Padding::symmetric(18.0, 16.0).child(
                ScrollArea::new().child(
                    VStack::new()
                        .spacing(12.0)
                        .child(crate::tabs::Boxed::new(body)),
                ),
            ),
        );
        self.root_child = Some(id);
        vec![id]
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
