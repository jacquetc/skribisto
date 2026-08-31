// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Deciding whether a word is misspelled, and what to offer instead.
//!
//! Hunspell dictionaries in, a predicate and a ranked suggestion list out. No
//! store, no widget, no application paths — a caller hands it the two files and
//! gets back something it can ask about a word. Everything that knows *where* a
//! dictionary lives, *when* to re-check, or *how* to paint a squiggle stays in
//! the application, because all three are properties of a particular editor
//! rather than of spell-checking.
//!
//! ## Encodings
//!
//! [`spellbook::Dictionary::new`] takes `&str` and does not honour the `.aff`'s
//! own `SET` line, so the pair is transcoded here before it is handed over. A
//! dictionary that still fails to parse is treated as absent, never as a crash:
//! an unusable dictionary should cost the writer a missing squiggle, not a
//! session.

use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

/// The encoding an `.aff` declares via its `SET <name>` directive (UTF-8 if none/unknown).
pub fn detect_encoding(aff_bytes: &[u8]) -> &'static encoding_rs::Encoding {
    // `SET` and the label are ASCII, so a lossy decode of the head is safe to scan.
    let head = String::from_utf8_lossy(&aff_bytes[..aff_bytes.len().min(1024)]);
    for line in head.lines() {
        if let Some(rest) = line.strip_prefix("SET ")
            && let Some(enc) = encoding_rs::Encoding::for_label(rest.trim().as_bytes())
        {
            return enc;
        }
    }
    encoding_rs::UTF_8
}

/// Read an `.aff`/`.dic` pair to UTF-8 `String`s, transcoding from the `.aff`'s declared
/// encoding (which governs both files).
pub fn read_pair(aff_path: &Path, dic_path: &Path) -> Option<(String, String)> {
    let aff_bytes = std::fs::read(aff_path).ok()?;
    let enc = detect_encoding(&aff_bytes);
    let (aff, _, _) = enc.decode(&aff_bytes);
    let dic_bytes = std::fs::read(dic_path).ok()?;
    let (dic, _, _) = enc.decode(&dic_bytes);
    Some((aff.into_owned(), dic.into_owned()))
}

/// Read, transcode and parse an `.aff`/`.dic` pair.
///
/// `None` when the files are absent or spellbook cannot parse them — an
/// unusable dictionary is absent, not an error to propagate. The application's
/// own loader finds the paths and caches the result; this is the part that does
/// not depend on where they came from.
pub fn load_pair(aff_path: &Path, dic_path: &Path) -> Option<Arc<spellbook::Dictionary>> {
    let (aff, dic) = read_pair(aff_path, dic_path)?;
    spellbook::Dictionary::new(&aff, &dic).ok().map(Arc::new)
}

/// Check that an `.aff`/`.dic` pair can actually be used, running the **exact** steps [`load_pair`]
/// does — read + transcode from the declared encoding, then parse with spellbook. Used by the
/// "Add dictionary" flow to refuse a bad pair up front (which would otherwise install silently
/// and simply never flag anything). A pair that validates here is one the engine can load.
pub fn validate_dictionary_files(aff_path: &Path, dic_path: &Path) -> Result<(), String> {
    let (aff, dic) = read_pair(aff_path, dic_path)
        .ok_or_else(|| "could not read the .aff / .dic files".to_string())?;
    spellbook::Dictionary::new(&aff, &dic)
        .map(|_| ())
        .map_err(|e| format!("not a valid Hunspell dictionary ({e:?})"))
}

/// Word tokens of a block, as `(char_offset, char_length, word)` — the coordinates
/// a rich-text highlighter expects (character positions, not bytes). UAX#29 word
/// segmentation keeps contractions and elisions together (`don't`, `l'auteur`), for both the
/// straight `'` and the curly `’`.
///
/// `pub(crate)` so the editor's "Add to dictionary" menu resolves selection/caret words with the
/// **same** tokenizer the squiggles use — what is addable and what is flagged can never disagree.
pub fn word_positions(text: &str) -> Vec<(usize, usize, &str)> {
    use unicode_segmentation::UnicodeSegmentation;
    let mut out = Vec::new();
    // Running byte→char cursor so the whole pass is O(n), not O(n) per word.
    let mut last_byte = 0usize;
    let mut char_pos = 0usize;
    for (byte_off, word) in text.unicode_word_indices() {
        char_pos += text[last_byte..byte_off].chars().count();
        let len = word.chars().count();
        out.push((char_pos, len, word));
        char_pos += len;
        last_byte = byte_off + word.len();
    }
    out
}

/// A misspelling predicate over one document's **active** dictionaries — the pure engine a
/// per-document spell session queries per word. Immutable: a snapshot of the active
/// dictionaries (primary first) + the Work's personal words. Built by
/// the application's own checker builder. `Clone` is cheap — `Arc` dictionaries — so one build can
/// feed both the main and synopsis sessions.
#[derive(Clone)]
pub struct SpellChecker {
    /// Active dictionaries, primary first; empty is never built (a checker with no dictionary is
    /// never produced — the caller clears the session instead).
    dicts: Vec<Arc<spellbook::Dictionary>>,
    /// The Work's personal words — the **last** checker, consulted only after every installed
    /// dictionary has rejected the word: the project's own fallback for words no dictionary knows.
    /// Removing one is just removing the entity.
    personal: HashSet<String>,
}

/// How many corrections the context menu offers at most. The suggestions sit flat at the top of
/// the menu, so this is a menu-length budget as much as a relevance one — past a handful, a list
/// of guesses is harder to scan than retyping the word.
pub const MAX_SUGGESTIONS: usize = 6;

/// The largest edit distance at which a *personal* word is offered as a correction. Two edits is
/// the usual typo radius (Hunspell's own replacement table works in the same neighbourhood);
/// wider than that and a short project term starts "correcting" to every other project term.
pub const MAX_PERSONAL_DISTANCE: usize = 2;

/// How many of [`MAX_SUGGESTIONS`] are held back for *distant* personal matches when any exist.
///
/// Without a reservation the project's own term is unreachable exactly when it matters. A
/// two-edit typo of a coined word (`Skiibsto` for `Skribisto`) is a word no installed dictionary
/// knows, which is precisely when Hunspell's ngram search is at its most talkative — it happily
/// returns six unrelated English guesses. Appending the personal matches after those and then
/// truncating would drop the only correction the writer wanted.
pub const PERSONAL_SUGGESTION_FLOOR: usize = 2;

impl SpellChecker {
    pub fn misspelled(&self, word: &str) -> bool {
        // Numbers, punctuation runs, and the like are not spell-checkable.
        if !word.chars().any(|c| c.is_alphabetic()) {
            return false;
        }
        // The true dictionaries first — correctly-spelled prose is accepted by the primary and
        // never reaches the personal set (`any` short-circuits). Only a word that *no* installed
        // dictionary knows falls through to the project's own word list, the final checker.
        if self.dicts.iter().any(|d| d.check(word)) {
            return false;
        }
        !self.personal.contains(word)
    }

    /// Ranked corrections for a misspelled `word`, drawn from **both** the installed dictionaries
    /// and the Work's own personal words. At most [`MAX_SUGGESTIONS`], deduped exact-case.
    ///
    /// ## Why the personal set is searched separately
    ///
    /// [`spellbook::Dictionary::suggest`] is closed over the *compiled* dictionary: it cannot see
    /// the personal word set at all, so a typo of a project's own coined term would never
    /// be corrected to it — the one case a writer most needs. Those near-matches are therefore
    /// found here, by bounded edit distance over the personal set.
    ///
    /// ## The ordering
    ///
    /// A personal word within **one** edit goes first: for an invented word the installed
    /// dictionary has nothing real to offer, and its ngram guesses are noise next to the term the
    /// writer actually meant. The dictionary's own ranked suggestions follow (they are the right
    /// answer for a typo of an ordinary word), and the looser personal matches come last — but
    /// with [`PERSONAL_SUGGESTION_FLOOR`] slots reserved for them, so a talkative dictionary can
    /// never crowd the project's own term off the end of the list.
    pub fn suggest(&self, word: &str) -> Vec<String> {
        if !word.chars().any(|c| c.is_alphabetic()) {
            return Vec::new();
        }
        // Each dictionary's suggestions, concatenated **lazily**: `map` + `flatten` pull one
        // dictionary at a time, so a second language's ngram search never runs once
        // `merge_suggestions` has stopped taking.
        let dict = self.dicts.iter().flat_map(|d| {
            let mut buf = Vec::new();
            d.suggest(word, &mut buf); // clears `buf` itself before filling it
            buf
        });
        merge_suggestions(word, self.personal_suggestions(word), dict)
    }

    /// Personal words within [`MAX_PERSONAL_DISTANCE`] edits of `word`, as `(distance, word)`,
    /// nearest first.
    ///
    /// Distance is measured on the **lower-cased** forms, so a personal word differing only in
    /// casing comes back at distance 0. That is not a curiosity but the common case: the personal
    /// set is matched exact-case, so typing `skribisto` when the project stores `Skribisto` *is* a
    /// misspelling — and the correction to offer is the stored casing.
    ///
    /// Ties break alphabetically: `personal` is a `HashSet`, whose iteration order varies run to
    /// run, and a context menu whose items reshuffle between right-clicks is unusable.
    ///
    /// The typed word itself is *not* filtered here — [`push_unique`] is the single gate that
    /// drops it, so every source is held to the same rule.
    fn personal_suggestions(&self, word: &str) -> Vec<(usize, String)> {
        // Collected once: the needle is invariant across the scan, and this runs over every
        // personal word (an imported list may hold thousands).
        let needle: Vec<char> = word.to_lowercase().chars().collect();
        let mut scored: Vec<(usize, String)> = self
            .personal
            .iter()
            .filter_map(|w| {
                let lower = w.to_lowercase();
                // Rule the candidate out on length before building its char vector: a gap wider
                // than the cap cannot be closed by any number of edits, and counting allocates
                // nothing. Counted on the *lower-cased* form, since lowercasing can change a
                // word's length (`İ` becomes two chars) and the distance is measured there.
                if lower.chars().count().abs_diff(needle.len()) > MAX_PERSONAL_DISTANCE {
                    return None;
                }
                let candidate: Vec<char> = lower.chars().collect();
                bounded_levenshtein(&needle, &candidate, MAX_PERSONAL_DISTANCE)
                    .map(|d| (d, w.clone()))
            })
            .collect();
        scored.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        scored
    }
}

impl SpellChecker {
    /// A checker over `dicts` (primary first) and the Work's `personal` words.
    ///
    /// The application builds this from the dictionaries it has installed and
    /// the project's own word list; the engine never decides either.
    pub fn new(dicts: Vec<Arc<spellbook::Dictionary>>, personal: HashSet<String>) -> Self {
        Self { dicts, personal }
    }

    /// The dictionaries this checker consults, primary first.
    pub fn dictionaries(&self) -> &[Arc<spellbook::Dictionary>] {
        &self.dicts
    }

    /// A checker over one tiny in-memory dictionary plus a personal set.
    ///
    /// Public rather than `#[cfg(test)]`, now that the engine is its own crate:
    /// a `cfg(test)` item is compiled out for every consumer, so the callers
    /// that need it — the application's menu-resolution tests — could not see
    /// it at all. It is a legitimate constructor anyway; the production path
    /// reads installed `.aff`/`.dic` pairs off disk, which a unit test has no
    /// business depending on.
    pub fn from_word_lists(dic_words: &[&str], personal: &[&str]) -> Self {
        let dic = format!("{}\n{}\n", dic_words.len(), dic_words.join("\n"));
        let dict = spellbook::Dictionary::new("SET UTF-8\n", &dic).expect("tiny dictionary parses");
        Self {
            dicts: vec![Arc::new(dict)],
            personal: personal.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// Rank and budget the suggestion sources into the final list.
///
/// Split out from [`SpellChecker::suggest`] as a pure function over its three inputs because the
/// budget cannot be tested through a real dictionary: a synthetic test dictionary has no `TRY`
/// table and so cannot be provoked into the ngram chattiness this exists to defend against, while
/// a real one would drag installed `.aff`/`.dic` files into a unit test.
///
/// `personal` is `(distance, word)` nearest-first; `dict` is pulled **lazily** and only as far as
/// the budget allows, so an unconsumed dictionary's suggester never runs.
pub fn merge_suggestions(
    typed: &str,
    personal: Vec<(usize, String)>,
    mut dict: impl Iterator<Item = String>,
) -> Vec<String> {
    // `near` (<= 1 edit) leads; `far` (2 edits) trails but is guaranteed room.
    let (near, far): (Vec<_>, Vec<_>) = personal.into_iter().partition(|(d, _)| *d <= 1);
    let mut out: Vec<String> = Vec::new();

    // Everything before the reserved slots shares one ceiling — `near` included. `near` is not
    // exempt just because it ranks first: `personal_suggestions` caps distance, not *count*, and
    // a glossary of similar short terms (a Kai / Kal / Kar naming family) can yield more one-edit
    // matches than the whole menu holds, which would push `far` past the end.
    let ceiling = MAX_SUGGESTIONS.saturating_sub(far.len().min(PERSONAL_SUGGESTION_FLOOR));
    for (_, w) in &near {
        if out.len() >= ceiling {
            break;
        }
        push_unique(&mut out, typed, w.clone());
    }
    // Pull only while there is room to keep what comes back: `for s in dict` would fetch one more
    // and discard it, and since the sources are concatenated lazily that wasted pull can cross
    // into the next dictionary and run a whole ngram search for an item this drops on the floor.
    while out.len() < ceiling {
        match dict.next() {
            Some(s) => push_unique(&mut out, typed, s),
            None => break,
        }
    }
    for (_, w) in &far {
        if out.len() >= MAX_SUGGESTIONS {
            break;
        }
        push_unique(&mut out, typed, w.clone());
    }
    // A `far` word that merely repeated something already listed leaves its reserved slot empty —
    // give it back to the dictionary rather than hand back a short menu while suggestions remain.
    while out.len() < MAX_SUGGESTIONS {
        match dict.next() {
            Some(s) => push_unique(&mut out, typed, s),
            None => break,
        }
    }
    // Every push above is gated on a ceiling, so no trailing truncate is needed — adding one back
    // would silently eat the reserved slots.
    out
}

/// Push `s` unless it is the word the writer typed, or an equal suggestion is already there.
///
/// The single gate every source passes through. Deduping keeps the first (better-ranked)
/// occurrence when two dictionaries offer the same correction; rejecting `typed` means no source
/// can echo the input back as its own correction — a menu item that would edit nothing.
pub fn push_unique(out: &mut Vec<String>, typed: &str, s: String) {
    if s != typed && !out.iter().any(|e| e == &s) {
        out.push(s);
    }
}

/// Levenshtein distance between `a` and `b`, or `None` once it is known to exceed `max`.
///
/// The cap is what keeps this cheap enough to run over the whole personal set on a right-click: a
/// survivor bails as soon as every path through a row is already too far, so a candidate costs a
/// few rows rather than a full matrix.
///
/// Takes **already-lower-cased `char` slices** rather than `&str`. Both are the caller's to
/// prepare: the needle is invariant across a scan and would otherwise be re-collected for every
/// candidate, and the cheap length gate belongs *before* a candidate's vector is built, not after
/// — measuring in `char`s, so accented terms count in letters rather than UTF-8 bytes.
pub fn bounded_levenshtein(a: &[char], b: &[char], max: usize) -> Option<usize> {
    if a.len().abs_diff(b.len()) > max {
        return None;
    }
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0usize; b.len() + 1];
    for i in 1..=a.len() {
        cur[0] = i;
        let mut row_min = cur[0];
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            cur[j] = (prev[j] + 1).min(cur[j - 1] + 1).min(prev[j - 1] + cost);
            row_min = row_min.min(cur[j]);
        }
        if row_min > max {
            return None;
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    Some(prev[b.len()]).filter(|d| *d <= max)
}
