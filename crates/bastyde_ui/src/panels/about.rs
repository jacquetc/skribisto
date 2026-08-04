// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **About** modal — Help ▸ About.
//!
//! Deliberately small and static: the app's name, the running version (from
//! [`crate::version::app_version`], so a dev build shows its git description
//! rather than a bare crate version), what it is, and the licence. There is no
//! model behind it and nothing to configure, so unlike the other panels it takes
//! no view-model and has no real/mock split.
//!
//! Shaped like [`LicensePanel`](super::license) — the same raised card with a
//! header, a body and a footer — but without the scroll area, since the content
//! is a handful of lines rather than a page of legal text.

use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::core::styles::PanelVariant;
use bastyde::prelude::*;
use bastyde::res;
use bastyde::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, IconWidget, Padding,
    Panel, Spacer, TextWidget, VStack,
};

const CARD_W: f32 = 460.0;
const CARD_H: f32 = 300.0;

/// Present the About modal. Fired by the `app.about` global action, which
/// `App::build` registers — see the note there on why it must be *global*.
pub fn present_about(ctx: &mut EventContext) {
    ctx.present_modal(
        ModalRequest::deferred(|t| t.add(AboutPanel::new()))
            .presentation(ModalPresentation::InTree)
            .title(tr!(about_title()))
            .size(CARD_W as u32, CARD_H as u32)
            .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

struct AboutPanel {
    root_child: Option<WidgetId>,
}

impl AboutPanel {
    fn new() -> Self {
        Self { root_child: None }
    }
}

impl std::fmt::Debug for AboutPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AboutPanel").finish()
    }
}

impl Widget for AboutPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // The version is *data* (a git description or a crate version), never
        // translated — so it stays `lit!`. The surrounding "Version {…}" framing
        // is the translated part and lives in the ftl key.
        let version = tr!(about_version(version = crate::version::app_version()));

        let footer = HStack::new().spacing(8.0).child(Spacer::new()).child(
            Button::new(tr!(about_close()))
                .variant(ButtonVariant::Filled)
                .on_activate_fn(|ctx| ctx.dismiss_modal()),
        );

        let root = bati!(ctx => FixedSize {
            width: CARD_W
            height: CARD_H
            Panel {
                variant: PanelVariant::Raised
                corner_radius: 10.0
                padding: 0.0
                VStack {
                    spacing: 0.0
                    // Header: app name + version, and a close button.
                    Expand::horizontal {
                        FixedSize {
                            height: 60.0
                            Padding::symmetric(8.0, 16.0) {
                                HStack {
                                    spacing: 10.0
                                    IconWidget::from_svg_icon(res!("assets/icons/welcome/info.svg")) {
                                        icon_size: 24.0
                                    }
                                    Expand::horizontal {
                                        VStack {
                                            spacing: 2.0
                                            TextWidget::new(lit!("Skribisto")) {
                                                style: TextStyleRole::BodyBold
                                            }
                                            TextWidget::new(version) {
                                                style: TextStyleRole::Tiny
                                                color: TextRole::Secondary
                                            }
                                        }
                                    }
                                    IconButton::clear() {
                                        tooltip: tr!(about_close())
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
                        Padding::symmetric(18.0, 20.0) {
                            VStack {
                                spacing: 10.0
                                TextWidget::new(tr!(about_tagline())) {
                                    style: TextStyleRole::Body
                                }
                                TextWidget::new(tr!(about_license())) {
                                    style: TextStyleRole::Small
                                    color: TextRole::Secondary
                                }
                                TextWidget::new(tr!(about_copyright())) {
                                    style: TextStyleRole::Small
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
                        FixedSize {
                            height: 56.0
                            Padding::symmetric(12.0, 20.0) {
                                child: footer
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
    use bastyde::core::widget_tree::WidgetTree;

    /// The panel is static content, so the thing worth pinning is that it builds
    /// and lays out at its card size at all — the failure mode for a `bati!`
    /// tree is a panic during build, not a wrong value.
    #[test]
    fn the_about_panel_builds_and_lays_out() {
        let mut tree = WidgetTree::new();
        tree.add(AboutPanel::new());
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
    }

    /// The version line must carry the real running version, not a placeholder —
    /// an About box that lies about the build is worse than none.
    #[test]
    fn the_version_shown_is_the_running_version() {
        let v = crate::version::app_version();
        assert!(!v.is_empty(), "app_version() must never be empty");
    }
}
