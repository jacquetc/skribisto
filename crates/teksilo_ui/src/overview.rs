// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Overview's business logic: [`OverviewViewModel`].
//!
//! One per container tab, built alongside the stream, Pace and Corkboard view-models
//! in `ContentTab::new` and gated on [`skribisto_model::overview_capable`] — a
//! deliberately *wider* gate than the stream's, since a notes folder has no
//! manuscript extent but does have a subtree worth tabulating. It owns the
//! search/sort state, the keyed selection, the inline-edit cursor and the
//! [`crate::models::OverviewRowsModel`] beneath them. The table, header and column
//! views that consume it live under [`crate::tabs::overview`].

mod overview_vm;

pub use overview_vm::{EditBuffer, EditingCell, OverviewViewModel};
