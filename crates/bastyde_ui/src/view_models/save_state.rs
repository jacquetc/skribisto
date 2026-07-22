// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SaveStateViewModel` — the save-tracking state for the open **Work**, shared by
//! every window onto it.
//!
//! ## Why this exists
//!
//! Skribisto is moving to N top-level windows over one project in one process. The
//! save operation is already global (one `.skrib` on disk, one `SaveQueue` making
//! sure at most one `save_work` runs at a time), but until this type existed the
//! *tracking* of it — the monotonic edit sequence (`dirty_seq`), the highest
//! sequence actually written (`saved_seq`), the "a write is in flight" flag
//! (`saving`), and the [`SaveQueue`] itself — was allocated **per window**
//! (`App::new` minted a fresh `dirty_seq`, `EditorsViewModel::new` a fresh
//! `saved_seq`/`saving`/queue). That is the same mistake as putting a document's
//! change count on a window controller instead of the document: AppKit's
//! `NSDocument` tracks `updateChangeCount:` once and fans it out to every window
//! controller; Qt's dirty flag lives on `QTextDocument`/`QUndoStack`, not on a
//! `QWidget`. Dirty state belongs to the document, never to a view or a window.
//!
//! With per-window state, a second window broke it outright: `SaveQueue::completed`
//! is **one-shot-consuming** (`Option::take_if`), so when the same broadcast
//! `LongOperation::Completed` event reached both windows' own queues, only the
//! window whose queue actually held that op got `Some(SaveCompleted)` back — the
//! other's `take_if` found `running` already gone (it was never `Some` there to
//! begin with, or held a different op) and returned `None`. That window's
//! `saved_seq` then never advanced again while its `dirty_seq` kept climbing off
//! the same global mutation events: permanently "unsaved", or — worse — a stale
//! `false` reported earlier that let a close proceed with real edits still
//! pending.
//!
//! The fix is this type: one instance, created once in `main`, registered as
//! `app_state`, and read/driven by every window's `App`/`EditorsViewModel` by
//! clone (cheap — `Rc`-backed) rather than by construction.
//!
//! ## Idempotency
//!
//! Sharing the object does not by itself fix the one-shot-consuming problem —
//! it *relocates* it. With one `SaveStateViewModel`, the *same* broadcast event
//! is now delivered to N windows' `App::build` subscriptions, each of which calls
//! [`Self::on_save_completed`] (or [`Self::on_save_failed`]) on the **same**
//! shared handle. The first call still finds the op `running` and consumes it;
//! every later call for that identical event must not see a used-up queue and
//! bail with `None` — every window needs the same true answer.
//!
//! So both handlers cache the outcome of the last op id they actually processed.
//! A repeat delivery for that same op id is answered from the cache — the real
//! queue transition, the `saved_seq` advance and any follow-up save are each
//! performed **exactly once**, no matter how many windows observe the event, and
//! every caller (first or Nth) gets back the identical [`SaveLanded`] /
//! error. `saved_seq` itself is additionally advanced with `max`, never a blind
//! overwrite — belt and braces against a stale or out-of-order completion ever
//! moving it backwards.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use bastyde::prelude::Signal;

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::common::event::Event;
use frontend::work_management::SaveWorkDto;

use crate::app_ids::AppIds;

use super::long_op::{event_id, parse_payload};
use super::save_queue::{SaveQueue, SaveRequest};

/// A `save_work` of ours landed — what a window needs to release whatever was
/// waiting on it (a deferred close, a parked project switch).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaveLanded {
    /// Everything mutated up to this edit sequence is now on disk.
    pub saved_seq: u64,
    /// Edits had arrived during that save, so a follow-up was due — and it could
    /// **not** be issued. Nothing further is coming: no further completion, no
    /// failure event (there is no operation to fail). Anything still parked on a
    /// sequence beyond [`Self::saved_seq`] would wait forever, so a caller must
    /// drop it and say so, exactly as it does for an outright failure.
    pub follow_up_failed: bool,
}

struct Inner {
    app_ctx: Rc<AppContext>,
    /// The Work this save state is scoped to (`AppIds::work_id`, shared with
    /// every other Tier-2 view-model on the same `WorkSession`) — what
    /// `start_save` stamps onto `SaveWorkDto.work_id`. Not a snapshot: reading
    /// it live means a `WorkSession` can never save the wrong Work, even if
    /// `ids.work_id` were ever reseeded out from under an existing session.
    ids: AppIds,
    /// Monotonic edit sequence: bumped on every mutation (typing, tree edits,
    /// metadata — see `App::mutation_origins`). A save started after a flush at
    /// seq *n* is said to *cover* n.
    dirty_seq: Signal<u64>,
    /// The highest edit sequence actually written to disk. With [`Self::dirty_seq`]
    /// this is the truth behind "unsaved" (`dirty_seq > saved_seq`), read the same
    /// way by every window.
    saved_seq: Signal<u64>,
    /// A `save_work` is in flight (or a follow-up is about to be). Drives every
    /// window's save indicator.
    saving: Signal<bool>,
    /// One `save_work` at a time, with coalescing — see [`SaveQueue`].
    queue: RefCell<SaveQueue>,
    /// The last `LongOperation::Completed` op id this object has already turned
    /// into a [`SaveLanded`], and what it produced — the idempotency cache
    /// described in the module docs. `SaveQueue::completed` is one-shot
    /// (`take_if`), so replaying this rather than calling it again is what lets
    /// every window's subscriber see the same completion, not just the first.
    last_completed: RefCell<Option<(String, SaveLanded)>>,
    /// The same idea for `LongOperation::Failed` — see [`Self::on_save_failed`].
    last_failed: RefCell<Option<(String, String)>>,
}

/// Work-scoped save-tracking state: `dirty_seq`, `saved_seq`, `saving` and the
/// [`SaveQueue`], in one place. Created once (in `main`) and registered as
/// `app_state`; every window's `App` and `EditorsViewModel` clone it rather than
/// minting their own copies — see the module docs for why a per-window copy of
/// any of this is a bug the moment a second window exists.
///
/// Cheap to clone (`Rc`-backed): every clone shares the same signals and queue.
#[derive(Clone)]
pub struct SaveStateViewModel {
    inner: Rc<Inner>,
}

impl SaveStateViewModel {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        Self {
            inner: Rc::new(Inner {
                app_ctx,
                ids,
                dirty_seq: Signal::new(0),
                saved_seq: Signal::new(0),
                saving: Signal::new(false),
                queue: RefCell::new(SaveQueue::default()),
                last_completed: RefCell::new(None),
                last_failed: RefCell::new(None),
            }),
        }
    }

    // ── Signals ──────────────────────────────────────────────────────────────

    /// The monotonic edit sequence. Bump with [`Self::bump_dirty`] on every
    /// mutation; read (with [`Self::saved_seq`]) to derive "unsaved".
    pub fn dirty_seq(&self) -> Signal<u64> {
        self.inner.dirty_seq.clone()
    }

    /// The highest edit sequence actually written to disk.
    pub fn saved_seq(&self) -> Signal<u64> {
        self.inner.saved_seq.clone()
    }

    /// A disk save is in flight.
    pub fn saving(&self) -> Signal<bool> {
        self.inner.saving.clone()
    }

    /// Whether the work holds edits not yet on disk (`dirty_seq > saved_seq`),
    /// read synchronously (no dependence on a derived-signal effect having
    /// fired yet).
    pub fn is_unsaved(&self) -> bool {
        self.inner.dirty_seq.get() > self.inner.saved_seq.get()
    }

    /// A mutation happened: this window's typing, or a tree/metadata event any
    /// window's backend call raised. Every window bumps the *same* counter, so
    /// it stays in lock-step across all of them regardless of which window's
    /// subscription actually ran it.
    pub fn bump_dirty(&self) {
        self.inner.dirty_seq.set(self.inner.dirty_seq.get() + 1);
    }

    // ── Save orchestration ───────────────────────────────────────────────────

    /// Ask for a disk write, returning the **edit sequence** the resulting save
    /// will cover — everything mutated up to that point is on disk once
    /// [`Self::saved_seq`] reaches it. The caller must flush its editors into
    /// the store *before* calling this (the sequence recorded here is only
    /// meaningful once the store actually holds everything up to it) — see
    /// `EditorsViewModel::request_save`, this type's only caller.
    ///
    /// **At most one `save_work` runs at a time** ([`SaveQueue`]): if one is
    /// already in flight (started by this window or another one sharing this
    /// object) this queues a follow-up instead of starting a second op.
    pub fn request_save(&self) -> Option<u64> {
        let covers = self.inner.dirty_seq.get();
        match self.inner.queue.borrow_mut().request(Instant::now()) {
            SaveRequest::Queued => Some(covers),
            SaveRequest::StartNow => self.start_save(covers).then_some(covers),
        }
    }

    /// Issue `save_work` and record it as the running op. `false` if the
    /// command could not be issued at all.
    ///
    /// `work_id` is read from this view-model's own `AppIds` (mirroring
    /// `SaveAsViewModel`) — the same `AppIds` shared by every other Tier-2
    /// view-model on this Work's `WorkSession` — rather than resolved via
    /// `get_all_work(ctx).next()`. The latter is only correct under the
    /// single-Work-at-a-time invariant Phase 0/1 still preserve; the moment a
    /// second `Work` is open in-process, "the first Work `get_all_work`
    /// happens to return" is not necessarily the Work this session (and the
    /// window calling `request_save`) is showing.
    fn start_save(&self, covers: u64) -> bool {
        let work_id = self.inner.ids.work_id.get().unwrap_or_default();
        match work_management_commands::save_work(
            &self.inner.app_ctx,
            &SaveWorkDto {
                work_id,
                file_name: String::new(),
                overwrite: true,
            },
        ) {
            Ok(op_id) => {
                self.inner
                    .queue
                    .borrow_mut()
                    .started(op_id, covers, Instant::now());
                self.inner.saving.set(true);
                true
            }
            Err(_) => {
                self.inner.queue.borrow_mut().start_failed();
                self.inner.saving.set(false);
                false
            }
        }
    }

    /// Route a `LongOperation::Completed`. `None` if it wasn't **our** save — a
    /// backup's, an import's or a Save As's completion is left to their own
    /// view-models.
    ///
    /// `flush` is called (at most once — see below) only if edits arrived while
    /// the completing op was running and a follow-up save must be started; the
    /// caller supplies "flush my editors into the store" (`EditorsViewModel`
    /// owns that, not this type).
    ///
    /// **Idempotent.** Every window's `App::build` subscribes to the same
    /// broadcast event and calls this on the same shared object, so this exact
    /// method may run once per window for one real completion. The first call
    /// for a given op id does the real work (advances `saved_seq`, issues the
    /// follow-up if one is due) and caches the [`SaveLanded`] it produced; every
    /// later call for that *same* op id replays the cached answer instead of
    /// re-entering [`SaveQueue::completed`] — which is one-shot-consuming and
    /// would otherwise hand every window but the first a `None`, exactly the bug
    /// this type exists to close.
    pub fn on_save_completed(&self, event: &Event, flush: impl FnOnce()) -> Option<SaveLanded> {
        let op_id = event_id(event)?;
        if let Some((last_op, landed)) = self.inner.last_completed.borrow().as_ref()
            && *last_op == op_id
        {
            return Some(*landed);
        }
        let done = self.inner.queue.borrow_mut().completed(&op_id)?;
        // Everything up to `covers` is now on disk. `max`, never a blind
        // overwrite: a stale or out-of-order event must never move this back.
        let saved_seq = self.inner.saved_seq.get().max(done.covers);
        self.inner.saved_seq.set(saved_seq);
        let mut follow_up_failed = false;
        if done.restart {
            // Edits landed while that op was in flight, and its snapshot
            // predates them. Save again, covering everything typed since —
            // exactly once, regardless of how many windows are about to ask
            // this same question for this same event.
            flush();
            let covers = self.inner.dirty_seq.get();
            follow_up_failed = !self.start_save(covers);
        } else {
            self.inner.saving.set(false);
        }
        let landed = SaveLanded {
            saved_seq: self.inner.saved_seq.get(),
            follow_up_failed,
        };
        *self.inner.last_completed.borrow_mut() = Some((op_id, landed));
        Some(landed)
    }

    /// Route a `LongOperation::Failed`. `Some(error)` if it was our save: the
    /// queue goes idle and any queued follow-up is dropped. The edits are
    /// untouched — still in the store, still dirty — so nothing is lost by not
    /// retrying behind the user's back; the caller reports it.
    ///
    /// Idempotent for the same reason as [`Self::on_save_completed`]: every
    /// window's subscriber calls this for the one broadcast `Failed` event, and
    /// every one of them must be able to report the failure — not just
    /// whichever got there first and drained [`SaveQueue::failed`].
    pub fn on_save_failed(&self, event: &Event) -> Option<String> {
        let op_id = event_id(event)?;
        if let Some((last_op, error)) = self.inner.last_failed.borrow().as_ref()
            && *last_op == op_id
        {
            return Some(error.clone());
        }
        if !self.inner.queue.borrow_mut().failed(&op_id) {
            return None;
        }
        self.inner.saving.set(false);
        let error = parse_payload(event)
            .and_then(|p| p.get("error").and_then(|e| e.as_str()).map(str::to_string))
            .unwrap_or_default();
        *self.inner.last_failed.borrow_mut() = Some((op_id, error.clone()));
        Some(error)
    }

    /// Mark everything currently in the store as "on disk" — a project just
    /// loaded, was created, or was closed: nothing is pending against *this*
    /// work.
    ///
    /// Also forgets any save still outstanding: it was a save of the
    /// **outgoing** project. Leaving it in the queue would let its completion
    /// fire the queued follow-up against the store that replaced it. The
    /// idempotency caches are cleared too — an op id from the outgoing project
    /// must not be replayed as if it were this one's answer (op ids are
    /// per-operation strings from the long-operation manager, so a collision
    /// with a future op is not realistically possible, but a new project
    /// starts this object's bookkeeping with a clean slate regardless).
    pub fn mark_clean(&self) {
        self.inner.queue.borrow_mut().reset();
        self.inner.saving.set(false);
        self.inner.saved_seq.set(self.inner.dirty_seq.get());
        *self.inner.last_completed.borrow_mut() = None;
        *self.inner.last_failed.borrow_mut() = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vm() -> SaveStateViewModel {
        SaveStateViewModel::new(Rc::new(AppContext::new()), AppIds::new())
    }

    fn completed_event(op_id: &str) -> Event {
        use frontend::common::event::{LongOperationEvent, Origin};
        Event {
            origin: Origin::LongOperation(LongOperationEvent::Completed),
            ids: Vec::new(),
            data: Some(format!(r#"{{"id":"{op_id}"}}"#)),
        }
    }

    fn failed_event(op_id: &str, error: &str) -> Event {
        use frontend::common::event::{LongOperationEvent, Origin};
        Event {
            origin: Origin::LongOperation(LongOperationEvent::Failed),
            ids: Vec::new(),
            data: Some(format!(r#"{{"id":"{op_id}","error":"{error}"}}"#)),
        }
    }

    /// Seed a "running" save directly through `SaveQueue`'s own (pure,
    /// backend-free) API, bypassing `request_save`/`start_save` — those call
    /// the real `save_work` command, which is irrelevant to what these tests
    /// exercise (the completion/failure routing once an op is already in
    /// flight). Mirrors `save_queue.rs`'s own test idiom.
    fn seed_running(vm: &SaveStateViewModel, op_id: &str, covers: u64) {
        vm.inner.queue.borrow_mut().request(Instant::now());
        vm.inner
            .queue
            .borrow_mut()
            .started(op_id.to_string(), covers, Instant::now());
        vm.inner.saving.set(true);
    }

    // ── The regression: N windows sharing one object ────────────────────────

    /// **The actual bug.** Two windows (modelled here as two clones of the same
    /// `SaveStateViewModel` — exactly how `EditorsViewModel` in window A and in
    /// window B would each hold one) both subscribe to the same broadcast
    /// `LongOperation::Completed` event and both call `on_save_completed` for
    /// it. Before the idempotency cache, the first call's `SaveQueue::completed`
    /// (a one-shot `take_if`) drains `running`, so the second call finds nothing
    /// left to take and returns `None` — the second window's `saved_seq` would
    /// then never advance for this write, while its `dirty_seq` (bumped by the
    /// same global mutation events) kept climbing: permanently "unsaved".
    ///
    /// Verified to fail without the fix: with the `last_completed` cache-check
    /// removed from `on_save_completed`, `landed_b` is `None` here.
    #[test]
    fn two_windows_sharing_one_save_state_both_observe_the_same_completion() {
        let vm = vm();
        let window_a = vm.clone();
        let window_b = vm.clone();

        seed_running(&window_a, "op-1", 1);
        let event = completed_event("op-1");

        let landed_a = window_a
            .on_save_completed(&event, || {})
            .expect("window A processes the completion");
        let landed_b = window_b.on_save_completed(&event, || {}).expect(
            "window B must ALSO see it — a one-shot queue consumption must not \
             starve every subscriber but the first",
        );

        assert_eq!(
            landed_a, landed_b,
            "every window must agree on what a save landed"
        );
        assert_eq!(landed_a.saved_seq, 1);
        assert_eq!(vm.saved_seq().get(), 1, "saved_seq actually advanced once");
    }

    /// The same idempotency guarantee for a *third* delivery (three windows),
    /// and for a follow-up save: the follow-up must be issued exactly once even
    /// though every window's subscriber runs the restart branch.
    #[test]
    fn a_follow_up_save_is_issued_exactly_once_no_matter_how_many_windows_observe_it() {
        let vm = vm();
        vm.bump_dirty(); // seq 1
        seed_running(&vm, "op-1", 1);
        vm.bump_dirty(); // seq 2, typed while op-1 was in flight
        // A second `request_save` call while op-1 is still running is what actually
        // marks a follow-up as due (`SaveQueue::request` sets `queued`) — mirroring
        // e.g. the autosave timer firing again before op-1 lands. Seeded directly
        // through the queue for the same reason `seed_running` bypasses the real
        // backend.
        vm.inner.queue.borrow_mut().request(Instant::now());

        let event = completed_event("op-1");
        let flushes = Rc::new(std::cell::Cell::new(0u32));

        for window in [vm.clone(), vm.clone(), vm.clone()] {
            let flushes = flushes.clone();
            let landed = window
                .on_save_completed(&event, move || flushes.set(flushes.get() + 1))
                .expect("every window observes the completion");
            assert_eq!(landed.saved_seq, 1, "only op-1's snapshot is on disk yet");
            assert!(!landed.follow_up_failed);
        }

        assert_eq!(
            flushes.get(),
            1,
            "the follow-up's flush must run exactly once, not once per window"
        );
    }

    /// `on_save_failed` gets the same idempotency treatment: every window must
    /// learn of the failure, but the queue transition happens once.
    #[test]
    fn on_save_failed_is_also_idempotent_across_windows() {
        let vm = vm();
        seed_running(&vm, "op-1", 1);
        let event = failed_event("op-1", "disk full");

        let window_a = vm.clone();
        let window_b = vm.clone();
        let error_a = window_a
            .on_save_failed(&event)
            .expect("window A sees the failure");
        let error_b = window_b
            .on_save_failed(&event)
            .expect("window B must ALSO see it");

        assert_eq!(error_a, error_b);
        assert_eq!(error_a, "disk full");
        assert!(!vm.saving().get());
    }

    // ── Monotonicity ─────────────────────────────────────────────────────────

    #[test]
    fn saved_seq_advances_monotonically_and_never_regresses() {
        let vm = vm();
        seed_running(&vm, "op-1", 5);
        vm.on_save_completed(&completed_event("op-1"), || {})
            .expect("our op");
        assert_eq!(vm.saved_seq().get(), 5);

        // A hypothetically stale/out-of-order completion covering an earlier
        // sequence must never move `saved_seq` backwards — belt and braces
        // alongside the idempotency cache above.
        seed_running(&vm, "op-0", 2);
        vm.on_save_completed(&completed_event("op-0"), || {})
            .expect("still a real (if stale) completion of a tracked op");
        assert_eq!(vm.saved_seq().get(), 5, "never regresses");
    }

    // ── Single-window behaviour is unchanged ────────────────────────────────

    #[test]
    fn a_foreign_op_is_ignored() {
        let vm = vm();
        seed_running(&vm, "op-1", 1);
        assert_eq!(vm.on_save_completed(&completed_event("backup-op"), || {}), None);
        assert_eq!(vm.on_save_failed(&failed_event("import-op", "x")), None);
        assert!(vm.saving().get(), "our save is untouched by someone else's op");
    }

    #[test]
    fn mark_clean_settles_the_dirty_flag_and_forgets_the_outgoing_save() {
        let vm = vm();
        vm.bump_dirty();
        vm.bump_dirty();
        seed_running(&vm, "op-old", 2);

        vm.mark_clean();

        assert!(!vm.is_unsaved(), "nothing pending against a freshly loaded work");
        assert_eq!(vm.saved_seq().get(), vm.dirty_seq().get());
        assert!(!vm.saving().get());
        // The outgoing project's completion must not resurrect anything.
        assert_eq!(vm.on_save_completed(&completed_event("op-old"), || {}), None);
    }

    #[test]
    fn bump_dirty_and_is_unsaved_track_each_other() {
        let vm = vm();
        assert!(!vm.is_unsaved());
        vm.bump_dirty();
        assert!(vm.is_unsaved());
        seed_running(&vm, "op-1", 1);
        vm.on_save_completed(&completed_event("op-1"), || {});
        assert!(!vm.is_unsaved(), "the save covered the only edit so far");
        vm.bump_dirty();
        assert!(vm.is_unsaved(), "a later edit is unsaved again");
    }
}
