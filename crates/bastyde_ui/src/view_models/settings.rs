//! `SettingsViewModel` — a facade over the persisted UI settings.
//!
//! Store-backed: holds only cached settings `Signal`s, so every instance is a
//! view over the same live state. Rebuild it anywhere via
//! `SettingsViewModel::new(ctx.settings())`. Ambient app mutations (theme/locale)
//! reach the live app through an `EventContext`; pure-state ops (column width) do
//! not.

use bastyde::prelude::*; // EventContext, Signal, intui
use bastyde::settings::SettingsStore;

use crate::{
    AUTOSAVE_KEY, DARK_KEY, EDITOR_WIDTH_DEFAULT, EDITOR_WIDTH_KEY, LOCALE_KEY, SHOW_WELCOME_KEY,
};

#[derive(Clone)]
pub struct SettingsViewModel {
    dark: Signal<bool>,
    locale: Signal<String>,
    column_width: Signal<f32>,
    autosave: Signal<bool>,
    show_welcome: Signal<bool>,
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
}
