//! Matching primitives shared by `run_search` and `replace_in_project`.
//!
//! The whole point of this module is one rule: **an offset computed in folded text is
//! not valid in the original text.** Case-folding can change a string's length —
//! `'İ'.to_lowercase()` is *two* chars — so a match position found in a lowercased
//! haystack lands in the wrong place, or off the end, when applied to the source.
//!
//! So folding here always produces an **index map** alongside the folded string, and
//! matches are always reported in ORIGINAL char offsets. This is the small, correct
//! version of the offset map the full matcher needs (it will grow to carry diacritic
//! folding and per-scene language); it exists now because the alternative silently
//! shifted every snippet in a Turkish scene, and panicked outright once enough `İ`
//! preceded the match.

/// A folded haystack plus the map back to where it came from.
pub struct Folded {
    /// The folded text — what we actually search in.
    pub text: String,
    /// `origin[i]` is the ORIGINAL char index that produced folded char `i`. There is
    /// one extra trailing entry (the original char count) so an *end* offset maps too.
    origin: Vec<usize>,
    /// Byte offset of each folded char, so a byte match can be turned into a folded
    /// char index without building a whole map per field.
    byte_of_char: Vec<usize>,
}

impl Folded {
    /// Fold `text` for matching. With `case_sensitive`, this is the identity (and the
    /// map is trivial); otherwise every char is lowercased, which may emit more than
    /// one char — each of which maps back to the single char it came from.
    pub fn new(text: &str, case_sensitive: bool) -> Self {
        let mut folded = String::with_capacity(text.len());
        let mut origin = Vec::with_capacity(text.len());
        let mut byte_of_char = Vec::with_capacity(text.len());

        for (i, c) in text.chars().enumerate() {
            if case_sensitive {
                byte_of_char.push(folded.len());
                folded.push(c);
                origin.push(i);
            } else {
                for lc in c.to_lowercase() {
                    byte_of_char.push(folded.len());
                    folded.push(lc);
                    origin.push(i);
                }
            }
        }
        // Sentinels, so a match that ends at the very end of the text still maps.
        byte_of_char.push(folded.len());
        origin.push(text.chars().count());

        Self {
            text: folded,
            origin,
            byte_of_char,
        }
    }

    /// Every occurrence of `needle` (already folded the same way), as
    /// `(original_char_start, original_char_len)` — ready to slice the ORIGINAL text.
    pub fn matches(&self, folded_needle: &str) -> Vec<(usize, usize)> {
        if folded_needle.is_empty() {
            return Vec::new();
        }
        let needle_chars = folded_needle.chars().count();
        self.text
            .match_indices(folded_needle)
            .filter_map(|(byte, _)| {
                // Byte → folded char index. Binary search over the per-char byte
                // offsets rather than a HashMap built for the whole haystack: the map
                // was being allocated for every field on every keystroke, including
                // the great majority that never match.
                let start = self.byte_of_char.binary_search(&byte).ok()?;
                let end = start + needle_chars;
                let from = *self.origin.get(start)?;
                let to = *self.origin.get(end)?;
                Some((from, to.saturating_sub(from)))
            })
            .collect()
    }
}

/// Fold a query the same way a haystack is folded, so the two can be compared.
pub fn fold_query(query: &str, case_sensitive: bool) -> String {
    if case_sensitive {
        query.to_string()
    } else {
        query.to_lowercase()
    }
}

/// Occurrences of `query` in `haystack`, in ORIGINAL char offsets.
pub fn occurrences(haystack: &str, query: &str, case_sensitive: bool) -> Vec<(usize, usize)> {
    if query.is_empty() {
        return Vec::new();
    }
    Folded::new(haystack, case_sensitive).matches(&fold_query(query, case_sensitive))
}

/// Replace every occurrence of `query`, rewriting the ORIGINAL text.
///
/// `case_of` decides what each individual occurrence is replaced with — it is handed
/// the matched text so a rename can preserve the case it found (`AURÉLIEN` → `AURÉLIAN`,
/// not `aurélian`).
pub fn replace_all(
    haystack: &str,
    query: &str,
    case_sensitive: bool,
    case_of: impl Fn(&str) -> String,
) -> String {
    let hits = occurrences(haystack, query, case_sensitive);
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
/// slicing it would panic or silently mangle the name we were asked to preserve.
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

    /// The bug this module exists for: `İ` lowercases to TWO chars, so an offset found
    /// in the lowercased haystack is wrong — or out of range — in the original.
    #[test]
    fn offsets_survive_a_case_fold_that_changes_length() {
        let text = "İİİİİİİİİİ ipsum";
        assert!(
            text.to_lowercase().chars().count() > text.chars().count(),
            "this text must actually grow when lowercased, or the test proves nothing"
        );

        let hits = occurrences(text, "ipsum", false);
        assert_eq!(hits.len(), 1);
        let (start, len) = hits[0];

        // The offset must be valid in the ORIGINAL — this is what used to panic.
        let chars: Vec<char> = text.chars().collect();
        let matched: String = chars[start..start + len].iter().collect();
        assert_eq!(
            matched, "ipsum",
            "the offset must point at the match, in the source"
        );
    }

    #[test]
    fn a_plain_case_insensitive_match_reports_source_offsets() {
        let hits = occurrences("She called Aurélien home", "aurélien", false);
        assert_eq!(hits, vec![(11, 8)]);
    }

    #[test]
    fn replace_preserves_the_case_it_found() {
        let out = replace_all(
            "Aurélien, AURÉLIEN and aurélien",
            "aurélien",
            false,
            |m| preserve_case(m, "aurélian"),
        );
        assert_eq!(out, "Aurélian, AURÉLIAN and aurélian");
    }

    #[test]
    fn replace_rewrites_correctly_even_when_folding_changes_length() {
        let out = replace_all("İİ ipsum İİ ipsum", "ipsum", false, |_| "LOREM".into());
        assert_eq!(out, "İİ LOREM İİ LOREM");
    }

    #[test]
    fn a_case_sensitive_search_does_not_match_the_other_case() {
        assert!(occurrences("Ipsum", "ipsum", true).is_empty());
        assert_eq!(occurrences("Ipsum", "Ipsum", true).len(), 1);
    }

    #[test]
    fn an_empty_query_matches_nothing() {
        assert!(occurrences("anything", "", false).is_empty());
        assert_eq!(
            replace_all("anything", "", false, |_| "x".into()),
            "anything"
        );
    }
}
