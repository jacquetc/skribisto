//! Synopsis-only folder tabs: `Folder/None` (a plain grouping folder) and
//! `Folder/Note` (a notes group). They carry only a synopsis.

use bastyde::prelude::*;
use bastyde::widgets::VStack;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    let mut col = VStack::new().spacing(8.0).child(shared::vspace(12.0));
    if let Some(s) = &tab.synopsis {
        col = col.child(shared::synopsis_section(
            &s.doc,
            &tab.column_width,
            tab.mark_dirty_fn(),
        ));
    }
    shared::tab_backdrop(col)
}
