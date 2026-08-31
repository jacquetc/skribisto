// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The project lifecycle: loading/creating/closing a `Work`, and every guard
//! that keeps a writer from losing an edit while doing it.
//!
//! [`ProjectLifecycleViewModel`] drives Open/New/Close themselves.
//! [`ProjectSwitchViewModel`] is the unsaved-changes guard every in-place
//! project switch — New Work, Open Work, the switcher's "Open here", the
//! import toast's "Open now" — must pass, parked behind an in-flight save when
//! one is running. [`project_switcher`] is the pure functions behind the
//! project-switcher popover (which open projects to list, and what raising one
//! does). [`open_failure_toast`] is the "couldn't open" toast text, shared by
//! every load path. [`QuitSequencer`] walks every open Work through the same
//! unsaved-changes guard, waits for each save's sequence, takes each Work's
//! on-close backup, then closes every window — cancel anywhere aborts the
//! whole quit.

mod open_failure;
mod project_lifecycle_vm;
mod project_switch_vm;
pub mod project_switcher;
mod quit_sequencer_vm;

pub use open_failure::open_failure_toast;
pub use project_lifecycle_vm::ProjectLifecycleViewModel;
pub(crate) use project_lifecycle_vm::reload_personal_words;
pub use project_switch_vm::{
    PendingSwitch, ProjectSwitchViewModel, UnsavedDecision, unsaved_decision,
};
pub use quit_sequencer_vm::QuitSequencer;
