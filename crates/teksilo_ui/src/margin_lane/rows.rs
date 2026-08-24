// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **Where each row of a stream actually ended up**, and how a lane hears about it
//! when that changes.
//!
//! ## The problem this exists for
//!
//! A Full Chapter, Part or Book is one scroll area over one column of many
//! documents. A lane over it has to know where each row starts and how tall it is,
//! and **nothing in the row-list layer knows**: `StreamRowsModel` subscribes to
//! structural events only, and ordinary typing touches none of them. It mutates the
//! row's own `TextDocument` through `OpenDoc`, an object graph that model never
//! watches. So `ListModel::observe_changes` never fires, `Repeater` never rebuilds
//! — and every row below the caret has just moved down the page.
//!
//! Nor is there a layout-completed hook or a bounds-changed signal anywhere in
//! teksilo-core to fall back on.
//!
//! ## What it does instead
//!
//! Exactly what [`ScrollArea`](teksilo::widgets::ScrollArea) does with its own
//! scroll metrics: publishes a layout-derived measurement into a signal, from the
//! layout pass, guarded so it only fires on a genuine change. Each row is wrapped in
//! a [`RowExtent`] that reports its placed top and height; the lane binds the
//! generation counter and re-derives.
//!
//! ⚠ **A consumer must bind the generation below `Relayout`.** `ScrollArea`'s own
//! source carries the warning, and it applies verbatim here: binding a
//! layout-published signal at `Relayout` on anything in the same subtree is an
//! instant layout loop.
//!
//! ## What the tops are for, and what they are **not** for
//!
//! Ordering, and the reflow signal. **Not** where a row sits on the lane: that is
//! its share of the manuscript's characters, decided in
//! [`LaneRows::resolve`](super::surface), for reasons recorded there at length. A
//! row's pixel top is a sum over estimated heights and is revised for as long as
//! the writer keeps reading, so a lane mapped from it never holds still. Sorting by
//! it is safe where measuring with it is not: an estimate can be wrong about how
//! tall a row is without being wrong about which row comes first.
//!
//! ## Why the tops are scroll-corrected
//!
//! `place_children` reports window-space bounds, which move on every scroll tick. A
//! lane fed those would re-derive sixty times a second while a writer scrolled, and
//! every mark would slide up the strip as the viewport moved — precisely backwards,
//! since the lane is the fixed map the viewport moves *over*. Adding the scroll
//! offset back turns them into positions in the scrolled content, which is what a
//! fraction of the whole extent has to be measured against.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use common::types::EntityId;
use teksilo::canvas::{Rect, Size};
use teksilo::core::widget::{LayoutContext, LayoutResponse, PaintContext, Widget, WidgetPlacement};
use teksilo::core::widget_id::WidgetId;
use teksilo::prelude::*;

/// Half a pixel: below this a row has not moved in any way a reader could see, and
/// re-deriving a hundred rows' marks for it would be pure cost.
const EPSILON: f32 = 0.5;

/// Where every mapped row of one stream sits, and a counter that moves when any of
/// them does.
#[derive(Clone)]
pub struct RowExtents {
    inner: Rc<Inner>,
}

struct Inner {
    rows: RefCell<HashMap<EntityId, (f32, f32)>>,
    generation: Signal<u64>,
}

impl Default for RowExtents {
    fn default() -> Self {
        Self {
            inner: Rc::new(Inner {
                rows: RefCell::new(HashMap::new()),
                generation: Signal::new(0),
            }),
        }
    }
}

impl std::fmt::Debug for RowExtents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RowExtents")
            .field("rows", &self.inner.rows.borrow().len())
            .finish()
    }
}

impl RowExtents {
    pub fn new() -> Self {
        Self::default()
    }

    /// Bumped whenever a row's placement actually changed. Bind it **below**
    /// `Relayout`.
    pub fn generation(&self) -> Signal<u64> {
        self.inner.generation.clone()
    }

    /// Record where a row landed, in the scrolled content's own coordinates.
    ///
    /// Guarded: a report identical to the last one for that row does not bump the
    /// counter. This is the whole reason the lane is not re-derived every frame,
    /// since layout runs on every frame that is dirty and typing dirties every one.
    pub fn report(&self, item: EntityId, top: f32, height: f32) {
        let changed = {
            let mut rows = self.inner.rows.borrow_mut();
            match rows.get(&item) {
                Some((t, h)) if (t - top).abs() < EPSILON && (h - height).abs() < EPSILON => false,
                _ => {
                    rows.insert(item, (top, height));
                    true
                }
            }
        };
        if changed {
            self.bump();
        }
    }

    /// Drop a row that has been torn down.
    ///
    /// A row scrolled far out of view is unbuilt by the `Repeater`, and an extent
    /// left behind for it would keep claiming a slice of the lane that something
    /// else now occupies.
    pub fn forget(&self, item: EntityId) {
        if self.inner.rows.borrow_mut().remove(&item).is_some() {
            self.bump();
        }
    }

    fn bump(&self) {
        let g = self.inner.generation.get();
        self.inner.generation.set(g.wrapping_add(1));
    }

    /// Where one row landed, in the scroll content's own pixels.
    ///
    /// ⚠ **Not where it goes on the lane.** This is the estimate-dependent number
    /// the module note warns about, and the only thing it may be used for is
    /// tracking what the writer can actually *see* -- the viewport box, and where a
    /// click on the strip should scroll to. Placing a mark with it is the bug this
    /// module exists to record.
    pub fn placement(&self, item: EntityId) -> Option<(f32, f32)> {
        self.inner.rows.borrow().get(&item).copied()
    }

    /// How many rows have been placed. For diagnostics and tests.
    pub fn placed(&self) -> usize {
        self.inner.rows.borrow().len()
    }

    /// The placed rows, **in document order**.
    ///
    /// This is the list a lane iterates, and using it rather than the stream's own
    /// row list is a correctness requirement, not an optimisation. Resolving a row's
    /// document goes through `StreamViewModel::row_doc`, which on a cache miss calls
    /// `OpenDocsStore::open` — a full synchronous Djot import, and the thing that
    /// once made switching a Book to Full Book freeze for seconds. A row that has
    /// been *placed* has been built, so its document is already open and the lookup
    /// is a cache hit. A row that has not is not asked about at all.
    pub fn items(&self) -> Vec<EntityId> {
        let mut rows: Vec<(EntityId, f32)> = self
            .inner
            .rows
            .borrow()
            .iter()
            .map(|(item, (top, _))| (*item, *top))
            .collect();
        rows.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        rows.into_iter().map(|(item, _)| item).collect()
    }
}

/// Wraps one stream row and reports where it landed.
///
/// Transparent: it takes its child's size and gives it the whole of its own bounds,
/// so inserting it changes no layout. The only thing it adds is the report.
pub struct RowExtent {
    item: EntityId,
    extents: RowExtents,
    /// The stream's scroll offset, added back so the reported top is a position in
    /// the content rather than on the screen. See the module note.
    scroll: Signal<f32>,
    child: Option<Box<dyn Widget>>,
    child_id: Option<WidgetId>,
}

impl std::fmt::Debug for RowExtent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RowExtent")
            .field("item", &self.item)
            .finish_non_exhaustive()
    }
}

impl RowExtent {
    pub fn new(
        item: EntityId,
        extents: RowExtents,
        scroll: Signal<f32>,
        child: impl Widget + 'static,
    ) -> Self {
        Self::boxed(item, extents, scroll, Box::new(child))
    }

    /// As [`new`](Self::new), for a child already behind a `Box` — the shape a
    /// branch that picks between two different widget types hands back.
    pub fn boxed(
        item: EntityId,
        extents: RowExtents,
        scroll: Signal<f32>,
        child: Box<dyn Widget>,
    ) -> Self {
        Self {
            item,
            extents,
            scroll,
            child: Some(child),
            child_id: None,
        }
    }
}

impl Drop for RowExtent {
    fn drop(&mut self) {
        // The row's own lifetime is its extent's lifetime, which is the same
        // discipline the formatting registry keeps for editor handles — and for the
        // same reason: what is left behind after a teardown is what a lane would go
        // on drawing against.
        self.extents.forget(self.item);
    }
}

impl Widget for RowExtent {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let Some(child) = self.child.take() else {
            return Vec::new();
        };
        let id = ctx.add_boxed(child);
        self.child_id = Some(id);
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
            child.origin = bounds.origin();
            child.size = Size::new(bounds.width, bounds.height);
        }
        self.extents
            .report(self.item, bounds.y + self.scroll.get(), bounds.height);
    }

    fn paint(&self, _bounds: Rect, _canvas: &mut teksilo::canvas::Canvas, _ctx: &PaintContext) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Rows come back in document order however their heights were estimated.
    /// This is all the ordering the lane takes from the placement, and it has to
    /// survive a height being wrong, because early in a stream's life every height
    /// below the fold is.
    #[test]
    fn rows_come_back_in_document_order() {
        let e = RowExtents::new();
        e.report(3, 800.0, 200.0);
        e.report(1, 0.0, 200.0);
        e.report(2, 200.0, 600.0);
        assert_eq!(e.items(), vec![1, 2, 3]);

        // The middle row's estimate is revised to a third of what it was. Nothing
        // about the order may move.
        e.report(2, 200.0, 200.0);
        e.report(3, 400.0, 200.0);
        assert_eq!(e.items(), vec![1, 2, 3]);
    }

    /// **The guard that stops the lane re-deriving sixty times a second.** Layout
    /// runs on every dirty frame, and while a writer types every frame is dirty; a
    /// report identical to the last one must cost nothing downstream.
    #[test]
    fn an_unchanged_report_does_not_move_the_counter() {
        let e = RowExtents::new();
        e.report(1, 0.0, 200.0);
        let g = e.generation().get();

        e.report(1, 0.0, 200.0);
        assert_eq!(
            e.generation().get(),
            g,
            "the same placement changed nothing"
        );
        e.report(1, 0.2, 200.1);
        assert_eq!(
            e.generation().get(),
            g,
            "and neither did a sub-pixel difference nobody can see"
        );

        e.report(1, 0.0, 260.0);
        assert_ne!(e.generation().get(), g, "a real reflow must be heard");
    }

    /// A row scrolled out of view is unbuilt, and an extent left behind for it would
    /// keep claiming a slice of the lane that something else now occupies.
    #[test]
    fn a_forgotten_row_stops_claiming_its_slice() {
        let e = RowExtents::new();
        e.report(1, 0.0, 500.0);
        e.report(2, 500.0, 500.0);
        assert_eq!(e.placed(), 2);

        e.forget(2);
        assert_eq!(e.placed(), 1);
        assert_eq!(e.items(), vec![1], "and it is gone from the mapped set");
    }

    /// Before the first layout there is nothing to map, and the lane must be handed
    /// an empty list rather than a row it would place at a made-up fraction.
    #[test]
    fn nothing_placed_maps_nothing() {
        let e = RowExtents::new();
        assert_eq!(e.placed(), 0);
        assert!(e.items().is_empty());
    }
}
