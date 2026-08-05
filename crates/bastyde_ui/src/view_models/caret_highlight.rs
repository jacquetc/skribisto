// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The ambient band around the caret — how much of the text it covers, and in what colour.
//!
//! Not a view-model (it owns no state of its own): the vocabulary that turns the stored
//! preference into the [`CaretHighlight`] the framework editor wants, plus the settings bundle
//! threaded down to every writing surface. Lives here rather than in the settings pane for the
//! same reason [`TypewriterAnchor`](super::TypewriterAnchor) does — it is business logic, read
//! by both the pane and the editor wiring.

use bastyde::prelude::Signal;
use bastyde::text_document::{Color, HighlightFormat};
use bastyde::widgets::rich_text::caret_highlight::{CaretHighlight, CaretHighlightScope};
use serde::{Deserialize, Serialize};

/// How much text around the caret is shaded while you write.
///
/// Three choices rather than a toggle, because the useful unit differs by how you work: a
/// sentence is the thing you are shaping word by word, a paragraph the thing you are shaping as
/// a whole. Neither is a superset of the other in usefulness, so the setting names both.
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum HighlightScope {
    /// No band. The default: shading is an aid some writers want and others find busy, and a
    /// writing app should look plain until asked otherwise.
    #[default]
    None,
    /// The sentence the caret is in, per the project's language.
    Sentence,
    /// The whole paragraph the caret is in.
    Paragraph,
}

impl HighlightScope {
    /// Every scope, in the order the `SegmentedControl` lists them: widening from nothing.
    pub fn all() -> [Self; 3] {
        [Self::None, Self::Sentence, Self::Paragraph]
    }

    /// This scope's slot in that control. Paired with [`from_index`](Self::from_index), which is
    /// how the enum crosses to the control's `Signal<usize>`.
    pub fn to_index(self) -> usize {
        match self {
            Self::None => 0,
            Self::Sentence => 1,
            Self::Paragraph => 2,
        }
    }

    /// The scope at a control slot. Anything out of range reads as [`None`](Self::None) rather
    /// than panicking — a stored index can outlive the list it was written against.
    pub fn from_index(index: usize) -> Self {
        match index {
            1 => Self::Sentence,
            2 => Self::Paragraph,
            _ => Self::None,
        }
    }

    /// The framework scope this maps to, or `None` when the band is off.
    fn editor_scope(self) -> Option<CaretHighlightScope> {
        match self {
            Self::None => Option::None,
            Self::Sentence => Some(CaretHighlightScope::Sentence),
            Self::Paragraph => Some(CaretHighlightScope::Paragraph),
        }
    }

    /// The band to hand `RichTextEditor::set_caret_highlight` — `None` means "draw nothing",
    /// which is also how the editor registers no highlight session at all.
    ///
    /// One function so every writing surface asks the same question the same way, the shape
    /// [`TypewriterAnchor::editor_anchor`](super::TypewriterAnchor::editor_anchor) already uses.
    ///
    /// `background_color` only: a paint-only format keeps the band a recolor rather than a
    /// reshape, and keeps it out of the accessibility tree — a screen-reader user has no use
    /// for "the sentence you are in is beige".
    pub fn caret_highlight(self, color: Color, locale: Option<String>) -> Option<CaretHighlight> {
        Some(CaretHighlight {
            scope: self.editor_scope()?,
            format: HighlightFormat {
                background_color: Some(color),
                ..Default::default()
            },
            content_locale: locale,
        })
    }
}

/// The live caret-band preference, threaded from Settings down to every writing surface.
///
/// Shaped like [`TypewriterSettings`](super::TypewriterSettings), and for the same reason: the
/// **source** signals rather than one derived value. A `Signal::map`/`zip` result is read-only
/// and *panics* on `observe()`, which is what `ctx.effect` uses.
///
/// The content locale is deliberately **not** in here. It is per document — the language of the
/// scene being edited — while these two are one shared value across every open tab, so it
/// travels beside `spell` at the call sites that already resolve per-document things.
#[derive(Clone)]
pub struct CaretHighlightSettings {
    /// How much text the band covers, straight from the settings store.
    pub scope: Signal<HighlightScope>,
    /// The band's colour, resolved from the theme's `editor_current_line_bg` role by
    /// [`from_context`](Self::from_context) and re-resolved there on every theme change. A
    /// concrete colour rather than a role, because it crosses into the document as a
    /// `HighlightFormat` field — the same trip the spell-check squiggle colour already makes.
    pub color: Signal<Color>,
}

impl CaretHighlightSettings {
    pub fn new(scope: Signal<HighlightScope>, color: Signal<Color>) -> Self {
        Self { scope, color }
    }

    /// The live bundle for this window: the stored scope, plus the band colour resolved from
    /// the theme's `editor_current_line_bg` role and kept current by an effect.
    ///
    /// One constructor because two unrelated places need the same bundle — `App::build` for
    /// every tab's writing surfaces, and the Search & Replace preview dock, which builds its
    /// editor by hand rather than through `TypographyBoundEditor`. Duplicating the theme effect
    /// is how one of them ends up not following a light/dark switch.
    ///
    /// The colour must be a **genuine source signal**, not a `.map()` of the theme: a derived
    /// signal is read-only and panics on `observe()`, which is what `ctx.effect` uses.
    pub fn from_context(ctx: &mut bastyde::prelude::BuildContext) -> Self {
        use bastyde::prelude::SettingsExt;
        let scope = crate::view_models::SettingsViewModel::new(ctx.settings()).highlight_scope();
        let color = Signal::new(Self::document_color(
            ctx.theme().colors.editor_current_line_bg,
        ));
        {
            let sig = color.clone();
            let theme_sig = ctx.theme_signal().clone();
            ctx.effect(&theme_sig, move |t| {
                let next = Self::document_color(t.colors.editor_current_line_bg);
                if sig.get() != next {
                    sig.set(next);
                }
            });
        }
        Self::new(scope, color)
    }

    /// No band — for the surfaces that never draw one, and for the widget tests, which build
    /// editors with no app around them.
    pub fn off() -> Self {
        Self {
            scope: Signal::new(HighlightScope::None),
            color: Signal::new(Color::rgb(0, 0, 0)),
        }
    }

    /// The band for a surface whose document is in `locale`.
    pub fn caret_highlight(&self, locale: Option<String>) -> Option<CaretHighlight> {
        self.scope.get().caret_highlight(self.color.get(), locale)
    }

    /// Carry a theme colour across into the document, **keeping the alpha**.
    ///
    /// The spell-check squiggle's converter drops alpha — a line drawn over the text wants to be
    /// solid. A band sits *behind* whole words and may well be specified translucent so the paper
    /// shows through, so this one must not.
    ///
    /// Public because the distraction-free surface resolves its band from its *theme* rather
    /// than from the app palette (see `distraction_free::theme`) and has to make the same
    /// crossing — a second converter there that dropped alpha would quietly paint a translucent
    /// theme band opaque.
    pub fn document_color(c: bastyde::tokens::Color) -> Color {
        let [r, g, b, a] = c.to_array();
        let to_u8 = |x: f32| (x.clamp(0.0, 1.0) * 255.0).round() as u8;
        Color::rgba(to_u8(r), to_u8(g), to_u8(b), to_u8(a))
    }
}

/// One writing surface's band: the app-wide preference, plus the language of the document that
/// surface is over.
///
/// The two travel together because a surface needs both and neither belongs to the other — the
/// preference is one shared value across every open tab, while the language is a property of
/// the scene being edited (and matters only to the sentence scope; a paragraph is a paragraph
/// in any language).
///
/// The language is resolved when the tab is built, from the same
/// `OpenDocsStore::effective_language` the spell-checker reads. Changing a project's language
/// therefore reaches open bands on the next tab build, exactly as it reaches their typography.
#[derive(Clone)]
pub struct CaretBand {
    pub settings: CaretHighlightSettings,
    pub locale: Option<String>,
}

impl CaretBand {
    pub fn new(settings: CaretHighlightSettings, locale: Option<String>) -> Self {
        Self { settings, locale }
    }

    /// No band — for the surfaces that never draw one, and for the widget tests, which build
    /// editors with no app around them.
    pub fn off() -> Self {
        Self {
            settings: CaretHighlightSettings::off(),
            locale: None,
        }
    }

    /// The band to push onto an editor right now.
    pub fn resolve(&self) -> Option<CaretHighlight> {
        self.settings.caret_highlight(self.locale.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The control is bound by index, so a stored scope that rendered as the wrong segment — or
    /// a segment that wrote back the wrong scope — would silently mis-set the preference.
    #[test]
    fn the_index_bridge_round_trips_every_variant() {
        for scope in HighlightScope::all() {
            assert_eq!(HighlightScope::from_index(scope.to_index()), scope);
        }
        assert_eq!(HighlightScope::None.to_index(), 0);
        assert_eq!(HighlightScope::Paragraph.to_index(), 2);
    }

    /// `all()` must list the variants in the order their indices claim, or the segments would
    /// be labelled in one order and selected in another.
    #[test]
    fn all_is_listed_in_index_order() {
        for (i, scope) in HighlightScope::all().into_iter().enumerate() {
            assert_eq!(scope.to_index(), i);
        }
    }

    /// An index past the end reads as "no band" instead of panicking.
    #[test]
    fn an_out_of_range_index_reads_as_no_band() {
        assert_eq!(HighlightScope::from_index(99), HighlightScope::None);
    }

    #[test]
    fn the_off_scope_asks_for_no_band_at_all() {
        assert!(
            HighlightScope::None
                .caret_highlight(Color::rgb(1, 2, 3), Some("en".into()))
                .is_none()
        );
        assert!(
            CaretHighlightSettings::off()
                .caret_highlight(None)
                .is_none()
        );
    }

    #[test]
    fn each_scope_maps_to_its_framework_counterpart() {
        let color = Color::rgb(255, 254, 235);
        let sentence = HighlightScope::Sentence
            .caret_highlight(color, Some("fr-FR".into()))
            .expect("a band");
        assert_eq!(sentence.scope, CaretHighlightScope::Sentence);
        assert_eq!(sentence.content_locale.as_deref(), Some("fr-FR"));

        let paragraph = HighlightScope::Paragraph
            .caret_highlight(color, None)
            .expect("a band");
        assert_eq!(paragraph.scope, CaretHighlightScope::Paragraph);
    }

    /// Paint-only, or the band would reshape the text on every caret move and announce itself
    /// to a screen reader.
    #[test]
    fn the_band_format_is_background_only() {
        let color = Color::rgb(255, 254, 235);
        let band = HighlightScope::Sentence
            .caret_highlight(color, None)
            .expect("a band");
        assert_eq!(band.format.background_color, Some(color));
        assert_eq!(
            band.format,
            HighlightFormat {
                background_color: Some(color),
                ..Default::default()
            }
        );
    }

    #[test]
    fn the_bundle_reads_its_live_signals() {
        let settings = CaretHighlightSettings::new(
            Signal::new(HighlightScope::None),
            Signal::new(Color::rgb(1, 2, 3)),
        );
        assert!(settings.caret_highlight(None).is_none());

        settings.scope.set(HighlightScope::Paragraph);
        let band = settings.caret_highlight(None).expect("a band");
        assert_eq!(band.scope, CaretHighlightScope::Paragraph);

        settings.color.set(Color::rgb(9, 9, 9));
        let band = settings.caret_highlight(None).expect("a band");
        assert_eq!(band.format.background_color, Some(Color::rgb(9, 9, 9)));
    }
}
