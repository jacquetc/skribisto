// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Words or characters: the project's counting unit, asked once in New Work and changeable
//! later in Settings.
//!
//! Two call sites, which is exactly the bar for living here rather than beside either of
//! them. They must ask the same question in the same words: a writer who sets it while
//! creating a project and later goes looking for it in Settings should recognise the
//! control, not have to work out whether it is the same setting.
//!
//! A `SegmentedControl` rather than a `Toggle`, following the Export panel's Format
//! picker: two *named* options, neither of which is the other switched off. "Characters"
//! is not "Words, disabled".

use teksilo::prelude::*;
use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{Segment, SegmentSizing, SegmentedControl};

use frontend::common::entities::GoalUnit;

use crate::tooltip_registry::GOAL_UNIT;

/// Positional index for the control, since the list is closed and two long.
pub fn index_of(unit: &GoalUnit) -> usize {
    match unit {
        GoalUnit::Words => 0,
        GoalUnit::Characters => 1,
    }
}

/// The inverse. Anything out of range reads as `Words`, the default a fresh project gets.
pub fn unit_of(index: usize) -> GoalUnit {
    match index {
        1 => GoalUnit::Characters,
        _ => GoalUnit::Words,
    }
}

/// The picker, over a plain index signal the caller keeps in step with its own state.
///
/// `on_change` rather than an effect on the index, because it is handed an
/// `EventContext` — which is what a caller needs to raise a dialog, and Settings has to ask
/// before it switches. An effect gets none.
///
/// The `SegmentId`s here are freshly minted and are **never persisted**: what is stored is
/// the `GoalUnit` on the `Work`, and what binds is the positional index. That is the whole
/// reason this uses `indexed` — the list is closed, local and two long, so position is the
/// meaning, exactly as it is for the Export panel's Format picker.
pub fn goal_unit_control(
    index: Signal<usize>,
    on_change: impl Fn(GoalUnit, &mut EventContext) + 'static,
) -> SegmentedControl {
    let control = SegmentedControl::indexed(index)
        .sizing(SegmentSizing::Fit)
        .segment(Segment::new(tr!(goal_unit_words())))
        .segment(Segment::new(tr!(goal_unit_characters())));
    let ids = control.segment_ids();
    control.on_change(move |id, ctx| {
        let i = ids.iter().position(|x| *x == id).unwrap_or(0);
        on_change(unit_of(i), ctx);
    })
}

/// The explainer both call sites hang on the control. A registry key, so it can be the
/// target of a `[label](:goal-unit)` link from the target field's own tooltip.
pub fn goal_unit_tooltip() -> TooltipContent {
    TooltipContent::new(GOAL_UNIT, tr!(goal_unit())).with_more(tr!(goal_unit_more()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_index_round_trips_and_a_stray_one_reads_as_words() {
        assert_eq!(unit_of(index_of(&GoalUnit::Words)), GoalUnit::Words);
        assert_eq!(
            unit_of(index_of(&GoalUnit::Characters)),
            GoalUnit::Characters
        );
        assert_eq!(unit_of(7), GoalUnit::Words);
    }
}
