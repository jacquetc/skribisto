// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The binder tree feature: [`OutlineViewModel`], its [`dock`], the outline's icons, its
//! placement math, the binder switcher, and the "＋ Create" vocabulary labels.
//!
//! [`OutlineViewModel`] owns the tree's `DockingModel` and selection; [`dock`] is the
//! `DockWidget` built from it. Some of what lives here — [`placement`], the promote/demote
//! guards behind `OutlineViewModel` — is also reached by the corkboard.

mod outline_vm;

pub(crate) mod create_labels;
pub mod dock;
pub(crate) mod icons;
pub(crate) mod placement;
pub(crate) mod switcher_button;

pub use outline_vm::OutlineViewModel;
