// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Mentions: who is named where, across the whole work.
//!
//! [`MentionIndex`] owns the last completed `scan_mentions` result and hands
//! out per-item views of it — one instance, shared app-wide through
//! `app_state`, so every roster and backlink list in the app resolves through
//! the same answer. It is not a view-model (no view of its own): a small
//! app-level service wired once in `App::build`, the same shape as
//! [`crate::shared::ProgressRecorder`].

mod mention_index;
mod presence;

pub use mention_index::{MentionIndex, MentionRow};
pub use presence::confirm as confirm_presence;
