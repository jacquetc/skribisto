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

// A deliberate, blunt instrument — and worth knowing exactly what it hides.
//
// This is a **binary** crate, so `pub` shields nothing and every item the views
// have not wired up yet reads as dead. Two thirds of what this silences is
// view-model surface built ahead of the view that will consume it (the idiom
// `singles.rs` already spells out per-impl as "public reactive surface; wired to
// consumers incrementally"), and much of the rest is live only under
// `--features mocks` — clippy lints each `#[cfg]` arm on its own, so a helper the
// mock models use is dead in the default arm and vice versa. **Deleting those
// would break the other feature set's build**, which is why this is an allow
// rather than a cleanup.
//
// The cost is real: genuine rot in this crate now goes unreported. The honest
// follow-up is a triage pass that deletes what is vestigial and annotates the
// rest per item with its reason, after which this line should come back out.
// It is here because CI's `-D warnings` gate cannot go green without it, and a
// gate that has never once been green teaches nobody anything.
#![allow(dead_code)]

mod a11y;
mod app;
mod app_ids;
mod backup;
mod binder;
mod date_convert;
mod docks;
mod export;
mod icons;
mod intents;
mod models;
mod panels;
mod sessions;
mod settings;
mod shell;
mod singles;
mod spellcheck;
mod statusbar;
mod tabs;
mod tags;
mod text_replacement;
mod toast_scope;
mod trash;
mod widgets;
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
use models::{BackupSettingsService, TreeExpansionService, WorkspaceLayoutService};
use sessions::{WorkRegistry, WorkSession};
use view_models::{
    BackupSettingsViewModel, ImportPlumeViewModel, OutlineViewModel, ProjectSwitchViewModel,
};

/// The currently-open project's path (from its `WorkInfo`), if any.
///
/// Resolved through the Phase-1 seam — `ids.work_info_id`, the id `WorkSession`
/// carries for exactly this project — rather than `get_all_work_info(ctx)`'s
/// first entry: the backend's `WorkInfo` list is no longer guaranteed to hold
/// only this window's project (Phase 0 scoped `Root.works`/`System.work_infos`
/// to support more than one open `Work`), so "whichever the store returns
/// first" stopped being a safe stand-in for "the one this window means" the
/// moment that scoping landed, even though today's single-Work-open sweep
/// still makes the two answers coincide in practice.
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

// ── Smart punctuation, application-level ────────────────────────────────────
//
// The default tier. A project that has not taken the override in
// Work ▸ Punctuation follows these, which is what `override_app_default: false`
// on its row means.
//
// Dashes, the ellipsis and curled quotes default **on**: that is what "smart
// punctuation" means to a writer, it is what Word, LibreOffice and Scrivener all
// do out of the box, and every one of them is reversible with a single Ctrl+Z on
// the keystroke that fired it.
//
// Pre-punctuation spacing defaults **off** even though it is equally correct for
// French. It inserts an *invisible* character, so a writer who has not asked for
// it would see their file change in ways they cannot see on screen — and unlike
// the others it applies to one language only.
pub const PUNCT_DASHES_KEY: &str = "editor.punctuation.dashes";
pub const PUNCT_DASHES_DEFAULT: bool = true;
pub const PUNCT_ELLIPSIS_KEY: &str = "editor.punctuation.ellipsis";
pub const PUNCT_ELLIPSIS_DEFAULT: bool = true;
pub const PUNCT_QUOTES_KEY: &str = "editor.punctuation.quotes";
pub const PUNCT_QUOTES_DEFAULT: bool = true;
pub const PUNCT_QUOTE_STYLE_KEY: &str = "editor.punctuation.quote_style";
pub const PUNCT_SPACING_KEY: &str = "editor.punctuation.pre_punctuation_spacing";
pub const PUNCT_SPACING_DEFAULT: bool = false;
pub const PUNCT_DIALOGUE_KEY: &str = "editor.punctuation.dialogue_marker";
/// Off, like the spacing rule: it rewrites the *shape* of a line rather than one
/// glyph inside it, and it is wrong outright in the languages that quote their
/// dialogue instead of dashing it.
pub const PUNCT_DIALOGUE_DEFAULT: bool = false;
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
        .supported_locales(["en-US".parse().unwrap(), "fr-FR".parse().unwrap()])
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
        ])
        .user_locale(locale_str.parse().ok())
        .auto_detect_os_locale(false)
        .fallback_locale("en-US".parse().unwrap())
        .framework_locales(framework_locales());

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
    // The Import-Plume view-model is a singleton (form + in-flight job + progress
    // toast). Registered as app-state so `App::build` can route the import's
    // long-operation events to it and the menu action can reach it to open the panel.
    let import_plume = ImportPlumeViewModel::new(app_ctx.clone());
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
    let backup_service = bastyde::settings::AppPaths::new("eu", "skribisto", "Skribisto")
        .and_then(|paths| {
            BackupSettingsService::open(&paths)
                .map_err(|e| eprintln!("backup settings: open failed: {e}"))
                .ok()
        })
        .unwrap_or_else(BackupSettingsService::in_memory_default);
    let backup_settings = BackupSettingsViewModel::new(backup_service);

    // The title-bar menu lives outside `App` (no `ctx.settings()` there), so the
    // autosave setting is mirrored into this plain signal by `App::build` and read
    // by the menu to hide the "Save" item. Seeded from the persisted value.
    let autosave_menu = Signal::new(autosave_init);
    // Same trick for the master spell-check switch: the title-bar toggle + View menu live
    // outside `App`, so `App::build` mirrors the persisted key into this plain signal.
    // Seeded from the store so a launch with it off never flashes the "on" icon.
    let spellcheck_menu = Signal::new(spellcheck_init);
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
        registry.clone(),
        spellcheck.clone(),
        backup_settings.clone(),
        workspace_layout_service,
        tree_expansion_service,
        autosave_menu.clone(),
        spellcheck_menu.clone(),
        main_window_state.clone(),
        format_vm.clone(),
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
    let (initial_window_config, initial_state) = if let Some(path) = initial_project.clone() {
        // Launch with a path on argv (file manager, CLI, `spawn_new_process`):
        // always skip the Launcher.
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
                let (config, state) = project_factory
                    .window_config(app::PendingAction::Load(recent.absolute_path));
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
                bastyde::widgets::DockingModel::new(),
                backup_settings.clone(),
                WorkspaceLayoutService::in_memory_default(),
                TreeExpansionService::in_memory_default(),
            ),
        }
    });

    // The switch guard (see the doc where `autosave_menu` is constructed
    // above) — built here, now that `initial_state` exists, so its
    // `unsaved`/`backup_mode` handles are a real Work's (Scope E/Phase 3:
    // `WorkSession::new` mints a fresh pair per Work; there is no
    // process-wide one any more). `ProjectSwitchViewModel` itself is still a
    // single, Tier-1 shared instance — this only fixes *which* Work's
    // `unsaved`/`backup_mode` it reads, the same way every other
    // Tier-2-via-`app_state` registration below reads off the first window's
    // session.
    let project_switch = ProjectSwitchViewModel::new(
        app_ctx.clone(),
        initial_state.session.unsaved.clone(),
        initial_state.session.backup_mode.clone(),
        autosave_menu.clone(),
    );

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
        .app_state(registry.clone())
        // The remaining `app_state` registrations below all come from
        // `initial_state` — the fresh, per-window Tier-2 bundle the *first*
        // window's own `window_config` call minted (see its doc, and
        // `sessions::WorkSession`'s module doc). This is a known, flagged
        // Phase-2 gap, not a full fix: a handful of widgets/panes
        // (`tags::tag_chip`, `view_models::overview`'s tree-expansion restore,
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
        .app_state(format_vm.clone())
        .app_state(initial_state.session.single_work.clone())
        .app_state(initial_state.session.single_work_info.clone())
        .app_state(initial_state.outline.clone())
        .app_state(initial_state.session.workspace_layout.clone())
        .app_state(initial_state.session.tree_expansion.clone())
        .app_state(initial_state.session.mention_index.clone())
        .app_state(initial_state.session.progress_recorder.clone())
        .app_state(import_plume.clone())
        .app_state(export_styles.clone())
        .app_state(initial_state.session.user_dictionary.clone())
        .app_state(backup_settings.clone())
        .app_state(initial_state.session.backup_scheduler.clone())
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
