// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What this crate's pure arithmetic must be true of, for every input.
//!
//! Until now the crate had no tests at all, which is defensible for the half of it that is a
//! thin wrapper over `spellbook` and indefensible for the half that is not. Three pieces here
//! are ordinary algorithms with ordinary invariants, and all three are on the path a writer
//! takes hundreds of times a page:
//!
//! * [`word_positions`] hand-rolls a byte-to-character cursor over UAX #29 segments and then
//!   splits each segment again. Its output is `(char offset, char length, &str)`, and the
//!   squiggle is drawn at those offsets — so a slice that does not match the offsets it is
//!   reported at is an underline on the wrong word, or a panic in whoever slices with it.
//! * [`bounded_levenshtein`] is a metric with a bail-out. The bail-out is the part that can be
//!   wrong: it is what makes the function cheap enough to run over the whole personal
//!   dictionary on a right-click, and a bail that fires one row early answers `None` for a
//!   word that was within the cap, so the writer's own term is missing from the menu exactly
//!   when they needed it.
//! * [`merge_suggestions`] budgets several sources into one menu, reserving slots for the
//!   project's own vocabulary. Off-by-one in a budget is a menu that is too long, or one that
//!   drops the entry the reservation existed to protect.
//!
//! # Why the alphabet is small
//!
//! Random Unicode would make every generated pair of words far apart and every generated
//! sentence a list of unique tokens. The interesting inputs are the near-misses and the
//! repeats, so the alphabets here are narrow on purpose, with the non-ASCII characters chosen
//! for what they do to offsets: `é` is two bytes, `日` three, the narrow no-break space is
//! what French typography puts inside guillemets, and `İ` lowercases to *two* code points.

use proptest::prelude::*;
use spellcheck_engine::{
    MAX_SUGGESTIONS, PERSONAL_SUGGESTION_FLOOR, bounded_levenshtein, merge_suggestions,
    push_unique, word_positions,
};

/// Text with the punctuation, spacing and scripts that make segmentation interesting.
fn text_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop::sample::select(vec![
            "a", "b", "cat", " ", "\n", "\t", "'", "\u{2019}", // typographic apostrophe
            "-", ".", "\u{e9}",   // é, two bytes
            "\u{65e5}", // 日, three bytes
            "\u{202f}", // narrow no-break space, inside French guillemets
            "\u{a0}",   // no-break space
            "\u{ab}",   // «
            "\u{bb}",   // »
            "\u{130}",  // İ, lowercases to two code points
            "1", "0",
        ]),
        0..30,
    )
    .prop_map(|parts| parts.concat())
}

/// Short words over a small alphabet, so generated pairs are usually near-misses.
fn word_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop::sample::select(vec!["a", "b", "c", "\u{e9}", "\u{65e5}"]),
        0..7,
    )
    .prop_map(|parts| parts.concat())
}

/// The textbook Levenshtein distance, full matrix, no cap. The oracle.
fn reference_levenshtein(a: &[char], b: &[char]) -> usize {
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0usize; b.len() + 1];
    for i in 1..=a.len() {
        cur[0] = i;
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            cur[j] = (prev[j] + 1).min(cur[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

proptest! {
    /// Every reported word sits exactly where it is reported to sit.
    ///
    /// The offsets are what the squiggle is drawn at and what "add to dictionary" slices with,
    /// so this is the difference between underlining the misspelled word and underlining the
    /// one beside it. `word_positions` walks bytes and characters at once, keeping a running
    /// `char_pos` alongside a `last_byte`, and then splits each segment a second time on
    /// whitespace while tracking three more cursors — six counters that have to agree.
    #[test]
    fn a_reported_word_is_the_text_at_its_offsets(text in text_strategy()) {
        let chars: Vec<char> = text.chars().collect();
        let mut prev_end = 0usize;
        for (start, len, slice) in word_positions(&text) {
            prop_assert!(
                start + len <= chars.len(),
                "word {slice:?} reported at {start}+{len}, past {} characters",
                chars.len()
            );
            let actual: String = chars[start..start + len].iter().collect();
            prop_assert_eq!(
                actual.as_str(),
                slice,
                "reported {:?} but the text at {}..{} holds {:?}",
                slice,
                start,
                start + len,
                actual.as_str()
            );
            prop_assert!(
                start >= prev_end,
                "word {slice:?} at {start} overlaps the previous one, which ended at {prev_end}"
            );
            prop_assert!(len > 0, "an empty word was reported at {start}");
            prop_assert!(
                !slice.chars().any(char::is_whitespace),
                "{slice:?} still holds whitespace, so it is more than one word"
            );
            prev_end = start + len;
        }
    }

    /// The bounded distance agrees with the real one, or correctly declines to answer.
    ///
    /// Two claims, and the second is the one with teeth. Inside the cap the answer must equal
    /// the full matrix — a cheaper algorithm that is merely close is not a distance. Outside
    /// it, `None` must mean genuinely outside: the row-minimum bail is an optimisation, and an
    /// optimisation that fires early is indistinguishable from "no such word" to every caller.
    #[test]
    fn the_bounded_distance_agrees_with_the_real_one(
        a in word_strategy(),
        b in word_strategy(),
        max in 0usize..5,
    ) {
        let av: Vec<char> = a.chars().collect();
        let bv: Vec<char> = b.chars().collect();
        let truth = reference_levenshtein(&av, &bv);

        match bounded_levenshtein(&av, &bv, max) {
            Some(d) => {
                prop_assert_eq!(
                    d, truth,
                    "{:?} vs {:?}: bounded said {}, the matrix says {}",
                    a, b, d, truth
                );
                prop_assert!(d <= max, "{a:?} vs {b:?}: {d} is past the cap of {max}");
            }
            None => prop_assert!(
                truth > max,
                "{a:?} vs {b:?}: declined at cap {max} but the real distance is {truth}"
            ),
        }
    }

    /// Distance is a metric: zero only on equal words, symmetric, and obeying the triangle
    /// inequality.
    ///
    /// Checked through the reference rather than through the bounded function, whose cap makes
    /// it a partial function and so not a metric — the property above is what ties the two
    /// together. Ranking suggestions by a quantity that was not symmetric would offer a
    /// different correction depending on which of the two words the writer happened to type.
    #[test]
    fn the_distance_is_a_metric(a in word_strategy(), b in word_strategy(), c in word_strategy()) {
        let (av, bv, cv): (Vec<char>, Vec<char>, Vec<char>) =
            (a.chars().collect(), b.chars().collect(), c.chars().collect());
        let forward = reference_levenshtein(&av, &bv);
        let backward = reference_levenshtein(&bv, &av);
        let b_to_c = reference_levenshtein(&bv, &cv);
        let a_to_c = reference_levenshtein(&av, &cv);

        prop_assert_eq!(
            forward, backward,
            "distance is not symmetric for {:?} and {:?}",
            a, b
        );
        prop_assert_eq!(forward == 0, a == b, "zero distance without equality, or the reverse");
        prop_assert!(
            a_to_c <= forward + b_to_c,
            "the triangle inequality fails on {a:?}, {b:?}, {c:?}"
        );
    }

    /// The suggestion menu is never too long, never repeats, and never offers the word the
    /// writer typed.
    ///
    /// Three rules that are each one line in `push_unique` and each invisible when broken: a
    /// menu that offers back the typed word is an item that edits nothing, a repeated entry is
    /// two identical rows, and a menu past its budget overflows a fixed-length context menu.
    #[test]
    fn the_suggestion_menu_respects_its_budget(
        typed in word_strategy(),
        personal in prop::collection::vec((0usize..4, word_strategy()), 0..8),
        dict in prop::collection::vec(word_strategy(), 0..12),
    ) {
        let out = merge_suggestions(&typed, personal.clone(), dict.clone().into_iter());

        prop_assert!(
            out.len() <= MAX_SUGGESTIONS,
            "the menu holds {} entries, past the budget of {MAX_SUGGESTIONS}",
            out.len()
        );
        prop_assert!(!out.contains(&typed), "the menu offers back the typed word {typed:?}");
        let mut seen = out.clone();
        seen.sort();
        seen.dedup();
        prop_assert_eq!(seen.len(), out.len(), "the menu repeats an entry: {:?}", out);

        // Every entry came from one of the sources; nothing is invented.
        for entry in &out {
            prop_assert!(
                personal.iter().any(|(_, w)| w == entry) || dict.contains(entry),
                "{entry:?} is in the menu but came from neither source"
            );
        }
    }

    /// A distant personal match keeps its reserved slot.
    ///
    /// The reservation exists because a two-edit typo of a coined word is exactly the word no
    /// installed dictionary knows, which is when Hunspell's ngram search is at its most
    /// talkative — six unrelated guesses, and the one correction the writer wanted appended
    /// after them and then truncated away. So when far matches and dictionary suggestions
    /// compete, at least `PERSONAL_SUGGESTION_FLOOR` of the former survive.
    #[test]
    fn a_distant_personal_match_keeps_its_reserved_slot(
        far in prop::collection::vec(word_strategy(), 1..5),
        dict in prop::collection::vec(word_strategy(), 6..14),
    ) {
        // Distinct, non-empty, and disjoint from the dictionary, so "did it survive" is not
        // confounded by the dedup or by the typed-word filter.
        let far: Vec<String> = far
            .iter()
            .enumerate()
            .map(|(i, w)| format!("far{i}{w}"))
            .collect();
        let dict: Vec<String> = dict
            .iter()
            .enumerate()
            .map(|(i, w)| format!("dic{i}{w}"))
            .collect();
        let personal: Vec<(usize, String)> = far.iter().map(|w| (2usize, w.clone())).collect();

        let out = merge_suggestions("typed", personal, dict.into_iter());
        let kept = out.iter().filter(|s| far.contains(s)).count();
        prop_assert!(
            kept >= far.len().min(PERSONAL_SUGGESTION_FLOOR),
            "only {kept} of {} far personal matches survived a talkative dictionary: {out:?}",
            far.len()
        );
    }

    /// The gate every source passes through admits a word exactly once, and never the typed
    /// one.
    ///
    /// Stated on its own because `merge_suggestions` calls it from four places, so a hole here
    /// would show up as four different-looking symptoms.
    #[test]
    fn the_dedup_gate_admits_a_word_once(
        typed in word_strategy(),
        words in prop::collection::vec(word_strategy(), 0..10),
    ) {
        let mut out: Vec<String> = Vec::new();
        for w in &words {
            push_unique(&mut out, &typed, w.clone());
        }
        prop_assert!(!out.contains(&typed));
        let mut seen = out.clone();
        seen.sort();
        seen.dedup();
        prop_assert_eq!(seen.len(), out.len());
        for w in &words {
            if w != &typed {
                prop_assert!(out.contains(w), "{w:?} was offered and then lost");
            }
        }
    }
}
