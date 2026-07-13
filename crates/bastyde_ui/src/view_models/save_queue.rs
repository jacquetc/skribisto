//! `SaveQueue` — one `save_work` at a time, with coalescing.
//!
//! Three independent triggers ask for a disk save: the autosave debounce, the
//! close/exit guard, and the project-switch guard (plus Ctrl+S). Each used to call
//! `save_work` outright, and nothing serialized them — `LongOperationManager`
//! spawns a fresh background thread per call, so two saves of the same project
//! could run at once.
//!
//! That is not merely wasteful. Each op reads its own frozen snapshot of the store
//! and writes the same `.skrib` path through a temp file + atomic rename, so the
//! file is never torn — but their completion order is **unspecified**. An older
//! op's snapshot can land *after* a newer one's, silently regressing the file to
//! stale content. And since the close/switch guards wipe the store the moment
//! their save reports success, that regression is unrecoverable: the edits exist
//! in neither the file nor the store.
//!
//! So: never two at once. The naive fix — *skip* a save while one is running, as
//! [`crate::view_models::BackupSchedulerViewModel`]'s `busy()` does for backups —
//! would be wrong here. A backup is periodic and losing one tick costs nothing,
//! but the running save may have gathered the store **before** the latest flush,
//! so dropping the new request could leave the last-typed sentence in no file at
//! all. Instead the request is *queued*: when the running op lands, a follow-up
//! save runs, gathering everything typed since.
//!
//! ## Sequence numbers
//!
//! "Did this save include my edits?" is answered with a monotonic **edit
//! sequence** (`dirty_seq` in `App`, bumped by every mutation — typing, tree
//! edits, metadata). A save started after a flush at seq *n* is said to *cover*
//! *n*: when it lands, everything up to *n* is on disk. That single number drives:
//!
//!   * **the dirty flag** — `unsaved = dirty_seq > saved_seq`, which is finally
//!     truthful while a save is in flight (typing during a save used to be marked
//!     clean the moment that save — which never contained it — landed, greying out
//!     Save on edits that were on no disk anywhere);
//!   * **the deferred guards** — a close or a project switch waits for
//!     `saved_seq >= its own covered seq`, not merely for "a save finished".
//!
//! Pure and backend-free (the clock is injected), so the whole state machine is
//! unit-tested below; `EditorsViewModel` owns one and wires it to `save_work`.

use std::time::{Duration, Instant};

/// How long a `save_work` may be outstanding before the queue stops believing in
/// it. Generous — a big manuscript to a slow network destination is allowed to take
/// its time; this is a stuck-op backstop, not a deadline.
const STALE_AFTER: Duration = Duration::from_secs(120);

/// The `save_work` operation currently running.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Running {
    /// Its long-operation id — how its completion/failure event is recognised.
    op_id: String,
    /// The edit sequence this op's snapshot includes.
    covers: u64,
    /// When it was issued — see [`STALE_AFTER`].
    started: Instant,
}

/// What a caller must do about a save request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SaveRequest {
    /// Nothing is running — issue `save_work` now, then report the op id back with
    /// [`SaveQueue::started`] (or [`SaveQueue::start_failed`] if it wouldn't).
    StartNow,
    /// A save is already in flight. A follow-up will be issued when it lands, and
    /// *that* one will cover this request — so nothing to do here.
    Queued,
}

/// What a completion means for the queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SaveCompleted {
    /// The edit sequence now on disk.
    pub covers: u64,
    /// A request arrived while that op was running: issue the follow-up save now.
    pub restart: bool,
}

#[derive(Debug, Default)]
pub(crate) struct SaveQueue {
    running: Option<Running>,
    /// Set when a save is requested while one is running. The follow-up re-flushes
    /// and covers the sequence *then* current, so the value itself is only a
    /// "something was asked for" marker.
    queued: bool,
}

impl SaveQueue {
    /// Ask for a save.
    ///
    /// **Self-healing.** `running` is otherwise only cleared by a completion or
    /// failure event carrying its op id, so a single lost or unrecognised event
    /// would wedge the queue *forever*: every later request would answer `Queued`,
    /// no op would ever be issued again, and the app would silently stop writing to
    /// disk while the indicator span on. So a save still outstanding after
    /// [`STALE_AFTER`] is presumed lost and replaced by a fresh one — the next time
    /// a save is actually wanted, which is exactly when it matters.
    pub(crate) fn request(&mut self, now: Instant) -> SaveRequest {
        if let Some(r) = &self.running
            && now.duration_since(r.started) >= STALE_AFTER
        {
            // Its events are never coming. Forget it and save again — writing the
            // manuscript twice is cheap; not writing it at all is not.
            self.running = None;
            self.queued = false;
        }
        if self.running.is_some() {
            self.queued = true;
            SaveRequest::Queued
        } else {
            SaveRequest::StartNow
        }
    }

    /// A `save_work` was issued: remember it, so its completion can be told from a
    /// backup's or an import's.
    pub(crate) fn started(&mut self, op_id: String, covers: u64, now: Instant) {
        self.running = Some(Running {
            op_id,
            covers,
            started: now,
        });
    }

    /// The command could not be issued at all — nothing is running.
    pub(crate) fn start_failed(&mut self) {
        self.running = None;
        self.queued = false;
    }

    /// Forget everything: the store this queue's save was for is gone (a project was
    /// loaded, created or closed). A completion arriving for the outgoing project's
    /// op must not be mistaken for the new one's, and above all must not fire the
    /// queued follow-up — which would save the *new* project (a pointless full
    /// write) or, after a close, save an empty store and report a failure for a
    /// project the user has already left.
    pub(crate) fn reset(&mut self) {
        self.running = None;
        self.queued = false;
    }

    /// A long operation completed. `None` if it wasn't our save (a backup, an
    /// import, a Save As).
    pub(crate) fn completed(&mut self, op_id: &str) -> Option<SaveCompleted> {
        let running = self.running.take_if(|r| r.op_id == op_id)?;
        let restart = std::mem::take(&mut self.queued);
        Some(SaveCompleted {
            covers: running.covers,
            restart,
        })
    }

    /// A long operation failed. `true` if it was our save — the queue goes idle and
    /// any queued follow-up is dropped: the caller reports the failure, and the
    /// edits are still in the store (still dirty), so nothing is lost by not
    /// retrying behind the user's back.
    pub(crate) fn failed(&mut self, op_id: &str) -> bool {
        if self.running.take_if(|r| r.op_id == op_id).is_none() {
            return false;
        }
        self.queued = false;
        true
    }

    /// Is a `save_work` in flight? (The live signal callers read is
    /// `EditorsViewModel::saving`; this is the queue's own view of it, for tests.)
    #[cfg(test)]
    pub(crate) fn is_running(&self) -> bool {
        self.running.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t0() -> Instant {
        Instant::now()
    }

    #[test]
    fn an_idle_queue_starts_the_save_itself() {
        let t = t0();
        let mut q = SaveQueue::default();
        assert_eq!(q.request(t), SaveRequest::StartNow);
        assert!(!q.is_running(), "not running until the command is issued");
        q.started("op-1".into(), 1, t);
        assert!(q.is_running());
    }

    #[test]
    fn a_second_request_never_starts_a_second_save() {
        // The whole point: two concurrent `save_work` ops write the same path with
        // unspecified completion order, so an older snapshot can land last.
        let t = t0();
        let mut q = SaveQueue::default();
        assert_eq!(q.request(t), SaveRequest::StartNow);
        q.started("op-1".into(), 1, t);
        assert_eq!(q.request(t), SaveRequest::Queued);
        assert_eq!(q.request(t), SaveRequest::Queued);
    }

    #[test]
    fn edits_made_during_a_save_get_a_follow_up_save() {
        // Skipping (the backup scheduler's `busy()` answer) would lose them: the
        // running op may have gathered the store before they were flushed.
        let t = t0();
        let mut q = SaveQueue::default();
        q.request(t);
        q.started("op-1".into(), 1, t);
        q.request(t); // typed while op-1 was in flight

        let done = q.completed("op-1").expect("our op");
        assert_eq!(done.covers, 1, "only what op-1's snapshot held is on disk");
        assert!(done.restart, "the edits at seq 2 still need a save");
        assert!(!q.is_running());
    }

    #[test]
    fn a_quiet_save_needs_no_follow_up() {
        let t = t0();
        let mut q = SaveQueue::default();
        q.request(t);
        q.started("op-1".into(), 4, t);
        let done = q.completed("op-1").expect("our op");
        assert_eq!(done.covers, 4);
        assert!(!done.restart);
    }

    #[test]
    fn a_foreign_op_is_ignored() {
        // Backups, imports and Save As all emit the same LongOperation events.
        let t = t0();
        let mut q = SaveQueue::default();
        q.request(t);
        q.started("op-1".into(), 1, t);
        assert_eq!(q.completed("backup-op"), None);
        assert!(!q.failed("import-op"));
        assert!(q.is_running(), "our save is untouched by someone else's op");
    }

    #[test]
    fn a_failed_save_goes_idle_and_drops_the_follow_up() {
        let t = t0();
        let mut q = SaveQueue::default();
        q.request(t);
        q.started("op-1".into(), 1, t);
        q.request(t);
        assert!(q.failed("op-1"));
        assert!(!q.is_running());
        // The next request starts fresh rather than silently riding a dead op.
        assert_eq!(q.request(t), SaveRequest::StartNow);
    }

    #[test]
    fn a_command_that_would_not_issue_leaves_the_queue_idle() {
        let t = t0();
        let mut q = SaveQueue::default();
        assert_eq!(q.request(t), SaveRequest::StartNow);
        q.start_failed();
        assert!(!q.is_running());
        assert_eq!(q.request(t), SaveRequest::StartNow, "not stuck 'running'");
    }

    #[test]
    fn an_op_whose_events_never_arrive_does_not_wedge_saving_forever() {
        // `running` is otherwise only cleared by an event carrying its op id, so one
        // lost/unrecognised completion would make every later request answer
        // `Queued` — no op ever issued again, the app silently stops writing to disk.
        let t = t0();
        let mut q = SaveQueue::default();
        q.request(t);
        q.started("lost-op".into(), 1, t);

        // Still believed in while it might yet land.
        assert_eq!(
            q.request(t + STALE_AFTER - Duration::from_secs(1)),
            SaveRequest::Queued
        );

        // Past the backstop: presumed lost, and the save actually happens.
        assert_eq!(q.request(t + STALE_AFTER), SaveRequest::StartNow);
        assert!(!q.is_running(), "the lost op was forgotten");

        // …and the abandoned op's completion, if it ever does turn up, is nobody's.
        assert_eq!(q.completed("lost-op"), None);
    }

    #[test]
    fn reset_forgets_the_outgoing_projects_save() {
        // On load/new/close the store is no longer the one that save was for. Its
        // completion must not fire the queued follow-up — that would save the *new*
        // project, or (after a close) an empty store, and report a failure for a
        // project the user has already left.
        let t = t0();
        let mut q = SaveQueue::default();
        q.request(t);
        q.started("op-old".into(), 1, t);
        q.request(t); // an edit landed mid-save, so a follow-up is queued

        q.reset();

        assert!(!q.is_running());
        assert_eq!(q.completed("op-old"), None, "not ours any more");
        assert!(!q.failed("op-old"));
        assert_eq!(
            q.request(t),
            SaveRequest::StartNow,
            "the new project saves fresh"
        );
    }
}
