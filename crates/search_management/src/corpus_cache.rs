// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The prose of every scene, parsed and folded — kept between keystrokes.
//!
//! `run_search` runs on **every keystroke** and redoes the same work on the same unchanged
//! prose each time, synchronously on the UI thread. Measured over a 300k-word manuscript
//! (4000 scenes), in release: parsing Djot to prose (`djot_to_plain_text`) is ~33 ms, folding
//! ~14 ms, scanning ~16 ms — the first two never change between keystrokes, so caching them
//! turns a typing stall into a scan.
//!
//! ## Content-addressed, so there is nothing to invalidate
//!
//! The cache keys on the **prose itself** — `(Djot source, fold rules)` — not the entity id.
//! An edit to a scene is a different string, hence a different key, hence a miss: no
//! invalidation logic, no events to subscribe to (update/remove/close/language-change), and
//! no way to serve stale prose. It also makes the cache safe to share across `AppContext`s
//! (the test suite creates many in one process): two stores holding the same prose get the
//! same answer, where an id-keyed cache would need per-store scoping.
//!
//! The key is the source `String`, not a hash of it: a 64-bit hash collision would silently
//! serve one scene's prose as another's, in a writer's manuscript. `HashMap` compares the
//! keys it stores, so there is no such window — the cost is holding the Djot a second time
//! (~1.7 MB for that 300k-word novel).
//!
//! ## Bounded
//!
//! Every edit mints a new key, so a session accumulates entries for prose that no longer
//! exists. The cache is bounded by total heap and **cleared wholesale** on overflow rather
//! than evicted one entry at a time: the working set is "the manuscript open right now", not
//! a recency distribution, so the cost of being wrong is one cold search.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use text_document::matching::{FoldSpec, FoldedText};
use text_document::{DjotImportOptions, djot_to_plain_text};

/// One scene's prose, parsed and folded. `folded.source()` is the prose itself — what a
/// snippet is cut from, and what the match offsets address.
pub type Corpus = FoldedText;

/// How much heap the cache may hold before it is cleared. A 300k-word novel folds to roughly
/// 20 MB, so this holds one comfortably, plus a long session's worth of edits.
const MAX_HEAP: usize = 128 * 1024 * 1024;

/// The cache. Process-global: one process can hold several open `Work`s at once (the
/// single-instance primary), but being content-addressed the cache stays correct regardless
/// — two Works with identical prose simply share an entry.
static CACHE: RwLock<Option<Store>> = RwLock::new(None);

/// The cache proper, with no global in it, so the tests below can exercise it
/// deterministically — Rust runs tests in parallel threads of one process, and a global
/// `clear()` in one test would race an `Arc::ptr_eq` in another.
///
/// Nested by `FoldSpec` rather than one map keyed by `(source, spec)`: `String: Borrow<str>`
/// lets the inner map be probed with a plain `&str`, so a hit costs no allocation. A flat
/// `(String, _)` key would force materialising the ~1.7 MB Djot string on every lookup, hit
/// or miss — on the exact hot path this cache exists to make fast.
struct Store {
    by_spec: HashMap<FoldSpec, HashMap<String, Arc<Corpus>>>,
    heap: usize,
    /// The budget. A field rather than a `const` read straight from `insert`, so the tests can
    /// exercise the two overflow paths with a small one instead of faking `heap` and hoping
    /// the arithmetic still means what it meant.
    max_heap: usize,
}

impl Default for Store {
    fn default() -> Self {
        Store {
            by_spec: HashMap::new(),
            heap: 0,
            max_heap: MAX_HEAP,
        }
    }
}

impl Store {
    /// Borrowed probe: no allocation on a hit.
    fn get(&self, djot: &str, spec: &FoldSpec) -> Option<Arc<Corpus>> {
        self.by_spec.get(spec)?.get(djot).map(Arc::clone)
    }

    fn insert(&mut self, djot: &str, spec: &FoldSpec, corpus: &Arc<Corpus>) {
        // `heap_size()` is only stable once the word-boundary table exists — it is built
        // lazily, on the first whole-word query, which for a cached entry is always *after*
        // it was measured. Force it, so the size recorded here is the size actually held; a
        // cache bounded by the sum of stale sizes holds materially more than it believes.
        corpus.prepare_word_boundaries();
        let size = corpus.heap_size() + djot.len();

        // An entry that alone exceeds the budget is served but NOT cached. Clearing to make
        // room for it would leave the cache over budget anyway, and the next insert would
        // clear again — turning every single lookup into a cold one, for ever.
        if size > self.max_heap {
            return;
        }
        // Cleared **wholesale** on overflow rather than evicted one entry at a time: the
        // working set is "the manuscript open right now", not a recency distribution, and the
        // cost of being wrong is one cold search.
        if self.heap + size > self.max_heap {
            self.by_spec.clear();
            self.heap = 0;
        }
        if self
            .by_spec
            .entry(*spec)
            .or_default()
            .insert(djot.to_string(), Arc::clone(corpus))
            .is_none()
        {
            self.heap += size;
        }
    }

    /// The expensive part: parse the Djot into prose, then fold the prose. Deliberately a
    /// free function taking no `&self` — it must never be called with the lock held (see
    /// [`corpus_for`]).
    fn build(djot: &str, spec: &FoldSpec) -> Arc<Corpus> {
        // Scene-break markers are deliberately NOT stripped here, unlike in
        // `skribisto_model::counting`: this corpus must be exactly what the document
        // searches, so a match found here maps to a replace performed there. Stripping
        // markers would shift every later offset and corrupt replacements.
        let prose = djot_to_plain_text(djot, &DjotImportOptions::default());
        Arc::new(FoldedText::new(&prose, spec))
    }
}

/// The parsed, folded prose of one `Content.data` — from the cache if it is there, built and
/// cached if not.
///
/// Returns an `Arc` so the caller can hold it across the scan without keeping the lock.
pub fn corpus_for(djot: &str, spec: &FoldSpec) -> Arc<Corpus> {
    if let Ok(guard) = CACHE.read()
        && let Some(hit) = guard.as_ref().and_then(|s| s.get(djot, spec))
    {
        return hit;
    }

    // Built OUTSIDE the write lock. The parse and the fold are the whole cost — holding the
    // lock across them would serialise every scene of the manuscript behind one of them,
    // which is precisely the stall this exists to remove.
    let corpus = Store::build(djot, spec);

    if let Ok(mut guard) = CACHE.write() {
        // Another thread may have inserted the same key while we were folding. Its entry and
        // ours are equal by construction — same source, same rules — so either will do.
        guard
            .get_or_insert_with(Store::default)
            .insert(djot, spec, &corpus);
    }

    corpus
}

/// Drop everything.
///
/// Called when a project closes — **not** for correctness (the cache is content-addressed and
/// cannot go stale) but because the app replaces the open project *in the same process* on
/// four paths (New Work, Open Work, the switcher's "Open here", the import toast's "Open
/// now"). Without this, the previous manuscript's corpus stays resident and permanently
/// unreachable: its keys are prose no longer in the store, so nothing will ever hit them
/// again, and only the overflow-clear would eventually free them.
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
    use text_document::matching::{FoldLocale, MatchOptions};

    fn spec() -> FoldSpec {
        FoldSpec::default()
    }

    /// A cache of one's own. The real one is process-global (Skribisto is one process per
    /// project, so a global *is* project-scoped) — but Rust runs tests in parallel threads of
    /// ONE process, so a test that cleared the global would race a test mid-lookup.
    fn fresh() -> Store {
        Store::default()
    }

    /// A lookup, done the way `corpus_for` does it: hit, or build-and-insert.
    fn get(store: &mut Store, djot: &str, spec: &FoldSpec) -> Arc<Corpus> {
        if let Some(hit) = store.get(djot, spec) {
            return hit;
        }
        let corpus = Store::build(djot, spec);
        store.insert(djot, spec, &corpus);
        corpus
    }

    /// How many entries the cache holds, across every fold spec.
    fn count(store: &Store) -> usize {
        store.by_spec.values().map(|m| m.len()).sum()
    }

    /// The same prose, twice, is the same entry — which is the whole point.
    #[test]
    fn the_same_source_is_parsed_and_folded_once() {
        let mut store = fresh();
        let djot = "Aurélien traversa la *forêt* qui portait l'odeur du sel.";
        let a = get(&mut store, djot, &spec());
        let b = get(&mut store, djot, &spec());
        assert!(Arc::ptr_eq(&a, &b), "a second lookup must not re-parse");
        assert_eq!(count(&store), 1);
    }

    /// …and edited prose is a different key, so it cannot serve the old text. This is the
    /// invalidation bug the design exists to make impossible: there is nothing to invalidate.
    #[test]
    fn edited_prose_is_a_different_entry() {
        let mut store = fresh();
        let before = get(&mut store, "Elena rentra chez elle.", &spec());
        let after = get(&mut store, "Elena rentra chez elle, enfin.", &spec());
        assert!(!Arc::ptr_eq(&before, &after));
        assert!(after.source().contains("enfin"));
        assert!(
            !before.source().contains("enfin"),
            "the old entry is untouched — and unreachable, because its key is the old text"
        );
    }

    /// The fold rules ARE part of the key. Prose folded case-insensitively cannot answer a
    /// case-sensitive query — it would report matches that are not there.
    #[test]
    fn different_fold_rules_are_different_entries() {
        let mut store = fresh();
        let djot = "Le CAFÉ était froid.";
        let loose = get(&mut store, djot, &spec());
        let strict = get(
            &mut store,
            djot,
            &FoldSpec {
                case_sensitive: true,
                ..spec()
            },
        );
        assert!(!Arc::ptr_eq(&loose, &strict));
        assert_eq!(loose.find_all("café", false).len(), 1);
        assert_eq!(
            strict.find_all("café", false).len(),
            0,
            "the case-sensitive entry must not match the uppercase spelling"
        );
    }

    /// `whole_word` must NOT split the cache. It decides which matches survive, not how the
    /// text folds — so **one** entry answers both kinds of query, and ticking the checkbox
    /// does not throw away a manuscript's worth of folding.
    #[test]
    fn one_entry_answers_both_whole_word_and_substring() {
        let mut store = fresh();
        let corpus = get(&mut store, "un arbre, le marbre", &spec());
        assert_eq!(
            corpus.find_all("arbre", false).len(),
            2,
            "substring: `arbre` occurs inside `marbre` too"
        );
        assert_eq!(
            corpus.find_all("arbre", true).len(),
            1,
            "whole word: only the standalone one — from the SAME fold"
        );
        assert_eq!(count(&store), 1, "one fold answered both");
    }

    /// The per-scene language reaches the cache, and two languages are two entries: in Turkish
    /// the dotless `ı` is a different letter, so the same prose folds differently.
    #[test]
    fn the_language_is_part_of_the_key() {
        let mut store = fresh();
        let djot = "KISA bir yol.";
        let root = get(&mut store, djot, &spec());
        let turkish = get(
            &mut store,
            djot,
            &FoldSpec {
                locale: FoldLocale::Turkic,
                ..spec()
            },
        );
        assert!(!Arc::ptr_eq(&root, &turkish));
        assert_eq!(root.find_all("kisa", false).len(), 1);
        assert_eq!(
            turkish.find_all("kisa", false).len(),
            0,
            "in Turkish, `kisa` is not `kısa`"
        );
    }

    /// The corpus is prose, not markup: a query must not match a link's URL.
    #[test]
    fn the_cached_corpus_is_prose_not_markup() {
        let mut store = fresh();
        let corpus = get(
            &mut store,
            "Voir [la note](https://exemple.test/note) ici.",
            &spec(),
        );
        assert_eq!(corpus.source(), "Voir la note ici.");
        assert_eq!(
            corpus.find_all("exemple", false).len(),
            0,
            "the URL is markup — the writer never typed it into their sentence"
        );
    }

    /// **A cached answer must equal an uncached one.** A cache that could disagree with the
    /// matcher would be worse than no cache: it would show the writer results their editor
    /// does not have.
    #[test]
    fn a_cached_search_agrees_with_an_uncached_one() {
        let mut store = fresh();
        let djot = "Aurélien et *Aurelie* dans la forêt; l'ombre d'Aurélien s'étirait.";
        for whole_word in [false, true] {
            let options = MatchOptions {
                whole_word,
                ..MatchOptions::default()
            };
            let corpus = get(&mut store, djot, &options.fold_spec());
            let cached = corpus.find_all("aurelien", whole_word);
            let uncached = text_document::matching::find_all(corpus.source(), "aurelien", &options);
            assert_eq!(cached, uncached, "whole_word={whole_word}");
            assert_eq!(cached.len(), 2, "both spellings of the name");
        }
    }

    /// **The recorded size must be the size actually held.** `FoldedText`'s word-boundary table
    /// is built lazily — on the first whole-word query, which for a cached entry is always
    /// *after* it was measured — so an entry measured naively grows behind the cache's back,
    /// and a budget summed from stale sizes bounds nothing.
    #[test]
    fn an_entrys_recorded_size_does_not_drift_when_whole_word_is_used() {
        let mut store = fresh();
        let djot = "un arbre, le marbre, et la forêt d'Aurélien. ".repeat(200);
        let corpus = get(&mut store, &djot, &spec());
        let recorded = store.heap;

        // The first whole-word query is exactly when the lazy table would have been built.
        corpus.find_all("arbre", true);

        assert_eq!(
            corpus.heap_size() + djot.len(),
            recorded,
            "the entry weighs what the cache recorded — the boundaries were prepared at insert"
        );
    }

    /// Overflow clears the cache rather than growing without bound. An afternoon's writing
    /// mints a new key on every edit, so it must have a ceiling.
    #[test]
    fn overflowing_the_budget_clears_rather_than_grows() {
        let mut store = fresh();
        // A budget big enough for one of these entries, but not two.
        let one = get(&mut store, "la première scène", &spec());
        store.max_heap = store.heap + 8;
        assert_eq!(count(&store), 1);
        drop(one);

        get(&mut store, "la seconde scène, tout à fait autre", &spec());
        assert_eq!(
            count(&store),
            1,
            "the overflow cleared the old entries and kept only the new one"
        );
        assert!(store.heap <= store.max_heap, "and the accounting was reset");
    }

    /// An entry that alone exceeds the budget is **served but not cached**. Caching it would
    /// leave the store over budget the moment it landed, so the next insert would clear again
    /// — turning every single lookup into a cold one, for ever.
    #[test]
    fn an_entry_too_big_for_the_budget_is_served_but_not_cached() {
        let mut store = fresh();
        let kept = get(&mut store, "une scène qui tient dans le budget", &spec());
        let held = store.heap;
        assert_eq!(count(&store), 1);

        // Now nothing bigger than what we already hold may be cached.
        store.max_heap = held;

        let giant = "mot ".repeat(4000);
        let corpus = get(&mut store, &giant, &spec());

        // Served, and correct…
        assert!(corpus.find_all("mot", false).len() >= 4000);
        // …but not cached, and — crucially — it did NOT evict what was already there.
        assert_eq!(count(&store), 1, "the giant entry was not cached");
        assert_eq!(
            store.heap, held,
            "and nothing was evicted to make room for it"
        );
        assert!(
            Arc::ptr_eq(
                &kept,
                &get(&mut store, "une scène qui tient dans le budget", &spec())
            ),
            "the entry that fits is still there"
        );
    }

    /// The global path works, and `clear()` frees it. Kept as ONE test so it cannot race the
    /// others — every test above uses its own `Store`.
    #[test]
    fn the_global_cache_works_and_can_be_cleared() {
        let djot = "une phrase que seule cette épreuve met en cache";
        let a = corpus_for(djot, &spec());
        let b = corpus_for(djot, &spec());
        assert!(Arc::ptr_eq(&a, &b));
        assert!(heap_size() > 0);

        clear();
        assert_eq!(heap_size(), 0);
    }
}
