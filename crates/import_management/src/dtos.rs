// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//
// NOTE: three hand-corrections, re-applied after every regeneration of this file.
// All three are generator bugs analysis_management/src/dtos.rs and
// mention_management/src/dtos.rs already record.
//
// 1. SPDX headers are stripped by generation; re-added.
//
// 2. The import inference emitted a `use common::entities::{...}` line naming the
//    enums this file declares itself from the use cases' own `enum_values` — none of
//    which are entities, so importing them collides with their own declarations.
//    Nothing outside this file is needed, so the line is removed rather than corrected.
//
// 3. `#[allow(clippy::large_enum_variant)]` on `ImportComment`, `DocumentImportRow` and
//    `ApplyImportRow`. `ImportComment::Found` is a whole comment — kind, uid, uid_tag,
//    author, initials, body, resolved/orphan state and the full quote anchor — beside an
//    `Empty` unit variant, and the other two's populated variants each carry a vector of
//    those. The size gap is real and inherent to the shape the generator produces from an
//    `enum_values` list. Boxing would change a generated type's public shape and be undone
//    by the next regeneration; the values are built once per imported row and moved once,
//    so the gap costs nothing measurable.
//    (`DocumentImportRow` joined the list when it gained `epigraph` — one more `String`
//    was what carried `Found` past the lint's threshold. Nothing about it is new in kind.)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ImportPlumeCreatorFileDto {
    pub source_path: String,
    pub output_path: String,
    pub overwrite: bool,
    pub manuscript_binder_name: String,
    pub story_bible_binder_name: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ImportPlumeCreatorFileResultDto {
    pub output_path: String,
    pub imported_items: i64,
    pub skipped_trashed: i64,
    pub warnings: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AnalyzeDocumentImportDto {
    pub work_id: u64,
    pub binder_id: u64,
    pub source_paths: Vec<String>,
    pub anchor_item_id: u64,
    pub drop_position: DropPosition,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum DropPosition {
    #[default]
    Before,
    After,
    Into,
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DocumentImportPlanDto {
    pub row: DocumentImportRow,
    pub rows: DocumentImportRows,
    pub row_kind: ImportRowKind,
    pub comment_kind: ImportCommentKind,
    pub orphan_reason: ImportOrphanReason,
    pub comment: ImportComment,
    pub reply: ImportReply,
    pub diagnostic: ImportDiagnosticRow,
    pub diagnostics: ImportDiagnosticRows,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum DocumentImportRow {
    #[default]
    Empty,
    Found {
        indent: i64,
        kind: ImportRowKind,
        title: String,
        stripped_ordinal: String,
        djot: String,
        epigraph: String,
        scene_breaks: i64,
        word_count: i64,
        comments: Vec<ImportComment>,
        origin: String,
        included: bool,
        source_uid_tag: String,
        source_digest: String,
        source_file_digest: String,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum DocumentImportRows {
    #[default]
    Empty,
    Found(Vec<DocumentImportRow>),
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ImportRowKind {
    #[default]
    Book,
    Part,
    Chapter,
    Scene,
    Note,
    NoteFolder,
    Folder,
    Paratext,
    ParatextFolder,
    EndOfBook,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ImportCommentKind {
    #[default]
    Range,
    Paragraph,
    Document,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ImportOrphanReason {
    #[default]
    NotOrphaned,
    TextNotFound,
    Ambiguous,
    TargetDeleted,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum ImportComment {
    #[default]
    Empty,
    Found {
        kind: ImportCommentKind,
        uid: Option<uuid::Uuid>,
        uid_tag: String,
        author_name: String,
        author_initials: String,
        created_at: String,
        body: String,
        resolved: bool,
        orphaned: bool,
        orphan_reason: ImportOrphanReason,
        range_start: i64,
        range_length: i64,
        quote_prefix: String,
        quote_exact: String,
        quote_exact_truncated: bool,
        quote_suffix: String,
        block_ordinal_hint: i64,
        replies: Vec<ImportReply>,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ImportReply {
    #[default]
    Empty,
    Found {
        uid: Option<uuid::Uuid>,
        author_name: String,
        author_initials: String,
        created_at: String,
        body: String,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ImportDiagnosticRow {
    #[default]
    Empty,
    Reported {
        key: String,
        severity: String,
        path: String,
        detail: String,
        count: i64,
        row_index: i64,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ImportDiagnosticRows {
    #[default]
    Empty,
    Reported(Vec<ImportDiagnosticRow>),
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ApplyDocumentImportDto {
    pub work_id: u64,
    pub binder_id: u64,
    pub anchor_item_id: u64,
    pub drop_position: DropPosition,
    pub row: ApplyImportRow,
    pub rows: ApplyImportRows,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum ApplyImportRow {
    #[default]
    Empty,
    Create {
        indent: i64,
        kind: ImportRowKind,
        title: String,
        djot: String,
        epigraph: String,
        comments: Vec<ImportComment>,
        source_uid_tag: String,
        source_file_name: String,
        source_file_digest: String,
    },
    Update {
        target_uid_tag: String,
        replace_prose: bool,
        djot: String,
        epigraph: String,
        comments: Vec<ImportComment>,
        source_file_name: String,
        source_file_digest: String,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ApplyImportRows {
    #[default]
    Empty,
    Create(Vec<ApplyImportRow>),
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ApplyDocumentImportResultDto {
    pub created_ids: Vec<u64>,
}
