// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The remembered desk: which tabs were open, which docks, and the splitter —
//! restored the way the writer left them.
//!
//! [`WorkspaceLayoutViewModel`] is the single-instance live state behind
//! `workspace.toml` (per-project, keyed by `Work.unique_id`): open tabs keyed
//! by durable `BinderItem.uid` (never a stream ordinal, which doesn't survive
//! a save→load), each tab's caret offset and page scroll, and the
//! `known_docks` roster that tells restore "a dock added since capture" from
//! "the user closed it".

mod workspace_layout_vm;

pub use workspace_layout_vm::WorkspaceLayoutViewModel;
