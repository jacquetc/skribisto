//! Title-bearing item tabs: Item/Chapter and Item/Part (a title field + a
//! synopsis) and Item/BookBegin (book title + subtitle, no synopsis). The fields
//! present are decided by `tab_for` from the constraint matrix, so this one
//! render fn covers all three.

use bastyde::prelude::*;
use bastyde::widgets::VStack;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    let mut col = VStack::new().spacing(8.0).child(shared::vspace(20.0));

    if let Some(t) = &tab.title {
        col = col.child(shared::centered(
            shared::title_input(t, tr!(placeholder_title())),
            &tab.column_width,
        ));
    }
    if let Some(st) = &tab.subtitle {
        col = col.child(shared::centered(
            shared::title_input(st, tr!(placeholder_subtitle())),
            &tab.column_width,
        ));
    }
    if let Some(s) = &tab.synopsis {
        col = col.child(shared::vspace(8.0)).child(shared::synopsis_section(
            &s.doc,
            &tab.column_width,
            tab.mark_dirty_fn(),
        ));
    }
    shared::tab_backdrop(col)
}
