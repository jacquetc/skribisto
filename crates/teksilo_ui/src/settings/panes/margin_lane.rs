// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Margin marks — the strip beside the scroll bar, and what it shows.
//!
//! ## Why this page iterates a registry rather than listing switches
//!
//! Every row under "What it marks" comes from
//! [`margin_lane::registered`](crate::margin_lane::registered), including rows
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

use teksilo::prelude::*;
use teksilo::widgets::{Center, FixedSize, HStack, RectWidget, VStack};

use crate::margin_lane::{self, LaneSurface};

#[allow(unused_imports)]
use super::super::*;

/// A small square of a provider's own colour, so the page and the strip agree.
///
/// Resolved through [`margin_lane::resolve_slot`] rather than from the spec, for
/// the same reason the lane resolves it: a provider names a palette slot, and
/// the theme decides what that looks like.
fn swatch(colors: &teksilo::tokens::ColorTokens, slot: u8) -> impl Widget {
    let fill = margin_lane::resolve_slot(colors, slot);
    // The same shape a tag's swatch uses, and for the same recorded reason: the
    // hairline is derived from the fill rather than taken from a border token,
    // because no token clears 1.4.11's 3:1 against an arbitrary fill. `FixedSize`
    // inside a `Center` rather than a minimum, or the greedy rect stretches to
    // the row's full height instead of staying a dot.
    Center::new().child(
        FixedSize::new().width(10.0).height(10.0).child(
            RectWidget::new()
                .background(fill)
                .corner_radius(teksilo::tokens::CornerRadius::uniform(2.0))
                .border_color(crate::tags::contrast::outline_on(fill))
                .border_width(1.0),
        ),
    )
}

pub(in crate::settings) fn margin_lane_pane(ctx: &mut BuildContext) -> impl Widget {
    let store = ctx.settings();
    let colors = ctx.theme().colors.clone();

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
        .full_width(Toggle::new(enabled.clone()).label(tr!(settings_margin_lane_enabled())))
        .full_width(hint(tr!(settings_margin_lane_enabled_hint())))
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
            form = form
                .full_width(
                    HStack::new()
                        .spacing(8.0)
                        .child(swatch(&colors, spec.palette_slot))
                        .child(Toggle::new(signal).label((spec.label)())),
                )
                .full_width(hint((spec.hint)()));
        }
    }

    // ── the texture column ───────────────────────────────────────────────────
    form = form
        .full_width(group(tr!(settings_group_margin_lane_texture())))
        .full_width(Toggle::new(texture).label(tr!(settings_margin_lane_texture())))
        .full_width(hint(tr!(settings_margin_lane_texture_hint())));

    // ── where it appears ─────────────────────────────────────────────────────
    form = form.full_width(group(tr!(settings_group_margin_lane_surfaces())));
    for surface in LaneSurface::all() {
        let signal = store.signal(
            &crate::margin_lane_surface_key(surface),
            crate::margin_lane_surface_default(surface),
        );
        form = form.full_width(Toggle::new(signal).label(surface_label(surface)));
    }
    form = form.full_width(hint(tr!(settings_margin_lane_surfaces_hint())));

    pane_frame(
        crumb(
            Some(tr!(settings_sec_editor())),
            tr!(settings_page_margin_lane()),
        ),
        VStack::new().child(form),
    )
}

fn surface_label(surface: LaneSurface) -> teksilo::i18n::LocalizedString {
    match surface {
        LaneSurface::Editor => tr!(settings_margin_lane_surface_editor()),
        LaneSurface::Stream => tr!(settings_margin_lane_surface_stream()),
        LaneSurface::SearchPreview => tr!(settings_margin_lane_surface_search_preview()),
    }
}
