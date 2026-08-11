// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Document ingest: everything between "the writer chose some files" and "here
//! is the tree we would create", with no knowledge of the store and no UI.
//!
//! The shape is one seam and one pipeline. A [`SourceScanner`] turns one file's
//! bytes into a [`SourceDocument`] — a flat list of headings, prose runs already
//! converted to Djot, and scene breaks. Everything after that is format-agnostic:
//! ordering across documents, inferring which heading depth means which kind of
//! row, cleaning ordinals out of titles, and building an [`ImportPlan`] the
//! writer can review before anything is committed.
//!
//! Adding a format touches exactly one of those halves. That is the whole reason
//! this is a crate rather than a module inside `import_management`: a feature
//! that is not an importer — splitting an existing document, pasting structured
//! text — needs the pipeline without the use case, and reaching across features
//! for it is a dependency this workspace does not allow. `skrib_format` and
//! `binder_ordering` were extracted for the same reason and say so in their own
//! manifests.
//!
//! Nothing here touches a `UnitOfWork`, so all of it is testable as plain Rust.

pub mod block;
pub mod diagnostics;
pub mod front_matter;
pub mod order;
pub mod plan;
pub mod scanner;
pub mod sources;
pub mod structure;
pub mod text;
pub mod title;

pub use block::{
    AnnotationKind, SourceAnnotation, SourceAnnotationReply, SourceBlock, SourceDocument,
    SourceMetadata,
};
pub use diagnostics::{DiagnosticSeverity, ImportDiagnostic};
pub use order::{natural_cmp, sort_documents};
pub use plan::{ImportPlan, PlannedRow, build_plan};
pub use scanner::{ScannerRegistry, SourceScanner};
pub use structure::{LevelRules, infer_rules, levels_used};
pub use title::{ExtractedOrdinal, extract_leading_ordinal};

/// Scan, order, infer and plan — the whole pipeline, in the one order it is
/// correct in.
///
/// Exists because the steps have a required sequence and nothing in their
/// signatures enforces it. Ordering in particular is easy to leave out and
/// invisible when you do: the plan looks right whenever the caller happened to
/// pass files already sorted, and comes out shuffled for the writer whose
/// filesystem handed them over in another order — which is precisely the bug
/// (the surveyed 4,1,2,5,3) that `order` exists to prevent. One entry point that
/// cannot skip a step is worth more than three that each document their place.
///
/// `start` is the kind the shallowest heading level should become — chosen from
/// the destination, since importing under a Book means the first level is a Part
/// while importing at the root means it is a Book.
pub fn scan_and_plan(
    registry: &ScannerRegistry,
    files: &[(std::path::PathBuf, Vec<u8>)],
    start: skribisto_model::CreateType,
    chapter_mode: common::entities::ChapterMode,
    base_indent: i64,
) -> ImportPlan {
    let mut docs: Vec<SourceDocument> = files
        .iter()
        .map(|(path, bytes)| registry.scan_bytes(path, bytes))
        .collect();
    sort_documents(&mut docs);
    let rules = infer_rules(&structure::levels_used(&docs), start);
    build_plan(&docs, &rules, chapter_mode, base_indent)
}
