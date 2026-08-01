// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Project lifecycle event wiring — Load / New / Close / Attach and the
//! post-Load backup sniff.
//!
//! Extracted from `App::build` so that god-function is not also the multi-window
//! project-lifecycle bus. Guarded subscribe helpers and window binding live
//! alongside.

use std::rc::Rc;

use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::prelude::*;
use bastyde::widgets::{
    DockOpenLocation, DockSide, DockWidgetId, Toast, ToastAction,
};

use frontend::AppContext;
use frontend::common::event::{Event, Origin, WorkManagementEvent};

use crate::app_ids::AppIds;
use crate::sessions::WorkSession;
use crate::settings::SettingsPanel;
use crate::singles::SingleWork;
use crate::toast_scope::ToastWorkExt;
use crate::view_models::{
    BackupRestoreViewModel, BackupSchedulerViewModel, BackupSettingsViewModel, OutlineViewModel,
    TreeExpansionViewModel, WorkspaceLayoutViewModel,
};

pub(in crate::app) use super::guards::{on_own_close, on_own_load_or_new};
pub(in crate::app) use super::window_bind::bind_window_to_work;

/// Second `LoadWork` subscriber: sniff backup-ness, restore desk, on-open backup
/// or choice modal. Must run **after** the first LoadWork seed (order of
/// registration in `App::build`).
pub(in crate::app) struct BackupSniffDeps {
    pub app_ctx: Rc<AppContext>,
    pub ids: AppIds,
    pub tree_expansion: TreeExpansionViewModel,
    pub backup_mode: Signal<bool>,
    pub backup_context: Signal<Option<crate::backup::BackupContext>>,
    pub restore_vm: BackupRestoreViewModel,
    pub single_work: SingleWork,
    pub backup_settings: BackupSettingsViewModel,
    pub backup_scheduler: BackupSchedulerViewModel,
    pub workspace_layout: Option<WorkspaceLayoutViewModel>,
    pub outline: OutlineViewModel,
    pub trash_dock: DockWidgetId,
    pub session: WorkSession,
}

pub(in crate::app) fn install_backup_sniff(ctx: &mut BuildContext, deps: BackupSniffDeps) {
    // Detect "a backup file was opened" and enter backup mode. A separate
    // `subscribe_event_with_ctx` (needs an `EventContext` to present the choice
    // modal) reads the just-loaded path and sniffs its manifest. Opening a
    // backup always happens in its own process (the redirect in the open entry
    // points), so this only ever fires in a window dedicated to that backup.
    {
        let app_ctx = deps.app_ctx;
        let ids = deps.ids;
        let tree_expansion = deps.tree_expansion;
        let backup_mode = deps.backup_mode;
        let backup_context = deps.backup_context;
        let restore_vm = deps.restore_vm;
        let single_work = deps.single_work;
        let backup_settings = deps.backup_settings;
        let backup_scheduler = deps.backup_scheduler;
        let workspace_layout = deps.workspace_layout;
        // A saved per-work layout serialized before the trash dock existed
        // won't contain it; re-mounting it after restore keeps the trash panel
        // reachable in every project (and it re-saves with trash thereafter).
        let outline = deps.outline;
        let trash_docking = outline.docking();
        // The outline's tree model, for re-applying its remembered chevrons below.
        let outline_model = outline.model();
        let trash_dock = deps.trash_dock;
        let outline_dock = outline.dock_id();
        let session_for_nudge = deps.session;
        ctx.subscribe_event_with_ctx(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |e: &Event, c: &mut EventContext| {
                    // Guarded (strict form): by the time this fires, the first
                    // `LoadWork` subscriber above (registered earlier in this same
                    // `build`, so it always runs first) has already re-seeded
                    // `ids.work_id` for THIS window's own load — so a sibling
                    // window's Load, which never touches this window's `ids`,
                    // reliably fails this check instead of popping this window's
                    // backup-choice modal / nudge toast for someone else's project.
                    if !ids.is_event_for_my_work(&e.ids) {
                        return;
                    }
                    // Resolved through the Phase-1 seam (`ids.work_info_id`, already
                    // re-seeded by the first `LoadWork` subscriber above — registered
                    // earlier in this same `build`, so it always runs first),
                    // rather than `get_all_work_info(&app_ctx)`'s first entry: see
                    // `main::current_project_path`'s doc for why that stopped being
                    // a safe stand-in for "this window's project" once the backend
                    // scoped `WorkInfo` to support more than one open `Work`.
                    let path = ids.work_info_id.get().and_then(|id| {
                        frontend::commands::work_info_commands::get_work_info(&app_ctx, &id)
                            .ok()
                            .flatten()
                            .and_then(|wi| wi.file_name)
                    });
                    // Sniff the manifest once: drives both the backup-mode branch
                    // below and the workspace-layout restore.
                    let backup = path.as_deref().and_then(crate::backup::backup_context_for);
                    // Restore the desk now that backup-ness is known: a backup gets a
                    // clean default desk (its saved layout is the *source's*), a normal
                    // project its saved tabs + docks. The first `LoadWork` subscriber
                    // has already re-seeded the ids/singles and cleared the old tabs.
                    if let Some(layout) = &workspace_layout {
                        layout.restore(backup.is_some());
                    }
                    // …and the outline's remembered chevrons, from the same moment and
                    // for the same reason: a backup shares its source project's uid, so
                    // its saved expansion is the *source's* and it gets the default tree
                    // instead. Keyed by durable uid, so what was written is what is read
                    // — no translation against the freshly re-minted store ids.
                    if backup.is_none() {
                        let remembered = tree_expansion.outline_expanded();
                        if !remembered.is_empty() {
                            outline_model.set_expanded_keys(&remembered);
                        }
                    }
                    // Ensure the trash dock survives restoring a pre-trash layout.
                    // `open_dock` selects the new tab, so re-reveal the outline to
                    // keep it the foreground leading panel on launch (the app default).
                    if !trash_docking.dock_open_signal(trash_dock).get() {
                        trash_docking.open_dock(
                            trash_dock,
                            DockOpenLocation::side(DockSide::Leading).new_tab(),
                        );
                        trash_docking.reveal_dock(outline_dock);
                    }
                    match backup {
                        Some(bc) => {
                            backup_mode.set(true);
                            backup_context.set(Some(bc.clone()));
                            let restore = restore_vm.clone();
                            let backup_mode_for_panel = backup_mode.clone();
                            let backup_context_for_panel = backup_context.clone();
                            c.present_modal(
                                ModalRequest::deferred(move |t| {
                                    t.add(crate::backup::choice_panel::BackupChoicePanel::new(
                                        restore.clone(),
                                        bc.clone(),
                                        backup_mode_for_panel.clone(),
                                        backup_context_for_panel.clone(),
                                    ))
                                })
                                .presentation(ModalPresentation::InTree)
                                .title(tr!(backup_choice_title()))
                                .close_behavior(ModalCloseBehavior::Manual)
                                .size(560, 320),
                            );
                        }
                        None => {
                            // A normal project — never in backup mode.
                            backup_mode.set(false);
                            backup_context.set(None);
                            // Take an on-open backup now (T2-4) — `backup_mode` is
                            // known false at this point, so a freshly-opened
                            // *backup* file (the `Some(bc)` arm above) can never be
                            // pumped into the real project's retention pool before
                            // anyone knew it was a backup.
                            backup_scheduler.on_open();
                            // One-time "no backups configured" nudge (only when the
                            // effective policy has every trigger off — a project on
                            // defaults still backs up on close, so it never nags).
                            let uid = single_work.unique_id().get();
                            if crate::models::uid_is_usable(&uid)
                                && backup_settings.effective_for(&uid).is_effectively_off()
                                && !backup_settings.was_nudged(&uid)
                            {
                                if let Some(p) = path.as_deref() {
                                    backup_settings.mark_nudged(&uid, p);
                                }
                                let session_for_action = session_for_nudge.clone();
                                // Work-scoped: this project's own backup policy.
                                c.show_toast(
                                    Toast::warning(tr!(backup_nudge_text()))
                                        .target_work(ids.work_id.get())
                                        .action(ToastAction::primary(
                                            tr!(backup_nudge_action()),
                                            move |c| {
                                                let session_for_action =
                                                    session_for_action.clone();
                                                c.present_modal(
                                                    ModalRequest::deferred(move |t| {
                                                        t.add(SettingsPanel::open_to_backup(
                                                            session_for_action,
                                                        ))
                                                    })
                                                    .presentation(ModalPresentation::InTree)
                                                    .title("Settings")
                                                    .size(920, 620)
                                                    .close_behavior(ModalCloseBehavior::Manual),
                                                );
                                            },
                                        )),
                                );
                            }
                        }
                    }
                },
            );
        }
}

