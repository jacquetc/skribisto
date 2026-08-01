// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A **distraction-free theme**: the four colours the mode's surface paints with,
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
//! ## Why not a serialized `Theme`
//!
//! bastyde's `Theme` is `Serialize`, but its token structs carry no
//! `#[serde(default)]`, so only a *full-fields* document round-trips — a
//! hand-edited or partial file fails to deserialize outright. A small
//! purpose-built record is both friendlier to edit by hand and immune to that.
//!
//! ## Built-ins are Rust, not data
//!
//! [`builtin_themes`] returns values, following `skribisto_compiler::builtin_presets`
//! and `tags::presets`: type-safe, and they cannot fail to parse at runtime. They
//! are read-only — the settings pane offers Duplicate on them and nothing else.

use bastyde::tokens::Color;
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
}

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

    /// Whether both pairings clear WCAG AA for body text.
    ///
    /// A **warning**, never a refusal — the same call `TagsViewModel` makes about
    /// a duplicate tag name. A writer who wants pale grey on cream at 3:1 for an
    /// hour is not making a mistake the app should refuse to carry out; they are
    /// making a choice the app should be honest about.
    pub fn meets_contrast(&self) -> bool {
        self.prose_contrast() >= WCAG_AA_BODY && self.widget_contrast() >= WCAG_AA_BODY
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
             widget: &str| DistractionFreeTheme {
        id: id.to_string(),
        name: name.to_string(),
        builtin: true,
        source: None,
        base,
        general_background: general.to_string(),
        editor_background: editor.to_string(),
        editor_text: ink.to_string(),
        widget_text: widget.to_string(),
    };
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
/// its dark page. Only then do the four axes land on the four roles the widgets
/// already tell apart.
pub fn apply_to(theme: &mut bastyde::prelude::Theme, t: &DistractionFreeTheme) {
    let base = match t.base {
        ThemeBase::Light => bastyde::prelude::intui::light(),
        ThemeBase::Dark => bastyde::prelude::intui::dark(),
    };
    theme.appearance = base.appearance;
    theme.colors = base.colors;
    theme.colors.surface_main = Color::from_hex(&t.general_background);
    theme.colors.surface_content = Color::from_hex(&t.editor_background);
    theme.colors.text_primary = Color::from_hex(&t.editor_text);
    theme.colors.text_secondary = Color::from_hex(&t.widget_text);
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
                "builtin `{}` is below WCAG AA: prose {:.2}:1, strip {:.2}:1",
                t.id,
                t.prose_contrast(),
                t.widget_contrast()
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
    }

    /// The override lands the four axes on four **different** roles, and swaps
    /// the base palette with them.
    ///
    /// The base swap is the half that is easy to get wrong and invisible in a
    /// unit test that only checks the four: `appearance` is a label, not a
    /// palette, so setting it alone left a dark theme with light borders and a
    /// light scroll bar around its dark page.
    #[test]
    fn applying_a_theme_swaps_the_base_palette_and_the_four_axes() {
        let night = builtin_themes()
            .into_iter()
            .find(|t| t.id == "night")
            .expect("Night is shipped");
        let mut theme = bastyde::prelude::intui::light();
        let light_border = theme.colors.border;
        apply_to(&mut theme, &night);

        assert_eq!(theme.appearance, bastyde::prelude::ThemeAppearance::Dark);
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
        let mut theme = bastyde::prelude::intui::dark();
        apply_to(&mut theme, &paper);
        assert_eq!(theme.appearance, bastyde::prelude::ThemeAppearance::Light);
        assert_eq!(
            theme.colors.border,
            bastyde::prelude::intui::light().colors.border
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
