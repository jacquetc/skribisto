// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Replace-while-typing: the per-project custom lexicon applied as the writer
//! types ("btw" + space → "by the way").
//!
//! Two halves, deliberately split so the hard part is testable without a GPU:
//!
//! * [`engine`] — pure matching. Given the text behind the caret, has a rule
//!   fired, and what does it expand to? No widget, no backend, no `#[cfg]`.
//! * [`session`] — the per-document state machine that decides *when* to ask
//!   the engine, applies the answer to the live `TextDocument`, and owns the
//!   backspace-revert.
//!
//! The lexicon itself lives in
//! [`TextReplacementRulesViewModel`](crate::view_models::TextReplacementRulesViewModel);
//! the session recompiles its engine whenever that changes.

pub mod engine;
pub mod session;
pub mod typography;

pub use session::TextReplacementSession;
