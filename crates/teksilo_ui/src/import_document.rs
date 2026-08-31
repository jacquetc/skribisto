// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Import documents** wizard: convert a folder of Markdown/DOCX/ODT files into a
//! reviewable plan before any of it touches the store.
//!
//! [`ImportDocumentViewModel`] is the whole feature's business logic — see its own doc for
//! the three-Stepper-step shape and why it is constructor-threaded rather than `app_state`.
//! [`panel`] is the view: thin, binding the view-model's signals and forwarding the Stepper's
//! footer to its methods.
//!
//! `import_document_vm` is `pub(crate)`, not just `mod` — [`crate::models::import_merge_source`]
//! reaches into it for the reconcile tree's row types, the same reason
//! `view_models::long_op` is `pub(crate)`.

/// Block-level accept/reject over a returning file's prose.
///
/// Store-free and headless: the part that is easy to get subtly wrong and
/// impossible to eyeball in a widget tree.
pub(crate) mod hunk_merge;
pub(crate) mod import_document_vm;
pub(crate) mod panel;

pub use import_document_vm::ImportDocumentViewModel;
