// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **Mounting a lane beside a writing surface.**
//!
//! The providers say where their marks are; this is what asks them, and when.
//!
//! ## Why the marks are resolved during layout
//!
//! Because that is the only moment the answer exists. A mark's position comes
//! from the editor's laid-out geometry, and the editor is laid out during layout;
//! before the first pass there is no geometry at all, and after every edit there
//! is different geometry. Resolving at build time would freeze the marks at
//! whatever the document looked like when the pane was constructed.
//!
//! So [`LaneHost`] does what [`CommentMargin`](crate::comments::margin) does:
//! computes in the layout pass and publishes the result. The difference is where
//! it publishes to — `CommentMargin` paints its own marks, and this hands them to
//! a widget that is not its own.
//!
//! **That publication is only safe because the lane binds its marks at
//! `RepaintOnly`.** Writing a signal from a layout pass is exactly the shape
//! `ScrollArea` uses for its own scroll metrics, and its source carries the
//! warning that binding one of them at `Relayout` "becomes an instant layout
//! loop". The lane's own tests pin the level.
//!
//! ## Why it is guarded twice
//!
//! Layout runs on every dirty frame, which while a writer is typing is every
//! frame. Walking the document sixty times a second to produce the same marks is
//! not acceptable, so nothing is recomputed unless an **input** actually moved —
//! a revision, a query, a comment, a height. And the publication itself is
//! guarded again by a fingerprint, so a recompute that produces identical marks
//! does not dirty the lane either.

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::widgets::{LaneBar, LaneMark, LaneSpan, MarginLane};
use common::types::EntityId;
use skribisto_model::analysis::prose_stats::DialogueMarkers;
use teksilo::canvas::Rect;
use teksilo::core::binding::BindingLevel;
use teksilo::core::widget::{LayoutContext, LayoutResponse, PaintContext, Widget, WidgetPlacement};
use teksilo::core::widget_id::WidgetId;
use teksilo::prelude::*;
use teksilo::widgets::ScrollArea;

use super::{CommentAnchor, LaneCall, LaneExtent, LaneSurface, locate, query, resolve, texture};
use crate::format::EditorKind;

/// One document the lane maps.
///
/// A tab has one; a stream has as many as it has rows.
#[derive(Clone)]
pub struct LaneRow {
    pub item: EntityId,
    pub doc: teksilo::text_document::TextDocument,
    /// This document's comment door, for the live anchors. `None` where the
    /// surface has none, and the comments provider then simply contributes
    /// nothing rather than falling back to the stored offsets, which lag.
    pub comments: Option<crate::comments::binding::CommentBinding>,
    /// How this item marks speech, for the texture. Resolved once: it changes
    /// only when the project's language or house quote style does, and both of
    /// those rebuild the surfaces anyway.
    pub markers: DialogueMarkers,
}

/// Everything a mounted lane reads, gathered once at build.
///
/// One shape for both flavours rather than two, because the tab case is genuinely
/// the stream case with one row and the whole extent — and two code paths that have
/// to agree about how a mark is placed is how they stop agreeing.
pub struct LaneInputs {
    pub app_ctx: Rc<frontend::AppContext>,
    pub ids: crate::app_ids::AppIds,
    pub surface: LaneSurface,
    pub kind: EditorKind,
    pub format: crate::format::FormatViewModel,
    pub rows: LaneRows,
}

/// What a lane maps: the documents currently **placed** on its surface.
///
/// One case, not two, and a tab is genuinely the degenerate stream: one row whose
/// slice is the whole strip. Keeping a separate single-document variant would have
/// been two code paths that must agree about where a mark belongs, which is how
/// they stop agreeing — and the tab needs the placement report anyway, because that
/// is the only thing that tells its lane a reflow happened.
pub enum LaneRows {
    /// The rows are taken from what has actually been **placed**, and resolved
    /// through `row` on demand.
    ///
    /// ⚠ `row` must not open a document. It is only ever called for an item the
    /// extents already hold, which means the row was built, which means its
    /// document is already open — `StreamViewModel::row_doc` is then a cache hit.
    /// Called for a row that was never built it would perform a full synchronous
    /// Djot import, which is what once made switching a Book to Full Book freeze
    /// for seconds.
    Placed {
        extents: super::RowExtents,
        row: Rc<dyn Fn(EntityId) -> Option<LaneRow>>,
    },
}

impl LaneRows {
    /// The rows themselves, without asking where they sit.
    ///
    /// For the binding pass in `build`, which wants each row's editor and nothing
    /// about its geometry — and where a placeholder extent would read as if the
    /// numbers meant something.
    fn placed_rows(&self) -> Vec<LaneRow> {
        let Self::Placed { extents, row } = self;
        extents
            .items()
            .into_iter()
            .filter_map(|item| row(item))
            .collect()
    }

    fn extents(&self) -> &super::RowExtents {
        let Self::Placed { extents, .. } = self;
        extents
    }
}

/// One mapped row, in **both** coordinate spaces.
///
/// The two are deliberately kept apart and deliberately both here, because they
/// answer different questions and only one of them may move.
///
/// `extent` is the **map**: the row's share of the manuscript, which must hold
/// still while the writer reads. `pixels` is where the row actually landed in the
/// scroll area, which is revised for as long as estimated heights keep being
/// replaced by real ones. Mapping marks with `pixels` is the bug this module's
/// history is mostly about; but the viewport box is not a mark, and drawing *it*
/// from the map would be the opposite mistake -- it would stop tracking what the
/// writer can actually see.
///
/// So: the map is stable, the box is truthful, and this struct is the only place
/// the two meet.
struct MappedRow {
    row: LaneRow,
    /// The whole row's slice of the map, furniture included.
    ///
    /// What the **viewport box** is measured against, because scrolling through an
    /// epigraph is still scrolling through this row.
    extent: LaneExtent,
    /// The slice the row's **prose** occupies, which is the extent above narrowed
    /// to where the editor's text actually sits inside the row.
    ///
    /// A writing page is not only its manuscript. A Scene carries an epigraph and a
    /// synopsis editor above the prose, a stream row carries its heading, and all of
    /// it is inside the row and scrolls with it. Placing marks against the whole row
    /// put the first paragraph of the scene at the top of the strip while the
    /// viewport box, which measures real pixels, was still down in the epigraph:
    /// the two disagreed by exactly the height of the furniture. Reported against a
    /// Scene view, where the furniture is tall enough to see; the streams had it too
    /// and hid it, their rows being separated by nothing thicker than a heading.
    ///
    /// `locate.rs` names this as its one deliberate approximation and rests it on
    /// "for a tab that is invisible: there is nothing else in the extent". There is.
    text: LaneExtent,
    /// Top and height in the scroll content's own pixels.
    pixels: (f32, f32),
}

/// One row's placement in both spaces, without the row.
///
/// What the two conversions actually need, and what the jump handler keeps: a
/// `LaneRow` owns a `TextDocument` and a comment binding, and holding those alive
/// in a closure to answer a question about geometry would be both wasteful and a
/// lifetime nobody intended.
#[derive(Clone, Copy)]
struct RowSpan {
    /// Top and height in the scroll content's own pixels.
    pixels: (f32, f32),
    /// The row's slice of the map.
    extent: LaneExtent,
}

impl MappedRow {
    fn span(&self) -> RowSpan {
        RowSpan {
            pixels: self.pixels,
            extent: self.extent,
        }
    }
}

/// A row's slice, narrowed to the part of it a text occupies.
///
/// Split out from [`LaneHost::text_extent`], which is the half that has to ask a
/// live editor where its text is; this is what it does with the answer, and the
/// half worth testing.
///
/// A text taller than the row holding it means the row has not settled yet.
/// Claiming more of the strip than the row owns would put its marks on top of its
/// neighbour's, so both ends clamp inside the slice.
fn narrow(extent: LaneExtent, row_height: f32, text_top: f32, text_height: f32) -> LaneExtent {
    let scale = (text_height / row_height).min(1.0);
    let offset = (text_top / row_height).min(1.0 - scale).max(0.0);
    LaneExtent {
        offset: extent.offset + extent.scale * offset,
        scale: extent.scale * scale,
    }
}

/// Where a **pixel** position in the scroll content falls on the map.
///
/// Rows are in document order, so this walks until it finds the one containing
/// `at` and interpolates inside it. A position above the first row or below the
/// last clamps to the ends rather than extrapolating, since there is no content
/// out there to be a fraction of.
fn map_fraction_at(rows: &[RowSpan], at: f32) -> f32 {
    let Some(first) = rows.first() else {
        return 0.0;
    };
    if at <= first.pixels.0 {
        return first.extent.offset;
    }
    for span in rows {
        let (top, height) = span.pixels;
        if height > 0.0 && at < top + height {
            return span.extent.place(((at - top) / height).clamp(0.0, 1.0));
        }
    }
    rows.last()
        .map(|s| s.extent.offset + s.extent.scale)
        .unwrap_or(1.0)
}

/// The inverse: where a fraction **of the map** is, in scroll content pixels.
///
/// What a click on the lane means. Going through the same rows in the same order
/// is what makes jumping land on the thing the writer aimed at, rather than at the
/// same fraction of a differently-shaped space.
fn pixel_at_fraction(rows: &[RowSpan], fraction: f32) -> f32 {
    let Some(first) = rows.first() else {
        return 0.0;
    };
    if fraction <= first.extent.offset {
        return first.pixels.0;
    }
    for span in rows {
        let LaneExtent { offset, scale } = span.extent;
        if fraction < offset + scale {
            let local = if scale > 0.0 {
                ((fraction - offset) / scale).clamp(0.0, 1.0)
            } else {
                0.0
            };
            return span.pixels.0 + local * span.pixels.1;
        }
    }
    rows.last().map(|s| s.pixels.0 + s.pixels.1).unwrap_or(0.0)
}

/// A row's weight from its character count, kept separate from the document so
/// the rule is testable without building one.
///
/// The floor is the rule: a scene with no prose still holds a hairline of the
/// strip. It has no reading length and deserves none, but it can carry a comment,
/// and a mark with nowhere to land is worse than a mark a pixel out of place.
fn row_weight_from_chars(chars: usize) -> f32 {
    chars.max(1) as f32
}

/// Build the lane for a single-document surface, beside `area`.
///
/// Always returns a widget, never an `Option`: whether the lane is *shown* is a
/// live preference, and a caller that decided it at construction time would leave
/// the writer flipping the View menu switch with nothing happening until the tab
/// was rebuilt for some other reason. The host reads the switches in its own
/// `build` and binds them, so the column appears and disappears where the writer
/// expects it to.
pub fn lane_for(area: &ScrollArea, inputs: LaneInputs) -> impl Widget {
    LaneHost {
        scroll: area.scroll_y_signal().clone(),
        max_scroll: area.max_scroll_y_signal().clone(),
        viewport_top: Signal::new(0.0),
        viewport_span: Signal::new(1.0),
        map: Rc::new(RefCell::new(Vec::new())),
        weights: RefCell::new(HashMap::new()),
        row_units: RefCell::new(HashMap::new()),
        caret_pulse: Signal::new(0),
        watched: RefCell::new(HashMap::new()),
        child: None,
        marks: Signal::new(Vec::new()),
        bars: Signal::new(Vec::new()),
        caret: Signal::new(None),
        texture_on: Cell::new(false),
        store: RefCell::new(None),
        colors: RefCell::new(None),
        inputs,
        last_inputs: Cell::new(0),
        last_output: Cell::new(0),
        texture_scale: Cell::new(0),
    }
}

/// The wrapper that resolves a lane's marks against live geometry.
struct LaneHost {
    scroll: Signal<f32>,
    max_scroll: Signal<f32>,
    /// Where the viewport's top and bottom sit **on the map**, which is not where
    /// they sit in the scroll area's pixels. See [`MappedRow`].
    viewport_top: Signal<f32>,
    viewport_span: Signal<f32>,
    /// The last pass's mapping, for the jump handler.
    ///
    /// Behind an `Rc` because `on_jump` is built once, in `build`, and has to see
    /// what the newest `place_children` learned. A closure capturing the rows by
    /// value would go on answering with the shape the book had when the lane was
    /// mounted.
    map: Rc<RefCell<Vec<RowSpan>>>,
    /// Each row's weight, against the document revision it was counted at.
    weights: RefCell<HashMap<EntityId, (u64, f32)>>,
    /// Each row's paragraphs in its own coordinates, against the text revision and
    /// laid-out height they were measured at. See [`LaneHost::row_units`].
    #[allow(clippy::type_complexity)]
    row_units: RefCell<HashMap<EntityId, ((u64, u32), Rc<Vec<texture::Paragraph>>)>>,
    /// Bumped by the cursor and focus watches, and bound at `Relayout` so the
    /// next pass re-reads the caret. A counter rather than the answer itself,
    /// because the watches fire where the answer cannot safely be computed -- see
    /// [`LaneHost::watch_carets`].
    caret_pulse: Signal<u64>,
    /// The live cursor and focus watches, one entry per mapped row. Dropping a
    /// handle detaches its observer, so pruning this is what stops a torn-down
    /// row's editor from going on driving the rule.
    watched: RefCell<HashMap<EntityId, Vec<teksilo::core::signal::ObserverHandle>>>,
    child: Option<WidgetId>,
    marks: Signal<Vec<LaneMark>>,
    bars: Signal<Vec<LaneBar>>,
    caret: Signal<Option<f32>>,
    texture_on: Cell<bool>,
    /// Read in `build`, where a `BuildContext` exists. The panes that mount a lane
    /// are plain `fn(&ContentTab) -> impl Widget` and have none.
    store: RefCell<Option<teksilo::settings::SettingsStore>>,
    colors: RefCell<Option<teksilo::tokens::ColorTokens>>,
    inputs: LaneInputs,
    /// Fingerprint of everything the marks are derived *from*.
    last_inputs: Cell<u64>,
    /// Fingerprint of the marks themselves.
    last_output: Cell<u64>,
    /// The longest paragraph the texture has ever scaled against, in words.
    ///
    /// Held here rather than derived per pass, and monotone — see
    /// [`texture::bars_from`] for the reported bug that made it so.
    texture_scale: Cell<usize>,
}

impl std::fmt::Debug for LaneHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LaneHost").finish_non_exhaustive()
    }
}

impl LaneHost {
    /// Has anything the marks depend on moved since the last pass?
    ///
    /// Every term is O(1). A document revision per row, the arbiter's generation,
    /// each comment set's generation, the row extents' generation (which is what a
    /// reflow moves), the text's own height where there is only one row, and the
    /// lane's height. Deliberately **not** the marks: deciding whether to recompute
    /// by recomputing would defeat the whole guard.
    /// Has anything the marks depend on moved since the last pass?
    ///
    /// Every term is O(1): a document revision per row, the arbiter's generation,
    /// each comment set's generation, the row extents' generation (which is what a
    /// reflow moves), and **each row's laid-out text height**.
    ///
    /// That last one is not redundant with the extents, and leaving it out was a real
    /// bug rather than a hypothetical. On the frame a page is first laid out there is
    /// no text geometry yet, so every mark resolves to nothing; on the next frame the
    /// geometry exists but every other term is identical, the guard fires, and the
    /// lane stays empty **for the life of the tab**. The height is the term that
    /// distinguishes "not laid out" from "laid out", so it is what lets the second
    /// frame do the work the first could not.
    ///
    /// Deliberately **not** the marks themselves: deciding whether to recompute by
    /// recomputing would defeat the whole guard.
    fn input_fingerprint(&self, rows: &[MappedRow], height: f32) -> u64 {
        let mut h = Fnv::new();
        h.add(query::active_query().generation());
        h.add(height.to_bits() as u64);
        h.add(self.inputs.rows.extents().generation().get());
        for MappedRow { row, .. } in rows {
            h.add(row.item);
            h.add(row.doc.content_revision());
            h.add(
                row.comments
                    .as_ref()
                    .map_or(0, |c| c.view_model().model().structure_signal().get()),
            );
            h.add(
                self.inputs
                    .format
                    .handle_for_item(row.item, self.inputs.kind)
                    .and_then(|handle| handle.content_height())
                    // A row with no laid-out text yet is distinguished from one whose
                    // text happens to be zero pixels tall, so the frame that gains
                    // geometry is never mistaken for the frame before it.
                    .map_or(u64::MAX, |px| px.to_bits() as u64),
            );
        }
        h.finish()
    }

    fn recompute(&self, bounds: Rect) {
        // **A lane that is not drawn does no work.** `place_children` runs even for a
        // widget with no children, so without this the writer's own switch would
        // stop the painting and leave every keystroke still resolving marks and
        // walking documents for a strip that is not on screen. `build` is where the
        // switches are read; this is the one flag that carries their answer out.
        if self.child.is_none() {
            return;
        }
        let store = self.store.borrow();
        let colors = self.colors.borrow();
        let (Some(store), Some(colors)) = (store.as_ref(), colors.as_ref()) else {
            return;
        };
        let texture_on = self.texture_on.get();
        if lane_debug() {
            eprintln!(
                "BOUNDS x={:.1} y={:.1} w={:.1} h={:.1} texture_on={texture_on} \
                 marks={} bars={} query={:?}",
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
                self.marks.get().len(),
                self.bars.get().len(),
                query::active_query().get().map(|q| q.text),
            );
        }
        let started = lane_debug().then(std::time::Instant::now);
        let rows = self.resolve(bounds.y);

        // **The viewport box, every pass and outside the guard.** Scrolling changes
        // no document, no extent and no mark, so the fingerprint below is
        // bit-identical for it -- and the box would sit still while the writer
        // scrolled past it. Two interpolations over a row list that is already in
        // hand, and its own `set_if_changed`.
        self.publish_viewport(&rows, bounds.height);

        // **The caret first, and outside the guard.** Moving the caret changes no
        // document revision, no comment set, no extent and no height, so the
        // fingerprint below is bit-identical for it — and a writer clicking into
        // another scene or arrowing through a paragraph would watch the lane's
        // caret rule sit where it was until their next keystroke. It is two O(1)
        // reads and its own `set_if_changed`, so paying for it every pass costs
        // nothing and asking the guard about it would cost a document walk.
        self.publish_caret(&rows);

        let fingerprint = self.input_fingerprint(&rows, bounds.height);
        if fingerprint == self.last_inputs.get() {
            if let Some(started) = started {
                eprintln!("LANE guard=hit us={}", started.elapsed().as_micros());
            }
            return;
        }
        self.last_inputs.set(fingerprint);

        let mut marks = Vec::new();
        let mut units = Vec::new();
        let (mut marks_us, mut units_us) = (0u128, 0u128);

        for MappedRow { row, text, .. } in &rows {
            // A row whose editor has not been built is simply skipped. That is the
            // normal state of everything below the fold in a long Book, and it is
            // why a lane must never substitute a zero position: a mark at fraction 0
            // is not "unknown", it is "the top of the manuscript", stated with total
            // confidence.
            let Some(handle) = self
                .inputs
                .format
                .handle_for_item(row.item, self.inputs.kind)
            else {
                continue;
            };
            // The prose's slice, not the row's: the marks belong to the text, and
            // the furniture above it is a gap they must not spill into.
            let extent = *text;

            let locate = locate::locator(handle.clone(), extent);
            let anchors = comment_anchors(row.comments.as_ref());
            let call = LaneCall {
                app_ctx: &self.inputs.app_ctx,
                ids: &self.inputs.ids,
                surface: self.inputs.surface,
                doc: &row.doc,
                item_id: row.item,
                comment_anchors: &anchors,
                locate: &locate,
            };
            let t_marks = lane_debug().then(std::time::Instant::now);
            marks.extend(resolve::marks(
                store,
                colors,
                self.inputs.surface,
                |spec, color, group| call.run(spec, color, group),
            ));
            if let Some(t) = t_marks {
                marks_us += t.elapsed().as_micros();
            }
            let t_units = lane_debug().then(std::time::Instant::now);
            if texture_on {
                // Remapped, not re-walked. The cache holds each paragraph in the
                // row's own `0.0..=1.0`, and `place` is linear, so applying the
                // extent afterwards gives exactly what locating against it would
                // have -- for two multiplications per paragraph instead of a
                // document walk and a layout query.
                units.extend(
                    self.row_units(row, &handle)
                        .iter()
                        .map(|p| texture::Paragraph {
                            span: LaneSpan::new(
                                extent.place(p.span.start),
                                extent.place(p.span.end),
                            ),
                            stats: p.stats,
                        }),
                );
            }
            if let Some(t) = t_units {
                units_us += t.elapsed().as_micros();
            }
        }

        // Scaled **once, across every row**. A bar's length is a fraction of the
        // longest paragraph, and in a Full Book "longest" has to mean the longest in
        // the book: normalising per scene would draw a two-paragraph note's bars the
        // same length as a chapter's, and the column would stop meaning anything
        // down the page.
        let units_len = units.len();
        let bars = if texture_on {
            texture::bars_from(units, bounds.height, &self.texture_scale)
        } else {
            Vec::new()
        };

        if lane_debug() {
            let with_handle = rows
                .iter()
                .filter(|MappedRow { row, .. }| {
                    self.inputs
                        .format
                        .handle_for_item(row.item, self.inputs.kind)
                        .is_some()
                })
                .count();
            eprintln!(
                "LANE guard=miss us={} marks_us={marks_us} units_us={units_us} rows={} handles={} placed={} max_scroll={:.1} \
                 units={} bars={} scale={}",
                started.map_or(0, |t| t.elapsed().as_micros()),
                rows.len(),
                with_handle,
                self.inputs.rows.extents().placed(),
                self.max_scroll.get(),
                units_len,
                bars.len(),
                self.texture_scale.get(),
            );
        }

        // Guarded again on the way out. A recompute that lands on the same marks —
        // a keystroke inside a paragraph that moved nothing — must not dirty the
        // lane, or every character typed repaints the strip.
        let out = output_fingerprint(&marks, &bars);
        if out == self.last_output.get() {
            return;
        }
        self.last_output.set(out);
        self.marks.set(marks);
        if texture_on {
            self.bars.set(bars);
        }
    }

    /// Where the writer's caret is, as a fraction of the lane.
    ///
    /// The row the writer is actually **in**: a stream has as many carets as it has
    /// editors, and only the focused one is theirs. `None` when the focus is
    /// anywhere else on the page, which is honest — the lane then shows no caret
    /// rule rather than the last place one happened to be.
    /// The rows to map this pass, each with where it sits on the lane.
    ///
    /// **A row's slice is its share of the manuscript's characters, not its share of
    /// the scroll area's pixels**, and that is the single decision this whole
    /// module exists to get right.
    ///
    /// Mapping pixels is the obvious implementation and it cannot be made to work.
    /// A row's pixel top is the sum of the heights of every row above it, and the
    /// scroll content's height is the sum of all of them; in a stream, the rows a
    /// writer has not looked at yet report *estimated* heights, so both of those
    /// sums keep being revised as they read. Measured on a real Book, `total` fell
    /// from 358 774 to 190 534 in the first seconds after opening and was still
    /// drifting by 1 293 px between consecutive frames. Every mark on the strip is
    /// placed as a fraction of that number, so every mark moved — including in
    /// chapters far from anything that had changed, which is what made this so hard
    /// to read from the screen. Estimating harder does not fix it: an estimate is
    /// revised by definition, and the map is a fixed thing the viewport moves over.
    ///
    /// Characters are exact, known without laying anything out, and they do not
    /// move. They are not free -- see [`weight_of`](Self::weight_of), which is why
    /// they are counted once per revision rather than once per frame. They are also the more honest
    /// quantity: the lane maps the manuscript, and a scene's share of a book is how
    /// much of the book it is, not how tall it happened to render.
    ///
    /// Within a row the conversion still goes through the editor's own geometry —
    /// see [`locate`](super::locate), whose reasoning against character fractions
    /// applies *inside* a document, where a heading or a scene break takes height
    /// without taking characters. Across whole scenes of prose that distortion
    /// averages out, and stability is worth incomparably more than it.
    fn resolve(&self, origin: f32) -> Vec<MappedRow> {
        // Ordering still comes from the placed tops, and that is sound where the
        // heights are not: a row laid out from a wrong estimate is still laid out
        // *below* the one before it, so document order survives every revision.
        let rows = self.inputs.rows.placed_rows();
        let extents = self.inputs.rows.extents();
        let scroll = self.scroll.get();
        let weights: Vec<f32> = rows.iter().map(|r| self.weight_of(r)).collect();
        let total: f32 = weights.iter().sum();
        let mut placed = Vec::with_capacity(rows.len());
        let mut cursor = 0.0;
        for (row, weight) in rows.into_iter().zip(weights) {
            // The cursor advances whether or not the row is kept, so a row that
            // cannot be sliced leaves a *gap* rather than shifting everything below
            // it up onto someone else's ground.
            if let Some(extent) = LaneExtent::slice(cursor, weight, total) {
                let pixels = extents
                    .placement(row.item)
                    .map(|(top, height)| (top - origin, height))
                    .unwrap_or((0.0, 0.0));
                let text = self.text_extent(&row, extent, pixels, origin, scroll);
                placed.push(MappedRow {
                    row,
                    extent,
                    text,
                    pixels,
                });
            }
            cursor += weight;
        }
        // A row the writer has closed stops being remembered, so a long session
        // does not accumulate weights for scenes that are gone.
        let live: HashSet<EntityId> = placed.iter().map(|m| m.row.item).collect();
        self.weights
            .borrow_mut()
            .retain(|item, _| live.contains(item));
        placed
    }

    /// Narrow a row's slice to the part of it the prose actually occupies.
    ///
    /// The composition is affine, which is why this is a change to the *extent* and
    /// not to [`locate`](super::locate). A mark at fraction `y` of the text lands at
    ///
    /// ```text
    /// extent.place((text_top + y * text_height) / row_height)
    /// ```
    ///
    /// and that is `place` against an extent whose offset has moved by
    /// `text_top / row_height` of the slice and whose scale has shrunk by
    /// `text_height / row_height`. So every cached local fraction stays valid and
    /// nothing has to be measured again.
    ///
    /// Falls back to the whole slice when the row has not been laid out or the
    /// editor has no text geometry yet. That is the old behaviour, and it is the
    /// right fallback: an unknown gap is better left closed than guessed at.
    fn text_extent(
        &self,
        row: &LaneRow,
        extent: LaneExtent,
        pixels: (f32, f32),
        origin: f32,
        scroll: f32,
    ) -> LaneExtent {
        let (row_top, row_height) = pixels;
        let Some(handle) = self
            .inputs
            .format
            .handle_for_item(row.item, self.inputs.kind)
        else {
            return extent;
        };
        let (Some(text_height), Some(first)) = (handle.content_height(), handle.range_rect(0, 0))
        else {
            return extent;
        };
        if row_height <= 0.0 || text_height <= 0.0 {
            return extent;
        }
        // `range_rect` is window space and the row's top is content space, so the
        // scroll offset and the viewport's own origin are what carry one into the
        // other. `RowExtent` reports `window_y + scroll`, which is why this is the
        // exact inverse of what it did.
        let row_window_top = row_top + origin - scroll;
        let text_top = (first.y - row_window_top).max(0.0);
        let out = narrow(extent, row_height, text_top, text_height);
        if lane_debug() {
            eprintln!(
                "TEXT item={} row_top={row_top:.1} row_h={row_height:.1} first_y={:.1} \
                 row_win_top={row_window_top:.1} text_top={text_top:.1} text_h={text_height:.1} \
                 -> ({:.4},{:.4}) from ({:.4},{:.4})",
                row.item, first.y, out.offset, out.scale, extent.offset, extent.scale,
            );
        }
        out
    }

    /// One row's measured paragraphs, in **its own** `0.0..=1.0`, remembered until
    /// either its text or its layout changes.
    ///
    /// This is where the lane's time went. Measured on a Book of 33 scenes and 2264
    /// paragraphs, a full recompute cost 189 ms, of which the marks were 2.6 ms and
    /// this walk was **185 ms** -- 98% of it. A keystroke changes one scene, and the
    /// other 32 were being re-walked and re-queried to produce the identical answer.
    ///
    /// The key is the pair that can invalidate it. `content_revision` covers the
    /// text; `content_height` covers the layout, because a row that reflows at a new
    /// width moves every paragraph inside it without changing a character. Keying on
    /// the revision alone would have left the bars wrong after a window resize,
    /// which is the sort of staleness that looks like a rendering bug for weeks.
    ///
    /// The height is a proxy for the width, and one case slips through it: a reflow
    /// where one paragraph gains a line and another loses one, leaving the row's
    /// total height bit-identical. Those bars stay as they were until the next edit
    /// to that scene. It needs the two changes to cancel to the bit, it moves a bar
    /// by one line, and it heals itself -- worth knowing about, not worth threading
    /// the laid-out width through the editor handle to prevent.
    ///
    /// Deliberately **not** keyed on the extent. Local coordinates do not depend on
    /// it, which is the whole reason they are stored that way: typing one character
    /// changes the total character count and so nudges every row's extent, and a
    /// cache keyed on that would miss on all 33 rows for every keystroke and buy
    /// nothing at all.
    fn row_units(
        &self,
        row: &LaneRow,
        handle: &teksilo::widgets::rich_text::EditorHandle,
    ) -> Rc<Vec<texture::Paragraph>> {
        let key = (
            row.doc.content_revision(),
            handle.content_height().unwrap_or(0.0).to_bits(),
        );
        if let Some((seen, cached)) = self.row_units.borrow().get(&row.item)
            && *seen == key
        {
            return cached.clone();
        }
        let span_of =
            |start: usize, end: usize| locate::locate_span(handle, LaneExtent::WHOLE, start, end);
        let measured = Rc::new(texture::units(&row.doc, row.markers, &span_of));
        self.row_units
            .borrow_mut()
            .insert(row.item, (key, measured.clone()));
        measured
    }

    /// One row's weight, remembered until its text changes.
    ///
    /// **Not** a micro-optimisation. `TextDocument::character_count` reads a field
    /// that is genuinely cached, but the only route to it runs
    /// `get_document_stats`, which opens a transaction, fetches every block, and
    /// materialises each one's text to whitespace-split a word count nobody here
    /// asked for. text-document's own `streaming.rs` records this cost against
    /// itself. Calling it per row per pass -- and `recompute` runs on every dirty
    /// frame, before the fingerprint guard -- is a full scan of the whole book on
    /// every keystroke and every scroll tick. That is exactly the cost this
    /// module's guards exist to avoid, and it was introduced here by a commit whose
    /// own doc comment called the count free. It is not free; it is cheap **once
    /// per revision**, which is what this makes it.
    ///
    /// `content_revision` is a plain field read, so the key costs nothing.
    fn weight_of(&self, row: &LaneRow) -> f32 {
        let revision = row.doc.content_revision();
        if let Some((seen, weight)) = self.weights.borrow().get(&row.item).copied()
            && seen == revision
        {
            return weight;
        }
        let weight = row_weight_from_chars(row.doc.character_count());
        self.weights
            .borrow_mut()
            .insert(row.item, (revision, weight));
        weight
    }

    /// Where the viewport sits **on the map**, and the mapping a jump needs.
    ///
    /// Both ends are converted separately rather than the top plus a ratio: a
    /// viewport spanning a densely-written scene and a sparse one covers different
    /// amounts of the map at each end, and a single ratio cannot say that.
    ///
    /// A span of `1.0` -- everything visible, or nothing mapped yet -- is what tells
    /// the widget not to draw a box at all.
    fn publish_viewport(&self, rows: &[MappedRow], height: f32) {
        let spans: Vec<RowSpan> = rows.iter().map(MappedRow::span).collect();
        let (top, span) = if spans.is_empty() {
            (0.0, 1.0)
        } else {
            let at = self.scroll.get();
            let top = map_fraction_at(&spans, at);
            (
                top,
                (map_fraction_at(&spans, at + height) - top).clamp(0.0, 1.0),
            )
        };
        if lane_debug() {
            let at = self.scroll.get();
            let here = spans
                .iter()
                .position(|s| at >= s.pixels.0 && at < s.pixels.0 + s.pixels.1);
            eprintln!(
                "VIEW scroll={:.1} h={:.1} row_at_top={:?} top={:.5} span={:.5}",
                at, height, here, top, span,
            );
        }
        self.map.replace(spans);
        let _ = self.viewport_top.set_if_changed(top);
        let _ = self.viewport_span.set_if_changed(span);
    }

    /// Publish the caret, and make sure the lane will be asked again when the
    /// writer moves it.
    ///
    /// **The watch is the point.** `recompute` runs from `place_children`, so it
    /// only runs when something has asked for a layout -- and moving the caret asks
    /// for none: no document changes, no row reflows, no extent moves. The rule
    /// therefore sat where it was until the writer happened to scroll or type,
    /// which is exactly what was reported.
    fn publish_caret(&self, rows: &[MappedRow]) {
        self.watch_carets(rows);
        let at = rows.iter().find_map(|MappedRow { row, text, .. }| {
            let handle = self
                .inputs
                .format
                .handle_for_item(row.item, self.inputs.kind)?;
            if !handle.focused_signal().get() {
                return None;
            }
            let offset = handle.cursor_position();
            let at = locate::locate_offset(&handle, *text, offset);
            if lane_debug() {
                eprintln!(
                    "CARET item={} offset={offset} rect_y={:?} text_h={:?} \
                     extent=({:.5},{:.5}) -> at={at:?}",
                    row.item,
                    handle.offset_content_rect(offset).map(|r| r.y),
                    handle.content_height(),
                    text.offset,
                    text.scale,
                );
            }
            at
        });
        let _ = self.caret.set_if_changed(at);
    }

    /// Watch each row's editor for the two things that move the rule: the cursor,
    /// and which editor has the focus.
    ///
    /// Focus matters as much as position. Clicking from one scene into another can
    /// leave both carets exactly where they were, and only the focus says which of
    /// them the rule is now about.
    ///
    /// ⚠ **An observer here must not touch the editor it is watching.** It fires
    /// from inside the editor's own mutation, which is holding that editor's state
    /// `RefCell` mutably; every query the caret needs -- `focused_signal`,
    /// `cursor_position`, `offset_content_rect`, `content_height` -- borrows it
    /// again. Reading them here panicked the application with "RefCell already
    /// mutably borrowed" on the first arrow key.
    ///
    /// So the observer does the one thing that is safe from inside a mutation:
    /// it moves a counter. The counter is bound at `Relayout`, so the read happens
    /// in the next layout pass, from `recompute`, where nothing is part-way through
    /// changing.
    fn watch_carets(&self, rows: &[MappedRow]) {
        let live: HashSet<EntityId> = rows.iter().map(|m| m.row.item).collect();
        self.watched
            .borrow_mut()
            .retain(|item, _| live.contains(item));

        for MappedRow { row, .. } in rows {
            if self.watched.borrow().contains_key(&row.item) {
                continue;
            }
            let Some(handle) = self
                .inputs
                .format
                .handle_for_item(row.item, self.inputs.kind)
            else {
                continue;
            };
            let pulse = self.caret_pulse.clone();
            let bump = move || {
                let n = pulse.get();
                pulse.set(n.wrapping_add(1));
            };
            let on_focus = {
                let bump = bump.clone();
                handle.focused_signal().observe(move |_| bump())
            };
            let on_move = handle.cursor_position_signal().observe(move |_| bump());
            self.watched
                .borrow_mut()
                .insert(row.item, vec![on_move, on_focus]);
        }
    }
}

impl Widget for LaneHost {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // `try_settings`, not `settings`: the latter panics, and the widget tests
        // build these panes with no application around them — the same reason every
        // other door into a pane (`comments`, `footnotes`, `images`) is an `Option`.
        // No store means no preference to read, and a lane whose switches cannot be
        // consulted must not assume they are on.
        let Some(store) = ctx.try_settings().cloned() else {
            self.child = None;
            return Vec::new();
        };
        let store = &store;
        let self_id = ctx.self_id();
        let registry = ctx.binding_registry();

        // Every switch that decides whether this column exists is bound at
        // `Rebuild`, so flipping the View menu's toggle — or a provider's row on the
        // settings page — puts the lane on screen or takes it off, rather than
        // waiting for the tab to be rebuilt for some unrelated reason.
        let enabled = store.signal(
            crate::MARGIN_LANE_ENABLED_KEY,
            crate::MARGIN_LANE_ENABLED_DEFAULT,
        );
        let on_surface = store.signal(
            &crate::margin_lane_surface_key(self.inputs.surface),
            crate::margin_lane_surface_default(self.inputs.surface),
        );
        let texture = store.signal(
            crate::MARGIN_LANE_TEXTURE_KEY,
            crate::MARGIN_LANE_TEXTURE_DEFAULT,
        );
        enabled.bind_to(self_id, registry, BindingLevel::Rebuild);
        on_surface.bind_to(self_id, registry, BindingLevel::Rebuild);
        texture.bind_to(self_id, registry, BindingLevel::Rebuild);
        for spec in super::registered_for(self.inputs.surface) {
            store.signal(&spec.settings_key(), spec.default_on).bind_to(
                self_id,
                registry,
                BindingLevel::Rebuild,
            );
        }

        // **What makes the strip re-derive.** Three bindings above `RepaintOnly`, and
        // each covers a case the others miss.
        //
        // Getting this wrong is not a stale mark, it is a permanently empty lane:
        // the marks are resolved in this widget's own layout pass, and a widget
        // nothing dirties is never laid out again. On the first frame the editors
        // have no text geometry yet and every mark resolves to nothing, so a lane
        // that is never asked a second time shows nothing for the life of the tab.
        //
        // The scroll range moves whenever the mapped content's height does, which
        // includes the first frame the text is laid out at all.
        self.max_scroll
            .bind_to(self_id, registry, BindingLevel::Relayout);

        // …but not when an edit leaves the height alone, which is most edits and
        // exactly the ones that shift the offsets a mark is anchored to. The
        // document's own version counter is what carries those. Read through the
        // registry, which the editors have already written to by the time a sibling
        // lane builds.
        for row in self.inputs.rows.placed_rows() {
            if let Some(handle) = self
                .inputs
                .format
                .handle_for_item(row.item, self.inputs.kind)
            {
                handle
                    .document_version()
                    .bind_to(self_id, registry, BindingLevel::Relayout);
            }
        }

        // **A stream's row extents, at `Relayout`.** The third, and it needs its
        // reason recorded separately.
        //
        // The rows are placed deep inside the scroll area; this host is their
        // *sibling*, and a parent places its children before their own subtrees are
        // laid out. So on the pass where a reflow happens, the extents this host
        // reads are last frame's. Asking for another layout is what closes that gap.
        //
        // It terminates because [`RowExtents::report`] is guarded: the extra pass
        // re-places the rows, they report what they already reported, the generation
        // does not move, and nothing asks for a third. The fixed point is the guard,
        // not the binding level, which is why removing the guard would turn this
        // into a live loop rather than merely a waste.
        self.inputs
            .rows
            .extents()
            .generation()
            .bind_to(self_id, registry, BindingLevel::Relayout);

        // **The caret's own wake-up.** Bound here rather than on each row's cursor
        // signal, for two reasons. It exists before any row does, so it works on
        // the first build, when `placed_rows` is still empty because layout has not
        // run; and it is a plain counter the watches can move from inside an
        // editor's mutation, where reading that editor would panic. See
        // [`LaneHost::watch_carets`].
        self.caret_pulse
            .bind_to(self_id, registry, BindingLevel::Relayout);

        self.colors.replace(Some(ctx.theme().colors.clone()));
        self.texture_on.set(texture.get());
        self.store.replace(Some(store.clone()));

        if !(enabled.get() && on_surface.get()) {
            self.child = None;
            return Vec::new();
        }

        let mut lane = MarginLane::new(
            self.marks.clone(),
            self.viewport_top.clone(),
            self.viewport_span.clone(),
        )
        .caret(self.caret.clone())
        .access_label(super::default_lane_label())
        .on_jump({
            // Through the same mapping, in reverse. Multiplying the fraction by
            // `max_scroll` would land at that fraction of the *pixels*, which is a
            // different place in the book from the mark the writer clicked on --
            // by a whole chapter where the scenes are unevenly dense.
            //
            // The fraction is where the viewport's TOP should go, not where the
            // pointer was: the lane owns the box and has already decided what a
            // click or a drag of it means. All that is owed here is the mapping,
            // which is the one thing the lane cannot do.
            let scroll = self.scroll.clone();
            let max_scroll = self.max_scroll.clone();
            let map = self.map.clone();
            move |fraction| {
                let rows = map.borrow();
                let at = if rows.is_empty() {
                    // Nothing mapped yet: the proportional answer is the only one
                    // available, and it is what a uniform scroll area would give.
                    fraction * max_scroll.get()
                } else {
                    pixel_at_fraction(&rows, fraction)
                };
                scroll.set(at.clamp(0.0, max_scroll.get()));
            }
        });
        if self.texture_on.get() {
            // `texture` is the switch: it takes the column's default width with it.
            lane = lane.texture(self.bars.clone());
        }

        // A rebuild resolves against a fresh editor and a fresh set of switches, so
        // the guards must not remember the previous one's answers.
        self.last_inputs.set(0);
        self.last_output.set(0);

        let id = ctx.add(lane);
        self.child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child
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
            child.origin = bounds.origin();
            child.size = teksilo::canvas::Size::new(bounds.width, bounds.height);
        }
        self.recompute(bounds);
    }

    fn paint(&self, _bounds: Rect, _canvas: &mut teksilo::canvas::Canvas, _ctx: &PaintContext) {}
}

/// The live anchors, flattened to what a provider is allowed to see.
///
/// The **live** ones, not the comment rows': a row's stored `range_start` is only
/// rewritten when some editor's comment margin rebuilds, so a mark using it would
/// drift a paragraph at a time while the writer typed above it.
fn comment_anchors(
    binding: Option<&crate::comments::binding::CommentBinding>,
) -> Vec<CommentAnchor> {
    let Some(binding) = binding else {
        return Vec::new();
    };
    if !binding.view_model().is_visible() {
        return Vec::new();
    }
    let rows = binding
        .view_model()
        .model()
        .rows_for_content(binding.content_id());
    binding
        .live()
        .into_iter()
        .filter(|a| !a.resolved && a.end > a.start)
        .filter_map(|a| {
            // The same predicate the margin uses: an anchor the session still lists
            // but the model no longer backs mounts no card, and must mark nothing.
            let row = rows.iter().find(|r| r.id == a.comment_id)?;
            Some(CommentAnchor {
                id: a.comment_id,
                start: a.start,
                end: a.end,
                label: crate::comments::dock::comment_snippet(row),
            })
        })
        .collect()
}

/// Whether `TEKSILO_LANE_DEBUG` asked for a line per recompute. Cached: this is
/// read from `place_children`, which runs on every dirty frame.
fn lane_debug() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("TEKSILO_LANE_DEBUG").is_some())
}

fn output_fingerprint(marks: &[LaneMark], bars: &[LaneBar]) -> u64 {
    let mut h = Fnv::new();
    for m in marks {
        h.add(m.id);
        h.add(u64::from(m.group));
        h.add(m.span.start.to_bits() as u64);
        h.add(m.span.end.to_bits() as u64);
        h.add(m.column as u64);
        h.add(m.shape as u64);
    }
    for b in bars {
        h.add(b.span.start.to_bits() as u64);
        h.add(b.span.end.to_bits() as u64);
        h.add(b.extent.to_bits() as u64);
        h.add(b.filled.to_bits() as u64);
    }
    h.finish()
}

/// FNV-1a over `u64`s. Named and stable, rather than `DefaultHasher`, whose output
/// std explicitly does not promise across releases — and a fingerprint that changed
/// meaning between builds would silently stop guarding.
struct Fnv(u64);

impl Fnv {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
    fn add(&mut self, v: u64) {
        for byte in v.to_le_bytes() {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    fn finish(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {

    /// **A writing page is not only its manuscript.** A Scene carries an epigraph
    /// and a synopsis above the prose; both scroll with it and both are inside the
    /// row. The marks belong to the text, so the strip has to open a gap for them.
    ///
    /// The numbers are a page 1000px tall whose prose starts 200px down and runs
    /// 800px: the marks may occupy the bottom four fifths of the slice and not one
    /// pixel above it.
    #[test]
    fn furniture_above_the_prose_leaves_a_gap_on_the_strip() {
        let text = narrow(LaneExtent::WHOLE, 1000.0, 200.0, 800.0);
        assert!((text.offset - 0.2).abs() < 1e-5, "offset {}", text.offset);
        assert!((text.scale - 0.8).abs() < 1e-5, "scale {}", text.scale);
        // The first paragraph of the prose, which used to draw at the very top.
        assert!((text.place(0.0) - 0.2).abs() < 1e-5);
        assert!((text.place(1.0) - 1.0).abs() < 1e-5);
    }

    /// The narrowing composes with the row's own slice, which is what lets a stream
    /// row keep its heading out of its neighbour's ground.
    #[test]
    fn the_gap_is_taken_out_of_the_rows_own_slice() {
        let row = LaneExtent {
            offset: 0.5,
            scale: 0.25,
        };
        let text = narrow(row, 400.0, 100.0, 300.0);
        assert!(
            (text.offset - 0.5625).abs() < 1e-5,
            "offset {}",
            text.offset
        );
        assert!((text.scale - 0.1875).abs() < 1e-5, "scale {}", text.scale);
        assert!(
            text.offset >= row.offset && text.offset + text.scale <= row.offset + row.scale + 1e-6,
            "the text stayed inside its row"
        );
    }

    /// A row whose text is reported taller than the row itself has not settled.
    /// Claiming more of the strip than the row owns would put its marks over its
    /// neighbour's, so the narrowing clamps instead.
    #[test]
    fn an_unsettled_row_cannot_claim_more_than_it_owns() {
        let row = LaneExtent {
            offset: 0.25,
            scale: 0.5,
        };
        let text = narrow(row, 100.0, 80.0, 900.0);
        assert!(text.scale <= row.scale + 1e-6);
        assert!(text.offset >= row.offset - 1e-6);
        assert!(text.offset + text.scale <= row.offset + row.scale + 1e-6);
    }

    use super::*;

    /// The two spaces, as a lane sees them: three rows of equal *content* whose
    /// rendered heights are wildly unequal, which is what a book of one dense scene
    /// between two airy ones looks like.
    fn uneven() -> Vec<RowSpan> {
        vec![
            RowSpan {
                pixels: (0.0, 600.0),
                extent: LaneExtent {
                    offset: 0.0,
                    scale: 1.0 / 3.0,
                },
            },
            RowSpan {
                pixels: (600.0, 200.0),
                extent: LaneExtent {
                    offset: 1.0 / 3.0,
                    scale: 1.0 / 3.0,
                },
            },
            RowSpan {
                pixels: (800.0, 600.0),
                extent: LaneExtent {
                    offset: 2.0 / 3.0,
                    scale: 1.0 / 3.0,
                },
            },
        ]
    }

    /// **A pixel position maps through the row it is in, not through the total.**
    ///
    /// The naive conversion — `at / content_height` — is what the viewport box used
    /// to do, and on this shape it is wrong by a sixth of the whole strip. Halfway
    /// down the *pixels* (700 of 1400) is the middle row, which is halfway through
    /// the *book*; the naive answer agrees here by luck. A quarter of the way down
    /// the pixels (350) is inside the first row, only 58% of the way through it, so
    /// it is 19% of the book — not 25%.
    #[test]
    fn a_pixel_position_maps_through_its_own_row() {
        let rows = uneven();
        assert!((map_fraction_at(&rows, 700.0) - 0.5).abs() < 1e-5);

        let quarter = map_fraction_at(&rows, 350.0);
        assert!(
            (quarter - 350.0 / 600.0 / 3.0).abs() < 1e-5,
            "inside the first row, at its own local fraction: {quarter}"
        );
        assert!(
            (quarter - 0.25).abs() > 0.05,
            "and that is materially not the naive pixel fraction"
        );
    }

    /// Above the first row and below the last, the ends clamp. There is no content
    /// out there to be a fraction of, and extrapolating would put a viewport box
    /// off the end of the strip.
    #[test]
    fn positions_outside_the_rows_clamp_to_the_ends() {
        let rows = uneven();
        assert!((map_fraction_at(&rows, -500.0) - 0.0).abs() < 1e-6);
        assert!((map_fraction_at(&rows, 0.0) - 0.0).abs() < 1e-6);
        assert!((map_fraction_at(&rows, 99_999.0) - 1.0).abs() < 1e-6);
        assert_eq!(
            map_fraction_at(&[], 42.0),
            0.0,
            "nothing mapped, no opinion"
        );
    }

    /// **A click on the lane lands where the writer aimed.** The inverse has to
    /// agree with the forward conversion, or jumping to a mark scrolls to somewhere
    /// near it and the writer has to hunt.
    #[test]
    fn a_jump_is_the_inverse_of_the_viewport_mapping() {
        let rows = uneven();
        for at in [0.0, 120.0, 599.0, 600.0, 700.0, 800.0, 1399.0] {
            let round_tripped = pixel_at_fraction(&rows, map_fraction_at(&rows, at));
            assert!(
                (round_tripped - at).abs() < 0.5,
                "{at}px mapped to the lane and back gave {round_tripped}px"
            );
        }
    }

    /// The jump clamps at both ends too, and says nothing when nothing is mapped.
    #[test]
    fn a_jump_outside_the_map_clamps() {
        let rows = uneven();
        assert!((pixel_at_fraction(&rows, -1.0) - 0.0).abs() < 1e-6);
        assert!((pixel_at_fraction(&rows, 2.0) - 1400.0).abs() < 1e-6);
        assert_eq!(pixel_at_fraction(&[], 0.5), 0.0);
    }

    /// **The map does not move when the pixels do**, which is the whole reason the
    /// row slices are made of characters. Every row's rendered height changes here —
    /// the first triples, the last halves, as estimates are replaced by real
    /// layouts — and not one mark may shift.
    #[test]
    fn revised_heights_move_the_viewport_and_not_the_map() {
        let before = uneven();
        let after: Vec<RowSpan> = vec![(0.0, 1800.0), (1800.0, 200.0), (2000.0, 300.0)]
            .into_iter()
            .zip(before.iter())
            .map(|(pixels, row)| RowSpan {
                pixels,
                extent: row.extent,
            })
            .collect();

        for (a, b) in before.iter().zip(&after) {
            assert_eq!(
                (a.extent.offset, a.extent.scale),
                (b.extent.offset, b.extent.scale),
                "a revised height changed a row's slice of the map"
            );
        }

        // The viewport box, by contrast, *must* move: the same scroll offset now
        // shows a different part of the book, and a box that stayed put would be
        // lying about what the writer can see.
        assert!(
            (map_fraction_at(&before, 700.0) - map_fraction_at(&after, 700.0)).abs() > 0.1,
            "the viewport box has to follow the pixels it is reporting on"
        );
    }

    /// A row with no prose still holds a hairline of the strip, so a comment on an
    /// empty scene has somewhere to land. Without the floor its weight is zero, its
    /// slice is `None`, and the mark vanishes.
    #[test]
    fn an_empty_row_still_has_somewhere_to_put_a_mark() {
        assert_eq!(row_weight_from_chars(0), 1.0);
        assert_eq!(row_weight_from_chars(4_000), 4_000.0);
    }
}
