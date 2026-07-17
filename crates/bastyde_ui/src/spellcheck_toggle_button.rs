// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SpellcheckToggleButton` — the title-bar master switch for spell-checking, left of the
//! Export split-button.
//!
//! A flat icon button carrying the whole answer to "is my prose being checked?": the glyph is
//! struck through when it is off. It exists because the app had **no** off switch at all — the
//! only control was a green check on a language pill, which mutes *one* dictionary for the
//! session. Under the union model (a word is wrong only when every active dictionary rejects
//! it) unchecking one of several languages changes nothing visible, so a writer who wanted
//! "stop underlining my Latin" had no way to say it and no way to see that they hadn't.
//!
//! Thin, per the house rules: it owns no state. It renders
//! [`SettingsViewModel::spellcheck_enabled`] (mirrored into a plain `Signal` by `App::build`,
//! since the title bar lives outside `App` and has no `ctx.settings()`), and clicking fires the
//! `spellcheck.toggle` intent — the same command F7 and View ▸ Check spelling fire, so all
//! three surfaces share one path and one truth.
//!
//! **Off is a shape, not a colour.** The struck-through glyph is a separate asset rather than a
//! dimmed tint: the state must survive a theme, a colour-blind reader, and a glance.

use bastyde::core::BindingLevel;
use bastyde::prelude::*;
use bastyde::widgets::{IconButton, IconButtonSize};

pub struct SpellcheckToggleButton {
    /// `App::build`'s plain mirror of the persisted `SPELLCHECK_ENABLED_KEY`.
    enabled: Signal<bool>,
    root_child: Option<WidgetId>,
}

impl SpellcheckToggleButton {
    pub fn new(enabled: Signal<bool>) -> Self {
        Self {
            enabled,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for SpellcheckToggleButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpellcheckToggleButton").finish()
    }
}

impl Widget for SpellcheckToggleButton {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // `IconButton` takes its glyph at construction, so swapping the icon means
        // rebuilding — the same reason `ExportSplitButton` binds at `Rebuild`.
        self.enabled
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        let on = self.enabled.get();

        let icon = if on {
            crate::editor_icons::spellcheck_on()
        } else {
            crate::editor_icons::spellcheck_off()
        };
        // Secondary when on: this is chrome, and a permanently-lit title-bar icon reads as an
        // alert. Disabled-grey when off would be wrong too — the control is still live — so
        // off leans on the struck-through glyph, not a colour.
        let id = ctx.add(
            IconButton::new(icon.color(if on {
                TextRole::Secondary
            } else {
                TextRole::Disabled
            }))
            .size(IconButtonSize::Large)
            .tooltip(if on {
                tr!(titlebar_spellcheck_on())
            } else {
                tr!(titlebar_spellcheck_off())
            })
            .on_activate_fn(|c| c.send_intent(Intent::new("spellcheck.toggle"))),
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
