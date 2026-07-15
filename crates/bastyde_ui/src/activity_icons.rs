//! Activity-rail icons — the glyphs shown in the leading dock's VS Code-style
//! activity bar (one per activity: Outline, and later Search/Characters/…).
//!
//! Icons default to `TextRole::Primary` and follow the theme; the rail tints the
//! selected item with the accent role. The **rail owns glyph sizing** — it
//! scales the icon to its `IconButtonSize` (Compact…Hero), so these factories
//! set no `icon_size`. `res!` embeds each asset at compile time and needs a
//! literal path per call site, so there is one function per icon.

use bastyde::res;
use bastyde::widgets::IconWidget;

/// The Outline (binder) activity icon: the manuscript-binder box glyph.
pub fn outline_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/activities/outline.svg"))
}

/// The Search activity icon: a magnifier. Fronts the **leading** search &
/// replace dock's rail (its query, options, and result list).
pub fn search_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/activities/search.svg"))
}

/// The Search activity icon: a magnifier. Fronts the bottom preview band's rail.
pub fn search_preview_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/activities/search.svg"))
}

/// The Inspector activity icon: a right side-panel glyph.
pub fn inspector_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/activities/inspector.svg"))
}

/// The sidebar (leading dock) toggle glyph: a left side-panel — the mirror of
/// [`inspector_icon`], for the status-bar show/hide-binder button.
pub fn sidebar_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/activities/sidebar.svg"))
}
