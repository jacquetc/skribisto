// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Counting **policy** + a content-addressed cache over Djot scene prose.
//!
//! `text-document` owns the pure primitive ([`count_djot`], language-agnostic — the method
//! is a parameter). Skribisto owns the *policy*: which method the writer chose (a global
//! setting), resolved per scene from that scene's effective language.
//!
//! The cache mirrors [`search_management::corpus_cache`] exactly — keyed on
//! `(Djot source, resolved method)`, **content-addressed** so there is nothing to
//! invalidate (edited prose is a different string → a different key → a miss), bounded by
//! total heap and cleared wholesale on overflow. The one difference: a count is a tiny
//! `Copy` value, so it is returned by value, not behind an `Arc`.

use std::collections::HashMap;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};
use text_document::count_djot;
pub use text_document::{CountMethod, WordCharCounts, count};

/// The writer's counting-method preference — a global user setting (like typography). `Auto`
/// resolves per scene from its effective language; the others force one method everywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CountingMethodSetting {
    /// Per-scene: CJK-hybrid for Chinese/Japanese prose, else the non-CJK fallback.
    #[default]
    Auto,
    Whitespace,
    UnicodeWords,
    CjkHybrid,
}

/// The concrete [`CountMethod`] for a scene, given the setting, the non-CJK fallback that
/// `Auto` uses for non-CJK prose, and the scene's **primary** language subtag (BCP-47
/// primary, e.g. `zh`, `ja`, `fr` — see [`crate::language::primary`]).
pub fn resolve_method(
    setting: CountingMethodSetting,
    non_cjk_fallback: CountMethod,
    primary_language: &str,
) -> CountMethod {
    match setting {
        CountingMethodSetting::Auto => {
            if is_cjk_language(primary_language) {
                CountMethod::CjkHybrid
            } else {
                non_cjk_fallback
            }
        }
        CountingMethodSetting::Whitespace => CountMethod::WhitespaceSplit,
        CountingMethodSetting::UnicodeWords => CountMethod::UnicodeWords,
        CountingMethodSetting::CjkHybrid => CountMethod::CjkHybrid,
    }
}

/// Chinese / Japanese are not space-delimited → per-character counting. Korean (`ko`) is,
/// so it stays on the word rule.
fn is_cjk_language(primary: &str) -> bool {
    matches!(primary, "zh" | "ja")
}

/// How much heap the cache may hold before it is cleared wholesale. A count is 24 bytes; the
/// weight is the Djot source strings held as keys (~1.7 MB for a 300k-word novel), so this
/// holds one comfortably plus a long session's edits.
const MAX_HEAP: usize = 64 * 1024 * 1024;

/// Process-global rather than per-Work: content-addressed on `(Djot source, method)`, so a
/// single cache stays correct even with several projects open in one process at once —
/// identical prose and method always count identically, whichever Work it came from.
static CACHE: RwLock<Option<Store>> = RwLock::new(None);

/// The cache proper, with no global in it — so the tests exercise it deterministically.
///
/// Nested by `CountMethod` (a handful) so the inner map is keyed by `String` alone and can
/// be probed with a plain `&str` — no allocation on a hit.
struct Store {
    by_method: HashMap<CountMethod, HashMap<String, WordCharCounts>>,
    heap: usize,
    max_heap: usize,
}

impl Default for Store {
    fn default() -> Self {
        Store { by_method: HashMap::new(), heap: 0, max_heap: MAX_HEAP }
    }
}

impl Store {
    fn get(&self, djot: &str, method: CountMethod) -> Option<WordCharCounts> {
        self.by_method.get(&method)?.get(djot).copied()
    }

    fn insert(&mut self, djot: &str, method: CountMethod, counts: WordCharCounts) {
        let size = djot.len() + std::mem::size_of::<WordCharCounts>();
        // An entry that alone exceeds the budget is served but not cached (caching it would
        // leave the store over budget, so the next insert would clear again, forever).
        if size > self.max_heap {
            return;
        }
        // Cleared wholesale on overflow: the working set is "the manuscript open right now".
        if self.heap + size > self.max_heap {
            self.by_method.clear();
            self.heap = 0;
        }
        if self
            .by_method
            .entry(method)
            .or_default()
            .insert(djot.to_string(), counts)
            .is_none()
        {
            self.heap += size;
        }
    }
}

/// The uncached count of one scene's Djot prose.
///
/// Scene-break markers are stripped first: a break is typographic furniture the
/// author placed, not three words they wrote. This is the single funnel every
/// raw-Djot counter goes through — the persisted pace/progress history and the
/// corkboard cards both land here — so they cannot disagree about it.
fn count_prose(djot: &str, method: CountMethod) -> WordCharCounts {
    count_djot(&crate::scene_break::strip_markers_djot(djot), method)
}

/// The count of one scene's Djot prose under `method` — from the cache if present, computed
/// and cached if not.
pub fn cached_count(djot: &str, method: CountMethod) -> WordCharCounts {
    if let Ok(guard) = CACHE.read()
        && let Some(hit) = guard.as_ref().and_then(|s| s.get(djot, method))
    {
        return hit;
    }

    // Built OUTSIDE the write lock — the parse dwarfs everything else; holding the lock
    // across it would serialise every scene behind one of them.
    let counts = count_prose(djot, method);

    if let Ok(mut guard) = CACHE.write() {
        guard.get_or_insert_with(Store::default).insert(djot, method, counts);
    }
    counts
}

/// Drop everything. Not for correctness — the cache is content-addressed and cannot go
/// stale — but frees the heap a closed project's entries were holding.
pub fn clear() {
    if let Ok(mut guard) = CACHE.write() {
        *guard = None;
    }
}

/// Bytes of heap held. For tests and diagnostics.
pub fn heap_size() -> usize {
    CACHE
        .read()
        .ok()
        .and_then(|g| g.as_ref().map(|s| s.heap))
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() -> Store {
        Store::default()
    }

    fn get(store: &mut Store, djot: &str, method: CountMethod) -> WordCharCounts {
        if let Some(hit) = store.get(djot, method) {
            return hit;
        }
        let counts = count_djot(djot, method);
        store.insert(djot, method, counts);
        counts
    }

    fn entries(store: &Store) -> usize {
        store.by_method.values().map(|m| m.len()).sum()
    }

    #[test]
    fn auto_picks_cjk_for_chinese_and_japanese_else_fallback() {
        let f = CountMethod::UnicodeWords;
        assert_eq!(resolve_method(CountingMethodSetting::Auto, f, "zh"), CountMethod::CjkHybrid);
        assert_eq!(resolve_method(CountingMethodSetting::Auto, f, "ja"), CountMethod::CjkHybrid);
        assert_eq!(resolve_method(CountingMethodSetting::Auto, f, "fr"), CountMethod::UnicodeWords);
        assert_eq!(resolve_method(CountingMethodSetting::Auto, f, "ko"), CountMethod::UnicodeWords);
    }

    #[test]
    fn explicit_settings_ignore_language() {
        let f = CountMethod::UnicodeWords;
        assert_eq!(
            resolve_method(CountingMethodSetting::Whitespace, f, "zh"),
            CountMethod::WhitespaceSplit
        );
        assert_eq!(
            resolve_method(CountingMethodSetting::CjkHybrid, f, "fr"),
            CountMethod::CjkHybrid
        );
    }

    #[test]
    fn the_same_source_is_counted_once() {
        let mut store = fresh();
        let djot = "Aurélien traversa la *forêt* qui portait l'odeur du sel.";
        let a = get(&mut store, djot, CountMethod::UnicodeWords);
        let b = get(&mut store, djot, CountMethod::UnicodeWords);
        assert_eq!(a, b);
        assert_eq!(entries(&store), 1, "a second lookup must not re-count");
    }

    #[test]
    fn edited_prose_is_a_different_entry() {
        let mut store = fresh();
        let before = get(&mut store, "Elena rentra chez elle.", CountMethod::UnicodeWords);
        let after = get(&mut store, "Elena rentra chez elle, enfin.", CountMethod::UnicodeWords);
        assert_eq!(before.words, 4);
        assert_eq!(after.words, 5);
        assert_eq!(entries(&store), 2);
    }

    #[test]
    fn different_methods_are_different_entries() {
        let mut store = fresh();
        // Punctuation-only tokens diverge between whitespace and unicode-words.
        let djot = "-- deux mots --";
        let ws = get(&mut store, djot, CountMethod::WhitespaceSplit);
        let uw = get(&mut store, djot, CountMethod::UnicodeWords);
        assert_eq!(ws.words, 4); // "--", "deux", "mots", "--"
        assert_eq!(uw.words, 2); // "deux", "mots"
        assert_eq!(entries(&store), 2);
    }

    #[test]
    fn the_count_is_of_prose_not_markup() {
        let mut store = fresh();
        // A link URL is markup, not prose the writer typed into the sentence.
        let c = get(&mut store, "Voir [la note](https://exemple.test/x) ici.", CountMethod::UnicodeWords);
        assert_eq!(c.words, 4, "Voir la note ici");
    }

    #[test]
    fn overflowing_the_budget_clears_rather_than_grows() {
        let mut store = fresh();
        let big = "la première scène, longue et détaillée, tient dans le budget";
        get(&mut store, big, CountMethod::UnicodeWords);
        // Budget big enough for one entry, but not two — and the *second* entry is smaller
        // than the first, so it can't trip the "too big to cache" path: the only way it
        // lands at count 1 is the wholesale clear.
        store.max_heap = store.heap + 8;
        assert_eq!(entries(&store), 1);
        get(&mut store, "court", CountMethod::UnicodeWords);
        assert_eq!(entries(&store), 1, "overflow cleared the old entry, kept the new");
        assert!(store.get(big, CountMethod::UnicodeWords).is_none(), "the old entry was cleared");
        assert!(
            store.get("court", CountMethod::UnicodeWords).is_some(),
            "the new entry survived the clear"
        );
        assert!(store.heap <= store.max_heap, "the accounting was reset");
    }

    #[test]
    fn the_global_cache_works_and_can_be_cleared() {
        let djot = "une phrase que seule cette épreuve met en cache";
        let a = cached_count(djot, CountMethod::UnicodeWords);
        let b = cached_count(djot, CountMethod::UnicodeWords);
        assert_eq!(a, b);
        assert!(heap_size() > 0);
        clear();
        assert_eq!(heap_size(), 0);
    }

    #[test]
    fn scene_break_markers_are_not_counted_as_words() {
        // A break is typographic furniture, not prose. `count_prose` is the
        // single funnel the pace/progress history and the corkboard cards both
        // go through, so pinning it here pins every raw-Djot counter at once.
        // Tested off the global cache so it cannot race the cache's own tests.
        let plain = "She closed the door.\n\nDawn found him waiting.";
        let marked = "She closed the door.\n\n\\* \\* \\*\n\nDawn found him waiting.";
        assert_eq!(
            count_prose(marked, CountMethod::UnicodeWords).words,
            count_prose(plain, CountMethod::UnicodeWords).words,
            "a scene break must not add words"
        );
    }

    #[test]
    fn a_major_marker_is_not_counted_either() {
        let plain = "One two three.";
        let marked = "One two three.\n\n\\# # #";
        assert_eq!(
            count_prose(marked, CountMethod::UnicodeWords).words,
            count_prose(plain, CountMethod::UnicodeWords).words
        );
    }

    #[test]
    fn prose_containing_an_asterisk_still_counts_normally() {
        // The stripper must not eat emphasis or a footnote mark.
        let escaped = "He was \\*emphatic\\* about it.";
        assert_eq!(
            count_prose(escaped, CountMethod::UnicodeWords).words,
            count_prose("He was emphatic about it.", CountMethod::UnicodeWords).words
        );
    }
}
