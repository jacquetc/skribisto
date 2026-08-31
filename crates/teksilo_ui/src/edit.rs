// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **Edit** — one Undo for the whole application.
//!
//! Skribisto has two undo engines and always will: `text-document` keeps a
//! word-level history per open document, and Qleany's `UndoRedoManager` keeps
//! the project's structural history. They are the right granularity for what
//! each records, and merging them into one list would be a mistake — see
//! [`undo_group_vm`] for why. What was missing is an **arbiter**: something that
//! answers "what does Ctrl+Z mean *here*", says so before the writer presses it,
//! and routes the command.
//!
//! That is [`UndoGroupViewModel`], modelled on Qt's `QUndoGroup`: several
//! domains, one active, and a menu that mirrors it. Tier 3 — per window, because
//! "where is the caret" is a property of a window and two windows on one project
//! can have it in different places.

pub mod domains;
pub mod undo_group_vm;
pub(crate) mod undo_labels;

pub use domains::EntityDomain;
pub(crate) use undo_group_vm::UndoTarget;
pub use undo_group_vm::{UndoClaim, UndoGroupViewModel, UndoSuspend};
pub(crate) use undo_labels::target_text;

#[cfg(test)]
mod tests;
