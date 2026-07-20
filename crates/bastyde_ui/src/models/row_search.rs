// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What counts as a **match** when a binder row is filtered by typed text.
//!
//! Two views filter the same rows with the same box: the Outline tree
//! ([`BinderBinderItemsTreeModel`](super::BinderBinderItemsTreeModel)) and the Overview
//! table ([`OverviewRowsModel`](super::OverviewRowsModel)). They used to each spell the
//! predicate inline, which is how "search finds it in the outline but not in the overview"
//! becomes possible without anyone editing either on purpose. It lives here so there is
//! exactly one answer.
//!
//! This is **not** the manuscript search (`search_management`'s `run_search`, which reads
//! prose, honours facets, case/diacritic/whole-word options and a corpus cache). This is
//! the cheap, live, structural filter over rows already in memory: the writer is looking
//! for a *row*, not an occurrence.

/// Fold a user-typed query into the form [`row_matches`] expects, or `None` when the
/// query is blank (= no filtering at all).
///
/// Lower-casing once at the top of a re-source, rather than per row, is not a
/// micro-optimisation: the source closure runs on every keystroke over the whole
/// container, so a per-row `to_lowercase` on the needle would allocate once per row per
/// keystroke.
pub(crate) fn needle(query: &str) -> Option<String> {
    let q = query.trim();
    (!q.is_empty()).then(|| q.to_lowercase())
}

/// Does a row whose searchable names are `fields` match `needle`?
///
/// `needle` must already be lower-cased — see [`needle`].
///
/// **Allocation-free.** The obvious spelling — `field.to_lowercase().contains(needle)` —
/// allocates a fresh `String` per field per row, and this runs inside the source closure
/// on *every keystroke*: an eight-character query over a 2000-row book would be tens of
/// thousands of transient allocations. Instead it walks each field once, comparing
/// lower-cased chars against the needle in a rolling fashion.
///
/// **Title and label only** (the two names a row shows). A row's prose is deliberately not
/// consulted: that is what the debounced, cached manuscript search is for. Nor is
/// anything the table does not display — a row kept by a filter for text the writer cannot
/// see anywhere reads as the filter being broken.
pub(crate) fn row_matches(needle: &str, fields: &[&str]) -> bool {
    fields.iter().any(|f| contains_fold(f, needle))
}

/// `haystack` contains `needle_lower` under simple (per-char) case folding, without
/// allocating.
///
/// Uses `char::to_lowercase`, which is the same Unicode simple-lowercase mapping
/// `str::to_lowercase` applies per character, so a match here and a match via the
/// allocating form agree — including on non-ASCII (é matches É, Ω matches ω).
///
/// The one Unicode case it deliberately does not handle is a fold that changes character
/// *count* (German ß → "ss", which `str::to_lowercase` leaves as ß anyway since it is
/// already lowercase). Searching "strasse" will not find "Straße" — but neither did the
/// allocating version, so this is not a regression, and full case-folding belongs in the
/// manuscript search, not a live row filter.
fn contains_fold(haystack: &str, needle_lower: &str) -> bool {
    if needle_lower.is_empty() {
        return true;
    }
    let hay: Vec<char> = haystack.chars().flat_map(char::to_lowercase).collect();
    let ndl: Vec<char> = needle_lower.chars().collect();
    if ndl.len() > hay.len() {
        return false;
    }
    hay.windows(ndl.len()).any(|w| w == ndl.as_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_blank_query_means_no_filtering() {
        assert_eq!(needle(""), None);
        assert_eq!(needle("   "), None, "whitespace is not a search");
        assert_eq!(needle("  Dawn "), Some("dawn".to_string()), "trimmed + folded");
    }

    #[test]
    fn matching_is_case_insensitive_across_every_field() {
        assert!(row_matches("dawn", &["Scene at Dawn", ""]));
        assert!(row_matches("beat", &["Scene 1", "opening BEAT"]));
        assert!(!row_matches("dusk", &["Scene at Dawn", "opening beat"]));
    }

    #[test]
    fn an_empty_field_list_never_matches() {
        assert!(!row_matches("dawn", &[]));
        assert!(!row_matches("dawn", &["", ""]));
    }

    /// A substring anywhere counts — the writer types a fragment they remember, not a
    /// prefix. (Pinned because "starts with" is the tempting cheaper rule.)
    #[test]
    fn a_fragment_matches_mid_word() {
        assert!(row_matches("onfront", &["Confrontation", ""]));
    }

    /// The allocation-free fold must agree with the allocating one it replaced,
    /// including on non-ASCII — the whole point of not hand-rolling ASCII-only folding.
    #[test]
    fn folding_agrees_with_the_allocating_form() {
        for (hay, ndl) in [
            ("Scène à l'Aube", "aube"),
            ("ÉLÉONORE", "éléonore"),
            ("Ωμέγα", "ωμέγα"),
            ("Confrontation", "ONFRONT"),
        ] {
            let ndl_lower = ndl.to_lowercase();
            assert_eq!(
                contains_fold(hay, &ndl_lower),
                hay.to_lowercase().contains(&ndl_lower),
                "{hay:?} vs {ndl:?}"
            );
        }
    }

    /// A needle longer than the field cannot match, and must not panic on the windowing.
    #[test]
    fn a_needle_longer_than_the_field_is_not_a_match() {
        assert!(!row_matches("a very long query", &["hi"]));
        assert!(!row_matches("x", &[""]));
    }
}
