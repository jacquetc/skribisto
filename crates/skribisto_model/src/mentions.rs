// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Finding the story bible in the prose — which characters, places and objects a scene
//! actually mentions, without the writer linking anything by hand.
//!
//! An item carrying a **discoverable** tag is story-bible material. It answers to its title
//! and to its `aliases` ("Elizabeth Bennet", "Lizzy", "Miss Bennet"). This module matches
//! those names against a scene's prose and reports where each was found. That is all it
//! does: it takes strings and returns offsets. It reads nothing, writes nothing, and knows
//! nothing about entities beyond the ids the caller hands it — the same charter
//! [`counting`](crate::counting) has, and for the same reason.
//!
//! ## Matching is case-SENSITIVE, and that is the whole design
//!
//! The framework folds case by default. Here it must not. Characters are called Grace, Hope,
//! Will, Rose, Mark, Faith — fold case and every one of them matches ordinary prose on every
//! page, and because those are *title* matches they sort to the top of the roster. A reader
//! looking for "who is in this scene" would be handed a list of English adverbs.
//!
//! Capitalisation is a nearly-free proper-noun signal in running prose, so the default keeps
//! it. Diacritics are still folded (`Aurelien` finds `Aurélien`) — that is an orthographic
//! accident, not a signal, and the two are independent settings.
//!
//! Whole-word matching is equally load-bearing: without it "Rose" matches inside "Roses" and
//! "Will" inside "willingly". It also comes with the apostrophe handled — the matcher treats
//! an apostrophe as a word boundary on both sides, so "Elena" finds "Elena's" and "Aurélien"
//! finds "d'Aurélien", which no amount of tuning here could have achieved.
//!
//! **A known false positive follows from that rule and is not fixable here:** whole-word
//! "Don" matches inside "don't". A character named Don generates a suggestion on every
//! contraction in the book. This is why every suggestion is shown with its evidence rather
//! than asserted — the writer reads "don't" in the excerpt and declines it.
//!
//! ## The cache key is compound, and that is not an optimisation detail
//!
//! [`counting`](crate::counting) is content-addressed on one input, so it cannot go stale. A mention result
//! depends on **two** inputs that change independently: the prose, and the alias table. Key
//! it on the prose alone and renaming an alias — with no scene touched — serves the previous
//! answer forever. So the key carries a fingerprint of the table as well; see
//! [`cached_mentions`].

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

use text_document::matching::{FoldLocale, FoldedText, MatchOptions};

/// One discoverable item's matching surface, as the caller assembles it.
///
/// The caller decides what is discoverable (an item carrying a tag whose `discoverable` flag
/// is set) and what is in scope (activated, untrashed). This module just matches names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscoverableEntity {
    pub id: u64,
    pub title: String,
    pub aliases: Vec<String>,
}

/// Names shorter than this never match.
///
/// Per *name*, not per entity: an item titled "Al" with an alias "Albert" still matches on
/// the alias. Two characters are hopeless — initials, "Mr", and every preposition in the
/// language collide with them, and the evidence tooltip cannot rescue a suggestion that
/// appears four hundred times.
pub const MIN_NAME_LEN: usize = 3;

/// One resolved hit, in **char offsets into the prose that was scanned**.
///
/// Carries no matched text: the caller has the entity table and can resolve `entity_id` +
/// [`is_title_match`](Self::is_title_match) back to the name, and duplicating the string
/// into every hit would dominate the cache it is stored in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mention {
    pub entity_id: u64,
    /// Whether the title matched, as opposed to one of the aliases. Titles rank above
    /// aliases when both land on the same span, and read better in the roster.
    pub is_title_match: bool,
    /// Index into the entity's `aliases`, when this was an alias match.
    pub alias_index: usize,
    pub char_start: usize,
    pub char_len: usize,
}

impl Mention {
    /// The name this hit matched, resolved against the entity that produced it.
    pub fn matched_name<'a>(&self, entity: &'a DiscoverableEntity) -> &'a str {
        if self.is_title_match {
            &entity.title
        } else {
            entity
                .aliases
                .get(self.alias_index)
                .map(String::as_str)
                .unwrap_or(&entity.title)
        }
    }
}

/// The one matching configuration mention-scanning uses. Not caller-configurable: see the
/// module docs for why case-sensitivity and whole-word are not preferences.
pub fn mention_options(locale: FoldLocale) -> MatchOptions {
    MatchOptions {
        case_sensitive: true,
        diacritic_sensitive: false,
        whole_word: true,
        locale,
    }
}

/// Every name an entity answers to, longest first, paired with how to describe the hit.
///
/// Longest first is what makes overlap resolution cheap below: the first name to claim a
/// span is the longest one that could.
fn names_of(entity: &DiscoverableEntity) -> Vec<(&str, bool, usize)> {
    let mut names: Vec<(&str, bool, usize)> = std::iter::once((entity.title.as_str(), true, 0))
        .chain(
            entity
                .aliases
                .iter()
                .enumerate()
                .map(|(i, a)| (a.as_str(), false, i)),
        )
        .filter(|(n, _, _)| n.chars().count() >= MIN_NAME_LEN)
        .collect();
    // Longest first; a title beats an alias of equal length.
    names.sort_by(|a, b| {
        b.0.chars()
            .count()
            .cmp(&a.0.chars().count())
            .then(b.1.cmp(&a.1))
    });
    names
}

/// Scan one plain-text prose string against the alias table.
///
/// The prose must already be plain — Djot markup would produce hits inside link syntax and
/// attribute blocks. Stripping is the caller's job because only the caller knows which
/// dialect the string came from.
///
/// Overlaps are resolved **longest-match-wins**: "Grace Kelly" and "Grace" are two different
/// characters, and a scene naming the former must not also credit the latter. Ties go to the
/// title over an alias.
pub fn scan_prose(prose: &str, table: &[DiscoverableEntity], locale: FoldLocale) -> Vec<Mention> {
    if prose.is_empty() || table.is_empty() {
        return Vec::new();
    }
    let options = mention_options(locale);
    // Folded ONCE for the whole table. `fold_spec` deliberately excludes `whole_word`, so a
    // single fold answers every name — which is the difference between one pass over the
    // scene and one pass per character in the book.
    let folded = FoldedText::new(prose, &options.fold_spec());

    let mut hits: Vec<Mention> = Vec::new();
    for entity in table {
        for (name, is_title, alias_index) in names_of(entity) {
            for m in folded.find_all(name, options.whole_word) {
                hits.push(Mention {
                    entity_id: entity.id,
                    is_title_match: is_title,
                    alias_index,
                    char_start: m.char_start,
                    char_len: m.char_len,
                });
            }
        }
    }

    resolve_overlaps(hits)
}

/// Drop every hit that overlaps a longer one.
///
/// Sorted by start, then by descending length, then title-before-alias: the first hit at any
/// position is the one that should win, so a single sweep keeping "the next hit that starts
/// at or after the end of the last kept one" is enough.
fn resolve_overlaps(mut hits: Vec<Mention>) -> Vec<Mention> {
    hits.sort_by(|a, b| {
        a.char_start
            .cmp(&b.char_start)
            .then(b.char_len.cmp(&a.char_len))
            .then(b.is_title_match.cmp(&a.is_title_match))
            .then(a.entity_id.cmp(&b.entity_id))
    });
    let mut kept: Vec<Mention> = Vec::with_capacity(hits.len());
    let mut end_of_last = 0usize;
    for h in hits {
        if kept.is_empty() || h.char_start >= end_of_last {
            end_of_last = h.char_start + h.char_len;
            kept.push(h);
        }
    }
    kept
}

/// Identity of an alias table, for cache keying.
///
/// Order-independent: the caller assembles the table by walking binders, and a reorder there
/// must not invalidate every cached scan. Any change to membership, a title or an alias does
/// change it — that is the whole point.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AliasTableFingerprint(u64);

pub fn fingerprint_alias_table(table: &[DiscoverableEntity]) -> AliasTableFingerprint {
    // XOR-folded per-entity hashes: commutative, so order drops out without sorting a
    // potentially large table on every scan.
    let mut acc: u64 = 0;
    for e in table {
        let mut h = DefaultHasher::new();
        e.id.hash(&mut h);
        e.title.hash(&mut h);
        // Aliases are hashed in order on purpose: they are the writer's own list, reordering
        // it is an edit, and the roster shows them in that order.
        e.aliases.hash(&mut h);
        acc ^= h.finish();
    }
    AliasTableFingerprint(acc)
}

/// How much heap the cache may hold before it is cleared wholesale.
///
/// Same budget and same reasoning as [`counting`](crate::counting): the weight is the prose
/// strings held as keys, not the hits.
const MAX_HEAP: usize = 64 * 1024 * 1024;

/// Process-global, exactly as [`counting`](crate::counting)'s is: keyed by a content
/// fingerprint of the alias table, so one cache stays correct across several projects open
/// in the same process at once.
static CACHE: RwLock<Option<Store>> = RwLock::new(None);

/// The cache proper, with no global in it, so tests exercise it deterministically.
///
/// Nested by `(fingerprint, locale)` so the inner map is keyed by the prose alone and can be
/// probed with a plain `&str` — no allocation on a hit. The outer key is what makes this
/// correct rather than merely fast: a renamed alias produces a different fingerprint, so it
/// lands in a *different slot* and cannot read the previous answer.
/// One alias table's cached scans, keyed by the prose that was scanned.
///
/// `Arc` because a hit hands the same result to every caller rather than cloning
/// a mention list per lookup.
type ScansByProse = HashMap<String, Arc<Vec<Mention>>>;

/// The cache's outer map: a slot per `(alias table, locale)` pair.
///
/// Named rather than written inline because the nesting is the whole design —
/// see [`Store`] — and clippy's `type_complexity` is right that three levels of
/// generics in a field declaration reads as an accident.
type ScansByTable = HashMap<(AliasTableFingerprint, FoldLocale), TableSlot>;

/// How many alias tables keep their cached scans.
///
/// A slot is reachable only while its fingerprint can be recomputed, and the
/// fingerprint folds each entity's **store id**. Those ids are re-minted by every
/// `load_work`, so reopening a project mints a slot and orphans the previous one
/// beyond any hope of a hit. Renaming one character does the same, mid-session.
/// Neither is a leak the heap budget catches quickly: at 64 MB it takes a hundred
/// reopenings, each holding the whole manuscript's prose as keys.
///
/// Eight is well past what is legitimately live (one per open project per fold
/// locale) and small enough that orphaned slots cannot accumulate.
const MAX_TABLES: usize = 8;

/// One alias table's scans, with the heap they hold and when they were last read.
///
/// The per-slot total is what lets a slot be evicted without rescanning the whole
/// store to find out what it was holding.
struct TableSlot {
    scans: ScansByProse,
    heap: usize,
    last_used: u64,
}

struct Store {
    by_table: ScansByTable,
    heap: usize,
    max_heap: usize,
    /// Monotonic tick stamped onto a slot each time it is read or written, so the
    /// least recently used slot can be found without an ordered container.
    clock: u64,
}

impl Default for Store {
    fn default() -> Self {
        Store {
            by_table: HashMap::new(),
            heap: 0,
            max_heap: MAX_HEAP,
            clock: 0,
        }
    }
}

impl Store {
    fn get(
        &mut self,
        prose: &str,
        key: (AliasTableFingerprint, FoldLocale),
    ) -> Option<Arc<Vec<Mention>>> {
        self.clock += 1;
        let clock = self.clock;
        let slot = self.by_table.get_mut(&key)?;
        slot.last_used = clock;
        slot.scans.get(prose).cloned()
    }

    fn insert(
        &mut self,
        prose: &str,
        key: (AliasTableFingerprint, FoldLocale),
        hits: Arc<Vec<Mention>>,
    ) {
        let size = prose.len() + hits.len() * std::mem::size_of::<Mention>();
        // An entry that alone exceeds the budget is served but not cached. Caching it would
        // leave the store over budget, so the next insert would clear again, forever.
        if size > self.max_heap {
            return;
        }
        if self.heap + size > self.max_heap {
            self.by_table.clear();
            self.heap = 0;
        }
        self.clock += 1;
        let clock = self.clock;
        let slot = self.by_table.entry(key).or_insert_with(|| TableSlot {
            scans: ScansByProse::new(),
            heap: 0,
            last_used: clock,
        });
        slot.last_used = clock;
        if slot.scans.insert(prose.to_string(), hits).is_none() {
            slot.heap += size;
            self.heap += size;
        }
        self.evict_stale_tables();
    }

    /// Drop the least recently used slots until at most [`MAX_TABLES`] remain.
    ///
    /// Eviction is per alias table rather than per entry on purpose: a slot goes
    /// cold as a whole, when the ids it was keyed on stop existing, and every
    /// entry in it goes cold with it.
    fn evict_stale_tables(&mut self) {
        while self.by_table.len() > MAX_TABLES {
            let Some(oldest) = self
                .by_table
                .iter()
                .min_by_key(|(_, slot)| slot.last_used)
                .map(|(key, _)| *key)
            else {
                return;
            };
            if let Some(slot) = self.by_table.remove(&oldest) {
                self.heap = self.heap.saturating_sub(slot.heap);
            }
        }
    }
}

/// [`scan_prose`], memoised on `(prose, alias table, locale)`.
///
/// The caller passes the fingerprint it already computed for the whole scan rather than
/// having this recompute it per scene.
pub fn cached_mentions(
    prose: &str,
    table: &[DiscoverableEntity],
    fingerprint: AliasTableFingerprint,
    locale: FoldLocale,
) -> Arc<Vec<Mention>> {
    let key = (fingerprint, locale);
    // A write lock for a read, because a hit stamps the slot's recency and that is
    // what keeps the live tables live under [`MAX_TABLES`]. The critical section is
    // two hash lookups; the scan below is what is expensive, and it stays outside.
    if let Ok(mut guard) = CACHE.write()
        && let Some(hit) = guard.as_mut().and_then(|s| s.get(prose, key))
    {
        return hit;
    }

    // Computed OUTSIDE the write lock: the fold dominates everything else, and holding the
    // lock across it would serialise every scene behind one of them.
    let hits = Arc::new(scan_prose(prose, table, locale));

    if let Ok(mut guard) = CACHE.write() {
        guard
            .get_or_insert_with(Store::default)
            .insert(prose, key, hits.clone());
    }
    hits
}

/// Drop everything.
///
/// Not for correctness: the key is compound and content-addressed, so nothing can go stale.
/// Frees the heap a closed project's entries were holding.
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

/// The sentence a hit sits in — cut on demand, never stored.
///
/// Scans outward to the nearest sentence terminator or blank line on each side. A fixed
/// context window would as often cut mid-clause, and the point of the excerpt is that the
/// writer can judge the suggestion by reading it: "…and Grace smiled" is evidence, "e and Gra"
/// is not.
///
/// Operates on chars, not bytes, because [`Mention`] offsets are char offsets and the prose
/// is full of accented characters in exactly the names this feature is about.
pub fn evidence_sentence(prose: &str, m: &Mention) -> String {
    let chars: Vec<char> = prose.chars().collect();
    if chars.is_empty() {
        return String::new();
    }
    let start = m.char_start.min(chars.len());
    let end = (m.char_start + m.char_len).min(chars.len());

    let is_break = |c: char| matches!(c, '.' | '!' | '?' | '\n' | '…');

    // Walk back to just after the previous terminator.
    let mut from = start;
    while from > 0 && !is_break(chars[from - 1]) {
        from -= 1;
    }
    // Walk forward to and including the next terminator.
    let mut to = end;
    while to < chars.len() && !is_break(chars[to]) {
        to += 1;
    }
    if to < chars.len() {
        to += 1;
    }

    chars[from..to]
        .iter()
        .collect::<String>()
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entity(id: u64, title: &str, aliases: &[&str]) -> DiscoverableEntity {
        DiscoverableEntity {
            id,
            title: title.to_string(),
            aliases: aliases.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Reopening a project mints new store ids, so it mints a new fingerprint and
    /// a new slot. Nothing can ever hit the old one again, so the store must not
    /// keep it. Driven through `Store` directly rather than the process-global
    /// cache, which other tests in this binary share.
    #[test]
    fn orphaned_alias_tables_are_evicted() {
        let mut store = Store::default();
        let prose = "Grace walked to the harbour and waited for Elias.";
        let hits = Arc::new(Vec::new());

        // One slot per "reopening": same prose, same names, ids re-minted.
        for generation in 0..(MAX_TABLES as u64 * 4) {
            let table = vec![
                entity(generation * 2 + 1, "Grace", &[]),
                entity(generation * 2 + 2, "Elias", &[]),
            ];
            let key = (fingerprint_alias_table(&table), FoldLocale::default());
            store.insert(prose, key, hits.clone());
        }

        assert!(
            store.by_table.len() <= MAX_TABLES,
            "expected at most {MAX_TABLES} alias tables, found {}",
            store.by_table.len()
        );
    }

    /// Eviction must take the slot's bytes off the running total with it, or the
    /// budget drifts upward until the wholesale clear fires for no reason.
    #[test]
    fn evicting_a_table_releases_its_heap() {
        let mut store = Store::default();
        let prose = "Grace walked to the harbour.";
        let hits = Arc::new(Vec::new());

        for generation in 0..(MAX_TABLES as u64 + 1) {
            let table = vec![entity(generation, "Grace", &[])];
            let key = (fingerprint_alias_table(&table), FoldLocale::default());
            store.insert(prose, key, hits.clone());
        }

        let summed: usize = store.by_table.values().map(|slot| slot.heap).sum();
        assert_eq!(
            store.heap, summed,
            "the running total must equal the sum of the surviving slots"
        );
    }

    /// A slot that is still being read must outlive one that is not, whatever
    /// order they were created in. Without this, a second project open in the
    /// same process would evict the project the writer is actually using.
    #[test]
    fn a_table_still_in_use_outlives_an_idle_one() {
        let mut store = Store::default();
        let prose = "Grace walked to the harbour.";
        let hits = Arc::new(Vec::new());

        let live_table = vec![entity(1, "Grace", &[])];
        let live_key = (fingerprint_alias_table(&live_table), FoldLocale::default());
        store.insert(prose, live_key, hits.clone());

        for generation in 0..(MAX_TABLES as u64 * 2) {
            let table = vec![entity(1000 + generation, "Elias", &[])];
            let key = (fingerprint_alias_table(&table), FoldLocale::default());
            store.insert(prose, key, hits.clone());
            // Keep reading the first table, as an open project's scans would.
            assert!(store.get(prose, live_key).is_some());
        }

        assert!(
            store.get(prose, live_key).is_some(),
            "the table being read must never be the one evicted"
        );
    }

    fn scan(prose: &str, table: &[DiscoverableEntity]) -> Vec<Mention> {
        scan_prose(prose, table, FoldLocale::default())
    }

    /// The reason the whole module departs from the framework default. Fold case and a
    /// character named Grace is "found" in every line of ordinary English.
    #[test]
    fn lowercase_prose_does_not_match_a_capitalised_name() {
        let table = [entity(1, "Grace", &["Hope"])];
        assert!(
            scan("she felt grace and hope in equal measure", &table).is_empty(),
            "case-folding would make every abstract noun a character sighting"
        );
    }

    #[test]
    fn a_capitalised_occurrence_matches_and_reports_its_span() {
        let table = [entity(1, "Grace", &[])];
        let hits = scan("Grace walked in.", &table);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].entity_id, 1);
        assert!(hits[0].is_title_match);
        assert_eq!(hits[0].char_start, 0);
        assert_eq!(hits[0].char_len, 5);
    }

    /// Whole-word, or "Will" is in "willingly" and "Rose" is in "Roses".
    #[test]
    fn a_name_inside_a_longer_word_is_not_a_match() {
        let table = [entity(1, "Character", &["Will"])];
        assert!(scan("Willingly he agreed", &table).is_empty());
        let table = [entity(1, "Rose", &[])];
        assert!(scan("The Roses bloomed", &table).is_empty());
    }

    /// Comes free with the matcher's apostrophe-as-boundary rule, and is worth pinning here
    /// because losing it would silently halve the hits in any English manuscript.
    #[test]
    fn a_possessive_still_matches_the_name() {
        let table = [entity(1, "Will", &[])];
        let hits = scan("Will's coat lay there.", &table);
        assert_eq!(hits.len(), 1);
        assert_eq!(
            hits[0].char_len, 4,
            "the apostrophe is not part of the name"
        );
    }

    /// French elision, the same rule from the other side.
    #[test]
    fn an_elided_article_still_matches_the_name() {
        let table = [entity(1, "Aurélien", &[])];
        let hits = scan("Le manteau d'Aurélien.", &table);
        assert_eq!(hits.len(), 1, "d'Aurélien must find Aurélien");
    }

    /// Diacritics fold even though case does not — they are independent settings, and an
    /// accent is an orthographic accident rather than a proper-noun signal.
    #[test]
    fn diacritics_fold_but_case_does_not() {
        let table = [entity(1, "Aurelien", &[])];
        assert_eq!(scan("Aurélien entered.", &table).len(), 1);
        assert!(
            scan("aurélien entered.", &table).is_empty(),
            "folding diacritics must not drag case-folding in with it"
        );
    }

    /// The length floor is per name, so a two-letter title does not disqualify a real alias.
    #[test]
    fn a_too_short_name_is_skipped_without_disabling_the_entity() {
        let table = [entity(1, "Al", &["Albert"])];
        let hits = scan("Al and Albert are the same man.", &table);
        assert_eq!(hits.len(), 1, "only the alias is long enough to match");
        assert!(!hits[0].is_title_match);
        assert_eq!(hits[0].matched_name(&table[0]), "Albert");
    }

    /// Two characters, one name a prefix of the other. Crediting both would put a character
    /// in a scene they are not in.
    #[test]
    fn the_longer_name_wins_an_overlap() {
        let table = [entity(1, "Grace Kelly", &[]), entity(2, "Grace", &[])];
        let hits = scan("Grace Kelly walked in.", &table);
        assert_eq!(hits.len(), 1, "one span, one hit");
        assert_eq!(hits[0].entity_id, 1);
    }

    /// …but the shorter name still matches where it stands alone.
    #[test]
    fn the_shorter_name_still_matches_on_its_own() {
        let table = [entity(1, "Grace Kelly", &[]), entity(2, "Grace", &[])];
        let hits = scan("Grace Kelly left. Grace stayed.", &table);
        let ids: Vec<u64> = hits.iter().map(|h| h.entity_id).collect();
        assert_eq!(ids, vec![1, 2]);
    }

    /// A title and an alias of equal length on the same span: the title reads better in the
    /// roster and is the more likely intent.
    #[test]
    fn a_title_beats_an_alias_of_the_same_length() {
        let table = [entity(1, "Robin", &[]), entity(2, "Other", &["Robin"])];
        let hits = scan("Robin arrived.", &table);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].entity_id, 1);
        assert!(hits[0].is_title_match);
    }

    #[test]
    fn a_fingerprint_ignores_table_order() {
        let a = entity(1, "Grace", &["Gracie"]);
        let b = entity(2, "Will", &[]);
        assert_eq!(
            fingerprint_alias_table(&[a.clone(), b.clone()]),
            fingerprint_alias_table(&[b, a])
        );
    }

    #[test]
    fn a_fingerprint_changes_when_an_alias_changes() {
        let before = [entity(1, "Grace", &["Gracie"])];
        let after = [entity(1, "Grace", &["Gigi"])];
        assert_ne!(
            fingerprint_alias_table(&before),
            fingerprint_alias_table(&after)
        );
    }

    #[test]
    fn a_fingerprint_changes_when_a_member_joins_or_leaves() {
        let one = [entity(1, "Grace", &[])];
        let two = [entity(1, "Grace", &[]), entity(2, "Will", &[])];
        assert_ne!(fingerprint_alias_table(&one), fingerprint_alias_table(&two));
    }

    /// The bug the compound key exists to prevent, exercised end to end.
    ///
    /// `counting`'s cache is keyed on one input and so cannot go stale. This one has two
    /// that move independently: rename an alias, touch no prose, and a prose-only key would
    /// hand back the previous answer for the rest of the session.
    #[test]
    fn renaming_an_alias_does_not_serve_the_previous_answer() {
        clear();
        let prose = "Gigi laughed at Gracie.";
        let before = [entity(1, "Grace", &["Gracie"])];
        let after = [entity(1, "Grace", &["Gigi"])];
        let fp_before = fingerprint_alias_table(&before);
        let fp_after = fingerprint_alias_table(&after);

        let hits_before = cached_mentions(prose, &before, fp_before, FoldLocale::default());
        assert_eq!(hits_before.len(), 1);
        assert_eq!(hits_before[0].matched_name(&before[0]), "Gracie");

        // Same prose, different table. A prose-only key would return "Gracie" again.
        let hits_after = cached_mentions(prose, &after, fp_after, FoldLocale::default());
        assert_eq!(hits_after.len(), 1);
        assert_eq!(hits_after[0].matched_name(&after[0]), "Gigi");

        // And the first answer is still cached under its own key, not overwritten.
        let again = cached_mentions(prose, &before, fp_before, FoldLocale::default());
        assert_eq!(again[0].matched_name(&before[0]), "Gracie");
        clear();
    }

    #[test]
    fn the_cache_returns_the_same_answer_as_a_direct_scan() {
        clear();
        let prose = "Grace met Will. Will nodded.";
        let table = [entity(1, "Grace", &[]), entity(2, "Will", &[])];
        let fp = fingerprint_alias_table(&table);
        let direct = scan(prose, &table);
        let cached = cached_mentions(prose, &table, fp, FoldLocale::default());
        assert_eq!(direct, *cached);
        clear();
    }

    #[test]
    fn evidence_is_the_sentence_the_hit_sits_in() {
        let prose = "He walked in. Grace smiled warmly at him. She left.";
        let table = [entity(1, "Grace", &[])];
        let hits = scan(prose, &table);
        assert_eq!(
            evidence_sentence(prose, &hits[0]),
            "Grace smiled warmly at him."
        );
    }

    /// A hit in the first or last sentence must not walk off either end.
    #[test]
    fn evidence_handles_a_hit_at_either_edge() {
        let table = [entity(1, "Grace", &[])];

        let first = "Grace opened the door. Then she left.";
        let hits = scan(first, &table);
        assert_eq!(evidence_sentence(first, &hits[0]), "Grace opened the door.");

        let last = "He waited. Then came Grace";
        let hits = scan(last, &table);
        assert_eq!(evidence_sentence(last, &hits[0]), "Then came Grace");
    }

    /// Offsets are char offsets, and this feature is about names full of accents — slicing
    /// them as bytes would panic mid-character.
    #[test]
    fn evidence_is_char_safe_around_multibyte_text() {
        let prose = "Café. Aurélien poussa la porte étroite. Fin.";
        let table = [entity(1, "Aurélien", &[])];
        let hits = scan(prose, &table);
        assert_eq!(
            evidence_sentence(prose, &hits[0]),
            "Aurélien poussa la porte étroite."
        );
    }

    #[test]
    fn an_empty_table_or_empty_prose_finds_nothing() {
        assert!(scan("", &[entity(1, "Grace", &[])]).is_empty());
        assert!(scan("Grace walked in.", &[]).is_empty());
    }

    /// The entry-larger-than-the-budget guard: served, but not cached, or the store would
    /// clear on every subsequent insert forever.
    #[test]
    fn an_oversized_entry_is_served_without_being_cached() {
        let mut store = Store {
            max_heap: 32,
            ..Default::default()
        };
        let table = [entity(1, "Grace", &[])];
        let fp = fingerprint_alias_table(&table);
        let big = "x".repeat(1024);
        store.insert(&big, (fp, FoldLocale::default()), Arc::new(Vec::new()));
        assert_eq!(store.heap, 0, "nothing was stored");
        assert!(store.get(&big, (fp, FoldLocale::default())).is_none());
    }

    #[test]
    fn overflowing_the_budget_clears_rather_than_grows() {
        let table = [entity(1, "Grace", &[])];
        let fp = fingerprint_alias_table(&table);
        let key = (fp, FoldLocale::default());
        let mut store = Store {
            max_heap: 256,
            ..Default::default()
        };
        for i in 0..40 {
            store.insert(&format!("{i}{}", "y".repeat(32)), key, Arc::new(Vec::new()));
        }
        assert!(
            store.heap <= store.max_heap,
            "heap {} exceeded its budget",
            store.heap
        );
    }
}
