// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Locale-aware case conversion — the Turkish/Azeri dotted-I tailoring.
//!
//! Rust's `str::to_lowercase` / `str::to_uppercase` implement the **default**
//! Unicode case mappings and are, by design, permanently locale-independent
//! (`std` says so outright). For nearly every language that is right. For
//! Turkish and Azeri it is wrong, because those languages treat the dotted and
//! dotless I as *different letters*:
//!
//! | | lowercase | uppercase |
//! |---|---|---|
//! | dotless | `ı` U+0131 | `I` U+0049 |
//! | dotted | `i` U+0069 | `İ` U+0130 |
//!
//! Under the default mappings `i` uppercases to `I` (the *dotless* capital, a
//! different letter in Turkish) and `I` lowercases to `i` (the *dotted* small
//! letter, likewise). Worse for us, default lowercasing of `İ` produces **two**
//! code points — `i` followed by U+0307 COMBINING DOT ABOVE — so a
//! case-insensitive comparison of `İ` against `i` fails even though a Turkish
//! reader considers them the same letter.
//!
//! ## Why this is hand-written rather than a crate
//!
//! What is needed here is not general Unicode casing — `std` already does that
//! correctly — but one small, fully-specified *tailoring* on top of it: the four
//! mappings above, plus dropping the combining dot. That delta is what Unicode's
//! `SpecialCasing.txt` lists under the `tr`/`az` conditions, and it is short
//! enough to state exactly and test exhaustively. Pulling in `icu_casemap` for
//! four characters would add a data pipeline far larger than the problem.
//!
//! ## What this deliberately does NOT implement
//!
//! The context-sensitive half of the `tr`/`az` rules — "lowercase `İ` to `i`
//! *and* remove a following combining dot above only when one intervenes" —
//! matters for decomposed text (`I` + U+0307). Prose in this app arrives from a
//! keyboard or an import, both of which produce the composed form, and the
//! conservative behaviour for the decomposed form (leave the stray combining
//! mark alone) is the same thing the default mapping does. It is called out
//! here so a future reader knows it was considered rather than missed.

/// Whether `tag` is a BCP-47 tag whose language subtag is Turkish or Azeri.
///
/// Matches on the primary subtag only, so `tr`, `tr-TR`, `az`, `az-Latn-AZ` all
/// qualify while `trv` (Taroko) does not — a prefix test would wrongly catch it.
pub fn uses_dotted_i(tag: &str) -> bool {
    let primary = tag
        .split(['-', '_'])
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    primary == "tr" || primary == "az"
}

/// Lowercase `s` for `tag`'s locale.
pub fn to_lowercase(s: &str, tag: &str) -> String {
    if uses_dotted_i(tag) {
        lowercase_tr(s)
    } else {
        s.to_lowercase()
    }
}

/// Uppercase `s` for `tag`'s locale.
pub fn to_uppercase(s: &str, tag: &str) -> String {
    if uses_dotted_i(tag) {
        uppercase_tr(s)
    } else {
        s.to_uppercase()
    }
}

/// Uppercase only the first character, leaving the rest untouched.
///
/// Goes through the locale-aware uppercase rather than an ASCII shortcut, so a
/// Turkish `i` titlecases to `İ` and a German `ß` still expands to `SS`.
pub fn capitalize_first(s: &str, tag: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => to_uppercase(&first.to_string(), tag) + chars.as_str(),
    }
}

/// The key two strings are compared under for a case-insensitive match.
///
/// This is what makes `İSTANBUL` and `istanbul` compare equal under `tr`, which
/// plain `to_lowercase` does not: the default mapping leaves a combining dot
/// behind on the first and not the second.
pub fn fold_key(s: &str, tag: &str) -> String {
    to_lowercase(s, tag)
}

/// Turkish/Azeri lowercase: dotted capital → plain `i`, dotless capital → `ı`.
fn lowercase_tr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            // U+0130. The default mapping yields "i\u{307}"; Turkish wants a
            // bare `i`, and the stray combining dot is exactly what would break
            // an equality comparison against typed text.
            'İ' => out.push('i'),
            'I' => out.push('ı'),
            other => out.extend(other.to_lowercase()),
        }
    }
    out
}

/// Turkish/Azeri uppercase: `i` → dotted capital, dotless `ı` → plain `I`.
fn uppercase_tr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            'i' => out.push('İ'),
            'ı' => out.push('I'),
            other => out.extend(other.to_uppercase()),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Which locales get the tailoring ──────────────────────────────────────

    #[test]
    fn turkish_and_azeri_are_tailored_by_language_subtag() {
        for tag in ["tr", "tr-TR", "TR", "az", "az-AZ", "az-Latn-AZ", "tr_TR"] {
            assert!(uses_dotted_i(tag), "{tag} must be tailored");
        }
    }

    /// A prefix test would wrongly catch these — the match is on the language
    /// subtag, not on the string starting with "tr".
    #[test]
    fn look_alike_tags_are_not_tailored() {
        for tag in ["trv", "tru", "azb-x", "en-US", "fr-FR", "", "  "] {
            assert!(!uses_dotted_i(tag), "{tag} must NOT be tailored");
        }
    }

    // ── The four mappings ────────────────────────────────────────────────────

    #[test]
    fn turkish_uppercase_keeps_the_dot_on_i() {
        assert_eq!(to_uppercase("istanbul", "tr-TR"), "İSTANBUL");
        assert_eq!(to_uppercase("ırmak", "tr-TR"), "IRMAK");
    }

    #[test]
    fn turkish_lowercase_keeps_the_two_letters_apart() {
        assert_eq!(to_lowercase("İSTANBUL", "tr-TR"), "istanbul");
        assert_eq!(to_lowercase("IRMAK", "tr-TR"), "ırmak");
    }

    /// The bug that motivates the whole module: under the DEFAULT mapping the
    /// dotted capital lowercases to two code points, so it does not compare
    /// equal to a typed `i`. A lexicon trigger would silently never match.
    #[test]
    fn the_default_mapping_leaves_a_combining_dot_that_breaks_comparison() {
        let default_folded = "İ".to_lowercase();
        assert_eq!(
            default_folded.chars().count(),
            2,
            "std lowercases U+0130 to i + U+0307 — this is the hazard"
        );
        assert_ne!(default_folded, "i", "…so a default fold does not match `i`");

        assert_eq!(fold_key("İ", "tr"), "i", "the Turkish fold does match");
        assert_eq!(fold_key("İSTANBUL", "tr"), fold_key("istanbul", "tr"));
    }

    /// Everywhere else the default mapping must be left exactly alone — porting
    /// the Turkish rule by accident would be its own bug.
    #[test]
    fn non_turkish_locales_keep_the_default_mapping() {
        assert_eq!(to_uppercase("istanbul", "en-US"), "ISTANBUL");
        assert_eq!(to_lowercase("ISTANBUL", "en-US"), "istanbul");
        assert_eq!(to_uppercase("i", "fr-FR"), "I");
    }

    // ── Round trips and the rest of Unicode ──────────────────────────────────

    #[test]
    fn turkish_case_round_trips_through_both_letters() {
        for word in ["ıslık", "iyi", "İki", "Irmak"] {
            let up = to_uppercase(word, "tr");
            let down = to_lowercase(&up, "tr");
            assert_eq!(
                down,
                to_lowercase(word, "tr"),
                "{word} must survive up-then-down"
            );
        }
    }

    /// The tailoring is only about I/i; every other character still goes through
    /// the full Unicode mapping, including one-to-many expansions.
    #[test]
    fn other_characters_still_use_full_unicode_mappings() {
        assert_eq!(to_uppercase("straße", "tr"), "STRASSE");
        assert_eq!(to_uppercase("çğöşü", "tr"), "ÇĞÖŞÜ");
        assert_eq!(to_lowercase("ÇĞÖŞÜ", "tr"), "çğöşü");
    }

    /// The two halves compose: an accented letter takes the full Unicode
    /// mapping *and* an `i` in the same word takes the Turkish one. Spelled out
    /// against both locales because it is genuinely surprising — the same word
    /// uppercases differently, and that is correct rather than a bug.
    #[test]
    fn the_tailoring_composes_with_the_default_mapping_in_one_word() {
        assert_eq!(to_uppercase("étoile", "fr-FR"), "ÉTOILE");
        assert_eq!(
            to_uppercase("étoile", "tr"),
            "ÉTOİLE",
            "é still maps by default, while i takes the dotted capital"
        );
    }

    #[test]
    fn capitalize_first_is_locale_aware_and_leaves_the_tail_alone() {
        assert_eq!(capitalize_first("istanbul", "tr"), "İstanbul");
        assert_eq!(capitalize_first("istanbul", "en-US"), "Istanbul");
        assert_eq!(capitalize_first("ırmak", "tr"), "Irmak");
        // Not an ASCII shortcut: one character can uppercase into two.
        assert_eq!(capitalize_first("ßeta", "en-US"), "SSeta");
        assert_eq!(capitalize_first("étoile", "fr-FR"), "Étoile");
        assert_eq!(capitalize_first("", "tr"), "");
    }

    #[test]
    fn a_caseless_script_is_unchanged_in_every_locale() {
        for tag in ["tr", "en-US", "ar"] {
            assert_eq!(to_uppercase("日本語", tag), "日本語");
            assert_eq!(to_lowercase("العربية", tag), "العربية");
        }
    }
}
