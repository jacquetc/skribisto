// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Icons for the comment docks' sort toggle.
//!
//! Line-style, `stroke="currentColor"`, so they follow the theme and the toggle
//! tint — the same contract as `icons::find`. `res!` embeds each asset at compile
//! time and needs a literal path per call site, so there is one function per icon.
//!
//! Only the *sort* is an icon. The four filters keep their words: they are a
//! single-choice facet whose current value has to be readable at a glance, and
//! four invented glyphs for "all / open / resolved / orphaned" would be four
//! things to learn where four short words are already understood. The sort is the
//! opposite case — two states, and its labels ("In document order" / "Dans
//! l'ordre du document") were the single widest thing in the dock.

use teksilo::res;
use teksilo::widgets::IconWidget;

/// In document order — lines of prose with a downward arrow beside them.
pub fn sort_document_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/comments/sort-document.svg"))
}

/// Newest first — a clock: the review-pass question is *when*, not *where*.
pub fn sort_newest_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/comments/sort-newest.svg"))
}
