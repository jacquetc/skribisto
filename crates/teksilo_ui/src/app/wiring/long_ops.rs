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
//! [`route`] collapses the resulting six-view-models × up-to-four-events grid of
//! near-identical `subscribe_event_with_ctx` blocks to one line per event.
//!
//! `EditorsViewModel`'s own save routing is **not** here: it lives in `App::build` beside
//! the exit guards, because its completion handler also drives the deferred close/switch
//! resumption (see `crate::save::save_queue::resume_deferred`) rather than only toasting.

use teksilo::prelude::*;

use frontend::common::event::{Event, LongOperationEvent, Origin, WorkManagementEvent};

use crate::backup::{BackupRestoreViewModel, BackupSchedulerViewModel};
use crate::export::ExportViewModel;
use crate::import_document::ImportDocumentViewModel;
use crate::import_plume::ImportPlumeViewModel;
use crate::mentions::MentionIndex;
use crate::save::SaveAsViewModel;
use crate::shared::ProgressRecorder;

/// One row of a long-operation dispatch table: which event, and what to run on
/// the view-model when it arrives.
type LongOpHandler<V> = (LongOperationEvent, fn(&V, &mut EventContext, &Event));

/// Subscribe `vm` to each `(event, handler)` pair.
///
/// The handlers are plain `fn` pointers rather than closures: every call site is a
/// non-capturing `|v, c, e| v.some_method(c, e)`, which coerces, and requiring that keeps
/// the table honest — a handler cannot quietly capture extra state and become something
/// other than "forward this event to the view-model".
fn route<V: Clone + 'static>(ctx: &mut BuildContext, vm: &V, handlers: &[LongOpHandler<V>]) {
    for (event, handler) in handlers {
        let vm = vm.clone();
        let handler = *handler;
        ctx.subscribe_event_with_ctx(Origin::LongOperation(event.clone()), move |e: &Event, c| {
            handler(&vm, c, e)
        });
    }
}

// One parameter per view-model that owns a background job; a bundle struct here would
// be a second `CommandDeps` whose only job is to be unpacked one field per `route` call.
#[allow(clippy::too_many_arguments)]
pub(in crate::app) fn install(
    ctx: &mut BuildContext,
    save_as_vm: &SaveAsViewModel,
    backup_scheduler: &BackupSchedulerViewModel,
    restore_vm: &BackupRestoreViewModel,
    export_vm: &ExportViewModel,
    import_document: &ImportDocumentViewModel,
    mention_index: &MentionIndex,
    progress_recorder: &ProgressRecorder,
) {
    // Import from Plume Creator — progress / cancel / success / error toast.
    // Its own table, on the view-model, because the Launcher subscribes to the
    // same four events on its own widget tree (see
    // `ImportPlumeViewModel::wire_long_operation`) and one of the two windows
    // that can start this import must not carry a private copy of its wiring.
    if let Some(vm) = ctx.app_state::<ImportPlumeViewModel>().cloned() {
        vm.wire_long_operation(ctx);
    }

    // Export — same shape as import. Threaded in (Tier 2, per-open-Work), not
    // resolved via `ctx.app_state::<ExportViewModel>()`: see `CommandDeps::export`'s
    // doc for why that lookup would be wrong the moment a second Work opens in a
    // second window.
    route(
        ctx,
        export_vm,
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

    // Import documents — the *analysis* half only (`apply_document_import` is synchronous
    // and undoable, so it reports nothing here). Its progress and its Cancel live inside the
    // wizard rather than in a toast: the wizard is modal, so a toast behind it would be a
    // surface the writer can see and not reach.
    //
    // Threaded in (Tier 3, per WINDOW — see `CommandDeps::import_document`), never
    // `ctx.app_state::<ImportDocumentViewModel>()`. Note this subscribes the view-model, not
    // the panel: the modal is built and torn down around it, and a plan that arrived while
    // the wizard was rebuilding must still land.
    route(
        ctx,
        import_document,
        &[
            (
                LongOperationEvent::Progress,
                |v: &ImportDocumentViewModel, c, e| v.on_long_op_progress(c, e),
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

    // Save As — no progress toast; on success it records the new file_name/shape into
    // WorkInfo synchronously on the UI thread (save_as itself is read-only).
    route(
        ctx,
        save_as_vm,
        &[
            (
                LongOperationEvent::Completed,
                |v: &SaveAsViewModel, c, e| v.on_long_op_completed(c, e),
            ),
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
    //
    // Threaded in (Tier 2, per-open-Work — it captures its own `AppIds`, see
    // `crate::shared::progress_recorder`'s module doc), not resolved via
    // `ctx.app_state::<ProgressRecorder>()`: that lookup can only ever answer with
    // whichever window built `main`'s bootstrap session, exactly the bug `export_vm`
    // and `mention_index` were fixed for above/below.
    {
        let recorder = progress_recorder.clone();
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
    // The mention index's cadence. Same shape as the progress recorder above, with one
    // difference: it also fires on Load and New. That recorder deliberately does not — a
    // historical word-count point should not be recorded for a project merely opened — but
    // an index is live state, and a roster that stays empty until the first save would make
    // the feature look broken on every project you open.
    //
    // Threaded in (Tier 2, per-open-Work), not resolved via
    // `ctx.app_state::<MentionIndex>()` — see the `progress_recorder` note just above.
    {
        let index = mention_index.clone();
        for event in [WorkManagementEvent::LoadWork, WorkManagementEvent::NewWork] {
            let i = index.clone();
            ctx.subscribe_event(Origin::WorkManagement(event), move |_e: &Event| i.rescan());
        }
        // ...and scan **now**, for the project that is already open by the time this runs.
        //
        // The subscription above only ever hears a `LoadWork` fired *after* it exists, and
        // on the ordinary startup path the project is opened by `startup::launch_maintenance`
        // before `App::build` installs any of this. So the event that should have seeded the
        // index has already been and gone with nobody listening, and the index stays empty
        // for the whole session unless the writer happens to edit a tag or save. What that
        // looks like from the writer's chair is the feature simply being broken: the Cast
        // picker offers nobody, an already-pinned cast member renders as a nameless row
        // (`cast_for` resolves a title through the discoverable table and falls back to an
        // empty string), and "Set point of view" opens an empty list on a project full of
        // discoverable notes.
        //
        // `fire` returns early when no project is open, so this is a no-op on a window that
        // opens onto nothing, and the `active` guard in `rescan` means a genuine `LoadWork`
        // arriving immediately after cannot start a second scan on top of this one.
        //
        // Same shape, and the same reason, as `wiring::spellcheck`'s own eager
        // `dictionaries.rescan()`: a registry that is only reconciled by future events is
        // wrong on arrival exactly once, at the moment the writer first looks at it.
        index.rescan();
        // The alias table changing is what makes a roster appear at all, so those events
        // rescan straight away rather than waiting for a save. A tag gaining its story-bible
        // flag, or an item gaining a tag or an alias, is a deliberate act — and the writer is
        // looking at the Inspector when they do it.
        for ev in [
            frontend::common::event::EntityEvent::Created,
            frontend::common::event::EntityEvent::Updated,
            frontend::common::event::EntityEvent::Removed,
        ] {
            for entity in [
                frontend::common::event::DirectAccessEntity::BinderTag(ev.clone()),
                frontend::common::event::DirectAccessEntity::BinderItem(ev.clone()),
            ] {
                let i = index.clone();
                ctx.subscribe_event(Origin::DirectAccess(entity), move |_e: &Event| i.rescan());
            }
        }
        // Prose changes only reach the index on a save, and are throttled — autosave fires
        // every few seconds. The scene being *written* does not wait for this: the Inspector
        // rescans the focused item's own prose live (see `MentionIndex::roster_for`).
        {
            let i = index.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::SaveWork),
                move |_e: &Event| i.rescan_throttled(),
            );
        }
        {
            let i = index.clone();
            ctx.subscribe_event(
                Origin::LongOperation(LongOperationEvent::Completed),
                move |e: &Event| i.on_completed(e),
            );
        }
        for event in [LongOperationEvent::Failed, LongOperationEvent::Cancelled] {
            let i = index.clone();
            ctx.subscribe_event(Origin::LongOperation(event), move |e: &Event| {
                i.on_failed_or_cancelled(e)
            });
        }
    }
}
