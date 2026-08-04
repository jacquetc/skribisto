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
//! **Rank by surprisal, not by frequency or length.** The recurring failure of repetition
//! reports elsewhere is that they surface `and then he` forever. Scoring against the
//! manuscript's own word distribution is the whole precision mechanism, and it needs no
//! per-language stopword list to work.
//!
//! ## Layout
//!
//! - [`tokens`](crate::analysis::tokens) — the one tokenizer and word-id vocabulary everything else shares.
//! - [`prose_stats`](crate::analysis::prose_stats) — sentence/paragraph lengths, punctuation density, dialogue share.
//! - [`lexical`](crate::analysis::lexical) — length-robust vocabulary diversity (MATTR, HD-D).
//! - [`repetition`](crate::analysis::repetition) — echoes within a scene, near-duplicate scenes across a book.
//! - [`synopsis`](crate::analysis::synopsis) — whether a scene does what its synopsis says it does.
//! - [`stats`](crate::analysis::stats) — the mean/stddev/median every one of the above shares.

pub mod lexical;
pub mod prose_stats;
pub mod repetition;
pub mod stats;
pub mod synopsis;
pub mod tokens;

pub use lexical::Diversity;
pub use prose_stats::{DialogueMarkers, ProseStats};
pub use repetition::{Echo, SceneSimilarity, ShingleSet};
pub use synopsis::{Coverage, DriftOutlier};
pub use tokens::{Token, Vocabulary, WordId};
