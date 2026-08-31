// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The lexicon trigger matcher, re-exported from where it now lives.
//!
//! Moved to [`skribisto_model::replacement`]: it answers "does this text end in
//! a fired trigger?" from a `&str` and a rule list, which is domain vocabulary,
//! not a property of this application's editor. [`session`](super::session) —
//! deciding *when* to ask and how to apply the answer, against a live
//! `EditorHandle` — stays here, because that part genuinely is a UI concern.

pub use skribisto_model::replacement::*;
