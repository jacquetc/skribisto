// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The export feature: three view-models plus their views (the export modal, its Choose…
//! checkbox tree, and the title-bar split button).
//!
//! [`ExportViewModel`] is the modal/split-button's business logic — see its own doc for the
//! focus-adaptive scope list and the panel state it owns. [`ExportStylesViewModel`] and
//! [`ParatextPresetsViewModel`] are the app-local siblings behind Settings ▸ Compile & Export:
//! the editable export-style presets and the paratext-structure presets New Work offers, each a
//! machine-wide preference that outlives any `Work`.

pub(crate) mod choose;
mod export_styles_vm;
mod export_vm;
pub(crate) mod panel;
mod paratext_presets_vm;
pub(crate) mod split_button;

pub use export_styles_vm::ExportStylesViewModel;
pub use export_vm::{ExportViewModel, format_label, scope_label};
pub use paratext_presets_vm::{ParatextPresetsViewModel, PresetRow};
