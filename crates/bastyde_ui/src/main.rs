// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

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

mod icons;
mod statusbar;
mod shell;
mod trash;
mod export;
mod binder;
mod panels;
mod a11y;
mod app;
mod app_ids;
mod backup;
mod date_convert;
mod docks;
mod intents;
mod models;
mod settings;
mod singles;
mod spellcheck;
mod tags;
mod widgets;
mod tabs;
// The pane tests that need fixture rows are mocks-gated, but the search preview's
// layout tests build their own `OpenDoc`, so they run on the real backend too —
// and both need an event source. Hence the plain `test` gate.
#[cfg(test)]
mod test_support;
mod tooltip_registry;
mod version;
mod view_models;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use bastyde::core::event_source::{EventSource, SubscriptionHandle};

use bastyde::core::app_event::AppEvent;
use bastyde::prelude::*; // also brings the file-dialog ext + FileDialogRequest/Result
use bastyde::settings::{AppPaths, SettingsStore};
use bastyde::widgets::framework_locales;

use shell::{ipc, open_registry, windows};

use frontend::AppContext;
use frontend::EventHubClient;
use frontend::commands::{handling_app_lifecycle_commands, work_info_commands};
use frontend::common::event::{Event, Origin};

use app::PendingExit;
use app_ids::AppIds;
use models::{BackupSettingsService, OpenDocsStore, TreeExpansionService, WorkspaceLayoutService};
use singles::{SingleDictWord, SingleWork, SingleWorkInfo};
use view_models::{
    BackupSchedulerViewModel, BackupSettingsViewModel, ExportViewModel, ImportPlumeViewModel,
    OutlineViewModel, ProgressRecorder, ProjectSwitchViewModel, BackupRestoreViewModel, SaveAsViewModel,
    TreeExpansionViewModel, WorkspaceLayoutViewModel,
};

/// The currently-open project's path (from `WorkInfo`), if any.
pub(crate) fn current_project_path(ctx: &AppContext) -> Option<String> {
    work_info_commands::get_all_work_info(ctx)
        .ok()?
        .into_iter()
        .next()?
        .file_name
}

/// The open work's base name (no extension), or `"work"` — pre-fills the
/// "Save as" dialog's file name. A folder work's entry is `…/project.skrib`
/// (the on-disk manifest name), so its directory name is used.
pub(crate) fn project_stem(ctx: &AppContext) -> String {
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
/// Max width (px) of the search preview editor (bottom band), so a too-wide
/// paragraph stays readable — same treatment as the scene column.
pub const PREVIEW_WIDTH_KEY: &str = "search.preview_width";
pub const PREVIEW_WIDTH_DEFAULT: f32 = 700.0;
/// When on, autosave to disk (and hide the manual Save / Ctrl+S affordances).
pub const AUTOSAVE_KEY: &str = "editor.autosave";
/// The master spell-check switch (default **on**) — the title-bar toggle, View ▸ Check
/// spelling, F7, and Settings ▸ Spelling all drive this one key.
///
/// App-wide and persisted here rather than on the `Work`: whether squiggles are drawn is a
/// preference of the person reading the screen, not a property of the manuscript, so it
/// follows the writer across projects and does not travel inside a `.skrib`.
///
/// It is deliberately *not* the same thing as the per-language pill checkmarks, which mute one
/// dictionary for the session. Under the union model (a word is wrong only when **every**
/// active dictionary rejects it) unchecking one of several languages changes nothing visible —
/// which is exactly why an unmistakable master switch has to exist beside them.
pub const SPELLCHECK_ENABLED_KEY: &str = "editor.spellcheck";
pub const SPELLCHECK_ENABLED_DEFAULT: bool = true;
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
pub const SCENE_LINE_HEIGHT_DEFAULT: f32 = 1.6;
pub const SCENE_FIRST_LINE_INDENT_KEY: &str = "editor.scene.first_line_indent";
// ≈ one line-height (1.6 × 18 ≈ 29 px) is the classic book indent; 24 px reads as
// a clear paragraph cue without shoving prose too far off the margin.
pub const SCENE_FIRST_LINE_INDENT_DEFAULT: f32 = 24.0;
pub const SCENE_PARA_SPACING_BEFORE_KEY: &str = "editor.scene.para_spacing_before";
pub const SCENE_PARA_SPACING_BEFORE_DEFAULT: f32 = 0.0;
pub const SCENE_PARA_SPACING_AFTER_KEY: &str = "editor.scene.para_spacing_after";
pub const SCENE_PARA_SPACING_AFTER_DEFAULT: f32 = 0.0;

/// Synopsis editor typography (the subordinate summary pane).
pub const SYNOPSIS_FONT_FAMILY_KEY: &str = "editor.synopsis.font_family";
pub const SYNOPSIS_FONT_FAMILY_DEFAULT: &str = "Literata";
pub const SYNOPSIS_SIZE_KEY: &str = "editor.synopsis.size";
// 85 % of the scene face (≈ 15 px against the 18 px anchor) — clearly subordinate
// summary text, not near-parity with the manuscript.
pub const SYNOPSIS_SIZE_DEFAULT: f32 = 0.85;
pub const SYNOPSIS_LINE_HEIGHT_KEY: &str = "editor.synopsis.line_height";
pub const SYNOPSIS_LINE_HEIGHT_DEFAULT: f32 = 1.35;
pub const SYNOPSIS_FIRST_LINE_INDENT_KEY: &str = "editor.synopsis.first_line_indent";
pub const SYNOPSIS_FIRST_LINE_INDENT_DEFAULT: f32 = 0.0;
pub const SYNOPSIS_PARA_SPACING_BEFORE_KEY: &str = "editor.synopsis.para_spacing_before";
pub const SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT: f32 = 0.0;
pub const SYNOPSIS_PARA_SPACING_AFTER_KEY: &str = "editor.synopsis.para_spacing_after";
// Block-style spacing: with no first-line indent, a paragraph must be delimited by
// vertical space or multi-paragraph summaries run together into one wall of text.
pub const SYNOPSIS_PARA_SPACING_AFTER_DEFAULT: f32 = 8.0;

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
// Block-style spacing (notes have no indent) — notes are fragments/lists, so a
// paragraph gap is what a notes surface is expected to look like.
pub const NOTES_PARA_SPACING_AFTER_DEFAULT: f32 = 8.0;

// ── Editor behaviour (Settings ▸ Editor ▸ Editor Behavior) ───────────────────
/// Show the synopsis pane above the manuscript in the dual-pane writing editor
/// (Skribisto's signature layout). Consumed live by the `shared::prose` body.
pub const SYNOPSIS_PANE_KEY: &str = "editor.synopsis_pane";
pub const SYNOPSIS_PANE_DEFAULT: bool = true;
/// Remember, per container item type (Book / Part / Chapter), the last
/// `SegmentedControl` view used — so opening a new chapter lands on the same view
/// (e.g. Full Chapter) as the last chapter. The per-type indices live under
/// `editor.last_view.*` (see [`view_models::EditorViewMemory`]).
pub const REMEMBER_VIEW_KEY: &str = "editor.remember_view";
pub const REMEMBER_VIEW_DEFAULT: bool = true;
/// Keep the caret line vertically centred while typing.
pub const TYPEWRITER_KEY: &str = "editor.typewriter_scroll";
pub const TYPEWRITER_DEFAULT: bool = true;
/// Highlight the sentence the caret is in.
pub const HIGHLIGHT_SENTENCE_KEY: &str = "editor.highlight_sentence";
pub const HIGHLIGHT_SENTENCE_DEFAULT: bool = false;

// ── Goals & word count (Settings ▸ Editor ▸ Goals) ────────────────────────────
/// How words are counted for the live status-bar / focused count: `Auto` (per the
/// scene's language — CJK-smart for zh/ja, Unicode words elsewhere) or a forced
/// method. A global USER preference; it drives only the *displayed* live count,
/// never the canonical progress snapshot (which is always `Auto`). Persisted as
/// the serialized `CountingMethodSetting`; the default is supplied by the VM.
pub const GOALS_COUNTING_METHOD_KEY: &str = "goals.counting_method";
/// Show the character count alongside the word count in the status bar.
pub const GOALS_SHOW_CHARACTERS_KEY: &str = "goals.show_characters";
pub const GOALS_SHOW_CHARACTERS_DEFAULT: bool = false;

// ── Corkboard (Settings ▸ Editor ▸ Corkboard) ─────────────────────────────────
/// Corkboard default mode: `true` = nested (a container's direct children; a
/// folder card drills in), `false` = flat (all descendant leaves at once).
pub const CORKBOARD_NESTED_KEY: &str = "corkboard.nested";
pub const CORKBOARD_NESTED_DEFAULT: bool = true;
/// Corkboard card size — the minimum tile width in px the size slider drives.
pub const CORKBOARD_CARD_SIZE_KEY: &str = "corkboard.card_size";
pub const CORKBOARD_CARD_SIZE_DEFAULT: f32 = 240.0;
/// The card-size slider range (shared by the header slider and the settings pane).
/// A wide span: compact index cards (210) up to near-full-width review cards (680).
pub const CORKBOARD_CARD_SIZE_MIN: f32 = 210.0;
pub const CORKBOARD_CARD_SIZE_MAX: f32 = 680.0;
pub const CORKBOARD_CARD_SIZE_STEP: f32 = 10.0;
/// Show a card's word count in its footer.
pub const CORKBOARD_SHOW_WORD_COUNT_KEY: &str = "corkboard.show_word_count";
pub const CORKBOARD_SHOW_WORD_COUNT_DEFAULT: bool = true;

/// Corkboard card synopsis typography — its own bundle (like Scene / Synopsis /
/// Notes), so the index-card summaries can read distinctly from the manuscript's
/// synopsis pane. Sensible defaults: the same serif as the synopsis, a touch smaller
/// and tighter for a compact card, block-style (no indent, a little space between
/// paragraphs).
pub const CORKBOARD_FONT_FAMILY_KEY: &str = "corkboard.font_family";
pub const CORKBOARD_FONT_FAMILY_DEFAULT: &str = "Literata";
pub const CORKBOARD_SIZE_KEY: &str = "corkboard.size";
pub const CORKBOARD_SIZE_DEFAULT: f32 = 0.8;
pub const CORKBOARD_LINE_HEIGHT_KEY: &str = "corkboard.line_height";
pub const CORKBOARD_LINE_HEIGHT_DEFAULT: f32 = 1.3;
pub const CORKBOARD_FIRST_LINE_INDENT_KEY: &str = "corkboard.first_line_indent";
pub const CORKBOARD_FIRST_LINE_INDENT_DEFAULT: f32 = 0.0;
pub const CORKBOARD_PARA_SPACING_BEFORE_KEY: &str = "corkboard.para_spacing_before";
pub const CORKBOARD_PARA_SPACING_BEFORE_DEFAULT: f32 = 0.0;
pub const CORKBOARD_PARA_SPACING_AFTER_KEY: &str = "corkboard.para_spacing_after";
pub const CORKBOARD_PARA_SPACING_AFTER_DEFAULT: f32 = 6.0;

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
        // The writing-serif design size: this is what a Scene / Synopsis editor's
        // `size` = 100 % resolves to (the `size` setting is a zoom multiplier, so the
        // absolute px is anchored here). 18 px is the comfortable long-form drafting
        // size; a Synopsis pane scales down from it via its 0.85 default.
        default_size_px: 18.0,
    };
    // Bytes come from the shared `skribisto_fonts` crate — the same blobs the PDF exporter
    // feeds to Typst, so the editor and an exported PDF render in the identical face.
    VecFontRegistrar::new(skribisto_fonts::all_faces().into_iter().map(face).collect())
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

    // Optional `.skrib` path to open on launch (`skribisto <path>`); `App` opens it
    // once on first build.
    //
    // Read **before** the window-state prune below, not at the point of use: the
    // prune forgets every `work-*` row whose project it cannot account for, and a
    // path handed to us on argv (a file-manager double-click, `spawn_new_process`)
    // is a project we are about to open *right now* — but it need not be in the
    // recents MRU (it can have aged out of the 12-entry cap) nor in the open
    // registry (nothing has claimed it yet). Pruning first would therefore delete
    // the saved geometry of the very window we are seconds away from restoring.
    let initial_project = std::env::args().nth(1).filter(|s| !s.trim().is_empty());

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
    if let Some(paths) = AppPaths::new("eu", "skribisto", "Skribisto") {
        match WindowStateService::open(&paths) {
            Ok(window_state) => {
                let known_paths = known_project_paths(initial_project.as_deref());
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
    let init_root_id = match handling_app_lifecycle_commands::initialize_app(&app_ctx) {
        Ok(res) => Some(res.root_id),
        Err(e) => {
            eprintln!("initialize_app failed: {e:#}");
            None
        }
    };

    // Read persisted UI prefs before constructing the app (same AppPaths the
    // builder will use via `.application(...)`).
    let (dark, locale_str, autosave_init, spellcheck_init, show_welcome_init) = read_prefs();

    let theme = if dark { intui::dark() } else { intui::light() };

    let i18n = I18nConfig::new()
        .source_locale("en-US".parse().unwrap())
        .supported_locales([
            "en-US".parse().unwrap(),
            "fr-FR".parse().unwrap(),
            "ar".parse().unwrap(),
        ])
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
                ],
            ),
            (
                "fr-FR",
                &[
                    include_str!("../locales/fr-FR/main.ftl"),
                    include_str!("../locales/fr-FR/tooltips.ftl"),
                    include_str!("../locales/fr-FR/tags.ftl"),
                ],
            ),
            // Partial: the keys it omits fall back to en-US per key.
            // Offering it at all is what lets a writer select Arabic, and
            // selecting Arabic is what turns on the framework's RTL chrome
            // mirroring. Only `main.ftl` — see its header for why there is
            // no tooltips/tags file.
            ("ar", &[include_str!("../locales/ar/main.ftl")]),
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
    // Spell-checking (Step 6): the engine is shared by every open document (one
    // `spellbook::Dictionary` per language), and the open-docs store owns the attach
    // loop — so hand the engine to the store, and register it as `app_state` so the
    // language-pill field and the personal-word list reach the same instance (mute set,
    // personal words).
    let spellcheck = spellcheck::SpellcheckService::new();
    open_docs.set_spellcheck(spellcheck.clone());
    // Dictionary management: accepted-licence store (cross-process, like `backup.toml`),
    // on-disk discovery, and the download view-model. App-local (a downloaded `.dic` is a
    // machine-wide resource, not `Work` state) — degrades to a throwaway temp settings
    // file if the config dir is unavailable, exactly as backup settings do.
    let dictionary_settings = bastyde::settings::AppPaths::new("eu", "skribisto", "Skribisto")
        .and_then(|paths| {
            models::DictionarySettingsService::open(&paths)
                .map_err(|e| eprintln!("dictionary settings: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(models::DictionarySettingsService::in_memory_default);
    let installed_dictionaries =
        models::InstalledDictionariesModel::new(dictionary_settings.clone());
    let dictionaries =
        view_models::DictionariesViewModel::new(dictionary_settings, installed_dictionaries);
    // Reactive single-entity handles (Layer A). Created here so the title-bar menu
    // can bind the project title (Bug 1) and shape (Bug 2); `App::build` wires
    // their event subscriptions and re-points them on each `LoadWork`.
    let single_work = SingleWork::new(app_ctx.clone());
    let single_work_info = SingleWorkInfo::new(app_ctx.clone());
    // The outline view-model is created here (no settings dependency) so the
    // title-bar menu can bind its reactive checkmark and the whole app can reach
    // it via `ctx.app_state::<OutlineViewModel>()`.
    let outline = OutlineViewModel::new_default(app_ctx.clone(), ids.clone());
    // Per-work workspace layout (open editor tabs + dock arrangement): opened
    // eagerly here so the restore fires on the first `LoadWork`. App-local config
    // (`workspace.toml`, keyed by `Work.unique_id`), orthogonal to the `.skrib`
    // document — degrades to a throwaway temp file if the config dir is
    // unavailable, exactly as the backup/search settings do. The VM reads the
    // project uid/path from the singles, drives the shared `DockingModel` (via the
    // outline handle), and is handed the editors once `App::build` creates them.
    let workspace_layout_service = bastyde::settings::AppPaths::new("eu", "skribisto", "Skribisto")
        .and_then(|paths| {
            WorkspaceLayoutService::open(&paths)
                .map_err(|e| eprintln!("workspace layout: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(WorkspaceLayoutService::in_memory_default);
    // Remembered Overview expand state, keyed per project + per container by durable uid
    // (`tree_expansion.toml`). A fourth `SettingsFile` sibling; on failure the feature
    // simply goes quiet rather than blocking startup, exactly as the layout service does.
    let tree_expansion_service = bastyde::settings::AppPaths::new("eu", "skribisto", "Skribisto")
        .and_then(|paths| {
            TreeExpansionService::open(&paths)
                .map_err(|e| eprintln!("tree expansion: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(TreeExpansionService::in_memory_default);
    // The progress recorder (writing-cadence): on each save it recounts the
    // project's words and records a daily `ProgressSnapshot` feeding the Pace
    // charts. Registered as app-state so `App::build` can route the save +
    // `count_words` long-operation events to it (and a future manual "recount").
    let progress_recorder = ProgressRecorder::new(app_ctx.clone(), ids.clone());
    // One instance app-wide: every roster and backlink list resolves through it, so a
    // second would mean a second scan and two answers.
    let mention_index = view_models::MentionIndex::new(app_ctx.clone(), ids.clone());
    // The Import-Plume view-model is a singleton (form + in-flight job + progress
    // toast). Registered as app-state so `App::build` can route the import's
    // long-operation events to it and the menu action can reach it to open the panel.
    let import_plume = ImportPlumeViewModel::new(app_ctx.clone());
    // The Export view-model is a singleton: it owns the focus-adaptive scope list (driving
    // both the title-bar Export split-button and the File ▸ Export submenu), the panel's
    // state, and the in-flight export job. Registered as app-state so the title-bar chrome
    // (built outside `App`) and `App::build`'s wiring reach the one instance.
    let export = ExportViewModel::new(app_ctx.clone(), ids.clone());
    // Export styles ("Compile & Export" formats) — the user's editable style presets, opened
    // eagerly here so the Settings pane and the Export panel's picker both read one instance.
    // App-local config (a style outlives any project); degrades to a throwaway temp file if the
    // config dir is unavailable, exactly as backup settings do.
    let export_styles_service = bastyde::settings::AppPaths::new("eu", "skribisto", "Skribisto")
        .and_then(|paths| {
            models::ExportStylesService::open(&paths)
                .map_err(|e| eprintln!("export styles: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(models::ExportStylesService::in_memory_default);
    let export_styles = view_models::ExportStylesViewModel::new(export_styles_service);
    // Personal-dictionary feature (per-project `DictWord`): a reactive list model
    // + a single (for inline rename), composed by the view-model. Registered as
    // app-state so the Settings pane and the editor's "Add to dictionary" global
    // action reach the one instance; `App::build` wires its subscriptions.
    let dict_words = models::DictWordListModel::new(app_ctx.clone());
    let single_dict_word = SingleDictWord::new(app_ctx.clone());
    let user_dictionary =
        view_models::UserDictionaryViewModel::new(dict_words, single_dict_word, ids.clone());
    // The tag palette + its view-model, on the same footing: registered as app-state so the
    // Inspector's tag section, the Settings pane and (from Stage 4) the chip popover all
    // reach the ONE instance. Sharing matters more here than for most view-models — every
    // chip in the app resolves its colour through this model's lookup signal, so a second
    // instance would mean a second subscription set and two palettes drifting apart.
    let work_tags = models::WorkTagsListModel::new(app_ctx.clone());
    let tags_vm = view_models::TagsViewModel::new(work_tags, ids.clone());
    // Backup-mode state: `backup_mode` is true while a *backup file* is open in
    // this window (Save + auto-backup off; the file is read-only, the content is
    // still editable). `backup_context` carries the open backup's details (drives
    // the permanent banner + restore). Created here so the title-bar menu can read
    // `backup_mode` (to hide Save / Back up now); `App::build` sets them on load.
    let backup_mode = Signal::new(false);
    let backup_context: Signal<Option<backup::BackupContext>> = Signal::new(None);
    // Per-work workspace layout view-model (built here, once `backup_mode` exists —
    // capture is inert in backup mode). It drives the shared `DockingModel` (via the
    // outline handle) and is handed the editors once `App::build` creates them.
    let tree_expansion =
        TreeExpansionViewModel::new(app_ctx.clone(), ids.clone(), tree_expansion_service);
    let workspace_layout = WorkspaceLayoutViewModel::new(
        app_ctx.clone(),
        workspace_layout_service,
        outline.docking(),
        single_work.clone(),
        single_work_info.clone(),
        ids.clone(),
        backup_mode.clone(),
    );
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
    let restore_vm = BackupRestoreViewModel::new(
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
    // Same trick for the master spell-check switch: the title-bar toggle + View menu live
    // outside `App`, so `App::build` mirrors the persisted key into this plain signal.
    // Seeded from the store so a launch with it off never flashes the "on" icon.
    let spellcheck_menu = Signal::new(spellcheck_init);
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
    // Created here, not in `App`: the menu bar is built alongside the app widget
    // rather than inside it, so the Format menu needs these signals before
    // `EditorsViewModel` exists. `App::build` attaches the editors on every
    // build — the same shape as the workspace-layout view-model.
    let format_vm = crate::view_models::FormatViewModel::detached();
    let project_factory = windows::ProjectWindowFactory::new(
        app_ctx.clone(),
        outline.clone(),
        export.clone(),
        single_work.clone(),
        single_work_info.clone(),
        autosave_menu.clone(),
        spellcheck_menu.clone(),
        save_as_vm.clone(),
        backup_mode.clone(),
        backup_context.clone(),
        unsaved.clone(),
        pending_exit.clone(),
        backup_scheduler.clone(),
        main_window_state.clone(),
        format_vm.clone(),
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
        // Writing-model rich tooltips (the "＋ Create" / "Convert to" explainers).
        // Registered here so their `[label](:key)` bodies cascade to one another.
        .register_tooltips(tooltip_registry::writing_model_tooltips())
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
        .app_state(spellcheck.clone())
        .app_state(dictionaries.clone())
        .app_state(tags_vm.clone())
        .app_state(format_vm.clone())
        .app_state(single_work.clone())
        .app_state(single_work_info.clone())
        .app_state(outline.clone())
        .app_state(workspace_layout.clone())
        .app_state(tree_expansion.clone())
        .app_state(mention_index.clone())
        .app_state(progress_recorder.clone())
        .app_state(import_plume.clone())
        .app_state(export.clone())
        .app_state(export_styles.clone())
        .app_state(user_dictionary.clone())
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
    crate::shell::ipc::cleanup_own_socket();
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
fn read_prefs() -> (bool, String, bool, bool, bool) {
    let Some(paths) = AppPaths::new("eu", "skribisto", "Skribisto") else {
        return (
            false,
            "en-US".to_string(),
            false,
            SPELLCHECK_ENABLED_DEFAULT,
            true,
        );
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
            store
                .signal(SPELLCHECK_ENABLED_KEY, SPELLCHECK_ENABLED_DEFAULT)
                .get(),
            store.signal(SHOW_WELCOME_KEY, true).get(),
        ),
        Err(_) => (
            false,
            "en-US".to_string(),
            false,
            SPELLCHECK_ENABLED_DEFAULT,
            true,
        ),
    }
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
///    12-entry MRU is in neither (1) nor (2), yet we are about to open it. Prune
///    without it and we delete the saved geometry of the very window we are
///    seconds away from restoring.
fn known_project_paths(initial_project: Option<&str>) -> Vec<String> {
    let mut known = models::RecentWorkListModel::all_raw_paths();
    known.extend(open_registry::scan().into_iter().map(|e| e.path));
    known.extend(initial_project.map(str::to_string));
    known
}

fn prune_orphaned_window_state(window_state: &WindowStateService, known_paths: &[String]) {
    let known_labels: std::collections::HashSet<String> = known_paths
        .iter()
        .map(|p| windows::window_id_for(p))
        .collect();
    for label in window_state.labels() {
        if !label.starts_with("work-") || known_labels.contains(&label) {
            continue;
        }
        if let Err(e) = window_state.forget(&label) {
            eprintln!("skribisto: could not forget stale window state '{label}': {e}");
        }
    }
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

        prune_orphaned_window_state(&window_state, &[known_path]);

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

    /// The argv project must count as "known" even when it is in neither the
    /// recents MRU nor the open registry.
    ///
    /// The bug this pins: the startup prune ran *before* argv was read, so a
    /// file-manager double-click on a project that had aged out of the 12-entry
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

        let without = known_project_paths(None);
        assert!(
            !without.iter().any(|p| p == argv),
            "precondition: this path is in neither recents nor the open registry"
        );

        let with = known_project_paths(Some(argv));
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

        prune_orphaned_window_state(&window_state, &[known_path]);

        assert_eq!(window_state.labels(), vec![known_label]);

        let _ = std::fs::remove_file(&path);
    }
}
