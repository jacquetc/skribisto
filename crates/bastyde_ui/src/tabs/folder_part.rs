//! Container tab for a `Folder/Part`. Same shape as the other folder tabs: a
//! `SegmentedControl` over a shared **Synopsis** view now, with Corkboard and
//! Overview as 🚧 future segments.

use bastyde::prelude::*;
use bastyde::widgets::{Segment, SegmentedControl, Switcher, VStack};

use super::{ContentTab, parts};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    let bar = SegmentedControl::new(tab.segment.clone())
        .segment(Segment::new(lit!("Synopsis")))
        .segment(Segment::new(lit!("Corkboard")).disabled(true))
        .segment(Segment::new(lit!("Overview")).disabled(true));
    let content = Switcher::new(tab.segment.clone()).child(parts::folder_synopsis_pane(tab));

    let col = VStack::new()
        .spacing(8.0)
        .child(parts::vspace(10.0))
        .child(parts::centered(bar, &tab.column_width))
        .child(content);
    parts::tab_backdrop(col)
}
