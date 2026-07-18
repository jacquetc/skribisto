// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Routing the long-operation event stream to the view-model that owns the op in flight.
//!
//! Every background job in the app — import, export, save, save-as, backup, restore —
//! reports through the **same** four `Origin::LongOperation` events. There is no per-feature
//! event channel, so every interested view-model subscribes to all of them and filters by
//! its own operation id. That is why an export's completion does not confuse the backup
//! scheduler: the scheduler sees the event and drops it.
//!
//! Six view-models × up to four events used to mean twenty-odd near-identical
//! `subscribe_event_with_ctx` blocks written out longhand — about 170 lines whose only
//! variation was a method name. [`route`] collapses that to one line per event.
//!
//! `EditorsViewModel`'s own save routing is **not** here: it lives in `App::build` beside
//! the exit guards, because its completion handler also drives the deferred close/switch
//! resumption (see `view_models::save_queue::resume_deferred`) rather than only toasting.

use bastyde::prelude::*;

use frontend::common::event::{Event, LongOperationEvent, Origin, WorkManagementEvent};

use crate::view_models::{
    BackupRestoreViewModel, BackupSchedulerViewModel, ExportViewModel, ImportPlumeViewModel,
    SaveAsViewModel,
};

/// Subscribe `vm` to each `(event, handler)` pair.
///
/// The handlers are plain `fn` pointers rather than closures: every call site is a
/// non-capturing `|v, c, e| v.some_method(c, e)`, which coerces, and requiring that keeps
/// the table honest — a handler cannot quietly capture extra state and become something
/// other than "forward this event to the view-model".
fn route<V: Clone + 'static>(
    ctx: &mut BuildContext,
    vm: &V,
    handlers: &[(LongOperationEvent, fn(&V, &mut EventContext, &Event))],
) {
    for (event, handler) in handlers {
        let vm = vm.clone();
        let handler = *handler;
        ctx.subscribe_event_with_ctx(Origin::LongOperation(event.clone()), move |e: &Event, c| {
            handler(&vm, c, e)
        });
    }
}

pub(in crate::app) fn install(
    ctx: &mut BuildContext,
    save_as_vm: &SaveAsViewModel,
    backup_scheduler: &BackupSchedulerViewModel,
    restore_vm: &BackupRestoreViewModel,
) {
    // Import from Plume Creator — progress / cancel / success / error toast.
    if let Some(vm) = ctx.app_state::<ImportPlumeViewModel>().cloned() {
        route(
            ctx,
            &vm,
            &[
                (
                    LongOperationEvent::Progress,
                    |v: &ImportPlumeViewModel, c, e| v.on_long_op_progress(c, e),
                ),
                (LongOperationEvent::Completed, |v, c, e| {
                    v.on_long_op_completed(c, e)
                }),
                (LongOperationEvent::Cancelled, |v, c, e| {
                    v.on_long_op_cancelled(c, e)
                }),
                (LongOperationEvent::Failed, |v, c, e| {
                    v.on_long_op_failed(c, e)
                }),
            ],
        );
    }

    // Export — same shape as import.
    if let Some(vm) = ctx.app_state::<ExportViewModel>().cloned() {
        route(
            ctx,
            &vm,
            &[
                (LongOperationEvent::Progress, |v: &ExportViewModel, c, e| {
                    v.on_long_op_progress(c, e)
                }),
                (LongOperationEvent::Completed, |v, c, e| {
                    v.on_long_op_completed(c, e)
                }),
                (LongOperationEvent::Cancelled, |v, c, e| {
                    v.on_long_op_cancelled(c, e)
                }),
                (LongOperationEvent::Failed, |v, c, e| {
                    v.on_long_op_failed(c, e)
                }),
            ],
        );
    }

    // Save As — no progress toast; on success it records the new file_name/shape into
    // WorkInfo synchronously on the UI thread (save_as itself is read-only).
    route(
        ctx,
        save_as_vm,
        &[
            (LongOperationEvent::Completed, |v: &SaveAsViewModel, c, e| {
                v.on_long_op_completed(c, e)
            }),
            (LongOperationEvent::Failed, |v, c, e| {
                v.on_long_op_failed(c, e)
            }),
        ],
    );

    // Backup — records the per-destination success hash + path, shows the progress and
    // summary toasts, and for an on-close backup performs the deferred close. (Retention
    // runs *inside* the operation — see the engine.) No Cancelled arm: a backup is not
    // user-cancellable.
    route(
        ctx,
        backup_scheduler,
        &[
            (
                LongOperationEvent::Progress,
                |v: &BackupSchedulerViewModel, c, e| v.on_long_op_progress(c, e),
            ),
            (LongOperationEvent::Completed, |v, c, e| {
                v.on_long_op_completed(c, e)
            }),
            (LongOperationEvent::Failed, |v, c, e| {
                v.on_long_op_failed(c, e)
            }),
        ],
    );

    // Restore — its own `save_as` op. On success it records WorkInfo and leaves backup mode.
    route(
        ctx,
        restore_vm,
        &[
            (
                LongOperationEvent::Completed,
                |v: &BackupRestoreViewModel, c, e| v.on_long_op_completed(c, e),
            ),
            (LongOperationEvent::Failed, |v, c, e| {
                v.on_long_op_failed(c, e)
            }),
        ],
    );

    // The progress-recorder cadence (silent, no toast): each `save_work` fires a throttled
    // `count_words`, and its completion records today's `ProgressSnapshot`.
    //
    // Hand-written rather than routed: it also listens on a `WorkManagement` origin, uses
    // plain `subscribe_event` (it shows no UI, so it needs no `EventContext`), and maps two
    // different events onto one method.
    if let Some(recorder) = ctx
        .app_state::<crate::view_models::ProgressRecorder>()
        .cloned()
    {
        {
            let r = recorder.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::SaveWork),
                move |_e: &Event| r.recount_throttled(),
            );
        }
        {
            let r = recorder.clone();
            ctx.subscribe_event(
                Origin::LongOperation(LongOperationEvent::Completed),
                move |e: &Event| r.on_completed(e),
            );
        }
        for event in [LongOperationEvent::Failed, LongOperationEvent::Cancelled] {
            let r = recorder.clone();
            ctx.subscribe_event(Origin::LongOperation(event), move |e: &Event| {
                r.on_failed_or_cancelled(e)
            });
        }
    }
}
