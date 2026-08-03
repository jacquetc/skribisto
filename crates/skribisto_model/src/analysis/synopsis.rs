// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Does the scene do what its synopsis says it does?
//!
//! Skribisto is the only tool that can ask this cheaply, because every writing row owns a
//! synopsis *and* its prose as first-class sibling fields — nothing has to be re-entered by
//! hand, and nothing has to be inferred from formatting.
//!
//! ## Why not similarity
//!
//! The reflex implementation — cosine or Jaccard between the synopsis and the prose — does
//! not work, and fails in a way that looks like it is working. A two-sentence synopsis
//! against a 2,400-word scene shares almost nothing by construction, so *every* scene scores
//! near zero and an accurate synopsis is indistinguishable from an abandoned one. It is the
//! same trap as raw type-token ratio: a number dominated by length, wearing the costume of
//! meaning.
//!
//! ## What is measured instead
//!
//! **Coverage, in one direction**: of the things the synopsis says this scene is about, how
//! many turn up in the prose? A synopsis promising "Constantine confronts the Senate about
//! the embargo" is checked for `Constantine`, `Senate` and `embargo` — not for the shape of
//! the sentence around them.
//!
//! Two term sources, in priority order:
//!
//! 1. **Named entities the caller resolved** — characters, places and objects from the story
//!    bible, already matched by the mention scanner with its folding, word boundaries and
//!    aliases. These are the high-signal terms, and using them means a name that declines or
//!    takes a possessive still counts.
//! 2. **Distinctive content words**, ranked by manuscript-wide surprisal, as the fallback for
//!    a project with no story bible. Weaker, but it degrades rather than disappearing.
//!
//! Weighting by surprisal is what stops the measure being about `the` and `went`.
//!
//! ## What the caller must do with it
//!
//! A coverage figure is only meaningful against the distribution of the *other* scenes in the
//! same book — some writers' synopses are terse, some are near-outlines, and neither is
//! wrong. [`drift_outliers`] does that comparison; the raw figure is never a verdict on its
//! own.

use super::stats;
use super::tokens::{Vocabulary, tokenize};

/// How well one scene's prose covers what its synopsis promised.
#[derive(Debug, Clone, PartialEq)]
pub struct Coverage {
    /// Surprisal-weighted share of the synopsis's terms found in the prose, `0.0..=1.0`.
    pub covered: f64,
    /// The synopsis terms that did **not** turn up. These are what the UI shows — "your
    /// synopsis mentions the locket; the prose does not" is actionable in a way a score is
    /// not.
    pub missing: Vec<String>,
    /// How many terms were weighed at all. A synopsis of "he leaves" yields almost none, and
    /// a coverage figure over one term is not a measurement.
    pub terms_weighed: usize,
    pub synopsis_words: usize,
    pub prose_words: usize,
}

impl Coverage {
    /// Synopsis length ÷ prose length. Near 1.0 means the "synopsis" is the scene (a
    /// summary that was never dramatised); very near 0 on a long scene means the outline
    /// never caught up with the draft.
    pub fn length_ratio(&self) -> Option<f64> {
        (self.prose_words > 0).then(|| self.synopsis_words as f64 / self.prose_words as f64)
    }
}

/// Terms below this surprisal are the language, not the content, and are never weighed.
/// Roughly: a word occurring more often than about one time in 400.
const MIN_TERM_SURPRISAL: f64 = 6.0;

/// Fewer weighed terms than this and coverage is not reportable — the caller must show
/// nothing rather than a number derived from one word.
pub const MIN_TERMS: usize = 3;

/// How far below the book's mean coverage a scene must sit before it is worth reporting,
/// regardless of how few standard deviations that happens to be.
///
/// This is the guard against a consistent book manufacturing outliers: covering 0.88 where
/// the rest cover 0.90 is not drift, however many sigmas a tight distribution makes it.
pub const MIN_ABSOLUTE_SHORTFALL: f64 = 0.10;

/// Measure one scene.
///
/// `entity_terms` are story-bible names the caller already resolved for this scene's
/// synopsis (see the mention scanner); pass an empty slice when there is no story bible and
/// the distinctive-word fallback will carry it. `vocab` must be the manuscript-wide
/// vocabulary, or surprisal means nothing.
pub fn coverage(
    synopsis: &str,
    prose: &str,
    entity_terms: &[String],
    vocab: &Vocabulary,
) -> Coverage {
    let synopsis_tokens = tokenize(synopsis);
    let prose_tokens = tokenize(prose);

    let prose_keys: std::collections::HashSet<&str> =
        prose_tokens.iter().map(|t| &*t.key).collect();

    // Weighed terms: resolved entity names first, then distinctive content words from the
    // synopsis that are not already covered by an entity term.
    let mut weighed: Vec<(String, f64, bool)> = Vec::new();

    for name in entity_terms {
        let key = name.to_lowercase();
        // An entity is worth more than an ordinary word: the caller resolved it against the
        // story bible, so its presence or absence is a fact rather than an inference.
        let found = tokenize(&key).iter().all(|t| prose_keys.contains(&*t.key));
        weighed.push((name.clone(), ENTITY_WEIGHT, found));
    }

    // The *words* of every entity name, so a synopsis token already covered by an entity
    // is not weighed twice. Word-level and not a substring scan: testing
    // `"constantine".contains(token)` would silently swallow an unrelated token like "tine",
    // and costs O(name length) per token besides.
    let already: std::collections::HashSet<Box<str>> = entity_terms
        .iter()
        .flat_map(|n| tokenize(&n.to_lowercase()).into_iter().map(|t| t.key))
        .collect();

    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for t in &synopsis_tokens {
        if already.contains(&t.key) || !seen.insert(&t.key) {
            continue;
        }
        let Some(id) = vocab.lookup(&t.key) else { continue };
        let s = vocab.surprisal(id);
        if s < MIN_TERM_SURPRISAL {
            continue;
        }
        weighed.push((t.key.to_string(), s, prose_keys.contains(&*t.key)));
    }

    let total: f64 = weighed.iter().map(|(_, w, _)| w).sum();
    let hit: f64 = weighed.iter().filter(|(_, _, f)| *f).map(|(_, w, _)| w).sum();

    Coverage {
        covered: if total > 0.0 { hit / total } else { 0.0 },
        missing: weighed
            .iter()
            .filter(|(_, _, found)| !found)
            .map(|(term, _, _)| term.clone())
            .collect(),
        terms_weighed: weighed.len(),
        synopsis_words: synopsis_tokens.len(),
        prose_words: prose_tokens.len(),
    }
}

/// A resolved story-bible name counts for as much as a fairly surprising word. Deliberately
/// not overwhelming: a synopsis is allowed to name a character the scene refers to only by
/// pronoun, and that should lower coverage a little, not condemn the scene.
const ENTITY_WEIGHT: f64 = 8.0;

/// One scene's place in the book's own distribution of coverage.
#[derive(Debug, Clone, PartialEq)]
pub struct DriftOutlier {
    /// Index into the slice handed to [`drift_outliers`].
    pub index: usize,
    pub covered: f64,
    pub missing: Vec<String>,
}

/// The scenes whose synopsis tracks their prose noticeably less well than the rest of this
/// book's do.
///
/// Self-referential on purpose. Terse outliners and near-outline writers both exist, and an
/// absolute threshold would flag one style wholesale. A scene is reported when it falls more
/// than `sigmas` population standard deviations below the book's own mean coverage.
///
/// Scenes with too few weighed terms are skipped entirely rather than counted as zero, which
/// would drag the mean down and make everything else look fine by comparison.
///
/// A sigma test alone is not enough, and the failure is not hypothetical: a book whose scenes
/// all cover 0.88–0.91 has a standard deviation near 0.01, so the 0.88 sits a statistically
/// impeccable 1.5σ below the mean and gets reported — as drift, in a book with none. Being
/// unusual for this book is necessary but not sufficient; a reported scene must also be
/// **meaningfully** worse in absolute terms, which is what [`MIN_ABSOLUTE_SHORTFALL`] adds.
pub fn drift_outliers(scenes: &[Coverage], sigmas: f64) -> Vec<DriftOutlier> {
    let usable: Vec<(usize, &Coverage)> = scenes
        .iter()
        .enumerate()
        .filter(|(_, c)| c.terms_weighed >= MIN_TERMS)
        .collect();
    if usable.len() < 3 {
        return Vec::new();
    }

    let covered: Vec<f64> = usable.iter().map(|(_, c)| c.covered).collect();
    let (Some(mean), Some(sd)) = (stats::mean(&covered), stats::population_stddev(&covered))
    else {
        return Vec::new();
    };
    if sd <= f64::EPSILON {
        return Vec::new();
    }

    let floor = (mean - sigmas * sd).min(mean - MIN_ABSOLUTE_SHORTFALL);
    let mut out: Vec<DriftOutlier> = usable
        .into_iter()
        .filter(|(_, c)| c.covered < floor)
        .map(|(index, c)| DriftOutlier {
            index,
            covered: c.covered,
            missing: c.missing.clone(),
        })
        .collect();
    out.sort_by(|a, b| a.covered.partial_cmp(&b.covered).unwrap_or(std::cmp::Ordering::Equal));
    out
}

#[cfg(test)]
mod tests {
    use super::super::tokens::intern_text;
    use super::*;

    /// A manuscript-wide vocabulary where the distinctive words really are rare.
    fn manuscript(extra: &str) -> Vocabulary {
        let mut v = Vocabulary::new();
        // Enough ordinary prose that function words are unsurprising.
        for _ in 0..40 {
            intern_text(&mut v, "the and of to a in he she it was that with for on at by");
        }
        intern_text(&mut v, extra);
        v
    }

    #[test]
    fn a_synopsis_whose_terms_all_appear_is_fully_covered() {
        let v = manuscript("constantine senate embargo");
        let c = coverage(
            "Constantine confronts the Senate about the embargo.",
            "Constantine rose. The Senate fell silent. She named the embargo and sat down.",
            &[],
            &v,
        );
        assert!(c.covered > 0.99, "got {c:?}");
        assert!(c.missing.is_empty());
    }

    #[test]
    fn a_missing_term_is_named_not_just_scored() {
        let v = manuscript("constantine senate embargo locket");
        let c = coverage(
            "Constantine confronts the Senate about the locket.",
            "Constantine rose. The Senate fell silent. She said nothing more.",
            &[],
            &v,
        );
        assert!(c.missing.iter().any(|m| m == "locket"), "got {c:?}");
        assert!(c.covered < 1.0);
    }

    /// The whole point of surprisal weighting: an accurate synopsis must not be penalised
    /// for the ordinary words it is made of.
    #[test]
    fn function_words_are_not_weighed() {
        let v = manuscript("carnelian");
        let c = coverage("He was in the room with it.", "The carnelian light fell.", &[], &v);
        assert_eq!(c.terms_weighed, 0, "nothing distinctive was promised: {c:?}");
    }

    #[test]
    fn a_resolved_entity_outweighs_an_ordinary_word() {
        let v = manuscript("constantine embargo");
        let present = coverage(
            "Constantine and the embargo.",
            "Constantine spoke of the embargo.",
            &["Constantine".to_string()],
            &v,
        );
        let absent = coverage(
            "Constantine and the embargo.",
            "The embargo was mentioned by nobody in particular.",
            &["Constantine".to_string()],
            &v,
        );
        assert!(present.covered > absent.covered);
        assert!(absent.missing.iter().any(|m| m == "Constantine"), "got {absent:?}");
    }

    /// The skip is word-level, not a substring scan: an entity called "Constantine" must
    /// not swallow an unrelated synopsis word like "tine" that merely substring-matches it.
    #[test]
    fn an_entity_name_does_not_swallow_an_unrelated_substring_of_itself() {
        let v = manuscript("constantine tine");
        let c = coverage(
            "Constantine and the tine.",
            "Constantine spoke at length about nothing in particular.",
            &["Constantine".to_string()],
            &v,
        );
        assert!(
            c.missing.iter().any(|m| m == "tine"),
            "`tine` is its own term, not part of `Constantine`; got {c:?}"
        );
    }

    /// The skip itself must still work: a word that really is part of an entity's name is
    /// weighed once, through the entity, not twice.
    #[test]
    fn an_entity_names_own_words_are_not_weighed_twice() {
        let v = manuscript("constantine");
        let c = coverage(
            "Constantine arrives.",
            "Constantine arrived.",
            &["Constantine".to_string()],
            &v,
        );
        assert_eq!(c.terms_weighed, 1, "one entity, not an entity plus its own word: {c:?}");
    }

    #[test]
    fn coverage_works_without_a_story_bible() {
        let v = manuscript("carnelian halyard");
        let c = coverage("The carnelian and the halyard.", "He touched the carnelian.", &[], &v);
        assert!(c.terms_weighed >= 2, "the fallback must still weigh something: {c:?}");
        assert!(c.missing.iter().any(|m| m == "halyard"));
    }

    #[test]
    fn length_ratio_reports_a_summary_that_was_never_dramatised() {
        let v = manuscript("");
        let c = coverage("He left and never came back at all", "He left and never came back", &[], &v);
        let r = c.length_ratio().unwrap();
        assert!(r > 0.9, "near-equal lengths mean the synopsis IS the scene: {r}");
    }

    #[test]
    fn length_ratio_is_absent_rather_than_infinite_for_empty_prose() {
        let v = manuscript("");
        assert_eq!(coverage("Something happens.", "", &[], &v).length_ratio(), None);
    }

    // ── outliers ──

    fn cov(covered: f64, terms: usize) -> Coverage {
        Coverage {
            covered,
            missing: vec!["thing".into()],
            terms_weighed: terms,
            synopsis_words: 10,
            prose_words: 1000,
        }
    }

    #[test]
    fn the_one_drifted_scene_in_a_consistent_book_is_found() {
        let scenes = vec![
            cov(0.90, 5),
            cov(0.88, 5),
            cov(0.92, 5),
            cov(0.91, 5),
            cov(0.10, 5),
            cov(0.89, 5),
        ];
        let out = drift_outliers(&scenes, 1.5);
        assert_eq!(out.len(), 1, "got {out:?}");
        assert_eq!(out[0].index, 4);
    }

    /// A writer whose synopses are uniformly terse is not drifting, and must not be told
    /// they are. This is what the self-referential baseline buys.
    #[test]
    fn a_uniformly_low_book_reports_nothing() {
        let scenes = vec![cov(0.2, 5), cov(0.21, 5), cov(0.19, 5), cov(0.2, 5), cov(0.22, 5)];
        assert!(drift_outliers(&scenes, 1.5).is_empty(), "consistent is not drifted");
    }

    #[test]
    fn scenes_with_too_few_terms_are_skipped_not_scored_zero() {
        let scenes = vec![
            cov(0.9, 5),
            cov(0.88, 5),
            cov(0.91, 5),
            cov(0.0, 1), // one weighed term — unmeasurable, not drifted
            cov(0.9, 5),
        ];
        let out = drift_outliers(&scenes, 1.5);
        assert!(
            out.iter().all(|o| o.index != 3),
            "an unmeasurable scene must not be reported as drifted: {out:?}"
        );
    }

    /// A sigma test alone would flag this: a ~0.011 standard deviation makes a 0.02
    /// shortfall look like 1.5σ, but a book this consistent has no drift in it.
    #[test]
    fn a_tight_distribution_does_not_manufacture_an_outlier() {
        let scenes = vec![cov(0.9, 5), cov(0.88, 5), cov(0.91, 5), cov(0.9, 5), cov(0.89, 5)];
        let out = drift_outliers(&scenes, 1.5);
        assert!(out.is_empty(), "0.88 among 0.88-0.91 is not drift: {out:?}");
    }

    /// The guard must not silence a genuine outlier, only a trivial one.
    #[test]
    fn a_real_shortfall_still_reports_even_in_a_tight_book() {
        let scenes = vec![cov(0.9, 5), cov(0.9, 5), cov(0.91, 5), cov(0.89, 5), cov(0.35, 5)];
        let out = drift_outliers(&scenes, 1.5);
        assert_eq!(out.len(), 1, "got {out:?}");
        assert_eq!(out[0].index, 4);
    }

    #[test]
    fn too_few_scenes_to_have_a_distribution_reports_nothing() {
        assert!(drift_outliers(&[cov(0.9, 5), cov(0.1, 5)], 1.5).is_empty());
    }

    #[test]
    fn outliers_come_back_worst_first() {
        let scenes = vec![
            cov(0.9, 5),
            cov(0.9, 5),
            cov(0.9, 5),
            cov(0.9, 5),
            cov(0.3, 5),
            cov(0.1, 5),
        ];
        let out = drift_outliers(&scenes, 1.0);
        assert!(out.len() >= 2, "got {out:?}");
        assert!(out[0].covered <= out[1].covered);
    }
}
