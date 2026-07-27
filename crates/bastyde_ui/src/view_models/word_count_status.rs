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
