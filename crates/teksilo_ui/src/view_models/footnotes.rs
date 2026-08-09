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
use std::time::Duration;

use teksilo::prelude::*;
use teksilo::text_document::TextDocument;
use teksilo::widgets::rich_text::EditorHandle;
use teksilo::widgets::{Toast, ToastAction};

use crate::models::{FootnoteRow, FootnotesListModel, OpenDocsStore};
use crate::toast_scope::ToastWorkExt;

/// How long the delete-note "Undo" toast stays live before the deletion is the
/// only outcome anyone still sees — matches the comment feature's own
/// "deleted — Undo" snackbar (`CommentsViewModel`'s `UNDO_GRACE`), the closer
/// sibling of this op: both delete one entity, offer a few seconds to take it
/// back, and (unlike Empty Trash / Delete Forever) never clear the project's
/// undo history when the window lapses — see [`FootnotesViewModel::delete`].
const FOOTNOTE_DELETE_UNDO_GRACE: Duration = Duration::from_secs(6);

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
    /// The row whose body is live under a caret right now.
    ///
    /// Set once, up front, by [`insert_at`](FootnotesViewModel::insert_at) —
    /// a brand-new note's body is about to be typed into before its editor
    /// even exists to report focus itself — and kept correct after that by
    /// the body editor's own real focus signal
    /// (`docks::footnotes::NoteBodyStyle::make_body`, via
    /// [`set_editing`](FootnotesViewModel::set_editing)). Consulted by
    /// [`body_doc`](FootnotesViewModel::body_doc) to decide whether an
    /// external body change is safe to fold into an already-cached document —
    /// see that function's own doc comment for why "is someone typing into
    /// this exact row right now" is the one thing that must gate a re-sync.
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
    pub fn insert_at(&self, handle: &EditorHandle, binding: &FootnoteBinding) -> Option<u64> {
        let content_id = binding.ensure_content_id(self.stack())?;
        let label = self.inner.model.mint_label();
        let id = self
            .inner
            .model
            .create(content_id, &label, "", self.stack())?;
        // Collapse a live selection to its **end** before inserting.
        //
        // `insert_djot` replaces the selection — right for typing, and exactly
        // wrong here: selecting the word you want to annotate and asking for a
        // footnote deleted the word and left the marker in its place. A footnote
        // annotates text, so its marker goes *after* it, which is where every
        // word processor puts it and where a reader looks for it.
        //
        // `max`, not `position`: a selection dragged right-to-left has its
        // position *before* its anchor, so collapsing to `position` would put the
        // marker at the head of the phrase and annotate the wrong word.
        let (anchor, position) = handle.selection();
        if anchor != position {
            let end = anchor.max(position);
            handle.select_range(end, end);
        }
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
    /// re-minting the document under a writer's caret drops it mid-word.
    ///
    /// **But "cached" must not mean "frozen forever".** A body can change out
    /// from under this cache without ever going through it — Search & Replace,
    /// an Undo, or a second window on the same `Work` all write `Footnote.body`
    /// directly, and nothing downstream of that pushes it into a document that
    /// already exists: `FootnotesListModel::refresh_bodies` only patches the
    /// *row* a fresh document would be seeded from (its own doc comment says
    /// so), never a live `TextDocument`. Left alone, the stale text sits
    /// invisibly in the dock until the writer's next keystroke here commits
    /// `doc.to_djot()` right back over whatever had just landed — silently
    /// discarding it.
    ///
    /// So this re-syncs whenever `initial` disagrees with what the cached
    /// document currently holds — but **only** when `id` is not the row
    /// [`editing`](Self::editing) says is live under a caret right now (kept
    /// current by the body editor's own real focus signal — see
    /// `docks::footnotes::NoteBodyStyle`). That gate is what keeps a self-typed
    /// edit from tripping this at all: `on_change` commits synchronously to the
    /// backend on every keystroke, and event dispatch here is synchronous too,
    /// so by the time this function runs again for a row the writer is
    /// actively in, `initial` already equals what they just typed — no
    /// disagreement, no re-sync attempted. It is only a genuinely external
    /// rewrite that disagrees, and gating on focus is what stops handling
    /// *that* from re-introducing the exact clobbered-caret bug this cache
    /// exists to prevent — the fix must not trade one bug for the other.
    pub fn body_doc(&self, id: u64, initial: &str) -> TextDocument {
        let mut docs = self.inner.body_docs.borrow_mut();
        if let Some(doc) = docs.get(&id) {
            let stale = doc.to_djot().unwrap_or_default() != initial;
            let live_under_a_caret = self.inner.editing.get() == Some(id);
            if stale && !live_under_a_caret {
                let _ = doc.set_djot_sync(initial);
            }
            return doc.clone();
        }
        let doc = TextDocument::new();
        let _ = doc.set_djot_sync(initial);
        docs.insert(id, doc.clone());
        doc
    }

    fn forget_body_doc(&self, id: u64) {
        self.inner.body_docs.borrow_mut().remove(&id);
    }

    pub fn set_body(&self, id: u64, body: &str) {
        self.inner.model.set_body(id, body, self.stack());
    }

    /// Delete a note and every reference to it, offering a few seconds to take
    /// it back.
    ///
    /// **Stopgap, not the final design.** Skribisto has no general Edit ▸ Undo
    /// surface yet — the only doors onto the backend's undo/redo stack today
    /// are a handful of per-feature "deleted — Undo" toasts (comments, search
    /// & replace, trash). This is the footnotes feature's door, and it should
    /// be replaced by a real one the day this app grows a general Undo command
    /// that can reach the same stack directly.
    ///
    /// Deliberately no confirmation dialog first: the whole point of this
    /// shape (see `TrashViewModel`'s "destructive ops keep their undo, then
    /// commit on a grace timer") is that the writer does not have to stop and
    /// answer a question before the click even lands — the toast is the
    /// safety net instead. Unlike Empty Trash / Delete Forever, letting the
    /// grace window lapse here does **not** clear the project's undo history:
    /// deleting one footnote is a single-entity edit, the same shape as
    /// deleting a comment, not a bulk purge — see
    /// `CommentsViewModel::delete_with_undo`'s own reasoning for exactly this
    /// distinction. It stays on the normal undo stack for good; the toast is
    /// only a convenience for the moment right after the click.
    ///
    /// `FootnotesListModel::delete` already ran the whole thing as one
    /// `begin_composite`/`end_composite` step, so its returned closure reverses
    /// **both** halves — the removed `Footnote` row and every `[^label]`
    /// reference the delete stripped — in one call; nothing here needs to
    /// remember what was removed in order to bring it back. `None` means the
    /// id was already gone (a stale row), so there is nothing to offer a toast
    /// for at all.
    pub fn delete(&self, ctx: &mut EventContext, id: u64) {
        let undo = self.inner.model.delete(id, self.stack());
        self.forget_body_doc(id);
        if self.inner.editing.get() == Some(id) {
            self.inner.editing.set(None);
        }
        self.push_markers();
        let Some(undo) = undo else { return };
        ctx.show_toast(
            Toast::warning(tr!(footnotes_deleted_toast()))
                // One toast for the whole feature, like the comment margin's —
                // a burst of deletes replaces its own snackbar instead of
                // stacking a tower of them.
                .scoped_id("footnotes.deleted", 0)
                .auto_dismiss_after(FOOTNOTE_DELETE_UNDO_GRACE)
                .action(ToastAction::primary(
                    tr!(footnotes_undo_delete()),
                    move |_c| {
                        undo();
                    },
                )),
        );
    }

    /// The row [`Inner::editing`]'s doc explains — read this before treating
    /// it as dead state; it drives `body_doc`'s clobber guard.
    pub fn editing(&self) -> Signal<Option<u64>> {
        self.inner.editing.clone()
    }

    /// Record which row's body is live under a caret, or that none is.
    ///
    /// Called by `docks::footnotes::NoteBodyStyle::make_body` on every real
    /// focus change of that row's editor — so this is not merely a label for
    /// the dock to show, it is the fact `body_doc` trusts before ever
    /// overwriting an already-cached document out from under the writer.
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
    pub fn binding(&self, content: crate::singles::SingleContent) -> FootnoteBinding {
        FootnoteBinding {
            vm: self.clone(),
            content,
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
    content: crate::singles::SingleContent,
}

impl FootnoteBinding {
    /// The row this editor writes into, **if it exists yet**.
    pub fn content_id(&self) -> Option<u64> {
        self.content.id()
    }

    /// The row this editor writes into, creating it if the writer has not typed
    /// anything here yet.
    ///
    /// A `Content` is created on first write, so a chapter folder or a fresh
    /// Note that nobody has typed into has no row to anchor to. Adding a
    /// footnote is a perfectly ordinary first thing to do there, so the row is
    /// minted rather than the command refusing — the alternative is a feature
    /// that works everywhere except the empty page, which is where a writer
    /// starts.
    ///
    /// Saving an empty field is exactly what the field's own first flush would
    /// do a moment later; doing it now only moves that forward.
    pub fn ensure_content_id(&self, stack: Option<u64>) -> Option<u64> {
        if let Some(id) = self.content.id() {
            return Some(id);
        }
        if let Err(e) = self.content.save(stack) {
            eprintln!("footnotes: could not create the annotated content row: {e}");
            return None;
        }
        self.content.id()
    }

    /// The label this editor was asked to reveal, if any.
    pub fn take_seek(&self) -> Option<String> {
        self.vm.take_seek(self.content.id()?)
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

    fn vm(app_ctx: &Rc<frontend::AppContext>) -> FootnotesViewModel {
        let ids = crate::app_ids::AppIds::new();
        let docs = OpenDocsStore::new(app_ctx.clone());
        let model = FootnotesListModel::new(app_ctx.clone(), ids, docs.clone());
        FootnotesViewModel::new(model, docs, Signal::new(None))
    }

    /// A parked seek belongs to one document. Handing it to the wrong editor
    /// would put the caret in a scene the writer did not ask for — and consume
    /// the request, so the right editor would never see it.
    #[test]
    fn a_seek_is_only_taken_by_the_document_it_names() {
        let vm = vm(&Rc::new(frontend::AppContext::new()));
        vm.request_seek(11, "fn1");
        assert_eq!(vm.take_seek(22), None, "another document took it");
        assert_eq!(vm.take_seek(11).as_deref(), Some("fn1"));
        assert_eq!(vm.take_seek(11), None, "a seek is consumed once");
    }

    /// The filter is the dock's whole state, and "this document" with nothing
    /// open is a question with no answer — not a claim that there are no notes.
    #[test]
    fn this_document_with_nothing_open_shows_nothing() {
        let vm = vm(&Rc::new(frontend::AppContext::new()));
        vm.set_filter(FootnoteFilter::ThisDocument);
        assert!(vm.visible_rows(None).is_empty());
    }

    /// A body's cache is not rebuilt from `initial` while the row is live under
    /// a caret — re-syncing there would be the exact clobbered-caret bug the
    /// cache exists to prevent, traded for the disappearing-edit bug this fixes.
    #[test]
    fn body_doc_does_not_resync_a_row_that_is_being_typed_into() {
        let vm = vm(&Rc::new(frontend::AppContext::new()));
        let first = vm.body_doc(1, "first draft");
        assert_eq!(first.to_djot().unwrap(), "first draft");

        vm.set_editing(Some(1));
        let still_cached = vm.body_doc(1, "an external rewrite landed here");
        assert_eq!(
            still_cached.to_djot().unwrap(),
            "first draft",
            "a row live under a caret must not be overwritten out from under the writer"
        );
    }

    /// A body's cache **is** refreshed once the row is no longer the one being
    /// typed into — an external rewrite (Search & Replace, an Undo, a second
    /// window) must not stay invisible in the dock forever, only while a caret
    /// actually sits in that exact row.
    #[test]
    fn body_doc_resyncs_a_row_that_is_not_being_typed_into() {
        let vm = vm(&Rc::new(frontend::AppContext::new()));
        let first = vm.body_doc(1, "first draft");
        assert_eq!(first.to_djot().unwrap(), "first draft");

        // Not editing this row (nor any row) — the default state whenever the
        // writer's caret is elsewhere.
        let resynced = vm.body_doc(1, "an external rewrite landed here");
        assert_eq!(
            resynced.to_djot().unwrap(),
            "an external rewrite landed here",
            "a body changed elsewhere must reach an already-cached document"
        );
        // The SAME cached `TextDocument` was updated in place, not replaced —
        // the row's live handle (already held by a mounted editor, if any)
        // must see the new text too.
        assert_eq!(first.to_djot().unwrap(), "an external rewrite landed here");
    }

    /// Editing state clears when the row it names goes away, or the dock keeps a
    /// body box open over a note that no longer exists.
    ///
    /// Note `7` does not exist in this test's empty store, so the model's
    /// `delete` returns `None` and no toast fires — this test is purely about
    /// the `editing` cleanup, which must happen regardless. `delete` now needs
    /// a real `EventContext` to be able to raise that toast at all, so this
    /// drives it through a wired `Button` + a dispatched click, like every
    /// other `on_activate_fn`-consuming view-model method in this crate's
    /// tests (see `test_support::click`).
    #[test]
    fn deleting_the_row_being_edited_closes_the_editor() {
        use teksilo::i18n::lit;
        use teksilo::widgets::Button;

        let app_ctx = Rc::new(frontend::AppContext::new());
        let vm = vm(&app_ctx);
        vm.set_editing(Some(7));

        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        let target = vm.clone();
        let btn = tree.add(Button::new(lit!("delete")).on_activate_fn(move |ctx| {
            target.delete(ctx, 7);
        }));
        tree.layout(SizeProposal::exact(200.0, 80.0));
        crate::test_support::click(&mut tree, btn);

        assert_eq!(vm.editing().get(), None);
    }

    /// **Regression.** Deleting a note used to have no path back at all: no
    /// confirmation, and — per the grep in the finding this fixes — no
    /// reachable call to `undo_redo_commands::undo` anywhere in the footnotes
    /// feature. This drives the real `FootnotesViewModel::delete` for a note
    /// that genuinely exists in the backend, through a real `EventContext`
    /// (wire a `Button`, then click it — not a direct fn call bypassing
    /// dispatch, the same discipline `TrashViewModel`'s own toast test
    /// follows), and checks a real `ToastRegistry` actually gained a live
    /// "Undo" toast — the door back that `delete`'s own doc comment promises.
    ///
    /// Real-backend only: under `--features mocks`, `FootnotesListModel`
    /// resolves to the fabricated `imp` whose `delete` always returns `None`
    /// (there is no undo/redo stack behind a fabricated list to reverse into —
    /// see that `delete`'s own doc comment), so no toast would ever appear
    /// regardless of anything this test seeds through the real backend
    /// commands above it.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn deleting_a_real_note_offers_an_undo_toast() {
        use teksilo::i18n::lit;
        use teksilo::widgets::{Button, ToastInstallOptions, ToastRegistry};
        use frontend::commands::{footnote_commands, work_commands, work_management_commands};
        use frontend::direct_access::CreateFootnoteDto;
        use frontend::work_management::LoadWorkDto;

        let app_ctx = Rc::new(frontend::AppContext::new());
        let ids = crate::app_ids::AppIds::new();
        work_management_commands::load_work(
            &app_ctx,
            &LoadWorkDto {
                media_root: crate::media_paths::media_root_string(),
                file_name: format!(
                    "{}/../../resources/test/skribisto_test_project.skrib",
                    env!("CARGO_MANIFEST_DIR")
                ),
            },
        )
        .expect("load fixture");
        let work_id = work_commands::get_all_work(&app_ctx)
            .expect("work")
            .first()
            .expect("one work")
            .id;
        ids.seed(&app_ctx, work_id);
        ids.open_stack(&app_ctx);

        // An orphan (no `Content`) is enough here — the toast only needs a
        // real, deletable `Footnote` row; `contents_referencing`'s own
        // behaviour is covered in `models::footnotes_list_model`'s tests.
        let now = chrono::Utc::now();
        let created = footnote_commands::create_footnote(
            &app_ctx,
            ids.stack_id.get(),
            &CreateFootnoteDto {
                uid: Default::default(),
                created_at: now,
                updated_at: now,
                content: None,
                label: "fn9".into(),
                body: "About to be deleted.".into(),
            },
            work_id,
            -1,
        )
        .expect("create footnote");

        let docs = OpenDocsStore::new(app_ctx.clone());
        let model = FootnotesListModel::new(app_ctx.clone(), ids.clone(), docs.clone());
        let vm = FootnotesViewModel::new(model, docs, ids.stack_id.clone());

        let registry = ToastRegistry::new(ToastInstallOptions {
            archive: None,
            ..ToastInstallOptions::default()
        });
        let mut tree = crate::test_support::tree_with_toast_registry(&app_ctx, &registry);
        let note_id = created.id;
        let btn = tree.add(Button::new(lit!("delete")).on_activate_fn(move |ctx| {
            vm.delete(ctx, note_id);
        }));
        tree.layout(SizeProposal::exact(200.0, 80.0));
        crate::test_support::click(&mut tree, btn);

        assert_eq!(
            registry.live_count(),
            1,
            "a real delete must raise an Undo toast the writer can act on"
        );
        assert!(
            footnote_commands::get_footnote(&app_ctx, &note_id)
                .expect("get_footnote")
                .is_none(),
            "the delete itself must have actually run"
        );
    }
}
