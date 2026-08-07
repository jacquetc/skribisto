// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A **distraction-free theme**: the five colours the mode's surface paints with,
//! plus which light/dark base they sit on.
//!
//! ## How a coloured theme stays inside the house rule
//!
//! Skribisto's rule is *semantic theme roles only, no raw colours* — that is what
//! keeps light and dark both working. A theme with a paper colour and an ink
//! colour looks like a violation and is not one, because the colours never reach
//! a widget: they are applied with `WidgetTree::set_theme_override`, which
//! rewrites what the **tokens** resolve to for one subtree. Every widget
//! underneath still asks only for `SurfaceRole::Content` or `TextRole::Primary`;
//! the theme decides what those *mean* there. The hex lives in data — the same
//! sanctioned status tag colours already have — but routed through the token
//! system rather than around it.
//!
//! Two **nested** overrides give the four separable axes, because
//! `WidgetArena::resolve_theme` walks ancestor overrides and a deeper one wins
//! inside its own subtree:
//!
//! | Override on            | Tokens                        | Controls                      |
//! |------------------------|-------------------------------|-------------------------------|
//! | the surface root       | `surface_main`, `text_*`      | general background, widget text |
//! | the editor's backdrop  | `surface_content`, `text_primary` | paper, ink                |
//!
//! ## …except the caret band, which is data, not paint
//!
//! [`caret_band`](DistractionFreeTheme::caret_band) — the shading around the
//! caret — is the one colour a token override *cannot* deliver on its own. The
//! band is not painted by a widget: it crosses into the text document as a
//! `HighlightFormat` field (see `view_models::caret_highlight`), so it is
//! resolved once, in Rust, and pushed onto the editor. A token override is
//! consulted at *paint* time by whoever asks for the role, and nothing asks for
//! this one.
//!
//! So the mode resolves it explicitly: `DistractionFreeSurfaceViewModel` keeps a
//! `Signal<Color>` fed from the theme in force and hands the surface's own tab a
//! `CaretHighlightSettings` built over it. [`apply_to`] still writes the matching
//! token, so a widget that *does* ask for `SurfaceRole::EditorCurrentLineBg`
//! inside the subtree agrees with the prose — but that write is not what makes
//! the band appear.
//!
//! ## Why not a serialized `Theme`
//!
//! teksilo's `Theme` is `Serialize`, but its token structs carry no
//! `#[serde(default)]`, so only a *full-fields* document round-trips — a
//! hand-edited or partial file fails to deserialize outright. A small
//! purpose-built record is both friendlier to edit by hand and immune to that.
//!
//! ## Built-ins are Rust, not data
//!
//! [`builtin_themes`] returns values, following `skribisto_compiler::builtin_presets`
//! and `tags::presets`: type-safe, and they cannot fail to parse at runtime. They
//! are read-only — the settings pane offers Duplicate on them and nothing else.

use teksilo::tokens::Color;
use serde::{Deserialize, Serialize};

/// Which light/dark base a theme's colours sit on.
///
/// Not cosmetic: it decides what every token the theme *doesn't* override
/// resolves to — borders, disabled text, the scroll bar — so a paper-white theme
/// declaring `Dark` would leave the writer with dark-mode chrome around a white
/// page.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeBase {
    #[default]
    Light,
    Dark,
}

/// One distraction-free theme.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DistractionFreeTheme {
    pub id: String,
    pub name: String,
    /// Shipped with the app: read-only, duplicate-to-edit. Always stamped
    /// `false` on load for a user entry, so a hand-edited file cannot
    /// masquerade one of its own as built-in.
    #[serde(default)]
    pub builtin: bool,
    /// Which built-in this was duplicated from, for provenance.
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub base: ThemeBase,
    /// Behind the page — the margin the manuscript floats on, and what the
    /// control strip sits on.
    pub general_background: String,
    /// The page itself.
    pub editor_background: String,
    /// The prose.
    pub editor_text: String,
    /// The control strip's readouts and labels.
    pub widget_text: String,
    /// The band shaded around the caret while you write — the sentence or the
    /// paragraph, per Settings ▸ Editor ▸ "Highlight around the caret".
    ///
    /// **Optional, and empty means "work it out"**: every theme written before
    /// this axis existed — a writer's own, one synced from another machine, one
    /// hand-edited — has no value here, and `Color::from_hex("")` is black,
    /// which would paint a bar of ink across the line being written. So the
    /// stored string is read through [`caret_band_color`](Self::caret_band_color),
    /// which falls back to a tint derived from this theme's own page and prose.
    ///
    /// **May carry alpha** (`#rrggbbaa`), unlike the other four: this is the one
    /// colour that sits *behind* prose on a page whose colour the theme already
    /// fixes, so a writer may reasonably want the paper to show through.
    #[serde(default)]
    pub caret_band: String,
}

/// How far a theme with no [`caret_band`](DistractionFreeTheme::caret_band) of
/// its own nudges its page toward its prose.
///
/// One factor for both bases rather than a light and a dark constant: mixing
/// *toward the ink* is already direction-aware — a dark theme's page moves
/// lighter and a light theme's darker — so a single number stays a tint on both.
/// Small enough not to read as a selection, large enough to be visible at a
/// glance on a near-white page.
const DERIVED_BAND_MIX: f32 = 0.1;

impl DistractionFreeTheme {
    /// The contrast ratio between the prose and its page — the one pairing a
    /// writer stares at for hours, and the one they can most easily wreck.
    pub fn prose_contrast(&self) -> f32 {
        Color::from_hex(&self.editor_background).contrast_ratio(Color::from_hex(&self.editor_text))
    }

    /// The contrast ratio between the strip's text and the **margin** it sits
    /// on.
    ///
    /// The margin, not the page: the surface floats the manuscript as a card and
    /// puts the control strip on the background beside it, so that is the pairing
    /// a writer actually has to read.
    pub fn widget_contrast(&self) -> f32 {
        Color::from_hex(&self.general_background).contrast_ratio(Color::from_hex(&self.widget_text))
    }

    /// The band's colour: the stored one, or — for a theme that carries none — a
    /// tint derived from this theme's own page and prose.
    ///
    /// The derivation is what keeps a theme written before this axis existed
    /// paintable: an empty (or malformed) hex reads as black through
    /// `Color::from_hex`, which behind prose is not a subtle miss but a bar of
    /// ink across the line being written.
    pub fn caret_band_color(&self) -> Color {
        if self.caret_band.trim().is_empty() {
            self.derived_caret_band()
        } else {
            Color::from_hex(&self.caret_band)
        }
    }

    /// The page nudged toward the prose — see [`DERIVED_BAND_MIX`].
    fn derived_caret_band(&self) -> Color {
        Color::from_hex(&self.editor_background)
            .mix(Color::from_hex(&self.editor_text), DERIVED_BAND_MIX)
    }

    /// The band as the eye actually sees it: composited over the page.
    ///
    /// `contrast_ratio` ignores alpha — it assumes both colours are already
    /// composited — so a translucent band has to be flattened here or a barely
    /// visible tint would be scored as though it were opaque.
    pub fn caret_band_over_page(&self) -> Color {
        let band = self.caret_band_color();
        Color::from_hex(&self.editor_background)
            .mix(band, band.a())
            .with_alpha(1.0)
    }

    /// The contrast ratio between the prose and the **band** it is shaded with.
    ///
    /// A third pairing rather than a variation on [`prose_contrast`](Self::prose_contrast):
    /// the band covers the sentence or paragraph being written, so it is the
    /// page under the words a writer is actually looking at — and it is the
    /// colour most easily pushed until it swallows them.
    pub fn caret_band_contrast(&self) -> f32 {
        self.caret_band_over_page()
            .contrast_ratio(Color::from_hex(&self.editor_text))
    }

    /// Whether all three pairings clear WCAG AA for body text.
    ///
    /// A **warning**, never a refusal — the same call `TagsViewModel` makes about
    /// a duplicate tag name. A writer who wants pale grey on cream at 3:1 for an
    /// hour is not making a mistake the app should refuse to carry out; they are
    /// making a choice the app should be honest about.
    pub fn meets_contrast(&self) -> bool {
        self.prose_contrast() >= WCAG_AA_BODY
            && self.widget_contrast() >= WCAG_AA_BODY
            && self.caret_band_contrast() >= WCAG_AA_BODY
    }
}

/// WCAG AA for body text.
pub const WCAG_AA_BODY: f32 = 4.5;

/// The shipped themes. Order is the order the settings pane and the popover list
/// them in, so the first is what a writer with no saved choice gets.
pub fn builtin_themes() -> Vec<DistractionFreeTheme> {
    let t = |id: &str,
             name: &str,
             base: ThemeBase,
             general: &str,
             editor: &str,
             ink: &str,
             widget: &str,
             band: &str| DistractionFreeTheme {
        id: id.to_string(),
        name: name.to_string(),
        builtin: true,
        source: None,
        base,
        general_background: general.to_string(),
        editor_background: editor.to_string(),
        editor_text: ink.to_string(),
        widget_text: widget.to_string(),
        caret_band: band.to_string(),
    };
    // Every band below is stated outright rather than left to the derivation.
    // The derivation exists for themes written before the axis did; a theme the
    // app ships should say what it means, and be checked saying it — the band
    // is one of the three pairings `meets_contrast` covers.
    vec![
        // Near-white page on a soft grey margin — the default, and the closest to
        // the docked editor a writer is coming from.
        t(
            "paper",
            "Paper",
            ThemeBase::Light,
            "#e8e6e1",
            "#fdfdfb",
            "#1f1f1d",
            "#5c5a55",
            // Warm and a touch darker than the page: the docked editor's own
            // pale-yellow band (`#fffeeb`) is *lighter* than this page and would
            // be invisible on it.
            "#f1eddd",
        ),
        // Warm, low-blue: the long-session light theme.
        t(
            "sepia",
            "Sepia",
            ThemeBase::Light,
            "#ddd0b8",
            "#f5ead6",
            "#3b3020",
            // Darker than it looks like it needs to be: the strip's text is read
            // against the *margin*, not the page, and the margin is the darker
            // of the two. At #6b5a42 this pairing was 4.35:1 — under AA, and
            // caught only once the contrast check was aimed at the right surface.
            "#5c4a30",
            // Deeper cream, on the same warm axis as the page — a neutral grey
            // band would read as a smudge on a paper this yellow.
            "#e8d8b8",
        ),
        // Dark, but not black-on-black: a slate page so the prose has an edge.
        t(
            "night",
            "Night",
            ThemeBase::Dark,
            "#14161a",
            "#1e2126",
            "#dfe3e8",
            "#98a0aa",
            // Lifted off the page, not dropped toward the margin: on a dark
            // theme the band has to be the *lighter* of the two or it merges
            // with the background the page already floats on.
            "#2b3038",
        ),
        // Maximum legibility, for tired eyes and bright rooms.
        t(
            "high-contrast",
            "High contrast",
            ThemeBase::Light,
            "#cfcfcf",
            "#ffffff",
            "#000000",
            "#2b2b2b",
            // Unambiguous rather than subtle — the whole point of this theme —
            // while still clearing AA against black prose with room to spare.
            "#d8d8d8",
        ),
        // The dark counterpart of High contrast.
        t(
            "midnight",
            "Midnight",
            ThemeBase::Dark,
            "#000000",
            "#0b0b0c",
            "#f2f2f2",
            "#bdbdbd",
            "#22222a",
        ),
    ]
}

/// Rewrite `theme`'s tokens so a subtree resolves to this distraction-free
/// theme. Installed as a `WidgetTree::set_theme_override` closure at the window
/// root; extracted here so it is a named, testable function rather than logic
/// buried in a window-factory closure.
///
/// It swaps the **whole** palette to the theme's base first. Setting
/// `appearance` alone changes nothing a widget reads: a Night theme over a light
/// palette keeps light borders, light scroll bars and light disabled text around
/// its dark page. Only then do the four painted axes land on the four roles the
/// widgets already tell apart.
///
/// The fifth — the caret band — is written here too, but it is **not** how the
/// band reaches the prose: see this module's docs. A widget under the surface
/// that asks for `SurfaceRole::EditorCurrentLineBg` gets the theme's answer
/// rather than the app palette's, which is the only thing this line buys.
pub fn apply_to(theme: &mut teksilo::prelude::Theme, t: &DistractionFreeTheme) {
    let base = match t.base {
        ThemeBase::Light => teksilo::prelude::intui::light(),
        ThemeBase::Dark => teksilo::prelude::intui::dark(),
    };
    theme.appearance = base.appearance;
    theme.colors = base.colors;
    theme.colors.surface_main = Color::from_hex(&t.general_background);
    theme.colors.surface_content = Color::from_hex(&t.editor_background);
    theme.colors.text_primary = Color::from_hex(&t.editor_text);
    theme.colors.text_secondary = Color::from_hex(&t.widget_text);
    theme.colors.editor_current_line_bg = t.caret_band_color();
}

/// Whether `id` names a shipped theme (cheap — the list is five long).
pub fn is_builtin(id: &str) -> bool {
    builtin_themes().iter().any(|t| t.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Every shipped theme must be legible. A writer can author an unreadable
    /// one of their own and be warned; one we shipped would just be a bug.
    #[test]
    fn every_builtin_theme_meets_contrast() {
        for t in builtin_themes() {
            assert!(
                t.meets_contrast(),
                "builtin `{}` is below WCAG AA: prose {:.2}:1, strip {:.2}:1, band {:.2}:1",
                t.id,
                t.prose_contrast(),
                t.widget_contrast(),
                t.caret_band_contrast()
            );
        }
    }

    /// A shipped theme states its band outright. The derivation is a rescue for
    /// files written before the axis existed, not a way for a built-in to skip
    /// making the choice — and a built-in that fell back would silently stop
    /// being the value this file shows.
    #[test]
    fn every_builtin_theme_names_its_own_caret_band() {
        for t in builtin_themes() {
            assert!(
                !t.caret_band.trim().is_empty(),
                "builtin `{}` leaves its caret band to the derivation",
                t.id
            );
            assert_eq!(
                t.caret_band_color(),
                Color::from_hex(&t.caret_band),
                "builtin `{}` does not read back the band it declares",
                t.id
            );
        }
    }

    /// The band has to be *visible*: a band the same colour as the page shades
    /// nothing, which is indistinguishable from the feature being off.
    #[test]
    fn every_builtin_band_is_distinguishable_from_its_page() {
        for t in builtin_themes() {
            let page = Color::from_hex(&t.editor_background);
            let band = t.caret_band_over_page();
            let delta = (page.r() - band.r()).abs()
                + (page.g() - band.g()).abs()
                + (page.b() - band.b()).abs();
            assert!(
                delta > 0.02,
                "builtin `{}`'s band is indistinguishable from its page ({} vs {})",
                t.id,
                t.editor_background,
                band.to_hex_lower(false)
            );
        }
    }

    #[test]
    fn builtin_ids_are_unique() {
        let ids: HashSet<String> = builtin_themes().into_iter().map(|t| t.id).collect();
        assert_eq!(ids.len(), builtin_themes().len());
    }

    #[test]
    fn every_builtin_theme_has_a_name() {
        for t in builtin_themes() {
            assert!(!t.name.trim().is_empty(), "`{}` has no name", t.id);
        }
    }

    /// The wire format the settings pane's Import/Export uses. If a built-in
    /// stopped round-tripping, an exported theme would not re-import — the
    /// failure `skribisto_compiler::preset` pins for export styles in the same way.
    #[test]
    fn builtin_themes_round_trip_through_json() {
        for t in builtin_themes() {
            let json = serde_json::to_string(&t).expect("serialize");
            let back: DistractionFreeTheme = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, t);
        }
    }

    /// A file written by a later build, or hand-edited, must not fail the load
    /// over a field it does not carry.
    #[test]
    fn a_missing_optional_field_defaults_rather_than_failing() {
        // `r##"…"##`: the hex colours contain `"#`, which would close an `r#"…"#`.
        let json = r##"{
            "id": "mine", "name": "Mine",
            "general_background": "#ffffff", "editor_background": "#ffffff",
            "editor_text": "#000000", "widget_text": "#333333"
        }"##;
        let t: DistractionFreeTheme = serde_json::from_str(json).expect("deserialize");
        assert!(!t.builtin);
        assert_eq!(t.source, None);
        assert_eq!(t.base, ThemeBase::Light);
        assert!(t.caret_band.is_empty());
    }

    /// **The one that matters for every theme a writer already has.** Their
    /// `distraction_free_themes.toml` predates this axis, so the stored string is
    /// empty — and `Color::from_hex("")` is *black*, which behind prose is a bar
    /// of ink across the sentence being written, not a subtle miss.
    #[test]
    fn a_theme_with_no_band_of_its_own_derives_a_tint_rather_than_black() {
        let mut t = builtin_themes()[0].clone();
        t.caret_band = String::new();
        let band = t.caret_band_color();
        assert_ne!(band, Color::BLACK);
        // A tint of the page: much nearer the paper than the ink.
        let page = Color::from_hex(&t.editor_background);
        let ink = Color::from_hex(&t.editor_text);
        assert!(
            band.contrast_ratio(page) < band.contrast_ratio(ink),
            "the derived band drifted off its own page"
        );
        assert!(t.meets_contrast(), "a derived band must stay legible");
    }

    /// Both bases, because the derivation mixes *toward the ink* rather than in
    /// a fixed direction: a dark theme's band has to come out lighter than its
    /// page, a light theme's darker. One constant does both only if this holds.
    #[test]
    fn the_derived_band_moves_toward_the_ink_on_either_base() {
        for id in ["paper", "night"] {
            let mut t = builtin_themes()
                .into_iter()
                .find(|t| t.id == id)
                .expect("shipped");
            t.caret_band = String::new();
            let page = Color::from_hex(&t.editor_background).relative_luminance();
            let ink = Color::from_hex(&t.editor_text).relative_luminance();
            let band = t.caret_band_color().relative_luminance();
            if ink > page {
                assert!(band > page, "`{id}`: dark theme's band must lift off it");
            } else {
                assert!(band < page, "`{id}`: light theme's band must darken it");
            }
        }
    }

    /// A translucent band is scored on what it actually looks like. `contrast_ratio`
    /// ignores alpha, so without the flattening a barely-there tint would be
    /// graded as though it were opaque — and a band at `…00` would be reported as
    /// the strongest possible contrast while showing nothing at all.
    #[test]
    fn a_translucent_band_is_flattened_over_the_page_before_it_is_scored() {
        let mut t = builtin_themes()[0].clone();
        t.caret_band = "#00000000".to_string(); // fully transparent black
        assert_eq!(
            t.caret_band_over_page(),
            Color::from_hex(&t.editor_background),
            "a fully transparent band is just the page"
        );
        assert!((t.caret_band_contrast() - t.prose_contrast()).abs() < 0.01);

        // Half-strength ink over the page lands between the two, not at either.
        t.caret_band = "#1f1f1d80".to_string();
        let flat = t.caret_band_over_page().relative_luminance();
        assert!(flat < Color::from_hex(&t.editor_background).relative_luminance());
        assert!(flat > Color::from_hex(&t.editor_text).relative_luminance());
    }

    /// The band is a third pairing, not a rewording of the first: a theme whose
    /// page and prose are perfectly legible can still hide the words under a
    /// band pushed too far, and that is exactly the mistake the editor makes
    /// invisible until you type into it.
    #[test]
    fn a_band_that_swallows_the_prose_fails_the_check_a_legible_page_passes() {
        let mut t = builtin_themes()[0].clone();
        assert!(t.meets_contrast());
        t.caret_band = t.editor_text.clone();
        assert!(t.prose_contrast() >= WCAG_AA_BODY, "the page is untouched");
        assert!(!t.meets_contrast(), "but the band now hides the prose");
        assert!(t.caret_band_contrast() < WCAG_AA_BODY);
    }

    /// The override lands the five axes on five **different** roles, and swaps
    /// the base palette with them.
    ///
    /// The base swap is the half that is easy to get wrong and invisible in a
    /// unit test that only checks the four: `appearance` is a label, not a
    /// palette, so setting it alone left a dark theme with light borders and a
    /// light scroll bar around its dark page.
    #[test]
    fn applying_a_theme_swaps_the_base_palette_and_the_five_axes() {
        let night = builtin_themes()
            .into_iter()
            .find(|t| t.id == "night")
            .expect("Night is shipped");
        let mut theme = teksilo::prelude::intui::light();
        let light_border = theme.colors.border;
        apply_to(&mut theme, &night);

        assert_eq!(theme.appearance, teksilo::prelude::ThemeAppearance::Dark);
        assert_eq!(
            theme.colors.surface_content,
            Color::from_hex(&night.editor_background)
        );
        assert_eq!(
            theme.colors.text_primary,
            Color::from_hex(&night.editor_text)
        );
        assert_eq!(
            theme.colors.text_secondary,
            Color::from_hex(&night.widget_text)
        );
        assert_eq!(
            theme.colors.surface_main,
            Color::from_hex(&night.general_background)
        );
        assert_eq!(
            theme.colors.editor_current_line_bg,
            Color::from_hex(&night.caret_band),
            "the band token still answers with the theme's colour, not the app \
             palette's, for whoever resolves the role inside the subtree"
        );
        assert_ne!(
            theme.colors.border, light_border,
            "the base palette did not swap — everything the four axes do not \
             name is still light around a dark page"
        );
    }

    /// A light theme leaves a light base, so the same swap is not one-way.
    #[test]
    fn a_light_theme_keeps_a_light_base() {
        let paper = builtin_themes().into_iter().next().unwrap();
        let mut theme = teksilo::prelude::intui::dark();
        apply_to(&mut theme, &paper);
        assert_eq!(theme.appearance, teksilo::prelude::ThemeAppearance::Light);
        assert_eq!(
            theme.colors.border,
            teksilo::prelude::intui::light().colors.border
        );
    }

    /// The warning is a warning about a real thing.
    #[test]
    fn an_illegible_pair_is_detected_but_representable() {
        let mut t = builtin_themes()[0].clone();
        t.editor_text = t.editor_background.clone();
        assert!(!t.meets_contrast());
        // …and still round-trips: it is the writer's choice to carry, not ours
        // to refuse.
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(
            serde_json::from_str::<DistractionFreeTheme>(&json).unwrap(),
            t
        );
    }
}
