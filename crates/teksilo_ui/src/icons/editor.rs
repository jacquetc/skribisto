// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor-chrome icons — the split-view toggle glyphs for the tab-strip trailing
//! slots. Like [`binder_icons`](crate::binder::icons), `res!` embeds each asset at
//! compile time (a literal path per call site) and the icon follows the theme via
//! `TextRole` tinting.

use teksilo::res;
use teksilo::widgets::IconWidget;

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

/// A one-way arrow with the way back struck out — the status bar's "Always
/// forward is being played, deleting is disabled" warning glyph.
pub fn forward_only() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/forward-only.svg")).icon_size(ICON_SIZE)
}

/// Save glyph with a filled dot — the status bar's "there are unsaved changes".
pub fn save_unsaved() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/save-unsaved.svg")).icon_size(ICON_SIZE)
}

/// Save glyph with a check — the status bar's "everything is on disk".
pub fn save_saved() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/save-saved.svg")).icon_size(ICON_SIZE)
}

/// Spell-check **on** — the title-bar toggle's glyph when checking is enabled.
pub fn spellcheck_on() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/spellcheck.svg")).icon_size(ICON_SIZE)
}

/// Spell-check **off** — the same glyph struck through. A separate asset rather than a tint:
/// the control must read as off at a glance and against either theme, and colour alone would
/// carry the whole meaning (which it must never do — see the title bar's other toggles).
pub fn spellcheck_off() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/spellcheck-off.svg")).icon_size(ICON_SIZE)
}

/// "Synopsis beside the manuscript" — the distraction-free strip's toggle for the
/// Side synopsis column.
pub fn synopsis_side() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/synopsis-side.svg")).icon_size(ICON_SIZE)
}

/// "Fold the synopsis column away" — the button in the Side pane's own header, so
/// a writer can reclaim the width for one document without a trip to Settings.
pub fn synopsis_collapse() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/synopsis-collapse.svg"))
        .icon_size(ICON_SIZE)
}

/// A drawing pin — the leading glyph of a **pinned** editor tab, standing in for
/// its sub-role icon.
///
/// The icon is the only channel a pin can use here. The title has to stay: a
/// manuscript's tabs are told apart by name and nothing else — twenty chapters
/// wearing the same document glyph — which is why the strip is clamped to
/// `MIN_EDITOR_TAB_WIDTH` (160 dp) instead of collapsing pinned tabs to
/// icon-only squares. And colour is already spoken for: a trashed item opened
/// from the Trash dock tints its tab icon warning-orange, so a second colour
/// meaning would either be lost under that tint or fight it for the same pixels.
///
/// So this glyph replaces the sub-role icon and is drawn in `currentColor`: it
/// *rides* whatever tint the tab already carries rather than competing with it,
/// and a tab that is both pinned and trashed still reads as both.
pub fn pinned_tab() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/editor/pinned-tab.svg")).icon_size(ICON_SIZE)
}

#[cfg(test)]
mod tests {
    use teksilo::canvas::SvgIcon;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::prelude::SizeProposal;

    /// Every glyph really produces geometry.
    ///
    /// This is the load-bearing half of the pair, and the reason it is a
    /// separate test from the layout check below: **laying out is not drawing**.
    /// `IconWidget` takes its design size from the viewBox, so an icon that
    /// parses to no path at all still lays out at exactly its declared 16 dp and
    /// looks perfectly healthy from the widget tree. A layout-only test would
    /// pass on a file with nothing in it.
    ///
    /// Belt and braces on purpose. `res!` already parses each asset at compile
    /// time and rejects one carrying no drawable geometry, so today a malformed
    /// file is a *compile* error rather than a red test — which is also why this
    /// list must name the same files the `res!` call sites above do, or it stops
    /// covering them. That guarantee lives in teksilo, though, and this test is
    /// what states the invariant on our side of the seam: if `res!` ever stops
    /// validating, or a glyph is one day loaded some other way, `SvgIcon::parse`
    /// plus a non-empty check is still the only thing that answers "is there a
    /// picture in there".
    ///
    /// Its reach is worth stating rather than overselling: `SvgIcon::is_empty`
    /// asks whether the file yielded *any* fill or stroke entry, so a `<path>`
    /// that kept its `stroke` but lost its `d` still counts as geometry. This
    /// catches an asset that is empty, truncated or not SVG at all — not one
    /// whose drawing was silently emptied a path at a time.
    #[test]
    fn every_editor_glyph_produces_geometry() {
        let sources = [
            ("split", include_str!("../../assets/icons/editor/split.svg")),
            (
                "close_split",
                include_str!("../../assets/icons/editor/split-close.svg"),
            ),
            (
                "forward_only",
                include_str!("../../assets/icons/editor/forward-only.svg"),
            ),
            (
                "save_unsaved",
                include_str!("../../assets/icons/editor/save-unsaved.svg"),
            ),
            (
                "save_saved",
                include_str!("../../assets/icons/editor/save-saved.svg"),
            ),
            (
                "spellcheck_on",
                include_str!("../../assets/icons/editor/spellcheck.svg"),
            ),
            (
                "spellcheck_off",
                include_str!("../../assets/icons/editor/spellcheck-off.svg"),
            ),
            (
                "synopsis_side",
                include_str!("../../assets/icons/editor/synopsis-side.svg"),
            ),
            (
                "synopsis_collapse",
                include_str!("../../assets/icons/editor/synopsis-collapse.svg"),
            ),
            (
                "pinned_tab",
                include_str!("../../assets/icons/editor/pinned-tab.svg"),
            ),
        ];
        for (name, svg) in sources {
            let icon = SvgIcon::parse(svg).unwrap_or_else(|e| panic!("{name} does not parse: {e}"));
            assert!(!icon.is_empty(), "the {name} icon parses to no geometry");
        }
    }

    /// …and each one lays out at the size the tab strip gives it.
    #[test]
    fn every_editor_glyph_lays_out() {
        for (name, icon) in [
            ("split", super::split()),
            ("close_split", super::close_split()),
            ("forward_only", super::forward_only()),
            ("save_unsaved", super::save_unsaved()),
            ("save_saved", super::save_saved()),
            ("spellcheck_on", super::spellcheck_on()),
            ("spellcheck_off", super::spellcheck_off()),
            ("synopsis_side", super::synopsis_side()),
            ("synopsis_collapse", super::synopsis_collapse()),
            ("pinned_tab", super::pinned_tab()),
        ] {
            let mut tree = WidgetTree::new();
            let id = tree.add_boxed(Box::new(icon));
            tree.layout(SizeProposal::exact(super::ICON_SIZE, super::ICON_SIZE));
            assert!(
                tree.bounds(id).width > 0.0,
                "the {name} icon laid out to nothing",
            );
        }
    }
}
