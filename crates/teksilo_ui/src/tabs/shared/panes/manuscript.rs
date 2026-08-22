// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The writing page itself: the prose column, and the synopsis beside it.
//!
//! The two are one arrangement rather than two widgets — where the synopsis
//! sits (below the prose, or in a second column that scrolls with it) is a
//! per-project setting, and both placements share the caret and scroll state
//! that keeps them in step.

use super::*;

/// The manuscript as a flowing page: the optional chapter title, the tag row, an
/// optional compact synopsis box, and the prose — all scrolling together.
///
/// `compact_synopsis` is the Top layout's gate. `None` means this page is the
/// manuscript **column of the Side layout**, where the synopsis lives in its own
/// splitter pane and must not also appear here.
pub(super) fn manuscript_page(
    tab: &ContentTab,
    compact_synopsis: Option<Signal<bool>>,
) -> impl Widget {
    // Both layouts arm the remembered offset, and deliberately so. Unlike a
    // container's segments, which are different pages of different heights, Top and
    // Side show the *same* prose at the same position: whichever the window's width
    // resolves to wants the offset the writer left. Which of them is on screen is
    // not knowable here anyway, because `WidthProbe` treats Side as a preference and
    // vetoes it when the column would be too narrow to write in, so a decision taken
    // from the setting alone would disarm the page that actually shows.
    let (area, port, page) = writing_page_scroll(tab, true);
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
                Some(tab.writing_games()),
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
            Some(tab.writing_games()),
            Some(page.clone()),
            tab.open_doc.comment_binding_main(),
            tab.open_doc.footnote_binding_main(),
            tab.open_doc.images(),
            tab.work_unique_id(),
            // A trashed item's text is read-only. The banner above it is a
            // statement, not a guard: before this the content beneath it was built
            // by the same editable render path as any other tab.
            tab.open_doc.trashed.get(),
        ));
    }

    // This page owns the tab's scroll and its editor handle only while it is the page
    // on screen: the two layouts each build one, and the tab holds a single caret and
    // a single scroll position. The port mounted at the end of the column is what
    // settles that on activation.
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
pub(super) fn side_synopsis_pane(tab: &ContentTab, sync: SideSync) -> impl Widget {
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
                    Some(tab.writing_games()),
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
