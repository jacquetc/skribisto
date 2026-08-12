// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The New Work feature: create a work, as a three-step wizard.
//!
//! [`NewWorkViewModel`] is the dialog's business logic — single-instance live state owning
//! the form's `Signal`s, created once per modal session so they survive panel rebuilds. See
//! its own doc for the three presentation contexts (already-open project, Launcher, a window
//! that cannot replace its project in place) that all funnel through
//! [`NewWorkViewModel::create`]. [`panel`] is the [`teksilo::widgets::Stepper`] view: thin,
//! binding the view-model's signals and forwarding the footer to its methods.

mod new_work_vm;
pub(crate) mod panel;

pub use new_work_vm::{NewWorkPurpose, NewWorkViewModel};
