// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Writing-session chrome icons — the status-bar play/pause + configure glyphs.
//! Like [`editor_icons`](crate::icons::editor), `res!` embeds each asset at compile time
//! and the icon follows the theme via `TextRole` tinting of `currentColor`.

use teksilo::res;
use teksilo::widgets::IconWidget;

/// Status-bar glyph size (dp), matching the save/split controls.
const ICON_SIZE: f32 = 16.0;

/// A play triangle — start (or resume) a writing session.
pub fn play() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/session/play.svg")).icon_size(ICON_SIZE)
}

/// Two bars — pause the running session.
pub fn pause() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/session/pause.svg")).icon_size(ICON_SIZE)
}

/// A cog — open the session-goal configure popover.
pub fn gear() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/session/gear.svg")).icon_size(ICON_SIZE)
}
