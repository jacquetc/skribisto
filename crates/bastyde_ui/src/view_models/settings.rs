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
//! The full-preferences window (`settings_panel.rs`) binds these signals into its
//! category panes. Theme and interface language are driven there by the
//! framework's drop-in `ThemeSwitcher` / `LanguageSwitcher` (which apply live via
//! `EventContext`); this VM still owns the persisted `dark` / `locale` mirrors so
//! `App` can keep `DARK_KEY` / `LOCALE_KEY` in sync for the startup restore.

use bastyde::prelude::*; // EventContext, Signal, intui
use bastyde::settings::SettingsStore;

use frontend::common::entities::BinderItemSubRole;
use skribisto_model::counting::CountingMethodSetting;

use crate::{
    AUTOSAVE_KEY, DARK_KEY, EDITOR_WIDTH_DEFAULT, EDITOR_WIDTH_KEY, GOALS_COUNTING_METHOD_KEY,
    GOALS_SHOW_CHARACTERS_DEFAULT, GOALS_SHOW_CHARACTERS_KEY, HIGHLIGHT_SENTENCE_DEFAULT,
    HIGHLIGHT_SENTENCE_KEY, LOCALE_KEY, NOTES_FIRST_LINE_INDENT_DEFAULT,
    NOTES_FIRST_LINE_INDENT_KEY, NOTES_FONT_FAMILY_DEFAULT, NOTES_FONT_FAMILY_KEY,
    NOTES_LINE_HEIGHT_DEFAULT, NOTES_LINE_HEIGHT_KEY, NOTES_PARA_SPACING_AFTER_DEFAULT,
    NOTES_PARA_SPACING_AFTER_KEY, NOTES_PARA_SPACING_BEFORE_DEFAULT, NOTES_PARA_SPACING_BEFORE_KEY,
    NOTES_SIZE_DEFAULT, NOTES_SIZE_KEY, PREVIEW_WIDTH_DEFAULT, PREVIEW_WIDTH_KEY,
    SCENE_FIRST_LINE_INDENT_DEFAULT,
    SCENE_FIRST_LINE_INDENT_KEY, SCENE_FONT_FAMILY_DEFAULT, SCENE_FONT_FAMILY_KEY,
    SCENE_LINE_HEIGHT_DEFAULT, SCENE_LINE_HEIGHT_KEY, SCENE_PARA_SPACING_AFTER_DEFAULT,
    SCENE_PARA_SPACING_AFTER_KEY, SCENE_PARA_SPACING_BEFORE_DEFAULT, SCENE_PARA_SPACING_BEFORE_KEY,
    SCENE_SIZE_DEFAULT, SCENE_SIZE_KEY, SHOW_WELCOME_KEY, SYNOPSIS_FIRST_LINE_INDENT_DEFAULT,
    SYNOPSIS_FIRST_LINE_INDENT_KEY, SYNOPSIS_FONT_FAMILY_DEFAULT, SYNOPSIS_FONT_FAMILY_KEY,
    SYNOPSIS_LINE_HEIGHT_DEFAULT, SYNOPSIS_LINE_HEIGHT_KEY, SYNOPSIS_PANE_DEFAULT,
    SYNOPSIS_PANE_KEY, SYNOPSIS_PARA_SPACING_AFTER_DEFAULT, SYNOPSIS_PARA_SPACING_AFTER_KEY,
    SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT, SYNOPSIS_PARA_SPACING_BEFORE_KEY, SYNOPSIS_SIZE_DEFAULT,
    REMEMBER_VIEW_DEFAULT, REMEMBER_VIEW_KEY, SYNOPSIS_SIZE_KEY, TYPEWRITER_DEFAULT, TYPEWRITER_KEY,
};

/// One editor type's four typography knobs. Cheap to clone — every field is a
/// `SettingsStore`-cached `Signal`, so all clones observe / drive the same
/// live value. `size` is a relative zoom multiplier (`1.0` = 100 %);
/// `line_height` is a multiple of the font size; `first_line_indent` is in px.
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

/// The three per-editor-type typography bundles (Scene / Synopsis / Notes),
/// created once and threaded through `EditorsViewModel` into every `ContentTab`.
#[derive(Clone)]
pub struct EditorTypographySet {
    pub scene: EditorTypography,
    pub synopsis: EditorTypography,
    pub notes: EditorTypography,
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
    book: Signal<usize>,
    part: Signal<usize>,
    chapter: Signal<usize>,
}

impl EditorViewMemory {
    pub fn new(store: &SettingsStore) -> Self {
        Self {
            enabled: store.signal(REMEMBER_VIEW_KEY, REMEMBER_VIEW_DEFAULT),
            book: store.signal("editor.last_view.book", 0usize),
            part: store.signal("editor.last_view.part", 0usize),
            chapter: store.signal("editor.last_view.chapter", 0usize),
        }
    }

    /// A store-less handle over fresh signals — for tests and any tab built without
    /// a `SettingsStore` in hand.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn detached(enabled: bool) -> Self {
        Self {
            enabled: Signal::new(enabled),
            book: Signal::new(0),
            part: Signal::new(0),
            chapter: Signal::new(0),
        }
    }

    /// The `editor.remember_view` toggle (bound by the Settings panel).
    pub fn enabled(&self) -> Signal<bool> {
        self.enabled.clone()
    }

    /// The persisted last-view signal for a segmented container sub-role, or `None`
    /// for a type with no `SegmentedControl`.
    fn stored(&self, sub_role: &BinderItemSubRole) -> Option<Signal<usize>> {
        use BinderItemSubRole::*;
        match sub_role {
            Book => Some(self.book.clone()),
            Part => Some(self.part.clone()),
            // The *folder* chapter (`Folder/ChapterScene`) is the only `ChapterScene`
            // container with a `SegmentedControl`; the flat chapter is a prose tab and
            // never asks for this.
            ChapterScene => Some(self.chapter.clone()),
            // `None` is shadowed by `BinderItemSubRole::None` under the glob import.
            _ => Option::None,
        }
    }

    /// The segment index a freshly-opened tab of `sub_role` should start on: the
    /// remembered view when enabled, else the container's own page (0).
    pub fn initial(&self, sub_role: &BinderItemSubRole) -> usize {
        if self.enabled.get() {
            self.stored(sub_role).map(|s| s.get()).unwrap_or(0)
        } else {
            0
        }
    }

    /// Record `segment` as the last view for `sub_role` (no-op when disabled, for a
    /// non-segmented type, or when already equal).
    pub fn remember(&self, sub_role: &BinderItemSubRole, segment: usize) {
        if self.enabled.get()
            && let Some(s) = self.stored(sub_role)
            && s.get() != segment
        {
            s.set(segment);
        }
    }
}

#[derive(Clone)]
pub struct SettingsViewModel {
    dark: Signal<bool>,
    locale: Signal<String>,
    column_width: Signal<f32>,
    preview_width: Signal<f32>,
    autosave: Signal<bool>,
    show_welcome: Signal<bool>,
    // ── Editor typography (per type) ──
    scene_typo: EditorTypography,
    synopsis_typo: EditorTypography,
    notes_typo: EditorTypography,
    // ── Editor behaviour ──
    synopsis_pane: Signal<bool>,
    typewriter: Signal<bool>,
    highlight_sentence: Signal<bool>,
    remember_view: Signal<bool>,
    // ── Goals & word count ──
    counting_method: Signal<CountingMethodSetting>,
    show_characters: Signal<bool>,
}

// Accessors/setters are the feature's public API; bound to widgets incrementally.
#[allow(dead_code)]
impl SettingsViewModel {
    pub fn new(store: &SettingsStore) -> Self {
        Self {
            dark: store.signal(DARK_KEY, false),
            locale: store.signal(LOCALE_KEY, "en-US".to_string()),
            column_width: store.signal(EDITOR_WIDTH_KEY, EDITOR_WIDTH_DEFAULT),
            preview_width: store.signal(PREVIEW_WIDTH_KEY, PREVIEW_WIDTH_DEFAULT),
            autosave: store.signal(AUTOSAVE_KEY, false),
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
            synopsis_pane: store.signal(SYNOPSIS_PANE_KEY, SYNOPSIS_PANE_DEFAULT),
            typewriter: store.signal(TYPEWRITER_KEY, TYPEWRITER_DEFAULT),
            highlight_sentence: store.signal(HIGHLIGHT_SENTENCE_KEY, HIGHLIGHT_SENTENCE_DEFAULT),
            remember_view: store.signal(REMEMBER_VIEW_KEY, REMEMBER_VIEW_DEFAULT),
            counting_method: store
                .signal(GOALS_COUNTING_METHOD_KEY, CountingMethodSetting::default()),
            show_characters: store.signal(GOALS_SHOW_CHARACTERS_KEY, GOALS_SHOW_CHARACTERS_DEFAULT),
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

    /// The three per-editor-type typography bundles (Scene / Synopsis / Notes).
    /// Every call returns clones of the same live signals, so a settings edit
    /// fans out to every open editor tab that holds them.
    pub fn editor_typography(&self) -> EditorTypographySet {
        EditorTypographySet {
            scene: self.scene_typo.clone(),
            synopsis: self.synopsis_typo.clone(),
            notes: self.notes_typo.clone(),
        }
    }

    /// Show the synopsis pane above the manuscript. Consumed live by the writing
    /// editor (`tabs::shared::prose`).
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
        self.synopsis_pane.set(SYNOPSIS_PANE_DEFAULT);
        self.typewriter.set(TYPEWRITER_DEFAULT);
        self.highlight_sentence.set(HIGHLIGHT_SENTENCE_DEFAULT);
        self.counting_method.set(CountingMethodSetting::default());
        self.show_characters.set(GOALS_SHOW_CHARACTERS_DEFAULT);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
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

    /// Reset restores all 12 per-type typography signals (the biggest, most
    /// error-prone part of the VM) to their `_DEFAULT` constants.
    #[test]
    fn reset_editor_defaults_restores_all_three_typography_bundles() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);
        let t = vm.editor_typography();
        for b in [&t.scene, &t.synopsis, &t.notes] {
            b.font_family.set("EB Garamond".into());
            b.size.set(1.3);
            b.line_height.set(2.1);
            b.first_line_indent.set(40.0);
        }

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
        // None of the three still holds the mutated value.
        for b in [&t.scene, &t.synopsis, &t.notes] {
            assert_ne!(b.font_family.get(), "EB Garamond");
            assert_ne!(b.first_line_indent.get(), 40.0);
        }
    }

    #[test]
    fn view_memory_round_trips_per_type() {
        use BinderItemSubRole::*;
        let m = EditorViewMemory::detached(true);
        assert_eq!(m.initial(&Book), 0, "starts on the container's own page");
        m.remember(&Book, 3);
        assert_eq!(m.initial(&Book), 3, "a new Book tab inherits the last view");
        // Per-type isolation.
        m.remember(&ChapterScene, 1);
        assert_eq!(m.initial(&ChapterScene), 1);
        assert_eq!(m.initial(&Book), 3, "types don't cross-contaminate");
        // A non-segmented type (a plain Scene) has no view memory.
        m.remember(&Scene, 2);
        assert_eq!(m.initial(&Scene), 0);
    }

    #[test]
    fn view_memory_disabled_is_inert() {
        use BinderItemSubRole::*;
        let m = EditorViewMemory::detached(true);
        m.remember(&Part, 2); // recorded while enabled
        m.enabled().set(false);
        assert_eq!(m.initial(&Part), 0, "disabled always starts on the own page");
        m.remember(&Part, 1); // no-op while disabled
        m.enabled().set(true);
        assert_eq!(m.initial(&Part), 2, "the disabled write was ignored");
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
