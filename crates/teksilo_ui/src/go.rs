// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Go: the six Next/Previous × Scene/Chapter/Note targets, and the per-window
//! availability that lights their menu rows.
//!
//! [`GoAvailability`] is per-window live state — the six rows' live "is there
//! a target" mirrors, minted fresh per window alongside `scene_focused` so a
//! second project window's Go menu never reflects the wrong window's focused
//! item. [`GoToViewModel`] resolves and performs one jump, reused by both the
//! Go menu and the status-bar "go to" button.

mod go_to_vm;
mod go_vm;

pub use go_to_vm::GoToViewModel;
pub use go_vm::GoAvailability;
