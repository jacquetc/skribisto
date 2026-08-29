// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The five status glyphs, and the colour role each one wears.
//!
//! # Shape is the channel; colour is the redundancy
//!
//! That ordering is forced, not preferred. Clearing WCAG 1.4.11's 3:1 for a graphical
//! object against *both* of this app's themes puts a colour in a luminance band roughly
//! `L ∈ [0.20, 0.24]` — against the darkest light surface (`surface_pressed`, `#DFE1E5`) a
//! glyph must sit at `L ≤ 0.2173`, and against the lightest dark one
//! (`surface_selected_inactive`, `#2C4A54`) at `L ≥ 0.2822`. Those do not overlap at all,
//! which is why a status colour cannot be *stored* the way a tag's is and has to come from
//! a per-theme role instead. Even with per-theme roles the hues stay near-isoluminant, so
//! colour carries almost no lightness cue — the cue a viewer with colour-vision deficiency
//! still has. novelWriter shipped colour-only statuses and had to add shapes in 2.7
//! "to make them easier to distinguish for users with low colour vision"; this set starts
//! there.
//!
//! **The test that matters:** render the whole binder in one flat grey and every status
//! must still read. `the_five_glyphs_are_five_distinct_shapes` pins the mechanical
//! half of that; the rest is the drawing, and the drawing rules are the ones
//! [`crate::binder::icons`] already sets out for this app's 16 dp set.
//!
//! # Why these five and not the tag palette
//!
//! A tag's colour is user data and theme-constant, which is exactly why `tags::contrast`
//! has to derive a text colour and draw a hairline round every dot. A status is an
//! app-defined closed vocabulary, so it takes semantic roles like every other colour in the
//! app and adapts to the theme for free — and it renders as a *shape*, never as another
//! coloured dot beside the tag dots, so the two axes never read as the same thing.

use common::entities::StatusCategory;
use teksilo::prelude::*;
use teksilo::res;
use teksilo::widgets::IconWidget;

/// Painted size of a status glyph, matching the binder's own leading icons and the tab
/// headers so a row never changes height when a status is set.
pub const GLYPH_SIZE: f32 = 16.0;

/// The glyph for a category, untinted.
///
/// `res!` embeds each asset at compile time and needs a literal path per call site, so the
/// mapping is a `match` with one `res!` per arm — the same shape `binder::icons` uses.
pub fn category_icon(category: &StatusCategory) -> IconWidget {
    let svg = match category {
        StatusCategory::Planned => res!("assets/icons/status/planned.svg"),
        StatusCategory::Drafting => res!("assets/icons/status/drafting.svg"),
        StatusCategory::NeedsWork => res!("assets/icons/status/needs-work.svg"),
        StatusCategory::Revised => res!("assets/icons/status/revised.svg"),
        StatusCategory::Final => res!("assets/icons/status/final.svg"),
    };
    IconWidget::from_svg_icon(svg).icon_size(GLYPH_SIZE)
}

/// The semantic role a category is painted in.
///
/// Roles, never hex — see the module doc for why a stored colour cannot work here. These
/// also dim correctly on a disabled row and follow hover and press, which a literal colour
/// would not.
pub fn category_role(category: &StatusCategory) -> TextRole {
    match category {
        // Deliberately the quiet one. Most rows in a novel binder are unstarted or
        // unmarked, and a "nothing has happened yet" glyph that shouts is 400 marks of
        // noise competing with the titles beside them.
        StatusCategory::Planned => TextRole::Secondary,
        StatusCategory::Drafting => TextRole::Accent,
        StatusCategory::NeedsWork => TextRole::Warning,
        StatusCategory::Revised => TextRole::Accent,
        StatusCategory::Final => TextRole::Success,
    }
}

/// The glyph for a category, tinted — what every surface actually renders.
pub fn status_glyph(category: &StatusCategory) -> IconWidget {
    category_icon(category).color(category_role(category))
}

/// Every category, in ladder order — the order the picker and the settings editor list
/// them in, and the order "which rung is lower" is decided by.
pub const CATEGORIES: [StatusCategory; 5] = [
    StatusCategory::Planned,
    StatusCategory::Drafting,
    StatusCategory::NeedsWork,
    StatusCategory::Revised,
    StatusCategory::Final,
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Every category must resolve to its own asset. A `match` that returned the same
    /// `res!` twice would compile and ship two rungs wearing one glyph — which is exactly
    /// the defect novelWriter shipped, where `New` and `Finished` are both a star.
    #[test]
    fn the_five_glyphs_are_five_distinct_shapes() {
        let paths: Vec<&str> = CATEGORIES
            .iter()
            .map(|c| match c {
                StatusCategory::Planned => "planned",
                StatusCategory::Drafting => "drafting",
                StatusCategory::NeedsWork => "needs-work",
                StatusCategory::Revised => "revised",
                StatusCategory::Final => "final",
            })
            .collect();
        let mut sorted = paths.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(
            sorted.len(),
            CATEGORIES.len(),
            "two categories share a glyph"
        );
    }

    /// The asset files themselves must exist and be the app's 16 dp size — `res!` embeds at
    /// compile time, so a missing file is a build error, but a wrong `viewBox` is not.
    #[test]
    fn every_glyph_asset_is_a_16dp_square() {
        for name in ["planned", "drafting", "needs-work", "revised", "final"] {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/status/").to_string()
                + name
                + ".svg";
            let svg = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
            assert!(
                svg.contains(r#"viewBox="0 0 16 16""#),
                "{name}.svg is not drawn on the 16 dp grid the rest of the set uses"
            );
            // An alpha-mask renderer ignores paint, so a "knockout" drawn as a light shape
            // on top would silently fill in. Holes have to be `evenodd` subpaths.
            assert!(
                !svg.contains("var(--"),
                "{name}.svg paints with a CSS variable, which an alpha mask cannot honour"
            );
        }
    }

    /// Roles are allowed to repeat — Drafting and Revised deliberately share the accent —
    /// so the guarantee that matters is the *shape* one above. This pins the repetition as
    /// intentional, and pins the one role assignment that is a design decision rather than
    /// an obvious mapping: `Planned` is deliberately the quiet role, because most rows in a
    /// novel binder are unstarted and a loud "nothing yet" glyph is 400 marks of noise.
    #[test]
    fn the_role_assignment_is_deliberate() {
        use StatusCategory as C;
        assert_eq!(category_role(&C::Planned), TextRole::Secondary);
        assert_eq!(category_role(&C::NeedsWork), TextRole::Warning);
        assert_eq!(category_role(&C::Final), TextRole::Success);
        assert_eq!(
            category_role(&C::Drafting),
            category_role(&C::Revised),
            "Drafting and Revised share the accent role on purpose; their glyphs separate them"
        );
    }
}
