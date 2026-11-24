// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `DimTextToggleButton` — the title-bar switch for the fade around the caret, between the
//! spell-check toggle and the distraction-free button.
//!
//! The fade is the kind of thing a writer turns on and off many times a session (on to draft,
//! off to reread), so it sits beside the other writing-surface toggle rather than three levels
//! down in Settings. F8 and View ▸ Fade the rest of the text do the same thing.
//!
//! Thin, per the house rules: it owns no state. It renders
//! [`SettingsViewModel::dim_enabled`](crate::settings::SettingsViewModel::dim_enabled), mirrored
//! into a plain `Signal` by `App::build` because the title bar lives outside `App` and has no
//! `ctx.settings()`, and clicking fires the `view.dim_text` intent: the same command the key
//! and the menu row fire, so every surface shares one path and one truth.
//!
//! **Off is a shape, not a colour**, as on the spell-check toggle: the glyph is struck through,
//! so the state survives a theme, a colour-blind reader and a glance.

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::{IconButton, IconButtonSize};

pub struct DimTextToggleButton {
    /// `App::build`'s plain mirror of the persisted `DIM_ENABLED_KEY`.
    enabled: Signal<bool>,
    root_child: Option<WidgetId>,
}

impl DimTextToggleButton {
    pub fn new(enabled: Signal<bool>) -> Self {
        Self {
            enabled,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for DimTextToggleButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DimTextToggleButton").finish()
    }
}

impl Widget for DimTextToggleButton {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // `IconButton` takes its glyph at construction, so swapping the icon means rebuilding,
        // the reason `SpellcheckToggleButton` binds at `Rebuild` too.
        self.enabled
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        let on = self.enabled.get();

        let icon = if on {
            crate::icons::editor::dim_text_on()
        } else {
            crate::icons::editor::dim_text_off()
        };
        // Secondary when on and Disabled-grey when off, matching the spell-check toggle beside
        // it, so the two read as a pair. The control stays live when off; the strike carries
        // the state, not the grey.
        let id = ctx.add(
            IconButton::new(icon.color(if on {
                TextRole::Secondary
            } else {
                TextRole::Disabled
            }))
            // `Toolbar` (30 dp), like every title-bar control: the bar is
            // `shell::TITLE_BAR_HEIGHT` tall and lets an oversized child overflow.
            .size(IconButtonSize::Toolbar)
            .tooltip(if on {
                tr!(titlebar_dim_text_on())
            } else {
                tr!(titlebar_dim_text_off())
            })
            .on_activate_fn(|c| c.send_intent(Intent::new("view.dim_text"))),
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

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::accesskit::Role;
    use teksilo::core::widget_tree::WidgetTree;

    /// The accessible names of the buttons in the tree.
    fn button_names(tree: &mut WidgetTree) -> Vec<String> {
        tree.sync_accessibility()
            .nodes
            .iter()
            .filter(|(_, n)| n.role() == Role::Button)
            .filter_map(|(_, n)| n.label().map(|s| s.to_string()))
            .collect()
    }

    /// The button follows the mirrored setting both ways: its name says which state the fade
    /// is in, and it changes when the setting does, without anybody rebuilding the title bar
    /// by hand.
    #[test]
    fn the_button_follows_the_setting() {
        let enabled = Signal::new(false);
        let mut tree = WidgetTree::new();
        tree.add(DimTextToggleButton::new(enabled.clone()));
        tree.layout(SizeProposal::exact(200.0, 40.0));
        let off = tr!(titlebar_dim_text_off()).resolve_now();
        assert_eq!(button_names(&mut tree), [off]);

        enabled.set(true);
        tree.layout(SizeProposal::exact(200.0, 40.0));
        let on = tr!(titlebar_dim_text_on()).resolve_now();
        assert_eq!(button_names(&mut tree), [on]);
    }
}
