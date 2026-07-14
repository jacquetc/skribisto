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
//! What stays here is the part that is genuinely about *replacing*: rewriting a string
//! and preserving the case it found. Both are string-level, and both move into
//! text-document with A2 (`find_and_replace`) — at which point this module goes away.

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

/// Rewrite `replacement` to carry the case of the text it is replacing.
///
/// Three classes, which is what a rename actually needs: ALL CAPS stays all caps,
/// Titlecase stays titlecase, anything else takes the replacement verbatim. Uppercasing
/// operates on the first **character**, never a byte slice — `É` is two bytes, and
/// slicing it would panic or silently mangle the very name we were asked to preserve.
pub fn preserve_case(matched: &str, replacement: &str) -> String {
    let letters: Vec<char> = matched.chars().filter(|c| c.is_alphabetic()).collect();
    if letters.is_empty() {
        return replacement.to_string();
    }
    let all_upper = letters.iter().all(|c| c.is_uppercase());
    if all_upper && letters.len() > 1 {
        return replacement.to_uppercase();
    }
    let title = letters[0].is_uppercase() && letters[1..].iter().all(|c| c.is_lowercase());
    if title {
        let mut cs = replacement.chars();
        return match cs.next() {
            Some(first) => first.to_uppercase().collect::<String>() + cs.as_str(),
            None => String::new(),
        };
    }
    replacement.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts(case_sensitive: bool, whole_word: bool) -> MatchOptions {
        MatchOptions {
            case_sensitive,
            whole_word,
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
            |m| preserve_case(m, "aurélian"),
        );
        assert_eq!(out, "Aurélian, AURÉLIAN and aurélian");
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
