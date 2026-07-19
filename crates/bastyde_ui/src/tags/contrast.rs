// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Turning a tag's stored colour into something legible.
//!
//! Tag colours are user *data* — the one sanctioned exception to the semantic-colour rule —
//! which leaves two contrast problems, and only one of them is about text.
//!
//! **Text colour is derived, never stored.** `Color::best_contrast_text` compares the actual
//! WCAG ratios rather than thresholding luminance, so it lands on the ~0.179 crossover where
//! black and white are equally legible. That yields a guarantee rather than a heuristic:
//! contrast against white is `1.05/(L+0.05)` and against black is `(L+0.05)/0.05`, they cross
//! at L ≈ 0.179 where both equal **4.58:1**, so taking the better of the two clears WCAG AA
//! (4.5:1) for *every possible* tag colour. A writer choosing their own text colour could
//! trivially produce an unreadable pair; the derived one cannot.
//!
//! **The fill needs a border.** None of the above helps the harder problem: a tag's colour is
//! theme-constant, so a near-black tag is invisible on a dark surface and a near-white one
//! vanishes on a light surface — and where tags render as dots there is no text involved at
//! all. WCAG SC 1.4.11 wants ≥3:1 for graphical objects, which a tag dot is. Hence
//! [`needs_outline`] and the hairline every chip and dot draws.

use bastyde::tokens::Color;

/// Used when a tag's stored colour is unparseable. Mid-slate: legible in both themes, and
/// visibly "unset" rather than pretending to be a choice.
pub const FALLBACK: &str = "#607d8b";

/// WCAG SC 1.4.11 — the floor for a UI component or graphical object against its background.
const GRAPHICAL_OBJECT_MIN: f32 = 3.0;

/// Parse a stored `#rrggbb`, falling back rather than yielding black.
///
/// `Color::from_hex` maps any unparseable byte to 0, so a malformed value would silently
/// become pure black — the single worst outcome, since it is invisible in a dark theme *and*
/// looks deliberate. A legacy or imported project can easily hold something odd here.
pub fn parse(hex: &str) -> Color {
    if is_hex(hex) {
        Color::from_hex(hex)
    } else {
        Color::from_hex(FALLBACK)
    }
}

fn is_hex(s: &str) -> bool {
    let body = s.strip_prefix('#').unwrap_or("");
    (body.len() == 6 || body.len() == 8) && body.chars().all(|c| c.is_ascii_hexdigit())
}

/// The text colour to paint on `fill` — always derived, never read from storage.
pub fn text_on(fill: Color) -> Color {
    fill.best_contrast_text()
}

/// Whether a chip/dot of this colour needs its hairline outline to stay visible against
/// `surface`.
///
/// Every chip draws the outline unconditionally in practice (it costs nothing and keeps the
/// look uniform); this is what the settings pane uses to *warn* the writer that a colour is
/// doing no work — it will be visible thanks to the border, but it will not read as the
/// colour they picked.
pub fn needs_outline(fill: Color, surface: Color) -> bool {
    fill.contrast_ratio(surface) < GRAPHICAL_OBJECT_MIN
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The guarantee the whole "derive it" decision rests on: for *any* fill, the better of
    /// black and white clears WCAG AA. If this ever fails, storing a user-chosen text colour
    /// would start to look reasonable again — so it is worth checking rather than asserting
    /// in a comment.
    #[test]
    fn derived_text_clears_wcag_aa_for_every_colour() {
        // Walk the cube coarsely; the crossover sits near L≈0.179, well inside this net.
        for r in (0..=255).step_by(15) {
            for g in (0..=255).step_by(15) {
                for b in (0..=255).step_by(15) {
                    let fill = Color::from_hex(&format!("#{r:02x}{g:02x}{b:02x}"));
                    let ratio = fill.contrast_ratio(text_on(fill));
                    assert!(
                        ratio >= 4.5,
                        "#{r:02x}{g:02x}{b:02x} got {ratio:.2}:1, below AA"
                    );
                }
            }
        }
    }

    #[test]
    fn a_light_fill_takes_black_text_and_a_dark_one_white() {
        assert_eq!(text_on(Color::from_hex("#ffffff")), Color::BLACK);
        assert_eq!(text_on(Color::from_hex("#000000")), Color::WHITE);
    }

    #[test]
    fn a_malformed_colour_falls_back_rather_than_going_black() {
        // The failure that matters: `Color::from_hex` would make these pure black, which is
        // invisible in a dark theme and looks like a deliberate choice.
        for bad in ["", "octarine", "#12345", "#gggggg", "1e7d32"] {
            assert_eq!(
                parse(bad),
                Color::from_hex(FALLBACK),
                "{bad:?} should fall back"
            );
        }
    }

    #[test]
    fn a_well_formed_colour_is_kept() {
        assert_eq!(parse("#2e7d32"), Color::from_hex("#2e7d32"));
        assert_eq!(parse("#2E7D32"), Color::from_hex("#2E7D32"));
        assert_eq!(parse("#2e7d32ff"), Color::from_hex("#2e7d32ff"));
    }

    /// The case the outline exists for: an extreme colour against the matching surface.
    #[test]
    fn extremes_need_an_outline_against_the_matching_surface() {
        let dark_surface = Color::from_hex("#1e1e1e");
        let light_surface = Color::from_hex("#ffffff");
        assert!(
            needs_outline(Color::from_hex("#1a1a1a"), dark_surface),
            "a near-black tag is invisible on a dark surface"
        );
        assert!(
            needs_outline(Color::from_hex("#fafafa"), light_surface),
            "a near-white tag is invisible on a light surface"
        );
        // A mid hue reads fine against both, so it raises no warning either way.
        assert!(!needs_outline(Color::from_hex("#2980b9"), light_surface));
    }
}
