// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! [`StreamViewModel`] — business logic for a container tab's **manuscript
//! streams**: the Full Chapter / Full Part / Full Book view and its Full Synopsis
//! twin ([`SplitFlavour`] names which one).
//!
//! One instance per open container tab, created by `ContentTab::new` alongside the
//! Corkboard/Overview/Pace view-models — it owns the container's ordered row list
//! and the row mutations (rename, set label, insert / add / move / merge / split /
//! trash). The pane that consumes it lives at
//! [`crate::tabs::shared::stream_pane`].

mod stream_vm;

pub use stream_vm::{SplitFlavour, StreamViewModel};
