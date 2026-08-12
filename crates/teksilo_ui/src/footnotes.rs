// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Footnotes: creating, editing and navigating the notes that render into the book.
//!
//! [`FootnotesViewModel`] is the feature's business logic: mint a label, insert a
//! reference at the caret, push the marker map to every open document, and keep a
//! per-note body editor. [`dock`] is its one surface — the trailing-rail dock
//! listing every note in reading order, editable in place. Deliberately **not**
//! the comment margin: a comment is a remark about the book, a footnote is part
//! of it.

mod footnotes_vm;

pub mod dock;

pub use footnotes_vm::{FootnoteBinding, FootnoteFilter, FootnotesViewModel};
