// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Icons for the search feature's option toggles — the matching options (case /
//! whole-word / accents), the field scopes (body / title / synopsis / label), and
//! the replace glyph. Line-style, `stroke="currentColor"`, so they follow the
//! theme and the toggle tint. `res!` embeds each asset at compile time and needs a
//! literal path per call site, so there is one function per icon.

use teksilo::res;
use teksilo::widgets::IconWidget;

/// Match case — an uppercase A beside a lowercase a.
pub fn case_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/case.svg"))
}

/// Whole word — a word bounded on both sides.
pub fn whole_word_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/whole-word.svg"))
}

/// Match accents (diacritic-sensitive) — an accented letter.
pub fn diacritics_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/diacritics.svg"))
}

/// Body scope — paragraph lines of prose.
pub fn body_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/body.svg"))
}

/// Title scope — a capital T (a heading).
pub fn title_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/title.svg"))
}

/// Synopsis scope — a summary card.
pub fn synopsis_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/synopsis.svg"))
}

/// Label scope — a tag.
pub fn label_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/label.svg"))
}

/// Replace — two opposing arrows (swap one string for another). Distinct from the
/// search magnifier so the "show replace" toggle doesn't read as another search.
pub fn replace_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/replace.svg"))
}

/// Previous match — an up chevron (the find banner's ↑).
pub fn nav_prev_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/nav-prev.svg"))
}

/// Next match — a down chevron (the find banner's ↓).
pub fn nav_next_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/nav-next.svg"))
}
