// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Analysis segment's business logic: [`AnalysisViewModel`].
//!
//! One per container tab, built for a `Folder/Book` container alongside the stream,
//! Pace, Corkboard and Overview view-models in `ContentTab::new`. It runs
//! `analyze_book`, holds the result, and knows whether that result still describes
//! the manuscript on screen. The category bar, header and per-category views that
//! consume it live under [`crate::tabs::analysis`], which is also the registry for
//! categories contributed from outside this crate.

mod analysis_vm;

pub use analysis_vm::{AnalysisCategory, AnalysisState, AnalysisViewModel};
