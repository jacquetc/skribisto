//! The dual-pane writing editor (Skribisto's signature): a synopsis editor over
//! the main-text editor. Opened for Item/Scene, Item/ChapterScene (which adds a
//! chapter-title field on top) and Item/Note. Renamed from the former
//! `editor_tab.rs`; shared layout primitives live in [`super::parts`].

use bastyde::prelude::*;
use bastyde::widgets::VStack;

use super::{ContentTab, parts};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    let mut col = VStack::new().spacing(5.0).child(parts::vspace(10.0));

    // ChapterScene opens a chapter — show its title field above the prose.
    if let Some(t) = &tab.title {
        col = col
            .child(parts::centered(
                parts::title_input(t, tr!(placeholder_chapter_title())),
                &tab.column_width,
            ))
            .child(parts::vspace(6.0));
    }
    if let Some(s) = &tab.synopsis {
        col = col.child(parts::synopsis_section(
            &s.doc,
            &tab.column_width,
            tab.mark_dirty_fn(),
        ));
    }
    if let Some(m) = &tab.main {
        col = col.child(parts::vspace(10.0)).child(parts::writing_section(
            &m.doc,
            &tab.column_width,
            tab.mark_dirty_fn(),
        ));
    }
    parts::tab_backdrop(col)
}
