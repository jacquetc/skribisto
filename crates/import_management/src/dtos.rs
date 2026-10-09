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
//    `ScrivenerStructure` is the same case: `Planned` is a whole structure plan beside the
//    `Empty` unit variant, built once per analysis or review change and moved once.
//
// 4. `ImportManuskriptProjectDto` and its result are declared by hand rather than
//    regenerated in. They are exactly what the generator emits for the manifest's
//    `import_manuskript_project`, and adding them by hand is what keeps the three
//    corrections above from having to be re-applied for the sake of two structs.
//    The Scrivener importer's two pairs (`AnalyzeScrivenerProjectDto` /
//    `ScrivenerAnalysisDto`, `ImportScrivenerProjectDto` /
//    `ImportScrivenerProjectResultDto`) and every `Scrivener*` type they carry are
//    declared the same way, copied from the generator's output with Qleany v1.12.0.
//    The import inference names three entity enums among its own: the one line kept
//    imports only those three, `BinderItemRole`, `BinderItemSubRole` and
//    `StatusCategory` (correction 2 above, for the rest).
//
// 5. `ScrivenerComputers` carries no derived `Debug`: it names the writer's
//    computers (the manifest's `Device`, the lock file's host), which the project card
//    may show and nothing may keep, and a `{:?}` of an analysis is the first thing that
//    ends up in a log. `import_management::scrivener` implements `Debug` for it,
//    saying only whether each name is known, as `scrivener_import::Card` does.

use common::entities::{BinderItemRole, BinderItemSubRole, StatusCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ImportPlumeCreatorFileDto {
    pub source_path: String,
    pub output_path: String,
    pub overwrite: bool,
    pub manuscript_binder_name: String,
    pub story_bible_binder_name: String,
    pub status_names: Vec<String>,
    pub seeded_names: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ImportPlumeCreatorFileResultDto {
    pub output_path: String,
    pub imported_items: i64,
    pub skipped_trashed: i64,
    pub warnings: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ImportManuskriptProjectDto {
    pub source_path: String,
    pub output_path: String,
    pub overwrite: bool,
    pub manuscript_binder_name: String,
    pub story_bible_binder_name: String,
    pub characters_group_name: String,
    pub world_group_name: String,
    pub plots_group_name: String,
    pub project_info_note_name: String,
    pub summary_note_name: String,
    pub importance_names: Vec<String>,
    pub section_names: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ImportManuskriptProjectResultDto {
    pub output_path: String,
    pub imported_items: i64,
    pub imported_revisions: i64,
    pub warnings: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AnalyzeScrivenerProjectDto {
    pub source_path: String,
    pub chosen_project: String,
    pub chosen_manifest: String,
    pub use_enclosing_project: bool,
    pub upgraded_sibling: ScrivenerSiblingChoice,
    pub accept_unmerged_mobile_edits: bool,
    pub accept_no_text: bool,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerSiblingChoice {
    #[default]
    Unanswered,
    Sibling,
    Backup,
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ScrivenerAnalysisDto {
    pub outcome: ScrivenerAnalysisOutcome,
    pub refusal: ScrivenerFinding,
    pub question: ScrivenerQuestion,
    pub pending: Vec<ScrivenerQuestion>,
    pub notices: Vec<ScrivenerFinding>,
    pub project_path: String,
    pub folder_title: String,
    pub template_title: String,
    pub container: ScrivenerContainer,
    pub generation: ScrivenerGeneration,
    pub platform: ScrivenerPlatform,
    pub writer_version: String,
    pub creator: String,
    pub manifest: String,
    pub manifest_choice: ScrivenerManifestChoice,
    pub modified: Option<i64>,
    pub modified_offset_minutes: Option<i64>,
    pub items: i64,
    pub documents: i64,
    pub manuscript_documents: i64,
    pub documents_present: i64,
    pub media_items: i64,
    pub research_files: i64,
    pub trashed_items: i64,
    pub snapshots: i64,
    pub computers: ScrivenerComputers,
    pub proposed_title: String,
    pub proposed_title_source: ScrivenerTitleSource,
    pub proposed_authors: Vec<String>,
    pub proposed_author_source: ScrivenerAuthorSource,
    pub proposed_language: String,
    pub proposed_language_source: ScrivenerLanguageSource,
    pub structure: ScrivenerStructure,
    pub review_data: String,
    pub fingerprint: String,
    pub finding_family: ScrivenerFindingFamily,
    pub report_group: ScrivenerReportGroup,
    pub place: ScrivenerPlace,
    pub arg: ScrivenerArg,
    pub manifest_candidate: ScrivenerManifestCandidate,
    pub manifest_evidence: ScrivenerManifestEvidence,
    pub mobile_edits: ScrivenerMobileEdits,
    pub preview_row: ScrivenerPreviewRow,
    pub binder: ScrivenerBinder,
    pub title_kind: ScrivenerTitleKind,
    pub signal_kind: ScrivenerSignal,
    pub control: ScrivenerControl,
    pub item_ref: ScrivenerItemRef,
    pub signal_count: ScrivenerSignalCount,
    pub shape_flag: ScrivenerShapeFlag,
    pub container_rung: ScrivenerContainerRung,
    pub text_kind: ScrivenerTextKind,
    pub destination: ScrivenerDestination,
    pub books_choice: ScrivenerBooksChoice,
    pub trash_choice: ScrivenerTrashChoice,
    pub shape: ScrivenerShape,
    pub chapter_encoding: ScrivenerChapterEncoding,
    pub choice: ScrivenerChoice,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerAnalysisOutcome {
    #[default]
    Read,
    Choose,
    Refused,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerFinding {
    #[default]
    Empty,
    Reported {
        key: String,
        family: ScrivenerFindingFamily,
        group: ScrivenerReportGroup,
        place: ScrivenerPlace,
        item: i64,
        binder_path: Vec<String>,
        item_uid: Option<uuid::Uuid>,
        args: Vec<ScrivenerArg>,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerQuestion {
    #[default]
    Empty,
    SeveralProjects {
        candidates: Vec<String>,
    },
    SeveralManifests {
        candidates: Vec<ScrivenerManifestCandidate>,
        preselected: Option<String>,
        evidence: Vec<ScrivenerManifestEvidence>,
    },
    EnclosingProject {
        project: String,
        levels_up: i64,
    },
    UpgradedSibling {
        backup: String,
        sibling: String,
        sibling_generation: Option<ScrivenerGeneration>,
        preselect_sibling: bool,
    },
    UnmergedMobileEdits {
        source: ScrivenerMobileEdits,
    },
    NoTextInCopy {
        text_items: i64,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerContainer {
    #[default]
    Folder,
    Zip,
    Template,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerGeneration {
    #[default]
    Format20,
    Format15,
    Format10,
    IosStub,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerPlatform {
    #[default]
    Unknown,
    Mac,
    Windows,
    Linux,
    Ios,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerManifestChoice {
    #[default]
    Only,
    Picked,
    Answered,
    Identical,
    Automatic,
    MobileBinder,
}

// No derived `Debug`: `import_management::scrivener` implements it, withholding both names.
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub enum ScrivenerComputers {
    #[default]
    Unknown,
    Named {
        device: Option<String>,
        lock_host: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerTitleSource {
    #[default]
    FolderName,
    CompileSettings,
    ProjectProperties,
    Template,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerAuthorSource {
    #[default]
    NotStated,
    CompileSettings,
    ProjectProperties,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerLanguageSource {
    #[default]
    NotStated,
    CompileSettings,
    Script,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum ScrivenerStructure {
    #[default]
    Empty,
    Planned {
        rows: Vec<ScrivenerPreviewRow>,
        controls: Vec<ScrivenerControl>,
        set_aside_trashed: i64,
        set_aside_recovered: i64,
        shape: ScrivenerShape,
        chapter_encoding: ScrivenerChapterEncoding,
        templates: Vec<String>,
        tags: Vec<String>,
        notices: Vec<ScrivenerFinding>,
        choices: Vec<ScrivenerChoice>,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerFindingFamily {
    #[default]
    Refusal,
    Source,
    Structure,
    Prose,
    Document,
    Metadata,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerReportGroup {
    #[default]
    Unfiled,
    CouldNotBeRead,
    CheckThese,
    BroughtAcrossDifferently,
    LeftBehind,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerPlace {
    #[default]
    Project,
    Item,
    Text,
    Notes,
    Synopsis,
    Media,
    CardImage,
    ProjectNote,
    Recovered,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerArg {
    #[default]
    Empty,
    Count {
        name: String,
        value: i64,
    },
    Text {
        name: String,
        value: String,
    },
    Texts {
        name: String,
        values: Vec<String>,
    },
    Flag {
        name: String,
        value: bool,
    },
    Token {
        name: String,
        value: String,
    },
    Day {
        name: String,
        value: String,
    },
    Item {
        name: String,
        binder_path: Vec<String>,
        item_uid: Option<uuid::Uuid>,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerManifestCandidate {
    #[default]
    Empty,
    Candidate {
        name: String,
        creator: Option<String>,
        version: Option<String>,
        modified: Option<i64>,
        items: Option<i64>,
        matches_autosave: bool,
        identical_to_preselected: bool,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerManifestEvidence {
    #[default]
    MatchesAutosave,
    FolderName,
    Newest,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerMobileEdits {
    #[default]
    MobileFolder,
    NewerMobileBinder,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerPreviewRow {
    #[default]
    Empty,
    Row {
        binder: ScrivenerBinder,
        depth: i64,
        role: BinderItemRole,
        sub_role: BinderItemSubRole,
        title: String,
        title_kind: ScrivenerTitleKind,
        exportable: bool,
        activated: bool,
        item: i64,
        why: ScrivenerSignal,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerBinder {
    #[default]
    Manuscript,
    Notes,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerTitleKind {
    #[default]
    Own,
    NotesOf,
    ProjectTitle,
    ProjectNotes,
    Recovered,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerSignal {
    #[default]
    SectionType,
    InheritedSectionType,
    Title,
    SiblingTitles,
    LevelType,
    Shape,
    Minority,
    ClaimsBelow,
    ParatextChildren,
    EmptyFolder,
    Floor,
    Choice,
    Text,
    Heading,
    Inside,
    Media,
    Matter,
    StoryBible,
    Notes,
    Synthesised,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerControl {
    #[default]
    Empty,
    Containers {
        depth: i64,
        containers: i64,
        adjustable: i64,
        detected: ScrivenerContainerRung,
        mixed: bool,
        chosen: Option<ScrivenerContainerRung>,
        examples: Vec<ScrivenerItemRef>,
        signals: Vec<ScrivenerSignalCount>,
        flags: Vec<ScrivenerShapeFlag>,
    },
    Texts {
        depth: i64,
        texts: i64,
        detected: ScrivenerTextKind,
        chosen: Option<ScrivenerTextKind>,
        examples: Vec<ScrivenerItemRef>,
        median_words: i64,
    },
    Root {
        item: ScrivenerItemRef,
        items: i64,
        detected: ScrivenerDestination,
        chosen: Option<ScrivenerDestination>,
        looks_like_manuscript: bool,
    },
    Books {
        claimed: Vec<ScrivenerItemRef>,
        detected: ScrivenerBooksChoice,
        chosen: Option<ScrivenerBooksChoice>,
    },
    Split {
        item: ScrivenerItemRef,
        headings: i64,
        detected: bool,
        chosen: Option<bool>,
    },
    Trash {
        items: i64,
        with_writing: i64,
        chosen: ScrivenerTrashChoice,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerItemRef {
    #[default]
    Empty,
    Item {
        item: i64,
        binder_path: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerSignalCount {
    #[default]
    Empty,
    Counted {
        signal: ScrivenerSignal,
        count: i64,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerShapeFlag {
    #[default]
    MedianDiffersFromMaximum,
    LoneWrapper,
    MinorityContainers,
    AdoptsTexts,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerContainerRung {
    #[default]
    Part,
    Chapter,
    Plain,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerTextKind {
    #[default]
    Scenes,
    Chapters,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerDestination {
    #[default]
    Notes,
    StoryBible,
    Manuscript,
    LeaveOut,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerBooksChoice {
    #[default]
    One,
    Several,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerTrashChoice {
    #[default]
    Import,
    LeaveOut,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerShape {
    #[default]
    Empty,
    Counted {
        books: i64,
        parts: i64,
        chapter_folders: i64,
        flat_chapters: i64,
        scenes: i64,
        paratexts: i64,
        plain_folders: i64,
        manuscript_notes: i64,
        notes_rows: i64,
        trashed_rows: i64,
        templates: i64,
        tags: i64,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerChapterEncoding {
    #[default]
    Folder,
    Flat,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerChoice {
    #[default]
    Empty,
    Containers {
        depth: i64,
        rung: ScrivenerContainerRung,
    },
    Texts {
        depth: i64,
        kind: ScrivenerTextKind,
    },
    Root {
        item: i64,
        destination: ScrivenerDestination,
    },
    Books {
        choice: ScrivenerBooksChoice,
    },
    Split {
        item: i64,
        split: bool,
    },
    Trash {
        choice: ScrivenerTrashChoice,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ImportScrivenerProjectDto {
    pub source_path: String,
    pub chosen_project: String,
    pub chosen_manifest: String,
    pub use_enclosing_project: bool,
    pub upgraded_sibling: ScrivenerSiblingChoice,
    pub accept_unmerged_mobile_edits: bool,
    pub accept_no_text: bool,
    pub fingerprint: String,
    pub choices: Vec<ScrivenerChoice>,
    pub blank_lines_as_scene_breaks: bool,
    pub ornaments_as_scene_breaks: bool,
    pub output_path: String,
    pub overwrite: bool,
    pub title: Option<String>,
    pub author: Option<String>,
    pub language: Option<String>,
    pub copy_research_files: bool,
    pub manuscript_binder_name: String,
    pub notes_binder_name: String,
    pub notes_of_pattern: String,
    pub project_notes_name: String,
    pub recovered_name: String,
    pub untitled_template_name: String,
    pub research_folder_pattern: String,
    pub list_separator: String,
    pub label_details_pattern: String,
    pub status_details_pattern: String,
    pub unnamed_label_pattern: String,
    pub unnamed_status_pattern: String,
    pub label_tag_pattern: String,
    pub untitled_tag_name: String,
    pub default_statuses: Vec<ScrivenerDefaultRung>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerDefaultRung {
    #[default]
    Empty,
    Rung {
        name: String,
        category: StatusCategory,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ImportScrivenerProjectResultDto {
    pub output_path: String,
    pub research_folder: String,
    pub totals: ScrivenerTotals,
    pub findings: Vec<ScrivenerFinding>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ScrivenerTotals {
    #[default]
    Empty,
    Counted {
        rows: i64,
        words: i64,
        footnotes: i64,
        comments: i64,
        pictures: i64,
        templates: i64,
        tags: i64,
        rungs: i64,
        trashed: i64,
        research_files: i64,
        synopses: i64,
        document_notes: i64,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AnalyzeDocumentImportDto {
    pub work_id: u64,
    pub binder_id: u64,
    pub source_paths: Vec<String>,
    pub anchor_item_id: u64,
    pub drop_position: DropPosition,
    pub picture_dir: String,
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
    pub footnote: ImportFootnote,
    pub diagnostic: ImportDiagnosticRow,
    pub diagnostics: ImportDiagnosticRows,
    pub picture: ImportPicture,
    pub pictures: ImportPictures,
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
        footnotes: Vec<ImportFootnote>,
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
pub enum ImportFootnote {
    #[default]
    Empty,
    Found {
        label: String,
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

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ImportPicture {
    #[default]
    Empty,
    Found {
        content_hash: String,
        file_name: String,
        mime_type: String,
        width: i64,
        height: i64,
        byte_size: i64,
    },
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum ImportPictures {
    #[default]
    Empty,
    Found(Vec<ImportPicture>),
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ApplyDocumentImportDto {
    pub work_id: u64,
    pub binder_id: u64,
    pub anchor_item_id: u64,
    pub drop_position: DropPosition,
    pub row: ApplyImportRow,
    pub rows: ApplyImportRows,
    pub pictures: ImportPictures,
    pub picture_dir: String,
    pub media_root: String,
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
        footnotes: Vec<ImportFootnote>,
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
        footnotes: Vec<ImportFootnote>,
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
