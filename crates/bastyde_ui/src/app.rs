// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The application body: a `DockingLayout` whose leading dock is the binder
//! tree and whose center is a `TabWidget` of editor tabs (Phase 3), with a thin
//! status bar underneath. The window chrome (custom `TitleBar` + hamburger menu)
//! lives at the window root in `main.rs`.
//!
//! Clicking a binder item opens (or focuses) its editor tab via the tree's
//! `KeyedSelectionModel<NodeId>` selection signal.
//!
//! Plain builder calls rather than `bati!`: the docking/tab/editor widgets are
//! generic over closures, which the DSL doesn't express cleanly. See
//! `settings_panel.rs` for the `bati!` style.

use std::rc::Rc;

use bastyde::core::DragPayload;
use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::settings::{Reloadable, SettingsExt, SettingsRegistry};
use bastyde::tokens::SurfaceRole::Hover;
use bastyde::widgets::{
    Divider, DockCorner, DockOpenLocation, DockRail, DockRailItemSize, DockSide, DockWidgetId,
    DockingLayout, DropRegion, DropTarget, DropTargetVariant, EventContextMessageBoxExt, Expand,
    HStack, IconButton, IconButtonSize, MessageBox, MessageBoxButton, MessageBoxButtons,
    NotificationArchiveModel, NotificationCenterButton, RowDragData, Spacer, Splitter,
    StandardButton, StatusBar, TabBarVisibility, TabWidget, TextWidget, Toast, ToastAction, VStack,
};

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::common::event::{
    DirectAccessEntity, EntityEvent, Event, LongOperationEvent, Origin, WorkManagementEvent,
};
use frontend::work_management::{LoadWorkDto, NewWorkDto};

use crate::app_ids::AppIds;
use crate::export_panel::ExportPanel;
use crate::import_plume_panel::ImportPlumePanel;
use crate::intents::AppIntent;
use crate::models::TreeNode;
use crate::new_work_panel::NewWorkPanel;
use crate::settings_panel::SettingsPanel;
use crate::singles::{SingleWork, SingleWorkInfo};
use crate::tabs::{ContentTab, tab_pane};
use crate::view_models::{
    BackupSchedulerViewModel, BackupSettingsViewModel, EditorsViewModel, ExportViewModel,
    ImportPlumeViewModel, OutlineViewModel, PendingSwitch, ProjectSwitchViewModel, SaveAsViewModel,
    SearchReplaceViewModel, SettingsViewModel, Side, SpinnerGate, UnsavedDecision,
    unsaved_decision,
};
use export_management::ExportScopeKind;

/// Build one editor pane's `TabWidget`: dynamic tabs, cross-pane migration
/// (`accept_external_tabs` + `on_tab_received` dedup + `on_transfer_out`
/// collapse), close, and `trailing` in the tab-strip trailing slot. Shared by
/// both panes so their chrome can't drift.
fn build_pane_tabs(
    editors: &EditorsViewModel,
    side: Side,
    trailing: impl Widget + 'static,
) -> TabWidget {
    let close = editors.clone();
    let recv = editors.clone();
    let out = editors.clone();
    TabWidget::new(editors.selected(side))
        .dynamic_tab::<ContentTab>("editor", |_handle, state| tab_pane(state))
        .dynamic_model(editors.tabs(side))
        .on_close(move |tab_id, _ctx| close.close_in(side, tab_id))
        .on_tab_received(move |handle, _idx, _ctx| recv.receive_tab(side, handle))
        .on_transfer_out(move |tab_id, _ctx| out.transfer_out(side, tab_id))
        .reorderable(true)
        .accept_external_tabs(true)
        .bar_visibility(TabBarVisibility::Always)
        .compact_bar()
        .selected_tab_background(SurfaceRole::Content)
        .hover_tab_background(Hover)
        .tab_dividers()
        .active_indicator(bastyde::widgets::TabIndicatorPosition::InnerEdge)
        .bar_trailing_slot(trailing)
}

/// Open every binder item in a dropped `RowDragData<TreeNode>` payload, routing
/// each `(item_id, title)` through `open` (which picks the pane / side). Binder
/// rows (no `item_id`) are ignored. Returns whether anything opened — the drop's
/// accept verdict.
fn drain_dropped(mut payload: DragPayload, mut open: impl FnMut(u64, &str)) -> bool {
    let Some(rd) = payload.take_typed::<RowDragData<TreeNode>>() else {
        return false;
    };
    let Some(items) = rd.items else {
        return false;
    };
    let mut opened = false;
    for node in items {
        if let Some(item_id) = node.item_id {
            open(item_id, &node.title);
            opened = true;
        }
    }
    opened
}

/// A close gesture deferred until the in-flight save finishes. The window's
/// close guard, the `work.close` action, `app.quit`'s action, and
/// `welcome.show` (aliased to `work.close` — see its action below) all set
/// this; `App` kicks the save, and the SaveWork-completion event performs the
/// action — so the async save is awaited.
///
/// Two outcomes today. Every guarded close of a project window that still
/// goes through `close_window()` (title-bar X, Alt+F4, Ctrl+W, File ▸ Close
/// Work, the brand icon / File ▸ Welcome…) returns to the Launcher — see
/// [`close_work_and_return_to_launcher`]. Ctrl+Q / File ▸ Quit is the one
/// exception: it never calls `close_window()` at all — `app.quit`'s action
/// runs the same unsaved-changes guard ([`guard_unsaved_exit`]) but ends in
/// [`quit_app`], which really terminates the process instead of reopening the
/// Launcher. Both outcomes share the exact same branch order
/// (`unsaved_decision`/`UnsavedDecision`); only the terminal action differs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PendingExit {
    #[default]
    None,
    /// Release the open project and return to the Launcher, once saved (and,
    /// if configured, once the on-close backup finishes).
    ReturnToLauncher,
    /// Release the open project and terminate the process entirely, once
    /// saved (and, if configured, once the on-close backup finishes).
    /// Unlike `ReturnToLauncher`, this does NOT open a fresh Launcher
    /// window — `quit_app` force-closes the project window with nothing
    /// reopened, so `WindowManager::is_empty()` trips and the event loop
    /// exits (bastyde-app/src/app.rs, `maybe_exit`/`event_loop.exit()`).
    /// Only valid when this project window is the sole open window, which
    /// is Skribisto's steady state.
    Quit,
}

/// A backend-mutating action deferred to a freshly-created project window's
/// first build (see `App::build`'s first-build logic below) — performed only
/// *after* that window's `LoadWork`/`NewWork` event subscriptions are live, so
/// the seeding they perform (`AppIds::seed`, `SingleWork::set_id`, the tree
/// reload, …) never races the event that would otherwise fire before anyone
/// is listening.
///
/// Mirrors the pre-existing "open the argv path once mounted" mechanism; the
/// Launcher's recents / file-picker / examples / New Work flows all build a
/// project window with one of these instead of touching the backend directly
/// from the Launcher's own (App-less) window.
#[derive(Clone, Debug)]
pub enum PendingAction {
    /// Load an existing `.skrib` at this path.
    Load(String),
    /// Create a brand-new work from this DTO (the Launcher's "New Work").
    New(NewWorkDto),
}

impl PendingAction {
    /// The on-disk path this action targets — used to derive the project
    /// window's persistence id ([`crate::windows::window_id_for`]) before the
    /// action itself has run.
    pub fn target_path(&self) -> &str {
        match self {
            PendingAction::Load(path) => path,
            PendingAction::New(dto) => &dto.file_name,
        }
    }
}

/// Release the open project and return to the Launcher: fires `CloseWork`
/// (releases the open-registry claim and clears `AppIds`/the tree/the
/// singles via the subscriber below), opens a fresh Launcher window, **then**
/// force-closes this project window.
///
/// Order matters: opening the Launcher before closing this window means the
/// process is never briefly windowless mid-transition — which would quit it
/// (see `main.rs`'s module docs). Callers invoking this from inside a
/// `on_close_requested` guard must return `CloseResponse::Veto` afterward:
/// this function performs the actual close itself, via `close_window_forced`,
/// rather than deferring to the guard's own return value.
pub fn close_work_and_return_to_launcher(app_ctx: &Rc<AppContext>, ctx: &mut EventContext) {
    // Capture the desk (open tabs + docks) while the store is still alive —
    // `close_work` tears the Work subtree out *before* publishing `CloseWork`, so a
    // subscriber could no longer translate a tab into its persistable ordinal.
    capture_workspace_layout(ctx);
    let _ = work_management_commands::close_work(app_ctx);
    ctx.open_window(crate::windows::launcher_window_config(app_ctx.clone()));
    ctx.close_window_forced();
}

/// Release the open project and terminate the process — the `Quit` sibling
/// of [`close_work_and_return_to_launcher`]. Deliberately does NOT open a
/// fresh Launcher window: once this (normally sole) window force-closes,
/// `WindowManager::is_empty()` trips and the event loop exits for real — the
/// framework's only process-exit mechanism (there is no
/// `EventContext::quit()`/`terminate()`).
///
/// Callers invoking this from inside a close guard must return
/// `CloseResponse::Veto` afterward, exactly like its Launcher-returning
/// sibling: this function performs the actual close itself, via
/// `close_window_forced`, rather than deferring to the guard's own return
/// value.
pub fn quit_app(app_ctx: &Rc<AppContext>, ctx: &mut EventContext) {
    // Persist the desk before the store is torn down — see
    // [`close_work_and_return_to_launcher`].
    capture_workspace_layout(ctx);
    let _ = work_management_commands::close_work(app_ctx);
    ctx.close_window_forced();
}

/// Persist the open project's workspace layout (open tabs + dock arrangement)
/// through the shared [`WorkspaceLayoutViewModel`]. Called at each "leave the
/// project" door while its store is still alive. A no-op if the view-model isn't
/// registered (e.g. a launcher window) or no project is open.
pub(crate) fn capture_workspace_layout(ctx: &mut EventContext) {
    if let Some(layout) = ctx
        .app_state::<crate::view_models::WorkspaceLayoutViewModel>()
        .cloned()
    {
        layout.capture();
    }
}

/// Perform `outcome` immediately — `close_work_and_return_to_launcher` for
/// `ReturnToLauncher`, `quit_app` for `Quit`. Shared by every "user picked
/// Discard" branch in [`guard_unsaved_exit`], so discarding unsaved edits
/// always skips the on-close backup (the last-saved state is what's kept),
/// exactly as it did before the guard's three call sites were unified.
fn perform_exit(outcome: PendingExit, app_ctx: &Rc<AppContext>, ctx: &mut EventContext) {
    match outcome {
        PendingExit::ReturnToLauncher => close_work_and_return_to_launcher(app_ctx, ctx),
        PendingExit::Quit => quit_app(app_ctx, ctx),
        PendingExit::None => unreachable!("guard_unsaved_exit is never invoked with outcome=None"),
    }
}

/// **The** unsaved-changes guard shared by `work.close`'s action, `app.quit`'s
/// action, and the project window's `on_close_requested` (`windows.rs`) — the
/// single branch order every exit path in the app uses, mirroring
/// [`ProjectSwitchViewModel::request`]'s four switch doors, which already
/// share [`unsaved_decision`]. `outcome` is `ReturnToLauncher` or `Quit`
/// (never `None` — that would mean nothing to guard, which none of the three
/// callers ever ask for).
///
/// * [`UnsavedDecision::Proceed`] — nothing to protect: hand straight to
///   `scheduler.on_close_flow`, which itself takes the on-close backup (if
///   configured and not suppressed by backup mode) before performing
///   `outcome`.
/// * [`UnsavedDecision::SaveThenProceed`] — autosave is on: just arm
///   `pending_exit = outcome`. The existing save-then-close machinery (the
///   `pending_exit` effect + the `LongOperation::Completed` handler, both in
///   `App::build`, unchanged) resumes the outcome once the save lands.
/// * [`UnsavedDecision::PromptDiscardOnly`] — a backup file is open here
///   (Save is off): Discard performs `outcome` at once (bypassing
///   `on_close_flow` — discarding a backup's edits never backs anything up);
///   Cancel does nothing.
/// * [`UnsavedDecision::PromptSaveDiscardCancel`] — Save arms
///   `pending_exit = outcome` (same resumption as `SaveThenProceed`); Discard
///   performs `outcome` at once; Cancel does nothing.
///
/// The dialog copy differs by `outcome` (quitting says so, rather than
/// reusing "closing the work") — see `quit-question` /
/// `quit-backup-discard-title` / `quit-backup-discard-text` in the locales.
#[allow(clippy::too_many_arguments)]
pub(crate) fn guard_unsaved_exit(
    ctx: &mut EventContext,
    app_ctx: &Rc<AppContext>,
    unsaved: bool,
    backup_mode: bool,
    autosave: bool,
    pending: &Signal<PendingExit>,
    scheduler: &BackupSchedulerViewModel,
    outcome: PendingExit,
) {
    match unsaved_decision(unsaved, backup_mode, autosave) {
        UnsavedDecision::Proceed => scheduler.on_close_flow(ctx, outcome),
        UnsavedDecision::SaveThenProceed => pending.set(outcome),
        UnsavedDecision::PromptDiscardOnly => {
            let app_ctx = app_ctx.clone();
            let (title, text) = match outcome {
                PendingExit::Quit => (
                    tr!(quit_backup_discard_title()),
                    tr!(quit_backup_discard_text()),
                ),
                _ => (
                    tr!(close_backup_discard_title()),
                    tr!(close_backup_discard_text()),
                ),
            };
            ctx.present_message_box(
                MessageBox::question(title)
                    .text(text)
                    .buttons(MessageBoxButtons::Custom(vec![
                        MessageBoxButton::standard(StandardButton::Discard),
                        MessageBoxButton::standard(StandardButton::Cancel),
                    ]))
                    .default_button(StandardButton::Cancel)
                    .escape_button(StandardButton::Cancel)
                    .on_result(move |r, ctx| {
                        if r.button == StandardButton::Discard {
                            perform_exit(outcome, &app_ctx, ctx);
                        }
                    }),
            );
        }
        UnsavedDecision::PromptSaveDiscardCancel => {
            let app_ctx = app_ctx.clone();
            let pe = pending.clone();
            let title = match outcome {
                PendingExit::Quit => tr!(quit_question()),
                _ => tr!(close_work_question()),
            };
            ctx.present_message_box(
                MessageBox::question(title)
                    .text(tr!(unsaved_changes()))
                    .buttons(MessageBoxButtons::SaveDiscardCancel)
                    .default_button(StandardButton::Save)
                    .escape_button(StandardButton::Cancel)
                    .on_result(move |r, ctx| match r.button {
                        StandardButton::Save => pe.set(outcome),
                        StandardButton::Discard => perform_exit(outcome, &app_ctx, ctx),
                        _ => {}
                    }),
            );
        }
    }
}

/// Convert a theme colour role to the `text_document` colour a highlight span carries. Used to
/// paint spell-check squiggles in the theme's `text_error` role — a semantic role, not a hex
/// literal, so light/dark both work.
fn spell_underline_color(c: bastyde::tokens::Color) -> bastyde::text_document::Color {
    let [r, g, b, _] = c.to_array();
    let to_u8 = |x: f32| (x.clamp(0.0, 1.0) * 255.0).round() as u8;
    bastyde::text_document::Color::rgb(to_u8(r), to_u8(g), to_u8(b))
}

/// After a project becomes live, offer to install any dictionary its declared languages need
/// but the machine lacks — one aggregated toast (never one per language), whose action opens
/// Settings ▸ Dictionaries with the missing set highlighted. Purely additive and dismissible,
/// so a toast, not a modal.
fn offer_missing_dictionaries(
    docs: &crate::models::OpenDocsStore,
    dictionaries: &crate::view_models::DictionariesViewModel,
    ctx: &mut EventContext,
) {
    let missing = dictionaries.missing_for(&docs.project_languages());
    if missing.is_empty() {
        return;
    }
    dictionaries.set_highlight(missing.clone());
    let n = missing.len() as i64;
    ctx.show_toast(
        bastyde::widgets::Toast::info(tr!(dict_missing_toast(count = n)))
            .id("dict.missing")
            .action(bastyde::widgets::ToastAction::primary(
                tr!(dict_missing_action()),
                move |c| {
                    c.present_modal(
                        ModalRequest::deferred(|t| t.add(SettingsPanel::open_to_dictionaries()))
                            .presentation(ModalPresentation::InTree)
                            .title("Settings")
                            .size(920, 620)
                            .close_behavior(ModalCloseBehavior::Manual),
                    );
                },
            )),
    );
}

pub struct App {
    app_ctx: Rc<AppContext>,
    /// The outline view-model is created in `main` (the title-bar menu needs a
    /// handle to it for the reactive checkmark) and shared with `App`.
    outline: OutlineViewModel,
    /// Plain mirror of the persisted autosave setting, read by the title-bar menu
    /// (outside `App`) to hide the manual "Save" item. `App::build` mirrors the
    /// store-backed setting into it.
    autosave_menu: Signal<bool>,
    /// Plain mirror of the persisted master spell-check switch, read by the title-bar
    /// (outside `App`) for the toggle's icon + the View ▸ Check spelling checkmark.
    /// `App::build` mirrors the store-backed setting into it.
    spellcheck_menu: Signal<bool>,
    /// `true` while the open work has edits not yet written to disk. Read by the
    /// close guard, `work.close` and the switch guard to decide whether to prompt,
    /// and by `can_save` for the Save affordances.
    ///
    /// **Derived**, not set by hand: `dirty_seq > editors.saved_seq()`. It used to
    /// be a flag set on mutation and cleared whenever *a* save landed — which lied
    /// while a save was in flight, because typing during that save was marked clean
    /// the moment it finished, even though its snapshot never contained those
    /// edits. Save then greyed out on prose that was on no disk anywhere.
    unsaved: Signal<bool>,
    /// Monotonic edit sequence: bumped on every mutation (typing via the editors'
    /// `edited` signal, plus the tree/metadata events in [`mutation_origins`]).
    /// Handed to the editors, which capture it when a save starts, so "are my edits
    /// on disk?" has an exact answer — see `view_models::save_queue`.
    dirty_seq: Signal<u64>,
    /// A deferred close: performed once the save it asked for actually covers the
    /// edits (see `exit_seq`). Shared with `main`'s window close guard.
    pending_exit: Signal<PendingExit>,
    /// The edit sequence [`Self::pending_exit`] is waiting to see on disk.
    exit_seq: Rc<std::cell::Cell<Option<u64>>>,
    /// The save indicator's "saving…" hysteresis, and the gate's answer. Held here,
    /// not in the widget: `build` constructs a fresh `SaveIndicator` on every run, so
    /// a gate owned by the widget would have a slow save's delay / min-display timers
    /// reset by any unrelated App rebuild.
    save_spinner: Rc<std::cell::RefCell<SpinnerGate>>,
    save_spinner_visible: Signal<bool>,
    /// True while a *backup file* is open here (Save + auto-backup off; content
    /// still editable). Shared with `main`'s title-bar menu.
    backup_mode: Signal<bool>,
    /// The open backup's details (drives the permanent banner + restore), or
    /// `None` for a normal project.
    backup_context: Signal<Option<crate::backup::BackupContext>>,
    /// The backend mutation this window's project performs once mounted (load
    /// an existing path, or create a brand-new work) — taken on first build,
    /// after the `LoadWork`/`NewWork` subscriptions are live so the full
    /// seed flow runs. See [`PendingAction`].
    initial_action: Option<PendingAction>,
    /// One-shot guard so `initial_action` runs only on the first build.
    initial_loaded: bool,
    /// Created once on first build (its column-width signal needs `ctx.settings()`).
    editors: Option<EditorsViewModel>,
    /// Stable id for the trailing Inspector dock (created once so a rebuild keeps
    /// the same dock in the `DockingModel`).
    inspector_dock: DockWidgetId,
    /// Stable id for the bottom search-preview dock.
    preview_dock: DockWidgetId,
    /// Stable id for the leading search & replace dock.
    search_dock: DockWidgetId,
    /// The search feature's shared view-model, created once on first build (like
    /// [`editors`](Self::editors) — its debounce/signals want a live context).
    search: Option<SearchReplaceViewModel>,
    /// Stable id for the leading trash dock (third rail tab).
    trash_dock: DockWidgetId,
    /// The trash feature's shared view-model, created once on first build.
    trash: Option<crate::view_models::TrashViewModel>,
    /// Keeps this window's `SearchSettingsService` `Reloadable` registration alive
    /// in the shared `SettingsRegistry`, so a peer process's `search.toml` writes
    /// are picked up live — the same story as [`backup_settings_reloadable`](Self::backup_settings_reloadable).
    search_settings_reloadable: Option<Rc<dyn Reloadable>>,
    root_child: Option<WidgetId>,
    /// Keeps `backup_settings`'s `Reloadable` registration alive in the app's
    /// shared `SettingsRegistry` (only a `Weak` is held internally — see
    /// `SettingsRegistry::register`'s docs) so the settings-file watcher keeps
    /// applying a peer process's `backup.toml` writes to this handle for as
    /// long as `App` lives. Registered once, on first build.
    backup_settings_reloadable: Option<Rc<dyn Reloadable>>,
    /// Keeps the dictionary accepted-licence store's `Reloadable` registration alive in the
    /// shared `SettingsRegistry` (mirrors `backup_settings_reloadable`).
    dictionary_settings_reloadable: Option<Rc<dyn Reloadable>>,
    /// Keeps the user export-styles store's `Reloadable` registration alive in the shared
    /// `SettingsRegistry` (mirrors `dictionary_settings_reloadable`).
    export_styles_reloadable: Option<Rc<dyn Reloadable>>,
}

impl App {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        outline: OutlineViewModel,
        autosave_menu: Signal<bool>,
        spellcheck_menu: Signal<bool>,
        unsaved: Signal<bool>,
        pending_exit: Signal<PendingExit>,
        backup_mode: Signal<bool>,
        backup_context: Signal<Option<crate::backup::BackupContext>>,
        initial_action: PendingAction,
    ) -> Self {
        Self {
            app_ctx,
            outline,
            autosave_menu,
            spellcheck_menu,
            unsaved,
            dirty_seq: Signal::new(0),
            pending_exit,
            exit_seq: Rc::new(std::cell::Cell::new(None)),
            save_spinner: Rc::new(std::cell::RefCell::new(SpinnerGate::default())),
            save_spinner_visible: Signal::new(false),
            backup_mode,
            backup_context,
            initial_action: Some(initial_action),
            initial_loaded: false,
            editors: None,
            // *Stable* ids (not `fresh()`) so the per-work dock-layout restore
            // can match these docks across launches — see `crate::docks` docs.
            inspector_dock: DockWidgetId::from_raw(crate::docks::INSPECTOR_DOCK_ID),
            preview_dock: DockWidgetId::from_raw(crate::docks::PREVIEW_DOCK_ID),
            search_dock: DockWidgetId::from_raw(crate::docks::SEARCH_DOCK_ID),
            search: None,
            trash_dock: DockWidgetId::from_raw(crate::docks::TRASH_DOCK_ID),
            trash: None,
            search_settings_reloadable: None,
            root_child: None,
            backup_settings_reloadable: None,
            dictionary_settings_reloadable: None,
            export_styles_reloadable: None,
        }
    }
}

/// Open the persistent search-settings service (`<config_dir>/search.toml`),
/// degrading to a throwaway per-process temp file if the config dir is
/// unavailable — so the app still runs, search preferences just won't persist.
fn open_search_settings() -> crate::models::SearchSettingsService {
    use crate::models::SearchSettingsService;
    match bastyde::settings::AppPaths::new("eu", "skribisto", "Skribisto") {
        Some(paths) => SearchSettingsService::open(&paths).unwrap_or_else(|e| {
            eprintln!("search settings: open failed ({e}); using an in-memory fallback");
            SearchSettingsService::in_memory_default()
        }),
        None => SearchSettingsService::in_memory_default(),
    }
}

/// The backend mutation events that mark the work "unsaved" (and reschedule the
/// autosave debounce). Editor *typing* is caught separately via the editors'
/// `edited` signal; `Content` events (which fire only on flush) are excluded so a
/// save's own flush doesn't loop the debounce.
fn mutation_origins() -> Vec<Origin> {
    use DirectAccessEntity::{Binder, BinderItem, BinderTag, DictWord, Work};
    let mut v = Vec::new();
    for ent in [
        Work(EntityEvent::Updated),
        BinderItem(EntityEvent::Created),
        BinderItem(EntityEvent::Updated),
        BinderItem(EntityEvent::Removed),
        Binder(EntityEvent::Created),
        Binder(EntityEvent::Updated),
        Binder(EntityEvent::Removed),
        BinderTag(EntityEvent::Created),
        BinderTag(EntityEvent::Updated),
        BinderTag(EntityEvent::Removed),
        DictWord(EntityEvent::Created),
        DictWord(EntityEvent::Updated),
        DictWord(EntityEvent::Removed),
    ] {
        v.push(Origin::DirectAccess(ent));
    }
    v
}

/// "Is there anything to save right now?" — the single truth the Save affordances
/// (menu item, `editor.save` action, Ctrl+S) all gate on.
///
/// True iff the work has edits not yet on disk **and** this window is not in backup
/// mode. Backup mode is excluded because [`EditorsViewModel::save_to_disk`] is inert
/// there (the backup file is read-only; edits leave only through Save As / Restore),
/// so a Save command would be a silent no-op — better to show it disabled.
///
/// Derived, so it stays reactive: it recomputes whenever either input changes.
pub fn can_save(unsaved: &Signal<bool>, backup_mode: &Signal<bool>) -> Signal<bool> {
    unsaved.and(&backup_mode.not())
}

/// Present the shared Export modal over an already-`prepare`d [`ExportViewModel`]. Both the
/// `export.scope` (quick scope) and `work.export` (Choose…) actions funnel through here so
/// the modal chrome stays identical.
fn present_export_panel(ctx: &mut EventContext, vm: ExportViewModel) {
    ctx.present_modal(
        ModalRequest::deferred(move |t| t.add(ExportPanel::new(vm)))
            .presentation(ModalPresentation::InTree)
            .title("Export")
            .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
            .size(760, 640),
    );
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").finish()
    }
}

impl Widget for App {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // ── Layer-B view-models: created once, then shared by clone ──────────
        let settings = SettingsViewModel::new(ctx.settings());

        let app_ctx = self.app_ctx.clone();
        let column_width = settings.column_width();
        let show_synopsis = settings.synopsis_pane();
        let typography = settings.editor_typography();
        let view_memory = crate::view_models::EditorViewMemory::new(ctx.settings());
        let corkboard_defaults = settings.corkboard_defaults();
        let ids = self.outline.ids();
        let docs = ctx
            .app_state::<crate::models::OpenDocsStore>()
            .cloned()
            .expect("OpenDocsStore registered in main");
        let backup_mode_for_editors = self.backup_mode.clone();
        let dirty_seq_for_editors = self.dirty_seq.clone();
        let editors = self
            .editors
            .get_or_insert_with(|| {
                EditorsViewModel::new(
                    app_ctx,
                    column_width,
                    show_synopsis,
                    typography,
                    view_memory,
                    corkboard_defaults,
                    ids,
                    docs,
                    backup_mode_for_editors,
                    dirty_seq_for_editors,
                )
            })
            .clone();

        // Hand the editors to the per-work workspace-layout restore. It was created
        // in `main` (before any `ctx.settings()`), so it starts editor-less and is
        // wired here, on every build — idempotent (`set_editors` just re-points).
        // Kept as a local so the Load/New subscribers can drive its restore.
        let workspace_layout = ctx
            .app_state::<crate::view_models::WorkspaceLayoutViewModel>()
            .cloned();
        if let Some(layout) = &workspace_layout {
            layout.set_editors(editors.clone());
        }

        // ── Spell-checking wiring (Step 6). `docs` above was moved into the editors VM, so
        // re-fetch the shared handles for the attach loop. ──
        let spell_docs = ctx
            .app_state::<crate::models::OpenDocsStore>()
            .cloned()
            .expect("OpenDocsStore registered in main");
        // Keep the store's cached language map honest: an item's `dict_language` or
        // `sub_role` edited in place leaves the binder's shape unchanged, so only the
        // entity event can invalidate it. Every build — the subscription is scoped to
        // this one (see `OpenDocsStore::wire`).
        spell_docs.wire(ctx);
        let spellcheck = ctx
            .app_state::<crate::spellcheck::SpellcheckService>()
            .cloned()
            .expect("SpellcheckService registered in main");
        let dictionaries = ctx
            .app_state::<crate::view_models::DictionariesViewModel>()
            .cloned()
            .expect("DictionariesViewModel registered in main");
        // Squiggle colour from the theme's error role (re-attaches only on a real change, e.g.
        // a light/dark switch).
        spell_docs.set_squiggle_color(spell_underline_color(ctx.theme().colors.text_error));
        // A dictionary installed or removed → drop the engine's per-id cache (so a cached miss
        // can't hide a fresh install, nor a cached `Arc` keep a removed dictionary alive), then
        // re-attach every open document (install paints new squiggles; remove degrades
        // gracefully, never rewriting `dict_language`).
        {
            let docs = spell_docs.clone();
            let spell = spellcheck.clone();
            ctx.effect(&dictionaries.changed_signal(), move |_| {
                spell.invalidate_dictionaries();
                docs.attach_all();
            });
        }
        // A personal-dictionary change — a word added/removed from the Settings
        // pane or the editor's "Add to dictionary", including its undo/redo —
        // re-runs the live spell-check: reload the personal words and re-attach
        // every open document so squiggles update immediately. This is the single
        // place a `DictWord` mutation touches the checker; the pane and the
        // context menu both just create/remove the entity.
        for dict_word_event in [
            EntityEvent::Created,
            EntityEvent::Updated,
            EntityEvent::Removed,
        ] {
            let app_ctx = self.app_ctx.clone();
            let docs = spell_docs.clone();
            let spell = spellcheck.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::DictWord(dict_word_event)),
                move |_event: &Event| {
                    crate::view_models::reload_personal_words(&app_ctx, &spell);
                    docs.attach_all();
                },
            );
        }
        // Cross-process staleness: a peer window may have installed/removed a dictionary while
        // this one was unfocused. Re-scan on the focus-regain edge — `rescan()` bumps `changed`,
        // whose effect (above) drops the engine cache and re-attaches every document, so this must
        // NOT invalidate/attach again or each focus-regain would do all that work twice.
        {
            let dictionaries = dictionaries.clone();
            let was_active = std::cell::Cell::new(true);
            let wsig = ctx.window_active_signal();
            ctx.effect(&wsig, move |active| {
                let regained = *active && !was_active.get();
                was_active.set(*active);
                if regained {
                    dictionaries.rescan();
                }
            });
        }
        // Live cross-process reload for `dictionaries.toml` (accepted licences), mirroring the
        // backup-settings registration below.
        if self.dictionary_settings_reloadable.is_none()
            && let Some(registry) = ctx.app_state::<SettingsRegistry>().cloned()
        {
            self.dictionary_settings_reloadable =
                Some(registry.register(dictionaries.settings_reloadable()));
        }
        // Live cross-process reload for `export_styles.toml` (user styles), pulled from
        // app-state since `App` doesn't own the view-model.
        if self.export_styles_reloadable.is_none()
            && let Some(registry) = ctx.app_state::<SettingsRegistry>().cloned()
            && let Some(styles) = ctx
                .app_state::<crate::view_models::ExportStylesViewModel>()
                .cloned()
        {
            self.export_styles_reloadable = Some(registry.register(styles.settings_reloadable()));
        }

        // "Unsaved" is *derived*: the work has edits not on disk iff more mutations
        // have happened than the last completed save covered. Recomputed whenever
        // either side moves — a mutation (typing, tree edit) or a save landing.
        {
            let unsaved = self.unsaved.clone();
            let dirty_seq = self.dirty_seq.clone();
            let saved_seq = editors.saved_seq();
            let recompute = Rc::new(move || {
                let is_unsaved = dirty_seq.get() > saved_seq.get();
                if unsaved.get() != is_unsaved {
                    unsaved.set(is_unsaved);
                }
            });
            {
                let r = recompute.clone();
                ctx.effect(&self.dirty_seq, move |_| r());
            }
            ctx.effect(&editors.saved_seq(), move |_| recompute());
        }

        // Export: keep the focus-adaptive quick-scope list in step with the focused editor
        // item, so the title-bar Export split-button and the File ▸ Export submenu both
        // re-derive whenever the selection changes. (A reactive trigger can't dispatch an
        // intent, but it can query + set a signal — which is all this does.)
        if let Some(export_vm) = ctx.app_state::<ExportViewModel>().cloned() {
            let active = editors.active_item();
            export_vm.recompute_applicable(active.get()); // seed for the current focus
            ctx.effect(&active, move |id| export_vm.recompute_applicable(*id));
        }

        let outline = self.outline.clone();

        // ── The search feature's shared view-model (both docks clone it) ──────
        // Created once; it holds the results model, the persisted `search.toml`
        // service, the shared open-docs store (so its preview edits the same
        // document a tab does), and the DockingModel (to reveal the bottom band).
        let search = {
            let app_ctx = self.app_ctx.clone();
            let ids = self.outline.ids();
            let docs = ctx
                .app_state::<crate::models::OpenDocsStore>()
                .cloned()
                .expect("OpenDocsStore registered in main");
            let docking = outline.docking();
            let search_dock = self.search_dock;
            let preview_dock = self.preview_dock;
            self.search
                .get_or_insert_with(|| {
                    let settings_svc = open_search_settings();
                    let results = crate::models::SearchResultsModel::new(
                        app_ctx.clone(),
                        ids.work_info_id.clone(),
                    );
                    SearchReplaceViewModel::new(
                        app_ctx,
                        ids,
                        results,
                        settings_svc,
                        docs,
                        docking,
                        preview_dock,
                        search_dock,
                    )
                })
                .clone()
        };

        // ── The trash feature's shared view-model ────────────────────────────
        // Shares the outline's DockingModel (the leading rail hosts several tabs)
        // and a distinct dock id; created once, not registered as app_state.
        let trash = {
            let app_ctx = self.app_ctx.clone();
            let ids = self.outline.ids();
            let docking = outline.docking();
            let trash_dock = self.trash_dock;
            self.trash
                .get_or_insert_with(|| {
                    let model =
                        crate::models::TrashTreeModel::new(app_ctx.clone(), ids.work_id.clone());
                    crate::view_models::TrashViewModel::new(app_ctx, ids, model, docking, trash_dock)
                })
                .clone()
        };
        // Live cross-process reload for `search.toml` — register this window's
        // service into the shared `SettingsRegistry` once (same story as
        // `backup_settings` above).
        if self.search_settings_reloadable.is_none()
            && let Some(registry) = ctx.app_state::<SettingsRegistry>().cloned()
        {
            self.search_settings_reloadable = Some(registry.register(search.settings_reloadable()));
        }
        // Re-seed the search inputs from the opened project's saved preferences,
        // and drop any preview held for the previous project.
        {
            let s = search.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |_e: &Event| s.restore_for_project(),
            );
        }
        {
            let s = search.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::NewWork),
                move |_e: &Event| s.restore_for_project(),
            );
        }
        {
            let s = search.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::CloseWork),
                move |_e: &Event| s.clear_preview(),
            );
        }

        // Persist live theme / interface-language changes into the keys the
        // startup restore reads. The Settings window drives these via the
        // framework's `ThemeSwitcher` / `LanguageSwitcher`, which apply the change
        // live (`EventContext::set_theme` / `set_locale`) but do not themselves
        // persist; these effects mirror the live value into `DARK_KEY` /
        // `LOCALE_KEY` so it restores next launch (see `main::read_prefs`).
        {
            let dark = settings.dark();
            let theme_sig = ctx.theme_signal().clone();
            ctx.effect(&theme_sig, move |t| {
                let is_dark = t.is_dark();
                if dark.get() != is_dark {
                    dark.set(is_dark);
                }
            });
        }
        if let Some(locale_sig) = bastyde::i18n::current_locale() {
            let persisted = settings.locale();
            ctx.effect(&locale_sig, move |l| {
                let tag = l.to_string();
                if persisted.get() != tag {
                    persisted.set(tag);
                }
            });
        }

        // ── Layer-A singles: id-only global state + reactive entity handles ──
        // Created in `main`, shared via `app_state`. `wire` installs each single's
        // event subscriptions on this (process-lifetime) widget; they are
        // re-pointed on `LoadWork` below.
        let ids = ctx
            .app_state::<AppIds>()
            .cloned()
            .expect("AppIds registered in main");
        let single_work = ctx
            .app_state::<SingleWork>()
            .cloned()
            .expect("SingleWork registered in main");
        let single_work_info = ctx
            .app_state::<SingleWorkInfo>()
            .cloned()
            .expect("SingleWorkInfo registered in main");
        single_work.wire(ctx);
        single_work_info.wire(ctx);

        // The project-lifecycle view-model: the shared Load/New/Close sequence, which was
        // three hand-kept-in-step closures here. Built fresh each build — it holds only
        // clones of handles that are themselves stable across builds, so re-creating it is
        // idempotent; the subscribers below capture their own clone.
        let lifecycle = crate::view_models::ProjectLifecycleViewModel::new(
            self.app_ctx.clone(),
            ids.clone(),
            outline.clone(),
            editors.clone(),
            trash.clone(),
            single_work.clone(),
            single_work_info.clone(),
            spell_docs.clone(),
            spellcheck.clone(),
            self.dirty_seq.clone(),
            self.backup_mode.clone(),
            self.backup_context.clone(),
            workspace_layout.clone(),
        );
        // The personal-dictionary view-model (registered in `main`) — wire its
        // held list-model + single so the Settings pane stays live and the
        // editor's "Add to dictionary" reaches a wired handle.
        if let Some(user_dictionary) = ctx
            .app_state::<crate::view_models::UserDictionaryViewModel>()
            .cloned()
        {
            user_dictionary.wire(ctx);
        }
        // Backup scheduler + settings (registered in `main`). The scheduler drives
        // every trigger and holds the singles; the settings VM tracks the active
        // project for the per-project settings pane.
        let backup_scheduler = ctx
            .app_state::<BackupSchedulerViewModel>()
            .cloned()
            .expect("BackupSchedulerViewModel registered in main");
        let backup_settings = ctx
            .app_state::<BackupSettingsViewModel>()
            .cloned()
            .expect("BackupSettingsViewModel registered in main");
        let restore_vm = ctx
            .app_state::<crate::view_models::BackupRestoreViewModel>()
            .cloned()
            .expect("BackupRestoreViewModel registered in main");
        let save_as_vm = ctx
            .app_state::<SaveAsViewModel>()
            .cloned()
            .expect("SaveAsViewModel registered in main");
        // Live cross-process reload for `backup.toml` (T1-6 continued): register
        // this window's `BackupSettingsService` into the app's shared
        // `SettingsRegistry` once, so the settings-file watcher (installed by
        // `BastydeAppBuilder` whenever a settings bundle is configured — see
        // `main.rs`) reloads it in place the moment a peer window (another
        // Skribisto process, one per project) writes an override / policy /
        // bookkeeping change. Without this, `backup_settings`'s reads would
        // only ever see this process's own last write (reads no longer poll —
        // see `models::backup_settings_file`'s module docs). Absent registry
        // (e.g. a headless/test build with no settings bundle) is a silent
        // no-op: the service still works, peer writes just aren't picked up
        // live until this process next writes something itself.
        if self.backup_settings_reloadable.is_none()
            && let Some(registry) = ctx.app_state::<SettingsRegistry>().cloned()
        {
            self.backup_settings_reloadable =
                Some(registry.register(backup_settings.service().as_reloadable()));
        }
        // T1-2: install the real flush hook — every backup trigger
        // (`backup_now` / `on_open` / `interval_tick` / `on_close_flow`) flushes
        // the live editor buffers into the store *before* it reads it, so a
        // long typing session in an unfocused tab is never lost to
        // skip-if-unchanged. The scheduler's `flush_hook` cell is shared across
        // every clone already handed out (the window close guard in `main.rs`,
        // `App`'s own exit-guard effect below), so installing it here — once,
        // before any trigger can fire — makes it visible everywhere at once.
        backup_scheduler.set_flush_hook(Rc::new({
            let editors = editors.clone();
            move || editors.flush_all()
        }));
        // The same invariant for the *other* two paths that serialize the store on
        // a background thread: Save As and Restore. Typing only marks a doc dirty
        // (`OpenDoc::mark_dirty_fn`) — the prose reaches `Content` only on a flush
        // — so a read-only background op that starts without one writes the
        // pre-edit text. `save_work` gets this via `EditorsViewModel::save_to_disk`
        // and the backups via the hook above; these two had no flush at all, which
        // is worst in backup mode, where Save is off and Save As / Restore are the
        // only ways the edits can be kept at all.
        save_as_vm.set_flush_hook(Rc::new({
            let editors = editors.clone();
            move || editors.flush_all()
        }));
        restore_vm.set_flush_hook(Rc::new({
            let editors = editors.clone();
            move || editors.flush_all()
        }));
        // The project-switch guard (New Work / Open Work / "Open here" / the import
        // toast — every command that replaces this window's project in place) needs
        // two things only the view layer has: a way to write the project to disk
        // (returning the op id, so a *failed* save drops the parked switch instead
        // of stranding it), and a way to put the New Work form on screen. Installed
        // here, once — the guard itself is created in `main` (it holds the same
        // `unsaved`/`backup_mode`/autosave signals as the close guard) and reached
        // from outside `App` via app-state.
        let project_switch = ctx
            .app_state::<ProjectSwitchViewModel>()
            .cloned()
            .expect("ProjectSwitchViewModel registered in main");
        project_switch.set_save_hook(Rc::new({
            let editors = editors.clone();
            move || editors.request_save()
        }));
        project_switch.set_new_work_form_hook(Rc::new({
            let app_ctx = self.app_ctx.clone();
            move |c: &mut EventContext| {
                let app_ctx = app_ctx.clone();
                c.present_modal(
                    ModalRequest::deferred(move |t| t.add(NewWorkPanel::new(app_ctx)))
                        .presentation(ModalPresentation::InTree)
                        .title("New Work")
                        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
                        .size(600, 680),
                );
            }
        }));
        // Keep the outline tree reactive to *all* structural mutations (incl. the
        // manuscript streams' rename/merge/split/add), not just the outline's own.
        self.outline.wire(ctx);
        // The trash model must be wired here too — at `App` (a stable root), not
        // lazily from its dock content — so it stays subscribed even while the
        // trash tab is a background rail tab and survives hide/reveal cycles.
        trash.wire(ctx);

        // ── App-global commands (the scriptable surface) ─────────────────────
        // Registered with `register_action_global` so they're reachable as a
        // dispatch fallback regardless of where the intent originates — the
        // title-bar menu (which renders in an overlay, NOT under `App`), a global
        // shortcut anchored at the root, or any content handler. A plain
        // `register_action` would only fire on `App`'s own source→root path,
        // which the chrome-fired menu never touches.
        // F9, not Ctrl+B: Ctrl+B is the editor's built-in bold command, and a
        // Global shortcut is resolved *before* the focused widget sees the raw
        // key — so a Ctrl+B binding here would shadow `RichTextEditor`'s bold.
        ctx.register_shortcut_global(
            Shortcut::new("outline.toggle")
                .name("Toggle Outline")
                .primary(KeyStroke::new(Key::F9, Modifiers::NONE))
                .build(),
        );
        // F7 — the spell-check key every office suite has used for thirty years. A bare
        // function key for the same reason F9 is: a Global shortcut resolves before the
        // focused widget sees the raw key, so any Ctrl+letter here would shadow one of
        // `RichTextEditor`'s built-in commands.
        ctx.register_shortcut_global(
            Shortcut::new("spellcheck.toggle")
                .name("Check Spelling")
                .primary(KeyStroke::new(Key::F7, Modifiers::NONE))
                .build(),
        );
        // Phase 0.2 stub — F10 collapses/reveals the BOTTOM band, so the probe can
        // exercise the `visible_when` park/unpark path that dock content takes when
        // its side hides. (F9 only relayouts; it never parks the bottom content.)
        // Removed with the stub.
        ctx.register_shortcut_global(
            Shortcut::new("preview.toggle")
                .name("Toggle Preview Band")
                .primary(KeyStroke::new(Key::F10, Modifiers::NONE))
                .build(),
        );
        // Ctrl+Shift+E → the Export Choose… picker (Ctrl+E is the editor's centre-align).
        ctx.register_shortcut_global(
            Shortcut::new("work.export")
                .name("Export…")
                .primary(KeyStroke::new(Key::E, Modifiers::CTRL | Modifiers::SHIFT))
                .build(),
        );
        {
            let docking = self.outline.docking();
            ctx.register_action_global(Action::new("preview.toggle").on_invoke(move |_i, _c| {
                docking.toggle_side_visible(DockSide::Bottom);
            }));
        }
        // Ctrl+F opens the per-editor find banner in the focused pane's active
        // tab (its `FindViewModel`). A *global* shortcut is resolved before the
        // focused widget sees the key — the editor must not eat Ctrl+F — but the
        // action reads which tab is focused, so it targets the right editor even
        // in a split view.
        ctx.register_shortcut_global(
            Shortcut::new("editor.find")
                .name("Find")
                .primary(KeyStroke::ctrl(Key::F))
                .build(),
        );
        {
            let editors = editors.clone();
            ctx.register_action_global(
                Action::new("editor.find").on_invoke(move |_i, _c| editors.open_find()),
            );
        }
        // Ctrl+R opens the find banner in replace mode; F3 / Shift+F3 step through
        // matches — the common find-bar chords, all targeting the focused tab.
        ctx.register_shortcut_global(
            Shortcut::new("editor.replace")
                .name("Replace")
                .primary(KeyStroke::ctrl(Key::R))
                .build(),
        );
        ctx.register_shortcut_global(
            Shortcut::new("editor.find_next")
                .name("Next Match")
                .primary(KeyStroke::new(Key::F3, Modifiers::NONE))
                .build(),
        );
        ctx.register_shortcut_global(
            Shortcut::new("editor.find_prev")
                .name("Previous Match")
                .primary(KeyStroke::new(Key::F3, Modifiers::SHIFT))
                .build(),
        );
        {
            let editors = editors.clone();
            ctx.register_action_global(
                Action::new("editor.replace").on_invoke(move |_i, _c| editors.open_find_replace()),
            );
        }
        {
            let editors = editors.clone();
            ctx.register_action_global(
                Action::new("editor.find_next").on_invoke(move |_i, c| editors.find_next(c)),
            );
        }
        {
            let editors = editors.clone();
            ctx.register_action_global(
                Action::new("editor.find_prev").on_invoke(move |_i, c| editors.find_prev(c)),
            );
        }
        // Ctrl+Shift+F reveals the search & replace dock; Ctrl+Shift+H reveals it
        // *and* discloses the replace row. Global (resolved before a focused
        // editor), and Shift-qualified so neither shadows Ctrl+F (find banner) or
        // an editor chord. Only ever fired by keystroke, so — like `work.open` /
        // `editor.save` — they are global actions with no `AppIntent` variant.
        ctx.register_shortcut_global(
            Shortcut::new("search.show")
                .name("Search in Project")
                .primary(KeyStroke::new(Key::F, Modifiers::CTRL | Modifiers::SHIFT))
                .build(),
        );
        ctx.register_shortcut_global(
            Shortcut::new("search.replace")
                .name("Replace in Project")
                .primary(KeyStroke::new(Key::H, Modifiers::CTRL | Modifiers::SHIFT))
                .build(),
        );
        {
            let docking = outline.docking();
            let search_dock = self.search_dock;
            ctx.register_action_global(Action::new("search.show").on_invoke(move |_i, _c| {
                docking.reveal_dock(search_dock);
            }));
        }
        {
            let docking = outline.docking();
            let search_dock = self.search_dock;
            let search = search.clone();
            ctx.register_action_global(Action::new("search.replace").on_invoke(move |_i, _c| {
                docking.reveal_dock(search_dock);
                search.set_show_replace(true);
            }));
        }
        // ── Trash dock commands ──────────────────────────────────────────────
        {
            let docking = outline.docking();
            let trash_dock = self.trash_dock;
            ctx.register_action_global(Action::new("trash.show").on_invoke(move |_i, _c| {
                docking.reveal_dock(trash_dock);
            }));
        }
        {
            let trash = trash.clone();
            ctx.register_action_global(
                Action::new("trash.empty").on_invoke(move |_i, c| trash.confirm_empty_trash(c)),
            );
        }
        {
            let trash = trash.clone();
            ctx.register_action_global(Action::new("trash.restore").on_invoke(move |i, c| {
                if let Some(AppIntent::RestoreTrashed { trash_info_ids }) =
                    AppIntent::from_intent(i)
                {
                    trash.restore(c, trash_info_ids);
                }
            }));
        }
        {
            let trash = trash.clone();
            ctx.register_action_global(Action::new("trash.restore_item").on_invoke(move |i, c| {
                if let Some(AppIntent::RestoreTrashedItem { item_id }) = AppIntent::from_intent(i) {
                    trash.restore_item(c, *item_id);
                }
            }));
        }
        {
            let trash = trash.clone();
            ctx.register_action_global(Action::new("trash.delete_forever").on_invoke(
                move |i, c| {
                    if let Some(AppIntent::DeleteTrashForever { trash_info_ids }) =
                        AppIntent::from_intent(i)
                    {
                        trash.confirm_delete_forever(c, trash_info_ids);
                    }
                },
            ));
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("outline.toggle").on_invoke(move |_i, _c| outline.toggle()),
            );
        }
        // The master spell-check switch. Flips the *setting* and nothing else — the effect
        // above owns the engine, so there is exactly one path from the key to `set_enabled`
        // no matter which surface fired. The toast lives here rather than in that effect
        // because only an action gets an `EventContext`.
        {
            let enabled = SettingsViewModel::new(ctx.settings()).spellcheck_enabled();
            let docs = spell_docs.clone();
            let dicts = dictionaries.clone();
            ctx.register_action_global(Action::new("spellcheck.toggle").on_invoke(
                move |_i, c: &mut EventContext| {
                    let now_on = !enabled.get();
                    enabled.set(now_on);
                    // Turning it back on with no dictionary installed reproduces the exact
                    // symptom this switch exists to end: a silent absence of squiggles.
                    // `offer_missing_dictionaries` otherwise only ever fires on Load/New.
                    if now_on {
                        offer_missing_dictionaries(&docs, &dicts, c);
                    }
                },
            ));
        }
        {
            let editors = editors.clone();
            ctx.register_action_global(Action::new("editor.open_item").on_invoke(move |i, _c| {
                if let Some(AppIntent::OpenItem { item_id, title }) = AppIntent::from_intent(i) {
                    editors.open_or_focus(*item_id, title);
                }
            }));
        }
        {
            let editors = editors.clone();
            ctx.register_action_global(Action::new("editor.open_item_to_side").on_invoke(
                move |i, _c| {
                    if let Some(AppIntent::OpenItemToSide { item_id, title }) =
                        AppIntent::from_intent(i)
                    {
                        editors.open_to_side(*item_id, title);
                    }
                },
            ));
        }
        // Add the resolved selection/caret word(s) to the personal dictionary,
        // fired from the editor's "Add to dictionary" context-menu item. The menu
        // mounts at the arena root, so only a **global** action reaches it. The
        // resulting `DictWord(Created)` event drives the live squiggle refresh
        // (the subscription above); here we just create the entities and toast.
        {
            ctx.register_action_global(Action::new("editor.add_to_dictionary").on_invoke(
                move |i, c| {
                    let Some(AppIntent::AddWordsToDictionary { words }) = AppIntent::from_intent(i)
                    else {
                        return;
                    };
                    let Some(vm) = c
                        .app_state::<crate::view_models::UserDictionaryViewModel>()
                        .cloned()
                    else {
                        return;
                    };
                    let sample = words.first().cloned();
                    let ids = vm.add_words(words);
                    vm.added_toast(c, ids, sample);
                },
            ));
        }
        // Export a quick scope resolved from the current focus. Fired (with the scope as
        // payload) by the title-bar Export split-button and the File ▸ Export submenu.
        // Flushes the editors + reads the anchor here, so the panel's preview and the
        // committed export both see current prose; the panel is modal, so no edit slips in
        // behind it.
        {
            let editors = editors.clone();
            ctx.register_action_global(Action::new("export.scope").on_invoke(move |i, c| {
                let Some(AppIntent::ExportScoped { scope }) = AppIntent::from_intent(i) else {
                    return;
                };
                let scope = scope.clone();
                let Some(vm) = c.app_state::<ExportViewModel>().cloned() else {
                    return;
                };
                editors.flush_all();
                let anchor = editors.active_item().get();
                vm.prepare(scope, anchor);
                present_export_panel(c, vm);
            }));
        }
        // The Choose… entry point (File ▸ Export ▸ Choose…, the split-button dropdown, and
        // Ctrl+Shift+E): open the panel straight into the checkbox tree (Custom scope),
        // independent of focus. Not Ctrl+E — that is the editor's centre-align.
        {
            let editors = editors.clone();
            ctx.register_action_global(Action::new("work.export").on_invoke(move |_i, c| {
                let Some(vm) = c.app_state::<ExportViewModel>().cloned() else {
                    return;
                };
                editors.flush_all();
                let anchor = editors.active_item().get();
                vm.prepare(ExportScopeKind::Custom, anchor);
                present_export_panel(c, vm);
            }));
        }
        // Ctrl+S: flush every editor to the store, then save the project to disk.
        // Gated on `can_save` (dirty && !backup mode) at *both* ends: the shortcut
        // stops matching the keystroke, and the action stops matching the intent —
        // so nothing to save means the menu item greys out (same signal, in `main`),
        // Ctrl+S is inert, and a scripted `editor.save` intent is a no-op instead of
        // a pointless disk write. The exit guards call `save_to_disk()` directly,
        // not through the intent, so save-then-close still works.
        let can_save = can_save(&self.unsaved, &self.backup_mode);
        ctx.register_shortcut_global(
            Shortcut::new("editor.save")
                .name("Save")
                .primary(KeyStroke::ctrl(Key::S))
                .enabled_when(can_save.clone())
                .build(),
        );
        {
            let editors = editors.clone();
            ctx.register_action_global(
                Action::new("editor.save")
                    .enabled_when(can_save)
                    .on_invoke(move |_i, _c| editors.save_to_disk()),
            );
        }
        // ── File / app commands (the scriptable surface for the title bar). ──
        // Global (not `register_action`/`register_shortcut`) so they're reached
        // from the title-bar overlay menu — which renders as a sibling of `App`,
        // NOT on `App`'s source→root path — as well as from their shortcuts.
        // New Work (Ctrl+N) and Open Work (Ctrl+O) both **replace this window's
        // project in place** (the backend closes the open Work first). So both go
        // through `ProjectSwitchViewModel` — the same Save/Discard/Cancel guard the
        // close paths use — instead of destroying unsaved edits outright, which is
        // what they did before. The guard performs the switch itself, now or once
        // the deferred save lands; these actions only *ask* for it.
        ctx.register_shortcut_global(
            Shortcut::new("work.new")
                .name("New Work")
                .primary(KeyStroke::ctrl(Key::N))
                .build(),
        );
        {
            let switch = project_switch.clone();
            ctx.register_action_global(
                Action::new("work.new")
                    .on_invoke(move |_i, c| switch.request(c, PendingSwitch::NewWork)),
            );
        }
        // Open Work (Ctrl+O): native picker for an existing `.skrib`, then the
        // guard, then load. The guard runs *after* the pick, so cancelling the
        // picker — or choosing a backup file, which opens in its own process and
        // leaves this project untouched — never prompts about unsaved changes.
        ctx.register_shortcut_global(
            Shortcut::new("work.open")
                .name("Open Work")
                .primary(KeyStroke::ctrl(Key::O))
                .build(),
        );
        {
            let switch = project_switch.clone();
            ctx.register_action_global(
                Action::new("work.open").on_invoke(move |_i, c| open_work_flow(switch.clone(), c)),
            );
        }
        // Open an already-chosen path (payload in the intent) — the switcher
        // popover's "Open here" and the import toast's "Open now", both of which
        // live outside `App` and pick the path themselves. Same guard, no picker.
        {
            let switch = project_switch.clone();
            ctx.register_action_global(Action::new("work.open_path").on_invoke(move |i, c| {
                if let Some(AppIntent::OpenWorkPath { path }) = AppIntent::from_intent(i) {
                    switch.request(c, PendingSwitch::OpenWork(path.clone()));
                }
            }));
        }
        // Import from Plume Creator: present the Import Plume modal (menu-only, no
        // shortcut). Global so the title-bar overlay menu reaches it — like work.new.
        // The panel is built over the shared, app-state `ImportPlumeViewModel` (the
        // same instance the long-operation events are routed to below), reset first
        // so a previous session's paths don't linger.
        ctx.register_action_global(Action::new("work.import_plume").on_invoke(move |_i, c| {
            let Some(vm) = c.app_state::<ImportPlumeViewModel>().cloned() else {
                return;
            };
            vm.reset_form();
            c.present_modal(
                ModalRequest::deferred(move |t| t.add(ImportPlumePanel::new(vm)))
                    .presentation(ModalPresentation::InTree)
                    .title("Import Plume Creator project")
                    .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
                    .size(600, 500),
            );
        }));
        // Close Work (Ctrl+W): the `work.close` *action* is registered further
        // down (it shares the unsaved-changes guard with the window close); here
        // we only add its global shortcut.
        ctx.register_shortcut_global(
            Shortcut::new("work.close")
                .name("Close Work")
                .primary(KeyStroke::ctrl(Key::W))
                .build(),
        );
        // Settings (Ctrl+,): present the settings modal.
        ctx.register_shortcut_global(
            Shortcut::new("app.settings")
                .name("Settings")
                .primary(KeyStroke::ctrl(Key::Character(',')))
                .build(),
        );
        ctx.register_action_global(Action::new("app.settings").on_invoke(|_i, c| {
            c.present_modal(
                ModalRequest::deferred(|t| t.add(SettingsPanel::new()))
                    .presentation(ModalPresentation::InTree)
                    .title("Settings")
                    .size(920, 620)
                    // Not easily dismissable — like a critical MessageBox. Only
                    // the panel's own close button / Cancel / OK close it (each
                    // calls `ctx.dismiss_modal()`); Escape and outside clicks do
                    // not, so a stray click never discards a settings session.
                    .close_behavior(ModalCloseBehavior::Manual),
            );
        }));
        // Quit (Ctrl+Q): really terminates the process (see `PendingExit::Quit`'s
        // docs), after the same unsaved-changes guard as every other exit path —
        // `guard_unsaved_exit`, shared with `work.close`'s action below and the
        // project window's own `on_close_requested` guard (`windows.rs`).
        // Deliberately does NOT go through `close_window()`: that would only ever
        // land back on the project window's close guard, which always returns to
        // the Launcher — never terminates. The title-bar X / Alt+F4 path is
        // unchanged (still `close_window()`, still returns to the Launcher).
        ctx.register_shortcut_global(
            Shortcut::new("app.quit")
                .name("Quit")
                .primary(KeyStroke::ctrl(Key::Q))
                .build(),
        );
        {
            let app_ctx3 = self.app_ctx.clone();
            let unsaved = self.unsaved.clone();
            let autosave = settings.autosave();
            let pending = self.pending_exit.clone();
            let scheduler = backup_scheduler.clone();
            let backup_mode = self.backup_mode.clone();
            ctx.register_action_global(Action::new("app.quit").on_invoke(move |_i, ctx| {
                guard_unsaved_exit(
                    ctx,
                    &app_ctx3,
                    unsaved.get(),
                    backup_mode.get(),
                    autosave.get(),
                    &pending,
                    &scheduler,
                    PendingExit::Quit,
                );
            }));
        }
        // "Welcome" now means "close this work and go back to the Launcher"
        // (the launcher-window model — Welcome is a real window, not a modal
        // any more). Fired from File ▸ Welcome… and the brand icon button.
        // A pure alias for `work.close`, dispatched by name, so it shares
        // that action's exact guard (unsaved-changes prompt, the on-close
        // backup, backup-mode handling) rather than bypassing it — do NOT
        // inline a second copy of that logic here. Global so the title-bar
        // overlay menu/button reach it (house rule).
        ctx.register_action_global(
            Action::new("welcome.show").on_invoke(|_i, c| c.send_intent(Intent::new("work.close"))),
        );

        // ── Binder-tree commands (the scriptable surface for the outline). ───
        // Each drives an `OutlineViewModel` method; the context menu and key
        // handlers below also call these methods directly.
        {
            let outline = outline.clone();
            ctx.register_action_global(Action::new("binder.new_item").on_invoke(move |i, _c| {
                if let Some(AppIntent::NewItem {
                    create_type,
                    relation,
                    anchor_item_id,
                }) = AppIntent::from_intent(i)
                {
                    // `None` anchors on the current Outline selection; a corkboard
                    // passes its drilled-into container id explicitly.
                    outline.add_recommended(
                        anchor_item_id.map(crate::models::BinderTreeKey::Item),
                        &skribisto_model::Recommendation {
                            create_type: *create_type,
                            relation: *relation,
                        },
                    );
                }
            }));
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.rename").on_invoke(move |_i, c| outline.rename_selected(c)),
            );
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.duplicate")
                    .on_invoke(move |_i, _c| outline.duplicate_selected()),
            );
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.trash_selected")
                    .on_invoke(move |_i, _c| outline.trash_selected()),
            );
        }
        {
            // Trash one specific binder (id in the intent payload) — fired from
            // the switcher popover's context menu after its confirmation.
            let outline = outline.clone();
            ctx.register_action_global(Action::new("binder.trash").on_invoke(move |i, _c| {
                if let Some(AppIntent::TrashBinder { binder_id }) = AppIntent::from_intent(i) {
                    outline.trash_binder(*binder_id as u64);
                }
            }));
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.indent").on_invoke(move |_i, _c| outline.indent_selected()),
            );
        }
        {
            let outline = outline.clone();
            ctx.register_action_global(
                Action::new("binder.outdent").on_invoke(move |_i, _c| outline.outdent_selected()),
            );
        }
        ctx.register_shortcut_global(
            Shortcut::new("binder.duplicate")
                .name("Duplicate")
                .primary(KeyStroke::ctrl(Key::D))
                .build(),
        );

        // On project load: hand off to the lifecycle view-model (seed the ids, open the
        // per-Work undo stack, re-point the singles, rebuild the tree, drop stale tabs).
        // The two `BinderItem` subscribers below are not lifecycle — they run for every
        // edit, not just at project boundaries — so they stay here.
        {
            // An open tab does not follow its item by itself: `TabInfo::title` is a plain
            // string baked in at open time, and the `ContentTab` payload is built once for
            // the item's `(role, sub_role)`. So a rename must push the new caption, and a
            // Promote must rebuild the tab — otherwise it keeps showing a chapter's
            // segments and editors for what is now a Part.
            {
                let editors = editors.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
                    move |event: &Event| editors.items_updated(&event.ids),
                );
            }
            // A hard-removed item (Delete Forever / Empty Trash of an open item)
            // must not leave a tab pointing at a vanished entity — close it.
            {
                let editors = editors.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Removed)),
                    move |event: &Event| editors.items_removed(&event.ids),
                );
            }

            // NOTE: the on-open backup trigger is fired from the SECOND `LoadWork`
            // subscriber below (T2-4), once `backup_mode` is known — firing it here,
            // before that sniff runs, could pump a freshly-opened *backup* file into the
            // real project's retention pool before anyone knew it was a backup.
            //
            // The workspace-layout restore (open tabs + docks) is likewise driven from
            // that SECOND subscriber: it already sniffs whether the file is a backup, and
            // restore needs that answer (a backup gets a clean default desk, not the
            // source project's) — so it is supplied there rather than re-sniffed here.
            let lifecycle_load = lifecycle.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |_event: &Event| lifecycle_load.on_load(),
            );
        }

        // Detect "a backup file was opened" and enter backup mode. A separate
        // `subscribe_event_with_ctx` (needs an `EventContext` to present the choice
        // modal) reads the just-loaded path and sniffs its manifest. Opening a
        // backup always happens in its own process (the redirect in the open entry
        // points), so this only ever fires in a window dedicated to that backup.
        {
            let app_ctx = self.app_ctx.clone();
            let backup_mode = self.backup_mode.clone();
            let backup_context = self.backup_context.clone();
            let restore_vm = restore_vm.clone();
            let single_work = single_work.clone();
            let backup_settings = backup_settings.clone();
            let backup_scheduler = backup_scheduler.clone();
            let workspace_layout = workspace_layout.clone();
            // A saved per-work layout serialized before the trash dock existed
            // won't contain it; re-mounting it after restore keeps the trash panel
            // reachable in every project (and it re-saves with trash thereafter).
            let trash_docking = outline.docking();
            let trash_dock = self.trash_dock;
            let outline_dock = outline.dock_id();
            ctx.subscribe_event_with_ctx(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |_e: &Event, c: &mut EventContext| {
                    let path = frontend::commands::work_info_commands::get_all_work_info(&app_ctx)
                        .ok()
                        .and_then(|v| v.into_iter().next())
                        .and_then(|wi| wi.file_name);
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
                                    t.add(crate::backup_choice_panel::BackupChoicePanel::new(
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
                                c.show_toast(Toast::warning(tr!(backup_nudge_text())).action(
                                    ToastAction::primary(tr!(backup_nudge_action()), |c| {
                                        c.present_modal(
                                            ModalRequest::deferred(|t| {
                                                t.add(SettingsPanel::open_to_backup())
                                            })
                                            .presentation(ModalPresentation::InTree)
                                            .title("Settings")
                                            .size(920, 620)
                                            .close_behavior(ModalCloseBehavior::Manual),
                                        );
                                    }),
                                ));
                            }
                        }
                    }
                },
            );
        }

        // Route the Plume-import long operation's events to the shared
        // `ImportPlumeViewModel`, which drives its progress / cancel / success /
        // error toast. `subscribe_event_with_ctx` (not `subscribe_event`) because
        // each callback needs a fresh `EventContext` to show/replace the toast —
        // a plain subscription callback gets none. The VM filters by operation id,
        // so events from other long operations (save / backup) are ignored.
        if let Some(import_vm) = ctx.app_state::<ImportPlumeViewModel>().cloned() {
            {
                let vm = import_vm.clone();
                ctx.subscribe_event_with_ctx(
                    Origin::LongOperation(LongOperationEvent::Progress),
                    move |e: &Event, c| vm.on_long_op_progress(c, e),
                );
            }
            {
                let vm = import_vm.clone();
                ctx.subscribe_event_with_ctx(
                    Origin::LongOperation(LongOperationEvent::Completed),
                    move |e: &Event, c| vm.on_long_op_completed(c, e),
                );
            }
            {
                let vm = import_vm.clone();
                ctx.subscribe_event_with_ctx(
                    Origin::LongOperation(LongOperationEvent::Cancelled),
                    move |e: &Event, c| vm.on_long_op_cancelled(c, e),
                );
            }
            {
                let vm = import_vm.clone();
                ctx.subscribe_event_with_ctx(
                    Origin::LongOperation(LongOperationEvent::Failed),
                    move |e: &Event, c| vm.on_long_op_failed(c, e),
                );
            }
        }

        // Route the Export long operation's events to the shared `ExportViewModel`, which
        // drives its progress / cancel / success / error toast. Filters by op id, so the
        // save / import / backup long ops are ignored.
        if let Some(export_vm) = ctx.app_state::<ExportViewModel>().cloned() {
            {
                let vm = export_vm.clone();
                ctx.subscribe_event_with_ctx(
                    Origin::LongOperation(LongOperationEvent::Progress),
                    move |e: &Event, c| vm.on_long_op_progress(c, e),
                );
            }
            {
                let vm = export_vm.clone();
                ctx.subscribe_event_with_ctx(
                    Origin::LongOperation(LongOperationEvent::Completed),
                    move |e: &Event, c| vm.on_long_op_completed(c, e),
                );
            }
            {
                let vm = export_vm.clone();
                ctx.subscribe_event_with_ctx(
                    Origin::LongOperation(LongOperationEvent::Cancelled),
                    move |e: &Event, c| vm.on_long_op_cancelled(c, e),
                );
            }
            {
                let vm = export_vm.clone();
                ctx.subscribe_event_with_ctx(
                    Origin::LongOperation(LongOperationEvent::Failed),
                    move |e: &Event, c| vm.on_long_op_failed(c, e),
                );
            }
        }

        // Route the Save-As long operation's completion/failure to the shared
        // `SaveAsViewModel`, which — on success — records the new file_name/shape
        // into WorkInfo synchronously on the UI thread (save_as itself is
        // read-only). Filters by op id, so import/backup events are ignored.
        {
            let vm = save_as_vm.clone();
            ctx.subscribe_event_with_ctx(
                Origin::LongOperation(LongOperationEvent::Completed),
                move |e: &Event, c| vm.on_long_op_completed(c, e),
            );
        }
        {
            let vm = save_as_vm.clone();
            ctx.subscribe_event_with_ctx(
                Origin::LongOperation(LongOperationEvent::Failed),
                move |e: &Event, c| vm.on_long_op_failed(c, e),
            );
        }

        // The progress-recorder cadence (silent, no toast): each `save_work`
        // fires a throttled `count_words`, and its completion records today's
        // `ProgressSnapshot`. Plain `subscribe_event` (no `EventContext`) — it
        // shows no UI. Filters the completion by op id, so save / import /
        // export / backup long ops are ignored.
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
            {
                let r = recorder.clone();
                ctx.subscribe_event(
                    Origin::LongOperation(LongOperationEvent::Failed),
                    move |e: &Event| r.on_failed_or_cancelled(e),
                );
            }
            {
                let r = recorder.clone();
                ctx.subscribe_event(
                    Origin::LongOperation(LongOperationEvent::Cancelled),
                    move |e: &Event| r.on_failed_or_cancelled(e),
                );
            }
        }

        // Route the backup long operation's progress/completion/failure to the
        // shared `BackupSchedulerViewModel`, which records the per-destination
        // success hash + path, shows a progress toast (retention now runs
        // *inside* the operation — see the engine), shows the summary toast, and
        // — for an on-close backup — performs the deferred close. Filters by op
        // id (import/save-as events are ignored).
        {
            let vm = backup_scheduler.clone();
            ctx.subscribe_event_with_ctx(
                Origin::LongOperation(LongOperationEvent::Progress),
                move |e: &Event, c| vm.on_long_op_progress(c, e),
            );
        }
        {
            let vm = backup_scheduler.clone();
            ctx.subscribe_event_with_ctx(
                Origin::LongOperation(LongOperationEvent::Completed),
                move |e: &Event, c| vm.on_long_op_completed(c, e),
            );
        }
        {
            let vm = backup_scheduler.clone();
            ctx.subscribe_event_with_ctx(
                Origin::LongOperation(LongOperationEvent::Failed),
                move |e: &Event, c| vm.on_long_op_failed(c, e),
            );
        }

        // Route the restore's `save_as` op completion/failure to `BackupRestoreViewModel`
        // (it filters by its own op id, so save-as / backup / import events pass
        // through). On success it records WorkInfo and leaves backup mode.
        {
            let vm = restore_vm.clone();
            ctx.subscribe_event_with_ctx(
                Origin::LongOperation(LongOperationEvent::Completed),
                move |e: &Event, c| vm.on_long_op_completed(c, e),
            );
        }
        {
            let vm = restore_vm.clone();
            ctx.subscribe_event_with_ctx(
                Origin::LongOperation(LongOperationEvent::Failed),
                move |e: &Event, c| vm.on_long_op_failed(c, e),
            );
        }

        // On new work: same seeding as load (a project is now open), then write
        // the freshly-created project to the chosen path immediately — a
        // create-and-save. `save_to_disk` resolves the target + shape from the
        // `WorkInfo` the use case just set (from the picker path + is_folder).
        // The new project isn't on disk yet, so it starts `unsaved = true`; the
        // async save is a long op, and the SaveWork-completion handler clears
        // `unsaved` only once the write actually lands — so an exit/close during
        // the in-flight write is caught by the guards instead of dropping the file.
        {
            let lifecycle_new = lifecycle.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::NewWork),
                move |_event: &Event| lifecycle_new.on_new(),
            );
        }

        // After a project becomes live (Load or New), offer any missing dictionaries its
        // declared languages need. Registered *after* the spell-wiring subscribers above, which
        // set the project language `offer_missing_dictionaries` reads — so it runs once those
        // have populated it. Needs an `EventContext` (to raise the toast), hence a separate
        // `subscribe_event_with_ctx` per event.
        for event in [WorkManagementEvent::LoadWork, WorkManagementEvent::NewWork] {
            let docs = spell_docs.clone();
            let dictionaries = dictionaries.clone();
            ctx.subscribe_event_with_ctx(
                Origin::WorkManagement(event),
                move |_e: &Event, c: &mut EventContext| {
                    offer_missing_dictionaries(&docs, &dictionaries, c)
                },
            );
        }

        // The search corpus cache holds the parsed, folded prose of every scene of the
        // project that is open. When a project is **replaced or closed**, that prose is
        // gone from the store — and because the cache is content-addressed, its keys are
        // strings nothing will ever ask for again. Left alone it is dead weight: ~19 MB
        // for a 300k-word novel, freed only when the 128 MB ceiling eventually trips.
        //
        // This is *not* a correctness hook. The cache cannot go stale (edited prose is a
        // different key), which is the whole point of keying it on the text. It is purely
        // about not carrying the previous manuscript around.
        //
        // All three events, because all three replace or drop the open project: New Work
        // and Open Work (and the switcher's "Open here", and the import toast's "Open
        // now") both go through `load_work`/`new_work`, which close the previous Work in
        // the backend rather than via a UI-side `close_work`.
        for event in [
            WorkManagementEvent::LoadWork,
            WorkManagementEvent::NewWork,
            WorkManagementEvent::CloseWork,
        ] {
            ctx.subscribe_event(Origin::WorkManagement(event), move |_event: &Event| {
                frontend::search_management::corpus_cache::clear();
            });
        }

        // On work close: forget the ids, empty the tree, drop the tabs, and clear
        // the singles (the store no longer holds the work).
        {
            let lifecycle_close = lifecycle.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::CloseWork),
                move |_event: &Event| lifecycle_close.on_close(),
            );
        }

        // App mediates the two peer view-models: *activating* a binder item
        // (click or Enter — NOT arrow navigation, which only moves the selection)
        // opens (or focuses) its editor tab. The tree fires this via
        // `TreeView::on_activate`; App supplies the open callback so neither
        // view-model imports the other.
        let on_open: crate::docks::outline::OpenItemFn = {
            let editors = editors.clone();
            Rc::new(move |item_id, title| editors.open_or_focus(item_id, &title))
        };

        // Keep the "open document" id in sync with each pane's active tab (open,
        // close, or a tab-bar click), so the binder's open-item marker tracks the
        // focused pane. A selection change in a pane also marks it focused and
        // flushes pending edits to the store — autosave on a natural boundary
        // (changed fields only; clean tabs are a no-op). One effect per pane.
        for side in [Side::Primary, Side::Secondary] {
            let editors = editors.clone();
            ctx.effect(&editors.selected(side), move |_| {
                editors.flush_all();
                editors.set_focused(side);
            });
        }

        // ── Autosave ─────────────────────────────────────────────────────────
        // Mirror the persisted setting into the menu's plain signal (the title-bar
        // menu lives outside `App` and can't read `ctx.settings()`).
        {
            self.autosave_menu.set(settings.autosave().get());
            let menu = self.autosave_menu.clone();
            ctx.effect(&settings.autosave(), move |a| menu.set(*a));
        }
        // ── The master spell-check switch ────────────────────────────────────
        // One setting, four surfaces (title-bar toggle, View ▸ Check spelling, F7,
        // Settings ▸ Spelling) — they all write `SPELLCHECK_ENABLED_KEY`, and this effect is
        // the only place that reads it into the engine. `set_enabled` gates `build_checker`,
        // so `attach_all` then rebuilds every open document through the existing degrade
        // path: off clears the squiggles, on paints them back.
        //
        // The *toast* for "you turned it on but have no dictionary" cannot live here —
        // `ctx.effect` gets no `EventContext` — so it hangs off the `spellcheck.toggle`
        // action below, which does. That covers the title bar, the menu and F7; the
        // Settings ▸ Spelling row writes the signal directly and shows no toast, which is
        // tolerable precisely there: Dictionaries is the next page down the same tree.
        {
            self.spellcheck_menu
                .set(settings.spellcheck_enabled().get());
            let menu = self.spellcheck_menu.clone();
            let spell = spellcheck.clone();
            let docs = spell_docs.clone();
            // Seed the engine before the first attach, or a launch with the switch off
            // would paint one frame of squiggles before the effect caught up.
            spell.set_enabled(settings.spellcheck_enabled().get());
            ctx.effect(&settings.spellcheck_enabled(), move |on| {
                menu.set(*on);
                // Not a real flip → don't re-attach every open document for nothing.
                if spell.set_enabled(*on) {
                    docs.attach_all();
                }
            });
        }
        // A hidden synopsis pane should not cost a full re-tokenise of its (often huge) text on
        // every re-attach. Mirror the global synopsis-pane setting into the store, which puts each
        // open doc's synopsis spell session to sleep while the pane is hidden and wakes it (with
        // one catch-up rebuild) when it returns. Seed it before the first document opens.
        {
            let docs = spell_docs.clone();
            docs.set_synopsis_visible(settings.synopsis_pane().get());
            ctx.effect(&settings.synopsis_pane(), move |v| {
                docs.set_synopsis_visible(*v)
            });
        }
        // Dirty tracking + debounced autosave-to-disk. Every mutation (editor
        // typing via the editors' `edited` signal, plus tree/metadata events)
        // marks the work `unsaved` and — when autosave is on — (re)schedules a
        // one-shot wake ~1.5 s out. `wake_at` keeps the loop asleep until the
        // deadline (no 60 fps drain); the `frame_tick` effect only runs on the
        // frames that actually pump, and fires the save when the deadline passes.
        // The countdown policy lives in `view_models::timers` (pure, `now`-injected,
        // unit-tested); `App` keeps only the two effects it must own — arming the
        // framework's `wake_at` and actually writing to disk.
        {
            use std::time::Instant;
            let countdown = Rc::new(crate::view_models::AutosaveCountdown::new());
            let wake = ctx.wake_at_handle();
            let autosave = settings.autosave();

            let on_mutation = {
                let countdown = countdown.clone();
                let wake = wake.clone();
                let autosave = autosave.clone();
                let dirty_seq = self.dirty_seq.clone();
                Rc::new(move || {
                    // Bump the edit sequence: this mutation is now ahead of whatever
                    // the last save covered, so the derived `unsaved` goes true —
                    // and stays true if the save in flight (if any) predates it.
                    dirty_seq.set(dirty_seq.get() + 1);
                    if let Some(at) = countdown.on_mutation(Instant::now(), autosave.get()) {
                        wake.set(Some(at));
                    }
                })
            };
            {
                let oc = on_mutation.clone();
                ctx.effect(&editors.edited_signal(), move |_| oc());
            }
            for origin in mutation_origins() {
                let oc = on_mutation.clone();
                ctx.subscribe_event(origin, move |_e: &Event| oc());
            }
            {
                let editors = editors.clone();
                let autosave = autosave.clone();
                let tick = ctx.frame_tick();
                ctx.effect(&tick, move |_| {
                    let (save, resleep) = countdown.tick(Instant::now(), autosave.get());
                    if save {
                        editors.save_to_disk();
                    }
                    if let Some(at) = resleep {
                        wake.set(Some(at));
                    }
                });
            }
        }

        // Periodic "every N hours" backup while a project is open. Mirrors the
        // autosave timer: a `wake_at` deadline keeps the loop asleep; the
        // `frame_tick` effect fires `interval_tick` when the deadline passes and
        // re-arms. The interval (and whether it's on) comes from the open project's
        // effective policy via the scheduler; `None` disarms it.
        {
            use crate::view_models::IntervalTick;
            use std::time::{Duration, Instant};
            let countdown = crate::view_models::IntervalCountdown::new(
                backup_scheduler.completed_epoch(),
            );
            let wake = ctx.wake_at_handle();
            let scheduler = backup_scheduler.clone();
            let tick = ctx.frame_tick();
            ctx.effect(&tick, move |_| {
                let interval = scheduler.interval_secs().map(Duration::from_secs);
                match countdown.tick(Instant::now(), interval, scheduler.completed_epoch()) {
                    IntervalTick::Disarmed => {}
                    IntervalTick::Sleep(at) => wake.set(Some(at)),
                    IntervalTick::Fire(next) => {
                        scheduler.interval_tick();
                        wake.set(Some(next));
                    }
                }
            });
        }

        // ── Exit guards (Close Work / Quit / window close) ───────────────────
        // The window close guard (in `main`) and the `work.close` action set
        // `pending_exit`; that asks for a disk save and remembers the edit sequence
        // it will cover. The close is performed only once *that* sequence is on disk
        // (below) — so the async write is awaited, never raced.
        {
            let editors = editors.clone();
            let exit_seq = self.exit_seq.clone();
            let pending = self.pending_exit.clone();
            ctx.effect(&self.pending_exit, move |pe| {
                if *pe == PendingExit::None {
                    return;
                }
                match editors.request_save() {
                    Some(covers) => exit_seq.set(Some(covers)),
                    // The command could not be issued, so no operation exists — no
                    // completion and no failure event will ever arrive. Leaving the
                    // close armed on a write that will never happen would make
                    // Ctrl+W / Ctrl+Q / the window's X do nothing at all, forever.
                    // Disarm instead: the window stays open and usable, and the next
                    // close attempt retries. (No toast: an `effect` has no
                    // `EventContext`. `save_work` can only fail to *start* on a
                    // poisoned store lock, at which point this log line is the least
                    // of it — every other path that can reach a context does toast.)
                    None => {
                        exit_seq.set(None);
                        pending.set(PendingExit::None);
                        eprintln!("skribisto: could not start the save for a deferred close");
                    }
                }
            });
        }
        // Both deferred flows — the close above and a parked project switch (New
        // Work / Open Work / "Open here" / the import toast, where the user chose
        // "Save") — resume here, off the **long-operation** events.
        //
        // They wait on the edit *sequence* their save covers, not on "a save
        // finished": `SaveQueue` runs one `save_work` at a time and coalesces, so
        // the op that finally carries these edits may be a follow-up issued when an
        // already-in-flight save landed. That in-flight save's snapshot can predate
        // our flush, and resuming on it would wipe the store while the last sentence
        // typed was still unwritten.
        {
            let editors = editors.clone();
            let pending = self.pending_exit.clone();
            let exit_seq = self.exit_seq.clone();
            let scheduler = backup_scheduler.clone();
            let switch = project_switch.clone();
            let workspace_layout = workspace_layout.clone();
            ctx.subscribe_event_with_ctx(
                Origin::LongOperation(LongOperationEvent::Completed),
                move |e: &Event, c| {
                    // Ours? (A backup's, an import's or a Save As's completion is
                    // their own view-model's business.) This also issues the
                    // follow-up save when edits arrived while that one was running.
                    let Some(landed) = editors.on_save_completed(e) else {
                        return;
                    };
                    // The store now matches disk — keep the persisted desk fresh and
                    // ordinal-aligned with the file, so a later hard exit (crash /
                    // kill, with no graceful close to capture at) still restores. A
                    // no-op if a follow-up save is due (still dirty): `capture` self-
                    // gates on `is_unsaved`, so only the final clean save persists.
                    if let Some(layout) = &workspace_layout {
                        layout.capture();
                    }
                    // That follow-up could not be issued: nothing further is coming
                    // for anything still parked beyond what just landed, so drop it
                    // rather than let it wait forever.
                    if landed.follow_up_failed {
                        abandon_deferred(c, &pending, &exit_seq, &switch, None);
                        return;
                    }
                    let saved = landed.saved_seq;
                    let pe = pending.get();
                    // A close outranks a switch: the project is leaving this window
                    // entirely, so a switch parked behind a save is moot either way.
                    // Note this returns even when the close is *not yet* covered —
                    // otherwise a switch parked on an earlier sequence would fire and
                    // replace the project out from under a close that is still
                    // waiting for its own write.
                    if pe != PendingExit::None {
                        if exit_seq.get().is_some_and(|s| saved >= s) {
                            pending.set(PendingExit::None);
                            exit_seq.set(None);
                            switch.cancel();
                            // Saved and consistent — now take the on-close backup (if
                            // configured) and then perform the deferred close. When
                            // no on-close backup applies, `on_close_flow` closes at
                            // once.
                            scheduler.on_close_flow(c, pe);
                        }
                        return;
                    }
                    switch.on_saved(c, saved);
                },
            );
        }
        // **A failed save is always a toast.** The write is asynchronous, so the only
        // other way the user could learn of it is the Save affordance staying live —
        // which is invisible under autosave, where Save is hidden entirely. It used
        // to be reported nowhere at all.
        //
        // Exactly one toast, and it says what was lost *besides* the write: if a
        // close or a project switch was parked behind that save, it is dropped and
        // the message says so (the deferred command silently never happening is the
        // more confusing half). The project itself is untouched — still open, still
        // dirty — so nothing is lost by staying put. Only *our* save's failure
        // counts here; a failing backup / import / Save As is reported by its own
        // view-model.
        {
            let editors = editors.clone();
            let pending = self.pending_exit.clone();
            let exit_seq = self.exit_seq.clone();
            let switch = project_switch.clone();
            ctx.subscribe_event_with_ctx(
                Origin::LongOperation(LongOperationEvent::Failed),
                move |e: &Event, c| {
                    let Some(error) = editors.on_save_failed(e) else {
                        return;
                    };
                    abandon_deferred(c, &pending, &exit_seq, &switch, Some(&error));
                },
            );
        }
        // `work.close` — the Close Work menu command (Ctrl+W), and the target of the
        // `welcome.show` alias. Every terminal path ends at
        // [`close_work_and_return_to_launcher`] — there is no more "close in place,
        // keep this (now-empty) window" outcome.
        //
        // Delegates to [`guard_unsaved_exit`] — the same guard `app.quit`'s action
        // and the project window's `on_close_requested` (`windows.rs`) call — rather
        // than re-deriving the branch order: what happens to your unsaved chapter
        // must not depend on which command is about to discard it. Only the
        // *outcome* differs — this one leaves for the Launcher instead of quitting.
        {
            let app_ctx2 = self.app_ctx.clone();
            let unsaved = self.unsaved.clone();
            let autosave = settings.autosave();
            let pending = self.pending_exit.clone();
            let scheduler = backup_scheduler.clone();
            let backup_mode = self.backup_mode.clone();
            ctx.register_action_global(Action::new("work.close").on_invoke(move |_i, ctx| {
                guard_unsaved_exit(
                    ctx,
                    &app_ctx2,
                    unsaved.get(),
                    backup_mode.get(),
                    autosave.get(),
                    &pending,
                    &scheduler,
                    PendingExit::ReturnToLauncher,
                );
            }));
        }
        // `backup.now` — the manual "Back up now" command. Runs a forced backup
        // to the configured destinations; `backup_now` itself flushes in-widget
        // edits into the store first (T1-2 — the flush hook installed above, at
        // the top of every trigger, not just this one). Registered globally (the
        // title-bar menu is an overlay).
        {
            let scheduler = backup_scheduler.clone();
            ctx.register_action_global(Action::new("backup.now").on_invoke(move |_i, ctx| {
                scheduler.backup_now(ctx);
            }));
        }
        // `backups.show` — open the browsable list of this project's backup files.
        {
            let single_work = single_work.clone();
            let single_work_info = single_work_info.clone();
            let backup_settings = backup_settings.clone();
            ctx.register_action_global(Action::new("backups.show").on_invoke(move |_i, c| {
                let uid = single_work.unique_id().get();
                let Some(path) = single_work_info.file_name().get() else {
                    return;
                };
                let dirs = backup_settings.effective_for(&uid).destinations;
                c.present_modal(
                    ModalRequest::deferred(move |t| {
                        t.add(crate::backups_list_panel::BackupsListPanel::new(
                            uid.clone(),
                            path.clone(),
                            dirs.clone(),
                        ))
                    })
                    .presentation(ModalPresentation::InTree)
                    .title(tr!(backups_title()))
                    .size(700, 540)
                    .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
                );
            }));
        }
        let active_item = editors.active_item();
        let split_active = editors.split_active();

        // ── Center: split editor — two panes in a Splitter ───────────────────
        // Each pane is a zoned `DropTarget` wrapping a `TabWidget`, so a binder
        // row dragged from the outline opens on the pane it's dropped over. The
        // primary pane's `Trailing` (right-edge) zone opens to the side; it
        // deactivates once split (`enabled(split_active.not())`). Closing a tab
        // saves it first (`on_close` is a pre-close intercept). Tabs migrate
        // between panes (`accept_external_tabs` + `on_tab_received` dedup).
        let split_button = {
            let editors = editors.clone();
            IconButton::new(crate::editor_icons::split())
                .tooltip(tr!(split_editor()))
                .icon_role(split_active.map(|on| {
                    if *on {
                        TextRole::Accent
                    } else {
                        TextRole::Primary
                    }
                }))
                .on_activate_fn(move |_ctx| editors.toggle_split())
        };
        let close_split_button = {
            let editors = editors.clone();
            IconButton::new(crate::editor_icons::close_split())
                .tooltip(tr!(close_split_view()))
                .on_activate_fn(move |_ctx| editors.close_split())
        };

        // Follow keyboard focus, not just tab selection: when focus enters a
        // pane's content (e.g. clicking into its editor), mark that pane focused so
        // the Inspector + open-item marker track the pane you're actually working
        // in. `focus_within` is set by the framework when a descendant has focus.
        let primary_focus = Signal::new(false);
        let secondary_focus = Signal::new(false);
        for (sig, side) in [
            (&primary_focus, Side::Primary),
            (&secondary_focus, Side::Secondary),
        ] {
            let editors = editors.clone();
            ctx.effect(sig, move |&focused| {
                if focused {
                    editors.set_focused(side);
                }
            });
        }

        let primary_pane = {
            let e = editors.clone();
            DropTarget::new()
                .variant(DropTargetVariant::Prominent)
                .zone_size_factor(0.3)
                .accept_when(|p| {
                    p.get_typed::<RowDragData<TreeNode>>()
                        .is_some_and(|d| d.is_export())
                })
                .region(DropRegion::Center, |z| {
                    z.hint(TextWidget::new(tr!(drop_open_here())))
                })
                .region(DropRegion::Trailing, |z| {
                    z.hint(TextWidget::new(tr!(drop_open_to_side())))
                        .enabled(split_active.not())
                })
                .on_region_drop(move |region, payload, _pos, _ctx| {
                    drain_dropped(payload, |item_id, title| match region {
                        DropRegion::Trailing => e.open_to_side(item_id, title),
                        _ => e.open_in(Side::Primary, item_id, title),
                    })
                })
                .child(build_pane_tabs(&editors, Side::Primary, split_button))
                .focus_within(primary_focus.clone())
        };

        let secondary_pane = {
            let e = editors.clone();
            DropTarget::new()
                .variant(DropTargetVariant::Prominent)
                .accept_when(|p| {
                    p.get_typed::<RowDragData<TreeNode>>()
                        .is_some_and(|d| d.is_export())
                })
                .region(DropRegion::Center, |z| {
                    z.hint(TextWidget::new(tr!(drop_open_here())))
                })
                .on_region_drop(move |_region, payload, _pos, _ctx| {
                    drain_dropped(payload, |item_id, title| {
                        e.open_in(Side::Secondary, item_id, title)
                    })
                })
                .child(build_pane_tabs(
                    &editors,
                    Side::Secondary,
                    close_split_button,
                ))
                .focus_within(secondary_focus.clone())
        };

        let center = Splitter::new(editors.splitter())
            .pane(primary_pane)
            .pane(secondary_pane);

        // ── Leading dock: the binder tree, fronted by a VS Code-style activity
        //    bar (icon rail). The OutlineViewModel owns the DockingModel; the
        //    dock content itself lives in `docks::outline`. ───────────────────
        // The trailing side hosts the context Inspector (a rail dock, like the
        // outline), sized + rail-fronted on the shared DockingModel.
        //
        // These are the DEFAULT dock config + arrangement, established **once** (on
        // first build): the shared `DockingModel` persists across widget rebuilds,
        // and the per-work restore below imports each project's saved sizes /
        // selected side-tab on `LoadWork`, so re-running these on every rebuild would
        // stomp a restored (or user-adjusted) layout. The `.dock(...)` registrations
        // on `DockingLayout::new(...)` still run every build — they rebuild the dock
        // *content*, not the arrangement.
        if !self.initial_loaded {
            let docking = outline.docking();
            docking.set_side_size(DockSide::Trailing, 300.0);
            docking.set_side_rail(DockSide::Trailing, 48.0);
            // The bottom search-preview band. The bottom-LEADING corner belongs to
            // the Leading side, so the binder column runs full height and the preview
            // spans only the width beside it. (Default is `Bottom`, i.e. a full-width
            // band under everything.)
            docking.set_side_size(DockSide::Bottom, 180.0);
            docking.set_corner(DockCorner::BottomLeading, DockSide::Leading);
            // An activity bar, not a tab strip: `set_side_rail` switches the side's
            // presentation from tabs to a rail of activity glyphs, and `Compact` keeps
            // them at the standard icon-button size so the band spends its height on
            // prose rather than on chrome.
            docking.set_side_rail(DockSide::Bottom, 36.0);
            docking.set_side_rail_size(DockSide::Bottom, DockRailItemSize::Compact);
        }
        let layout = DockingLayout::new(outline.docking())
            .rail(
                DockRail::new(DockSide::Leading)
                    .background(SurfaceRole::Main)
                    .divider(),
            )
            .rail(
                DockRail::new(DockSide::Trailing)
                    .background(SurfaceRole::Main)
                    .divider(),
            )
            .rail(
                DockRail::new(DockSide::Bottom)
                    .background(SurfaceRole::Main)
                    .divider(),
            )
            .center(center)
            .dock(crate::docks::outline::outline_dock(
                outline.clone(),
                self.app_ctx.clone(),
                on_open.clone(),
                active_item.clone(),
            ))
            .dock(crate::docks::inspector::inspector_dock(
                self.app_ctx.clone(),
                outline.clone(),
                active_item,
                self.inspector_dock,
            ))
            .dock(crate::docks::search::search_dock(
                search.clone(),
                self.search_dock,
            ))
            .dock(crate::docks::search_preview::search_preview_dock(
                search.clone(),
                self.preview_dock,
            ))
            .dock(crate::docks::trash::trash_dock(
                trash.clone(),
                self.trash_dock,
                on_open,
            ));
        // First-build-only default arrangement (see the config block above on why
        // it must not re-run on rebuilds).
        if !self.initial_loaded {
            // The leading side hosts TWO activity docks (binder + search) as separate
            // switchable rail tabs — VS Code style: the rail shows both glyphs, and
            // selecting one shows only its panel. `.new_tab()` is what makes them
            // distinct tabs; the default `side()` placement *stacks* (a vertical
            // split showing both at once, which starves the binder). The binder is
            // revealed last so it is the selected leading panel on launch.
            let docking = outline.docking();
            docking.open_dock(
                outline.dock_id(),
                DockOpenLocation::side(DockSide::Leading).new_tab(),
            );
            docking.open_dock(
                self.search_dock,
                DockOpenLocation::side(DockSide::Leading).new_tab(),
            );
            docking.open_dock(
                self.trash_dock,
                DockOpenLocation::side(DockSide::Leading).new_tab(),
            );
            docking.reveal_dock(outline.dock_id());
            // Mount the inspector on the trailing side (otherwise the side shows the
            // empty "drop a panel here" placeholder).
            docking.open_dock(
                self.inspector_dock,
                DockOpenLocation::side(DockSide::Trailing),
            );
            // Mount the bottom preview band, then hide it: it is the transient
            // search-preview band, always hidden at start (a result click reveals it
            // thereafter). `open_dock` makes its side visible as a side effect, so the
            // hide must follow the mount — and it is *immediate* to avoid an
            // opening-then-closing flash on launch.
            docking.open_dock(self.preview_dock, DockOpenLocation::side(DockSide::Bottom));
            docking.set_side_visible_immediate(DockSide::Bottom, false);
            // Snapshot this pristine arrangement as the reset target for a project
            // that has no saved layout (so an in-place switch to an unconfigured
            // project doesn't inherit the previous one's docks).
            if let Some(layout_vm) = ctx
                .app_state::<crate::view_models::WorkspaceLayoutViewModel>()
                .cloned()
            {
                layout_vm.set_default_docks(docking.export_state());
            }
        }

        // ── Status bar (thin) with the notification bell ─────────────────────
        let archive = ctx
            .app_state::<Rc<NotificationArchiveModel>>()
            .cloned()
            .expect("install_toast_default registers the notification archive");
        // Status-bar dock toggles: hide/show the leading (binder) and trailing
        // (inspector) sides — like Bastyde's `docking` example.
        let dock_lead = outline.docking();
        let dock_trail = outline.docking();
        // The save indicator sits right after the binder toggle: the quiet, always-
        // there answer to "is my last paragraph on disk?" — the one thing autosave
        // mode had no way to tell you (it hides Save + Ctrl+S). Failures are toasts;
        // this is only the steady state.
        let save_indicator = crate::save_indicator::SaveIndicator::new(
            editors.clone(),
            self.unsaved.clone(),
            settings.autosave(),
            self.backup_mode.clone(),
            // A work is open iff its `WorkInfo` shape is known (same test the File
            // menu uses to collapse its project-only items).
            single_work_info.shape().map(|s| s.is_some()),
            self.save_spinner.clone(),
            self.save_spinner_visible.clone(),
        );
        // The focused item's live word count sits right after the save glyph — the
        // quiet "how many words in this scene" a writer glances at. Counts the open
        // document's live text (so it tracks typing), off `StatsModel` over the shared
        // `OpenDocsStore`.
        let stats = crate::models::StatsModel::new(
            ctx.app_state::<crate::models::OpenDocsStore>()
                .cloned()
                .expect("OpenDocsStore registered in main"),
            editors.active_item(),
            settings.counting_method(),
        );
        let word_count_indicator = crate::word_count_indicator::WordCountIndicator::new(
            stats.clone(),
            single_work_info.shape().map(|s| s.is_some()),
            settings.show_characters(),
        );
        // The writing session: a play/pause sprint timer + word tracker (ephemeral —
        // only its targets persist). Sits on the right of the status bar.
        let session_vm =
            crate::view_models::WritingSessionViewModel::new(stats.clone(), ctx.settings());
        let session_item = crate::session_status_item::SessionStatusItem::new(
            session_vm.clone(),
            single_work_info.shape().map(|s| s.is_some()),
        );
        // `session.toggle` — scriptable start/pause (palette / automation); the play
        // button is the primary control. Global so it fires regardless of focus.
        {
            let vm = session_vm.clone();
            ctx.register_action_global(
                Action::new("session.toggle").on_invoke(move |_i, _c| vm.toggle()),
            );
        }
        let status = StatusBar::new().background(SurfaceRole::Main).child(
            HStack::new()
                .spacing(8.0)
                // Leading (binder) toggle on the left; trailing (inspector) toggle
                // pushed to the right next to the notification bell.
                .child(
                    IconButton::new(crate::activity_icons::sidebar_icon())
                        .size(IconButtonSize::Compact)
                        .tooltip(tr!(statusbar_toggle_outline()))
                        .on_activate_fn(move |_| dock_lead.toggle_side_visible(DockSide::Leading)),
                )
                .child(save_indicator)
                .child(word_count_indicator)
                .child(Spacer::new())
                .child(session_item)
                .child(
                    IconButton::new(crate::activity_icons::inspector_icon())
                        .size(IconButtonSize::Compact)
                        .tooltip(tr!(statusbar_toggle_inspector()))
                        .on_activate_fn(move |_| {
                            dock_trail.toggle_side_visible(DockSide::Trailing)
                        }),
                )
                .child(NotificationCenterButton::new(archive).size(IconButtonSize::Compact)),
        );

        // The permanent backup banner sits above everything while a backup file is
        // open (zero height otherwise).
        let backup_banner = crate::backup_banner::BackupBanner::new(
            self.backup_context.clone(),
            restore_vm.clone(),
            ctx.app_state::<SaveAsViewModel>()
                .cloned()
                .expect("SaveAsViewModel registered in main"),
            single_work.clone(),
        );

        let root = ctx.add(
            VStack::new()
                .spacing(0.0)
                .child(backup_banner)
                .child(Divider::new())
                .child(Expand::new().child(layout))
                .child(status),
        );
        self.root_child = Some(root);

        // Perform this project window's one backend mutation (load the argv
        // path / a Launcher-picked recent, or create a brand-new work)
        // exactly once — now that the `LoadWork`/`NewWork` subscriptions above
        // are live, so the handler runs the full seed flow (ids, tree,
        // singles). There is no "show the Welcome modal" branch any more:
        // whether this process opens a Launcher or a project window at all is
        // decided in `main` before any window (hence any `App`) exists — see
        // `windows.rs` and the launcher-window model in `main.rs`'s module docs.
        if !self.initial_loaded {
            self.initial_loaded = true;
            match self.initial_action.take() {
                Some(PendingAction::Load(path)) => {
                    if let Err(e) = work_management_commands::load_work(
                        &self.app_ctx,
                        &LoadWorkDto {
                            file_name: path.clone(),
                        },
                    ) {
                        eprintln!("skribisto: could not open '{path}': {e}");
                    }
                }
                Some(PendingAction::New(dto)) => {
                    let target = dto.file_name.clone();
                    if let Err(e) = work_management_commands::new_work(&self.app_ctx, &dto) {
                        eprintln!("skribisto: could not create '{target}': {e}");
                    }
                }
                None => {}
            }
        }

        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// The disk write a deferred flow was waiting on will never land — it failed, or a
/// follow-up save could not even be started. Drop whatever was parked on it and say
/// so, rather than leave the user with a command that silently never happens.
///
/// **Exactly one toast**, naming what was lost *besides* the write, because the
/// deferred command not happening is the more confusing half. A close outranks a
/// switch (it is where the project was headed); with neither parked, this is a plain
/// autosave / Ctrl+S and only the write itself is reported. `error` is `None` when
/// the operation never started, so there is no message from the backend to quote.
///
/// The project is untouched — still open, still dirty — so nothing is lost by
/// staying put; the edits are exactly where the user left them.
fn abandon_deferred(
    c: &mut EventContext,
    pending: &Signal<PendingExit>,
    exit_seq: &Rc<std::cell::Cell<Option<u64>>>,
    switch: &ProjectSwitchViewModel,
    error: Option<&str>,
) {
    if pending.get() != PendingExit::None {
        pending.set(PendingExit::None);
        exit_seq.set(None);
        switch.cancel();
        c.show_toast(Toast::error(match error {
            Some(e) => tr!(close_save_failed(error = e.to_string())),
            None => tr!(close_save_not_started()),
        }));
        return;
    }
    if switch.on_save_failed(c, error) {
        return; // it toasted "…so it wasn't replaced"
    }
    // Nothing was waiting: report just the failed write.
    if let Some(e) = error {
        c.show_toast(Toast::error(tr!(save_error(error = e.to_string()))));
    } else {
        c.show_toast(Toast::error(tr!(save_not_started())));
    }
}

/// Present the native picker for an existing `.skrib` and load it. Backs the
/// global `work.open` command (File ▸ Open Work… and Ctrl+O) — the most-used
/// file path in the app, so the backup sniff below (a blocking `File::open` +
/// zip parse with no timeout — see `crate::backup::is_backup_path`) must never
/// run on the UI thread: a recent entry on a disconnected network/FUSE mount
/// would otherwise hang the whole app on an ordinary click (T2-3).
///
/// Loading replaces this window's project **in place**, so the load itself goes
/// through the unsaved-changes guard (`ProjectSwitchViewModel`) rather than
/// straight to `load_work` — which is what used to throw away the open project's
/// unsaved edits without a word. The guard runs *after* the pick and *after* the
/// backup sniff, so neither cancelling the picker nor choosing a backup (which
/// opens in its own process, leaving this project alone) prompts about anything.
fn open_work_flow(switch: ProjectSwitchViewModel, ctx: &mut EventContext) {
    let req = FileDialogRequest::pick_file()
        .title("Open Skribisto work")
        .add_filter("Skribisto work", &["skrib"]);
    let _ = ctx.pick_file(req, move |res, ectx| {
        if let FileDialogResult::File(Some(path)) = res {
            let file = path.to_string_lossy().into_owned();
            let switch = switch.clone();
            let file_for_check = file.clone();
            ectx.spawn_local_with(
                async move {
                    spawn_blocking(move || crate::backup::is_backup_path(&file_for_check))
                        .await
                        .unwrap_or(false)
                },
                move |is_backup, ectx2| {
                    // A backup always opens in its own instance (never replacing
                    // the project in this window) — see the backup-mode invariant.
                    // Nothing here is destroyed, so there is nothing to guard.
                    if is_backup {
                        ectx2.request_activation_token_self(Box::new(move |tok| {
                            crate::project_switcher_button::spawn_new_process(&file, tok);
                        }));
                        return;
                    }
                    switch.request(ectx2, PendingSwitch::OpenWork(file.clone()));
                },
            )
            .detach();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The four states the Save affordances gate on. Nothing to save, or a
    /// read-only backup window → disabled.
    #[test]
    fn can_save_only_when_dirty_and_not_in_backup_mode() {
        for (dirty, backup, want) in [
            (false, false, false), // clean project — nothing to write
            (true, false, true),   // the only case that saves
            (false, true, false),  // backup window, clean
            (true, true, false),   // backup window with edits: Save As / Restore, not Save
        ] {
            let unsaved = Signal::new(dirty);
            let backup_mode = Signal::new(backup);
            assert_eq!(
                can_save(&unsaved, &backup_mode).get(),
                want,
                "dirty={dirty} backup_mode={backup}"
            );
        }
    }

    /// The signal is *derived*, not sampled: flipping either input after the fact
    /// must move it (otherwise the menu item would freeze at its build-time value).
    #[test]
    fn can_save_tracks_later_input_changes() {
        let unsaved = Signal::new(false);
        let backup_mode = Signal::new(false);
        let can = can_save(&unsaved, &backup_mode);
        assert!(!can.get());

        unsaved.set(true); // an edit lands
        assert!(can.get());

        backup_mode.set(true); // …in a backup window: still nothing Save can do
        assert!(!can.get());

        backup_mode.set(false); // Save As turned it back into a normal project
        assert!(can.get());

        unsaved.set(false); // saved to disk
        assert!(!can.get());
    }

    /// The shortcut and the action must *both* follow the signal — the keystroke
    /// and the intent are two independent entry points into `save_to_disk`.
    #[test]
    fn save_shortcut_and_action_follow_can_save() {
        let unsaved = Signal::new(false);
        let backup_mode = Signal::new(false);
        let can = can_save(&unsaved, &backup_mode);

        let shortcut = Shortcut::new("editor.save")
            .name("Save")
            .primary(KeyStroke::ctrl(Key::S))
            .enabled_when(can.clone())
            .build();
        let action = Action::new("editor.save")
            .enabled_when(can)
            .on_invoke(|_i, _c| {});

        assert!(!shortcut.is_enabled(), "Ctrl+S inert on a clean project");
        assert!(!action.is_enabled(), "editor.save inert on a clean project");

        unsaved.set(true);
        assert!(shortcut.is_enabled());
        assert!(action.is_enabled());

        backup_mode.set(true);
        assert!(!shortcut.is_enabled(), "Ctrl+S inert in backup mode");
        assert!(!action.is_enabled(), "editor.save inert in backup mode");
    }
}
