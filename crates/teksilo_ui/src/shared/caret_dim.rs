// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The fade around the caret: everything but the sentence or paragraph being written is drawn
//! lighter, so the eye stays on the words in hand.
//!
//! Not a view-model (it owns no state of its own): the vocabulary that turns the stored
//! preference into the [`CaretDim`] the framework editor wants, plus the settings bundle threaded
//! to every writing surface beside the [caret band](super::CaretHighlightSettings). The two are
//! separate preferences and combine: a sentence band over a paragraph fade is a common setup.
//!
//! The fade is drawn by each editor's own typesetter and never written to the document, so it
//! belongs to the view that shows it. See teksilo's `rich_text::caret_dim`.

use serde::{Deserialize, Serialize};
use teksilo::prelude::Signal;
use teksilo::widgets::rich_text::caret_dim::CaretDim;
use teksilo::widgets::rich_text::caret_highlight::CaretHighlightScope;

/// Which unit stays at full strength around the caret.
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum DimScope {
    /// The sentence the caret is in, per the document's language.
    Sentence,
    /// The whole paragraph the caret is in. The default: it is what the writers who asked for
    /// the feature described, and the gentler of the two.
    #[default]
    Paragraph,
}

impl DimScope {
    /// Every scope, in the order the `SegmentedControl` lists them: narrowest first, like the
    /// band's own control.
    pub fn all() -> [Self; 2] {
        [Self::Sentence, Self::Paragraph]
    }

    /// This scope's slot in that control. Paired with [`from_index`](Self::from_index).
    pub fn to_index(self) -> usize {
        match self {
            Self::Sentence => 0,
            Self::Paragraph => 1,
        }
    }

    /// The scope at a control slot. Anything out of range reads as the default rather than
    /// panicking: a stored index can outlive the list it was written against.
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Sentence,
            1 => Self::Paragraph,
            _ => Self::default(),
        }
    }

    fn editor_scope(self) -> CaretHighlightScope {
        match self {
            Self::Sentence => CaretHighlightScope::Sentence,
            Self::Paragraph => CaretHighlightScope::Paragraph,
        }
    }
}

/// How far the rest of the text fades.
///
/// Three presets rather than a slider, following the settings' convention (the typewriter
/// position is presets too). The strength is the choice writers disagree about most: some want
/// the context barely softened, others want it nearly gone.
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum DimStrength {
    /// The rest stays easy to read, only softened.
    Light,
    /// Clearly receded, still readable at a glance. The default.
    #[default]
    Medium,
    /// Nearly gone: shapes of paragraphs rather than words.
    Strong,
}

impl DimStrength {
    /// Every strength, weakest first.
    pub fn all() -> [Self; 3] {
        [Self::Light, Self::Medium, Self::Strong]
    }

    /// This strength's slot in the `SegmentedControl`. Paired with
    /// [`from_index`](Self::from_index).
    pub fn to_index(self) -> usize {
        match self {
            Self::Light => 0,
            Self::Medium => 1,
            Self::Strong => 2,
        }
    }

    /// The strength at a control slot; out of range reads as the default.
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Light,
            1 => Self::Medium,
            2 => Self::Strong,
            _ => Self::default(),
        }
    }

    /// The opacity the faded text is drawn at.
    ///
    /// Multiplied into the theme's own text colour, so the same three values hold on a light,
    /// a dark or a sepia page: the faded text keeps its hue and only gives up contrast.
    pub fn opacity(self) -> f32 {
        match self {
            Self::Light => 0.55,
            Self::Medium => 0.35,
            Self::Strong => 0.18,
        }
    }
}

/// The live fade preference, threaded from Settings down to every writing surface.
///
/// The **source** signals rather than one derived value, for the reason
/// [`CaretHighlightSettings`](super::CaretHighlightSettings) gives: a `Signal::map`/`zip` result
/// is read-only and panics on `observe()`, which is what `ctx.effect` uses.
///
/// The on/off switch is its own signal, apart from the scope, so the quick toggle (F8, the title
/// bar, View ▸ Fade the rest of the text) flips the fade without forgetting which unit and
/// how strong.
#[derive(Clone)]
pub struct CaretDimSettings {
    /// Whether the fade is on.
    pub enabled: Signal<bool>,
    /// Which unit stays bright.
    pub scope: Signal<DimScope>,
    /// How far the rest fades.
    pub strength: Signal<DimStrength>,
}

impl CaretDimSettings {
    pub fn new(
        enabled: Signal<bool>,
        scope: Signal<DimScope>,
        strength: Signal<DimStrength>,
    ) -> Self {
        Self {
            enabled,
            scope,
            strength,
        }
    }

    /// The live bundle over the settings store: the same three store-backed signals the
    /// Settings page, the menu and the title bar move.
    pub fn from_settings(settings: &crate::settings::SettingsViewModel) -> Self {
        Self::new(
            settings.dim_enabled(),
            settings.dim_scope(),
            settings.dim_strength(),
        )
    }

    /// No fade: for the surfaces that never draw one (the Search & Replace preview, the story
    /// bible's reading) and for the widget tests, which build editors with no app around them.
    pub fn off() -> Self {
        Self::new(
            Signal::new(false),
            Signal::new(DimScope::default()),
            Signal::new(DimStrength::default()),
        )
    }

    /// The fade to hand `EditorHandle::set_caret_dim` for a surface whose document is in
    /// `locale`, or `None` while the fade is off.
    pub fn caret_dim(&self, locale: Option<String>) -> Option<CaretDim> {
        self.enabled.get().then(|| CaretDim {
            scope: self.scope.get().editor_scope(),
            opacity: self.strength.get().opacity(),
            content_locale: locale,
            // The page is the surface's, not the preference's: see `CaretBand::on_page`.
            page: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The controls are bound by index, so a stored value that rendered as the wrong segment, or
    /// a segment that wrote back the wrong value, would silently mis-set the preference.
    #[test]
    fn the_index_bridges_round_trip_every_variant() {
        for (i, scope) in DimScope::all().into_iter().enumerate() {
            assert_eq!(scope.to_index(), i, "listed in index order");
            assert_eq!(DimScope::from_index(i), scope);
        }
        for (i, strength) in DimStrength::all().into_iter().enumerate() {
            assert_eq!(strength.to_index(), i, "listed in index order");
            assert_eq!(DimStrength::from_index(i), strength);
        }
    }

    #[test]
    fn an_out_of_range_index_reads_as_the_default() {
        assert_eq!(DimScope::from_index(9), DimScope::Paragraph);
        assert_eq!(DimStrength::from_index(9), DimStrength::Medium);
    }

    /// Stronger means fainter, and every preset still draws something: an opacity of zero would
    /// hide the text the writer may need to glance back at.
    #[test]
    fn each_strength_is_fainter_than_the_last_and_never_invisible() {
        let opacities: Vec<f32> = DimStrength::all().iter().map(|s| s.opacity()).collect();
        assert!(opacities.windows(2).all(|w| w[0] > w[1]), "{opacities:?}");
        assert!(opacities.iter().all(|o| *o > 0.0 && *o < 1.0));
    }

    #[test]
    fn off_asks_for_no_fade() {
        assert!(
            CaretDimSettings::off()
                .caret_dim(Some("en".into()))
                .is_none()
        );
    }

    #[test]
    fn each_scope_maps_to_its_framework_counterpart() {
        let settings = CaretDimSettings::new(
            Signal::new(true),
            Signal::new(DimScope::Sentence),
            Signal::new(DimStrength::Strong),
        );
        let dim = settings.caret_dim(Some("fr-FR".into())).expect("a fade");
        assert_eq!(dim.scope, CaretHighlightScope::Sentence);
        assert_eq!(dim.opacity, DimStrength::Strong.opacity());
        assert_eq!(dim.content_locale.as_deref(), Some("fr-FR"));

        settings.scope.set(DimScope::Paragraph);
        let dim = settings.caret_dim(None).expect("a fade");
        assert_eq!(dim.scope, CaretHighlightScope::Paragraph);
    }

    /// The bundle reads its live signals each time, so a change in Settings reaches the next
    /// push without the bundle being rebuilt. Turning it off keeps the scope and strength.
    #[test]
    fn the_bundle_reads_its_live_signals() {
        let settings = CaretDimSettings::off();
        settings.strength.set(DimStrength::Light);
        settings.enabled.set(true);
        assert_eq!(
            settings.caret_dim(None).map(|d| d.opacity),
            Some(DimStrength::Light.opacity())
        );
        settings.enabled.set(false);
        assert!(settings.caret_dim(None).is_none());
        assert_eq!(
            settings.strength.get(),
            DimStrength::Light,
            "the choice survives"
        );
    }

    /// The enums are stored by variant name in `general.toml`; a renamed variant would read
    /// every existing preference back as the default.
    #[test]
    fn the_stored_names_are_the_variant_names() {
        assert_eq!(
            toml::Value::try_from(DimScope::Sentence).ok(),
            Some(toml::Value::String("Sentence".into()))
        );
        assert_eq!(
            toml::Value::try_from(DimStrength::Strong).ok(),
            Some(toml::Value::String("Strong".into()))
        );
    }
}
