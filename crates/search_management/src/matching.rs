// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Replace-side helpers, over the **shared** matcher.
//!
//! Finding is *not* done here. It is done by `text_document::matching`, the one
//! definition of "a match" — the same one the editor's own find and find-and-replace
//! use. This crate used to carry its own copy, and two matchers drift: the writer meets
//! that as "the editor found it but the search panel didn't", or worse, as a whole-word
//! Replace All that renames a character everywhere except in the possessives.
//!
//! It also inherits the rule that copy existed to enforce, for free: **an offset computed
//! in folded text is not valid in the original text** (`'İ'.to_lowercase()` is two chars,
//! so a match found in a lowercased haystack lands in the wrong place in the source).
//!
//! What stays here is the one thing text-document cannot do for us: rewrite a **plain
//! string**. A title and a label are not documents — there is no parser, no format run and
//! no `BatchDocument` to splice inside — so they get a string rewrite. Prose does not: it
//! goes through `BatchDocument::find_and_replace`, which splices inside the parsed document
//! at the offsets the parser itself reports.
//!
//! `preserve_case` used to live here too. It now comes from `text_document::matching`,
//! because it needs the scene's **locale**: in Turkish the uppercase of `i` is `İ`, and a
//! case-preserver blind to that would rewrite Turkish prose into a different word. That is
//! the same class of knowledge as the fold, so it lives with the fold.

use text_document::matching::{MatchOptions, find_all};

/// Occurrences of `query` in `haystack`, as `(char_start, char_len)` into the ORIGINAL.
///
/// A thin adapter over the shared matcher, kept only so the call sites read in the terms
/// this crate thinks in.
pub fn occurrences(haystack: &str, query: &str, options: MatchOptions) -> Vec<(usize, usize)> {
    find_all(haystack, query, &options)
        .into_iter()
        .map(|m| (m.char_start, m.char_len))
        .collect()
}

/// Replace every occurrence of `query`, rewriting the ORIGINAL text.
///
/// `case_of` decides what each individual occurrence becomes — it is handed the matched
/// text, so a rename can preserve the case it found (`AURÉLIEN` → `AURÉLIAN`, not
/// `aurélian`).
pub fn replace_all(
    haystack: &str,
    query: &str,
    options: MatchOptions,
    case_of: impl Fn(&str) -> String,
) -> String {
    let hits = occurrences(haystack, query, options);
    if hits.is_empty() {
        return haystack.to_string();
    }
    let chars: Vec<char> = haystack.chars().collect();
    let mut out = String::with_capacity(haystack.len());
    let mut cursor = 0usize;
    for (start, len) in hits {
        if start < cursor {
            continue; // overlapping match; the earlier one won
        }
        out.extend(&chars[cursor..start]);
        let matched: String = chars[start..(start + len).min(chars.len())]
            .iter()
            .collect();
        out.push_str(&case_of(&matched));
        cursor = start + len;
    }
    out.extend(&chars[cursor.min(chars.len())..]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use text_document::matching::{FoldLocale, preserve_case};

    fn opts(case_sensitive: bool, whole_word: bool) -> MatchOptions {
        MatchOptions {
            case_sensitive,
            whole_word,
            ..MatchOptions::default()
        }
    }

    /// The property this crate no longer has to implement itself, but still depends on:
    /// an offset from folded text must be valid in the source.
    #[test]
    fn the_shared_matcher_reports_offsets_in_the_source() {
        let text = "İİİİİİİİİİ ipsum";
        assert!(text.to_lowercase().chars().count() > text.chars().count());

        let hits = occurrences(text, "ipsum", opts(false, false));
        assert_eq!(hits.len(), 1);
        let (start, len) = hits[0];
        let chars: Vec<char> = text.chars().collect();
        assert_eq!(
            chars[start..start + len].iter().collect::<String>(),
            "ipsum"
        );
    }

    /// And the one this crate now gets for free: whole-word finds the possessive, so a
    /// Replace All cannot leave a manuscript half-renamed.
    #[test]
    fn whole_word_reaches_the_possessive() {
        let out = replace_all(
            "Elena went home. Elena's coat stayed.",
            "Elena",
            opts(false, true),
            |_| "Marta".to_string(),
        );
        assert_eq!(out, "Marta went home. Marta's coat stayed.");
    }

    #[test]
    fn replace_preserves_the_case_it_found() {
        let out = replace_all(
            "Aurélien, AURÉLIEN and aurélien",
            "aurélien",
            opts(false, false),
            |m| preserve_case(m, "aurélian", FoldLocale::Root),
        );
        assert_eq!(out, "Aurélian, AURÉLIAN and aurélian");
    }

    /// A title in a Turkish scene. The case-preserver is the shared, **locale-aware** one:
    /// the untailored uppercase of `i` is `I`, which in Turkish is the capital of a
    /// different letter — so an unaware rename would write `ILK` where the prose needs
    /// `İLK`, silently turning the word into another word.
    #[test]
    fn replace_preserves_turkish_case_correctly() {
        let turkish = MatchOptions {
            locale: FoldLocale::Turkic,
            ..MatchOptions::default()
        };
        let out = replace_all("KISA yol", "kısa", turkish, |m| {
            preserve_case(m, "ilk", FoldLocale::Turkic)
        });
        assert_eq!(out, "İLK yol");
    }

    /// A plain ASCII query finds accented prose — the fold reaches the plain-string fields
    /// (a title, a label) too, not just the parsed prose.
    #[test]
    fn a_plain_query_folds_onto_an_accented_title() {
        let hits = occurrences("La forêt d'Aurélien", "aurelien", opts(false, false));
        assert_eq!(hits.len(), 1);
        let out = replace_all(
            "La forêt d'Aurélien",
            "aurelien",
            opts(false, false),
            |m| preserve_case(m, "aurélian", FoldLocale::Root),
        );
        assert_eq!(out, "La forêt d'Aurélian");
    }

    #[test]
    fn replace_rewrites_correctly_even_when_folding_changes_length() {
        let out = replace_all("İİ ipsum İİ ipsum", "ipsum", opts(false, false), |_| {
            "LOREM".into()
        });
        assert_eq!(out, "İİ LOREM İİ LOREM");
    }

    #[test]
    fn a_case_sensitive_search_does_not_match_the_other_case() {
        assert!(occurrences("Ipsum", "ipsum", opts(true, false)).is_empty());
        assert_eq!(occurrences("Ipsum", "Ipsum", opts(true, false)).len(), 1);
    }

    #[test]
    fn an_empty_query_matches_nothing() {
        assert!(occurrences("anything", "", opts(false, false)).is_empty());
        assert_eq!(
            replace_all("anything", "", opts(false, false), |_| "x".into()),
            "anything"
        );
    }
}
