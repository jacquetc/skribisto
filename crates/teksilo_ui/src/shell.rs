// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The window/process shell: what a window *is*, and how instances find each other.
//!
//! Skribisto is **single-instance**: the first live copy wins an election and becomes the
//! primary, and every later launch forwards its command line to it and exits, so "open that
//! project" is a new *window* in one process rather than a new process. [`instance`] runs
//! that election and the handoff, [`ipc`] carries the protocol and serves both sockets
//! (the primary one and this instance's own per-pid one), [`open_registry`] is the lock-file
//! directory listing what is open across whatever instances do exist, [`process`] holds the
//! path helpers left over from the multi-process era, and [`project_switcher_button`] is the
//! title-bar control that ties them together. [`windows`] builds the two window kinds
//! (Launcher and project), each with its own menu model ([`launcher_menu`],
//! [`project_menus`]).
//!
//! Several processes are still reachable and still supported — `--new-instance` asks for one
//! outright, and a wedged primary degrades to one — which is why the open registry and the
//! per-pid socket remain rather than being folded away.

/// Logical-pixel height of every window's title bar — the project windows and the
/// Launcher alike, so the two never disagree by a couple of pixels.
///
/// Below Teksilo's own 40 dp default: this app's chrome is a thin strip that has to
/// stay out of the writer's way, and 40 read as heavy next to the content. The floor
/// is set by what the bar *contains*, since [`teksilo::widgets::TitleBar`] reports its
/// configured height and lets a taller child overflow rather than growing:
/// the window-control cells are 32 dp, and the leading/trailing icon buttons must
/// therefore stay at [`IconButtonSize::Toolbar`](teksilo::widgets::IconButtonSize)
/// (30 dp) — `Large` is 40 and would spill out of the strip.
pub(crate) const TITLE_BAR_HEIGHT: f32 = 34.0;

pub(crate) mod first_run_window;
pub(crate) mod instance;
pub(crate) mod ipc;
pub(crate) mod launcher_menu;
pub(crate) mod launcher_window;
pub(crate) mod open_registry;
pub(crate) mod process;
pub(crate) mod project_menus;
pub(crate) mod project_switcher_button;
pub(crate) mod window_ids;
pub(crate) mod windows;

// Path→window identity lives in `window_ids` and is re-exported from `windows`
// so existing `shell::windows::window_id_for` call sites keep compiling.
