// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Manuscript analysis — the pure measurements behind the Analysis view.
//!
//! Everything here reads strings and ids and returns numbers. No store, no entities, no
//! threads, no UI: the same posture as [`crate::mentions`] and [`crate::counting`], and for
//! the same reason — these are the pieces that must be testable headlessly and reusable from
//! a long operation, a headless test and a future CLI alike.
//!
//! ## The house rules these measures obey
//!
//! **Compare a manuscript only to itself.** Not one figure here is scored against an external
//! corpus, a genre norm or a target value. There is no correct sentence length, dialogue
//! ratio or vocabulary richness, and a tool that implies otherwise is wrong about writing
//! rather than merely unhelpful. Every threshold is derived from the book's own distribution.
//!
//! **Say "not measurable" rather than zero.** A language with no dialogue convention, a scene
//! too short for a diversity index, a synopsis with no distinctive terms — each returns
//! `None`, never a `0.0` that reads as a finding. This is the difference between a tool that
//! is quiet where it is ignorant and one that is confidently wrong.
//!
//! **Rank by surprisal, not by frequency or length.** Scoring a word against the
//! manuscript's own distribution — rather than against a frequency count or a per-language
//! stopword list — is what keeps `and then he` out of a report. `tokens::Vocabulary`
//! carries that distribution, and it is the reason this module is a shared vocabulary
//! rather than three unrelated measures.
//!
//! ## Layout
//!
//! - `tokens` — the one tokenizer and word-id vocabulary everything else shares.
//! - [`prose_stats`](crate::analysis::prose_stats) — sentence/paragraph lengths, punctuation density, dialogue share.
//! - `stats` — the mean/stddev/median the measures share.
//!
//! ## What is deliberately *not* here
//!
//! Vocabulary diversity, within-scene echoes, near-duplicate scenes and synopsis coverage
//! were all measured here once, for the Analysis view's Repetition, Synopsis and Voice
//! categories. Those categories are no longer part of this application, so their measures
//! are not either — a measure with no reader is dead weight in every build and a cost on
//! every analysis run.
//!
//! What stays is what [`crate::analysis`]'s remaining reader needs, plus the two pieces any
//! further measure would build on: `tokens` (the shared tokenizer and surprisal-bearing
//! vocabulary) and `stats`. Both are public and both are complete — an out-of-tree
//! measure interns against the same vocabulary the built-in one does, so two readers of the
//! same manuscript cannot disagree about what a word is.

pub mod prose_stats;
pub mod stats;
pub mod tokens;

pub use prose_stats::{DialogueMarkers, ProseStats};
pub use tokens::{Token, Vocabulary, WordId};
