// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Composite pane renders shared across several `(role, sub_role)` tabs.
//!
//! Each function here is a whole tab body that more than one combination reuses:
//! the [`heading`] form (Item Chapter / Part / BookBegin), the dual-pane
//! [`prose`] editor (Item Scene / ChapterScene / Note), the [`placeholder`] for
//! contentless rows (Item BookEnd / Text), and the folder-container bodies
//! ([`folder_synopsis_only`] for a plain grouping folder, [`folder_synopsis_with_overview`]
//! for a notes folder, [`folder_segmented`] for the three structural containers). The
//! fields each body shows are decided by the constraint matrix (via `tab_for`), so one
//! body covers every combination in its group. The manuscript-stream pane the
//! containers share lives in [`stream`](super::stream).

use bastyde::core::widget::WidgetPlacement;
use bastyde::i18n::LocalizedString;
use bastyde::prelude::*;
use bastyde::widgets::{
    Accordion, Center, Expand, GroupHeader, HStack, IconButton, IconButtonSize, Padding,
    RectWidget, ScrollArea, Segment, SegmentedControl, Spacer, Splitter, Switcher, TextWidget,
    VStack, ZStack,
};

use frontend::common::entities::BinderItemSubRole;

use crate::tabs::{Boxed, ContentTab};
use crate::view_models::{EditorViewMemory, SplitFlavour};

use super::editor::SideSync;
use super::{
    VisibleWhen, centered, side_synopsis_editor, stream_pane, synopsis_column, synopsis_section,
    tab_backdrop, title_input, vspace, writing_section,
};

/// The epigraph disclosure — the quotation set at the head of this book, part or chapter.
///
/// `None` for the combinations the matrix gives no `EpigraphText`, which is what keeps a
/// scene or a note from sprouting one even though `prose()` is shared with the chapter
/// that can have it. Folded away when empty and open when authored (see
/// [`ContentTab::epigraph_expanded`]), so a project that never uses epigraphs never pays
/// for the affordance and one that does never has to go looking for it.
///
/// An `Accordion` rather than a fourth segment on purpose: the container bars pair a
/// `SegmentedControl` with a `Switcher` **by index**, so a segment added out of order
/// silently shows the previous view under the new label. This lives inside segment 0 and
/// touches none of that.
fn epigraph_section(tab: &ContentTab) -> Option<impl Widget> {
    let field = tab.epigraph()?;
    Some(
        Accordion::new(tr!(epigraph()), tab.epigraph_expanded.clone()).content(synopsis_column(
            &field.doc,
            &tab.column_width,
            // Scene typography, not the synopsis's: an epigraph is finished-book matter
            // that ships in the manuscript, not editorial commentary about it.
            tab.main_typography(),
            tab.mark_dirty_fn(),
            Option::None,
            tab.open_doc.spell_epigraph(),
            tab.open_doc.replacement_epigraph(),
            // No handle sink and no comment binding: the format dock acts on the
            // manuscript the caret is in, and a comment anchors to the author's own
            // prose — see `OpenDoc::build` for why the epigraph gets no comment layer.
            Option::None,
            Some(tab.format.clone()),
            Some(tab.typewriter.clone()),
            Some(tab.caret_band()),
            Option::None,
            tab.open_doc.images(),
            // A trashed item's text is read-only. The banner above it is a
            // statement, not a guard: before this the content beneath it was built
            // by the same editable render path as any other tab.
            tab.open_doc.trashed.get(),
        )),
    )
}

/// The `ScrollArea` every writing surface in the app scrolls inside — the one
/// door, so the scroll range and the editors' pin can never be configured apart.
///
/// The editors on these pages are intrinsic-height with their own scroll bars
/// suppressed ("flowing page" mode), so this is what actually scrolls, and it is
/// therefore what has to buy the range past the last line that lets the final
/// paragraph reach the typewriter pin. Without it the pin would quietly stop
/// working over the last page — exactly where a writer spends their time. The
/// range collapses to zero when typewriter scrolling is off, so a page without
/// the feature cannot be scrolled past its own end.
///
/// It is also where this tab's **page scroll** is published to its view-state
/// ports. The scroll a writer wants restored is this area's, not any editor's:
/// the editors here run with `ScrollPolicy::AlwaysOff` and grow to their
/// content, so `RichTextEditor::scroll_y()` on a prose column is permanently 0
/// and persisting it would persist nothing.
pub(crate) fn writing_page_scroll(tab: &ContentTab) -> ScrollArea {
    let area = ScrollArea::new().scroll_past_end(tab.typewriter.scroll_past_end_signal());
    tab.view_state_ports().attach_page_scroll(
        area.scroll_y_signal().clone(),
        area.max_scroll_y_signal().clone(),
    );
    area
}

/// As [`writing_page_scroll`], but for a page that is **one of several** a tab can
/// show — it hands back a zero-size companion that claims the tab's view-state
/// ports only while this page is the one on screen.
///
/// The plain function above attaches immediately, which is right for a body with a
/// single scrolling page. The dual-pane editor has two (the Top layout's flowing
/// page, and the Side layout's manuscript column), and the ports hold one slot: an
/// immediate attach from both would leave the tab restoring, and reporting, the
/// scroll of whichever happened to be built last. Mount the companion anywhere
/// inside the same page.
pub(crate) fn switchable_page_scroll(tab: &ContentTab) -> (ScrollArea, impl Widget) {
    let area = ScrollArea::new().scroll_past_end(tab.typewriter.scroll_past_end_signal());
    let port = super::editor::PageScrollPort::new(
        tab.view_state_ports(),
        area.scroll_y_signal().clone(),
        area.max_scroll_y_signal().clone(),
    );
    (area, port)
}

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
    col = col.child(centered(subtitle_tag_dots(tab), &tab.column_width));
    // After the title, before the body — where CMOS §13.36 puts a chapter epigraph, and
    // where the compiler emits it, so the page reads in the order the export writes.
    if let Some(epi) = epigraph_section(tab) {
        col = col
            .child(vspace(4.0))
            .child(centered(epi, &tab.column_width));
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
                tab.open_doc.spell_synopsis(),
                tab.open_doc.replacement_synopsis(),
                Some(tab.synopsis_handle_sink()),
                Some(tab.format.clone()),
                Some(tab.typewriter.clone()),
                Some(tab.caret_band()),
                tab.open_doc.comment_binding_synopsis(),
                tab.open_doc.images(),
                // A trashed item's text is read-only. The banner above it is a
                // statement, not a guard: before this the content beneath it was built
                // by the same editable render path as any other tab.
                tab.open_doc.trashed.get(),
            ));
    }
    // A chapter folder's own prose. Absent for a Part or a Book — the matrix gives
    // them no `SceneText`.
    if let Some(m) = tab.main() {
        col = col.child(vspace(10.0)).child(writing_section(
            &m.doc,
            tab.main_column_width(),
            tab.main_typography(),
            tab.mark_dirty_fn(),
            None, // the container's own page has no find banner (no top strip here)
            tab.open_doc.spell_main(),
            tab.open_doc.replacement_main(),
            Some(tab.format.clone()),
            Some(tab.typewriter.clone()),
            Some(tab.caret_band()),
            Some(tab.view_state_binding()),
            tab.open_doc.comment_binding_main(),
            tab.open_doc.footnote_binding_main(),
            tab.open_doc.images(),
            // A trashed item's text is read-only. The banner above it is a
            // statement, not a guard: before this the content beneath it was built
            // by the same editable render path as any other tab.
            tab.open_doc.trashed.get(),
        ));
    }
    // Flowing page: the editors are intrinsic-height, so this `ScrollArea` scrolls the
    // whole thing rather than each editor scrolling inside its own box.
    writing_page_scroll(tab).child(col.child(vspace(28.0)))
}

/// The dual-pane writing editor (Skribisto's signature): an optional title, a
/// user-toggleable synopsis editor, and the main-text editor, scrolling together
/// as one flowing page. Shared by Item/Scene, Item/ChapterScene (which adds the
/// chapter-title field) and Item/Note (which switches to Notes typography).
pub fn prose(tab: &ContentTab) -> Box<dyn Widget> {
    use crate::tabs::shared::editor::{SideSync, SynopsisPaneEffects, WidthProbe};

    let wants_side = tab.synopsis_placement.map(|p| p.is_side());

    // The Top layout — today's flowing page, unchanged: title, tags, the compact
    // synopsis box and the prose all scroll together.
    let top = manuscript_page(tab, Some(tab.show_synopsis.clone()));

    // The Side layout, built lazily by the `WidthProbe`'s `Switcher` and only if the
    // writer ever actually gets it — a Top-placement project never pays for it.
    let side = {
        // One handle, shared by the effects widget (which drives the pane from the
        // setting) and the header's fold button — so "folded" has a single owner.
        let sync = SideSync::new(tab.side_splitter.clone(), tab.synopsis_side_width.clone());
        let synopsis = side_synopsis_pane(tab, sync.clone());
        let manuscript = manuscript_page(tab, None);
        let mut splitter = Splitter::new(tab.side_splitter.clone())
            .pane(synopsis)
            .pane(manuscript);
        splitter = splitter
            .pane_label(0, tr!(synopsis()))
            .pane_label(1, tr!(pane_manuscript()));
        // Drives pane 0's visibility (and its weight — see `SideSync`) from the same
        // boolean that decides whether the synopsis is on screen, and owns the spell
        // session's dormancy while this branch is the live one.
        VStack::new()
            .spacing(0.0)
            .child(SynopsisPaneEffects::new(
                tab.open_doc.clone(),
                tab.show_synopsis.clone(),
                Some(sync),
            ))
            .child(Expand::new().child(splitter))
    };

    // Placement is a preference; the width is the veto. `WidthProbe` measures what
    // it was actually given and falls back to Top when a Side layout would leave a
    // prose column too narrow to write in — see its docs.
    let body = WidthProbe::new(
        wants_side,
        tab.synopsis_side_width.clone(),
        Box::new(top),
        Box::new(side),
    );

    // `prose` is the only one of this file's `tab_backdrop` composites that gets the
    // find banner: `heading` / `placeholder` / `folder_synopsis_only` have no main
    // writing surface to search, and `folder_segmented` / `folder_synopsis_with_overview`
    // wrap a `Switcher` whose pages have no single "focused editor" to target.
    //
    // The banner is applied **inside** each layout's manuscript column rather than
    // over the whole tab, so under Side it spans the prose it searches instead of
    // stretching across the synopsis strip as well.
    crate::tabs::shared::editor::tab_backdrop(tab.backdrop_role(), body)
}

/// The manuscript as a flowing page: the optional chapter title, the tag row, an
/// optional compact synopsis box, and the prose — all scrolling together.
///
/// `compact_synopsis` is the Top layout's gate. `None` means this page is the
/// manuscript **column of the Side layout**, where the synopsis lives in its own
/// splitter pane and must not also appear here.
fn manuscript_page(tab: &ContentTab, compact_synopsis: Option<Signal<bool>>) -> impl Widget {
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
    // A plain Scene has no title field here — its name lives in the tab — so this is the
    // only place its tags can appear while it is being written. Untagged scenes, which are
    // most of them, get nothing: the row collapses to zero.
    col = col.child(centered(subtitle_tag_dots(tab), &tab.column_width));

    // Self-gating: this body is shared with Item/Scene and Item/Note, and the matrix gives
    // neither of those an `EpigraphText`, so `epigraph()` is `None` there and the section
    // never appears. Only the flat chapter (Item/ChapterScene) shows it.
    if let Some(epi) = epigraph_section(tab) {
        col = col
            .child(centered(epi, &tab.column_width))
            .child(vspace(4.0));
    }

    if let (Some(s), Some(showing)) = (tab.synopsis(), compact_synopsis) {
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
        col = col.child(crate::tabs::shared::editor::SynopsisPaneEffects::new(
            tab.open_doc.clone(),
            tab.show_synopsis.clone(),
            None,
        ));
        col = col.child(VisibleWhen::new(
            showing,
            synopsis_section(
                &s.doc,
                &tab.column_width,
                &tab.typography.synopsis,
                tab.mark_dirty_fn(),
                tab.open_doc.spell_synopsis(),
                tab.open_doc.replacement_synopsis(),
                Some(tab.synopsis_handle_sink()),
                Some(tab.format.clone()),
                Some(tab.caret_band()),
                tab.open_doc.comment_binding_synopsis(),
                tab.open_doc.images(),
                // A trashed item's synopsis is read-only for the same reason its prose
                // is — see `writing_column`.
                tab.open_doc.trashed.get(),
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
            tab.main_column_width(),
            tab.main_typography(),
            tab.mark_dirty_fn(),
            find.clone(),
            tab.open_doc.spell_main(),
            tab.open_doc.replacement_main(),
            Some(tab.format.clone()),
            Some(tab.typewriter.clone()),
            Some(tab.caret_band()),
            Some(tab.view_state_binding()),
            tab.open_doc.comment_binding_main(),
            tab.open_doc.footnote_binding_main(),
            tab.open_doc.images(),
            // A trashed item's text is read-only. The banner above it is a
            // statement, not a guard: before this the content beneath it was built
            // by the same editable render path as any other tab.
            tab.open_doc.trashed.get(),
        ));
    }

    // This page owns the tab's scroll only while it is the page on screen — the two
    // layouts each have one, and the tab remembers a single scroll position.
    let (area, port) = switchable_page_scroll(tab);
    let page = area.child(col.child(port));
    let body: Box<dyn Widget> = match find {
        Some(find) => Box::new(crate::tabs::shared::editor::find_banner_over(find, page)),
        None => Box::new(page),
    };
    Boxed::new(body)
}

/// The Side layout's left pane: the synopsis on the window's own chrome colour,
/// filling the pane and scrolling inside it.
///
/// Painted with a `RectWidget`, not a `Panel` — the distraction-free surface
/// mounts this very pane under a theme-token override where `Panel` resolved its
/// background against the base palette instead of the theme, and every paint there
/// has had to be a `RectWidget` since.
fn side_synopsis_pane(tab: &ContentTab, sync: SideSync) -> impl Widget {
    let body: Box<dyn Widget> = match tab.synopsis() {
        Some(s) => Box::new(
            VStack::new()
                .spacing(6.0)
                .child(vspace(10.0))
                .child(
                    HStack::new()
                        .spacing(4.0)
                        .child(
                            Expand::horizontal().child(
                                GroupHeader::new(tr!(synopsis()))
                                    .style(TextStyleRole::SmallBold)
                                    .color(TextRole::Secondary),
                            ),
                        )
                        // Fold this column away for *this* document. Side costs a
                        // permanent slice of the tab's width — far more than Top's
                        // few lines of height — so reclaiming it must not mean a
                        // Settings trip that changes every tab and every project.
                        .child(
                            IconButton::new(crate::icons::editor::synopsis_collapse())
                                .size(IconButtonSize::Compact)
                                .icon_role(TextRole::Secondary)
                                .tooltip(tr!(synopsis_collapse_tooltip()))
                                .on_activate_fn(move |_| sync.fold()),
                        ),
                )
                .child(Expand::new().child(side_synopsis_editor(
                    &s.doc,
                    &tab.typography.synopsis,
                    tab.mark_dirty_fn(),
                    tab.open_doc.spell_synopsis(),
                    tab.open_doc.replacement_synopsis(),
                    Some(tab.synopsis_handle_sink()),
                    Some(tab.format.clone()),
                    Some(tab.caret_band()),
                    tab.open_doc.comment_binding_synopsis(),
                    tab.open_doc.images(),
                    // A trashed item's synopsis is read-only for the same reason its prose
                    // is — see `writing_column`.
                    tab.open_doc.trashed.get(),
                ))),
        ),
        None => Box::new(vspace(0.0)),
    };
    ZStack::new()
        .child(RectWidget::new().background(SurfaceRole::Main))
        .child(Padding::new(0.0, 12.0, 0.0, 12.0).child(Boxed::new(body)))
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
    col = col.child(centered(subtitle_tag_dots(tab), &tab.column_width));
    // The flat encodings of the same two containers `folder_own_pane` covers, so the
    // epigraph sits in the same place on both — a Part written flat and a Part written as
    // a folder are the same Part.
    if let Some(epi) = epigraph_section(tab) {
        col = col
            .child(vspace(8.0))
            .child(centered(epi, &tab.column_width));
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
                tab.open_doc.spell_synopsis(),
                tab.open_doc.replacement_synopsis(),
                Some(tab.synopsis_handle_sink()),
                Some(tab.format.clone()),
                Some(tab.typewriter.clone()),
                Some(tab.caret_band()),
                tab.open_doc.comment_binding_synopsis(),
                tab.open_doc.images(),
                // A trashed item's text is read-only. The banner above it is a
                // statement, not a guard: before this the content beneath it was built
                // by the same editable render path as any other tab.
                tab.open_doc.trashed.get(),
            ));
    }
    tab_backdrop(
        tab.backdrop_role(),
        writing_page_scroll(tab).child(col.child(vspace(28.0))),
    )
}

/// A quiet placeholder for contentless rows (Item/BookEnd, Item/Text): they carry
/// no editable content, so opening one shows an explanatory label, not an empty
/// editor.
pub fn placeholder(tab: &ContentTab) -> Box<dyn Widget> {
    tab_backdrop(
        tab.backdrop_role(),
        bati!(
            Center {
                child: TextWidget::new(tr!(no_content())) {
                    color: TextRole::Secondary
                }
            }
        ),
    )
}

/// A synopsis-only folder body (Folder/None): a plain grouping folder, which the matrix
/// gives *only* a synopsis — so there is nothing to segment, and no stream (it has no
/// manuscript extent).
///
/// Its synopsis is the page, not a footnote to one, so it grows with its content like
/// any other primary writing surface.
///
/// A **notes** folder used to share this body; it now gets
/// [`folder_synopsis_with_overview`] instead, because it does have a subtree to tabulate.
pub fn folder_synopsis_only(tab: &ContentTab) -> Box<dyn Widget> {
    tab_backdrop(tab.backdrop_role(), folder_synopsis_body(tab))
}

/// The synopsis page itself, without the tab backdrop — so it can be either a whole tab
/// body ([`folder_synopsis_only`]) or one segment of one
/// ([`folder_synopsis_with_overview`]), which owns the backdrop for the pair.
fn folder_synopsis_body(tab: &ContentTab) -> impl Widget {
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
                tab.open_doc.spell_synopsis(),
                tab.open_doc.replacement_synopsis(),
                Some(tab.synopsis_handle_sink()),
                Some(tab.format.clone()),
                Some(tab.typewriter.clone()),
                Some(tab.caret_band()),
                tab.open_doc.comment_binding_synopsis(),
                tab.open_doc.images(),
                // A trashed item's text is read-only. The banner above it is a
                // statement, not a guard: before this the content beneath it was built
                // by the same editable render path as any other tab.
                tab.open_doc.trashed.get(),
            ));
    }
    writing_page_scroll(tab).child(col.child(vspace(28.0)))
}

/// A **notes folder**'s body: its own synopsis page, plus an Overview of what it holds.
///
/// Two segments, not five. A notes folder has no manuscript extent — the compiler never
/// walks into it — so Full Chapter / Full Part / Full Synopsis and the Corkboard (which
/// is a view *of* a manuscript stream) would all be empty by construction. What it does
/// have is a subtree: a research folder with thirty notes in it is exactly the thing you
/// want tabulated. So it gets the one segment that applies.
///
/// This is why [`skribisto_model::overview_capable`] is not
/// `StreamLevel::for_container` — they disagree here, and only here.
pub fn folder_synopsis_with_overview(tab: &ContentTab) -> Box<dyn Widget> {
    let bar = SegmentedControl::new(tab.segment.clone())
        .segment(Segment::new(tr!(segment_notes())))
        .segment(Segment::new(tr!(overview())));
    let content = Switcher::new(tab.segment.clone())
        .child(folder_synopsis_body(tab))
        .child_boxed(crate::tabs::overview::overview_pane(tab));

    let col = VStack::new()
        .spacing(8.0)
        .child(vspace(10.0))
        .child(centered(bar, &tab.column_width))
        .child(Expand::new().child(content));
    // Remembered per type, exactly like the five-segment containers: reopening a notes
    // folder returns to whichever of its two views you last used.
    Box::new(RememberSegment {
        segment: tab.segment.clone(),
        memory: tab.view_memory.clone(),
        sub_role: tab.sub_role().clone(),
        child: Some(tab_backdrop(tab.backdrop_role(), col)),
        child_id: None,
    })
}

/// The body every folder container shares: a `SegmentedControl` over
///
/// 1. the container's **own page** — named after the container itself ("Chapter" /
///    "Part" / "Book"), because it *is* that item as a writing surface: title,
///    synopsis, and — for a chapter — its own prose;
/// 2. the **manuscript stream** — Full Chapter / Full Part / Full Book: the container
///    *and everything inside it*, as one continuous manuscript;
/// 3. **Full Synopsis** — the same rows, showing each one's synopsis instead;
/// 4. `extras`, in order (a Book's "Pace" and "Analysis"; empty for Chapter/Part);
/// 5. the **Corkboard** — the same rows as index cards;
/// 6. the **Overview** — the same rows as a sortable table.
///
/// The pairing reads as "this one" vs "this one and all of it": `Chapter` /
/// `Full Chapter`.
///
/// The `Switcher` mounts only the child at the selected index, and an out-of-range
/// selection mounts nothing (no panic).
pub fn folder_segmented(
    tab: &ContentTab,
    own_label: impl Into<LocalizedString>,
    manuscript_label: impl Into<LocalizedString>,
    extras: Vec<(LocalizedString, Box<dyn Widget>)>,
) -> Box<dyn Widget> {
    // Container-specific segments (the Book's "Pace" and "Analysis") are inserted here, in
    // order, before Corkboard and Overview. A `Vec` rather than a single `Option` because
    // the Book now has two of them, and because the positional SegmentedControl↔Switcher
    // contract is easier to keep honest when both lists are appended from the same loop
    // than when a second `Option` has to be threaded through in the same order twice.
    let mut bar = SegmentedControl::new(tab.segment.clone())
        .segment(Segment::new(own_label))
        .segment(Segment::new(manuscript_label))
        .segment(Segment::new(tr!(full_synopsis())));
    let mut content = Switcher::new(tab.segment.clone())
        .child(folder_own_pane(tab))
        .child(stream_pane(tab, SplitFlavour::Prose))
        .child(stream_pane(tab, SplitFlavour::Synopsis));
    for (label, pane) in extras {
        bar = bar.segment(Segment::new(label));
        content = content.child_boxed(pane);
    }
    // Corkboard and Overview are both real segments now; each Switcher child must sit at
    // the same positional index as its segment — the two are matched by position, not by
    // name, so a segment added without its child (or vice versa) silently shifts every
    // later view by one.
    bar = bar.segment(Segment::new(tr!(corkboard())));
    content = content.child_boxed(crate::tabs::corkboard::corkboard_pane(tab));
    let bar = bar.segment(Segment::new(tr!(overview())));
    let content = content.child_boxed(crate::tabs::overview::overview_pane(tab));

    let col = VStack::new()
        .spacing(8.0)
        .child(vspace(10.0))
        .child(centered(bar, &tab.column_width))
        // Fill the remaining height so the selected segment (especially a stream's
        // `ScrollArea`) gets a bounded viewport to fill.
        .child(Expand::new().child(content));
    // Persist the chosen view per container type, so a new tab of this type inherits
    // it (gated by the `editor.remember_view` toggle inside the memory).
    Box::new(RememberSegment {
        segment: tab.segment.clone(),
        memory: tab.view_memory.clone(),
        sub_role: tab.sub_role().clone(),
        child: Some(tab_backdrop(tab.backdrop_role(), col)),
        child_id: None,
    })
}

/// Transparent passthrough that persists the container's `SegmentedControl`
/// selection into the per-type [`EditorViewMemory`] whenever it changes, so a
/// newly-opened tab of the same item type inherits it.
///
/// `SegmentedControl` has no change-callback and [`folder_segmented`] has no build
/// context, so the effect is set up here (in a widget's `build`). Mirrors
/// `editor::DirtyOnEdit`: it adds one child and forwards layout to it unchanged.
struct RememberSegment {
    segment: Signal<usize>,
    memory: EditorViewMemory,
    sub_role: BinderItemSubRole,
    child: Option<Box<dyn Widget>>,
    child_id: Option<WidgetId>,
}

impl std::fmt::Debug for RememberSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RememberSegment").finish_non_exhaustive()
    }
}

impl Widget for RememberSegment {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let child = self.child.take().expect("RememberSegment built once");
        let id = ctx.add_boxed(child);
        self.child_id = Some(id);
        let (memory, sub_role) = (self.memory.clone(), self.sub_role.clone());
        // `ctx.effect` fires only on *changes*, not on setup — so a rebuild installs
        // a fresh observer that stays quiet until the user actually switches the
        // `SegmentedControl`. That's what keeps a rebuild of one tab from writing its
        // segment over the view another same-type tab just chose (regression-tested by
        // `tabs::tests::same_type_tabs_share_one_last_view_and_the_last_switch_wins`).
        ctx.effect(&self.segment, move |v| memory.remember(&sub_role, *v));
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child_id
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = Point::new(bounds.x, bounds.y);
            child.size = bounds.size();
        }
    }

    // A filling child must be reported here too (not just from `build`), or the
    // layout pass never places it — the container's segmented bar + panes vanish.
    // (Mirrors `editor::VisibleWhen`, which wraps the same kind of boxed body.)
    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

/// The editor's tag dots, under the subtitle.
///
/// A free function rather than an inline `.child(..)` only so the two panes that render a
/// subtitle (`folder_own_pane` and `heading`) cannot drift apart. `TagDotsRow` collapses to
/// nothing when the item is untagged, so this costs an untagged item no height.
fn subtitle_tag_dots(tab: &ContentTab) -> impl Widget {
    // The dots hug their content, and `centered` centres whatever it is given inside the
    // pane — so on their own they floated in the middle of the page, aligned with nothing.
    // The trailing spacer pushes the row out to the full column width so the dots start at
    // the same left edge as the title and the prose beneath them.
    HStack::new()
        .child(crate::tags::TagDotsRow::new(
            tab.open_doc.tags.clone(),
            tab.set_tags_fn(),
            crate::tags::tag_chip::MAX_VISIBLE_EDITOR,
        ))
        .child(Expand::horizontal().child(Spacer::new()))
}
