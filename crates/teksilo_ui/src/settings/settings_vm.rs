// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SettingsViewModel` — a facade over the persisted UI settings.
//!
//! Store-backed: holds only cached settings `Signal`s, so every instance is a
//! view over the same live state. Rebuild it anywhere via
//! `SettingsViewModel::new(ctx.settings())`. Ambient app mutations (theme/locale)
//! reach the live app through an `EventContext`; pure-state ops (column width,
//! editor typography) do not.
//!
//! The full-preferences window (`settings.rs`'s `SettingsPanel`) binds these signals
//! into its category panes. Theme and interface language are driven there by the
//! framework's drop-in `ThemeSwitcher` / `LanguageSwitcher` (which apply live via
//! `EventContext`); this VM still owns the persisted `dark` / `locale` mirrors so
//! `App` can keep `DARK_KEY` / `LOCALE_KEY` in sync for the startup restore.

use teksilo::prelude::*;
use teksilo::widgets::SegmentId;

use crate::tabs::shared::segments as seg; // EventContext, Signal, intui
use teksilo::settings::SettingsStore;

use frontend::common::entities::BinderItemSubRole;
use skribisto_model::counting::CountingMethodSetting;

use frontend::common::entities::QuoteStyle;

use crate::shared::{HighlightScope, SynopsisPlacement, TypewriterAnchor};

use crate::{
    AUTOSAVE_KEY, COMMENTS_VISIBLE_DEFAULT, COMMENTS_VISIBLE_KEY, CORKBOARD_CARD_SIZE_DEFAULT,
    CORKBOARD_CARD_SIZE_KEY, CORKBOARD_FIRST_LINE_INDENT_DEFAULT, CORKBOARD_FIRST_LINE_INDENT_KEY,
    CORKBOARD_FONT_FAMILY_DEFAULT, CORKBOARD_FONT_FAMILY_KEY, CORKBOARD_LINE_HEIGHT_DEFAULT,
    CORKBOARD_LINE_HEIGHT_KEY, CORKBOARD_MODAL_SIZE_DEFAULT, CORKBOARD_MODAL_SIZE_KEY,
    CORKBOARD_NESTED_DEFAULT, CORKBOARD_NESTED_KEY, CORKBOARD_PARA_SPACING_AFTER_DEFAULT,
    CORKBOARD_PARA_SPACING_AFTER_KEY, CORKBOARD_PARA_SPACING_BEFORE_DEFAULT,
    CORKBOARD_PARA_SPACING_BEFORE_KEY, CORKBOARD_SHOW_CARD_NUMBERS_DEFAULT,
    CORKBOARD_SHOW_CARD_NUMBERS_KEY, CORKBOARD_SHOW_WORD_COUNT_DEFAULT,
    CORKBOARD_SHOW_WORD_COUNT_KEY, CORKBOARD_SIZE_DEFAULT, CORKBOARD_SIZE_KEY, DARK_KEY,
    DISTRACTION_FREE_FIRST_LINE_INDENT_DEFAULT, DISTRACTION_FREE_FIRST_LINE_INDENT_KEY,
    DISTRACTION_FREE_FONT_FAMILY_DEFAULT, DISTRACTION_FREE_FONT_FAMILY_KEY,
    DISTRACTION_FREE_GO_DEFAULT, DISTRACTION_FREE_GO_KEY, DISTRACTION_FREE_GO_TO_DEFAULT,
    DISTRACTION_FREE_GO_TO_KEY, DISTRACTION_FREE_LINE_HEIGHT_DEFAULT,
    DISTRACTION_FREE_LINE_HEIGHT_KEY, DISTRACTION_FREE_PARA_SPACING_AFTER_DEFAULT,
    DISTRACTION_FREE_PARA_SPACING_AFTER_KEY, DISTRACTION_FREE_PARA_SPACING_BEFORE_DEFAULT,
    DISTRACTION_FREE_PARA_SPACING_BEFORE_KEY, DISTRACTION_FREE_SESSION_DEFAULT,
    DISTRACTION_FREE_SESSION_KEY, DISTRACTION_FREE_SIZE_DEFAULT, DISTRACTION_FREE_SIZE_KEY,
    DISTRACTION_FREE_THEME_DEFAULT, DISTRACTION_FREE_THEME_KEY, DISTRACTION_FREE_TITLE_DEFAULT,
    DISTRACTION_FREE_TITLE_KEY, DISTRACTION_FREE_WIDTH_DEFAULT, DISTRACTION_FREE_WIDTH_KEY,
    DISTRACTION_FREE_WORD_COUNT_DEFAULT, DISTRACTION_FREE_WORD_COUNT_KEY, EDITOR_WIDTH_DEFAULT,
    EDITOR_WIDTH_KEY, GAMES_FORWARD_PROSE_KEY, GAMES_FORWARD_SYNOPSIS_KEY,
    GOALS_COUNTING_METHOD_KEY, GOALS_SHOW_CHARACTERS_DEFAULT, GOALS_SHOW_CHARACTERS_KEY,
    HIGHLIGHT_SCOPE_KEY, LOCALE_KEY, NOTES_FIRST_LINE_INDENT_DEFAULT, NOTES_FIRST_LINE_INDENT_KEY,
    NOTES_FONT_FAMILY_DEFAULT, NOTES_FONT_FAMILY_KEY, NOTES_LINE_HEIGHT_DEFAULT,
    NOTES_LINE_HEIGHT_KEY, NOTES_PARA_SPACING_AFTER_DEFAULT, NOTES_PARA_SPACING_AFTER_KEY,
    NOTES_PARA_SPACING_BEFORE_DEFAULT, NOTES_PARA_SPACING_BEFORE_KEY, NOTES_SIZE_DEFAULT,
    NOTES_SIZE_KEY, PREVIEW_WIDTH_DEFAULT, PREVIEW_WIDTH_KEY, PUNCT_DASHES_DEFAULT,
    PUNCT_DASHES_KEY, PUNCT_DIALOGUE_DEFAULT, PUNCT_DIALOGUE_KEY, PUNCT_ELLIPSIS_DEFAULT,
    PUNCT_ELLIPSIS_KEY, PUNCT_QUOTE_STYLE_KEY, PUNCT_QUOTES_DEFAULT, PUNCT_QUOTES_KEY,
    PUNCT_SPACING_DEFAULT, PUNCT_SPACING_KEY, REMEMBER_VIEW_DEFAULT, REMEMBER_VIEW_KEY,
    SCENE_FIRST_LINE_INDENT_DEFAULT, SCENE_FIRST_LINE_INDENT_KEY, SCENE_FONT_FAMILY_DEFAULT,
    SCENE_FONT_FAMILY_KEY, SCENE_LINE_HEIGHT_DEFAULT, SCENE_LINE_HEIGHT_KEY,
    SCENE_PARA_SPACING_AFTER_DEFAULT, SCENE_PARA_SPACING_AFTER_KEY,
    SCENE_PARA_SPACING_BEFORE_DEFAULT, SCENE_PARA_SPACING_BEFORE_KEY, SCENE_SIZE_DEFAULT,
    SCENE_SIZE_KEY, SHOW_WELCOME_KEY, SPELLCHECK_ENABLED_DEFAULT, SPELLCHECK_ENABLED_KEY,
    SYNOPSIS_FIRST_LINE_INDENT_DEFAULT, SYNOPSIS_FIRST_LINE_INDENT_KEY,
    SYNOPSIS_FONT_FAMILY_DEFAULT, SYNOPSIS_FONT_FAMILY_KEY, SYNOPSIS_LINE_HEIGHT_DEFAULT,
    SYNOPSIS_LINE_HEIGHT_KEY, SYNOPSIS_PANE_DEFAULT, SYNOPSIS_PANE_KEY,
    SYNOPSIS_PARA_SPACING_AFTER_DEFAULT, SYNOPSIS_PARA_SPACING_AFTER_KEY,
    SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT, SYNOPSIS_PARA_SPACING_BEFORE_KEY, SYNOPSIS_PLACEMENT_KEY,
    SYNOPSIS_SIDE_WIDTH_DEFAULT, SYNOPSIS_SIDE_WIDTH_KEY, SYNOPSIS_SIZE_DEFAULT, SYNOPSIS_SIZE_KEY,
    TYPEWRITER_ANCHOR_KEY, TYPEWRITER_DEFAULT, TYPEWRITER_KEY, USER_INITIALS_KEY, USER_NAME_KEY,
};

/// One editor type's four typography knobs. Cheap to clone — every field is a
/// `SettingsStore`-cached `Signal`, so all clones observe / drive the same
/// live value. `size` is a relative font-size scale (`1.0` = 100 %), composed
/// with interface a11y text scale; `line_height` is a multiple of the font
/// size; `first_line_indent` is in px.
#[derive(Clone)]
pub struct EditorTypography {
    pub font_family: Signal<String>,
    pub size: Signal<f32>,
    pub line_height: Signal<f32>,
    pub first_line_indent: Signal<f32>,
    /// Space (px) above each body paragraph.
    pub para_spacing_before: Signal<f32>,
    /// Space (px) below each body paragraph.
    pub para_spacing_after: Signal<f32>,
}

/// The per-editor-type typography bundles (Scene / Synopsis / Notes / Corkboard),
/// created once and threaded through `EditorsViewModel` into every `ContentTab`.
#[derive(Clone)]
pub struct EditorTypographySet {
    pub scene: EditorTypography,
    pub synopsis: EditorTypography,
    pub notes: EditorTypography,
    /// The corkboard card's synopsis editor — its own bundle so cards can read
    /// distinctly from the Full-Synopsis pane.
    pub corkboard: EditorTypography,
    /// Distraction-free mode's own bundle — selected by
    /// `ContentTab::main_typography` in place of Scene/Notes whenever the tab's
    /// window is in distraction-free mode. Independent, compile-time defaults
    /// like every bundle here (never seeded from Scene).
    pub distraction_free: EditorTypography,
}

/// Per-container-type "last view" memory: the `SegmentedControl` index a freshly
/// opened Book / Part / Chapter tab starts on, so opening a new chapter lands on
/// the same view (e.g. Full Chapter) as the last one — gated by the
/// `editor.remember_view` toggle. Store-backed (one cached signal per key), so it
/// is created wherever a [`SettingsStore`] is in hand and every clone drives the
/// same live state. Threaded through `EditorsViewModel` into every `ContentTab`,
/// like [`EditorTypographySet`].
///
/// Only the three folder-container sub-roles carry a `SegmentedControl` (a Scene /
/// Note / heading tab has no views to remember); [`Self::stored`] returns `None`
/// for everything else, so `initial`/`remember` are inert there.
#[derive(Clone)]
pub struct EditorViewMemory {
    enabled: Signal<bool>,
    // Segment **ids**, not indices. A position stopped naming a segment stably the moment
    // `container.segments` let one be contributed; see `tabs::shared::segments`. Stored as
    // the string rather than the derived `SegmentId` because the derivation is one-way —
    // and because a human-legible `general.toml` is the point of the settings schema.
    book: Signal<String>,
    part: Signal<String>,
    chapter: Signal<String>,
    note: Signal<String>,
}

impl EditorViewMemory {
    pub fn new(store: &SettingsStore) -> Self {
        Self {
            enabled: store.signal(REMEMBER_VIEW_KEY, REMEMBER_VIEW_DEFAULT),
            book: store.signal("editor.last_view.book", seg::SEG_OWN.to_string()),
            part: store.signal("editor.last_view.part", seg::SEG_OWN.to_string()),
            chapter: store.signal("editor.last_view.chapter", seg::SEG_OWN.to_string()),
            note: store.signal("editor.last_view.note", seg::SEG_NOTES.to_string()),
        }
    }

    /// A store-less handle over fresh signals — for tests and any tab built without
    /// a `SettingsStore` in hand.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn detached(enabled: bool) -> Self {
        Self {
            enabled: Signal::new(enabled),
            book: Signal::new(seg::SEG_OWN.to_string()),
            part: Signal::new(seg::SEG_OWN.to_string()),
            chapter: Signal::new(seg::SEG_OWN.to_string()),
            note: Signal::new(seg::SEG_NOTES.to_string()),
        }
    }

    /// The `editor.remember_view` toggle (bound by the Settings panel).
    pub fn enabled(&self) -> Signal<bool> {
        self.enabled.clone()
    }

    /// The persisted last-view signal for a segmented container sub-role, or `None`
    /// for a type with no `SegmentedControl`.
    fn stored(&self, sub_role: &BinderItemSubRole) -> Option<Signal<String>> {
        use BinderItemSubRole::*;
        match sub_role {
            Book => Some(self.book.clone()),
            Part => Some(self.part.clone()),
            // The *folder* chapter (`Folder/ChapterScene`) is the only `ChapterScene`
            // container with a `SegmentedControl`; the flat chapter is a prose tab and
            // never asks for this.
            ChapterScene => Some(self.chapter.clone()),
            // A notes folder has a two-segment bar of its own
            // (`folder_synopsis_with_overview`), and it was wrapped in `RememberSegment`
            // like the others while this arm was missing — so its remembered view was a
            // silent permanent no-op, and the comment claiming otherwise was false.
            //
            // `Paratext` shares that arm, not a key of its own: `folder_paratext::render`
            // delegates to the very same `folder_synopsis_with_overview`, so the two have
            // the same two segments and the same choice to remember. Giving Paratext its
            // own key would split one setting into two for one bar.
            Note | Paratext => Some(self.note.clone()),
            // `None` is shadowed by `BinderItemSubRole::None` under the glob import.
            _ => Option::None,
        }
    }

    /// The segment index a freshly-opened tab of `sub_role` should start on: the
    /// remembered view when enabled, else the container's own page (0).
    /// The segment a freshly-opened tab of `sub_role` should start on: the remembered one
    /// when enabled, else `None` — which the bar resolves to its first segment.
    ///
    /// Deriving the `SegmentId` here rather than storing it is what makes the stored
    /// string authoritative: the number is recomputed every launch, so it cannot drift, and
    /// an id whose segment no longer exists resolves to slot 0 through
    /// `segmented_control::index_signal` exactly as an absent one does.
    pub fn initial(&self, sub_role: &BinderItemSubRole) -> Option<SegmentId> {
        if !self.enabled.get() {
            return Option::None;
        }
        self.stored(sub_role)
            .map(|s| s.get())
            .filter(|id| !id.is_empty())
            .map(|id| seg::segment_id(&id))
    }

    /// Record `segment` as the last view for `sub_role` (no-op when disabled, for a
    /// non-segmented type, or when already equal).
    pub fn remember(&self, sub_role: &BinderItemSubRole, segment: &str) {
        if self.enabled.get()
            && let Some(s) = self.stored(sub_role)
            && s.get() != segment
        {
            s.set(segment.to_string());
        }
    }
}

/// The corkboard's default presentation, shared live into every container tab's
/// [`CorkboardViewModel`](crate::corkboard::CorkboardViewModel). Every field is a
/// store-backed signal, so editing it in Settings fans out to open boards at once.
#[derive(Clone)]
pub struct CorkboardDefaults {
    pub nested: Signal<bool>,
    pub card_size: Signal<f32>,
    pub show_word_count: Signal<bool>,
    pub show_card_numbers: Signal<bool>,
    /// The expanded-synopsis editor's own font-size scale.
    pub modal_size: Signal<f32>,
    pub counting_method: Signal<CountingMethodSetting>,
}

impl CorkboardDefaults {
    /// Fresh, unshared signals at the built-in defaults — for a standalone tab
    /// (tests) that has no `SettingsStore` to bind against.
    pub fn detached() -> Self {
        Self {
            nested: Signal::new(CORKBOARD_NESTED_DEFAULT),
            card_size: Signal::new(CORKBOARD_CARD_SIZE_DEFAULT),
            show_word_count: Signal::new(CORKBOARD_SHOW_WORD_COUNT_DEFAULT),
            show_card_numbers: Signal::new(CORKBOARD_SHOW_CARD_NUMBERS_DEFAULT),
            modal_size: Signal::new(CORKBOARD_MODAL_SIZE_DEFAULT),
            counting_method: Signal::new(CountingMethodSetting::default()),
        }
    }
}

#[derive(Clone)]
pub struct SettingsViewModel {
    dark: Signal<bool>,
    locale: Signal<String>,
    /// Who is using this installation — see [`crate::USER_NAME_KEY`]. App-level
    /// on purpose: the per-project author name is the *book's* byline, and an
    /// editor opening someone else's `.skrib` must not sign their remarks with it.
    user_name: Signal<String>,
    /// Explicit initials, overriding the derivation — see [`crate::USER_INITIALS_KEY`].
    user_initials: Signal<String>,
    column_width: Signal<f32>,
    preview_width: Signal<f32>,
    autosave: Signal<bool>,
    spellcheck_enabled: Signal<bool>,
    comments_visible: Signal<bool>,
    show_welcome: Signal<bool>,
    // ── Editor typography (per type) ──
    scene_typo: EditorTypography,
    synopsis_typo: EditorTypography,
    notes_typo: EditorTypography,
    corkboard_typo: EditorTypography,
    distraction_free_typo: EditorTypography,
    /// Max width (px) of the writing column while distraction-free mode is
    /// active — independent from [`Self::column_width`].
    distraction_free_width: Signal<f32>,
    /// Which items the distraction-free control strip keeps. Exit is
    /// The id of the distraction-free theme in force. An ordinary setting so
    /// there is one live handle behind it — see `DistractionFreeThemesViewModel`.
    distraction_free_theme: Signal<String>,
    distraction_free_title: Signal<bool>,
    distraction_free_word_count: Signal<bool>,
    distraction_free_session: Signal<bool>,
    distraction_free_go: Signal<bool>,
    distraction_free_go_to: Signal<bool>,
    // ── Editor behaviour ──
    synopsis_pane: Signal<bool>,
    synopsis_placement: Signal<SynopsisPlacement>,
    synopsis_side_width: Signal<f32>,
    /// Application-level smart punctuation — the tier a project follows when
    /// its own `SmartPunctuation` row leaves `override_app_default` off.
    punct_dashes: Signal<bool>,
    punct_ellipsis: Signal<bool>,
    punct_quotes: Signal<bool>,
    punct_quote_style: Signal<QuoteStyle>,
    punct_spacing: Signal<bool>,
    punct_dialogue: Signal<bool>,
    typewriter: Signal<bool>,
    typewriter_anchor: Signal<Option<TypewriterAnchor>>,
    highlight_scope: Signal<HighlightScope>,
    remember_view: Signal<bool>,
    // ── Goals & word count ──
    counting_method: Signal<CountingMethodSetting>,
    show_characters: Signal<bool>,
    // ── Writing games ──
    // Which surfaces "Always forward" covers. Whether it is being *played* is
    // per-`Work` session state and lives on `WritingGamesViewModel`, not here.
    games_forward_prose: Signal<bool>,
    games_forward_synopsis: Signal<bool>,
    // ── Corkboard ──
    corkboard_nested: Signal<bool>,
    corkboard_card_size: Signal<f32>,
    corkboard_show_word_count: Signal<bool>,
    corkboard_show_card_numbers: Signal<bool>,
    corkboard_modal_size: Signal<f32>,
}

// Accessors/setters are the feature's public API; bound to widgets incrementally.
#[allow(dead_code)]
impl SettingsViewModel {
    pub fn new(store: &SettingsStore) -> Self {
        Self {
            dark: store.signal(DARK_KEY, false),
            locale: store.signal(LOCALE_KEY, "en-US".to_string()),
            user_name: store.signal(USER_NAME_KEY, String::new()),
            user_initials: store.signal(USER_INITIALS_KEY, String::new()),
            column_width: store.signal(EDITOR_WIDTH_KEY, EDITOR_WIDTH_DEFAULT),
            preview_width: store.signal(PREVIEW_WIDTH_KEY, PREVIEW_WIDTH_DEFAULT),
            autosave: store.signal(AUTOSAVE_KEY, false),
            spellcheck_enabled: store.signal(SPELLCHECK_ENABLED_KEY, SPELLCHECK_ENABLED_DEFAULT),
            comments_visible: store.signal(COMMENTS_VISIBLE_KEY, COMMENTS_VISIBLE_DEFAULT),
            show_welcome: store.signal(SHOW_WELCOME_KEY, true),
            scene_typo: EditorTypography {
                font_family: store
                    .signal(SCENE_FONT_FAMILY_KEY, SCENE_FONT_FAMILY_DEFAULT.to_string()),
                size: store.signal(SCENE_SIZE_KEY, SCENE_SIZE_DEFAULT),
                line_height: store.signal(SCENE_LINE_HEIGHT_KEY, SCENE_LINE_HEIGHT_DEFAULT),
                first_line_indent: store
                    .signal(SCENE_FIRST_LINE_INDENT_KEY, SCENE_FIRST_LINE_INDENT_DEFAULT),
                para_spacing_before: store.signal(
                    SCENE_PARA_SPACING_BEFORE_KEY,
                    SCENE_PARA_SPACING_BEFORE_DEFAULT,
                ),
                para_spacing_after: store.signal(
                    SCENE_PARA_SPACING_AFTER_KEY,
                    SCENE_PARA_SPACING_AFTER_DEFAULT,
                ),
            },
            synopsis_typo: EditorTypography {
                font_family: store.signal(
                    SYNOPSIS_FONT_FAMILY_KEY,
                    SYNOPSIS_FONT_FAMILY_DEFAULT.to_string(),
                ),
                size: store.signal(SYNOPSIS_SIZE_KEY, SYNOPSIS_SIZE_DEFAULT),
                line_height: store.signal(SYNOPSIS_LINE_HEIGHT_KEY, SYNOPSIS_LINE_HEIGHT_DEFAULT),
                first_line_indent: store.signal(
                    SYNOPSIS_FIRST_LINE_INDENT_KEY,
                    SYNOPSIS_FIRST_LINE_INDENT_DEFAULT,
                ),
                para_spacing_before: store.signal(
                    SYNOPSIS_PARA_SPACING_BEFORE_KEY,
                    SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT,
                ),
                para_spacing_after: store.signal(
                    SYNOPSIS_PARA_SPACING_AFTER_KEY,
                    SYNOPSIS_PARA_SPACING_AFTER_DEFAULT,
                ),
            },
            notes_typo: EditorTypography {
                font_family: store
                    .signal(NOTES_FONT_FAMILY_KEY, NOTES_FONT_FAMILY_DEFAULT.to_string()),
                size: store.signal(NOTES_SIZE_KEY, NOTES_SIZE_DEFAULT),
                line_height: store.signal(NOTES_LINE_HEIGHT_KEY, NOTES_LINE_HEIGHT_DEFAULT),
                first_line_indent: store
                    .signal(NOTES_FIRST_LINE_INDENT_KEY, NOTES_FIRST_LINE_INDENT_DEFAULT),
                para_spacing_before: store.signal(
                    NOTES_PARA_SPACING_BEFORE_KEY,
                    NOTES_PARA_SPACING_BEFORE_DEFAULT,
                ),
                para_spacing_after: store.signal(
                    NOTES_PARA_SPACING_AFTER_KEY,
                    NOTES_PARA_SPACING_AFTER_DEFAULT,
                ),
            },
            corkboard_typo: EditorTypography {
                font_family: store.signal(
                    CORKBOARD_FONT_FAMILY_KEY,
                    CORKBOARD_FONT_FAMILY_DEFAULT.to_string(),
                ),
                size: store.signal(CORKBOARD_SIZE_KEY, CORKBOARD_SIZE_DEFAULT),
                line_height: store.signal(CORKBOARD_LINE_HEIGHT_KEY, CORKBOARD_LINE_HEIGHT_DEFAULT),
                first_line_indent: store.signal(
                    CORKBOARD_FIRST_LINE_INDENT_KEY,
                    CORKBOARD_FIRST_LINE_INDENT_DEFAULT,
                ),
                para_spacing_before: store.signal(
                    CORKBOARD_PARA_SPACING_BEFORE_KEY,
                    CORKBOARD_PARA_SPACING_BEFORE_DEFAULT,
                ),
                para_spacing_after: store.signal(
                    CORKBOARD_PARA_SPACING_AFTER_KEY,
                    CORKBOARD_PARA_SPACING_AFTER_DEFAULT,
                ),
            },
            distraction_free_typo: EditorTypography {
                font_family: store.signal(
                    DISTRACTION_FREE_FONT_FAMILY_KEY,
                    DISTRACTION_FREE_FONT_FAMILY_DEFAULT.to_string(),
                ),
                size: store.signal(DISTRACTION_FREE_SIZE_KEY, DISTRACTION_FREE_SIZE_DEFAULT),
                line_height: store.signal(
                    DISTRACTION_FREE_LINE_HEIGHT_KEY,
                    DISTRACTION_FREE_LINE_HEIGHT_DEFAULT,
                ),
                first_line_indent: store.signal(
                    DISTRACTION_FREE_FIRST_LINE_INDENT_KEY,
                    DISTRACTION_FREE_FIRST_LINE_INDENT_DEFAULT,
                ),
                para_spacing_before: store.signal(
                    DISTRACTION_FREE_PARA_SPACING_BEFORE_KEY,
                    DISTRACTION_FREE_PARA_SPACING_BEFORE_DEFAULT,
                ),
                para_spacing_after: store.signal(
                    DISTRACTION_FREE_PARA_SPACING_AFTER_KEY,
                    DISTRACTION_FREE_PARA_SPACING_AFTER_DEFAULT,
                ),
            },
            distraction_free_width: store
                .signal(DISTRACTION_FREE_WIDTH_KEY, DISTRACTION_FREE_WIDTH_DEFAULT),
            distraction_free_theme: store.signal(
                DISTRACTION_FREE_THEME_KEY,
                DISTRACTION_FREE_THEME_DEFAULT.to_string(),
            ),
            distraction_free_title: store
                .signal(DISTRACTION_FREE_TITLE_KEY, DISTRACTION_FREE_TITLE_DEFAULT),
            distraction_free_word_count: store.signal(
                DISTRACTION_FREE_WORD_COUNT_KEY,
                DISTRACTION_FREE_WORD_COUNT_DEFAULT,
            ),
            distraction_free_session: store.signal(
                DISTRACTION_FREE_SESSION_KEY,
                DISTRACTION_FREE_SESSION_DEFAULT,
            ),
            distraction_free_go: store.signal(DISTRACTION_FREE_GO_KEY, DISTRACTION_FREE_GO_DEFAULT),
            distraction_free_go_to: store
                .signal(DISTRACTION_FREE_GO_TO_KEY, DISTRACTION_FREE_GO_TO_DEFAULT),
            synopsis_pane: store.signal(SYNOPSIS_PANE_KEY, SYNOPSIS_PANE_DEFAULT),
            synopsis_placement: store.signal(SYNOPSIS_PLACEMENT_KEY, SynopsisPlacement::default()),
            synopsis_side_width: store.signal(SYNOPSIS_SIDE_WIDTH_KEY, SYNOPSIS_SIDE_WIDTH_DEFAULT),
            punct_dashes: store.signal(PUNCT_DASHES_KEY, PUNCT_DASHES_DEFAULT),
            punct_ellipsis: store.signal(PUNCT_ELLIPSIS_KEY, PUNCT_ELLIPSIS_DEFAULT),
            punct_quotes: store.signal(PUNCT_QUOTES_KEY, PUNCT_QUOTES_DEFAULT),
            punct_quote_style: store.signal(PUNCT_QUOTE_STYLE_KEY, QuoteStyle::default()),
            punct_spacing: store.signal(PUNCT_SPACING_KEY, PUNCT_SPACING_DEFAULT),
            punct_dialogue: store.signal(PUNCT_DIALOGUE_KEY, PUNCT_DIALOGUE_DEFAULT),
            typewriter: store.signal(TYPEWRITER_KEY, TYPEWRITER_DEFAULT),
            typewriter_anchor: store
                .signal(TYPEWRITER_ANCHOR_KEY, Some(TypewriterAnchor::default())),
            highlight_scope: store.signal(HIGHLIGHT_SCOPE_KEY, HighlightScope::default()),
            remember_view: store.signal(REMEMBER_VIEW_KEY, REMEMBER_VIEW_DEFAULT),
            counting_method: store
                .signal(GOALS_COUNTING_METHOD_KEY, CountingMethodSetting::default()),
            show_characters: store.signal(GOALS_SHOW_CHARACTERS_KEY, GOALS_SHOW_CHARACTERS_DEFAULT),
            games_forward_prose: store.signal(
                GAMES_FORWARD_PROSE_KEY,
                crate::writing_session::FORWARD_PROSE_DEFAULT,
            ),
            games_forward_synopsis: store.signal(
                GAMES_FORWARD_SYNOPSIS_KEY,
                crate::writing_session::FORWARD_SYNOPSIS_DEFAULT,
            ),
            corkboard_nested: store.signal(CORKBOARD_NESTED_KEY, CORKBOARD_NESTED_DEFAULT),
            corkboard_card_size: store.signal(CORKBOARD_CARD_SIZE_KEY, CORKBOARD_CARD_SIZE_DEFAULT),
            corkboard_show_word_count: store.signal(
                CORKBOARD_SHOW_WORD_COUNT_KEY,
                CORKBOARD_SHOW_WORD_COUNT_DEFAULT,
            ),
            corkboard_show_card_numbers: store.signal(
                CORKBOARD_SHOW_CARD_NUMBERS_KEY,
                CORKBOARD_SHOW_CARD_NUMBERS_DEFAULT,
            ),
            corkboard_modal_size: store
                .signal(CORKBOARD_MODAL_SIZE_KEY, CORKBOARD_MODAL_SIZE_DEFAULT),
        }
    }

    /// Whether to autosave to disk (hides the manual Save affordances when on).
    /// Store-backed, so toggling it persists.
    pub fn autosave(&self) -> Signal<bool> {
        self.autosave.clone()
    }

    /// The master spell-check switch (default **on**) — the one truth behind the title-bar
    /// toggle, View ▸ Check spelling, F7 and the Settings ▸ Spelling row. Store-backed, so
    /// every surface reads the same signal and toggling it persists.
    ///
    /// Distinct from the per-language pill mutes (session-only, one dictionary each): this is
    /// the "no squiggles at all" answer a writer actually looks for.
    pub fn spellcheck_enabled(&self) -> Signal<bool> {
        self.spellcheck_enabled.clone()
    }

    /// Whether anchored comments are drawn in the editor — Tools ▸ Comments.
    pub fn comments_visible(&self) -> Signal<bool> {
        self.comments_visible.clone()
    }

    /// Whether to show the Welcome modal at startup (default on). Same cached
    /// `SHOW_WELCOME_KEY` signal the Welcome dialog's inline checkbox binds.
    pub fn show_welcome(&self) -> Signal<bool> {
        self.show_welcome.clone()
    }

    // ── reactive accessors for binding ──
    pub fn column_width(&self) -> Signal<f32> {
        self.column_width.clone()
    }
    /// Max width of the search preview editor (bottom band).
    pub fn preview_width(&self) -> Signal<f32> {
        self.preview_width.clone()
    }
    pub fn dark(&self) -> Signal<bool> {
        self.dark.clone()
    }
    pub fn locale(&self) -> Signal<String> {
        self.locale.clone()
    }

    /// The signing name for comments — empty when unset, which the resolver in
    /// [`crate::comments::signature`] reads as "fall back to the book's byline".
    pub fn user_name(&self) -> Signal<String> {
        self.user_name.clone()
    }

    /// Explicit initials — empty means "derive them from the name", never
    /// "show none".
    pub fn user_initials(&self) -> Signal<String> {
        self.user_initials.clone()
    }

    /// The per-editor-type typography bundles (Scene / Synopsis / Notes / Corkboard).
    /// Every call returns clones of the same live signals, so a settings edit
    /// fans out to every open editor tab that holds them.
    pub fn editor_typography(&self) -> EditorTypographySet {
        EditorTypographySet {
            scene: self.scene_typo.clone(),
            synopsis: self.synopsis_typo.clone(),
            notes: self.notes_typo.clone(),
            corkboard: self.corkboard_typo.clone(),
            distraction_free: self.distraction_free_typo.clone(),
        }
    }

    /// The corkboard card synopsis typography bundle — bound by the Corkboard
    /// settings pane, read by every open board's cards.
    pub fn corkboard_typo(&self) -> EditorTypography {
        self.corkboard_typo.clone()
    }

    /// Max width of the writing column while distraction-free mode is active
    /// (Settings ▸ Editor ▸ Editor Behavior's "Distraction-free" group) —
    /// independent from [`Self::column_width`].
    pub fn distraction_free_width(&self) -> Signal<f32> {
        self.distraction_free_width.clone()
    }

    // ── Which chrome distraction-free mode keeps ─────────────────────────
    //
    // Read by `App::build` (the tab strip) and `FocusStrip` (the other three).
    // Each gates a `VisibleWhen` *inside* the mode; none of them has any
    // effect while the mode is off.

    /// Keep the editor tab strip while distraction-free mode is active
    /// (default off).
    /// Keep the word count in the distraction-free strip (default on).
    pub fn distraction_free_theme(&self) -> Signal<String> {
        self.distraction_free_theme.clone()
    }

    pub fn distraction_free_title(&self) -> Signal<bool> {
        self.distraction_free_title.clone()
    }

    pub fn distraction_free_word_count(&self) -> Signal<bool> {
        self.distraction_free_word_count.clone()
    }
    /// Keep the writing-session readout in the distraction-free strip
    /// (default on).
    pub fn distraction_free_session(&self) -> Signal<bool> {
        self.distraction_free_session.clone()
    }
    /// Keep the Previous/Next pair in the distraction-free strip (default on).
    pub fn distraction_free_go(&self) -> Signal<bool> {
        self.distraction_free_go.clone()
    }
    /// Keep the "Go to…" jump button in the distraction-free strip (default on).
    pub fn distraction_free_go_to(&self) -> Signal<bool> {
        self.distraction_free_go_to.clone()
    }

    /// Show the synopsis pane above the manuscript. Consumed live by the writing
    /// editor (`tabs::shared::prose`).
    pub fn synopsis_pane(&self) -> Signal<bool> {
        self.synopsis_pane.clone()
    }

    /// Where that pane sits — above the manuscript or beside it. A preference,
    /// not a guarantee: a tab too narrow for two columns renders Top regardless.
    pub fn synopsis_placement(&self) -> Signal<SynopsisPlacement> {
        self.synopsis_placement.clone()
    }

    /// Width of the Side synopsis column. Written back when its divider is
    /// dragged, so the next tab opens at the width the writer settled on.
    pub fn synopsis_side_width(&self) -> Signal<f32> {
        self.synopsis_side_width.clone()
    }
    // ── Smart punctuation, application-level ─────────────────────────────
    pub fn punct_dashes(&self) -> Signal<bool> {
        self.punct_dashes.clone()
    }
    pub fn punct_ellipsis(&self) -> Signal<bool> {
        self.punct_ellipsis.clone()
    }
    pub fn punct_quotes(&self) -> Signal<bool> {
        self.punct_quotes.clone()
    }
    pub fn punct_quote_style(&self) -> Signal<QuoteStyle> {
        self.punct_quote_style.clone()
    }
    pub fn punct_spacing(&self) -> Signal<bool> {
        self.punct_spacing.clone()
    }
    pub fn punct_dialogue(&self) -> Signal<bool> {
        self.punct_dialogue.clone()
    }

    /// Typewriter scrolling (keep the caret line centred).
    pub fn typewriter(&self) -> Signal<bool> {
        self.typewriter.clone()
    }
    /// Which height the pinned line sits at. `Option` because it binds straight
    /// to a `ComboBox` selection; read it through
    /// [`TypewriterAnchor::resolve`] rather than unwrapping.
    pub fn typewriter_anchor(&self) -> Signal<Option<TypewriterAnchor>> {
        self.typewriter_anchor.clone()
    }
    /// How much text around the caret gets an ambient band while you write.
    pub fn highlight_scope(&self) -> Signal<HighlightScope> {
        self.highlight_scope.clone()
    }
    /// Remember the last `SegmentedControl` view per container type (Book / Part /
    /// Chapter). Same cached `REMEMBER_VIEW_KEY` signal [`EditorViewMemory`] reads.
    pub fn remember_view(&self) -> Signal<bool> {
        self.remember_view.clone()
    }

    /// The word-counting method for the live status-bar / focused count (a global
    /// USER preference). `Auto` resolves per scene language; the canonical progress
    /// snapshot ignores this and always counts with `Auto`.
    pub fn counting_method(&self) -> Signal<CountingMethodSetting> {
        self.counting_method.clone()
    }
    /// Show the character count beside the word count in the status bar.
    pub fn show_characters(&self) -> Signal<bool> {
        self.show_characters.clone()
    }

    // ── Writing games (which surfaces "Always forward" covers) ──
    /// Whether "Always forward" freezes manuscript prose while it is played.
    pub fn games_forward_prose(&self) -> Signal<bool> {
        self.games_forward_prose.clone()
    }
    /// Whether "Always forward" also freezes synopses while it is played.
    pub fn games_forward_synopsis(&self) -> Signal<bool> {
        self.games_forward_synopsis.clone()
    }

    // ── Corkboard (default mode + card presentation; shared live into every tab) ──
    /// Default corkboard mode: `true` = nested (direct children, drillable),
    /// `false` = flat (all descendant leaves).
    pub fn corkboard_nested(&self) -> Signal<bool> {
        self.corkboard_nested.clone()
    }
    /// Corkboard card size (minimum tile width, px) — the size slider drives it.
    pub fn corkboard_card_size(&self) -> Signal<f32> {
        self.corkboard_card_size.clone()
    }
    /// Show a card's word count in its footer.
    pub fn corkboard_show_word_count(&self) -> Signal<bool> {
        self.corkboard_show_word_count.clone()
    }
    /// Number the cards in board order.
    pub fn corkboard_show_card_numbers(&self) -> Signal<bool> {
        self.corkboard_show_card_numbers.clone()
    }
    /// The expanded-synopsis editor's own font-size scale.
    pub fn corkboard_modal_size(&self) -> Signal<f32> {
        self.corkboard_modal_size.clone()
    }

    /// The corkboard defaults bundle threaded into every container tab's
    /// [`CorkboardViewModel`](crate::corkboard::CorkboardViewModel). Bundled so a
    /// tab constructor takes one handle rather than five signals. `counting_method`
    /// is the same live setting the status-bar word count uses.
    pub fn corkboard_defaults(&self) -> CorkboardDefaults {
        CorkboardDefaults {
            nested: self.corkboard_nested.clone(),
            card_size: self.corkboard_card_size.clone(),
            show_word_count: self.corkboard_show_word_count.clone(),
            show_card_numbers: self.corkboard_show_card_numbers.clone(),
            modal_size: self.corkboard_modal_size.clone(),
            counting_method: self.counting_method.clone(),
        }
    }

    // ── business API ──

    /// Switch theme live and persist the choice.
    pub fn set_dark(&self, ctx: &mut EventContext, dark: bool) {
        ctx.set_theme(if dark { intui::dark() } else { intui::light() });
        self.dark.set(dark); // same cached signal → persisted
    }

    /// Switch locale live and persist the choice.
    pub fn set_locale(&self, ctx: &mut EventContext, locale: &str) {
        ctx.set_locale(locale);
        self.locale.set(locale.to_string());
    }

    /// Set the centered writing-column width (persisted; every open editor
    /// resizes live because they share this signal).
    pub fn set_column_width(&self, w: f32) {
        self.column_width.set(w);
    }

    /// Set the search preview editor's max width (persisted).
    pub fn set_preview_width(&self, w: f32) {
        self.preview_width.set(w);
    }

    /// Reset every setting this VM owns to its default (used by the Settings
    /// window's "Reset to defaults"). Theme / locale / text-scale live outside
    /// this VM, so the panel resets those alongside this call.
    pub fn reset_editor_defaults(&self) {
        self.column_width.set(EDITOR_WIDTH_DEFAULT);
        self.preview_width.set(PREVIEW_WIDTH_DEFAULT);
        self.autosave.set(false);
        self.spellcheck_enabled.set(SPELLCHECK_ENABLED_DEFAULT);
        self.comments_visible.set(COMMENTS_VISIBLE_DEFAULT);
        self.show_welcome.set(true);
        // Scene
        self.scene_typo
            .font_family
            .set(SCENE_FONT_FAMILY_DEFAULT.to_string());
        self.scene_typo.size.set(SCENE_SIZE_DEFAULT);
        self.scene_typo.line_height.set(SCENE_LINE_HEIGHT_DEFAULT);
        self.scene_typo
            .first_line_indent
            .set(SCENE_FIRST_LINE_INDENT_DEFAULT);
        self.scene_typo
            .para_spacing_before
            .set(SCENE_PARA_SPACING_BEFORE_DEFAULT);
        self.scene_typo
            .para_spacing_after
            .set(SCENE_PARA_SPACING_AFTER_DEFAULT);
        // Synopsis
        self.synopsis_typo
            .font_family
            .set(SYNOPSIS_FONT_FAMILY_DEFAULT.to_string());
        self.synopsis_typo.size.set(SYNOPSIS_SIZE_DEFAULT);
        self.synopsis_typo
            .line_height
            .set(SYNOPSIS_LINE_HEIGHT_DEFAULT);
        self.synopsis_typo
            .first_line_indent
            .set(SYNOPSIS_FIRST_LINE_INDENT_DEFAULT);
        self.synopsis_typo
            .para_spacing_before
            .set(SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT);
        self.synopsis_typo
            .para_spacing_after
            .set(SYNOPSIS_PARA_SPACING_AFTER_DEFAULT);
        // Notes
        self.notes_typo
            .font_family
            .set(NOTES_FONT_FAMILY_DEFAULT.to_string());
        self.notes_typo.size.set(NOTES_SIZE_DEFAULT);
        self.notes_typo.line_height.set(NOTES_LINE_HEIGHT_DEFAULT);
        self.notes_typo
            .first_line_indent
            .set(NOTES_FIRST_LINE_INDENT_DEFAULT);
        self.notes_typo
            .para_spacing_before
            .set(NOTES_PARA_SPACING_BEFORE_DEFAULT);
        self.notes_typo
            .para_spacing_after
            .set(NOTES_PARA_SPACING_AFTER_DEFAULT);
        // Corkboard card synopsis
        self.corkboard_typo
            .font_family
            .set(CORKBOARD_FONT_FAMILY_DEFAULT.to_string());
        self.corkboard_typo.size.set(CORKBOARD_SIZE_DEFAULT);
        self.corkboard_typo
            .line_height
            .set(CORKBOARD_LINE_HEIGHT_DEFAULT);
        self.corkboard_typo
            .first_line_indent
            .set(CORKBOARD_FIRST_LINE_INDENT_DEFAULT);
        self.corkboard_typo
            .para_spacing_before
            .set(CORKBOARD_PARA_SPACING_BEFORE_DEFAULT);
        self.corkboard_typo
            .para_spacing_after
            .set(CORKBOARD_PARA_SPACING_AFTER_DEFAULT);
        // Distraction-free
        self.distraction_free_typo
            .font_family
            .set(DISTRACTION_FREE_FONT_FAMILY_DEFAULT.to_string());
        self.distraction_free_typo
            .size
            .set(DISTRACTION_FREE_SIZE_DEFAULT);
        self.distraction_free_typo
            .line_height
            .set(DISTRACTION_FREE_LINE_HEIGHT_DEFAULT);
        self.distraction_free_typo
            .first_line_indent
            .set(DISTRACTION_FREE_FIRST_LINE_INDENT_DEFAULT);
        self.distraction_free_typo
            .para_spacing_before
            .set(DISTRACTION_FREE_PARA_SPACING_BEFORE_DEFAULT);
        self.distraction_free_typo
            .para_spacing_after
            .set(DISTRACTION_FREE_PARA_SPACING_AFTER_DEFAULT);
        self.distraction_free_width
            .set(DISTRACTION_FREE_WIDTH_DEFAULT);
        self.distraction_free_theme
            .set(DISTRACTION_FREE_THEME_DEFAULT.to_string());
        self.distraction_free_title
            .set(DISTRACTION_FREE_TITLE_DEFAULT);
        self.distraction_free_word_count
            .set(DISTRACTION_FREE_WORD_COUNT_DEFAULT);
        self.distraction_free_session
            .set(DISTRACTION_FREE_SESSION_DEFAULT);
        self.distraction_free_go.set(DISTRACTION_FREE_GO_DEFAULT);
        self.distraction_free_go_to
            .set(DISTRACTION_FREE_GO_TO_DEFAULT);
        self.synopsis_pane.set(SYNOPSIS_PANE_DEFAULT);
        self.synopsis_placement.set(SynopsisPlacement::default());
        self.synopsis_side_width.set(SYNOPSIS_SIDE_WIDTH_DEFAULT);
        self.punct_dashes.set(PUNCT_DASHES_DEFAULT);
        self.punct_ellipsis.set(PUNCT_ELLIPSIS_DEFAULT);
        self.punct_quotes.set(PUNCT_QUOTES_DEFAULT);
        self.punct_quote_style.set(QuoteStyle::default());
        self.punct_spacing.set(PUNCT_SPACING_DEFAULT);
        self.punct_dialogue.set(PUNCT_DIALOGUE_DEFAULT);
        self.typewriter.set(TYPEWRITER_DEFAULT);
        self.typewriter_anchor
            .set(Some(TypewriterAnchor::default()));
        self.highlight_scope.set(HighlightScope::default());
        self.counting_method.set(CountingMethodSetting::default());
        self.show_characters.set(GOALS_SHOW_CHARACTERS_DEFAULT);
        self.games_forward_prose
            .set(crate::writing_session::FORWARD_PROSE_DEFAULT);
        self.games_forward_synopsis
            .set(crate::writing_session::FORWARD_SYNOPSIS_DEFAULT);
        self.corkboard_nested.set(CORKBOARD_NESTED_DEFAULT);
        self.corkboard_card_size.set(CORKBOARD_CARD_SIZE_DEFAULT);
        self.corkboard_show_word_count
            .set(CORKBOARD_SHOW_WORD_COUNT_DEFAULT);
        self.corkboard_show_card_numbers
            .set(CORKBOARD_SHOW_CARD_NUMBERS_DEFAULT);
        self.corkboard_modal_size.set(CORKBOARD_MODAL_SIZE_DEFAULT);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        CORKBOARD_SIZE_DEFAULT, DISTRACTION_FREE_SIZE_DEFAULT, DISTRACTION_FREE_WIDTH_DEFAULT,
        NOTES_FONT_FAMILY_DEFAULT, SCENE_FIRST_LINE_INDENT_DEFAULT, SCENE_FONT_FAMILY_DEFAULT,
        SCENE_LINE_HEIGHT_DEFAULT, SCENE_SIZE_DEFAULT, SYNOPSIS_SIZE_DEFAULT,
    };
    use std::sync::atomic::{AtomicU32, Ordering};

    fn temp_store() -> SettingsStore {
        static N: AtomicU32 = AtomicU32::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "skribisto_settings_test_{}_{n}.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        SettingsStore::open(path).expect("open temp settings store")
    }

    /// Reset restores every per-type typography signal (the biggest, most
    /// error-prone part of the VM) to its `_DEFAULT` constant — all four bundles.
    #[test]
    fn reset_editor_defaults_restores_all_typography_bundles() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);
        let t = vm.editor_typography();
        for b in [
            &t.scene,
            &t.synopsis,
            &t.notes,
            &t.corkboard,
            &t.distraction_free,
        ] {
            b.font_family.set("EB Garamond".into());
            b.size.set(1.3);
            b.line_height.set(2.1);
            b.first_line_indent.set(40.0);
        }
        vm.distraction_free_width().set(999.0);

        vm.reset_editor_defaults();

        let t = vm.editor_typography();
        assert_eq!(t.scene.font_family.get(), SCENE_FONT_FAMILY_DEFAULT);
        assert_eq!(t.scene.size.get(), SCENE_SIZE_DEFAULT);
        assert_eq!(t.scene.line_height.get(), SCENE_LINE_HEIGHT_DEFAULT);
        assert_eq!(
            t.scene.first_line_indent.get(),
            SCENE_FIRST_LINE_INDENT_DEFAULT
        );
        assert_eq!(t.synopsis.size.get(), SYNOPSIS_SIZE_DEFAULT);
        assert_eq!(t.notes.font_family.get(), NOTES_FONT_FAMILY_DEFAULT);
        assert_eq!(t.corkboard.size.get(), CORKBOARD_SIZE_DEFAULT);
        assert_eq!(t.distraction_free.size.get(), DISTRACTION_FREE_SIZE_DEFAULT);
        assert_eq!(
            vm.distraction_free_width().get(),
            DISTRACTION_FREE_WIDTH_DEFAULT
        );
        // None of the five still holds the mutated value.
        for b in [
            &t.scene,
            &t.synopsis,
            &t.notes,
            &t.corkboard,
            &t.distraction_free,
        ] {
            assert_ne!(b.font_family.get(), "EB Garamond");
            assert_ne!(b.first_line_indent.get(), 40.0);
        }
    }

    /// The four distraction-free chrome toggles round-trip through the store
    /// and are restored by Reset. Easy to add a setting and forget the reset
    /// arm — the typography test above only covers the bundles.
    /// The caret band's scope is an enum in a store that speaks TOML scalars, so each variant
    /// has to survive the round trip — and Reset has to reach it like every other editor
    /// preference.
    #[test]
    fn the_highlight_scope_persists_every_variant_and_resets() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);
        assert_eq!(
            vm.highlight_scope().get(),
            HighlightScope::None,
            "a writing app looks plain until asked otherwise"
        );

        for scope in [
            HighlightScope::Sentence,
            HighlightScope::Paragraph,
            HighlightScope::None,
        ] {
            vm.highlight_scope().set(scope);
            let reopened = SettingsViewModel::new(&store);
            assert_eq!(
                reopened.highlight_scope().get(),
                scope,
                "{scope:?} must survive a fresh view-model over the same store"
            );
        }

        vm.highlight_scope().set(HighlightScope::Paragraph);
        vm.reset_editor_defaults();
        assert_eq!(vm.highlight_scope().get(), HighlightScope::default());
    }

    /// The synopsis placement and its column width round-trip the store and come
    /// back from "Reset to defaults" — the latter being the step with no drift
    /// test behind it, so a new setting silently escapes the reset button unless
    /// something like this pins it.
    #[test]
    fn the_synopsis_placement_and_width_persist_and_reset() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);
        assert_eq!(
            vm.synopsis_placement().get(),
            SynopsisPlacement::Top,
            "existing projects must look exactly as they did"
        );
        assert_eq!(
            vm.synopsis_side_width().get(),
            crate::SYNOPSIS_SIDE_WIDTH_DEFAULT
        );

        for placement in SynopsisPlacement::all() {
            vm.synopsis_placement().set(placement);
            let reopened = SettingsViewModel::new(&store);
            assert_eq!(
                reopened.synopsis_placement().get(),
                placement,
                "{placement:?} must survive a fresh view-model over the same store"
            );
        }

        vm.synopsis_side_width().set(340.0);
        assert_eq!(
            SettingsViewModel::new(&store).synopsis_side_width().get(),
            340.0,
            "a dragged divider width is remembered for the next tab"
        );

        vm.synopsis_placement().set(SynopsisPlacement::Side);
        vm.reset_editor_defaults();
        assert_eq!(vm.synopsis_placement().get(), SynopsisPlacement::default());
        assert_eq!(
            vm.synopsis_side_width().get(),
            crate::SYNOPSIS_SIDE_WIDTH_DEFAULT
        );
    }

    #[test]
    fn distraction_free_chrome_toggles_persist_and_reset() {
        use crate::{
            DISTRACTION_FREE_GO_DEFAULT, DISTRACTION_FREE_GO_TO_DEFAULT,
            DISTRACTION_FREE_SESSION_DEFAULT, DISTRACTION_FREE_WORD_COUNT_DEFAULT,
        };
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);

        // Every strip item starts shown; the writer opts each one away.
        assert!(vm.distraction_free_word_count().get());
        assert!(vm.distraction_free_session().get());
        assert!(vm.distraction_free_go().get());
        assert!(vm.distraction_free_go_to().get());

        // Flip every one away from its default...
        vm.distraction_free_word_count().set(false);
        vm.distraction_free_session().set(false);
        vm.distraction_free_go().set(false);
        vm.distraction_free_go_to().set(false);

        // ...they survive a fresh view-model over the same store (persisted,
        // not merely cached in this instance)...
        let reopened = SettingsViewModel::new(&store);
        assert!(!reopened.distraction_free_word_count().get());
        assert!(!reopened.distraction_free_session().get());
        assert!(!reopened.distraction_free_go().get());
        assert!(!reopened.distraction_free_go_to().get());

        // ...and Reset puts all four back.
        vm.reset_editor_defaults();
        assert_eq!(
            vm.distraction_free_word_count().get(),
            DISTRACTION_FREE_WORD_COUNT_DEFAULT
        );
        assert_eq!(
            vm.distraction_free_session().get(),
            DISTRACTION_FREE_SESSION_DEFAULT
        );
        assert_eq!(vm.distraction_free_go().get(), DISTRACTION_FREE_GO_DEFAULT);
        assert_eq!(
            vm.distraction_free_go_to().get(),
            DISTRACTION_FREE_GO_TO_DEFAULT
        );
    }

    #[test]
    fn view_memory_round_trips_per_type() {
        use BinderItemSubRole::*;
        let m = EditorViewMemory::detached(true);
        let id = |s: &str| Some(seg::segment_id(s));
        assert_eq!(
            m.initial(&Book),
            id(seg::SEG_OWN),
            "starts on the container's own page"
        );
        m.remember(&Book, seg::SEG_ANALYSIS);
        assert_eq!(
            m.initial(&Book),
            id(seg::SEG_ANALYSIS),
            "a new Book tab inherits the last view"
        );
        // Per-type isolation.
        m.remember(&ChapterScene, seg::SEG_MANUSCRIPT);
        assert_eq!(m.initial(&ChapterScene), id(seg::SEG_MANUSCRIPT));
        assert_eq!(
            m.initial(&Book),
            id(seg::SEG_ANALYSIS),
            "types don't cross-contaminate"
        );
        // A non-segmented type (a plain Scene) has no view memory.
        m.remember(&Scene, seg::SEG_MANUSCRIPT);
        assert_eq!(m.initial(&Scene), Option::None);
    }

    /// A notes folder remembers its view like every other segmented container.
    ///
    /// `stored()` had no `Note` arm, so this was a silent permanent no-op while the
    /// wrapper around `folder_synopsis_with_overview` claimed otherwise in a comment.
    #[test]
    fn a_notes_folder_remembers_its_view() {
        use BinderItemSubRole::*;
        let m = EditorViewMemory::detached(true);
        assert_eq!(
            m.initial(&Note),
            Some(seg::segment_id(seg::SEG_NOTES)),
            "a notes folder starts on its own page"
        );
        m.remember(&Note, seg::SEG_OVERVIEW);
        assert_eq!(
            m.initial(&Note),
            Some(seg::segment_id(seg::SEG_OVERVIEW)),
            "and returns to the view it was left on"
        );
        assert_eq!(
            m.initial(&Book),
            Some(seg::segment_id(seg::SEG_OWN)),
            "without touching another container type's memory"
        );
    }

    /// A front/back-matter folder shares the notes folder's memory.
    ///
    /// `folder_paratext::render` delegates to the same
    /// `shared::folder_synopsis_with_overview` as `folder_note`, so the two have the same
    /// two segments and the same choice to remember — one key, not two. Missed on the
    /// first pass of the `Note` fix, which left `Paratext` with the identical silent
    /// no-op the fix existed to remove.
    #[test]
    fn a_paratext_folder_shares_the_notes_view_memory() {
        use BinderItemSubRole::*;
        let m = EditorViewMemory::detached(true);
        m.remember(&Paratext, seg::SEG_OVERVIEW);
        assert_eq!(
            m.initial(&Paratext),
            Some(seg::segment_id(seg::SEG_OVERVIEW)),
            "a paratext folder must remember its view at all"
        );
        assert_eq!(
            m.initial(&Note),
            Some(seg::segment_id(seg::SEG_OVERVIEW)),
            "…out of the same store as the notes folder, since it is the same bar"
        );
    }

    #[test]
    fn view_memory_disabled_is_inert() {
        use BinderItemSubRole::*;
        let m = EditorViewMemory::detached(true);
        m.remember(&Part, seg::SEG_SYNOPSIS); // recorded while enabled
        m.enabled().set(false);
        assert_eq!(
            m.initial(&Part),
            Option::None,
            "disabled starts wherever the bar's first segment is"
        );
        m.remember(&Part, seg::SEG_MANUSCRIPT); // no-op while disabled
        m.enabled().set(true);
        assert_eq!(
            m.initial(&Part),
            Some(seg::segment_id(seg::SEG_SYNOPSIS)),
            "the disabled write was ignored"
        );
    }

    #[test]
    fn view_memory_shares_the_toggle_with_the_settings_vm() {
        // Both read the same cached `REMEMBER_VIEW_KEY` signal, so the panel toggle
        // drives the memory live.
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);
        let mem = EditorViewMemory::new(&store);
        vm.remember_view().set(false);
        assert!(!mem.enabled().get());
        vm.remember_view().set(true);
        assert!(mem.enabled().get());
    }
}
