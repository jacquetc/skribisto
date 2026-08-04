// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Icons for the corkboard header's order toggle.
//!
//! Line-style, `stroke="currentColor"`, so they follow the theme — the same
//! contract as [`icons::comments`](super::comments), and for the same reason.
//! The order control's labels ("Manuscript order" / "Ordre du manuscrit") were
//! the widest thing in a header that already carries a breadcrumb, a count, a
//! Nested/Flat toggle, a create button, a filter field and a size slider: as a
//! combo box it starved the filter field — the row's only flexible child — down
//! to ~55px. Three states is still few enough for an icon toggle, and the glyph
//! shown is the *current* order with the tooltip naming it.
//!
//! `res!` embeds each asset at compile time and needs a literal path per call
//! site, so there is one function per icon.

use bastyde::res;
use bastyde::widgets::IconWidget;

/// Manuscript order — the cards as the stream has them, unsorted.
pub fn sort_manuscript_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/corkboard/sort-manuscript.svg"))
}

/// Title A–Z — short line to long, arrow down.
pub fn sort_title_asc_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/corkboard/sort-title-asc.svg"))
}

/// Title Z–A — long line to short, arrow up.
pub fn sort_title_desc_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/corkboard/sort-title-desc.svg"))
}
