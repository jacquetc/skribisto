//! The prose of every scene, parsed and folded — kept between keystrokes.
//!
//! ## What it is for
//!
//! `run_search` runs on **every keystroke** of the search box, and it re-does the same work
//! on the same unchanged prose each time. Measured over a 300k-word manuscript (4000 scenes),
//! in release:
//!
//! | | |
//! |---|---|
//! | parse the Djot into prose (`djot_to_plain_text`) | 33 ms |
//! | fold the prose for matching | 14 ms |
//! | actually scan it | 16 ms |
//!
//! …and none of the first two changes between one keystroke and the next. All of it happens
//! **synchronously on the UI thread**, so it is not "a slow feature", it is a stall in the
//! writer's typing every time they pause.
//!
//! ## Content-addressed, so there is nothing to invalidate
//!
//! The obvious cache keys on the entity — `content_id`, versioned by `updated_at` — and then
//! has to be told when to forget: on `Content` updated, on removed, on a project close, on a
//! language change. Every one of those is a place to get it wrong, and the failure mode is
//! **serving a writer stale prose**: a search that finds a word they deleted, or misses one
//! they just typed.
//!
//! So it keys on the **prose itself**. The entry for a given `(Djot source, fold rules)` is
//! the parse and the fold of exactly that text, and it cannot go stale — if the writer edits
//! a scene, the source is a different string, which is a different key, which is a miss. No
//! invalidation, no events to subscribe to, no ordering to get right.
//!
//! It also makes the cache safe to share across `AppContext`s (which the test suite creates
//! by the dozen, in one process): two stores holding the same prose *should* get the same
//! answer. An id-keyed cache would have had to be scoped per store, and a store's identity is
//! its address — which is reused after it is dropped.
//!
//! The key is the source `String`, not a hash of it. A 64-bit hash collision would serve one
//! scene's prose as another's — silently, and in a *writer's manuscript*. `HashMap` compares
//! the keys it stores, so there is no such window; the cost is holding the Djot a second time
//! (~1.7 MB for that 300k-word novel), which is the smallest of the four things kept here.
//!
//! ## Bounded
//!
//! Every edit to a scene mints a new key, so an afternoon's writing accumulates entries for
//! prose that no longer exists. The cache is bounded by total heap and **cleared wholesale**
//! when it overflows, rather than evicted one entry at a time: the working set is "the
//! manuscript open right now", not a recency distribution, and the cost of being wrong is one
//! cold search.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use text_document::matching::{FoldSpec, FoldedText};
use text_document::{DjotImportOptions, djot_to_plain_text};

/// One scene's prose, parsed and folded. `folded.source()` is the prose itself — what a
/// snippet is cut from, and what the match offsets address.
pub type Corpus = FoldedText;

/// What makes two corpora the same corpus: the same source text, folded by the same rules.
///
/// **`whole_word` is not part of it.** It decides which matches survive, not how a character
/// folds — so one entry answers both kinds of query, and ticking the checkbox does not throw
/// away a manuscript's worth of folding. That distinction lives in text-document's types
/// (`FoldSpec` vs `MatchOptions`), which is why it cannot be got wrong here.
#[derive(PartialEq, Eq, Hash, Clone)]
struct Key {
    source: String,
    spec: FoldSpec,
}

/// How much heap the cache may hold before it is cleared. A 300k-word novel folds to roughly
/// 20 MB, so this holds one comfortably, plus a long session's worth of edits.
const MAX_HEAP: usize = 128 * 1024 * 1024;

/// The cache. Process-global on purpose: Skribisto is **one process per project**, so a global
/// *is* project-scoped — and being content-addressed, it would be correct even if it were not.
static CACHE: RwLock<Option<Store>> = RwLock::new(None);

/// The cache proper, with no global in it — so the tests below can exercise it
/// deterministically. Rust runs tests in **parallel threads of one process**, and a global
/// would make `clear()` in one test race an `Arc::ptr_eq` in another.
#[derive(Default)]
struct Store {
    entries: HashMap<Key, Arc<Corpus>>,
    heap: usize,
}

impl Store {
    fn get(&self, key: &Key) -> Option<Arc<Corpus>> {
        self.entries.get(key).map(Arc::clone)
    }

    fn insert(&mut self, key: Key, corpus: &Arc<Corpus>) {
        let size = corpus.heap_size() + key.source.capacity();
        // Cleared **wholesale** on overflow rather than evicted one entry at a time: the
        // working set is "the manuscript open right now", not a recency distribution, and the
        // cost of being wrong is one cold search.
        if self.heap + size > MAX_HEAP {
            self.entries.clear();
            self.heap = 0;
        }
        if self.entries.insert(key, Arc::clone(corpus)).is_none() {
            self.heap += size;
        }
    }

    /// The expensive part: parse the Djot into prose, then fold the prose. Deliberately a
    /// free function taking no `&self` — it must never be called with the lock held (see
    /// [`corpus_for`]).
    fn build(djot: &str, spec: &FoldSpec) -> Arc<Corpus> {
        let prose = djot_to_plain_text(djot, &DjotImportOptions::default());
        Arc::new(FoldedText::new(&prose, spec))
    }
}

/// The parsed, folded prose of one `Content.data` — from the cache if it is there, built and
/// cached if not.
///
/// Returns an `Arc` so the caller can hold it across the scan without keeping the lock.
pub fn corpus_for(djot: &str, spec: &FoldSpec) -> Arc<Corpus> {
    let key = Key {
        source: djot.to_string(),
        spec: *spec,
    };

    if let Ok(guard) = CACHE.read()
        && let Some(hit) = guard.as_ref().and_then(|s| s.get(&key))
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
            .insert(key, &corpus);
    }

    corpus
}

/// Drop everything. Called when a project closes — not for correctness (the cache cannot go
/// stale) but so a long-lived process does not hold the prose of a manuscript nobody has open.
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
        let key = Key {
            source: djot.to_string(),
            spec: *spec,
        };
        if let Some(hit) = store.get(&key) {
            return hit;
        }
        let corpus = Store::build(djot, spec);
        store.insert(key, &corpus);
        corpus
    }

    /// The same prose, twice, is the same entry — which is the whole point.
    #[test]
    fn the_same_source_is_parsed_and_folded_once() {
        let mut store = fresh();
        let djot = "Aurélien traversa la *forêt* qui portait l'odeur du sel.";
        let a = get(&mut store, djot, &spec());
        let b = get(&mut store, djot, &spec());
        assert!(Arc::ptr_eq(&a, &b), "a second lookup must not re-parse");
        assert_eq!(store.entries.len(), 1);
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
        assert_eq!(store.entries.len(), 1, "one fold answered both");
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

    /// Overflow clears the cache rather than growing without bound. An afternoon's writing
    /// mints a new key on every edit, so it must have a ceiling.
    #[test]
    fn overflowing_the_budget_clears_rather_than_grows() {
        let mut store = fresh();
        get(&mut store, &"mot ".repeat(2000), &spec());
        assert_eq!(store.entries.len(), 1);
        let held = store.heap;
        assert!(held > 0);

        store.heap = MAX_HEAP; // pretend we are at the ceiling
        get(&mut store, "une autre scène", &spec());
        assert_eq!(
            store.entries.len(),
            1,
            "the overflow cleared the old entries and kept only the new one"
        );
        assert!(store.heap < held, "the byte accounting was reset too");
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
