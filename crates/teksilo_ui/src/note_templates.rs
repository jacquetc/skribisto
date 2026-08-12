// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The note-template feature: business logic and UI pieces.
//!
//! [`NoteTemplatesViewModel`] is the template feature's business logic, shared by the
//! Settings ▸ Work ▸ Templates pane, the Document menu's insert submenu, and the
//! Save-as-template dialog. Everything else here — the built-in preset catalogue — is
//! presentation and the data that is only meaningful to it.
//!
//! The Save-as-template name prompt is not here — it is an `InputDialog` raised straight
//! from `app::commands::templates`, the same way `binder.rename` raises its own. A
//! single-field prompt is exactly what that widget is for, and a hand-built modal beside
//! it would only be a second thing to keep looking like the first.
//!
//! Not to be confused with `NewWorkTemplate` (`work_management`), which is a whole *project*
//! skeleton picked in the New Work dialog. These are bodies of prose dropped into a single
//! Note.

mod note_templates_vm;

pub mod presets;

pub use note_templates_vm::NoteTemplatesViewModel;
pub use presets::Preset;
