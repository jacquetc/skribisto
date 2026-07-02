//! `SettingsViewModel` — a facade over the persisted UI settings.
//!
//! Store-backed: holds only cached settings `Signal`s, so every instance is a
//! view over the same live state. Rebuild it anywhere via
//! `SettingsViewModel::new(ctx.settings())`. Ambient app mutations (theme/locale)
//! reach the live app through an `EventContext`; pure-state ops (column width,
//! manuscript typography) do not.
//!
//! The full-preferences window (`settings_panel.rs`) binds these signals into its
//! category panes. Theme and interface language are driven there by the
//! framework's drop-in `ThemeSwitcher` / `LanguageSwitcher` (which apply live via
//! `EventContext`); this VM still owns the persisted `dark` / `locale` mirrors so
//! `App` can keep `DARK_KEY` / `LOCALE_KEY` in sync for the startup restore.

use bastyde::prelude::*; // EventContext, Signal, intui
use bastyde::settings::SettingsStore;

use crate::{
    AUTOSAVE_KEY, DARK_KEY, EDITOR_WIDTH_DEFAULT, EDITOR_WIDTH_KEY, FONT_FAMILY_DEFAULT,
    FONT_FAMILY_KEY, HIGHLIGHT_SENTENCE_DEFAULT, HIGHLIGHT_SENTENCE_KEY, LINE_HEIGHT_DEFAULT,
    LINE_HEIGHT_KEY, LOCALE_KEY, SHOW_WELCOME_KEY, SYNOPSIS_PANE_DEFAULT, SYNOPSIS_PANE_KEY,
    TYPEWRITER_DEFAULT, TYPEWRITER_KEY,
};

#[derive(Clone)]
pub struct SettingsViewModel {
    dark: Signal<bool>,
    locale: Signal<String>,
    column_width: Signal<f32>,
    autosave: Signal<bool>,
    show_welcome: Signal<bool>,
    // ── Manuscript & Fonts ──
    font_family: Signal<String>,
    line_height: Signal<f32>,
    synopsis_pane: Signal<bool>,
    typewriter: Signal<bool>,
    highlight_sentence: Signal<bool>,
}

// Accessors/setters are the feature's public API; bound to widgets incrementally.
#[allow(dead_code)]
impl SettingsViewModel {
    pub fn new(store: &SettingsStore) -> Self {
        Self {
            dark: store.signal(DARK_KEY, false),
            locale: store.signal(LOCALE_KEY, "en-US".to_string()),
            column_width: store.signal(EDITOR_WIDTH_KEY, EDITOR_WIDTH_DEFAULT),
            autosave: store.signal(AUTOSAVE_KEY, false),
            show_welcome: store.signal(SHOW_WELCOME_KEY, true),
            font_family: store.signal(FONT_FAMILY_KEY, FONT_FAMILY_DEFAULT.to_string()),
            line_height: store.signal(LINE_HEIGHT_KEY, LINE_HEIGHT_DEFAULT),
            synopsis_pane: store.signal(SYNOPSIS_PANE_KEY, SYNOPSIS_PANE_DEFAULT),
            typewriter: store.signal(TYPEWRITER_KEY, TYPEWRITER_DEFAULT),
            highlight_sentence: store.signal(HIGHLIGHT_SENTENCE_KEY, HIGHLIGHT_SENTENCE_DEFAULT),
        }
    }

    /// Whether to autosave to disk (hides the manual Save affordances when on).
    /// Store-backed, so toggling it persists.
    pub fn autosave(&self) -> Signal<bool> {
        self.autosave.clone()
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
    pub fn dark(&self) -> Signal<bool> {
        self.dark.clone()
    }
    pub fn locale(&self) -> Signal<String> {
        self.locale.clone()
    }

    /// Manuscript typeface family (persisted preference). A `Signal<String>` (not
    /// `Option`) so it always serialises cleanly to TOML; the Typeface `ComboBox`
    /// bridges it to its `Option<String>` selection.
    pub fn font_family(&self) -> Signal<String> {
        self.font_family.clone()
    }
    /// Manuscript line height (leading multiple).
    pub fn line_height(&self) -> Signal<f32> {
        self.line_height.clone()
    }
    /// Show the synopsis pane above the manuscript. Consumed live by the writing
    /// editor (`item_scene_tab`).
    pub fn synopsis_pane(&self) -> Signal<bool> {
        self.synopsis_pane.clone()
    }
    /// Typewriter scrolling (keep the caret line centred).
    pub fn typewriter(&self) -> Signal<bool> {
        self.typewriter.clone()
    }
    /// Highlight the current sentence.
    pub fn highlight_sentence(&self) -> Signal<bool> {
        self.highlight_sentence.clone()
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

    /// Reset every setting this VM owns to its default (used by the Settings
    /// window's "Reset to defaults"). Theme / locale / text-scale live outside
    /// this VM, so the panel resets those alongside this call.
    pub fn reset_editor_defaults(&self) {
        self.column_width.set(EDITOR_WIDTH_DEFAULT);
        self.autosave.set(false);
        self.show_welcome.set(true);
        self.font_family.set(FONT_FAMILY_DEFAULT.to_string());
        self.line_height.set(LINE_HEIGHT_DEFAULT);
        self.synopsis_pane.set(SYNOPSIS_PANE_DEFAULT);
        self.typewriter.set(TYPEWRITER_DEFAULT);
        self.highlight_sentence.set(HIGHLIGHT_SENTENCE_DEFAULT);
    }
}
