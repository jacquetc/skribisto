// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Icons for the Versions dock's pin control — a drawing pin, filled when the
//! version is kept and outlined when it is not. One shape in two weights rather
//! than two shapes: the control says what the row *would become*, and a different
//! glyph would read as a different action. `res!` embeds each asset at compile
//! time and needs a literal path per call site, so there is one function per icon.

use bastyde::res;
use bastyde::widgets::IconWidget;

/// This backup is pinned: the automatic cleanup will never delete it.
///
/// A guarantee about the *file*, held by identity rather than by date, so it
/// survives a retention policy being tightened underneath it. It is not
/// protection from a writer deleting the backup themselves from the Backups
/// list — that stays deliberate and immediate.
pub fn pinned() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/versions/pinned.svg"))
}

/// This backup is not pinned — the automatic cleanup may sweep it away in time.
pub fn unpinned() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/versions/unpinned.svg"))
}

/// Narrow to the recent end — the "last 30 days" preset.
///
/// One glyph for both surfaces that offer it: the same control in the Versions
/// rail and in the Timeline band has to be the same picture, or it reads as two
/// different filters that happen to sit in similar places.
pub fn recent() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/versions/recent.svg"))
}

/// Drop the window and show everything again.
pub fn reset() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/versions/reset.svg"))
}

#[cfg(test)]
mod tests {
    use bastyde::canvas::SvgIcon;
    use bastyde::core::widget_tree::WidgetTree;
    use bastyde::prelude::SizeProposal;

    /// Every glyph really produces geometry.
    ///
    /// **Laying out is not drawing**, which is what the earlier version of this
    /// test got wrong: `IconWidget` falls back to an *empty* path when the SVG
    /// fails to parse, and an empty path still lays out at its declared size. A
    /// button with nothing in it passed. `SvgIcon::parse` is what actually
    /// answers the question, so the assertion is on that.
    #[test]
    fn every_versions_glyph_produces_geometry() {
        let sources = [
            (
                "pinned",
                include_str!("../../assets/icons/versions/pinned.svg"),
            ),
            (
                "unpinned",
                include_str!("../../assets/icons/versions/unpinned.svg"),
            ),
            (
                "recent",
                include_str!("../../assets/icons/versions/recent.svg"),
            ),
            (
                "reset",
                include_str!("../../assets/icons/versions/reset.svg"),
            ),
        ];
        for (name, svg) in sources {
            let icon = SvgIcon::parse(svg).unwrap_or_else(|e| panic!("{name} does not parse: {e}"));
            assert!(!icon.is_empty(), "the {name} icon parses to no geometry");
        }
    }

    /// …and each one lays out at the size a compact icon button gives it.
    #[test]
    fn every_versions_glyph_lays_out() {
        for (name, icon) in [
            ("pinned", super::pinned()),
            ("unpinned", super::unpinned()),
            ("recent", super::recent()),
            ("reset", super::reset()),
        ] {
            let mut tree = WidgetTree::new();
            let id = tree.add_boxed(Box::new(icon));
            tree.layout(SizeProposal::exact(16.0, 16.0));
            assert!(
                tree.bounds(id).width > 0.0,
                "the {name} icon laid out to nothing",
            );
        }
    }
}
