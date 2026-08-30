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
use frontend::common::event::{Event, LongOperationEvent, Origin, WorkManagementEvent};

use crate::app_ids::AppIds;
use crate::backup::{BackupRestoreViewModel, BackupSchedulerViewModel, BackupSettingsViewModel};
use crate::binder::OutlineViewModel;
use crate::editors::EditorsViewModel;
use crate::models::OpenDocsStore;
use crate::project::ProjectLifecycleViewModel;
use crate::search::SearchReplaceViewModel;
use crate::sessions::{WorkRegistry, WorkSession};
use crate::settings::SettingsPanel;
use crate::settings::TreeExpansionViewModel;
use crate::singles::SingleWork;
use crate::spellcheck::DictionariesViewModel;
use crate::toast_scope::ToastWorkExt;
use crate::trash::TrashViewModel;
use crate::workspace_layout::WorkspaceLayoutViewModel;

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
    /// This window's undo group — a modal opened from here suspends it.
    pub undo_group: crate::edit::UndoGroupViewModel,
    /// Fires `count_words` so the plan summary reads a current number.
    pub progress_recorder: crate::shared::ProgressRecorder,
    /// Set when a count has been fired *for* the summary, cleared when it is shown. Keeps
    /// the completion handler from opening a panel for the recorder's own save-path counts.
    pub pace_pending: Signal<bool>,
    /// The `pace.summary_on_open` setting, so the panel's checkbox can write it.
    pub pace_show_on_open: Signal<bool>,
    pub editors: EditorsViewModel,
}

pub(in crate::app) fn install_backup_sniff(ctx: &mut BuildContext, deps: BackupSniffDeps) {
    // Detect "a backup file was opened" and enter backup mode. A separate
    // `subscribe_event_with_ctx` (needs an `EventContext` to present the choice
    // modal) reads the just-loaded path and sniffs its manifest. Opening a
    // backup always happens in its own window (the redirect in the open entry
    // points), so this only ever fires in a window dedicated to that backup.
    let sniff_app_ctx = deps.app_ctx.clone();
    let sniff_ids = deps.ids.clone();
    let sniff_pending = deps.pace_pending.clone();
    let sniff_show_on_open = deps.pace_show_on_open.clone();
    let sniff_editors = deps.editors.clone();
    // Captured up front, beside the other `sniff_*` handles: `deps.session` is moved
    // further down, and the Pace summary needs the ladder to render its completion half.
    let sniff_statuses = deps.session.statuses.clone();
    {
        let app_ctx = deps.app_ctx;
        let ids = deps.ids;
        let pace_shown = deps.session.pace_summary_shown.clone();
        let pace_pending = deps.pace_pending.clone();
        let recorder = deps.progress_recorder.clone();
        let summary_enabled = deps.pace_show_on_open.clone();
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
        let undo_for_nudge = deps.undo_group;
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
                            let undo_for_action = undo_for_nudge.clone();
                            // Work-scoped: this project's own backup policy.
                            c.show_toast(
                                Toast::warning(tr!(backup_nudge_text()))
                                    .target_work(ids.work_id.get())
                                    .action(ToastAction::primary(
                                        tr!(backup_nudge_action()),
                                        move |c| {
                                            let session_for_action = session_for_action.clone();
                                            let undo_for_action = undo_for_action.clone();
                                            crate::settings::present(c, move || {
                                                SettingsPanel::open_to_backup(
                                                    session_for_action,
                                                    &undo_for_action,
                                                )
                                            });
                                        },
                                    )),
                            );
                        }
                        // ── The writing-plan summary ──────────────────────
                        //
                        // Only here, in the non-backup arm: a backup window is
                        // a window onto a copy, and greeting it with "where the
                        // book stands" would be answering about the wrong file.
                        //
                        // A fresh count is fired first rather than reading the
                        // snapshot history straight off: that history is written
                        // on save, so at opening its newest point can be days
                        // old. Recording today's point also closes a real gap —
                        // a project opened and never saved used to leave a hole
                        // in the streak and the chart. The panel is presented on
                        // that count's completion, below.
                        if !pace_shown.get()
                            && summary_enabled.get()
                            && crate::pace::panel::has_active_plan(&app_ctx, &ids)
                        {
                            pace_shown.set(true);
                            pace_pending.set(true);
                            recorder.recount();
                        }
                    }
                }
            },
        );
    }
    // The other half: present once the count that was just fired lands.
    //
    // Registered **after** the recorder's own completion handler in
    // `wiring::long_ops`, so today's snapshot is already recorded by the time
    // this runs and the panel's history is current rather than one count behind.
    {
        let app_ctx = sniff_app_ctx;
        let ids = sniff_ids;
        let pace_pending = sniff_pending;
        let show_on_open = sniff_show_on_open;
        let editors = sniff_editors;
        ctx.subscribe_event_with_ctx(
            Origin::LongOperation(LongOperationEvent::Completed),
            move |_e: &Event, c: &mut EventContext| {
                if !pace_pending.get() {
                    return;
                }
                pace_pending.set(false);
                let editors = editors.clone();
                crate::pace::panel::present(
                    c,
                    app_ctx.clone(),
                    ids.clone(),
                    show_on_open.clone(),
                    sniff_statuses.clone(),
                    std::rc::Rc::new(move |book_item_id, _c: &mut EventContext| {
                        // The panel's one forward action: open the Book, which is
                        // where every number it showed can be edited. The title
                        // argument only seeds the tab caption — the editors model
                        // re-reads the item's own name as it opens.
                        editors.open_or_focus(book_item_id, "");
                    }),
                );
            },
        );
    }
}

/// Keep [`WorkSession::pace_summary_available`](crate::sessions::WorkSession::pace_summary_available)
/// — the Work menu's "Writing plan…" `enabled` — in step with the store.
///
/// The row is *enabled*, never hidden: a greyed row says the feature exists and, with its
/// tooltip, what turns it on; a hidden one says nothing at all. Which means the answer has
/// to be live, because the writer flips `Pace active` and sets the Book's word target from
/// inside the app and expects the menu to have noticed.
///
/// Three families of event can change it. The `Pace` itself (created by "Start planning",
/// and its `active` flag toggled from the planner), the `BinderItem` that carries the
/// Book's word target (the goal is a `BinderItem` field, not a `Pace` one, and a plan with
/// no target has nothing to report), and the project lifecycle — a load or a close swaps
/// the whole answer out from under the row.
///
/// **Coalesced.** `BinderItem(Updated)` arrives once per row in a bulk operation, and this
/// recomputes by querying the store; the shared coalescer collapses a burst into one
/// recompute on the next frame. That is also why [`has_active_plan`](crate::pace::panel::has_active_plan)
/// deliberately does *not* measure any word counts — see its sibling `plan_books`.
pub(in crate::app) fn install_pace_availability(
    ctx: &mut BuildContext,
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    available: Signal<bool>,
) {
    use frontend::common::event::DirectAccessEntity::{BinderItem, Pace};
    use frontend::common::event::EntityEvent::{Created, Removed, Updated};
    let origins = [
        Origin::DirectAccess(Pace(Created)),
        Origin::DirectAccess(Pace(Updated)),
        Origin::DirectAccess(Pace(Removed)),
        Origin::DirectAccess(BinderItem(Updated)),
        Origin::DirectAccess(BinderItem(Removed)),
        Origin::WorkManagement(WorkManagementEvent::LoadWork),
        Origin::WorkManagement(WorkManagementEvent::NewWork),
        Origin::WorkManagement(WorkManagementEvent::CloseWork),
    ];
    crate::models::coalesced_reload::reload_on_events(ctx, origins, move || {
        // Read through THIS window's own `ids`, so an event published by a sibling
        // window's Work resolves against this one's project and changes nothing.
        let now = crate::pace::panel::has_active_plan(&app_ctx, &ids);
        if available.get() != now {
            available.set(now);
        }
    });
}

// ── Load / New / Close / Attach seed ────────────────────────────────────────

/// Deps for the full project lifecycle install (seed, attach, new, dict, close).
pub(in crate::app) struct LifecycleDeps {
    pub app_ctx: Rc<AppContext>,
    pub session: WorkSession,
    /// This window's undo group — a modal opened from here suspends it.
    pub undo_group: crate::edit::UndoGroupViewModel,
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
    pub import_document: crate::import_document::ImportDocumentViewModel,
    pub cold_start_import: ColdStartImport,
    pub starters: PendingStarters,
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

/// What the New Work form asked its project to start with, beyond the template's own
/// rows: a tag palette, a set of note templates, or neither.
///
/// One value rather than a one-shot each, because the two are armed together and taken
/// together — a project that inherited the previous project's palette but not its
/// templates would be a bug with no name for it.
///
/// `None` on either side is a writer who chose nothing there, which is the default and a
/// real answer, not an unset one.
///
/// `pub` rather than `pub(crate)` because it rides on `PendingAction::New`, which is
/// `pub` — the same reason the `Option<tags::Preset>` it replaces was.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProjectStarters {
    pub tags: Option<crate::tags::Preset>,
    pub templates: Option<crate::note_templates::StarterSet>,
    /// Which workflow ladder the project starts with.
    ///
    /// `Option` for symmetry with the two above, but it is **not** treated like them: a
    /// `None` here still seeds [`crate::statuses::Preset::DEFAULT`] rather than seeding nothing.
    /// An empty tag palette is a project that has no tags yet and works fine; an empty
    /// ladder is a project with no status feature at all, and nothing in the UI would
    /// prompt the writer to go and create one.
    pub statuses: Option<crate::statuses::Preset>,
}

impl ProjectStarters {
    /// Nothing *optional* to lay down. Deliberately does not consider `statuses`: the
    /// ladder is seeded on every new project, so the caller runs the status step before
    /// asking this.
    fn is_empty(&self) -> bool {
        self.tags.is_none() && self.templates.is_none()
    }
}

/// The [`ProjectStarters`] a New Work form chose, waiting for the project they belong to
/// to exist.
///
/// The same one-shot shape as [`ColdStartImport`], and for the same reason: the form
/// answers the question, but there is nothing to apply it to until `new_work` has run.
/// `App::build` arms it immediately before that call and the `NewWork` subscriber
/// **takes** it, so the second project created in the same window does not inherit the
/// first one's answers.
#[derive(Clone, Default)]
pub(crate) struct PendingStarters(Rc<Cell<ProjectStarters>>);

impl PendingStarters {
    pub(crate) fn arm(&self, starters: ProjectStarters) {
        self.0.set(starters);
    }

    fn take(&self) -> ProjectStarters {
        self.0.replace(ProjectStarters::default())
    }
}

/// Install Load seed, attach-seed builder, backup sniff, New seed, missing-dict
/// offer, corpus clear, and Close. Returns the attach-seed closure for
/// `PendingAction::AttachExisting` (fired once on first build).
pub(in crate::app) fn install_lifecycle(
    ctx: &mut BuildContext,
    deps: LifecycleDeps,
) -> Rc<dyn Fn(u64, usize, Option<u64>)> {
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
                            crate::app::build_project_close_out(lifecycle_load.clone()),
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
    let attach_seed: Rc<dyn Fn(u64, usize, Option<u64>)> = {
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
        let lifecycle = deps.lifecycle.clone();
        Rc::new(
            move |work_id: u64, ordinal: usize, open_item: Option<u64>| {
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
                        crate::app::build_project_close_out(lifecycle.clone()),
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
                // The visible payload, last: the tab-strip menu's "Move into a new
                // window" names the one item this window is being opened *for*.
                //
                // Here, and not in the caller, because this is the only point at
                // which this window's own `EditorsViewModel` exists with its
                // subscriptions live — and after the teardown wiring above, so a
                // window that opens a tab is already able to give it back.
                //
                // An attached window restores no desk of its own (it is
                // `WindowRole::Attached`, so nothing calls `layout.restore`), which
                // is exactly what makes this work: the tab arrives alone instead of
                // being joined by every other tab the project remembers.
                if let Some(item_id) = open_item {
                    editors.open_by_id(crate::editors::Side::Primary, item_id);
                }
            },
        )
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
            undo_group: deps.undo_group.clone(),
            progress_recorder: deps.session.progress_recorder.clone(),
            pace_pending: Signal::new(false),
            pace_show_on_open: ctx.settings().signal(crate::PACE_SUMMARY_ON_OPEN_KEY, true),
            editors: deps.editors.clone(),
        },
    );

    // ── The Work menu's "Writing plan…" row, kept in step ──────────────────
    install_pace_availability(
        ctx,
        deps.app_ctx.clone(),
        deps.ids.clone(),
        deps.session.pace_summary_available.clone(),
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
                            crate::app::build_project_close_out(lifecycle_new.clone()),
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
                    crate::import_document::panel::present_import_document(
                        c,
                        import.clone(),
                        crate::import_document::panel::ImportDocumentOptions::default(),
                    );
                }
            },
        );
    }

    // ── What the project starts with: the tag palette, the note templates ──
    //
    // Ordered after the seed above for the same reason the import wizard is: the
    // project exists and `work_id` is set by the time this runs, which is what
    // `TagsViewModel::apply_preset` and its templates sibling need to have anywhere to
    // write.
    //
    // Deliberately not undoable-as-a-separate-step in the writer's mind: it lands on
    // the fresh project's own stack alongside everything else the template laid down,
    // and a writer who wants a different palette changes it in Settings rather than
    // pressing Ctrl+Z on a project they have not typed in yet.
    // ── Nothing seeds a ladder on LOAD, deliberately ────────────────────────
    //
    // A `LoadWork` subscriber used to heal a project that arrived without a ladder,
    // on the grounds that a pre-v14 project would otherwise have the status feature
    // dead. It cannot live here, and the reason generalises to anything else tempted
    // to write on load:
    //
    // Every entity `load_work` itself creates is invisible to the dirty flag, because
    // those events are dispatched before the `LoadWork` subscriber that seeds this
    // window's `work_id` — and `App::mutation_origins`' guard drops a mutation it
    // cannot attribute to an open Work. A *subscriber* runs on the other side of that
    // line: its writes are attributed, so they are counted. The heal created four
    // `BinderStatus` rows and so bumped `dirty_seq` twice (`Work(Updated)` for the
    // junction, then `BinderStatus(Created)`), and every project opened, untouched,
    // reading "unsaved changes" — the same "open-driven unconditional write" failure
    // `mutation_origins`' own doc warns a new whitelisted kind about.
    //
    // Seeding therefore belongs to `NewWork` (below), which every project goes
    // through, and to Settings ▸ Work ▸ Statuses ▸ Apply preset for one that somehow
    // has none. A load-time heal would have to move inside `load_work` to be silent.
    {
        let pending = deps.starters.clone();
        let session = deps.session.clone();
        let my_ids = deps.ids.clone();
        ctx.subscribe_event(
            Origin::WorkManagement(WorkManagementEvent::NewWork),
            move |event: &Event| {
                if !my_ids.is_bootstrap_or_own(&event.ids) {
                    return;
                }
                let starters = pending.take();

                // The ladder first, and unconditionally — see `ProjectStarters::statuses`.
                // `seed` refuses to run over a project that already has one, so a replayed
                // event cannot duplicate it.
                let ladder = starters
                    .statuses
                    .unwrap_or(crate::statuses::Preset::DEFAULT);
                if session.statuses.seed(ladder) == 0 {
                    eprintln!("skribisto: new work: the {ladder:?} status ladder seeded nothing");
                }

                if starters.is_empty() {
                    return;
                }
                if let Some(preset) = starters.tags {
                    let summary = session.tags.apply_preset(preset);
                    if summary.added == 0 {
                        eprintln!("new work: the chosen tag palette added nothing");
                    }
                }
                if let Some(set) = starters.templates {
                    let summary = session.note_templates.apply_starter_set(set);
                    if summary.added == 0 {
                        eprintln!("new work: the chosen note templates added nothing");
                    }
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
        let undo_for_toast = deps.undo_group.clone();
        ctx.subscribe_event_with_ctx(
            Origin::WorkManagement(event),
            move |e: &Event, c: &mut EventContext| {
                if my_ids.is_event_for_my_work(&e.ids) {
                    crate::app::offer_missing_dictionaries(
                        &docs,
                        &dictionaries,
                        &session_for_toast,
                        &undo_for_toast,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note_templates::StarterSet;
    use crate::tags::Preset;

    fn tags(preset: Preset) -> ProjectStarters {
        ProjectStarters {
            statuses: None,
            tags: Some(preset),
            templates: None,
        }
    }

    /// **A one-shot is taken once.** The whole point of `take` rather than a read: two
    /// projects created in the same window must not both get the first one's answers.
    #[test]
    fn a_taken_answer_is_not_taken_twice() {
        let pending = PendingStarters::default();
        pending.arm(tags(Preset::Basic));
        assert_eq!(pending.take(), tags(Preset::Basic));
        assert_eq!(
            pending.take(),
            ProjectStarters::default(),
            "the second project created in this window must choose for itself"
        );
    }

    /// **Never armed and armed-with-nothing are the same outcome**, which is what lets
    /// this carry `Option`s rather than having to tell the two apart. A writer who left
    /// both pickers alone chose no tags and no templates, and that is a real answer.
    #[test]
    fn an_unarmed_one_shot_and_an_explicit_nothing_agree() {
        let never = PendingStarters::default();
        assert!(never.take().is_empty());

        let explicit = PendingStarters::default();
        explicit.arm(ProjectStarters::default());
        assert!(explicit.take().is_empty());
    }

    /// **Arming again replaces.** A wizard that failed to create, then was reopened and
    /// completed, must apply the second answer and not the first.
    #[test]
    fn arming_again_replaces_the_pending_answer() {
        let pending = PendingStarters::default();
        pending.arm(tags(Preset::Fantasy));
        pending.arm(tags(Preset::Mystery));
        assert_eq!(pending.take(), tags(Preset::Mystery));
    }

    /// **The two halves travel together.** They are armed and taken as one value
    /// precisely so a project cannot end up with the palette it asked for and the
    /// templates the *previous* project asked for.
    #[test]
    fn the_palette_and_the_templates_are_one_answer() {
        let both = ProjectStarters {
            statuses: None,
            tags: Some(Preset::SciFi),
            templates: Some(StarterSet::Essentials),
        };
        let pending = PendingStarters::default();
        pending.arm(both);
        assert_eq!(pending.take(), both);
        assert!(pending.take().is_empty());
    }

    /// One side chosen and not the other is ordinary, and must not read as "nothing".
    #[test]
    fn templates_alone_is_a_real_answer() {
        let only_templates = ProjectStarters {
            statuses: None,
            tags: None,
            templates: Some(StarterSet::Everything),
        };
        assert!(!only_templates.is_empty());
    }
}
