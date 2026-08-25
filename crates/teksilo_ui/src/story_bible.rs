// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The story-bible creation pipeline: one modal, one transaction, three doors.
//!
//! A name, a location, tags, aliases, a template and a small body, gathered in
//! local state and committed as a single undo step only when **Create** is
//! pressed; nothing lands before that. The same pipeline is reached three
//! ways, and all three end up here:
//!
//! - the outline's ＋ Create vocabulary (`CreateType::StoryBibleEntry`), which
//!   creates its row immediately, exactly like every sibling type, then opens
//!   this module's *configuration* step on the row that now exists;
//! - "Add as note", on any text selection in any prose editor, which opens the
//!   *creation* step pre-filled from the selection; and
//! - (a future door, not built yet) a "+ New entry" button on the Story bible
//!   place itself.
//!
//! `create` holds the pure transaction (create the item, its content, and,
//! for the configuration path, its tags/aliases/books, as one composite);
//! `infer_book` holds the two ways a fresh entry's `books` pre-set is guessed,
//! never written until the writer confirms it; `modal` is the widget.

pub mod create;
pub mod infer_book;
pub mod modal;
