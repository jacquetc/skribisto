// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The one tokenizer every repetition and vocabulary measure in [`crate::analysis`] shares.
//!
//! Echoes, shingles, repeated passages and lexical diversity must agree about what a word
//! *is*, or they contradict each other in the same panel: a repeat the phrase finder reports
//! at four words would be three to the shingler, and a writer meets that as "the numbers
//! don't add up". So there is exactly one definition here and everything reads it.
//!
//! ## Two forms, on purpose
//!
//! A token carries a **normalised key** for comparison and a **byte range** into the source
//! for display. They cannot be collapsed: matching wants `Door` and `door` to be one word,
//! while the panel must quote back what the author actually typed, and navigating to the
//! occurrence needs the real offset. Recovering the range from the key is impossible and
//! recovering the key from the range means re-normalising per lookup.
//!
//! ## Why not bytes
//!
//! Repeated-passage detection over raw UTF-8 bytes finds matches that start mid-word — the
//! classic being `into the door` "repeating" inside `doorway` — and mid-codepoint besides.
//! Every measure here therefore runs over *token* sequences, and [`Vocabulary`] maps those to
//! dense `u32` ids so a suffix array or a shingle hash operates on integers rather than
//! strings.
//!
//! ## What normalisation does and does not do
//!
//! Case is folded (locale-independently — a locale-aware fold would make `I` behave
//! differently in Turkish than in English for the *same* manuscript, which is worse than
//! wrong, it is inconsistent). Diacritics are **kept**: `pêcher` and `pecher` are different
//! words in French, and folding them together would report a repetition that is not one.
//! Morphology is untouched — `door` and `doors` stay distinct. That is a real limit for
//! morphologically rich languages, and it is why the measures built on this report
//! *within-manuscript* comparisons only.

use std::collections::HashMap;
use std::ops::Range;

use unicode_segmentation::UnicodeSegmentation;

/// One word of prose: how it compares, and where it sits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// Case-folded form, used for every comparison.
    pub key: Box<str>,
    /// Byte range in the plain text this was tokenized from, for quoting and navigation.
    pub range: Range<usize>,
}

/// Split plain text into words, in order.
///
/// Expects **plain text** — run Djot through `text_document::djot_to_plain_text` first, or
/// the markup is tokenized as prose (`_emphasis_` becomes the word `emphasis` either way,
/// but a link URL becomes a fistful of nonsense words).
pub fn tokenize(text: &str) -> Vec<Token> {
    text.unicode_word_indices()
        .map(|(at, word)| Token {
            key: word.to_lowercase().into_boxed_str(),
            range: at..at + word.len(),
        })
        .collect()
}

/// A dense `u32` id per distinct word, so the sequence measures work on integers.
///
/// Ids are assigned in first-appearance order, which makes them stable for a given input and
/// therefore makes every measure built on them reproducible — a property the tests rely on
/// and a writer benefits from, since a re-run must not reshuffle a findings list.
#[derive(Debug, Default, Clone)]
pub struct Vocabulary {
    by_key: HashMap<Box<str>, u32>,
    keys: Vec<Box<str>>,
    /// Occurrences of each id across everything interned so far, for surprisal scoring.
    counts: Vec<u32>,
    total: u64,
}

/// A word's dense id within one [`Vocabulary`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WordId(pub u32);

impl Vocabulary {
    pub fn new() -> Self {
        Self::default()
    }

    /// The id for `key`, assigning a new one if this is its first appearance, and counting
    /// the occurrence.
    pub fn intern(&mut self, key: &str) -> WordId {
        let id = match self.by_key.get(key) {
            Some(&id) => id,
            None => {
                let id = self.keys.len() as u32;
                let boxed: Box<str> = key.into();
                self.by_key.insert(boxed.clone(), id);
                self.keys.push(boxed);
                self.counts.push(0);
                id
            }
        };
        self.counts[id as usize] += 1;
        self.total += 1;
        WordId(id)
    }

    /// The id for `key` if it has been seen, without interning or counting it.
    pub fn lookup(&self, key: &str) -> Option<WordId> {
        self.by_key.get(key).copied().map(WordId)
    }

    /// The word behind an id.
    pub fn word(&self, id: WordId) -> &str {
        &self.keys[id.0 as usize]
    }

    /// How many distinct words have been interned.
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Total occurrences interned, across all words.
    pub fn total_occurrences(&self) -> u64 {
        self.total
    }

    /// Occurrences of one word.
    pub fn count(&self, id: WordId) -> u32 {
        self.counts[id.0 as usize]
    }

    /// How surprising this word is **in this manuscript**, as `-ln(p)`.
    ///
    /// This is what separates a repetition worth reading from an idiom. `the` recurring
    /// forty times is the language working; `carnelian` recurring twice in a paragraph is
    /// the writer repeating themselves. Ranking by surprisal rather than by raw frequency or
    /// match length is the whole precision mechanism — without it a repeated-phrase report
    /// is a list of `and then he` and nothing else.
    ///
    /// Measured against the manuscript's own distribution rather than an external word list,
    /// so it needs no per-language resource and adapts to a text whose ordinary vocabulary
    /// *is* unusual (a nautical novel's `halyard` is not a surprise).
    pub fn surprisal(&self, id: WordId) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        let p = f64::from(self.count(id)) / self.total as f64;
        -p.ln()
    }
}

/// Tokenize `text` and intern every word, returning the id sequence.
///
/// The companion [`Token`] ranges are returned alongside because a caller that finds
/// something in the id sequence needs to point back at the prose.
pub fn intern_text(vocab: &mut Vocabulary, text: &str) -> (Vec<WordId>, Vec<Token>) {
    let tokens = tokenize(text);
    let ids = tokens.iter().map(|t| vocab.intern(&t.key)).collect();
    (ids, tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(text: &str) -> Vec<String> {
        tokenize(text).into_iter().map(|t| t.key.into_string()).collect()
    }

    #[test]
    fn words_are_split_and_case_folded() {
        assert_eq!(keys("The Door closed."), ["the", "door", "closed"]);
    }

    #[test]
    fn punctuation_is_not_a_word() {
        assert_eq!(keys("Wait — what? Yes!"), ["wait", "what", "yes"]);
    }

    /// French elision is one orthographic word to UAX #29's word rule, and splitting it
    /// would make `l'homme` and `homme` different words in one measure and the same in
    /// another.
    #[test]
    fn french_elision_and_apostrophes_hold_together() {
        assert_eq!(keys("L'homme s'arrêta."), ["l'homme", "s'arrêta"]);
    }

    /// Diacritics distinguish real words in French; folding them would invent repetitions.
    #[test]
    fn diacritics_are_kept_distinct() {
        assert_ne!(keys("pêcher")[0], keys("pecher")[0]);
    }

    /// German ß lower-cases to itself; uppercasing instead would produce `SS` and change the
    /// token's length, which would break every range this module hands back.
    #[test]
    fn eszett_survives_folding() {
        assert_eq!(keys("Straße STRASSE"), ["straße", "strasse"]);
        assert_ne!(keys("Straße")[0], keys("STRASSE")[0], "these are different spellings");
    }

    #[test]
    fn ranges_point_at_the_original_text() {
        let text = "Elle ouvrit la porte.";
        let toks = tokenize(text);
        assert_eq!(&text[toks[3].range.clone()], "porte");
    }

    #[test]
    fn ranges_are_byte_correct_through_multibyte_text() {
        let text = "Ééé porte";
        let toks = tokenize(text);
        assert_eq!(&text[toks[1].range.clone()], "porte");
    }

    #[test]
    fn ids_are_assigned_in_first_appearance_order() {
        let mut v = Vocabulary::new();
        let (ids, _) = intern_text(&mut v, "b a b c");
        assert_eq!(ids, [WordId(0), WordId(1), WordId(0), WordId(2)]);
        assert_eq!(v.word(WordId(0)), "b");
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn counts_and_totals_track_every_occurrence() {
        let mut v = Vocabulary::new();
        intern_text(&mut v, "the cat the hat the end");
        let the = v.lookup("the").unwrap();
        assert_eq!(v.count(the), 3);
        assert_eq!(v.total_occurrences(), 6);
    }

    #[test]
    fn lookup_does_not_intern_or_count() {
        let mut v = Vocabulary::new();
        intern_text(&mut v, "solo");
        assert!(v.lookup("absent").is_none());
        assert_eq!(v.len(), 1, "a failed lookup must not create a word");
        assert_eq!(v.total_occurrences(), 1, "nor count one");
    }

    /// The precision mechanism: a common word must score below a rare one, so ranking by
    /// surprisal buries the idioms a raw-frequency ranking would surface.
    #[test]
    fn a_rare_word_is_more_surprising_than_a_common_one() {
        let mut v = Vocabulary::new();
        intern_text(&mut v, "the the the the the the the the carnelian the");
        let common = v.surprisal(v.lookup("the").unwrap());
        let rare = v.surprisal(v.lookup("carnelian").unwrap());
        assert!(rare > common, "carnelian ({rare}) must out-score the ({common})");
    }

    #[test]
    fn an_empty_vocabulary_has_no_surprisal_to_report() {
        let v = Vocabulary::new();
        assert_eq!(v.total_occurrences(), 0);
        assert!(v.is_empty());
    }
}
