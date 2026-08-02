// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Dock widgets placed on the app's `DockingLayout`. Each dock owns its own
//! content builder and packages it as a `DockWidget` for `App` to mount on a
//! side; `App` only wires the cross-view-model effects around them.
//!
//! Currently the sole dock is the binder [`outline`]; further container docks
//! (corkboard, search results, …) will land here beside it.
//!
//! ## Stable dock ids
//!
//! `DockLayoutState` (bastyde's serialisable dock layout) keys panes by raw
//! `DockWidgetId` (`u64`). For the per-work layout restore
//! ([`crate::view_models::WorkspaceLayoutViewModel`]) to match a saved layout to
//! this run's docks, each logical dock must carry the **same** id every launch —
//! a `DockWidgetId::fresh()` (a per-process atomic counter) would mint a
//! different id each run and `import_state` would drop the whole saved tree as
//! "unknown". So the four app docks use these fixed ids instead of `fresh()`.
//!
//! The base is deliberately high so it can never collide with a `fresh()` id
//! (the framework mints those from `1`, e.g. for a user-dragged dock split).

/// Base for the fixed app-dock ids (see the module docs). Chosen well above any
/// `DockWidgetId::fresh()` value the process could reach.
const DOCK_ID_BASE: u64 = 0xD0C_0000;
/// The binder outline (leading rail).
pub const OUTLINE_DOCK_ID: u64 = DOCK_ID_BASE + 1;
/// Search & replace (leading rail, second tab).
pub const SEARCH_DOCK_ID: u64 = DOCK_ID_BASE + 2;
/// The context inspector (trailing rail).
pub const INSPECTOR_DOCK_ID: u64 = DOCK_ID_BASE + 3;
/// The bottom search-preview band.
pub const PREVIEW_DOCK_ID: u64 = DOCK_ID_BASE + 4;
/// The trash panel (leading rail, third tab).
pub const TRASH_DOCK_ID: u64 = DOCK_ID_BASE + 5;
/// The trailing Format dock — the manuscript's formatting controls.
pub const FORMAT_DOCK_ID: u64 = DOCK_ID_BASE + 6;
/// Project-wide comments (leading rail, fourth tab).
pub const COMMENTS_DOCK_ID: u64 = DOCK_ID_BASE + 7;
/// This document's comments (trailing rail, third tab).
pub const DOC_COMMENTS_DOCK_ID: u64 = DOCK_ID_BASE + 8;

pub mod create_split_button;
pub mod comments;
pub mod format;
pub mod inspector;
pub mod outline;
pub mod search;
pub mod search_preview;
pub mod search_replace_flow;
pub mod trash;
