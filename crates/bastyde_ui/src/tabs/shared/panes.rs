//! Composite pane renders shared across several `(role, sub_role)` tabs.
//!
//! Each function here is a whole tab body that more than one combination reuses:
//! the [`heading`] form (Item Chapter / Part / BookBegin), the dual-pane
//! [`prose`] editor (Item Scene / ChapterScene / Note), the [`placeholder`] for
//! contentless rows (Item BookEnd / Text), and the folder-container bodies
//! ([`folder_synopsis_only`] for plain grouping folders, [`folder_segmented`] for
//! the three structural containers). The fields each body shows are decided by the
//! constraint matrix (via `tab_for`), so one body covers every combination in its
//! group. The manuscript-stream pane the containers share lives in
//! [`stream`](super::stream).

use bastyde::i18n::LocalizedString;
use bastyde::prelude::*;
use bastyde::widgets::{
    Center, Expand, GroupHeader, ScrollArea, Segment, SegmentedControl, Switcher, TextWidget,
    VStack,
};

use crate::tabs::ContentTab;
use crate::view_models::SplitFlavour;

use super::{
    VisibleWhen, centered, stream_pane, synopsis_column, synopsis_section, tab_backdrop,
    title_input, vspace, writing_section,
};

/// The container's **own page** — the first segment of every folder container tab.
///
/// It is the container as a *writing surface*, not a summary of one: its title (and
/// subtitle, for a Book), its synopsis, and — for a chapter folder — **its own prose**.
/// A chapter folder carries a `SceneText` exactly like the flat chapter it promotes
/// to, so a synopsis-only page here would hide the writer's actual text; that is why
/// this pane is named after the container ("Chapter" / "Part" / "Book") rather than
/// "Synopsis", and why it reads like a Scene tab.
///
/// Which fields appear is decided by the constraint matrix (via `OpenDoc::build`), so
/// one body covers all three containers: a Part and a Book simply have no prose to show.
/// The synopsis here is a *primary* surface, so it grows with its content (unlike the
/// compact box that sits above a scene's prose in the dual-pane editor).
pub fn folder_own_pane(tab: &ContentTab) -> impl Widget {
    let mut col = VStack::new().spacing(8.0).child(vspace(12.0));
    if let Some(t) = tab.title() {
        col = col.child(centered(
            title_input(
                t,
                tr!(placeholder_title()),
                tab.mark_dirty_fn(),
                tab.commit_names_fn(),
            ),
            &tab.column_width,
        ));
    }
    if let Some(st) = tab.subtitle() {
        col = col.child(centered(
            title_input(
                st,
                tr!(placeholder_subtitle()),
                tab.mark_dirty_fn(),
                tab.commit_names_fn(),
            ),
            &tab.column_width,
        ));
    }
    if let Some(s) = tab.synopsis() {
        col = col
            .child(vspace(4.0))
            .child(
                GroupHeader::new(tr!(synopsis()))
                    .style(TextStyleRole::SmallBold)
                    .color(TextRole::Secondary),
            )
            .child(synopsis_column(
                &s.doc,
                &tab.column_width,
                &tab.typography.synopsis,
                tab.mark_dirty_fn(),
                Option::None,
            ));
    }
    // A chapter folder's own prose. Absent for a Part or a Book — the matrix gives
    // them no `SceneText`.
    if let Some(m) = tab.main() {
        col = col.child(vspace(10.0)).child(writing_section(
            &m.doc,
            &tab.column_width,
            tab.main_typography(),
            tab.mark_dirty_fn(),
            None, // the container's own page has no find banner (no top strip here)
        ));
    }
    // Flowing page: the editors are intrinsic-height, so this `ScrollArea` scrolls the
    // whole thing rather than each editor scrolling inside its own box.
    ScrollArea::new().child(col.child(vspace(28.0)))
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
                title_input(
                    t,
                    tr!(placeholder_chapter_title()),
                    tab.mark_dirty_fn(),
                    tab.commit_names_fn(),
                ),
                &tab.column_width,
            ))
            .child(vspace(6.0));
    }
    if let Some(s) = tab.synopsis() {
        // The synopsis pane is user-toggleable (Settings ▸ Manuscript & Fonts).
        // Hidden, it goes dormant: no space, no paint, out of the a11y tree and the
        // Tab order — while the writing editor stays mounted and the synopsis's own
        // document (owned by the shared `OpenDoc`) survives to be re-shown.
        //
        // **Not a `Switcher`.** A `Switcher` reports its child's *natural* width and
        // ignores the bounded width it is proposed, so this one claimed the synopsis's
        // full column width (~656px) even in a 300px window — making the tab overhang
        // to the right for the entire height of the scene. That overhang is what wedged
        // the renderer: the inspector striped the overflow, and a single hazard band
        // across a scene-tall strip became a 229 MB path the atlas re-rasterized every
        // frame. See `shared::editor::VisibleWhen`.
        col = col.child(VisibleWhen::new(
            tab.show_synopsis.clone(),
            synopsis_section(
                &s.doc,
                &tab.column_width,
                &tab.typography.synopsis,
                tab.mark_dirty_fn(),
            ),
        ));
    }
    // The per-editor find banner's view-model (Ctrl+F). `Some` for every prose
    // tab — Scene / ChapterScene / Note all have a main field. The editor built by
    // `writing_section` attaches its handle to this vm; the banner above binds it.
    let find = tab.find().cloned();
    if let Some(m) = tab.main() {
        col = col.child(vspace(10.0)).child(writing_section(
            &m.doc,
            &tab.column_width,
            tab.main_typography(),
            tab.mark_dirty_fn(),
            find.clone(),
        ));
    }
    // The whole dual-pane body scrolls as one flowing page: the main editor is
    // intrinsic-sized with its own scroll bar suppressed (see
    // `shared::writing_column`), so title, synopsis and prose scroll together here
    // instead of the prose scrolling inside a fixed pane.
    //
    // `prose` is the only one of the five `tab_backdrop` composites that gets the
    // find banner: `heading` / `placeholder` / `folder_synopsis_only` have no main
    // writing surface to search, and `folder_segmented` wraps a stream `Switcher`
    // whose rows have no single "focused editor" to target.
    match find {
        Some(find) => crate::tabs::shared::editor::tab_backdrop_with_find(
            find,
            ScrollArea::new().child(col),
        ),
        None => crate::tabs::shared::editor::tab_backdrop(ScrollArea::new().child(col)),
    }
}

/// A title (+ optional subtitle / synopsis) form. Shared by the title-bearing item
/// tabs: Item/Part (title + synopsis) and Item/BookBegin (book title + subtitle +
/// synopsis). The fields present are decided by `tab_for` from the constraint matrix,
/// so one body covers both.
pub fn heading(tab: &ContentTab) -> Box<dyn Widget> {
    let mut col = VStack::new().spacing(8.0).child(vspace(20.0));

    if let Some(t) = tab.title() {
        col = col.child(centered(
            title_input(
                t,
                tr!(placeholder_title()),
                tab.mark_dirty_fn(),
                tab.commit_names_fn(),
            ),
            &tab.column_width,
        ));
    }
    if let Some(st) = tab.subtitle() {
        col = col.child(centered(
            title_input(
                st,
                tr!(placeholder_subtitle()),
                tab.mark_dirty_fn(),
                tab.commit_names_fn(),
            ),
            &tab.column_width,
        ));
    }
    if let Some(s) = tab.synopsis() {
        // On a heading tab the synopsis *is* the page — it grows, like any primary
        // writing surface (contrast the compact box above a scene's prose).
        col = col
            .child(vspace(8.0))
            .child(
                GroupHeader::new(tr!(synopsis()))
                    .style(TextStyleRole::SmallBold)
                    .color(TextRole::Secondary),
            )
            .child(synopsis_column(
                &s.doc,
                &tab.column_width,
                &tab.typography.synopsis,
                tab.mark_dirty_fn(),
                Option::None,
            ));
    }
    tab_backdrop(ScrollArea::new().child(col.child(vspace(28.0))))
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

/// A synopsis-only folder body (Folder/None, Folder/Note): a plain grouping / notes
/// folder, which the matrix gives *only* a synopsis — so there is nothing to segment,
/// and no stream (it has no manuscript extent).
///
/// Its synopsis is the page, not a footnote to one, so it grows with its content like
/// any other primary writing surface.
pub fn folder_synopsis_only(tab: &ContentTab) -> Box<dyn Widget> {
    let mut col = VStack::new().spacing(8.0).child(vspace(12.0));
    if let Some(s) = tab.synopsis() {
        col = col
            .child(
                GroupHeader::new(tr!(synopsis()))
                    .style(TextStyleRole::SmallBold)
                    .color(TextRole::Secondary),
            )
            .child(synopsis_column(
                &s.doc,
                &tab.column_width,
                &tab.typography.synopsis,
                tab.mark_dirty_fn(),
                Option::None,
            ));
    }
    tab_backdrop(ScrollArea::new().child(col.child(vspace(28.0))))
}

/// The body every folder container shares: a `SegmentedControl` over
///
/// 1. the container's **own page** — named after the container itself ("Chapter" /
///    "Part" / "Book"), because it *is* that item as a writing surface: title,
///    synopsis, and — for a chapter — its own prose. (It used to be called "Synopsis";
///    that became a lie the moment a chapter folder started carrying prose.)
/// 2. the **manuscript stream** — Full Chapter / Full Part / Full Book: the container
///    *and everything inside it*, as one continuous manuscript;
/// 3. **Full Synopsis** — the same rows, showing each one's synopsis instead;
///
/// plus Corkboard and Overview as 🚧 future segments (FEATURES.md), shown disabled.
///
/// The pairing reads as "this one" vs "this one and all of it": `Chapter` /
/// `Full Chapter`.
///
/// The `Switcher` mounts only the child at the selected index; the two disabled
/// segments have no child, and an out-of-range selection mounts nothing (no panic).
pub fn folder_segmented(
    tab: &ContentTab,
    own_label: impl Into<LocalizedString>,
    manuscript_label: impl Into<LocalizedString>,
    extra: Option<(LocalizedString, Box<dyn Widget>)>,
) -> Box<dyn Widget> {
    // An optional container-specific segment (the Book's "Pace") is inserted **before**
    // the two disabled placeholders, so every existing index stays put and the
    // SegmentedControl↔Switcher positional contract holds (disabled segments never become
    // the current index).
    let mut bar = SegmentedControl::new(tab.segment.clone())
        .segment(Segment::new(own_label))
        .segment(Segment::new(manuscript_label))
        .segment(Segment::new(tr!(full_synopsis())));
    let mut content = Switcher::new(tab.segment.clone())
        .child(folder_own_pane(tab))
        .child(stream_pane(tab, SplitFlavour::Prose))
        .child(stream_pane(tab, SplitFlavour::Synopsis));
    if let Some((label, pane)) = extra {
        bar = bar.segment(Segment::new(label));
        content = content.child_boxed(pane);
    }
    let bar = bar
        .segment(Segment::new(tr!(corkboard())).disabled(true))
        .segment(Segment::new(tr!(overview())).disabled(true));

    let col = VStack::new()
        .spacing(8.0)
        .child(vspace(10.0))
        .child(centered(bar, &tab.column_width))
        // Fill the remaining height so the selected segment (especially a stream's
        // `ScrollArea`) gets a bounded viewport to fill.
        .child(Expand::new().child(content));
    tab_backdrop(col)
}
