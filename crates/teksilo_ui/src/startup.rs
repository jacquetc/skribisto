// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One-time work at launch, before the first window exists.
//!
//! Pruning the window-geometry rows of projects that are no longer anywhere,
//! opening the process-wide (Tier 1) services every window shares, assembling
//! the theme/i18n bundle the app builder needs, and the final teardown once
//! the last window has closed — the launch/shutdown work that happens once
//! per process and belongs to no feature.

use std::rc::Rc;

use frontend::AppContext;
use frontend::commands::handling_app_lifecycle_commands;

use teksilo::core::Theme;
use teksilo::core::presets::intui;
use teksilo::i18n::I18nConfig;
use teksilo::settings::{AppPaths, WindowStateService};
use teksilo::widgets::framework_locales;

use crate::backup::BackupSettingsViewModel;
use crate::cli;
use crate::export::{ExportStylesViewModel, ParatextPresetsViewModel};
use crate::first_run;
use crate::import_plume::ImportPlumeViewModel;
use crate::locales;
use crate::models;
use crate::models::{BackupSettingsService, TreeExpansionService, WorkspaceLayoutService};
use crate::sessions::WorkRegistry;
use crate::shell::{open_registry, windows};
use crate::spellcheck;

/// Resolve the first-run offer, sweep orphaned window-state rows, and seed the
/// shared Root/System frame — everything that has to happen before a single
/// settings service is opened for real.
///
/// `initial_project` is the `.skrib` path from argv, if any: it must count as
/// a "known" project for the prune below even though it is in neither the
/// recents MRU nor the open registry (see [`known_project_paths`]'s doc for
/// the bug that ordering mistake caused).
pub(crate) fn launch_maintenance(
    app_ctx: &AppContext,
    initial_project: Option<&str>,
) -> (Option<AppPaths>, Option<u64>) {
    // ── Is this an edition's very first launch? ───────────────────────────────
    // Resolved **here**, before a single settings service is opened, and not at
    // the point of use. `first_run::pending` asks (among other things) whether
    // this edition's `general.toml` exists yet — and opening the services below
    // creates it, populated with defaults. Asked any later, the answer is always
    // "no offer" on the one launch the offer exists for.
    let first_run_offer = first_run::pending();

    // ── One-time startup maintenance: prune orphaned window-state rows (F4b) ──
    // `window_state.toml` gets a `work-{hash}` row every time a project window
    // opens, but nothing ever removed one — a project tried once (or an
    // automation-test tempdir that no longer exists) leaves a permanent,
    // default-geometry row behind forever. Sweep once, synchronously, before
    // the app builder opens its own long-lived `WindowStateService` handle,
    // and before any `RecentWorkListModel` is constructed for real (see
    // `models::RecentWorkListModel::all_raw_paths`'s docs on why a short-lived
    // handle here is safe). A maintenance sweep, not a reactive per-close
    // mechanism: most orphaned rows point at tempdirs that still existed at
    // close time and were only deleted after process exit by the test
    // harness, so hooking `close_work` wouldn't have caught them; a raw
    // row-count cap was also rejected, since `window_state.toml`'s row order
    // is insertion order, not LRU, so trimming it would need a new recency
    // field. `"main"`/`"launcher"`/any other fixed label is never touched —
    // only `work-*` labels are ever considered.
    if let Some(paths) = crate::identity::app_paths() {
        match WindowStateService::open(&paths) {
            Ok(window_state) => {
                let known_paths = known_project_paths(initial_project);
                prune_orphaned_window_state(&window_state, &known_paths);
                if let Err(e) = window_state.flush_now() {
                    eprintln!("skribisto: window-state prune: flush failed: {e}");
                }
            }
            Err(e) => eprintln!("skribisto: window-state prune: open failed: {e}"),
        }
    }

    // Seed the single shared Root + System frame into the (empty) store at
    // startup — before any work is opened — and keep the returned Root id to
    // point `AppIds` at it. `initialize_app` is idempotent: a later load/new
    // reuses this frame instead of creating a second Root/System.
    let init_root_id = match handling_app_lifecycle_commands::initialize_app(app_ctx) {
        Ok(res) => Some(res.root_id),
        Err(e) => {
            eprintln!("initialize_app failed: {e:#}");
            None
        }
    };

    (first_run_offer, init_root_id)
}

/// The pure half of the F4(b) startup sweep: forget every `work-*` label in
/// `window_state` that doesn't hash (via [`windows::window_id_for`]) to one of
/// `known_paths`. Factored out from `main`'s production wiring (which resolves
/// `known_paths` from the real recents file + `open_registry::scan()`) so it's
/// directly testable against a temp-dir-backed `WindowStateService`, without
/// touching the real user's `window_state.toml` or recents file.
///
/// `"main"`, `"launcher"`, and any other label that doesn't start with
/// `"work-"` are never touched, regardless of `known_paths` — they are fixed,
/// not per-project.
/// Every project path this process can account for — the set the prune above
/// treats as "still wanted". Three sources, and **all three are load-bearing**:
///
/// 1. the recents MRU (the usual case);
/// 2. the open registry (a project another live instance is holding — it never
///    reaches *our* recents, but its geometry must survive);
/// 3. **`initial_project`** — the path handed to us on argv. This one is easy to
///    forget and is exactly the bug this function exists to make untestable-by-
///    omission: a file-manager double-click on a project that has aged out of the
///    30-entry MRU is in neither (1) nor (2), yet we are about to open it. Prune
///    without it and we delete the saved geometry of the very window we are
///    seconds away from restoring.
pub(crate) fn known_project_paths(initial_project: Option<&str>) -> Vec<String> {
    let mut known = models::RecentWorkListModel::all_raw_paths();
    known.extend(open_registry::scan().into_iter().map(|e| e.path));
    known.extend(initial_project.map(str::to_string));
    known
}

pub(crate) fn prune_orphaned_window_state(
    window_state: &WindowStateService,
    known_paths: &[String],
) {
    let known_labels: std::collections::HashSet<String> = known_paths
        .iter()
        .map(|p| windows::window_id_for(p))
        .collect();
    for label in window_state.labels() {
        if !label.starts_with("work-") || is_known_window_label(&label, &known_labels) {
            continue;
        }
        if let Err(e) = window_state.forget(&label) {
            eprintln!("skribisto: could not forget stale window state '{label}': {e}");
        }
    }
}

/// Does `label` name a window of a project we still know about?
///
/// Not a plain set lookup, because one project can own **several** geometry
/// rows since Work ▸ New Window: its first window saves under the bare
/// `windows::window_id_for(path)`, and each further one under
/// `{that}-w{ordinal}` (see `windows::attached_window_id_for`). Testing only for
/// exact membership would leave every `-w2`/`-w3` row unmatched and therefore
/// pruned — on *every* launch, so a second window's size and placement could
/// never survive one, and the loss would look like the geometry feature simply
/// not working for second windows rather than like a sweep deleting it.
///
/// The suffix must parse as a number rather than merely being present: `-w` is
/// otherwise the start of any string at all, and this decides what gets
/// deleted.
pub(crate) fn is_known_window_label(
    label: &str,
    known_labels: &std::collections::HashSet<String>,
) -> bool {
    if known_labels.contains(label) {
        return true;
    }
    match label.rsplit_once("-w") {
        Some((base, ordinal)) => {
            !ordinal.is_empty()
                && ordinal.bytes().all(|b| b.is_ascii_digit())
                && known_labels.contains(base)
        }
        None => false,
    }
}

/// The theme, the compiled `I18nConfig`, and the persisted UI-prefs booleans
/// the app builder needs before its first window exists.
pub(crate) struct UiConfig {
    pub theme: Theme,
    pub i18n: I18nConfig,
    pub autosave_init: bool,
    pub spellcheck_init: bool,
    pub show_welcome_init: bool,
}

pub(crate) fn build_ui_config() -> UiConfig {
    // Read persisted UI prefs before constructing the app (same AppPaths the
    // builder will use via `.application(...)`).
    let (dark, locale_str, autosave_init, spellcheck_init, show_welcome_init) = cli::read_prefs();

    let theme = if dark { intui::dark() } else { intui::light() };

    // The one place the app's supported locales are named. `locales::registered_locales`
    // is filtered against exactly this list, so an extension can never make a
    // language selectable that the app itself has no strings for.
    const SUPPORTED_LOCALES: &[&str] = &["en-US", "fr-FR"];

    let i18n = I18nConfig::new()
        .source_locale("en-US".parse().unwrap())
        .supported_locales(
            SUPPORTED_LOCALES
                .iter()
                .map(|l| l.parse().expect("a supported locale tag must parse")),
        )
        // Directory layout: one `.ftl` per topic per locale. The `tr!` macro
        // auto-detects `locales/en-US/` and validates keys across every file
        // in it, so the writing-model tooltips can live in their own file.
        .compile_in(&[
            (
                "en-US",
                &[
                    include_str!("../locales/en-US/main.ftl"),
                    include_str!("../locales/en-US/tooltips.ftl"),
                    include_str!("../locales/en-US/tags.ftl"),
                    include_str!("../locales/en-US/templates.ftl"),
                    include_str!("../locales/en-US/story_bible.ftl"),
                ],
            ),
            (
                "fr-FR",
                &[
                    include_str!("../locales/fr-FR/main.ftl"),
                    include_str!("../locales/fr-FR/tooltips.ftl"),
                    include_str!("../locales/fr-FR/tags.ftl"),
                    include_str!("../locales/fr-FR/templates.ftl"),
                    include_str!("../locales/fr-FR/story_bible.ftl"),
                ],
            ),
        ])
        .user_locale(locale_str.parse().ok())
        .auto_detect_os_locale(false)
        .fallback_locale("en-US".parse().unwrap())
        .framework_locales(framework_locales());

    // Extension strings, folded in **after** the app's own. `compile_in` extends
    // (teksilo `b924a27e` — before that a second call silently discarded these
    // four files and the window came up in message keys), and the manager merges
    // per locale keeping the FIRST definition of a key. So the app's own strings
    // win every collision: an extension gets its namespaced keys onto the screen
    // and cannot redefine `work-save` under the File menu.
    let extension_locales = locales::registered_locales(SUPPORTED_LOCALES);
    let i18n = extension_locales.iter().fold(i18n, |cfg, bundle| {
        cfg.compile_in(&[(bundle.locale.as_str(), bundle.resources.as_slice())])
    });

    UiConfig {
        theme,
        i18n,
        autosave_init,
        spellcheck_init,
        show_welcome_init,
    }
}

/// Everything Tier 1 (process-wide, shared by every window regardless of which
/// `Work` it has open) needs opened before the first window exists: the
/// settings-backed services and the app-global view-models built on top of
/// them.
pub(crate) struct Tier1Services {
    pub registry: WorkRegistry,
    pub spellcheck: spellcheck::SpellcheckService,
    pub dictionaries: spellcheck::DictionariesViewModel,
    pub workspace_layout_service: WorkspaceLayoutService,
    pub tree_expansion_service: TreeExpansionService,
    pub import_plume: ImportPlumeViewModel,
    pub folder_memory: models::FolderMemoryService,
    pub import_prefs: models::ImportPrefsService,
    pub export_styles: ExportStylesViewModel,
    pub paratext_presets: ParatextPresetsViewModel,
    pub df_themes: crate::distraction_free::DistractionFreeThemesViewModel,
    pub backup_settings: BackupSettingsViewModel,
    /// Which Book a note's "In prose" segment last showed, per project.
    pub note_book_choice: models::NoteBookChoiceService,
    /// What "Add as note" remembers between captures, per project.
    pub note_capture: models::NoteCaptureService,
}

pub(crate) fn open_tier1_services(
    app_ctx: &Rc<AppContext>,
    init_root_id: Option<u64>,
) -> Tier1Services {
    // The app-global (Tier 1) registry: `root_id` (see `app_ids.rs`'s module doc
    // for why that one field lives here and not on the per-Work `AppIds`) plus
    // the real `work_id`-keyed table of every currently-open `WorkSession`
    // (Phase 2 — see `sessions::WorkRegistry`'s module doc). Registered as
    // `app_state` so any future consumer can reach it the same way as everything
    // else here.
    let registry = WorkRegistry::new();
    // Point the app at the shared Root seeded by `initialize_app` above, so the
    // root id is known before any work is opened (a load/new refreshes it later).
    if let Some(root_id) = init_root_id {
        registry.set_root_id(Some(root_id));
    }
    // Spell-checking (Step 6): the engine is shared by every open document (one
    // `spellbook::Dictionary` per language) — a genuinely machine-wide resource
    // (Tier 1), handed to each window's own session so its `OpenDocsStore` can
    // attach it. Registered as `app_state` so the language-pill field and the
    // personal-word list reach the same instance (mute set, personal words).
    let spellcheck = spellcheck::SpellcheckService::new();
    // Dictionary management: accepted-licence store (cross-process, like `backup.toml`),
    // on-disk discovery, and the download view-model. App-local (a downloaded `.dic` is a
    // machine-wide resource, not `Work` state) — degrades to a throwaway temp settings
    // file if the config dir is unavailable, exactly as backup settings do.
    let dictionary_settings = crate::identity::app_paths()
        .and_then(|paths| {
            models::DictionarySettingsService::open(&paths)
                .map_err(|e| eprintln!("dictionary settings: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(models::DictionarySettingsService::in_memory_default);
    let installed_dictionaries =
        models::InstalledDictionariesModel::new(dictionary_settings.clone());
    let dictionaries =
        spellcheck::DictionariesViewModel::new(dictionary_settings, installed_dictionaries);
    // Per-work workspace layout (open editor tabs + dock arrangement): opened
    // eagerly here so the restore fires on the first `LoadWork`. App-local config
    // (`workspace.toml`, keyed by `Work.unique_id`), orthogonal to the `.skrib`
    // document — degrades to a throwaway temp file if the config dir is
    // unavailable, exactly as the backup/search settings do. The VM reads the
    // project uid/path from the singles, drives the shared `DockingModel` (via the
    // outline handle), and is handed the editors once `App::build` creates them.
    let workspace_layout_service = crate::identity::app_paths()
        .and_then(|paths| {
            WorkspaceLayoutService::open(&paths)
                .map_err(|e| eprintln!("workspace layout: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(WorkspaceLayoutService::in_memory_default);
    // Remembered Overview expand state, keyed per project + per container by durable uid
    // (`tree_expansion.toml`). A fourth `SettingsFile` sibling; on failure the feature
    // simply goes quiet rather than blocking startup, exactly as the layout service does.
    let tree_expansion_service = crate::identity::app_paths()
        .and_then(|paths| {
            TreeExpansionService::open(&paths)
                .map_err(|e| eprintln!("tree expansion: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(TreeExpansionService::in_memory_default);
    // Which Book a note's "In prose" segment was last showing, one row per project
    // (`note_book_choice.toml`). A fifth `SettingsFile` sibling, opened and degraded the
    // same way: a writer whose config dir is unavailable simply reopens on the first Book
    // rather than losing the segment.
    let note_book_choice = crate::identity::app_paths()
        .and_then(|paths| {
            models::NoteBookChoiceService::open(&paths)
                .map_err(|e| eprintln!("note book choice: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(models::NoteBookChoiceService::in_memory_default);
    // Which tags this writer captures under, and where an untagged note goes
    // (`note_capture.toml`). A sixth `SettingsFile` sibling, degraded the same way: with
    // no config directory the capture menu still works, it simply stops remembering.
    let note_capture = crate::identity::app_paths()
        .and_then(|paths| {
            models::NoteCaptureService::open(&paths)
                .map_err(|e| eprintln!("note capture: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(models::NoteCaptureService::in_memory_default);
    // The Import-Plume view-model is a singleton (form + in-flight job + progress
    // toast). Registered as app-state so `App::build` can route the import's
    // long-operation events to it and the menu action can reach it to open the panel.
    let import_plume = ImportPlumeViewModel::new(app_ctx.clone());
    // Export styles ("Compile & Export" formats) — the user's editable style presets, opened
    // eagerly here so the Settings pane and the Export panel's picker both read one instance.
    // Where each kind of file dialog last opened. App-global by nature — the folder a
    // writer exports to is theirs, not any one manuscript's — so it is opened once here
    // and read through `app_state`, the one tier that slot is actually right for.
    let folder_memory = crate::identity::app_paths()
        .and_then(|paths| {
            models::FolderMemoryService::open(&paths)
                .map_err(|e| eprintln!("folder memory: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(models::FolderMemoryService::in_memory_default);
    // Where each project's last document import landed. Per-project rows in one
    // app-global file, like tree expansion.
    let import_prefs = crate::identity::app_paths()
        .and_then(|paths| {
            models::ImportPrefsService::open(&paths)
                .map_err(|e| eprintln!("import prefs: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(models::ImportPrefsService::in_memory_default);
    // App-local config (a style outlives any project); degrades to a throwaway temp file if the
    // config dir is unavailable, exactly as backup settings do.
    let export_styles_service = crate::identity::app_paths()
        .and_then(|paths| {
            models::ExportStylesService::open(&paths)
                .map_err(|e| eprintln!("export styles: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(models::ExportStylesService::in_memory_default);
    let export_styles = ExportStylesViewModel::new(export_styles_service);
    // Paratext presets — the front/back matter structures New Work can start a project
    // with, and the Settings pane edits. Opened here for the same reason export styles
    // are: one instance, so a preset written in Settings is the one New Work offers.
    let paratext_presets_service = crate::identity::app_paths()
        .and_then(|paths| {
            models::ParatextPresetsService::open(&paths)
                .map_err(|e| eprintln!("paratext presets: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(models::ParatextPresetsService::in_memory_default);
    let paratext_presets = ParatextPresetsViewModel::new(paratext_presets_service);
    // The distraction-free theme library, on the same footing and for the same
    // reasons (a theme outlives any project, and the settings pane and the
    // mode's own picker must read one instance).
    let df_themes_service = crate::identity::app_paths()
        .and_then(|paths| {
            models::DistractionFreeThemesService::open(&paths)
                .map_err(|e| eprintln!("distraction-free themes: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(models::DistractionFreeThemesService::in_memory_default);
    let df_themes = crate::distraction_free::DistractionFreeThemesViewModel::new(df_themes_service);
    // Backup-mode state (`backup_mode` true while a *backup file* is open — Save
    // + auto-backup off, the file read-only, the content still editable;
    // `backup_context` carries the open backup's details, driving the permanent
    // banner + restore) is **not** constructed here any more (Phase 3): it used
    // to be one process-wide `Signal` pair threaded unchanged into every window,
    // which let one Work's backup-mode flag leak into a second, simultaneously-
    // open Work's window. `WorkSession::new` now mints a fresh pair per Work —
    // see its module doc — so the title-bar menu / `App` read theirs off
    // `session.backup_mode`/`session.backup_context` instead (see
    // `ProjectWindowFactory::window_config`). The personal dictionary, tag
    // palette, tree-expansion and workspace-layout view-models moved the same
    // way (`session.user_dictionary`/`session.tags`/`session.tree_expansion`/
    // `session.workspace_layout`), and `save_as_vm` is minted per window by
    // `ProjectWindowFactory` for the same reason. The punctuation house style
    // and the per-project custom replacement lexicon ("btw" → "by the way")
    // moved the same way too (`session.smart_punctuation`/
    // `session.text_replacements`) — `WorkSession::new` mints a fresh instance
    // of each per Work, so a second, simultaneously-open Work never shares
    // this Work's punctuation row or lexicon.

    // Backup ("Copies de secours") settings — opened eagerly here (before any
    // project loads) so the on-open/on-close/interval hooks and the scheduler see
    // it. Degrades to a throwaway temp file if the config dir is unavailable,
    // exactly as the recents MRU does.
    let backup_service = crate::identity::app_paths()
        .and_then(|paths| {
            BackupSettingsService::open(&paths)
                .map_err(|e| eprintln!("backup settings: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(BackupSettingsService::in_memory_default);
    let backup_settings = BackupSettingsViewModel::new(backup_service);

    Tier1Services {
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
        note_book_choice,
        note_capture,
    }
}

/// Teardown once the last window has closed and `TeksiloAppBuilder::run()` has
/// returned, in the order that keeps it from hanging: flush what still owes a
/// synchronous write, release this instance's cross-process claims, then fire
/// the backend's own `CleanUpBeforeExit` before the event thread stops.
pub(crate) fn shutdown(
    app_ctx: &Rc<AppContext>,
    backup_settings: &BackupSettingsViewModel,
    is_primary: bool,
) {
    // Flush the recent-works MRU synchronously so a just-opened project isn't
    // lost inside the debounce window, and *release* it while the app is still
    // alive — its writer is parked in a thread-local, whose destructor would
    // otherwise run during process teardown, after the settings-writer thread it
    // waits on has been killed (an unkillable hang; see `shutdown`'s docs). Then
    // tear the shared Root/System frame down and fire `CleanUpBeforeExit` before
    // the event thread is stopped.
    crate::models::RecentWorkListModel::shutdown();
    // Flush any pending backup-settings write (last-success hashes / timestamps /
    // nudge flag / edited policy) so it survives the debounce window on exit.
    backup_settings.flush_now();
    // Drop every open-registry claim this instance holds (process exit — the
    // "release everything" point, unlike `CloseWork`'s single-path release) so
    // its project(s) stop showing as open in other instances' switchers, and
    // unlink this instance's IPC socket so a later `scan()` never has to reap
    // it as stale.
    crate::shell::open_registry::release_all();
    // Unlink both sockets this instance bound — its own per-pid one, and (only
    // if it was the primary) the well-known election socket. Leaving the latter
    // behind would make the *next* launch pay one failed connect before it could
    // unlink and claim it.
    crate::shell::ipc::cleanup_own_sockets(is_primary);
    if let Err(e) = handling_app_lifecycle_commands::clean_up_before_exit(app_ctx) {
        eprintln!("clean_up_before_exit failed: {e:#}");
    }
    app_ctx.shutdown();
}
