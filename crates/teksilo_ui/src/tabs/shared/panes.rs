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

use super::segments;
use teksilo::core::widget::WidgetPlacement;
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::widgets::{
    Accordion, Center, Expand, GroupHeader, HStack, IconButton, IconButtonSize, Padding,
    RectWidget, ScrollArea, Segment, SegmentedControl, Spacer, Splitter, Switcher, TextWidget,
    VStack, ZStack,
};
use teksilo::widgets::{SegmentId, segmented_control};

use frontend::common::entities::BinderItemSubRole;

use crate::settings::EditorViewMemory;
use crate::stream::SplitFlavour;
use crate::tabs::{Boxed, ContentTab};

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
mod manuscript;
mod remember;

use manuscript::*;
use remember::*;

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
            Some(tab.writing_games()),
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
    // How long this container is meant to be, how far along it is, and what the targets
    // set inside it add up to — the last of which is explicitly not a target.
    col = col.child(centered(
        crate::tabs::shared::goal_header::container_goal_header(tab),
        &tab.column_width,
    ));
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
                Some(tab.writing_games()),
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
            Some(tab.writing_games()),
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
                Some(tab.writing_games()),
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
        teksu!(
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
                Some(tab.writing_games()),
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
    let items: Vec<(&str, LocalizedString, Box<dyn Widget>)> = vec![
        (
            segments::SEG_NOTES,
            tr!(segment_notes()),
            Box::new(folder_synopsis_body(tab)) as Box<dyn Widget>,
        ),
        (
            segments::SEG_OVERVIEW,
            tr!(overview()),
            crate::tabs::overview::overview_pane(tab),
        ),
    ];
    // Remembered per type, exactly like the five-segment containers: reopening a notes
    // folder returns to whichever of its two views you last used. That claim used to be
    // false — `EditorViewMemory::stored` had no `Note` arm, so this wrapper was a silent
    // permanent no-op here. It has one now.
    Box::new(RememberSegment::wrap(tab, items, |bar, content| {
        VStack::new()
            .spacing(8.0)
            .child(vspace(10.0))
            .child(centered(bar, &tab.column_width))
            .child(Expand::new().child(content))
    }))
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
/// The segmented body every manuscript container shares.
///
/// `extras` are the container's own additional segments — the Book's Pace and Analysis —
/// each carrying a stable id alongside its label. Segments registered through
/// [`segments::register_container_segment`] are appended after them and before Corkboard
/// and Overview, which is where an analytical view belongs rather than trailing the two
/// views *of* the manuscript.
pub fn folder_segmented(
    tab: &ContentTab,
    own_label: impl Into<LocalizedString>,
    manuscript_label: impl Into<LocalizedString>,
    extras: Vec<(&'static str, LocalizedString, Box<dyn Widget>)>,
) -> Box<dyn Widget> {
    // ONE ordered list, resolved once, feeding the bar, the `Switcher` and the
    // remembered-view lookup alike. Building any of the three from a separate pass would
    // let a registration landing in between shift one and not the others — the same
    // reasoning `tabs::analysis` records for the category bar.
    let sub_role = tab.sub_role().clone();
    let mut items: Vec<(&str, LocalizedString, Box<dyn Widget>)> = vec![
        (
            segments::SEG_OWN,
            own_label.into(),
            Box::new(folder_own_pane(tab)) as Box<dyn Widget>,
        ),
        (
            segments::SEG_MANUSCRIPT,
            manuscript_label.into(),
            Box::new(stream_pane(tab, SplitFlavour::Prose)),
        ),
        (
            segments::SEG_SYNOPSIS,
            tr!(full_synopsis()),
            Box::new(stream_pane(tab, SplitFlavour::Synopsis)),
        ),
    ];
    items.extend(extras.into_iter().map(|(id, label, pane)| {
        let id: &str = id;
        (id, label, pane)
    }));
    for spec in segments::registered_for(&sub_role) {
        // Leaked so the id borrows for the rest of this build. Bounded by the number of
        // distinct registered segment ids in the process — a handful, registered once at
        // startup — not by how often a tab is built.
        let id: &'static str = Box::leak(spec.id.clone().into_boxed_str());
        items.push((id, (spec.label)(), (spec.view)(tab)));
    }
    items.push((
        segments::SEG_CORKBOARD,
        tr!(corkboard()),
        crate::tabs::corkboard::corkboard_pane(tab),
    ));
    items.push((
        segments::SEG_OVERVIEW,
        tr!(overview()),
        crate::tabs::overview::overview_pane(tab),
    ));

    Box::new(RememberSegment::wrap(tab, items, |bar, content| {
        VStack::new()
            .spacing(8.0)
            .child(vspace(10.0))
            .child(centered(bar, &tab.column_width))
            // Fill the remaining height so the selected segment (especially a stream's
            // `ScrollArea`) gets a bounded viewport to fill.
            .child(Expand::new().child(content))
    }))
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
