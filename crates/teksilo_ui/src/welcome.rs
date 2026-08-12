// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Welcome feature: the Launcher window's start-screen content.
//!
//! [`WelcomeViewModel`] is the business logic — open a recent/example work, pick a file,
//! create a new work, and the recents search (the query signal, the filtered projection,
//! and the cursor that follows). [`panel`] is the view: [`panel::WelcomePanel`] hosts it as
//! the Launcher window's root (see [`crate::shell::windows::launcher_window_config`]), not a
//! modal.

mod welcome_vm;

pub mod panel;

pub use welcome_vm::{DISCORD_URL, GITHUB_URL, WelcomeViewModel};
