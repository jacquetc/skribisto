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
//! binder) reads as "nowhere", and nowhere never replaces the row. It is forgotten when the
//! view in front changes (another tab, the other pane), and then re-read from where the
//! caret is at that moment, so a click that moved the caret and the pane in one gesture is
//! not lost to the order the two signals happened to fire in. Changing segment keeps it:
//! back on the Full Chapter from the Corkboard, the dock shows the row the writer left.
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
//! a fresh one, so [`ScratchpadViewModel::state`] re-reads it on every call, and
//! [`ScratchpadViewModel::poll_document`] notices the swap and bumps
//! [`ScratchpadViewModel::generation`] for the panel to rebuild on.
//!
//! ## A search hit
//!
//! Opening a search result found in a scratchpad opens its item, and the dock follows the
//! item as it follows any other. The hit waits here
//! ([`ScratchpadViewModel::select_when_shown`]) until the panel shows that item, and the
//! panel then takes it ([`ScratchpadViewModel::take_hit_for`]) and selects it. Waiting
//! rather than selecting at once is what keeps the order of the two events free: the
//! request can arrive before the dock has moved to the item, and a selection made in
//! the scratchpad still on screen would land in the wrong item's notes.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use teksilo::prelude::*;
use teksilo::widgets::SegmentId;

use frontend::AppContext;

use crate::app_ids::AppIds;
use crate::models::{OpenDoc, OpenDocsStore};
use crate::search::editor_result::EditorHit;
use crate::tabs::ProseField;

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
    /// The focused pane's active tab item.
    active: Cell<Option<u64>>,
    /// Whether that tab shows a stream.
    in_stream: Cell<bool>,
    /// The tab editor the caret was last in, kept until the view in front changes.
    row: Cell<Option<u64>>,
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
        Self {
            inner: Rc::new(Inner {
                app_ctx,
                docs,
                ids,
                active: Cell::new(None),
                in_stream: Cell::new(false),
                row: Cell::new(None),
                target: Signal::new(None),
                held: Cell::new(None),
                carets: RefCell::new(HashMap::new()),
                shown: RefCell::new(Weak::new()),
                generation: Signal::new(0),
                title: Signal::new(String::new()),
                pending_hit: RefCell::new(None),
                hit_requests: Signal::new(0),
            }),
        }
    }

    // ── Inputs ──────────────────────────────────────────────────────────────

    /// The view in front changed: another tab, the other pane, or the project closed.
    /// `active` is the focused tab's item, `in_stream` whether it shows a stream, and
    /// `caret_now` where the caret is at this moment (the live caret item), which becomes
    /// the row to follow. The row of the previous view is forgotten.
    pub fn view_changed(&self, active: Option<u64>, in_stream: bool, caret_now: Option<u64>) {
        self.inner.active.set(active);
        self.inner.in_stream.set(in_stream);
        self.inner.row.set(caret_now);
        self.resolve();
    }

    /// The tab in front switched segment: a stream, or another of its views. The row is
    /// kept, so coming back to the stream shows the row the writer left.
    pub fn segment_changed(&self, in_stream: bool) {
        if self.inner.in_stream.get() != in_stream {
            self.inner.in_stream.set(in_stream);
            self.resolve();
        }
    }

    /// The caret moved: `Some(item)` into a tab editor showing `item`'s text, `None`
    /// anywhere else. Only the first moves the row: the caret going into the scratchpad,
    /// a dock or the binder leaves the dock on the row it was showing.
    pub fn caret_moved(&self, item: Option<u64>) {
        if let Some(item) = item
            && self.inner.row.get() != Some(item)
        {
            self.inner.row.set(Some(item));
            self.resolve();
        }
    }

    /// The window's project closed, or the window is going away: give the reference
    /// back and show nothing. What was remembered about carets goes too, since the ids it
    /// is keyed by mean nothing in the next project.
    pub fn release(&self) {
        self.inner.active.set(None);
        self.inner.in_stream.set(false);
        self.inner.row.set(None);
        self.switch_to(None);
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

    /// The range to select in `item`'s scratchpad, `doc`, if a hit is waiting for that
    /// item: `(start, end)` in the document's characters. Taken once.
    ///
    /// A hit for another item is left waiting. A hit whose words are no longer where the
    /// search found them (the notes were edited since) is dropped and selects nothing:
    /// selecting whatever now sits there would point the writer at the wrong words.
    pub fn take_hit_for(
        &self,
        item: u64,
        doc: &teksilo::text_document::TextDocument,
    ) -> Option<(usize, usize)> {
        let hit = {
            let mut pending = self.inner.pending_hit.borrow_mut();
            match pending.as_ref() {
                Some((waiting, _)) if *waiting == item => pending.take().map(|(_, hit)| hit),
                _ => None,
            }
        }?;
        let len = hit.matched.chars().count();
        let end = hit.start.checked_add(len)?;
        // A search's one-line snippet shows a soft line break as a space.
        let found = doc.text_at(hit.start, len).ok()?;
        (found.replace(teksilo::text_document::LINE_SEPARATOR, " ") == hit.matched)
            .then_some((hit.start, end))
    }

    // ── The document under the target ───────────────────────────────────────

    /// Record the document the panel built its editor over.
    pub fn note_shown(&self, doc: &Rc<OpenDoc>) {
        *self.inner.shown.borrow_mut() = Rc::downgrade(doc);
    }

    /// Compare the document the panel shows with the one the store holds for the target,
    /// and bump [`Self::generation`] when a Promote replaced it. Called once a frame: the
    /// swap happens inside other handlers of the same event, in an order nothing here can
    /// rely on, and a frame later every one of them has run.
    pub fn poll_document(&self) {
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

    // ── Internals ───────────────────────────────────────────────────────────

    fn resolve(&self) {
        let target = resolve_target(
            self.inner.active.get(),
            self.inner.in_stream.get(),
            self.inner.row.get(),
        );
        self.switch_to(target);
    }

    /// Point the dock at `target`: the previous reference is given back **before** the
    /// next is taken, so a document that only this view-model still held is flushed and
    /// let go, and the next one is opened once.
    fn switch_to(&self, target: Option<u64>) {
        if self.inner.held.get() == target && self.inner.target.get() == target {
            return;
        }
        let stack = self.inner.ids.stack_id.get();
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
