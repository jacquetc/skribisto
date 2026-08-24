// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Skribisto desktop UI (Teksilo). Wires the Qleany backend to a Teksilo shell.
//!
//! ## Single instance
//!
//! The **first** live copy of Skribisto wins an election ([`shell::instance`]) and
//! becomes the *primary*. Every later launch is a *remote*: it forwards what it
//! was asked to do over a socket and exits, in milliseconds, without ever
//! building an event hub, a store, a settings writer or a window. The primary
//! answers by opening — or focusing — a window of its own.
//!
//! That is why the election runs at the very **top** of `main`, before
//! `AppContext::new()`. A remote that had already run `initialize_app`, pruned
//! `window_state.toml` and opened settings handles would be doing all of it
//! against files the primary is concurrently using, for a process about to exit.
//!
//! Several processes are still reachable: `--new-instance` asks for one outright
//! (the two-process automation test needs it), and an unreachable or wedged
//! primary degrades to `Standalone`, which is exactly the pre-Phase-4 behaviour.
//!
//! ## The launcher-window model
//!
//! A project window must only ever be created once its project is already known —
//! otherwise the window's persistence id (see [`windows::window_id_for`]) has to
//! be fixed before any project exists, and per-project geometry becomes
//! impossible to key correctly (the bug this model replaces).
//!
//! - **Bare launch** (no `.skrib` on argv), with the "show at startup" setting
//!   on, or with it off but no reachable recent project: opens a **Launcher**
//!   window (the Welcome UI as a real window — see [`windows::launcher_window_config`]).
//! - **Bare launch with the setting off and a reachable recent project**:
//!   skips the Launcher and opens that project directly.
//! - **Launch with a path on argv** (file manager, CLI): always skips the
//!   Launcher and opens that project directly — in the primary's process when
//!   there is one.
//! - Picking / creating / importing a project from the Launcher opens a
//!   **project window**, then closes the Launcher.
//! - Closing a project (its window's own close, Ctrl+W / File ▸ Close Work, or
//!   File ▸ Welcome…) closes every window on that Work and, **only when no
//!   other Work still has an open window**, opens a fresh Launcher — see
//!   [`app::close_work_and_return_to_launcher`]. With another Work open
//!   elsewhere, the Launcher stays closed and a surviving project window is
//!   focused. The process stays alive while any window remains; it only quits
//!   once the last window (Launcher or otherwise) is closed. Ctrl+Q / File ▸
//!   Quit is separate: it terminates the whole process (every Work).
//!
//! **Critical ordering rule**: the process quits when its last window closes,
//! so every transition above always opens the new window *before* closing the
//! old one. Getting this backwards quits the app.

// A deliberate, blunt instrument — and worth knowing exactly what it hides.
//
// Dead code is reported again, as of the triage this crate's own comment
// promised. What silenced it was real: clippy lints each `#[cfg]` arm on its
// own, so a helper the mock models use is dead in the default arm and vice
// versa, and deleting from one arm's point of view breaks the other's build.
// That is now scoped to the three files where it actually happens
// (`models/footnote_numbering.rs`, `models/footnotes_list_model.rs`,
// `tabs/tests.rs`), each carrying a `cfg_attr` allow that says which arm and
// why, instead of one blanket line covering the whole crate.
//
// Everything else was triaged item by item: vestigial code deleted, test-only
// helpers gated behind `cfg(test)`, and the handful that is real API nothing
// reads yet annotated where it stands. The mistake worth knowing about is the
// one this pass made and the mocks build caught — three tree-walking helpers in
// `tabs/tests.rs` look dead from the default arm and are not.

// Intra-doc links to private items, allowed rather than fixed.
//
// The gate documents this crate with `--document-private-items`, so these links
// resolve and render — the lint fires on the *visibility* of the target, not on
// whether the reader can follow it. And the targets are the interesting half:
// `windows::window_id_for`, `open_registry::namespace`, `structure_key`. These
// are notes from one maintainer to the next, and rewriting 133 of them as plain
// backticks would cost the navigation and buy nothing.
#![allow(rustdoc::private_intra_doc_links)]

//! ## Module visibility
//!
//! The modules below are `pub` because this crate **is** the application and the
//! extension seam has to reach into it — a private module is unreachable from a
//! downstream crate however public its contents are. That makes this a large
//! surface, deliberately: the alternative is re-exporting types one at a time as
//! each extension needs them, which turns every extension into a change here.
//!
//! `test_support` is the exception and stays crate-private: it is `#[cfg(test)]`
//! scaffolding, not API.

pub mod active_context;
pub mod analysis;
pub mod app;
pub mod app_ids;
/// `App`'s own `BuildContext`, for an extension that has no widget of its own to
/// hang state on. The `commands_ext` arrangement, for state rather than a verb.
pub mod app_wiring;
pub mod backup;
pub mod backup_paths;
pub mod binder;
pub mod cli;
pub mod commands_ext;
pub mod comments;
pub mod corkboard;
pub mod crash_report;
pub mod date_convert;
pub mod distraction_free;
pub mod docks;
pub mod editors;
pub mod export;
pub mod ext;
pub mod first_run;
pub mod footnotes;
pub mod format;
pub mod go;
pub mod goals;
pub mod help;
pub mod icons;
pub mod identity;
pub mod import_document;
pub mod import_plume;
pub mod intents;
pub mod ipc_serve;
pub mod locales;
pub mod margin_lane;
pub mod media_paths;
pub mod mentions;
pub mod models;
pub mod new_work;
pub mod note_templates;
pub mod overview;
pub mod pace;
pub mod panels;
pub mod project;
pub mod read_signal;
pub mod save;
pub mod search;
pub mod sessions;
pub mod settings;
pub mod settings_ext;
pub mod settings_keys;

// The ~130 persisted-setting key constants moved into `settings_keys`, beside
// the schema table that has to name each of them. Re-exported here because
// every reader in the app spells them `crate::DARK_KEY` — and because a key is
// crate-level vocabulary, whatever file happens to declare it.
pub use settings_keys::*;
pub mod shared;
pub mod shell;
pub mod singles;
pub mod spellcheck;
pub mod startup;
pub mod statusbar;
pub mod stream;
pub mod tabs;
pub mod tags;
pub mod text_replacement;
pub mod timeline;
pub mod toast_scope;
pub mod trash;
pub mod versions;
pub mod widgets;
pub mod workspace_layout;
pub mod writing_session;
// The pane tests that need fixture rows are mocks-gated, but the search preview's
// layout tests build their own `OpenDoc`, so they run on the real backend too —
// and both need an event source. Hence the plain `test` gate.
#[cfg(test)]
mod test_support;
pub mod tooltip_registry;
pub mod version;
pub mod welcome;

use std::rc::Rc;
use std::sync::Arc;

use teksilo::core::event_source::{EventSource, SubscriptionHandle};

use teksilo::prelude::*; // also brings the file-dialog ext + FileDialogRequest/Result

use shell::{ipc, windows};

use frontend::AppContext;
use frontend::EventHubClient;
use frontend::commands::work_info_commands;
use frontend::common::event::{Event, Origin};

use app_ids::AppIds;
use binder::OutlineViewModel;
use models::{TreeExpansionService, WorkspaceLayoutService};
use sessions::WorkSession;
use startup::{Tier1Services, UiConfig};

/// The currently-open project's path (from its `WorkInfo`), if any.
///
/// Resolved through `ids.work_info_id` — the id `WorkSession` carries for
/// exactly this window's project — never `get_all_work_info(ctx)`'s first
/// entry: several Works can be open at once (each project window mints its
/// own `WorkSession`), so "whichever the store returns first" is not this
/// window's project in general.
pub(crate) fn current_project_path(ctx: &AppContext, ids: &AppIds) -> Option<String> {
    let work_info_id = ids.work_info_id.get()?;
    work_info_commands::get_work_info(ctx, &work_info_id)
        .ok()??
        .file_name
}

/// The open work's base name (no extension), or `"work"` — pre-fills the
/// "Save as" dialog's file name. A folder work's entry is `…/project.skrib`
/// (the on-disk manifest name), so its directory name is used.
pub(crate) fn project_stem(ctx: &AppContext, ids: &AppIds) -> String {
    use std::path::Path;
    current_project_path(ctx, ids)
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

/// The bundled writing typefaces (OFL-1.1), registered additively into the
/// shared typesetter at startup so the defaults render on every machine and the
/// `FontPicker` lists them. Inter (sans, the Notes default) stays the app-wide
/// default face — every serif here is registered non-default. Family names are
/// exactly "Literata", "EB Garamond", "Source Serif 4" (verified via the font
/// name tables), matching the `*_FONT_FAMILY_DEFAULT` values above.
fn register_editor_fonts() -> teksilo::text::VecFontRegistrar {
    use std::sync::Arc;
    use teksilo::text::{FontFaceSpec, VecFontRegistrar};
    let face = |bytes: &'static [u8]| FontFaceSpec {
        data: Arc::new(bytes.to_vec()),
        is_default: false,
        // The writing-serif design size: this is what a Scene / Synopsis editor's
        // `size` = 100 % resolves to (the `size` setting is a font-size scale, so
        // the absolute px is anchored here). 18 px is the comfortable long-form
        // drafting size; a Synopsis pane scales down from it via its 0.85 default.
        default_size_px: 18.0,
    };
    // Bytes come from the shared `skribisto_fonts` crate — the same blobs the PDF exporter
    // feeds to Typst, so the editor and an exported PDF render in the identical face.
    VecFontRegistrar::new(skribisto_fonts::all_faces().into_iter().map(face).collect())
}

/// Adapts the Qleany-generated `EventHubClient` to Teksilo's `EventSource`
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

/// Build and run the Skribisto application.
///
/// This is the whole binary: `src/bin/skribisto.rs` is a one-line `fn main` that
/// calls it. The split is not cosmetic — a `[[bin]]`-only crate cannot be
/// depended on, so nothing outside this workspace could name `ContentTab`,
/// `AppIds`, a view-model, or any other type here. Everything the extension seam
/// needs to reach lives behind this boundary.
pub fn run() {
    let (initial_project, is_primary) = match shell::instance::bootstrap() {
        shell::instance::Bootstrap::Exit => return,
        shell::instance::Bootstrap::Continue {
            initial_project,
            is_primary,
        } => (initial_project, is_primary),
    };

    let app_ctx = Rc::new(AppContext::new());

    // Background event-dispatch thread.
    let client = EventHubClient::new(&app_ctx.event_hub);
    client.start(app_ctx.shutdown_rx.clone());

    // The optional `.skrib` path from argv (`skribisto <path>`) was read above,
    // which also puts it **before** the window-state prune below — that ordering
    // is load-bearing, not incidental. The prune forgets every `work-*` row whose
    // project it cannot account for, and a path handed to us on argv is a project
    // we are about to open *right now* — but it need not be in the recents MRU
    // (it can have aged out of the 30-entry cap) nor in the open registry
    // (nothing has claimed it yet). Pruning first would therefore delete the
    // saved geometry of the very window we are seconds away from restoring.

    let (first_run_offer, init_root_id) =
        startup::launch_maintenance(&app_ctx, initial_project.as_deref());

    let UiConfig {
        theme,
        i18n,
        autosave_init,
        spellcheck_init,
        show_welcome_init,
    } = startup::build_ui_config();

    let Tier1Services {
        registry,
        spellcheck,
        dictionaries,
        workspace_layout_service,
        tree_expansion_service,
        import_plume,
        folder_memory,
        import_prefs,
        export_styles,
        paratext_presets,
        df_themes,
        backup_settings,
    } = startup::open_tier1_services(&app_ctx, init_root_id);

    // The margin lane's own mark sources, registered through the same door an
    // extension uses and held for the life of the process. Before any window, because
    // a surface reads the registry as a **snapshot** when it is built: a provider
    // registered afterwards would be missing from every lane already on screen and
    // from the settings page listing them, with nothing to show that it had happened.
    let _lane_providers = margin_lane::install_builtin_providers();

    // The title-bar menu lives outside `App` (no `ctx.settings()` there), so the
    // autosave setting is mirrored into this plain signal by `App::build` and read
    // by the menu to hide the "Save" item. Seeded from the persisted value.
    let autosave_menu = Signal::new(autosave_init);
    // Same trick for the master spell-check switch: the title-bar toggle + View menu live
    // outside `App`, so `App::build` mirrors the persisted key into this plain signal.
    // Seeded from the store so a launch with it off never flashes the "on" icon.
    let spellcheck_menu = Signal::new(spellcheck_init);
    // Seeded from the *default*, not from `read_prefs`, unlike `spellcheck_menu` above.
    // That one feeds a title-bar button which paints from the very first frame, so a
    // wrong seed is a visible flicker; this one feeds only the Tools ▸ Comments
    // checkmark, which cannot be looked at before `App::build` has already written the
    // stored value into it in the same frame.
    let comments_menu = Signal::new(COMMENTS_VISIBLE_DEFAULT);
    // Same reasoning as `comments_menu`: seeded from the default, because it
    // feeds only a menu checkmark, and `App::build` writes the stored value into
    // it in the same frame — before the menu can be opened.
    let margin_lane_menu = Signal::new(MARGIN_LANE_ENABLED_DEFAULT);
    // `unsaved` (exit-guard state shared between the window close guard / Close
    // Work menu and `App`) and `pending_exit` (a deferred close/quit awaiting an
    // in-flight save) are **not** constructed here any more (Scope E): both used
    // to be one process-wide `Signal` threaded unchanged into every window —
    // the same shape `backup_mode`/`backup_context` had before their own
    // Phase-3 fix. `unsaved` is genuinely Tier 2 (per-Work — two windows on the
    // SAME Work should agree on its dirty state), so it now lives on
    // `WorkSession` (see its own doc); `pending_exit` is genuinely Tier 3
    // (per-WINDOW — even two windows on the SAME Work must resolve their own
    // close/quit independently), so `ProjectWindowFactory::window_config` mints
    // a fresh one per window instead (see its own doc).
    // The save-tracking state (`dirty_seq`/`saved_seq`/`saving` + the `SaveQueue`)
    // for the open Work now lives on `session.save_state` — the seed this whole
    // `WorkSession` bundle grew from (see its module doc). `App::build` reads it
    // straight off `self.session` rather than via `ctx.app_state::<SaveStateViewModel>()`,
    // so — unlike every other Tier-2 field above — it is deliberately NOT also
    // registered as its own `app_state` entry below: nothing else in the crate
    // looked it up that way.
    // The *switch* guard — the same unsaved-changes prompt for the four commands
    // that replace this window's project in place without going through a close
    // (New Work, Open Work, the switcher's "Open here", the import toast's "Open
    // now"); all four used to destroy unsaved edits silently. It guards on the
    // same `unsaved`/`backup_mode`/autosave signals as the close guard, and is
    // registered as app-state so the two doors outside `App` (the switcher
    // popover, the import toast) reach it. `App::build` installs its save +
    // New-Work-form hooks (both need the editors / the widget tree).
    //
    // Its `backup_mode` handle is now sourced from `initial_state.session` (see
    // below), not a process-wide local — `WorkSession::new` mints a fresh
    // `backup_mode` per Work (Phase 3) and there is no such thing as "the"
    // backup-mode signal before a Work exists. `ProjectSwitchViewModel` itself
    // stays a single, Tier-1 shared instance — a known, disclosed Phase-3
    // boundary (see its own module doc) unrelated to this fix, so it is
    // constructed after `initial_state` is available, alongside the other
    // Tier-2-via-`app_state` registrations that already read off the first
    // window's session.
    // (The "is this project already open somewhere? then raise and exit" check
    // that used to live here is gone: the election above subsumes it. A remote
    // hands *every* launch to the primary, which resolves "already open" by
    // string id — `find_window(window_id_for(path))` — and focuses that exact
    // window. The old check could only ever answer for a *different* process,
    // and reached it through a `main_window_state` slot pointing at whichever
    // project window opened most recently, which with two open was the wrong
    // one.)

    // Everything a project window needs to build itself — bundled once here
    // and registered as `app_state` so both this initial-window decision and
    // a later runtime `ctx.open_window(...)` (from the Launcher, or the
    // Close-Work → Launcher path) build an identical window. See
    // `windows.rs`.
    // Created here, not in `App`: the menu bar is built alongside the app widget
    // rather than inside it, so the Format menu needs these signals before
    // `EditorsViewModel` exists. `App::build` attaches the editors on every
    // build — the same shape as the workspace-layout view-model.
    let project_factory = windows::ProjectWindowFactory::new(
        app_ctx.clone(),
        registry.clone(),
        spellcheck.clone(),
        backup_settings.clone(),
        workspace_layout_service,
        tree_expansion_service,
        autosave_menu.clone(),
        spellcheck_menu.clone(),
        comments_menu.clone(),
        margin_lane_menu.clone(),
    );

    // ── Decide the initial window: launcher-window model ────────────────
    // A project window is only ever created once its project is already
    // known (see the module docs above) — so this is the one place that
    // decides, up front, whether the very first window is the Launcher or a
    // project, based on argv / the persisted "show at startup" setting / the
    // most recent reachable project.
    //
    // `project_factory.window_config` now also hands back the fresh
    // `InitialWindowState` (session + outline + export) it just minted for
    // that window (Phase 2: each project window gets its own, never a shared
    // one) — captured here as `initial_state` so the small number of
    // remaining Tier-2 `app_state` registrations below (see
    // `sessions::WorkSession`'s module doc) have a real value for *this*, the
    // very first window. `None` when the initial window is the Launcher (no
    // Work open yet).
    // What this launch *would* open with, had there been no first-run offer.
    // Captured as a closure so the first-run window can perform exactly the same
    // decision once the writer has answered — see `first_run_window::answer`.
    let open_initial = {
        let ctx_for_initial = app_ctx.clone();
        let argv_path = initial_project.clone();
        move |ectx: &mut EventContext| match argv_path.clone() {
            Some(path) => {
                windows::open_or_focus_project(ectx, &path);
            }
            None => {
                ectx.open_window(windows::launcher_window_config(ctx_for_initial.clone()));
            }
        }
    };
    // The first launch of an edition that has its own config directory and finds
    // the community installation's settings beside it: offer to bring them
    // across before anything else happens. `first_run::pending` was resolved
    // before the settings services opened (see its doc) — by now this edition's
    // own `general.toml` exists, full of defaults, and the answer would be wrong.
    let (initial_window_config, initial_state) = if let Some(source) = first_run_offer.clone() {
        let open_initial = open_initial.clone();
        let source_for_answer = source.clone();
        (
            shell::first_run_window::first_run_window_config(
                source,
                std::rc::Rc::new(move |ectx: &mut EventContext, import: bool| {
                    shell::first_run_window::answer(
                        ectx,
                        &source_for_answer,
                        import,
                        &open_initial,
                    );
                }),
            ),
            None,
        )
    } else if let Some(path) = initial_project.clone() {
        // Launch with a path on argv (file manager, CLI): always skip the
        // Launcher.
        let (config, state) = project_factory.window_config(app::PendingAction::Load(path));
        (config, Some(state))
    } else if show_welcome_init {
        (windows::launcher_window_config(app_ctx.clone()), None)
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
                let (config, state) =
                    project_factory.window_config(app::PendingAction::Load(recent.absolute_path));
                (config, Some(state))
            }
            None => (windows::launcher_window_config(app_ctx.clone()), None),
        }
    };
    // Fallback used only when the initial window is the Launcher (no Work
    // open yet): a throwaway, never-seeded bundle so the `app_state`
    // registrations below always have *some* value of the right type — every
    // lookup simply answers "nothing open" until a real project window
    // provides its own constructor-threaded handle (the normal path for
    // everything except the few residual `app_state` readers named in
    // `ProjectWindowFactory::window_config`'s doc).
    let initial_state = initial_state.unwrap_or_else(|| {
        let throwaway_ids = AppIds::new();
        windows::InitialWindowState {
            outline: OutlineViewModel::new_default(app_ctx.clone(), throwaway_ids.clone()),
            session: WorkSession::new(
                app_ctx.clone(),
                throwaway_ids,
                spellcheck.clone(),
                teksilo::widgets::DockingModel::new(),
                backup_settings.clone(),
                WorkspaceLayoutService::in_memory_default(),
                TreeExpansionService::in_memory_default(),
            ),
        }
    });

    // Format + ProjectSwitch are per-window now (minted inside
    // `ProjectWindowFactory::build_window`). Residual `app_state` registrations
    // below still seed a throwaway Format for widgets that still look it up
    // that way (search preview / writing editors) — each real window's own
    // `App::build` re-registers its Format into the process map on build so
    // those surfaces pick up the *current* window's instance. That is still a
    // last-write-wins compromise for app_state readers; the attach/menu/dock
    // paths use constructor-threaded handles and are correct.

    TeksiloAppBuilder::new()
        .theme(theme)
        .application("eu", "skribisto", "Skribisto")
        .settings(SettingsBundle::new().with_window_state(true))
        // Ship the writing serifs so the manuscript defaults render everywhere
        // and the typeface picker lists them (additive; Inter stays the default).
        .register_fonts(register_editor_fonts())
        // Writing-model rich tooltips (the "＋ Create" / "Convert to" explainers).
        // Registered here so their `[label](:key)` bodies cascade to one another.
        .register_tooltips(tooltip_registry::writing_model_tooltips())
        .i18n(i18n)
        .install_inspector_in_debug()
        .install_automation_bridge_in_debug()
        .install_file_dialog()
        // Without this the OS never offers the window a drag at all: the
        // preview stops dead at the window border, and every drop handler
        // inside — the editor's, the binder's — is unreachable rather than
        // broken. It is per-application, not per-widget, which is why adding
        // handlers alone changed nothing.
        .install_external_dnd()
        // The OS menu service. On macOS it registers a real `NSMenu` backend, so
        // each project window's `MenuBar` (built `from_model`, flagged
        // `NativeMenuMode::Suppress` in `shell::windows`) mirrors its `MenuModel`
        // into the global bar at the top of the screen and hides the in-window
        // strip — the platform's own convention, and the only place a Mac user
        // looks for File/Edit. Everywhere else the backend is a no-op and the
        // in-window hamburger is unchanged, so this call costs Linux and Windows
        // nothing.
        //
        // Registered here and not per window on purpose: the handle is Tier 1
        // (one global bar for the process, following window focus), and the
        // widget-side bridge only ever *reads* it out of app-state.
        .install_native_menu()
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
        .app_state(registry.clone())
        // Tier 1, and genuinely so: there is one Help window per process and its
        // content explains the *application*, not any open `Work`. Every entry point
        // (F1, the Help menu, the Launcher's Learn pane) reads this one instance, which
        // is what lets a reader who reopens Help find the topic they left it on.
        .app_state(crate::help::help_vm::HelpViewModel::new())
        // Tier 1, like the registry above: one folder memory per process, correct for
        // every window because it is about the writer's habits and not about any Work.
        .app_state(folder_memory.clone())
        .app_state(import_prefs.clone())
        // The remaining `app_state` registrations below all come from
        // `initial_state` — the fresh, per-window Tier-2 bundle the *first*
        // window's own `window_config` call minted (see its doc, and
        // `sessions::WorkSession`'s module doc). This is a known, flagged
        // Phase-2 gap, not a full fix: a handful of widgets/panes
        // (`tags::tag_chip`, `overview`'s tree-expansion restore,
        // `app::capture_tree_expansion`, the Settings ▸ Work panes) still
        // resolve their Tier-2 view-model this way rather than via a
        // constructor-threaded handle, so they see only the *first* window's
        // Work, not a second Work opened in a second window later. Every
        // other consumer (the vast majority — confirmed by grep) already
        // reaches its Tier-2 state through a real constructor parameter, and
        // is unaffected by this limitation. `smart_punctuation`/
        // `text_replacements` are NOT in that handful — every consumer of
        // either (`App::build`, `SettingsPanel::build`) already reads them off
        // `session` directly, so unlike the fields below they need no
        // `app_state` registration at all.
        .app_state(initial_state.session.ids.clone())
        .app_state(initial_state.session.open_docs.clone())
        .app_state(spellcheck.clone())
        .app_state(dictionaries.clone())
        .app_state(initial_state.session.tags.clone())
        // Seed so first-build app_state readers find *some* Format; each
        // window re-points this on `App::build` to its own instance.
        .app_state(crate::format::FormatViewModel::detached())
        .app_state(initial_state.session.single_work.clone())
        .app_state(initial_state.session.single_work_info.clone())
        .app_state(initial_state.outline.clone())
        .app_state(initial_state.session.workspace_layout.clone())
        .app_state(initial_state.session.tree_expansion.clone())
        .app_state(initial_state.session.mention_index.clone())
        .app_state(initial_state.session.progress_recorder.clone())
        .app_state(import_plume.clone())
        .app_state(export_styles.clone())
        .app_state(paratext_presets.clone())
        .app_state(df_themes.clone())
        .app_state(initial_state.session.user_dictionary.clone())
        .app_state(backup_settings.clone())
        .app_state(initial_state.session.backup_scheduler.clone())
        .app_state(project_factory.clone())
        // Bind this instance's sockets: its own per-pid one always, and the
        // well-known primary one if it won the election. Both feed the router
        // below.
        .on_ready(move |proxy| ipc::spawn_listener(proxy, is_primary))
        // Serve remote launches. `on_external_with_ctx`, not `on_app_event`:
        // answering an `Open` means *opening a window*, and `on_app_event`
        // receives `&AppEvent` with no tree and no `WindowOps` — `open_window`
        // on a standalone context panics. This hook (added to teksilo for
        // exactly this) runs the closure against a live window's `EventContext`.
        .on_external_with_ctx({
            let app_ctx = app_ctx.clone();
            move |payload, ctx| {
                let Some(incoming) = payload.downcast_ref::<ipc::IncomingRequest>() else {
                    return false;
                };
                ipc_serve::serve_instance_request(&app_ctx, &incoming.request, ctx);
                // Acknowledge only after the window work is done, so a remote
                // that is told "accepted" really has had its request honoured.
                incoming.accept();
                true
            }
        })
        .initial_window(initial_window_config)
        .run();

    startup::shutdown(&app_ctx, &backup_settings, is_primary);
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // ── F4(b): pruning orphaned window-state rows ───────────────────────────

    /// A `work-*` label matching nothing in `known_paths` must be forgotten; a
    /// `work-*` label matching a known path, and the two fixed labels, must
    /// survive untouched. Fails on the old code (no pruning ever ran at all,
    /// so `window_state.toml` only ever grew) because the orphan label would
    /// still be present afterward.
    #[test]
    fn prune_orphaned_window_state_forgets_only_unmatched_work_labels() {
        let path = std::env::temp_dir().join(format!(
            "skribisto_test_{}_window_state_prune.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        let window_state = WindowStateService::open_at(path.clone(), std::time::Duration::ZERO)
            .expect("open a fresh window-state file");

        let known_path = "/tmp/skribisto-prune-test-known-project.skrib".to_string();
        let known_label = windows::window_id_for(&known_path);
        let orphan_label = "work-0000000000000000".to_string();

        let sample = |label: &str| PerWindowState {
            label: label.to_string(),
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            placement: WindowPlacement::Floating,
        };
        window_state.record(sample("main")).unwrap();
        window_state.record(sample("launcher")).unwrap();
        window_state.record(sample(&known_label)).unwrap();
        window_state.record(sample(&orphan_label)).unwrap();

        startup::prune_orphaned_window_state(&window_state, &[known_path]);

        let labels: std::collections::HashSet<String> = window_state.labels().into_iter().collect();
        assert!(labels.contains("main"), "fixed labels must never be pruned");
        assert!(
            labels.contains("launcher"),
            "fixed labels must never be pruned"
        );
        assert!(
            labels.contains(&known_label),
            "a work-* label matching a known path must survive"
        );
        assert!(
            !labels.contains(&orphan_label),
            "a work-* label matching nothing must be pruned"
        );

        let _ = std::fs::remove_file(&path);
    }

    /// Work ▸ New Window: a project owns one geometry row per window it has ever
    /// had — the bare id for the first, `-w{ordinal}` for the rest. The sweep
    /// must keep them all, and it runs on **every** launch, so getting this
    /// wrong would not lose a second window's geometry occasionally, it would
    /// lose it always — looking like the feature had simply never worked.
    #[test]
    fn prune_orphaned_window_state_keeps_a_known_projects_further_windows() {
        let path = std::env::temp_dir().join(format!(
            "skribisto_test_{}_window_state_prune_attached.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        let window_state = WindowStateService::open_at(path.clone(), std::time::Duration::ZERO)
            .expect("open a fresh window-state file");

        let known_path = "/tmp/skribisto-prune-test-attached-project.skrib".to_string();
        let known_label = windows::window_id_for(&known_path);
        let second = windows::attached_window_id_for(&known_path, 2);
        let third = windows::attached_window_id_for(&known_path, 7);
        // Same shape, unknown project — must still be pruned: the suffix is not
        // a licence to keep anything, only to trace a row back to its project.
        let orphan_second = "work-0000000000000000-w2".to_string();

        let sample = |label: &str| PerWindowState {
            label: label.to_string(),
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            placement: WindowPlacement::Floating,
        };
        window_state.record(sample(&known_label)).unwrap();
        window_state.record(sample(&second)).unwrap();
        window_state.record(sample(&third)).unwrap();
        window_state.record(sample(&orphan_second)).unwrap();

        startup::prune_orphaned_window_state(&window_state, &[known_path]);

        let labels: std::collections::HashSet<String> = window_state.labels().into_iter().collect();
        assert!(
            labels.contains(&known_label),
            "the first window's row must survive"
        );
        assert!(
            labels.contains(&second),
            "a second window's row must survive"
        );
        assert!(
            labels.contains(&third),
            "any ordinal's row must survive, not just -w2"
        );
        assert!(
            !labels.contains(&orphan_second),
            "a -w suffix on an unknown project must not rescue the row"
        );

        let _ = std::fs::remove_file(&path);
    }

    /// The suffix test decides what gets deleted, so it must not be fooled by a
    /// label that merely contains `-w`.
    #[test]
    fn a_window_label_suffix_must_be_a_number_to_count() {
        let mut known = std::collections::HashSet::new();
        known.insert("work-abc".to_string());

        assert!(
            startup::is_known_window_label("work-abc", &known),
            "the base id itself"
        );
        assert!(startup::is_known_window_label("work-abc-w2", &known));
        assert!(startup::is_known_window_label("work-abc-w13", &known));
        assert!(
            !startup::is_known_window_label("work-abc-w", &known),
            "no ordinal at all"
        );
        assert!(
            !startup::is_known_window_label("work-abc-wx", &known),
            "not a number"
        );
        assert!(
            !startup::is_known_window_label("work-abcd", &known),
            "a different project"
        );
        assert!(
            !startup::is_known_window_label("work-def-w2", &known),
            "an unknown project"
        );
    }

    /// The argv project must count as "known" even when it is in neither the
    /// recents MRU nor the open registry.
    ///
    /// The bug this pins: the startup prune ran *before* argv was read, so a
    /// file-manager double-click on a project that had aged out of the 30-entry
    /// MRU had its saved geometry forgotten milliseconds before that very window
    /// was restored — the window then opened at the default size and position,
    /// and the row was silently gone from `window_state.toml`.
    ///
    /// `known_project_paths` exists precisely so this is assertable: the failure
    /// was a data-flow ordering mistake in `main()`, invisible to a unit test of
    /// `prune_orphaned_window_state` alone (which was, and still is, correct —
    /// it was simply handed an incomplete set).
    #[test]
    fn the_argv_project_is_a_known_path_even_when_it_is_in_no_recents_list() {
        let argv = "/tmp/skribisto-argv-not-in-recents.skrib";

        let without = startup::known_project_paths(None);
        assert!(
            !without.iter().any(|p| p == argv),
            "precondition: this path is in neither recents nor the open registry"
        );

        let with = startup::known_project_paths(Some(argv));
        assert!(
            with.iter().any(|p| p == argv),
            "the project we were launched with must be treated as known, or the \
             prune deletes the geometry of the window it is about to restore"
        );
        assert_eq!(
            with.len(),
            without.len() + 1,
            "argv adds exactly itself — it must not disturb the other sources"
        );
    }

    #[test]
    fn prune_orphaned_window_state_is_a_no_op_when_everything_is_known() {
        let path = std::env::temp_dir().join(format!(
            "skribisto_test_{}_window_state_prune_noop.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        let window_state = WindowStateService::open_at(path.clone(), std::time::Duration::ZERO)
            .expect("open a fresh window-state file");

        let known_path = "/tmp/skribisto-prune-test-noop-project.skrib".to_string();
        let known_label = windows::window_id_for(&known_path);
        window_state
            .record(PerWindowState {
                label: known_label.clone(),
                x: 1,
                y: 2,
                width: 900,
                height: 700,
                placement: WindowPlacement::Floating,
            })
            .unwrap();

        startup::prune_orphaned_window_state(&window_state, &[known_path]);

        assert_eq!(window_state.labels(), vec![known_label]);

        let _ = std::fs::remove_file(&path);
    }
}
