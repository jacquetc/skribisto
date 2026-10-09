// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The one project-import job every whole-project importer drives.
//!
//! Plume Creator and Manuskript (and the Scrivener importer to come) each read a
//! foreign project their own way, and each asks for it with a form of its own.
//! Everything from the moment the writer presses **Import** is the same, and
//! lives here, in [`ProjectImportJob`]:
//!
//! * **One import at a time, in the whole process.** A second request, from any
//!   importer and any window, is refused with a message while the first runs:
//!   the progress notice, its Cancel button and the hold on the target all
//!   belong to one import, and a second would take all three over.
//! * **The destination is never a project that is open**, checked when Import is
//!   pressed, again after the overwrite question, and once more after the target
//!   is held (`shared::import_destination`).
//! * **A loading notice with the current phase and Cancel.** The converter
//!   reports a phase code, or the title of the entry it is converting, which
//!   `shared::progress_text` words in the writer's language or shows as data; a
//!   screen reader hears one sentence per phase, never a percentage or a title.
//! * **Completion**: a result with **Open now**, which the notification log can
//!   still replay once the notice is gone, and, when the importer has something to
//!   say, the persistent report notice of `shared::import_warnings`, reopenable
//!   from the notification log.
//! * **Failure**: the typed reason worded by `shared::import_failure`, the
//!   technical chain behind **Details**.
//! * **Cancel is over when the converter has stopped**, not when it was asked
//!   to. The backend announces a cancellation the moment it is requested, while
//!   a converter writing its file looks at no cancel flag until the file is
//!   written; the notice says *Stopping*, and the target stays held, until the
//!   converter's own end is heard.
//! * **Stopping for a quit** ([`ProjectImportJob::stop_then`]): the quit sequence
//!   cancels the import and waits until the converter has stopped, so no
//!   half-written file is left when the process exits (the converters themselves
//!   leave nothing behind on any other way out).
//! * **Stopping at exit** ([`ProjectImportJob::stop_at_exit`]): closing the last
//!   window (the Launcher) ends the process without the quit sequence, so once
//!   the event loop is over the converter is cancelled and waited for there too.
//!
//! What stays with each importer is described by one [`Importer`] value: its
//! request type, the command that starts it, the one that reads its result into
//! an [`ImportOutcome`] in the writer's words, and the few sentences that are its
//! own.
//!
//! **Tier 1** (see `sessions::WorkSession`): one job per process, created in
//! `startup`, handed to each importer's view-model and to the quit sequence, and
//! registered as app state. An import belongs to no open Work: it writes a
//! brand-new `.skrib` nobody has opened yet. Every toast it raises is therefore
//! broadcast, and every window that can start one subscribes to its events
//! ([`ProjectImportJob::wire_long_operation`]); each handler filters on the
//! running operation, so whichever window hears an event second finds nothing
//! left to do.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use frontend::AppContext;
use frontend::commands::long_operation_commands;
use frontend::common::event::{Event, LongOperationEvent, Origin};
use frontend::common::long_operation::lock_or_recover;
use frontend::import_management::failure::{FailureReason, ProjectImportFailure};
use teksilo::i18n::{LocalizedString, localized};
use teksilo::prelude::*;
use teksilo::widgets::{
    MessageBox, MessageBoxButtons, StandardButton, Toast, ToastAction, ToastPriority,
};

use crate::intents::AppIntent;
use crate::shared::import_destination::{BusyRefusal, TargetHold, refuse_if_busy, refuse_if_open};
use crate::shared::import_failure::{self, OwnWords};
use crate::shared::import_warnings::{LiveNotice, WarningsNotice};
use crate::shared::long_op::{event_id, parse_payload, payload_id};
use crate::shared::progress_text;
use crate::shared::progress_toast::ProgressToast;

/// What the name of an archived **Open now** starts with, the path its import wrote
/// following it. The notification log keeps no closure, only an action's name, so the
/// name says which project to open, and the log's replay hook
/// (`shared::import_warnings::replay_archived_action`) opens it. Stable: it is persisted
/// in the notification archive.
const ARCHIVED_OPEN_PREFIX: &str = "import.open_now:";

/// The name an archived **Open now** of the import that wrote `output_path` keeps.
pub(crate) fn archived_open_name(output_path: &str) -> String {
    format!("{ARCHIVED_OPEN_PREFIX}{output_path}")
}

/// The project an archived action's `name` opens, if it is an archived **Open now**.
pub(crate) fn archived_open_path(name: &str) -> Option<&str> {
    name.strip_prefix(ARCHIVED_OPEN_PREFIX)
        .filter(|path| !path.is_empty())
}

/// What a finished import produced, in the writer's words.
pub(crate) struct ImportOutcome {
    /// The `.skrib` it wrote, for **Open now**.
    pub(crate) output_path: String,
    /// The result line: what came across.
    pub(crate) result: LocalizedString,
    /// A short title for the result notice, the result line then its body; `None`
    /// keeps the result line as the title, for a result short enough to be one.
    pub(crate) headline: Option<LocalizedString>,
    /// What the writer should know, one line each; empty when there is nothing.
    pub(crate) report: Vec<LocalizedString>,
    /// The result waits for the writer instead of leaving after a few seconds: it says
    /// something the writer has to act on, and is the only place that says it.
    pub(crate) persistent: bool,
    /// **See report** beside **Open now**, for an importer whose report is a dialog of its
    /// own; `None` for one whose report is the notice of `shared::import_warnings`.
    pub(crate) see_report: Option<SeeReport>,
}

/// The result notice's **See report**: an intent the window it is pressed in answers by
/// showing the report of this import, and the name of the action the notification log
/// replays once the notice's closures are gone.
#[derive(Clone, Copy)]
pub(crate) struct SeeReport {
    /// The action's label.
    pub(crate) label: fn() -> LocalizedString,
    /// The intent that shows the report of the import that wrote this path.
    pub(crate) intent: fn(String) -> AppIntent,
    /// The name an archived row replays, given the path the import wrote: an archived row
    /// keeps no closure, so its name says which import's report it opens. Stable: it is
    /// persisted in the notification archive.
    pub(crate) archived_as: fn(&str) -> String,
}

/// What a failure notice offers beside **Details** when the importer's own review answers
/// the failure: the project changed since it was reviewed, say. Its action opens the
/// importer again on the same project.
#[derive(Clone, Copy)]
pub(crate) struct ReviewAgain {
    /// Whether a fresh review answers this failure.
    pub(crate) applies: fn(&FailureReason) -> bool,
    /// The action's label.
    pub(crate) label: fn() -> LocalizedString,
    /// The intent that opens the importer on the project the import read.
    pub(crate) intent: fn(String) -> AppIntent,
}

/// One project importer, as the job sees it.
///
/// A `static` per importer, beside its form's view-model. `R` is the importer's
/// request DTO.
pub(crate) struct Importer<R: 'static> {
    /// The update-in-place id of the import's own toast, which goes from progress
    /// to result, cancellation or failure. Kept per importer: the notification
    /// log keeps one row per id, which a later import under the same id brings
    /// back to the top.
    pub(crate) toast_id: &'static str,
    /// The report notice, and the name its **Details** is archived under.
    pub(crate) notice: &'static WarningsNotice,
    /// The progress notice's title.
    pub(crate) progress_title: fn() -> LocalizedString,
    /// The importer's own sentence for the failures it words itself.
    pub(crate) own_words: OwnWords,
    /// The `.skrib` a request writes.
    pub(crate) target: fn(&R) -> String,
    /// The request, allowed to replace an existing target.
    pub(crate) allow_overwrite: fn(R) -> R,
    /// Start the conversion, returning its long-operation id.
    pub(crate) launch: fn(&AppContext, &R) -> anyhow::Result<String>,
    /// Read the finished operation's result, in the writer's words, or `None`
    /// when there is none to read.
    pub(crate) outcome: fn(&AppContext, &str) -> Option<ImportOutcome>,
    /// The project a request reads, for [`ReviewAgain`].
    pub(crate) source: fn(&R) -> String,
    /// What a failure the importer's review answers offers; `None` for an importer
    /// with no review.
    pub(crate) review_again: Option<ReviewAgain>,
    /// What **Open now** asks the window it is pressed in to do with the new project.
    pub(crate) open_now: fn(String) -> AppIntent,
}

/// The import that is running: what its own toasts need, whatever its request.
#[derive(Clone)]
struct Running {
    op_id: String,
    /// Cancel was asked for and the converter has not stopped yet: the notice
    /// says so, and its progress ticks no longer move it.
    stopping: bool,
    /// The last phase code the converter reported: the stretch of work it is
    /// in, which the notice announces while the entries it converts tick by.
    phase: Option<String>,
    target: String,
    toast_id: &'static str,
    notice: &'static WarningsNotice,
    progress_title: fn() -> LocalizedString,
    own_words: OwnWords,
    outcome: fn(&AppContext, &str) -> Option<ImportOutcome>,
    /// The project the import reads, for [`ReviewAgain`].
    source: String,
    review_again: Option<ReviewAgain>,
    open_now: fn(String) -> AppIntent,
}

/// Why a second import is refused while one runs.
static BUSY: BusyRefusal = BusyRefusal {
    title: || tr!(project_import_busy_title()),
    text: || tr!(project_import_busy_text()),
};

/// What to do once the running import has ended.
type AfterStop = Box<dyn FnOnce(&mut EventContext)>;

/// How long a notice that asks nothing of the writer stays up.
const BRIEF: Duration = Duration::from_secs(4);

/// How long a process whose last window has closed waits for a cancelled
/// converter to stop ([`ProjectImportJob::stop_at_exit`]). A converter stops at
/// its next row, or once the file it is writing is whole; writing even a very
/// large project takes a few seconds, so this is only ever reached by a converter
/// that is stuck.
pub(crate) const EXIT_PATIENCE: Duration = Duration::from_secs(30);

/// The shared project-import job. Cheap to clone; clones share everything.
#[derive(Clone)]
pub(crate) struct ProjectImportJob {
    inner: Rc<Inner>,
}

struct Inner {
    app_ctx: Rc<AppContext>,
    /// The long-operation id of the import running now, if any: set on start,
    /// cleared on its first terminal event. Also what [`refuse_if_busy`] reads.
    active: Signal<Option<String>>,
    /// What the running import's toasts need.
    running: RefCell<Option<Running>>,
    /// The target the running import is writing, held until it ends.
    target_hold: TargetHold,
    /// Each importer's report notice on screen, by its archived action name: one
    /// importer's next report replaces its own last one, never another's.
    notices: RefCell<HashMap<&'static str, LiveNotice>>,
    /// What to do once the running import has ended (see [`ProjectImportJob::stop_then`]).
    after_stop: RefCell<Vec<AfterStop>>,
    /// The running import's loading notice, and the notice that ends it.
    progress: ProgressToast,
}

impl ProjectImportJob {
    pub(crate) fn new(app_ctx: Rc<AppContext>) -> Self {
        Self {
            inner: Rc::new(Inner {
                app_ctx,
                active: Signal::new(None),
                running: RefCell::new(None),
                target_hold: TargetHold::default(),
                notices: RefCell::new(HashMap::new()),
                after_stop: RefCell::new(Vec::new()),
                progress: ProgressToast::default(),
            }),
        }
    }

    /// Whether an import is running.
    pub(crate) fn is_running(&self) -> bool {
        self.inner.active.get().is_some()
    }

    /// Whether an import is running, as a signal a form can bind: a second import cannot
    /// start until this one has ended. Derived from the job's own state, so reading it
    /// touches nothing.
    pub(crate) fn running(&self) -> Signal<bool> {
        self.inner.active.map(|active| active.is_some())
    }

    /// The long-operation id of the running import, if any.
    #[cfg(test)]
    pub(crate) fn active_operation(&self) -> Option<String> {
        self.inner.active.get()
    }

    /// The file name of the `.skrib` the running import is writing, if one runs.
    pub(crate) fn running_target_name(&self) -> Option<String> {
        let running = self.inner.running.borrow();
        let target = &running.as_ref()?.target;
        Some(
            Path::new(target)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| target.clone()),
        )
    }

    /// The writer pressed **Import** on a form whose fields have just been
    /// checked again: refuse a second import and a target that is open, ask
    /// before replacing an existing file, then start.
    ///
    /// Everything below acts on `request`, taken from the form now. A form is one
    /// view-model every window shares, and the overwrite question is modal in its
    /// own window only: while it waits, another window can open the importer and
    /// fill the form in afresh. OK then imports the file that was asked about and
    /// checked, never the one the form names by that time.
    pub(crate) fn import<R: Clone + 'static>(
        &self,
        ctx: &mut EventContext,
        importer: &'static Importer<R>,
        request: R,
    ) {
        if refuse_if_busy(ctx, &self.inner.active, &BUSY) {
            return;
        }
        let target = (importer.target)(&request);
        if refuse_if_open(ctx, &target) {
            return;
        }
        if !Path::new(&target).exists() {
            self.start(ctx, importer, request);
            return;
        }
        let job = self.clone();
        let name = Path::new(&target)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        MessageBox::warning(tr!(project_import_overwrite_title()))
            .text(tr!(project_import_overwrite_text(name = name)))
            .buttons(MessageBoxButtons::OkCancel)
            .on_result(move |r, c| {
                // The confirmation can sit open while the writer opens that very
                // project in another window, so the refusal is asked again at
                // the last moment.
                if r.button == StandardButton::Ok && !refuse_if_open(c, &target) {
                    job.start(c, importer, (importer.allow_overwrite)(request.clone()));
                }
            })
            .present(ctx);
    }

    /// Start the conversion of `request`, a long operation. Returns immediately:
    /// the form's panel closes and the loading toast takes over. Only a failure
    /// to *start* is reported here; the import's own arrive as a `Failed` event.
    pub(crate) fn start<R: 'static>(
        &self,
        ctx: &mut EventContext,
        importer: &'static Importer<R>,
        request: R,
    ) {
        // Asked again here: an overwrite question can wait while another window
        // starts an import, which this one would take over.
        if refuse_if_busy(ctx, &self.inner.active, &BUSY) {
            return;
        }
        let target = (importer.target)(&request);
        // Held before the conversion starts, so there is no moment it runs
        // unheld, and looked at again once held, for a load another copy of
        // Skribisto started meanwhile.
        if !self.inner.target_hold.hold_unless_open(ctx, &target) {
            return;
        }
        let running = Running {
            op_id: String::new(),
            stopping: false,
            phase: None,
            target,
            toast_id: importer.toast_id,
            notice: importer.notice,
            progress_title: importer.progress_title,
            own_words: importer.own_words,
            outcome: importer.outcome,
            source: (importer.source)(&request),
            review_again: importer.review_again,
            open_now: importer.open_now,
        };
        match (importer.launch)(&self.inner.app_ctx, &request) {
            Ok(op_id) => {
                let running = Running {
                    op_id: op_id.clone(),
                    ..running
                };
                // Close the form. `dismiss_top_overlay`, not `dismiss_modal`: on the
                // overwrite path this runs from the question's `on_result`, whose
                // context is anchored at the tree root, so `dismiss_modal`'s walk to
                // the enclosing modal would do nothing. The form is the topmost
                // overlay either way.
                ctx.dismiss_top_overlay();
                let progress = self.progress_toast(&running, 0.0, Stage::Starting);
                self.inner.progress.show(ctx, progress);
                *self.inner.running.borrow_mut() = Some(running);
                self.inner.active.set(Some(op_id));
            }
            Err(e) => {
                self.inner.target_hold.release();
                self.show_failure(
                    ctx,
                    &running,
                    &ProjectImportFailure::internal(&format!("{e:#}")),
                );
            }
        }
    }

    /// Cancel the running import (its toast's Cancel, or a quit). The converter
    /// stops at its next checkpoint, which in its writing phase is only once the
    /// whole file is written; until then the notice says the import is stopping,
    /// and the import is not over (see [`Self::on_long_op_cancelled`]). Asking
    /// again while it stops changes nothing.
    pub(crate) fn cancel(&self, ctx: &mut EventContext) {
        let stopping = {
            let mut running = self.inner.running.borrow_mut();
            let Some(running) = running.as_mut().filter(|r| !r.stopping) else {
                return;
            };
            running.stopping = true;
            running.clone()
        };
        if long_operation_commands::cancel_operation(&self.inner.app_ctx, &stopping.op_id) {
            self.inner.progress.show(ctx, stopping_toast(&stopping));
            return;
        }
        // The manager no longer knows the operation: it ended long ago, and the
        // event saying so reached no window of this process. Nothing is left to
        // stop, and no event will come to close the import, so it closes now.
        if let Some(running) = self.end(Some(&stopping.op_id)) {
            self.cancelled(ctx, &running);
        }
    }

    /// Whether the running import's converter has stopped for good: its worker
    /// has returned, having cleaned up whatever it wrote. `true` with nothing
    /// running.
    fn converter_stopped(&self) -> bool {
        let Some(op_id) = self.inner.active.get() else {
            return true;
        };
        lock_or_recover(&self.inner.app_ctx.long_operation_manager)
            .completion_signal()
            .is_finished(&op_id)
    }

    /// Stop the running import, and call `then` once it has ended: cancelled,
    /// failed, or finished before the cancel reached it. With nothing running,
    /// `then` runs now.
    ///
    /// For the quit sequence: the process must not exit while a converter is
    /// still writing, or the half-written file stays on disk. Waiting for the
    /// converter's own end (never the notice that a cancel was requested) is what
    /// guarantees it has cleaned up after itself.
    pub(crate) fn stop_then(
        &self,
        ctx: &mut EventContext,
        then: impl FnOnce(&mut EventContext) + 'static,
    ) {
        if !self.is_running() {
            then(ctx);
            return;
        }
        self.inner.after_stop.borrow_mut().push(Box::new(then));
        self.cancel(ctx);
    }

    /// The process is ending with an import still running: stop its converter
    /// and wait, for at most `patience`, until it has stopped. Returns whether it
    /// did (or nothing was running).
    ///
    /// For after the event loop, when no window is left. Closing the Launcher
    /// while it is the last window ends the process without the quit sequence
    /// ever running, so nothing asked [`Self::stop_then`] to wait: the converter
    /// would die mid-write, leaving its staged file beside the target and its
    /// assembly folder in the temporary folder. The converters clean up on the
    /// way out of a cancelled import, so waiting for their end is enough. The
    /// wait is bounded: a converter that never stops must not keep a process
    /// with no window alive for ever.
    pub(crate) fn stop_at_exit(&self, patience: Duration) -> bool {
        let Some(op_id) = self.inner.active.get() else {
            return true;
        };
        long_operation_commands::cancel_operation(&self.inner.app_ctx, &op_id);
        // Taken out of the manager, and the manager's lock let go of, before
        // waiting, as `completion_signal` asks: a wait holding that lock would
        // stall every other use of the manager for as long as it lasts.
        let completion =
            lock_or_recover(&self.inner.app_ctx.long_operation_manager).completion_signal();
        let stopped = completion.wait_for(&op_id, Some(patience));
        if stopped {
            // No event loop is left to hear the converter's end: its lock file
            // goes now rather than waiting for another copy to reap it.
            self.inner.target_hold.release();
        }
        stopped
    }

    /// Subscribe a widget tree to the import's four long-operation events.
    ///
    /// **Called once per window that can start an import**, and there are two:
    /// `app::wiring::long_ops` for a project window, and `WelcomePanel::build`
    /// for the Launcher. The Launcher's is not optional: subscriptions are per
    /// `WidgetTree`, so an import started with no project window open would
    /// otherwise sit under a toast that never ticked and never offered **Open
    /// now**. Subscribing from several windows at once is safe by construction
    /// (see the module doc).
    pub(crate) fn wire_long_operation(&self, ctx: &mut BuildContext) {
        type Handler = fn(&ProjectImportJob, &mut EventContext, &Event);
        const HANDLERS: &[(LongOperationEvent, Handler)] = &[
            (LongOperationEvent::Progress, |j, c, e| {
                j.on_long_op_progress(c, e)
            }),
            (LongOperationEvent::Completed, |j, c, e| {
                j.on_long_op_completed(c, e)
            }),
            (LongOperationEvent::Cancelled, |j, c, e| {
                j.on_long_op_cancelled(c, e)
            }),
            (LongOperationEvent::Failed, |j, c, e| {
                j.on_long_op_failed(c, e)
            }),
        ];
        for (event, handler) in HANDLERS {
            let job = self.clone();
            let handler = *handler;
            ctx.subscribe_event_with_ctx(
                Origin::LongOperation(event.clone()),
                move |e: &Event, c| handler(&job, c, e),
            );
        }
    }

    /// A progress tick: update the loading toast in place.
    pub(crate) fn on_long_op_progress(&self, ctx: &mut EventContext, event: &Event) {
        let Some(payload) = parse_payload(event) else {
            return;
        };
        let Some(running) = self.running_for(payload_id(&payload)) else {
            return;
        };
        // A tick sent before the converter saw the cancel would put the Cancel
        // button back on a notice that already says it is stopping.
        if running.stopping {
            return;
        }
        let percent = payload
            .get("percentage")
            .and_then(|p| p.as_f64())
            .unwrap_or(0.0) as f32;
        let message = payload
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or_default();
        // A phase code names the stretch of work until the next one; anything else
        // is data (the entry being converted), shown as it was sent.
        let running = if progress_text::phase_name(message).is_some() {
            let phase = Some(message.to_string());
            if let Some(live) = self.inner.running.borrow_mut().as_mut() {
                live.phase = phase.clone();
            }
            Running { phase, ..running }
        } else {
            running
        };
        let stage = if message.is_empty() {
            Stage::Starting
        } else {
            Stage::Said(message.to_string())
        };
        let progress = self.progress_toast(&running, percent, stage);
        self.inner.progress.show(ctx, progress);
    }

    /// The import finished: the result, with **Open now**, and its report.
    pub(crate) fn on_long_op_completed(&self, ctx: &mut EventContext, event: &Event) {
        let id = event_id(event);
        let Some(running) = self.end(id.as_deref()) else {
            return;
        };
        match (running.outcome)(&self.inner.app_ctx, &running.op_id) {
            Some(outcome) => {
                let output = outcome.output_path.clone();
                let open_now = running.open_now;
                let toast = match &outcome.headline {
                    Some(headline) => Toast::success(headline.clone()).body(outcome.result.clone()),
                    None => Toast::success(outcome.result.clone()),
                };
                let report_of = output.clone();
                // Named after this import's path, so the notification log can still open
                // the project once this notice and its closures are gone: its archived
                // row used to show an **Open now** that did nothing.
                let archived_open = archived_open_name(&output);
                let mut toast = import_toast(running.toast_id, toast).action(
                    ToastAction::primary(tr!(project_import_open_now()), move |c| {
                        // Through an intent the window answers, never `load_work`
                        // outright: replacing the project in a window goes through
                        // its unsaved-changes guard (`work.open_path`), and opening
                        // one beside it through the one door every window uses.
                        c.send_intent(open_now(output.clone()));
                    })
                    .shortcut_id(archived_open),
                );
                if let Some(see) = outcome.see_report {
                    let archived = (see.archived_as)(&report_of);
                    // Reading the report is not dismissing the result: the notice keeps
                    // **Open now** for after it, where the Launcher has no other way to
                    // the new project. Named after this import's path, so the
                    // notification log can still open its report once this notice and its
                    // closures are gone.
                    toast = toast.action(
                        ToastAction::new((see.label)(), move |c| {
                            c.send_intent((see.intent)(report_of.clone()))
                        })
                        .closes_toast(false)
                        .shortcut_id(archived),
                    );
                }
                if outcome.persistent {
                    toast = toast.persistent();
                }
                self.inner.progress.end_in_place(ctx, toast);
                // What the importer could not carry across, or had to guess at.
                // Read by nobody, that is an import that quietly lost data, so it
                // gets a notice of its own that waits for the writer and can be
                // reopened from the notification log. Raised after the result,
                // which its spoken name repeats: a screen reader hears the notice
                // over it.
                let lines: Vec<String> = outcome.report.iter().map(|l| l.resolve_now()).collect();
                let live = self.live_notice(running.notice);
                running.notice.show(ctx, &lines, &outcome.result, &live);
            }
            // Completed without a result to read (should not happen): clear the
            // loading toast rather than leave a spinner up for ever. An `Info`
            // notice, so it replaces the loading one rather than updating it.
            None => {
                self.inner.progress.end_replacing(
                    ctx,
                    import_toast(running.toast_id, Toast::info((running.progress_title)()))
                        .auto_dismiss_after(BRIEF),
                );
            }
        }
        self.after_stop(ctx);
    }

    /// The import was cancelled: a neutral notice that goes by itself.
    ///
    /// Only once the converter has stopped. The manager also sends `Cancelled`
    /// the moment a cancel is requested (marked `requested`), while a converter
    /// in its writing phase goes on until its file is written. Taking that for
    /// the end let a quit exit under a converter still writing, and let go of the
    /// target while it still meant to write there. The converter's own
    /// `Cancelled` follows when it stops. A request heard after the converter
    /// has stopped is the end: the converter's own event, if it ever comes,
    /// then finds nothing left to close.
    pub(crate) fn on_long_op_cancelled(&self, ctx: &mut EventContext, event: &Event) {
        let Some(payload) = parse_payload(event) else {
            return;
        };
        let requested = payload
            .get("requested")
            .and_then(|r| r.as_bool())
            .unwrap_or(false);
        if requested && !self.converter_stopped() {
            return;
        }
        let Some(running) = self.end(payload_id(&payload)) else {
            return;
        };
        self.cancelled(ctx, &running);
    }

    /// Say that the import `running` was cancelled, then run what waited for it.
    ///
    /// An `Info` notice, so it replaces the loading one rather than updating it:
    /// updated in place it would keep the spinner (see
    /// [`ProgressToast`](crate::shared::progress_toast::ProgressToast)).
    fn cancelled(&self, ctx: &mut EventContext, running: &Running) {
        self.inner.progress.end_replacing(
            ctx,
            import_toast(
                running.toast_id,
                Toast::info(tr!(project_import_cancelled())),
            )
            .auto_dismiss_after(BRIEF),
        );
        self.after_stop(ctx);
    }

    /// The import failed: the reason, and the technical chain behind **Details**.
    pub(crate) fn on_long_op_failed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(payload) = parse_payload(event) else {
            return;
        };
        let Some(running) = self.end(payload_id(&payload)) else {
            return;
        };
        let message = payload
            .get("error")
            .and_then(|e| e.as_str())
            .unwrap_or_default();
        self.show_failure(
            ctx,
            &running,
            &ProjectImportFailure::from_any_message(message),
        );
        self.after_stop(ctx);
    }

    /// The running import, if `id` names it.
    fn running_for(&self, id: Option<&str>) -> Option<Running> {
        let running = self.inner.running.borrow();
        let running = running.as_ref()?;
        (id == Some(running.op_id.as_str())).then(|| running.clone())
    }

    /// End the running import, if `id` names it: forget it and let go of its
    /// target. Returns what its last toasts need.
    fn end(&self, id: Option<&str>) -> Option<Running> {
        let running = self.running_for(id)?;
        self.inner.running.borrow_mut().take();
        self.inner.active.set(None);
        self.inner.target_hold.release();
        Some(running)
    }

    /// Run what was waiting for the import to end.
    fn after_stop(&self, ctx: &mut EventContext) {
        // Taken out first: a continuation may start another import, or ask to
        // be told when that one stops.
        let waiting = std::mem::take(&mut *self.inner.after_stop.borrow_mut());
        for then in waiting {
            then(ctx);
        }
    }

    /// The report notice slot of the importer whose notice is `notice`.
    fn live_notice(&self, notice: &'static WarningsNotice) -> LiveNotice {
        self.inner
            .notices
            .borrow_mut()
            .entry(notice.action)
            .or_default()
            .clone()
    }

    /// The loading toast the import lives in: a spinner, the percentage and the
    /// phase, and a **Cancel** button, re-shown under the same id so the one
    /// surface updates in place.
    fn progress_toast(&self, running: &Running, percent: f32, stage: Stage) -> Toast {
        let job = self.clone();
        let title = running.progress_title;
        let mut toast = import_toast(running.toast_id, Toast::loading(title()))
            .body(progress_line(percent, stage))
            .action(
                ToastAction::destructive(tr!(project_import_cancel()), move |c| job.cancel(c))
                    .closes_toast(false),
            );
        // One sentence per phase for a screen reader: the phase's name, which
        // stays the same while the entries under it tick by, never the percentage
        // or an entry's title.
        if let Some(phase) = running.phase.clone() {
            toast = toast.announcement(localized(move || {
                tr!(project_import_progress_announcement(
                    title = title().resolve_now(),
                    phase = progress_text::phase_name(&phase)
                        .map(|name| name.resolve_now())
                        .unwrap_or_default()
                ))
                .resolve_now()
            }));
        }
        toast
    }

    /// Raise the error toast: the reason in the body, the technical chain behind
    /// **Details**, persistent so the writer dismisses it themselves.
    fn show_failure(
        &self,
        ctx: &mut EventContext,
        running: &Running,
        failure: &ProjectImportFailure,
    ) {
        let (body, details) = import_failure::failure_text(failure, running.own_words);
        let mut toast = import_toast(
            running.toast_id,
            Toast::error(tr!(project_import_error_title())),
        )
        .body(body)
        .persistent();
        // A failure the importer's own review answers: offer the review again, on the
        // project the import read, before the technical account.
        let offer = running
            .review_again
            .filter(|offer| (offer.applies)(&failure.reason));
        if let Some(offer) = offer {
            let source = running.source.clone();
            toast = toast.action(ToastAction::primary((offer.label)(), move |c| {
                c.send_intent((offer.intent)(source.clone()));
            }));
        }
        let details = move |c: &mut EventContext| {
            MessageBox::warning(tr!(project_import_error_title()))
                .text(lit!(details.clone()))
                .buttons(MessageBoxButtons::Ok)
                .present(c);
        };
        // The one primary action is what fixes the failure, when there is one.
        let details = if offer.is_some() {
            ToastAction::new(tr!(project_import_details()), details)
        } else {
            ToastAction::primary(tr!(project_import_details()), details)
        };
        self.inner.progress.end_in_place(ctx, toast.action(details));
    }
}

/// How far the import has got, as its progress notice says it.
#[derive(Clone)]
enum Stage {
    /// Started, before the converter said anything.
    Starting,
    /// What the converter last reported: a phase code, worded by
    /// `shared::progress_text`, or the title of the entry it is converting, shown
    /// as it was sent.
    Said(String),
}

/// The import's own toast, in whichever state it is in: one entry updated in
/// place under `id`, broadcast (see the module doc), and admitted at `High`
/// priority. At the default priority a toast reaching a corner that already holds
/// five is dropped without being logged, and this one carries the only Cancel,
/// the only Open now and the only report of a failure.
fn import_toast(id: &'static str, toast: Toast) -> Toast {
    toast.id(id).priority(ToastPriority::High).broadcast()
}

/// The notice while a cancelled import's converter finishes what it was doing:
/// still a spinner, with no Cancel left to press.
fn stopping_toast(running: &Running) -> Toast {
    import_toast(running.toast_id, Toast::loading((running.progress_title)()))
        .body(tr!(project_import_stopping()))
}

/// The progress notice's body: the percentage, then the phase or the entry being
/// converted, as `shared::progress_text` says them. Resolved when it is read, so it
/// follows a change of language.
fn progress_line(percent: f32, stage: Stage) -> LocalizedString {
    localized(move || match &stage {
        Stage::Starting => progress_text::percent_body(percent, None).resolve_now(),
        Stage::Said(message) => progress_text::progress_body(percent, message).resolve_now(),
    })
}

#[cfg(test)]
mod tests;
