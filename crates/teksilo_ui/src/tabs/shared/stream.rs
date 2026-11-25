// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **manuscript stream** pane — one body, two flavours, three containers.
//!
//! A continuous, editable view of everything inside a container: the chapter's scenes
//! (Full Chapter), a part's chapters and their scenes (Full Part), or a whole book's
//! parts, chapters and scenes (Full Book). Each row carries its own header + options
//! menu; an "Add" action closes the pane.
//!
//! The **flavour** ([`SplitFlavour`]) picks which of a row's two writing surfaces the
//! stream shows: its prose (`Prose`) or its synopsis (`Synopsis`). Everything else —
//! the rows, the headings, the menus, the split/merge gating — is identical, which is
//! why the Full Synopsis stream is this same function with one argument flipped.
//!
//! **Which rows get which editor is decided by the constraint matrix**, never by a
//! hardcoded sub_role list: `is_prose_bearing` / `is_synopsis_bearing` ask
//! `skribisto_model::content_allowed`. So a chapter folder shows a prose editor (it
//! carries its own `SceneText`, exactly like the flat chapter it promotes to), while a
//! part heading shows one only in the Synopsis flavour. Nothing here special-cases a
//! row kind for content — only for *chrome*.
//!
//! **Set in by level.** Each row sits further in the deeper it is in the outline (a
//! chapter under its part, a scene under its chapter), its header and its editor by the
//! same amount, so the two keep their offset at every depth. The depth comes from the
//! rows' own structure (`skribisto_model::compile::row_depths`), the step from Settings
//! ([`crate::stream::StreamIndent`]), and how a column is narrowed from
//! [`crate::shared::column_inset`]. A screen reader hears the same outline: each row's
//! title is a heading one level under the row it sits in.
//!
//! **Levels shown.** A Full Book or Full Part can be read as its outline: its scenes, or
//! its chapters and scenes, hidden ([`DepthFilter`]). A banner pinned above the page says
//! so while it is, and offers the way back; adding, moving, merging and splitting rows
//! are turned off meanwhile, since what they would move or make could be out of sight.
//!
//! **Read as exported.** The Prose flavour can also show the container as its reader will
//! get it, in the book's remembered export style: only what an export of it would print,
//! front and back matter included, each row under the heading the export gives it and
//! with its epigraph where the style sets it, and no working chrome (status, menu, tags,
//! add). The rows and what each prints come from the compiler itself (see
//! [`crate::models::ReadingRow`]); the editors are the same ones, so a passage that reads
//! wrong is fixed where it is read. Pages and type are not shown, and not claimed.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ComboBox, Divider, Expand, HStack, IconButton, IconWidget, MenuItem, MenuList,
    PopoverIconButton, Repeater, ScrollArea, Spacer, TextWidget, Toggle, VStack,
};

use skribisto_model::SubRoleExt;

use skribisto_compiler::EpigraphPlacement;
use skribisto_compiler::plan::{HeadingKind, RowHeading};

use crate::export::ExportStylesViewModel;
use crate::margin_lane::RowExtents;
use crate::models::{OpenDoc, ReadingRow, StreamRow};
use crate::settings::EditorTypography;
use crate::shared::{is_prose_bearing, is_synopsis_bearing};
use crate::stream::{DepthFilter, SplitFlavour, StreamViewModel};
use crate::widgets::Rebuilding;

use super::{
    HEADING_PROSE_MIN_LINES, MAIN_MIN_LINES, SplitAction, centered, synopsis_column, vspace,
    writing_column,
};

/// Pencil icon for the inline rename affordances (no built-in "edit" icon).
const EDIT_SVG: &str = include_str!("../../../resources/icons/edit.svg");

/// The scrollable manuscript: container header, the container's own writing surface,
/// one editor per row, and the add action.
///
/// Structurally identical to the single-scene tab (an outer `ScrollArea` over a flowing
/// `VStack`) so the flowing writing columns size correctly; a zero-size [`WireOnBuild`]
/// child subscribes the view-model when this pane first mounts.
pub fn stream_pane(tab: &super::super::ContentTab, flavour: SplitFlavour) -> impl Widget {
    // Two widths, not one. The chrome around the manuscript — the container
    // header, each row's header, the add button — stays on the tab's normal
    // column so the page furniture keeps its usual measure; the *prose* follows
    // whichever surface this tab belongs to, so a Full Chapter read in the
    // distraction-free surface is typeset like the mode's single-scene view
    // rather than like a docked editor.
    //
    // Without this the surface's whole point stops at the container's first
    // segment: `main_column_width()`/`main_typography()` are exactly the
    // per-surface axis, and a stream that reads `tab.column_width` /
    // `tab.typography.scene` directly opts out of it.
    let header_cw = tab.column_width.clone();
    let mut col = VStack::new().spacing(0.0);
    // Two of a container's segments come through here, and they are different
    // pages of different heights: only the one about to be shown restores the
    // remembered offset.
    let will_show = crate::tabs::shared::panes::segment_will_show(
        tab,
        match flavour {
            SplitFlavour::Prose => crate::tabs::shared::segments::SEG_MANUSCRIPT,
            SplitFlavour::Synopsis => crate::tabs::shared::segments::SEG_SYNOPSIS,
        },
    );
    // Built inside the `if let` below, because a page with no stream view-model has
    // no rows to map and needs neither the extents nor the lane.
    let mut mapped: Option<(
        crate::stream::StreamViewModel,
        crate::margin_lane::RowExtents,
        crate::margin_lane::LaneScope,
        ScrollArea,
    )> = None;
    // What the levels banner's "Show everything" runs first, to keep the passage: built
    // with the page's anchor below, used by the banner the page is wrapped in.
    let mut mapped_levels: Option<Rc<dyn Fn()>> = None;

    if let Some(vm) = tab.stream().cloned() {
        let mark_dirty: Rc<dyn Fn()> = Rc::new(tab.mark_dirty_fn());
        // A row's editor is a scene's, whichever flavour: prose uses the tab's
        // main bundle (Scene, or the distraction-free one on the surface),
        // synopses use the Synopsis bundle — a synopsis is a working note at any
        // size of window, so it does not follow the surface.
        let (editor_cw, editor_typo) = match flavour {
            SplitFlavour::Prose => (tab.main_column_width().clone(), tab.main_typography()),
            SplitFlavour::Synopsis => (tab.column_width.clone(), tab.typography.synopsis.clone()),
        };
        let format = tab.format.clone();
        // Where every mapped document on this page lands, so the lane can turn one
        // row's offset into a fraction of the whole stream. Nothing in the row-list
        // layer can answer this: typing in row 1 pushes row 2 down and fires no
        // event any of these view-models watch — see `margin_lane::rows`.
        let extents = crate::margin_lane::RowExtents::new();
        // One token for this page. Every editor on it — the container's own field
        // and each row — is the same surface as the lane beside them, and a scene
        // that is also open in a tab of its own must not answer for this one. See
        // `crate::margin_lane::LaneScope`.
        let scope = crate::margin_lane::LaneScope::in_tab(tab.item_id());
        let (page, port, _binding) =
            crate::tabs::shared::panes::writing_page_scroll(tab, will_show);
        let page_scroll = page.scroll_y_signal().clone();
        let page_max = page.max_scroll_y_signal().clone();
        // The zero-size companion that claims this tab's view-state ports on
        // activation. It goes anywhere inside the page, so it goes here rather
        // than being threaded out through `mapped` alongside the area.
        col = col.child(port);
        mapped = Some((vm.clone(), extents.clone(), scope, page));
        // The container's own surface is a commentable editor like any row's.
        let own_comments = match flavour {
            SplitFlavour::Prose => tab.open_doc.comment_binding_main(),
            SplitFlavour::Synopsis => tab.open_doc.comment_binding_synopsis(),
        };
        // One gutter for the whole page — see `ColumnWithMargin::reserve`. Seeded
        // here and kept fresh by `WireOnBuild`, which owns the only `BuildContext`
        // inside this pane.
        let gutter = Signal::new(page_gutter(&vm, own_comments.as_ref(), flavour));
        let factory = {
            let vm = vm.clone();
            let header_cw = header_cw.clone();
            let editor_cw = editor_cw.clone();
            let md = mark_dirty.clone();
            let typo = editor_typo.clone();
            let format = format.clone();
            let tw = tab.typewriter.clone();
            let band = tab.caret_band();
            let games = tab.writing_games();
            let gutter = gutter.clone();
            // Resolved once for the page rather than per row: it is one store
            // read, and a stream can be a hundred rows.
            let arrival_project = tab.arrival_project();
            let tags = tab.tags();
            let extents = extents.clone();
            let page_scroll = page_scroll.clone();
            let indent = tab.stream_indent.clone();
            move |row: &StreamRow| -> Box<dyn Widget> {
                let item = row.item_id;
                let body: Box<dyn Widget> = match &row.reading {
                    Some(reading) => Box::new(reading_row(
                        &vm,
                        row,
                        reading,
                        &ReadingKit {
                            header_width: header_cw.clone(),
                            editor_width: editor_cw.clone(),
                            typo: typo.clone(),
                            mark_dirty: md.clone(),
                            format: format.clone(),
                            typewriter: tw.clone(),
                            caret: band.clone(),
                            games: games.clone(),
                            palette: arrival_project.as_ref().map(|p| {
                                crate::tabs::shared::editor::CapturePalette {
                                    tags: tags.clone(),
                                    work_uid: p.uid.clone(),
                                }
                            }),
                        },
                        &gutter,
                        arrival_project.as_ref(),
                        scope,
                        &tags,
                    )),
                    None => Box::new(stream_row(
                        &vm,
                        row,
                        &header_cw,
                        &editor_cw,
                        &typo,
                        flavour,
                        &md,
                        &format,
                        &tw,
                        &band,
                        &games,
                        &gutter,
                        arrival_project.as_ref(),
                        scope,
                        Some(&tags),
                        &indent,
                    )),
                };
                Box::new(crate::margin_lane::RowExtent::new(
                    item,
                    extents.clone(),
                    page_scroll.clone(),
                    body,
                ))
            }
        };

        // Where the writer was reading when the stream switched between the draft and
        // the book read as exported, or showed more or fewer levels, so the switch
        // leaves them on the same passage.
        let anchor = PassageAnchor::default();
        col = col
            .child(WireOnBuild::new(
                vm.clone(),
                flavour,
                gutter.clone(),
                own_comments.clone(),
                PagePlace {
                    anchor: anchor.clone(),
                    extents: extents.clone(),
                    scroll: page_scroll.clone(),
                    max_scroll: page_max.clone(),
                },
            ))
            .child(vspace(12.0));
        // Note the passage before the levels shown change, so the page stays on it: on
        // the row at the top of the page if it stays, else on the nearest row it sits
        // under that does (the chapter a hidden scene was in).
        let on_levels: Rc<dyn Fn()> = {
            let anchor = anchor.clone();
            let extents = extents.clone();
            let page_scroll = page_scroll.clone();
            let vm = vm.clone();
            Rc::new(move || {
                anchor.note_within(&extents, page_scroll.get(), |id| vm.row_parents(id))
            })
        };
        // The bar over the rows. Read as exported is a Prose matter (a synopsis is not
        // part of the book); the levels shown are a Book's and a Part's, on both pages.
        let offers_levels = !vm.depth_filters_offered().is_empty();
        if flavour == SplitFlavour::Prose || offers_levels {
            let on_flip: Rc<dyn Fn()> = {
                let anchor = anchor.clone();
                let extents = extents.clone();
                let page_scroll = page_scroll.clone();
                Rc::new(move || anchor.note(&extents, page_scroll.get()))
            };
            col = col
                .child(centered(
                    stream_bar(&vm, flavour, on_flip, on_levels.clone()),
                    &header_cw,
                ))
                .child(vspace(8.0));
        }
        mapped_levels = Some(on_levels);
        col = col
            .child(centered(container_header(&vm), &header_cw).visible_when(vm.working_on()))
            .child(vspace(4.0));
        if flavour == SplitFlavour::Prose {
            // The container's own heading and epigraph, as the export opens it on them.
            // Its own prose is the editor just below, the same in both modes.
            col = col.child(head_reading(
                &vm,
                tab.open_doc.clone(),
                ReadingKit {
                    header_width: header_cw.clone(),
                    editor_width: editor_cw.clone(),
                    typo: editor_typo.clone(),
                    mark_dirty: mark_dirty.clone(),
                    format: format.clone(),
                    typewriter: tab.typewriter.clone(),
                    caret: tab.caret_band(),
                    games: tab.writing_games(),
                    palette: tab.capture_palette(),
                },
            ));
        }

        // The container's **own** content — it is the pane header, never a row. In the
        // Prose flavour that is a chapter folder's own `SceneText` (present after
        // promoting a flat chapter, and where that prose stays editable); a Part or a
        // Book has none. In the Synopsis flavour every container has one.
        let own = match flavour {
            SplitFlavour::Prose => tab.main(),
            SplitFlavour::Synopsis => tab.synopsis(),
        };
        if let Some(field) = own {
            // The container's own prose is one more mapped document on this page, so
            // it reports where it landed exactly as a row does. Without this the
            // lane would resolve it and then skip it for having no extent, and a
            // chapter folder's own text would be the one thing on the page with no
            // marks beside it.
            let own_col: Box<dyn Widget> = match flavour {
                SplitFlavour::Prose => Box::new(writing_column(
                    &field.doc,
                    &editor_cw,
                    &editor_typo,
                    MAIN_MIN_LINES,
                    tab.mark_dirty_fn(),
                    Option::None,
                    Option::None,
                    tab.open_doc.spell_main(),
                    tab.open_doc.replacement_main(),
                    Some(tab.format.clone()),
                    Some(tab.typewriter.clone()),
                    Some(tab.caret_band()),
                    Some(tab.writing_games()),
                    // No view-state ports: a stream is many editors on one page,
                    // so "the caret of this tab" has no single answer here. Same
                    // reason the synopsis rows below take no handle sink.
                    Option::None,
                    // Comments, on the other hand, are per *document* and each
                    // surface here has its own — so the container's own prose is
                    // commentable in the stream exactly as it is on its own tab.
                    own_comments.clone().map(|b| b.with_gutter(gutter.clone())),
                    // Same story for footnotes, with one asymmetry: the dock's
                    // "reveal this note" seek is consumed beside the view-state
                    // ports above, which a stream row deliberately has none of.
                    // So this carries only the outward half — the caret report,
                    // which is what lights up a dock row when a writer clicks a
                    // marker in a Full Book.
                    tab.open_doc.footnote_binding_main(),
                    tab.open_doc.images(),
                    tab.arrival_project(),
                    // The container's own prose, read-only while it is in the trash.
                    tab.open_doc.trashed.get(),
                    // The container itself: its own prose is one more item's text on
                    // this page, and a lane must be able to reach it by name like any
                    // row's.
                    Some(crate::margin_lane::LaneAnchor::new(tab.item_id(), scope)),
                    // Every mapped document on this page guesses its height until it
                    // has laid out, including the container's own prose: the page's
                    // height is the sum of these claims, and most of them are below
                    // the fold.
                    true,
                    Some(tab.tags()),
                    // Nothing private on a split half: the default `all()`.
                    None,
                    tr!(editor_text_name()),
                    Some(tab.open_doc.content_language()),
                )),
                SplitFlavour::Synopsis => Box::new(synopsis_column(
                    &field.doc,
                    &editor_cw,
                    &editor_typo,
                    tab.mark_dirty_fn(),
                    Option::None,
                    tab.open_doc.spell_synopsis(),
                    tab.open_doc.replacement_synopsis(),
                    // No per-tab sink: a stream shows one synopsis per row, so
                    // the last row built would win it. These rows reach the
                    // formatting surfaces through the editor registry instead
                    // (see `FormatViewModel`), which resolves by focus and so
                    // can name the row the caret is actually in.
                    Option::None,
                    Some(tab.format.clone()),
                    Some(tab.typewriter.clone()),
                    Some(tab.caret_band()),
                    Some(tab.writing_games()),
                    // The synopsis is its own `Content` row with its own threads —
                    // see the prose column above.
                    own_comments.clone().map(|b| b.with_gutter(gutter.clone())),
                    tab.open_doc.images(),
                    // The container's own prose, read-only while it is in the trash.
                    tab.open_doc.trashed.get(),
                    // One synopsis per stream row, so there is no single "the"
                    // caret for the tab to remember. Same reason as the absent sink.
                    Option::None,
                    // The container itself, like its prose column above.
                    Some(crate::margin_lane::LaneAnchor::new(tab.item_id(), scope)),
                    // Every mapped document on this page guesses its height until it
                    // has laid out, including the container's own prose: the page's
                    // height is the sum of these claims, and most of them are below
                    // the fold.
                    true,
                    tab.capture_palette(),
                    tr!(synopsis()),
                    Some(tab.open_doc.content_language()),
                )),
            };
            // Read as exported, the container's own prose is in the book only when the
            // container is: one whose own *Include in exports* is off leaves the export,
            // and the page with it.
            let in_book = vm
                .working_on()
                .zip(&vm.head_reading())
                .map(|(working, head)| *working || head.is_some());
            col = col
                .child(
                    crate::margin_lane::RowExtent::boxed(
                        tab.item_id(),
                        extents.clone(),
                        page_scroll.clone(),
                        own_col,
                    )
                    .visible_when(in_book),
                )
                .child(vspace(6.0));
        }
        if flavour == SplitFlavour::Prose {
            // The container's synopsis, after its prose, when the style prints synopses.
            col = col.child(head_synopsis(
                &vm,
                tab.open_doc.clone(),
                tab.open_doc
                    .comment_binding_synopsis()
                    .map(|b| b.with_gutter(gutter.clone())),
                ReadingKit {
                    header_width: header_cw.clone(),
                    editor_width: editor_cw.clone(),
                    // As every row's synopsis in a reading: the export prints it as prose.
                    typo: editor_typo.clone(),
                    mark_dirty: mark_dirty.clone(),
                    format: format.clone(),
                    typewriter: tab.typewriter.clone(),
                    caret: tab.caret_band(),
                    games: tab.writing_games(),
                    palette: tab.capture_palette(),
                },
            ));
        }

        col = col
            .child(Repeater::new(vm.list(), factory))
            .child(vspace(10.0))
            // Adding a row is working furniture; a reading has none.
            .child(centered(add_button(&vm), &header_cw).visible_when(vm.working_on()))
            .child(vspace(28.0));
    }
    let body: Box<dyn Widget> = match mapped {
        Some((vm, extents, scope, page)) => {
            let laned = crate::tabs::shared::panes::laned_stream(
                tab, &vm, flavour, scope, extents, page, col,
            );
            // The levels banner goes outside the lane and inside the find banner, for the
            // reason the find banner's own note gives: a strip above the page, pinned.
            let laned = levels_banner_over(
                &vm,
                mapped_levels.clone().unwrap_or_else(|| Rc::new(|| {})),
                laned,
            );
            // **Ctrl+F reads the whole stream.** A Full Chapter, Part or Book is one
            // manuscript to the writer in front of it, so the banner searches the
            // container's own prose and every row's as one run, and Next steps out of a
            // row into the next rather than stopping at its last line. Which of the two
            // stream flavours it is searching comes from the segment bar, so one
            // view-model serves both — only one of them is ever mounted.
            //
            // The lane goes **inside** the wrapper, for the reason `manuscript_page`
            // records: the banner is a strip above the page, and a lane running past it
            // would map an extent that starts below its own top.
            match tab.page_find().cloned() {
                Some(find) => Box::new(crate::tabs::shared::editor::find_banner_over(find, laned)),
                None => Box::new(laned) as Box<dyn Widget>,
            }
        }
        // No stream view-model, so no rows and nothing to map -- and no page was
        // built above either, so this one gets its own.
        None => {
            let (area, port, _binding) =
                crate::tabs::shared::panes::writing_page_scroll(tab, will_show);
            Box::new(area.child(col.child(port))) as Box<dyn Widget>
        }
    };
    crate::tabs::Boxed::new(body)
}

/// The gutter this page reserves: the margin's full column once anything on the
/// page has a card, nothing at all before that.
///
/// A stream asks the question **once for the page** rather than letting each row's
/// margin answer for itself. The margin claims its width only when its own document
/// has a comment, so per-row answers would put the commented rows on a different
/// measure from the rest — invisible in a wide window, but in a pane too tight to
/// fit the pair centred (`place_pane`'s rule 3) the commented rows shift left and
/// the manuscript zigzags down the page.
fn page_gutter(
    vm: &StreamViewModel,
    own: Option<&crate::comments::binding::CommentBinding>,
    flavour: SplitFlavour,
) -> f32 {
    let any = own.is_some_and(|b| b.has_live_cards()) || vm.any_row_has_comments(flavour);
    if any {
        crate::comments::margin::reserved_width()
    } else {
        0.0
    }
}

/// Zero-size child that wires the stream view-model on build (subscribing the row list
/// and the per-row metadata) — the one place inside the pane's widget tree that gets a
/// `BuildContext`. `wire` is idempotent.
///
/// It also keeps the page's [gutter](page_gutter) current. That belongs here for the
/// same reason the subscriptions do: the reservation is a fact about the whole page,
/// and this is the only widget in the pane positioned to watch for it changing.
struct WireOnBuild {
    vm: StreamViewModel,
    flavour: SplitFlavour,
    gutter: Signal<f32>,
    own: Option<crate::comments::binding::CommentBinding>,
    place: PagePlace,
}

/// The passage to land back on after a switch to or from the reading, and what it needs
/// to find it: where every row is, and the page's scroll and how far it can go.
struct PagePlace {
    anchor: PassageAnchor,
    extents: RowExtents,
    scroll: Signal<f32>,
    max_scroll: Signal<f32>,
}

impl WireOnBuild {
    fn new(
        vm: StreamViewModel,
        flavour: SplitFlavour,
        gutter: Signal<f32>,
        own: Option<crate::comments::binding::CommentBinding>,
        place: PagePlace,
    ) -> Self {
        Self {
            vm,
            flavour,
            gutter,
            own,
            place,
        }
    }
}

impl std::fmt::Debug for WireOnBuild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WireOnBuild").finish()
    }
}

impl Widget for WireOnBuild {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.wire(ctx);
        // Both stream pages show one row list, and the reading belongs to the prose page
        // alone: each page says when it is the one on screen.
        self.vm.follow_page(ctx, self.flavour);
        // The rows report where they land as they lay out after a switch, a few frames
        // running; each report is a chance to put the page back on the noted passage.
        {
            let anchor = self.place.anchor.clone();
            let extents = self.place.extents.clone();
            let scroll = self.place.scroll.clone();
            let max = self.place.max_scroll.clone();
            ctx.effect(&self.place.extents.generation(), move |_| {
                anchor.restore(&extents, &scroll, max.get());
            });
        }
        // Recompute the page's gutter when the comment set's *shape* changes (a
        // thread created, deleted or resolved) or when Tools ▸ Comments toggles —
        // the same two signals each row's margin rebuilds on, so the reservation
        // and the cards it makes room for move together.
        if let Some(vm) = self
            .own
            .as_ref()
            .map(|b| b.view_model())
            .or_else(|| self.vm.row_comments_any_view_model(self.flavour))
        {
            // Two effects, not one on a `zip`: a combined signal is **read-only**,
            // and `ctx.effect` observes — which panics on one. The pane looked fine
            // in every headless test because they build a stream with no comments
            // view-model at all, so this block never ran; it took launching the app
            // to find. Two subscriptions on the same recompute cost nothing here,
            // since the recompute is idempotent.
            let recompute = {
                let stream = self.vm.clone();
                let own = self.own.clone();
                let flavour = self.flavour;
                let gutter = self.gutter.clone();
                move || gutter.set(page_gutter(&stream, own.as_ref(), flavour))
            };
            let structure = vm.model().structure_signal();
            let visible = vm.visible_signal();
            {
                let recompute = recompute.clone();
                ctx.effect(&structure, move |_| recompute());
            }
            ctx.effect(&visible, move |_| recompute());
        }
        Vec::new()
    }

    fn layout_response(&self, _proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }
}

/// The container's title + a flat rename (pencil) button.
fn container_header(vm: &StreamViewModel) -> impl Widget {
    let rename_vm = vm.clone();
    HStack::new()
        .child(
            TextWidget::new(lit!(""))
                .text(vm.container_title())
                .style(TextStyleRole::BodyBold)
                .color(TextRole::Primary)
                // The rename button after the `Spacer` has to stay reachable, and a
                // wrapping label reports its full width — so a container named at any
                // length would push it off the end. See `binder::dock`.
                .single_line()
                // The top of the page's outline: every row's title is a heading under it
                // (see `OutlineHeading`).
                .access_role(teksilo::core::accesskit::Role::Heading)
                .access_customize(|node| node.set_level(1)),
        )
        .child(Spacer::new())
        .child(
            IconButton::new(IconWidget::from_svg(EDIT_SVG).icon_size(14.0))
                .embedded()
                .tooltip(tr!(rename()))
                .on_activate_fn(move |ctx| rename_vm.begin_rename_container(ctx)),
        )
}

/// A row's title, heard by a screen reader as a heading one level under the row it sits
/// in: level 2 for a row at the first level, under the container's own level-1 title.
///
/// The indent's counterpart. A sighted writer sees a scene set in under its chapter; a
/// screen reader user hears a level-3 heading under a level-2 one, and can move through
/// a Full Book by heading level as through any outline. The level follows the row's
/// depth live: the depth signal is bound at `AccessibilityOnly`, so a part added above
/// re-levels the rows below it without rebuilding them.
///
/// Laid out as its title alone: the title's own response, its shrink weight included,
/// is what lets a long one truncate in the header's row.
struct OutlineHeading {
    depth: Signal<u8>,
    pending: Option<Box<dyn Widget>>,
    child_id: Option<WidgetId>,
}

impl OutlineHeading {
    fn new(depth: Signal<u8>, title: impl Widget + 'static) -> Self {
        Self {
            depth,
            pending: Some(Box::new(title)),
            child_id: None,
        }
    }
}

impl std::fmt::Debug for OutlineHeading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OutlineHeading").finish_non_exhaustive()
    }
}

impl Widget for OutlineHeading {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        if let Some(title) = self.pending.take() {
            self.child_id = Some(ctx.add_boxed(title));
        }
        if let Some(title) = self.child_id {
            self.depth.bind_to(
                ctx.self_id(),
                ctx.binding_registry(),
                BindingLevel::AccessibilityOnly,
            );
            let depth = self.depth.clone();
            ctx.apply_handlers(
                title,
                teksilo::core::widget_builder::HandlerSet::new().access_customize(move |node| {
                    node.set_role(teksilo::core::accesskit::Role::Heading);
                    node.set_level(heading_level(depth.get()));
                }),
            );
        }
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child_id
            .and_then(|id| ctx.child_layout_response(id, proposal))
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [teksilo::core::widget::WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

/// The heading level of a row `depth` levels deep: one under the container's level 1.
/// A row whose depth is not known yet reads as the first level.
fn heading_level(depth: u8) -> usize {
    usize::from(depth.max(1)) + 1
}

/// The trailing "Add …" action. What it *creates* is the model's own default
/// recommendation for this container — a Scene inside a chapter, a Chapter inside a
/// part or a book — and the label, the dialog and the creation all ask the view-model
/// for that one type.
fn add_button(vm: &StreamViewModel) -> impl Widget {
    let row = HStack::new().child(Spacer::new());
    match vm.add_label() {
        Some(label) => {
            let editable = vm.structure_editable();
            let vm = vm.clone();
            row.child(
                Button::new(label)
                    // Greyed while levels are hidden: what it adds could be one of them.
                    .enabled(editable)
                    .on_activate_fn(move |ctx| vm.begin_add_row(ctx)),
            )
        }
        // A container with nothing to add: no button rather than one that does nothing.
        None => row,
    }
}

/// One row: a header, and — when the matrix allows it for this flavour — the row's
/// editor. A chapter or part heading gets heavier chrome than a scene, so a Full Book
/// reads as a manuscript rather than a flat pile of prose.
// The two column widths are the reason this is over the argument limit: the
// row's chrome and the row's prose deliberately measure differently (see
// `stream_pane`), and collapsing them back into one is the bug being fixed.
#[allow(clippy::too_many_arguments)]
fn stream_row(
    vm: &StreamViewModel,
    row: &StreamRow,
    // The row's header, on the tab's normal column.
    header_width: &Signal<f32>,
    // The row's prose, on whichever column this tab's surface uses.
    editor_width: &Signal<f32>,
    typo: &EditorTypography,
    flavour: SplitFlavour,
    mark_dirty: &Rc<dyn Fn()>,
    format: &crate::format::FormatViewModel,
    typewriter: &crate::shared::TypewriterSettings,
    caret: &crate::shared::CaretBand,
    // The writing games this project is playing — every row of a Full Chapter /
    // Part / Book is a manuscript surface like the tab's own, so they freeze
    // together or not at all.
    games: &crate::writing_session::WritingGamesViewModel,
    // The page's gutter reservation, shared by every row so the manuscript keeps
    // one measure down the page (see `ColumnWithMargin::reserve`).
    gutter: &Signal<f32>,
    // Which project this row's typing belongs to — forwarded straight to
    // [`writing_column`], see its own note.
    arrival_project: Option<&crate::tabs::shared::editor::ArrivalProject>,
    // The stream page this row is on, so its editor answers to that page's lane and
    // not to a tab open on the same scene — see `crate::margin_lane::LaneScope`.
    scope: crate::margin_lane::LaneScope,
    // This project's palette, for the capture submenu on each row's editor. Threaded,
    // not reached for: Tier-2 state, so `app_state` would answer with another Work's.
    tags: Option<&crate::tags::TagsViewModel>,
    // How far each level is set in, live from Settings.
    indent: &Signal<crate::stream::StreamIndent>,
) -> impl Widget {
    let id = row.item_id;
    let is_heading = row.sub_role.opens_chapter() || row.sub_role.opens_part();
    // How far in this row sits: its depth, against the deepest level the page shows, at
    // the step Settings asks for, measured against the tab's own text width. One inset
    // for the header and the editor alike, so the two keep the offset they have with no
    // indent at all, whatever the row's depth and whichever column the prose follows.
    let inset = crate::stream::row_inset(
        vm.row_depth(id),
        vm.deepest(),
        indent.clone(),
        header_width.clone(),
        // The page's comment gutter: once a row has cards, every row measures against
        // what they leave, so a squeezed row's title and text stay aligned and the step
        // stays even down the page.
        gutter.clone(),
    );

    let mut col = VStack::new()
        .spacing(4.0)
        // Structure headings breathe more than the scenes under them.
        .child(vspace(if is_heading { 22.0 } else { 10.0 }))
        .child(centered(row_header(vm, row, tags), header_width).indent(inset.clone()));

    // Does this row have a surface in *this* flavour? The constraint matrix answers —
    // a part heading has no prose, but it does have a synopsis.
    let shows_editor = match flavour {
        SplitFlavour::Prose => is_prose_bearing(&row.role, &row.sub_role),
        SplitFlavour::Synopsis => is_synopsis_bearing(&row.role, &row.sub_role),
    };
    if shows_editor && let Some(doc) = vm.row_doc(id) {
        let md = mark_dirty.clone();
        let on_change = move || md();

        // "Split scene" reads the caret and splits *this* editor's text there; the other
        // role stays whole on the source. Only offered where the backend accepts it.
        // And only while every level is shown: a split makes a scene, and while levels
        // are hidden it could be one of them.
        let split: Option<SplitAction> = vm.can_split(id).then(|| {
            let offered = vm.structure_editable();
            let vm = vm.clone();
            SplitAction::new(move |ctx: &mut EventContext, caret: usize| {
                vm.split_row(ctx, id, flavour, caret)
            })
            .offered_while(offered)
        });

        match flavour {
            SplitFlavour::Prose => {
                if let Some(field) = doc.main.as_ref() {
                    // A chapter heading's own prose is subordinate to its scenes — one
                    // line, growing with content, instead of an empty ten-line box under
                    // every chapter of a Full Book.
                    let min_lines = if is_heading {
                        HEADING_PROSE_MIN_LINES
                    } else {
                        MAIN_MIN_LINES
                    };
                    col = col.child(
                        writing_column(
                            &field.doc,
                            editor_width,
                            typo,
                            min_lines,
                            on_change,
                            split,
                            Option::None,
                            doc.spell_main(),
                            doc.replacement_main(),
                            Some(format.clone()),
                            Some(typewriter.clone()),
                            Some(caret.clone()),
                            Some(games.clone()),
                            // Per-row editor — see the container's own column above.
                            Option::None,
                            // This row's own threads. `row_doc` shares its document with
                            // any tab open on the same item, so the binding — and the
                            // cards it puts in the margin — are the same either way.
                            vm.row_comments(id, flavour)
                                .map(|b| b.with_gutter(gutter.clone())),
                            // The caret report only — see the container's own column
                            // above for why a stream row takes no seek.
                            doc.footnote_binding_main(),
                            doc.images(),
                            // Every row of a stream is the same project's, so they all count
                            // into one tally — which is the whole point of it being the
                            // project's rather than the editor's.
                            arrival_project.cloned(),
                            // Each row answers for itself: a stream shows many items, and only the
                            // ones actually in the trash are locked.
                            doc.trashed.get(),
                            // Which row this is. The reason the registry carries it at all:
                            // a lane maps every row on this page at once, and all but one
                            // of them will never have focus.
                            Some(crate::margin_lane::LaneAnchor::new(id, scope)),
                            // See the container's own column above.
                            true,
                            tags.cloned(),
                            // Nothing private on a stream row: the default `all()`.
                            None,
                            tr!(editor_text_name()),
                            Some(doc.content_language()),
                        )
                        .indent(inset.clone()),
                    );
                }
            }
            SplitFlavour::Synopsis => {
                if let Some(field) = doc.synopsis.as_ref() {
                    col = col.child(
                        synopsis_column(
                            &field.doc,
                            editor_width,
                            typo,
                            on_change,
                            split,
                            doc.spell_synopsis(),
                            doc.replacement_synopsis(),
                            // One synopsis per stream row — see the sibling call.
                            // Formatting reaches it through the editor registry.
                            Option::None,
                            Some(format.clone()),
                            Some(typewriter.clone()),
                            Some(caret.clone()),
                            Some(games.clone()),
                            // This row's synopsis threads — see above.
                            vm.row_comments(id, flavour)
                                .map(|b| b.with_gutter(gutter.clone())),
                            doc.images(),
                            // Each row answers for itself: a stream shows many items, and only the
                            // ones actually in the trash are locked.
                            doc.trashed.get(),
                            // One synopsis per stream row, so there is no single "the"
                            // caret for the tab to remember. Same reason as the absent sink.
                            Option::None,
                            // Which row this is — see the prose flavour above.
                            Some(crate::margin_lane::LaneAnchor::new(id, scope)),
                            // See the container's own column above.
                            true,
                            // Built here rather than taken from the tab: a stream row is
                            // handed the palette and the project separately, and this is
                            // the same pair the prose flavour above passes.
                            tags.map(|tags| crate::tabs::shared::editor::CapturePalette {
                                tags: tags.clone(),
                                work_uid: arrival_project.and_then(|p| p.uid.clone()),
                            }),
                            tr!(synopsis()),
                            Some(doc.content_language()),
                        )
                        .indent(inset),
                    );
                }
            }
        }
    }
    col
}

/// A row's header: its title, its label, a rule, and the options menu. A structure
/// heading is `BodyBold`/Primary above a full-width rule; a scene stays `SmallBold`/
/// Secondary, so the hierarchy reads at a glance. (`TextStyleRole` has no dedicated
/// heading variant — the prominence comes from weight, colour and spacing.)
fn row_header(
    vm: &StreamViewModel,
    row: &StreamRow,
    tags: Option<&crate::tags::TagsViewModel>,
) -> impl Widget {
    let id = row.item_id;
    let is_heading = row.sub_role.opens_chapter() || row.sub_role.opens_part();
    let (style, color) = if is_heading {
        (TextStyleRole::BodyBold, TextRole::Primary)
    } else {
        (TextStyleRole::SmallBold, TextRole::Secondary)
    };

    // An untitled chapter is named by its ordinal rather than showing a bare "3.".
    let (_, badge) = crate::models::label_and_badge("", row.fallback_label.as_deref(), row.number);
    let fallback = row.fallback_label.clone();
    let mut tag_dots = crate::tags::TagDotsRow::new(
        vm.row_tags(id),
        {
            let vm = vm.clone();
            std::rc::Rc::new(move |ids: Vec<u64>, _c: &mut EventContext| {
                vm.set_row_tags(id, &ids);
            })
        },
        crate::tags::tag_chip::MAX_VISIBLE_STREAM,
    );
    // This project's palette and not `app_state`'s, which is a throwaway one on a launch
    // that starts at the Launcher: a tagged row then resolved no tag and drew no dots.
    if let Some(tags) = tags {
        tag_dots = tag_dots.with_palette(tags.clone());
    }
    HStack::new()
        .spacing(8.0)
        .child(crate::binder::icons::sub_role_icon(&row.sub_role).icon_size(14.0))
        // The ordinal, on the structure headings only — which is exactly where the
        // exporter puts it. A scene has none, and `StructureNumber` renders nothing for
        // `None`, so the titles below stay aligned with the ones above.
        .child(crate::widgets::StructureNumber::new(badge))
        .child(OutlineHeading::new(
            vm.row_depth(id),
            // The generated name when the row has no title of its own, else the live one —
            // a rename must still show here without a reload, and a generated name has
            // nothing live to follow.
            match &fallback {
                Some(f) => TextWidget::new(lit!(f.clone()))
                    .style(style)
                    .color(color)
                    .single_line(),
                None => TextWidget::new(lit!(""))
                    .text(vm.row_title(id))
                    .style(style)
                    .color(color)
                    // Everything after this in the row — the label, the tag dots, the
                    // rule and the options menu — is pushed out by a long title unless
                    // the title truncates. See `binder::dock`.
                    .single_line(),
            },
        ))
        // The row's free-text label (blank when unset).
        .child(
            TextWidget::new(lit!(""))
                .text(vm.row_label(id))
                .color(TextRole::Secondary)
                .single_line(),
        )
        // A row the export leaves out says so, here where the draft still shows it: the
        // working stream keeps every row, and only this tells a cut scene from a kept one.
        .child(
            TextWidget::new(tr!(stream_not_exported()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary)
                .single_line()
                .visible_when(vm.row_exportable(id).map(|on| !on)),
        )
        // Tag dots, between the label and the rule. Takes no space when the row is untagged.
        .child(tag_dots)
        .child(Expand::horizontal().child(Divider::new()))
        // The rung, immediately before the options menu — the trailing edge, where the
        // glyphs line up into a scannable column down the stream and never compete with
        // the tag dots in the middle band.
        .child(status_button(vm, id))
        .child(row_menu(vm, row))
}

/// The row's status picker.
///
/// Shown on every row: `folder_segmented` only ever streams rows the writing model gives a
/// manuscript extent, all of which carry content and so are `status_capable`. The check
/// still belongs on the surfaces that can show a `BookEnd` or a `Text` — the Inspector and
/// the Overview — rather than being assumed everywhere.
fn status_button(vm: &StreamViewModel, id: u64) -> impl Widget {
    let statuses = vm.statuses();
    let current = vm.row_status(id).get();
    let set: crate::statuses::SetStatus = {
        let statuses = statuses.clone();
        std::rc::Rc::new(move |status| statuses.set_item_status(id, status))
    };
    crate::statuses::status_picker(&statuses, current, set)
}

/// The per-row options menu. Merge and split are offered only where they are legal:
/// `can_merge_into_previous` refuses to merge a row away across a chapter or part
/// boundary, and `can_split` only offers a split on a prose-bearing row. Move stays
/// enabled and no-ops at the boundaries (the row is reused, not rebuilt, on reorder, so
/// a build-time bool would go stale).
fn row_menu(vm: &StreamViewModel, row: &StreamRow) -> impl Widget {
    let id = row.item_id;
    let mk = |f: fn(&StreamViewModel, &mut EventContext, u64)| {
        let vm = vm.clone();
        move |ctx: &mut EventContext| f(&vm, ctx, id)
    };

    let mut list = MenuList::new()
        .item(
            MenuItem::new(tr!(rename()))
                .icon(IconWidget::from_svg(EDIT_SVG))
                .on_activate_fn(mk(|v, c, id| v.begin_rename_row(c, id))),
        )
        .item(
            MenuItem::new(tr!(set_label())).on_activate_fn(mk(|v, c, id| v.begin_set_label(c, id))),
        )
        .separator();
    // Named after what it creates; absent when the row recommends nothing. Greyed while
    // levels are hidden, like Move and Merge below, and bound to a signal rather than
    // decided here, because the row is reused, not rebuilt, when the levels shown change:
    // each of these acts on rows a hidden level can be made of (a flat chapter moved up
    // past a hidden scene takes that scene into it), and an action whose effect would
    // land out of sight is not offered.
    if let Some(label) = vm.insert_label(id) {
        list = list.item(
            MenuItem::new(label)
                .enabled(vm.structure_editable())
                .on_activate_fn(mk(|v, c, id| v.begin_insert_after(c, id))),
        );
    }
    list = list
        .item(
            MenuItem::new(tr!(move_up()))
                .enabled(vm.structure_editable())
                .on_activate_fn(mk(|v, c, id| v.move_row_up(c, id))),
        )
        .item(
            MenuItem::new(tr!(move_down()))
                .enabled(vm.structure_editable())
                .on_activate_fn(mk(|v, c, id| v.move_row_down(c, id))),
        );

    if vm.can_merge_into_previous(id) {
        list = list.item(
            MenuItem::new(tr!(merge_with_previous()))
                .enabled(vm.structure_editable())
                .on_activate_fn(mk(|v, c, id| v.merge_into_previous(c, id))),
        );
    }

    list = list.separator().item(
        MenuItem::new(tr!(move_to_trash()))
            .text_role(TextRole::Error)
            .on_activate_fn(mk(|v, c, id| v.trash_row(c, id))),
    );

    // The kebab is already the "there is more here" glyph, so the disclosure
    // caret `PopoverIconButton` paints in its corner would be a second one
    // competing with it — the same reason the comments card suppresses it under
    // its chevron.
    PopoverIconButton::new(IconButton::more())
        .bare()
        .show_disclosure_caret(false)
        .content(list)
}

// ── Read as exported ─────────────────────────────────────────────────────────

/// The bar over the rows: the switch to read the book as exported and its style, on the
/// prose page; the levels shown, for a Book or a Part, on both pages.
fn stream_bar(
    vm: &StreamViewModel,
    flavour: SplitFlavour,
    on_flip: Rc<dyn Fn()>,
    on_levels: Rc<dyn Fn()>,
) -> impl Widget {
    let mut bar = HStack::new().spacing(12.0);
    bar = if flavour == SplitFlavour::Prose {
        bar.child(Expand::horizontal().child(reading_bar(vm, on_flip)))
    } else {
        bar.child(Spacer::new())
    };
    if !vm.depth_filters_offered().is_empty() {
        bar = bar.child(levels_control(vm, on_levels));
    }
    bar
}

/// The levels shown: the choices this container offers (see [`DepthFilter::offered`]).
/// Gone while the book is read as exported, which shows every row it prints.
///
/// `before` runs ahead of every change, to note the passage being read.
fn levels_control(vm: &StreamViewModel, before: Rc<dyn Fn()>) -> impl Widget {
    let pick = vm.clone();
    ComboBox::from_items(
        vm.depth_filters_offered().iter().copied(),
        vm.depth_choice(),
        |filter: &DepthFilter| filter.label(),
    )
    .label(tr!(stream_levels_shown()))
    .on_select(move |filter, _ctx| {
        before();
        pick.set_depth_filter(*filter);
    })
    .visible_when(vm.working_on())
}

/// `body` under the levels banner: what is hidden, that the structural actions are off,
/// and the way back. Pinned above the page like the find banner, since the writer deep
/// in a long Full Book is the one who needs reminding; occupying nothing while every
/// level is shown.
fn levels_banner_over(
    vm: &StreamViewModel,
    before: Rc<dyn Fn()>,
    body: impl Widget + 'static,
) -> impl Widget {
    VStack::new()
        .spacing(0.0)
        .child(crate::tabs::shared::editor::VisibleWhen::new(
            vm.filter_on(),
            levels_banner(vm, before),
        ))
        .child(Expand::new().child(body))
}

/// The banner's content, rebuilt when the choice changes so its title names what is
/// hidden now. An info banner, not a warning: nothing is wrong, something is out of
/// sight. It reads as a status to a screen reader, so the change is announced.
fn levels_banner(vm: &StreamViewModel, before: Rc<dyn Fn()>) -> impl Widget {
    let choice = vm.depth_choice();
    let make_vm = vm.clone();
    Rebuilding::new(
        move |ctx: &mut BuildContext| {
            choice.bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        },
        move |ctx: &mut BuildContext| {
            // Never shown without something hidden; the fallback only fills the slot.
            let title = make_vm
                .depth_filter()
                .hidden()
                .unwrap_or_else(|| tr!(stream_show_everything()));
            let show_all = make_vm.clone();
            let before = before.clone();
            ctx.add(
                teksilo::widgets::Banner::info(title)
                    .description(tr!(stream_hidden_locked()))
                    .action(Button::new(tr!(stream_show_everything())).on_activate_fn(
                        move |_ctx| {
                            before();
                            show_all.set_depth_filter(DepthFilter::Everything);
                        },
                    )),
            )
        },
    )
}

/// The switch between the working stream and the book read as exported, and the style
/// the reading follows.
///
/// Rebuilt when the writer's own styles change (Settings ▸ Export Formats), so the picker
/// always offers what the Export panel offers. `on_flip` runs just before the switch, to
/// note the passage being read.
fn reading_bar(vm: &StreamViewModel, on_flip: Rc<dyn Fn()>) -> impl Widget {
    let make_vm = vm.clone();
    Rebuilding::new(
        |ctx: &mut BuildContext| {
            if let Some(styles) = ctx.app_state::<ExportStylesViewModel>().cloned() {
                styles.changed_signal().bind_to(
                    ctx.self_id(),
                    ctx.binding_registry(),
                    BindingLevel::Rebuild,
                );
            }
        },
        move |ctx: &mut BuildContext| {
            let catalogue =
                crate::export::style::catalogue(ctx.app_state::<ExportStylesViewModel>());
            let toggle_vm = make_vm.clone();
            let flip = on_flip.clone();
            let pick_vm = make_vm.clone();
            // A new style adds and removes rows too (a dedication, a synopsis), so it keeps
            // the passage exactly as the switch does.
            let pick_flip = on_flip.clone();
            let style = ComboBox::from_items(catalogue, make_vm.reading_style(), |p| {
                crate::export::preset_display_name(p)
            })
            .label(tr!(stream_reading_style()))
            .on_select(move |p, _ctx| {
                pick_flip();
                pick_vm.choose_reading_style(p);
            })
            .visible_when(make_vm.reading_on());
            ctx.add(
                HStack::new()
                    .spacing(12.0)
                    .child(
                        Toggle::new(make_vm.reading_on())
                            .label(tr!(stream_read_as_exported()))
                            .tooltip(tr!(stream_read_as_exported_tip()))
                            .on_change(move |on, _ctx| {
                                flip();
                                toggle_vm.set_reading_on(on);
                            }),
                    )
                    .child(Expand::horizontal().child(style)),
            )
        },
    )
}

/// The furniture every editor of a reading shares, threaded once rather than as a
/// dozen arguments per row.
#[derive(Clone)]
struct ReadingKit {
    /// Headings and page rules, on the tab's normal column.
    header_width: Signal<f32>,
    /// The prose, on whichever column this tab's surface uses.
    editor_width: Signal<f32>,
    typo: EditorTypography,
    mark_dirty: Rc<dyn Fn()>,
    format: crate::format::FormatViewModel,
    typewriter: crate::shared::TypewriterSettings,
    caret: crate::shared::CaretBand,
    games: crate::writing_session::WritingGamesViewModel,
    palette: Option<crate::tabs::shared::editor::CapturePalette>,
}

/// The container's own heading and epigraph, read as exported. Empty in the working
/// stream, and for a container that prints neither (a book under a title page).
fn head_reading(vm: &StreamViewModel, doc: Rc<OpenDoc>, kit: ReadingKit) -> impl Widget {
    let head = vm.head_reading();
    let make_head = head.clone();
    Rebuilding::new(
        move |ctx: &mut BuildContext| {
            head.bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        },
        move |ctx: &mut BuildContext| match make_head.get() {
            Some(reading) => ctx.add(reading_opening(&reading, Some(&doc), &kit)),
            None => ctx.add(VStack::new()),
        },
    )
}

/// The container's own synopsis in a reading, when the style prints synopses. Empty
/// otherwise, and in the working stream, where the Full Synopsis view shows it.
fn head_synopsis(
    vm: &StreamViewModel,
    doc: Rc<OpenDoc>,
    comments: Option<crate::comments::binding::CommentBinding>,
    kit: ReadingKit,
) -> impl Widget {
    let head = vm.head_reading();
    let make_head = head.clone();
    Rebuilding::new(
        move |ctx: &mut BuildContext| {
            head.bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        },
        move |ctx: &mut BuildContext| {
            let shown = make_head.get().is_some_and(|r| r.synopsis);
            match doc.synopsis.as_ref().filter(|_| shown) {
                Some(field) => {
                    let md = kit.mark_dirty.clone();
                    ctx.add(synopsis_column(
                        &field.doc,
                        &kit.editor_width,
                        &kit.typo,
                        move || md(),
                        Option::None,
                        doc.spell_synopsis(),
                        doc.replacement_synopsis(),
                        Option::None,
                        Some(kit.format.clone()),
                        Some(kit.typewriter.clone()),
                        Some(kit.caret.clone()),
                        Some(kit.games.clone()),
                        comments.clone(),
                        doc.images(),
                        doc.trashed.get(),
                        Option::None,
                        Option::None,
                        true,
                        kit.palette.clone(),
                        tr!(synopsis()),
                        Some(doc.content_language()),
                    ))
                }
                None => ctx.add(VStack::new()),
            }
        },
    )
}

/// What opens a row of a reading, above its prose: the gap or page rule that separates
/// it from the row before, then the epigraph and the heading in the order the style sets.
fn reading_opening(reading: &ReadingRow, doc: Option<&OpenDoc>, kit: &ReadingKit) -> VStack {
    let mut col = VStack::new().spacing(4.0).child(reading_gap(reading, kit));
    let epigraph = |col: VStack| match doc.and_then(|d| epigraph_editor(d, kit)) {
        Some(editor) => col.child(editor).child(vspace(6.0)),
        None => col,
    };
    if reading.epigraph == Some(EpigraphPlacement::BeforeHeading) {
        col = epigraph(col);
    }
    if let Some(heading) = &reading.heading {
        col = col
            .child(centered(reading_heading(heading), &kit.header_width))
            .child(vspace(6.0));
    }
    if reading.epigraph == Some(EpigraphPlacement::AfterHeading) {
        col = epigraph(col);
    }
    col
}

/// The space above a row of a reading: a rule where the style opens a new page, room
/// above a heading, and nothing at all between two scenes the export runs together.
///
/// That last one is the point. The working stream heads every scene, which reads as a
/// break; the export only breaks where the writer typed a marker, so two scenes with none
/// between them reach the reader as one run of prose, and this is the only place they
/// can see that.
fn reading_gap(reading: &ReadingRow, kit: &ReadingKit) -> Box<dyn Widget> {
    if reading.first {
        Box::new(vspace(8.0))
    } else if reading.starts_page {
        Box::new(
            VStack::new()
                .child(vspace(26.0))
                .child(centered(Divider::new(), &kit.header_width))
                .child(vspace(26.0)),
        )
    } else if reading.heading.is_some() {
        Box::new(vspace(22.0))
    } else {
        Box::new(vspace(0.0))
    }
}

/// A heading as the export words it: centred, structure headings in the primary colour,
/// a kept scene title quieter under them.
fn reading_heading(heading: &RowHeading) -> impl Widget {
    let color = match heading.kind {
        HeadingKind::Structure(_) => TextRole::Primary,
        HeadingKind::SceneTitle => TextRole::Secondary,
    };
    TextWidget::new(lit!(heading.text.clone()))
        .style(TextStyleRole::BodyBold)
        .color(color)
        .text_align(TextAlign::Center)
}

/// A row's epigraph editor, or `None` for a row that has no epigraph field.
///
/// The same editor the chapter's own tab shows (see `panes::epigraph_section`), set in
/// Scene typography for the same reason: an epigraph is finished-book matter. Without
/// that tab's attribution button, which is working furniture, and without comments,
/// which anchor to the author's own prose and not to quoted matter.
fn epigraph_editor(doc: &OpenDoc, kit: &ReadingKit) -> Option<impl Widget> {
    let field = doc.epigraph.as_ref()?;
    let md = kit.mark_dirty.clone();
    Some(synopsis_column(
        &field.doc,
        &kit.editor_width,
        &kit.typo,
        move || md(),
        Option::None,
        doc.spell_epigraph(),
        doc.replacement_epigraph(),
        Option::None,
        Some(kit.format.clone()),
        Some(kit.typewriter.clone()),
        Some(kit.caret.clone()),
        Some(kit.games.clone()),
        Option::None,
        doc.images(),
        doc.trashed.get(),
        Option::None,
        // Not this item, for the reason `panes::epigraph_section` gives: the registry keys
        // an editor by `(item, kind)`, and the epigraph shares its kind with the synopsis.
        Option::None,
        true,
        kit.palette.clone(),
        tr!(epigraph()),
        Some(doc.content_language()),
    ))
}

/// One row of a reading: its opening (see [`reading_opening`]), its prose in the same
/// editor the working stream gives it, and its synopsis if the style prints one. No
/// header, status, menu or tags: a reader sees none of them.
#[allow(clippy::too_many_arguments)]
fn reading_row(
    vm: &StreamViewModel,
    row: &StreamRow,
    reading: &ReadingRow,
    kit: &ReadingKit,
    gutter: &Signal<f32>,
    arrival_project: Option<&crate::tabs::shared::editor::ArrivalProject>,
    scope: crate::margin_lane::LaneScope,
    tags: &crate::tags::TagsViewModel,
) -> impl Widget {
    let id = row.item_id;
    let doc = vm.row_doc(id);
    let mut col = reading_opening(reading, doc.as_deref(), kit);
    let Some(doc) = doc else {
        return col;
    };
    // The row's own prose, whatever it is: a scene's, a chapter's, a paratext page's. An
    // empty one prints nothing and shows a single blank line, so it neither hides a row
    // the writer is in the middle of clearing nor opens a gap the reader will not see.
    if let Some(field) = doc.main.as_ref() {
        let md = kit.mark_dirty.clone();
        col = col.child(writing_column(
            &field.doc,
            &kit.editor_width,
            &kit.typo,
            HEADING_PROSE_MIN_LINES,
            move || md(),
            // No split here: cutting a scene in two is restructuring, which a reading
            // leaves to the working stream.
            Option::None,
            Option::None,
            doc.spell_main(),
            doc.replacement_main(),
            Some(kit.format.clone()),
            Some(kit.typewriter.clone()),
            Some(kit.caret.clone()),
            Some(kit.games.clone()),
            Option::None,
            // A read-through is when remarks get written, so the margin stays.
            vm.row_comments(id, SplitFlavour::Prose)
                .map(|b| b.with_gutter(gutter.clone())),
            doc.footnote_binding_main(),
            doc.images(),
            arrival_project.cloned(),
            doc.trashed.get(),
            Some(crate::margin_lane::LaneAnchor::new(id, scope)),
            true,
            Some(tags.clone()),
            None,
            tr!(editor_text_name()),
            Some(doc.content_language()),
        ));
    }
    if reading.synopsis
        && let Some(field) = doc.synopsis.as_ref()
    {
        let md = kit.mark_dirty.clone();
        col = col.child(synopsis_column(
            &field.doc,
            &kit.editor_width,
            &kit.typo,
            move || md(),
            Option::None,
            doc.spell_synopsis(),
            doc.replacement_synopsis(),
            Option::None,
            Some(kit.format.clone()),
            Some(kit.typewriter.clone()),
            Some(kit.caret.clone()),
            Some(kit.games.clone()),
            vm.row_comments(id, SplitFlavour::Synopsis)
                .map(|b| b.with_gutter(gutter.clone())),
            doc.images(),
            doc.trashed.get(),
            Option::None,
            Option::None,
            true,
            kit.palette.clone(),
            tr!(synopsis()),
            Some(doc.content_language()),
        ));
    }
    col
}

/// How long after a switch the page keeps steering back to the noted passage. The rows
/// re-lay out over a few frames; past this, a row moving is the writer's own doing
/// (typing) and the page must not be pulled away from them. A scroll ends it sooner.
const ANCHOR_WINDOW: std::time::Duration = std::time::Duration::from_millis(1500);

/// The passage the writer was reading when the stream switched between the draft and the
/// book read as exported, or showed more or fewer levels.
///
/// Noted by row rather than by pixel: the two modes put different rows above it (a
/// dedication appears, a cut scene goes, the scenes are hidden), so the same offset would
/// land on a different page of the book. If the noted row is not in the new mode, the
/// first row after it that is takes its place, or for a change of levels the nearest row
/// it sits under (see [`note_within`](Self::note_within)).
#[derive(Clone, Default)]
struct PassageAnchor(Rc<RefCell<Option<Anchored>>>);

struct Anchored {
    /// The row at the top of the page, then every row after it, in page order.
    candidates: Vec<u64>,
    /// How far into the first one the page had scrolled. Negative only when the page
    /// had stopped in the gap between two rows; never at the top of the page, which
    /// notes nothing at all (see [`PassageAnchor::note`]).
    offset: f32,
    since: std::time::Instant,
    /// Consecutive reports that found the page already in place.
    settled: u8,
    /// Where the page was last put, by the switch or by this anchor. The page anywhere
    /// else means the writer scrolled it, and the anchor lets go at once.
    placed_at: f32,
}

impl PassageAnchor {
    /// Note the row at the top of the page, scrolled `scroll_y` down.
    ///
    /// **Nothing is noted while the page's head is on screen**: the bar, the container's
    /// header and anything above the first row. That is where every switch lives, and
    /// the writer who just used one is looking at it; the page stays exactly where it is,
    /// whatever appears or goes below. Noting the first row there and putting it back
    /// would scroll the page by whatever changed above that row (front matter a reading
    /// adds, an interlude scene a filter hides or shows), and with it the switch out of
    /// sight.
    fn note(&self, extents: &RowExtents, scroll_y: f32) {
        let mut placed: Vec<(u64, f32, f32)> = extents
            .items()
            .into_iter()
            .filter_map(|id| extents.placement(id).map(|(top, h)| (id, top, h)))
            .collect();
        placed.sort_by(|a, b| a.1.total_cmp(&b.1));
        if placed.first().is_some_and(|(_, top, _)| *top > scroll_y) {
            *self.0.borrow_mut() = None;
            return;
        }
        let next = placed
            .iter()
            .position(|(_, top, h)| top + h > scroll_y)
            .map(|first| Anchored {
                candidates: placed[first..].iter().map(|p| p.0).collect(),
                offset: scroll_y - placed[first].1,
                since: std::time::Instant::now(),
                settled: 0,
                placed_at: scroll_y,
            });
        *self.0.borrow_mut() = next;
    }

    /// Note the row at the top of the page, scrolled `scroll_y` down, before a change of
    /// the levels shown. Should the change hide that row, the page goes to the nearest of
    /// `parents(row)` still shown (the chapter a hidden scene was in, then its part)
    /// before any row after it: hiding the scenes of the chapter being read leaves the
    /// writer on that chapter, not on the next one.
    fn note_within(&self, extents: &RowExtents, scroll_y: f32, parents: impl Fn(u64) -> Vec<u64>) {
        self.note(extents, scroll_y);
        let mut slot = self.0.borrow_mut();
        if let Some(a) = slot.as_mut()
            && let Some(&top) = a.candidates.first()
        {
            let parents = parents(top);
            let after: Vec<u64> = a.candidates[1..]
                .iter()
                .copied()
                .filter(|id| !parents.contains(id))
                .collect();
            a.candidates = std::iter::once(top).chain(parents).chain(after).collect();
        }
    }

    /// Put the page back on the noted passage, now that the rows have moved.
    ///
    /// `max_scroll` tells a page the area pulled back itself (rows left and the content
    /// got shorter) from one the writer scrolled: only the second lets go.
    fn restore(&self, extents: &RowExtents, scroll: &Signal<f32>, max_scroll: f32) {
        let target = {
            let mut slot = self.0.borrow_mut();
            let Some(a) = slot.as_mut() else {
                return;
            };
            let now = scroll.get();
            let clamped = a.placed_at > max_scroll && (now - max_scroll).abs() < 1.0;
            let scrolled_away = (now - a.placed_at).abs() >= 1.0 && !clamped;
            if scrolled_away || a.since.elapsed() > ANCHOR_WINDOW || a.settled >= 2 {
                *slot = None;
                return;
            }
            // Clamped here, before it is compared, so a page that is already at its top
            // reads as in place rather than as a target it can never reach.
            let target = a.candidates.iter().enumerate().find_map(|(i, id)| {
                extents
                    .placement(*id)
                    .map(|(top, _)| if i == 0 { top + a.offset } else { top }.max(0.0))
            });
            match target {
                Some(t) if (scroll.get() - t).abs() < 1.0 => {
                    a.settled += 1;
                    None
                }
                Some(t) => {
                    a.settled = 0;
                    a.placed_at = t;
                    Some(t)
                }
                None => None,
            }
        };
        // Set outside the borrow: moving the page can report extents straight back.
        if let Some(t) = target {
            scroll.set(t);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_ids::AppIds;
    use crate::models::OpenDocsStore;
    use frontend::AppContext;
    use frontend::common::entities::BinderItemRole::Folder;
    use frontend::common::entities::BinderItemSubRole::ChapterScene;

    fn stream() -> StreamViewModel {
        let ctx = Rc::new(AppContext::new());
        let docs = OpenDocsStore::new(ctx.clone());
        StreamViewModel::new(ctx, AppIds::new(), docs, 1, &Folder, &ChapterScene)
            .expect("a chapter folder hosts a stream")
    }

    /// A stream whose store has the project's comments view-model installed, the
    /// way `App::build` wires it — but with no rows yet.
    ///
    /// Real model only: the mock `StreamRowsModel` ignores the head id and hands
    /// back a fabricated stream for every container, which is the whole point of
    /// the mocks build but leaves no way to construct the empty one this fixture
    /// is for.
    #[cfg(not(feature = "mocks"))]
    fn stream_with_comments_installed() -> StreamViewModel {
        use crate::comments::CommentsViewModel;
        use crate::models::CommentsListModel;
        let ctx = Rc::new(AppContext::new());
        let docs = OpenDocsStore::new(ctx.clone());
        docs.set_comments(CommentsViewModel::new(
            CommentsListModel::new(ctx.clone(), AppIds::new()),
            ctx.clone(),
            Signal::new(None),
        ));
        StreamViewModel::new(ctx, AppIds::new(), docs, 1, &Folder, &ChapterScene)
            .expect("a chapter folder hosts a stream")
    }

    /// A switch lands back on the passage being read: the noted row moved down 400 px
    /// (a dedication appeared above it), and the page follows it, keeping the offset into
    /// the row.
    #[test]
    fn the_reading_anchor_follows_the_noted_row() {
        let extents = RowExtents::new();
        extents.report(1, 0.0, 300.0);
        extents.report(2, 300.0, 500.0);
        let scroll = Signal::new(350.0);
        let anchor = PassageAnchor::default();
        anchor.note(&extents, scroll.get());

        extents.report(2, 700.0, 500.0);
        anchor.restore(&extents, &scroll, 5000.0);
        assert_eq!(scroll.get(), 750.0, "row 2, 50 px in, as before the switch");
    }

    /// A noted row the new mode leaves out hands the passage to the next one still there.
    #[test]
    fn a_noted_row_that_left_hands_over_to_the_next() {
        let extents = RowExtents::new();
        extents.report(1, 0.0, 300.0);
        extents.report(2, 300.0, 200.0);
        extents.report(3, 500.0, 400.0);
        let scroll = Signal::new(320.0);
        let anchor = PassageAnchor::default();
        anchor.note(&extents, scroll.get());

        extents.forget(2);
        extents.report(3, 300.0, 400.0);
        anchor.restore(&extents, &scroll, 5000.0);
        assert_eq!(
            scroll.get(),
            300.0,
            "the cut scene's successor, from its top"
        );
    }

    /// **The anchor never fights the writer.** Once they scroll, a row moving afterwards
    /// (typing, a late layout) leaves the page where they put it.
    #[test]
    fn a_scroll_by_the_writer_releases_the_anchor() {
        let extents = RowExtents::new();
        extents.report(1, 0.0, 300.0);
        extents.report(2, 300.0, 500.0);
        let scroll = Signal::new(350.0);
        let anchor = PassageAnchor::default();
        anchor.note(&extents, scroll.get());

        scroll.set(1200.0);
        extents.report(2, 700.0, 500.0);
        anchor.restore(&extents, &scroll, 5000.0);
        assert_eq!(scroll.get(), 1200.0, "left where the writer scrolled");
        extents.report(2, 800.0, 500.0);
        anchor.restore(&extents, &scroll, 5000.0);
        assert_eq!(scroll.get(), 1200.0, "and for good");
    }

    /// The page pulled back by the area itself, because the content got shorter, is not
    /// the writer scrolling: the anchor still puts the page on its row.
    #[test]
    fn a_page_the_area_clamped_is_still_anchored() {
        let extents = RowExtents::new();
        extents.report(1, 0.0, 300.0);
        extents.report(2, 2000.0, 500.0);
        let scroll = Signal::new(2100.0);
        let anchor = PassageAnchor::default();
        anchor.note(&extents, scroll.get());

        // A long cut scene above row 2 left: the content is shorter, and the area clamped.
        scroll.set(900.0);
        extents.report(2, 600.0, 500.0);
        anchor.restore(&extents, &scroll, 900.0);
        assert_eq!(scroll.get(), 700.0, "row 2, 100 px in");
    }

    /// **A switch made at the top of the page leaves it at the top**, whatever the
    /// switch changes below: the bar and the header the writer just used stay in sight.
    /// Noting the first row there used to put the page onto it, out of sight of both.
    #[test]
    fn a_switch_at_the_top_of_the_page_leaves_it_at_the_top() {
        // Rows that go: the levels hidden take the second row away.
        let extents = RowExtents::new();
        extents.report(1, 120.0, 300.0);
        extents.report(2, 420.0, 500.0);
        let scroll = Signal::new(0.0);
        let anchor = PassageAnchor::default();
        anchor.note_within(&extents, scroll.get(), |_| Vec::new());
        extents.forget(2);
        anchor.restore(&extents, &scroll, 5000.0);
        assert_eq!(
            scroll.get(),
            0.0,
            "rows going below leave the page where it was"
        );

        // The first row itself goes (a prologue chapter beside the parts, under "Show
        // parts only"), with nothing above it to hand over to.
        let anchor = PassageAnchor::default();
        anchor.note_within(&extents, scroll.get(), |_| Vec::new());
        extents.forget(1);
        extents.report(3, 120.0, 400.0);
        anchor.restore(&extents, &scroll, 5000.0);
        assert_eq!(scroll.get(), 0.0, "the first row going leaves it too");

        // Something appears above the first row (front matter a reading prints).
        let anchor = PassageAnchor::default();
        anchor.note(&extents, scroll.get());
        extents.report(3, 150.0, 400.0);
        anchor.restore(&extents, &scroll, 5000.0);
        assert_eq!(scroll.get(), 0.0, "and so does something growing above it");
    }

    /// Hiding the passage being read lands the page on the row it sat under (the
    /// chapter a hidden scene was in), not on the next row still shown.
    #[test]
    fn a_hidden_row_hands_the_passage_to_the_row_it_sits_under() {
        let extents = RowExtents::new();
        extents.report(1, 0.0, 100.0);
        extents.report(2, 100.0, 500.0);
        extents.report(3, 600.0, 100.0);
        let scroll = Signal::new(350.0);
        let anchor = PassageAnchor::default();
        anchor.note_within(&extents, scroll.get(), |id| {
            if id == 2 { vec![1] } else { Vec::new() }
        });

        // The scene is hidden: it leaves, and the next chapter moves up under the first.
        extents.forget(2);
        extents.report(3, 100.0, 100.0);
        anchor.restore(&extents, &scroll, 5000.0);
        assert_eq!(scroll.get(), 0.0, "on the chapter the scene was in");
    }

    /// Showing more levels keeps the row being read: its offset into the row holds
    /// while the rows that appear above push it down.
    #[test]
    fn showing_more_levels_keeps_the_row_being_read() {
        let extents = RowExtents::new();
        extents.report(1, 0.0, 100.0);
        extents.report(3, 100.0, 300.0);
        let scroll = Signal::new(150.0);
        let anchor = PassageAnchor::default();
        anchor.note_within(&extents, scroll.get(), |_| Vec::new());

        extents.report(2, 100.0, 500.0);
        extents.report(3, 600.0, 300.0);
        anchor.restore(&extents, &scroll, 5000.0);
        assert_eq!(scroll.get(), 650.0, "row 3, 50 px in, as before");
    }

    /// **The majority case, and the one that must not regress.** A page with no
    /// comments anywhere reserves nothing, so every manuscript that has never been
    /// annotated is typeset exactly as it was before the margin existed.
    ///
    /// The reservation is a floor applied to *every* row at once, so getting this
    /// wrong would not be subtle — it would put a permanent empty 328 dp column
    /// beside every stream in the app.
    #[test]
    fn a_page_with_no_comments_reserves_no_gutter() {
        let vm = stream();
        assert_eq!(page_gutter(&vm, None, SplitFlavour::Prose), 0.0);
        assert_eq!(page_gutter(&vm, None, SplitFlavour::Synopsis), 0.0);
    }

    /// Enumerating the page's comments is safe with no project behind it — the
    /// state every headless widget test builds a stream in.
    #[test]
    fn asking_a_bare_stream_for_its_comments_is_a_safe_no() {
        let vm = stream();
        assert!(!vm.any_row_has_comments(SplitFlavour::Prose));
        assert!(vm.row_comments(42, SplitFlavour::Prose).is_none());
        assert!(
            vm.row_comments_any_view_model(SplitFlavour::Prose)
                .is_none()
        );
    }

    /// **An empty container still has to watch the comment store.**
    ///
    /// `WireOnBuild` installs the gutter-recompute effect once, during the pane's
    /// build, and only if it can resolve a comments view-model. Resolving it via
    /// some row's binding answers `None` for a container that has no rows *yet* —
    /// a freshly created Chapter, a Part whose scenes are unwritten — and a
    /// container that also has no prose of its own (a Part, a Book) has no `own`
    /// binding to fall back on either. The page then never subscribes at all, and
    /// its gutter stays frozen at `0.0` for the life of the tab: add a scene,
    /// comment on it, and that row's margin grows while every other row keeps the
    /// stale reservation — the zigzag this whole mechanism exists to prevent.
    ///
    /// So the resolution must not depend on a row existing.
    ///
    /// Real model only, and not because the behaviour is: the *precondition* is
    /// unconstructable under `mocks`, where `StreamRowsModel` fabricates rows for
    /// every container regardless of head id. Running it there would assert
    /// nothing — a stream with rows resolves the view-model through a row binding,
    /// which is exactly the path this test exists to prove unnecessary.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn a_rowless_stream_still_resolves_the_comments_view_model_to_watch() {
        let vm = stream_with_comments_installed();
        assert!(
            vm.rows().is_empty(),
            "the point of the test is a container with no rows"
        );
        for flavour in [SplitFlavour::Prose, SplitFlavour::Synopsis] {
            assert!(
                vm.row_comments_any_view_model(flavour).is_some(),
                "a rowless page must still find a view-model to subscribe to \
                 ({flavour:?}), or its gutter never updates again"
            );
        }
    }
}
