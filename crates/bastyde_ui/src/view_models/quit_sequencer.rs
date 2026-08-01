// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `QuitSequencer` — Ctrl+Q with several projects open.
//!
//! ## What it replaces
//!
//! Quit used to *refuse*: with any other Work dirty it named them in a message box
//! and did nothing (`other_dirty_work_titles`), and with none dirty it force-closed
//! only **its own** window — so with two windows open, "Quit" did not quit. Both
//! halves were defensible while two simultaneous projects were an exotic case. Since
//! Phase 4 made Skribisto single-instance, several windows in one process is the
//! ordinary shape, and a Quit that neither quits nor explains itself is not.
//!
//! ## What it does instead
//!
//! Accounts for **every** open Work in turn, then closes **every** window:
//!
//! 1. Snapshot the open Works ([`WorkRegistry::open_work_ids`]) — once, at the
//!    start. A window opened while the sequence is running is deliberately not
//!    swept: the user asked to quit the app they could see.
//! 2. For each, apply the app's one shared branch order
//!    ([`crate::view_models::unsaved_decision`]) — the same one `work.close`, the
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
//! 5. When the queue drains, close every window. bastyde exits once its window map
//!    is empty (`maybe_exit`), so there is no separate "terminate" step.
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

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::widgets::{
    MessageBox, MessageBoxButton, MessageBoxButtons, StandardButton,
};
use frontend::commands::work_management_commands;
use frontend::work_management::CloseWorkDto;
use frontend::{AppContext, Event};

use crate::app::PendingExit;
use crate::sessions::{WorkRegistry, WorkSession};
use crate::view_models::{UnsavedDecision, unsaved_decision};

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
}

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
            }),
        }
    }

    /// Is a quit currently running? (Read by the Quit command so a second
    /// Ctrl+Q is a no-op rather than a second sequence.)
    pub fn is_active(&self) -> bool {
        self.inner.active.get()
    }

    /// Start quitting. No-op if one is already under way.
    ///
    /// The queue is sorted by `work_id` purely so the order is *deterministic* —
    /// `open_work_ids` reads a `HashMap`, and a quit that prompted for projects
    /// in a different order each time would be impossible to script or to test.
    /// Ascending `work_id` is open order in practice, which is also the least
    /// surprising order to be asked in.
    pub fn begin(&self, ctx: &mut EventContext) {
        if self.inner.active.get() {
            return;
        }
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
            let step = step_for(
                session.unsaved.get(),
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
    fn ask_save_discard_cancel(
        &self,
        ctx: &mut EventContext,
        work_id: u64,
        session: &WorkSession,
    ) {
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
        session.backup_scheduler.on_close_flow(ctx, PendingExit::Quit);
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
        session
            .save_state
            .on_save_completed(event, move || flush_session.backup_scheduler.flush_all_windows());
        if session.save_state.saved_seq().get() >= covers {
            self.inner.waiting.borrow_mut().take();
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
        self.inner.active.set(false);
    }

    /// Every Work is accounted for: close each Work's backend subtree, then every
    /// window. bastyde's `maybe_exit` ends the process once none are left, so
    /// there is nothing to call after this.
    ///
    /// The desk is captured *before* `close_work` — that call tears the Work
    /// subtree out before publishing `CloseWork`, after which a tab can no longer
    /// be translated into anything persistable.
    ///
    /// **Closing windows.** Do not rely on `ctx.windows()` alone. During event
    /// dispatch bastyde pulls the *current* window out of the window map, so
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut tree = bastyde::core::widget_tree::WidgetTree::new();
        tree.run_with_event_context(&mut bastyde::core::NoopWindowOps, |ctx| q.begin(ctx));
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
        let mut tree = bastyde::core::widget_tree::WidgetTree::new();
        tree.run_with_event_context(&mut bastyde::core::NoopWindowOps, |ctx| {
            q.discard_and_continue(ctx);
        });
        assert!(!q.is_active(), "discard finishes the quit when nothing remains");
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
        let mut tree = bastyde::core::widget_tree::WidgetTree::new();
        tree.run_with_event_context(&mut bastyde::core::NoopWindowOps, |ctx| {
            q.on_long_op_completed(&event, ctx);
            q.on_long_op_failed(&event, ctx);
        });
        assert!(!q.is_active(), "and must not start one either");
    }
}
