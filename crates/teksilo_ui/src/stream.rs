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
//!
//! Each row is set in by how deep it sits in the outline, a chapter further in than its
//! part and a scene further in than its chapter: [`StreamIndent`] is the setting and
//! [`level_inset`] the rule.

mod indent;
mod stream_vm;

pub use indent::{INDENT_SHARE, StreamIndent, level_inset, row_inset};
pub use stream_vm::{SplitFlavour, StreamViewModel};
