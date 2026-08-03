// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Repetition, at two scales: a distinctive word coming back too soon, and two scenes that
//! say the same thing.
//!
//! ## Echoes are the common flaw
//!
//! The rare accident is pasting a paragraph twice. The *ordinary* craft slip is the echo —
//! `glanced` twice in three sentences, `suddenly` three times on a page — and it is what a
//! writer actually wants flagged. It is also far cheaper to find than a longest-repeated
//! substring, which is why [`echoes`] leads here.
//!
//! The precision mechanism is [surprisal](super::tokens::Vocabulary::surprisal), not
//! frequency: `the` recurring inside a window is the language working, `carnelian` recurring
//! is the writer repeating themselves. Ranked by raw count instead, an echo report is a list
//! of function words and nothing else — which is exactly how the feature fails in tools that
//! ship it that way.
//!
//! ## Near-duplicate scenes need containment, not Jaccard
//!
//! Jaccard is the reflex measure and the wrong one here. A 500-word scene reproduced whole
//! inside a 3,000-word scene scores |A∩B| / |A∪B| ≈ 0.17 and reads as unrelated — while the
//! writer's actual situation, "I expanded this scene and forgot to delete the old one", is a
//! *containment* of 1.0. So [`compare_scenes`] leads with containment
//! (|A∩B| / min(|A|,|B|), Broder's containment coefficient) and reports Jaccard beside it as
//! the overall-similarity column.
//!
//! At realistic sizes the naive all-pairs comparison is right: a 500-scene project is 125k
//! pairs of a few thousand sorted `u64`s, well under a second. MinHash/LSH exists to dodge
//! quadratic cost at N in the thousands and would trade exactness for nothing here.

use std::collections::HashMap;

use super::tokens::{Token, Vocabulary, WordId};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Echoes
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// How far apart two uses of a word can be and still read as an echo. Roughly a page of
/// prose — beyond it a reader has stopped hearing the first use.
pub const DEFAULT_ECHO_WINDOW: usize = 250;

/// A word repeating inside the window.
#[derive(Debug, Clone, PartialEq)]
pub struct Echo {
    pub word: String,
    /// Byte ranges of each occurrence in the scene's plain text, in order.
    pub occurrences: Vec<std::ops::Range<usize>>,
    /// Closest gap in words between two of the occurrences.
    pub closest_gap: usize,
    /// Ranking score: surprisal of the word, scaled by how tightly it recurs. Higher means
    /// more worth a look.
    pub score: f64,
}

/// Find words repeating within `window` words of themselves.
///
/// `vocab` supplies the manuscript-wide surprisal that ranks the results, so it must be the
/// vocabulary the *whole* manuscript was interned into — one built from this scene alone
/// would call every word equally surprising and rank by nothing.
pub fn echoes(
    ids: &[WordId],
    tokens: &[Token],
    vocab: &Vocabulary,
    window: usize,
    min_surprisal: f64,
) -> Vec<Echo> {
    // A text shorter than the window cannot be judged by it: every pair of occurrences is
    // inside the window by construction, so the measure degenerates into "every word used
    // twice" — noisiest on the shortest texts (a dedication, a copyright page).
    if ids.len() < window {
        return Vec::new();
    }
    let mut positions: HashMap<WordId, Vec<usize>> = HashMap::new();
    for (i, id) in ids.iter().enumerate() {
        positions.entry(*id).or_default().push(i);
    }

    let mut out = Vec::new();
    for (id, at) in positions {
        if at.len() < 2 {
            continue;
        }
        let surprisal = vocab.surprisal(id);
        if surprisal < min_surprisal {
            continue;
        }
        // Only the occurrences that actually participate in a close pair are reported —
        // a word used twice at opposite ends of a long scene is not an echo.
        let Some(closest_gap) = at.windows(2).map(|w| w[1] - w[0]).filter(|&g| g <= window).min()
        else {
            continue;
        };
        let mut involved: Vec<usize> = Vec::new();
        for pair in at.windows(2) {
            if pair[1] - pair[0] <= window {
                if involved.last() != Some(&pair[0]) {
                    involved.push(pair[0]);
                }
                involved.push(pair[1]);
            }
        }

        // Tighter recurrence reads louder, so the gap scales the surprisal rather than
        // merely filtering it.
        let tightness = (window as f64 / closest_gap.max(1) as f64).sqrt();
        out.push(Echo {
            word: vocab.word(id).to_string(),
            occurrences: involved.iter().map(|&i| tokens[i].range.clone()).collect(),
            closest_gap,
            score: surprisal * tightness,
        });
    }

    // Descending score, then the word itself, so a re-run never reshuffles equal-scoring rows.
    out.sort_by(|a, b| {
        b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.word.cmp(&b.word))
    });
    out
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Near-duplicate scenes
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Words per shingle. Five is long enough that ordinary phrasing does not collide and short
/// enough to survive the light editing a duplicated scene usually receives.
pub const DEFAULT_SHINGLE: usize = 5;

/// Below this many distinct shingles a scene is too short to compare — two three-word scenes
/// are trivially "contained" in each other and would top every report.
pub const MIN_SHINGLES: usize = 30;

/// One scene's shingle set: sorted, deduplicated hashes, ready for a merge intersection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShingleSet {
    hashes: Vec<u64>,
}

impl ShingleSet {
    /// Build from a scene's interned word ids.
    pub fn new(ids: &[WordId], k: usize) -> Self {
        let mut hashes: Vec<u64> = if k == 0 || ids.len() < k {
            Vec::new()
        } else {
            ids.windows(k).map(hash_shingle).collect()
        };
        hashes.sort_unstable();
        hashes.dedup();
        Self { hashes }
    }

    pub fn len(&self) -> usize {
        self.hashes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hashes.is_empty()
    }

    /// Shingles present in both, by a two-pointer merge over the sorted sets.
    pub fn intersection_size(&self, other: &Self) -> usize {
        let (mut i, mut j, mut n) = (0, 0, 0);
        while i < self.hashes.len() && j < other.hashes.len() {
            match self.hashes[i].cmp(&other.hashes[j]) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j += 1,
                std::cmp::Ordering::Equal => {
                    n += 1;
                    i += 1;
                    j += 1;
                }
            }
        }
        n
    }
}

/// One `u64` per shingle, via the same `DefaultHasher` [`crate::mentions::fingerprint_alias_table`]
/// already uses to fold a compound value into a comparison key.
///
/// `DefaultHasher::new()` is fixed-key and therefore stable across processes — unlike
/// `RandomState`, which is what `HashMap::new()` seeds randomly. That distinction matters
/// here: a findings list that reshuffled between runs would be unreproducible.
fn hash_shingle(window: &[WordId]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    window.hash(&mut h);
    h.finish()
}

/// How alike two scenes are.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SceneSimilarity {
    /// |A∩B| / min(|A|,|B|) — "one of these is inside the other". The alert metric.
    pub containment: f64,
    /// |A∩B| / |A∪B| — overall resemblance. The context column.
    pub jaccard: f64,
    pub shared_shingles: usize,
}

/// Compare two scenes' shingle sets.
///
/// `None` when either is too short to compare — saying nothing beats a confident 1.0 on two
/// one-line scenes.
pub fn compare_scenes(a: &ShingleSet, b: &ShingleSet) -> Option<SceneSimilarity> {
    if a.len() < MIN_SHINGLES || b.len() < MIN_SHINGLES {
        return None;
    }
    let shared = a.intersection_size(b);
    let union = a.len() + b.len() - shared;
    Some(SceneSimilarity {
        containment: shared as f64 / a.len().min(b.len()) as f64,
        jaccard: if union == 0 { 0.0 } else { shared as f64 / union as f64 },
        shared_shingles: shared,
    })
}

#[cfg(test)]
mod tests {
    use super::super::tokens::intern_text;
    use super::*;

    fn prep(text: &str) -> (Vec<WordId>, Vec<Token>, Vocabulary) {
        let mut v = Vocabulary::new();
        let (ids, toks) = intern_text(&mut v, text);
        (ids, toks, v)
    }

    // ── echoes ──

    #[test]
    fn a_repeated_distinctive_word_is_an_echo() {
        let (ids, toks, v) = prep("She glanced away. He glanced back.");
        let found = echoes(&ids, &toks, &v, 5, 0.0);
        assert!(found.iter().any(|e| e.word == "glanced"), "got {found:?}");
    }

    #[test]
    fn a_word_used_once_is_not_an_echo() {
        let (ids, toks, v) = prep("She glanced away and said nothing at all.");
        assert!(echoes(&ids, &toks, &v, 5, 0.0).is_empty());
    }

    /// The precision mechanism in action: with a surprisal floor, function words drop out
    /// and only the distinctive repeat survives.
    #[test]
    fn common_words_rank_below_rare_ones_and_can_be_filtered_out() {
        let text = "the cat sat on the mat and the dog sat on the log \
                    while the carnelian light fell on the carnelian stone";
        let (ids, toks, v) = prep(text);

        let all = echoes(&ids, &toks, &v, 15, 0.0);
        let top = &all[0];
        assert_eq!(top.word, "carnelian", "the rare repeat must rank first, got {all:?}");

        let the = v.lookup("the").unwrap();
        let filtered = echoes(&ids, &toks, &v, 15, v.surprisal(the) + 0.01);
        assert!(
            filtered.iter().all(|e| e.word != "the"),
            "a surprisal floor must drop function words: {filtered:?}"
        );
    }

    /// A text shorter than the window has no echoes: every pair of occurrences in one is
    /// inside the window, so the measure stops saying anything.
    #[test]
    fn a_text_shorter_than_the_window_has_no_echoes() {
        let (ids, toks, v) = prep("She glanced away. He glanced back.");
        assert!(
            echoes(&ids, &toks, &v, 250, 0.0).is_empty(),
            "a 7-word text cannot be judged by a 250-word window"
        );
        // …and the same text is judged normally once the window fits inside it.
        assert!(echoes(&ids, &toks, &v, 5, 0.0).iter().any(|e| e.word == "glanced"));
    }

    #[test]
    fn a_repeat_beyond_the_window_is_not_an_echo() {
        let mut text = String::from("glanced ");
        text.push_str(&"filler ".repeat(300));
        text.push_str("glanced");
        let (ids, toks, v) = prep(&text);
        assert!(
            echoes(&ids, &toks, &v, 250, 0.0).iter().all(|e| e.word != "glanced"),
            "300 words apart is not an echo at a 250-word window"
        );
    }

    #[test]
    fn a_tighter_echo_outranks_a_looser_one() {
        let mut text = String::from("alpha alpha ");
        text.push_str("beta ");
        text.push_str(&"pad ".repeat(200));
        text.push_str("beta ");
        // Trailing filler only so the text clears the window — the gates under test are the
        // two gaps above it, which it leaves alone.
        text.push_str(&"pad ".repeat(100));
        let (ids, toks, v) = prep(&text);
        let found = echoes(&ids, &toks, &v, 250, 0.0);
        let alpha = found.iter().find(|e| e.word == "alpha").unwrap();
        let beta = found.iter().find(|e| e.word == "beta").unwrap();
        assert!(alpha.score > beta.score, "adjacent must beat distant: {alpha:?} vs {beta:?}");
        assert_eq!(alpha.closest_gap, 1);
    }

    #[test]
    fn echo_occurrences_point_back_at_the_prose() {
        let text = "She glanced away. He glanced back.";
        let (ids, toks, v) = prep(text);
        let e = echoes(&ids, &toks, &v, 5, 0.0).into_iter().find(|e| e.word == "glanced").unwrap();
        assert_eq!(e.occurrences.len(), 2);
        for r in &e.occurrences {
            assert_eq!(&text[r.clone()], "glanced");
        }
    }

    #[test]
    fn results_are_ordered_deterministically() {
        let (ids, toks, v) = prep("alpha alpha beta beta gamma gamma");
        let a = echoes(&ids, &toks, &v, 250, 0.0);
        let b = echoes(&ids, &toks, &v, 250, 0.0);
        assert_eq!(a, b, "a re-run must not reshuffle the list");
    }

    // ── near-duplicate scenes ──

    fn shingles_of(v: &mut Vocabulary, text: &str) -> ShingleSet {
        let (ids, _) = intern_text(v, text);
        ShingleSet::new(&ids, DEFAULT_SHINGLE)
    }

    /// `n` words of distinct filler, so a scene clears MIN_SHINGLES without accidental repeats.
    fn filler(tag: &str, n: usize) -> String {
        (0..n).map(|i| format!("{tag}{i}")).collect::<Vec<_>>().join(" ")
    }

    #[test]
    fn an_identical_scene_is_fully_contained_and_fully_similar() {
        let mut v = Vocabulary::new();
        let text = filler("w", 100);
        let a = shingles_of(&mut v, &text);
        let b = shingles_of(&mut v, &text);
        let s = compare_scenes(&a, &b).unwrap();
        assert!((s.containment - 1.0).abs() < 1e-9);
        assert!((s.jaccard - 1.0).abs() < 1e-9);
    }

    #[test]
    fn unrelated_scenes_share_nothing() {
        let mut v = Vocabulary::new();
        let a = shingles_of(&mut v, &filler("a", 100));
        let b = shingles_of(&mut v, &filler("b", 100));
        let s = compare_scenes(&a, &b).unwrap();
        assert_eq!(s.shared_shingles, 0);
        assert_eq!(s.containment, 0.0);
    }

    /// The case Jaccard gets wrong and containment gets right — the whole reason this
    /// module leads with containment.
    #[test]
    fn a_short_scene_swallowed_by_a_long_one_reads_as_contained_not_as_unrelated() {
        let mut v = Vocabulary::new();
        let short_text = filler("s", 100);
        let long_text = format!("{} {}", short_text, filler("x", 500));

        let short = shingles_of(&mut v, &short_text);
        let long = shingles_of(&mut v, &long_text);
        let s = compare_scenes(&short, &long).unwrap();

        assert!(s.containment > 0.95, "containment must catch it: {s:?}");
        assert!(
            s.jaccard < 0.25,
            "and Jaccard must be shown to miss it, which is why it is not the alert: {s:?}"
        );
    }

    #[test]
    fn scenes_too_short_to_judge_are_not_judged() {
        let mut v = Vocabulary::new();
        let a = shingles_of(&mut v, "far too short to compare");
        let b = shingles_of(&mut v, "far too short to compare");
        assert_eq!(compare_scenes(&a, &b), None, "two tiny identical scenes must not top the list");
    }

    #[test]
    fn a_scene_shorter_than_one_shingle_has_no_shingles() {
        let mut v = Vocabulary::new();
        assert!(shingles_of(&mut v, "one two three").is_empty());
    }

    #[test]
    fn shingle_hashing_is_stable_across_runs() {
        let mut v1 = Vocabulary::new();
        let mut v2 = Vocabulary::new();
        let text = filler("w", 100);
        assert_eq!(
            shingles_of(&mut v1, &text),
            shingles_of(&mut v2, &text),
            "hashes must not depend on a per-process random seed"
        );
    }

    #[test]
    fn intersection_is_symmetric() {
        let mut v = Vocabulary::new();
        let a = shingles_of(&mut v, &format!("{} {}", filler("p", 60), filler("q", 60)));
        let b = shingles_of(&mut v, &format!("{} {}", filler("q", 60), filler("r", 60)));
        assert_eq!(a.intersection_size(&b), b.intersection_size(&a));
    }
}
