// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Import a Scrivener project feature: a three-step wizard (Project, Review, Save) that
//! reads a Scrivener project, shows what it holds and how its binder comes across, and
//! writes a new `.skrib` on the shared project-import job.
//!
//! [`ImportScrivenerViewModel`] is the wizard's business logic, one per window (Tier 3);
//! `messages` words everything the converter says; `preview_source` lays the planned binder
//! out as a tree; `panel` is the thin view. A finished import's report is `report` (its
//! content), `report_vm` (the dialog's view-model) and `report_panel` (the dialog).

mod import_scrivener_vm;
pub(crate) mod messages;
pub mod panel;
pub(crate) mod preview_source;
pub(crate) mod report;
pub(crate) mod report_panel;
pub(crate) mod report_vm;

#[cfg(test)]
pub(crate) use import_scrivener_vm::archived_report_name;
pub use import_scrivener_vm::{
    CardView, ChoiceLine, ImportRecord, ImportScrivenerViewModel, NoticeView, PickTarget,
    QuestionView, RefusalView, ReviewControl, ReviewRow, ScrivenerImportResult, ToggleTarget,
};
pub(crate) use import_scrivener_vm::{REPORT_ACTION, archived_report_path};
pub use report_vm::ImportReportViewModel;
