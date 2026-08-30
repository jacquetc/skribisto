// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What *Reset to defaults* restores, and what "anything differs from factory"
//! means — as **one** list.
//!
//! The two used to be two: a `reset_editor_defaults` that wrote 64 signals, and a
//! `build_not_defaults` that compared 42. The footer button is
//! `.enabled(not_defaults)`, so the 25 keys in the gap could differ from factory
//! while the button that would restore them was dead — turn spell-check off with
//! F7 and *Reset to defaults* greyed itself out. Four whole settings pages were
//! in that gap.
//!
//! Adding the 25 comparisons was the backlog; **the list is the fix**. Every
//! resettable knob is one [`ResetTarget`] carrying three things — how to put it
//! back, how to tell it apart from its default, and how a test moves it off that
//! default — so a knob cannot reach one function without reaching the other, and
//! the pairing test below drives all three for every row. A 65th knob is a 65th
//! row; there is nowhere to add it that skips the gate.
//!
//! Three things are deliberately **not** rows, because they are not this
//! view-model's to hold: the theme, the interface language and the interface
//! text scale are ambient app state, applied through an `EventContext` and
//! restored by [`crate::settings::reset_appearance`]. They are compared inline
//! at the bottom of [`build_not_defaults`].

use skribisto_model::counting::CountingMethodSetting;
use teksilo::core::styles::Theme;
use teksilo::prelude::*;

use frontend::common::entities::QuoteStyle;

use super::TEXT_SCALE_DEFAULT;
use crate::settings::{EditorTypography, SettingsViewModel};
use crate::shared::{HighlightScope, SynopsisPlacement, TypewriterAnchor};

// ── One resettable knob ──────────────────────────────────────────────────────

/// One setting *Reset to defaults* restores, and the reactive answer to "is it
/// still at that default?".
///
/// Built over the live store-backed `Signal`, so `reset` reaches every open
/// window and `differs` recomputes without anybody wiring an observer.
pub(crate) struct ResetTarget {
    /// The `general.toml` key this knob writes — the id a failing test names,
    /// and what ties a row to [`crate::settings_keys::SETTINGS`].
    ///
    /// Carried in every build, not just under `cfg(test)`: it is what makes a
    /// row self-identifying, and a row that could be written without naming its
    /// key is a row the schema test below cannot check.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) id: String,
    reset: Box<dyn Fn()>,
    differs: Signal<bool>,
    /// Move this knob *off* its default.
    ///
    /// Only the pairing test calls it, and it is a field rather than a
    /// `#[cfg(test)]` afterthought on purpose: a row cannot be written without
    /// saying how to disturb it, which is what makes the test's coverage
    /// automatic rather than hand-maintained.
    #[cfg_attr(not(test), allow(dead_code))]
    perturb: Box<dyn Fn()>,
}

impl ResetTarget {
    /// Put this knob back to its factory value.
    pub(crate) fn reset(&self) {
        (self.reset)()
    }

    /// Reactive "this knob is not at its factory value".
    pub(crate) fn differs(&self) -> Signal<bool> {
        self.differs.clone()
    }

    #[cfg(test)]
    fn perturb(&self) {
        (self.perturb)()
    }
}

/// A knob with a small set of legal values, compared by equality.
///
/// `other` is any value that is not `default` — the test needs *a* way off the
/// default, not a meaningful one.
fn choice<T: Clone + PartialEq + 'static>(
    id: impl Into<String>,
    signal: Signal<T>,
    default: T,
    other: T,
) -> ResetTarget {
    let (for_reset, for_perturb) = (signal.clone(), signal.clone());
    let factory = default.clone();
    ResetTarget {
        id: id.into(),
        differs: signal.map(move |v| *v != default),
        reset: Box::new(move || for_reset.set(factory.clone())),
        perturb: Box::new(move || for_perturb.set(other.clone())),
    }
}

/// A two-state knob. Its "other" value writes itself.
fn flag(id: impl Into<String>, signal: Signal<bool>, default: bool) -> ResetTarget {
    choice(id, signal, default, !default)
}

/// A free-text knob (a font family, a theme id).
fn text(id: impl Into<String>, signal: Signal<String>, default: &str, other: &str) -> ResetTarget {
    choice(id, signal, default.to_string(), other.to_string())
}

/// A measurement. `tolerance` is the width of "unchanged" — `f32::EPSILON` for a
/// scale factor, a hundredth of a pixel for a length, exactly as each comparison
/// was written before this list existed.
fn number(id: impl Into<String>, signal: Signal<f32>, default: f32, tolerance: f32) -> ResetTarget {
    let (for_reset, for_perturb) = (signal.clone(), signal.clone());
    // Comfortably past `tolerance` whichever of the two it is, and still a
    // plausible value for every knob here (the largest is a 480 px card).
    let off_default = default + 1.0 + tolerance;
    ResetTarget {
        id: id.into(),
        differs: signal.map(move |v| (*v - default).abs() > tolerance),
        reset: Box::new(move || for_reset.set(default)),
        perturb: Box::new(move || for_perturb.set(off_default)),
    }
}

/// The six knobs every editor typography bundle carries.
struct TypographyDefaults {
    family: &'static str,
    size: f32,
    line_height: f32,
    first_line_indent: f32,
    para_spacing_before: f32,
    para_spacing_after: f32,
}

/// One bundle's six rows. `prefix` is its settings-key namespace
/// (`"editor.scene"`, `"corkboard"`, …), so the ids are the real keys.
fn typography(prefix: &str, t: &EditorTypography, d: &TypographyDefaults) -> Vec<ResetTarget> {
    vec![
        text(
            format!("{prefix}.font_family"),
            t.font_family.clone(),
            d.family,
            "EB Garamond",
        ),
        number(
            format!("{prefix}.size"),
            t.size.clone(),
            d.size,
            f32::EPSILON,
        ),
        number(
            format!("{prefix}.line_height"),
            t.line_height.clone(),
            d.line_height,
            f32::EPSILON,
        ),
        number(
            format!("{prefix}.first_line_indent"),
            t.first_line_indent.clone(),
            d.first_line_indent,
            0.01,
        ),
        number(
            format!("{prefix}.para_spacing_before"),
            t.para_spacing_before.clone(),
            d.para_spacing_before,
            0.01,
        ),
        number(
            format!("{prefix}.para_spacing_after"),
            t.para_spacing_after.clone(),
            d.para_spacing_after,
            0.01,
        ),
    ]
}

// ── The list ─────────────────────────────────────────────────────────────────

/// Every setting *Reset to defaults* restores.
///
/// ## What is in, and why
///
/// The confirmation the writer reads says: *"Restores this application's
/// appearance, editor and writing settings. Your project's own settings,
/// keyboard shortcuts and saved styles are not affected."* This list is that
/// sentence, and each exclusion below is a deliberate reading of it — the
/// previous sentence promised "every setting on all pages" and was wrong in both
/// directions.
///
/// **In:** every app-level editor, writing and presentation scalar — layout and
/// saving, the five typography bundles, distraction-free, editor behaviour,
/// application-level smart punctuation, goals, the writing game, the corkboard,
/// the margin lane (its master switch, its texture column, and every per-surface
/// and per-provider row, which are settings the writer set on the same page and
/// would otherwise survive a "reset" as the only trace of the old state).
///
/// **Out, and correctly so:**
///
/// * **`user.name` / `user.initials`** — that is the *person*, not a setting.
///   Wiping the name someone signs their remarks with, because they asked to
///   restore their editor's defaults, is a data loss dressed as a preference.
/// * **Keyboard shortcuts** — the sentence excludes them, and they are not
///   scalars in this store at all.
/// * **The `SettingsFile<T>` siblings** — dictionaries, export styles,
///   distraction-free themes, `backup.toml`, `search.toml`,
///   `tree_expansion.toml`. Versioned documents holding things the writer
///   *made* (a custom style, a word they taught the app), not knobs they set.
///   `editor.distraction_free.theme` **is** here — that is which of them is
///   selected, a scalar; the themes themselves are not.
/// * **Everything on a `Work`** — author, language, per-project punctuation,
///   tags, templates, text replacements. Manuscript data inside the `.skrib`;
///   with a project open, ten of the Settings window's leaves are `Work:` pages,
///   which is the half of the old sentence that over-promised.
/// * **`editor.last_view.*`** — where the writer was, not what they chose.
///   `editor.remember_view`, the setting that decides whether the app remembers
///   at all, *is* reset.
pub(crate) fn reset_targets(vm: &SettingsViewModel) -> Vec<ResetTarget> {
    let typo = vm.editor_typography();
    let mut rows = vec![
        // ── Editor: layout & saving ──
        number(
            crate::EDITOR_WIDTH_KEY,
            vm.column_width(),
            crate::EDITOR_WIDTH_DEFAULT,
            0.01,
        ),
        number(
            crate::PREVIEW_WIDTH_KEY,
            vm.preview_width(),
            crate::PREVIEW_WIDTH_DEFAULT,
            0.01,
        ),
        flag(crate::AUTOSAVE_KEY, vm.autosave(), false),
        flag(
            crate::SPELLCHECK_ENABLED_KEY,
            vm.spellcheck_enabled(),
            crate::SPELLCHECK_ENABLED_DEFAULT,
        ),
        flag(
            crate::COMMENTS_VISIBLE_KEY,
            vm.comments_visible(),
            crate::COMMENTS_VISIBLE_DEFAULT,
        ),
        flag(crate::SHOW_WELCOME_KEY, vm.show_welcome(), true),
        flag(
            crate::REMEMBER_VIEW_KEY,
            vm.remember_view(),
            crate::REMEMBER_VIEW_DEFAULT,
        ),
        // A one-way door until now: the writer ticks "don't ask again" in the
        // large-image prompt and nothing in the app ever asks again. Group D is
        // giving it a control; reset has to be able to reopen it either way.
        text(
            crate::IMAGE_SIZE_POLICY_KEY,
            vm.image_size_policy(),
            crate::IMAGE_SIZE_POLICY_DEFAULT,
            "keep",
        ),
        flag(
            crate::PACE_SUMMARY_ON_OPEN_KEY,
            vm.pace_summary_on_open(),
            true,
        ),
        // ── The margin lane ──
        flag(
            crate::MARGIN_LANE_ENABLED_KEY,
            vm.margin_lane_enabled(),
            crate::MARGIN_LANE_ENABLED_DEFAULT,
        ),
        flag(
            crate::MARGIN_LANE_TEXTURE_KEY,
            vm.margin_lane_texture(),
            crate::MARGIN_LANE_TEXTURE_DEFAULT,
        ),
    ];

    // The lane's per-surface and per-provider rows are synthesised from
    // `LaneSurface::key` and each registration's id, so they cannot be listed at
    // compile time — an extension's providers are not known until it registers.
    // Read here through the same registry the settings page reads, a snapshot
    // taken when the Settings window is built, which is exactly the set of rows
    // that page can show.
    for surface in crate::margin_lane::LaneSurface::all() {
        rows.push(flag(
            crate::margin_lane_surface_key(surface),
            vm.margin_lane_surface(surface),
            crate::margin_lane_surface_default(surface),
        ));
    }
    for provider in crate::margin_lane::registered() {
        rows.push(flag(
            provider.settings_key(),
            vm.margin_lane_provider(&provider),
            provider.default_on,
        ));
    }

    // ── Editor typography, one bundle at a time ──
    rows.extend(typography(
        "editor.scene",
        &typo.scene,
        &TypographyDefaults {
            family: crate::SCENE_FONT_FAMILY_DEFAULT,
            size: crate::SCENE_SIZE_DEFAULT,
            line_height: crate::SCENE_LINE_HEIGHT_DEFAULT,
            first_line_indent: crate::SCENE_FIRST_LINE_INDENT_DEFAULT,
            para_spacing_before: crate::SCENE_PARA_SPACING_BEFORE_DEFAULT,
            para_spacing_after: crate::SCENE_PARA_SPACING_AFTER_DEFAULT,
        },
    ));
    rows.extend(typography(
        "editor.synopsis",
        &typo.synopsis,
        &TypographyDefaults {
            family: crate::SYNOPSIS_FONT_FAMILY_DEFAULT,
            size: crate::SYNOPSIS_SIZE_DEFAULT,
            line_height: crate::SYNOPSIS_LINE_HEIGHT_DEFAULT,
            first_line_indent: crate::SYNOPSIS_FIRST_LINE_INDENT_DEFAULT,
            para_spacing_before: crate::SYNOPSIS_PARA_SPACING_BEFORE_DEFAULT,
            para_spacing_after: crate::SYNOPSIS_PARA_SPACING_AFTER_DEFAULT,
        },
    ));
    rows.extend(typography(
        "editor.notes",
        &typo.notes,
        &TypographyDefaults {
            family: crate::NOTES_FONT_FAMILY_DEFAULT,
            size: crate::NOTES_SIZE_DEFAULT,
            line_height: crate::NOTES_LINE_HEIGHT_DEFAULT,
            first_line_indent: crate::NOTES_FIRST_LINE_INDENT_DEFAULT,
            para_spacing_before: crate::NOTES_PARA_SPACING_BEFORE_DEFAULT,
            para_spacing_after: crate::NOTES_PARA_SPACING_AFTER_DEFAULT,
        },
    ));
    rows.extend(typography(
        "corkboard",
        &typo.corkboard,
        &TypographyDefaults {
            family: crate::CORKBOARD_FONT_FAMILY_DEFAULT,
            size: crate::CORKBOARD_SIZE_DEFAULT,
            line_height: crate::CORKBOARD_LINE_HEIGHT_DEFAULT,
            first_line_indent: crate::CORKBOARD_FIRST_LINE_INDENT_DEFAULT,
            para_spacing_before: crate::CORKBOARD_PARA_SPACING_BEFORE_DEFAULT,
            para_spacing_after: crate::CORKBOARD_PARA_SPACING_AFTER_DEFAULT,
        },
    ));
    rows.extend(typography(
        "editor.distraction_free",
        &typo.distraction_free,
        &TypographyDefaults {
            family: crate::DISTRACTION_FREE_FONT_FAMILY_DEFAULT,
            size: crate::DISTRACTION_FREE_SIZE_DEFAULT,
            line_height: crate::DISTRACTION_FREE_LINE_HEIGHT_DEFAULT,
            first_line_indent: crate::DISTRACTION_FREE_FIRST_LINE_INDENT_DEFAULT,
            para_spacing_before: crate::DISTRACTION_FREE_PARA_SPACING_BEFORE_DEFAULT,
            para_spacing_after: crate::DISTRACTION_FREE_PARA_SPACING_AFTER_DEFAULT,
        },
    ));

    rows.extend([
        // ── Distraction-free surface & chrome ──
        number(
            crate::DISTRACTION_FREE_WIDTH_KEY,
            vm.distraction_free_width(),
            crate::DISTRACTION_FREE_WIDTH_DEFAULT,
            0.01,
        ),
        text(
            crate::DISTRACTION_FREE_THEME_KEY,
            vm.distraction_free_theme(),
            crate::DISTRACTION_FREE_THEME_DEFAULT,
            "midnight",
        ),
        flag(
            crate::DISTRACTION_FREE_TITLE_KEY,
            vm.distraction_free_title(),
            crate::DISTRACTION_FREE_TITLE_DEFAULT,
        ),
        flag(
            crate::DISTRACTION_FREE_WORD_COUNT_KEY,
            vm.distraction_free_word_count(),
            crate::DISTRACTION_FREE_WORD_COUNT_DEFAULT,
        ),
        flag(
            crate::DISTRACTION_FREE_SESSION_KEY,
            vm.distraction_free_session(),
            crate::DISTRACTION_FREE_SESSION_DEFAULT,
        ),
        flag(
            crate::DISTRACTION_FREE_GO_KEY,
            vm.distraction_free_go(),
            crate::DISTRACTION_FREE_GO_DEFAULT,
        ),
        flag(
            crate::DISTRACTION_FREE_GO_TO_KEY,
            vm.distraction_free_go_to(),
            crate::DISTRACTION_FREE_GO_TO_DEFAULT,
        ),
        // ── Editor behaviour ──
        flag(
            crate::SYNOPSIS_PANE_KEY,
            vm.synopsis_pane(),
            crate::SYNOPSIS_PANE_DEFAULT,
        ),
        choice(
            crate::SYNOPSIS_PLACEMENT_KEY,
            vm.synopsis_placement(),
            SynopsisPlacement::default(),
            SynopsisPlacement::Side,
        ),
        number(
            crate::SYNOPSIS_SIDE_WIDTH_KEY,
            vm.synopsis_side_width(),
            crate::SYNOPSIS_SIDE_WIDTH_DEFAULT,
            0.01,
        ),
        flag(
            crate::TYPEWRITER_KEY,
            vm.typewriter(),
            crate::TYPEWRITER_DEFAULT,
        ),
        choice(
            crate::TYPEWRITER_ANCHOR_KEY,
            vm.typewriter_anchor(),
            Some(TypewriterAnchor::default()),
            Some(TypewriterAnchor::BottomQuarter),
        ),
        choice(
            crate::HIGHLIGHT_SCOPE_KEY,
            vm.highlight_scope(),
            HighlightScope::default(),
            HighlightScope::Paragraph,
        ),
        // ── Smart punctuation, application level ──
        flag(
            crate::PUNCT_DASHES_KEY,
            vm.punct_dashes(),
            crate::PUNCT_DASHES_DEFAULT,
        ),
        flag(
            crate::PUNCT_ELLIPSIS_KEY,
            vm.punct_ellipsis(),
            crate::PUNCT_ELLIPSIS_DEFAULT,
        ),
        flag(
            crate::PUNCT_QUOTES_KEY,
            vm.punct_quotes(),
            crate::PUNCT_QUOTES_DEFAULT,
        ),
        choice(
            crate::PUNCT_QUOTE_STYLE_KEY,
            vm.punct_quote_style(),
            QuoteStyle::default(),
            QuoteStyle::Guillemets,
        ),
        flag(
            crate::PUNCT_SPACING_KEY,
            vm.punct_spacing(),
            crate::PUNCT_SPACING_DEFAULT,
        ),
        flag(
            crate::PUNCT_DIALOGUE_KEY,
            vm.punct_dialogue(),
            crate::PUNCT_DIALOGUE_DEFAULT,
        ),
        // ── Goals & word count ──
        choice(
            crate::GOALS_COUNTING_METHOD_KEY,
            vm.counting_method(),
            CountingMethodSetting::default(),
            CountingMethodSetting::Whitespace,
        ),
        flag(
            crate::GOALS_SHOW_CHARACTERS_KEY,
            vm.show_characters(),
            crate::GOALS_SHOW_CHARACTERS_DEFAULT,
        ),
        // ── Writing games ──
        flag(
            crate::GAMES_FORWARD_PROSE_KEY,
            vm.games_forward_prose(),
            crate::writing_session::FORWARD_PROSE_DEFAULT,
        ),
        flag(
            crate::GAMES_FORWARD_SYNOPSIS_KEY,
            vm.games_forward_synopsis(),
            crate::writing_session::FORWARD_SYNOPSIS_DEFAULT,
        ),
        // ── Corkboard presentation ──
        flag(
            crate::CORKBOARD_NESTED_KEY,
            vm.corkboard_nested(),
            crate::CORKBOARD_NESTED_DEFAULT,
        ),
        number(
            crate::CORKBOARD_CARD_SIZE_KEY,
            vm.corkboard_card_size(),
            crate::CORKBOARD_CARD_SIZE_DEFAULT,
            0.01,
        ),
        flag(
            crate::CORKBOARD_SHOW_WORD_COUNT_KEY,
            vm.corkboard_show_word_count(),
            crate::CORKBOARD_SHOW_WORD_COUNT_DEFAULT,
        ),
        flag(
            crate::CORKBOARD_SHOW_CARD_NUMBERS_KEY,
            vm.corkboard_show_card_numbers(),
            crate::CORKBOARD_SHOW_CARD_NUMBERS_DEFAULT,
        ),
        number(
            crate::CORKBOARD_MODAL_SIZE_KEY,
            vm.corkboard_modal_size(),
            crate::CORKBOARD_MODAL_SIZE_DEFAULT,
            f32::EPSILON,
        ),
    ]);

    rows
}

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
    let mut diffs: Vec<Signal<bool>> = reset_targets(vm).iter().map(ResetTarget::differs).collect();

    // ── The three that are not this view-model's to hold ──
    //
    // The theme is read as a *mode*, never as `is_dark()`. The factory answer is
    // "follow the desktop", so on a dark desktop at factory state `is_dark()`
    // was true, the gate said "not default", and the button lit up with nothing
    // to do — pressing it changed nothing the gate could see, because there was
    // nothing wrong. `theme_mode_of` reads the answer out of the live theme's own
    // id (see `crate::SYSTEM_THEME_ID`), so this is right before anything has been
    // persisted, and right again the instant the writer picks System.
    diffs.push(theme.map(|t| crate::settings_keys::theme_mode_of(t) != crate::THEME_MODE_DEFAULT));
    diffs.push(scale.map(|s| (*s - TEXT_SCALE_DEFAULT).abs() > f32::EPSILON));

    if let Some(loc) = locale {
        // Compare against a once-parsed default rather than allocating a String
        // per change (clippy::cmp_owned).
        //
        // `os_default_locale()`, not a flat `en-US`: the factory language is
        // whatever a fresh install on this machine would pick, so on a French
        // account this must read `fr-FR` or *Reset to defaults* is lit at
        // factory state and stays lit after a reset has already run.
        match crate::startup::os_default_locale().parse::<teksilo::i18n::LanguageIdentifier>() {
            Ok(default_locale) => diffs.push(loc.map(move |l| *l != default_locale)),
            // Unreachable in practice (`SUPPORTED_LOCALES` is tested for
            // parseability), and not worth a panic if it ever is: the language
            // simply stops being one of the things the button waits for.
            Err(e) => eprintln!("settings: cannot parse the default locale: {e}"),
        }
    }
    any_true(diffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use std::sync::atomic::{AtomicU32, Ordering};
    use teksilo::settings::SettingsStore;

    fn temp_store() -> SettingsStore {
        static N: AtomicU32 = AtomicU32::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "skribisto_reset_targets_test_{}_{n}.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        SettingsStore::open(path).expect("open temp settings store")
    }

    /// A factory-state world: the theme the launch seeds for a writer who has
    /// never chosen (follow the desktop), and no interface language chosen.
    fn factory_gate(vm: &SettingsViewModel) -> Signal<bool> {
        let theme = Signal::new(crate::startup::theme_for(
            Some(crate::THEME_MODE_SYSTEM),
            false,
            false,
        ));
        build_not_defaults(&theme, &None, &Signal::new(TEXT_SCALE_DEFAULT), vm)
    }

    /// **The pairing test.** For every knob the button restores: it starts at
    /// its default, disturbing it lights the button, and pressing the button
    /// puts it out again.
    ///
    /// This is the binding the 25-key gap needed. Reset and the gate are two
    /// readings of one list, so a knob cannot reach one without the other — and
    /// this drives every row of that list through both, so a 65th knob is
    /// covered the day it is added rather than the day somebody remembers.
    #[test]
    fn every_reset_target_lights_the_gate_and_reset_puts_it_out() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);
        let gate = factory_gate(&vm);

        assert!(
            !gate.get(),
            "a fresh install must not offer to reset anything"
        );

        for target in reset_targets(&vm) {
            target.perturb();
            assert!(
                gate.get(),
                "{} can differ from its default while the Reset button is dead",
                target.id
            );
            vm.reset_editor_defaults();
            assert!(
                !gate.get(),
                "Reset to defaults leaves {} off its default",
                target.id
            );
        }
    }

    /// The same, all at once: every knob off its default, one press, all back.
    /// Separate from the row-by-row walk because a reset arm that clobbers a
    /// *neighbour* passes that one and fails this.
    #[test]
    fn one_press_restores_every_knob_at_once() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);
        let gate = factory_gate(&vm);

        for target in reset_targets(&vm) {
            target.perturb();
        }
        assert!(gate.get());

        vm.reset_editor_defaults();

        for target in reset_targets(&vm) {
            assert!(
                !target.differs().get(),
                "{} survived a full reset",
                target.id
            );
        }
        assert!(!gate.get());
    }

    /// The four pages the gate used to miss entirely. Named one by one because
    /// the failure they caused was invisible — the button was simply dead, with
    /// nothing on screen to say why — and a list is what stops them quietly
    /// leaving again.
    #[test]
    fn the_gate_covers_the_pages_it_used_to_miss() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);
        let ids: BTreeSet<String> = reset_targets(&vm).into_iter().map(|t| t.id).collect();

        for key in [
            // Spelling — F7 alone was enough to strand the button.
            crate::SPELLCHECK_ENABLED_KEY,
            // Punctuation, all six.
            crate::PUNCT_DASHES_KEY,
            crate::PUNCT_ELLIPSIS_KEY,
            crate::PUNCT_QUOTES_KEY,
            crate::PUNCT_QUOTE_STYLE_KEY,
            crate::PUNCT_SPACING_KEY,
            crate::PUNCT_DIALOGUE_KEY,
            // Corkboard, all eleven.
            crate::CORKBOARD_NESTED_KEY,
            crate::CORKBOARD_CARD_SIZE_KEY,
            crate::CORKBOARD_SHOW_WORD_COUNT_KEY,
            crate::CORKBOARD_SHOW_CARD_NUMBERS_KEY,
            crate::CORKBOARD_MODAL_SIZE_KEY,
            crate::CORKBOARD_FONT_FAMILY_KEY,
            crate::CORKBOARD_SIZE_KEY,
            crate::CORKBOARD_LINE_HEIGHT_KEY,
            crate::CORKBOARD_FIRST_LINE_INDENT_KEY,
            crate::CORKBOARD_PARA_SPACING_BEFORE_KEY,
            crate::CORKBOARD_PARA_SPACING_AFTER_KEY,
            // The writing game.
            crate::GAMES_FORWARD_PROSE_KEY,
            crate::GAMES_FORWARD_SYNOPSIS_KEY,
            // The stragglers.
            crate::SYNOPSIS_PLACEMENT_KEY,
            crate::SYNOPSIS_SIDE_WIDTH_KEY,
            crate::DISTRACTION_FREE_THEME_KEY,
            crate::COMMENTS_VISIBLE_KEY,
            crate::PREVIEW_WIDTH_KEY,
        ] {
            assert!(ids.contains(key), "{key} is not on the Reset list");
        }
    }

    /// The keys the reworded confirmation promises reset now reaches, and did
    /// not before. Each was a setting on a page the sentence covers that
    /// survived the button untouched.
    #[test]
    fn the_confirmation_sentence_is_true_of_the_settings_it_names() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);
        let ids: BTreeSet<String> = reset_targets(&vm).into_iter().map(|t| t.id).collect();

        for key in [
            crate::REMEMBER_VIEW_KEY,
            crate::IMAGE_SIZE_POLICY_KEY,
            crate::PACE_SUMMARY_ON_OPEN_KEY,
            crate::MARGIN_LANE_ENABLED_KEY,
            crate::MARGIN_LANE_TEXTURE_KEY,
        ] {
            assert!(ids.contains(key), "{key} is still outside Reset");
        }

        // …and the two it must NOT reach. The signing name is the person, not a
        // preference: restoring editor defaults may not wipe it.
        for key in [crate::USER_NAME_KEY, crate::USER_INITIALS_KEY] {
            assert!(
                !ids.contains(key),
                "{key} is a person, not a setting — Reset must leave it alone"
            );
        }
    }

    /// Every row writes a real, registered key. Ties this list to
    /// `settings_keys::SETTINGS`, so a row cannot name a key `--dump-config`
    /// has never heard of (and a typo in a row's id cannot go unnoticed).
    ///
    /// The lane's per-surface and per-provider rows are exempt: their keys are
    /// synthesised at runtime from a surface's `key()` and a registration's id,
    /// which is exactly why they cannot appear in a compile-time table.
    #[test]
    fn every_row_names_a_registered_setting() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);
        let _guard = crate::settings_ext::lock_registry();

        for target in reset_targets(&vm) {
            if target.id.starts_with("editor.margin_lane.surface.")
                || target.id.starts_with("editor.margin_lane.provider.")
            {
                continue;
            }
            assert!(
                crate::settings_keys::spec(&target.id).is_some(),
                "{} is reset but has no SettingSpec",
                target.id
            );
        }
    }

    /// One knob, one row. A duplicate would make the row-by-row walk above pass
    /// for a key whose *other* row is broken.
    #[test]
    fn no_key_is_listed_twice() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);
        let rows = reset_targets(&vm);
        let unique: BTreeSet<&str> = rows.iter().map(|t| t.id.as_str()).collect();
        assert_eq!(unique.len(), rows.len(), "a key is listed more than once");
    }

    /// The theme is compared as a *mode*. On a dark desktop, factory state is a
    /// dark theme — and must still read as "nothing to reset". This is the bug
    /// that made the button permanently lit, and permanently useless, for every
    /// writer whose desktop is dark.
    #[test]
    fn a_dark_desktop_at_factory_state_does_not_light_the_button() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);

        for desktop_is_dark in [false, true] {
            let theme = Signal::new(crate::startup::theme_for(
                Some(crate::THEME_MODE_SYSTEM),
                false,
                desktop_is_dark,
            ));
            let gate = build_not_defaults(&theme, &None, &Signal::new(TEXT_SCALE_DEFAULT), &vm);
            assert!(
                !gate.get(),
                "following a {} desktop is the factory answer, not a change",
                if desktop_is_dark { "dark" } else { "light" }
            );
        }
    }

    /// …and a theme the writer pinned by hand still lights it, in both
    /// directions. The mode comparison must not have turned the check off.
    #[test]
    fn a_hand_picked_theme_lights_the_button() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);

        for theme in [crate::style::light(), crate::style::dark()] {
            let gate = build_not_defaults(
                &Signal::new(theme),
                &None,
                &Signal::new(TEXT_SCALE_DEFAULT),
                &vm,
            );
            assert!(gate.get(), "a pinned theme is a change from factory");
        }
    }

    /// The lane's synthesised rows are real rows. Its per-surface switches are
    /// three keys built from `LaneSurface::key`, and each registered provider
    /// adds a fourth kind — none of which can be listed at compile time,
    /// because an extension's providers are not known until it registers. A
    /// "reset" that silently skipped them would leave the lane wearing the old
    /// configuration as the only trace of it.
    #[test]
    fn the_margin_lanes_synthesised_rows_reset_like_any_other() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);

        let _handle = crate::margin_lane::register_lane_provider(
            "test.reset",
            crate::margin_lane::LaneProviderSpec {
                id: "reset.probe".to_string(),
                label: std::rc::Rc::new(|| teksilo::prelude::lit!("Probe")),
                hint: std::rc::Rc::new(|| teksilo::prelude::lit!("A test provider")),
                column: crate::widgets::LaneColumn::Left,
                shape: crate::widgets::LaneShape::Square,
                palette_slot: 0,
                surfaces: &[crate::margin_lane::LaneSurface::Editor],
                default_on: true,
                refresh: crate::margin_lane::LaneRefresh::Manual,
                marks: std::rc::Rc::new(|_| Vec::new()),
            },
        )
        .expect("a fresh namespace registers");

        let ids: BTreeSet<String> = reset_targets(&vm).into_iter().map(|t| t.id).collect();
        for surface in crate::margin_lane::LaneSurface::all() {
            assert!(ids.contains(&crate::margin_lane_surface_key(surface)));
        }
        assert!(
            ids.contains("editor.margin_lane.provider.reset.probe"),
            "a registered provider's row is missing from Reset"
        );

        let gate = factory_gate(&vm);
        assert!(!gate.get());
        let provider_row = reset_targets(&vm)
            .into_iter()
            .find(|t| t.id == "editor.margin_lane.provider.reset.probe")
            .expect("the row we just asserted is present");
        provider_row.perturb();
        assert!(gate.get(), "a provider switch must reach the Reset button");
        vm.reset_editor_defaults();
        assert!(!gate.get(), "and must come back with it");
    }

    /// The text scale is still watched — it is one of the three the view-model
    /// does not own, and the easiest to drop while moving the other 69 into a
    /// list.
    #[test]
    fn the_interface_text_scale_still_reaches_the_button() {
        let store = temp_store();
        let vm = SettingsViewModel::new(&store);
        let theme = Signal::new(crate::startup::theme_for(
            Some(crate::THEME_MODE_SYSTEM),
            false,
            false,
        ));
        let scale = Signal::new(TEXT_SCALE_DEFAULT);
        let gate = build_not_defaults(&theme, &None, &scale, &vm);

        assert!(!gate.get());
        scale.set(TEXT_SCALE_DEFAULT + 0.25);
        assert!(gate.get());
        scale.set(TEXT_SCALE_DEFAULT);
        assert!(!gate.get());
    }
}
