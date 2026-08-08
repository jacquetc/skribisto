// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Project lifecycle event wiring — Load / New / Close / Attach and the
//! post-Load backup sniff.
//!
//! Extracted from `App::build` so that god-function is not also the multi-window
//! project-lifecycle bus. Guarded subscribe helpers and window binding live
//! alongside.
//!
//! **Registration order is load-bearing** for `LoadWork`: seed first, then
//! [`install_backup_sniff`] (called from here after the seed subscriber). Same
//! for `NewWork`: seed first, then the cold-start import — the wizard reads the
//! `work_id` the seed writes.

use std::cell::Cell;
use std::rc::Rc;

use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::prelude::*;
use teksilo::widgets::{
    DockOpenLocation, DockSide, DockWidgetId, Toast, ToastAction, ToastRegistry,
};

use frontend::AppContext;
use frontend::common::event::{Event, Origin, WorkManagementEvent};

use crate::app_ids::AppIds;
use crate::models::OpenDocsStore;
use crate::sessions::{WorkRegistry, WorkSession};
use crate::settings::SettingsPanel;
use crate::singles::SingleWork;
use crate::toast_scope::ToastWorkExt;
use crate::view_models::{
    BackupRestoreViewModel, BackupSchedulerViewModel, BackupSettingsViewModel,
    DictionariesViewModel, EditorsViewModel, OutlineViewModel, ProjectLifecycleViewModel,
    SearchReplaceViewModel, TrashViewModel, TreeExpansionViewModel, WorkspaceLayoutViewModel,
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
    // backup always happens in its own window (the redirect in the open entry
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
                // Point this Work's document store at its media directory, before
                // any tab opens: an image's bytes are resolved as its document
                // loads, so a document built before this is known would show
                // every picture as a correctly-sized blank.
                if let Some(p) = path.as_deref() {
                    let uid = frontend::commands::work_commands::get_work(
                        &app_ctx,
                        &ids.work_id.get().unwrap_or_default(),
                    )
                    .ok()
                    .flatten()
                    .map(|w| w.unique_id)
                    .unwrap_or_default();
                    session_for_nudge
                        .open_docs
                        .set_media_dir(skrib_format::media::media_dir(
                            std::path::Path::new(p),
                            &uid,
                            std::path::Path::new(&crate::media_paths::media_root_string()),
                            &uid,
                        ));
                }

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
                                            let session_for_action = session_for_action.clone();
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

// ── Load / New / Close / Attach seed ────────────────────────────────────────

/// Deps for the full project lifecycle install (seed, attach, new, dict, close).
pub(in crate::app) struct LifecycleDeps {
    pub app_ctx: Rc<AppContext>,
    pub session: WorkSession,
    pub ids: AppIds,
    pub registry: WorkRegistry,
    pub lifecycle: ProjectLifecycleViewModel,
    pub editors: EditorsViewModel,
    pub outline: OutlineViewModel,
    pub trash: TrashViewModel,
    pub search: SearchReplaceViewModel,
    pub backup_scheduler: BackupSchedulerViewModel,
    pub toast_registry: Option<ToastRegistry>,
    pub window_id: Option<TeksiloWindowId>,
    pub window_ordinal: Signal<usize>,
    pub spell_docs: OpenDocsStore,
    pub dictionaries: DictionariesViewModel,
    // backup sniff (installed after Load seed, same call)
    pub tree_expansion: TreeExpansionViewModel,
    pub backup_mode: Signal<bool>,
    pub backup_context: Signal<Option<crate::backup::BackupContext>>,
    pub restore_vm: BackupRestoreViewModel,
    pub single_work: SingleWork,
    pub backup_settings: BackupSettingsViewModel,
    pub workspace_layout: Option<WorkspaceLayoutViewModel>,
    pub trash_dock: DockWidgetId,
    // cold-start import (installed after the New seed, same call)
    pub import_document: crate::view_models::ImportDocumentViewModel,
    pub cold_start_import: ColdStartImport,
}

/// A Launcher "From documents…" waiting for the project it is about to fill to
/// exist.
///
/// A one-shot: `App::build` arms it immediately before `new_work`, and the
/// `NewWork` subscriber below **takes** it. Taking rather than reading is what
/// makes the second project created in the same window not re-open a wizard
/// nobody asked for.
///
/// A flag rather than the picked file list it used to carry: the files are now
/// chosen inside the import wizard, which is the only surface that can review
/// and order them anyway.
#[derive(Clone, Default)]
pub(in crate::app) struct ColdStartImport(Rc<Cell<bool>>);

impl ColdStartImport {
    pub(in crate::app) fn arm(&self) {
        self.0.set(true);
    }

    fn take(&self) -> bool {
        self.0.replace(false)
    }
}

/// Install Load seed, attach-seed builder, backup sniff, New seed, missing-dict
/// offer, corpus clear, and Close. Returns the attach-seed closure for
/// `PendingAction::AttachExisting` (fired once on first build).
pub(in crate::app) fn install_lifecycle(
    ctx: &mut BuildContext,
    deps: LifecycleDeps,
) -> Rc<dyn Fn(u64, usize)> {
    let window_id = deps.window_id;

    // ── LoadWork seed (must be the first LoadWork subscriber) ──────────────
    {
        let lifecycle_load = deps.lifecycle.clone();
        let my_ids = deps.ids.clone();
        let my_session = deps.session.clone();
        let registry_for_load = deps.registry.clone();
        let toast_registry_for_load = deps.toast_registry.clone();
        let editors_for_teardown = deps.editors.clone();
        let app_ctx_for_teardown = deps.app_ctx.clone();
        let backup_scheduler_for_teardown = deps.backup_scheduler.clone();
        let window_ordinal_for_load = deps.window_ordinal.clone();
        ctx.subscribe_event(
            Origin::WorkManagement(WorkManagementEvent::LoadWork),
            move |event: &Event| {
                if !my_ids.is_bootstrap_or_own(&event.ids) {
                    return;
                }
                if let Some(&work_id) = event.ids.first() {
                    lifecycle_load.on_load(work_id);
                    registry_for_load.register(work_id, my_session.clone());
                    if let Some(window_id) = window_id {
                        let stack_teardown = crate::app::build_stack_teardown(
                            app_ctx_for_teardown.clone(),
                            my_ids.stack_id.get(),
                        );
                        let window_teardown = crate::app::build_window_teardown(
                            editors_for_teardown.clone(),
                            backup_scheduler_for_teardown.clone(),
                            toast_registry_for_load.clone(),
                            window_id,
                            my_ids.stack_id.get(),
                        );
                        bind_window_to_work(
                            &registry_for_load,
                            window_id,
                            work_id,
                            None,
                            stack_teardown,
                            window_teardown,
                            &window_ordinal_for_load,
                            &toast_registry_for_load,
                        );
                    }
                }
            },
        );
    }

    // ── Attach seed (Work ▸ New Window — no LoadWork will fire) ─────────────
    let attach_seed: Rc<dyn Fn(u64, usize)> = {
        let outline = deps.outline.clone();
        let trash = deps.trash.clone();
        let registry = deps.registry.clone();
        let ids = deps.ids.clone();
        let app_ctx = deps.app_ctx.clone();
        let editors = deps.editors.clone();
        let backup_scheduler = deps.backup_scheduler.clone();
        let toast_registry = deps.toast_registry.clone();
        let window_ordinal = deps.window_ordinal.clone();
        let search = deps.search.clone();
        let tree_expansion = deps.tree_expansion.clone();
        Rc::new(move |work_id: u64, ordinal: usize| {
            outline.set_binder_filter(None);
            outline.clear_search();
            outline.reload();
            trash.reload();
            let remembered = tree_expansion.outline_expanded();
            if !remembered.is_empty() {
                outline.model().set_expanded_keys(&remembered);
            }
            search.restore_for_project();
            if let Some(window_id) = window_id {
                let stack_teardown =
                    crate::app::build_stack_teardown(app_ctx.clone(), ids.stack_id.get());
                let window_teardown = crate::app::build_window_teardown(
                    editors.clone(),
                    backup_scheduler.clone(),
                    toast_registry.clone(),
                    window_id,
                    ids.stack_id.get(),
                );
                bind_window_to_work(
                    &registry,
                    window_id,
                    work_id,
                    Some(ordinal),
                    stack_teardown,
                    window_teardown,
                    &window_ordinal,
                    &toast_registry,
                );
            }
        })
    };

    // ── Second LoadWork: backup sniff (after seed) ─────────────────────────
    install_backup_sniff(
        ctx,
        BackupSniffDeps {
            app_ctx: deps.app_ctx.clone(),
            ids: deps.ids.clone(),
            tree_expansion: deps.tree_expansion.clone(),
            backup_mode: deps.backup_mode.clone(),
            backup_context: deps.backup_context.clone(),
            restore_vm: deps.restore_vm.clone(),
            single_work: deps.single_work.clone(),
            backup_settings: deps.backup_settings.clone(),
            backup_scheduler: deps.backup_scheduler.clone(),
            workspace_layout: deps.workspace_layout.clone(),
            outline: deps.outline.clone(),
            trash_dock: deps.trash_dock,
            session: deps.session.clone(),
        },
    );

    // ── NewWork seed ───────────────────────────────────────────────────────
    {
        let lifecycle_new = deps.lifecycle.clone();
        let my_ids = deps.ids.clone();
        let my_session = deps.session.clone();
        let registry_for_new = deps.registry.clone();
        let toast_registry_for_new = deps.toast_registry.clone();
        let editors_for_teardown = deps.editors.clone();
        let app_ctx_for_teardown = deps.app_ctx.clone();
        let backup_scheduler_for_teardown = deps.backup_scheduler.clone();
        let window_ordinal_for_new = deps.window_ordinal.clone();
        ctx.subscribe_event(
            Origin::WorkManagement(WorkManagementEvent::NewWork),
            move |event: &Event| {
                if !my_ids.is_bootstrap_or_own(&event.ids) {
                    return;
                }
                if let Some(&work_id) = event.ids.first() {
                    lifecycle_new.on_new(work_id);
                    registry_for_new.register(work_id, my_session.clone());
                    // Point the new project's document store at its media
                    // directory, exactly as the `LoadWork` seed does. A new Work
                    // has no file on disk yet, but `new_work` mints its
                    // `unique_id`, and that alone resolves the uid-keyed
                    // directory its images will be written to. Without this the
                    // media directory stays empty and every image command
                    // reports "open a project first" at a writer who has one
                    // open — until they save, close and reopen it.
                    let uid = frontend::commands::work_commands::get_work(
                        &app_ctx_for_teardown,
                        &work_id,
                    )
                    .ok()
                    .flatten()
                    .map(|w| w.unique_id)
                    .unwrap_or_default();
                    my_session
                        .open_docs
                        .set_media_dir(skrib_format::media::media_dir(
                            std::path::Path::new(""),
                            &uid,
                            std::path::Path::new(&crate::media_paths::media_root_string()),
                            &uid,
                        ));
                    if let Some(window_id) = window_id {
                        let stack_teardown = crate::app::build_stack_teardown(
                            app_ctx_for_teardown.clone(),
                            my_ids.stack_id.get(),
                        );
                        let window_teardown = crate::app::build_window_teardown(
                            editors_for_teardown.clone(),
                            backup_scheduler_for_teardown.clone(),
                            toast_registry_for_new.clone(),
                            window_id,
                            my_ids.stack_id.get(),
                        );
                        bind_window_to_work(
                            &registry_for_new,
                            window_id,
                            work_id,
                            None,
                            stack_teardown,
                            window_teardown,
                            &window_ordinal_for_new,
                            &toast_registry_for_new,
                        );
                    }
                }
            },
        );
    }

    // ── Cold-start import (must be registered AFTER the New seed) ──────────
    //
    // The Launcher's "From documents…" made this project *for* an import. The
    // project now exists and its ids are seeded (that is what the ordering buys
    // — the wizard reads `work_id` to analyse into), so the wizard opens over
    // it, on its own first step: choose the files.
    //
    // The form that created the project said this was coming (its last step is
    // about nothing else), so the wizard is expected rather than startling.
    {
        let import = deps.import_document.clone();
        let pending = deps.cold_start_import.clone();
        let my_ids = deps.ids.clone();
        ctx.subscribe_event_with_ctx(
            Origin::WorkManagement(WorkManagementEvent::NewWork),
            move |event: &Event, c: &mut EventContext| {
                if !my_ids.is_bootstrap_or_own(&event.ids) {
                    return;
                }
                if pending.take() {
                    crate::panels::import_document::present_import_document(
                        c,
                        import.clone(),
                        crate::panels::import_document::ImportDocumentOptions::default(),
                    );
                }
            },
        );
    }

    // ── Missing dictionaries toast ─────────────────────────────────────────
    for event in [WorkManagementEvent::LoadWork, WorkManagementEvent::NewWork] {
        let docs = deps.spell_docs.clone();
        let dictionaries = deps.dictionaries.clone();
        let my_ids = deps.ids.clone();
        let session_for_toast = deps.session.clone();
        ctx.subscribe_event_with_ctx(
            Origin::WorkManagement(event),
            move |e: &Event, c: &mut EventContext| {
                if my_ids.is_event_for_my_work(&e.ids) {
                    crate::app::offer_missing_dictionaries(
                        &docs,
                        &dictionaries,
                        &session_for_toast,
                        c,
                    );
                }
            },
        );
    }

    // ── Search corpus cache clear (unguarded, Tier-1) ──────────────────────
    for event in [
        WorkManagementEvent::LoadWork,
        WorkManagementEvent::NewWork,
        WorkManagementEvent::CloseWork,
    ] {
        ctx.subscribe_event(Origin::WorkManagement(event), move |_event: &Event| {
            frontend::search_management::corpus_cache::clear();
        });
    }

    // ── CloseWork ──────────────────────────────────────────────────────────
    {
        let lifecycle_close = deps.lifecycle.clone();
        let my_ids = deps.ids.clone();
        ctx.subscribe_event_with_ctx(
            Origin::WorkManagement(WorkManagementEvent::CloseWork),
            move |event: &Event, c: &mut EventContext| {
                if !my_ids.is_event_for_my_work(&event.ids) {
                    return;
                }
                c.dismiss_top_overlay();
                lifecycle_close.on_close();
            },
        );
    }

    attach_seed
}
