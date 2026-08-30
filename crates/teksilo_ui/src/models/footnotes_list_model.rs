// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Under `--features mocks` the fabricated arm answers from a static fixture and
// never calls the real readers below, so every one of them is dead in that build
// and live in the other. That asymmetry is the whole reason this crate carried a
// blanket `allow(dead_code)`; scoping it to the two files that actually have it
// is what lets the rest of the crate report real rot again.
#![cfg_attr(feature = "mocks", allow(dead_code))]

//! Reactive list model over the open Work's footnotes.
//!
//! A row is one note: its label (machinery, never shown), its prose, the number it
//! prints, and where its reference sits. Two of those four are **not** stored
//! anywhere — the number and the home are recomputed from the manuscript by
//! [`crate::models::footnote_numbering`] on every refresh, because both are facts
//! about where the reference sits in the prose, and prose moves.
//!
//! Two `#[cfg]`-gated `mod imp` variants share one public surface: the real one
//! reads `Work.footnotes` and re-numbers against the live manuscript, staying
//! current on `Footnote`/`Content`/`BinderItem` events and project switches; the
//! mock one holds a fabricated list anchored to real mock content, since
//! `--features mocks` has no backend to own a created `Footnote`.
//!
//! # Rows are ordered by the book, not by the clock
//!
//! Manuscript order ([`NotePlacement::ordinal`](crate::models::NotePlacement)),
//! then orphans, then label. A writer scanning the dock is reading the notes in the
//! order a reader will meet them; creation order would scatter them.
//!
//! # Why deleting a note deletes its reference
//!
//! A `[^label]` left in the prose with no note behind it is not a recoverable
//! state: the marker still renders, and the export has nothing to print at the foot
//! of the page. So [`delete`](imp::FootnotesListModel::delete) strips every
//! reference to the note from the owning prose and removes the row **in one
//! composite undo step** — one action the writer took, one Ctrl+Z. Losing the
//! reference and keeping the note (or the reverse) is exactly the half-applied
//! state undo exists to prevent.
//!
//! The reverse gesture is not symmetric, and deliberately: deleting the *marker* in
//! the prose orphans the note rather than destroying it. The words are the writer's,
//! the format keeps them, and the dock says so — see
//! [`skribisto_model::footnote_numbering::orphaned_labels`].

/// One footnote as the dock shows it.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct FootnoteRow {
    pub id: u64,
    /// Minted by Skribisto, never typed by a writer, never shown — it is what
    /// `[^…]` in the prose names. See [`mint_label`].
    pub label: String,
    /// The note's prose, Djot.
    pub body: String,
    /// The `Content` row the reference sits in, `None` for an orphan.
    pub content_id: Option<u64>,
    /// The `BinderItem` that row belongs to, `None` for an orphan.
    pub item_id: Option<u64>,
    /// That item's title — the dock's breadcrumb. Empty for an orphan.
    pub item_title: String,
    /// What the marker prints, `None` when the note's row is not in the book (see
    /// [`crate::models::NotePlacement::number`]) or when it is orphaned.
    pub number: Option<usize>,
    /// Manuscript position. Orphans take [`usize::MAX`] so they sort last.
    pub ordinal: usize,
    pub orphaned: bool,
}

impl FootnoteRow {
    /// What the dock draws in the marker chip.
    pub fn marker(&self) -> String {
        crate::models::marker_for(self.number)
    }
}

/// A fingerprint of everything about the note set **except body text**.
///
/// The dock's list rebuilds on this rather than on every change, and the
/// distinction is load bearing: a row's body editor writes back on each
/// keystroke, which refreshes the model, which would re-render the row, which
/// would re-mint the very editor being typed into — taking the keyboard focus
/// with it after a single character. The comment margin next door carries the
/// same fingerprint for the same reason.
///
/// Ids, order, the printed number and orphan state all belong here, because each
/// changes what the dock must draw. The body does not: it only changes what is
/// *inside* a row that already exists, and the editor showing it already holds
/// the newer text.
fn structure_key(rows: &[FootnoteRow]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    for r in rows {
        r.id.hash(&mut h);
        r.label.hash(&mut h);
        r.item_id.hash(&mut h);
        r.content_id.hash(&mut h);
        r.number.hash(&mut h);
        r.ordinal.hash(&mut h);
        r.orphaned.hash(&mut h);
        r.item_title.hash(&mut h);
    }
    h.finish()
}

/// Manuscript order, orphans last, then label so equal-ordinal rows are stable.
fn sort_rows(rows: &mut [FootnoteRow]) {
    rows.sort_by(|a, b| {
        a.ordinal
            .cmp(&b.ordinal)
            .then_with(|| a.label.cmp(&b.label))
    });
}

/// The next free `fn…` label, given every label the project already uses.
///
/// Sequential rather than random so a `.skrib` opened in a text editor reads
/// sensibly, and `[^fn1]` cannot collide inside `[^fn10]` because
/// [`skribisto_model::footnote_numbering::references_in`] brackets its search on
/// both sides.
///
/// It counts past the highest existing number rather than filling gaps: reusing
/// `fn3` after its note was deleted would hand a fresh note the label a stale
/// reference elsewhere in the prose still names, and silently adopt it.
pub fn mint_label(existing: &[String]) -> String {
    let highest = existing
        .iter()
        .filter_map(|l| l.strip_prefix("fn"))
        .filter_map(|n| n.parse::<u64>().ok())
        .max()
        .unwrap_or(0);
    format!("fn{}", highest + 1)
}

/// Every occurrence of `[^label]` removed from `prose`.
///
/// Used by delete, and separate from the rest so the substring surgery is
/// unit-testable without a store behind it.
pub fn strip_references(prose: &str, label: &str) -> String {
    prose.replace(&format!("[^{label}]"), "")
}

/// The comparable signature of a set of `(item_id, field_kind, label)`
/// references — sorted, deduplicated, and encoded so the same label sitting in
/// two different homes cannot collapse into the signature it would have sitting
/// in just one.
///
/// This is the fix for the bug a bare `Vec<String>` of labels had: keying by
/// label alone, `note_live_edit`'s gate no-ops whenever the *set of distinct
/// labels* repeats — which it does at every step of an ordinary "move this
/// citation" edit (copy into the new home, then delete the original), because
/// the label is present *somewhere* the whole time. Keying by where each
/// reference sits, not just what it names, makes every step of that edit change
/// the signature. Separated out and pure so the property is unit-testable
/// without a live document behind it — `live_labels` (in `imp`) is the
/// backend-facing half that builds the input this consumes.
fn signature_of(refs: Vec<(u64, &'static str, String)>) -> Vec<String> {
    let mut out: Vec<String> = refs
        .into_iter()
        .map(|(item_id, kind, label)| format!("{item_id}:{kind}:{label}"))
        .collect();
    out.sort();
    out.dedup();
    out
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::collections::HashMap;
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{
        content_commands, footnote_commands, undo_redo_commands, work_commands,
    };
    use frontend::common::direct_access::footnote::FootnoteRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::event::{
        DirectAccessEntity, EntityEvent, Event, Origin, WorkManagementEvent,
    };
    use frontend::direct_access::{
        CreateFootnoteDto, FootnoteRelationshipDto, UpdateContentDto, UpdateFootnoteDto,
    };

    use crate::app_ids::AppIds;
    use crate::models::{OpenDocsStore, footnote_numbering};

    use super::{FootnoteRow, sort_rows, strip_references};

    struct Inner {
        model: ListModel<FootnoteRow>,
        version: Signal<u64>,
        /// Bumped only when [`structure_key`](super::structure_key) moves — what
        /// the dock rebuilds on, so typing in a body cannot re-mint its editor.
        structure: Signal<u64>,
        last_structure: Cell<u64>,
        /// Bumped only when the *set of labels a document references* changes —
        /// what [`FootnotesListModel::note_live_edit`] gates on.
        subscribed: Cell<bool>,
        ctx: Rc<AppContext>,
        ids: AppIds,
        /// The live overlay's source: a reference typed a moment ago is in an open
        /// document long before it reaches its `Content` row.
        docs: OpenDocsStore,
        /// Every label any open document currently references, as of the last
        /// refresh — the cheap comparison that keeps typing from re-reading the
        /// whole manuscript on every keystroke.
        live_labels: std::cell::RefCell<Vec<String>>,
    }

    #[derive(Clone)]
    pub struct FootnotesListModel {
        inner: Rc<Inner>,
    }

    impl FootnotesListModel {
        /// The backend handle this model was built with — for a caller that
        /// must reach the undo manager to offer one of this model's own
        /// commands back (see [`delete`](Self::delete)).
        pub fn app_ctx(&self) -> Rc<AppContext> {
            self.inner.ctx.clone()
        }

        pub fn new(ctx: Rc<AppContext>, ids: AppIds, docs: OpenDocsStore) -> Self {
            let me = Self {
                inner: Rc::new(Inner {
                    model: ListModel::from_vec(Vec::new()),
                    version: Signal::new(0),
                    structure: Signal::new(0),
                    last_structure: Cell::new(0),
                    subscribed: Cell::new(false),
                    ctx,
                    ids,
                    docs,
                    live_labels: std::cell::RefCell::new(Vec::new()),
                }),
            };
            me.refresh();
            me
        }

        /// Subscribe (once) so the list stays live.
        ///
        /// Four sources, and each of them can change an answer no other one can:
        /// a `Footnote` mutation is the obvious case; a `Content` update is a flush
        /// (or an undo, or a project-wide replace) that may have moved or removed a
        /// reference; a `BinderItem` mutation reorders the manuscript, and every
        /// number below the move changes with it; and a project switch replaces the
        /// whole set.
        ///
        /// Typing is **not** among them — see [`note_live_edit`](Self::note_live_edit).
        pub fn wire(&self, ctx: &mut BuildContext) {
            if self.inner.subscribed.replace(true) {
                return;
            }
            for ev in [
                EntityEvent::Created,
                EntityEvent::Updated,
                EntityEvent::Removed,
            ] {
                for origin in [
                    Origin::DirectAccess(DirectAccessEntity::Content(ev.clone())),
                    Origin::DirectAccess(DirectAccessEntity::BinderItem(ev.clone())),
                ] {
                    let me = self.clone();
                    ctx.subscribe_event(origin, move |_event: &Event| me.refresh());
                }
            }
            // A note appearing or vanishing changes the numbering; a note being
            // *edited* cannot. Only its body is mutable — the label is minted once
            // and never rewritten — so an `Updated` takes the cheap path. That is
            // not a micro-optimisation: the dock commits a body on every
            // keystroke, exactly as a comment card does, and a full manuscript
            // re-read per character typed is the difference between a dock and a
            // stall.
            for ev in [EntityEvent::Created, EntityEvent::Removed] {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::Footnote(ev)),
                    move |_event: &Event| me.refresh(),
                );
            }
            {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::Footnote(EntityEvent::Updated)),
                    move |_event: &Event| me.refresh_bodies(),
                );
            }
            for wev in [WorkManagementEvent::LoadWork, WorkManagementEvent::NewWork] {
                let me = self.clone();
                ctx.subscribe_event(Origin::WorkManagement(wev), move |event: &Event| {
                    if !me.inner.ids.is_bootstrap_or_own(&event.ids) {
                        return;
                    }
                    // Prefer the seeded id; fall back to the one the event carries.
                    // See `refresh_for` — the signal is still `None` at this point.
                    let work_id = me
                        .inner
                        .ids
                        .work_id
                        .get()
                        .or_else(|| event.ids.first().copied());
                    me.refresh_for(work_id);
                });
            }
            {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::WorkManagement(WorkManagementEvent::CloseWork),
                    move |event: &Event| {
                        if me.inner.ids.is_event_for_my_work(&event.ids) {
                            me.refresh_for(None)
                        }
                    },
                );
            }
            // Catch up when this window is already seeded — a rebuild, or a wire
            // that ran after the project was open.
            self.refresh();
        }

        /// An edit landed in some open document — renumber **only if a reference
        /// moved**.
        ///
        /// Called on every edit generation, so it has to be cheap in the common
        /// case, and it is: `footnote_references` is a lookup over the open
        /// documents' own anchor tables, not a re-parse and not a store read. The
        /// full pass behind it walks the whole manuscript once per label, which is
        /// fine at reference-changed cadence and ruinous at keystroke cadence.
        ///
        /// The comparison is keyed, not a bare label set — see
        /// [`live_labels`](Self::live_labels)'s own doc for why a label set alone
        /// misses a reference moving between two already-open documents.
        pub fn note_live_edit(&self) {
            let now = self.live_labels();
            if *self.inner.live_labels.borrow() == now {
                return;
            }
            self.refresh();
        }

        pub fn list_model(&self) -> ListModel<FootnoteRow> {
            self.inner.model.clone()
        }

        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        /// Bumped only when the *shape* of the list changes — see
        /// [`structure_key`](super::structure_key).
        pub fn structure_signal(&self) -> Signal<u64> {
            self.inner.structure.clone()
        }

        pub fn rows(&self) -> Vec<FootnoteRow> {
            snapshot(&self.inner.model)
        }

        pub fn len(&self) -> usize {
            self.inner.model.len()
        }

        pub fn is_empty(&self) -> bool {
            self.inner.model.len() == 0
        }

        /// The notes whose reference sits in `item_id` — the trailing dock's scope.
        pub fn rows_for_item(&self, item_id: u64) -> Vec<FootnoteRow> {
            self.rows()
                .into_iter()
                .filter(|r| r.item_id == Some(item_id))
                .collect()
        }

        pub fn orphan_count(&self) -> usize {
            self.rows().iter().filter(|r| r.orphaned).count()
        }

        /// Every label's marker, for
        /// [`TextDocument::set_footnote_markers`](teksilo::text_document::TextDocument::set_footnote_markers).
        ///
        /// The whole project's map, handed to every document rather than filtered
        /// per item: a label is unique and carries one number wherever it is cited,
        /// so a scene quoting a note first introduced two chapters earlier must draw
        /// that note's number, not fall back to printing the raw label.
        pub fn markers(&self) -> HashMap<String, String> {
            self.rows()
                .into_iter()
                .map(|r| (r.label.clone(), r.marker()))
                .collect()
        }

        /// A label no note in this project uses.
        pub fn mint_label(&self) -> String {
            let existing: Vec<String> = self.rows().into_iter().map(|r| r.label).collect();
            super::mint_label(&existing)
        }

        /// Create an empty note named `label`, annotating `content_id`.
        ///
        /// The reference itself is the caller's job: it goes into the *document*,
        /// which is the editor's business and not this model's.
        pub fn create(
            &self,
            content_id: u64,
            label: &str,
            body: &str,
            stack_id: Option<u64>,
        ) -> Option<u64> {
            let work_id = self.inner.ids.work_id.get()?;
            let now = chrono::Utc::now();
            let created = footnote_commands::create_footnote(
                &self.inner.ctx,
                stack_id,
                &CreateFootnoteDto {
                    uid: Default::default(),
                    created_at: now,
                    updated_at: now,
                    content: Some(content_id),
                    label: label.to_string(),
                    body: body.to_string(),
                },
                work_id,
                -1,
            )
            .map_err(|e| eprintln!("footnotes: create failed: {e}"))
            .ok()?;

            // The scalar the create DTO carries is not what the save path reads —
            // `hydrate_footnotes` asks for the *relationship* — so the link has to
            // be wired explicitly or the note reaches disk with no home and lands
            // in the bundle-root orphanage.
            if let Err(e) = footnote_commands::set_footnote_relationship(
                &self.inner.ctx,
                stack_id,
                &FootnoteRelationshipDto {
                    id: created.id,
                    field: FootnoteRelationshipField::Content,
                    right_ids: vec![content_id],
                },
            ) {
                eprintln!("footnotes: content wiring failed: {e}");
            }
            // Re-read now rather than waiting for the event to come back round: the
            // caller's next act is to insert the reference and feed the document its
            // markers, and a marker map that does not yet know this label prints the
            // label itself into the writer's prose.
            self.refresh();
            Some(created.id)
        }

        pub fn set_body(&self, id: u64, body: &str, stack_id: Option<u64>) {
            let Ok(Some(dto)) = footnote_commands::get_footnote(&self.inner.ctx, &id) else {
                return;
            };
            if dto.body == body {
                return;
            }
            if let Err(e) = footnote_commands::update_footnote(
                &self.inner.ctx,
                stack_id,
                &UpdateFootnoteDto {
                    // Carried through unchanged: a nil here would write over the row's durable
                    // identity on every edit, orphaning anything that references it.
                    uid: dto.uid,
                    id,
                    created_at: dto.created_at,
                    updated_at: chrono::Utc::now(),
                    label: dto.label,
                    body: body.to_string(),
                },
            ) {
                eprintln!("footnotes: body update failed: {e}");
            }
        }

        /// Remove the note **and every reference to it**, as one undo step.
        ///
        /// Open documents are flushed first and re-read after, the way
        /// `replace_in_project` handles the same problem: the prose is edited in the
        /// `Content` row, so a live editor holding unwritten text would otherwise
        /// have that text overwritten, and a live editor holding *stale* text would
        /// write the reference straight back on its next flush.
        ///
        /// Returns a closure that reverses the whole thing, or `None` when there
        /// was nothing to delete (a stale row — the id was already gone). The
        /// closure needs no captured entity data of its own: the delete above ran
        /// as one `begin_composite`/`end_composite` step, so a single
        /// `undo_redo_commands::undo` on `stack_id` pops **both** halves back —
        /// the removed `Footnote` row and every `[^label]` reference the strip
        /// took out — in one shot. `stack_id` is the only thing worth capturing,
        /// and it cannot go stale between the write and a later call: unlike the
        /// footnote and its references, it does not get deleted by this function.
        /// The caller (`FootnotesViewModel::delete`) hands the closure to a
        /// grace-window "Undo" toast — see its own doc comment for why a toast
        /// and not a confirmation dialog.
        /// Delete one note and every reference to it, as **one** undo entry.
        ///
        /// Returns that entry's sequence number, for a caller offering to take
        /// it back — not a ready-made undo closure, which could only ever have
        /// popped whatever was on top by the time it ran.
        pub fn delete(&self, id: u64, stack_id: Option<u64>) -> Option<u64> {
            let Ok(Some(dto)) = footnote_commands::get_footnote(&self.inner.ctx, &id) else {
                return None;
            };
            self.inner.docs.flush_all(stack_id);

            let touched = self.contents_referencing(&dto.label);
            let _ = undo_redo_commands::begin_composite(&self.inner.ctx, stack_id);
            for (_, content_id, stripped) in &touched {
                if let Ok(Some(c)) = content_commands::get_content(&self.inner.ctx, content_id)
                    && let Err(e) = content_commands::update_content(
                        &self.inner.ctx,
                        stack_id,
                        &UpdateContentDto {
                            // Carried through unchanged: a nil here would write over the row's durable
                            // identity on every edit, orphaning anything that references it.
                            uid: c.uid,
                            id: *content_id,
                            created_at: c.created_at,
                            updated_at: chrono::Utc::now(),
                            role: c.role,
                            data: stripped.clone(),
                            activated: c.activated,
                        },
                    )
                {
                    eprintln!("footnotes: reference removal failed: {e}");
                }
            }
            if let Err(e) = footnote_commands::remove_footnote(&self.inner.ctx, stack_id, &id) {
                eprintln!("footnotes: delete failed: {e}");
            }
            undo_redo_commands::end_composite(&self.inner.ctx);
            // Stamped the instant the group closes: `reload_open` below writes
            // nothing, but a caller that reordered these lines would otherwise
            // silently hand its toast somebody else's command.
            let seq = crate::shared::undo_toast::stamp(&self.inner.ctx);

            // Re-read the documents whose prose just changed under them, from the
            // ids captured *before* the write. The caller pumps a frame;
            // `set_djot` only queues a document event.
            let items: Vec<u64> = {
                let mut ids: Vec<u64> = touched.iter().map(|(item, _, _)| *item).collect();
                ids.sort_unstable();
                ids.dedup();
                ids
            };
            // Only the fields that genuinely moved: an unconditional reload
            // clears each document's undo history, and most of these rows are
            // untouched by a note's deletion.
            self.inner.docs.reload_if_diverged(&items);
            self.refresh();

            seq
        }

        /// Every `Content` row naming `label`, paired with its prose minus that
        /// note's references.
        /// Every `Content` naming `label`: the item that owns it, the row, and
        /// its prose with that note's references removed.
        ///
        /// The **item id comes back with it**, and that is not tidiness. The
        /// owning items used to be looked up in a second pass *after* the strip
        /// had been written — and `read_work` deliberately drops prose with no
        /// `[^` left in it, so that second pass found nothing, reloaded no
        /// document, and left the open editor showing a reference to a note that
        /// no longer existed. Its label was gone from the marker map by then, so
        /// it drew the raw `fn3`. One pass, before the write, cannot go stale.
        fn contents_referencing(&self, label: &str) -> Vec<(u64, u64, String)> {
            let Some(work_id) = self.inner.ids.work_id.get() else {
                return Vec::new();
            };
            let needle = format!("[^{label}]");
            // Read from the store, not the live overlay: the flush above has just
            // put every open document's text there, and the rows are what gets
            // written back.
            footnote_numbering::read_work(&self.inner.ctx, work_id, None)
                .into_iter()
                .flat_map(|row| {
                    let item = row.meta.id;
                    row.contents
                        .into_iter()
                        .map(move |(id, data)| (item, id, data))
                })
                .filter(|(_, _, data)| data.contains(&needle))
                .map(|(item, id, data)| (item, id, strip_references(&data, label)))
                .collect()
        }

        /// Every reference any open document currently carries, sorted — the
        /// signature `note_live_edit` compares.
        ///
        /// Keyed by **item and field**, not label alone (see
        /// [`signature_of`](super::signature_of)'s own doc for why): a bare set
        /// of distinct labels cannot see a reference *move* between two
        /// already-open documents. Copy `[^fn1]` from Scene A into Scene B —
        /// an ordinary "move this citation to where I actually discuss it"
        /// edit — and for one instant both hold it; the label set is
        /// `{fn1, …}` before, during, and after the writer then deletes the
        /// original from A, because `fn1` is present *somewhere* the whole
        /// time. A label-only comparison never fires, and the dock's cached
        /// home for that note (content/item id, set at the last full refresh)
        /// goes stale until an unrelated event happens to force a refresh.
        fn live_labels(&self) -> Vec<String> {
            let mut refs: Vec<(u64, &'static str, String)> = Vec::new();
            for item_id in self.inner.docs.open_item_ids() {
                let Some(doc) = self.inner.docs.peek(item_id) else {
                    continue;
                };
                for (kind, field) in [
                    ("main", &doc.main),
                    ("synopsis", &doc.synopsis),
                    ("epigraph", &doc.epigraph),
                ] {
                    let Some(field) = field else { continue };
                    refs.extend(
                        field
                            .doc
                            .footnote_references()
                            .into_iter()
                            .map(|(_, label)| (item_id, kind, label)),
                    );
                }
            }
            super::signature_of(refs)
        }

        /// Re-read the notes' prose only, leaving every number and placement as
        /// it was. The keystroke path — see [`wire`](Self::wire).
        fn refresh_bodies(&self) {
            let bodies: std::collections::HashMap<u64, String> = match self.inner.ids.work_id.get()
            {
                Some(work_id) => note_ids(&self.inner.ctx, work_id)
                    .into_iter()
                    .filter_map(|id| {
                        footnote_commands::get_footnote(&self.inner.ctx, &id)
                            .ok()
                            .flatten()
                    })
                    .map(|n| (n.id, n.body))
                    .collect(),
                None => return,
            };
            let mut rows = snapshot(&self.inner.model);
            let mut changed = false;
            for row in &mut rows {
                if let Some(body) = bodies.get(&row.id)
                    && *body != row.body
                {
                    row.body = body.clone();
                    changed = true;
                }
            }
            if !changed {
                return;
            }
            self.inner.model.reconcile_by_key(rows, |r| r.id);
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
            // Deliberately no structure bump: this path only ever rewrites
            // bodies, which is exactly what must not disturb the dock.
        }

        fn refresh(&self) {
            self.refresh_for(self.inner.ids.work_id.get());
        }

        /// Reload against an explicit Work rather than the `ids.work_id` signal.
        ///
        /// Same reason `CommentsListModel::refresh_for` exists: this model is wired
        /// in `App::build` *before* the lifecycle seed writes `ids.work_id`, so a
        /// `LoadWork` handler reading the signal reads `None` and the dock stays
        /// empty until the next `Footnote` entity event.
        fn refresh_for(&self, work_id: Option<u64>) {
            *self.inner.live_labels.borrow_mut() = self.live_labels();
            let rows = work_id
                .map(|id| load_rows(&self.inner.ctx, id, &self.inner.docs))
                .unwrap_or_default();
            let key = super::structure_key(&rows);
            self.inner.model.reconcile_by_key(rows, |r| r.id);
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
            if self.inner.last_structure.replace(key) != key {
                let st = &self.inner.structure;
                st.set(st.get().wrapping_add(1));
            }
        }
    }

    /// This window's own Work's notes, numbered and placed against the live
    /// manuscript — **not** `get_all_footnote`, which would merge a second
    /// simultaneously-open Work's notes into this one's dock.
    /// This Work's own note ids — `Work.footnotes`, never `get_all_footnote`.
    fn note_ids(ctx: &AppContext, work_id: u64) -> Vec<u64> {
        work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Footnotes)
            .unwrap_or_default()
    }

    fn load_rows(ctx: &AppContext, work_id: u64, docs: &OpenDocsStore) -> Vec<FootnoteRow> {
        let note_ids = note_ids(ctx, work_id);
        let notes: Vec<frontend::direct_access::FootnoteDto> =
            footnote_commands::get_footnote_multi(ctx, &note_ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .collect();
        if notes.is_empty() {
            return Vec::new();
        }
        let labels: Vec<String> = notes.iter().map(|n| n.label.clone()).collect();
        let manuscript = footnote_numbering::read_work(ctx, work_id, Some(docs));
        let places = footnote_numbering::places(&manuscript, &labels);

        let mut rows: Vec<FootnoteRow> = notes
            .into_iter()
            .map(|n| match places.placed.get(&n.label) {
                Some(p) => FootnoteRow {
                    id: n.id,
                    label: n.label,
                    body: n.body,
                    content_id: Some(p.content_id),
                    item_id: Some(p.item_id),
                    item_title: p.item_title.clone(),
                    number: p.number,
                    ordinal: p.ordinal,
                    orphaned: false,
                },
                None => FootnoteRow {
                    id: n.id,
                    label: n.label,
                    body: n.body,
                    content_id: None,
                    item_id: None,
                    item_title: String::new(),
                    number: None,
                    // Orphans sort last, whatever else is true of them.
                    ordinal: usize::MAX,
                    orphaned: true,
                },
            })
            .collect();
        sort_rows(&mut rows);
        rows
    }

    fn snapshot(model: &ListModel<FootnoteRow>) -> Vec<FootnoteRow> {
        (0..model.len())
            .filter_map(|i| model.with_item(i, |r| r.clone()))
            .collect()
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::Cell;
    use std::collections::HashMap;
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;

    use crate::app_ids::AppIds;
    use crate::models::OpenDocsStore;

    use super::{FootnoteRow, sort_rows};

    /// The binder item the fabricated notes hang off — "Scene 1", the mock binder
    /// tree's first scene under Chapter Two.
    ///
    /// A **real** row of the mock binder, whose prose is a **real** fabricated
    /// document, for the reason the comments fixture next door records in full: an
    /// invented item id files every row under "no home", and an invented content id
    /// leaves the editor with nothing to navigate to. The whole margin was dead in
    /// the mocks build the first time that fixture guessed.
    const MOCK_ITEM: u64 = 201;
    const MOCK_ITEM_TITLE: &str = "Scene 1";

    /// The `Content` row the fabricated notes annotate: `MOCK_ITEM`'s prose,
    /// derived through the very function the mock `SingleContent` uses so the two
    /// cannot drift apart.
    fn mock_scene_content() -> u64 {
        crate::singles::mock_content_id(
            MOCK_ITEM,
            &frontend::common::entities::ContentRole::SceneText,
        )
    }

    fn fabricated() -> Vec<FootnoteRow> {
        let content = mock_scene_content();
        let mut rows = vec![
            FootnoteRow {
                id: 1,
                label: "fn1".into(),
                body: "The ridgeline is the old border, abandoned after the second treaty.".into(),
                content_id: Some(content),
                item_id: Some(MOCK_ITEM),
                item_title: MOCK_ITEM_TITLE.into(),
                number: Some(1),
                ordinal: 1,
                orphaned: false,
            },
            FootnoteRow {
                id: 2,
                label: "fn2".into(),
                body: "Guild records give the name as Aleyn; the spelling here follows the \
                       parish register."
                    .into(),
                content_id: Some(content),
                item_id: Some(MOCK_ITEM),
                item_title: MOCK_ITEM_TITLE.into(),
                number: Some(2),
                ordinal: 2,
                orphaned: false,
            },
            // A deliberate orphan: the dock's badge, its empty navigation and the
            // export preflight all need one to be visible in a mocks build.
            FootnoteRow {
                id: 3,
                label: "fn3".into(),
                body: "A note whose sentence was cut. Its words are still here.".into(),
                content_id: None,
                item_id: None,
                item_title: String::new(),
                number: None,
                ordinal: usize::MAX,
                orphaned: true,
            },
        ];
        sort_rows(&mut rows);
        rows
    }

    struct Inner {
        /// Kept even though a mocks build has no backend behind it: handing a
        /// caller a *freshly minted* `AppContext` instead would be a second,
        /// permanently empty store — the trap the extension-seam rules name —
        /// and it would look like it worked.
        ctx: Rc<AppContext>,
        model: ListModel<FootnoteRow>,
        version: Signal<u64>,
        /// Mirrors the real model's: bumped on a shape change, not on a body
        /// edit, so the dock's list is not re-minted under a writer's caret.
        structure: Signal<u64>,
        next_id: Cell<u64>,
    }

    #[derive(Clone)]
    pub struct FootnotesListModel {
        inner: Rc<Inner>,
    }

    impl FootnotesListModel {
        /// The backend handle this model was built with — for a caller that
        /// must reach the undo manager to offer one of this model's own
        /// commands back (see [`delete`](Self::delete)).
        pub fn app_ctx(&self) -> Rc<AppContext> {
            self.inner.ctx.clone()
        }

        pub fn new(ctx: Rc<AppContext>, _ids: AppIds, _docs: OpenDocsStore) -> Self {
            Self {
                inner: Rc::new(Inner {
                    ctx,
                    model: ListModel::from_vec(fabricated()),
                    version: Signal::new(0),
                    structure: Signal::new(0),
                    next_id: Cell::new(4),
                }),
            }
        }

        /// Inert: a mock build has no backend to emit entity events.
        pub fn wire(&self, _ctx: &mut BuildContext) {}

        /// Inert for the same reason — there is no manuscript to renumber against.
        pub fn note_live_edit(&self) {}

        pub fn list_model(&self) -> ListModel<FootnoteRow> {
            self.inner.model.clone()
        }

        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        pub fn structure_signal(&self) -> Signal<u64> {
            self.inner.structure.clone()
        }

        pub fn rows(&self) -> Vec<FootnoteRow> {
            (0..self.inner.model.len())
                .filter_map(|i| self.inner.model.with_item(i, |r| r.clone()))
                .collect()
        }

        pub fn len(&self) -> usize {
            self.inner.model.len()
        }

        pub fn is_empty(&self) -> bool {
            self.inner.model.len() == 0
        }

        pub fn rows_for_item(&self, item_id: u64) -> Vec<FootnoteRow> {
            self.rows()
                .into_iter()
                .filter(|r| r.item_id == Some(item_id))
                .collect()
        }

        pub fn orphan_count(&self) -> usize {
            self.rows().iter().filter(|r| r.orphaned).count()
        }

        pub fn markers(&self) -> HashMap<String, String> {
            self.rows()
                .into_iter()
                .map(|r| (r.label.clone(), r.marker()))
                .collect()
        }

        pub fn mint_label(&self) -> String {
            let existing: Vec<String> = self.rows().into_iter().map(|r| r.label).collect();
            super::mint_label(&existing)
        }

        /// Fabricated create: no backend, so the list is mutated in place. The new
        /// row takes the next manuscript position, so the dock's ordering stays
        /// meaningful under mocks.
        pub fn create(
            &self,
            content_id: u64,
            label: &str,
            body: &str,
            _stack_id: Option<u64>,
        ) -> Option<u64> {
            let id = self.inner.next_id.get();
            self.inner.next_id.set(id + 1);
            let ordinal = self
                .rows()
                .iter()
                .filter(|r| !r.orphaned)
                .map(|r| r.ordinal)
                .max()
                .unwrap_or(0)
                + 1;
            let mut rows = self.rows();
            rows.push(FootnoteRow {
                id,
                label: label.to_string(),
                body: body.to_string(),
                content_id: Some(content_id),
                item_id: Some(MOCK_ITEM),
                item_title: MOCK_ITEM_TITLE.into(),
                number: Some(ordinal),
                ordinal,
                orphaned: false,
            });
            sort_rows(&mut rows);
            self.inner.model.replace_all(rows);
            self.bump();
            self.bump_structure();
            Some(id)
        }

        pub fn set_body(&self, id: u64, body: &str, _stack_id: Option<u64>) {
            let mut rows = self.rows();
            for r in &mut rows {
                if r.id == id {
                    r.body = body.to_string();
                }
            }
            self.inner.model.replace_all(rows);
            self.bump();
        }

        /// Fabricated delete. Mirrors the real `delete`'s signature so the two
        /// `imp` variants stay swappable, but a mocks build has no backend
        /// undo/redo stack behind the fabricated list — there is nothing a
        /// toast's Undo button could call, so this always returns `None`.
        /// Mock twin of the real [`delete`]: same signature, and `None` because
        /// a fabricated store records no command for a toast to name.
        pub fn delete(&self, id: u64, _stack_id: Option<u64>) -> Option<u64> {
            let keep: Vec<FootnoteRow> = self.rows().into_iter().filter(|r| r.id != id).collect();
            self.inner.model.replace_all(keep);
            self.bump();
            self.bump_structure();
            None
        }

        fn bump(&self) {
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }

        /// Only for create/delete — a body edit must leave this alone.
        fn bump_structure(&self) {
            let st = &self.inner.structure;
            st.set(st.get().wrapping_add(1));
        }
    }
}

pub use imp::FootnotesListModel;

#[cfg(test)]
mod tests {
    use super::*;

    fn row(id: u64, label: &str, ordinal: usize, orphaned: bool) -> FootnoteRow {
        FootnoteRow {
            id,
            label: label.to_string(),
            ordinal,
            orphaned,
            ..Default::default()
        }
    }

    /// Manuscript order, orphans last — a writer scanning the dock reads the notes
    /// in the order a reader will meet them.
    #[test]
    fn rows_sort_by_the_book_with_orphans_last() {
        let mut rows = vec![
            row(3, "fn3", usize::MAX, true),
            row(2, "fn2", 2, false),
            row(1, "fn1", 1, false),
        ];
        sort_rows(&mut rows);
        assert_eq!(
            rows.iter().map(|r| r.label.as_str()).collect::<Vec<_>>(),
            vec!["fn1", "fn2", "fn3"]
        );
    }

    /// Minting counts past the highest label rather than filling the gap a deleted
    /// note left: `fn3` reissued would silently adopt any stale `[^fn3]` still in
    /// the prose.
    #[test]
    fn a_minted_label_never_reuses_a_deleted_one() {
        let existing = vec!["fn1".to_string(), "fn3".to_string()];
        assert_eq!(mint_label(&existing), "fn4");
        assert_eq!(mint_label(&[]), "fn1");
    }

    /// Labels the writer never sees are still labels the project may hold from an
    /// import or a hand edit; minting must not trip over one it did not write.
    #[test]
    fn minting_ignores_labels_it_did_not_shape() {
        let existing = vec!["source".to_string(), "fn2".to_string(), "fnX".to_string()];
        assert_eq!(mint_label(&existing), "fn3");
    }

    /// Stripping is bracket-exact, so removing `fn1` cannot damage `fn10`.
    #[test]
    fn stripping_a_reference_leaves_its_longer_namesake_alone() {
        let prose = "first[^fn1] then[^fn10] end";
        assert_eq!(strip_references(prose, "fn1"), "first then[^fn10] end");
    }

    /// Every occurrence goes: one note cited twice leaves no marker behind.
    #[test]
    fn stripping_removes_every_occurrence() {
        assert_eq!(
            strip_references("a[^fn2] b[^fn2] c", "fn2"),
            "a b c",
            "a surviving marker would render with no note behind it"
        );
    }

    /// **The regression `signature_of` closes.** A reference moving between two
    /// already-open documents — copy it into its new home, *then* delete the
    /// original — leaves the deduplicated label set unchanged at every step
    /// (the label is present somewhere the whole time), so a label-only
    /// comparison never fires. Keying by `(item, field, label)` catches both
    /// steps: the label briefly gaining a second home, and then losing its
    /// first one.
    #[test]
    fn a_reference_moving_between_open_documents_changes_the_signature() {
        let scene_a = 100u64;
        let scene_b = 200u64;
        let before = signature_of(vec![(scene_a, "main", "fn1".to_string())]);

        // Copied into B; not yet deleted from A, so it briefly sits in both.
        let both = signature_of(vec![
            (scene_a, "main", "fn1".to_string()),
            (scene_b, "main", "fn1".to_string()),
        ]);
        assert_ne!(
            before, both,
            "a second home must change the signature even though the label set alone does not"
        );

        // The writer deletes the original in A, leaving only B's copy.
        let after = signature_of(vec![(scene_b, "main", "fn1".to_string())]);
        assert_ne!(
            both, after,
            "losing the first home must also change the signature"
        );
        assert_ne!(
            before, after,
            "the end state must not be mistaken for the state it started in"
        );
    }

    /// The same label in two different **fields** of one item (main prose vs.
    /// synopsis) is two distinct homes too — a label-only signature would
    /// collapse them into one, missing a citation added to the synopsis of an
    /// item whose prose already names the same note.
    #[test]
    fn the_same_label_in_two_fields_of_one_item_is_two_distinct_homes() {
        let item = 7u64;
        let one_field = signature_of(vec![(item, "main", "fn1".to_string())]);
        let two_fields = signature_of(vec![
            (item, "main", "fn1".to_string()),
            (item, "synopsis", "fn1".to_string()),
        ]);
        assert_ne!(one_field, two_fields);
    }

    /// The signature is a true set: re-adding an already-present reference (an
    /// unrelated keystroke re-triggering the scan) must not change it, or the
    /// gate would never settle.
    #[test]
    fn the_signature_is_stable_under_reordering_and_repetition() {
        let a = signature_of(vec![
            (1, "main", "fn1".to_string()),
            (2, "main", "fn2".to_string()),
        ]);
        let b = signature_of(vec![
            (2, "main", "fn2".to_string()),
            (1, "main", "fn1".to_string()),
            (1, "main", "fn1".to_string()),
        ]);
        assert_eq!(a, b);
    }

    /// A note outside the book prints a bullet, not its label — the label is
    /// machinery the writer never typed.
    #[test]
    fn an_unnumbered_row_draws_a_bullet() {
        let r = row(1, "fn1", 1, false);
        assert_eq!(r.marker(), crate::models::UNNUMBERED_MARKER);
        let numbered = FootnoteRow {
            number: Some(12),
            ..row(1, "fn1", 1, false)
        };
        assert_eq!(numbered.marker(), "12");
    }

    /// **The regression that made the dock unusable.** A row's body editor commits
    /// on every keystroke, so the model refreshes on every character. If the dock
    /// rebuilt its list on that, it would re-mint the editor being typed into and
    /// the writer would lose the caret after one letter — which is exactly what
    /// shipped. Body text is therefore outside the fingerprint the list rebuilds
    /// on; everything that changes what a row *is* stays inside it.
    #[test]
    fn typing_in_a_note_does_not_change_the_lists_shape() {
        let base = vec![row(1, "fn1", 1, false), row(2, "fn2", 2, false)];

        let mut edited = base.clone();
        edited[0].body = "the writer typed a word".into();
        assert_eq!(
            structure_key(&base),
            structure_key(&edited),
            "a body edit must not rebuild the list under the caret"
        );

        type Mutation = (&'static str, fn(&mut FootnoteRow));
        let mutations: [Mutation; 4] = [
            ("a renumbering", |r| r.number = Some(9)),
            ("losing its reference", |r| r.orphaned = true),
            ("moving in the manuscript", |r| r.ordinal = 7),
            ("changing home", |r| r.item_id = Some(42)),
        ];
        for (what, mutate) in mutations {
            let mut changed = base.clone();
            mutate(&mut changed[0]);
            assert_ne!(
                structure_key(&base),
                structure_key(&changed),
                "{what} changes what the dock must draw"
            );
        }

        let mut fewer = base.clone();
        fewer.pop();
        assert_ne!(structure_key(&base), structure_key(&fewer));
    }
}

/// `delete`'s returned closure, driven against the **real** backend rather than
/// the fabricated mocks list — the whole point is that
/// `undo_redo_commands::undo` genuinely reverses the store, which only the real
/// `imp` talks to at all (the mocks `delete` returns `None`, tested inline
/// above).
#[cfg(all(test, not(feature = "mocks")))]
mod real_backend_tests {
    use std::rc::Rc;

    use frontend::AppContext;
    use frontend::commands::{
        binder_item_commands, content_commands, footnote_commands, work_commands,
        work_management_commands,
    };
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::direct_access::footnote::FootnoteRelationshipField;
    use frontend::direct_access::{CreateFootnoteDto, FootnoteRelationshipDto, UpdateContentDto};
    use frontend::work_management::LoadWorkDto;

    use crate::app_ids::AppIds;
    use crate::models::OpenDocsStore;

    use super::FootnotesListModel;

    /// Load the crate's shared test fixture into a fresh store and open a
    /// per-Work undo stack — a real `begin_composite`/`end_composite`/`undo`
    /// round trip needs one, unlike the read-only tests elsewhere in this
    /// crate that only ever `seed` the ids.
    fn loaded(app_ctx: &Rc<AppContext>) -> AppIds {
        let ids = AppIds::new();
        work_management_commands::load_work(
            app_ctx,
            &LoadWorkDto {
                media_root: crate::media_paths::media_root_string(),
                file_name: format!(
                    "{}/../../resources/test/skribisto_test_project.skrib",
                    env!("CARGO_MANIFEST_DIR")
                ),
            },
        )
        .expect("load fixture");
        let work_id = work_commands::get_all_work(app_ctx)
            .expect("work")
            .first()
            .expect("one work")
            .id;
        ids.seed(app_ctx, work_id);
        ids.open_stack(app_ctx);
        ids
    }

    /// A `Content` row that footnote machinery will actually look at: activated,
    /// on an activated item, and the lowest such id so the answer never moves.
    ///
    /// **Not** `get_all_binder_item().find_map(..)`. That reads a `HashMap` store in
    /// whatever order it hands rows back, so the pick changed from run to run — and
    /// the shipped test fixture contains trashed rows, whose `Content` is
    /// `activated: false`. Land on one of those and the seeded `[^fn1]` sits in prose
    /// that `footnote_numbering::prose_of_item` deliberately skips, so nothing strips
    /// it and the assertion fails. The production code was right every time; the test
    /// was asking it to rewrite a trashed row roughly one run in four.
    fn live_prose_content(app_ctx: &Rc<AppContext>) -> u64 {
        let mut candidates: Vec<u64> = binder_item_commands::get_all_binder_item(app_ctx)
            .expect("items")
            .into_iter()
            .filter(|it| it.activated)
            .flat_map(|it| {
                binder_item_commands::get_binder_item_relationship(
                    app_ctx,
                    &it.id,
                    &BinderItemRelationshipField::Contents,
                )
                .unwrap_or_default()
            })
            .filter(|cid| {
                content_commands::get_content(app_ctx, cid)
                    .ok()
                    .flatten()
                    .is_some_and(|c| {
                        c.activated && c.role == frontend::common::entities::ContentRole::SceneText
                    })
            })
            .collect();
        candidates.sort_unstable();
        candidates
            .into_iter()
            .next()
            .expect("the fixture must carry at least one live scene")
    }

    /// **Regression.** Deleting a note used to be a one-way door: nothing in
    /// the UI ever reached `undo_redo_commands::undo`, so the note's body and
    /// every `[^label]` reference it carried were gone for good on one menu
    /// click. `delete` now hands back a closure that does exactly that, and
    /// this drives it against the real backend to prove it restores **both**
    /// halves — not just the `Footnote` row, which would leave a dangling
    /// `[^fn1]` with nothing behind it in the prose.
    #[test]
    fn deleting_a_note_can_be_reversed_by_the_returned_closure() {
        let app_ctx = Rc::new(AppContext::new());
        let ids = loaded(&app_ctx);
        let work_id = ids.work_id.get().expect("work open");
        let stack = ids.stack_id.get();

        let content_id = live_prose_content(&app_ctx);

        let original = content_commands::get_content(&app_ctx, &content_id)
            .expect("get content")
            .expect("content exists");
        let prose = format!("{}\n\nCited here[^fn1].\n", original.data);
        content_commands::update_content(
            &app_ctx,
            stack,
            &UpdateContentDto {
                // Carried through unchanged: a nil here would write over the row's durable
                // identity on every edit, orphaning anything that references it.
                uid: original.uid,
                id: content_id,
                created_at: original.created_at,
                updated_at: chrono::Utc::now(),
                role: original.role,
                data: prose,
                activated: original.activated,
            },
        )
        .expect("seed a reference to delete");

        let now = chrono::Utc::now();
        let created = footnote_commands::create_footnote(
            &app_ctx,
            stack,
            &CreateFootnoteDto {
                uid: Default::default(),
                created_at: now,
                updated_at: now,
                content: Some(content_id),
                label: "fn1".into(),
                body: "A note about to be deleted.".into(),
            },
            work_id,
            -1,
        )
        .expect("create footnote");
        footnote_commands::set_footnote_relationship(
            &app_ctx,
            stack,
            &FootnoteRelationshipDto {
                id: created.id,
                field: FootnoteRelationshipField::Content,
                right_ids: vec![content_id],
            },
        )
        .expect("wire content");

        let docs = OpenDocsStore::new(app_ctx.clone());
        let model = FootnotesListModel::new(app_ctx.clone(), ids.clone(), docs);

        let seq = model
            .delete(created.id, stack)
            .expect("a real delete must hand back the sequence that names it");

        assert!(
            footnote_commands::get_footnote(&app_ctx, &created.id)
                .expect("get_footnote")
                .is_none(),
            "the Footnote row must be gone right after delete"
        );
        let after_delete = content_commands::get_content(&app_ctx, &content_id)
            .expect("get content")
            .expect("content still exists");
        assert!(
            !after_delete.data.contains("[^fn1]"),
            "the reference must be stripped from the prose right after delete"
        );

        // Reversed **by name**, the way the toast now does it: `undo_if_head`
        // refuses if anything else has landed since, so this also proves the
        // sequence the model handed back really is the delete's own.
        assert_eq!(
            frontend::commands::undo_redo_commands::undo_if_head(&app_ctx, stack, seq)
                .expect("undo_if_head"),
            common::undo_redo::UndoStatus::Undone,
        );

        assert!(
            footnote_commands::get_footnote(&app_ctx, &created.id)
                .expect("get_footnote")
                .is_some(),
            "undo must restore the Footnote row"
        );
        let after_undo = content_commands::get_content(&app_ctx, &content_id)
            .expect("get content")
            .expect("content still exists");
        assert!(
            after_undo.data.contains("[^fn1]"),
            "undo must restore the stripped reference too, not just the note itself"
        );
    }

    /// A stale id (already gone, or never existed) has nothing to reverse:
    /// `None`, not a closure that would silently `undo` some unrelated *later*
    /// edit if a caller ever called it anyway.
    #[test]
    fn deleting_a_missing_note_returns_no_undo() {
        let app_ctx = Rc::new(AppContext::new());
        let ids = loaded(&app_ctx);
        let docs = OpenDocsStore::new(app_ctx.clone());
        let model = FootnotesListModel::new(app_ctx.clone(), ids.clone(), docs);
        assert!(model.delete(999_999, ids.stack_id.get()).is_none());
    }
}
