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

/// Epigraph scope — an opening quotation mark over the passage it introduces.
///
/// Its own glyph rather than [`body_icon`]'s, although the *scope toggle* gates
/// epigraphs together with the prose. The toggle answers "search here as well";
/// this answers "your hit is here", and drawing an epigraph as prose would make
/// the one column whose job is to distinguish them refuse to.
pub fn epigraph_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/epigraph.svg"))
}

/// Comment reply scope — one bubble and the arrow that answers it.
///
/// Distinct from [`activity::comments_icon`](crate::icons::activity::comments_icon),
/// which is two bubbles and means *a thread*. In a search result the thread is
/// the row above; this says which end of it the hit is at, and a reply and the
/// remark it answers are two different places to send a writer to.
pub fn comment_reply_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/comment-reply.svg"))
}

/// Collapse all — a chevron folding a list upward.
///
/// There is deliberately no matching "expand all". Expanding every item of a
/// search means fetching every occurrence of every field, which is the work the
/// results tree is lazy precisely to avoid: the row set is capped at ten thousand
/// fields, the occurrences under them are not capped at all. Collapsing is the
/// gesture that earns its place anyway — it is what a writer reaches for when a
/// long result has buried the shape of it.
pub fn collapse_all_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/collapse-all.svg"))
}

/// Dismiss — a cross, for taking one result out of the list.
///
/// Not a bin: nothing is deleted. The hit stays in the manuscript, the row comes
/// back with one undo, and what a dismissal leaves is the *review*.
pub fn dismiss_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/dismiss.svg"))
}

/// Bring back the last dismissal — an arrow turning back on itself.
///
/// The undo curve rather than a second cross: what it restores is a row, and a
/// glyph mirroring [`dismiss_icon`] would read as dismissing something else.
pub fn undo_dismiss_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/undo-dismiss.svg"))
}

/// Preserve case — an uppercase A and a lowercase a, carried across by an arrow.
///
/// Built on the same two letterforms as [`case_icon`], because the two options are
/// about the same property of a match and a writer reads them side by side. The
/// arrow underneath is the difference: this one is about the case being *carried*
/// into the replacement, not about it being compared.
pub fn preserve_case_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/find/preserve-case.svg"))
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
