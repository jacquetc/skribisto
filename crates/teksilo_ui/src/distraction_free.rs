// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Distraction-free mode — a **surface of its own**, not a collapsed shell.
//!
//! Shift+F11 slides a panel in over the whole window and parks the project shell
//! behind it, dormant. The panel hosts one document (through the same
//! `tabs::tab_pane` dispatch a pane uses) plus the always-visible control strip.
//!
//! It replaced an in-place chrome collapse, which was good at *hiding* things
//! and silently broken at *substituting* them: the mode's own typography and
//! column width resolve as a pane builds, panes are memoized, and nothing
//! re-read the flag — so a scene opened before pressing Shift+F11 kept its
//! normal typeface and its normal column for the whole session. A separate
//! surface dissolves that: its tab is built once, with the flag a constant.

pub mod quick_settings;
pub mod surface;
pub mod theme;

pub use surface::DistractionFreeSurface;
