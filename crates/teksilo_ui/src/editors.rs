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

//! The tab strip's own context menu lives beside them, one layer up:
//! [`TabMenuViewModel`] owns which rows a tab is offered (as plain data, so the
//! policy is unit-testable), and [`tab_menu`] renders them. `EditorsViewModel`
//! never names either — it receives a [`TabMenuInstaller`] instead, so the
//! dependency runs one way.

mod editors_vm;
pub mod tab_menu;
mod tab_menu_vm;

pub use editors_vm::{EditorsViewModel, Side, TabMenuInstaller};
pub use tab_menu_vm::{TabMenuItem, TabMenuRow, TabMenuViewModel};

/// Fixtures shared by this feature's tests — see [`editors_vm::test_support`].
#[cfg(test)]
pub(crate) use editors_vm::test_support;
