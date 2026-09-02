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

use std::rc::Rc;

use teksilo::prelude::*;
use teksilo::widgets::{
    Button, Divider, Expand, HStack, IconButton, IconWidget, MenuItem, MenuList, PopoverIconButton,
    Repeater, ScrollArea, Spacer, TextWidget, VStack,
};

use skribisto_model::{CreateType, SubRoleExt};

use crate::models::{StreamLevel, StreamRow};
use crate::settings::EditorTypography;
use crate::shared::{is_prose_bearing, is_synopsis_bearing};
use crate::stream::{SplitFlavour, StreamViewModel};

use super::{
    HEADING_PROSE_MIN_LINES, MAIN_MIN_LINES, SplitFn, centered, synopsis_column, vspace,
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

    if let Some(vm) = tab.stream().cloned() {
        let mark_dirty: Rc<dyn Fn()> = Rc::new(tab.mark_dirty_fn());
        // A row's editor is a scene's, whichever flavour: prose uses the tab's
        // main bundle (Scene, or the distraction-free one on the surface),
        // synopses use the Synopsis bundle — a synopsis is a working note at any
        // size of window, so it does not follow the surface.
        let (editor_cw, editor_typo) = match flavour {
            SplitFlavour::Prose => (
                tab.main_column_width().clone(),
                tab.main_typography().clone(),
            ),
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
        let scope = crate::margin_lane::LaneScope::fresh();
        let (page, port, _binding) =
            crate::tabs::shared::panes::writing_page_scroll(tab, will_show);
        let page_scroll = page.scroll_y_signal().clone();
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
            let arrival_project = tab.work_unique_id();
            let tags = tab.tags();
            let extents = extents.clone();
            let page_scroll = page_scroll.clone();
            move |row: &StreamRow| -> Box<dyn Widget> {
                let item = row.item_id;
                Box::new(crate::margin_lane::RowExtent::new(
                    item,
                    extents.clone(),
                    page_scroll.clone(),
                    stream_row(
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
                        arrival_project.as_deref(),
                        scope,
                        Some(&tags),
                    ),
                ))
            }
        };

        col = col
            .child(WireOnBuild::new(
                vm.clone(),
                flavour,
                gutter.clone(),
                own_comments.clone(),
            ))
            .child(vspace(12.0))
            .child(centered(container_header(&vm), &header_cw))
            .child(vspace(4.0));

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
                    tab.work_unique_id(),
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
                )),
            };
            col = col
                .child(crate::margin_lane::RowExtent::boxed(
                    tab.item_id(),
                    extents.clone(),
                    page_scroll.clone(),
                    own_col,
                ))
                .child(vspace(6.0));
        }

        col = col
            .child(Repeater::new(vm.list(), factory))
            .child(vspace(10.0))
            .child(centered(add_button(&vm), &header_cw))
            .child(vspace(28.0));
    }
    let body: Box<dyn Widget> = match mapped {
        Some((vm, extents, scope, page)) => {
            let laned = crate::tabs::shared::panes::laned_stream(
                tab, &vm, flavour, scope, extents, page, col,
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
}

impl WireOnBuild {
    fn new(
        vm: StreamViewModel,
        flavour: SplitFlavour,
        gutter: Signal<f32>,
        own: Option<crate::comments::binding::CommentBinding>,
    ) -> Self {
        Self {
            vm,
            flavour,
            gutter,
            own,
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
                .single_line(),
        )
        .child(Spacer::new())
        .child(
            IconButton::new(IconWidget::from_svg(EDIT_SVG).icon_size(14.0))
                .embedded()
                .tooltip(tr!(rename()))
                .on_activate_fn(move |ctx| rename_vm.begin_rename_container(ctx)),
        )
}

/// The trailing "Add …" action. What it *creates* is the model's own default
/// recommendation for this container — a Scene inside a chapter, a Chapter inside a
/// part or a book — so the label names that type rather than hardcoding "scene".
fn add_button(vm: &StreamViewModel) -> impl Widget {
    let label = match vm.level() {
        StreamLevel::Chapter => tr!(add_scene()),
        StreamLevel::Part | StreamLevel::Book => tr!(add_chapter()),
    };
    let vm = vm.clone();
    HStack::new()
        .child(Spacer::new())
        .child(Button::new(label).on_activate_fn(move |ctx| vm.begin_add_row(ctx)))
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
    // [`writing_column`], see its own note. `None` on an unsaved project.
    arrival_project: Option<&str>,
    // The stream page this row is on, so its editor answers to that page's lane and
    // not to a tab open on the same scene — see `crate::margin_lane::LaneScope`.
    scope: crate::margin_lane::LaneScope,
    // This project's palette, for the capture submenu on each row's editor. Threaded,
    // not reached for: Tier-2 state, so `app_state` would answer with another Work's.
    tags: Option<&crate::tags::TagsViewModel>,
) -> impl Widget {
    let id = row.item_id;
    let is_heading = row.sub_role.opens_chapter() || row.sub_role.opens_part();

    let mut col = VStack::new()
        .spacing(4.0)
        // Structure headings breathe more than the scenes under them.
        .child(vspace(if is_heading { 22.0 } else { 10.0 }))
        .child(centered(row_header(vm, row), header_width));

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
        let split: Option<SplitFn> = vm.can_split(id).then(|| {
            let vm = vm.clone();
            Rc::new(move |ctx: &mut EventContext, caret: usize| {
                vm.split_row(ctx, id, flavour, caret)
            }) as SplitFn
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
                    col = col.child(writing_column(
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
                        arrival_project.map(str::to_string),
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
                    ));
                }
            }
            SplitFlavour::Synopsis => {
                if let Some(field) = doc.synopsis.as_ref() {
                    col = col.child(synopsis_column(
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
                            work_uid: arrival_project.map(str::to_string),
                        }),
                    ));
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
fn row_header(vm: &StreamViewModel, row: &StreamRow) -> impl Widget {
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
    HStack::new()
        .spacing(8.0)
        .child(crate::binder::icons::sub_role_icon(&row.sub_role).icon_size(14.0))
        // The ordinal, on the structure headings only — which is exactly where the
        // exporter puts it. A scene has none, and `StructureNumber` renders nothing for
        // `None`, so the titles below stay aligned with the ones above.
        .child(crate::widgets::StructureNumber::new(badge))
        .child(
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
        )
        // The row's free-text label (blank when unset).
        .child(
            TextWidget::new(lit!(""))
                .text(vm.row_label(id))
                .color(TextRole::Secondary)
                .single_line(),
        )
        // Tag dots, between the label and the rule. Takes no space when the row is untagged.
        .child(crate::tags::TagDotsRow::new(
            vm.row_tags(id),
            {
                let vm = vm.clone();
                std::rc::Rc::new(move |ids: Vec<u64>, _c: &mut EventContext| {
                    vm.set_row_tags(id, &ids);
                })
            },
            crate::tags::tag_chip::MAX_VISIBLE_STREAM,
        ))
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
        .separator()
        .item(
            MenuItem::new(insert_label(row))
                .on_activate_fn(mk(|v, c, id| v.begin_insert_after(c, id))),
        )
        .item(MenuItem::new(tr!(move_up())).on_activate_fn(mk(|v, c, id| v.move_row_up(c, id))))
        .item(
            MenuItem::new(tr!(move_down())).on_activate_fn(mk(|v, c, id| v.move_row_down(c, id))),
        );

    if vm.can_merge_into_previous(id) {
        list = list.item(
            MenuItem::new(tr!(merge_with_previous()))
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

/// What "Insert …" on this row will actually create — the model's default
/// recommendation for it. A scene (or a chapter head) recommends a Scene; a *part*
/// heading recommends a Chapter, so the label must say so rather than lie about a scene.
fn insert_label(row: &StreamRow) -> LocalizedString {
    let recommended = skribisto_model::recommendations(&row.role, &row.sub_role)
        .first()
        .map(|r| r.create_type);
    match recommended {
        Some(CreateType::Chapter) => tr!(insert_chapter()),
        _ => tr!(insert_scene()),
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
