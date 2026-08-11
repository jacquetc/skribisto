// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The first-run import offer — "bring your settings across?".
//!
//! Shown once, as the very first window, when an edition with its own config
//! directory finds the community installation's settings beside it. See
//! [`crate::first_run`] for when the offer is made and what a copy moves.
//!
//! Deliberately plain: two buttons, the two paths spelled out, and no way to get
//! it wrong. Both answers are final (they write the marker), so the screen says
//! what each one means rather than making the writer guess which is safe.

use std::rc::Rc;

use teksilo::core::styles::PanelVariant;
use teksilo::prelude::*;
use teksilo::settings::AppPaths;
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, HStack, Padding, Panel, Spacer, TextWidget, VStack,
};

/// What the window does once the writer has answered: `true` to import.
pub type OnAnswer = Rc<dyn Fn(&mut EventContext, bool)>;

pub struct FirstRunPanel {
    source: AppPaths,
    on_answer: OnAnswer,
    root_child: Option<WidgetId>,
}

impl FirstRunPanel {
    pub fn new(source: AppPaths, on_answer: OnAnswer) -> Self {
        Self {
            source,
            on_answer,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for FirstRunPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FirstRunPanel").finish()
    }
}

impl Widget for FirstRunPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let app = crate::identity::display_name();
        // Paths are data — never translated, and shown in full so the writer can
        // see for themselves that the source is not being touched.
        let from = crate::first_run::source_label(&self.source);
        let to = crate::first_run::destination_label();

        let import = self.on_answer.clone();
        let fresh = self.on_answer.clone();

        let buttons = HStack::new()
            .spacing(8.0)
            .child(Spacer::new())
            .child(
                Button::new(tr!(first_run_start_fresh()))
                    .on_activate_fn(move |c| (fresh)(c, false)),
            )
            .child(
                Button::new(tr!(first_run_import()))
                    .variant(ButtonVariant::Filled)
                    .on_activate_fn(move |c| (import)(c, true)),
            );

        let root = teksu!(ctx => Panel {
            variant: PanelVariant::Raised
            corner_radius: 10.0
            padding: 0.0
            VStack {
                spacing: 0.0
                Expand::horizontal {
                    Padding::symmetric(18.0, 20.0) {
                        VStack {
                            spacing: 6.0
                            TextWidget::new(tr!(first_run_title(app = app.clone()))) {
                                style: TextStyleRole::BodyBold
                            }
                            TextWidget::new(tr!(first_run_body(app = app.clone()))) {
                                style: TextStyleRole::Body
                                color: TextRole::Secondary
                            }
                        }
                    }
                }
                Expand::horizontal {
                    Divider
                }
                Expand::vertical {
                    Padding::symmetric(14.0, 20.0) {
                        VStack {
                            spacing: 8.0
                            TextWidget::new(tr!(first_run_from())) {
                                style: TextStyleRole::Small
                                color: TextRole::Secondary
                            }
                            TextWidget::new(lit!(from)) {
                                style: TextStyleRole::Small
                            }
                            TextWidget::new(tr!(first_run_to())) {
                                style: TextStyleRole::Small
                                color: TextRole::Secondary
                            }
                            TextWidget::new(lit!(to)) {
                                style: TextStyleRole::Small
                            }
                            TextWidget::new(tr!(first_run_copy_note())) {
                                style: TextStyleRole::Tiny
                                color: TextRole::Secondary
                            }
                            Spacer
                        }
                    }
                }
                Expand::horizontal {
                    Divider
                }
                Expand::horizontal {
                    Padding::symmetric(12.0, 20.0) {
                        child: buttons
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
    use teksilo::core::widget_tree::WidgetTree;

    /// The panel is static content, so what is worth pinning is that its `teksu!`
    /// tree builds and lays out — the failure mode is a panic during build, and a
    /// panel nobody can see is a first launch nobody can get past.
    #[test]
    fn the_first_run_panel_builds_and_lays_out() {
        let dir = std::env::temp_dir().join(format!("sk-first-run-panel-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut tree = WidgetTree::new();
        tree.add(FirstRunPanel::new(
            AppPaths::for_testing(&dir),
            Rc::new(|_ctx, _import| {}),
        ));
        tree.layout(SizeProposal::exact(520.0, 360.0));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
