// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Help ▸ Report a Translation Error…
//!
//! A writer who meets a wrong string finds it by the words they saw, says what is
//! wrong and how they would put it, and continues to a prefilled public issue on the
//! project's tracker, in their own browser. The application itself sends nothing.
//!
//! - [`catalogue`]: every translated string the app ships, with its file and line.
//! - [`display`]: what a catalogue line shows on screen, for the search to match.
//! - [`matching`]: finding the line a writer saw, from the words they type.
//! - [`report`]: what the report says, and the link that carries it.
//! - [`translation_report_vm`]: the window's state and actions.
//! - [`window`] and [`panel`]: the window, opened through one door.
//!
//! The design, its rejected alternatives and the measurements it rests on (what
//! GitHub accepts in a link, what each platform's launcher passes on) are recorded
//! with the brainstorm of 2026-11-16.

pub(crate) mod catalogue;
pub(crate) mod display;
pub(crate) mod matching;
pub mod panel;
pub(crate) mod report;
pub mod translation_report_vm;
pub mod window;
