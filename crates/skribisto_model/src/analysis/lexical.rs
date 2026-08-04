// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Lexical diversity — how varied the word choice is, measured so that length does not decide
//! the answer.
//!
//! ## Why not type-token ratio
//!
//! The obvious measure, distinct words ÷ total words, is unusable for comparing texts of
//! different lengths — and comparing is the only thing anyone wants it for. Heaps' law makes
//! vocabulary grow as `V ≈ K·N^β` with β roughly 0.4–0.6 for prose, so TTR ≈ `K·N^(β−1)`
//! *falls* as a text gets longer, smoothly and predictably. Put a 40,000-word point of view
//! beside a 12,000-word one and the longer scores lower every time, whether or not the two
//! characters draw on remotely similar vocabularies. The number would be reporting length
//! wearing the costume of style.
//!
//! ## What is used instead
//!
//! [`mattr`] — the moving-average type-token ratio: TTR of a fixed-size window, averaged over
//! every window position. Because every window is the same size, the length dependence is
//! gone by construction, and what remains is comparable across texts. It is O(N) with a
//! rolling count, needs no reference corpus and no per-language resource.
//!
//! McCarthy & Jarvis (2010) recommend never reporting a lone diversity index, so [`hdd`] is
//! offered alongside: the hypergeometric distribution measure, which asks how likely each
//! word is to appear in a random 42-token draw. It fails differently from MATTR — HD-D is
//! sensitive to the rare tail, MATTR to local variety — so agreement between them means more
//! than either alone.
//!
//! ## The limit that stays
//!
//! Both measure *surface forms*. In a morphologically rich language — Polish, Czech, Finnish,
//! German compounds — inflection inflates apparent variety, so a German text will score above
//! an English one for reasons that have nothing to do with the writer. Every figure here is
//! therefore only ever compared **within one manuscript and one language**, which is a
//! property of how the caller presents it, not something this module can enforce.

use std::collections::HashMap;

use super::tokens::WordId;

/// Windows shorter than this cannot produce a stable ratio, and a text shorter than one
/// window has no MATTR at all.
pub const DEFAULT_WINDOW: usize = 50;

/// Below this many words a diversity figure is noise, and the UI should say "not enough text
/// yet" rather than print it.
pub const MIN_RELIABLE_WORDS: usize = 500;

/// Moving-average type-token ratio over `window`-sized windows.
///
/// `None` when the text is shorter than one window — the honest answer, since a partial
/// window would silently be a raw TTR again, which is the very thing this avoids.
pub fn mattr(ids: &[WordId], window: usize) -> Option<f64> {
    if window == 0 || ids.len() < window {
        return None;
    }

    // Rolling multiset: distinct-count is maintained incrementally, so the whole pass is O(N)
    // rather than O(N·window).
    let mut seen: HashMap<WordId, usize> = HashMap::new();
    for id in &ids[..window] {
        *seen.entry(*id).or_insert(0) += 1;
    }
    let mut sum = seen.len() as f64;
    let mut windows = 1usize;

    for i in window..ids.len() {
        let leaving = ids[i - window];
        if let Some(c) = seen.get_mut(&leaving) {
            *c -= 1;
            if *c == 0 {
                seen.remove(&leaving);
            }
        }
        *seen.entry(ids[i]).or_insert(0) += 1;
        sum += seen.len() as f64;
        windows += 1;
    }

    Some(sum / windows as f64 / window as f64)
}

/// The sample size HD-D draws, fixed by convention at 42 tokens (McCarthy & Jarvis).
const HDD_SAMPLE: usize = 42;

/// HD-D: the expected contribution of each word type to a random `HDD_SAMPLE`-token draw.
///
/// `None` when the text is shorter than the sample — the hypergeometric is undefined there.
pub fn hdd(ids: &[WordId]) -> Option<f64> {
    let n = ids.len();
    if n < HDD_SAMPLE {
        return None;
    }

    let mut counts: HashMap<WordId, usize> = HashMap::new();
    for id in ids {
        *counts.entry(*id).or_insert(0) += 1;
    }

    // For each type: P(appears at least once in a draw of HDD_SAMPLE)
    //              = 1 − C(n − k, s) / C(n, s), with k its occurrences.
    // Computed as a product of ratios so nothing overflows on a novel-sized n.
    let mut total = 0.0;
    for &k in counts.values() {
        let absent = if n - k < HDD_SAMPLE {
            0.0
        } else {
            let mut p = 1.0f64;
            for i in 0..HDD_SAMPLE {
                p *= (n - k - i) as f64 / (n - i) as f64;
            }
            p
        };
        total += (1.0 - absent) / HDD_SAMPLE as f64;
    }
    Some(total)
}

/// Raw type-token ratio.
///
/// Provided because it is what a reader expects to see reported, **not** as a comparison
/// figure — see the module docs. Present it beside its word count or not at all.
pub fn ttr(ids: &[WordId]) -> Option<f64> {
    if ids.is_empty() {
        return None;
    }
    let distinct: std::collections::HashSet<WordId> = ids.iter().copied().collect();
    Some(distinct.len() as f64 / ids.len() as f64)
}

/// Everything a caller needs to render one pool's diversity honestly: the figures, and the
/// sample size that says how much to trust them.
#[derive(Debug, Clone, PartialEq)]
pub struct Diversity {
    pub words: usize,
    pub distinct_words: usize,
    pub mattr: Option<f64>,
    pub hdd: Option<f64>,
    /// Whether the pool clears [`MIN_RELIABLE_WORDS`]. The UI must show "not enough text yet"
    /// rather than a number when this is false — a diversity figure over 80 words is a
    /// coin toss dressed as a measurement.
    pub reliable: bool,
}

/// Measure one pool of text (a scene, a chapter, everything one point of view narrates).
pub fn measure(ids: &[WordId]) -> Diversity {
    let distinct: std::collections::HashSet<WordId> = ids.iter().copied().collect();
    Diversity {
        words: ids.len(),
        distinct_words: distinct.len(),
        mattr: mattr(ids, DEFAULT_WINDOW),
        hdd: hdd(ids),
        reliable: ids.len() >= MIN_RELIABLE_WORDS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `n` words drawn round-robin from `distinct` types — variety is controlled exactly.
    fn cycle(distinct: u32, n: usize) -> Vec<WordId> {
        (0..n).map(|i| WordId(i as u32 % distinct)).collect()
    }

    #[test]
    fn a_text_shorter_than_one_window_has_no_mattr() {
        assert_eq!(mattr(&cycle(5, 49), 50), None);
        assert!(mattr(&cycle(5, 50), 50).is_some());
    }

    #[test]
    fn all_distinct_words_score_one_and_all_identical_score_the_floor() {
        let all_new: Vec<WordId> = (0..100).map(WordId).collect();
        assert_eq!(mattr(&all_new, 50), Some(1.0));

        let all_same = vec![WordId(0); 100];
        assert_eq!(mattr(&all_same, 50), Some(1.0 / 50.0));
    }

    /// The property that motivates the whole module: doubling the text must not move the
    /// score, where a raw TTR would fall.
    #[test]
    fn mattr_does_not_depend_on_length_but_ttr_does() {
        let short = cycle(30, 1_000);
        let long = cycle(30, 8_000);

        let m_short = mattr(&short, DEFAULT_WINDOW).unwrap();
        let m_long = mattr(&long, DEFAULT_WINDOW).unwrap();
        assert!(
            (m_short - m_long).abs() < 1e-9,
            "MATTR must be length-independent: {m_short} vs {m_long}"
        );

        let t_short = ttr(&short).unwrap();
        let t_long = ttr(&long).unwrap();
        assert!(
            t_long < t_short / 2.0,
            "raw TTR collapses with length, which is why it is not the headline: {t_short} vs {t_long}"
        );
    }

    #[test]
    fn more_variety_scores_higher() {
        let dull = cycle(5, 2_000);
        let varied = cycle(200, 2_000);
        assert!(mattr(&varied, DEFAULT_WINDOW) > mattr(&dull, DEFAULT_WINDOW));
    }

    #[test]
    fn hdd_needs_at_least_a_full_sample() {
        assert_eq!(hdd(&cycle(5, 41)), None);
        assert!(hdd(&cycle(5, 42)).is_some());
    }

    #[test]
    fn hdd_ranks_variety_the_same_way_mattr_does() {
        let dull = hdd(&cycle(5, 2_000)).unwrap();
        let varied = hdd(&cycle(200, 2_000)).unwrap();
        assert!(
            varied > dull,
            "HD-D must agree on direction: {varied} vs {dull}"
        );
    }

    #[test]
    fn hdd_stays_in_range_on_real_shaped_input() {
        let d = hdd(&cycle(60, 3_000)).unwrap();
        assert!((0.0..=1.0).contains(&d), "HD-D out of range: {d}");
    }

    #[test]
    fn a_short_pool_is_flagged_unreliable_but_still_measured() {
        let d = measure(&cycle(20, 120));
        assert!(!d.reliable, "120 words cannot support a diversity claim");
        assert_eq!(d.words, 120);
        assert!(
            d.mattr.is_some(),
            "the figure still exists; the caller decides to show it"
        );
    }

    #[test]
    fn a_long_pool_is_reliable() {
        assert!(measure(&cycle(80, MIN_RELIABLE_WORDS)).reliable);
    }

    #[test]
    fn an_empty_pool_measures_as_nothing() {
        let d = measure(&[]);
        assert_eq!(d.words, 0);
        assert_eq!(d.distinct_words, 0);
        assert_eq!(d.mattr, None);
        assert_eq!(d.hdd, None);
        assert!(!d.reliable);
        assert_eq!(ttr(&[]), None);
    }
}
