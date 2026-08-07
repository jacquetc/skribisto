// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The schema for the app-level scalar settings — every key `general.toml` can hold.
//!
//! `SettingsStore` is a dynamic dotted-key TOML store: a key springs into existence the
//! first time somebody calls `store.signal(key, default)`, and nothing anywhere knows the
//! set of legal keys. That is fine for the app (it only ever asks for keys it declares)
//! and actively hostile to anyone *writing* a settings file by hand — an agent driving a
//! probe, most of all. A typo is not an error; it is silence: the app boots on defaults and
//! the probe asserts against a state it never reached, several layers from the real cause
//! (see `automation_fixture::isolated_config`'s docstring for a worked example, over
//! `ui.locale`).
//!
//! So this module states the schema once: for each key its type, its default, a
//! validator, and a line of prose. That buys three things which do not otherwise exist:
//!
//! * **`--config <file.toml>`** ([`load_pins`] + [`merge_into`]) — pin a known set of
//!   options for one run, with an unknown or mistyped key a hard startup error naming the
//!   key and its nearest legal neighbour, rather than a silent no-op.
//! * **`--dump-config`** ([`dump`]) — print the effective configuration, self-documenting,
//!   in exactly the syntax a pins file wants. "What can I pin, and what is it now?"
//!   answered without reading source.
//! * **A drift test** (`tests::every_declared_key_is_registered`) — the registry is
//!   hand-written, so it can rot. The test walks the crate's own sources for
//!   `*_KEY: &str` declarations and fails if one is missing here.
//!
//! ## Scope: this is `general.toml`, and only `general.toml`
//!
//! The `SettingsFile<T>` siblings (`workspace.toml`, `backup.toml`, `search.toml`,
//! `tree_expansion.toml`, the dictionary/export-style/distraction-free-theme services) are
//! versioned, migratable structs, not scalars — they are deliberately out of scope, and a
//! pins file naming one gets the same unknown-key error as a typo. So is everything that
//! lives on a `Work` (author, language, per-project punctuation, tags): that is manuscript
//! data inside the `.skrib`, not app configuration. A fresh config sandbox gives all of
//! those their defaults, which is what a probe usually wants anyway.

use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;

use frontend::common::entities::QuoteStyle;
use skribisto_model::counting::CountingMethodSetting;

use crate::view_models::{HighlightScope, TypewriterAnchor};

/// One settable key: what it is called, what it holds, and what it means.
pub struct SettingSpec {
    /// The dotted TOML path, exactly as it appears in `general.toml`.
    pub key: &'static str,
    /// Human-readable type, shown in error messages and in [`dump`]'s comments.
    /// Prose rather than a Rust type name — `"one of: Auto | Whitespace | …"` tells a
    /// caller what to write; `CountingMethodSetting` does not.
    pub ty: &'static str,
    /// The value the app uses when the key is absent. Must equal the default passed to
    /// the matching `store.signal(key, default)` call.
    ///
    /// Hand-maintained, like the use-case/UoW macro lists in the backend — no test can
    /// check the pairing, since the real defaults live at the `signal` call sites as typed
    /// Rust values a test would need a full `SettingsStore` to reach. A wrong default here
    /// only makes [`dump`] misreport an unset key; the drift test still catches the case
    /// that matters, a key with no spec at all.
    pub default: fn() -> toml::Value,
    /// Rejects a value of the wrong shape, by running the real deserialization
    /// `SettingsStore::signal` would run (which otherwise falls back to `default`
    /// *silently* on a type mismatch, so a bad value would simply never arrive). Validating
    /// through the same call keeps this in lockstep with the store's own coercions — e.g. a
    /// bare `1` is accepted for an `f32` key because the store accepts it too.
    pub check: fn(&toml::Value) -> Result<(), String>,
    /// One line on what the setting does, for [`dump`].
    pub doc: &'static str,
}

/// Serialize a default into its TOML form.
///
/// Panics only on a type that cannot be represented in TOML at all (e.g. a bare `None`),
/// which would be a bug in this table and is caught by `tests::defaults_are_valid`.
fn val<T: Serialize>(v: T) -> toml::Value {
    toml::Value::try_from(v).expect("a settings default must be TOML-representable")
}

/// Validate by round-tripping through the very deserialization the settings store runs.
fn check<T: DeserializeOwned>(v: &toml::Value) -> Result<(), String> {
    match v.clone().try_into::<T>() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.message().to_string()),
    }
}

/// Every key `general.toml` may hold.
///
/// Grouped as the settings panes group them. Order is the dump order, so it should read
/// top-down like a settings window rather than alphabetically.
pub static SETTINGS: &[SettingSpec] = &[
    // ── Appearance & behaviour ────────────────────────────────────────────────
    SettingSpec {
        key: crate::IMAGE_SIZE_POLICY_KEY,
        ty: "string (\"ask\" | \"keep\" | \"downscale\")",
        default: || val("ask"),
        check: check::<String>,
        doc: "What to do with a large image on insert. \"ask\" prompts once per \
              image; the prompt's \"don't ask again\" box writes \"keep\" or \
              \"downscale\" here.",
    },
    SettingSpec {
        key: crate::DARK_KEY,
        ty: "bool",
        default: || val(false),
        check: check::<bool>,
        doc: "Dark theme.",
    },
    SettingSpec {
        key: crate::LOCALE_KEY,
        ty: "string (BCP-47: \"en-US\" | \"fr-FR\")",
        default: || val("en-US"),
        check: check::<String>,
        doc: "Interface language. The OS locale is never consulted, so a probe asserting \
              on translated text must set this.",
    },
    SettingSpec {
        key: crate::SHOW_WELCOME_KEY,
        ty: "bool",
        default: || val(true),
        check: check::<bool>,
        doc: "Open the Launcher on a bare launch; when false, reopen the most recent \
              reachable project instead.",
    },
    SettingSpec {
        key: teksilo::settings::TEXT_SCALE_KEY.key,
        ty: "float (1.0 = 100 %)",
        default: || val((teksilo::settings::TEXT_SCALE_KEY.default)()),
        check: check::<f32>,
        doc: "Interface text scale. Teksilo's own key, read before the first window opens.",
    },
    // ── Editor: layout & saving ───────────────────────────────────────────────
    SettingSpec {
        key: crate::EDITOR_WIDTH_KEY,
        ty: "float (px)",
        default: || val(crate::EDITOR_WIDTH_DEFAULT),
        check: check::<f32>,
        doc: "Max width of the centered manuscript writing column.",
    },
    SettingSpec {
        key: crate::PREVIEW_WIDTH_KEY,
        ty: "float (px)",
        default: || val(crate::PREVIEW_WIDTH_DEFAULT),
        check: check::<f32>,
        doc: "Max width of the search preview editor in the bottom band.",
    },
    SettingSpec {
        key: crate::AUTOSAVE_KEY,
        ty: "bool",
        default: || val(false),
        check: check::<bool>,
        doc: "Autosave to disk, hiding the manual Save / Ctrl+S affordances.",
    },
    SettingSpec {
        key: crate::SPELLCHECK_ENABLED_KEY,
        ty: "bool",
        default: || val(crate::SPELLCHECK_ENABLED_DEFAULT),
        check: check::<bool>,
        doc: "The master spell-check switch.",
    },
    SettingSpec {
        key: crate::COMMENTS_VISIBLE_KEY,
        ty: "bool",
        default: || val(crate::COMMENTS_VISIBLE_DEFAULT),
        check: check::<bool>,
        doc: "Show anchored comments in the editor (the marks and the margin).",
    },
    SettingSpec {
        key: crate::SYNOPSIS_PANE_KEY,
        ty: "bool",
        default: || val(crate::SYNOPSIS_PANE_DEFAULT),
        check: check::<bool>,
        doc: "Show the synopsis pane above the manuscript in the dual-pane editor.",
    },
    SettingSpec {
        key: crate::SYNOPSIS_PLACEMENT_KEY,
        ty: "one of: Top | Side",
        default: || val(crate::view_models::SynopsisPlacement::default()),
        check: check::<crate::view_models::SynopsisPlacement>,
        doc: "Where the synopsis sits: Top (above the manuscript) or Side (beside it). \
              Side needs room for two columns — a tab too narrow for both falls back to Top.",
    },
    SettingSpec {
        key: crate::SYNOPSIS_SIDE_WIDTH_KEY,
        ty: "float (px, 180–560)",
        default: || val(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
        check: check::<f32>,
        doc: "Width of the Side synopsis column. Updated by dragging its divider; the \
              range is enforced by the app, not by this schema.",
    },
    SettingSpec {
        key: crate::REMEMBER_VIEW_KEY,
        ty: "bool",
        default: || val(crate::REMEMBER_VIEW_DEFAULT),
        check: check::<bool>,
        doc: "Reopen a container tab on the same segment the last one of its type used.",
    },
    SettingSpec {
        key: "editor.last_view.book",
        ty: "integer (segment index)",
        default: || val(0),
        check: check::<usize>,
        doc: "Remembered segment index for Book tabs (written by the app; see \
              editor.remember_view).",
    },
    SettingSpec {
        key: "editor.last_view.part",
        ty: "integer (segment index)",
        default: || val(0),
        check: check::<usize>,
        doc: "Remembered segment index for Part tabs.",
    },
    SettingSpec {
        key: "editor.last_view.chapter",
        ty: "integer (segment index)",
        default: || val(0),
        check: check::<usize>,
        doc: "Remembered segment index for Chapter tabs.",
    },
    SettingSpec {
        key: crate::TYPEWRITER_KEY,
        ty: "bool",
        default: || val(crate::TYPEWRITER_DEFAULT),
        check: check::<bool>,
        doc: "Typewriter scrolling: hold the caret's line at a fixed height.",
    },
    SettingSpec {
        key: crate::TYPEWRITER_ANCHOR_KEY,
        ty: "one of: Middle | TopThird | BottomQuarter",
        default: || val(Some(TypewriterAnchor::default())),
        check: check::<Option<TypewriterAnchor>>,
        doc: "Which height the typewriter-pinned line sits at.",
    },
    SettingSpec {
        key: crate::HIGHLIGHT_SCOPE_KEY,
        ty: "one of: None | Sentence | Paragraph",
        default: || val(HighlightScope::default()),
        check: check::<HighlightScope>,
        doc: "How much text around the caret gets an ambient highlight band.",
    },
    // ── Editor: smart punctuation (the app-level tier) ────────────────────────
    SettingSpec {
        key: crate::PUNCT_DASHES_KEY,
        ty: "bool",
        default: || val(crate::PUNCT_DASHES_DEFAULT),
        check: check::<bool>,
        doc: "Convert hyphen runs to en/em dashes while typing.",
    },
    SettingSpec {
        key: crate::PUNCT_ELLIPSIS_KEY,
        ty: "bool",
        default: || val(crate::PUNCT_ELLIPSIS_DEFAULT),
        check: check::<bool>,
        doc: "Convert three dots to an ellipsis while typing.",
    },
    SettingSpec {
        key: crate::PUNCT_QUOTES_KEY,
        ty: "bool",
        default: || val(crate::PUNCT_QUOTES_DEFAULT),
        check: check::<bool>,
        doc: "Curl straight quotes while typing.",
    },
    SettingSpec {
        key: crate::PUNCT_QUOTE_STYLE_KEY,
        ty: "one of: LocaleDefault | CurlyDouble | Guillemets | LowHigh",
        default: || val(QuoteStyle::default()),
        check: check::<QuoteStyle>,
        doc: "Which quotation marks the curling rule produces.",
    },
    SettingSpec {
        key: crate::PUNCT_SPACING_KEY,
        ty: "bool",
        default: || val(crate::PUNCT_SPACING_DEFAULT),
        check: check::<bool>,
        doc: "Insert the French narrow no-break space before ; : ! ? and inside guillemets.",
    },
    SettingSpec {
        key: crate::PUNCT_DIALOGUE_KEY,
        ty: "bool",
        default: || val(crate::PUNCT_DIALOGUE_DEFAULT),
        check: check::<bool>,
        doc: "Rewrite a line-opening hyphen as a dialogue em dash.",
    },
    // ── Editor typography: Scene ──────────────────────────────────────────────
    SettingSpec {
        key: crate::SCENE_FONT_FAMILY_KEY,
        ty: "string (a font family the typesetter can resolve)",
        default: || val(crate::SCENE_FONT_FAMILY_DEFAULT),
        check: check::<String>,
        doc: "Manuscript body typeface.",
    },
    SettingSpec {
        key: crate::SCENE_SIZE_KEY,
        ty: "float (1.0 = 100 %)",
        default: || val(crate::SCENE_SIZE_DEFAULT),
        check: check::<f32>,
        doc: "Manuscript body font-size scale.",
    },
    SettingSpec {
        key: crate::SCENE_LINE_HEIGHT_KEY,
        ty: "float (multiple of font size)",
        default: || val(crate::SCENE_LINE_HEIGHT_DEFAULT),
        check: check::<f32>,
        doc: "Manuscript body line height.",
    },
    SettingSpec {
        key: crate::SCENE_FIRST_LINE_INDENT_KEY,
        ty: "float (px)",
        default: || val(crate::SCENE_FIRST_LINE_INDENT_DEFAULT),
        check: check::<f32>,
        doc: "Manuscript body first-line indent.",
    },
    SettingSpec {
        key: crate::SCENE_PARA_SPACING_BEFORE_KEY,
        ty: "float (px)",
        default: || val(crate::SCENE_PARA_SPACING_BEFORE_DEFAULT),
        check: check::<f32>,
        doc: "Manuscript body space above each paragraph.",
    },
    SettingSpec {
        key: crate::SCENE_PARA_SPACING_AFTER_KEY,
        ty: "float (px)",
        default: || val(crate::SCENE_PARA_SPACING_AFTER_DEFAULT),
        check: check::<f32>,
        doc: "Manuscript body space below each paragraph.",
    },
    // ── Editor typography: Synopsis ───────────────────────────────────────────
    SettingSpec {
        key: crate::SYNOPSIS_FONT_FAMILY_KEY,
        ty: "string (a font family the typesetter can resolve)",
        default: || val(crate::SYNOPSIS_FONT_FAMILY_DEFAULT),
        check: check::<String>,
        doc: "Synopsis pane typeface.",
    },
    SettingSpec {
        key: crate::SYNOPSIS_SIZE_KEY,
        ty: "float (1.0 = 100 %)",
        default: || val(crate::SYNOPSIS_SIZE_DEFAULT),
        check: check::<f32>,
        doc: "Synopsis pane font-size scale.",
    },
    SettingSpec {
        key: crate::SYNOPSIS_LINE_HEIGHT_KEY,
        ty: "float (multiple of font size)",
        default: || val(crate::SYNOPSIS_LINE_HEIGHT_DEFAULT),
        check: check::<f32>,
        doc: "Synopsis pane line height.",
    },
    SettingSpec {
        key: crate::SYNOPSIS_FIRST_LINE_INDENT_KEY,
        ty: "float (px)",
        default: || val(crate::SYNOPSIS_FIRST_LINE_INDENT_DEFAULT),
        check: check::<f32>,
        doc: "Synopsis pane first-line indent.",
    },
    SettingSpec {
        key: crate::SYNOPSIS_PARA_SPACING_BEFORE_KEY,
        ty: "float (px)",
        default: || val(crate::SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT),
        check: check::<f32>,
        doc: "Synopsis pane space above each paragraph.",
    },
    SettingSpec {
        key: crate::SYNOPSIS_PARA_SPACING_AFTER_KEY,
        ty: "float (px)",
        default: || val(crate::SYNOPSIS_PARA_SPACING_AFTER_DEFAULT),
        check: check::<f32>,
        doc: "Synopsis pane space below each paragraph.",
    },
    // ── Editor typography: Notes ──────────────────────────────────────────────
    SettingSpec {
        key: crate::NOTES_FONT_FAMILY_KEY,
        ty: "string (a font family the typesetter can resolve)",
        default: || val(crate::NOTES_FONT_FAMILY_DEFAULT),
        check: check::<String>,
        doc: "Notes editor typeface.",
    },
    SettingSpec {
        key: crate::NOTES_SIZE_KEY,
        ty: "float (1.0 = 100 %)",
        default: || val(crate::NOTES_SIZE_DEFAULT),
        check: check::<f32>,
        doc: "Notes editor font-size scale.",
    },
    SettingSpec {
        key: crate::NOTES_LINE_HEIGHT_KEY,
        ty: "float (multiple of font size)",
        default: || val(crate::NOTES_LINE_HEIGHT_DEFAULT),
        check: check::<f32>,
        doc: "Notes editor line height.",
    },
    SettingSpec {
        key: crate::NOTES_FIRST_LINE_INDENT_KEY,
        ty: "float (px)",
        default: || val(crate::NOTES_FIRST_LINE_INDENT_DEFAULT),
        check: check::<f32>,
        doc: "Notes editor first-line indent.",
    },
    SettingSpec {
        key: crate::NOTES_PARA_SPACING_BEFORE_KEY,
        ty: "float (px)",
        default: || val(crate::NOTES_PARA_SPACING_BEFORE_DEFAULT),
        check: check::<f32>,
        doc: "Notes editor space above each paragraph.",
    },
    SettingSpec {
        key: crate::NOTES_PARA_SPACING_AFTER_KEY,
        ty: "float (px)",
        default: || val(crate::NOTES_PARA_SPACING_AFTER_DEFAULT),
        check: check::<f32>,
        doc: "Notes editor space below each paragraph.",
    },
    // ── Distraction-free mode ─────────────────────────────────────────────────
    SettingSpec {
        key: crate::DISTRACTION_FREE_WIDTH_KEY,
        ty: "float (px)",
        default: || val(crate::DISTRACTION_FREE_WIDTH_DEFAULT),
        check: check::<f32>,
        doc: "Max width of the writing column in distraction-free mode.",
    },
    SettingSpec {
        key: crate::DISTRACTION_FREE_THEME_KEY,
        ty: "string (a distraction-free theme id)",
        default: || val(crate::DISTRACTION_FREE_THEME_DEFAULT),
        check: check::<String>,
        doc: "Distraction-free surface theme; an unknown id falls back to the first built-in.",
    },
    SettingSpec {
        key: crate::DISTRACTION_FREE_TITLE_KEY,
        ty: "bool",
        default: || val(crate::DISTRACTION_FREE_TITLE_DEFAULT),
        check: check::<bool>,
        doc: "Keep the item name in the distraction-free strip.",
    },
    SettingSpec {
        key: crate::DISTRACTION_FREE_WORD_COUNT_KEY,
        ty: "bool",
        default: || val(crate::DISTRACTION_FREE_WORD_COUNT_DEFAULT),
        check: check::<bool>,
        doc: "Keep the word-count readout in the distraction-free strip.",
    },
    SettingSpec {
        key: crate::DISTRACTION_FREE_SESSION_KEY,
        ty: "bool",
        default: || val(crate::DISTRACTION_FREE_SESSION_DEFAULT),
        check: check::<bool>,
        doc: "Keep the writing-session readout in the distraction-free strip.",
    },
    SettingSpec {
        key: crate::DISTRACTION_FREE_GO_KEY,
        ty: "bool",
        default: || val(crate::DISTRACTION_FREE_GO_DEFAULT),
        check: check::<bool>,
        doc: "Keep the Previous/Next pair in the distraction-free strip.",
    },
    SettingSpec {
        key: crate::DISTRACTION_FREE_GO_TO_KEY,
        ty: "bool",
        default: || val(crate::DISTRACTION_FREE_GO_TO_DEFAULT),
        check: check::<bool>,
        doc: "Keep the \"Go to…\" jump button in the distraction-free strip.",
    },
    SettingSpec {
        key: crate::DISTRACTION_FREE_FONT_FAMILY_KEY,
        ty: "string (a font family the typesetter can resolve)",
        default: || val(crate::DISTRACTION_FREE_FONT_FAMILY_DEFAULT),
        check: check::<String>,
        doc: "Distraction-free typeface.",
    },
    SettingSpec {
        key: crate::DISTRACTION_FREE_SIZE_KEY,
        ty: "float (1.0 = 100 %)",
        default: || val(crate::DISTRACTION_FREE_SIZE_DEFAULT),
        check: check::<f32>,
        doc: "Distraction-free font-size scale.",
    },
    SettingSpec {
        key: crate::DISTRACTION_FREE_LINE_HEIGHT_KEY,
        ty: "float (multiple of font size)",
        default: || val(crate::DISTRACTION_FREE_LINE_HEIGHT_DEFAULT),
        check: check::<f32>,
        doc: "Distraction-free line height.",
    },
    SettingSpec {
        key: crate::DISTRACTION_FREE_FIRST_LINE_INDENT_KEY,
        ty: "float (px)",
        default: || val(crate::DISTRACTION_FREE_FIRST_LINE_INDENT_DEFAULT),
        check: check::<f32>,
        doc: "Distraction-free first-line indent.",
    },
    SettingSpec {
        key: crate::DISTRACTION_FREE_PARA_SPACING_BEFORE_KEY,
        ty: "float (px)",
        default: || val(crate::DISTRACTION_FREE_PARA_SPACING_BEFORE_DEFAULT),
        check: check::<f32>,
        doc: "Distraction-free space above each paragraph.",
    },
    SettingSpec {
        key: crate::DISTRACTION_FREE_PARA_SPACING_AFTER_KEY,
        ty: "float (px)",
        default: || val(crate::DISTRACTION_FREE_PARA_SPACING_AFTER_DEFAULT),
        check: check::<f32>,
        doc: "Distraction-free space below each paragraph.",
    },
    // ── Goals & word count ────────────────────────────────────────────────────
    SettingSpec {
        key: crate::GOALS_COUNTING_METHOD_KEY,
        ty: "one of: Auto | Whitespace | UnicodeWords | CjkHybrid",
        default: || val(CountingMethodSetting::default()),
        check: check::<CountingMethodSetting>,
        doc: "How the live status-bar word count counts. Never affects the stored progress \
              snapshot, which is always Auto.",
    },
    SettingSpec {
        key: crate::GOALS_SHOW_CHARACTERS_KEY,
        ty: "bool",
        default: || val(crate::GOALS_SHOW_CHARACTERS_DEFAULT),
        check: check::<bool>,
        doc: "Show the character count beside the word count in the status bar.",
    },
    // ── Writing session ───────────────────────────────────────────────────────
    SettingSpec {
        key: "session.word_target",
        ty: "integer (words; 0 = none)",
        default: || val(0_i64),
        check: check::<i64>,
        doc: "Remembered writing-session word goal.",
    },
    SettingSpec {
        key: "session.time_target_min",
        ty: "integer (minutes; 0 = none)",
        default: || val(0_i64),
        check: check::<i64>,
        doc: "Remembered writing-session time limit.",
    },
    // ── Corkboard ─────────────────────────────────────────────────────────────
    SettingSpec {
        key: crate::CORKBOARD_NESTED_KEY,
        ty: "bool",
        default: || val(crate::CORKBOARD_NESTED_DEFAULT),
        check: check::<bool>,
        doc: "Corkboard shows a container's direct children (true) or every descendant \
              leaf (false).",
    },
    SettingSpec {
        key: crate::CORKBOARD_CARD_SIZE_KEY,
        ty: "float (px, 210–680)",
        default: || val(crate::CORKBOARD_CARD_SIZE_DEFAULT),
        check: check::<f32>,
        doc: "Minimum corkboard tile width.",
    },
    SettingSpec {
        key: crate::CORKBOARD_SHOW_WORD_COUNT_KEY,
        ty: "bool",
        default: || val(crate::CORKBOARD_SHOW_WORD_COUNT_DEFAULT),
        check: check::<bool>,
        doc: "Show a card's word count in its footer.",
    },
    SettingSpec {
        key: crate::CORKBOARD_MODAL_SIZE_KEY,
        ty: "float (scale, 0.7–2.0)",
        default: || val(crate::CORKBOARD_MODAL_SIZE_DEFAULT),
        check: check::<f32>,
        doc: "Expanded-synopsis editor font-size scale.",
    },
    SettingSpec {
        key: crate::CORKBOARD_SHOW_CARD_NUMBERS_KEY,
        ty: "bool",
        default: || val(crate::CORKBOARD_SHOW_CARD_NUMBERS_DEFAULT),
        check: check::<bool>,
        doc: "Number the corkboard's cards in board order.",
    },
    SettingSpec {
        key: crate::CORKBOARD_FONT_FAMILY_KEY,
        ty: "string (a font family the typesetter can resolve)",
        default: || val(crate::CORKBOARD_FONT_FAMILY_DEFAULT),
        check: check::<String>,
        doc: "Corkboard card synopsis typeface.",
    },
    SettingSpec {
        key: crate::CORKBOARD_SIZE_KEY,
        ty: "float (1.0 = 100 %)",
        default: || val(crate::CORKBOARD_SIZE_DEFAULT),
        check: check::<f32>,
        doc: "Corkboard card synopsis font-size scale.",
    },
    SettingSpec {
        key: crate::CORKBOARD_LINE_HEIGHT_KEY,
        ty: "float (multiple of font size)",
        default: || val(crate::CORKBOARD_LINE_HEIGHT_DEFAULT),
        check: check::<f32>,
        doc: "Corkboard card synopsis line height.",
    },
    SettingSpec {
        key: crate::CORKBOARD_FIRST_LINE_INDENT_KEY,
        ty: "float (px)",
        default: || val(crate::CORKBOARD_FIRST_LINE_INDENT_DEFAULT),
        check: check::<f32>,
        doc: "Corkboard card synopsis first-line indent.",
    },
    SettingSpec {
        key: crate::CORKBOARD_PARA_SPACING_BEFORE_KEY,
        ty: "float (px)",
        default: || val(crate::CORKBOARD_PARA_SPACING_BEFORE_DEFAULT),
        check: check::<f32>,
        doc: "Corkboard card synopsis space above each paragraph.",
    },
    SettingSpec {
        key: crate::CORKBOARD_PARA_SPACING_AFTER_KEY,
        ty: "float (px)",
        default: || val(crate::CORKBOARD_PARA_SPACING_AFTER_DEFAULT),
        check: check::<f32>,
        doc: "Corkboard card synopsis space below each paragraph.",
    },
];

/// The spec for `key`, if it is a settable one.
pub fn spec(key: &str) -> Option<&'static SettingSpec> {
    SETTINGS.iter().find(|s| s.key == key)
}

// ─────────────────────────────────────────────────────────────────────────────
// Dotted-path helpers
//
// `general.toml` nests (`[editor.scene] size = 1.0`), while this module speaks
// dotted keys throughout — they are what the app's own constants hold, what an
// error message should name, and what `dump` emits so its output pastes straight
// back in as a pins file. These two convert.
// ─────────────────────────────────────────────────────────────────────────────

/// Resolve a dotted key against a parsed TOML tree.
fn lookup<'a>(root: &'a toml::Value, key: &str) -> Option<&'a toml::Value> {
    let mut cur = root;
    for segment in key.split('.') {
        cur = cur.as_table()?.get(segment)?;
    }
    Some(cur)
}

/// Set a dotted key in a parsed TOML tree, creating intermediate tables.
///
/// A non-table sitting where a table is needed is replaced rather than refused: it can
/// only be a stale value from an older schema (the store has no migrator), and failing
/// the launch over one would be a worse answer than overwriting it.
fn insert(root: &mut toml::Value, key: &str, value: toml::Value) {
    let (leaf, parents) = match key.split('.').collect::<Vec<_>>() {
        segments if segments.is_empty() => return,
        mut segments => {
            let leaf = segments.pop().expect("split always yields one segment");
            (leaf.to_string(), segments)
        }
    };

    let mut cur = root;
    for segment in parents {
        // `cur` is a table on every iteration — the caller starts from one, and the
        // loop replaces any non-table it descends into (below) before moving on. The
        // `else` is therefore unreachable, and returns rather than panicking: dropping
        // one pin beats aborting a launch over an invariant the type cannot express.
        let Some(table) = cur.as_table_mut() else {
            return;
        };
        let next = table
            .entry(segment.to_string())
            .or_insert_with(|| toml::Value::Table(toml::Table::new()));
        if !next.is_table() {
            *next = toml::Value::Table(toml::Table::new());
        }
        cur = next;
    }

    if let Some(table) = cur.as_table_mut() {
        table.insert(leaf, value);
    }
}

/// Flatten a parsed TOML tree into dotted key → scalar pairs.
///
/// Every registered setting is a scalar, so any table is structure and any non-table is a
/// leaf — including an array, which reaches [`load_pins`] as a leaf and is then rejected by
/// the key's own `check`. That is the right order: "editor.scene.size wants a float" is a
/// better message than "arrays are not supported".
fn flatten(prefix: &str, value: &toml::Value, out: &mut Vec<(String, toml::Value)>) {
    match value.as_table() {
        Some(table) => {
            for (k, v) in table {
                let path = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{prefix}.{k}")
                };
                flatten(&path, v, out);
            }
        }
        None => out.push((prefix.to_string(), value.clone())),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// `--config`
// ─────────────────────────────────────────────────────────────────────────────

/// Read and validate a pins file, returning its keys.
///
/// Sorted, not in file order: `toml::Table` is a `BTreeMap`, so parsing has already lost
/// where each line sat. That is fine for applying them (every key is independent) and it
/// makes the error report deterministic, which matters more — a probe that fails twice
/// should print the same thing twice.
///
/// Every problem in the file is reported at once, not just the first: an agent editing a
/// pins file should need one round trip, not one per typo.
pub fn load_pins(path: &Path) -> Result<Vec<(String, toml::Value)>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read config file {}: {e}", path.display()))?;
    let parsed: toml::Value = toml::from_str(&text)
        .map_err(|e| format!("cannot parse config file {}: {e}", path.display()))?;
    if !parsed.is_table() {
        return Err(format!(
            "config file {} must be a table of settings",
            path.display()
        ));
    }

    let mut pins = Vec::new();
    flatten("", &parsed, &mut pins);

    let mut problems = Vec::new();
    for (key, value) in &pins {
        match spec(key) {
            None => {
                let mut line = format!("  unknown setting `{key}`");
                if let Some(near) = nearest(key) {
                    line.push_str(&format!(" — did you mean `{near}`?"));
                }
                problems.push(line);
            }
            Some(spec) => {
                if let Err(e) = (spec.check)(value) {
                    problems.push(format!(
                        "  `{key}` expects {} but got `{value}` ({e})",
                        spec.ty
                    ));
                }
            }
        }
    }

    if problems.is_empty() {
        Ok(pins)
    } else {
        Err(format!(
            "{} rejected:\n{}\n\nRun `skribisto --dump-config` for every settable key, its \
             type and its current value.",
            path.display(),
            problems.join("\n")
        ))
    }
}

/// Merge validated pins into the settings store's file, creating it if absent.
///
/// Writes rather than overlays, deliberately. The store is read at two independent points
/// during startup — `read_prefs` before any widget tree exists (it decides Launcher vs.
/// project) and the app builder's own bundle — and a value on disk is the one thing both
/// see. An in-memory overlay would have to be threaded through both, and would still lose
/// to the debounced writer the moment any setting changed.
///
/// The corollary is that `--config` **modifies the config directory it runs against**,
/// which is why the caller prints the resolved path and why the flag is debug-only. Pair
/// it with a sandboxed `XDG_CONFIG_HOME` (as `automation_fixture.isolated_config` does)
/// and it cannot touch the operator's own settings.
pub fn merge_into(general: &Path, pins: &[(String, toml::Value)]) -> Result<(), String> {
    let mut root = match std::fs::read_to_string(general) {
        Ok(text) => toml::from_str::<toml::Value>(&text)
            .map_err(|e| format!("cannot parse {}: {e}", general.display()))?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            toml::Value::Table(toml::Table::new())
        }
        Err(e) => return Err(format!("cannot read {}: {e}", general.display())),
    };
    if !root.is_table() {
        root = toml::Value::Table(toml::Table::new());
    }

    for (key, value) in pins {
        insert(&mut root, key, value.clone());
    }

    if let Some(parent) = general.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
    }
    let text = toml::to_string(&root).map_err(|e| format!("cannot serialize settings: {e}"))?;
    std::fs::write(general, text).map_err(|e| format!("cannot write {}: {e}", general.display()))
}

/// The registered key closest to `key`, when one is close enough to be a plausible typo.
///
/// The threshold scales with the key's length — a third of it, capped at 6 — so a short key
/// does not match half the table while a long dotted one still tolerates a wrong segment.
fn nearest(key: &str) -> Option<&'static str> {
    let budget = (key.len() / 3).clamp(2, 6);
    SETTINGS
        .iter()
        .map(|s| (distance(key, s.key), s.key))
        .filter(|(d, _)| *d <= budget)
        .min_by_key(|(d, _)| *d)
        .map(|(_, k)| k)
}

/// Levenshtein distance, two rows at a time.
fn distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0; b.len() + 1];
    for (i, ca) in a.chars().enumerate() {
        cur[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let cost = usize::from(ca != *cb);
            cur[j + 1] = (prev[j] + cost).min(prev[j + 1] + 1).min(cur[j] + 1);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

// ─────────────────────────────────────────────────────────────────────────────
// `--dump-config`
// ─────────────────────────────────────────────────────────────────────────────

/// Render every settable key with its effective value, as a pins file.
///
/// Effective means "what the app will use": the value on disk when the key is present,
/// otherwise the registered default. Emitted as flat dotted keys rather than `[tables]`
/// precisely so a caller can delete the lines they do not care about and pass the rest
/// straight back as `--config` — a `[section]` header would break the moment a line under
/// it was removed.
pub fn dump(general: &Path) -> String {
    let disk = std::fs::read_to_string(general)
        .ok()
        .and_then(|text| toml::from_str::<toml::Value>(&text).ok())
        .unwrap_or_else(|| toml::Value::Table(toml::Table::new()));

    let mut out = String::new();
    out.push_str("# Skribisto effective settings.\n");
    out.push_str(&format!("# Source: {}\n", general.display()));
    out.push_str(
        "# Every line below is settable via `--config <file.toml>` (debug builds).\n\
         # Keep the lines you want pinned, delete the rest.\n\n",
    );

    for spec in SETTINGS {
        let on_disk = lookup(&disk, spec.key);
        let value = on_disk.cloned().unwrap_or_else(|| (spec.default)());
        let origin = if on_disk.is_some() { "set" } else { "default" };
        out.push_str(&format!("# {}\n", spec.doc));
        out.push_str(&format!("# {} — {origin}\n", spec.ty));
        out.push_str(&format!("{} = {}\n\n", spec.key, value));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    /// Every default must survive its own validator. A mismatch here means the table
    /// declares a type it does not actually hold — which would surface as a startup
    /// rejection of a perfectly good pins file.
    #[test]
    fn defaults_are_valid() {
        for spec in SETTINGS {
            let value = (spec.default)();
            assert!(
                (spec.check)(&value).is_ok(),
                "{}: default {value} fails its own check ({})",
                spec.key,
                (spec.check)(&value).unwrap_err()
            );
        }
    }

    #[test]
    fn keys_are_unique() {
        let mut seen = BTreeMap::new();
        for (i, spec) in SETTINGS.iter().enumerate() {
            if let Some(first) = seen.insert(spec.key, i) {
                panic!("{} is registered twice (entries {first} and {i})", spec.key);
            }
        }
    }

    /// Collect every `*_KEY: &str = "…"` declared anywhere under `src/`.
    ///
    /// A directory walk rather than a fixed `include_str!` list: the point of the drift
    /// test is to catch a key added in a file nobody thought to add here, and a hard-coded
    /// file list is blind to exactly that case. Handles the multi-line form
    /// (`pub const X: &str =\n    "…";`) by scanning forward to the next string literal.
    fn declared_keys() -> BTreeMap<String, String> {
        fn walk(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    out.push(path);
                }
            }
        }

        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        walk(&src, &mut files);

        let mut found = BTreeMap::new();
        for file in files {
            // This file declares no keys, and it necessarily contains the scanner's
            // own needle as a string literal — so scanning it finds the scanner
            // rather than a setting.
            if file.file_name().is_some_and(|n| n == "settings_keys.rs") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&file) else {
                continue;
            };
            for (index, _) in text.match_indices("_KEY: &str =") {
                let tail = &text[index..];
                let Some(open) = tail.find('"') else { continue };
                let Some(close) = tail[open + 1..].find('"') else {
                    continue;
                };
                let key = &tail[open + 1..open + 1 + close];
                let name_start = text[..index]
                    .rfind(|c: char| c.is_whitespace())
                    .map(|i| i + 1)
                    .unwrap_or(0);
                let name = format!("{}_KEY", &text[name_start..index]);
                found.insert(key.to_string(), name);
            }
        }
        found
    }

    /// The registry is hand-written and the app's keys are not: this is what keeps them
    /// in step. A key declared in the crate but missing here is unpinnable and invisible
    /// to `--dump-config`, silently — the exact failure mode this module exists to remove.
    #[test]
    fn every_declared_key_is_registered() {
        let declared = declared_keys();
        assert!(
            declared.len() > 50,
            "the source scan found only {} keys — the scan itself is broken",
            declared.len()
        );

        let missing: Vec<String> = declared
            .iter()
            .filter(|(key, _)| spec(key).is_none())
            .map(|(key, name)| format!("  {key}  (declared as {name})"))
            .collect();

        assert!(
            missing.is_empty(),
            "these settings keys are declared in the crate but missing from SETTINGS:\n{}\n\
             Add a SettingSpec for each, or --config and --dump-config will not see them.",
            missing.join("\n")
        );
    }

    /// The other direction. A registered key that no longer exists in the app is dead
    /// weight that `--dump-config` still advertises as settable.
    ///
    /// Four keys are legitimately not declared as `*_KEY` constants and are exempt: the
    /// three `editor.last_view.*` (inline literals in `EditorViewMemory::new`) and
    /// teksilo's own `accessibility.text_scale` (a `SettingsKey<f32>` in the framework).
    #[test]
    fn every_registered_key_still_exists() {
        let declared = declared_keys();
        let exempt = [
            "editor.last_view.book",
            "editor.last_view.part",
            "editor.last_view.chapter",
            teksilo::settings::TEXT_SCALE_KEY.key,
        ];

        let orphans: Vec<&str> = SETTINGS
            .iter()
            .map(|s| s.key)
            .filter(|k| !declared.contains_key(*k) && !exempt.contains(k))
            .collect();

        assert!(
            orphans.is_empty(),
            "these keys are registered but no longer declared anywhere in the crate: {orphans:?}"
        );
    }

    #[test]
    fn a_dotted_key_round_trips_through_the_tree() {
        let mut root = toml::Value::Table(toml::Table::new());
        insert(&mut root, "editor.scene.size", toml::Value::Float(1.25));
        insert(&mut root, "editor.autosave", toml::Value::Boolean(true));

        assert_eq!(
            lookup(&root, "editor.scene.size"),
            Some(&toml::Value::Float(1.25))
        );
        assert_eq!(
            lookup(&root, "editor.autosave"),
            Some(&toml::Value::Boolean(true))
        );
        // Sibling keys under one prefix must coexist — the whole file is one tree.
        assert!(lookup(&root, "editor").unwrap().is_table());
    }

    /// A stale scalar where a table now belongs is overwritten, not fatal.
    #[test]
    fn a_scalar_in_the_way_is_replaced_by_a_table() {
        let mut root = toml::Value::Table(toml::Table::new());
        insert(&mut root, "editor", toml::Value::Boolean(true));
        insert(&mut root, "editor.autosave", toml::Value::Boolean(true));
        assert_eq!(
            lookup(&root, "editor.autosave"),
            Some(&toml::Value::Boolean(true))
        );
    }

    #[test]
    fn flatten_produces_dotted_keys() {
        let parsed: toml::Value = toml::from_str(
            "[editor]\nautosave = true\n[editor.scene]\nsize = 1.5\n[ui]\ndark = true\n",
        )
        .unwrap();
        let mut out = Vec::new();
        flatten("", &parsed, &mut out);
        let keys: Vec<&str> = out.iter().map(|(k, _)| k.as_str()).collect();
        assert!(keys.contains(&"editor.autosave"));
        assert!(keys.contains(&"editor.scene.size"));
        assert!(keys.contains(&"ui.dark"));
        assert_eq!(keys.len(), 3, "only leaves, never the tables above them");
    }

    /// Both spellings a caller might reach for must parse to the same pins, or the
    /// dotted-key form `dump` emits would not be re-readable by `--config`.
    #[test]
    fn dotted_and_sectioned_pins_are_equivalent() {
        let dir = tempfile::tempdir().unwrap();

        let dotted = dir.path().join("dotted.toml");
        std::fs::write(&dotted, "editor.autosave = true\nui.dark = true\n").unwrap();

        let sectioned = dir.path().join("sectioned.toml");
        std::fs::write(&sectioned, "[editor]\nautosave = true\n[ui]\ndark = true\n").unwrap();

        assert_eq!(load_pins(&dotted).unwrap(), load_pins(&sectioned).unwrap());
    }

    #[test]
    fn an_unknown_key_is_rejected_with_a_suggestion() {
        let dir = tempfile::tempdir().unwrap();
        let pins = dir.path().join("pins.toml");
        std::fs::write(&pins, "ui.local = \"fr-FR\"\n").unwrap();

        let err = load_pins(&pins).unwrap_err();
        assert!(err.contains("unknown setting `ui.local`"), "{err}");
        assert!(err.contains("did you mean `ui.locale`"), "{err}");
    }

    /// Nothing in the table is within typo range of this, so the message must stand on
    /// its own rather than suggest an unrelated key.
    #[test]
    fn a_wild_key_is_rejected_without_a_bogus_suggestion() {
        let dir = tempfile::tempdir().unwrap();
        let pins = dir.path().join("pins.toml");
        std::fs::write(&pins, "backup.retention.daily = 7\n").unwrap();

        let err = load_pins(&pins).unwrap_err();
        assert!(
            err.contains("unknown setting `backup.retention.daily`"),
            "{err}"
        );
        assert!(
            !err.contains("did you mean"),
            "no near neighbour exists: {err}"
        );
    }

    /// A bare `1` for a float key is accepted, because `SettingsStore::signal`'s
    /// `T::deserialize` accepts it. The check must not be stricter than the store, or it
    /// would refuse a pins file that would have worked perfectly.
    #[test]
    fn an_integer_is_accepted_where_the_store_accepts_one() {
        let dir = tempfile::tempdir().unwrap();
        let pins = dir.path().join("pins.toml");
        std::fs::write(
            &pins,
            "editor.scene.size = 1\naccessibility.text_scale = 2\n",
        )
        .unwrap();
        assert!(load_pins(&pins).is_ok(), "{:?}", load_pins(&pins).err());
    }

    #[test]
    fn a_mistyped_value_is_rejected_naming_the_expected_type() {
        let dir = tempfile::tempdir().unwrap();
        let pins = dir.path().join("pins.toml");
        std::fs::write(&pins, "editor.autosave = \"yes\"\n").unwrap();

        let err = load_pins(&pins).unwrap_err();
        assert!(err.contains("`editor.autosave` expects bool"), "{err}");
    }

    /// An enum takes its variant name; a wrong one must name the legal set, since that is
    /// the whole reason `ty` is prose and not a Rust type name.
    #[test]
    fn an_unknown_enum_variant_is_rejected_naming_the_variants() {
        let dir = tempfile::tempdir().unwrap();
        let pins = dir.path().join("pins.toml");
        std::fs::write(&pins, "editor.highlight_scope = \"Line\"\n").unwrap();

        let err = load_pins(&pins).unwrap_err();
        assert!(
            err.contains("None | Sentence | Paragraph"),
            "the error must list the legal variants: {err}"
        );
    }

    /// Every problem at once, so a pins file takes one round trip to fix rather than one
    /// per mistake.
    #[test]
    fn every_problem_is_reported_together() {
        let dir = tempfile::tempdir().unwrap();
        let pins = dir.path().join("pins.toml");
        std::fs::write(
            &pins,
            "ui.dark = \"yes\"\nui.nonsense = 1\neditor.autosave = 3\n",
        )
        .unwrap();

        let err = load_pins(&pins).unwrap_err();
        assert!(err.contains("ui.dark"), "{err}");
        assert!(err.contains("ui.nonsense"), "{err}");
        assert!(err.contains("editor.autosave"), "{err}");
    }

    #[test]
    fn valid_pins_survive_the_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let pins = dir.path().join("pins.toml");
        std::fs::write(
            &pins,
            "ui.dark = true\nui.locale = \"fr-FR\"\neditor.scene.size = 1.25\n\
             editor.highlight_scope = \"Paragraph\"\n",
        )
        .unwrap();

        let general = dir.path().join("general.toml");
        merge_into(&general, &load_pins(&pins).unwrap()).unwrap();

        let written: toml::Value =
            toml::from_str(&std::fs::read_to_string(&general).unwrap()).unwrap();
        assert_eq!(
            lookup(&written, "ui.dark"),
            Some(&toml::Value::Boolean(true))
        );
        assert_eq!(
            lookup(&written, "ui.locale"),
            Some(&toml::Value::String("fr-FR".into()))
        );
        assert_eq!(
            lookup(&written, "editor.scene.size"),
            Some(&toml::Value::Float(1.25))
        );
        assert_eq!(
            lookup(&written, "editor.highlight_scope"),
            Some(&toml::Value::String("Paragraph".into()))
        );
    }

    /// Pins overwrite the keys they name and leave every other key alone — the file is a
    /// merge target, not a replacement.
    #[test]
    fn merging_preserves_unrelated_keys() {
        let dir = tempfile::tempdir().unwrap();
        let general = dir.path().join("general.toml");
        std::fs::write(
            &general,
            "[ui]\ndark = false\nlocale = \"en-US\"\n[editor]\nautosave = true\n",
        )
        .unwrap();

        let pins = dir.path().join("pins.toml");
        std::fs::write(&pins, "ui.dark = true\n").unwrap();
        merge_into(&general, &load_pins(&pins).unwrap()).unwrap();

        let written: toml::Value =
            toml::from_str(&std::fs::read_to_string(&general).unwrap()).unwrap();
        assert_eq!(
            lookup(&written, "ui.dark"),
            Some(&toml::Value::Boolean(true))
        );
        assert_eq!(
            lookup(&written, "ui.locale"),
            Some(&toml::Value::String("en-US".into())),
            "an unpinned key must survive"
        );
        assert_eq!(
            lookup(&written, "editor.autosave"),
            Some(&toml::Value::Boolean(true)),
            "so must a key in another section"
        );
    }

    /// A missing settings file is the normal case for a fresh sandbox, not an error.
    #[test]
    fn merging_into_a_missing_file_creates_it() {
        let dir = tempfile::tempdir().unwrap();
        let general = dir.path().join("nested").join("general.toml");

        let pins = dir.path().join("pins.toml");
        std::fs::write(&pins, "ui.dark = true\n").unwrap();
        merge_into(&general, &load_pins(&pins).unwrap()).unwrap();

        let written: toml::Value =
            toml::from_str(&std::fs::read_to_string(&general).unwrap()).unwrap();
        assert_eq!(
            lookup(&written, "ui.dark"),
            Some(&toml::Value::Boolean(true))
        );
    }

    /// The load-bearing property of `dump`: its output is a legal pins file. If it ever
    /// stops being one, the documented workflow ("dump, delete lines, pass it back")
    /// silently breaks.
    #[test]
    fn a_dump_is_a_valid_pins_file() {
        let dir = tempfile::tempdir().unwrap();
        let general = dir.path().join("general.toml");
        std::fs::write(&general, "[ui]\ndark = true\n").unwrap();

        let rendered = dir.path().join("dumped.toml");
        std::fs::write(&rendered, dump(&general)).unwrap();

        let pins = load_pins(&rendered).expect("a dump must be re-readable as pins");
        assert_eq!(
            pins.len(),
            SETTINGS.len(),
            "a dump lists every key exactly once"
        );
        let dark = pins.iter().find(|(k, _)| k == crate::DARK_KEY).unwrap();
        assert_eq!(dark.1, toml::Value::Boolean(true), "the disk value wins");
    }

    #[test]
    fn a_dump_marks_which_values_came_from_disk() {
        let dir = tempfile::tempdir().unwrap();
        let general = dir.path().join("general.toml");
        std::fs::write(&general, "[ui]\ndark = true\n").unwrap();

        let text = dump(&general);
        let dark_line = format!("{} = true", crate::DARK_KEY);
        let dark_at = text.find(&dark_line).expect("ui.dark must be listed");
        assert!(
            text[..dark_at].ends_with("bool — set\n"),
            "a key present on disk is marked `set`"
        );
        assert!(
            text.contains("— default"),
            "keys absent from disk are marked `default`"
        );
    }

    /// A dump against a config directory that does not exist yet must still work: it is
    /// the first thing an agent runs, quite possibly before ever launching the app.
    #[test]
    fn a_dump_of_a_missing_file_is_all_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let text = dump(&dir.path().join("absent.toml"));
        assert!(!text.contains("— set"), "nothing can have been set");
        assert!(text.contains(&format!("{} = false", crate::DARK_KEY)));
    }
}
