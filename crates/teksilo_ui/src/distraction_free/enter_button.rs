// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The title bar's way into distraction-free mode, beside the spell-check switch.
//!
//! Enter-only by construction, so it needs no state and no checked look: the mode
//! parks the whole project shell, title bar included, so this button is never on
//! screen while the mode is active. The way out is the strip's Exit button, F11
//! or Escape.
//!
//! It fires the `view.focus_mode` intent, the same command F11 and View ▸
//! Distraction-free Mode fire, so every entrance shares one path.

use teksilo::prelude::*;
use teksilo::widgets::{IconButton, IconButtonSize};

pub fn enter_button() -> IconButton {
    // Secondary, like the spell-check switch beside it: title-bar chrome, not an alert.
    IconButton::new(crate::icons::editor::distraction_free().color(TextRole::Secondary))
        // `Toolbar` (30 dp): the title bar is `shell::TITLE_BAR_HEIGHT` tall and
        // lets an oversized child overflow rather than growing to fit it.
        .size(IconButtonSize::Toolbar)
        .tooltip(tr!(titlebar_distraction_free()))
        .on_activate_fn(|c| c.send_intent(Intent::new("view.focus_mode")))
}
