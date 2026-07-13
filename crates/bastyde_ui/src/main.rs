//! Skribisto desktop UI (Bastyde). Wires the Qleany backend to a Bastyde shell.
//!
//! ## The launcher-window model
//!
//! Skribisto is one process per project, so a project window must only ever
//! be created once its project is already known — otherwise the window's
//! persistence id (see [`windows::window_id_for`]) has to be fixed before any
//! project exists, and per-project geometry becomes impossible to key
//! correctly (the bug this model replaces).
//!
//! - **Bare launch** (no `.skrib` on argv), with the "show at startup" setting
//!   on, or with it off but no reachable recent project: opens a **Launcher**
//!   window (the Welcome UI as a real window — see [`windows::launcher_window_config`]).
//! - **Bare launch with the setting off and a reachable recent project**:
//!   skips the Launcher and opens that project directly.
//! - **Launch with a path on argv** (file manager, CLI, `spawn_new_process`):
//!   always skips the Launcher and opens that project directly.
//! - Picking / creating / importing a project from the Launcher opens a
//!   **project window**, then closes the Launcher.
//! - Closing a project (its window's own close, Ctrl+Q, Ctrl+W / File ▸ Close
//!   Work, or the brand icon / File ▸ Welcome…) opens a fresh Launcher window,
//!   then closes the project window — see
//!   [`app::close_work_and_return_to_launcher`]. The process stays alive; it
//!   only quits once the Launcher itself is closed.
//!
//! **Critical ordering rule**: the process quits when its last window closes,
//! so every transition above always opens the new window *before* closing the
//! old one. Getting this backwards quits the app.

mod activity_icons;
mod app;
mod app_ids;
mod backup;
mod backup_banner;
mod backup_choice_panel;
mod backups_list_panel;
mod binder_icons;
mod binder_placement;
mod binder_switcher_button;
mod create_labels;
mod docks;
mod editor_icons;
mod import_plume_panel;
mod intents;
mod ipc;
mod models;
mod new_work_panel;
mod open_registry;
mod project_switcher_button;
mod settings_backup;
mod settings_panel;
mod singles;
mod tabs;
mod view_models;
mod welcome_panel;
mod windows;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use bastyde::core::event_source::{EventSource, SubscriptionHandle};

use bastyde::core::app_event::AppEvent;
use bastyde::prelude::*; // also brings the file-dialog ext + FileDialogRequest/Result
use bastyde::settings::{AppPaths, SettingsStore};
use bastyde::widgets::framework_locales;

use frontend::AppContext;
use frontend::EventHubClient;
use frontend::commands::{handling_app_lifecycle_commands, work_info_commands};
use frontend::common::event::{Event, Origin};

use app::PendingExit;
use app_ids::AppIds;
use models::{BackupSettingsService, OpenDocsStore};
use singles::{SingleWork, SingleWorkInfo};
use view_models::{
    BackupSchedulerViewModel, BackupSettingsViewModel, ImportPlumeViewModel, OutlineViewModel,
    ProjectSwitchViewModel, RestoreViewModel, SaveAsViewModel,
};

/// The currently-open project's path (from `WorkInfo`), if any.
fn current_project_path(ctx: &AppContext) -> Option<String> {
    work_info_commands::get_all_work_info(ctx)
        .ok()?
        .into_iter()
        .next()?
        .file_name
}

/// The open work's base name (no extension), or `"work"` — pre-fills the
/// "Save as" dialog's file name. A folder work's entry is `…/project.skrib`
/// (the on-disk manifest name), so its directory name is used.
fn project_stem(ctx: &AppContext) -> String {
    use std::path::Path;
    current_project_path(ctx)
        .as_deref()
        .map(|cur| {
            let p = Path::new(cur);
            let base = if p.file_name().and_then(|n| n.to_str()) == Some("project.skrib") {
                p.parent().unwrap_or(p)
            } else {
                p
            };
            base.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("work")
                .to_string()
        })
        .unwrap_or_else(|| "work".to_string())
}

/// Sanitize a `Work` title into a folder name valid on Linux, Windows and macOS.
///
/// Replaces every character forbidden on *any* of the three (`/ \ : * ? " < > |`
/// and NUL) with `_`, strips leading/trailing dots and spaces (Windows rejects
/// them), rejects the Windows reserved device names, caps the length well under
/// the 255-byte component limit, and falls back to `"work"` when nothing
/// usable remains.
fn sanitize_folder_name(raw: &str) -> String {
    /// Reserved device names on Windows (case-insensitive, with or without an
    /// extension); a folder named any of these is unusable there.
    const WINDOWS_RESERVED: &[&str] = &[
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
    let trim = |s: &str| {
        s.trim_matches(|c: char| c == '.' || c == ' ' || c.is_whitespace())
            .to_string()
    };
    let mut name: String = raw
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if (c as u32) < 0x20 => '_', // control chars incl. NUL
            c => c,
        })
        .collect();
    name = trim(&name);
    // Cap to 200 chars (leaves headroom under the 255-byte NTFS/ext4 limit), then
    // re-trim in case truncation exposed a trailing dot/space.
    name = trim(&name.chars().take(200).collect::<String>());
    // A reserved stem (the part before the first `.`) makes the whole name invalid.
    let stem = name.split('.').next().unwrap_or("").to_ascii_uppercase();
    if name.is_empty() || WINDOWS_RESERVED.contains(&stem.as_str()) {
        return "work".to_string();
    }
    name
}

/// Persisted-setting keys (also read at startup in `main`).
pub const DARK_KEY: &str = "ui.dark";
pub const LOCALE_KEY: &str = "ui.locale";
/// Max width (px) of the centered main-text writing column.
pub const EDITOR_WIDTH_KEY: &str = "editor.column_width";
pub const EDITOR_WIDTH_DEFAULT: f32 = 700.0;
/// When on, autosave to disk (and hide the manual Save / Ctrl+S affordances).
pub const AUTOSAVE_KEY: &str = "editor.autosave";
/// When on (default) and no work was passed on the command line, a bare
/// launch opens the Launcher window (the Welcome UI). When off, a bare launch
/// instead opens the most recent *reachable* project directly — falling back
/// to the Launcher only if there is none (JetBrains' "reopen last project on
/// startup"). Toggled in Settings ▸ Appearance & Behaviour (the Launcher
/// itself has no inline copy of this — see `welcome_panel.rs`'s module docs).
pub const SHOW_WELCOME_KEY: &str = "ui.show_welcome";

// ── Editor typography (Settings ▸ Editor ▸ Scene / Synopsis / Notes) ──────────
// Non-destructive per-editor-type defaults. Font family / line height /
// first-line indent are applied via `RichTextEditor::typography_defaults`
// (a display-time snapshot fill that never mutates the document); size is a
// per-editor `zoom` multiplier (`1.0` = 100 %). These are NOT the char-format
// `set_font_family`/`set_font_size` (which mutate the selection + document).
// `font_family` must resolve via the shared typesetter — an installed system
// font or one registered by `register_editor_fonts` below; the `FontPicker`
// control only ever offers names that will render.

/// Scene / manuscript body editor typography.
pub const SCENE_FONT_FAMILY_KEY: &str = "editor.scene.font_family";
pub const SCENE_FONT_FAMILY_DEFAULT: &str = "Literata";
pub const SCENE_SIZE_KEY: &str = "editor.scene.size";
pub const SCENE_SIZE_DEFAULT: f32 = 1.0;
pub const SCENE_LINE_HEIGHT_KEY: &str = "editor.scene.line_height";
pub const SCENE_LINE_HEIGHT_DEFAULT: f32 = 1.5;
pub const SCENE_FIRST_LINE_INDENT_KEY: &str = "editor.scene.first_line_indent";
pub const SCENE_FIRST_LINE_INDENT_DEFAULT: f32 = 28.0;
pub const SCENE_PARA_SPACING_BEFORE_KEY: &str = "editor.scene.para_spacing_before";
pub const SCENE_PARA_SPACING_BEFORE_DEFAULT: f32 = 0.0;
pub const SCENE_PARA_SPACING_AFTER_KEY: &str = "editor.scene.para_spacing_after";
pub const SCENE_PARA_SPACING_AFTER_DEFAULT: f32 = 0.0;

/// Synopsis editor typography (the subordinate summary pane).
pub const SYNOPSIS_FONT_FAMILY_KEY: &str = "editor.synopsis.font_family";
pub const SYNOPSIS_FONT_FAMILY_DEFAULT: &str = "Literata";
pub const SYNOPSIS_SIZE_KEY: &str = "editor.synopsis.size";
pub const SYNOPSIS_SIZE_DEFAULT: f32 = 0.95;
pub const SYNOPSIS_LINE_HEIGHT_KEY: &str = "editor.synopsis.line_height";
pub const SYNOPSIS_LINE_HEIGHT_DEFAULT: f32 = 1.35;
pub const SYNOPSIS_FIRST_LINE_INDENT_KEY: &str = "editor.synopsis.first_line_indent";
pub const SYNOPSIS_FIRST_LINE_INDENT_DEFAULT: f32 = 0.0;
pub const SYNOPSIS_PARA_SPACING_BEFORE_KEY: &str = "editor.synopsis.para_spacing_before";
pub const SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT: f32 = 0.0;
pub const SYNOPSIS_PARA_SPACING_AFTER_KEY: &str = "editor.synopsis.para_spacing_after";
pub const SYNOPSIS_PARA_SPACING_AFTER_DEFAULT: f32 = 0.0;

/// Notes editor typography.
pub const NOTES_FONT_FAMILY_KEY: &str = "editor.notes.font_family";
pub const NOTES_FONT_FAMILY_DEFAULT: &str = "Inter";
pub const NOTES_SIZE_KEY: &str = "editor.notes.size";
pub const NOTES_SIZE_DEFAULT: f32 = 1.0;
pub const NOTES_LINE_HEIGHT_KEY: &str = "editor.notes.line_height";
pub const NOTES_LINE_HEIGHT_DEFAULT: f32 = 1.5;
pub const NOTES_FIRST_LINE_INDENT_KEY: &str = "editor.notes.first_line_indent";
pub const NOTES_FIRST_LINE_INDENT_DEFAULT: f32 = 0.0;
pub const NOTES_PARA_SPACING_BEFORE_KEY: &str = "editor.notes.para_spacing_before";
pub const NOTES_PARA_SPACING_BEFORE_DEFAULT: f32 = 0.0;
pub const NOTES_PARA_SPACING_AFTER_KEY: &str = "editor.notes.para_spacing_after";
pub const NOTES_PARA_SPACING_AFTER_DEFAULT: f32 = 0.0;

// ── Editor behaviour (Settings ▸ Editor ▸ Editor Behavior) ───────────────────
/// Show the synopsis pane above the manuscript in the dual-pane writing editor
/// (Skribisto's signature layout). Consumed live by the `shared::prose` body.
pub const SYNOPSIS_PANE_KEY: &str = "editor.synopsis_pane";
pub const SYNOPSIS_PANE_DEFAULT: bool = true;
/// Keep the caret line vertically centred while typing.
pub const TYPEWRITER_KEY: &str = "editor.typewriter_scroll";
pub const TYPEWRITER_DEFAULT: bool = true;
/// Highlight the sentence the caret is in.
pub const HIGHLIGHT_SENTENCE_KEY: &str = "editor.highlight_sentence";
pub const HIGHLIGHT_SENTENCE_DEFAULT: bool = false;

/// The bundled writing typefaces (OFL-1.1), registered additively into the
/// shared typesetter at startup so the defaults render on every machine and the
/// `FontPicker` lists them. Inter (sans, the Notes default) stays the app-wide
/// default face — every serif here is registered non-default. Family names are
/// exactly "Literata", "EB Garamond", "Source Serif 4" (verified via the font
/// name tables), matching the `*_FONT_FAMILY_DEFAULT` values above.
fn register_editor_fonts() -> bastyde::text::VecFontRegistrar {
    use bastyde::text::{FontFaceSpec, VecFontRegistrar};
    use std::sync::Arc;
    let face = |bytes: &'static [u8]| FontFaceSpec {
        data: Arc::new(bytes.to_vec()),
        is_default: false,
        default_size_px: 16.0,
    };
    VecFontRegistrar::new(vec![
        face(include_bytes!("../assets/fonts/Literata-Variable.ttf")),
        face(include_bytes!(
            "../assets/fonts/Literata-Italic-Variable.ttf"
        )),
        face(include_bytes!("../assets/fonts/EBGaramond-Variable.ttf")),
        face(include_bytes!(
            "../assets/fonts/EBGaramond-Italic-Variable.ttf"
        )),
        face(include_bytes!("../assets/fonts/SourceSerif4-Variable.ttf")),
        face(include_bytes!(
            "../assets/fonts/SourceSerif4-Italic-Variable.ttf"
        )),
    ])
}

/// Adapts the Qleany-generated `EventHubClient` to Bastyde's `EventSource`
/// (orphan rule prevents implementing the trait directly on the client).
#[derive(Clone)]
struct EventHubSource {
    client: EventHubClient,
}

impl EventSource for EventHubSource {
    type Origin = Origin;
    type Event = Event;

    fn subscribe(
        &self,
        origin: Self::Origin,
        callback: Arc<dyn Fn(Self::Event) + Send + Sync + 'static>,
    ) -> SubscriptionHandle {
        let token = self.client.subscribe(origin, move |event| callback(event));
        SubscriptionHandle::new(token)
    }
}

fn main() {
    let app_ctx = Rc::new(AppContext::new());

    // Background event-dispatch thread.
    let client = EventHubClient::new(&app_ctx.event_hub);
    client.start(app_ctx.shutdown_rx.clone());

    // Seed the single shared Root + System frame into the (empty) store at
    // startup — before any work is opened — and keep the returned Root id to
    // point `AppIds` at it. `initialize_app` is idempotent: a later load/new
    // reuses this frame instead of creating a second Root/System.
    let init_root_id = match handling_app_lifecycle_commands::initialize_app(&app_ctx) {
        Ok(res) => Some(res.root_id),
        Err(e) => {
            eprintln!("initialize_app failed: {e:#}");
            None
        }
    };

    // Read persisted UI prefs before constructing the app (same AppPaths the
    // builder will use via `.application(...)`).
    let (dark, locale_str, autosave_init, show_welcome_init) = read_prefs();

    let theme = if dark { intui::dark() } else { intui::light() };

    let i18n = I18nConfig::new()
        .source_locale("en-US".parse().unwrap())
        .supported_locales(["en-US".parse().unwrap(), "fr-FR".parse().unwrap()])
        .compile_in(&[
            ("en-US", &[include_str!("../locales/en-US.ftl")]),
            ("fr-FR", &[include_str!("../locales/fr-FR.ftl")]),
        ])
        .user_locale(locale_str.parse().ok())
        .auto_detect_os_locale(false)
        .fallback_locale("en-US".parse().unwrap())
        .framework_locales(framework_locales());

    // The app's id-only global state (root/work/work-info/undo-stack ids). Created
    // here, shared into the outline, the singles, and the title-bar menu, and
    // registered as `app_state` so any widget can reach it.
    let ids = AppIds::new();
    // Point the app at the shared Root seeded by `initialize_app` above, so the
    // root id is known before any work is opened (a load/new refreshes it later).
    if let Some(root_id) = init_root_id {
        ids.root_id.set(Some(root_id));
    }
    // The shared open-document store (Layer A): one live `OpenDoc` per open item,
    // holding the editors' `TextDocument`s + write-back. Registered as `app_state`
    // so the split editor's two panes (and any future view) share one document per
    // item — the documents are usable outside the `TabWidget`s.
    let open_docs = OpenDocsStore::new(app_ctx.clone());
    // Reactive single-entity handles (Layer A). Created here so the title-bar menu
    // can bind the project title (Bug 1) and shape (Bug 2); `App::build` wires
    // their event subscriptions and re-points them on each `LoadWork`.
    let single_work = SingleWork::new(app_ctx.clone());
    let single_work_info = SingleWorkInfo::new(app_ctx.clone());
    // The outline view-model is created here (no settings dependency) so the
    // title-bar menu can bind its reactive checkmark and the whole app can reach
    // it via `ctx.app_state::<OutlineViewModel>()`.
    let outline = OutlineViewModel::new_default(app_ctx.clone(), ids.clone());
    // The Import-Plume view-model is a singleton (form + in-flight job + progress
    // toast). Registered as app-state so `App::build` can route the import's
    // long-operation events to it and the menu action can reach it to open the panel.
    let import_plume = ImportPlumeViewModel::new(app_ctx.clone());
    // Backup-mode state: `backup_mode` is true while a *backup file* is open in
    // this window (Save + auto-backup off; the file is read-only, the content is
    // still editable). `backup_context` carries the open backup's details (drives
    // the permanent banner + restore). Created here so the title-bar menu can read
    // `backup_mode` (to hide Save / Back up now); `App::build` sets them on load.
    let backup_mode = Signal::new(false);
    let backup_context: Signal<Option<backup::BackupContext>> = Signal::new(None);
    // The Save-As view-model records the new path/shape into WorkInfo on the UI
    // thread when a background "Save As" completes (save_as itself is read-only).
    // It also clears backup mode on success — a Save As out of a backup window makes
    // that window the freshly-written (regular) project.
    // Registered as app-state so `App::build` routes the long-operation events to it.
    let save_as_vm = SaveAsViewModel::new(
        app_ctx.clone(),
        ids.clone(),
        single_work.clone(),
        backup_mode.clone(),
        backup_context.clone(),
    );
    // Backup ("Copies de secours") settings — opened eagerly here (before any
    // project loads) so the on-open/on-close/interval hooks and the scheduler see
    // it. Degrades to a throwaway temp file if the config dir is unavailable,
    // exactly as the recents MRU does.
    let backup_service = bastyde::settings::AppPaths::new("eu", "skribisto", "Skribisto")
        .and_then(|paths| {
            BackupSettingsService::open(&paths)
                .map_err(|e| eprintln!("backup settings: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(BackupSettingsService::in_memory_default);
    let backup_settings = BackupSettingsViewModel::new(backup_service);
    // The backup scheduler drives every trigger (manual / on-open / interval /
    // on-close). It reads the open project's uid/path from the singles and the
    // policy from `backup_settings`. Registered as app-state so `App::build` routes
    // the backup long-operation events to it and fires the triggers.
    let backup_scheduler = BackupSchedulerViewModel::new(
        app_ctx.clone(),
        backup_settings.clone(),
        single_work.clone(),
        single_work_info.clone(),
        backup_mode.clone(),
    );
    // Restore-a-backup view-model — overwrites the original project with the open
    // backup's content (reusing `save_as`), then leaves backup mode. Registered as
    // app-state so `App::build` routes its long-operation events + the choice modal
    // / banner reach it.
    let restore_vm = RestoreViewModel::new(
        app_ctx.clone(),
        ids.clone(),
        single_work.clone(),
        backup_mode.clone(),
        backup_context.clone(),
    );
    // The title-bar menu lives outside `App` (no `ctx.settings()` there), so the
    // autosave setting is mirrored into this plain signal by `App::build` and read
    // by the menu to hide the "Save" item. Seeded from the persisted value.
    let autosave_menu = Signal::new(autosave_init);
    // Exit-guard state shared between the window close guard / Close Work menu and
    // `App` (which maintains `unsaved` and performs the deferred close on save).
    let unsaved = Signal::new(false);
    let pending_exit = Signal::new(PendingExit::None);
    // The *switch* guard — the same unsaved-changes prompt for the four commands
    // that replace this window's project in place without going through a close
    // (New Work, Open Work, the switcher's "Open here", the import toast's "Open
    // now"); all four used to destroy unsaved edits silently. Built here because
    // it guards on the same `unsaved`/`backup_mode`/autosave signals as the close
    // guard, and registered as app-state so the two doors outside `App` (the
    // switcher popover, the import toast) reach it. `App::build` installs its
    // save + New-Work-form hooks (both need the editors / the widget tree).
    let project_switch = ProjectSwitchViewModel::new(
        app_ctx.clone(),
        unsaved.clone(),
        backup_mode.clone(),
        autosave_menu.clone(),
    );
    // Optional `.skrib` path to open on launch (`skribisto <path>`); `App` opens it
    // once on first build.
    let initial_project = std::env::args().nth(1).filter(|s| !s.trim().is_empty());

    // If the requested project is already open in another live instance, raise
    // that instance (forwarding our launch activation token for a real Wayland
    // raise) and exit instead of opening a duplicate window.
    if let Some(path) = initial_project.as_ref() {
        let canon = std::fs::canonicalize(path)
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| path.clone());
        if let Some(existing) = open_registry::scan().into_iter().find(|e| e.path == canon) {
            let token = std::env::var("XDG_ACTIVATION_TOKEN").ok();
            let _ = ipc::send_raise(existing.pid, token);
            return;
        }
    }

    // Shared handle to the *current* project window's `WindowState`, captured
    // in its root builder — lets IPC "raise" events (handled in
    // `on_app_event`) focus it directly without a WindowManager id lookup,
    // which misses while that window is dispatching its own events. Never
    // pointed at the Launcher (see `windows::launcher_window_config`'s docs):
    // IPC raise is only ever targeted at a process holding an open-registry
    // claim on a specific project path, which a Launcher-only process never
    // has.
    let main_window_state: Rc<RefCell<Option<WindowState>>> = Rc::new(RefCell::new(None));

    // Everything a project window needs to build itself — bundled once here
    // and registered as `app_state` so both this initial-window decision and
    // a later runtime `ctx.open_window(...)` (from the Launcher, or the
    // Close-Work → Launcher path) build an identical window. See
    // `windows.rs`.
    let project_factory = windows::ProjectWindowFactory::new(
        app_ctx.clone(),
        outline.clone(),
        single_work.clone(),
        single_work_info.clone(),
        autosave_menu.clone(),
        save_as_vm.clone(),
        backup_mode.clone(),
        backup_context.clone(),
        unsaved.clone(),
        pending_exit.clone(),
        backup_scheduler.clone(),
        main_window_state.clone(),
    );

    // ── Decide the initial window: launcher-window model ────────────────
    // A project window is only ever created once its project is already
    // known (see the module docs above) — so this is the one place that
    // decides, up front, whether the very first window is the Launcher or a
    // project, based on argv / the persisted "show at startup" setting / the
    // most recent reachable project.
    let initial_window_config = if let Some(path) = initial_project.clone() {
        // Launch with a path on argv (file manager, CLI, `spawn_new_process`):
        // always skip the Launcher.
        project_factory.window_config(app::PendingAction::Load(path))
    } else if show_welcome_init {
        windows::launcher_window_config(app_ctx.clone())
    } else {
        // "Show at startup" is off: open the most recent *reachable* project
        // directly (`RecentWorkListModel` already filters unreachable/backup
        // entries) — falling back to the Launcher only if there is none.
        match crate::models::RecentWorkListModel::new(app_ctx.clone())
            .items()
            .into_iter()
            .next()
        {
            Some(recent) => {
                project_factory.window_config(app::PendingAction::Load(recent.absolute_path))
            }
            None => windows::launcher_window_config(app_ctx.clone()),
        }
    };

    BastydeAppBuilder::new()
        .theme(theme)
        .application("eu", "skribisto", "Skribisto")
        .settings(SettingsBundle::new().with_window_state(true))
        // Ship the writing serifs so the manuscript defaults render everywhere
        // and the typeface picker lists them (additive; Inter stays the default).
        .register_fonts(register_editor_fonts())
        .i18n(i18n)
        .install_inspector_in_debug()
        .install_automation_bridge_in_debug()
        .install_file_dialog()
        .install_toast_default()
        // Main-thread async executor: `spawn_blocking` gets pure-filesystem work
        // (backup sniffing, destination-reachability probes) off the UI thread.
        // These are UI concerns — "is this directory writable, so I can draw a
        // ✓", "is this file a backup, so I know which panel to show" — not
        // business rules: no entity, no invariant, no undo, no event. A Qleany
        // `LongOperation` would be ceremony around a unit of work that touches
        // no unit of work.
        //
        // `install_async_async_std`, not the bare `install_async`: async-std is
        // already linked via the default `file-dialog` (rfd) feature, so the
        // reactor costs nothing. The bare executor has *no* reactor, and would
        // silently never wake a future awaiting a native timer/socket.
        .install_async_async_std()
        .event_source(EventHubSource { client })
        .app_state(ids.clone())
        .app_state(open_docs.clone())
        .app_state(single_work.clone())
        .app_state(single_work_info.clone())
        .app_state(outline.clone())
        .app_state(import_plume.clone())
        .app_state(save_as_vm.clone())
        .app_state(backup_settings.clone())
        .app_state(backup_scheduler.clone())
        .app_state(restore_vm.clone())
        .app_state(project_switch.clone())
        .app_state(project_factory.clone())
        // Bind this instance's IPC listener (multi-process window switching); an
        // incoming raise request focuses the captured main window.
        .on_ready(ipc::spawn_listener)
        .on_app_event({
            let main_window_state = main_window_state.clone();
            move |event| {
                if let AppEvent::External(payload) = event
                    && let Some(req) = payload.downcast_ref::<ipc::RaiseMainWindow>()
                    && let Some(state) = main_window_state.borrow().as_ref()
                {
                    if let Some(token) = req.activation_token.clone() {
                        state.set_activation_token(token);
                    }
                    state.focus();
                }
            }
        })
        .initial_window(initial_window_config)
        .run();

    // Flush the recent-works MRU synchronously so a just-opened project isn't
    // lost inside the debounce window, then tear the shared Root/System frame
    // down and fire `CleanUpBeforeExit` before the event thread is stopped.
    crate::models::RecentWorkListModel::flush_now();
    // Flush any pending backup-settings write (last-success hashes / timestamps /
    // nudge flag / edited policy) so it survives the debounce window on exit.
    backup_settings.flush_now();
    // Drop every open-registry claim this instance holds (process exit — the
    // "release everything" point, unlike `CloseWork`'s single-path release) so
    // its project(s) stop showing as open in other instances' switchers, and
    // unlink this instance's IPC socket so a later `scan()` never has to reap
    // it as stale.
    crate::open_registry::release_all();
    crate::ipc::cleanup_own_socket();
    if let Err(e) = handling_app_lifecycle_commands::clean_up_before_exit(&app_ctx) {
        eprintln!("clean_up_before_exit failed: {e:#}");
    }
    app_ctx.shutdown();
}

/// Best-effort read of persisted theme/locale/autosave/show-welcome; defaults
/// if anything is missing. `show_welcome` is read here (not just via
/// `ctx.settings()` inside `App::build`) because it decides whether a bare
/// launch's *initial window* is the Launcher or a project — a decision made
/// in `main`, before any widget tree (hence any `BuildContext`) exists.
fn read_prefs() -> (bool, String, bool, bool) {
    let Some(paths) = AppPaths::new("eu", "skribisto", "Skribisto") else {
        return (false, "en-US".to_string(), false, true);
    };
    // `config_file` appends `.toml`, and the settings bundle opens its K/V
    // store under the name "general" (-> general.toml). Pass the bare name
    // here too, otherwise this reads `general.toml.toml` and never sees the
    // values the settings panel wrote, so prefs don't restore on restart.
    match SettingsStore::open(paths.config_file("general")) {
        Ok(store) => (
            store.signal(DARK_KEY, false).get(),
            store.signal(LOCALE_KEY, "en-US".to_string()).get(),
            store.signal(AUTOSAVE_KEY, false).get(),
            store.signal(SHOW_WELCOME_KEY, true).get(),
        ),
        Err(_) => (false, "en-US".to_string(), false, true),
    }
}

#[cfg(test)]
mod tests {
    use super::sanitize_folder_name;

    // Per-project window geometry (`windows::window_id_for`) replaces the old
    // shared-slot stopgap — see that module's tests for its id-stability
    // coverage.

    #[test]
    fn keeps_a_clean_title_verbatim() {
        assert_eq!(sanitize_folder_name("My Novel"), "My Novel");
        assert_eq!(sanitize_folder_name("Война и мир"), "Война и мир");
    }

    #[test]
    fn replaces_cross_os_forbidden_chars() {
        // `/ \ : * ? " < > |` and control chars → `_`.
        assert_eq!(
            sanitize_folder_name("a/b\\c:d*e?f\"g<h>i|j"),
            "a_b_c_d_e_f_g_h_i_j"
        );
        assert_eq!(sanitize_folder_name("tab\there"), "tab_here");
        // All-forbidden becomes underscores (a valid, if ugly, folder name) —
        // the `project` fallback is only for empty/dots/reserved.
        assert_eq!(sanitize_folder_name("///"), "___");
    }

    #[test]
    fn strips_leading_and_trailing_dots_and_spaces() {
        assert_eq!(sanitize_folder_name("  .hidden.  "), "hidden");
        assert_eq!(sanitize_folder_name("trailing."), "trailing");
    }

    #[test]
    fn rejects_windows_reserved_names() {
        // Case-insensitive, with or without an extension.
        assert_eq!(sanitize_folder_name("CON"), "work");
        assert_eq!(sanitize_folder_name("nul"), "work");
        assert_eq!(sanitize_folder_name("LPT1.txt"), "work");
        // A reserved word as a substring is fine.
        assert_eq!(sanitize_folder_name("Console"), "Console");
    }

    #[test]
    fn falls_back_to_work_when_empty() {
        assert_eq!(sanitize_folder_name(""), "work");
        assert_eq!(sanitize_folder_name("   "), "work");
        assert_eq!(sanitize_folder_name("..."), "work");
    }

    #[test]
    fn caps_length_and_re_trims() {
        let long = "a".repeat(500);
        let out = sanitize_folder_name(&long);
        assert_eq!(out.chars().count(), 200);
    }
}
