// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The window/process shell: what a window *is*, and how instances find each other.
//!
//! Skribisto runs **one process per project**, so "open that project" is often "spawn or
//! raise another instance" rather than anything in-window. [`open_registry`] is the lock-file
//! directory listing what is open across instances, [`ipc`] the per-instance socket that
//! receives a raise request, [`process`] the spawn itself, and [`project_switcher_button`]
//! the title-bar control that ties them together. [`windows`] builds the two window kinds
//! (Launcher and project).

pub(crate) mod ipc;
pub(crate) mod open_registry;
pub(crate) mod process;
pub(crate) mod project_switcher_button;
pub(crate) mod windows;
