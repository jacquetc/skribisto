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
/// The `toml` this crate's public API speaks.
///
/// [`SettingSpec`] carries `toml::Value` in its `default` and `check` fields, so
/// an extension declaring its own `toml` dependency at a different version gets
/// "expected `toml::value::Value`, found `toml::Value`" — an error that names one
/// type twice and explains nothing. Re-exported so a registration writes
/// `teksilo_ui::settings_keys::toml::Value` and the mismatch is unexpressible.
pub use toml;

// ── The keys themselves ──────────────────────────────────────────────────────
//
// Declared here rather than at the crate root because the `SETTINGS` table below
// has to name every one of them and the drift test exists only because the two
// used to sit a thousand lines apart, in different files. They are re-exported
// from the crate root (`crate::DARK_KEY`, …), which is where the rest of the app
// still reads them from.

/// Persisted-setting keys (also read at startup in `main`).
pub const DARK_KEY: &str = "ui.dark";

/// What to do with a large image on insert: `"ask"` (default), `"keep"` or
/// `"downscale"`.
///
/// A string rather than a bool because there are three answers, and the writer
/// reaches the third — "stop asking, and do this from now on" — by ticking a box
/// in the prompt. Storing "ask" separately from the two decisions is what lets
/// the prompt come back if they ever want it to.
pub const IMAGE_SIZE_POLICY_KEY: &str = "editor.image_size_policy";
pub const LOCALE_KEY: &str = "ui.locale";
/// Who is using this installation — the name a comment or reply is signed with.
///
/// **Not** `Work.author_name`, which is the *book's* byline: that one travels
/// inside the `.skrib`, so signing a remark with it means an editor opening the
/// project signs their notes with the novelist's name. This is app-level and
/// per-installation, so it stays right whoever holds the file. Empty is the
/// ordinary unset state — see [`crate::comments::signature`] for the fallback.
pub const USER_NAME_KEY: &str = "user.name";
/// The initials shown beside a comment in Word (`w:initials`), overriding what
/// would otherwise be derived from [`USER_NAME_KEY`].
///
/// Explicit because no derivation rule gets every name right — "Mary-Jane
/// O'Brien" is as plausibly `MO` as `MJO` — and initials are the sort of thing
/// people are particular about. Empty means "derive it", not "show none".
pub const USER_INITIALS_KEY: &str = "user.initials";
/// Max width (px) of the centered main-text writing column.
pub const EDITOR_WIDTH_KEY: &str = "editor.column_width";
pub const EDITOR_WIDTH_DEFAULT: f32 = 700.0;
/// Max width (px) of the search preview editor (bottom band), so a too-wide
/// paragraph stays readable — same treatment as the scene column.
pub const PREVIEW_WIDTH_KEY: &str = "search.preview_width";
pub const PREVIEW_WIDTH_DEFAULT: f32 = 700.0;
/// Max width (px) of the writing column while distraction-free mode is active
/// (Settings ▸ Editor ▸ Editor Behavior). Independent from [`EDITOR_WIDTH_KEY`]
/// — the two never share a value, so widening the normal Scene column can
/// never silently widen (or narrow) the distraction-free one. ~68 characters
/// is the width every typography source surveyed converges on for comfortable
/// reading (Bringhurst ~66, Butterick 45–90, Dyson & Haselgrove ~55); 620 px is
/// that measure at the distraction-free bundle's own default face/size.
pub const DISTRACTION_FREE_WIDTH_KEY: &str = "editor.distraction_free.column_width";
pub const DISTRACTION_FREE_WIDTH_DEFAULT: f32 = 620.0;
// ── Which pieces of chrome distraction-free mode keeps ──
//
// The mode's job is to take chrome away, so each of these defaults to the
// *quieter* answer and the writer opts back in — except the Exit button,
// which is deliberately **not** a setting at all. It is the strip's
// documented way out (see `statusbar/focus_strip.rs`), so it must survive
// every combination of these four: a wedged Escape must never be able to
// combine with a settings choice to leave someone stuck in the mode.
// (There was a "keep the editor tab strip" setting here. The mode no longer
// undresses the project shell — it covers it with its own surface, which shows
// exactly one document and has no tab row for a setting to act on. The orphan
// key left behind in an existing `settings.toml` is harmless: the store is plain
// key/value with no migrator, and nothing reads it.)
/// The **id** of the distraction-free theme in force. Resolved through
/// `DistractionFreeThemesService::resolve`, which falls back to the first
/// built-in — so a theme the writer deleted, or one named by a config synced
/// from another machine and never imported here, leaves them with a working
/// surface rather than an unpainted one.
pub const DISTRACTION_FREE_THEME_KEY: &str = "editor.distraction_free.theme";
pub const DISTRACTION_FREE_THEME_DEFAULT: &str = "paper";
/// Keep the current item's **name** in the distraction-free strip (default
/// **on**). With the tab strip and the title bar both gone, and a plain Scene
/// carrying no title field in its own pane, this is the only thing on screen
/// that says which document the writer is in — which matters most right after
/// Alt+Up / Alt+Down have moved them to another one.
pub const DISTRACTION_FREE_TITLE_KEY: &str = "editor.distraction_free.title";
pub const DISTRACTION_FREE_TITLE_DEFAULT: bool = true;
/// Keep the word-count readout in the distraction-free strip (default **on**).
pub const DISTRACTION_FREE_WORD_COUNT_KEY: &str = "editor.distraction_free.word_count";
pub const DISTRACTION_FREE_WORD_COUNT_DEFAULT: bool = true;
/// Keep the writing-session readout in the distraction-free strip
/// (default **on**).
pub const DISTRACTION_FREE_SESSION_KEY: &str = "editor.distraction_free.session";
pub const DISTRACTION_FREE_SESSION_DEFAULT: bool = true;
/// Keep the Previous/Next pair in the distraction-free strip (default **on**).
/// One key for both buttons: they are a single navigational affordance, and a
/// strip offering only one direction would be a worse answer than either
/// showing or hiding the pair.
pub const DISTRACTION_FREE_GO_KEY: &str = "editor.distraction_free.go_buttons";
pub const DISTRACTION_FREE_GO_DEFAULT: bool = true;
/// Keep the "Go to…" jump button in the distraction-free strip (default **on**).
/// Distinct from [`DISTRACTION_FREE_GO_KEY`]: the arrows step relative to where
/// you are, this one jumps anywhere, and a writer may well want one without the
/// other. Ctrl+G still works either way — hiding a button never removes its
/// command.
pub const DISTRACTION_FREE_GO_TO_KEY: &str = "editor.distraction_free.go_to";
pub const DISTRACTION_FREE_GO_TO_DEFAULT: bool = true;
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
/// Show anchored comments in the editor (default **on**) — Tools ▸ Comments.
///
/// App-wide and persisted here rather than on the `Work`, for the same reason the
/// spell-check switch is: whether the ochre marks and the margin are drawn is a
/// preference of the person reading the screen, not a property of the manuscript, so it
/// follows the writer across projects and does not travel inside a `.skrib`.
///
/// It hides the *presentation*, never the data: the two comment docks keep listing every
/// thread, and a screen reader keeps announcing them. Hiding the marks is a way to read
/// the prose cleanly, not a way to stop having comments — a toggle that also emptied the
/// docks would leave a writer who decluttered the page with no way to act on the notes
/// they just hid.
pub const COMMENTS_VISIBLE_KEY: &str = "editor.comments";
pub const COMMENTS_VISIBLE_DEFAULT: bool = true;
/// When on (default) and no work was passed on the command line, a bare
/// launch opens the Launcher window (the Welcome UI). When off, a bare launch
/// instead opens the most recent *reachable* project directly — falling back
/// to the Launcher only if there is none (JetBrains' "reopen last project on
/// startup"). Toggled in Settings ▸ Appearance & Behaviour (the Launcher
/// itself has no inline copy of this — see `welcome_panel.rs`'s module docs).
pub const SHOW_WELCOME_KEY: &str = "ui.show_welcome";

/// Show the writing-plan summary when a project with an active plan opens.
///
/// App-global rather than per-project, because the projects it *would* be wrong for are
/// already excluded: a project with no active plan never shows it at all, so there is no
/// second answer left to give.
pub const PACE_SUMMARY_ON_OPEN_KEY: &str = "pace.summary_on_open";

// ── Editor typography (Settings ▸ Editor ▸ Scene / Synopsis / Notes) ──────────
// Non-destructive per-editor-type defaults. Font family / line height /
// first-line indent are applied via `RichTextEditor::typography_defaults`
// (a display-time snapshot fill that never mutates the document); size is a
// per-editor `font_size_scale` multiplier (`1.0` = 100 %), composed with the
// interface a11y text scale — real shaping (sharp), not page zoom. These are
// NOT the char-format `set_font_family`/`set_font_size` (which mutate the
// selection + document). `font_family` must resolve via the shared typesetter
// — an installed system font or one registered by `register_editor_fonts`
// below; the `FontPicker` control only ever offers names that will render.

/// The size slider's range, shared by every typography bundle except the
/// corkboard's *expanded* card editor (which has its own, wider ceiling —
/// `CORKBOARD_MODAL_SIZE_MIN`/`_MAX` below).
///
/// One source for three surfaces: the Settings sliders, Ctrl+Wheel over an
/// editor, and Ctrl+= / Ctrl+− / Ctrl+0. They were three copies of `0.7, 1.6,
/// 0.05` until the gestures arrived and made a drift between them a real bug —
/// a wheel that can reach a size the slider cannot show is a value the writer
/// can set and then never get back to.
pub const EDITOR_TYPO_SIZE_MIN: f32 = 0.7;
pub const EDITOR_TYPO_SIZE_MAX: f32 = 1.6;
pub const EDITOR_TYPO_SIZE_STEP: f32 = 0.05;

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

/// Distraction-free mode's own typography (Settings ▸ Editor ▸ Typography ▸
/// Distraction-free) — a fifth bundle alongside Scene / Synopsis / Notes /
/// Corkboard, selected at render time by `ContentTab::main_typography` whenever
/// the tab's window is in distraction-free mode (Shift+F11), in place of the
/// normal Scene/Notes bundle. Independent compile-time constants like every
/// other bundle here — deliberately **not** seeded from Scene at runtime, so
/// there is no ordering-fragile "copy on first entry" machinery. A touch larger
/// and more open than Scene's own defaults (a bigger zoom, taller line height):
/// distraction-free is the one surface meant to be read at arm's length with
/// nothing else on screen.
pub const DISTRACTION_FREE_FONT_FAMILY_KEY: &str = "editor.distraction_free.font_family";
pub const DISTRACTION_FREE_FONT_FAMILY_DEFAULT: &str = "Literata";
pub const DISTRACTION_FREE_SIZE_KEY: &str = "editor.distraction_free.size";
pub const DISTRACTION_FREE_SIZE_DEFAULT: f32 = 1.15;
pub const DISTRACTION_FREE_LINE_HEIGHT_KEY: &str = "editor.distraction_free.line_height";
pub const DISTRACTION_FREE_LINE_HEIGHT_DEFAULT: f32 = 1.8;
pub const DISTRACTION_FREE_FIRST_LINE_INDENT_KEY: &str =
    "editor.distraction_free.first_line_indent";
pub const DISTRACTION_FREE_FIRST_LINE_INDENT_DEFAULT: f32 = 24.0;
pub const DISTRACTION_FREE_PARA_SPACING_BEFORE_KEY: &str =
    "editor.distraction_free.para_spacing_before";
pub const DISTRACTION_FREE_PARA_SPACING_BEFORE_DEFAULT: f32 = 0.0;
pub const DISTRACTION_FREE_PARA_SPACING_AFTER_KEY: &str =
    "editor.distraction_free.para_spacing_after";
pub const DISTRACTION_FREE_PARA_SPACING_AFTER_DEFAULT: f32 = 0.0;

// ── Editor behaviour (Settings ▸ Editor ▸ Editor Behavior) ───────────────────
/// Show the synopsis pane above the manuscript in the dual-pane writing editor
/// (Skribisto's signature layout). Consumed live by the `shared::prose` body.
pub const SYNOPSIS_PANE_KEY: &str = "editor.synopsis_pane";
pub const SYNOPSIS_PANE_DEFAULT: bool = true;
/// Where that pane sits: above the manuscript (the default, and what every
/// existing project looks like) or beside it in its own column. A
/// [`crate::shared::SynopsisPlacement`], stored by variant name.
pub const SYNOPSIS_PLACEMENT_KEY: &str = "editor.synopsis_placement";
/// Width (px) of the Side synopsis column. Written back when the divider is
/// dragged, and read as the seed by every tab opened afterwards.
pub const SYNOPSIS_SIDE_WIDTH_KEY: &str = "editor.synopsis_side_width";
pub const SYNOPSIS_SIDE_WIDTH_DEFAULT: f32 = 280.0;
/// Bounds the Side column can be dragged between. Wide enough to read a sentence
/// of synopsis at the low end; never more than a third of a typical window at the
/// high end, since the manuscript is what the writer came for.
pub const SYNOPSIS_SIDE_WIDTH_MIN: f32 = 180.0;
pub const SYNOPSIS_SIDE_WIDTH_MAX: f32 = 560.0;
/// Remember, per container item type (Book / Part / Chapter), the last
/// `SegmentedControl` view used — so opening a new chapter lands on the same view
/// (e.g. Full Chapter) as the last chapter. The per-type indices live under
/// `editor.last_view.*` (see [`crate::settings::EditorViewMemory`]).
pub const REMEMBER_VIEW_KEY: &str = "editor.remember_view";
pub const REMEMBER_VIEW_DEFAULT: bool = true;
/// Keep the caret line pinned at a fixed height while typing.
pub const TYPEWRITER_KEY: &str = "editor.typewriter_scroll";
pub const TYPEWRITER_DEFAULT: bool = true;
/// Which height the pinned line sits at — a [`crate::shared::TypewriterAnchor`]
/// preset. `Option` because that is the shape a `ComboBox` selection takes; a
/// missing value resolves to the default rather than disabling the pin.
pub const TYPEWRITER_ANCHOR_KEY: &str = "editor.typewriter_anchor";

// ── Smart punctuation, application-level ────────────────────────────────────
//
// The default tier. A project that has not taken the override in
// Work ▸ Punctuation follows these, which is what `override_app_default: false`
// on its row means.
//
// Dashes, the ellipsis and curled quotes default **on**: that is what "smart
// punctuation" means to a writer, it is what Word and LibreOffice both
// do out of the box, and every one of them is reversible with a single Ctrl+Z on
// the keystroke that fired it.
//
// Pre-punctuation spacing defaults **off** even though it is equally correct for
// French. It inserts an *invisible* character, so a writer who has not asked for
// it would see their file change in ways they cannot see on screen — and unlike
// the others it applies to one language only.
// ── Editor: the margin lane ──────────────────────────────────────────────────
//
// ⚠ Every key here is written into the writer's `general.toml`, so all of them
// are frozen once shipped. The per-provider keys are not listed: they are
// synthesised from each registration's id (`editor.margin_lane.provider.<id>`),
// because an extension's providers cannot be known at compile time. See
// `margin_lane::LaneProviderSpec::settings_key`.
pub const MARGIN_LANE_ENABLED_KEY: &str = "editor.margin_lane.enabled";
pub const MARGIN_LANE_ENABLED_DEFAULT: bool = true;
/// The texture column, off by default — it costs 28 dp and not every writer
/// wants it. On is the *recommended* state, not the assumed one.
pub const MARGIN_LANE_TEXTURE_KEY: &str = "editor.margin_lane.texture";
pub const MARGIN_LANE_TEXTURE_DEFAULT: bool = false;

/// The settings key gating the lane on one surface.
///
/// ⚠ `LaneSurface::key` supplies the fragment and is frozen with it.
pub fn margin_lane_surface_key(surface: crate::margin_lane::LaneSurface) -> String {
    format!("editor.margin_lane.surface.{}", surface.key())
}

/// Whether the lane appears on `surface` for a writer who has never said.
///
/// **All of them**, and the reason it is still a function is that the answer used
/// to differ: two surfaces were listed that no lane was ever mounted on, and their
/// rows were off by default so nobody would notice they did nothing. Both are gone
/// (see [`LaneSurface::all`](crate::margin_lane::LaneSurface::all)), and what is
/// left is exactly the set where a position is worth knowing — which is why every
/// one of them is on.
///
/// Kept per-surface rather than folded into the single master switch because the
/// switches mean different things: the master one is "I do not want this feature",
/// a surface one is "not while I am reading search results".
pub fn margin_lane_surface_default(_surface: crate::margin_lane::LaneSurface) -> bool {
    true
}

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
/// How much text around the caret gets an ambient band — none, the sentence, or the whole
/// paragraph. See [`crate::shared::HighlightScope`].
///
/// A **new key**, not the `editor.highlight_sentence` boolean this replaces: the stored value
/// went from a `bool` to a scope, and a key named `highlight_sentence` holding `Paragraph`
/// would be a lie. The old key is simply never read again — it never had an effect to lose,
/// since nothing outside the settings pane ever consumed it.
pub const HIGHLIGHT_SCOPE_KEY: &str = "editor.highlight_scope";

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

// ── Writing games (Settings ▸ Editor ▸ Writing games) ────────────────────────
//
// Which surfaces the "Always forward" game covers **when it is being played**.
// Only these two are settings: whether the game is *on* is deliberately session
// state, minted per open `Work` and never persisted — see
// [`crate::writing_session::writing_games_vm`] for why a commitment device that outlives the
// sitting it was made in reads as a broken keyboard rather than as a rule.
/// Does "Always forward" freeze manuscript prose? On: prose is what the game is for.
pub const GAMES_FORWARD_PROSE_KEY: &str = "games.always_forward.prose";
/// Does "Always forward" freeze synopses? Off: the synopsis is where a writer
/// notes the fix the game has just forbidden them from making.
pub const GAMES_FORWARD_SYNOPSIS_KEY: &str = "games.always_forward.synopsis";

// ── Corkboard (Settings ▸ Editor ▸ Corkboard) ─────────────────────────────────
/// Corkboard default mode: `true` = nested (a container's direct children; a
/// folder card drills in), `false` = flat (all descendant leaves at once).
pub const CORKBOARD_NESTED_KEY: &str = "corkboard.nested";
pub const CORKBOARD_NESTED_DEFAULT: bool = true;
/// Corkboard card size — the minimum tile width in px the size slider drives.
///
/// 480 rather than the compact 240 it started at: at 240 a card is barely wider
/// than its own header row, so the synopsis — the thing an index card exists to
/// show — got two or three wrapped lines before it started scrolling. Twice that
/// gives the synopsis room to be read at a glance, which is the whole point of
/// the board, and still fits several columns across on a normal window.
pub const CORKBOARD_CARD_SIZE_KEY: &str = "corkboard.card_size";
pub const CORKBOARD_CARD_SIZE_DEFAULT: f32 = 480.0;
/// The card-size slider range (shared by the header slider and the settings pane).
/// A wide span: compact index cards (210) up to near-full-width review cards (680).
pub const CORKBOARD_CARD_SIZE_MIN: f32 = 210.0;
pub const CORKBOARD_CARD_SIZE_MAX: f32 = 680.0;
pub const CORKBOARD_CARD_SIZE_STEP: f32 = 10.0;
/// Show a card's word count in its footer.
pub const CORKBOARD_SHOW_WORD_COUNT_KEY: &str = "corkboard.show_word_count";
pub const CORKBOARD_SHOW_WORD_COUNT_DEFAULT: bool = true;
/// The **expanded** synopsis editor's own font-size scale.
///
/// Separate from the card's: a card's synopsis is sized to be scannable at a
/// glance in a small tile, and the writer opens the expanded editor precisely
/// when they want to *write* rather than scan. Defaults to 1.0 — full size,
/// against the card's compact 0.8.
pub const CORKBOARD_MODAL_SIZE_KEY: &str = "corkboard.modal_size";
pub const CORKBOARD_MODAL_SIZE_DEFAULT: f32 = 1.0;
/// …and its own, higher ceiling. The expanded editor is the one place on the
/// board meant for sustained writing rather than scanning, so a comfortable
/// reading size beats fitting a tile — which is why this alone goes past
/// `EDITOR_TYPO_SIZE_MAX`. The step stays `EDITOR_TYPO_SIZE_STEP` so a wheel
/// notch feels the same everywhere.
pub const CORKBOARD_MODAL_SIZE_MIN: f32 = 0.7;
pub const CORKBOARD_MODAL_SIZE_MAX: f32 = 2.0;
/// Being *wider* than the shared range is this bundle's entire reason to carry
/// a range of its own. Pinned at compile time rather than in a test, because if
/// the two ever became equal the right fix would be to delete the special case,
/// not to keep a passing test that no longer means anything.
const _: () = assert!(CORKBOARD_MODAL_SIZE_MAX > EDITOR_TYPO_SIZE_MAX);
/// Number the cards in board order, the way index cards are numbered. Off by default:
/// the number is a reading aid for a structure pass, not something the writer
/// needs on every card all the time.
pub const CORKBOARD_SHOW_CARD_NUMBERS_KEY: &str = "corkboard.show_card_numbers";
pub const CORKBOARD_SHOW_CARD_NUMBERS_DEFAULT: bool = false;

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

use frontend::common::entities::QuoteStyle;
use skribisto_model::counting::CountingMethodSetting;

use crate::shared::{HighlightScope, TypewriterAnchor};

/// One settable key: what it is called, what it holds, and what it means.
///
/// `Copy`, and every field is `'static` — a `&'static str` or a plain `fn`
/// pointer — which is what lets the built-in table and an extension's registered
/// keys be handled as one list without leaking or cloning. It also makes the
/// whole type `Send + Sync`, so the registry behind [`crate::settings_ext`] can be
/// an ordinary `RwLock` rather than a `thread_local!`.
#[derive(Clone, Copy)]
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
    // ── User ──────────────────────────────────────────────────────────────────
    SettingSpec {
        key: crate::USER_NAME_KEY,
        ty: "string",
        default: || val(""),
        check: check::<String>,
        doc: "Who is using this installation — the name comments and replies are signed \
              with. Distinct from the project's own author name, which is the book's \
              byline and travels inside the `.skrib`. Empty falls back to that byline.",
    },
    SettingSpec {
        key: crate::USER_INITIALS_KEY,
        ty: "string",
        default: || val(""),
        check: check::<String>,
        doc: "The initials shown beside a comment in Word (`w:initials`). Empty derives \
              them from the signing name rather than showing none.",
    },
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
        key: crate::PACE_SUMMARY_ON_OPEN_KEY,
        ty: "bool",
        default: || val(true),
        check: check::<bool>,
        doc: "Show where the book stands when a project with an active writing plan \
              opens. Projects without one never show it.",
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
        default: || val(crate::shared::SynopsisPlacement::default()),
        check: check::<crate::shared::SynopsisPlacement>,
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
    // Segment *ids*, not indices. A position stopped being a stable name for a segment
    // the moment `container.segments` let one be contributed — see
    // `tabs::shared::segments`. An install written before this carries an integer here;
    // `SettingsStore` falls back to the default on a deserialize failure, so the
    // remembered view resets once and sticks again after the next manual switch.
    SettingSpec {
        key: "editor.last_view.book",
        ty: "string (segment id)",
        default: || val(crate::tabs::shared::segments::SEG_OWN),
        check: check::<String>,
        doc: "Remembered segment id for Book tabs — e.g. \"own\", \"manuscript\", \
              \"analysis\", \"overview\" (written by the app; see editor.remember_view).",
    },
    SettingSpec {
        key: "editor.last_view.part",
        ty: "string (segment id)",
        default: || val(crate::tabs::shared::segments::SEG_OWN),
        check: check::<String>,
        doc: "Remembered segment id for Part tabs.",
    },
    SettingSpec {
        key: "editor.last_view.chapter",
        ty: "string (segment id)",
        default: || val(crate::tabs::shared::segments::SEG_OWN),
        check: check::<String>,
        doc: "Remembered segment id for Chapter-folder tabs.",
    },
    SettingSpec {
        key: "editor.last_view.note",
        ty: "string (segment id)",
        default: || val(crate::tabs::shared::segments::SEG_NOTES),
        check: check::<String>,
        doc: "Remembered segment id for notes-folder tabs (\"notes\" or \"overview\").",
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
    // ── Editor: the margin lane ───────────────────────────────────────────────
    SettingSpec {
        key: crate::MARGIN_LANE_ENABLED_KEY,
        ty: "bool",
        default: || val(crate::MARGIN_LANE_ENABLED_DEFAULT),
        check: check::<bool>,
        doc: "Show the strip beside the scroll bar that maps where things are.",
    },
    SettingSpec {
        key: crate::MARGIN_LANE_TEXTURE_KEY,
        ty: "bool",
        default: || val(crate::MARGIN_LANE_TEXTURE_DEFAULT),
        check: check::<bool>,
        doc: "Add a column showing each paragraph's length and how much of it is spoken.",
    },
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
    // ── Writing games ─────────────────────────────────────────────────────────
    //
    // Which surfaces "Always forward" covers. Whether it is being *played* is
    // per-session state and so has no key here at all — pinning it from
    // `--config` would be pinning a commitment the writer never made.
    SettingSpec {
        key: crate::GAMES_FORWARD_PROSE_KEY,
        ty: "bool",
        default: || val(crate::writing_session::FORWARD_PROSE_DEFAULT),
        check: check::<bool>,
        doc: "\"Always forward\" freezes manuscript prose while it is being played.",
    },
    SettingSpec {
        key: crate::GAMES_FORWARD_SYNOPSIS_KEY,
        ty: "bool",
        default: || val(crate::writing_session::FORWARD_SYNOPSIS_DEFAULT),
        check: check::<bool>,
        doc: "\"Always forward\" also freezes synopses while it is being played.",
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

/// Every settable key: the application's own, then anything an extension
/// registered through [`crate::settings_ext`].
///
/// Read through this rather than [`SETTINGS`] everywhere below, so an extension's
/// keys reach `--dump-config`, `--config` and the did-you-mean suggestion by the
/// same route the app's own do. A key that could be *set* but not *dumped* would
/// be exactly the silent-configuration failure this module exists to abolish,
/// one layer out.
pub fn all_specs() -> Vec<SettingSpec> {
    SETTINGS
        .iter()
        .copied()
        .chain(crate::settings_ext::registered_settings())
        .collect()
}

/// The top-level section of every key the **application** declares (`editor`,
/// `ui`, `backup`, …).
///
/// Derived from [`SETTINGS`] at runtime, never hand-listed: this is what
/// [`crate::settings_ext::register_settings`] refuses an extension key under, and
/// a hand-copied list would rot into a guard that passes while the collision it
/// exists to catch goes through — which is precisely what happened to
/// `commands_ext`'s first cut.
pub fn app_sections() -> std::collections::BTreeSet<&'static str> {
    SETTINGS
        .iter()
        .filter_map(|s| s.key.split('.').next())
        .collect()
}

/// The spec for `key`, if it is a settable one.
pub fn spec(key: &str) -> Option<SettingSpec> {
    all_specs().into_iter().find(|s| s.key == key)
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
    all_specs()
        .into_iter()
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

    for spec in all_specs() {
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
mod tests;
