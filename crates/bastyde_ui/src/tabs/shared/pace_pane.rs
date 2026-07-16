//! The Book's **Pace** segment — the writing-schedule planner.
//!
//! Only the Book container shows it (see `folder_book`): the writer sets a word-count goal
//! and an end date, picks which weekdays count, and Skribisto derives a pace + shows the
//! progression (from the ProgressSnapshot history) and per-day words as charts, plus the
//! usual stats (streak, % done, days left, ahead/behind).
//!
//! Built out over M4b–M4d; this is the segment shell.

use bastyde::prelude::*;
use bastyde::widgets::{Center, TextWidget};

use crate::tabs::ContentTab;

pub fn pace_pane(_tab: &ContentTab) -> Box<dyn Widget> {
    Box::new(Center::new().child(TextWidget::new(tr!(pace_placeholder())).color(TextRole::Secondary)))
}
