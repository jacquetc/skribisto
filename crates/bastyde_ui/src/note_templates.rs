// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The note-template feature's UI pieces: the built-in preset catalogue.
//!
//! The Save-as-template name prompt is not here — it is an `InputDialog` raised straight
//! from `app::commands::templates`, the same way `binder.rename` raises its own. A
//! single-field prompt is exactly what that widget is for, and a hand-built modal beside
//! it would only be a second thing to keep looking like the first.
//!
//! Business logic lives in
//! [`NoteTemplatesViewModel`](crate::view_models::NoteTemplatesViewModel); this module is
//! presentation and the data that is only meaningful to it.
//!
//! Not to be confused with `NewWorkTemplate` (`work_management`), which is a whole *project*
//! skeleton picked in the New Work dialog. These are bodies of prose dropped into a single
//! Note.

pub mod presets;

pub use presets::Preset;
