// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Writing sessions and the writer's self-imposed drafting constraints.
//!
//! [`WritingSessionViewModel`] is the status bar's session timer: elapsed
//! time, a words-written gauge against today's goal, and the countdown/count-up
//! display it derives from them. [`writing_games_vm`] is the games
//! themselves — today, "Always forward" — presented in [`dock`] as a card a
//! writer switches on and off where it is being played, rather than burying it
//! in a preferences window; the Settings page owns the *options*, the dock
//! owns the *playing*.

pub mod writing_games_vm;
mod writing_session_vm;

pub mod dock;

pub use writing_games_vm::{
    FORWARD_PROSE_DEFAULT, FORWARD_SYNOPSIS_DEFAULT, WritingGameOptions, WritingGamesViewModel,
};
pub use writing_session_vm::{
    WritingSessionViewModel, format_mmss, gauge_role, remaining, words_progress,
};
