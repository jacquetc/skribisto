// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `FootnotesViewModel` — creating, editing and navigating the notes that render
//! into the book.
//!
//! One instance per window, shared by the dock and by every open document (through
//! a [`FootnoteBinding`]). It owns three things no widget should: the minted
//! label, the marker map every document paints from, and the parked navigation
//! request the dock hands to an editor that does not exist yet.
//!
//! # The marker map is pushed, not pulled
//!
//! A document cannot work out what its own markers print. The number is a fact
//! about the whole manuscript — a scene citing a note first introduced two
//! chapters earlier draws *that* note's number — so the map is computed once, from
//! the model, and pushed down to every open document through
//! [`OpenDocsStore::set_footnote_markers`](crate::models::OpenDocsStore::set_footnote_markers).
//! [`push_markers`](FootnotesViewModel::push_markers) runs on every model version
//! bump, which is what keeps a renumbering after a chapter move from stopping at
//! the dock.
//!
//! # Navigation is by label, never by offset
//!
//! [`request_seek`](FootnotesViewModel::request_seek) parks a *label*, and the
//! editor resolves it to a position as it attaches. Offsets are the one thing that
//! cannot survive the trip: the dock knows a Djot byte offset, the editor wants a
//! character position in the addressable view, and between the click and the build
//! the writer may have typed. Asking the live document where `[^label]` is now is
//! the only answer that cannot be stale — and it is a lookup, not a search.

use std::collections::HashMap;
use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::text_document::TextDocument;
use bastyde::widgets::rich_text::EditorHandle;

use crate::models::{FootnoteRow, FootnotesListModel, OpenDocsStore};

/// Which notes the dock is showing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FootnoteFilter {
    #[default]
    All,
    /// Only the notes referenced from the document in front of the writer.
    ThisDocument,
    /// Only the notes nothing points at any more.
    Orphaned,
}

struct Inner {
    model: FootnotesListModel,
    docs: OpenDocsStore,
    stack_id: Signal<Option<u64>>,
    /// `(content_id, label)` — where the dock asked an editor to put the caret.
    pending_seek: Signal<Option<(u64, String)>>,
    /// The note the caret is sitting on (or immediately after), if any. The
    /// dock's reverse highlight: click a marker in the prose and its row lights
    /// up, without the dock having to watch every editor itself.
    caret_label: Signal<Option<String>>,
    /// The row whose body the dock has open for editing.
    editing: Signal<Option<u64>>,
    filter: Signal<FootnoteFilter>,
    /// One live document per note's body editor — see [`body_doc`](FootnotesViewModel::body_doc).
    body_docs: std::cell::RefCell<HashMap<u64, TextDocument>>,
}

#[derive(Clone)]
pub struct FootnotesViewModel {
    inner: Rc<Inner>,
}

impl FootnotesViewModel {
    pub fn new(
        model: FootnotesListModel,
        docs: OpenDocsStore,
        stack_id: Signal<Option<u64>>,
    ) -> Self {
        Self {
            inner: Rc::new(Inner {
                model,
                docs,
                stack_id,
                pending_seek: Signal::new(None),
                caret_label: Signal::new(None),
                editing: Signal::new(None),
                filter: Signal::new(FootnoteFilter::default()),
                body_docs: std::cell::RefCell::new(HashMap::new()),
            }),
        }
    }

    /// Wire the model's subscriptions and keep the documents' marker map current.
    ///
    /// The effect is the load-bearing half. Without it the map is pushed once, at
    /// startup, and a note created later — or renumbered by a chapter that moved
    /// above it — prints a stale marker in prose that has already been re-laid
    /// out.
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.inner.model.wire(ctx);
        let me = self.clone();
        ctx.effect(&self.inner.model.version_signal(), move |_| {
            me.push_markers();
        });
        self.push_markers();
    }

    pub fn model(&self) -> FootnotesListModel {
        self.inner.model.clone()
    }

    fn stack(&self) -> Option<u64> {
        self.inner.stack_id.get()
    }

    /// Hand every open document the current marker map.
    ///
    /// Orphans are included, mapped to the bullet. They have no reference to
    /// paint, so the entry is inert — but it is also the only thing standing
    /// between a writer and the raw label: a note exists for a moment before its
    /// reference does (see [`insert_at`](Self::insert_at)), and a map that
    /// omitted it would draw `fn7` into the prose for that frame.
    pub fn push_markers(&self) {
        self.inner
            .docs
            .set_footnote_markers(self.inner.model.markers());
    }

    /// An edit landed somewhere — renumber if a reference moved.
    pub fn note_live_edit(&self) {
        self.inner.model.note_live_edit();
    }

    // ── Creating ─────────────────────────────────────────────────────────────

    /// Put a new, empty note at `handle`'s caret, annotating `content_id`.
    ///
    /// Order matters and is not the obvious one. The **note is created first**, so
    /// a backend failure leaves the prose untouched rather than stranding a
    /// `[^label]` with nothing behind it; then the reference goes in; then the
    /// model is told to re-read, which is what gives the marker its number before
    /// anything paints. Returns the new note's id.
    pub fn insert_at(&self, handle: &EditorHandle, content_id: u64) -> Option<u64> {
        let label = self.inner.model.mint_label();
        let id = self
            .inner
            .model
            .create(content_id, &label, "", self.stack())?;
        handle.insert_djot(&format!("[^{label}]"));
        // Explicit, rather than waiting for the frame's edit signal: the dock is
        // about to open this note for typing, and a row that has not been placed
        // yet has no number to show beside the box.
        self.inner.model.note_live_edit();
        self.push_markers();
        self.inner.editing.set(Some(id));
        self.inner.caret_label.set(Some(label));
        Some(id)
    }

    // ── Navigating ───────────────────────────────────────────────────────────

    /// Ask the editor showing `content_id` to select `label`'s reference when it
    /// next attaches.
    pub fn request_seek(&self, content_id: u64, label: &str) {
        self.inner
            .pending_seek
            .set(Some((content_id, label.to_string())));
    }

    /// Consume a parked seek if it targets `content_id`.
    pub fn take_seek(&self, content_id: u64) -> Option<String> {
        match self.inner.pending_seek.get() {
            Some((cid, label)) if cid == content_id => {
                self.inner.pending_seek.set(None);
                Some(label)
            }
            _ => None,
        }
    }

    /// The note the caret is on, for the dock's highlight.
    pub fn caret_label(&self) -> Signal<Option<String>> {
        self.inner.caret_label.clone()
    }

    /// Report where the caret is, in a document.
    ///
    /// Checks the position itself **and** the one before it, because selecting a
    /// reference leaves the caret past it: a click on the marker, or the dock's
    /// own seek, both end with the caret at `marker + 1`, and only looking at
    /// `pos` would light up no row at exactly the moment the writer asked for one.
    pub fn caret_moved(&self, doc: &TextDocument, pos: usize) {
        let found = doc.footnote_reference_at(pos).or_else(|| {
            pos.checked_sub(1)
                .and_then(|p| doc.footnote_reference_at(p))
        });
        if self.inner.caret_label.get() != found {
            self.inner.caret_label.set(found);
        }
    }

    // ── Editing ──────────────────────────────────────────────────────────────

    /// The live document behind one note's body editor, cached per note.
    ///
    /// Cached, and not rebuilt from `initial` on every pass, for the reason the
    /// comment card records: the dock re-renders whenever the row changes, and
    /// re-minting the document under a writer's caret drops it mid-word. The
    /// stored text is only ever used to *seed* a document that does not exist
    /// yet.
    pub fn body_doc(&self, id: u64, initial: &str) -> TextDocument {
        let mut docs = self.inner.body_docs.borrow_mut();
        docs.entry(id)
            .or_insert_with(|| {
                let doc = TextDocument::new();
                let _ = doc.set_djot_sync(initial);
                doc
            })
            .clone()
    }

    fn forget_body_doc(&self, id: u64) {
        self.inner.body_docs.borrow_mut().remove(&id);
    }

    pub fn set_body(&self, id: u64, body: &str) {
        self.inner.model.set_body(id, body, self.stack());
    }

    /// Delete a note and every reference to it — see the model's own notes on why
    /// the two go together.
    pub fn delete(&self, id: u64) {
        self.inner.model.delete(id, self.stack());
        self.forget_body_doc(id);
        if self.inner.editing.get() == Some(id) {
            self.inner.editing.set(None);
        }
        self.push_markers();
    }

    pub fn editing(&self) -> Signal<Option<u64>> {
        self.inner.editing.clone()
    }

    pub fn set_editing(&self, id: Option<u64>) {
        self.inner.editing.set(id);
    }

    // ── The dock's own state ─────────────────────────────────────────────────

    pub fn filter(&self) -> Signal<FootnoteFilter> {
        self.inner.filter.clone()
    }

    pub fn set_filter(&self, filter: FootnoteFilter) {
        self.inner.filter.set(filter);
    }

    pub fn orphan_count(&self) -> usize {
        self.inner.model.orphan_count()
    }

    /// The rows the dock should show, given its filter and the document in front
    /// of the writer.
    pub fn visible_rows(&self, focus: Option<u64>) -> Vec<FootnoteRow> {
        let rows = self.inner.model.rows();
        match self.inner.filter.get() {
            FootnoteFilter::All => rows,
            FootnoteFilter::Orphaned => rows.into_iter().filter(|r| r.orphaned).collect(),
            // No open document is not "no notes": it is a question with no answer
            // yet, and an empty list says so honestly.
            FootnoteFilter::ThisDocument => match focus {
                Some(item) => rows
                    .into_iter()
                    .filter(|r| r.item_id == Some(item))
                    .collect(),
                None => Vec::new(),
            },
        }
    }

    /// A per-document handle for the editors — see [`FootnoteBinding`].
    pub fn binding(&self, content_id: u64) -> FootnoteBinding {
        FootnoteBinding {
            vm: self.clone(),
            content_id,
        }
    }
}

/// One document's door to the footnote feature: the view-model plus the `Content`
/// row that document holds.
///
/// Minted by the [`OpenDoc`](crate::models::OpenDoc) that knows which row a
/// document came from, exactly as `CommentBinding` is — an editor is handed one
/// rather than the view-model itself, so it can never act on another document's
/// notes by holding the wrong id.
#[derive(Clone)]
pub struct FootnoteBinding {
    vm: FootnotesViewModel,
    content_id: u64,
}

impl FootnoteBinding {
    pub fn content_id(&self) -> u64 {
        self.content_id
    }

    /// The label this editor was asked to reveal, if any.
    pub fn take_seek(&self) -> Option<String> {
        self.vm.take_seek(self.content_id)
    }

    /// Report this editor's caret to the dock.
    pub fn caret_moved(&self, doc: &TextDocument, pos: usize) {
        self.vm.caret_moved(doc, pos);
    }

    /// Where in `doc` the reference named `label` sits, if it is still there.
    ///
    /// A lookup over the document's own anchor table, not a search of its text —
    /// which is why the dock can park a label and let the editor resolve it at
    /// the last possible moment.
    pub fn position_of(doc: &TextDocument, label: &str) -> Option<usize> {
        doc.footnote_references()
            .into_iter()
            .find(|(_, l)| l == label)
            .map(|(pos, _)| pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vm() -> FootnotesViewModel {
        let app_ctx = Rc::new(frontend::AppContext::new());
        let ids = crate::app_ids::AppIds::new();
        let docs = OpenDocsStore::new(app_ctx.clone());
        let model = FootnotesListModel::new(app_ctx, ids, docs.clone());
        FootnotesViewModel::new(model, docs, Signal::new(None))
    }

    /// A parked seek belongs to one document. Handing it to the wrong editor
    /// would put the caret in a scene the writer did not ask for — and consume
    /// the request, so the right editor would never see it.
    #[test]
    fn a_seek_is_only_taken_by_the_document_it_names() {
        let vm = vm();
        vm.request_seek(11, "fn1");
        assert_eq!(vm.take_seek(22), None, "another document took it");
        assert_eq!(vm.take_seek(11).as_deref(), Some("fn1"));
        assert_eq!(vm.take_seek(11), None, "a seek is consumed once");
    }

    /// The filter is the dock's whole state, and "this document" with nothing
    /// open is a question with no answer — not a claim that there are no notes.
    #[test]
    fn this_document_with_nothing_open_shows_nothing() {
        let vm = vm();
        vm.set_filter(FootnoteFilter::ThisDocument);
        assert!(vm.visible_rows(None).is_empty());
    }

    /// Editing state clears when the row it names goes away, or the dock keeps a
    /// body box open over a note that no longer exists.
    #[test]
    fn deleting_the_row_being_edited_closes_the_editor() {
        let vm = vm();
        vm.set_editing(Some(7));
        vm.delete(7);
        assert_eq!(vm.editing().get(), None);
    }
}
