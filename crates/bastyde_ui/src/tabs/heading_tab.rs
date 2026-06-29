//! Title-bearing item tabs: Item/Chapter and Item/Part (a title field + a
//! synopsis) and Item/BookBegin (book title + subtitle, no synopsis). The fields
//! present are decided by `tab_for` from the constraint matrix, so this one
//! render fn covers all three.

use bastyde::prelude::*;
use bastyde::widgets::VStack;

use super::{ContentTab, parts};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    let mut col = VStack::new().spacing(8.0).child(parts::vspace(20.0));

    if let Some(t) = &tab.title {
        col = col.child(parts::centered(
            parts::title_input(t, "Title…"),
            &tab.column_width,
        ));
    }
    if let Some(st) = &tab.subtitle {
        col = col.child(parts::centered(
            parts::title_input(st, "Subtitle…"),
            &tab.column_width,
        ));
    }
    if let Some(s) = &tab.synopsis {
        col = col.child(parts::vspace(8.0)).child(parts::synopsis_section(
            &s.doc,
            &tab.column_width,
            tab.mark_dirty_fn(),
        ));
    }
    parts::tab_backdrop(col)
}
