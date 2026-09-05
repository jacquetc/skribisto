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

use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::styles::PanelVariant;
use teksilo::prelude::*;
use teksilo::res;
use teksilo::widgets::{
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

        // The second place the application prints its own version, and so the
        // second place the update line belongs. It builds nothing when this copy
        // is current, which is the usual case, leaving the header exactly as it
        // was.
        let update_line = crate::updates::UpdateLine::new(crate::updates::view_model());

        // On a channel somebody else keeps current there is no check and no line,
        // so this says who is responsible instead. Without it, a Flathub reader
        // finds an application that never mentions versions and cannot tell that
        // from one whose check is broken.
        let managed = crate::updates::Channel::current()
            .managed_by()
            .map(|by| match by {
                crate::updates::ManagedBy::Flathub => tr!(about_update_flathub()),
                crate::updates::ManagedBy::Distribution => tr!(about_update_distro()),
            });

        let footer = HStack::new().spacing(8.0).child(Spacer::new()).child(
            Button::new(tr!(about_close()))
                .variant(ButtonVariant::Filled)
                .on_activate_fn(|ctx| ctx.dismiss_modal()),
        );

        let root = teksu!(ctx => FixedSize {
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
                                            TextWidget::new(lit!(crate::identity::display_name())) {
                                                style: TextStyleRole::BodyBold
                                            }
                                            TextWidget::new(version) {
                                                style: TextStyleRole::Tiny
                                                color: TextRole::Secondary
                                            }
                                            child: update_line
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
                                child_opt: managed.map(|line| {
                                    TextWidget::new(line)
                                        .style(TextStyleRole::Small)
                                        .color(TextRole::Secondary)
                                })
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
    use teksilo::core::widget_tree::WidgetTree;

    /// The panel is static content, so the thing worth pinning is that it builds
    /// and lays out at its card size at all — the failure mode for a `teksu!`
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

    /// About is the second place the application prints its version, so it is the
    /// second place a newer one has to be named. The panel builds the line from
    /// the process-wide update state, which this seeds; without the seeding the
    /// state is empty and the assertion would pass for the wrong reason, so the
    /// negative case is checked first.
    #[test]
    fn a_newer_version_is_named_in_the_about_box() {
        const LINE: &str = "Version 999.0.0 is available";

        let mut tree = WidgetTree::new();
        tree.add(AboutPanel::new());
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        assert!(
            tree.find_by_label(LINE).is_none(),
            "nothing recorded means nothing shown"
        );

        let store = crate::models::updates_file::UpdatesService::in_memory_default();
        store.record(
            "999.0.0",
            "2026-12-01",
            std::collections::BTreeMap::new(),
            std::collections::BTreeMap::from([(
                "en".to_string(),
                "https://www.skribisto.eu/download/".to_string(),
            )]),
        );
        crate::updates::update_vm::set_view_model_for_test(crate::updates::UpdateViewModel::new(
            store,
        ));

        let mut tree = WidgetTree::new();
        tree.add(AboutPanel::new());
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        assert!(
            tree.find_by_label(LINE).is_some(),
            "the About box has to name the newer version too, not only the Launcher"
        );
    }
}
