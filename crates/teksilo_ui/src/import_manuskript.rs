// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Import Manuskript feature: pick a Manuskript project and a destination,
//! then convert it into a new `.skrib`.
//!
//! [`ImportManuskriptViewModel`] is the business logic — single-instance live
//! state, registered as `app_state` because a Manuskript import needs no open
//! project at all. [`panel`] is the modal view: thin, binding the view-model's
//! signals and forwarding the footer buttons to its methods.

mod import_manuskript_vm;
pub(crate) mod panel;

pub use import_manuskript_vm::ImportManuskriptViewModel;
