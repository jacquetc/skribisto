// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The one way this app draws "how far along is this against its target".
//!
//! Four surfaces show it — the Inspector's readout under the target field, a container
//! page's own header, the status bar beside the live count, and the Distribute preview's
//! footer — and they all come through here. Scrivener's outliner and its editor footer draw
//! the same fact two different ways (the outliner silently drops the over-target state the
//! footer has), which is a filed complaint against it and exactly the drift a shared widget
//! prevents.
//!
//! Everything here is a plain builder rather than `teksu!`, matching the files that host it
//! (`docks/inspector.rs`, `statusbar/`, `tabs/pace/` are all chained builders).

use teksilo::prelude::*;
use teksilo::tokens::{Orientation, SurfaceRole};
use teksilo::widgets::{FixedSize, HStack, ProgressBar, TextWidget};

use frontend::common::entities::GoalUnit;

use crate::goals::format::{format_count, format_goal};
use crate::goals::progress;

/// Width of the inline bar. Narrow on purpose: it is a glance, and the numbers beside it
/// are the precise answer.
pub const BAR_WIDTH: f32 = 64.0;
/// Thickness matching the writing-session gauge, so two bars in the same status bar do not
/// read as two different kinds of thing.
pub const BAR_THICKNESS: f32 = 4.0;

/// "1 234 of 2 000 words", in the project's unit.
pub fn progress_label(written: i64, goal: i64, unit: &GoalUnit) -> LocalizedString {
    let count = format_goal(written);
    let goal_text = format_goal(goal);
    let g = goal;
    // `$g` carries the bare number for the plural rule; `$goal` carries what is printed.
    // Grouping the digits first would leave Fluent selecting on a string.
    match unit {
        GoalUnit::Words => tr!(goal_progress_words(count = count, goal = goal_text, g = g)),
        GoalUnit::Characters => {
            tr!(goal_progress_characters(
                count = count,
                goal = goal_text,
                g = g
            ))
        }
    }
}

/// "1 234 words" — the same phrasing without a target, for a row that carries none.
pub fn count_label(written: usize, unit: &GoalUnit) -> LocalizedString {
    let count = format_count(written);
    let n = written as i64;
    match unit {
        GoalUnit::Words => tr!(goal_count_words(count = count, n = n)),
        GoalUnit::Characters => tr!(goal_count_characters(count = count, n = n)),
    }
}

/// The bar alone, coloured by how far along the writing is.
///
/// Fixed width rather than filling: in a status bar it sits between two text items, and a
/// greedy bar there would push the count off the strip.
pub fn bar(written: i64, goal: i64) -> impl Widget + use<> {
    let ratio = progress::ratio(written, goal).unwrap_or(0.0);
    FixedSize::new().width(BAR_WIDTH).child(
        ProgressBar::new(progress::bar_fill(ratio))
            .orientation(Orientation::Horizontal)
            .thickness(BAR_THICKNESS)
            .fill_color(ColorProp::from(progress::target_role(ratio)))
            .track_color(SurfaceRole::Sunken),
    )
}

/// Bar plus label, the composite the Inspector and the container pages use.
///
/// Renders the plain count with no bar when there is no target: a bar against nothing would
/// be a progress indicator for a journey with no destination.
pub fn line(written: usize, goal: i64, unit: &GoalUnit) -> impl Widget + use<> {
    let row = HStack::new().spacing(8.0);
    if goal > 0 {
        row.child(bar(written as i64, goal)).child(
            TextWidget::new(progress_label(written as i64, goal, unit))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary)
                .single_line(),
        )
    } else {
        row.child(
            TextWidget::new(count_label(written, unit))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary)
                .single_line(),
        )
    }
}
