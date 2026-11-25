// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Scratchpad: the writer's private notes beside each item. A question to settle
//! later, a list of what the scene still needs, a fact to check: free text that belongs
//! to the item and never to the book. It is never exported, never counted towards a word
//! count, a goal or a pace, never numbered and never a story-bible mention; it is saved
//! with the project, backed up and kept in its history like any other text.
//!
//! The text lives on the item as its `ContentRole::ScratchpadText` row, and the editor over
//! it on the item's shared `OpenDoc`, built lazily (`OpenDocsStore::ensure_scratchpad`).
//! This module is how it is shown: [`ScratchpadViewModel`] decides which item's scratchpad
//! is in front of the writer and holds its document open, and [`dock`] is the panel on the
//! right of the window that shows it.

pub mod dock;
mod scratchpad_vm;

pub use scratchpad_vm::{
    Caret, ScratchpadState, ScratchpadViewModel, is_stream_segment, resolve_target,
};
