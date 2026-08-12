// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Book's writing **Pace**: deadline + schedule + progress statistics.
//!
//! [`PaceViewModel`] is the reactive shell — it exists only for a Book container,
//! gated on the same [`crate::models::StreamLevel::for_container`] the stream and
//! the tab use — mirroring a `PaceModel` into `Signal`s and exposing the pure,
//! date-injected arithmetic the module keeps unit-tested and backend-free. The
//! planner, editors and chart views that consume it live under
//! [`crate::tabs::pace`]; [`panel`] is the "where the book stands" summary shown
//! once when a project with an active plan opens.

mod pace_vm;
pub(crate) mod panel;

pub use pace_vm::PaceViewModel;
