// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The trash feature's own views.
//!
//! Small today — the destination picker for restoring an item whose original spot is gone.
//! It lived at the top level as `restore_target_panel.rs`, where its name and its chrome
//! both read as *backup* restore; it depends only on [`crate::view_models::TrashViewModel`]
//! and the binder tree model, and nothing in backup. The trash dock itself is
//! [`crate::docks::trash`].

pub(crate) mod restore_target_panel;
