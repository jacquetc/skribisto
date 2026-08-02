// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The note-template feature's UI pieces: the built-in preset catalogue, and the
//! save-as-template dialog.
//!
//! Business logic lives in
//! [`NoteTemplatesViewModel`](crate::view_models::NoteTemplatesViewModel); this module is
//! presentation and the data that is only meaningful to it.
//!
//! Not to be confused with `NewWorkTemplate` (`work_management`), which is a whole *project*
//! skeleton picked in the New Work dialog. These are bodies of prose dropped into a single
//! Note.

pub mod presets;
pub mod save_as_dialog;

pub use presets::Preset;
pub use save_as_dialog::SaveAsTemplatePanel;
