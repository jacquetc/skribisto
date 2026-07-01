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
