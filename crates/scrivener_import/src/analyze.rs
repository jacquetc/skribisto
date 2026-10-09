// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `analyze`: from the path a writer picked to a read project, a question, or a
//! refusal.
//!
//! The steps, in order, each able to stop the analysis:
//!
//! 1. **The pick.** A folder, a file inside one, a manifest, an archive or a
//!    template, recognised by content. A folder with no project
//!    at its top is searched two levels down; a folder inside a project offers
//!    the project; several projects ask which.
//! 2. **The copy.** A folder named `<X> Backup` beside an upgraded `<X>` asks which
//!    to import. Nothing is ever merged.
//! 3. **The manifest.** One is read; several are chosen between automatically only
//!    on converging evidence (Scrivener's own last autosave matches one, it alone
//!    carries the folder's name, every other one is a sync copy of it, and no
//!    other one lists writing it does not), and asked about otherwise.
//! 4. **The generation.** Decided by the manifest's `Version` and corroborated by
//!    `Files/version.txt`, failing closed above what is known: a project read with
//!    an older reader looks imported and is quietly wrong. Scrivener 1 for Mac is
//!    refused, by decision. A damaged manifest is refused until spare binders are
//!    read.
//! 5. **The model.** The binder and project blocks, each item's files, snapshots
//!    counted, what Scrivener's search index still remembers for missing files,
//!    and every other file of the project classified.
//! 6. **Consistency.** Everything read is looked at again; if Scrivener or a sync
//!    service wrote meanwhile, the whole analysis runs once more, and a project
//!    still moving is refused.
//!
//! Nothing here writes anywhere, and nothing is read outside the project folder
//! except the listings that find it (the folder above an internal folder the
//! writer picked, and the sibling of a pre-upgrade copy).

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use serde::{Deserialize, Serialize};

use crate::completeness;
use crate::content::{self, ContentFacts};
use crate::disk::{Cancellable, Disk, EntryKind, RealDisk};
use crate::index::{Index, MAX_MEMBER_BYTES, MAX_TEXT_BYTES, ReadError, file_name, join};
use crate::layout::{self, Layout, Referenced, Tree, uuid_name};
use crate::lock;
use crate::manifest::{self, ManifestRead};
use crate::metadata::{self, Proposals, Refused, TitleSource};
use crate::model::{
    BinderRow, CompileFacts, CompileFile, CompileLanguage, Container, Counts, Generation, IdScheme,
    ItemKey, ItemKind, ManifestChoice, ManifestOrigin, MemberRef, Platform, Project, ProjectNote,
    SettingsFiles, SourceFacts, Stamp, Writer,
};
use crate::names::{conflict_original, folded, nfc};
use crate::outcome::{
    DanglingKind, ManifestCandidate, ManifestEvidence, MobileEdits, Notice, ProjectCandidate,
    Question, Refusal, TemplateDamage,
};
use crate::pick::{self, Sniffed, Strength};
use crate::structure::{self, StructurePlan};
use crate::template;
use crate::xml;

/// The newest `Files/version.txt` this importer knows (Scrivener 3.5).
pub const NEWEST_STAMP: u32 = 23;
/// The largest manifest read.
pub(crate) const MAX_MANIFEST_BYTES: u64 = 64 << 20;
/// Small metadata files: a lock, a version stamp, a settings file.
const MAX_SMALL_BYTES: u64 = 1 << 20;

/// The writer's answers to the questions an earlier analysis asked.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Answers {
    /// Which project, when the pick held several: a candidate's `path`.
    pub project: Option<String>,
    /// Which manifest, when the project holds several: a candidate's `name`.
    pub manifest: Option<String>,
    /// Import the project the pick sits inside.
    pub use_enclosing_project: bool,
    /// Which of a pre-upgrade copy and its upgraded sibling to import.
    pub upgraded_sibling: Option<SiblingChoice>,
    /// Import the desktop state without the unmerged iPhone or iPad edits.
    pub accept_unmerged_mobile_edits: bool,
    /// Import a copy whose documents are missing, as empty documents.
    pub accept_no_text: bool,
}

/// Which copy the writer chose when a pre-upgrade copy sits beside its upgraded
/// project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SiblingChoice {
    /// The upgraded project beside the pick.
    Sibling,
    /// The pre-upgrade copy picked.
    Backup,
}

/// What the analysis needs from its surroundings.
#[derive(Clone, PartialEq, Eq)]
pub struct Options {
    /// This computer's host name, to tell a lock left here from one left
    /// elsewhere. Compared, never kept, and withheld from `Debug` as the card is.
    pub this_host: Option<String>,
}

impl std::fmt::Debug for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Options")
            .field("this_host", &self.this_host.as_ref().map(|_| "<withheld>"))
            .finish()
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            this_host: lock::this_host(),
        }
    }
}

/// The result of an analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Analysis {
    /// The project cannot be imported.
    Refused {
        refusal: Refusal,
        notices: Vec<Notice>,
    },
    /// Nothing more can be read until the writer chooses.
    Choose {
        question: Question,
        notices: Vec<Notice>,
    },
    /// The project was read.
    Read(Box<ReadProject>),
}

/// A project read, with what the writer should know and what they must still
/// accept.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadProject {
    pub project: Project,
    /// What each document holds, measured once: the structure mapping, the
    /// review's preview and the conversion all decide on these.
    pub content: ContentFacts,
    /// The structure mapping with the detected defaults, which the review step
    /// shows and lets the writer change ([`structure::adjust`]).
    pub structure: StructurePlan,
    /// The title, authors and language the project states, for the Save page to pre-fill
    /// (D4-15): the writer confirms or changes each, and the conversion takes their answer.
    pub proposals: Proposals,
    pub card: Card,
    pub notices: Vec<Notice>,
    /// Questions the writer must accept before importing; empty when none.
    pub pending: Vec<Question>,
    /// A digest of the manifest and of every file the model refers to, so an
    /// import can tell the project did not change since this analysis.
    pub fingerprint: String,
    /// Where the project read is on disk: its folder, or the archive or template file it
    /// came in. Not always the path the writer picked, which may be a folder around the
    /// project, a file inside it, a link to it, or a pre-upgrade copy beside the project
    /// read instead: what is beside the project, and what is inside it, is decided from
    /// this.
    #[serde(default)]
    pub location: PathBuf,
}

/// What the project card may show and nothing may keep: personal data.
///
/// The manifest's `Device` and the lock file's host name the writer's
/// computers. They may be shown to the writer before the import; they are never
/// written into the imported project, a log line or a notice's payload. So its
/// `Debug` says only whether each is known: a `{:?}` of an analysis is the first
/// thing that ends up in a log.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    pub device: Option<String>,
    pub lock_host: Option<String>,
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let withheld = |value: &Option<String>| value.as_ref().map(|_| "<withheld>");
        f.debug_struct("Card")
            .field("device", &withheld(&self.device))
            .field("lock_host", &withheld(&self.lock_host))
            .finish()
    }
}

impl Analysis {
    /// The analysis, with the project it read located at `at` when it read one.
    fn located(mut self, at: &Path) -> Self {
        if let Analysis::Read(read) = &mut self {
            read.location = at.to_path_buf();
        }
        self
    }

    /// Every notice the analysis produced.
    pub fn notices(&self) -> &[Notice] {
        match self {
            Analysis::Refused { notices, .. } | Analysis::Choose { notices, .. } => notices,
            Analysis::Read(read) => &read.notices,
        }
    }
}

/// Analyse the Scrivener project at `path`.
pub fn analyze(path: &Path, answers: &Answers, options: &Options) -> Analysis {
    analyze_with(&RealDisk, path, answers, options)
}

/// [`analyze`], stopped when `cancel` is set: `None` then. A folder project stops at the
/// next file it would have read, an archive or a template at the next member.
pub fn analyze_cancellable(
    path: &Path,
    answers: &Answers,
    options: &Options,
    cancel: &AtomicBool,
) -> Option<Analysis> {
    if cancel.load(Ordering::Relaxed) {
        return None;
    }
    let disk = Cancellable {
        inner: &RealDisk,
        cancel,
    };
    let analysis = analyze_with(&disk, path, answers, options);
    (!cancel.load(Ordering::Relaxed)).then_some(analysis)
}

/// [`analyze`] over any disk: the seam tests use.
pub(crate) fn analyze_with(
    disk: &dyn Disk,
    path: &Path,
    answers: &Answers,
    options: &Options,
) -> Analysis {
    analyze_visiting(disk, path, answers, options, &mut None)
}

/// A member moved while a caller of [`analyze_visiting`] read it.
pub(crate) struct Moved;

/// What a caller does with a project once it is read, while it is still open: the
/// conversion. Its reads go through the analysis's own index, so the check that nothing
/// moved while the project was read covers them too, and a project a sync service writes
/// to during the conversion is read again, then refused, as it would be during the review.
pub(crate) type Visit<'v> =
    Option<&'v mut dyn FnMut(&Tree<'_, '_>, &ReadProject) -> Result<(), Moved>>;

/// [`analyze_with`], handing the project read to `visit` before its reads are checked.
/// `visit` runs once per attempt: a second time when the first read found the project
/// moving.
pub(crate) fn analyze_visiting(
    disk: &dyn Disk,
    path: &Path,
    answers: &Answers,
    options: &Options,
    visit: &mut Visit<'_>,
) -> Analysis {
    match attempt(disk, path, answers, options, visit) {
        Attempt::Done(analysis) => analysis,
        // Something wrote while the project was read: read it all again, once.
        Attempt::Changed(_) => match attempt(disk, path, answers, options, visit) {
            Attempt::Done(analysis) => analysis,
            Attempt::Changed(count) => Analysis::Refused {
                refusal: Refusal::ProjectChangedDuringRead { count },
                notices: Vec::new(),
            },
        },
    }
}

enum Attempt {
    Done(Analysis),
    /// Members moved while they were read; how many.
    Changed(u32),
}

/// Why an evaluation stopped before the model was complete.
enum Stop {
    Refuse(Refusal),
    Choose(Question),
    Changed(u32),
}

impl From<Refusal> for Stop {
    fn from(refusal: Refusal) -> Self {
        Stop::Refuse(refusal)
    }
}

fn finish(result: Result<Analysis, Stop>, notices: Vec<Notice>) -> Attempt {
    match result {
        Ok(analysis) => Attempt::Done(analysis),
        Err(Stop::Refuse(refusal)) => Attempt::Done(Analysis::Refused { refusal, notices }),
        Err(Stop::Choose(question)) => Attempt::Done(Analysis::Choose { question, notices }),
        Err(Stop::Changed(count)) => Attempt::Changed(count),
    }
}

fn attempt(
    disk: &dyn Disk,
    pick: &Path,
    answers: &Answers,
    options: &Options,
    visit: &mut Visit<'_>,
) -> Attempt {
    let mut notices = Vec::new();
    let result = open_pick(disk, pick, answers, options, &mut notices, visit);
    finish(result, notices)
}

fn open_pick(
    disk: &dyn Disk,
    pick: &Path,
    answers: &Answers,
    options: &Options,
    notices: &mut Vec<Notice>,
    visit: &mut Visit<'_>,
) -> Result<Analysis, Stop> {
    let meta = match disk.metadata_following(pick) {
        Ok(meta) => meta,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(Refusal::Missing {
                file: file_name(pick),
            }
            .into());
        }
        Err(e) => {
            return Err(Refusal::Unreadable {
                file: file_name(pick),
                reason: e.to_string(),
            }
            .into());
        }
    };
    // A link the writer picked (a manifest, an archive or a project linked from
    // somewhere handier) is followed once, here, and the project read where it
    // leads; no link inside the project is ever followed. Read through the link
    // itself, a manifest's folder would be the link's, and a file would not
    // open at all.
    let resolved;
    let pick = if disk
        .metadata(pick)
        .is_ok_and(|m| m.kind == EntryKind::Symlink)
    {
        resolved = disk.resolve(pick).map_err(|e| Refusal::Unreadable {
            file: file_name(pick),
            reason: e.to_string(),
        })?;
        resolved.as_path()
    } else {
        pick
    };
    if let Some(sign) = pick::backup_sign(pick) {
        notices.push(Notice::SourceIsBackup { sign });
    }
    match meta.kind {
        EntryKind::Dir => folder_pick(disk, pick, None, answers, options, notices, visit),
        EntryKind::File => file_pick(disk, pick, answers, options, notices, visit),
        EntryKind::Symlink | EntryKind::Other => Err(Refusal::NotAScrivenerProject.into()),
    }
}

fn folder_pick(
    disk: &dyn Disk,
    pick: &Path,
    pinned: Option<String>,
    answers: &Answers,
    options: &Options,
    notices: &mut Vec<Notice>,
    visit: &mut Visit<'_>,
) -> Result<Analysis, Stop> {
    let listing = pick::list_disk(disk, pick).map_err(|e| Refusal::Unreadable {
        file: file_name(pick),
        reason: e.to_string(),
    })?;
    let root = if pinned.is_some() || listing.strength() == Strength::Strong {
        pick.to_path_buf()
    } else if let Some((enclosing, levels_up)) = pick::enclosing_project(disk, pick) {
        if !answers.use_enclosing_project {
            return Err(Stop::Choose(Question::EnclosingProject {
                project: file_name(&enclosing),
                levels_up,
            }));
        }
        enclosing
    } else {
        let found = pick::search_disk(disk, pick, pick::FOLDER_SEARCH_DEPTH);
        let rel = choose_root(&found, answers, notices, |weak| {
            weak_refusal_disk(disk, pick, weak)
        })?;
        if rel.is_empty() {
            pick.to_path_buf()
        } else {
            pick.join(&rel)
        }
    };
    // Reading the upgraded sibling instead reads its own manifest, whatever the
    // pre-upgrade copy's manifest the writer picked was called.
    let (root, pinned) = match upgraded_sibling(disk, &root, answers, notices)? {
        Some(sibling) => (sibling, None),
        None => (root, pinned),
    };
    let index = Index::of_folder(disk, &root)?;
    let tree = Tree {
        index: &index,
        root: String::new(),
    };
    let result = evaluate(
        &tree,
        &Context {
            container: Container::Folder,
            folder_name: file_name(&root),
            pinned,
            template_title: None,
        },
        answers,
        options,
        notices,
        visit,
    );
    settle(&index, result).map(|analysis| analysis.located(&root))
}

/// Check the index for members that moved, and turn the evaluation into an
/// analysis.
fn settle(index: &Index<'_>, result: Result<Analysis, Stop>) -> Result<Analysis, Stop> {
    let moved = index.changed_members();
    if !moved.is_empty() {
        return Err(Stop::Changed(
            u32::try_from(moved.len()).unwrap_or(u32::MAX),
        ));
    }
    result
}

/// Pick the project root among what a search found.
fn choose_root(
    found: &pick::Found,
    answers: &Answers,
    notices: &mut Vec<Notice>,
    weak_refusal: impl FnOnce(&str) -> Refusal,
) -> Result<String, Stop> {
    if found.too_wide {
        return Err(Refusal::SearchTooWide {
            limit: pick::SEARCH_LIMIT,
        }
        .into());
    }
    let chosen = match found.strong.as_slice() {
        [] => {
            return Err(match found.weak.first() {
                Some(weak) => weak_refusal(weak),
                None => Refusal::NotAScrivenerProject,
            }
            .into());
        }
        [only] => only.clone(),
        several => match &answers.project {
            Some(answer) if several.contains(answer) => answer.clone(),
            _ => {
                return Err(Stop::Choose(Question::SeveralProjects {
                    candidates: several
                        .iter()
                        .map(|path| ProjectCandidate { path: path.clone() })
                        .collect(),
                }));
            }
        },
    };
    if !chosen.is_empty() {
        notices.push(Notice::ProjectFoundBelowPick {
            path: chosen.clone(),
        });
    }
    Ok(chosen)
}

/// The refusal for a folder that holds only a manifest: an iOS stub without its
/// data, or a manifest without the project's files.
fn weak_refusal_disk(disk: &dyn Disk, pick: &Path, weak: &str) -> Refusal {
    let folder = if weak.is_empty() {
        pick.to_path_buf()
    } else {
        pick.join(weak)
    };
    let manifests = pick::list_disk(disk, &folder)
        .map(|l| l.manifests())
        .unwrap_or_default();
    let stub = manifests.len() == 1
        && disk
            .read_head(&folder.join(&manifests[0]), 64 << 10)
            .ok()
            .and_then(|head| manifest::header(&head))
            .is_some_and(|h| h.is_mobile_stub());
    weak_refusal(manifests.first().cloned(), stub)
}

fn weak_refusal(manifest: Option<String>, stub: bool) -> Refusal {
    if stub {
        Refusal::IosStubWithoutMobileData
    } else {
        Refusal::ManifestWithoutFiles {
            manifest: manifest.unwrap_or_default(),
        }
    }
}

fn file_pick(
    disk: &dyn Disk,
    pick: &Path,
    answers: &Answers,
    options: &Options,
    notices: &mut Vec<Notice>,
    visit: &mut Visit<'_>,
) -> Result<Analysis, Stop> {
    let name = file_name(pick);
    let lower = name.to_lowercase();
    // A bare file name has an empty parent: the current folder.
    let parent = match pick.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
        _ => PathBuf::from("."),
    };
    // Scrivener 1's binder or its info.plist: the folder around it is the
    // project, whatever generation it turns out to be.
    if lower == "binder.scrivproj" || lower == "info.plist" {
        return folder_pick(disk, &parent, None, answers, options, notices, visit);
    }
    // A file only ever found inside a project's own folders.
    if matches!(lower.as_str(), "binder.autosave" | "binder.backup")
        && let Some((enclosing, levels_up)) = pick::enclosing_project(disk, pick)
    {
        if !answers.use_enclosing_project {
            return Err(Stop::Choose(Question::EnclosingProject {
                project: file_name(&enclosing),
                levels_up,
            }));
        }
        return folder_pick(disk, &enclosing, None, answers, options, notices, visit);
    }
    let head = disk
        .read_head(pick, 4096)
        .map_err(|e| Refusal::Unreadable {
            file: name.clone(),
            reason: e.to_string(),
        })?;
    match pick::sniff(&head) {
        Sniffed::Zip => archive_pick(disk, pick, answers, options, notices, visit),
        Sniffed::Template => template_pick(disk, pick, answers, options, notices, visit),
        Sniffed::Manifest => manifest_pick(disk, &parent, name, answers, options, notices, visit),
        Sniffed::BinaryPlist | Sniffed::Other if lower.ends_with(".scrivx") => {
            manifest_pick(disk, &parent, name, answers, options, notices, visit)
        }
        Sniffed::BinaryPlist | Sniffed::Other => Err(Refusal::NotAScrivenerProject.into()),
    }
}

/// A picked manifest: its folder is the project, and that manifest is the one
/// read, whatever sits beside it.
fn manifest_pick(
    disk: &dyn Disk,
    parent: &Path,
    name: String,
    answers: &Answers,
    options: &Options,
    notices: &mut Vec<Notice>,
    visit: &mut Visit<'_>,
) -> Result<Analysis, Stop> {
    // A stale manifest inside a project's own folders (`X.scriv/Files/Workflow.scrivx`,
    // inherited from a template) means the project around it.
    let parent_name = file_name(parent);
    if crate::names::is_internal_folder(&parent_name)
        && let Some((enclosing, levels_up)) = pick::enclosing_project(disk, parent)
    {
        if !answers.use_enclosing_project {
            return Err(Stop::Choose(Question::EnclosingProject {
                project: file_name(&enclosing),
                levels_up: levels_up + 1,
            }));
        }
        return folder_pick(disk, &enclosing, None, answers, options, notices, visit);
    }
    folder_pick(disk, parent, Some(name), answers, options, notices, visit)
}

fn archive_pick(
    disk: &dyn Disk,
    pick: &Path,
    answers: &Answers,
    options: &Options,
    notices: &mut Vec<Notice>,
    visit: &mut Visit<'_>,
) -> Result<Analysis, Stop> {
    let index = Index::of_archive_file(disk, pick)?;
    archive_root(
        &index,
        Container::Zip,
        None,
        answers,
        options,
        notices,
        visit,
    )
    .map(|analysis| analysis.located(pick))
}

fn template_pick(
    disk: &dyn Disk,
    pick: &Path,
    answers: &Answers,
    options: &Options,
    notices: &mut Vec<Notice>,
    visit: &mut Visit<'_>,
) -> Result<Analysis, Stop> {
    let file = file_name(pick);
    let meta = disk.metadata(pick).map_err(|e| Refusal::Unreadable {
        file: file.clone(),
        reason: e.to_string(),
    })?;
    if meta.len > template::MAX_TEMPLATE_BYTES {
        return Err(Refusal::TemplateUndecodable {
            file,
            damage: TemplateDamage::TooLarge {
                limit_mib: template::MAX_TEMPLATE_BYTES >> 20,
            },
        }
        .into());
    }
    let bytes = disk
        .read(pick, template::MAX_TEMPLATE_BYTES)
        .map_err(|e| Refusal::Unreadable {
            file: file.clone(),
            reason: e.to_string(),
        })?;
    let decoded =
        template::decode(&file, &bytes).map_err(|damage| Refusal::TemplateUndecodable {
            file: file.clone(),
            damage,
        })?;
    drop(bytes);
    notices.push(Notice::SourceIsTemplate {
        title: decoded.title.clone(),
    });
    let mut index = Index::of_archive_bytes(disk, decoded.zip, file)?;
    index.set_container_file(pick.to_path_buf(), meta);
    archive_root(
        &index,
        Container::Template(decoded.encoding),
        decoded.title,
        answers,
        options,
        notices,
        visit,
    )
    .map(|analysis| analysis.located(pick))
}

fn archive_root(
    index: &Index<'_>,
    container: Container,
    template_title: Option<String>,
    answers: &Answers,
    options: &Options,
    notices: &mut Vec<Notice>,
    visit: &mut Visit<'_>,
) -> Result<Analysis, Stop> {
    let found = pick::search_index(index, pick::ARCHIVE_SEARCH_DEPTH);
    let root = choose_root(&found, answers, notices, |weak| {
        let listing = pick::list_index(index, weak);
        let manifests = listing.manifests();
        let stub = manifests.len() == 1
            && index
                .read(&join(weak, &manifests[0]), MAX_MANIFEST_BYTES)
                .ok()
                .and_then(|bytes| manifest::header(&bytes))
                .is_some_and(|h| h.is_mobile_stub());
        weak_refusal(manifests.first().cloned(), stub)
    })?;
    let folder_name = if root.is_empty() {
        String::new()
    } else {
        root.rsplit('/').next().unwrap_or_default().to_string()
    };
    let tree = Tree { index, root };
    let result = evaluate(
        &tree,
        &Context {
            container,
            folder_name,
            pinned: None,
            template_title,
        },
        answers,
        options,
        notices,
        visit,
    );
    settle(index, result)
}

/// When `root` is a pre-upgrade copy with its upgraded project beside it, the
/// sibling to read instead if the writer chose it.
fn upgraded_sibling(
    disk: &dyn Disk,
    root: &Path,
    answers: &Answers,
    notices: &mut Vec<Notice>,
) -> Result<Option<PathBuf>, Stop> {
    let backup = file_name(root);
    let Some(names) = pick::upgraded_sibling_names(&backup) else {
        return Ok(None);
    };
    let Some(parent) = root.parent() else {
        return Ok(None);
    };
    let sibling = names.into_iter().map(|n| parent.join(n)).find(|candidate| {
        disk.metadata(candidate)
            .is_ok_and(|m| m.kind == EntryKind::Dir)
            && pick::list_disk(disk, candidate).is_ok_and(|l| l.strength() == Strength::Strong)
    });
    let Some(sibling) = sibling else {
        return Ok(None);
    };
    match answers.upgraded_sibling {
        Some(SiblingChoice::Sibling) => Ok(Some(sibling)),
        Some(SiblingChoice::Backup) => {
            notices.push(Notice::ImportingPreUpgradeCopy);
            Ok(None)
        }
        None => {
            let (generation, clean) = probe_generation(disk, &sibling);
            Err(Stop::Choose(Question::UpgradedSibling {
                backup,
                sibling: file_name(&sibling),
                sibling_generation: generation,
                preselect_sibling: clean,
            }))
        }
    }
}

/// A cheap look at a project folder: its generation from the manifest's start
/// tag, and whether it would pass the gate with no mismatch to report.
fn probe_generation(disk: &dyn Disk, folder: &Path) -> (Option<Generation>, bool) {
    let Ok(listing) = pick::list_disk(disk, folder) else {
        return (None, false);
    };
    let manifests = listing.manifests();
    let [only] = manifests.as_slice() else {
        return (None, false);
    };
    let Some(header) = disk
        .read_head(&folder.join(only), 64 << 10)
        .ok()
        .and_then(|head| manifest::header(&head))
    else {
        return (None, false);
    };
    if header.is_mobile_stub() {
        return (Some(Generation::IosStub), true);
    }
    let generation = match header.get("Version") {
        Some("1.0") => Generation::Format10,
        Some("1.5") => Generation::Format15,
        Some("2.0") => Generation::Format20,
        _ => return (None, false),
    };
    let stamp = disk
        .read_head(&folder.join("Files").join("version.txt"), 16)
        .ok()
        .and_then(|b| String::from_utf8_lossy(&b).trim().parse::<u32>().ok());
    let stamp_agrees = stamp.is_none_or(|s| s == expected_stamp(generation));
    let files = pick::list_disk(disk, &folder.join("Files")).ok();
    let mixed = files.is_some_and(|f| f.has_folder("Docs") && f.has_folder("Data"));
    (Some(generation), stamp_agrees && !mixed)
}

fn expected_stamp(generation: Generation) -> u32 {
    match generation {
        Generation::Format10 => 16,
        Generation::Format15 => 18,
        Generation::Format20 | Generation::IosStub => 23,
    }
}

/// What evaluation needs to know about how the project was reached.
struct Context {
    container: Container,
    folder_name: String,
    pinned: Option<String>,
    template_title: Option<String>,
}

/// One manifest of several, read for the choice between them.
///
/// What the choice needs is kept: its hash (for its bytes), its start tag, how
/// many items it lists and the names those items give their files. The manifest
/// read is kept only when it is the only one; among several, the chosen one is
/// read again, so a folder of large copies never sits in memory side by side
/// (a binder row is a kilobyte, and each copy can list a hundred thousand).
struct Candidate {
    name: String,
    hash: blake3::Hash,
    header: manifest::Header,
    /// Its binder items, or why it could not be read.
    items: Result<u32, crate::outcome::ManifestDamage>,
    /// The names its items give their files: a UUID as Scrivener names the
    /// item's folder, a number as the item's flat files begin.
    file_names: BTreeSet<String>,
    /// The manifest read, when it is the only candidate.
    read: Option<ManifestRead>,
    /// Why the manifest could not be read at all, when it could not.
    unreadable: Option<Refusal>,
}

/// The binder items the manifests at a project's top are read with, together:
/// what their file names cost to keep for the choice stays bounded however many
/// copies a folder holds.
const MAX_ITEMS_TOGETHER: usize = 2 * manifest::MAX_ITEMS;

/// Read a metadata member. One that cannot be read is said so, and costs only
/// what it held.
fn read_member(
    tree: &Tree<'_, '_>,
    rel: &str,
    limit: u64,
    notices: &mut Vec<Notice>,
) -> Result<Option<Vec<u8>>, Stop> {
    match tree.read(rel, limit) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(ReadError::Changed) => Err(Stop::Changed(1)),
        Err(ReadError::Refused(refusal)) => Err(Stop::Refuse(refusal)),
        Err(e) => {
            if let Some(problem) = e.problem() {
                notices.push(Notice::MetadataUnreadable {
                    member: rel.to_string(),
                    problem,
                });
            }
            Ok(None)
        }
    }
}

fn evaluate(
    tree: &Tree<'_, '_>,
    ctx: &Context,
    answers: &Answers,
    options: &Options,
    notices: &mut Vec<Notice>,
    visit: &mut Visit<'_>,
) -> Result<Analysis, Stop> {
    let top = pick::list_index(tree.index, &tree.root);
    let manifests = top.manifests();
    if manifests.is_empty() {
        return Err(if top.has_file("binder.scrivproj") {
            Refusal::ScrivenerOneForMac
        } else {
            Refusal::NotAScrivenerProject
        }
        .into());
    }
    let mut used: BTreeSet<String> = BTreeSet::new();
    let mut candidates = read_candidates(tree, &manifests, &mut used)?;
    if top.strength() == Strength::Weak {
        let pinned = ctx
            .pinned
            .as_deref()
            .and_then(|pinned| pinned_index(&candidates, pinned));
        let lone = pinned.unwrap_or(0);
        let stub = candidates
            .get(lone)
            .is_some_and(|c| c.header.is_mobile_stub());
        return Err(weak_refusal(candidates.get(lone).map(|c| c.name.clone()), stub).into());
    }

    // The manifest.
    let autosave = read_autosave(tree, &mut used, notices)?;
    let (chosen_index, choice) =
        choose_manifest(tree, &candidates, ctx, answers, autosave.as_ref(), notices)?;
    let mut chosen = candidates.swap_remove(chosen_index);
    drop(candidates);
    if let Some(refusal) = &chosen.unreadable {
        return Err(refusal.clone().into());
    }
    let stub = chosen.header.is_mobile_stub();
    let mut card = Card {
        device: chosen.header.get("Device").map(str::to_string),
        lock_host: None,
    };

    // The binder, from the manifest or, for an iOS stub, from Mobile/binder.mob.
    let (read, attrs, generation, origin) = if stub {
        let Some(mob) = tree.lookup("Mobile/binder.mob") else {
            return Err(Refusal::IosStubWithoutMobileData.into());
        };
        let Some(bytes) = read_member(tree, &mob, MAX_MANIFEST_BYTES, notices)? else {
            return Err(Refusal::IosStubWithoutMobileData.into());
        };
        used.insert(mob.clone());
        let read = manifest::read(&mob, &bytes, manifest::MAX_ITEMS)
            .map_err(|_| Refusal::IosStubWithoutMobileData)?;
        if read.attrs.get("Version").map(String::as_str) != Some("2.0") {
            return Err(Refusal::UnknownFormatVersion {
                version: read.attrs.get("Version").cloned().unwrap_or_default(),
            }
            .into());
        }
        if card.device.is_none() {
            card.device = read.attrs.get("Device").cloned();
        }
        notices.push(Notice::ImportedFromIosStub);
        let attrs = read.attrs.clone();
        (
            read,
            attrs,
            Generation::IosStub,
            ManifestChoice::MobileBinder,
        )
    } else {
        // The chosen manifest's rows are taken, never copied: a binder can hold
        // a hundred thousand rows of a kilobyte each.
        let parsed = match (chosen.read.take(), &chosen.items) {
            (Some(read), _) => Ok(read),
            (None, Err(damage)) => Err(damage.clone()),
            (None, Ok(_)) => read_again(tree, &chosen)?,
        };
        let read = match parsed {
            Ok(read) => read,
            Err(damage) => {
                let spare_copies = ["Files/binder.autosave", "Files/binder.backup"]
                    .iter()
                    .filter(|p| tree.lookup(p).is_some())
                    .count() as u32;
                return Err(Refusal::DamagedManifest {
                    manifest: chosen.name.clone(),
                    damage,
                    spare_copies,
                }
                .into());
            }
        };
        let generation = match read.attrs.get("Version").map(String::as_str) {
            Some("1.0") => Generation::Format10,
            Some("1.5") => Generation::Format15,
            Some("2.0") => Generation::Format20,
            other => {
                return Err(Refusal::UnknownFormatVersion {
                    version: other.unwrap_or_default().to_string(),
                }
                .into());
            }
        };
        let attrs = read.attrs.clone();
        (read, attrs, generation, choice)
    };
    let version = attrs.get("Version").cloned().unwrap_or_default();

    // The version stamp: corroboration, and the ceiling.
    let mut version_stamp = None;
    if !stub {
        if let Some(rel) = tree.lookup("Files/version.txt") {
            // A stamp that cannot be read cannot say the project is not newer
            // than this importer: the gate fails closed.
            let bytes = match tree.read(&rel, 64) {
                Ok(bytes) => bytes,
                Err(ReadError::Changed) => return Err(Stop::Changed(1)),
                Err(_) => return Err(Refusal::VersionStampUnreadable.into()),
            };
            used.insert(rel);
            let text = String::from_utf8_lossy(&bytes);
            let stamp: u32 = text
                .trim()
                .parse()
                .map_err(|_| Refusal::VersionStampUnreadable)?;
            if stamp > NEWEST_STAMP {
                return Err(Refusal::NewerFormatStamp {
                    stamp,
                    newest: NEWEST_STAMP,
                }
                .into());
            }
            if stamp != expected_stamp(generation) {
                notices.push(Notice::VersionStampMismatch {
                    stamp,
                    version: version.clone(),
                });
            }
            version_stamp = Some(stamp);
        }
        // A desktop manifest found beside `Mobile/` alone: the desktop files are
        // not here.
        if !tree
            .lookup("Files")
            .is_some_and(|files| tree.is_dir(&files))
        {
            return Err(Refusal::ManifestWithoutFiles {
                manifest: chosen.name.clone(),
            }
            .into());
        }
    }

    let writer = writer_of(attrs.get("Creator").map(String::as_str));
    if writer.platform == Platform::Unknown {
        notices.push(Notice::UnrecognisedWriter {
            creator: writer.creator.clone(),
        });
    }

    // Identifiers: the grammar, the scheme, duplicates.
    let ManifestRead {
        mut rows,
        blocks,
        invalid_identifiers,
        ..
    } = read;
    if invalid_identifiers > 0 {
        notices.push(Notice::InvalidItemIdentifier {
            count: invalid_identifiers,
        });
    }
    let id_scheme = id_scheme(&rows, generation);
    if id_scheme != expected_scheme(generation) {
        notices.push(Notice::IdSchemeMismatch {
            version: version.clone(),
        });
    }
    let layout = Layout::for_scheme(id_scheme, stub);
    let keyless = rows
        .iter()
        .filter(|row| match layout {
            Layout::Flat => row.number.is_none(),
            Layout::Folders(_) => !matches!(row.key, Some(ItemKey::Uuid(_))),
        })
        .count() as u32;
    if keyless as usize == rows.len() {
        return Err(Refusal::NoItemIdentifiers.into());
    }
    if keyless > 0 {
        notices.push(Notice::ItemsWithoutIdentifier { count: keyless });
    }
    mark_duplicates(&mut rows, notices);
    let number_keys = normalise_references(&mut rows);
    let mut blocks = blocks;
    renumber_blocks(&mut blocks, &number_keys);

    // Where the files are.
    let resolution = layout::resolve_payloads(tree, &mut rows, layout);
    used.extend(resolution.used.iter().cloned());
    if !stub && tree.lookup("Files/Docs").is_some() && tree.lookup("Files/Data").is_some() {
        notices.push(Notice::MixedPayloadTrees);
    }
    let text_items = rows.iter().filter(|r| r.kind == ItemKind::Text).count() as u32;
    if !stub && version_stamp.is_none() && !layout::has_payload(tree) && text_items > 0 {
        return Err(Refusal::IncompleteCopy.into());
    }
    if let Some(link) = resolution.blocked.links.first() {
        return Err(Refusal::NeededMemberIsLink {
            member: link.clone(),
        }
        .into());
    }
    if !resolution.blocked.placeholders.is_empty() {
        return Err(Refusal::CloudPlaceholders {
            count: resolution.blocked.placeholders.len() as u32,
            examples: resolution
                .blocked
                .placeholders
                .iter()
                .take(3)
                .cloned()
                .collect(),
        }
        .into());
    }
    if !resolution.blocked.unreadable.is_empty() {
        let mut folders = resolution.blocked.unreadable.clone();
        folders.sort();
        folders.dedup();
        return Err(Refusal::UnreadableMembers {
            count: folders.len() as u32,
            examples: folders.into_iter().take(3).collect(),
        }
        .into());
    }

    // What is missing, and what the search index remembers of it.
    let found = completeness::check(tree, &mut rows, layout);
    if found.changed {
        return Err(Stop::Changed(1));
    }
    used.extend(found.used.iter().cloned());
    notices.extend(found.notices.iter().cloned());
    for key in &resolution.missing_media {
        notices.push(Notice::MissingMedia { item: *key });
    }

    let mut counts = Counts::default();
    layout::count_snapshots(tree, &mut rows, &mut counts);

    // Project-level files the later steps read.
    let project_notes = read_project_notes(tree, &mut used, notices)?;
    let mut settings = SettingsFiles {
        compile_xml: member(tree, "Settings/compile.xml", &mut used),
        compile_plist: member(tree, "Settings/compile.plist", &mut used),
        compile_ini: member(tree, "Settings/compile.ini", &mut used),
        template_info: member(tree, "Settings/templateinfo.xml", &mut used),
        project_preferences: member(tree, "Settings/projectpreferences.xml", &mut used),
        compile_formats: Vec::new(),
    };
    let compile = read_compile(tree, &mut settings, &mut used, notices)?;
    let created_from_template = match &settings.template_info {
        Some(info) => template_info_title(tree, info, notices)?,
        None => None,
    };
    let styles = if stub {
        None
    } else {
        member(tree, "Files/styles.xml", &mut used)
    };
    let writing_history = if stub {
        member(tree, "Mobile/writing.history", &mut used)
    } else {
        member(tree, "Files/writing.history", &mut used)
    };
    let custom_icons = match tree.lookup("Icons") {
        Some(icons) => tree
            .children(&icons)
            .into_iter()
            .filter_map(|name| tree.member_ref(&join(&icons, &name)))
            .collect(),
        None => Vec::new(),
    };

    // The lock.
    if let Some(rel) = tree.lookup("Files/user.lock")
        && let Some(bytes) = read_member(tree, &rel, MAX_SMALL_BYTES, notices)?
    {
        used.insert(rel);
        let lock = lock::parse(&bytes);
        if tree.index.is_archive() {
            notices.push(Notice::ArchivedWhileOpen);
        } else {
            let this_machine = match (&lock.host, &options.this_host) {
                (Some(host), Some(here)) => lock::names_this_machine(host, here),
                _ => false,
            };
            notices.push(Notice::LockPresent { this_machine });
        }
        card.lock_host = lock.host;
    }

    // What stays in the Scrivener project: its snapshots and its daily writing history
    // (decision 7, D6.8), counted so the report can say so.
    if counts.snapshots + counts.orphan_snapshots > 0 {
        notices.push(Notice::SnapshotsNotImported {
            snapshots: counts.snapshots + counts.orphan_snapshots,
            documents: counts.items_with_snapshots,
            of_deleted: counts.orphan_snapshots,
        });
    }
    let history = match &writing_history {
        Some(member) => read_writing_history(tree, member, notices)?,
        None => WritingDays::default(),
    };
    let history = history.or_recent(blocks.recent_writing_day.as_ref());
    if history.days > 0 {
        notices.push(Notice::WritingHistoryNotImported {
            days: history.days,
            first_day: history.first.map(|d| d.to_string()),
            last_day: history.last.map(|d| d.to_string()),
            unreadable: history.unreadable,
        });
    }

    // The project's name as a title, never keeping a computer's name the project gives.
    let hosts: Vec<String> = [&options.this_host, &card.lock_host, &card.device]
        .into_iter()
        .flatten()
        .cloned()
        .collect();
    let named = if ctx.folder_name.is_empty() {
        tree.index
            .container_facts()
            .map(|(name, _, _)| name)
            .unwrap_or_default()
    } else {
        ctx.folder_name.clone()
    };
    let upgradable = matches!(generation, Generation::Format10 | Generation::Format15);
    let folder_title = crate::names::title_from_name(&named, &hosts, upgradable);

    // Edits made on an iPhone or iPad that a desktop never merged.
    let mut pending = Vec::new();
    if !stub {
        mobile_questions(tree, &attrs, answers, &mut pending, notices, &mut used)?;
    }

    // Everything else in the project.
    let (folders, numbers) = owned_keys(&rows, layout);
    let debris = layout::classify(
        tree,
        &used,
        &Referenced {
            folders: &folders,
            numbers: &numbers,
            number_keys: &number_keys,
            stub,
        },
    );
    used.extend(debris.used.iter().cloned());
    debris_notices(&debris, &resolution, tree, notices);

    // Counts, and whether a copy that lost its documents is being read.
    fill_counts(&rows, &mut counts, &debris, &resolution);
    let texts_anywhere = rows.iter().any(|r| {
        r.kind == ItemKind::Text
            && (r.payload.text.is_some()
                || r.payload.notes.is_some()
                || r.payload.synopsis.is_some())
    });
    if text_items > 0 && counts.texts_present == 0 {
        if text_items >= 10 && !texts_anywhere && found.salvaged == 0 && debris.recovered.is_empty()
        {
            if !answers.accept_no_text {
                pending.push(Question::NoTextInCopy { text_items });
            }
        } else {
            notices.push(Notice::NoTextFound { text_items });
        }
    }
    dangling_notices(&rows, &blocks, notices);

    // What a project may hold, counting only what the import reads.
    let media: BTreeSet<String> = rows
        .iter()
        .flat_map(|r| [r.payload.file.as_ref(), r.payload.card_image.as_ref()])
        .flatten()
        .map(|m| m.path.clone())
        .chain(custom_icons.iter().map(|m| m.path.clone()))
        .collect();
    // The manifests read are among `used`.
    let mut text_bytes: u64 = 0;
    for rel in used.iter().filter(|rel| !media.contains(*rel)) {
        let size = tree.entry(rel).map_or(0, |e| e.size);
        // A size is what the file or the archive says; an archive can say
        // anything, so no one member is taken past what a member may be.
        if size > MAX_MEMBER_BYTES {
            return Err(if tree.index.is_archive() {
                Refusal::ArchiveMemberTooLarge {
                    member: rel.clone(),
                    limit_mib: MAX_MEMBER_BYTES >> 20,
                }
            } else {
                Refusal::TooLarge {
                    limit_mib: MAX_TEXT_BYTES >> 20,
                }
            }
            .into());
        }
        text_bytes = text_bytes.saturating_add(size);
    }
    if text_bytes > MAX_TEXT_BYTES {
        return Err(Refusal::TooLarge {
            limit_mib: MAX_TEXT_BYTES >> 20,
        }
        .into());
    }
    for icon in &custom_icons {
        used.insert(icon.path.clone());
    }

    // What each document holds: every text and notes file decoded once, now
    // that the project is known to be within what may be read.
    let mut read = |m: &MemberRef| match tree.read(&m.path, MAX_MEMBER_BYTES) {
        Ok(bytes) => Ok(content::Read::Bytes(bytes)),
        Err(ReadError::Changed) => Err(content::Interrupted::Changed),
        Err(ReadError::Refused(refusal)) => Err(content::Interrupted::Refused(refusal)),
        Err(_) => Ok(content::Read::Unreadable),
    };
    let content = match content::measure(&rows, &project_notes, &debris.recovered, &mut read) {
        Ok(facts) => facts,
        Err(content::Interrupted::Changed) => return Err(Stop::Changed(1)),
        Err(content::Interrupted::Refused(refusal)) => return Err(Stop::Refuse(refusal)),
    };

    let fingerprint = fingerprint(tree, &chosen.hash, &used);
    let project = Project {
        source: SourceFacts {
            container: ctx.container,
            generation,
            writer,
            manifest: ManifestOrigin {
                name: chosen.name.clone(),
                chosen: origin,
            },
            folder_name: ctx.folder_name.clone(),
            folder_title,
            identifier: attrs.get("Identifier").cloned(),
            modified: attrs.get("Modified").map(|m| crate::dates::stamp(m)),
            version_stamp,
            id_scheme,
            template_title: ctx.template_title.clone(),
            created_from_template,
        },
        binder: rows,
        labels: blocks.labels,
        statuses: blocks.statuses,
        keywords: blocks.keywords,
        custom_fields: blocks.custom_fields,
        section_types: blocks.section_types,
        collections: blocks.collections,
        project_links: blocks.project_links,
        project_notes,
        targets: blocks.targets,
        template_folder: blocks.template_folder,
        bookmarks_folder: blocks.bookmarks_folder,
        favorites: blocks.favorites,
        auto_complete: blocks.auto_complete,
        properties: blocks.properties,
        settings,
        compile,
        styles,
        writing_history,
        recent_writing_day: blocks.recent_writing_day,
        custom_icons,
        recovered: debris.recovered,
        counts,
    };
    let structure = structure::plan(&project, &content, &structure::Choices::default())
        .map_err(|error| Refusal::StructureUnmappable { error })?;
    let (proposals, refused) = metadata::propose(&project, &content);
    proposal_notices(&proposals, &refused, &project, notices);
    let read = ReadProject {
        project,
        content,
        structure,
        proposals,
        card,
        notices: std::mem::take(notices),
        pending,
        fingerprint,
        location: PathBuf::new(),
    };
    if let Some(visit) = visit.as_mut() {
        visit(tree, &read).map_err(|Moved| Stop::Changed(1))?;
    }
    Ok(Analysis::Read(Box::new(read)))
}

/// Read every top-level manifest. One that cannot be read at all (too large, a
/// cloud placeholder, unreadable) stays a candidate carrying why: it refuses
/// the import only if it is the one to read, so a stray copy cannot block a
/// project.
fn read_candidates(
    tree: &Tree<'_, '_>,
    names: &[String],
    used: &mut BTreeSet<String>,
) -> Result<Vec<Candidate>, Stop> {
    let mut candidates = Vec::new();
    let mut items_left = MAX_ITEMS_TOGETHER;
    for name in names {
        let entry_size = tree.entry(name).map(|e| e.size).unwrap_or(0);
        let read = if entry_size > MAX_MANIFEST_BYTES {
            Err(Refusal::ManifestTooLarge {
                manifest: name.clone(),
                limit_mib: MAX_MANIFEST_BYTES >> 20,
            })
        } else {
            match tree.read(name, MAX_MANIFEST_BYTES) {
                Ok(bytes) => Ok(bytes),
                Err(ReadError::Changed) => return Err(Stop::Changed(1)),
                Err(ReadError::Refused(refusal)) => Err(refusal),
                Err(ReadError::Link) => Err(Refusal::NeededMemberIsLink {
                    member: name.clone(),
                }),
                Err(ReadError::Placeholder) => Err(Refusal::CloudPlaceholders {
                    count: 1,
                    examples: vec![name.clone()],
                }),
                Err(_) => Err(Refusal::UnreadableMembers {
                    count: 1,
                    examples: vec![name.clone()],
                }),
            }
        };
        let (bytes, unreadable) = match read {
            Ok(bytes) => (bytes, None),
            Err(refusal) => (Vec::new(), Some(refusal)),
        };
        used.insert(name.clone());
        let header = manifest::header(&bytes).unwrap_or_default();
        let (items, file_names, read) = match manifest::read(name, &bytes, items_left) {
            Ok(read) => {
                items_left = items_left.saturating_sub(read.rows.len());
                let items = u32::try_from(read.rows.len()).unwrap_or(u32::MAX);
                let file_names = file_names_of(&read.rows);
                (Ok(items), file_names, (names.len() == 1).then_some(read))
            }
            Err(damage) => (Err(damage), BTreeSet::new(), None),
        };
        candidates.push(Candidate {
            name: name.clone(),
            hash: blake3::hash(&bytes),
            header,
            items,
            file_names,
            read,
            unreadable,
        });
    }
    Ok(candidates)
}

/// The names a manifest's items give their files: a UUID as Scrivener names an
/// item's folder, a number as an item's flat files begin.
fn file_names_of(rows: &[BinderRow]) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for row in rows {
        if let Some(ItemKey::Uuid(uuid)) = row.key {
            names.insert(uuid_name(&uuid));
        }
        if let Some(number) = row.number {
            names.insert(number.to_string());
        }
    }
    names
}

/// The chosen manifest read again, when it was one of several and not kept. It
/// must be the manifest the choice was made on: a different one, or one that no
/// longer reads, is a project something is writing to.
fn read_again(
    tree: &Tree<'_, '_>,
    chosen: &Candidate,
) -> Result<Result<ManifestRead, crate::outcome::ManifestDamage>, Stop> {
    let bytes = match tree.read(&chosen.name, MAX_MANIFEST_BYTES) {
        Ok(bytes) => bytes,
        Err(ReadError::Refused(refusal)) => return Err(Stop::Refuse(refusal)),
        Err(_) => return Err(Stop::Changed(1)),
    };
    if blake3::hash(&bytes) != chosen.hash {
        return Err(Stop::Changed(1));
    }
    Ok(manifest::read(&chosen.name, &bytes, manifest::MAX_ITEMS))
}

/// Scrivener's last autosave: a zip holding one manifest. Its hash and `ModID`.
struct Autosave {
    hash: blake3::Hash,
    mod_id: Option<String>,
}

fn read_autosave(
    tree: &Tree<'_, '_>,
    used: &mut BTreeSet<String>,
    notices: &mut Vec<Notice>,
) -> Result<Option<Autosave>, Stop> {
    let Some(rel) = tree.lookup("Files/binder.autosave") else {
        return Ok(None);
    };
    let Some(bytes) = read_member(tree, &rel, MAX_MANIFEST_BYTES, notices)? else {
        return Ok(None);
    };
    used.insert(rel.clone());
    let Ok(inner) = Index::of_archive_bytes(tree.index.disk(), bytes, rel) else {
        return Ok(None);
    };
    let manifests: Vec<String> = inner
        .children("")
        .into_iter()
        .filter(|n| n.to_lowercase().ends_with(".scrivx"))
        .collect();
    let [only] = manifests.as_slice() else {
        return Ok(None);
    };
    let Ok(bytes) = inner.read(only, MAX_MANIFEST_BYTES) else {
        return Ok(None);
    };
    Ok(Some(Autosave {
        hash: blake3::hash(&bytes),
        mod_id: manifest::header(&bytes).and_then(|h| h.get("ModID").map(str::to_string)),
    }))
}

/// The candidate a picked manifest's name means. The pick comes from the
/// writer's file dialog, a typed path or another program, and a disk that
/// compares names regardless of case or Unicode normalisation (APFS, NTFS) opens
/// it however it is spelled, while the listing spells it as stored: exactly,
/// then by NFC, then case-folded, and only when one candidate matches.
fn pinned_index(candidates: &[Candidate], pinned: &str) -> Option<usize> {
    if let Some(index) = candidates.iter().position(|c| c.name == pinned) {
        return Some(index);
    }
    let only = |key: &dyn Fn(&str) -> String| {
        let wanted = key(pinned);
        let mut matching = candidates
            .iter()
            .enumerate()
            .filter(|(_, c)| key(&c.name) == wanted)
            .map(|(index, _)| index);
        match (matching.next(), matching.next()) {
            (Some(index), None) => Some(index),
            _ => None,
        }
    };
    only(&nfc).or_else(|| only(&folded))
}

/// Choose the manifest to read among the candidates.
fn choose_manifest(
    tree: &Tree<'_, '_>,
    candidates: &[Candidate],
    ctx: &Context,
    answers: &Answers,
    autosave: Option<&Autosave>,
    notices: &mut Vec<Notice>,
) -> Result<(usize, ManifestChoice), Stop> {
    let position = |name: &str| candidates.iter().position(|c| c.name == name);
    if let Some(pinned) = &ctx.pinned {
        let index = pinned_index(candidates, pinned).ok_or(Refusal::NotAScrivenerProject)?;
        let pinned = &candidates[index].name;
        if candidates.len() > 1 {
            notices.push(Notice::PinnedManifestHasSiblings {
                others: candidates.len() as u32 - 1,
            });
            if let Some((_, original)) = conflict_original(pinned)
                && let Some(original) = candidates
                    .iter()
                    .find(|c| folded(&c.name) == folded(&original))
            {
                notices.push(Notice::PinnedManifestLooksLikeACopy {
                    name: pinned.clone(),
                    original: original.name.clone(),
                });
            }
        }
        return Ok((index, ManifestChoice::Picked));
    }
    if let [_] = candidates {
        return Ok((0, ManifestChoice::Only));
    }
    if let Some(answer) = &answers.manifest
        && let Some(index) = position(answer)
    {
        return Ok((index, ManifestChoice::Answered));
    }
    let stem = folded(
        ctx.folder_name
            .strip_suffix(".scriv")
            .unwrap_or(&ctx.folder_name),
    );
    let matches_autosave = |c: &Candidate| {
        autosave.is_some_and(|a| {
            a.hash == c.hash || (a.mod_id.is_some() && a.mod_id.as_deref() == c.header.get("ModID"))
        })
    };
    let names_folder =
        |c: &Candidate| folded(c.name.strip_suffix(".scrivx").unwrap_or(&c.name)) == stem;
    let modified = |c: &Candidate| {
        c.header
            .get("Modified")
            .and_then(|m| crate::dates::stamp(m).unix_seconds)
    };

    // All the same bytes: any is the project. Prefer the one named after the
    // folder, else the first by name.
    if candidates.iter().all(|c| c.hash == candidates[0].hash) {
        notices.push(Notice::IdenticalManifestCopies {
            count: candidates.len() as u32,
        });
        let index = candidates.iter().position(names_folder).unwrap_or(0);
        return Ok((index, ManifestChoice::Identical));
    }

    let by_autosave: Vec<usize> = (0..candidates.len())
        .filter(|&i| matches_autosave(&candidates[i]))
        .collect();
    let by_folder: Vec<usize> = (0..candidates.len())
        .filter(|&i| names_folder(&candidates[i]))
        .collect();
    // Saved strictly after every other; a manifest that does not say when it
    // was saved counts as older than one that does.
    let newest = {
        let dates: Vec<Option<i64>> = candidates.iter().map(modified).collect();
        dates
            .iter()
            .enumerate()
            .filter_map(|(i, d)| d.map(|d| (i, d)))
            .max_by_key(|(_, d)| *d)
            .filter(|(i, d)| {
                dates
                    .iter()
                    .enumerate()
                    .all(|(j, other)| j == *i || other.is_none_or(|o| o < *d))
            })
            .map(|(i, _)| i)
    };
    let mut evidence = Vec::new();
    let mut preselected: Option<usize> = None;
    if let [one] = by_autosave.as_slice() {
        evidence.push(ManifestEvidence::MatchesAutosave);
        preselected = Some(*one);
    }
    if let [one] = by_folder.as_slice()
        && preselected.is_none_or(|p| p == *one)
    {
        evidence.push(ManifestEvidence::FolderName);
        preselected = Some(*one);
    }
    if let Some(one) = newest
        && preselected.is_none_or(|p| p == one)
    {
        evidence.push(ManifestEvidence::Newest);
        preselected = Some(one);
    }

    // The automatic case: the autosave and the folder's name agree, every other
    // manifest is a sync copy of that one (or the same bytes), and no other one
    // lists writing that one does not.
    if let Some(best) = preselected
        && evidence.contains(&ManifestEvidence::MatchesAutosave)
        && evidence.contains(&ManifestEvidence::FolderName)
    {
        let best_name = folded(&candidates[best].name);
        let all_copies = candidates.iter().enumerate().all(|(i, c)| {
            i == best
                || c.hash == candidates[best].hash
                || conflict_original(&c.name)
                    .is_some_and(|(_, original)| folded(&original) == best_name)
        });
        if all_copies && !others_list_more_writing(tree, candidates, best) {
            notices.push(Notice::ManifestCopiesIgnored {
                chosen: candidates[best].name.clone(),
                copies: candidates
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != best)
                    .map(|(_, c)| c.name.clone())
                    .collect(),
            });
            return Ok((best, ManifestChoice::Automatic));
        }
    }
    let preselected_name = preselected.map(|i| candidates[i].name.clone());
    Err(Stop::Choose(Question::SeveralManifests {
        candidates: candidates
            .iter()
            .map(|c| ManifestCandidate {
                name: c.name.clone(),
                creator: c.header.get("Creator").map(str::to_string),
                version: c.header.get("Version").map(str::to_string),
                modified: modified(c),
                items: c.items.as_ref().ok().copied(),
                matches_autosave: matches_autosave(c),
                identical_to_preselected: preselected.is_some_and(|p| candidates[p].hash == c.hash),
            })
            .collect(),
        preselected: preselected_name,
        evidence,
    }))
}

/// Whether any manifest but `best` lists an item whose files hold something
/// `best` does not list: ignoring it would then lose writing.
fn others_list_more_writing(tree: &Tree<'_, '_>, candidates: &[Candidate], best: usize) -> bool {
    let best_keys = &candidates[best].file_names;
    // What holds writing, listed once: a copy can name a hundred thousand items,
    // and asking the index about each would list `Files/Docs` once per item.
    // Item folders by their case-folded name, as a lookup finds them; flat files
    // by the number they start with (`12.rtf`, `12_notes.rtf`, never `123.rtf`).
    let data_with_files: BTreeSet<String> = tree
        .lookup("Files/Data")
        .map(|data| {
            tree.children(&data)
                .into_iter()
                .filter(|name| !tree.children(&join(&data, name)).is_empty())
                .map(|name| folded(&name))
                .collect()
        })
        .unwrap_or_default();
    let numbered_files: BTreeSet<String> = tree
        .lookup("Files/Docs")
        .map(|docs| {
            tree.children(&docs)
                .into_iter()
                .filter_map(|file| {
                    let digits = file.bytes().take_while(u8::is_ascii_digit).count();
                    matches!(file.as_bytes().get(digits), Some(b'.' | b'_'))
                        .then(|| file[..digits].to_string())
                        .filter(|number| !number.is_empty())
                })
                .collect()
        })
        .unwrap_or_default();
    let has_payload = |name: &str| -> bool {
        if crate::names::is_uuid(name) {
            data_with_files.contains(&folded(name))
        } else {
            numbered_files.contains(name)
        }
    };
    candidates.iter().enumerate().any(|(i, c)| {
        i != best
            && c.file_names
                .difference(best_keys)
                .any(|name| has_payload(name))
    })
}

fn writer_of(creator: Option<&str>) -> Writer {
    let parse = |raw: &str| -> Option<(Platform, String)> {
        let rest = raw.strip_prefix("SCR")?;
        let platform = match rest.get(..3)? {
            "MAC" => Platform::Mac,
            "WIN" => Platform::Windows,
            "LIN" => Platform::Linux,
            "IOS" => Platform::Ios,
            _ => return None,
        };
        let rest = rest.get(3..)?;
        let rest = rest.strip_prefix('-').or_else(|| rest.strip_prefix('/'))?;
        let version: String = rest
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        (!version.is_empty() && version.starts_with(|c: char| c.is_ascii_digit()))
            .then_some((platform, version))
    };
    match creator.and_then(parse) {
        Some((platform, version)) => Writer {
            platform,
            version: Some(version),
            creator: creator.map(str::to_string),
        },
        None => Writer {
            platform: Platform::Unknown,
            version: None,
            creator: creator.map(str::to_string),
        },
    }
}

fn expected_scheme(generation: Generation) -> IdScheme {
    match generation {
        Generation::Format10 => IdScheme::Id,
        Generation::Format15 => IdScheme::IdAndUuid,
        Generation::Format20 | Generation::IosStub => IdScheme::Uuid,
    }
}

/// The identifier scheme most items use; the version's own on a tie.
fn id_scheme(rows: &[BinderRow], generation: Generation) -> IdScheme {
    let mut counts: HashMap<IdScheme, usize> = HashMap::new();
    for row in rows {
        let scheme = match (
            row.number.is_some(),
            matches!(row.key, Some(ItemKey::Uuid(_))),
        ) {
            (true, true) => IdScheme::IdAndUuid,
            (true, false) => IdScheme::Id,
            (false, true) => IdScheme::Uuid,
            (false, false) => continue,
        };
        *counts.entry(scheme).or_default() += 1;
    }
    let expected = expected_scheme(generation);
    let best = counts.values().copied().max().unwrap_or(0);
    if counts.get(&expected).copied().unwrap_or(0) == best {
        return expected;
    }
    [IdScheme::Uuid, IdScheme::IdAndUuid, IdScheme::Id]
        .into_iter()
        .find(|s| counts.get(s).copied().unwrap_or(0) == best)
        .unwrap_or(expected)
}

/// The first item with a key owns it; a later one keeps its place and loses its
/// files, and is reported.
fn mark_duplicates(rows: &mut [BinderRow], notices: &mut Vec<Notice>) {
    let mut keys = BTreeSet::new();
    let mut numbers = BTreeSet::new();
    for row in rows.iter_mut() {
        let key_taken = row.key.is_some_and(|k| !keys.insert(k));
        let number_taken = row.number.is_some_and(|n| !numbers.insert(n));
        if key_taken || number_taken {
            row.owns_key = false;
            if let Some(key) = row.key.or(row.number.map(ItemKey::Number)) {
                notices.push(Notice::DuplicateItemKey { key });
            }
        }
    }
}

/// Format 1.x names items by number in links, collections and templates; when
/// the binder's identity is the UUID (format 1.5), those numbers become the
/// items' own keys, so every reference in the model names an item one way. The
/// map from number to key, for the grammar.
fn normalise_references(rows: &mut [BinderRow]) -> HashMap<u32, ItemKey> {
    let map: HashMap<u32, ItemKey> = rows
        .iter()
        .filter(|r| r.owns_key)
        .filter_map(|r| Some((r.number?, r.key?)))
        .collect();
    let fix = |key: &mut Option<ItemKey>| {
        if let Some(ItemKey::Number(n)) = key
            && let Some(own) = map.get(n)
        {
            *key = Some(*own);
        }
    };
    for row in rows.iter_mut() {
        for link in &mut row.links {
            fix(&mut link.target);
        }
        fix(&mut row.default_child_template);
    }
    map
}

fn renumber_blocks(blocks: &mut manifest::Blocks, map: &HashMap<u32, ItemKey>) {
    let fix_key = |key: &mut ItemKey| {
        if let ItemKey::Number(n) = key
            && let Some(own) = map.get(n)
        {
            *key = *own;
        }
    };
    for collection in &mut blocks.collections {
        collection.members.iter_mut().for_each(fix_key);
    }
    blocks.favorites.iter_mut().for_each(fix_key);
    for link in &mut blocks.project_links {
        if let Some(target) = &mut link.target {
            fix_key(target);
        }
    }
    if let Some(key) = &mut blocks.template_folder {
        fix_key(key);
    }
    if let Some(key) = &mut blocks.bookmarks_folder {
        fix_key(key);
    }
}

/// The folders and numbers items own, for the grammar.
fn owned_keys(rows: &[BinderRow], layout: Layout) -> (BTreeSet<String>, BTreeSet<u32>) {
    let mut folders = BTreeSet::new();
    let mut numbers = BTreeSet::new();
    for row in rows.iter().filter(|r| r.owns_key) {
        match layout {
            Layout::Folders(_) => {
                if let Some(ItemKey::Uuid(uuid)) = row.key {
                    folders.insert(uuid_name(&uuid));
                }
            }
            Layout::Flat => {
                if let Some(n) = row.number {
                    numbers.insert(n);
                }
            }
        }
    }
    (folders, numbers)
}

/// A regular file of the project at `rel`, recorded as used.
fn member(tree: &Tree<'_, '_>, wanted: &str, used: &mut BTreeSet<String>) -> Option<MemberRef> {
    let rel = tree.lookup(wanted)?;
    let member = tree.member_ref(&rel)?;
    used.insert(rel);
    Some(member)
}

/// Format 1.x project notes: `Files/ProjectNotes/index.xml` and a file per note.
///
/// A note file the index does not list, or every note file when the index did
/// not come along (a Windows 1.x project in the corpus arrives so, its note 106
/// words long), is still the project's: carried with its number and without the
/// title only the index held, never left for the grammar to pass over.
fn read_project_notes(
    tree: &Tree<'_, '_>,
    used: &mut BTreeSet<String>,
    notices: &mut Vec<Notice>,
) -> Result<Vec<ProjectNote>, Stop> {
    let mut notes = listed_project_notes(tree, used, notices)?;
    let Some(folder) = tree.lookup("Files/ProjectNotes") else {
        return Ok(notes);
    };
    for name in tree.children(&folder) {
        let rel = join(&folder, &name);
        let lower = name.to_lowercase();
        let Some(id) = lower
            .strip_prefix("note-")
            .and_then(|rest| rest.strip_suffix(".rtf"))
            .filter(|digits| crate::names::is_item_number(digits))
            .and_then(|digits| digits.parse::<u32>().ok())
        else {
            continue;
        };
        if used.contains(&rel) {
            continue;
        }
        if let Some(file) = tree.member_ref(&rel) {
            used.insert(rel);
            notes.push(ProjectNote {
                id: Some(id),
                title: None,
                modified: None,
                file: Some(file),
            });
        }
    }
    Ok(notes)
}

/// The project notes `Files/ProjectNotes/index.xml` lists, each with its file.
fn listed_project_notes(
    tree: &Tree<'_, '_>,
    used: &mut BTreeSet<String>,
    notices: &mut Vec<Notice>,
) -> Result<Vec<ProjectNote>, Stop> {
    let Some(rel) = tree.lookup("Files/ProjectNotes/index.xml") else {
        return Ok(Vec::new());
    };
    let Some(bytes) = read_member(tree, &rel, MAX_SMALL_BYTES * 16, notices)? else {
        return Ok(Vec::new());
    };
    used.insert(rel.clone());
    let parsed = xml::text_of(&bytes).and_then(|text| {
        let doc = xml::parse(&rel, text)?;
        Ok(xml::children(doc.root_element(), "Note")
            .map(|note| {
                let id: Option<u32> = note.attribute("ID").and_then(|v| v.trim().parse().ok());
                let file = id.and_then(|id| {
                    member(tree, &format!("Files/ProjectNotes/Note-{id}.rtf"), used)
                });
                ProjectNote {
                    id,
                    title: xml::child_text(note, "Title"),
                    modified: note.attribute("Modified").map(crate::dates::stamp),
                    file,
                }
            })
            .collect::<Vec<_>>())
    });
    match parsed {
        Ok(notes) => Ok(notes),
        Err(problem) => {
            notices.push(Notice::MetadataUnreadable {
                member: rel,
                problem: problem.problem(),
            });
            Ok(Vec::new())
        }
    }
}

/// The most compile formats read from one project: the corpus's largest holds four.
const MAX_COMPILE_FORMATS: usize = 64;

/// The largest compile settings file or compile format read (the largest real one is under
/// 150 KB).
const MAX_COMPILE_BYTES: u64 = 8 << 20;

/// What the compile settings state (`metadata::compile`), each file read once and kept in
/// `used`. A file that cannot be read is said, and states nothing.
fn read_compile(
    tree: &Tree<'_, '_>,
    settings: &mut SettingsFiles,
    used: &mut BTreeSet<String>,
    notices: &mut Vec<Notice>,
) -> Result<CompileFacts, Stop> {
    let mut facts = CompileFacts::default();
    let unreadable = |notices: &mut Vec<Notice>, member: &str, problem: xml::XmlProblem| {
        notices.push(Notice::MetadataUnreadable {
            member: member.to_string(),
            problem: problem.problem(),
        });
    };
    let mut struck_xml = None;
    if let Some(m) = &settings.compile_xml
        && let Some(bytes) = read_member(tree, &m.path, MAX_COMPILE_BYTES, notices)?
    {
        match metadata::compile::read_xml(&m.path, &bytes) {
            Ok(read) => {
                facts.title = read.title;
                facts.abbreviated_title = read.abbreviated_title;
                facts.authors = read.authors;
                facts.surname = read.surname;
                facts.forename = read.forename;
                facts.last_format = read.last_format;
                facts.subjects = read.subjects;
                facts.company = read.company;
                facts.replacements = read.replacements;
                // Scrivener 3 writes the switch only when it is on: silent is off.
                struck_xml = Some(read.delete_struck_through.unwrap_or(false));
                if let Some(value) = read.language {
                    facts.languages.push(CompileLanguage {
                        file: CompileFile::Xml,
                        value,
                    });
                }
            }
            Err(problem) => unreadable(notices, &m.path, problem),
        }
    }
    let mut struck_plist = None;
    if let Some(m) = &settings.compile_plist
        && let Some(bytes) = read_member(tree, &m.path, MAX_COMPILE_BYTES, notices)?
    {
        match metadata::compile::read_plist(&m.path, &bytes) {
            Ok(read) => {
                struck_plist = read.delete_struck_through;
                facts.replacements.extend(read.replacements);
                if let Some(value) = read.language {
                    facts.languages.push(CompileLanguage {
                        file: CompileFile::Plist,
                        value,
                    });
                }
            }
            Err(problem) => unreadable(notices, &m.path, problem),
        }
    }
    if let Some(m) = &settings.compile_ini
        && let Some(bytes) = read_member(tree, &m.path, MAX_COMPILE_BYTES, notices)?
        && let Some(value) = metadata::compile::read_ini_language(&bytes)
    {
        facts.languages.push(CompileLanguage {
            file: CompileFile::Ini,
            value,
        });
    }
    // The file of the generation that wrote the project decides, once it is read; a
    // Scrivener 2 file left beside a Scrivener 3 one after an upgrade no longer compiles
    // anything.
    facts.delete_struck_through = struck_xml.or(struck_plist).unwrap_or(false);
    if let Some(folder) = tree.lookup("Settings/Compile Formats")
        && tree.is_dir(&folder)
    {
        let names: Vec<String> = tree
            .children(&folder)
            .into_iter()
            .filter(|name| name.to_ascii_lowercase().ends_with(".scrformat"))
            .take(MAX_COMPILE_FORMATS)
            .collect();
        for name in names {
            let rel = join(&folder, &name);
            let Some(format) = member(tree, &rel, used) else {
                continue;
            };
            if let Some(bytes) = read_member(tree, &rel, MAX_COMPILE_BYTES, notices)? {
                match metadata::compile::read_format(&rel, &bytes) {
                    Ok(read) => facts.formats.push(read),
                    Err(problem) => unreadable(notices, &rel, problem),
                }
            }
            settings.compile_formats.push(format);
        }
    }
    Ok(facts)
}

/// The largest writing history read: 173 days is the corpus's longest, a few kilobytes.
const MAX_HISTORY_BYTES: u64 = 16 << 20;

/// The days Scrivener's writing history covers.
#[derive(Debug, Clone, Default)]
struct WritingDays {
    days: u32,
    first: Option<chrono::NaiveDate>,
    last: Option<chrono::NaiveDate>,
    /// Entries whose day could not be read.
    unreadable: u32,
}

impl WritingDays {
    fn add(&mut self, day: chrono::NaiveDate) {
        self.days = self.days.saturating_add(1);
        self.first = Some(self.first.map_or(day, |f| f.min(day)));
        self.last = Some(self.last.map_or(day, |l| l.max(day)));
    }

    /// With no history file of any use, the day the manifest caches, in the writer's own
    /// calendar.
    fn or_recent(mut self, recent: Option<&Stamp>) -> WritingDays {
        if self.days == 0
            && let Some(day) = recent.and_then(crate::dates::local_date)
        {
            self.add(day);
        }
        self
    }
}

/// Count the days of `Files/writing.history`: `<Day …>YYYY-MM-DD</Day>`, one per day
/// written. Its figures are not read: they never become Skribisto's (D6.8).
fn read_writing_history(
    tree: &Tree<'_, '_>,
    member: &MemberRef,
    notices: &mut Vec<Notice>,
) -> Result<WritingDays, Stop> {
    let mut out = WritingDays::default();
    let Some(bytes) = read_member(tree, &member.path, MAX_HISTORY_BYTES, notices)? else {
        return Ok(out);
    };
    let parsed = xml::text_of(&bytes).and_then(|text| {
        let doc = xml::parse(&member.path, text)?;
        for day in xml::children(doc.root_element(), "Day") {
            match xml::text(day).as_deref().and_then(crate::dates::parse_day) {
                Some(date) => out.add(date),
                None => out.unreadable = out.unreadable.saturating_add(1),
            }
        }
        Ok(())
    });
    if let Err(problem) = parsed {
        notices.push(Notice::MetadataUnreadable {
            member: member.path.clone(),
            problem: problem.problem(),
        });
    }
    Ok(out)
}

/// What the writer should know about the title, authors and language proposed.
fn proposal_notices(
    proposals: &Proposals,
    refused: &[Refused],
    project: &Project,
    notices: &mut Vec<Notice>,
) {
    for refusal in refused {
        notices.push(match refusal {
            Refused::Title(source) => Notice::TitlePlaceholderRejected { source: *source },
            Refused::Author(source) => Notice::AuthorPlaceholderRejected { source: *source },
        });
    }
    let stated = matches!(
        proposals.title.source,
        TitleSource::CompileSettings | TitleSource::ProjectProperties
    );
    if stated
        && metadata::folded(&proposals.title.title)
            != metadata::folded(&metadata::folder_title(project))
    {
        notices.push(Notice::TitleFromProject {
            source: proposals.title.source,
        });
    }
    if let Some(author) = &proposals.author {
        notices.push(Notice::AuthorFromProject {
            source: author.source,
        });
    }
    match &proposals.language {
        None => notices.push(Notice::LanguageNotStated),
        Some(language) if language.source == metadata::LanguageSource::Script => {
            notices.push(Notice::LanguageFromScript {
                tag: language.tag.clone(),
            });
        }
        Some(_) => {}
    }
}

/// The title of the template the project was created from.
fn template_info_title(
    tree: &Tree<'_, '_>,
    info: &MemberRef,
    notices: &mut Vec<Notice>,
) -> Result<Option<String>, Stop> {
    let Some(bytes) = read_member(tree, &info.path, MAX_SMALL_BYTES, notices)? else {
        return Ok(None);
    };
    let parsed = xml::text_of(&bytes).and_then(|text| {
        let doc = xml::parse(&info.path, text)?;
        Ok(xml::child_text(doc.root_element(), "Title")
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty()))
    });
    match parsed {
        Ok(title) => Ok(title),
        Err(problem) => {
            notices.push(Notice::MetadataUnreadable {
                member: info.path.clone(),
                problem: problem.problem(),
            });
            Ok(None)
        }
    }
}

/// The unmerged iOS edits a desktop project may hold.
fn mobile_questions(
    tree: &Tree<'_, '_>,
    attrs: &BTreeMap<String, String>,
    answers: &Answers,
    pending: &mut Vec<Question>,
    notices: &mut Vec<Notice>,
    used: &mut BTreeSet<String>,
) -> Result<(), Stop> {
    let mut source = None;
    if let Some(mobile) = tree.lookup("Mobile")
        && tree.is_dir(&mobile)
    {
        let pending_edits = tree.lookup("Mobile/binder.mob").is_some()
            || tree.lookup("Mobile/Data").is_some_and(|data| {
                tree.index.keys_under(&tree.key(&data)).iter().any(|key| {
                    tree.index
                        .entry(key)
                        .is_some_and(|e| e.kind == EntryKind::File)
                })
            });
        if pending_edits {
            source = Some(MobileEdits::MobileFolder);
        }
    }
    if let Some(rel) = tree.lookup("Files/binder.mob")
        && let Some(bytes) = read_member(tree, &rel, MAX_MANIFEST_BYTES, notices)?
    {
        used.insert(rel);
        let mob_modified = manifest::header(&bytes).and_then(|h| {
            h.get("Modified")
                .and_then(|m| crate::dates::stamp(m).unix_seconds)
        });
        let own_modified = manifest::modified_seconds(attrs);
        match (mob_modified, own_modified) {
            (Some(mob), Some(own)) if mob > own => {
                source.get_or_insert(MobileEdits::NewerMobileBinder);
            }
            _ => notices.push(Notice::StaleMobileBinderIgnored),
        }
    }
    if let Some(source) = source
        && !answers.accept_unmerged_mobile_edits
    {
        pending.push(Question::UnmergedMobileEdits { source });
    }
    Ok(())
}

fn debris_notices(
    debris: &layout::Debris,
    resolution: &layout::Resolution,
    tree: &Tree<'_, '_>,
    notices: &mut Vec<Notice>,
) {
    if debris.legacy_v1 > 0 {
        notices.push(Notice::LegacyV1FilesBeside {
            count: debris.legacy_v1,
        });
    }
    if debris.conflict_copies + debris.orphan_copies > 0 {
        notices.push(Notice::SyncConflictCopies {
            count: debris.conflict_copies + debris.orphan_copies,
            original_missing: debris.orphan_copies,
        });
    }
    if debris.unreferenced_texts > 0 {
        notices.push(Notice::RecoveredUnreferencedText {
            count: debris.unreferenced_texts,
        });
    }
    if debris.sync_debris > 0 {
        notices.push(Notice::SyncDebrisIgnored {
            count: debris.sync_debris,
        });
    }
    if !debris.unrecognised.is_empty() {
        notices.push(Notice::UnrecognisedFiles {
            count: debris.unrecognised.len() as u32,
            examples: debris.unrecognised.iter().take(3).cloned().collect(),
        });
    }
    if !debris.links.is_empty() {
        notices.push(Notice::SymlinkNotFollowed {
            count: debris.links.len() as u32,
            examples: debris.links.iter().take(3).cloned().collect(),
        });
    }
    let mut unreadable: Vec<String> = debris.specials.clone();
    unreadable.extend(
        tree.index
            .unreadable_folders
            .keys()
            .map(|key| tree.rel(key).to_string()),
    );
    if !unreadable.is_empty() {
        notices.push(Notice::UnreadableFilesSkipped {
            count: unreadable.len() as u32,
            examples: unreadable.into_iter().take(3).collect(),
        });
    }
    let placeholders = debris.placeholders.len()
        + tree.index.placeholder_folders.len()
        + resolution.media_placeholders as usize;
    if placeholders > 0 {
        notices.push(Notice::CloudPlaceholdersSkipped {
            count: placeholders as u32,
        });
    }
}

fn fill_counts(
    rows: &[BinderRow],
    counts: &mut Counts,
    debris: &layout::Debris,
    resolution: &layout::Resolution,
) {
    let parents = crate::model::parents_of(rows.iter().map(|r| r.depth));
    let mut in_trash = vec![false; rows.len()];
    for i in 0..rows.len() {
        in_trash[i] =
            parents[i].is_some_and(|p| in_trash[p] || rows[p].kind == ItemKind::TrashFolder);
    }
    counts.items = rows.len() as u32;
    for (i, row) in rows.iter().enumerate() {
        match row.kind {
            ItemKind::Text => counts.text_items += 1,
            _ if row.kind.is_media() => counts.media_items += 1,
            _ => {}
        }
        if row.payload.text.is_some() {
            counts.texts_present += 1;
            if matches!(row.kind, ItemKind::Folder) {
                counts.folders_with_text += 1;
            }
        }
        if in_trash[i] {
            counts.trashed_items += 1;
        }
    }
    counts.media_missing = resolution.missing_media.len() as u32;
    counts.research_files = rows
        .iter()
        .filter(|r| r.kind.is_media() && r.kind != ItemKind::Image && r.payload.file.is_some())
        .count() as u32;
    counts.unreferenced_files = debris.unreferenced_other;
    counts.unreferenced_empty_files = debris.empty_texts;
}

/// References to ids the project does not define.
fn dangling_notices(rows: &[BinderRow], blocks: &manifest::Blocks, notices: &mut Vec<Notice>) {
    let keyword_ids: BTreeSet<i64> = blocks.keywords.iter().map(|k| k.id).collect();
    let label_ids: BTreeSet<i64> = blocks.labels.entries.iter().map(|e| e.id).collect();
    let status_ids: BTreeSet<i64> = blocks.statuses.entries.iter().map(|e| e.id).collect();
    let section_ids: BTreeSet<String> = blocks
        .section_types
        .definitions
        .iter()
        .map(|d| d.id.to_uppercase())
        .collect();
    let field_ids: BTreeSet<&str> = blocks.custom_fields.iter().map(|f| f.id.as_str()).collect();
    let item_keys: BTreeSet<ItemKey> = rows
        .iter()
        .filter(|r| r.owns_key)
        .filter_map(|r| r.key)
        .collect();
    let mut counts: BTreeMap<u8, (DanglingKind, u32)> = BTreeMap::new();
    let mut bump = |order: u8, kind: DanglingKind| counts.entry(order).or_insert((kind, 0)).1 += 1;
    for row in rows {
        for id in &row.keyword_ids {
            if !keyword_ids.contains(id) {
                bump(0, DanglingKind::Keyword);
            }
        }
        if row.meta.label.is_some_and(|l| !label_ids.contains(&l)) {
            bump(1, DanglingKind::Label);
        }
        if row.meta.status.is_some_and(|s| !status_ids.contains(&s)) {
            bump(2, DanglingKind::Status);
        }
        for section in [&row.meta.section_type, &row.meta.section_child_default]
            .into_iter()
            .flatten()
        {
            if !section_ids.contains(&section.to_uppercase()) {
                bump(3, DanglingKind::SectionType);
            }
        }
        for value in &row.meta.custom {
            if !field_ids.contains(value.field.as_str()) {
                bump(4, DanglingKind::CustomField);
            }
        }
        for link in &row.links {
            if link.target.is_some_and(|t| !item_keys.contains(&t)) {
                bump(5, DanglingKind::Item);
            }
        }
    }
    for collection in &blocks.collections {
        for member in &collection.members {
            if !item_keys.contains(member) {
                bump(5, DanglingKind::Item);
            }
        }
    }
    for (kind, count) in counts.into_values() {
        notices.push(Notice::DanglingReferences { kind, count });
    }
}

/// A digest of the manifest and of what the model was built from: each member's
/// name (NFC) and its content, in order, so that the conversion can tell the project it
/// reads from the one the writer reviewed.
///
/// A member's content is the hash of its bytes wherever they are at hand: every member
/// the analysis read (the texts, the notes and synopses, the settings it looked into),
/// and every other small one, read here (the comments beside a text, the style sheet, the
/// preferences). Only a large member no reading needed (a research PDF, a film) is known
/// by its size and time, as the consistent read knows it. So the same bytes written again,
/// or a sync service touching a folder, change nothing, and other bytes behind an
/// unchanged size and time are still seen. The lock Scrivener writes while it has the
/// project open is left out: it says who has it open, never what the import writes.
fn fingerprint(
    tree: &Tree<'_, '_>,
    manifest_hash: &blake3::Hash,
    used: &BTreeSet<String>,
) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(manifest_hash.as_bytes());
    // An archive's members carry no time of their own; the archive's does.
    if let Some((name, size, modified)) = tree.index.container_facts() {
        hasher.update(name.as_bytes());
        hasher.update(&size.to_le_bytes());
        hasher.update(&modified.unwrap_or(0).to_le_bytes());
    }
    let lock = tree.lookup("Files/user.lock");
    let mut rows: Vec<(String, &str)> = used
        .iter()
        .filter(|rel| Some(rel.as_str()) != lock.as_deref())
        .map(|rel| (nfc(rel), rel.as_str()))
        .collect();
    rows.sort();
    let read = tree.index.read_hashes();
    for (name, rel) in rows {
        let entry = tree.entry(rel);
        let (size, modified) = entry
            .map(|e| (e.size, e.modified_ns.unwrap_or(0)))
            .unwrap_or((0, 0));
        let small = entry.is_some_and(|e| e.kind == EntryKind::File && e.size <= MAX_SMALL_BYTES);
        let content = read.get(&tree.key(rel)).copied().or_else(|| {
            small
                .then(|| tree.read(rel, MAX_SMALL_BYTES).ok())
                .flatten()
                .map(|bytes| blake3::hash(&bytes))
        });
        hasher.update(name.as_bytes());
        hasher.update(b"\0");
        match content {
            Some(hash) => {
                hasher.update(b"=");
                hasher.update(hash.as_bytes());
            }
            None => {
                hasher.update(&size.to_le_bytes());
                hasher.update(&modified.to_le_bytes());
            }
        }
        hasher.update(b"\n");
    }
    hasher.finalize().to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::disk::testing::{Clouded, Recording, Restless};
    use scrivener_fixtures::{Generation as FixtureGeneration, Platform as FixturePlatform, Spec};

    fn written(spec: &Spec) -> (tempfile::TempDir, PathBuf) {
        let dir = match tempfile::tempdir() {
            Ok(dir) => dir,
            Err(e) => panic!("scratch folder: {e}"),
        };
        let folder = dir.path().join(spec.folder());
        if let Err(e) = spec.build().write_to(&folder) {
            panic!("writing the fixture: {e}");
        }
        (dir, folder)
    }

    fn options() -> Options {
        Options { this_host: None }
    }

    /// Scrivener autosaves while the project is read: the analysis reads it all
    /// again, and a project that settles is read.
    #[test]
    fn a_project_written_to_once_while_read_is_read_again() {
        let (_dir, folder) = written(&Spec::novel(
            FixtureGeneration::Format20,
            FixturePlatform::Mac,
        ));
        let disk = Restless::new("Lighthouse.scrivx", Some(1));
        let analysis = analyze_with(&disk, &folder, &Answers::default(), &options());
        assert!(matches!(analysis, Analysis::Read(_)), "{analysis:?}");
    }

    /// One that keeps moving is refused: something is writing to it.
    #[test]
    fn a_project_that_keeps_changing_is_refused() {
        let (_dir, folder) = written(&Spec::novel(
            FixtureGeneration::Format20,
            FixturePlatform::Mac,
        ));
        let disk = Restless::new("Lighthouse.scrivx", None);
        let analysis = analyze_with(&disk, &folder, &Answers::default(), &options());
        assert!(
            matches!(
                analysis,
                Analysis::Refused {
                    refusal: Refusal::ProjectChangedDuringRead { .. },
                    ..
                }
            ),
            "{analysis:?}"
        );
    }

    /// A document macOS or Windows reports as a cloud placeholder is never read
    /// (reading it would download it): the import is refused before.
    #[test]
    fn a_needed_placeholder_refuses_without_being_read() {
        let spec = Spec::novel(FixtureGeneration::Format20, FixturePlatform::Mac);
        let (_dir, folder) = written(&spec);
        let arrival = spec
            .items
            .iter()
            .position(|i| i.title.as_deref() == Some("Arrival"))
            .unwrap_or(0);
        let disk = Clouded {
            suffixes: vec![spec.text_path(arrival)],
        };
        let analysis = analyze_with(&disk, &folder, &Answers::default(), &options());
        assert!(
            matches!(
                &analysis,
                Analysis::Refused {
                    refusal: Refusal::CloudPlaceholders { count: 1, .. },
                    ..
                }
            ),
            "{analysis:?}"
        );
    }

    /// A dataless folder (macOS 14 and later) is not even listed, and what the
    /// import needs below it refuses; a picture in the cloud is skipped.
    #[test]
    fn a_dataless_folder_is_never_listed() {
        let spec = Spec::novel(FixtureGeneration::Format20, FixturePlatform::Mac);
        let (_dir, folder) = written(&spec);
        let arrival = spec
            .items
            .iter()
            .position(|i| i.title.as_deref() == Some("Arrival"))
            .unwrap_or(0);
        let item_folder = spec
            .payload_prefix(arrival)
            .trim_end_matches('/')
            .to_string();
        let disk = Clouded {
            suffixes: vec![item_folder],
        };
        let analysis = analyze_with(&disk, &folder, &Answers::default(), &options());
        assert!(
            matches!(
                &analysis,
                Analysis::Refused {
                    refusal: Refusal::CloudPlaceholders { .. },
                    ..
                }
            ),
            "{analysis:?}"
        );
        let map = spec
            .items
            .iter()
            .position(|i| i.title.as_deref() == Some("Harbour map"))
            .unwrap_or(0);
        let disk = Clouded {
            suffixes: vec![format!("{}content.png", spec.payload_prefix(map))],
        };
        let analysis = analyze_with(&disk, &folder, &Answers::default(), &options());
        let Analysis::Read(read) = analysis else {
            panic!("a picture in the cloud does not block the import: {analysis:?}");
        };
        assert!(
            read.notices
                .contains(&Notice::CloudPlaceholdersSkipped { count: 1 })
        );
    }

    /// A sync copy of the manifest that is a cloud placeholder is never read, and
    /// does not block the project: only the manifest read has to be readable.
    #[test]
    fn an_unreadable_manifest_copy_does_not_block_the_project() {
        let spec = Spec::novel(FixtureGeneration::Format20, FixturePlatform::Mac);
        let mut files = spec.build();
        let copy = scrivener_fixtures::traps::copy_name(
            "Lighthouse.scrivx",
            scrivener_fixtures::traps::CopyScheme::IsoConflicted,
        );
        files.put(copy.clone(), b"never read".to_vec());
        let dir = match tempfile::tempdir() {
            Ok(dir) => dir,
            Err(e) => panic!("scratch folder: {e}"),
        };
        let folder = dir.path().join("Lighthouse.scriv");
        if let Err(e) = files.write_to(&folder) {
            panic!("writing the fixture: {e}");
        }
        let disk = Clouded {
            suffixes: vec![copy.clone()],
        };
        let analysis = analyze_with(&disk, &folder, &Answers::default(), &options());
        let Analysis::Read(read) = analysis else {
            panic!("the project reads: {analysis:?}");
        };
        assert_eq!(
            read.project.source.manifest.chosen,
            ManifestChoice::Automatic
        );
    }

    /// S5: reading a project touches nothing outside it. The only paths outside
    /// the project folder ever looked at are the ones that find it.
    #[test]
    fn nothing_outside_the_project_is_touched() {
        let spec = Spec::novel(FixtureGeneration::Format15, FixturePlatform::Windows);
        let (dir, folder) = written(&spec);
        let disk = Recording::default();
        let analysis = analyze_with(&disk, &folder, &Answers::default(), &options());
        assert!(matches!(analysis, Analysis::Read(_)));
        let outside: Vec<PathBuf> = disk
            .touched
            .borrow()
            .iter()
            .filter(|p| !p.starts_with(&folder))
            .cloned()
            .collect();
        assert!(outside.is_empty(), "{outside:?}");

        // Picked from the folder around it, the search lists that folder, and
        // reads nothing in it.
        let disk = Recording::default();
        let _ = analyze_with(&disk, dir.path(), &Answers::default(), &options());
        let outside: Vec<PathBuf> = disk
            .touched
            .borrow()
            .iter()
            .filter(|p| !p.starts_with(&folder) && p.as_path() != dir.path())
            .cloned()
            .collect();
        assert!(outside.is_empty(), "{outside:?}");
    }

    /// On a disk that compares names regardless of case or Unicode normalisation
    /// (APFS, NTFS), a pick spelled otherwise than the folder lists it (a typed
    /// path, a path from another program) still names the manifest picked.
    #[test]
    fn a_picked_manifest_is_found_however_the_pick_spells_it() {
        let mut spec = Spec::novel(FixtureGeneration::Format20, FixturePlatform::Mac);
        // Decomposed on disk, as HFS+ writes it.
        spec.manifest_stem = Some("E\u{301}cume".to_string());
        let (_dir, folder) = written(&spec);
        for spelling in ["\u{c9}cume.scrivx", "E\u{301}CUME.scrivx"] {
            let mut notices = Vec::new();
            match folder_pick(
                &RealDisk,
                &folder,
                Some(spelling.to_string()),
                &Answers::default(),
                &options(),
                &mut notices,
                &mut None,
            ) {
                Ok(Analysis::Read(read)) => {
                    assert_eq!(
                        read.project.source.manifest.chosen,
                        ManifestChoice::Picked,
                        "{spelling}"
                    );
                    assert_eq!(read.project.source.manifest.name, "E\u{301}cume.scrivx");
                }
                Ok(other) => panic!("{spelling}: {other:?}"),
                Err(Stop::Refuse(refusal)) => panic!("{spelling}: refused {refusal:?}"),
                Err(Stop::Choose(question)) => panic!("{spelling}: asked {question:?}"),
                Err(Stop::Changed(count)) => panic!("{spelling}: {count} changed"),
            }
        }
    }

    #[test]
    fn a_creator_names_its_platform_and_version() {
        let mac = writer_of(Some("SCRMAC-3.5.2-17487"));
        assert_eq!(
            (mac.platform, mac.version.as_deref()),
            (Platform::Mac, Some("3.5.2"))
        );
        let win = writer_of(Some("SCRWIN-1.9.16.0"));
        assert_eq!(
            (win.platform, win.version.as_deref()),
            (Platform::Windows, Some("1.9.16.0"))
        );
        let stub = writer_of(Some("SCRMAC/2.5/25238"));
        assert_eq!(
            (stub.platform, stub.version.as_deref()),
            (Platform::Mac, Some("2.5"))
        );
        assert_eq!(writer_of(Some("scrivener-rs")).platform, Platform::Unknown);
        assert_eq!(writer_of(None).platform, Platform::Unknown);
    }
}
