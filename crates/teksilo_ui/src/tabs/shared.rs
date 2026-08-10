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
//! file it lives in. Corkboard, Pace and Analysis are whole features with one call site
//! each, so they live beside the tab modules that use them ([`crate::tabs::corkboard`],
//! [`crate::tabs::pace`], [`crate::tabs::analysis`]), not here.

pub(crate) mod charts;
pub(crate) mod dictionary_menu;
pub(crate) mod editor;
mod panes;
/// The `container.segments` extension slot and the stable ids the bar is keyed by.
pub mod segments;
mod stream;

pub use charts::*;
pub use editor::*;
pub use panes::*;
// No `pub use segments::*` — `segments` is already a `pub mod`, so a glob adds
// nothing and warns as an unused import. Reach it by path
// (`tabs::shared::segments::…`), which is also how an extension names the slot.
pub use stream::*;
