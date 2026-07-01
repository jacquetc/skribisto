//! Container tab for a `Folder/Book` (the manuscript root). Its shared
//! **Synopsis** view shows the book title + subtitle above the synopsis; a
//! `SegmentedControl` selects it, with Corkboard and Overview as 🚧 future
//! segments.

use bastyde::prelude::*;
use bastyde::widgets::{Segment, SegmentedControl, Switcher, VStack};

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    let bar = SegmentedControl::new(tab.segment.clone())
        .segment(Segment::new(tr!(synopsis())))
        .segment(Segment::new(tr!(corkboard())).disabled(true))
        .segment(Segment::new(tr!(overview())).disabled(true));
    let content = Switcher::new(tab.segment.clone()).child(shared::folder_synopsis_pane(tab));

    let col = VStack::new()
        .spacing(8.0)
        .child(shared::vspace(10.0))
        .child(shared::centered(bar, &tab.column_width))
        .child(content);
    shared::tab_backdrop(col)
}
