// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The body of a tag's hover tooltip: swatch, name, and what the tag means.
//!
//! This is what makes dot-only rendering defensible. A dot with no name is decoration; a dot
//! whose tooltip carries the name *and* the description is a real affordance — and tag
//! meaning decays fast ("Thread B" is opaque six months later), which is why `details`
//! exists at all.
//!
//! Composite rather than rich: the body is a widget tree (a swatch beside text), not a
//! string. It is attached through
//! [`attach_labelled_composite_tooltip`](crate::widgets::attach_labelled_composite_tooltip)
//! so it carries a real accessible name instead of announcing "Tooltip".

use teksilo::prelude::*;
use teksilo::widgets::{Center, FixedSize, HStack, RectWidget, TextWidget, VStack};

use crate::models::TagRow;
use crate::tags::contrast;

/// Diameter of the tooltip's colour dot.
const SWATCH: f32 = 10.0;

// The body **must** wrap rather than run: horizontal overflow in a composite tooltip is
// silently clipped, not scrolled. `TextWidget` wraps within whatever width it is given, and
// the tooltip surface bounds that, so no explicit cap is needed here.

/// One tag's tooltip body.
pub fn tag_tooltip_body(tag: &TagRow) -> impl Widget {
    let fill = contrast::parse(&tag.color);

    // Both lines take `TooltipText`, the role for the tooltip's own surface — not the
    // content-surface roles a body normally uses. This is not a nicety: the tooltip
    // background is dark in *both* themes (`tooltip_bg` #1E1F22), while light-theme
    // `text_primary` is #000000. Defaulting the name to Primary painted black on near-black
    // at 1.27:1, and Secondary on the details line reached only 2.75:1. Hierarchy between
    // the two lines comes from the type scale (Body vs Tiny), which is surface-independent.
    let mut text = VStack::new().spacing(2.0).child(
        TextWidget::new(lit!(tag.name.clone()))
            .style(TextStyleRole::Body)
            .color(TextRole::TooltipText),
    );

    // The details line is omitted entirely rather than left blank, so a tag with no
    // description gets a tight one-line tooltip instead of one with a hole in it.
    if !tag.details.trim().is_empty() {
        text = text.child(
            TextWidget::new(lit!(tag.details.clone()))
                .style(TextStyleRole::Tiny)
                .color(TextRole::TooltipText),
        );
    }

    // The swatch is decorative — the name is right beside it and carries the meaning. A
    // `RectWidget` contributes no accessibility node of its own, so it needs no explicit
    // hiding; the tooltip's own `access_label` is what a screen reader announces.
    // The hairline is derived from the fill, not taken from a border token, for the same
    // reason as everywhere else a tag colour is drawn: no token clears SC 1.4.11's 3:1
    // against an arbitrary fill, and `BorderRole::Default` here would additionally be a
    // content-surface token sitting on the tooltip surface.
    // `FixedSize` inside a `Center`, not `MinSize`: a minimum only floors the size, so the
    // greedy `RectWidget` would stretch to the full height of the tooltip body instead of
    // staying a dot.
    let swatch = Center::new().child(
        FixedSize::new().width(SWATCH).height(SWATCH).child(
            RectWidget::new()
                .background(fill)
                .corner_radius(teksilo::tokens::CornerRadius::uniform(9999.0))
                .border_color(contrast::outline_on(fill))
                .border_width(1.0),
        ),
    );

    HStack::new().spacing(8.0).child(swatch).child(text)
}
