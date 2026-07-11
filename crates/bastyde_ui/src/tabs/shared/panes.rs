//! Composite pane renders shared across several `(role, sub_role)` tabs.
//!
//! Each function here is a whole tab body that more than one combination reuses:
//! the [`heading`] form (Item Chapter / Part / BookBegin), the dual-pane
//! [`prose`] editor (Item Scene / ChapterScene / Note), the [`placeholder`] for
//! contentless rows (Item BookEnd / Text), and the folder-container bodies
//! ([`folder_synopsis_only`] for plain grouping folders, [`folder_segmented`] for
//! the Book/Part containers). The fields each body shows are decided by the
//! constraint matrix (via `tab_for`), so one body covers every combination in its
//! group. Combination-specific bodies (e.g. the Full Chapter view) stay in their
//! own module.

use bastyde::prelude::*;
use bastyde::widgets::{
    Center, ScrollArea, Segment, SegmentedControl, Switcher, TextWidget, VStack,
};

use crate::tabs::ContentTab;

use super::{centered, synopsis_section, tab_backdrop, title_input, vspace, writing_section};

/// The shared **Synopsis** view for the folder container tabs: the folder's title
/// (and subtitle, for a Book) above its synopsis editor. The Book/Part/Chapter
/// folders each own a `SegmentedControl`; this is the synopsis segment they share.
pub fn folder_synopsis_pane(tab: &ContentTab) -> impl Widget {
    let mut col = VStack::new().spacing(8.0).child(vspace(12.0));
    if let Some(t) = tab.title() {
        col = col.child(centered(
            title_input(t, tr!(placeholder_title())),
            &tab.column_width,
        ));
    }
    if let Some(st) = tab.subtitle() {
        col = col.child(centered(
            title_input(st, tr!(placeholder_subtitle())),
            &tab.column_width,
        ));
    }
    if let Some(s) = tab.synopsis() {
        col = col.child(vspace(4.0)).child(synopsis_section(
            &s.doc,
            &tab.column_width,
            &tab.typography.synopsis,
            tab.mark_dirty_fn(),
        ));
    }
    col
}

/// The dual-pane writing editor (Skribisto's signature): an optional title, a
/// user-toggleable synopsis editor, and the main-text editor, scrolling together
/// as one flowing page. Shared by Item/Scene, Item/ChapterScene (which adds the
/// chapter-title field) and Item/Note (which switches to Notes typography).
pub fn prose(tab: &ContentTab) -> Box<dyn Widget> {
    let mut col = VStack::new().spacing(5.0).child(vspace(10.0));

    // ChapterScene opens a chapter — show its title field above the prose.
    if let Some(t) = tab.title() {
        col = col
            .child(centered(
                title_input(t, tr!(placeholder_chapter_title())),
                &tab.column_width,
            ))
            .child(vspace(6.0));
    }
    if let Some(s) = tab.synopsis() {
        // The synopsis pane is user-toggleable (Settings ▸ Manuscript & Fonts).
        // A `Switcher` keeps the writing editor mounted while the (hidden)
        // synopsis is dropped — only the active page is mounted, so the loaded
        // synopsis document is preserved and re-shown on toggle-back.
        let visible = tab.show_synopsis.map(|on| if *on { 1 } else { 0 });
        col = col.child(
            Switcher::new(visible)
                .child(vspace(0.0))
                .child(synopsis_section(
                    &s.doc,
                    &tab.column_width,
                    &tab.typography.synopsis,
                    tab.mark_dirty_fn(),
                )),
        );
    }
    if let Some(m) = tab.main() {
        col = col.child(vspace(10.0)).child(writing_section(
            &m.doc,
            &tab.column_width,
            tab.main_typography(),
            tab.mark_dirty_fn(),
        ));
    }
    // The whole dual-pane body scrolls as one flowing page: the main editor is
    // intrinsic-sized with its own scroll bar suppressed (see
    // `shared::writing_column`), so title, synopsis and prose scroll together here
    // instead of the prose scrolling inside a fixed pane.
    tab_backdrop(ScrollArea::new().child(col))
}

/// A title (+ optional subtitle / synopsis) form. Shared by the title-bearing
/// item tabs: Item/Chapter and Item/Part (title + synopsis) and Item/BookBegin
/// (book title + subtitle, no synopsis). The fields present are decided by
/// `tab_for` from the constraint matrix, so this one body covers all three.
pub fn heading(tab: &ContentTab) -> Box<dyn Widget> {
    let mut col = VStack::new().spacing(8.0).child(vspace(20.0));

    if let Some(t) = tab.title() {
        col = col.child(centered(
            title_input(t, tr!(placeholder_title())),
            &tab.column_width,
        ));
    }
    if let Some(st) = tab.subtitle() {
        col = col.child(centered(
            title_input(st, tr!(placeholder_subtitle())),
            &tab.column_width,
        ));
    }
    if let Some(s) = tab.synopsis() {
        col = col.child(vspace(8.0)).child(synopsis_section(
            &s.doc,
            &tab.column_width,
            &tab.typography.synopsis,
            tab.mark_dirty_fn(),
        ));
    }
    tab_backdrop(col)
}

/// A quiet placeholder for contentless rows (Item/BookEnd, Item/Text): they carry
/// no editable content, so opening one shows an explanatory label, not an empty
/// editor.
pub fn placeholder(_tab: &ContentTab) -> Box<dyn Widget> {
    tab_backdrop(bati!(
        Center {
            child: TextWidget::new(tr!(no_content())) {
                color: TextRole::Secondary
            }
        }
    ))
}

/// A synopsis-only folder body (Folder/None, Folder/Note): a plain grouping /
/// notes folder carrying only a synopsis, with no segmented control.
pub fn folder_synopsis_only(tab: &ContentTab) -> Box<dyn Widget> {
    let mut col = VStack::new().spacing(8.0).child(vspace(12.0));
    if let Some(s) = tab.synopsis() {
        col = col.child(synopsis_section(
            &s.doc,
            &tab.column_width,
            &tab.typography.synopsis,
            tab.mark_dirty_fn(),
        ));
    }
    tab_backdrop(col)
}

/// A folder container body with a `SegmentedControl` over the shared **Synopsis**
/// view (Folder/Book, Folder/Part). Corkboard and Overview are 🚧 future segments
/// (FEATURES.md), shown disabled.
pub fn folder_segmented(tab: &ContentTab) -> Box<dyn Widget> {
    let bar = SegmentedControl::new(tab.segment.clone())
        .segment(Segment::new(tr!(synopsis())))
        .segment(Segment::new(tr!(corkboard())).disabled(true))
        .segment(Segment::new(tr!(overview())).disabled(true));
    let content = Switcher::new(tab.segment.clone()).child(folder_synopsis_pane(tab));

    let col = VStack::new()
        .spacing(8.0)
        .child(vspace(10.0))
        .child(centered(bar, &tab.column_width))
        .child(content);
    tab_backdrop(col)
}
