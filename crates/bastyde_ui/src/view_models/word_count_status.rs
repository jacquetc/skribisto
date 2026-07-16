//! The status-bar word count's display decision — pure.
//!
//! A live count of the **focused** editor item's prose, sitting quietly in the status bar
//! (like the save glyph). It shows only when a project is open and something prose-bearing
//! is focused; a container tab, an unopened item, or no project shows nothing and takes no
//! width. The count is cheap (one scene) so it updates live as the writer types — no
//! debounce hysteresis like the save spinner needs.

/// What the status-bar word count shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CountDisplay {
    /// No project, or nothing prose-bearing focused — show nothing (and no width).
    Hidden,
    /// The focused item's live word count.
    Words(usize),
}

/// The display state from the two inputs: whether a project is open, and the focused
/// item's word count (`None` when nothing prose-bearing is focused).
pub fn count_display(has_work: bool, focused: Option<usize>) -> CountDisplay {
    match (has_work, focused) {
        (true, Some(n)) => CountDisplay::Words(n),
        _ => CountDisplay::Hidden,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hidden_without_a_project() {
        assert_eq!(count_display(false, Some(42)), CountDisplay::Hidden);
    }

    #[test]
    fn hidden_when_nothing_prose_bearing_is_focused() {
        // A container tab / an unopened item / no selection → the model yields None.
        assert_eq!(count_display(true, None), CountDisplay::Hidden);
    }

    #[test]
    fn shows_the_count_when_focused_on_prose() {
        assert_eq!(count_display(true, Some(0)), CountDisplay::Words(0));
        assert_eq!(count_display(true, Some(1234)), CountDisplay::Words(1234));
    }
}
