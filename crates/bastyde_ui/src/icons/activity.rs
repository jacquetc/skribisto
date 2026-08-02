// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

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

/// The Trash activity icon: a wastebasket. Fronts the leading rail's third tab
/// (beside Outline and Search) and the trash panel's Empty Trash… button.
pub fn trash_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/activities/trash.svg"))
}

/// The Comments activity icon: a speech bubble with a second bubble behind it.
///
/// Two overlapping bubbles rather than one, because both docks list *threads*
/// (a comment plus its replies), not single notes — and because the trailing rail
/// already carries the Inspector's panel silhouette, so a second outline shape
/// there would be indistinguishable at rail size.
pub fn comments_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/activities/comments.svg"))
}

/// The Format activity icon: a capital A over a baseline rule.
///
/// Deliberately a *type* glyph rather than another panel outline — it shares the
/// trailing rail with the Inspector, and two side-panel silhouettes there would
/// be indistinguishable at rail size.
pub fn format_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/activities/format.svg"))
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

/// The Settings cog. Unlike every other glyph here it fronts no activity — it is
/// the leading rail's pinned `DockAction`, sitting past the spacer at the bottom
/// of the bar (the VS Code Manage-gear position). A cog rather than a sixth
/// panel silhouette, so it reads as "a command" and not "one more dock".
pub fn settings_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/activities/settings.svg"))
}
