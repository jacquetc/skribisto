// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What reading a Scrivener project says to the writer, as values rather than
//! sentences.
//!
//! Three kinds, following the three things the interface can do with them:
//!
//! * a [`Refusal`]: the project cannot be imported, and why;
//! * a [`Question`]: the writer must choose (which project, which manifest,
//!   which copy) or accept (import without the unmerged iPad edits, import the
//!   documents empty) before the import can go ahead;
//! * a [`Notice`]: what the analysis found that the writer should know.
//!
//! None of them words anything: each carries the data its sentence needs and a
//! stable [`key`](Refusal::key) the interface translates, in the writer's own
//! language. Every key starts with `scrivener-`, so none can collide with the
//! document importer's keys in one list.
//!
//! **Paths.** A value names a file inside the project by its path in the
//! project (`Files/Data/<UUID>/content.rtf`), and anything outside the project by
//! its file name alone, never a path into the writer's home folder. **Personal
//! data** (the lock file's host and user, the manifest's `Device`) never appears
//! here: a notice says whether the lock names this computer, not which computer
//! it names.

use serde::{Deserialize, Serialize};

use crate::metadata::{AuthorSource, TitleSource};
use crate::model::{Generation, ItemKey, TextRole};
use crate::structure::PlanError;

/// Why a project could not be imported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Refusal {
    /// Nothing is at the chosen path.
    Missing { file: String },
    /// The chosen file or folder could not be read. `reason` is the system's own
    /// words.
    Unreadable { file: String, reason: String },
    /// No Scrivener project within reach of the pick.
    NotAScrivenerProject,
    /// Only a project's manifest, without the `Files` folder that holds its
    /// writing: a manifest published on its own, or a copy that lost its files.
    ManifestWithoutFiles { manifest: String },
    /// The folder picked holds too many folders, two levels down, to search for
    /// a project in (a whole home folder): pick the project itself. `limit` is
    /// how many folders the search lists.
    SearchTooWide { limit: u32 },
    /// A Scrivener 1 for Mac project (2007 to 2010), which this importer does not
    /// read. Scrivener 2 for Mac upgrades it, and Scrivener can compile it to a
    /// Word document.
    ScrivenerOneForMac,
    /// A project created on an iPhone or iPad whose `Mobile` folder, which holds
    /// its binder, is missing or damaged.
    IosStubWithoutMobileData,
    /// A manifest `Version` this importer does not know: a newer Scrivener wrote
    /// it, or something else did.
    UnknownFormatVersion { version: String },
    /// `Files/version.txt` says a newer Scrivener wrote the project.
    NewerFormatStamp { stamp: u32, newest: u32 },
    /// `Files/version.txt` is not a number, so whether a newer Scrivener wrote the
    /// project cannot be told.
    VersionStampUnreadable,
    /// The manifest is damaged. Scrivener's spare binder copies are not read yet;
    /// `spare_copies` says how many of them the project holds.
    DamagedManifest {
        manifest: String,
        damage: ManifestDamage,
        spare_copies: u32,
    },
    /// The manifest is larger than this importer reads.
    ManifestTooLarge { manifest: String, limit_mib: u64 },
    /// No binder item carries a usable identifier, so no item's files can be
    /// found.
    NoItemIdentifiers,
    /// The `Files` folder is there but its documents are not: a sync service
    /// copied only part of the project.
    IncompleteCopy,
    /// An archive that could not be read. `reason` is the archive reader's own
    /// words.
    ArchiveUnreadable { file: String, reason: String },
    /// An archive member whose name would reach outside the archive or is not a
    /// name at all.
    ArchiveUnsafeName {
        file: String,
        member: String,
        why: UnsafeName,
    },
    /// An archive holding more members than this importer reads.
    ArchiveTooManyMembers { limit: u64 },
    /// An archive member that unpacks past what one member may.
    ArchiveMemberTooLarge { member: String, limit_mib: u64 },
    /// An archive that unpacks past what an archive may, or at a ratio no real
    /// project comes near.
    ArchiveTooLarge { limit_mib: u64 },
    /// An encrypted archive.
    ArchiveEncrypted { file: String },
    /// An archive member compressed with a method this importer does not read
    /// (7-Zip's Deflate64, bzip2, LZMA, zstd).
    ArchiveUnsupportedCompression { member: String, method: String },
    /// An archive listing members under one name more than once.
    DuplicateArchiveMembers {
        file: String,
        member: Option<String>,
    },
    /// A `.scrivtemplate` whose project could not be decoded.
    TemplateUndecodable {
        file: String,
        damage: TemplateDamage,
    },
    /// Files the import needs are cloud placeholders, not yet downloaded.
    CloudPlaceholders { count: u32, examples: Vec<String> },
    /// Files the import needs could not be read (permissions, a failing disk).
    UnreadableMembers { count: u32, examples: Vec<String> },
    /// A file the import needs is a link to somewhere else; links are never
    /// followed.
    NeededMemberIsLink { member: String },
    /// The project changed while it was being read, twice: Scrivener or a sync
    /// service is writing to it.
    ProjectChangedDuringRead { count: u32 },
    /// More files than a project may hold.
    TooManyMembers { limit: u64 },
    /// More writing, in mebibytes, than a project may hold.
    TooLarge { limit_mib: u64 },
    /// The binder could not be mapped onto Skribisto's structure without
    /// losing or misplacing something. A defect of this importer, refused
    /// rather than written; `error` names the broken invariant.
    StructureUnmappable { error: PlanError },
}

/// What is wrong with a damaged manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManifestDamage {
    /// Not UTF-8 text.
    NotText,
    /// Not well-formed XML. `reason` is the parser's own words.
    NotXml { reason: String },
    /// Nested deeper than any Scrivener manifest, so refused before parsing.
    TooDeep { depth: u32, line: u32 },
    /// Declares an entity, which no Scrivener manifest does: a small file that
    /// would expand past any memory. Refused before anything was expanded.
    DeclaresEntities,
    /// Lists more binder items than any real project (the largest seen lists
    /// 2,509), or, beside other manifests, more than a project's manifests may
    /// list together. Refused while read, before the rows fill memory.
    TooManyItems { limit: u32 },
    /// The root element is not `ScrivenerProject`.
    NotAManifest,
    /// No `Binder`, or a binder without a single item.
    EmptyBinder,
    /// An empty `NewProject="Yes"` placeholder.
    NewProjectPlaceholder,
}

/// Why an archive member's name is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnsafeName {
    Empty,
    Absolute,
    ParentFolder,
    ControlCharacter,
}

/// What is wrong with a `.scrivtemplate`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateDamage {
    /// Not well-formed XML, or not UTF-8.
    NotXml,
    /// The root element is not `ScrivenerTemplate`.
    NotATemplate,
    /// No `ProjectZipData` in it.
    NoProjectData,
    /// A `Type` this importer does not know.
    UnknownEncoding { encoding: String },
    /// The data does not decode (a truncated download).
    Undecodable,
    /// The decoded data is not a zip archive.
    NotAZip,
    /// Larger than a template may be, in mebibytes.
    TooLarge { limit_mib: u64 },
}

/// Whether a question asks the writer to choose between things or to accept one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tier {
    /// Nothing can be read until the writer chooses.
    Choose,
    /// The project was read; the import waits until the writer accepts.
    Accept,
}

/// A question the writer must answer before the import can go ahead.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Question {
    /// The pick holds several projects. No default: nothing singles one out.
    SeveralProjects { candidates: Vec<ProjectCandidate> },
    /// The project holds several manifests. `preselected` is set only when
    /// evidence singles one out; `evidence` says which.
    SeveralManifests {
        candidates: Vec<ManifestCandidate>,
        preselected: Option<String>,
        evidence: Vec<ManifestEvidence>,
    },
    /// The pick is a folder or file inside a project: import the whole project?
    /// `levels_up` says how far above the pick the project folder is.
    EnclosingProject { project: String, levels_up: u32 },
    /// The pick is the copy Scrivener made before upgrading a project, and the
    /// upgraded project sits beside it. The upgraded one is preselected only
    /// when it reads cleanly.
    UpgradedSibling {
        backup: String,
        sibling: String,
        sibling_generation: Option<Generation>,
        preselect_sibling: bool,
    },
    /// Edits made on an iPhone or iPad wait in the project and were never merged
    /// by a desktop Scrivener. The import can go ahead without them; they are
    /// never merged here.
    UnmergedMobileEdits { source: MobileEdits },
    /// Many documents and not one file of writing: a copy that is still syncing,
    /// or that lost its documents. Importing it gives empty documents.
    NoTextInCopy { text_items: u32 },
}

/// Where unmerged iOS edits were found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MobileEdits {
    /// A `Mobile` folder beside the desktop project.
    MobileFolder,
    /// A `Files/binder.mob` newer than the manifest.
    NewerMobileBinder,
}

/// One project found below a pick.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectCandidate {
    /// Its path below the pick, `/`-separated.
    pub path: String,
}

/// One manifest a project holds, with what tells it from the others.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestCandidate {
    pub name: String,
    pub creator: Option<String>,
    pub version: Option<String>,
    /// When it was last saved, in seconds since the Unix epoch, when it says.
    pub modified: Option<i64>,
    /// Binder items, or `None` when it could not be read.
    pub items: Option<u32>,
    /// Scrivener's own last autosave is a copy of this one.
    pub matches_autosave: bool,
    /// Byte for byte the same as the preselected one.
    pub identical_to_preselected: bool,
}

/// What singles a manifest out among several.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManifestEvidence {
    /// Scrivener's last autosave is a copy of it, and of no other.
    MatchesAutosave,
    /// It alone carries the project folder's name.
    FolderName,
    /// It was saved strictly after every other.
    Newest,
}

/// Something the writer should know about a project that can be imported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Notice {
    /// The project was found below the folder picked, at this path.
    ProjectFoundBelowPick { path: String },
    /// The source is a `.scrivtemplate`: a starter project, usually full of
    /// instructions.
    SourceIsTemplate { title: Option<String> },
    /// The source looks like one of Scrivener's automatic backups, so its text
    /// may be older than the working project.
    SourceIsBackup { sign: BackupSign },
    /// The writer chose to import the copy Scrivener kept from before an
    /// upgrade.
    ImportingPreUpgradeCopy,
    /// The project was created on an iPhone or iPad and read from its `Mobile`
    /// folder.
    ImportedFromIosStub,
    /// A `Files/binder.mob` older than the manifest: a leftover of an iOS sync
    /// already merged, ignored.
    StaleMobileBinderIgnored,
    /// Several manifests, all byte for byte the same.
    IdenticalManifestCopies { count: u32 },
    /// Several manifests; the live one was chosen on converging evidence and the
    /// others, sync copies of it, were ignored.
    ManifestCopiesIgnored { chosen: String, copies: Vec<String> },
    /// The manifest picked is named like a sync copy of another manifest beside
    /// it.
    PinnedManifestLooksLikeACopy { name: String, original: String },
    /// The manifest picked has other manifests beside it, which were not read.
    PinnedManifestHasSiblings { others: u32 },
    /// Files of a Scrivener 1 project left beside this one, ignored.
    LegacyV1FilesBeside { count: u32 },
    /// Both format 1.x and format 2.0 document folders are present; files were
    /// looked for where the manifest says.
    MixedPayloadTrees,
    /// The binder's identifiers are not the ones its version uses; files were
    /// looked for where the identifiers point.
    IdSchemeMismatch { version: String },
    /// `Files/version.txt` disagrees with the manifest's version.
    VersionStampMismatch { stamp: u32, version: String },
    /// A `Creator` this importer does not recognise.
    UnrecognisedWriter { creator: Option<String> },
    /// Binder identifiers that are not identifiers, dropped.
    InvalidItemIdentifier { count: u32 },
    /// Binder items with no usable identifier, whose files cannot be found.
    ItemsWithoutIdentifier { count: u32 },
    /// A binder item whose identifier an earlier item already carries: it keeps
    /// its place, and the files go to the first.
    DuplicateItemKey { key: ItemKey },
    /// A lock file: Scrivener had the project open, on this computer or another,
    /// when it was copied, or crashed with it open.
    LockPresent { this_machine: bool },
    /// The archive was made while Scrivener had the project open.
    ArchivedWhileOpen,
    /// A document file is missing. `salvaged` when Scrivener's search index still
    /// held its words, which were kept as plain text, possibly an older version.
    ContentMissing {
        item: ItemKey,
        role: TextRole,
        salvaged: bool,
    },
    /// A document file that is empty on disk although Scrivener recorded it with
    /// content: a sync that stopped half-way.
    ContentTruncated { item: ItemKey, role: TextRole },
    /// A picture, PDF or other research file the binder lists is missing.
    MissingMedia { item: ItemKey },
    /// Sync copies of documents, brought across as recovered texts.
    SyncConflictCopies { count: u32, original_missing: u32 },
    /// Documents on disk that no binder item lists, brought across as recovered
    /// texts.
    RecoveredUnreferencedText { count: u32 },
    /// Sync debris that holds no writing (copies of settings and indexes),
    /// ignored.
    SyncDebrisIgnored { count: u32 },
    /// Files that are not part of a Scrivener project, not imported. `examples`
    /// are paths inside the project.
    UnrecognisedFiles { count: u32, examples: Vec<String> },
    /// Links inside the project, never followed.
    SymlinkNotFollowed { count: u32, examples: Vec<String> },
    /// Files that could not be read and that the import does not need.
    UnreadableFilesSkipped { count: u32, examples: Vec<String> },
    /// Cloud placeholders among files the import does not need, skipped.
    CloudPlaceholdersSkipped { count: u32 },
    /// Text items and not a word on disk, in a project that looks genuinely
    /// empty.
    NoTextFound { text_items: u32 },
    /// A file of Scrivener's own that could not be read; what it held did not
    /// arrive. `reason` is the reader's own words.
    MetadataUnreadable {
        member: String,
        problem: ReadProblem,
    },
    /// References to ids the project does not define (a keyword, a label, a
    /// section type), left as they are for the mapping to report.
    DanglingReferences { kind: DanglingKind, count: u32 },
    /// Snapshots, which are not imported (decision 7): the Scrivener project keeps every
    /// one. `snapshots` in all, of `documents` documents, `of_deleted` of them kept for
    /// documents since deleted.
    SnapshotsNotImported {
        snapshots: u32,
        documents: u32,
        of_deleted: u32,
    },
    /// Scrivener's daily writing history, which does not become Skribisto's (D6.8): its
    /// figures cannot be turned into a running total without inventing numbers, so
    /// Skribisto starts its own count at the first save. `days` days written, from
    /// `first_day` to `last_day` (`YYYY-MM-DD`, the writer's own calendar); `unreadable`
    /// entries that named no day.
    WritingHistoryNotImported {
        days: u32,
        first_day: Option<String>,
        last_day: Option<String>,
        unreadable: u32,
    },
    /// The title the settings state is a template's placeholder (`<$template_projectName>`),
    /// not proposed.
    TitlePlaceholderRejected { source: TitleSource },
    /// An author's name the settings state is a template's placeholder, not proposed.
    AuthorPlaceholderRejected { source: AuthorSource },
    /// The title proposed is the one the project's settings state, not its folder's name.
    TitleFromProject { source: TitleSource },
    /// The author proposed is the one the project's settings name. A template's author can
    /// linger there, and prints on a title page.
    AuthorFromProject { source: AuthorSource },
    /// The project states no language (Scrivener records none): the interface proposes its
    /// own, which decides how words are counted and quotes are written.
    LanguageNotStated,
    /// The language proposed is the one the Draft's script names (`zh`, `ja`, `ko`, `he`,
    /// `ar`), not one the project states.
    LanguageFromScript { tag: String },
}

/// Why one file of the project could not be read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadProblem {
    /// A link, never followed.
    Link,
    /// A cloud placeholder: reading it would download it.
    CloudPlaceholder,
    /// Larger than this importer reads.
    TooLarge,
    /// Not a regular file (a folder, a FIFO, a device).
    NotAFile,
    /// Not UTF-8 text.
    NotText,
    /// XML nested past the ceiling, refused before parsing.
    TooDeep { depth: u32, line: u32 },
    /// XML declaring an entity, which no Scrivener file does, refused before
    /// anything was expanded.
    DeclaresEntities,
    /// Not well-formed XML. `reason` is the parser's own words.
    Malformed { reason: String },
    /// The system or the archive reader refused it. `reason` is its own words.
    System { reason: String },
}

/// What makes a source look like an automatic backup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackupSign {
    /// A folder named `Backups` on its path.
    BackupsFolder,
    /// An archive named `.bak.zip` or `.bak<N>.zip`.
    BakArchive,
    /// A name stamped `[YYYY_MM_DD_HH_MM_SS]`.
    DateStamped,
}

/// Which kind of id a reference names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DanglingKind {
    Keyword,
    Label,
    Status,
    SectionType,
    CustomField,
    Item,
}

impl Refusal {
    /// Every key a refusal can have, for the interface's translation tests.
    pub const KEYS: &'static [&'static str] = &[
        "scrivener-refusal-missing",
        "scrivener-refusal-unreadable",
        "scrivener-refusal-not-a-scrivener-project",
        "scrivener-refusal-manifest-without-files",
        "scrivener-refusal-search-too-wide",
        "scrivener-refusal-scrivener-one-for-mac",
        "scrivener-refusal-ios-stub-without-mobile-data",
        "scrivener-refusal-unknown-format-version",
        "scrivener-refusal-newer-format-stamp",
        "scrivener-refusal-version-stamp-unreadable",
        "scrivener-refusal-damaged-manifest",
        "scrivener-refusal-manifest-too-large",
        "scrivener-refusal-no-item-identifiers",
        "scrivener-refusal-incomplete-copy",
        "scrivener-refusal-archive-unreadable",
        "scrivener-refusal-archive-unsafe-name",
        "scrivener-refusal-archive-too-many-members",
        "scrivener-refusal-archive-member-too-large",
        "scrivener-refusal-archive-too-large",
        "scrivener-refusal-archive-encrypted",
        "scrivener-refusal-archive-unsupported-compression",
        "scrivener-refusal-duplicate-archive-members",
        "scrivener-refusal-template-undecodable",
        "scrivener-refusal-cloud-placeholders",
        "scrivener-refusal-unreadable-members",
        "scrivener-refusal-needed-member-is-link",
        "scrivener-refusal-project-changed-during-read",
        "scrivener-refusal-too-many-members",
        "scrivener-refusal-too-large",
        "scrivener-refusal-structure-unmappable",
    ];

    /// The stable key the interface words this refusal by.
    pub fn key(&self) -> &'static str {
        match self {
            Refusal::Missing { .. } => "scrivener-refusal-missing",
            Refusal::Unreadable { .. } => "scrivener-refusal-unreadable",
            Refusal::NotAScrivenerProject => "scrivener-refusal-not-a-scrivener-project",
            Refusal::ManifestWithoutFiles { .. } => "scrivener-refusal-manifest-without-files",
            Refusal::SearchTooWide { .. } => "scrivener-refusal-search-too-wide",
            Refusal::ScrivenerOneForMac => "scrivener-refusal-scrivener-one-for-mac",
            Refusal::IosStubWithoutMobileData => "scrivener-refusal-ios-stub-without-mobile-data",
            Refusal::UnknownFormatVersion { .. } => "scrivener-refusal-unknown-format-version",
            Refusal::NewerFormatStamp { .. } => "scrivener-refusal-newer-format-stamp",
            Refusal::VersionStampUnreadable => "scrivener-refusal-version-stamp-unreadable",
            Refusal::DamagedManifest { .. } => "scrivener-refusal-damaged-manifest",
            Refusal::ManifestTooLarge { .. } => "scrivener-refusal-manifest-too-large",
            Refusal::NoItemIdentifiers => "scrivener-refusal-no-item-identifiers",
            Refusal::IncompleteCopy => "scrivener-refusal-incomplete-copy",
            Refusal::ArchiveUnreadable { .. } => "scrivener-refusal-archive-unreadable",
            Refusal::ArchiveUnsafeName { .. } => "scrivener-refusal-archive-unsafe-name",
            Refusal::ArchiveTooManyMembers { .. } => "scrivener-refusal-archive-too-many-members",
            Refusal::ArchiveMemberTooLarge { .. } => "scrivener-refusal-archive-member-too-large",
            Refusal::ArchiveTooLarge { .. } => "scrivener-refusal-archive-too-large",
            Refusal::ArchiveEncrypted { .. } => "scrivener-refusal-archive-encrypted",
            Refusal::ArchiveUnsupportedCompression { .. } => {
                "scrivener-refusal-archive-unsupported-compression"
            }
            Refusal::DuplicateArchiveMembers { .. } => {
                "scrivener-refusal-duplicate-archive-members"
            }
            Refusal::TemplateUndecodable { .. } => "scrivener-refusal-template-undecodable",
            Refusal::CloudPlaceholders { .. } => "scrivener-refusal-cloud-placeholders",
            Refusal::UnreadableMembers { .. } => "scrivener-refusal-unreadable-members",
            Refusal::NeededMemberIsLink { .. } => "scrivener-refusal-needed-member-is-link",
            Refusal::ProjectChangedDuringRead { .. } => {
                "scrivener-refusal-project-changed-during-read"
            }
            Refusal::TooManyMembers { .. } => "scrivener-refusal-too-many-members",
            Refusal::TooLarge { .. } => "scrivener-refusal-too-large",
            Refusal::StructureUnmappable { .. } => "scrivener-refusal-structure-unmappable",
        }
    }
}

impl Question {
    /// Every key a question can have.
    pub const KEYS: &'static [&'static str] = &[
        "scrivener-question-several-projects",
        "scrivener-question-several-manifests",
        "scrivener-question-enclosing-project",
        "scrivener-question-upgraded-sibling",
        "scrivener-question-unmerged-mobile-edits",
        "scrivener-question-no-text-in-copy",
    ];

    /// The stable key the interface words this question by.
    pub fn key(&self) -> &'static str {
        match self {
            Question::SeveralProjects { .. } => "scrivener-question-several-projects",
            Question::SeveralManifests { .. } => "scrivener-question-several-manifests",
            Question::EnclosingProject { .. } => "scrivener-question-enclosing-project",
            Question::UpgradedSibling { .. } => "scrivener-question-upgraded-sibling",
            Question::UnmergedMobileEdits { .. } => "scrivener-question-unmerged-mobile-edits",
            Question::NoTextInCopy { .. } => "scrivener-question-no-text-in-copy",
        }
    }

    /// Whether the writer chooses between things, or accepts one.
    pub fn tier(&self) -> Tier {
        match self {
            Question::SeveralProjects { .. }
            | Question::SeveralManifests { .. }
            | Question::EnclosingProject { .. }
            | Question::UpgradedSibling { .. } => Tier::Choose,
            Question::UnmergedMobileEdits { .. } | Question::NoTextInCopy { .. } => Tier::Accept,
        }
    }
}

impl Notice {
    /// Every key a notice can have.
    pub const KEYS: &'static [&'static str] = &[
        "scrivener-notice-project-found-below-pick",
        "scrivener-notice-source-is-template",
        "scrivener-notice-source-is-backup",
        "scrivener-notice-importing-pre-upgrade-copy",
        "scrivener-notice-imported-from-ios-stub",
        "scrivener-notice-stale-mobile-binder-ignored",
        "scrivener-notice-identical-manifest-copies",
        "scrivener-notice-manifest-copies-ignored",
        "scrivener-notice-pinned-manifest-looks-like-a-copy",
        "scrivener-notice-pinned-manifest-has-siblings",
        "scrivener-notice-legacy-v1-files-beside",
        "scrivener-notice-mixed-payload-trees",
        "scrivener-notice-id-scheme-mismatch",
        "scrivener-notice-version-stamp-mismatch",
        "scrivener-notice-unrecognised-writer",
        "scrivener-notice-invalid-item-identifier",
        "scrivener-notice-items-without-identifier",
        "scrivener-notice-duplicate-item-key",
        "scrivener-notice-lock-present",
        "scrivener-notice-archived-while-open",
        "scrivener-notice-content-missing",
        "scrivener-notice-content-truncated",
        "scrivener-notice-missing-media",
        "scrivener-notice-sync-conflict-copies",
        "scrivener-notice-recovered-unreferenced-text",
        "scrivener-notice-sync-debris-ignored",
        "scrivener-notice-unrecognised-files",
        "scrivener-notice-symlink-not-followed",
        "scrivener-notice-unreadable-files-skipped",
        "scrivener-notice-cloud-placeholders-skipped",
        "scrivener-notice-no-text-found",
        "scrivener-notice-metadata-unreadable",
        "scrivener-notice-dangling-references",
        "scrivener-notice-snapshots-not-imported",
        "scrivener-notice-writing-history-not-imported",
        "scrivener-notice-title-placeholder-rejected",
        "scrivener-notice-author-placeholder-rejected",
        "scrivener-notice-title-from-project",
        "scrivener-notice-author-from-project",
        "scrivener-notice-language-not-stated",
        "scrivener-notice-language-from-script",
    ];

    /// The stable key the interface words this notice by.
    pub fn key(&self) -> &'static str {
        match self {
            Notice::ProjectFoundBelowPick { .. } => "scrivener-notice-project-found-below-pick",
            Notice::SourceIsTemplate { .. } => "scrivener-notice-source-is-template",
            Notice::SourceIsBackup { .. } => "scrivener-notice-source-is-backup",
            Notice::ImportingPreUpgradeCopy => "scrivener-notice-importing-pre-upgrade-copy",
            Notice::ImportedFromIosStub => "scrivener-notice-imported-from-ios-stub",
            Notice::StaleMobileBinderIgnored => "scrivener-notice-stale-mobile-binder-ignored",
            Notice::IdenticalManifestCopies { .. } => "scrivener-notice-identical-manifest-copies",
            Notice::ManifestCopiesIgnored { .. } => "scrivener-notice-manifest-copies-ignored",
            Notice::PinnedManifestLooksLikeACopy { .. } => {
                "scrivener-notice-pinned-manifest-looks-like-a-copy"
            }
            Notice::PinnedManifestHasSiblings { .. } => {
                "scrivener-notice-pinned-manifest-has-siblings"
            }
            Notice::LegacyV1FilesBeside { .. } => "scrivener-notice-legacy-v1-files-beside",
            Notice::MixedPayloadTrees => "scrivener-notice-mixed-payload-trees",
            Notice::IdSchemeMismatch { .. } => "scrivener-notice-id-scheme-mismatch",
            Notice::VersionStampMismatch { .. } => "scrivener-notice-version-stamp-mismatch",
            Notice::UnrecognisedWriter { .. } => "scrivener-notice-unrecognised-writer",
            Notice::InvalidItemIdentifier { .. } => "scrivener-notice-invalid-item-identifier",
            Notice::ItemsWithoutIdentifier { .. } => "scrivener-notice-items-without-identifier",
            Notice::DuplicateItemKey { .. } => "scrivener-notice-duplicate-item-key",
            Notice::LockPresent { .. } => "scrivener-notice-lock-present",
            Notice::ArchivedWhileOpen => "scrivener-notice-archived-while-open",
            Notice::ContentMissing { .. } => "scrivener-notice-content-missing",
            Notice::ContentTruncated { .. } => "scrivener-notice-content-truncated",
            Notice::MissingMedia { .. } => "scrivener-notice-missing-media",
            Notice::SyncConflictCopies { .. } => "scrivener-notice-sync-conflict-copies",
            Notice::RecoveredUnreferencedText { .. } => {
                "scrivener-notice-recovered-unreferenced-text"
            }
            Notice::SyncDebrisIgnored { .. } => "scrivener-notice-sync-debris-ignored",
            Notice::UnrecognisedFiles { .. } => "scrivener-notice-unrecognised-files",
            Notice::SymlinkNotFollowed { .. } => "scrivener-notice-symlink-not-followed",
            Notice::UnreadableFilesSkipped { .. } => "scrivener-notice-unreadable-files-skipped",
            Notice::CloudPlaceholdersSkipped { .. } => {
                "scrivener-notice-cloud-placeholders-skipped"
            }
            Notice::NoTextFound { .. } => "scrivener-notice-no-text-found",
            Notice::MetadataUnreadable { .. } => "scrivener-notice-metadata-unreadable",
            Notice::DanglingReferences { .. } => "scrivener-notice-dangling-references",
            Notice::SnapshotsNotImported { .. } => "scrivener-notice-snapshots-not-imported",
            Notice::WritingHistoryNotImported { .. } => {
                "scrivener-notice-writing-history-not-imported"
            }
            Notice::TitlePlaceholderRejected { .. } => {
                "scrivener-notice-title-placeholder-rejected"
            }
            Notice::AuthorPlaceholderRejected { .. } => {
                "scrivener-notice-author-placeholder-rejected"
            }
            Notice::TitleFromProject { .. } => "scrivener-notice-title-from-project",
            Notice::AuthorFromProject { .. } => "scrivener-notice-author-from-project",
            Notice::LanguageNotStated => "scrivener-notice-language-not-stated",
            Notice::LanguageFromScript { .. } => "scrivener-notice-language-from-script",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The notices about what a project states and what stays in Scrivener have their keys
    /// listed, once each, with the rest.
    #[test]
    fn the_metadata_notices_have_listed_keys() {
        let notices = [
            Notice::SnapshotsNotImported {
                snapshots: 2,
                documents: 1,
                of_deleted: 0,
            },
            Notice::WritingHistoryNotImported {
                days: 3,
                first_day: Some("2024-03-02".to_string()),
                last_day: Some("2024-04-01".to_string()),
                unreadable: 0,
            },
            Notice::TitlePlaceholderRejected {
                source: TitleSource::CompileSettings,
            },
            Notice::AuthorPlaceholderRejected {
                source: AuthorSource::ProjectProperties,
            },
            Notice::TitleFromProject {
                source: TitleSource::ProjectProperties,
            },
            Notice::AuthorFromProject {
                source: AuthorSource::CompileSettings,
            },
            Notice::LanguageNotStated,
            Notice::LanguageFromScript {
                tag: "ja".to_string(),
            },
        ];
        for notice in &notices {
            assert!(Notice::KEYS.contains(&notice.key()), "{notice:?}");
        }
        let mut keys = Notice::KEYS.to_vec();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), Notice::KEYS.len(), "a key listed twice");
        assert!(
            Notice::KEYS
                .iter()
                .all(|k| k.starts_with("scrivener-notice-"))
        );
    }
}
