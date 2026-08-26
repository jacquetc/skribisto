// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Composite pane renders shared across several `(role, sub_role)` tabs.
//!
//! Each function here is a whole tab body that more than one combination reuses:
//! the [`heading`] form (Item Chapter / Part / BookBegin), the dual-pane
//! [`prose`] editor (Item Scene / ChapterScene, and the "Note" segment of Item Note's
//! own tab), the [`placeholder`] for contentless rows (Item BookEnd / Text), and the
//! folder-container bodies ([`folder_synopsis_only`] for a plain grouping folder,
//! [`folder_synopsis_with_overview`] for a notes folder, [`folder_segmented`] for the
//! three structural containers). [`item_note_segmented`] is the one function here
//! built for a single combination rather than several: it still belongs beside the
//! others because it is the third caller (after `folder_synopsis_with_overview` and
//! `folder_segmented`) of the segmented-tab machinery in this module's own
//! `remember` submodule, and that machinery is what actually needs to live in one
//! place. The fields each body
//! shows are decided by the constraint matrix (via `tab_for`), so one body covers
//! every combination in its group. The manuscript-stream pane the containers share
//! lives in [`stream`](super::stream).

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use teksilo::text_document::Alignment;
use teksilo::widgets::rich_text::EditorHandle;

use super::segments;
use teksilo::core::widget::WidgetPlacement;
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::widgets::{
    Accordion, Button, ButtonVariant, Center, Expand, GroupHeader, HStack, IconButton,
    IconButtonSize, Padding, RectWidget, ScrollArea, Segment, SegmentedControl, Spacer, Splitter,
    Switcher, TextWidget, VStack, ZStack,
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

mod manuscript;
mod remember;

use manuscript::*;
use remember::*;

/// The epigraph disclosure — the quotation set at the head of this part or chapter.
///
/// Not a book: the matrix allows `EpigraphText` on exactly four combinations (part and
/// chapter, both encodings), and a book's two rows carry only title, subtitle and
/// synopsis. This comment said "book, part or chapter" for a while, and that wording
/// reached the feature checklist and from there a help page draft before the matrix was
/// re-read.
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
    // The epigraph editor's own handle, so the attribution control below can act on the
    // block the caret is in. `synopsis_column` has always taken this sink; the epigraph
    // passed `None` because until now nothing here needed to reach the editor.
    let handle: Rc<RefCell<Option<EditorHandle>>> = Rc::new(RefCell::new(None));
    let body = VStack::new()
        .spacing(4.0)
        .child(attribution_control(handle.clone()))
        .child(synopsis_column(
            &field.doc,
            &tab.column_width,
            // Scene typography, not the synopsis's: an epigraph is finished-book matter
            // that ships in the manuscript, not editorial commentary about it.
            tab.main_typography(),
            tab.mark_dirty_fn(),
            Option::None,
            tab.open_doc.spell_epigraph(),
            tab.open_doc.replacement_epigraph(),
            // The sink is live now, for the attribution control. No comment binding
            // still: a comment anchors to the author's own prose, and an epigraph is
            // quoted matter — see `OpenDoc::build`.
            Some(handle.clone()),
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
            // An epigraph is a field on a page, never the page. Whatever else this
            // tab shows owns its remembered position.
            Option::None,
            // **Not** this item, deliberately. The registry keys an editor by
            // `(item, kind)`, and an epigraph is a second `Synopsis`-kind editor on
            // the same item as the synopsis below it — naming both would make
            // "this item's synopsis editor" resolve to whichever was built first,
            // and a lane would then convert its offsets against the wrong text.
            Option::None,
            // A tab's editor is on screen and lays out on its first frame.
            false,
            tab.capture_palette(),
        ));
    Some(crate::widgets::tip::RichTip::new(
        crate::tooltip_registry::CONCEPT_EPIGRAPH,
        Accordion::new(tr!(epigraph()), tab.epigraph_expanded.clone()).content(body),
    ))
}

/// The control that marks the caret's line as the quotation's source.
///
/// **Why this is not "align right".** Every writer already keys the attribution off
/// `Alignment::Right` inside a `SemanticRole::Epigraph`: DOCX and ODT give it the
/// `EpigraphAttribution` named style, LaTeX and Typst their own attribution slot. But
/// the Format dock deliberately does not offer Right at all, and its reason
/// (`format_vm`'s `ALIGN_OTHER`: "Right has no manuscript use") is still right for the
/// manuscript at large. So the alignment stays off the general control surface and gets
/// one scoped affordance here, where it means something: not a typographic choice, but
/// a statement that this line is the source.
///
/// Without it the whole path was dead. Nothing in the application could produce a
/// right-aligned block, so no export ever took the attribution branch, in any format.
///
/// It toggles: a line already marked goes back to `Left`. `set_alignment` acts on the
/// caret's block, so the writer marks the line their caret is in, exactly as every other
/// block-level command in the app behaves.
fn attribution_control(handle: Rc<RefCell<Option<EditorHandle>>>) -> impl Widget + 'static {
    // No `HStack` + `Spacer` to push this to one side: that pair proposes an unbounded
    // width, so the spacer takes everything and the button lands off the edge. It did,
    // and the control was simply absent from the pane until this was reduced to the
    // button itself.
    Padding::new(0.0, 0.0, 2.0, 8.0).child(
        Button::new(tr!(epigraph_mark_attribution()))
            .variant(ButtonVariant::Plain)
            .tooltip(tr!(epigraph_mark_attribution_tip()))
            .on_activate_fn(move |_ctx| {
                if let Some(editor) = handle.borrow().as_ref() {
                    let next = if editor.get_alignment() == Alignment::Right {
                        Alignment::Left
                    } else {
                        Alignment::Right
                    };
                    editor.set_alignment(next);
                }
            }),
    )
}

/// **The margin lane beside a writing page**, and the one place a lane is put
/// there.
///
/// Wrapping rather than a parameter on [`writing_page_scroll`] because the lane is
/// the scroll area's *sibling*, not its content: it maps the extent the area
/// scrolls, so it has to be outside it and beside it. `Expand` on the prose side
/// so the lane takes its declared width and the manuscript keeps the rest — the
/// measure the application centres must not move because a strip appeared.
///
/// The lane decides for itself whether to draw anything, so this is unconditional
/// and a writer's switch does not have to reach five call sites.
pub(crate) fn laned(
    tab: &ContentTab,
    surface: crate::margin_lane::LaneSurface,
    scope: crate::margin_lane::LaneScope,
    area: ScrollArea,
    content: impl Widget + 'static,
) -> impl Widget {
    // The page reports where it landed, exactly as a stream row does — and for the
    // same reason, which is not the geometry but the **signal**. The marks are
    // resolved in the lane's own layout pass, and a widget nothing dirties is never
    // laid out again: on the first frame the editor has no text geometry yet and
    // every mark resolves to nothing, so without something to hear about the reflow
    // the strip stays empty for the life of the tab.
    //
    // With one row the arithmetic is the identity — offset 0, scale 1 — so this
    // costs a wrapper and changes no position. It also means a tab and a stream go
    // down exactly one code path, which is what stops them drifting apart about
    // where a mark belongs.
    let extents = crate::margin_lane::RowExtents::new();
    let inputs = lane_inputs(tab, surface, scope, extents.clone());
    let lane = crate::margin_lane::lane_for(&area, inputs);
    let page = crate::margin_lane::RowExtent::new(
        tab.item_id(),
        extents,
        area.scroll_y_signal().clone(),
        content,
    );
    HStack::new()
        .child(Expand::new().child(area.child(page)))
        .child(lane)
}

/// **The margin lane over a stream**: one strip mapping many documents.
///
/// The rows are taken from what has actually been *placed* rather than from the
/// stream's row list, and that is a correctness requirement. Resolving a row's
/// document goes through `StreamViewModel::row_doc`, which on a cache miss opens it
/// — a full synchronous Djot import, and what once made switching a Book to Full
/// Book freeze for seconds. A placed row is a built row, so its document is already
/// open.
#[allow(clippy::too_many_arguments)]
pub(crate) fn laned_stream(
    tab: &ContentTab,
    vm: &crate::stream::StreamViewModel,
    flavour: crate::stream::SplitFlavour,
    // The page's own token, shared with every editor on it — see
    // `crate::margin_lane::LaneScope`.
    scope: crate::margin_lane::LaneScope,
    extents: crate::margin_lane::RowExtents,
    area: ScrollArea,
    content: impl Widget + 'static,
) -> impl Widget {
    use crate::margin_lane::{LaneInputs, LaneRow, LaneRows, LaneSurface};
    let app_ctx = tab.app_ctx();
    let work_id = tab.ids().work_id.get();
    let synopsis = flavour == crate::stream::SplitFlavour::Synopsis;
    // The container's own field is one more mapped document, and it registers an
    // extent like any row.
    let own = if synopsis { tab.synopsis() } else { tab.main() }.map(|f| f.doc.clone());
    let own_item = tab.item_id();
    let own_comments = if synopsis {
        tab.open_doc.comment_binding_synopsis()
    } else {
        tab.open_doc.comment_binding_main()
    };
    let own_spell = if synopsis {
        tab.open_doc.spell_synopsis()
    } else {
        tab.open_doc.spell_main()
    };
    // One backend read per item, kept: a hundred rows would otherwise pay three
    // reads each on every recompute to learn something that changes only when the
    // project's language or house quote style does.
    let markers: Rc<
        RefCell<HashMap<u64, skribisto_model::analysis::prose_stats::DialogueMarkers>>,
    > = Rc::new(RefCell::new(HashMap::new()));

    let row = {
        let vm = vm.clone();
        let app_ctx = app_ctx.clone();
        let markers = markers.clone();
        Rc::new(move |item: u64| -> Option<LaneRow> {
            let markers_for = |item: u64| {
                *markers.borrow_mut().entry(item).or_insert_with(|| {
                    crate::margin_lane::texture::markers_for_item(&app_ctx, work_id, item)
                })
            };
            if item == own_item {
                return Some(LaneRow {
                    item,
                    doc: own.clone()?,
                    comments: own_comments.clone(),
                    spell: own_spell.clone(),
                    markers: markers_for(item),
                });
            }
            let doc = vm.row_doc(item)?;
            let field = if synopsis {
                doc.synopsis.as_ref()
            } else {
                doc.main.as_ref()
            }?;
            Some(LaneRow {
                item,
                doc: field.doc.clone(),
                comments: vm.row_comments(item, flavour),
                // The row's own session, off the shared store -- the very one its
                // editor squiggles from, so the lane cannot disagree with the page.
                spell: if synopsis {
                    doc.spell_synopsis()
                } else {
                    doc.spell_main()
                },
                markers: markers_for(item),
            })
        }) as Rc<dyn Fn(u64) -> Option<LaneRow>>
    };

    let lane = crate::margin_lane::lane_for(
        &area,
        LaneInputs {
            app_ctx,
            ids: tab.ids().clone(),
            surface: LaneSurface::Stream,
            kind: if synopsis {
                crate::format::EditorKind::Synopsis
            } else {
                crate::format::EditorKind::Prose
            },
            scope,
            format: tab.format.clone(),
            rows: LaneRows::Placed { extents, row },
        },
    );
    HStack::new()
        .child(Expand::new().child(area.child(content)))
        .child(lane)
}

/// What this tab's lane reads.
fn lane_inputs(
    tab: &ContentTab,
    surface: crate::margin_lane::LaneSurface,
    scope: crate::margin_lane::LaneScope,
    extents: crate::margin_lane::RowExtents,
) -> crate::margin_lane::LaneInputs {
    let app_ctx = tab.app_ctx();
    let item = tab.item_id();
    // A tab's lane maps the **manuscript**. A synopsis is a working note beside it,
    // a few lines long, and it has no scroll area of its own for a lane to sit
    // against — see [`LaneSurface::all`](crate::margin_lane::LaneSurface::all).
    let doc = tab.main().map(|f| f.doc.clone());
    let comments = tab.open_doc.comment_binding_main();
    let spell = tab.open_doc.spell_main();
    let row = crate::margin_lane::LaneRow {
        item,
        // An empty document rather than no lane: a tab whose field the matrix does
        // not give it still scrolls, and a lane over nothing draws nothing, which is
        // the correct picture of a page with no prose on it.
        doc: doc.unwrap_or_default(),
        comments,
        spell,
        // Resolved once, not per frame: it changes only when the project's language
        // or its house quote style does, and both rebuild these surfaces.
        markers: crate::margin_lane::texture::markers_for_item(
            &app_ctx,
            tab.ids().work_id.get(),
            item,
        ),
    };
    crate::margin_lane::LaneInputs {
        app_ctx,
        ids: tab.ids().clone(),
        surface,
        kind: crate::format::EditorKind::Prose,
        // The surface this lane is *on*, so it resolves the editor beside it rather
        // than another render of the same item — a dual-pane tab has two, and a
        // scene open in a Full Chapter beside this tab is a third.
        scope,
        format: tab.format.clone(),
        rows: crate::margin_lane::LaneRows::Placed {
            extents,
            row: Rc::new(move |_| Some(row.clone())),
        },
    }
}

/// The `ScrollArea` every writing surface in the app scrolls inside, and the
/// zero-size companion that claims this tab's view-state ports while the page is
/// the one on screen. The **one** door, so the scroll range, the editors' pin and
/// the position that gets remembered can never be configured apart.
///
/// The editors on these pages are intrinsic-height with their own scroll bars
/// suppressed ("flowing page" mode), so this is what actually scrolls, and it is
/// therefore what has to buy the range past the last line that lets the final
/// paragraph reach the typewriter pin. Without it the pin would quietly stop
/// working over the last page — exactly where a writer spends their time. The
/// range collapses to zero when typewriter scrolling is off, so a page without
/// the feature cannot be scrolled past its own end.
///
/// It is also where the writer's remembered position lands, in both directions.
/// The scroll they want restored is this area's, not any editor's: the editors
/// here run with `ScrollPolicy::AlwaysOff` and grow to their content, so
/// `RichTextEditor::scroll_y()` on a prose column is permanently 0 and persisting
/// it would persist nothing.
///
/// **`restore_scroll_y`, not a write after the fact.** `ScrollArea` clamps any
/// offset to its maximum on every layout pass, and that maximum is 0 until the
/// content has been measured, so an offset written at build time is silently
/// dropped. The one-shot lands it during the first layout that gives the area a
/// real range instead, which is also what stops the page painting at the top for
/// a frame before jumping. Re-seeded on **every** build on purpose: a rebuild (a
/// Promote, a settings-driven relayout) mints a fresh `ScrollArea` at offset 0,
/// and without this it would throw the writer back to the top of the document.
///
/// **The companion is why a tab can have several pages.** The ports hold one slot,
/// so an immediate attach from every page would leave the tab restoring, and
/// reporting, the scroll of whichever happened to be *constructed* last rather
/// than the one being looked at. `folder_segmented` builds its own page and both
/// of its streams in a single pass, so that was not a hypothetical. Attaching on
/// activation instead makes the answer "the visible one" by construction. Mount
/// the companion anywhere inside the same page.
pub(crate) fn writing_page_scroll(
    tab: &ContentTab,
    will_show: bool,
) -> (ScrollArea, impl Widget, crate::shared::ViewStateBinding) {
    let area = ScrollArea::new()
        .scroll_past_end(tab.typewriter.scroll_past_end_signal())
        // Only the page the tab is about to *show* restores the offset. A container
        // builds every one of its pages in a single pass but mounts one, and a page
        // mounted later, when the writer switches to it, would otherwise lay out for
        // the first time at a position measured on a different page entirely.
        //
        // Unlike the editor handle below, this cannot be settled on activation: the
        // offset has to be armed while the `ScrollArea` is being constructed, because
        // landing it during the first laid-out frame is the whole point of
        // `restore_scroll_y`. So the page says up front whether it is the one.
        .restore_scroll_y(if will_show {
            tab.view_state().get().scroll
        } else {
            0.0
        });
    let binding = crate::shared::ViewStateBinding {
        initial: tab.view_state().get(),
        ports: tab.view_state_ports(),
        page_editor: Rc::new(RefCell::new(None)),
    };
    let port = super::editor::PageScrollPort::new(
        tab.view_state_ports(),
        area.scroll_y_signal().clone(),
        area.max_scroll_y_signal().clone(),
        binding.page_editor.clone(),
    );
    (area, port, binding)
}

/// Whether the segment `id` is the one this tab is about to show.
///
/// Asked against the **seed** first, and only then against the live signal: a
/// restored tab's own remembered page has not been applied to `tab.segment` yet at
/// the point its pages are constructed, because `RememberSegment` is what applies
/// it and it wraps them afterwards. With no seed there is nothing to apply and the
/// signal already holds what will be shown, which is the app-global remembered view
/// `ContentTab::new` seeded. A seed naming a segment this build no longer has
/// matches no page at all, so the tab opens at the top rather than restoring a
/// position onto a page it was never measured on.
pub(crate) fn segment_will_show(tab: &ContentTab, id: &str) -> bool {
    match tab.peek_segment_seed() {
        Some(seed) => seed == id,
        None => tab.segment.get() == Some(segments::segment_id(id)),
    }
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
    // The page before its content: the editors below stage their handle into this
    // page's binding, and the port that promotes it is mounted at the end.
    let (area, port, page) = writing_page_scroll(tab, segment_will_show(tab, segments::SEG_OWN));
    // One token for this page: the editors below and the lane beside them are the
    // same surface, and nothing else may answer for it. See
    // `crate::margin_lane::LaneScope`.
    let scope = crate::margin_lane::LaneScope::fresh();

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
                // A **chapter** folder carries its own prose below, and that is this
                // tab's main widget; a Part or a Book has none, so here the synopsis is
                // the page and the position worth remembering is its.
                tab.main().is_none().then(|| page.clone()),
                Some(crate::margin_lane::LaneAnchor::new(tab.item_id(), scope)),
                // A tab's editor is on screen and lays out on its first frame.
                false,
                tab.capture_palette(),
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
            Some(page.clone()),
            tab.open_doc.comment_binding_main(),
            tab.open_doc.footnote_binding_main(),
            tab.open_doc.images(),
            tab.work_unique_id(),
            // A trashed item's text is read-only. The banner above it is a
            // statement, not a guard: before this the content beneath it was built
            // by the same editable render path as any other tab.
            tab.open_doc.trashed.get(),
            // This tab's own item, so the margin lane can reach this editor by
            // name rather than through focus.
            Some(crate::margin_lane::LaneAnchor::new(tab.item_id(), scope)),
            // A tab's editor is on screen and lays out on its first frame.
            false,
            Some(tab.tags()),
        ));
    }
    // Flowing page: the editors are intrinsic-height, so this `ScrollArea` scrolls the
    // whole thing rather than each editor scrolling inside its own box.
    laned(
        tab,
        crate::margin_lane::LaneSurface::Editor,
        scope,
        area,
        col.child(vspace(28.0)).child(port),
    )
}

/// The dual-pane writing editor (Skribisto's signature): an optional title, a
/// user-toggleable synopsis editor, and the main-text editor, scrolling together
/// as one flowing page. Shared by Item/Scene, Item/ChapterScene (which adds the
/// chapter-title field) and the "Note" segment of an `Item/Note` tab (which switches
/// to Notes typography).
///
/// Body and backdrop are split the same way [`folder_synopsis_body`] and
/// [`folder_synopsis_only`] are, and for the identical reason: [`prose_body`] is the
/// content alone, reused by [`item_note_segmented`] as one segment among several
/// sharing the *one* outer backdrop [`RememberSegment::wrap`] applies to the whole
/// bar-plus-`Switcher` pair, while [`prose`] itself adds that backdrop directly for
/// the two combinations (Item/Scene, Item/ChapterScene) that use it as their entire
/// tab, with no segment bar around it at all. Wrapping [`prose_body`] a second time
/// inside a segmented tab would nest two backdrops one inside the other, which is
/// harmless to look at (same background, zero padding) but not what "byte-identical"
/// means here.
pub fn prose(tab: &ContentTab) -> Box<dyn Widget> {
    crate::tabs::shared::editor::tab_backdrop(tab.backdrop_role(), prose_body(tab))
}

/// [`prose`] without its outer backdrop. See that function's own doc for why the two
/// are split.
fn prose_body(tab: &ContentTab) -> impl Widget {
    use crate::tabs::shared::editor::{SideSync, SynopsisPaneEffects, WidthProbe};

    let wants_side = tab.synopsis_placement.map(|p| p.is_side());

    // **One scope per arm, and this is where the difference is made.** Both arms
    // build a prose column for the *same* item, and `Switcher` keeps whichever it
    // has mounted alive for the tab's life — so both stay registered, both keep the
    // geometry of their last layout, and "the editor showing this item" stopped
    // being a question with one answer. Each arm's editors and the lane beside them
    // are handed the same token; nothing else holds it. See
    // `crate::margin_lane::LaneScope`, which records what went wrong without it.
    let top_scope = crate::margin_lane::LaneScope::fresh();
    let side_scope = crate::margin_lane::LaneScope::fresh();

    // The Top layout — today's flowing page, unchanged: title, tags, the compact
    // synopsis box and the prose all scroll together.
    let top = manuscript_page(tab, Some(tab.show_synopsis.clone()), top_scope);

    // The Side layout, built lazily by the `WidthProbe`'s `Switcher` and only if the
    // writer ever actually gets it — a Top-placement project never pays for it.
    let side = {
        // One handle, shared by the effects widget (which drives the pane from the
        // setting) and the header's fold button — so "folded" has a single owner.
        let sync = SideSync::new(tab.side_splitter.clone(), tab.synopsis_side_width.clone());
        let synopsis = side_synopsis_pane(tab, sync.clone(), side_scope);
        let manuscript = manuscript_page(tab, None, side_scope);
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
    //
    // No `tab_backdrop` here, and no find banner to add either: `prose` (this body's
    // only non-segmented caller) is the one composite in this file whose backdrop
    // carries a find banner, and that banner is already applied **inside** each
    // layout's manuscript column, by `manuscript_page` itself, rather than over the
    // whole tab. So under Side it spans the prose it searches instead of stretching
    // across the synopsis strip as well. That placement is unaffected by which of
    // this body's two callers adds the outer backdrop, or when.
    WidthProbe::new(
        wants_side,
        tab.synopsis_side_width.clone(),
        Box::new(top),
        Box::new(side),
    )
}

/// A title (+ optional subtitle / synopsis) form. Shared by the title-bearing item
/// tabs: Item/Part (title + synopsis) and Item/BookBegin (book title + subtitle +
/// synopsis). The fields present are decided by `tab_for` from the constraint matrix,
/// so one body covers both.
pub fn heading(tab: &ContentTab) -> Box<dyn Widget> {
    // One page, always the one shown. Built first so the synopsis below can stage
    // its handle into it.
    let (area, port, page) = writing_page_scroll(tab, true);
    // One token for this page: the editors below and the lane beside them are the
    // same surface, and nothing else may answer for it. See
    // `crate::margin_lane::LaneScope`.
    let scope = crate::margin_lane::LaneScope::fresh();

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
                // On a heading tab the synopsis is the page, so it is this tab's main
                // widget and what its remembered caret belongs to.
                Some(page.clone()),
                Some(crate::margin_lane::LaneAnchor::new(tab.item_id(), scope)),
                // A tab's editor is on screen and lays out on its first frame.
                false,
                tab.capture_palette(),
            ));
    }
    tab_backdrop(
        tab.backdrop_role(),
        laned(
            tab,
            crate::margin_lane::LaneSurface::Editor,
            scope,
            area,
            col.child(vspace(28.0)).child(port),
        ),
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
    tab_backdrop(tab.backdrop_role(), folder_synopsis_body(tab, true))
}

/// The synopsis page itself, without the tab backdrop — so it can be either a whole tab
/// body ([`folder_synopsis_only`]) or one segment of one
/// ([`folder_synopsis_with_overview`]), which owns the backdrop for the pair.
fn folder_synopsis_body(tab: &ContentTab, will_show: bool) -> impl Widget {
    let (area, port, page) = writing_page_scroll(tab, will_show);
    // One token for this page: the editors below and the lane beside them are the
    // same surface, and nothing else may answer for it. See
    // `crate::margin_lane::LaneScope`.
    let scope = crate::margin_lane::LaneScope::fresh();

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
                // The synopsis *is* this page, so it is the tab's main widget.
                Some(page.clone()),
                Some(crate::margin_lane::LaneAnchor::new(tab.item_id(), scope)),
                // A tab's editor is on screen and lays out on its first frame.
                false,
                tab.capture_palette(),
            ));
    }
    laned(
        tab,
        crate::margin_lane::LaneSurface::Editor,
        scope,
        area,
        col.child(vspace(28.0)).child(port),
    )
}

/// A **notes folder**'s body: its own synopsis page, its story-bible card grid, an
/// overview of what it holds, and whatever else has been registered for it.
///
/// Three built-in segments, not five. A notes folder has no manuscript extent: the
/// compiler never walks into it, so Full Chapter / Full Part / Full Synopsis and the
/// Corkboard (which is a view *of* a manuscript stream) would all be empty by
/// construction. What it does have is a subtree: a research folder with thirty notes in
/// it is exactly the thing you want tabulated, and (unlike a plain research folder) a
/// notes folder is also where a discoverable cast lives, which is what the Story bible
/// segment is for.
///
/// This is why [`skribisto_model::overview_capable`] is not
/// `StreamLevel::for_container` — they disagree here, and only here.
///
/// **The Story bible segment is ordinary code, hardcoded here exactly like Notes and
/// Overview.** It does not go through [`segments::register_container_segment`]: that
/// door is for an out-of-tree extension, and this is the community edition finishing a
/// feature it already half-built (the notes folder, the discoverable flag, aliases).
/// See [`crate::tabs::story_bible_place`]'s own module doc.
///
/// Registered segments (see [`segments::register_container_segment`]) are appended
/// after Story bible and before Overview, the same relative position `folder_segmented`
/// gives them: after the container's own built-in views, before the view that closes
/// every bar. A `BinderItemSubRole::Note` gate is accepted at registration with no
/// error, so a segment registered for it and never consulted here would fail silently
/// rather than loudly; this is the other half of that contract.
pub fn folder_synopsis_with_overview(tab: &ContentTab) -> Box<dyn Widget> {
    let sub_role = tab.sub_role().clone();
    let mut items: Vec<(&str, LocalizedString, Box<dyn Widget>)> = vec![
        (
            segments::SEG_NOTES,
            tr!(segment_notes()),
            Box::new(folder_synopsis_body(
                tab,
                segment_will_show(tab, segments::SEG_NOTES),
            )) as Box<dyn Widget>,
        ),
        (
            segments::SEG_STORY_BIBLE,
            tr!(segment_story_bible()),
            crate::tabs::story_bible_place::story_bible_pane(tab),
        ),
    ];
    for spec in segments::registered_for(&sub_role) {
        // Leaked so the id borrows for the rest of this build: see `folder_segmented`,
        // which leaks the same way for the same reason: bounded by the number of
        // distinct registered segment ids in the process, not by how often a tab is
        // built.
        let id: &'static str = Box::leak(spec.id.clone().into_boxed_str());
        items.push((id, (spec.label)(), (spec.view)(tab)));
    }
    items.push((
        segments::SEG_OVERVIEW,
        tr!(overview()),
        crate::tabs::overview::overview_pane(tab),
    ));
    // Remembered per type, exactly like the five-segment containers: reopening a notes
    // folder returns to whichever view you last used. That claim used to be false:
    // `EditorViewMemory::stored` had no `Note` arm, so this wrapper was a silent
    // permanent no-op here. It has one now.
    Box::new(RememberSegment::wrap(tab, items, &[], |bar, content| {
        VStack::new()
            .spacing(8.0)
            .child(vspace(10.0))
            .child(centered(bar, &tab.column_width))
            .child(Expand::new().child(content))
    }))
}

/// An `Item/Note` tab: today's dual-pane "Note" editor plus the "Details" story-bible
/// page, and, only when the note itself carries a tag flagged discoverable, an
/// "In prose" segment reading the manuscript prose it has been declared present in.
/// See [`crate::tabs::item_note`].
///
/// Two segments *visible* for an ordinary note (a stray thought, a research clipping
/// with no story-bible tag at all): a permanently-shown, forever-blank "In prose" chip
/// would read as the feature being broken, not as "nothing declared yet", the same
/// "never state a zero as a presence" instinct the rest of this crate's story-bible
/// surfaces already follow. So the chip itself is what the discoverable-tag gate
/// controls, not whether the segment is declared at all.
///
/// **The gate is live, not read once.** All three segments are always declared here,
/// in the one static list `RememberSegment::wrap` builds its bar and `Switcher` from,
/// exactly like every other segmented tab; what makes the third one behave as if it
/// were absent for an ordinary note is `Segment::visible`, bound to
/// [`note_discoverable_signal`] rather than left at its default `true`. A bound
/// `visible` prop re-runs the bar's own overflow/selection plan on a change with no
/// rebuild of anything above the chip itself (see `SegmentedControl`'s own docs for
/// "select the neighbour"), so adding this note's first discoverable tag while the tab
/// is open grows the bar immediately, and removing the last one while the writer is on
/// that very segment lands them on the neighbour it clamps to instead of a blank pane.
/// Nothing here tears down or rebuilds the "Note" or "Details" panes, or the
/// `Switcher` that holds them, in response to a tag edit: their editors keep their
/// caret, undo history and scroll exactly as they were, because only the chip's own
/// live prop moved.
///
/// The "In prose" pane itself is still constructed unconditionally, but at no cost to
/// an ordinary note: `Switcher` lazily mounts a page only once its index is first
/// selected, and a hidden chip is unreachable by click or keyboard (see
/// `Segment::visible`'s own doc), so `note_in_prose_pane`'s own backend reads never run
/// for a note that never shows the chip that would let a writer reach it.
///
/// Reads the tags directly off [`ContentTab::open_doc`]'s `tags` field, the same shared
/// per-item mirror [`crate::tabs::note_details::note_details_pane`]'s own Tags section
/// binds, rather than issuing a fresh binder-item read: an item just opened for the
/// first time already has it populated (see `OpenDocsStore::open`), and it is the
/// shortest path to "this note's own tags" a plain function outside any
/// `BuildContext` can reach. It is also what makes the gate live at no extra
/// subscription cost: [`Signal::map`] derives from it, so the same edit that already
/// moves the tag dots row is what redrives this.
pub fn item_note_segmented(tab: &ContentTab) -> Box<dyn Widget> {
    let items: Vec<(&str, LocalizedString, Box<dyn Widget>)> = vec![
        (
            segments::SEG_NOTE_OWN,
            tr!(segment_note_own()),
            Box::new(prose_body(tab)) as Box<dyn Widget>,
        ),
        (
            segments::SEG_NOTE_DETAILS,
            tr!(segment_note_details()),
            crate::tabs::note_details::note_details_pane(tab),
        ),
        (
            segments::SEG_NOTE_IN_PROSE,
            tr!(segment_note_in_prose()),
            crate::tabs::note_in_prose::note_in_prose_pane(tab),
        ),
    ];
    let visible = [(
        segments::SEG_NOTE_IN_PROSE,
        Prop::from(note_discoverable_signal(tab)),
    )];
    Box::new(RememberSegment::wrap(
        tab,
        items,
        &visible,
        |bar, content| {
            VStack::new()
                .spacing(8.0)
                .child(vspace(10.0))
                .child(centered(bar, &tab.column_width))
                .child(Expand::new().child(content))
        },
    ))
}

/// A live reading of whether `tab`'s own item carries at least one tag flagged
/// discoverable, which is the question [`item_note_segmented`] gates its third
/// segment's *chip* by. [`Signal::map`] makes this a **derived** signal: every read
/// re-runs [`any_tag_discoverable`] against whatever [`OpenDoc::tags`](crate::models::OpenDoc::tags)
/// currently holds, so `SegmentedControl` binding to it (via `Segment::visible`) picks
/// up a tag edit the moment it fires, with no separate cache of this function's own to
/// keep in step.
///
/// A note carries at most a handful of tags, so each recompute is one small batched
/// read (`get_binder_tag_multi` over just *this item's* tag ids), never a scan of the
/// project's whole palette, the same shape
/// [`crate::tabs::story_bible_place::scene_mention_counts`] already uses for its own
/// small batched lookup. Reads are bounded by how often the bar itself rebuilds (a real
/// tag edit, or the tab's own first build), not by frame rate: see [`Signal::map`]'s own
/// docs for why a derived signal recomputing on every *read* is not the same as
/// recomputing on every *frame*.
fn note_discoverable_signal(tab: &ContentTab) -> Signal<bool> {
    let app_ctx = tab.app_ctx();
    tab.open_doc
        .tags
        .map(move |tags| any_tag_discoverable(&app_ctx, tags))
}

/// Whether any of `tag_ids` is flagged discoverable. Split out from
/// [`note_discoverable_signal`] purely so the actual decision is testable against a
/// real backend fixture without having to stand up a whole [`ContentTab`] (let alone a
/// live `Signal`) just to reach it, the same shape
/// [`crate::tabs::story_bible_place::scene_mention_counts`] is tested at.
fn any_tag_discoverable(ctx: &frontend::AppContext, tag_ids: &[u64]) -> bool {
    if tag_ids.is_empty() {
        return false;
    }
    frontend::commands::binder_tag_commands::get_binder_tag_multi(ctx, tag_ids)
        .unwrap_or_default()
        .into_iter()
        .flatten()
        .any(|t| t.discoverable)
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

    Box::new(RememberSegment::wrap(tab, items, &[], |bar, content| {
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

#[cfg(test)]
mod item_note_segmented_tests {
    use super::*;

    use frontend::commands::{
        binder_commands, binder_item_commands, binder_tag_commands, work_commands,
    };
    use frontend::common::entities::BinderItemRole;
    use frontend::direct_access::{
        CreateBinderDto, CreateBinderItemDto, CreateBinderTagDto, CreateWorkDto,
    };

    use crate::app_ids::AppIds;
    use crate::settings::{EditorTypography, EditorTypographySet};
    use crate::tabs::tab_for;

    /// A freshly-opened Work, with nothing filed in it yet: enough for
    /// `binder_tag_commands::create_binder_tag` to have a live owner to attach to.
    fn seed_work() -> (Rc<frontend::AppContext>, u64) {
        let ctx = Rc::new(frontend::AppContext::new());
        let work_id = work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default())
            .expect("create work")
            .id;
        (ctx, work_id)
    }

    /// A typography bundle with no real Settings behind it, mirroring
    /// `remember::tests::test_typography`: the tests below exercise segment
    /// reactivity, never the fonts an editor draws with.
    fn test_typography() -> EditorTypographySet {
        let bundle = |family: &str| EditorTypography {
            font_family: Signal::new(family.to_string()),
            size: Signal::new(1.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
            size_range: crate::settings::TypographySizeRange::default(),
        };
        EditorTypographySet {
            scene: bundle("Literata"),
            synopsis: bundle("Literata"),
            notes: bundle("Inter"),
            corkboard: bundle("Literata"),
            distraction_free: bundle("Literata"),
        }
    }

    /// A real `Item/Note` tab, built against a freshly-created Binder and note item
    /// inside `work_id`, so [`note_details::note_details_pane`](crate::tabs::note_details::note_details_pane)
    /// and [`note_in_prose::note_in_prose_pane`](crate::tabs::note_in_prose::note_in_prose_pane),
    /// both unconditionally constructed by [`item_note_segmented`] and not merely the
    /// one currently selected, have a real backend to read once the writer actually
    /// selects them.
    fn note_tab(ctx: &Rc<frontend::AppContext>, work_id: u64) -> ContentTab {
        let binder = binder_commands::create_binder(
            ctx,
            None,
            &CreateBinderDto {
                name: "B".into(),
                activated: true,
                ..Default::default()
            },
            work_id,
            0,
        )
        .expect("create binder");
        let item = binder_item_commands::create_binder_item(
            ctx,
            None,
            &CreateBinderItemDto {
                title: "A note".into(),
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Note,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            binder.id,
            -1,
        )
        .expect("create note item");
        let ids = AppIds::new();
        ids.work_id.set(Some(work_id));
        tab_for(
            ctx,
            item.id,
            &BinderItemRole::Item,
            &BinderItemSubRole::Note,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &ids,
        )
    }

    fn create_tag(ctx: &frontend::AppContext, work_id: u64, discoverable: bool) -> u64 {
        let now = chrono::Utc::now();
        binder_tag_commands::create_binder_tag(
            ctx,
            None,
            &CreateBinderTagDto {
                uid: Default::default(),
                created_at: now,
                updated_at: now,
                name: "character".to_string(),
                color: "#2e7d32".to_string(),
                details: String::new(),
                discoverable,
                creates_in: None,
                note_template: None,
            },
            work_id,
            -1,
        )
        .expect("create tag")
        .id
    }

    /// An untagged note (a stray thought, a research clipping) never shows the
    /// third segment: the same "no third, forever-blank tab" case
    /// [`item_note_segmented`]'s own doc explains.
    #[test]
    fn a_note_with_no_tags_at_all_is_not_discoverable() {
        let (ctx, _work_id) = seed_work();
        assert!(!any_tag_discoverable(&ctx, &[]));
    }

    /// A note carrying only ordinary, non-discoverable tags (a `status/…` workflow
    /// tag, say) stays two segments too: the gate is about the *discoverable* flag
    /// specifically, never merely "has any tag at all".
    #[test]
    fn a_note_with_only_non_discoverable_tags_is_not_discoverable() {
        let (ctx, work_id) = seed_work();
        let workflow = create_tag(&ctx, work_id, false);
        assert!(!any_tag_discoverable(&ctx, &[workflow]));
    }

    /// One discoverable tag among several is enough: the writer does not have to
    /// carry *only* story-bible tags to get the "In prose" segment.
    #[test]
    fn a_note_with_one_discoverable_tag_among_several_is_discoverable() {
        let (ctx, work_id) = seed_work();
        let workflow = create_tag(&ctx, work_id, false);
        let character = create_tag(&ctx, work_id, true);
        assert!(any_tag_discoverable(&ctx, &[workflow, character]));
    }

    /// A tag id the palette no longer holds (deleted between the read that
    /// produced this note's `tags` list and this check) is simply absent from
    /// `get_binder_tag_multi`'s result, not a reason to panic or to treat the
    /// note as discoverable by default.
    #[test]
    fn a_stale_tag_id_that_no_longer_resolves_is_not_discoverable() {
        let (ctx, _work_id) = seed_work();
        assert!(!any_tag_discoverable(&ctx, &[999_999]));
    }

    /// Before any discoverable tag exists, a seed naming "In prose" (a workspace
    /// restore from a session where the note *was* discoverable, or a tag removed
    /// between sessions) does not stick: `SegmentedControl`'s own self-heal lands the
    /// bar on a real, visible segment instead of the hidden one. Companion to
    /// [`a_live_discoverable_tag_grows_the_in_prose_chip_onto_the_bar`]: together they
    /// show the chip is genuinely gated, not merely coincidentally reachable.
    #[test]
    fn an_unreachable_in_prose_seed_falls_back_to_a_real_segment() {
        let (ctx, work_id) = seed_work();
        let tab = note_tab(&ctx, work_id);
        tab.seed_segment(segments::SEG_NOTE_IN_PROSE);

        let mut tree = crate::test_support::tree_with_events(&ctx);
        tree.add_boxed(item_note_segmented(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        assert_ne!(
            tab.segment_shown(),
            segments::SEG_NOTE_IN_PROSE,
            "a seed naming a hidden chip must not select it"
        );
        assert!(
            !tab.segment_shown().is_empty(),
            "it must land on a real segment, not a blank state"
        );
    }

    /// Adding this note's first discoverable tag while the tab is open grows the "In
    /// prose" chip onto the bar immediately, with no need to close and reopen the tab.
    /// Regression test for the behaviour `item_note_segmented`'s own doc used to record
    /// as a documented limitation.
    #[test]
    fn a_live_discoverable_tag_grows_the_in_prose_chip_onto_the_bar() {
        let (ctx, work_id) = seed_work();
        let tab = note_tab(&ctx, work_id);

        let mut tree = crate::test_support::tree_with_events(&ctx);
        tree.add_boxed(item_note_segmented(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        let tag = create_tag(&ctx, work_id, true);
        tab.open_doc.tags.set(vec![tag]);
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        tab.segment
            .set(Some(segments::segment_id(segments::SEG_NOTE_IN_PROSE)));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        assert_eq!(
            tab.segment_shown(),
            segments::SEG_NOTE_IN_PROSE,
            "the In prose chip must be selectable the moment a discoverable tag is \
             added, with no need to leave and reopen the tab"
        );
    }

    /// Removing the last discoverable tag while the writer is actually reading "In
    /// prose" must not leave them on a blank pane: the chip disappears and the bar
    /// clamps to a real neighbour, matching `SegmentedControl`'s own "select the
    /// neighbour" convention.
    #[test]
    fn removing_the_last_discoverable_tag_moves_off_in_prose_to_a_real_segment() {
        let (ctx, work_id) = seed_work();
        let tab = note_tab(&ctx, work_id);
        let tag = create_tag(&ctx, work_id, true);
        tab.open_doc.tags.set(vec![tag]);

        let mut tree = crate::test_support::tree_with_events(&ctx);
        tree.add_boxed(item_note_segmented(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        tab.segment
            .set(Some(segments::segment_id(segments::SEG_NOTE_IN_PROSE)));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));
        assert_eq!(
            tab.segment_shown(),
            segments::SEG_NOTE_IN_PROSE,
            "precondition: the writer is reading In prose"
        );

        tab.open_doc.tags.set(vec![]);
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        let shown = tab.segment_shown();
        assert_ne!(
            shown,
            segments::SEG_NOTE_IN_PROSE,
            "removing the last discoverable tag must move the writer off a chip that no \
             longer exists"
        );
        assert!(
            shown == segments::SEG_NOTE_OWN || shown == segments::SEG_NOTE_DETAILS,
            "it must land on one of the two real segments, not a blank state: landed on \
             {shown:?}"
        );
    }

    /// An unrelated tag edit (adding a second discoverable tag, say) must not throw the
    /// writer back to the "Note" segment while they are reading "In prose": only the
    /// chip's own live prop should move, never the selection.
    #[test]
    fn an_unrelated_tag_edit_does_not_move_a_writer_off_in_prose() {
        let (ctx, work_id) = seed_work();
        let tab = note_tab(&ctx, work_id);
        let first = create_tag(&ctx, work_id, true);
        tab.open_doc.tags.set(vec![first]);

        let mut tree = crate::test_support::tree_with_events(&ctx);
        tree.add_boxed(item_note_segmented(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        tab.segment
            .set(Some(segments::segment_id(segments::SEG_NOTE_IN_PROSE)));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));
        assert_eq!(
            tab.segment_shown(),
            segments::SEG_NOTE_IN_PROSE,
            "precondition: the writer is reading In prose"
        );

        let second = create_tag(&ctx, work_id, true);
        tab.open_doc.tags.set(vec![first, second]);
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        assert_eq!(
            tab.segment_shown(),
            segments::SEG_NOTE_IN_PROSE,
            "an unrelated tag edit must not move the writer off the segment they were \
             reading"
        );
    }
}
