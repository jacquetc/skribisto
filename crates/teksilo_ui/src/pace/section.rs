// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Pace feature's one card shape, shared by the planner and the summary panel.
//!
//! The Book's Pace segment draws its five dashboard sections with it
//! ([`crate::tabs::pace`]), and the "where the book stands" summary
//! ([`super::panel`]) draws its blocks with the same call — so the card a writer meets
//! on the way into a project is the card they meet again inside the Book's own planner,
//! down to the padding and the heading style. It lived in `tabs/pace.rs` as a private
//! `panel_section` while the planner was its only caller; a second caller is what moves
//! it here, under the rule that what more than one surface uses belongs to neither.

use teksilo::prelude::*;
use teksilo::widgets::{Padding, Panel, TextWidget, VStack};

/// A card's inner padding. Also what the summary panel pads its column by, so a card's
/// text and the panel's own edge keep one rhythm.
pub(crate) const CARD_PADDING: f32 = 14.0;

/// One card with a heading over its body.
///
/// The heading is `single_line`: the planner's own headings are short constants, but the
/// summary titles each card with a **book title**, which is data and can be any length.
/// Wrapping one would grow the card under the numbers it is meant to introduce.
pub(crate) fn panel_section(title: LocalizedString, body: impl Widget + 'static) -> impl Widget {
    panel_card(
        VStack::new()
            .spacing(10.0)
            .child(
                TextWidget::new(title)
                    .style(TextStyleRole::BodyBold)
                    .color(TextRole::Primary)
                    .single_line(),
            )
            .child(body),
    )
}

/// The same card with no heading — for a body that already carries its own.
pub(crate) fn panel_card(body: impl Widget + 'static) -> impl Widget {
    Panel::new().child(Padding::uniform(CARD_PADDING).child(body))
}
