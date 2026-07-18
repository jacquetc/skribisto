// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Shared building blocks for the per-combination editor tabs.
//!
//! **Shared means shared.** Everything here is used by more than one tab module or by more
//! than one pane:
//!
//! * [`editor`] — the low-level primitives (writing / synopsis columns, the title field, the
//!   live typography plumbing, the editor style).
//! * [`panes`] — the composite pane renders (heading form, dual-pane prose, no-content
//!   placeholder, folder synopsis) that several `(role, sub_role)` combinations share.
//! * [`stream`] — the manuscript-stream pane the three folder containers share (Full
//!   Chapter / Part / Book, and their Full Synopsis twins).
//! * [`dictionary_menu`] — the spelling half of the editor's context menu, used by
//!   [`editor`].
//!
//! All are re-exported here, so every tab module calls `shared::foo` without caring which
//! file it lives in.
//!
//! The Corkboard and Pace panes used to live here too, at ~1 250 and ~860 lines with a
//! dozen bespoke widgets between them — and exactly **one call site each**. They were whole
//! features filed under "shared" because that is where the tab tree happened to be, so they
//! now sit beside the tab modules that use them, in [`crate::tabs::corkboard`] and
//! [`crate::tabs::pace`].

pub(crate) mod dictionary_menu;
pub(crate) mod editor;
mod panes;
mod stream;

pub use editor::*;
pub use panes::*;
pub use stream::*;
