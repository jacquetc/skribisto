//! Editor-chrome icons — the split-view toggle glyphs for the tab-strip trailing
//! slots. Like [`binder_icons`](crate::binder_icons), `res!` embeds each asset at
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
