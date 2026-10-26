// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `QuitSequencer` — Ctrl+Q with several projects open.
//!
//! Accounts for **every** open Work in turn (not just this window's), then
//! closes **every** window:
//!
//! 1. Snapshot the open Works ([`WorkRegistry::open_work_ids`]) — once, at the
//!    start. A window opened while the sequence is running is deliberately not
//!    swept: the user asked to quit the app they could see.
//! 2. For each, apply the app's one shared branch order
//!    ([`crate::project::unsaved_decision`]) — the same one `work.close`, the
//!    window close guard and the four in-place switch doors use. Nothing here
//!    invents a second opinion about what unsaved edits deserve.
//! 3. **Discard skips the on-close backup** (and never flushes dirty editors
//!    first). Same rule as Close Work's Discard: the last-saved state is what
//!    is kept. Routing Discard through `on_close_flow` would flush those
//!    discarded buffers into the store and then write them into a backup — the
//!    opposite of discarding.
//! 4. **Cancel anywhere aborts the whole quit** and leaves every window open,
//!    including ones already dealt with. A quit is one decision, not a run of
//!    independent ones; discarding project A's edits and then stopping at project
//!    B would leave A destroyed for a quit that never happened.
//! 5. When the queue drains, close every window. teksilo exits once its window map
//!    is empty (`maybe_exit`), so there is no separate "terminate" step.
//!
//! ## A project import still running
//!
//! Before any of that, a project import that is still running is asked about:
//! the process exiting under it would kill the converter mid-write, leaving its
//! half-written file beside the target. *Stop the import and quit* cancels it and
//! waits for the import's own end (`ProjectImportJob::stop_then`), by which time
//! the converter has removed what it wrote; only then are the Works walked.
//! *Cancel* keeps both the import and every window. The job is Tier 1, so it is
//! found through `app_state`, the one tier that slot is right for.
//!
//! ## Why the prompt names the project
//!
//! Each prompt says which project it is about rather than focusing that project's
//! window first. Focusing another window and then raising a modal in *this* one is
//! the worst of both — the user's attention moves to a window that is not asking
//! the question. The Work's own title in the dialog answers "which one?" without
//! moving anything.
//!
//! ## Waiting for a save
//!
//! `save_work` is a long operation, so "Save, then continue" cannot be answered
//! inline. The sequencer waits on the **edit sequence** the save covers
//! (`saved_seq >= covers`), never on "a save finished" — `SaveQueue` coalesces, so
//! the op that finally carries those edits may be a follow-up, and an earlier op's
//! snapshot can predate the flush. This is the same contract the single-window
//! deferred close already uses.
//!
//! It is driven from `App::build`'s `LongOperation::Completed` subscriber, which is
//! the only place in the crate holding both an `EventContext` and that event. The
//! sequencer routes the event into the *waiting* Work's own
//! `SaveStateViewModel::on_save_completed` — documented idempotent, and already
//! called once per window showing that Work — precisely so it does not depend on
//! whether some other window's subscriber happened to run first.
//!
//! ## A quit someone else asks for
//!
//! teksilo also quits on requests that come from outside the app's own Quit
//! command, and asks every window's close guard as it goes, naming the source
//! (`EventContext::quit_source`). Two kinds reach a guard:
//!
//! * **A quit that can be answered**: the macOS Dock menu's Quit, AppleScript's
//!   `quit`, any other `terminate:`, a Windows `taskkill` reaching the app while
//!   windows are open. The guard hands it to this sequence
//!   ([`QuitSequencer::defer_close_to_quit`]), so the writer gets the same
//!   questions as for Work ▸ Quit, and holds its window's close on a
//!   `CloseDecision` until the sequence ends: [`finish`](QuitSequencer::finish)
//!   proceeds it, a Cancel anywhere cancels it, and teksilo tells the system how
//!   the quit went (a cancelled one stays cancelled, on macOS with AppKit told
//!   so).
//! * **A quit nobody can answer**: `SIGTERM` or `SIGHUP`, the session ending
//!   (log out, restart, shut down). teksilo closes the window whatever the guard
//!   says, so a question would never be seen. The guard saves instead
//!   ([`QuitSequencer::close_without_asking`]) and lets the window close.
//!
//! On macOS, AppKit ends the process itself once such a quit completes, so the
//! teardown `run` does after the event loop does not happen then. Nothing in it
//! is lost: teksilo writes every settings file on the way out, the open-registry
//! locks and the IPC sockets are reaped by the next launch, the backend's
//! `clean_up_before_exit` only empties a store that dies with the process, and a
//! running import has been stopped already, by this sequence's question or by
//! the guard that saved without asking.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Duration;

use frontend::commands::work_management_commands;
use frontend::work_management::CloseWorkDto;
use frontend::{AppContext, Event};
use teksilo::prelude::*;
use teksilo::widgets::{MessageBox, MessageBoxButton, MessageBoxButtons, StandardButton};

use crate::app::PendingExit;
use crate::project::{UnsavedDecision, unsaved_decision};
use crate::sessions::{WorkRegistry, WorkSession};
use crate::shared::project_import::ProjectImportJob;

/// What the sequencer does with one Work — the pure half, so the ordering and
/// cancellation rules are unit-testable without a widget tree.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum QuitStep {
    /// Nothing unsaved: move straight on.
    Skip,
    /// Autosave is on — the user already said "just save". Save, no prompt.
    SaveSilently,
    /// Ask Save / Discard / Cancel.
    AskSaveDiscardCancel,
    /// A backup file: read-only, so there is no Save to offer.
    AskDiscardCancel,
}

/// Map the app's shared unsaved-changes verdict onto a quit step.
///
/// A thin translation, deliberately: the *decision* stays in
/// [`unsaved_decision`] so a quit can never disagree with a close about what
/// unsaved edits are worth.
pub fn step_for(unsaved: bool, backup_mode: bool, autosave: bool) -> QuitStep {
    match unsaved_decision(unsaved, backup_mode, autosave) {
        UnsavedDecision::Proceed => QuitStep::Skip,
        UnsavedDecision::SaveThenProceed => QuitStep::SaveSilently,
        UnsavedDecision::PromptSaveDiscardCancel => QuitStep::AskSaveDiscardCancel,
        UnsavedDecision::PromptDiscardOnly => QuitStep::AskDiscardCancel,
    }
}

struct Inner {
    app_ctx: Rc<AppContext>,
    registry: WorkRegistry,
    /// The master autosave switch — read per Work, at the moment that Work is
    /// considered, not snapshotted at `begin`.
    autosave: Signal<bool>,
    /// Works still to account for. Empty when idle.
    queue: RefCell<VecDeque<u64>>,
    /// A quit is in progress. Guards against Ctrl+Q pressed twice, which would
    /// otherwise start a second sequence over the same Works and prompt twice
    /// for each.
    active: Cell<bool>,
    /// The Work whose save we are waiting on, and the edit sequence that save
    /// covers. `None` unless a `Save` answer is outstanding.
    waiting: RefCell<Option<(u64, u64)>>,
    /// How many extra saves the Work being waited on has already been asked for
    /// because typing went on during the previous one.
    resaves: Cell<u8>,
    /// The closes held for this sequence: windows a quit teksilo runs (the macOS
    /// Dock's Quit, a `terminate:`) asked while it was under way. Proceeded when
    /// the sequence finishes, cancelled when it is abandoned, so the system hears
    /// how the quit ended. See the module doc.
    deferred: RefCell<Vec<CloseDecision>>,
}

/// How many extra saves one Work may be asked for during a quit.
const MAX_RESAVES: u8 = 3;

/// How long a quit nobody can answer may spend writing one Work to disk.
///
/// A service manager allows 90 seconds by default between `SIGTERM` and
/// `SIGKILL`; Windows lets a session end within a few seconds and asks no
/// application's permission. An interrupted save loses nothing (every save
/// replaces the file atomically), so this only bounds a save that hangs.
const EXIT_SAVE_BUDGET: Duration = Duration::from_secs(30);

/// App-global (Tier 1): one quit at a time, over every open Work.
#[derive(Clone)]
pub struct QuitSequencer {
    inner: Rc<Inner>,
}

impl QuitSequencer {
    pub fn new(app_ctx: Rc<AppContext>, registry: WorkRegistry, autosave: Signal<bool>) -> Self {
        Self {
            inner: Rc::new(Inner {
                app_ctx,
                registry,
                autosave,
                queue: RefCell::new(VecDeque::new()),
                active: Cell::new(false),
                waiting: RefCell::new(None),
                resaves: Cell::new(0),
                deferred: RefCell::new(Vec::new()),
            }),
        }
    }

    /// Is a quit currently running? (Read by the Quit command so a second
    /// Ctrl+Q is a no-op rather than a second sequence.)
    pub fn is_active(&self) -> bool {
        self.inner.active.get()
    }

    /// Start quitting. No-op if one is already under way. A project import still
    /// running is asked about first (see the module doc).
    pub fn begin(&self, ctx: &mut EventContext) {
        if self.inner.active.get() {
            return;
        }
        if let Some(job) = ctx.app_state::<ProjectImportJob>().cloned()
            && job.is_running()
        {
            return self.ask_about_the_import(ctx, job);
        }
        self.walk(ctx);
    }

    /// A quit teksilo runs, one that can be answered (`QuitSource::Application`
    /// or `UserQuit`: the macOS Dock's Quit, AppleScript, a `terminate:`), has
    /// reached a window's close guard. Answer it the way Work ▸ Quit is answered:
    /// start this sequence (or join the one under way) and hold the window's close
    /// until it ends. The guard returns what this returns.
    ///
    /// The sequence closes every window itself when it finishes, so the decision
    /// it proceeds then only tells teksilo the quit went through; cancelling it is
    /// what keeps the quit, and every window, from going on.
    pub fn defer_close_to_quit(&self, ctx: &mut EventContext) -> CloseResponse {
        let decision = CloseDecision::new();
        self.inner.deferred.borrow_mut().push(decision.clone());
        // Nothing to do when a sequence is already under way: its end answers
        // this decision too.
        self.begin(ctx);
        CloseResponse::Defer(decision)
    }

    /// A quit nobody can answer (`QuitSource::Signal`, `SessionEnd`, or a source
    /// teksilo adds later) is closing a window, the one showing `session`'s Work
    /// or the Launcher: keep what the writer did, without a question, and let the
    /// window go. The guard returns what this returns.
    ///
    /// Saves the Work unless it holds nothing unsaved, or is a backup open
    /// read-only (there is no file to write it to, and no one to ask for one; its
    /// edits go, as a Discard would). Called for every window teksilo closes, so a
    /// Work with two windows is written once: the second finds it saved. The
    /// desk (open tabs and docks) is captured for every Work, as
    /// [`finish`](Self::finish) captures it.
    ///
    /// Saving here does not wait for the autosave setting: with autosave off the
    /// writer would have been asked, and a session that ends leaves nobody to
    /// ask. Keeping the edits is the answer that can still be undone.
    ///
    /// A project import still running is stopped and waited for, as `run`'s
    /// teardown would: on macOS a logout ends the process before that teardown
    /// runs, and the converter would die mid-write.
    pub fn close_without_asking(
        &self,
        ctx: &EventContext,
        session: Option<&WorkSession>,
    ) -> CloseResponse {
        if let Some(session) = session {
            session.workspace_layout.capture();
            session.workspace_layout.capture_tree_expansion();
            if session.save_state.is_unsaved() && !session.backup_mode.get() {
                // A buffer that cannot be flushed is reported by the flush
                // itself; what the store holds is still written.
                session.backup_scheduler.flush_all_windows();
                if !session.save_state.save_before_exit(EXIT_SAVE_BUDGET) {
                    eprintln!(
                        "{}: quitting without a chance to ask, and the project could not be saved",
                        session.single_work.title().get()
                    );
                }
            }
        }
        if let Some(job) = ctx.app_state::<ProjectImportJob>()
            && !job.stop_at_exit(crate::shared::project_import::EXIT_PATIENCE)
        {
            eprintln!("an import was still writing when Skribisto closed; it did not stop in time");
        }
        CloseResponse::Close
    }

    /// Answer every close held by [`Self::defer_close_to_quit`]: `proceed` once
    /// the sequence has finished, cancel when it was abandoned.
    fn settle_deferred(&self, proceed: bool) {
        for decision in self.inner.deferred.borrow_mut().drain(..) {
            if proceed {
                decision.proceed();
            } else {
                decision.cancel();
            }
        }
    }

    /// A project import is still running: ask whether to stop it and quit. See
    /// the module doc.
    ///
    /// The quit counts as under way while the question is up, so a second Ctrl+Q
    /// (from another window) does not ask it twice; Cancel sets it back.
    fn ask_about_the_import(&self, ctx: &mut EventContext, job: ProjectImportJob) {
        self.inner.active.set(true);
        let me = self.clone();
        let name = job.running_target_name().unwrap_or_default();
        MessageBox::question(tr!(quit_import_running_title()))
            .text(tr!(quit_import_running_text(name = name)))
            .buttons(MessageBoxButtons::Custom(vec![
                MessageBoxButton::standard(StandardButton::Discard).label(tr!(quit_import_stop())),
                MessageBoxButton::standard(StandardButton::Cancel),
            ]))
            .default_button(StandardButton::Cancel)
            .escape_button(StandardButton::Cancel)
            .on_result(move |r, c| match r.button {
                StandardButton::Discard => {
                    let me = me.clone();
                    job.stop_then(c, move |c| me.walk(c));
                }
                // Cancel, the escape key and closing the dialog keep the import
                // running and every window open.
                _ => {
                    me.inner.active.set(false);
                    me.settle_deferred(false);
                }
            })
            .present(ctx);
    }

    /// Account for every open Work, then close every window (see the module
    /// doc): the quit itself, once nothing else stands in its way.
    ///
    /// The queue is sorted by `work_id` purely so the order is *deterministic* —
    /// `open_work_ids` reads a `HashMap`, and a quit that prompted for projects
    /// in a different order each time would be impossible to script or to test.
    /// Ascending `work_id` is open order in practice, which is also the least
    /// surprising order to be asked in.
    fn walk(&self, ctx: &mut EventContext) {
        let mut ids = self.inner.registry.open_work_ids();
        ids.sort_unstable();
        self.inner.active.set(true);
        *self.inner.queue.borrow_mut() = ids.into();
        self.advance(ctx);
    }

    /// Work through the queue until something needs the user, a save is
    /// outstanding, or nothing is left.
    fn advance(&self, ctx: &mut EventContext) {
        loop {
            let Some(work_id) = self.inner.queue.borrow_mut().pop_front() else {
                return self.finish(ctx);
            };
            // A Work that closed while we were asking about another one needs
            // no account: it is already gone.
            let Some(session) = self.inner.registry.session_for(work_id) else {
                continue;
            };
            // The Work's own answer, not the pair's: a change an extension
            // reported from another thread counts (see
            // `SaveStateViewModel::poll_external`).
            let step = step_for(
                session.save_state.is_unsaved(),
                session.backup_mode.get(),
                self.inner.autosave.get(),
            );
            match step {
                QuitStep::Skip => return self.back_up_then_continue(ctx, &session),
                QuitStep::SaveSilently => {
                    if self.start_save(work_id, &session) {
                        return; // resumes from `on_long_op_completed`
                    }
                    // The save could not even be issued. Stopping is the only
                    // safe answer: continuing would close a window over edits
                    // that were never written, having promised to write them.
                    return self.abort();
                }
                QuitStep::AskSaveDiscardCancel => {
                    return self.ask_save_discard_cancel(ctx, work_id, &session);
                }
                QuitStep::AskDiscardCancel => {
                    return self.ask_discard_cancel(ctx, work_id, &session);
                }
            }
        }
    }

    /// Prompt for a Work whose edits can still be written.
    fn ask_save_discard_cancel(&self, ctx: &mut EventContext, work_id: u64, session: &WorkSession) {
        let me = self.clone();
        let session = session.clone();
        MessageBox::question(tr!(quit_save_work_question(
            title = session.single_work.title().get()
        )))
        .text(tr!(unsaved_changes()))
        .buttons(MessageBoxButtons::SaveDiscardCancel)
        .default_button(StandardButton::Save)
        .escape_button(StandardButton::Cancel)
        .on_result(move |r, c| match r.button {
            StandardButton::Save => {
                if me.start_save(work_id, &session) {
                    return; // resumes from `on_long_op_completed`
                }
                me.abort();
            }
            StandardButton::Discard => me.discard_and_continue(c),
            // Cancel — and the escape key, and closing the dialog — abort the
            // whole quit, not just this Work. See the module doc.
            _ => me.abort(),
        })
        .present(ctx);
    }

    /// Prompt for a Work open in backup mode: the file is read-only, so Save is
    /// not on offer (Save As and Restore, both on the banner, are how those edits
    /// are kept).
    fn ask_discard_cancel(&self, ctx: &mut EventContext, _work_id: u64, session: &WorkSession) {
        let me = self.clone();
        let session = session.clone();
        MessageBox::question(tr!(quit_backup_discard_work_question(
            title = session.single_work.title().get()
        )))
        .text(tr!(quit_backup_discard_text()))
        .buttons(MessageBoxButtons::Custom(vec![
            MessageBoxButton::standard(StandardButton::Discard),
            MessageBoxButton::standard(StandardButton::Cancel),
        ]))
        .default_button(StandardButton::Cancel)
        .escape_button(StandardButton::Cancel)
        .on_result(move |r, c| match r.button {
            StandardButton::Discard => me.discard_and_continue(c),
            _ => me.abort(),
        })
        .present(ctx);
    }

    /// Discard unsaved edits for this Work and move on — **no** flush, **no**
    /// on-close backup.
    ///
    /// Parity with Close Work's Discard (which skips `on_close_flow`): the
    /// last-saved state is what is kept. Calling [`Self::back_up_then_continue`]
    /// here would flush dirty editor buffers into the store and then write them
    /// into a backup, which is the opposite of discarding.
    fn discard_and_continue(&self, ctx: &mut EventContext) {
        self.advance(ctx);
    }

    /// This Work is accounted for **without discarding edits** (clean, or just
    /// saved). Take its **on-close backup** if the project's policy asks for one,
    /// then carry on with the next Work.
    ///
    /// Quitting is a close, and "Back up when closing" has always applied to it —
    /// but only when the Work's current state is the one the user is keeping.
    /// Discard uses [`Self::discard_and_continue`] instead.
    ///
    /// Reusing `on_close_flow` rather than re-deriving the rules keeps every one
    /// of them — backup mode suppresses it, an already-running backup is attached
    /// to instead of a second being started, and an unreachable destination
    /// *blocks* the exit with a Retry prompt rather than quietly skipping the
    /// backup the user asked for.
    ///
    /// It is asynchronous (a reachability probe off the UI thread, then the
    /// backup itself), so control comes back through `do_close`'s
    /// `PendingExit::Quit` arm → [`Self::on_backup_done`]. When no backup applies,
    /// `on_close_flow` calls that immediately and this is effectively a
    /// straight-through step.
    fn back_up_then_continue(&self, ctx: &mut EventContext, session: &WorkSession) {
        session
            .backup_scheduler
            .on_close_flow(ctx, PendingExit::Quit);
    }

    /// One Work's on-close backup finished (or did not apply): move on.
    ///
    /// Reached from `BackupSchedulerViewModel::do_close`. If the user cancelled
    /// the quit while the backup was running, `active` is already false and this
    /// must not resurrect it.
    pub fn on_backup_done(&self, ctx: &mut EventContext) {
        if !self.inner.active.get() {
            return;
        }
        self.advance(ctx);
    }

    /// Flush `work_id`'s live editor buffers into the store and ask for a disk
    /// write. Returns whether a save is now outstanding.
    ///
    /// The flush is not optional: `request_save` records the edit sequence the
    /// write will cover, and that sequence only means anything once the store
    /// actually holds everything up to it. Text still sitting in an editor would
    /// otherwise be counted as saved and then closed away.
    fn start_save(&self, work_id: u64, session: &WorkSession) -> bool {
        session.backup_scheduler.flush_all_windows();
        let Some(covers) = session.save_state.request_save() else {
            return false;
        };
        *self.inner.waiting.borrow_mut() = Some((work_id, covers));
        true
    }

    /// Route a `LongOperation::Completed` while a quit is waiting on a save.
    ///
    /// Called from `App::build`'s completion subscriber — the crate's only place
    /// holding both this event and an `EventContext`. Every window calls it on the
    /// same shared sequencer, which is safe for the same reason
    /// `SaveStateViewModel::on_save_completed` is: that method is idempotent per
    /// op id, and routing the event through it *here* is what guarantees the
    /// waiting Work's `saved_seq` is up to date before it is read — rather than
    /// depending on whether some other window's subscriber ran first.
    pub fn on_long_op_completed(&self, event: &Event, ctx: &mut EventContext) {
        let Some((work_id, covers)) = *self.inner.waiting.borrow() else {
            return;
        };
        let Some(session) = self.inner.registry.session_for(work_id) else {
            // The Work vanished mid-save. Nothing left to protect.
            self.inner.waiting.borrow_mut().take();
            return self.advance(ctx);
        };
        let flush_session = session.clone();
        session.save_state.on_save_completed(event, move || {
            flush_session.backup_scheduler.flush_all_windows();
            true
        });
        if session.save_state.saved_seq().get() >= covers {
            // Text typed while that save was writing is not in it: one more
            // write, unless the flush itself failed (it would again) or this
            // Work has already been asked a few times.
            if session.save_state.is_unsaved()
                && !session.save_state.last_flush_failed()
                && self.inner.resaves.get() < MAX_RESAVES
            {
                self.inner.resaves.set(self.inner.resaves.get() + 1);
                if self.start_save(work_id, &session) {
                    return;
                }
            }
            self.inner.waiting.borrow_mut().take();
            self.inner.resaves.set(0);
            // Saved and consistent — now this Work's on-close backup, then the
            // next Work. Same order the single-window deferred close uses: the
            // backup must capture the saved state, not the state before it.
            self.back_up_then_continue(ctx, &session);
        }
    }

    /// Route a `LongOperation::Failed` while a quit is waiting on a save.
    ///
    /// A write that did not happen must never be followed by closing the window
    /// over it. Abort, leaving every window open and every edit intact; the
    /// failure itself is reported by the existing save-failure toast, which does
    /// not need a second voice here.
    pub fn on_long_op_failed(&self, event: &Event, _ctx: &mut EventContext) {
        let Some((work_id, _)) = *self.inner.waiting.borrow() else {
            return;
        };
        let Some(session) = self.inner.registry.session_for(work_id) else {
            return;
        };
        if session.save_state.on_save_failed(event).is_some() {
            self.abort();
        }
    }

    /// Give up on the whole quit: clear the queue and let every window stay.
    fn abort(&self) {
        self.inner.queue.borrow_mut().clear();
        self.inner.waiting.borrow_mut().take();
        self.inner.resaves.set(0);
        self.inner.active.set(false);
        self.settle_deferred(false);
    }

    /// Every Work is accounted for: close each Work's backend subtree, then every
    /// window. teksilo's `maybe_exit` ends the process once none are left, so
    /// there is nothing to call after this.
    ///
    /// The desk is captured *before* `close_work` — that call tears the Work
    /// subtree out before publishing `CloseWork`, after which a tab can no longer
    /// be translated into anything persistable.
    ///
    /// **Closing windows.** Do not rely on `ctx.windows()` alone. During event
    /// dispatch teksilo pulls the *current* window out of the window map, so
    /// `windows()` omits the tree this handler is running on. A single-window
    /// quit that only iterated `windows()` would close every Work and leave an
    /// empty zombie project window — process never exits, looks like a broken
    /// Close Work. Same shape as [`crate::app::close_work_and_return_to_launcher`]:
    /// force-close siblings by id, force-close the current tree via the
    /// post-dispatch flag.
    fn finish(&self, ctx: &mut EventContext) {
        for work_id in self.inner.registry.open_work_ids() {
            if let Some(session) = self.inner.registry.session_for(work_id) {
                session.workspace_layout.capture();
                session.workspace_layout.capture_tree_expansion();
                let _ = work_management_commands::close_work(
                    &self.inner.app_ctx,
                    &CloseWorkDto { work_id },
                );
            }
        }
        let current = ctx.window().map(|w| w.id());
        // Snapshot first: each close mutates the manager's map, and `windows()`
        // is a live read of it.
        let siblings: Vec<_> = ctx
            .windows()
            .into_iter()
            .map(|w| w.id())
            .filter(|id| Some(*id) != current)
            .collect();
        for id in siblings {
            // Forced (queue_close), not guarded — the project is already gone.
            ctx.close_window_by_id(id);
        }
        // The invoking window is absent from `windows()` during its own
        // dispatch; this is the only reliable way to close it.
        ctx.close_window_forced();
        self.inner.active.set(false);
        self.settle_deferred(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::commands::long_operation_commands;

    #[test]
    fn a_clean_work_is_skipped() {
        assert_eq!(step_for(false, false, false), QuitStep::Skip);
        assert_eq!(
            step_for(false, true, true),
            QuitStep::Skip,
            "nothing unsaved outranks every other consideration, as it does for a close"
        );
    }

    #[test]
    fn autosave_saves_without_asking() {
        assert_eq!(step_for(true, false, true), QuitStep::SaveSilently);
    }

    #[test]
    fn a_dirty_work_without_autosave_is_asked_about() {
        assert_eq!(step_for(true, false, false), QuitStep::AskSaveDiscardCancel);
    }

    /// A backup is read-only, so offering Save would offer something that cannot
    /// happen. Note autosave does NOT turn this into a silent save.
    #[test]
    fn a_backup_never_offers_save() {
        assert_eq!(step_for(true, true, false), QuitStep::AskDiscardCancel);
        assert_eq!(step_for(true, true, true), QuitStep::AskDiscardCancel);
    }

    /// The quit branch order must not be a second opinion — it is a translation
    /// of the one every other exit door already shares.
    #[test]
    fn every_step_tracks_the_shared_unsaved_decision() {
        for unsaved in [false, true] {
            for backup in [false, true] {
                for autosave in [false, true] {
                    let expected = match unsaved_decision(unsaved, backup, autosave) {
                        UnsavedDecision::Proceed => QuitStep::Skip,
                        UnsavedDecision::SaveThenProceed => QuitStep::SaveSilently,
                        UnsavedDecision::PromptSaveDiscardCancel => QuitStep::AskSaveDiscardCancel,
                        UnsavedDecision::PromptDiscardOnly => QuitStep::AskDiscardCancel,
                    };
                    assert_eq!(step_for(unsaved, backup, autosave), expected);
                }
            }
        }
    }

    fn sequencer() -> QuitSequencer {
        QuitSequencer::new(
            Rc::new(AppContext::new()),
            WorkRegistry::new(),
            Signal::new(false),
        )
    }

    #[test]
    fn a_fresh_sequencer_is_idle() {
        assert!(!sequencer().is_active());
    }

    /// A second Ctrl+Q while a quit is running must not start a second sequence
    /// over the same Works — that is two prompts per project, and two answers
    /// racing each other into `advance`.
    #[test]
    fn begin_is_ignored_while_a_quit_is_already_running() {
        let q = sequencer();
        q.inner.active.set(true);
        q.inner.queue.borrow_mut().push_back(7);
        // `begin` would clear and re-snapshot the queue; the guard must stop it
        // before it does.
        let mut tree = teksilo::core::widget_tree::WidgetTree::new();
        tree.run_with_event_context(&mut teksilo::core::NoopWindowOps, |ctx| q.begin(ctx));
        assert_eq!(
            q.inner.queue.borrow().len(),
            1,
            "the running sequence's queue must survive a second Ctrl+Q"
        );
    }

    /// Cancel is a decision about the *quit*, not about one project: it must
    /// leave nothing queued and nothing waiting, so no later save completion can
    /// resume a quit the user called off.
    #[test]
    fn abort_clears_the_queue_and_the_outstanding_save() {
        let q = sequencer();
        q.inner.active.set(true);
        q.inner.queue.borrow_mut().extend([1, 2, 3]);
        *q.inner.waiting.borrow_mut() = Some((1, 42));

        q.abort();

        assert!(q.inner.queue.borrow().is_empty());
        assert!(q.inner.waiting.borrow().is_none());
        assert!(!q.is_active());
    }

    /// Discard must not go through `back_up_then_continue` / `on_close_flow`.
    /// With an empty registry the only thing we can pin without a live Work is
    /// that Discard still drains the queue and finishes the quit (same as Skip
    /// with nothing left to back up) — i.e. it does not hang waiting on a backup.
    #[test]
    fn discard_and_continue_drains_an_empty_queue_without_hanging() {
        let q = sequencer();
        q.inner.active.set(true);
        // Queue empty: discard's `advance` must call `finish` and go idle.
        let mut tree = teksilo::core::widget_tree::WidgetTree::new();
        tree.run_with_event_context(&mut teksilo::core::NoopWindowOps, |ctx| {
            q.discard_and_continue(ctx);
        });
        assert!(
            !q.is_active(),
            "discard finishes the quit when nothing remains"
        );
        assert!(q.inner.queue.borrow().is_empty());
    }

    /// With nothing waiting, a stray completion (a backup's, an import's) must
    /// not be mistaken for the save a quit is parked on.
    #[test]
    fn a_completion_with_nothing_outstanding_is_ignored() {
        let q = sequencer();
        use frontend::common::event::{LongOperationEvent, Origin};
        let event = Event {
            origin: Origin::LongOperation(LongOperationEvent::Completed),
            ids: Vec::new(),
            data: Some(r#"{"id":"some-other-op"}"#.to_string()),
        };
        let mut tree = teksilo::core::widget_tree::WidgetTree::new();
        tree.run_with_event_context(&mut teksilo::core::NoopWindowOps, |ctx| {
            q.on_long_op_completed(&event, ctx);
            q.on_long_op_failed(&event, ctx);
        });
        assert!(!q.is_active(), "and must not start one either");
    }

    /// Run `f` with a live `EventContext` over `tree`, as a close guard runs, and
    /// hand back what it returned.
    fn in_guard<T>(
        tree: &mut teksilo::core::widget_tree::WidgetTree,
        f: impl FnOnce(&mut EventContext) -> T,
    ) -> T {
        let mut f = Some(f);
        let mut out = None;
        tree.run_with_event_context(&mut teksilo::core::NoopWindowOps, |ctx| {
            if let Some(f) = f.take() {
                out = Some(f(ctx));
            }
        });
        out.expect("the guard ran")
    }

    /// The answer a held close got, if any.
    fn answer_of(response: &CloseResponse) -> Option<teksilo::core::window::CloseAnswer> {
        match response {
            CloseResponse::Defer(decision) => decision.receipt().answer(),
            _ => panic!("a quit the system runs is held, not answered on the spot"),
        }
    }

    /// A quit teksilo runs (the macOS Dock's Quit) and that can be answered goes
    /// through this sequence: the window's close is held, and released once the
    /// sequence is done. Nothing is open here, so it is done at once.
    #[test]
    fn a_quit_the_system_asks_for_is_held_until_the_sequence_finishes() {
        let q = sequencer();
        let mut tree = teksilo::core::widget_tree::WidgetTree::new();
        let response = in_guard(&mut tree, |ctx| q.defer_close_to_quit(ctx));
        assert_eq!(
            answer_of(&response),
            Some(teksilo::core::window::CloseAnswer::Proceed),
            "with nothing to account for, the quit goes through"
        );
        assert!(!q.is_active());
    }

    /// Cancel in the sequence cancels the quit the system asked for, so it stops
    /// there (on macOS, AppKit is told the quit was refused) instead of closing
    /// the windows anyway. A second request joining the first is answered with it.
    #[test]
    fn cancelling_the_sequence_cancels_the_held_quit() {
        let _registry = crate::test_support::IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let (mut tree, job, app_ctx, op) = importing(dir.path());
        let q = sequencer();

        let first = Rc::new(RefCell::new(None));
        let second = Rc::new(RefCell::new(None));
        let (quitting, slot) = (q.clone(), first.clone());
        crate::test_support::press(&mut tree, move |c| {
            *slot.borrow_mut() = Some(quitting.defer_close_to_quit(c));
        });
        let (quitting, slot) = (q.clone(), second.clone());
        crate::test_support::press(&mut tree, move |c| {
            *slot.borrow_mut() = Some(quitting.defer_close_to_quit(c));
        });
        for held in [&first, &second] {
            let held = held.borrow();
            assert_eq!(
                answer_of(held.as_ref().expect("the guard answered")),
                None,
                "both closes wait while the import question is up"
            );
        }
        answer(
            &mut tree,
            &tr!(quit_import_running_title()).resolve_now(),
            &StandardButton::Cancel.default_label().resolve_now(),
        );
        for held in [&first, &second] {
            assert_eq!(
                answer_of(held.borrow().as_ref().expect("the guard answered")),
                Some(teksilo::core::window::CloseAnswer::Cancel)
            );
        }
        assert!(!q.is_active());
        assert!(job.is_running(), "the import goes on, as for Work ▸ Quit");
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
        while long_operation_commands::is_operation_finished(&app_ctx, &op) != Some(true) {
            assert!(std::time::Instant::now() < deadline);
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }

    /// A quit nobody can answer stops a project import still writing and waits
    /// for its end before the window goes: on macOS the process ends right after,
    /// and the converter would otherwise die mid-write.
    #[test]
    fn a_quit_nobody_can_answer_stops_a_running_import() {
        let _registry = crate::test_support::IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let (mut tree, _job, app_ctx, op) = importing(dir.path());
        let q = sequencer();
        let response = in_guard(&mut tree, |ctx| q.close_without_asking(ctx, None));
        assert!(matches!(response, CloseResponse::Close));
        assert_eq!(
            long_operation_commands::is_operation_finished(&app_ctx, &op),
            Some(true),
            "the import has ended by the time the window may close"
        );
        assert!(
            !dir.path().join("novel.skrib.importing").exists(),
            "no half-written file is left"
        );
    }

    /// A session over a real project, the way a project window holds one.
    #[cfg(not(feature = "mocks"))]
    fn session_over(project: &crate::test_support::RealProject) -> WorkSession {
        WorkSession::new(
            project.app_ctx.clone(),
            project.ids.clone(),
            crate::spellcheck::SpellcheckService::new(),
            teksilo::widgets::DockingModel::new(),
            crate::backup::BackupSettingsViewModel::new(
                crate::models::BackupSettingsService::in_memory_default(),
            ),
            crate::models::WorkspaceLayoutService::in_memory_default(),
            crate::models::TreeExpansionService::in_memory_default(),
        )
    }

    /// The titles of the project saved at `project.path`, read back the way
    /// reopening it does.
    #[cfg(not(feature = "mocks"))]
    fn saved_titles(project: &crate::test_support::RealProject) -> Vec<String> {
        let reopened = project.reopened();
        let work_id = frontend::commands::work_commands::get_all_work(&reopened)
            .expect("get_all_work")
            .pop()
            .expect("the file holds a Work")
            .id;
        crate::models::binder_stream::ordered_binder_items(&reopened, work_id)
            .into_iter()
            .map(|row| row.title)
            .collect()
    }

    /// A quit nobody can answer writes what the writer did, with autosave off
    /// (which would have meant a question), and is on disk by the time the
    /// window may close.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn a_quit_nobody_can_answer_saves_the_project_first() {
        let project = crate::test_support::RealProject::empty_novel();
        project.save();
        let session = session_over(&project);
        let (first, _) = project.scenes()[0];
        project.add_scene_after(first, "Written at the last moment");
        session.save_state.bump_dirty();
        assert!(session.save_state.is_unsaved());

        let q = sequencer();
        let mut tree = teksilo::core::widget_tree::WidgetTree::new();
        let response = in_guard(&mut tree, |ctx| q.close_without_asking(ctx, Some(&session)));
        assert!(matches!(response, CloseResponse::Close));
        assert!(!session.save_state.is_unsaved(), "the save is credited");
        assert!(
            saved_titles(&project).contains(&"Written at the last moment".to_string()),
            "the scene added last is in the file"
        );
    }

    /// A backup open read-only has no file to be written to, so a quit nobody
    /// can answer leaves the file as it was, as Discard would.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn a_quit_nobody_can_answer_never_writes_over_a_backup() {
        let project = crate::test_support::RealProject::empty_novel();
        project.save();
        let session = session_over(&project);
        session.backup_mode.set(true);
        let (first, _) = project.scenes()[0];
        project.add_scene_after(first, "Typed into a backup");
        session.save_state.bump_dirty();

        let q = sequencer();
        let mut tree = teksilo::core::widget_tree::WidgetTree::new();
        in_guard(&mut tree, |ctx| q.close_without_asking(ctx, Some(&session)));
        assert!(
            !saved_titles(&project).contains(&"Typed into a backup".to_string()),
            "the file is untouched"
        );
    }

    /// A tree whose app state holds `job`, as the running app's does, and a Plume
    /// import of a small project started on it, not yet heard to have ended.
    fn importing(
        dir: &std::path::Path,
    ) -> (
        teksilo::core::widget_tree::WidgetTree,
        ProjectImportJob,
        Rc<AppContext>,
        String,
    ) {
        use std::any::{Any, TypeId};
        use std::io::Write;

        let source = dir.join("Lonely.plume");
        let mut zip = zip::ZipWriter::new(std::fs::File::create(&source).unwrap());
        zip.start_file("tree", zip::write::SimpleFileOptions::default())
            .unwrap();
        zip.write_all(
            br#"<!DOCTYPE plume-tree><plume-tree version="0.5" projectName="Lonely">
                <book number="1" name="Book"><scene number="2" name="Scene"/></book>
              </plume-tree>"#,
        )
        .unwrap();
        zip.finish().unwrap();

        let app_ctx = Rc::new(AppContext::new());
        let job = ProjectImportJob::new(app_ctx.clone());
        let state: std::collections::HashMap<TypeId, Box<dyn Any>> = [(
            TypeId::of::<ProjectImportJob>(),
            Box::new(job.clone()) as Box<dyn Any>,
        )]
        .into();
        let mut tree = crate::test_support::tree_with_app_state(&app_ctx, state);
        let form = crate::import_plume::ImportPlumeViewModel::new(job.clone());
        form.source().set(source.to_string_lossy().into_owned());
        form.location().set(dir.to_string_lossy().into_owned());
        form.name().set("novel".into());
        crate::test_support::press(&mut tree, move |c| form.import(c));
        let op = job.active_operation().expect("the import started");
        (tree, job, app_ctx, op)
    }

    /// Click the button labelled `label` in the one question a press raised.
    fn answer(tree: &mut teksilo::core::widget_tree::WidgetTree, title: &str, label: &str) {
        use teksilo::core::ModalContent;
        let request = tree
            .drain_pending_modal_requests()
            .pop()
            .expect("a question is asked")
            .request;
        assert_eq!(request.title.as_deref(), Some(title));
        let ModalContent::Deferred(builder) = request.content else {
            panic!("a MessageBox presents deferred content");
        };
        builder(tree);
        tree.layout(SizeProposal::exact(900.0, 600.0));
        let button = tree.find_by_label(label).expect("the question offers it");
        crate::test_support::click(tree, button);
    }

    /// Quitting while a project import runs asks first. Stopping it cancels the
    /// import and waits for its end before any Work is accounted for or any
    /// window closes, so the process never exits under a converter still writing.
    #[test]
    fn quitting_during_an_import_stops_it_first_and_waits_for_its_end() {
        use frontend::common::event::{LongOperationEvent, Origin};
        use frontend::common::long_operation::OperationStatus;
        use std::time::Duration;

        let _registry = crate::test_support::IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let (mut tree, job, app_ctx, op) = importing(dir.path());
        let q = sequencer();

        let quitting = q.clone();
        crate::test_support::press(&mut tree, move |c| quitting.begin(c));
        answer(
            &mut tree,
            &tr!(quit_import_running_title()).resolve_now(),
            &tr!(quit_import_stop()).resolve_now(),
        );
        assert!(q.is_active(), "the quit is under way");
        assert!(job.is_running(), "and waits for the import to end");

        // The worker's own end, not `is_operation_finished`: that answers `true`
        // the moment the cancel is requested, while the converter may still be
        // writing, and the event built below stands for the one its worker sends.
        let completion =
            frontend::common::long_operation::lock_or_recover(&app_ctx.long_operation_manager)
                .completion_signal();
        assert!(
            completion.wait_for(&op, Some(Duration::from_secs(60))),
            "the import never ended"
        );
        let status = long_operation_commands::get_operation_status(&app_ctx, &op);
        let ended = Event {
            origin: Origin::LongOperation(match status {
                Some(OperationStatus::Completed) => LongOperationEvent::Completed,
                _ => LongOperationEvent::Cancelled,
            }),
            ids: Vec::new(),
            data: Some(format!(r#"{{"id":"{op}"}}"#)),
        };
        let ending = job.clone();
        crate::test_support::press(&mut tree, move |c| match ended.origin {
            Origin::LongOperation(LongOperationEvent::Completed) => {
                ending.on_long_op_completed(c, &ended)
            }
            _ => ending.on_long_op_cancelled(c, &ended),
        });

        assert!(!job.is_running());
        assert!(
            !q.is_active(),
            "with the import ended and nothing open, the quit finished"
        );
        assert!(
            !dir.path().join("novel.skrib.importing").exists(),
            "no half-written file is left"
        );
    }

    /// Cancel keeps the import running and every window open, and a later quit
    /// asks again.
    #[test]
    fn cancelling_the_question_keeps_the_import_and_the_app() {
        let _registry = crate::test_support::IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let (mut tree, job, app_ctx, op) = importing(dir.path());
        let q = sequencer();

        let quitting = q.clone();
        crate::test_support::press(&mut tree, move |c| quitting.begin(c));
        answer(
            &mut tree,
            &tr!(quit_import_running_title()).resolve_now(),
            &StandardButton::Cancel.default_label().resolve_now(),
        );
        assert!(!q.is_active());
        assert!(job.is_running(), "the import goes on");
        assert_ne!(
            long_operation_commands::get_operation_status(&app_ctx, &op),
            Some(frontend::common::long_operation::OperationStatus::Cancelled),
            "and was not cancelled"
        );

        let quitting = q.clone();
        crate::test_support::press(&mut tree, move |c| quitting.begin(c));
        assert_eq!(
            crate::test_support::drain_dialog_titles(&mut tree),
            vec![tr!(quit_import_running_title()).resolve_now()],
            "a later quit asks again"
        );
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
        while long_operation_commands::is_operation_finished(&app_ctx, &op) != Some(true) {
            assert!(std::time::Instant::now() < deadline);
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }
}
