// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Margin marks — the strip beside the scroll bar, and what it shows.
//!
//! ## Why this page iterates a registry rather than listing switches
//!
//! Every row under "What it marks" comes from
//! [`crate::margin_lane::registered`], including rows
//! this crate did not write. An extension registers a provider and its row
//! appears here, with its own label, its own hint and its own swatch, without
//! the community edition knowing what it is.
//!
//! That is deliberate, and it is what a settings *page* seam would not have
//! given. A registered page can only ever land under a fixed Extensions
//! section, on purpose — letting a registration name a parent would make the
//! settings tree's shape a compatibility promise. So the lane's own page lives
//! here, in the app's tree where it belongs, and extensions contribute **rows**
//! rather than a page of their own. The provider list and the settings list are
//! then the same list, and cannot disagree.
//!
//! ## Why the explanations are tooltips and not lines on the page
//!
//! This page is a list of switches, and a paragraph under each one buries the
//! list it is explaining: a writer scanning for the switch they came to flip
//! reads three sentences to find each of them. So the switches are the page, and
//! what each one means is on the switch — a plain tooltip where a line is
//! enough, a rich one where there is a second thing worth saying underneath.
//!
//! Nothing is lost to a screen reader by that: a tooltip's text is harvested as
//! the control's accessible description whether or not it is showing.

use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{Center, FixedSize, HStack, RectWidget, VStack};

use crate::margin_lane::{self, LaneSurface};

#[allow(unused_imports)]
use super::super::*;

/// A small square of a provider's own colour, so the page and the strip agree.
///
/// Resolved through [`margin_lane::resolve_slot`] rather than from the spec, for
/// the same reason the lane resolves it: a provider names a palette slot, and
/// the theme decides what that looks like.
///
/// **Bound to the theme signal, never resolved at build time.** `WidgetTree::set_theme`
/// deliberately does not rebuild, so a colour cloned out of `ctx.theme()` in `build`
/// is frozen at whatever palette happened to be in force when the page was
/// constructed. On *this* page that is the likeliest failure in the whole window:
/// the reader flips Light ⇄ Dark on the Appearance page one click away and comes
/// back to swatches still painted in the old palette, and only reopening Settings
/// clears it.
fn swatch(theme: &Signal<teksilo::core::styles::Theme>, slot: u8) -> impl Widget {
    let fill = theme.map(move |t| margin_lane::resolve_slot(&t.colors, slot));
    // The same shape a tag's swatch uses, and for the same recorded reason: the
    // hairline is derived from the fill rather than taken from a border token,
    // because no token clears 1.4.11's 3:1 against an arbitrary fill. `FixedSize`
    // inside a `Center` rather than a minimum, or the greedy rect stretches to
    // the row's full height instead of staying a dot.
    let outline = theme.map(move |t| {
        crate::tags::contrast::outline_on(margin_lane::resolve_slot(&t.colors, slot))
    });
    Center::new().child(
        FixedSize::new().width(10.0).height(10.0).child(
            RectWidget::new()
                .background(fill)
                .corner_radius(teksilo::tokens::CornerRadius::uniform(2.0))
                .border_color(outline)
                .border_width(1.0),
        ),
    )
}

pub(in crate::settings) fn margin_lane_pane(
    ctx: &mut BuildContext,
    crumbs: &Crumbs,
) -> impl Widget {
    let store = ctx.settings();
    let theme = ctx.theme_signal().clone();

    let enabled = store.signal(
        crate::MARGIN_LANE_ENABLED_KEY,
        crate::MARGIN_LANE_ENABLED_DEFAULT,
    );
    let texture = store.signal(
        crate::MARGIN_LANE_TEXTURE_KEY,
        crate::MARGIN_LANE_TEXTURE_DEFAULT,
    );

    let mut form = FormLayout::new()
        .label(tr!(settings_page_margin_lane()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(
            Toggle::new(enabled.clone())
                .label(tr!(settings_margin_lane_enabled()))
                .rich_tooltip_content(
                    TooltipContent::new(
                        "settings.margin_lane_enabled",
                        tr!(settings_margin_lane_enabled_hint()),
                    )
                    // The promise, behind the disclosure: it is the reason the
                    // lane is shaped the way it is, and it is not what someone
                    // hovering the switch came to find out.
                    .with_more(tr!(settings_margin_lane_enabled_more())),
                ),
        )
        .full_width(group(tr!(settings_group_margin_lane_marks())));

    // ── one row per registered provider ──────────────────────────────────────
    //
    // Read as a snapshot when the page is built, which is the same contract
    // every other registry in this seam has. Registration happens at startup,
    // so a page built afterwards sees the whole set.
    let providers = margin_lane::registered();
    if providers.is_empty() {
        form = form.full_width(hint(tr!(settings_margin_lane_no_providers())));
    } else {
        for spec in providers {
            let signal = store.signal(&spec.settings_key(), spec.default_on);
            // A provider's hint is one line by construction (it names what the
            // marks are), so a plain tooltip carries it whole. On the toggle
            // rather than the row: the label is what a pointer aims at.
            form = form.full_width(
                HStack::new()
                    .spacing(8.0)
                    .child(swatch(&theme, spec.palette_slot))
                    .child(
                        Toggle::new(signal)
                            .label((spec.label)())
                            .tooltip((spec.hint)()),
                    ),
            );
        }
    }

    // ── the texture column ───────────────────────────────────────────────────
    form = form
        .full_width(group(tr!(settings_group_margin_lane_texture())))
        .full_width(
            Toggle::new(texture)
                .label(tr!(settings_margin_lane_texture()))
                .rich_tooltip_content(
                    TooltipContent::new(
                        "settings.margin_lane_texture",
                        tr!(settings_margin_lane_texture_hint()),
                    )
                    // The caveat, behind the disclosure: it applies to some
                    // languages and not others, and it is not the answer to
                    // "what is this".
                    .with_more(tr!(settings_margin_lane_texture_more())),
                ),
        );

    // ── where it appears ─────────────────────────────────────────────────────
    form = form.full_width(group(tr!(settings_group_margin_lane_surfaces())));
    for surface in LaneSurface::all() {
        let signal = store.signal(
            &crate::margin_lane_surface_key(surface),
            crate::margin_lane_surface_default(surface),
        );
        form = form.full_width(Toggle::new(signal).label(surface_label(surface)));
    }

    pane_frame(crumbs.of(Pane::MarginLane), VStack::new().child(form))
}

fn surface_label(surface: LaneSurface) -> teksilo::i18n::LocalizedString {
    match surface {
        LaneSurface::Editor => tr!(settings_margin_lane_surface_editor()),
        LaneSurface::Stream => tr!(settings_margin_lane_surface_stream()),
        LaneSurface::SearchPreview => tr!(settings_margin_lane_surface_search_preview()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::widget_tree::WidgetTree;

    /// Flipping Light ⇄ Dark must move the swatches.
    ///
    /// `WidgetTree::set_theme` deliberately does not rebuild, so a colour taken
    /// from `ctx.theme().colors` in `build` is frozen for the life of the page —
    /// and the Appearance page that flips the theme is one click away from this
    /// one, which made this the likeliest stale paint in the window. The contrast
    /// is the point of the assertion: the cloned palette below is what the page
    /// used to hold, and it does not move.
    #[test]
    fn a_swatch_colour_follows_a_theme_change_with_no_rebuild() {
        let mut tree = WidgetTree::new().with_theme(crate::style::light());
        let theme = tree.theme_signal().clone();

        // What the page used to capture at build time…
        let frozen = margin_lane::resolve_slot(&tree.theme().colors, 0);
        // …and what it binds now.
        let bound = theme.map(|t| margin_lane::resolve_slot(&t.colors, 0));
        let before = bound.get();
        assert_eq!(before, frozen, "the two must start out agreeing");

        tree.set_theme(crate::style::dark());

        assert_ne!(
            bound.get(),
            frozen,
            "the swatch must repaint in the new palette; a build-time clone would \
             still be showing {frozen:?}"
        );
    }
}
