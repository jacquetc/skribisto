// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Small secondary-text builders shared across features.
//!
//! [`caption`] is the one treatment; [`field_label`] and [`hint`] name the two
//! intents that reach for it, because a form reads better when the call says
//! which of the two it means. They existed as three identical private copies —
//! in `settings.rs`, `panels/new_work.rs` and `export/panel.rs` — plus a fourth
//! in `tabs/analysis.rs` under a third name.

use teksilo::prelude::*;
use teksilo::tokens::{TextRole, TextStyleRole};
use teksilo::widgets::{Padding, TextWidget};

/// Small, secondary-coloured text: the app's caption treatment.
pub fn caption(text: impl Into<LocalizedString>) -> TextWidget {
    TextWidget::new(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary)
}

/// The label above a form field.
pub fn field_label(text: impl Into<LocalizedString>) -> TextWidget {
    caption(text)
}

/// The dimmed line below a form field explaining what it does.
pub fn hint(text: impl Into<LocalizedString>) -> TextWidget {
    caption(text)
}

/// A dock's empty-state line: secondary text with room to breathe around it.
///
/// Distinct from [`caption`] — it keeps the body text size and adds the 16 dp
/// inset that stops "nothing here yet" from sitting against the dock edge.
pub fn dock_note(text: impl Into<LocalizedString>) -> impl Widget {
    Padding::uniform(16.0).child(TextWidget::new(text).color(TextRole::Secondary))
}
