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

use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use skribisto_model::counting::CountingMethodSetting;

use frontend::common::entities::QuoteStyle;

use crate::shared::{HighlightScope, SynopsisPlacement, TypewriterAnchor};

use crate::{
    AUTOSAVE_KEY, CHECK_FOR_UPDATES_DEFAULT, CHECK_FOR_UPDATES_KEY, COMMENTS_VISIBLE_DEFAULT,
    COMMENTS_VISIBLE_KEY, CORKBOARD_CARD_SIZE_DEFAULT, CORKBOARD_CARD_SIZE_KEY,
    CORKBOARD_FIRST_LINE_INDENT_DEFAULT, CORKBOARD_FIRST_LINE_INDENT_KEY,
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
    HIGHLIGHT_SCOPE_KEY, IMAGE_SIZE_POLICY_DEFAULT, IMAGE_SIZE_POLICY_KEY, LOCALE_KEY,
    MARGIN_LANE_ENABLED_DEFAULT, MARGIN_LANE_ENABLED_KEY, MARGIN_LANE_TEXTURE_DEFAULT,
    MARGIN_LANE_TEXTURE_KEY, NOTES_FIRST_LINE_INDENT_DEFAULT, NOTES_FIRST_LINE_INDENT_KEY,
    NOTES_FONT_FAMILY_DEFAULT, NOTES_FONT_FAMILY_KEY, NOTES_LINE_HEIGHT_DEFAULT,
    NOTES_LINE_HEIGHT_KEY, NOTES_PARA_SPACING_AFTER_DEFAULT, NOTES_PARA_SPACING_AFTER_KEY,
    NOTES_PARA_SPACING_BEFORE_DEFAULT, NOTES_PARA_SPACING_BEFORE_KEY, NOTES_SIZE_DEFAULT,
    NOTES_SIZE_KEY, PACE_SUMMARY_ON_OPEN_KEY, PREVIEW_WIDTH_DEFAULT, PREVIEW_WIDTH_KEY,
    PUNCT_DASHES_DEFAULT, PUNCT_DASHES_KEY, PUNCT_DIALOGUE_DEFAULT, PUNCT_DIALOGUE_KEY,
    PUNCT_ELLIPSIS_DEFAULT, PUNCT_ELLIPSIS_KEY, PUNCT_QUOTE_STYLE_KEY, PUNCT_QUOTES_DEFAULT,
    PUNCT_QUOTES_KEY, PUNCT_SPACING_DEFAULT, PUNCT_SPACING_KEY, REMEMBER_VIEW_DEFAULT,
    REMEMBER_VIEW_KEY, SCENE_FIRST_LINE_INDENT_DEFAULT, SCENE_FIRST_LINE_INDENT_KEY,
    SCENE_FONT_FAMILY_DEFAULT, SCENE_FONT_FAMILY_KEY, SCENE_LINE_HEIGHT_DEFAULT,
    SCENE_LINE_HEIGHT_KEY, SCENE_PARA_SPACING_AFTER_DEFAULT, SCENE_PARA_SPACING_AFTER_KEY,
    SCENE_PARA_SPACING_BEFORE_DEFAULT, SCENE_PARA_SPACING_BEFORE_KEY, SCENE_SIZE_DEFAULT,
    SCENE_SIZE_KEY, SHOW_WELCOME_KEY, SPELLCHECK_ENABLED_DEFAULT, SPELLCHECK_ENABLED_KEY,
    SYNOPSIS_FIRST_LINE_INDENT_DEFAULT, SYNOPSIS_FIRST_LINE_INDENT_KEY,
    SYNOPSIS_FONT_FAMILY_DEFAULT, SYNOPSIS_FONT_FAMILY_KEY, SYNOPSIS_LINE_HEIGHT_DEFAULT,
    SYNOPSIS_LINE_HEIGHT_KEY, SYNOPSIS_PANE_DEFAULT, SYNOPSIS_PANE_KEY,
    SYNOPSIS_PARA_SPACING_AFTER_DEFAULT, SYNOPSIS_PARA_SPACING_AFTER_KEY,
    SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT, SYNOPSIS_PARA_SPACING_BEFORE_KEY, SYNOPSIS_PLACEMENT_KEY,
    SYNOPSIS_SIDE_WIDTH_DEFAULT, SYNOPSIS_SIDE_WIDTH_KEY, SYNOPSIS_SIZE_DEFAULT, SYNOPSIS_SIZE_KEY,
    THEME_MODE_DARK, THEME_MODE_DEFAULT, THEME_MODE_KEY, THEME_MODE_LIGHT, THEME_MODE_SYSTEM,
    TYPEWRITER_ANCHOR_KEY, TYPEWRITER_DEFAULT, TYPEWRITER_KEY, USER_INITIALS_KEY, USER_NAME_KEY,
};

/// Which of the six writing surfaces a typography bundle dresses.
///
/// The six `Signal`s below cannot say this about themselves, and two callers
/// need it: Ctrl+0 (what does "default" mean for *this* editor?) and the
/// size gesture's toast (which of the six did the writer just change?).
/// Deliberately finer-grained than `format::EditorKind`, which only separates
/// prose from synopsis — a `Prose` editor may be dressed by Scene, Notes *or*
/// Distraction-free, and a `Synopsis` one by three different bundles again.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TypographyKind {
    #[default]
    Scene,
    Synopsis,
    Notes,
    Corkboard,
    /// The corkboard's **expanded** card editor, which has its own size signal
    /// and its own wider range — see [`TypographySizeRange`].
    CorkboardExpanded,
    DistractionFree,
}

/// The legal span of a bundle's `size`, and what it resets to.
///
/// A plain `Copy` value rather than `Signal`s: no bundle changes its own range
/// at runtime. Carried on the bundle so the Settings slider, Ctrl+Wheel and
/// Ctrl+= / Ctrl+− / Ctrl+0 all read one source and cannot offer each other
/// values the others reject.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TypographySizeRange {
    pub min: f32,
    pub max: f32,
    pub step: f32,
    /// The compile-time default this bundle's size resets to.
    pub default: f32,
    pub kind: TypographyKind,
}

impl TypographySizeRange {
    /// The range every bundle shares but the corkboard's expanded editor.
    pub const fn standard(kind: TypographyKind, default: f32) -> Self {
        Self {
            min: crate::EDITOR_TYPO_SIZE_MIN,
            max: crate::EDITOR_TYPO_SIZE_MAX,
            step: crate::EDITOR_TYPO_SIZE_STEP,
            default,
            kind,
        }
    }

    /// Clamp `value` into the range **and** snap it onto the step grid, so a
    /// value set by the wheel or the keyboard is always one the slider can
    /// also produce. Without the snap, `1.0 + 0.05 × 3` lands on
    /// `1.1500001` and the slider and the gesture quietly stop agreeing.
    pub fn snap(&self, value: f32) -> f32 {
        let clamped = value.clamp(self.min, self.max);
        let steps = ((clamped - self.min) / self.step).round();
        (self.min + steps * self.step).clamp(self.min, self.max)
    }
}

impl Default for TypographySizeRange {
    /// Only for the widget tests, which build bundles with no settings store
    /// behind them and never exercise the range.
    fn default() -> Self {
        Self::standard(TypographyKind::Scene, crate::SCENE_SIZE_DEFAULT)
    }
}

/// One editor type's typography knobs. Cheap to clone — every `Signal` field is
/// `SettingsStore`-cached, so all clones observe / drive the same live value.
/// `size` is a relative font-size scale (`1.0` = 100 %), composed with the
/// interface a11y text scale; `line_height` is a multiple of the font size;
/// `first_line_indent` is in px.
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
    /// Which bundle this is, how far its `size` may travel, and where it
    /// resets to. Not a `Signal` — see [`TypographySizeRange`].
    pub size_range: TypographySizeRange,
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
    /// A **single note**, not a notes folder.
    ///
    /// Its own slot because `stored` used to key on `sub_role` alone, and both
    /// `Folder/Note` and `Item/Note` answer to `Note`. They now carry different bars
    /// (a folder has Notes / Story bible / Overview; a note has its own prose, Details
    /// and In prose), so one shared slot would have each writing an id the other cannot
    /// resolve: every visit to a note's In prose segment would reset the notes folder to
    /// its first segment, silently and permanently, with no error anywhere.
    item_note: Signal<String>,
}

impl EditorViewMemory {
    pub fn new(store: &SettingsStore) -> Self {
        Self {
            enabled: store.signal(REMEMBER_VIEW_KEY, REMEMBER_VIEW_DEFAULT),
            book: store.signal("editor.last_view.book", seg::SEG_OWN.to_string()),
            part: store.signal("editor.last_view.part", seg::SEG_OWN.to_string()),
            chapter: store.signal("editor.last_view.chapter", seg::SEG_OWN.to_string()),
            note: store.signal("editor.last_view.note", seg::SEG_NOTES.to_string()),
            item_note: store.signal("editor.last_view.item_note", seg::SEG_NOTE_OWN.to_string()),
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
            item_note: Signal::new(seg::SEG_NOTE_OWN.to_string()),
        }
    }

    /// The `editor.remember_view` toggle (bound by the Settings panel).
    pub fn enabled(&self) -> Signal<bool> {
        self.enabled.clone()
    }

    /// The persisted last-view signal for a segmented container sub-role, or `None`
    /// for a type with no `SegmentedControl`.
    /// **Keyed on the pair, never on `sub_role` alone.** `Folder/Note` and `Item/Note`
    /// share a sub-role and carry entirely different bars; keying on the sub-role gave
    /// them one slot, so each would store an id the other resolves to nothing and falls
    /// back from. See [`Self::item_note`].
    fn stored(
        &self,
        role: &BinderItemRole,
        sub_role: &BinderItemSubRole,
    ) -> Option<Signal<String>> {
        use BinderItemSubRole::*;
        // A single note's own bar, distinct from the notes *folder* that shares its
        // sub-role.
        if *role == BinderItemRole::Item && matches!(sub_role, Note) {
            return Some(self.item_note.clone());
        }
        match sub_role {
            Book => Some(self.book.clone()),
            Part => Some(self.part.clone()),
            // The *folder* chapter (`Folder/ChapterScene`) is the only `ChapterScene`
            // container with a `SegmentedControl`; the flat chapter is a prose tab and
            // never asks for this.
            ChapterScene => Some(self.chapter.clone()),
            // A notes folder has a three-segment bar of its own
            // (`folder_synopsis_with_overview`: Notes, Story bible, Overview), and it was
            // wrapped in `RememberSegment` like the others while this arm was missing — so
            // its remembered view was a silent permanent no-op, and the comment claiming
            // otherwise was false.
            //
            // `Paratext` shares that arm, not a key of its own: `folder_paratext::render`
            // delegates to the very same `folder_synopsis_with_overview`, which drops the
            // Story bible segment for `Paratext` (it is not a story bible and cannot become
            // one), so a paratext folder's bar has two segments where a notes folder's has
            // three. They still share one remembered setting: giving Paratext its own key
            // would split one setting into two for one function's worth of bar.
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
    pub fn initial(
        &self,
        role: &BinderItemRole,
        sub_role: &BinderItemSubRole,
    ) -> Option<SegmentId> {
        if !self.enabled.get() {
            return Option::None;
        }
        self.stored(role, sub_role)
            .map(|s| s.get())
            .filter(|id| !id.is_empty())
            .map(|id| seg::segment_id(&id))
    }

    /// Record `segment` as the last view for `sub_role` (no-op when disabled, for a
    /// non-segmented type, or when already equal).
    pub fn remember(&self, role: &BinderItemRole, sub_role: &BinderItemSubRole, segment: &str) {
        if self.enabled.get()
            && let Some(s) = self.stored(role, sub_role)
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
    /// The store every signal above is cached in, kept so the handful of keys
    /// that are *synthesised* at runtime — one per margin-lane surface, one per
    /// registered lane provider — can be reached the same way. A field rather
    /// than a re-open: `SettingsStore` is `Rc`-backed, so this is the very same
    /// store, and a second `open` of the same path would be a second live copy
    /// of one file.
    store: SettingsStore,
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
    check_for_updates: Signal<bool>,
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
            store: store.clone(),
            dark: store.signal(DARK_KEY, false),
            // Deliberately a flat "en-US" rather than `startup::os_default_locale()`,
            // unlike every other reading of this key. This signal is the *persisted*
            // language, and `App`'s locale effect writes the live locale into it
            // whenever the two differ — so seeding it with the value detection just
            // produced would make the two agree, nothing would be written, and the
            // detected language would stay unrecorded. A seed that cannot match a
            // detected non-English locale is what turns the first launch's detection
            // into a choice on disk.
            locale: store.signal(LOCALE_KEY, "en-US".to_string()),
            user_name: store.signal(USER_NAME_KEY, String::new()),
            user_initials: store.signal(USER_INITIALS_KEY, String::new()),
            column_width: store.signal(EDITOR_WIDTH_KEY, EDITOR_WIDTH_DEFAULT),
            preview_width: store.signal(PREVIEW_WIDTH_KEY, PREVIEW_WIDTH_DEFAULT),
            autosave: store.signal(AUTOSAVE_KEY, false),
            spellcheck_enabled: store.signal(SPELLCHECK_ENABLED_KEY, SPELLCHECK_ENABLED_DEFAULT),
            comments_visible: store.signal(COMMENTS_VISIBLE_KEY, COMMENTS_VISIBLE_DEFAULT),
            show_welcome: store.signal(SHOW_WELCOME_KEY, true),
            check_for_updates: store.signal(CHECK_FOR_UPDATES_KEY, CHECK_FOR_UPDATES_DEFAULT),
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
                size_range: TypographySizeRange::standard(
                    TypographyKind::Scene,
                    SCENE_SIZE_DEFAULT,
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
                size_range: TypographySizeRange::standard(
                    TypographyKind::Synopsis,
                    SYNOPSIS_SIZE_DEFAULT,
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
                size_range: TypographySizeRange::standard(
                    TypographyKind::Notes,
                    NOTES_SIZE_DEFAULT,
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
                size_range: TypographySizeRange::standard(
                    TypographyKind::Corkboard,
                    CORKBOARD_SIZE_DEFAULT,
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
                size_range: TypographySizeRange::standard(
                    TypographyKind::DistractionFree,
                    DISTRACTION_FREE_SIZE_DEFAULT,
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

    /// Whether the once-a-day update check may run. See
    /// [`crate::CHECK_FOR_UPDATES_KEY`] for why this is opt-out.
    ///
    /// The value is honoured only where the channel checks at all; a Flathub or
    /// distribution install stays silent regardless, and the Settings row is not
    /// rendered there rather than being shown with no effect.
    pub fn check_for_updates(&self) -> Signal<bool> {
        self.check_for_updates.clone()
    }

    // ── Reached through the store rather than held as fields ─────────────
    //
    // `store.signal(key, default)` *seeds* a missing key, and the store writes
    // its whole table back to disk — so a field here is a key written into
    // every install the moment a view-model is constructed, which `App::build`
    // does on every launch. For the keys below that is either pointless (they
    // already have owners that seed them where they are actually used) or
    // actively wrong (see [`Self::theme_mode`]). Reaching them on demand keeps
    // the footprint of merely *having* a view-model at zero and costs nothing:
    // the store caches one cell per key, so these hand back the same live
    // signal every other reader holds.

    /// What to do with an over-large image on insert: `"ask"`, `"keep"` or
    /// `"downscale"`. The same cached [`crate::IMAGE_SIZE_POLICY_KEY`] signal the
    /// insert path reads, so a control bound here changes the next insert.
    pub fn image_size_policy(&self) -> Signal<String> {
        self.store
            .signal(IMAGE_SIZE_POLICY_KEY, IMAGE_SIZE_POLICY_DEFAULT.to_string())
    }

    /// Show where the book stands when a project with an active writing plan
    /// opens. Projects without a plan never show it, whatever this says.
    pub fn pace_summary_on_open(&self) -> Signal<bool> {
        self.store.signal(PACE_SUMMARY_ON_OPEN_KEY, true)
    }

    /// The margin lane's master switch.
    pub fn margin_lane_enabled(&self) -> Signal<bool> {
        self.store
            .signal(MARGIN_LANE_ENABLED_KEY, MARGIN_LANE_ENABLED_DEFAULT)
    }

    /// The margin lane's texture column.
    pub fn margin_lane_texture(&self) -> Signal<bool> {
        self.store
            .signal(MARGIN_LANE_TEXTURE_KEY, MARGIN_LANE_TEXTURE_DEFAULT)
    }

    /// Whether the lane appears on one surface.
    ///
    /// Its key is synthesised from the surface (see
    /// [`crate::margin_lane_surface_key`]), so it cannot be a field — but it is
    /// the same cached signal every reader of that key holds, and the settings
    /// page writes it through the same door.
    pub fn margin_lane_surface(&self, surface: crate::margin_lane::LaneSurface) -> Signal<bool> {
        self.store.signal(
            &crate::margin_lane_surface_key(surface),
            crate::margin_lane_surface_default(surface),
        )
    }

    /// Whether one registered lane provider draws its marks.
    ///
    /// Synthesised from the registration's id, so — unlike every other setting
    /// here — the *set* of them is not known until every extension has
    /// registered. Read through the registry, never guessed.
    pub fn margin_lane_provider(
        &self,
        provider: &crate::margin_lane::LaneProviderSpec,
    ) -> Signal<bool> {
        self.store
            .signal(&provider.settings_key(), provider.default_on)
    }

    /// Which of the three answers the theme picker holds — `"light"`, `"dark"`
    /// or `"system"`. See [`crate::THEME_MODE_KEY`] for why [`Self::dark`]
    /// cannot stand in for it.
    ///
    /// ⚠ **Read this only where you are about to write it** — today that is
    /// [`Self::set_theme_mode`] and the effect in `App::build` that mirrors the
    /// live theme back into the store. Touching it *seeds* it, and a key seeded
    /// with `"system"` on an install that predates it is not a harmless
    /// default: the next launch would hand the desktop a vote the writer never
    /// gave it, and someone who had pinned Dark on a light desktop would come
    /// back to a light one. That is exactly why
    /// [`crate::startup::theme_for`] reads an absent key as "fall back to
    /// `ui.dark`", and why `cli::persisted_theme_mode` reads the file directly
    /// — an upgrade path that depends on a key staying absent cannot have a
    /// reader that creates it. The Reset gate deliberately reads the *live
    /// theme's* id instead ([`crate::settings_keys::theme_mode_of`]), so
    /// nothing has to be on disk for it to be right.
    pub fn theme_mode(&self) -> Signal<String> {
        self.store
            .signal(THEME_MODE_KEY, THEME_MODE_DEFAULT.to_string())
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
    ///
    /// Records the *mode* as well as the resulting light/dark state: "the writer
    /// asked for dark" is an answer `ui.dark` alone cannot hold, because a
    /// writer following a dark desktop stores the same `true`.
    pub fn set_dark(&self, ctx: &mut EventContext, dark: bool) {
        self.set_theme_mode(
            ctx,
            if dark {
                THEME_MODE_DARK
            } else {
                THEME_MODE_LIGHT
            },
        );
        self.dark.set(dark); // same cached signal → persisted
    }

    /// Apply one of the three theme answers live, and persist it.
    ///
    /// `"system"` hands the desktop the decision (and keeps handing it over, as
    /// the desktop changes) rather than resolving it once here — which is the
    /// whole difference between this and [`Self::set_dark`], and the reason the
    /// mode is stored at all.
    ///
    /// An unrecognised mode is ignored rather than guessed at: the three answers
    /// are validated at every door the value can come through
    /// (`crate::settings_keys`' spec, `cli::persisted_theme_mode`), so reaching
    /// here with a fourth means a caller invented one.
    pub fn set_theme_mode(&self, ctx: &mut EventContext, mode: &str) {
        match mode {
            THEME_MODE_LIGHT => ctx.set_theme(crate::style::light()),
            THEME_MODE_DARK => ctx.set_theme(crate::style::dark()),
            THEME_MODE_SYSTEM => ctx.follow_system_theme(),
            _ => {
                eprintln!("settings: ignoring unknown theme mode {mode:?}");
                return;
            }
        }
        let persisted = self.theme_mode();
        if persisted.get() != mode {
            persisted.set(mode.to_string());
        }
        // `dark` is the *resolved* state and is mirrored from the live theme by
        // `App`'s theme effect, which runs whichever route the change took
        // (here, or the Settings window's `ThemeSwitcher`). Deliberately not
        // written here too: under `"system"` this function does not yet know
        // what the desktop will answer.
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

    /// Reset every setting this view-model owns to its default (the Settings
    /// window's *Reset to defaults*).
    ///
    /// One loop over [`super::defaults::reset_targets`], and that is the whole
    /// implementation on purpose. This used to be 64 hand-written `set` calls
    /// with a *separate* 42-comparison list deciding whether the button that
    /// calls it was even enabled, and the 25 in the gap could differ from
    /// factory while the button sat greyed out. The list is now the single
    /// source both readings come from, so:
    ///
    /// ⚠ **A new setting is a new row in `reset_targets`, never a `set` here.**
    /// A line added to this function would restore a knob the enable-gate
    /// cannot see, which is exactly the defect the list replaced.
    ///
    /// The theme, the interface language and the interface text scale are *not*
    /// here: they are ambient app state rather than settings this view-model
    /// holds, and are restored by [`reset_appearance`] and the footer's own
    /// scale reset.
    pub fn reset_editor_defaults(&self) {
        for target in super::defaults::reset_targets(self) {
            target.reset();
        }
    }
}

/// Restore the application's *appearance* to factory state: follow the
/// desktop's light/dark preference again, and speak the language a fresh
/// install on this machine would have picked.
///
/// A free function rather than a method because neither half is state this
/// view-model owns — both are ambient app state that only an `EventContext` can
/// reach. It sits here so the answer to "what is the factory appearance?" is
/// beside the answer to "what are the factory settings?".
///
/// ⚠ It replaces `ctx.set_theme(crate::style::light())`, which was wrong twice
/// over. Light is not the factory theme — the factory answer is *follow the
/// desktop*, which on a dark desktop is dark. And setting a theme explicitly
/// puts teksilo into `ThemeMode::Manual`, so the OS could never move it again:
/// a "reset" that took a choice away from the writer instead of giving one
/// back, and then left the Reset button lit on every dark desktop, because the
/// enable-gate could see the mismatch the reset had just created.
///
/// Neither value is written to the store here. Both are mirrored out of the
/// live theme and locale by `App`'s own effects — the same route the Settings
/// window's `ThemeSwitcher` and `LanguageSwitcher` take — so persisting them a
/// second time here would be a second writer for one fact.
pub(crate) fn reset_appearance(ctx: &mut EventContext) {
    ctx.follow_system_theme();
    // The factory language is "whatever a fresh install on this machine would
    // have picked", not a flat en-US — resetting a French account to English
    // would be restoring somebody else's default.
    ctx.set_locale(crate::startup::os_default_locale());
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

    /// Merely *having* a view-model must not decide the writer's theme for
    /// them.
    ///
    /// `store.signal(key, default)` seeds a missing key, and `App::build`
    /// constructs one of these on every launch — so a `theme_mode` field would
    /// write `ui.theme_mode = "system"` into every install that predates the
    /// key. The launch after that would hand the desktop a vote nobody gave
    /// it, and a writer who had pinned Dark on a light desktop would come back
    /// to a light one. The upgrade path in `startup::theme_for` depends on the
    /// key staying absent until something deliberately writes it, and this is
    /// what keeps it absent.
    #[test]
    fn constructing_the_view_model_does_not_choose_a_theme_mode() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);
        assert!(
            !store.has(THEME_MODE_KEY),
            "constructing a SettingsViewModel must not create {THEME_MODE_KEY}"
        );

        // Resetting the editor settings must not create it either: the theme is
        // `reset_appearance`'s to restore, live, and the mirror records what
        // that produced.
        vm.reset_editor_defaults();
        assert!(
            !store.has(THEME_MODE_KEY),
            "Reset to defaults must not write a theme mode"
        );

        // Reaching for it is the act of writing it, and that is the only route
        // by which it ever appears.
        assert_eq!(vm.theme_mode().get(), THEME_MODE_DEFAULT);
        assert!(store.has(THEME_MODE_KEY));
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
            m.initial(&BinderItemRole::Folder, &Book),
            id(seg::SEG_OWN),
            "starts on the container's own page"
        );
        m.remember(&BinderItemRole::Folder, &Book, seg::SEG_ANALYSIS);
        assert_eq!(
            m.initial(&BinderItemRole::Folder, &Book),
            id(seg::SEG_ANALYSIS),
            "a new Book tab inherits the last view"
        );
        // Per-type isolation.
        m.remember(&BinderItemRole::Folder, &ChapterScene, seg::SEG_MANUSCRIPT);
        assert_eq!(
            m.initial(&BinderItemRole::Folder, &ChapterScene),
            id(seg::SEG_MANUSCRIPT)
        );
        assert_eq!(
            m.initial(&BinderItemRole::Folder, &Book),
            id(seg::SEG_ANALYSIS),
            "types don't cross-contaminate"
        );
        // A non-segmented type (a plain Scene) has no view memory.
        m.remember(&BinderItemRole::Folder, &Scene, seg::SEG_MANUSCRIPT);
        assert_eq!(m.initial(&BinderItemRole::Folder, &Scene), Option::None);
    }

    /// **The two `Note` bars must not share one remembered view.** A `Folder/Note`
    /// carries Notes / Story bible / Overview; an `Item/Note` carries its own prose,
    /// Details and In prose. Keyed on the sub-role alone, as this was, they shared a
    /// single slot: every visit to a note's In prose segment wrote an id the folder
    /// cannot resolve, so the folder fell back to its first segment from then on,
    /// permanently, with no error anywhere. Neither bar existed when that key was
    /// written, which is why nothing caught it until the second one was built.
    #[test]
    fn a_note_and_a_notes_folder_remember_their_views_separately() {
        use BinderItemSubRole::*;
        let m = EditorViewMemory::detached(true);
        let id = |s: &str| Some(seg::segment_id(s));
        m.remember(&BinderItemRole::Folder, &Note, seg::SEG_OVERVIEW);
        m.remember(&BinderItemRole::Item, &Note, seg::SEG_NOTE_IN_PROSE);
        assert_eq!(
            m.initial(&BinderItemRole::Folder, &Note),
            id(seg::SEG_OVERVIEW),
            "the notes folder keeps the view the writer chose on it"
        );
        assert_eq!(
            m.initial(&BinderItemRole::Item, &Note),
            id(seg::SEG_NOTE_IN_PROSE),
            "and the single note keeps its own, which the folder can never resolve"
        );
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
            m.initial(&BinderItemRole::Folder, &Note),
            Some(seg::segment_id(seg::SEG_NOTES)),
            "a notes folder starts on its own page"
        );
        m.remember(&BinderItemRole::Folder, &Note, seg::SEG_OVERVIEW);
        assert_eq!(
            m.initial(&BinderItemRole::Folder, &Note),
            Some(seg::segment_id(seg::SEG_OVERVIEW)),
            "and returns to the view it was left on"
        );
        assert_eq!(
            m.initial(&BinderItemRole::Folder, &Book),
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
        m.remember(&BinderItemRole::Folder, &Paratext, seg::SEG_OVERVIEW);
        assert_eq!(
            m.initial(&BinderItemRole::Folder, &Paratext),
            Some(seg::segment_id(seg::SEG_OVERVIEW)),
            "a paratext folder must remember its view at all"
        );
        assert_eq!(
            m.initial(&BinderItemRole::Folder, &Note),
            Some(seg::segment_id(seg::SEG_OVERVIEW)),
            "…out of the same store as the notes folder, since it is the same bar"
        );
    }

    #[test]
    fn view_memory_disabled_is_inert() {
        use BinderItemSubRole::*;
        let m = EditorViewMemory::detached(true);
        m.remember(&BinderItemRole::Folder, &Part, seg::SEG_SYNOPSIS); // recorded while enabled
        m.enabled().set(false);
        assert_eq!(
            m.initial(&BinderItemRole::Folder, &Part),
            Option::None,
            "disabled starts wherever the bar's first segment is"
        );
        m.remember(&BinderItemRole::Folder, &Part, seg::SEG_MANUSCRIPT); // no-op while disabled
        m.enabled().set(true);
        assert_eq!(
            m.initial(&BinderItemRole::Folder, &Part),
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
