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

use bastyde::prelude::*;
use bastyde::widgets::{HStack, MinSize, RectWidget, TextWidget, VStack};

use crate::models::TagRow;
use crate::tags::contrast;

// The body **must** wrap rather than run: horizontal overflow in a composite tooltip is
// silently clipped, not scrolled. `TextWidget` wraps within whatever width it is given, and
// the tooltip surface bounds that, so no explicit cap is needed here.

/// One tag's tooltip body.
pub fn tag_tooltip_body(tag: &TagRow) -> impl Widget {
    let fill = contrast::parse(&tag.color);

    let mut text = VStack::new().spacing(2.0).child(
        TextWidget::new(lit!(tag.name.clone())).style(TextStyleRole::Body),
    );

    // The details line is omitted entirely rather than left blank, so a tag with no
    // description gets a tight one-line tooltip instead of one with a hole in it.
    if !tag.details.trim().is_empty() {
        text = text.child(
            TextWidget::new(lit!(tag.details.clone()))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Secondary),
        );
    }

    // The swatch is decorative — the name is right beside it and carries the meaning. A
    // `RectWidget` contributes no accessibility node of its own, so it needs no explicit
    // hiding; the tooltip's own `access_label` is what a screen reader announces.
    let swatch = MinSize::new(10.0, 10.0).child(
        RectWidget::new()
            .background(fill)
            .corner_radius(bastyde::tokens::CornerRadius::uniform(9999.0))
            .border_color(BorderRole::Default)
            .border_width(1.0),
    );

    HStack::new().spacing(8.0).child(swatch).child(text)
}
