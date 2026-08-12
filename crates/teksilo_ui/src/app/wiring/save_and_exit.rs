// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Deferred close / switch resumption off long-operation save events.
//!
//! The window close guard and `work.close` arm [`PendingExit`]; that asks for a
//! disk save and remembers the edit sequence it will cover. The close (or parked
//! project switch) is performed only once *that* sequence is on disk — so the
//! async write is awaited, never raced.
//!
//! Extracted from `App::build` so that god-function is not also the exit state
//! machine.

use std::cell::Cell;
use std::rc::Rc;

use teksilo::prelude::*;
use teksilo::widgets::Toast;

use frontend::common::event::{Event, LongOperationEvent, Origin};

use crate::app::PendingExit;
use crate::app_ids::AppIds;
use crate::backup::BackupSchedulerViewModel;
use crate::editors::EditorsViewModel;
use crate::project::{ProjectSwitchViewModel, QuitSequencer};
use crate::save::{DeferredResume, SaveStateViewModel};
use crate::toast_scope::ToastWorkExt;
use crate::workspace_layout::WorkspaceLayoutViewModel;

/// Dedup id (Work-scoped — see [`crate::toast_scope`]) shared by every "the save
/// failed" toast [`abandon_deferred`] raises. One id on purpose: a specific
/// report and a generic one are two texts for the *same* failure, so the
/// registry's update-in-place keeps exactly one toast per Work no matter which
/// window's subscriber runs first, and the specific text wins either way.
const SAVE_FAILED_TOAST_ID: &str = "save.failed";

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
}

/// Install the pending-exit effect and the long-op Completed/Failed handlers that
/// resume a deferred close or project switch.
pub(in crate::app) fn install(ctx: &mut BuildContext, deps: &SaveAndExitDeps) {
    // The window close guard and the `work.close` action set `pending_exit`; that
    // asks for a disk save and remembers the edit sequence it will cover. The
    // close is performed only once *that* sequence is on disk.
    {
        let editors = deps.editors.clone();
        let exit_seq = deps.exit_seq.clone();
        let pending = deps.pending_exit.clone();
        ctx.effect(&deps.pending_exit, move |pe| {
            if *pe == PendingExit::None {
                return;
            }
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
        let scheduler = deps.backup_scheduler.clone();
        let switch = deps.project_switch.clone();
        let workspace_layout = deps.workspace_layout.clone();
        let ids = deps.ids.clone();
        let quit_for_completed = deps.quit.clone();
        let save_state_for_completed = deps.save_state.clone();
        ctx.subscribe_event_with_ctx(
            Origin::LongOperation(LongOperationEvent::Completed),
            move |e: &Event, c| {
                quit_for_completed.on_long_op_completed(e, c);
                let Some(landed) = editors.on_save_completed(e) else {
                    return;
                };
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
                    DeferredResume::Abandon => abandon_deferred(
                        c,
                        &pending,
                        &exit_seq,
                        &switch,
                        None,
                        ids.work_id.get(),
                        &save_state_for_completed,
                        e,
                    ),
                    DeferredResume::Close => {
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

    // A failed save is always a toast. If a close or switch was parked behind it,
    // it is dropped and the message says so.
    {
        let editors = deps.editors.clone();
        let pending = deps.pending_exit.clone();
        let exit_seq = deps.exit_seq.clone();
        let switch = deps.project_switch.clone();
        let ids = deps.ids.clone();
        let quit_for_failed = deps.quit.clone();
        let save_state_for_failed = deps.save_state.clone();
        ctx.subscribe_event_with_ctx(
            Origin::LongOperation(LongOperationEvent::Failed),
            move |e: &Event, c| {
                quit_for_failed.on_long_op_failed(e, c);
                let Some(error) = editors.on_save_failed(e) else {
                    return;
                };
                abandon_deferred(
                    c,
                    &pending,
                    &exit_seq,
                    &switch,
                    Some(&error),
                    ids.work_id.get(),
                    &save_state_for_failed,
                    e,
                );
            },
        );
    }
}

/// The disk write a deferred flow was waiting on will never land — it failed, or a
/// follow-up save could not even be started. Drop whatever was parked on it and say so.
#[allow(clippy::too_many_arguments)]
pub(in crate::app) fn abandon_deferred(
    c: &mut EventContext,
    pending: &Signal<PendingExit>,
    exit_seq: &Rc<Cell<Option<u64>>>,
    switch: &ProjectSwitchViewModel,
    error: Option<&str>,
    work_id: Option<u64>,
    save_state: &SaveStateViewModel,
    event: &Event,
) {
    if pending.get() != PendingExit::None {
        pending.set(PendingExit::None);
        exit_seq.set(None);
        switch.cancel();
        save_state.note_specific_failure_report(event);
        c.show_toast(
            Toast::error(match error {
                Some(e) => tr!(close_save_failed(error = e.to_string())),
                None => tr!(close_save_not_started()),
            })
            .scoped_id(SAVE_FAILED_TOAST_ID, work_id)
            .target_work(work_id),
        );
        return;
    }
    if switch.on_save_failed(c, error) {
        save_state.note_specific_failure_report(event);
        return;
    }
    if !save_state.claim_generic_failure_report(event) {
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
