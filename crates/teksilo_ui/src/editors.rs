// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editors: the split editor's live state — two panes of open tabs (a primary
//! and a secondary/side pane) sharing one `OpenDocsStore`, plus which pane is
//! focused.
//!
//! [`EditorsViewModel`] is single-instance per window (`App` creates exactly
//! one and shares it by clone) and is the most reached-into view-model in the
//! crate — open/close, focus tracking, Go, save, comments and formatting all
//! resolve their target document through it. [`Side`] is which of the two
//! panes (`Primary` / `Secondary`).

mod editors_vm;

pub use editors_vm::{EditorsViewModel, Side};
