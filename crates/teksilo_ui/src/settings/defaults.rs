// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! "Does anything differ from the factory defaults?", as one reactive fold.
//!
//! The answer drives exactly one thing — whether *Reset to defaults* in the
//! footer is enabled — but it has to consider every persisted setting the window
//! can change, so it is long by nature rather than by accident. Keeping it here
//! rather than inline in the footer means adding a setting is a one-line edit to
//! a list, next to the other forty.

use skribisto_model::counting::CountingMethodSetting;
use teksilo::core::styles::Theme;
use teksilo::prelude::*;

use super::TEXT_SCALE_DEFAULT;
use crate::view_models::{HighlightScope, SettingsViewModel, TypewriterAnchor};
use crate::{
    DISTRACTION_FREE_FIRST_LINE_INDENT_DEFAULT, DISTRACTION_FREE_FONT_FAMILY_DEFAULT,
    DISTRACTION_FREE_GO_DEFAULT, DISTRACTION_FREE_GO_TO_DEFAULT,
    DISTRACTION_FREE_LINE_HEIGHT_DEFAULT, DISTRACTION_FREE_PARA_SPACING_AFTER_DEFAULT,
    DISTRACTION_FREE_PARA_SPACING_BEFORE_DEFAULT, DISTRACTION_FREE_SESSION_DEFAULT,
    DISTRACTION_FREE_SIZE_DEFAULT, DISTRACTION_FREE_TITLE_DEFAULT, DISTRACTION_FREE_WIDTH_DEFAULT,
    DISTRACTION_FREE_WORD_COUNT_DEFAULT, EDITOR_WIDTH_DEFAULT, GOALS_SHOW_CHARACTERS_DEFAULT,
    NOTES_FIRST_LINE_INDENT_DEFAULT, NOTES_FONT_FAMILY_DEFAULT, NOTES_LINE_HEIGHT_DEFAULT,
    NOTES_PARA_SPACING_AFTER_DEFAULT, NOTES_PARA_SPACING_BEFORE_DEFAULT, NOTES_SIZE_DEFAULT,
    SCENE_FIRST_LINE_INDENT_DEFAULT, SCENE_FONT_FAMILY_DEFAULT, SCENE_LINE_HEIGHT_DEFAULT,
    SCENE_PARA_SPACING_AFTER_DEFAULT, SCENE_PARA_SPACING_BEFORE_DEFAULT, SCENE_SIZE_DEFAULT,
    SYNOPSIS_FIRST_LINE_INDENT_DEFAULT, SYNOPSIS_FONT_FAMILY_DEFAULT, SYNOPSIS_LINE_HEIGHT_DEFAULT,
    SYNOPSIS_PANE_DEFAULT, SYNOPSIS_PARA_SPACING_AFTER_DEFAULT,
    SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT, SYNOPSIS_SIZE_DEFAULT, TYPEWRITER_DEFAULT,
};

// ── Reset-to-defaults enable state ───────────────────────────────────────────

/// `true` if any signal in `sigs` is currently `true` — a reactive OR-fold.
fn any_true(sigs: Vec<Signal<bool>>) -> Signal<bool> {
    let mut it = sigs.into_iter();
    match it.next() {
        None => Signal::new(false),
        Some(first) => it.fold(first, |acc, s| acc.or(&s)),
    }
}

/// Reactive "the live settings differ from the factory defaults" — drives the
/// Reset-to-defaults button's enabled state.
pub(crate) fn build_not_defaults(
    theme: &Signal<Theme>,
    locale: &Option<Signal<teksilo::i18n::LanguageIdentifier>>,
    scale: &Signal<f32>,
    vm: &SettingsViewModel,
) -> Signal<bool> {
    let typo = vm.editor_typography();
    let mut diffs = vec![
        theme.map(|t| t.is_dark()), // default = light
        scale.map(|s| (*s - TEXT_SCALE_DEFAULT).abs() > f32::EPSILON),
        vm.column_width()
            .map(|w| (*w - EDITOR_WIDTH_DEFAULT).abs() > 0.01),
        vm.autosave().map(|a| *a),      // default = off
        vm.show_welcome().map(|s| !*s), // default = on
        // ── Scene typography ──
        typo.scene
            .font_family
            .map(|f| f.as_str() != SCENE_FONT_FAMILY_DEFAULT),
        typo.scene
            .size
            .map(|s| (*s - SCENE_SIZE_DEFAULT).abs() > f32::EPSILON),
        typo.scene
            .line_height
            .map(|h| (*h - SCENE_LINE_HEIGHT_DEFAULT).abs() > f32::EPSILON),
        typo.scene
            .first_line_indent
            .map(|i| (*i - SCENE_FIRST_LINE_INDENT_DEFAULT).abs() > 0.01),
        typo.scene
            .para_spacing_before
            .map(|v| (*v - SCENE_PARA_SPACING_BEFORE_DEFAULT).abs() > 0.01),
        typo.scene
            .para_spacing_after
            .map(|v| (*v - SCENE_PARA_SPACING_AFTER_DEFAULT).abs() > 0.01),
        // ── Synopsis typography ──
        typo.synopsis
            .font_family
            .map(|f| f.as_str() != SYNOPSIS_FONT_FAMILY_DEFAULT),
        typo.synopsis
            .size
            .map(|s| (*s - SYNOPSIS_SIZE_DEFAULT).abs() > f32::EPSILON),
        typo.synopsis
            .line_height
            .map(|h| (*h - SYNOPSIS_LINE_HEIGHT_DEFAULT).abs() > f32::EPSILON),
        typo.synopsis
            .first_line_indent
            .map(|i| (*i - SYNOPSIS_FIRST_LINE_INDENT_DEFAULT).abs() > 0.01),
        typo.synopsis
            .para_spacing_before
            .map(|v| (*v - SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT).abs() > 0.01),
        typo.synopsis
            .para_spacing_after
            .map(|v| (*v - SYNOPSIS_PARA_SPACING_AFTER_DEFAULT).abs() > 0.01),
        // ── Notes typography ──
        typo.notes
            .font_family
            .map(|f| f.as_str() != NOTES_FONT_FAMILY_DEFAULT),
        typo.notes
            .size
            .map(|s| (*s - NOTES_SIZE_DEFAULT).abs() > f32::EPSILON),
        typo.notes
            .line_height
            .map(|h| (*h - NOTES_LINE_HEIGHT_DEFAULT).abs() > f32::EPSILON),
        typo.notes
            .first_line_indent
            .map(|i| (*i - NOTES_FIRST_LINE_INDENT_DEFAULT).abs() > 0.01),
        typo.notes
            .para_spacing_before
            .map(|v| (*v - NOTES_PARA_SPACING_BEFORE_DEFAULT).abs() > 0.01),
        typo.notes
            .para_spacing_after
            .map(|v| (*v - NOTES_PARA_SPACING_AFTER_DEFAULT).abs() > 0.01),
        // ── Distraction-free typography ──
        typo.distraction_free
            .font_family
            .map(|f| f.as_str() != DISTRACTION_FREE_FONT_FAMILY_DEFAULT),
        typo.distraction_free
            .size
            .map(|s| (*s - DISTRACTION_FREE_SIZE_DEFAULT).abs() > f32::EPSILON),
        typo.distraction_free
            .line_height
            .map(|h| (*h - DISTRACTION_FREE_LINE_HEIGHT_DEFAULT).abs() > f32::EPSILON),
        typo.distraction_free
            .first_line_indent
            .map(|i| (*i - DISTRACTION_FREE_FIRST_LINE_INDENT_DEFAULT).abs() > 0.01),
        typo.distraction_free
            .para_spacing_before
            .map(|v| (*v - DISTRACTION_FREE_PARA_SPACING_BEFORE_DEFAULT).abs() > 0.01),
        typo.distraction_free
            .para_spacing_after
            .map(|v| (*v - DISTRACTION_FREE_PARA_SPACING_AFTER_DEFAULT).abs() > 0.01),
        vm.distraction_free_width()
            .map(|w| (*w - DISTRACTION_FREE_WIDTH_DEFAULT).abs() > 0.01),
        vm.distraction_free_title()
            .map(|s| *s != DISTRACTION_FREE_TITLE_DEFAULT),
        vm.distraction_free_word_count()
            .map(|s| *s != DISTRACTION_FREE_WORD_COUNT_DEFAULT),
        vm.distraction_free_session()
            .map(|s| *s != DISTRACTION_FREE_SESSION_DEFAULT),
        vm.distraction_free_go()
            .map(|s| *s != DISTRACTION_FREE_GO_DEFAULT),
        vm.distraction_free_go_to()
            .map(|s| *s != DISTRACTION_FREE_GO_TO_DEFAULT),
        // ── Editor behaviour ──
        vm.synopsis_pane().map(|s| *s != SYNOPSIS_PANE_DEFAULT),
        vm.typewriter().map(|s| *s != TYPEWRITER_DEFAULT),
        vm.typewriter_anchor()
            .map(|a| *a != Some(TypewriterAnchor::default())),
        vm.highlight_scope()
            .map(|s| *s != HighlightScope::default()),
        // ── Goals & word count ──
        vm.counting_method()
            .map(|m| *m != CountingMethodSetting::default()),
        vm.show_characters()
            .map(|s| *s != GOALS_SHOW_CHARACTERS_DEFAULT),
    ];
    if let Some(loc) = locale {
        // Compare against a once-parsed default rather than allocating a String
        // per change (clippy::cmp_owned).
        let default_locale: teksilo::i18n::LanguageIdentifier =
            "en-US".parse().expect("valid default locale");
        diffs.push(loc.map(move |l| *l != default_locale));
    }
    any_true(diffs)
}
