// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Skribisto binary.
//!
//! Deliberately one line of work. Everything that was previously `fn main` lives
//! in the library as [`teksilo_ui::run`], because a crate that is only a `[[bin]]`
//! cannot be depended on — and the whole UI half of the extension seam is
//! unreachable until it can be.

// Rust links for the *console* subsystem by default on Windows, so a desktop
// launch drags a terminal window along (and the GPU driver's stderr chatter with
// it). Link for the GUI subsystem instead; terminal output for `--dump-config`
// and argument errors survives via the AttachConsole call at the top of `run()`.
// A subsystem is a per-binary linker flag, never inherited from the library —
// every other binary calling `teksilo_ui::run()` must repeat this line itself.
#![cfg_attr(windows, windows_subsystem = "windows")]

fn main() {
    teksilo_ui::run();
}
