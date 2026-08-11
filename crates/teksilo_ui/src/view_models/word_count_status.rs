// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The status-bar word count's display decision — pure.
//!
//! A live count of the **focused** editor item's prose, sitting quietly in the status bar
//! (like the save glyph). It shows only when a project is open and something prose-bearing
//! is focused; a container tab, an unopened item, or no project shows nothing and takes no
//! width. The count is cheap (one scene) so it updates live as the writer types — no
//! debounce hysteresis like the save spinner needs. When the Goals setting
//! `goals.show_characters` is on, the character count rides alongside the words.

/// What the status-bar word count shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CountDisplay {
    /// No project, or nothing prose-bearing focused — show nothing (and no width).
    Hidden,
    /// The focused item's live word count.
    Words(usize),
    /// The focused item's word **and** character count (`goals.show_characters` on).
    WordsChars { words: usize, chars: usize },
}

/// Whether the focused item's target bar shows, and what it is measuring.
///
/// Separate from [`CountDisplay`] rather than another variant of it, because the two are
/// governed by different things: the count shows whenever prose is focused, the bar only
/// once that piece has been given a target. A writer with no targets anywhere sees the
/// status bar exactly as it was.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GoalDisplay {
    /// No project, nothing prose-bearing focused, or no target on it.
    Hidden,
    /// How much is written, against the target, in the project's unit.
    Progress { written: i64, goal: i64 },
}

/// The target bar's state, from the same three facts plus the focused item's own target.
///
/// `0` is the no-target sentinel, so it hides — the bar exists to answer "how close am I",
/// which is not a question about a piece nobody has set a length for.
pub fn goal_display(has_work: bool, written: Option<usize>, goal: i64) -> GoalDisplay {
    match (has_work, written) {
        (true, Some(written)) if goal > 0 => GoalDisplay::Progress {
            written: written as i64,
            goal,
        },
        _ => GoalDisplay::Hidden,
    }
}

/// The display state from three inputs: whether a project is open, the focused item's
/// `(words, chars)` counts (`None` when nothing prose-bearing is focused), and whether the
/// user asked to see characters too.
pub fn count_display(
    has_work: bool,
    focused: Option<(usize, usize)>,
    show_chars: bool,
) -> CountDisplay {
    match (has_work, focused) {
        (true, Some((words, chars))) => {
            if show_chars {
                CountDisplay::WordsChars { words, chars }
            } else {
                CountDisplay::Words(words)
            }
        }
        _ => CountDisplay::Hidden,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hidden_without_a_project() {
        assert_eq!(
            count_display(false, Some((42, 210)), false),
            CountDisplay::Hidden
        );
        assert_eq!(
            count_display(false, Some((42, 210)), true),
            CountDisplay::Hidden
        );
    }

    #[test]
    fn hidden_when_nothing_prose_bearing_is_focused() {
        // A container tab / an unopened item / no selection → the model yields None.
        assert_eq!(count_display(true, None, false), CountDisplay::Hidden);
        assert_eq!(count_display(true, None, true), CountDisplay::Hidden);
    }

    #[test]
    fn shows_the_word_count_when_focused_on_prose() {
        assert_eq!(
            count_display(true, Some((0, 0)), false),
            CountDisplay::Words(0)
        );
        assert_eq!(
            count_display(true, Some((1234, 6789)), false),
            CountDisplay::Words(1234)
        );
    }

    #[test]
    fn the_target_bar_hides_until_there_is_a_target() {
        assert_eq!(goal_display(true, Some(500), 0), GoalDisplay::Hidden);
        assert_eq!(goal_display(true, Some(500), -1), GoalDisplay::Hidden);
        assert_eq!(
            goal_display(true, Some(500), 2_000),
            GoalDisplay::Progress {
                written: 500,
                goal: 2_000
            }
        );
    }

    /// It hides wherever the count itself hides, so the two never appear apart.
    #[test]
    fn the_target_bar_hides_wherever_the_count_does() {
        assert_eq!(goal_display(false, Some(500), 2_000), GoalDisplay::Hidden);
        assert_eq!(goal_display(true, None, 2_000), GoalDisplay::Hidden);
    }

    /// Writing past the target keeps the bar, with the raw numbers: the caller's colour
    /// band is what says "over", and clamping here would hide it.
    #[test]
    fn writing_past_the_target_still_reports_the_real_numbers() {
        assert_eq!(
            goal_display(true, Some(5_000), 2_000),
            GoalDisplay::Progress {
                written: 5_000,
                goal: 2_000
            }
        );
    }

    #[test]
    fn adds_characters_when_asked() {
        assert_eq!(
            count_display(true, Some((1234, 6789)), true),
            CountDisplay::WordsChars {
                words: 1234,
                chars: 6789
            }
        );
    }
}
