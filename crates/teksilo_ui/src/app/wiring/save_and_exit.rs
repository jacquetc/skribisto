// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Deferred close / switch resumption off long-operation save events.
//!
//! The window close guard and `work.close` arm [`PendingExit`]; that asks for a
//! disk save and remembers the edit sequence it will cover. The close (or parked
//! project switch) is performed only once *that* sequence is on disk — so the
//! async write is awaited, never raced.
//!
//! When the save a close or a switch was parked on fails, the writer is asked
//! ([`crate::project::save_failed_question`]) instead of the request being
//! dropped: with autosave on, a dropped close saved again on the next Ctrl+W,
//! failed again and stayed again, with no way out.
//!
//! Extracted from `App::build` so that god-function is not also the exit state
//! machine.

use std::cell::Cell;
use std::rc::Rc;

use frontend::AppContext;

use teksilo::prelude::*;
use teksilo::widgets::Toast;

use frontend::common::event::{Event, LongOperationEvent, Origin};

use crate::app::PendingExit;
use crate::app_ids::AppIds;
use crate::backup::BackupSchedulerViewModel;
use crate::editors::EditorsViewModel;
use crate::project::save_failed_question::{Leaving, SaveFailedAnswer, ask_after_failed_save};
use crate::project::{ProjectSwitchViewModel, QuitSequencer};
use crate::save::{DeferredResume, SaveStateViewModel};
use crate::toast_scope::ToastWorkExt;
use crate::workspace_layout::WorkspaceLayoutViewModel;

/// Dedup id (Work-scoped — see [`crate::toast_scope`]) of the plain "the save
/// failed" toast [`abandon_deferred`] raises when nothing was parked on the save.
/// One id per Work, so N windows on it keep exactly one such toast.
const SAVE_FAILED_TOAST_ID: &str = "save.failed";

/// Dedup id (Work-scoped) of the "these documents could not be handed to the
/// save" toast [`report_flush_failures`] raises.
const FLUSH_FAILED_TOAST_ID: &str = "save.flush_failed";

/// How many extra saves a deferred close may ask for while typing continues.
const MAX_DEFERRED_RESAVES: u8 = 3;

/// Handles the install function needs from `App::build`.
pub(in crate::app) struct SaveAndExitDeps {
    pub editors: EditorsViewModel,
    pub pending_exit: Signal<PendingExit>,
    pub exit_seq: Rc<Cell<Option<u64>>>,
    pub backup_scheduler: BackupSchedulerViewModel,
    pub project_switch: ProjectSwitchViewModel,
    /// `None` for an attached window that does not own the desk.
    pub workspace_layout: Option<WorkspaceLayoutViewModel>,
    pub ids: AppIds,
    pub quit: QuitSequencer,
    pub save_state: SaveStateViewModel,
    pub app_ctx: Rc<AppContext>,
    /// This window's Work's title, for the question asked when a parked save
    /// fails.
    pub work_title: Signal<String>,
}

/// What a parked close or switch needs once its save will never land: what was
/// parked, and what leaving without saving takes. Cheap to clone (handles).
#[derive(Clone)]
pub(in crate::app) struct ParkedOnSave {
    pub pending: Signal<PendingExit>,
    pub exit_seq: Rc<Cell<Option<u64>>>,
    pub switch: ProjectSwitchViewModel,
    pub save_state: SaveStateViewModel,
    pub app_ctx: Rc<AppContext>,
    pub ids: AppIds,
    pub scheduler: BackupSchedulerViewModel,
    pub work_title: Signal<String>,
}

impl SaveAndExitDeps {
    fn parked(&self) -> ParkedOnSave {
        ParkedOnSave {
            pending: self.pending_exit.clone(),
            exit_seq: self.exit_seq.clone(),
            switch: self.project_switch.clone(),
            save_state: self.save_state.clone(),
            app_ctx: self.app_ctx.clone(),
            ids: self.ids.clone(),
            scheduler: self.backup_scheduler.clone(),
            work_title: self.work_title.clone(),
        }
    }
}

/// Install the pending-exit effect and the long-op Completed/Failed handlers that
/// resume a deferred close or project switch.
pub(in crate::app) fn install(ctx: &mut BuildContext, deps: &SaveAndExitDeps) {
    // Extra saves this window's deferred close has asked for — see the `Close`
    // arm below. Reset each time a close is armed.
    let resaves = Rc::new(Cell::new(0u8));
    // The window close guard and the `work.close` action set `pending_exit`; that
    // asks for a disk save and remembers the edit sequence it will cover. The
    // close is performed only once *that* sequence is on disk.
    {
        let editors = deps.editors.clone();
        let exit_seq = deps.exit_seq.clone();
        let pending = deps.pending_exit.clone();
        let resaves = resaves.clone();
        ctx.effect(&deps.pending_exit, move |pe| {
            if *pe == PendingExit::None {
                return;
            }
            resaves.set(0);
            match editors.request_save() {
                Some(covers) => exit_seq.set(Some(covers)),
                // The command could not be issued, so no operation exists — no
                // completion and no failure event will ever arrive. Disarm instead.
                None => {
                    exit_seq.set(None);
                    pending.set(PendingExit::None);
                    eprintln!("skribisto: could not start the save for a deferred close");
                }
            }
        });
    }

    // Both deferred flows — the close above and a parked project switch — resume
    // here off the long-operation events. They wait on the edit *sequence* their
    // save covers, not on "a save finished" (SaveQueue coalesces).
    {
        let editors = deps.editors.clone();
        let pending = deps.pending_exit.clone();
        let exit_seq = deps.exit_seq.clone();
        let resaves = resaves.clone();
        let scheduler = deps.backup_scheduler.clone();
        let switch = deps.project_switch.clone();
        let workspace_layout = deps.workspace_layout.clone();
        let ids = deps.ids.clone();
        let quit_for_completed = deps.quit.clone();
        let parked = deps.parked();
        ctx.subscribe_event_with_ctx(
            Origin::LongOperation(LongOperationEvent::Completed),
            move |e: &Event, c| {
                quit_for_completed.on_long_op_completed(e, c);
                let Some(landed) = editors.on_save_completed(e) else {
                    return;
                };
                report_flush_failures(c, &editors, ids.work_id.get());
                if let Some(layout) = &workspace_layout {
                    layout.capture();
                }
                let saved = landed.saved_seq;
                let pe = pending.get();
                match crate::save::resume_deferred(
                    landed.follow_up_failed,
                    saved,
                    pe != PendingExit::None,
                    exit_seq.get(),
                ) {
                    DeferredResume::Abandon => abandon_deferred(c, &parked, None, e),
                    DeferredResume::Close => {
                        // Text typed while that save was writing is not in it.
                        // Unless the flush itself failed — it would again, and
                        // the toast above has said so — ask for one more write
                        // and keep the close parked on it. Bounded, so a project
                        // that never settles still closes after a few.
                        if editors.is_unsaved()
                            && !editors.last_flush_failed()
                            && resaves.get() < MAX_DEFERRED_RESAVES
                        {
                            resaves.set(resaves.get() + 1);
                            match editors.request_save() {
                                Some(covers) => exit_seq.set(Some(covers)),
                                None => abandon_deferred(c, &parked, None, e),
                            }
                            return;
                        }
                        pending.set(PendingExit::None);
                        exit_seq.set(None);
                        switch.cancel();
                        scheduler.on_close_flow(c, pe);
                    }
                    DeferredResume::Wait => {}
                    DeferredResume::Switch => switch.on_saved(c, saved),
                }
            },
        );
    }

    // A failed save is always reported. If a close or switch was parked behind
    // it, the writer is asked what to do about it; otherwise it is a toast.
    {
        let editors = deps.editors.clone();
        let quit_for_failed = deps.quit.clone();
        let parked = deps.parked();
        ctx.subscribe_event_with_ctx(
            Origin::LongOperation(LongOperationEvent::Failed),
            move |e: &Event, c| {
                quit_for_failed.on_long_op_failed(e, c);
                let Some(error) = editors.on_save_failed(e) else {
                    return;
                };
                abandon_deferred(c, &parked, Some(&error), e);
            },
        );
    }
}

/// The disk write a deferred flow was waiting on will never land — it failed, or a
/// follow-up save could not even be started (`error` is then `None`).
///
/// A close parked on it is not dropped: the writer is asked whether to close
/// without saving or stay ([`ask_after_failed_save`]), and only the explicit
/// button closes, the way Discard does (no on-close backup of edits being
/// left behind). A parked switch is asked the same through
/// [`ProjectSwitchViewModel::on_save_failed`]. With nothing parked, it is the
/// plain save-failure toast, once per Work.
pub(in crate::app) fn abandon_deferred(
    c: &mut EventContext,
    parked: &ParkedOnSave,
    error: Option<&str>,
    event: &Event,
) {
    let work_id = parked.ids.work_id.get();
    let outcome = parked.pending.get();
    if outcome != PendingExit::None {
        parked.pending.set(PendingExit::None);
        parked.exit_seq.set(None);
        parked.switch.cancel();
        parked.save_state.note_specific_failure_report(event);
        let leave = parked.clone();
        ask_after_failed_save(
            c,
            Leaving::Close,
            Some(parked.work_title.get()),
            error,
            move |answer, c| {
                if answer == SaveFailedAnswer::LeaveWithoutSaving {
                    crate::app::perform_exit(
                        outcome,
                        &leave.app_ctx,
                        &leave.ids,
                        leave.scheduler.workspace_layout(),
                        c,
                    );
                }
            },
        );
        return;
    }
    if parked.switch.on_save_failed(c, error) {
        parked.save_state.note_specific_failure_report(event);
        return;
    }
    if !parked.save_state.claim_generic_failure_report(event) {
        return;
    }
    if let Some(e) = error {
        c.show_toast(
            Toast::error(tr!(save_error(error = e.to_string())))
                .scoped_id(SAVE_FAILED_TOAST_ID, work_id)
                .target_work(work_id),
        );
    } else {
        c.show_toast(
            Toast::error(tr!(save_not_started()))
                .scoped_id(SAVE_FAILED_TOAST_ID, work_id)
                .target_work(work_id),
        );
    }
}

/// Say which documents the save that just landed could not include.
///
/// A flush fails on a timer (autosave) as often as on a keystroke (Ctrl+S), and
/// the timer has no `EventContext` to speak with; the completion of the save the
/// flush preceded is the first moment with one. The project has stayed unsaved
/// in the meantime (`SaveStateViewModel::request_save_after_flush`), so the glyph
/// already says so — this says *what*, and that the words are still in their
/// editor and nowhere else.
fn report_flush_failures(c: &mut EventContext, editors: &EditorsViewModel, work_id: Option<u64>) {
    let failures = editors.take_flush_failures();
    let Some(first) = failures.first() else {
        return;
    };
    let titles = failures
        .iter()
        .map(|f| {
            if f.title.trim().is_empty() {
                tr!(untitled()).resolve_now()
            } else {
                f.title.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    c.show_toast(
        Toast::error(tr!(save_flush_failed(
            titles = titles,
            error = first.error.clone()
        )))
        .scoped_id(FLUSH_FAILED_TOAST_ID, work_id)
        .target_work(work_id),
    );
}

#[cfg(all(test, not(feature = "mocks")))]
mod tests {
    use super::*;
    use crate::test_support::{RealProject, answer_dialog, answer_dialog_with, press};
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::core::window::{TeksiloWindowId, WindowConfig, WindowOps, WindowState};
    use teksilo::widgets::StandardButton;

    /// A window sink that opens nothing but counts what it was asked to open,
    /// where a bare tree's own refuses outright: closing a project opens the
    /// Launcher when it was the last one.
    #[derive(Default)]
    struct CountingWindows {
        opened: u64,
    }

    impl WindowOps for CountingWindows {
        fn open_window(&mut self, _config: WindowConfig) -> TeksiloWindowId {
            self.opened += 1;
            TeksiloWindowId::new(1000 + self.opened)
        }
        fn find_window(&self, _string_id: &str) -> Option<TeksiloWindowId> {
            None
        }
        fn window_state(&self, _id: TeksiloWindowId) -> Option<WindowState> {
            None
        }
        fn windows(&self) -> Vec<WindowState> {
            Vec::new()
        }
        fn focus_window(&mut self, _id: TeksiloWindowId) {}
        fn close_window_by_id(&mut self, _id: TeksiloWindowId) {}
    }

    /// A close of `project` parked on a save, as Ctrl+W with autosave on parks it.
    fn parked_close(project: &RealProject) -> ParkedOnSave {
        let session = project.session();
        session.single_work.title().set("The Lighthouse".into());
        ParkedOnSave {
            pending: Signal::new(PendingExit::ReturnToLauncher),
            exit_seq: Rc::new(Cell::new(Some(3))),
            switch: ProjectSwitchViewModel::new(
                project.app_ctx.clone(),
                Signal::new(true),
                Signal::new(false),
                Signal::new(true),
            ),
            save_state: session.save_state.clone(),
            app_ctx: project.app_ctx.clone(),
            ids: project.ids.clone(),
            scheduler: session.backup_scheduler.clone(),
            work_title: session.single_work.title(),
        }
    }

    /// The save the close was parked on failed, as on a full disk.
    fn the_save_fails(tree: &mut WidgetTree, parked: &ParkedOnSave) {
        let parked = parked.clone();
        press(tree, move |c| {
            let failed = Event {
                origin: Origin::LongOperation(LongOperationEvent::Failed),
                ids: Vec::new(),
                data: Some(r#"{"id":"save-1","error":"No space left on device"}"#.into()),
            };
            abandon_deferred(c, &parked, Some("No space left on device"), &failed);
        });
    }

    fn open_works(project: &RealProject) -> usize {
        frontend::commands::work_commands::get_all_work(&project.app_ctx)
            .map(|works| works.len())
            .unwrap_or_default()
    }

    /// With autosave on, a close whose save fails asks, naming the project, and
    /// "Close without saving" closes it. Before, the close was dropped with a
    /// toast, and the next Ctrl+W did the same, for as long as the disk was full.
    #[test]
    fn a_close_whose_save_failed_asks_and_can_close_without_saving() {
        let project = RealProject::empty_novel();
        let parked = parked_close(&project);
        let mut tree = WidgetTree::new();
        the_save_fails(&mut tree, &parked);
        assert_eq!(
            parked.pending.get(),
            PendingExit::None,
            "the close is no longer waiting on a save"
        );
        let mut windows = CountingWindows::default();
        answer_dialog_with(
            &mut tree,
            &tr!(save_failed_close_title(title = "The Lighthouse")).resolve_now(),
            &tr!(save_failed_close()).resolve_now(),
            &mut windows,
        );
        assert_eq!(open_works(&project), 0, "the project was closed");
        assert_eq!(windows.opened, 1, "and the Launcher took its place");
    }

    /// Cancel keeps the project open, with everything in it.
    #[test]
    fn cancelling_after_a_failed_save_keeps_the_project_open() {
        let project = RealProject::empty_novel();
        let parked = parked_close(&project);
        let mut tree = WidgetTree::new();
        the_save_fails(&mut tree, &parked);
        answer_dialog(
            &mut tree,
            &tr!(save_failed_close_title(title = "The Lighthouse")).resolve_now(),
            &StandardButton::Cancel.default_label().resolve_now(),
        );
        assert_eq!(open_works(&project), 1, "the project stays open");
        assert_eq!(parked.pending.get(), PendingExit::None);
    }
}
