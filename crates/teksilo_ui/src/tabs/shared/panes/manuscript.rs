// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The writing page itself: the prose column, and the synopsis beside it.
//!
//! The two are one arrangement rather than two widgets — where the synopsis
//! sits (below the prose, or in a second column that scrolls with it) is a
//! per-project setting, and both placements share the caret and scroll state
//! that keeps them in step.

use super::*;

use crate::shared::is_prose_bearing;

/// The manuscript as a flowing page: the optional chapter title, the tag row, an
/// optional compact synopsis box, and the prose — all scrolling together.
///
/// `compact_synopsis` is the Top layout's gate. `None` means this page is the
/// manuscript **column of the Side layout**, where the synopsis lives in its own
/// splitter pane and must not also appear here.
///
/// `scope` is minted by the caller rather than here, and that is the whole point:
/// `prose` builds this page **twice** — once per synopsis layout — so the two must
/// be told apart, and the Side layout's synopsis pane, which this function does not
/// build, has to share the arm's answer. An editor and the lane beside it agree on
/// which surface they are only because one value reaches both. See
/// [`LaneScope`](crate::margin_lane::LaneScope).
pub(super) fn manuscript_page(
    tab: &ContentTab,
    compact_synopsis: Option<Signal<bool>>,
    scope: crate::margin_lane::LaneScope,
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
                // This tab's item and this arm's surface, so a lane on the synopsis
                // surface can reach this editor and not the other arm's. The Top
                // layout and the Side layout below are two renders of the same
                // field, and both stay mounted once built.
                Some(crate::margin_lane::LaneAnchor::new(tab.item_id(), scope)),
                // See the prose column above.
                false,
                tab.capture_palette(),
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
            // Only a page whose main field is the book's own prose counts what is
            // typed into it. See `counts_toward_the_manuscript`: this page is built
            // for four combinations and only three of them are the manuscript. The
            // uid is read only once the answer is yes, because it is a store read
            // and a Note tab has no use for it.
            counts_toward_the_manuscript(tab)
                .then(|| tab.work_unique_id())
                .flatten(),
            // A trashed item's text is read-only. The banner above it is a
            // statement, not a guard: before this the content beneath it was built
            // by the same editable render path as any other tab.
            tab.open_doc.trashed.get(),
            // This tab's own item and this arm's surface, so the margin lane beside
            // *this* page reaches this editor rather than the other arm's.
            Some(crate::margin_lane::LaneAnchor::new(tab.item_id(), scope)),
            // A tab's editor is on screen and lays out on its first frame.
            false,
            Some(tab.tags()),
        ));
    }

    // This page owns the tab's scroll and its editor handle only while it is the page
    // on screen: the two layouts each build one, and the tab holds a single caret and
    // a single scroll position. The port mounted at the end of the column is what
    // settles that on activation.
    //
    // The lane goes **inside** the find banner's wrapper, beside the page it maps:
    // the banner is a strip above the whole editor, and a lane running past it would
    // be mapping an extent that starts below its own top.
    let page = super::laned(
        tab,
        crate::margin_lane::LaneSurface::Editor,
        scope,
        area,
        col.child(port),
    );
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
pub(super) fn side_synopsis_pane(
    tab: &ContentTab,
    sync: SideSync,
    scope: crate::margin_lane::LaneScope,
) -> impl Widget {
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
                    // See the Top layout's call above. The scope is the Side arm's,
                    // shared with the manuscript column beside it.
                    Some(crate::margin_lane::LaneAnchor::new(tab.item_id(), scope)),
                    false,
                    tab.capture_palette(),
                ))),
        ),
        None => Box::new(vspace(0.0)),
    };
    ZStack::new()
        .child(RectWidget::new().background(SurfaceRole::Main))
        .child(Padding::new(0.0, 12.0, 0.0, 12.0).child(Boxed::new(body)))
}

/// Whether what is typed on this page is the manuscript, and so whether the
/// arrival tally should be armed for it.
///
/// [`manuscript_page`] is built for four combinations and only three of them are
/// the book: `shared::prose` reaches it for `Item/Scene` and `Item/ChapterScene`,
/// `item_paratext` for `Item/Paratext`, and the Note tab's segmented body for
/// `Item/Note`. The last two carry `ParatextText` and `NoteText`, which the
/// constraint matrix keeps out of `SceneText` deliberately, and which word
/// counting therefore already excludes. Counting their keystrokes reports a note
/// as prose in the Arrivals category, beside a word count that never saw it.
///
/// The predicate is `is_prose_bearing` rather than a list of sub_roles, for the
/// reason the stream gives at its own call: the constraint matrix is the one
/// place that decides what carries scene prose, and a second list here would be a
/// second answer to drift from it.
///
/// Do not reach for `prose_kind_for` instead. It maps `Item/Paratext` to
/// `ProseKind::Scene` on purpose, because a preface is typeset like the body it
/// sits beside, so it answers a question about typography and this is a question
/// about the manuscript.
fn counts_toward_the_manuscript(tab: &ContentTab) -> bool {
    is_prose_bearing(tab.role(), tab.sub_role())
}

#[cfg(test)]
mod tests {
    use super::*;

    use frontend::AppContext;
    use frontend::common::entities::BinderItemRole;

    use crate::app_ids::AppIds;
    use crate::editors::test_support::test_typography;
    use crate::settings::EditorViewMemory;
    use crate::tabs::tab_for;

    fn tab(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> ContentTab {
        let ctx = Rc::new(AppContext::new());
        tab_for(
            &ctx,
            1,
            role,
            sub_role,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            EditorViewMemory::detached(true),
            &AppIds::new(),
        )
    }

    /// The three combinations the constraint matrix gives a `SceneText` to are the
    /// manuscript, in both of a chapter's encodings.
    #[test]
    fn a_page_showing_the_books_own_prose_counts_what_is_typed_into_it() {
        for (role, sub_role) in [
            (BinderItemRole::Item, BinderItemSubRole::Scene),
            (BinderItemRole::Item, BinderItemSubRole::ChapterScene),
            (BinderItemRole::Folder, BinderItemSubRole::ChapterScene),
        ] {
            assert!(
                counts_toward_the_manuscript(&tab(&role, &sub_role)),
                "{role:?}/{sub_role:?} carries the book's own prose and must be counted"
            );
        }
    }

    /// ⚠ The regression this exists for. Both of these reach [`manuscript_page`]
    /// through the same shared body, and both used to arm the tally, so a writer
    /// pasting research into a note had it signed into the record as manuscript
    /// text that arrived by paste.
    #[test]
    fn a_note_or_a_paratext_page_does_not() {
        for (role, sub_role) in [
            (BinderItemRole::Item, BinderItemSubRole::Note),
            (BinderItemRole::Item, BinderItemSubRole::Paratext),
        ] {
            assert!(
                !counts_toward_the_manuscript(&tab(&role, &sub_role)),
                "{role:?}/{sub_role:?} is not the manuscript and must not be counted"
            );
        }
    }
}
