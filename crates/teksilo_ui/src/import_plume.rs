// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Import Plume Creator feature: pick a `.plume`/`.plume_backup` project and a
//! destination, then convert it into a new `.skrib`.
//!
//! [`ImportPlumeViewModel`] is the business logic — single-instance live state,
//! registered as `app_state` because a Plume import needs no open project at all. [`panel`]
//! is the modal view: thin, binding the view-model's signals and forwarding the footer
//! buttons to its methods.

mod import_plume_vm;
pub(crate) mod panel;

pub use import_plume_vm::ImportPlumeViewModel;
