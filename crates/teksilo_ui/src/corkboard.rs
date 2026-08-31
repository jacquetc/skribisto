// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Corkboard's business logic: [`CorkboardViewModel`].
//!
//! One per container tab (Chapter/Part/Book), built alongside the stream and Pace
//! view-models in `ContentTab::new` and gated on the same
//! [`StreamLevel::for_container`](crate::models::StreamLevel::for_container). It owns
//! the drilled-into navigation (a folder card drills in *in place*, no new tab), the
//! free search/sort state, and the per-card synopsis documents shared with every
//! other open view of the same item. The grid, header and card views that consume it
//! live under [`crate::tabs::corkboard`].

mod corkboard_vm;

pub use corkboard_vm::{CorkboardViewModel, SORT_TITLE};
