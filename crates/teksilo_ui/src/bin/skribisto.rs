// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Skribisto binary.
//!
//! Deliberately one line of work. Everything that was previously `fn main` lives
//! in the library as [`teksilo_ui::run`], because a crate that is only a `[[bin]]`
//! cannot be depended on — and the whole UI half of the extension seam is
//! unreachable until it can be.

fn main() {
    teksilo_ui::run();
}
