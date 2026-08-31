// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The search feature: business logic, plus the two docks and the in-editor find banner.
//!
//! Two view-models: [`SearchReplaceViewModel`] is the single shared handle behind both
//! project-wide docks — the leading query/options/result-list dock and the bottom editable
//! preview — and behind Replace All; [`FindViewModel`] is the per-editor find banner (Ctrl+F),
//! one per prose tab, driven by the same locale-aware matcher so the two features can never
//! disagree about what a match is. [`dock`]/[`preview_dock`] are the two docks' views;
//! [`replace_flow`] is Replace All's confirmation dialog, execution and undo — living beside
//! the view rather than on the view-model because it needs an `EventContext`.

pub mod dock;
mod find_vm;
pub mod preview_dock;
pub mod replace_flow;
mod search_replace_vm;

pub use find_vm::{FindViewModel, PageDocuments, ResolveEditor};
pub use search_replace_vm::SearchReplaceViewModel;
