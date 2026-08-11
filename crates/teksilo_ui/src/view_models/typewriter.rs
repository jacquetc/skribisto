// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Typewriter scrolling — where the pinned line sits.
//!
//! Not a view-model (it owns no state): the pure vocabulary that turns the
//! stored preset into the viewport fraction `RichTextEditor::typewriter` wants,
//! and into the scroll-past-end fraction its enclosing `ScrollArea` needs. Lives
//! here rather than in the settings pane because it is business logic — the kind
//! of thing the house rules keep out of `build()` — and because both the pane
//! and the editor wiring read it.

use serde::{Deserialize, Serialize};
use teksilo::prelude::Signal;

/// Where the caret's line is held on screen while typewriter scrolling is on.
///
/// Presets rather than a free percentage, following established practice:
/// the three named positions are the ones writers actually reach for, and a
/// slider mostly invites people to pick a meaningless 37 %.
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypewriterAnchor {
    /// Dead centre. The classic, and what every implementation defaults to.
    #[default]
    Middle,
    /// A third of the way down — more of the coming page stays in view, which
    /// suits writers who read ahead while drafting.
    TopThird,
    /// A quarter up from the bottom. Closest to how an actual typewriter sits,
    /// and the position argued for by the widely-cited critique of full
    /// centering: keep the line low, just not jammed against the edge.
    BottomQuarter,
}

impl TypewriterAnchor {
    /// Fraction of the viewport height the caret's line is pinned at, where
    /// `0.0` is flush with the top and `1.0` flush with the bottom.
    pub fn fraction(self) -> f32 {
        match self {
            Self::Middle => 0.5,
            Self::TopThird => 1.0 / 3.0,
            // "Bottom quarter" names where the line sits *measured up from the
            // bottom*, so it is three quarters of the way *down*.
            Self::BottomQuarter => 0.75,
        }
    }

    /// How much scrollable room the page needs past its last line for the pin
    /// to still be reachable there — the complement of [`Self::fraction`], as a
    /// fraction of the viewport height.
    ///
    /// Without this the pin quietly stops working over the final page, which is
    /// exactly where a writer spends their time.
    pub fn scroll_past_end(self) -> f32 {
        1.0 - self.fraction()
    }

    /// Every preset, in the order the settings dropdown lists them: top to
    /// bottom, matching where each one puts the line on screen.
    pub fn all() -> [Self; 3] {
        [Self::TopThird, Self::Middle, Self::BottomQuarter]
    }

    /// Resolve the stored setting, which is an `Option` because that is the
    /// shape a `ComboBox` selection takes. A missing or cleared value falls back
    /// to the default rather than disabling the feature — "no preset chosen" is
    /// not a request to stop pinning.
    pub fn resolve(stored: Option<Self>) -> Self {
        stored.unwrap_or_default()
    }

    /// The anchor to hand `RichTextEditor::typewriter`, given whether the
    /// feature is on and which preset is stored. `None` means "do not pin".
    ///
    /// One function so the editors and their scroll areas can never disagree
    /// about whether pinning is active — a disagreement would show up as a page
    /// that scrolls past its own end for no reason.
    pub fn editor_anchor(enabled: bool, stored: Option<Self>) -> Option<f32> {
        enabled.then(|| Self::resolve(stored).fraction())
    }

    /// The `ScrollArea::scroll_past_end` fraction for the same pair — `0.0` when
    /// pinning is off, so a page with the feature disabled cannot be scrolled
    /// past its last line.
    pub fn page_scroll_past_end(enabled: bool, stored: Option<Self>) -> f32 {
        if enabled {
            Self::resolve(stored).scroll_past_end()
        } else {
            0.0
        }
    }
}

/// The live typewriter-scrolling preference, threaded from Settings down to
/// every writing surface — the pair of source signals rather than one derived
/// value, so consumers can pick the form each of them needs.
///
/// Shaped like [`EditorTypographySet`](super::EditorTypography): one bundle held
/// by `ContentTab`, shared by every editor it builds, so a preference change
/// fans out to all open tabs at once.
///
/// **Why two signals and not one derived one.** A `Signal::map`/`zip` result is
/// read-only and *panics* on `observe()`, which is what `ctx.effect` uses — the
/// same trap the six per-field typography effects exist to avoid. Widget
/// **props** take a different path (`bind_to`, which understands multi-source
/// derived signals), so the scroll-range side can be derived while the
/// editor-push side registers one effect per source.
#[derive(Clone)]
pub struct TypewriterSettings {
    /// Whether the feature is on at all.
    pub enabled: Signal<bool>,
    /// Which preset the pinned line uses. `Option` because it binds straight to
    /// a `ComboBox`; resolve it with [`TypewriterAnchor::resolve`].
    pub preset: Signal<Option<TypewriterAnchor>>,
}

impl TypewriterSettings {
    pub fn new(enabled: Signal<bool>, preset: Signal<Option<TypewriterAnchor>>) -> Self {
        Self { enabled, preset }
    }

    /// Typewriter scrolling off — for the standalone tabs the tab tests build,
    /// and for surfaces that deliberately never pin (the compact synopsis box,
    /// corkboard cards).
    pub fn off() -> Self {
        Self {
            enabled: Signal::new(false),
            preset: Signal::new(Some(TypewriterAnchor::default())),
        }
    }

    /// Current anchor for `RichTextEditor::typewriter` — `None` when off.
    pub fn editor_anchor(&self) -> Option<f32> {
        TypewriterAnchor::editor_anchor(self.enabled.get(), self.preset.get())
    }

    /// Reactive `ScrollArea::scroll_past_end` fraction for a page whose editors
    /// pin. Derived, which is fine (and correct) for a widget prop.
    pub fn scroll_past_end_signal(&self) -> Signal<f32> {
        self.enabled
            .zip(&self.preset)
            .map(|(on, preset)| TypewriterAnchor::page_scroll_past_end(*on, *preset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_preset_maps_to_its_named_position() {
        assert_eq!(TypewriterAnchor::Middle.fraction(), 0.5);
        assert!((TypewriterAnchor::TopThird.fraction() - 0.333_333_34).abs() < 1e-6);
        // A quarter *up from the bottom* is three quarters down — the easy one
        // to get backwards.
        assert_eq!(TypewriterAnchor::BottomQuarter.fraction(), 0.75);
    }

    #[test]
    fn every_preset_stays_inside_the_viewport() {
        for a in TypewriterAnchor::all() {
            let f = a.fraction();
            assert!((0.0..=1.0).contains(&f), "{a:?} maps outside the viewport");
        }
    }

    #[test]
    fn scroll_past_end_complements_the_anchor() {
        // The page must be able to scroll exactly the distance between the pin
        // and the bottom, so the last line can reach the pin and no further.
        for a in TypewriterAnchor::all() {
            assert!((a.fraction() + a.scroll_past_end() - 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn a_missing_preset_falls_back_to_the_default() {
        assert_eq!(TypewriterAnchor::resolve(None), TypewriterAnchor::Middle);
        assert_eq!(
            TypewriterAnchor::resolve(Some(TypewriterAnchor::TopThird)),
            TypewriterAnchor::TopThird
        );
    }

    #[test]
    fn disabling_the_feature_removes_both_the_pin_and_the_extra_range() {
        assert_eq!(
            TypewriterAnchor::editor_anchor(false, Some(TypewriterAnchor::Middle)),
            None
        );
        assert_eq!(
            TypewriterAnchor::page_scroll_past_end(false, Some(TypewriterAnchor::Middle)),
            0.0,
            "a page with pinning off must not scroll past its last line"
        );
    }

    #[test]
    fn enabling_it_pins_at_the_stored_preset() {
        assert_eq!(
            TypewriterAnchor::editor_anchor(true, Some(TypewriterAnchor::BottomQuarter)),
            Some(0.75)
        );
        assert_eq!(
            TypewriterAnchor::page_scroll_past_end(true, Some(TypewriterAnchor::BottomQuarter)),
            0.25
        );
        // …and at the default when nothing is stored yet.
        assert_eq!(TypewriterAnchor::editor_anchor(true, None), Some(0.5));
    }

    #[test]
    fn the_preset_list_reads_top_to_bottom() {
        let order = TypewriterAnchor::all().map(|a| a.fraction());
        assert!(
            order.windows(2).all(|w| w[0] < w[1]),
            "the dropdown must list positions in the order they appear on screen"
        );
    }
}
