//! The dual-pane writing editor (Skribisto's signature): a synopsis editor over
//! the main-text editor. Opened for Item/Scene, Item/ChapterScene (which adds a
//! chapter-title field on top) and Item/Note. Renamed from the former
//! `editor_tab.rs`; shared layout primitives live in [`super::shared`].

use bastyde::prelude::*;
use bastyde::widgets::{ScrollArea, Switcher, VStack};

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    let mut col = VStack::new().spacing(5.0).child(shared::vspace(10.0));

    // ChapterScene opens a chapter — show its title field above the prose.
    if let Some(t) = tab.title() {
        col = col
            .child(shared::centered(
                shared::title_input(t, tr!(placeholder_chapter_title())),
                &tab.column_width,
            ))
            .child(shared::vspace(6.0));
    }
    if let Some(s) = tab.synopsis() {
        // The synopsis pane is user-toggleable (Settings ▸ Manuscript & Fonts).
        // A `Switcher` keeps the writing editor mounted while the (hidden)
        // synopsis is dropped — only the active page is mounted, so the loaded
        // synopsis document is preserved and re-shown on toggle-back.
        let visible = tab.show_synopsis.map(|on| if *on { 1 } else { 0 });
        col = col.child(Switcher::new(visible).child(shared::vspace(0.0)).child(
            shared::synopsis_section(
                &s.doc,
                &tab.column_width,
                &tab.typography.synopsis,
                tab.mark_dirty_fn(),
            ),
        ));
    }
    if let Some(m) = tab.main() {
        col = col
            .child(shared::vspace(10.0))
            .child(shared::writing_section(
                &m.doc,
                &tab.column_width,
                tab.main_typography(),
                tab.mark_dirty_fn(),
            ));
    }
    // The whole dual-pane body scrolls as one flowing page: the main editor is
    // intrinsic-sized with its own scroll bar suppressed (see
    // `shared::writing_column`), so title, synopsis and prose scroll together
    // here instead of the prose scrolling inside a fixed pane.
    shared::tab_backdrop(ScrollArea::new().child(col))
}
