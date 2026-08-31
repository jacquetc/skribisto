// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The trash feature: business logic, dock and panel.
//!
//! [`TrashViewModel`] is the dock's business logic — list the trashed roots, restore (in
//! place, with an orphan → destination-picker fallback), restore a single item to a chosen
//! destination, permanently delete an entry, and empty the whole trash. [`dock`] builds the
//! trash panel itself; [`restore_target_panel`] is the destination picker for restoring an
//! item whose original spot is gone. It lived at the top level as `restore_target_panel.rs`,
//! where its name and its chrome both read as *backup* restore; it depends only on
//! [`TrashViewModel`] and the binder tree model, and nothing in backup.

mod trash_vm;

pub mod dock;
pub(crate) mod restore_target_panel;

pub use trash_vm::TrashViewModel;
