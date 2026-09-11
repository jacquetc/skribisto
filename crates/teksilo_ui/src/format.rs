// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Formatting: the single live state behind every surface that shows or
//! applies it — the trailing Format dock, the Format menu, and the four-button
//! row on the editor's right-click menu.
//!
//! [`FormatViewModel`] is single-instance live state: it resolves the current
//! editor on demand through a closure `App` supplies (never caches an
//! `EditorHandle`, which `RichTextEditor::construct()` re-mints on a theme or
//! locale switch), and pushes its mirror signals from a frame tick rather than
//! deriving them, since the editor's own `format_version` fires its observers
//! from inside a borrow. [`dock`] is the reflowing grid view: groups that
//! don't apply to what the caret is in are hidden rather than greyed, and the
//! dock reflows exactly once — by one group — between a scene and its
//! synopsis.

mod format_vm;

pub mod dock;
pub mod link_panel;
pub mod marks;

pub use format_vm::{
    ALIGN_CENTER, ALIGN_LEFT, DIR_AUTO, DIR_LTR, DIR_RTL, EditorKind, FormatSurface,
    FormatViewModel, LinkRequest,
};
