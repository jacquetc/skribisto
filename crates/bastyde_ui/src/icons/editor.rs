// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor-chrome icons — the split-view toggle glyphs for the tab-strip trailing
//! slots. Like [`binder_icons`](crate::binder::icons), `res!` embeds each asset at
//! compile time (a literal path per call site) and the icon follows the theme via
//! `TextRole` tinting.

use bastyde::res;
use bastyde::widgets::IconWidget;

/// Icon-button glyph size (dp) for the tab-strip controls.
const ICON_SIZE: f32 = 16.0;

/// "Split editor" — reveal the side pane (primary pane's trailing slot).
pub fn split() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/split.svg")).icon_size(ICON_SIZE)
}

/// "Close split view" — collapse the side pane (side pane's trailing slot).
pub fn close_split() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/split-close.svg")).icon_size(ICON_SIZE)
}

/// Save glyph with an asterisk — the status bar's "there are unsaved changes".
pub fn save_unsaved() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/save-unsaved.svg")).icon_size(ICON_SIZE)
}

/// Save glyph with a check — the status bar's "everything is on disk".
pub fn save_saved() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/save-saved.svg")).icon_size(ICON_SIZE)
}

/// Spell-check **on** — the title-bar toggle's glyph when checking is enabled.
pub fn spellcheck_on() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/spellcheck.svg")).icon_size(ICON_SIZE)
}

/// Spell-check **off** — the same glyph struck through. A separate asset rather than a tint:
/// the control must read as off at a glance and against either theme, and colour alone would
/// carry the whole meaning (which it must never do — see the title bar's other toggles).
pub fn spellcheck_off() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/spellcheck-off.svg")).icon_size(ICON_SIZE)
}
