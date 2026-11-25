// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! [`ScratchpadViewModel`]: which item the Scratchpad dock shows, the reference it holds on
//! that item's open document, and the caret it left in each scratchpad.
//!
//! One per window (Tier 3): two windows on one Work look at different items, but the
//! document they edit is the Work's own (`OpenDocsStore` is Tier 2), so two windows showing
//! the same item type into one scratchpad.
//!
//! ## Which item
//!
//! [`resolve_target`] is the whole rule, and it reads three inputs the wiring pushes in:
//!
//! * the item of the focused pane's active tab (`EditorsViewModel::active_context`);
//! * whether that tab shows one of its **streams** (the Full Chapter, Part or Book, or the
//!   Full Synopsis) rather than its own page, its Corkboard, its Overview or its Analysis;
//! * the **row** whose editor last had the caret, kept here from the live
//!   [`FormatViewModel::caret_item`](crate::format::FormatViewModel::caret_item).
//!
//! In a stream the dock follows the row; anywhere else, the tab's item. The row is
//! *sticky*: the caret leaving the tabs (into the scratchpad itself, another dock, the
//! binder) reads as "nowhere", and nowhere never replaces the row. Only a caret in an
//! editor **of the tab in front** moves it ([`CaretItem::tab`]): the caret is read once a
//! frame, so for a frame after the writer clicks another tab it still names the editor
//! of the tab they left, and taking that as a row of the new tab followed a scene the
//! new tab does not show.
//!
//! The row is forgotten when the view in front really changes ([`FrontView`]: another
//! tab, the other pane, another segment), and then re-read from where the caret is at
//! that moment, under the same rule, so a click that moved the caret and the pane in one
//! gesture is not lost to the order the two signals happened to fire in. The edges the
//! wiring pushes on fire far more often than that (each time focus enters a pane, each
//! tab selection, even of the tab already selected), so a push that names the view
//! already in front changes nothing: clicking a row's title after typing in the
//! scratchpad no longer sends the dock back to the chapter. Changing segment keeps the
//! row: back on the Full Chapter from the Corkboard, the dock shows the row the writer
//! left.
//!
//! The row is also checked after every structural change
//! ([`ScratchpadViewModel::structure_changed`]): trashed, deleted or moved out of the
//! stream in front, it is no longer a row of it, and the dock goes back to the tab's
//! item.
//!
//! None of this changes what the other docks follow. They read `active_item`, which this
//! module only reads.
//!
//! ## The open document
//!
//! The target's `OpenDoc` is held with a real reference (`OpenDocsStore::open`), released
//! before the next one is taken, as the search preview does, so a document still live in a
//! tab is neither evicted early nor pinned for ever. The id is held, never the `Rc`: a
//! Promote rebuilds the document in place (`OpenDocsStore::rebuild`) and hands every holder
//! a fresh one, so [`ScratchpadViewModel::state`] re-reads it on every call. The store
//! says when it rebuilt one ([`OpenDocsStore::rebuilt`]), after the swap; the view-model
//! then compares and bumps [`ScratchpadViewModel::generation`] for the panel to rebuild
//! on.
//!
//! ## A search hit
//!
//! Opening a search result found in a scratchpad opens its item, and the dock follows the
//! item as it follows any other. The hit waits here
//! ([`ScratchpadViewModel::select_when_shown`]) until the panel shows that item, and the
//! panel then takes it ([`ScratchpadViewModel::take_hit_for`]) and selects it. Waiting
//! rather than selecting at once is what keeps the order of the two events free: the
//! request can arrive before the dock has moved to the item, and a selection made in
//! the scratchpad still on screen would land in the wrong item's notes. A hit still
//! waiting when the dock moves on to some other item is dropped: the writer went
//! elsewhere, and selecting it whenever they next pass by that item would be a surprise.
//! The panel hands it to the same `Destination` as a tab's editor
//! ([`crate::search::editor_result`]), which checks the words are still there and
//! focuses the editor either way.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use teksilo::core::ObserverHandle;
use teksilo::prelude::*;
use teksilo::widgets::SegmentId;

use frontend::AppContext;

use crate::app_ids::AppIds;
use crate::editors::Side;
use crate::format::CaretItem;
use crate::models::{OpenDoc, OpenDocsStore};
use crate::search::editor_result::EditorHit;
use crate::tabs::ProseField;

/// What is in front of the writer: the focused pane, its active tab's item, and the
/// segment that tab's bar shows. The row is forgotten only when this changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrontView {
    /// The focused pane's active tab item, `None` with no tab open.
    pub active: Option<u64>,
    /// The focused pane.
    pub pane: Side,
    /// The segment the tab's bar shows. `None` is the bar's first segment.
    pub segment: Option<SegmentId>,
}

impl FrontView {
    /// Nothing in front: no tab, in the main pane.
    pub const NOTHING: FrontView = FrontView {
        active: None,
        pane: Side::Primary,
        segment: None,
    };

    /// Whether the tab shows one of its streams.
    pub fn in_stream(&self) -> bool {
        is_stream_segment(self.segment)
    }
}

/// Which item the dock shows: the stream row the caret was last in when the tab in front
/// shows a stream, else the tab's own item, and nothing with no tab open.
pub fn resolve_target(active: Option<u64>, in_stream: bool, row: Option<u64>) -> Option<u64> {
    let active = active?;
    if in_stream {
        Some(row.unwrap_or(active))
    } else {
        Some(active)
    }
}

/// Whether the segment in front of a container tab is one of its streams, the views made
/// of one editor per row: the Full Chapter, Part or Book, and the Full Synopsis. `None` is
/// "nothing chosen", which a segment bar resolves to its first segment, the container's
/// own page.
pub fn is_stream_segment(segment: Option<SegmentId>) -> bool {
    use crate::tabs::shared::segments::{SEG_MANUSCRIPT, SEG_SYNOPSIS, segment_id};
    segment.is_some_and(|s| s == segment_id(SEG_MANUSCRIPT) || s == segment_id(SEG_SYNOPSIS))
}

#[cfg(all(test, not(feature = "mocks")))]
impl FrontView {
    /// A tab in the main pane showing `item`'s own page.
    pub(crate) fn page(item: u64) -> Self {
        Self {
            active: Some(item),
            pane: Side::Primary,
            segment: None,
        }
    }

    /// A tab in the main pane showing `item`'s Full Chapter, Part or Book.
    pub(crate) fn stream(item: u64) -> Self {
        use crate::tabs::shared::segments::{SEG_MANUSCRIPT, segment_id};
        Self {
            active: Some(item),
            pane: Side::Primary,
            segment: Some(segment_id(SEG_MANUSCRIPT)),
        }
    }
}

/// What the dock has to show.
pub enum ScratchpadState {
    /// No tab is open in front of the writer.
    NoItem,
    /// The item in front is a marker that carries no text (the end of a book, a text
    /// marker), so it has no scratchpad.
    NoScratchpad,
    /// The item's open document and its scratchpad field.
    Ready {
        doc: Rc<OpenDoc>,
        field: Rc<ProseField>,
    },
}

impl std::fmt::Debug for ScratchpadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoItem => f.write_str("NoItem"),
            Self::NoScratchpad => f.write_str("NoScratchpad"),
            Self::Ready { doc, .. } => write!(f, "Ready({})", doc.item_id),
        }
    }
}

/// A caret and the end of the selection it extends: `(anchor, position)`, as
/// `EditorHandle::selection` reads it and `select_range` writes it back.
pub type Caret = (usize, usize);

struct Inner {
    app_ctx: Rc<AppContext>,
    docs: OpenDocsStore,
    ids: AppIds,
    /// What is in front of the writer.
    view: Cell<FrontView>,
    /// The editor of the tab in front the caret was last in, kept until the view in
    /// front changes, or until a structural change takes it out of that tab.
    row: Cell<Option<u64>>,
    /// The row a structural change last took out of the tab in front, until the row
    /// or the view changes again: a merge empties the row before it says which scene
    /// the row went into (its entity events come first), and the dock follows it there.
    let_go: Cell<Option<u64>>,
    /// What the dock shows. Mutable, so the panel can rebuild on it.
    target: Signal<Option<u64>>,
    /// The item this view-model holds a store reference on. Never the `Rc`: see the
    /// module docs on Promote.
    held: Cell<Option<u64>>,
    /// The caret the writer left in each item's scratchpad, by store id. Session-local,
    /// as the ids are; never written down.
    carets: RefCell<HashMap<u64, Caret>>,
    /// The document the panel last built its editor over, to notice a Promote's swap.
    shown: RefCell<Weak<OpenDoc>>,
    /// Bumped when the document under the target was replaced, so the panel rebuilds.
    generation: Signal<u64>,
    /// The target's name, for the header and the editor's description.
    title: Signal<String>,
    /// A search hit waiting for the panel to show its item: the item and the hit.
    pending_hit: RefCell<Option<(u64, EditorHit)>>,
    /// Bumped on each hit asked for, so the panel rebuilds and takes it even when it
    /// already shows the item.
    hit_requests: Signal<u64>,
    /// The store's "a document was rebuilt" signal, observed for as long as this
    /// view-model lives.
    rebuilt_watch: RefCell<Option<ObserverHandle>>,
}

/// The Scratchpad dock's per-window logic. Cheap to clone; clones share one state.
#[derive(Clone)]
pub struct ScratchpadViewModel {
    inner: Rc<Inner>,
}

impl std::fmt::Debug for ScratchpadViewModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScratchpadViewModel")
            .field("target", &self.inner.target.get())
            .field("held", &self.inner.held.get())
            .finish_non_exhaustive()
    }
}

impl ScratchpadViewModel {
    /// A view-model over this Work's open documents, showing nothing until the wiring
    /// tells it what is in front.
    pub fn new(app_ctx: Rc<AppContext>, docs: OpenDocsStore, ids: AppIds) -> Self {
        let rebuilt = docs.rebuilt();
        let vm = Self {
            inner: Rc::new(Inner {
                app_ctx,
                docs,
                ids,
                view: Cell::new(FrontView::NOTHING),
                row: Cell::new(None),
                let_go: Cell::new(None),
                target: Signal::new(None),
                held: Cell::new(None),
                carets: RefCell::new(HashMap::new()),
                shown: RefCell::new(Weak::new()),
                generation: Signal::new(0),
                title: Signal::new(String::new()),
                pending_hit: RefCell::new(None),
                hit_requests: Signal::new(0),
                rebuilt_watch: RefCell::new(None),
            }),
        };
        // Weak: the store outlives this view-model, and its signal must not keep it.
        let weak = Rc::downgrade(&vm.inner);
        let watch = rebuilt.observe(move |_| {
            if let Some(inner) = weak.upgrade() {
                ScratchpadViewModel { inner }.document_rebuilt();
            }
        });
        *vm.inner.rebuilt_watch.borrow_mut() = Some(watch);
        vm
    }

    // ── Inputs ──────────────────────────────────────────────────────────────

    /// The wiring saw an edge at which the view in front may have changed: a pane
    /// gaining focus, a tab selected, the active item moving. `view` is what is in
    /// front now, and `caret_now` where the caret is at this moment (the live caret).
    ///
    /// Nothing happens unless `view` differs from the view already in front: those
    /// edges fire on every click into a pane, and forgetting the row on each of them
    /// sent the dock back to the chapter whenever the writer clicked a row's title.
    /// When it does differ, the row of the previous view is forgotten and the caret
    /// becomes the row, if it is in an editor of the tab now in front.
    pub fn view_changed(&self, view: FrontView, caret_now: Option<CaretItem>) {
        if self.inner.view.get() == view {
            return;
        }
        self.inner.view.set(view);
        self.inner
            .row
            .set(caret_now.and_then(|caret| self.row_of(caret)));
        self.inner.let_go.set(None);
        self.resolve();
    }

    /// The tab in front switched segment: a stream, or another of its views. The row is
    /// kept, so coming back to the stream shows the row the writer left.
    pub fn segment_changed(&self, segment: Option<SegmentId>) {
        let mut view = self.inner.view.get();
        if view.segment == segment {
            return;
        }
        let was = view.in_stream();
        view.segment = segment;
        self.inner.view.set(view);
        if view.in_stream() != was {
            self.resolve();
        }
    }

    /// The caret moved: into an editor in a tab, or anywhere else (`None`). Only a caret
    /// in an editor of the tab in front moves the row: the caret going into the
    /// scratchpad, a dock or the binder leaves the dock on the row it was showing, and
    /// so does a caret still read in the tab the writer has just left.
    pub fn caret_moved(&self, caret: Option<CaretItem>) {
        if let Some(item) = caret.and_then(|caret| self.row_of(caret))
            && self.inner.row.get() != Some(item)
        {
            self.inner.row.set(Some(item));
            self.inner.let_go.set(None);
            self.resolve();
        }
    }

    /// `merged` was merged into `survivor`: its text, synopsis and scratchpad are now on
    /// `survivor`, and `merged` is in the Trash. When the dock was on `merged`, it goes
    /// on to `survivor`, where the notes it showed now are, instead of falling back to
    /// the chapter as it does for a scene trashed, moved or deleted. The stream's merge
    /// leaves the caret where it was, so no caret move would bring it there.
    pub fn scenes_merged(&self, survivor: u64, merged: u64) {
        let was = self.inner.row.get().or(self.inner.let_go.get());
        if was == Some(merged) && self.row_belongs_to_tab(survivor) {
            self.inner.row.set(Some(survivor));
            self.inner.let_go.set(None);
            self.resolve();
        }
    }

    /// Something moved in the binder: an item created, moved, trashed, restored,
    /// deleted, merged or split, a binder reordered, the numbering changed. The row is
    /// dropped if it is no longer a row of the stream in front (trashed, deleted, moved
    /// to another chapter), and the name in the header is re-read if the renumbering
    /// may have renamed it.
    pub fn structure_changed(&self) {
        self.check_row();
        self.renumbered();
    }

    /// Items were updated (`ids`): a rename, a type change, a trashing. The target's
    /// own name is re-read when it is among them; any other may have renumbered it,
    /// or taken the row out of the stream in front.
    pub fn items_updated(&self, ids: &[u64]) {
        self.check_row();
        match self.inner.target.get() {
            Some(target) if ids.contains(&target) => self.refresh_title(),
            _ => self.renumbered(),
        }
    }

    /// The window's project closed: give the reference back and show nothing. What was
    /// remembered about carets goes too, since the ids it is keyed by mean nothing in
    /// the next project.
    pub fn release(&self) {
        self.release_on(self.inner.ids.stack_id.get());
    }

    /// [`Self::release`], flushing on `stack`: the window's own stack, snapshotted
    /// before its teardown began, since the live id may already have been cleared or
    /// handed to the next project by then.
    pub fn release_on(&self, stack: Option<u64>) {
        self.inner.view.set(FrontView::NOTHING);
        self.inner.row.set(None);
        self.switch_to(None, stack);
        self.inner.carets.borrow_mut().clear();
        *self.inner.pending_hit.borrow_mut() = None;
    }

    // ── Outputs ─────────────────────────────────────────────────────────────

    /// The item the dock shows, live. The panel rebuilds on it.
    pub fn target(&self) -> Signal<Option<u64>> {
        self.inner.target.clone()
    }

    /// Bumped when the target's document was replaced under the dock (a Promote). The
    /// panel rebuilds on it.
    pub fn generation(&self) -> Signal<u64> {
        self.inner.generation.clone()
    }

    /// The target's name: its title, the name the manuscript gives an untitled chapter,
    /// or "Untitled". Empty with no target.
    pub fn title(&self) -> Signal<String> {
        self.inner.title.clone()
    }

    /// What the dock shows now, read fresh: the target's current document (re-read, so a
    /// Promote's rebuilt document is the one handed back) and its scratchpad, built on
    /// first ask.
    pub fn state(&self) -> ScratchpadState {
        let Some(doc) = self.doc() else {
            return ScratchpadState::NoItem;
        };
        match self.inner.docs.ensure_scratchpad(&doc) {
            Some(field) => ScratchpadState::Ready { doc, field },
            None => ScratchpadState::NoScratchpad,
        }
    }

    /// The target's open document, re-read from the store.
    pub fn doc(&self) -> Option<Rc<OpenDoc>> {
        self.inner
            .held
            .get()
            .and_then(|id| self.inner.docs.peek(id))
    }

    /// The language the target is written in, for the caret band.
    pub fn locale(&self, item_id: u64) -> Option<String> {
        self.inner.docs.effective_language(item_id).first().cloned()
    }

    // ── The caret ───────────────────────────────────────────────────────────

    /// Remember where the caret was in `item`'s scratchpad.
    pub fn remember_caret(&self, item: u64, caret: Caret) {
        self.inner.carets.borrow_mut().insert(item, caret);
    }

    /// Where the caret was left in `item`'s scratchpad, if the writer was ever in it.
    pub fn remembered_caret(&self, item: u64) -> Option<Caret> {
        self.inner.carets.borrow().get(&item).copied()
    }

    // ── A search hit ────────────────────────────────────────────────────────

    /// Select `hit` in `item`'s scratchpad once the panel shows that item: a search
    /// result found in a scratchpad and opened. The dock follows the item the result
    /// opened, so this only waits for it to get there. A later request replaces an
    /// earlier one still waiting.
    pub fn select_when_shown(&self, item: u64, hit: EditorHit) {
        *self.inner.pending_hit.borrow_mut() = Some((item, hit));
        self.inner
            .hit_requests
            .set(self.inner.hit_requests.get().wrapping_add(1));
    }

    /// Bumped by [`Self::select_when_shown`]. The panel rebuilds on it.
    pub fn hit_requests(&self) -> Signal<u64> {
        self.inner.hit_requests.clone()
    }

    /// The hit waiting for `item`, if one is: taken once, for the panel to hand to the
    /// editor's `Destination` ([`crate::search::editor_result`]), which selects it if
    /// its words are still where the search found them and focuses the editor either
    /// way. A hit for another item is left waiting.
    pub fn take_hit_for(&self, item: u64) -> Option<EditorHit> {
        let mut pending = self.inner.pending_hit.borrow_mut();
        match pending.as_ref() {
            Some((waiting, _)) if *waiting == item => pending.take().map(|(_, hit)| hit),
            _ => None,
        }
    }

    // ── The document under the target ───────────────────────────────────────

    /// Record the document the panel built its editor over.
    pub fn note_shown(&self, doc: &Rc<OpenDoc>) {
        *self.inner.shown.borrow_mut() = Rc::downgrade(doc);
    }

    /// The store rebuilt a document (a Promote retyped its item): compare the one the
    /// panel shows with the one the store now holds for the target, and bump
    /// [`Self::generation`] when it was replaced. Called by the store's own signal,
    /// after the swap.
    pub fn document_rebuilt(&self) {
        let shown = self.inner.shown.borrow().upgrade();
        let Some(shown) = shown else {
            return;
        };
        let current = self.doc();
        if !current.is_some_and(|doc| Rc::ptr_eq(&doc, &shown)) {
            *self.inner.shown.borrow_mut() = Weak::new();
            self.inner
                .generation
                .set(self.inner.generation.get().wrapping_add(1));
        }
    }

    /// Re-read the target's name: after a rename, or a renumbering that renamed an
    /// untitled chapter.
    pub fn refresh_title(&self) {
        let title = match self.inner.target.get() {
            Some(id) => self.name_of(id),
            None => String::new(),
        };
        if self.inner.title.get() != title {
            self.inner.title.set(title);
        }
    }

    /// Re-read the target's name after a change that renumbers the manuscript, when
    /// the name can depend on it: only an untitled target is named by its number, so a
    /// titled one costs one read of its title and no walk of the manuscript.
    pub fn renumbered(&self) {
        let Some(id) = self.inner.target.get() else {
            return;
        };
        let titled = crate::shared::binder_ops::item_dto(&self.inner.app_ctx, id)
            .is_some_and(|dto| !dto.title.trim().is_empty());
        if !titled {
            self.refresh_title();
        }
    }

    // ── Internals ───────────────────────────────────────────────────────────

    /// The row a caret names, when its editor belongs to the tab in front.
    fn row_of(&self, caret: CaretItem) -> Option<u64> {
        let active = self.inner.view.get().active?;
        (caret.tab == Some(active)).then_some(caret.item)
    }

    /// Drop the row if it no longer belongs to the tab in front.
    fn check_row(&self) {
        if let Some(row) = self.inner.row.get()
            && !self.row_belongs_to_tab(row)
        {
            self.inner.row.set(None);
            self.inner.let_go.set(Some(row));
            self.resolve();
        }
    }

    /// Whether `row` still belongs to the tab in front: the tab's own item, or a row of
    /// the stream it heads.
    fn row_belongs_to_tab(&self, row: u64) -> bool {
        let Some(active) = self.inner.view.get().active else {
            return false;
        };
        row == active
            || crate::models::stream_has_row(
                &self.inner.app_ctx,
                self.inner.ids.work_id.get(),
                active,
                row,
            )
    }

    fn resolve(&self) {
        let view = self.inner.view.get();
        let target = resolve_target(view.active, view.in_stream(), self.inner.row.get());
        self.switch_to(target, self.inner.ids.stack_id.get());
    }

    /// Point the dock at `target`: the previous reference is given back **before** the
    /// next is taken, so a document that only this view-model still held is flushed (on
    /// `stack`) and let go, and the next one is opened once. A search hit still waiting
    /// for some other item is dropped: the dock went elsewhere.
    fn switch_to(&self, target: Option<u64>, stack: Option<u64>) {
        if self.inner.held.get() == target && self.inner.target.get() == target {
            return;
        }
        if let Some(to) = target {
            let mut pending = self.inner.pending_hit.borrow_mut();
            if pending.as_ref().is_some_and(|(item, _)| *item != to) {
                *pending = None;
            }
        }
        if let Some(old) = self.inner.held.take() {
            self.inner.docs.release(old, stack);
        }
        let opened = target.and_then(|id| self.inner.docs.open(id).map(|_| id));
        self.inner.held.set(opened);
        if self.inner.target.get() != opened {
            self.inner.target.set(opened);
        }
        self.refresh_title();
    }

    /// What the writer calls `item_id`: its title, else the name the manuscript gives an
    /// untitled structural row ("Chapter 3"), else "Untitled".
    fn name_of(&self, item_id: u64) -> String {
        let app_ctx = &*self.inner.app_ctx;
        let title = crate::shared::binder_ops::item_dto(app_ctx, item_id)
            .map(|dto| dto.title)
            .unwrap_or_default();
        if !title.trim().is_empty() {
            return title;
        }
        let generated = self.inner.ids.work_id.get().and_then(|work_id| {
            let names = crate::models::NameContext::read(app_ctx, work_id);
            let it = names.item(item_id)?;
            names.generated_name(it)
        });
        generated.unwrap_or_else(|| tr!(untitled()).resolve_now())
    }
}

#[cfg(test)]
mod tests;
