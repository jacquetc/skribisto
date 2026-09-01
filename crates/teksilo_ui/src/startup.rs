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
use teksilo::i18n::{I18nConfig, I18nManager, compile_in_locales};
use teksilo::settings::{AppPaths, WindowStateService};
use teksilo::widgets::framework_locales;

use crate::backup::BackupSettingsViewModel;
use crate::cli;
use crate::export::{ExportStylesViewModel, ParatextPresetsViewModel};
use crate::first_run;
use crate::import_manuskript::ImportManuskriptViewModel;
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

/// The one place the app's supported interface languages are named.
///
/// [`locales::registered_locales`] is filtered against exactly this list, so an
/// extension can never make a language selectable that the app itself has no
/// strings for, and [`os_default_locale`] resolves the writer's OS languages
/// against it too.
pub(crate) const SUPPORTED_LOCALES: &[&str] = &["en-US", "fr-FR"];

/// What the interface language is when nobody has ever chosen one: the closest
/// supported match for the writer's OS languages, or `en-US` if none of them is
/// a language this app speaks.
///
/// Resolved by teksilo rather than here so this and the real startup path can
/// not disagree — [`I18nManager::resolve_initial_locale`] reads only the four
/// fields set below, so a bundle-free config answers exactly what the compiled
/// one would. It is the answer `--dump-config` must print for an unset
/// `ui.locale`, and the one *Reset to defaults* must restore: reporting a flat
/// `en-US` while a French account launches in French is the precise kind of
/// quiet lie [`crate::settings_keys`] exists to prevent.
pub(crate) fn os_default_locale() -> String {
    let cfg = I18nConfig::new()
        .supported_locales(
            SUPPORTED_LOCALES
                .iter()
                .map(|l| l.parse().expect("a supported locale tag must parse")),
        )
        .auto_detect_os_locale(true)
        .fallback_locale("en-US".parse().expect("the fallback tag must parse"));
    I18nManager::resolve_initial_locale(&cfg).to_string()
}

/// The `.ftl` files each locale directory holds, one per topic.
///
/// Only the *names*: the cross-product with [`SUPPORTED_LOCALES`] is built by
/// [`app_locales`]. Adding a topic file means adding it here once, not once
/// per locale.
pub(crate) const LOCALE_FILES: &[&str] = &[
    "main.ftl",
    "tooltips.ftl",
    "tags.ftl",
    "templates.ftl",
    "story_bible.ftl",
];

/// Every locale's catalogue, compiled into the binary.
///
/// `compile_in_locales!` expands to the `locales × files` cross-product of
/// `include_str!` calls: the same twenty-odd lines this used to spell out by
/// hand, minus the chance of forgetting one. A missing file is a compile error
/// naming it, because `include_str!` cannot resolve it; a file present on disk
/// but absent from [`LOCALE_FILES`] is silently not shipped, which is what the
/// drift test in this module's `mod tests` is for.
///
/// `base` is relative to **this source file**, not the crate root. The path is
/// handed to `include_str!`, which resolves against the file it appears in. So
/// `../locales/` from `src/startup.rs` means `crates/teksilo_ui/locales/`.
///
/// Kept as a function rather than a `const` so the macro's expansion has one
/// name to attach a doc comment and a test to.
pub(crate) fn app_locales() -> &'static [(&'static str, &'static [&'static str])] {
    // Directory layout: one `.ftl` per topic per locale. The `tr!` macro
    // auto-detects `locales/en-US/` and validates keys across every file in it,
    // so the writing-model tooltips can live in their own file.
    //
    // The two lists below cannot be `SUPPORTED_LOCALES` and `LOCALE_FILES`:
    // `include_str!` needs literals at expansion time and cannot read a const.
    // The drift test holds them in step instead.
    compile_in_locales!(
        base = "../locales/",
        locales = ["en-US", "fr-FR"],
        files = [
            "main.ftl",
            "tooltips.ftl",
            "tags.ftl",
            "templates.ftl",
            "story_bible.ftl",
        ],
    )
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

/// Whether the desktop is asking for a dark interface, with `fallback` used when
/// it has no preference to report (or cannot be asked).
///
/// Teksilo's own query, not a second implementation of it: the same call
/// `WindowManager::apply_os_theme` makes when the app is following the system,
/// so the theme this seeds at launch and the theme teksilo would resolve a
/// moment later cannot disagree.
fn os_prefers_dark(fallback: bool) -> bool {
    match teksilo::platform::os_theme::query_color_scheme() {
        teksilo::tokens::ColorSchemePreference::Dark => true,
        teksilo::tokens::ColorSchemePreference::Light => false,
        teksilo::tokens::ColorSchemePreference::NoPreference => fallback,
    }
}

/// The theme to launch in, given the writer's recorded choice.
///
/// Pure, and separated from [`build_ui_config`] for that reason — the OS query
/// and the config directory are both machine state, and this is the decision
/// worth pinning down.
///
/// * `mode` — [`crate::THEME_MODE_KEY`], or `None` for an install written before
///   that key existed (or holding an illegal value; `cli::persisted_theme_mode`
///   collapses the two).
/// * `dark_key` — [`crate::DARK_KEY`], the legacy answer.
/// * `system_dark` — what the desktop is asking for right now.
///
/// `None` falls back to `dark_key` so **nobody's theme changes on upgrade**:
/// the new key is inert until something writes it, and until then the app
/// launches in exactly the theme it launched in before.
///
/// The `"system"` arm stamps [`crate::SYSTEM_THEME_ID`] on the result, which is
/// not cosmetic. It is the same id teksilo gives a theme it resolved from the
/// OS, so the picker's *System* entry is selected at launch (it matches by id),
/// and the Reset gate reads the mode back out of the live theme rather than
/// having to be told. Stamping it on the *active style's* light or dark keeps
/// `--style` intact for the launch, where teksilo's own follow-OS path would
/// drop the run back to IntUI.
pub(crate) fn theme_for(mode: Option<&str>, dark_key: bool, system_dark: bool) -> Theme {
    match mode {
        Some(crate::THEME_MODE_LIGHT) => crate::style::light(),
        Some(crate::THEME_MODE_DARK) => crate::style::dark(),
        Some(crate::THEME_MODE_SYSTEM) => {
            crate::style::theme(system_dark).with_id(crate::SYSTEM_THEME_ID)
        }
        _ => crate::style::theme(dark_key),
    }
}

/// Build the launch-time UI configuration.
///
/// `translation_dev` is the (usually empty) set of `.ftl` directories
/// `--translation-dev` asked to hot-reload, already validated against the
/// filesystem by [`cli::resolve_translation_dev`].
pub(crate) fn build_ui_config(
    translation_dev: Vec<(teksilo::prelude::LanguageIdentifier, std::path::PathBuf)>,
) -> UiConfig {
    // Read persisted UI prefs before constructing the app (same AppPaths the
    // builder will use via `.application(...)`).
    let cli::Prefs {
        dark,
        theme_mode,
        locale: chosen_locale,
        autosave: autosave_init,
        spellcheck: spellcheck_init,
        show_welcome: show_welcome_init,
    } = cli::read_prefs();

    // The active design language's light or dark theme — IntUI unless `--style`
    // named another (see `crate::style`). This is the one site that decides the
    // app's *chrome*: widget shapes resolve when a widget is built, so every
    // later `set_theme` only ever retints within the same family.
    //
    // Which of the two, and whether the desktop gets a say, is
    // `ui.theme_mode`'s to answer — see [`theme_for`]. `ui.dark` is the
    // fallback for an install written before that key existed, and the
    // tie-break when the desktop reports no preference at all.
    let theme = theme_for(theme_mode.as_deref(), dark, os_prefers_dark(dark));

    let i18n = I18nConfig::new()
        .source_locale("en-US".parse().unwrap())
        .supported_locales(
            SUPPORTED_LOCALES
                .iter()
                .map(|l| l.parse().expect("a supported locale tag must parse")),
        )
        .compile_in(app_locales())
        // `None` when the writer has never picked a language — which is the only
        // state that lets the OS step below run at all, since a `user_locale`
        // teksilo supports short-circuits resolution. `read_prefs` returning a
        // flat `"en-US"` for an unset key is what made a French Windows account
        // launch in English: the app was telling teksilo the writer had chosen
        // English.
        .user_locale(chosen_locale.and_then(|l| l.parse().ok()))
        // Consulted only for that unset case, and only through
        // `SUPPORTED_LOCALES` — an OS reporting `fr`, `fr-CA` or `fr-BE` lands
        // on `fr-FR`, and a language the app has no strings for falls through
        // to the fallback below.
        .auto_detect_os_locale(true)
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

    // `--translation-dev`: watch a locale's directory and rebuild its bundle on
    // every save. Registered **last**, after the extension catalogues, because a
    // reload replaces that locale's whole bundle, so while the flag is in use,
    // the watched locale is exactly what is on disk in that directory and
    // nothing else. That is the point (the translator is previewing their own
    // files), and it is also why this is a development flag rather than a
    // setting: an extension's strings for the watched locale disappear until the
    // app is restarted without it.
    let i18n = translation_dev
        .into_iter()
        .fold(i18n, |cfg, (locale, dir)| cfg.runtime_override(locale, dir));

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
    pub import_manuskript: ImportManuskriptViewModel,
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
    /// Which copies of a manuscript have gone out for review, per project.
    pub exchange: models::ExchangeService,
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
    // Which copies have gone out for review (`exchange.toml`). A seventh sibling,
    // degraded the same way: with no config directory an export still works, it
    // simply stops being remembered — and the readouts that depend on it go quiet
    // rather than lying.
    let exchange = crate::identity::app_paths()
        .and_then(|paths| {
            models::ExchangeService::open(&paths)
                .map_err(|e| eprintln!("exchange record: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(models::ExchangeService::in_memory_default);
    // The Import-Plume view-model is a singleton (form + in-flight job + progress
    // toast). Registered as app-state so `App::build` can route the import's
    // long-operation events to it and the menu action can reach it to open the panel.
    let import_manuskript = ImportManuskriptViewModel::new(app_ctx.clone());
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
        import_manuskript,
        import_plume,
        folder_memory,
        import_prefs,
        export_styles,
        paratext_presets,
        df_themes,
        backup_settings,
        note_book_choice,
        note_capture,
        exchange,
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

#[cfg(test)]
mod tests {
    /// `app_locales`' `locales = [...]` list cannot *be* [`SUPPORTED_LOCALES`]
    /// (`include_str!` needs literals at expansion time and cannot read a
    /// const), so the two lists are held in step here instead.
    ///
    /// Without this, adding a language to `SUPPORTED_LOCALES` and forgetting
    /// the macro gives a language the picker offers and the app has no strings
    /// for: every label falls back to `en-US` and nothing reports a problem.
    #[test]
    fn every_supported_locale_ships_a_catalogue() {
        let compiled: Vec<&str> = super::app_locales().iter().map(|(tag, _)| *tag).collect();
        assert_eq!(
            compiled,
            super::SUPPORTED_LOCALES.to_vec(),
            "`app_locales` and `SUPPORTED_LOCALES` must list the same locales, in the same order"
        );
    }

    /// Every locale ships every topic file, and [`LOCALE_FILES`] names exactly
    /// those files.
    ///
    /// The macro guarantees a *missing* file is a compile error (`include_str!`
    /// cannot resolve it). What it cannot catch is the other direction: a new
    /// `.ftl` added to `locales/en-US/` and never added to the macro's `files`
    /// list is simply not compiled in, and every key in it resolves to its own
    /// name on screen.
    #[test]
    fn every_locale_directory_matches_locale_files() {
        for (tag, resources) in super::app_locales() {
            assert_eq!(
                resources.len(),
                super::LOCALE_FILES.len(),
                "{tag} compiles in {} resources but LOCALE_FILES names {}",
                resources.len(),
                super::LOCALE_FILES.len()
            );

            let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("locales")
                .join(tag);
            let mut on_disk: Vec<String> = std::fs::read_dir(&dir)
                .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
                .filter_map(Result::ok)
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .filter(|name| name.ends_with(".ftl"))
                .collect();
            on_disk.sort();

            let mut expected: Vec<String> = super::LOCALE_FILES
                .iter()
                .map(|f| (*f).to_string())
                .collect();
            expected.sort();

            assert_eq!(
                on_disk, expected,
                "{tag} holds .ftl files LOCALE_FILES does not name (or vice versa)"
            );
        }
    }

    /// The catalogues are not empty and not each other's copy. This guards
    /// against a `base`/`locales` typo that happened to resolve to a real but
    /// wrong directory.
    #[test]
    fn each_locale_compiles_in_its_own_strings() {
        let locales = super::app_locales();
        for (tag, resources) in locales {
            for (index, body) in resources.iter().enumerate() {
                assert!(
                    !body.trim().is_empty(),
                    "{tag}/{} is empty",
                    super::LOCALE_FILES[index]
                );
            }
        }
        let [(_, first), (_, second)] = locales else {
            panic!("expected exactly two locales");
        };
        assert_ne!(
            first[0], second[0],
            "two locales compiled in the same main.ftl"
        );
    }

    use super::*;

    /// The one hard invariant of [`os_default_locale`]. `set_locale` silently
    /// no-ops on a locale outside `supported_locales`, and teksilo does *not*
    /// validate `fallback_locale` against that list — so a default outside the
    /// set would not raise anything, it would leave the interface stuck in
    /// whatever language it already showed, with `--dump-config` cheerfully
    /// printing the unreachable tag as the effective value.
    ///
    /// Machine-dependent by nature (it reads this host's OS languages), which is
    /// the point: the assertion holds whatever the operator's account is set to.
    #[test]
    fn the_default_locale_is_always_one_the_app_has_strings_for() {
        let got = os_default_locale();
        assert!(
            SUPPORTED_LOCALES.contains(&got.as_str()),
            "os_default_locale() returned {got:?}, which is not in {SUPPORTED_LOCALES:?}"
        );
    }

    /// The list teksilo resolves against has to be parseable, or
    /// [`os_default_locale`] panics on the first launch of every install rather
    /// than on a developer's machine.
    #[test]
    fn every_supported_locale_tag_parses() {
        for tag in SUPPORTED_LOCALES {
            assert!(
                tag.parse::<teksilo::i18n::LanguageIdentifier>().is_ok(),
                "{tag} is not a parseable BCP-47 tag"
            );
        }
    }

    // ── The theme the launch seeds ───────────────────────────────────────

    /// The upgrade guarantee. An install with no `ui.theme_mode` line launches
    /// in exactly the theme `ui.dark` alone used to pick — whatever the desktop
    /// happens to prefer, which is the value that must NOT leak in here.
    #[test]
    fn an_install_from_before_the_key_still_launches_on_ui_dark() {
        for system_dark in [false, true] {
            assert!(theme_for(None, true, system_dark).is_dark());
            assert!(!theme_for(None, false, system_dark).is_dark());
        }
    }

    /// An illegal value is the same answer as an absent one — it must not fall
    /// through into the system arm and quietly hand the desktop a vote.
    #[test]
    fn an_unrecognised_mode_falls_back_to_ui_dark() {
        assert!(theme_for(Some("System"), true, false).is_dark());
        assert!(!theme_for(Some("nonsense"), false, true).is_dark());
    }

    /// The two manual answers pin the app's own theme whatever the desktop says.
    #[test]
    fn a_manual_mode_ignores_the_desktop() {
        for system_dark in [false, true] {
            assert!(!theme_for(Some(crate::THEME_MODE_LIGHT), true, system_dark).is_dark());
            assert!(theme_for(Some(crate::THEME_MODE_DARK), false, system_dark).is_dark());
        }
    }

    /// System follows the desktop rather than the last mirrored `ui.dark` — the
    /// bug this key exists for: choosing System used to pin itself as manual
    /// light or dark on the very next launch.
    #[test]
    fn system_follows_the_desktop_not_the_last_mirrored_state() {
        assert!(theme_for(Some(crate::THEME_MODE_SYSTEM), false, true).is_dark());
        assert!(!theme_for(Some(crate::THEME_MODE_SYSTEM), true, false).is_dark());
    }

    /// …and says so in its id, which is what lets the picker show *System* at
    /// launch and the Reset gate read the mode back off the live theme. Both
    /// match by id, so a plain light/dark theme here would read as a manual
    /// choice and light the Reset button at factory state.
    #[test]
    fn a_system_theme_is_labelled_as_one_and_the_others_are_not() {
        for system_dark in [false, true] {
            let theme = theme_for(Some(crate::THEME_MODE_SYSTEM), false, system_dark);
            assert_eq!(theme.id.as_str(), crate::SYSTEM_THEME_ID);
            assert_eq!(
                crate::settings_keys::theme_mode_of(&theme),
                crate::THEME_MODE_SYSTEM
            );
        }
        for (mode, dark_key) in [
            (Some(crate::THEME_MODE_LIGHT), false),
            (Some(crate::THEME_MODE_DARK), true),
            (None, true),
            (None, false),
        ] {
            let theme = theme_for(mode, dark_key, true);
            assert_ne!(
                theme.id.as_str(),
                crate::SYSTEM_THEME_ID,
                "a pinned theme must not claim to be following the desktop"
            );
        }
    }

    /// `--style` survives the system arm. Teksilo's own follow-OS path drops to
    /// its IntUI presets by design; the launch seed must not, or a run started
    /// under another design language would come up in the wrong one and every
    /// widget built in it would keep IntUI's shapes for the life of the process.
    #[test]
    fn the_system_arm_stays_inside_the_active_design_language() {
        for system_dark in [false, true] {
            let seeded = theme_for(Some(crate::THEME_MODE_SYSTEM), false, system_dark);
            let styled = crate::style::theme(system_dark);
            assert_eq!(seeded.colors.surface_main, styled.colors.surface_main);
            assert_eq!(seeded.is_dark(), system_dark);
        }
    }

    /// The desktop's "no preference" is not "light": it falls back to the last
    /// state the app recorded, so a writer on a desktop that cannot be asked
    /// keeps the theme they were looking at.
    #[test]
    fn no_preference_falls_back_to_the_recorded_state() {
        // `os_prefers_dark` is the only place that decision lives; exercised
        // through it rather than restated, so the two cannot drift.
        let asked = os_prefers_dark(true);
        let asked_light = os_prefers_dark(false);
        assert!(
            asked == asked_light || (asked && !asked_light),
            "the fallback may only decide the NoPreference case"
        );
    }
}
