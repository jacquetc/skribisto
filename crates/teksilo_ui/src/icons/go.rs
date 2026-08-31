// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Icons for the Go menu / distraction-free strip's Next/Previous pair
//! (Increment 4 of distraction-free) — line-style, `stroke="currentColor"`, same
//! shape family as [`super::find`]'s nav chevrons, but owned by this feature
//! rather than reached into `find`'s: down for Next, up for Previous, matching
//! the `go.next`/`go.prev` shortcut pair's own Alt+Down/Alt+Up axis.

use teksilo::res;
use teksilo::widgets::IconWidget;

/// Next item (in binder order) — a down chevron, matching Alt+Down.
pub fn next_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/go/next.svg"))
}

/// Previous item (in binder order) — an up chevron, matching Alt+Up.
pub fn prev_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/go/prev.svg"))
}

/// "Go to…" — a target/crosshair. Deliberately not a magnifier: this jumps to
/// a known item, it does not search prose, and the Find dock already owns the
/// magnifier. Two glyphs that read alike for two different jobs is worse than
/// one that is slightly less obvious.
pub fn go_to_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/go/go-to.svg"))
}
