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
/// Every chip draws the outline unconditionally (see [`outline_on`]); this is what the
/// settings pane uses to *warn* the writer that a colour is doing no work — it will be
/// visible thanks to the border, but it will not read as the colour they picked.
pub fn needs_outline(fill: Color, surface: Color) -> bool {
    fill.contrast_ratio(surface) < GRAPHICAL_OBJECT_MIN
}

/// The hairline colour for a chip/dot of this fill — derived from the fill, exactly like
/// [`text_on`], and for the same reason.
///
/// The obvious implementation, a themed border token, does not work and the numbers say so:
/// `BorderRole::Default` resolves to `#E3E5EA`, which is **1.26:1** against a white card, and
/// even `Strong` (`#A8ADBD`) only reaches **2.24:1** — both under SC 1.4.11's 3:1, so a
/// near-white tag stayed invisible with the hairline dutifully drawn around it.
///
/// Deriving from the fill instead gives a *guarantee*. The ring is the fill's own
/// best-contrast extreme, so it is always the far end of the luminance scale from the fill.
/// Whenever the fill blends into the surface, the fill is near the surface in luminance —
/// which puts the ring at the far end from the *surface* too. The two therefore cover each
/// other: whichever one the surface swallows, the other stands out.
/// `outline_guarantees_a_visible_boundary` walks the colour cube against a light and a dark
/// surface and pins the worst case, which is 3.6:1.
///
/// This is also why the ring needs no theme awareness: it is a function of the tag colour
/// alone, so it cannot fall out of step when the theme changes under it.
pub fn outline_on(fill: Color) -> Color {
    fill.best_contrast_text()
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

    /// The claim [`outline_on`] rests on, checked rather than asserted in prose: for *every*
    /// fill, against a light and a dark surface, either the fill or its ring clears SC
    /// 1.4.11's 3:1 — so the dot always has a visible boundary.
    ///
    /// A themed border token cannot give this guarantee — `BorderRole::Default` is 1.26:1 on a
    /// white card, well under 3:1 — which is why the outline is derived instead.
    #[test]
    fn outline_guarantees_a_visible_boundary() {
        let surfaces = [
            Color::from_hex("#ffffff"), // light theme card
            Color::from_hex("#1e1e1e"), // dark theme card
        ];
        let mut worst = f32::MAX;
        for surface in surfaces {
            for r in (0..=255).step_by(5) {
                for g in (0..=255).step_by(5) {
                    for b in (0..=255).step_by(5) {
                        let fill = Color::from_hex(&format!("#{r:02x}{g:02x}{b:02x}"));
                        // The boundary reads if *either* edge of it does.
                        let best = fill
                            .contrast_ratio(surface)
                            .max(outline_on(fill).contrast_ratio(surface));
                        assert!(
                            best >= GRAPHICAL_OBJECT_MIN,
                            "#{r:02x}{g:02x}{b:02x} got {best:.2}:1 against {surface:?}, \
                             below SC 1.4.11"
                        );
                        worst = worst.min(best);
                    }
                }
            }
        }
        // Pinned so a future tweak that erodes the margin shows up as a failure rather than
        // as a dot nobody can see.
        assert!(worst > 3.5, "worst case fell to {worst:.2}:1");
    }

    /// The two fills that actually shipped broken, named so the regression is unmistakable.
    #[test]
    fn the_legacy_extremes_are_rescued_by_their_ring() {
        let white_card = Color::from_hex("#ffffff");
        let dark_card = Color::from_hex("#1e1e1e");

        // "A" from the v2.0.7 test project: 1.03:1 against a white card on its own.
        let near_white = Color::from_hex("#FFFAFA");
        assert!(near_white.contrast_ratio(white_card) < GRAPHICAL_OBJECT_MIN);
        assert!(outline_on(near_white).contrast_ratio(white_card) >= GRAPHICAL_OBJECT_MIN);

        // "very looooooooooong tag" from the same project, in the dark theme.
        let black = Color::from_hex("#000000");
        assert!(black.contrast_ratio(dark_card) < GRAPHICAL_OBJECT_MIN);
        assert!(outline_on(black).contrast_ratio(dark_card) >= GRAPHICAL_OBJECT_MIN);
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
