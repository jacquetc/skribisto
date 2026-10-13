// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The converter: a Scrivener project, the structure the writer reviewed, and its prose,
//! written as a newest-version `.skrib`.
//!
//! Store-free, like the Plume and Manuskript converters: nothing here touches an entity.
//! The interface opens the result with the ordinary `load_work`.
//!
//! # What it writes
//!
//! Exactly what the [`StructurePlan`] says, recomputed from the same analysis and the
//! writer's [`Choices`], so the review and the import cannot disagree: two binders of rows,
//! every row's contents made from the Scrivener texts the plan lists for it, footnotes and
//! comments beside the prose that cites them, pictures as assets, the story bible's tags,
//! the labels as tags and the status list as the workflow ladder on the rows that wore them
//! ([`metadata::vocabularies`]), the keywords, fields and collections as tags, the goals,
//! the references, the dictionary, the pace plan and the "From Scrivener" folder's notes
//! ([`metadata::extended`]), the note templates, and one trash entry per trashed subtree. The project's title, author and language are the Save page's, else what the
//! project states ([`metadata::Proposals`]). A row carries the dates of the Scrivener item
//! whose data it holds, read strictly; an unreadable one, and everything the import makes
//! up, carries the import's own instant (D6.9). Titles and names the import adds are the
//! caller's words in the writer's language ([`Words`]); a label's or a rung's name is the
//! writer's own, stored as written: nothing English is written into the project.
//!
//! # Reading once, consistently
//!
//! The project is analysed again and converted while it is still open, through the same
//! index: the check that nothing moved while the project was read covers the conversion's
//! reads too. A project that changed since the review (its fingerprint moved) is refused,
//! for the interface to analyse again.
//!
//! # Nothing half-written
//!
//! The `.skrib` is staged beside its target and moved into place only once whole and not
//! cancelled (`skrib_format::importing::StagedOutput`): a refusal, a cancellation, an error
//! or a panic leaves the target as it was and no staged file behind. A failure to write
//! names the folder that refused. The research files copied beside it go into place with it
//! or not at all.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use chrono::{DateTime, NaiveDate, Utc};
use common::entities::{CommentAnchorKind, ContentRole, StatusCategory};
use document_ingest::pictures::{DocumentPictures, PictureRefusal};
use document_ingest::plan::{PlannedComment, RowPart, assemble_row};
use document_ingest::sources::rich::{self, RichBlock, RichDocument, Run};
use document_ingest::sources::rtf::SourcedPicture;
use document_ingest::{ImportDiagnostic, SourceDocument};
use serde::{Deserialize, Serialize};
use skrib_format::importing::{OutputRefused, StagedOutput};
use skrib_format::{
    BinderFile, BinderItemFile, BinderStatusFile, BinderTagFile, BundledBinder, BundledItem,
    CommentFile, DictWordFile, FORMAT_VERSION, FootnoteFile, HolidayFile, InlineContent,
    NoteTemplateFile, PaceFile, ProjectManifest, ProseRef, ShapeTag, TrashInfoFile, WorkBundle,
    WorkFile, binder_dir_name, new_unique_id, prose_file_name, prose_kind, prose_relpath,
};
use skribisto_model::comment_anchor;
use uuid::Uuid;

use crate::analyze::{self, Answers, Moved, Options, ReadProject};
use crate::disk::{Cancellable, RealDisk};
use crate::index::{MAX_MEMBER_BYTES, ReadError};
use crate::layout::Tree;
use crate::metadata::{self, Extended, MetadataNotice, MetadataWords, PALETTE};
use crate::model::{
    Generation, ItemKey, ItemKind, MemberRef, Platform, Project, SourceFacts, SourceNames, TextRole,
};
use crate::outcome::{Notice, Question, Refusal};
use crate::prose::{self, Pictures, Place, ProseChoices, ProseNotice, dialect, sidecar, styles};
use crate::structure::{
    self, BinderKind, Body, ChapterEncoding, Choices, NoteSection, Origin, PlanError, PlannedRow,
    PlannedTitle, RowRef, Source, StructureNotice, StructurePlan, SubRole,
};

mod research;
use research::ResearchCopy;

/// Where a pattern's value goes in [`Words`]: the one slot every importer's patterns hold,
/// so the interface resolves all of them alike.
pub use skrib_format::importing::TITLE_SLOT;

/// The words the import writes into the project, in the writer's language. The interface
/// resolves them once, when it starts the import; the converter stores them as they are.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Words {
    pub manuscript_binder: String,
    pub notes_binder: String,
    /// The title of a note holding an item's document notes, with [`TITLE_SLOT`] where the
    /// item's title goes. A pattern without the slot has the title put before it.
    pub notes_of: String,
    /// The folder of format 1.x project notes.
    pub project_notes: String,
    /// The folder of texts found on disk that no binder item listed.
    pub recovered: String,
    /// A note template whose document had no title.
    pub untitled_template: String,
    /// The folder beside the project that research files are copied into, with
    /// [`TITLE_SLOT`] where the project's file name goes.
    pub research_folder: String,
    /// What goes between two authors' names when the project names several (`, `).
    pub list_separator: String,
    /// A label tag's details, saying where it came from: [`TITLE_SLOT`] is where the
    /// Scrivener field's own name goes ("Label", or "Time" where the writer renamed it).
    pub label_details: String,
    /// A status rung's details, the same way.
    pub status_details: String,
    /// A label with no name, with [`TITLE_SLOT`] where its number goes.
    pub unnamed_label: String,
    /// A status with no name, the same way.
    pub unnamed_status: String,
    /// A label tag whose name another tag already holds, with [`TITLE_SLOT`] where the
    /// label's name goes (the English interface's `… (label)`).
    pub label_tag: String,
    /// A story-bible tag named after a folder that had no title.
    pub untitled_tag: String,
    /// The workflow ladder a project with no status list starts with: Skribisto's default
    /// preset, resolved in the writer's language, as a new project gets it. Empty seeds
    /// nothing.
    pub default_statuses: Vec<DefaultRung>,
    /// The words of the second half of the metadata mapping: the keyword, field and
    /// collection tags, the pace plan, and the "From Scrivener" folder.
    pub metadata: MetadataWords,
}

/// A rung of the default ladder, as the interface resolved it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefaultRung {
    pub name: String,
    pub category: StatusCategory,
}

/// One conversion.
#[derive(Debug, Clone)]
pub struct Request<'a> {
    /// What the writer picked, as the analysis was given it.
    pub source: &'a Path,
    pub answers: &'a Answers,
    pub options: &'a Options,
    /// The writer's choices from the review.
    pub choices: &'a Choices,
    pub prose: ProseChoices,
    /// The fingerprint of the analysis the writer reviewed. The conversion refuses a
    /// project that changed since; `None` converts whatever it reads.
    pub fingerprint: Option<&'a str>,
    /// Where the `.skrib` goes.
    pub output: &'a str,
    pub overwrite: bool,
    pub words: &'a Words,
    /// The project's title as the Save page has it; `None` takes the one the project
    /// states ([`crate::metadata::Proposals`]).
    pub title: Option<&'a str>,
    /// The author's name as the Save page has it; `None` takes the authors the project
    /// names, joined with [`Words::list_separator`], or none.
    pub author: Option<&'a str>,
    /// The manuscript's language, a language tag (`fr-FR`), as the Save page has it: the
    /// one the project states, else the interface's own. `None` takes the one the project
    /// states, or none.
    pub language: Option<&'a str>,
    /// Copy the research files Skribisto cannot open into a folder beside the project
    /// (UQ-37: on by default). Off, their notes name where they are in the Scrivener
    /// project, which keeps them.
    pub copy_research_files: bool,
    /// The instant of the import: the date of everything Scrivener does not date.
    pub now: DateTime<Utc>,
    /// The day of the import on the writer's own calendar, which a pace plan starts on and
    /// a deadline must not be behind: the interface's, which knows the writer's time zone
    /// (`now`'s UTC date is a day early or late for part of every day).
    pub today: NaiveDate,
}

/// What the new project holds.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Totals {
    /// Rows written, across both binders.
    pub rows: u32,
    /// Words of the manuscript's prose, as the app counts them.
    pub words: u64,
    pub footnotes: u32,
    pub comments: u32,
    /// Pictures stored, each once.
    pub pictures: u32,
    pub templates: u32,
    /// Tags: the story bible's and the labels'.
    pub tags: u32,
    /// Rungs of the workflow ladder.
    pub rungs: u32,
    /// Rows written in the Trash.
    pub trashed: u32,
    /// Research files copied beside the project.
    pub research_files: u32,
}

/// What a conversion did.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    pub output_path: String,
    pub totals: Totals,
    /// What the analysis said, read again with the conversion.
    pub notices: Vec<Notice>,
    /// What the structure mapping decided that the writer should know.
    pub structure: Vec<StructureNotice>,
    /// What converting the prose did.
    pub prose: Vec<ProseNotice>,
    /// What mapping the labels, the status list and the dates did.
    pub metadata: Vec<MetadataNotice>,
    /// Where each binder item went (D3.21), for the report and the tests.
    pub placements: Vec<structure::Placement>,
    /// Every row written, in the plan's order: the uid each carries in the new project, so
    /// the report can open the row a notice is about, and what it came from.
    pub rows: Vec<WrittenRow>,
    /// How the writer knew each item, project note and recovered text the notices name.
    pub names: SourceNames,
    /// The folder beside the project the research files were copied into, by name.
    pub research_folder: Option<String>,
}

/// A row the conversion wrote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WrittenRow {
    /// Which planned row it is: what [`Summary::placements`] point at.
    pub at: RowRef,
    /// The uid it carries in the new project, which a reload keeps.
    pub uid: Uuid,
    pub origin: Origin,
}

/// Why a conversion wrote nothing.
#[derive(Debug, Clone, PartialEq)]
pub enum ConvertError {
    /// The project cannot be imported: the analysis's own refusal.
    Refused(Refusal),
    /// The analysis needs an answer first.
    Choose(Question),
    /// Questions the writer must accept before importing, not yet accepted.
    NotAccepted(Vec<Question>),
    /// The project changed since the review: analyse it again.
    ChangedSinceReview,
    /// The writer's choices do not make a plan.
    Plan(PlanError),
    /// The new project could not be put in place.
    Output(OutputRefused),
    /// The writer cancelled. Nothing was written.
    Cancelled,
}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::Refused(refusal) => write!(f, "refused: {}", refusal.key()),
            ConvertError::Choose(question) => write!(f, "a question first: {}", question.key()),
            ConvertError::NotAccepted(questions) => {
                write!(f, "{} question(s) not accepted", questions.len())
            }
            ConvertError::ChangedSinceReview => f.write_str("the project changed since the review"),
            ConvertError::Plan(e) => write!(f, "{e}"),
            ConvertError::Output(e) => write!(f, "{e}"),
            ConvertError::Cancelled => f.write_str("the import was cancelled"),
        }
    }
}

impl std::error::Error for ConvertError {}

/// Convert the Scrivener project `request.source` names into a `.skrib` at
/// `request.output`, reporting progress and honouring `cancel` between rows.
///
/// `report(percent, label)` drives the progress notice. `label` is either a phase
/// code (`import.opening`, `import.outline`, `import.converting`, `import.writing`,
/// `import.done`), which the interface words (this crate has no catalogue), or the
/// title of the row just converted, shown as data.
pub fn convert(
    request: &Request<'_>,
    report: &dyn Fn(f32, &str),
    cancel: &AtomicBool,
) -> Result<Summary, ConvertError> {
    // A project read again because it moved under the read (a sync service catching up)
    // goes through its phases a second time; the share of the work the writer watches
    // never goes back, and the phase says what is happening again.
    let furthest = Cell::new(0.0f32);
    let report = &|percent: f32, label: &str| {
        let percent = percent.max(furthest.get());
        furthest.set(percent);
        report(percent, label);
    };
    // Before the project is even opened: whether the target may be replaced.
    let output =
        StagedOutput::begin(request.output, request.overwrite).map_err(ConvertError::Output)?;
    let mut research = request
        .copy_research_files
        .then(|| ResearchCopy::beside(request.output, &request.words.research_folder));
    report(2.0, "import.opening");
    let mut built: Option<Result<Built, ConvertError>> = None;
    let mut visit = |tree: &Tree<'_, '_>, read: &ReadProject| -> Result<(), Moved> {
        report(12.0, "import.outline");
        if let Some(research) = research.as_mut() {
            research.restart();
        }
        match build(tree, read, request, report, cancel, research.as_mut()) {
            Err(Stop::Moved) => Err(Moved),
            Err(Stop::Error(e)) => {
                built = Some(Err(e));
                Ok(())
            }
            Ok(done) => {
                built = Some(Ok(done));
                Ok(())
            }
        }
    };
    let mut visit_ref: analyze::Visit<'_> = Some(&mut visit);
    // Every read stops once the writer cancels, so a project on a slow drive is not read to
    // its end first; what the refused reads made of the analysis is then set aside.
    let disk = Cancellable {
        inner: &RealDisk,
        cancel,
    };
    let analysis = analyze::analyze_visiting(
        &disk,
        request.source,
        request.answers,
        request.options,
        &mut visit_ref,
    );
    if cancel.load(Ordering::Relaxed) {
        return Err(ConvertError::Cancelled);
    }
    let (notices, names) = match analysis {
        analyze::Analysis::Refused { refusal, .. } => return Err(ConvertError::Refused(refusal)),
        analyze::Analysis::Choose { question, .. } => return Err(ConvertError::Choose(question)),
        analyze::Analysis::Read(read) => (read.notices, read.project.source_names()),
    };
    let built = match built {
        Some(result) => result?,
        // The analysis read the project yet never handed it over: a defect of this crate,
        // reported as the project having moved rather than as an import that wrote nothing.
        None => return Err(ConvertError::ChangedSinceReview),
    };
    if cancel.load(Ordering::Relaxed) {
        return Err(ConvertError::Cancelled);
    }

    report(92.0, "import.writing");
    output.write(&built.bundle).map_err(ConvertError::Output)?;
    if cancel.load(Ordering::Relaxed) {
        return Err(ConvertError::Cancelled);
    }
    // The research folder just before the project, so that neither stands without the
    // other: one that cannot be placed fails the import whole, and one placed goes again
    // if the project then cannot be (dropped, it removes itself).
    let research_folder = match research.as_mut() {
        Some(research) => research.place().map_err(ConvertError::Output)?,
        None => None,
    };
    output.commit().map_err(ConvertError::Output)?;
    if let Some(research) = research {
        research.keep();
    }
    report(100.0, "import.done");
    Ok(Summary {
        output_path: request.output.to_string(),
        totals: built.totals,
        notices,
        structure: built.structure,
        prose: built.prose,
        metadata: built.metadata,
        placements: built.placements,
        rows: built.rows,
        names,
        research_folder,
    })
}

/// A project converted, not yet written.
struct Built {
    bundle: WorkBundle,
    totals: Totals,
    structure: Vec<StructureNotice>,
    prose: Vec<ProseNotice>,
    metadata: Vec<MetadataNotice>,
    placements: Vec<structure::Placement>,
    rows: Vec<WrittenRow>,
}

/// Why building stopped.
enum Stop {
    /// A member moved under a read: the whole project is read again.
    Moved,
    Error(ConvertError),
}

impl From<ConvertError> for Stop {
    fn from(e: ConvertError) -> Self {
        Stop::Error(e)
    }
}

fn build(
    tree: &Tree<'_, '_>,
    read: &ReadProject,
    request: &Request<'_>,
    report: &dyn Fn(f32, &str),
    cancel: &AtomicBool,
    research: Option<&mut ResearchCopy>,
) -> Result<Built, Stop> {
    if let Some(expected) = request.fingerprint
        && expected != read.fingerprint
    {
        return Err(ConvertError::ChangedSinceReview.into());
    }
    if !read.pending.is_empty() {
        return Err(ConvertError::NotAccepted(read.pending.clone()).into());
    }
    let plan = structure::plan(&read.project, &read.content, request.choices)
        .map_err(ConvertError::Plan)?;
    let mut converter = Converter::new(tree, read, request, &plan, research);
    converter.load_shared()?;
    let bundle = converter.run(report, cancel)?;
    converter.totals.research_files = converter.research.as_ref().map_or(0, |r| r.copied());
    Ok(Built {
        totals: converter.totals,
        structure: plan.notices.clone(),
        prose: std::mem::take(&mut converter.notices),
        metadata: std::mem::take(&mut converter.metadata),
        placements: plan.placements.clone(),
        rows: std::mem::take(&mut converter.written),
        bundle,
    })
}

/// Sequential file ids: they only have to be consistent within the bundle, and the load
/// path mints fresh store ids from them.
#[derive(Default)]
struct Ids(u64);

impl Ids {
    fn take(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }
}

/// What a row wears and holds beyond its writing: its tags (its story-bible tag, its
/// label's, its keywords', its fields', its collections') and its rung, as file ids, its
/// goals, and what its own note lists of its metadata.
struct Worn<'e> {
    tags: Vec<u64>,
    status: Option<u64>,
    /// Its word goal and its character goal.
    goals: (i64, i64),
    /// Djot to follow its note text: its own custom metadata and bookmarks (D4-8, D4-20).
    sheet: Option<&'e str>,
    /// The metadata mapping's notes, for a row of the "From Scrivener" folder.
    extended: &'e Extended,
}

/// One text's contribution to a content, before the content is assembled.
struct Piece {
    doc: SourceDocument,
    at: Place,
    links: Vec<String>,
    placeholders: BTreeMap<String, u32>,
}

/// A row's content, assembled.
struct ContentOut {
    role: ContentRole,
    djot: String,
    comments: Vec<CommentFile>,
    footnotes: Vec<FootnoteFile>,
    words: usize,
}

struct Converter<'c, 't, 'i, 'd, 'r> {
    tree: &'t Tree<'i, 'd>,
    /// Where research files are copied, when they are.
    research: Option<&'r mut ResearchCopy>,
    read: &'c ReadProject,
    project: &'c Project,
    request: &'c Request<'c>,
    plan: &'c StructurePlan,
    now: String,
    ids: Ids,
    styles: Option<styles::StyleTable>,
    footnote_marker: Option<String>,
    pictures: ProjectPictures<'t, 'i, 'd, 'c>,
    /// The last footnote label minted: labels are `fn1`, `fn2`… project-wide, in the order
    /// the rows cite them.
    label: Cell<u32>,
    /// Texts split at their chapter headings, kept by item until each part is asked for.
    split: HashMap<u32, Vec<Option<Piece>>>,
    /// Asset paths the stored prose names.
    referenced: BTreeSet<String>,
    notices: Vec<ProseNotice>,
    metadata: Vec<MetadataNotice>,
    /// Items, and format 1.x project notes, whose dates did not read: dated the import.
    undated: RefCell<BTreeSet<(bool, u32)>>,
    /// The content being made is manuscript prose that exports: what Scrivener's compile
    /// left out of it will print, and is said.
    printed: Cell<bool>,
    totals: Totals,
    /// Every row written, in the plan's order.
    written: Vec<WrittenRow>,
}

impl<'c, 't, 'i, 'd, 'r> Converter<'c, 't, 'i, 'd, 'r> {
    fn new(
        tree: &'t Tree<'i, 'd>,
        read: &'c ReadProject,
        request: &'c Request<'c>,
        plan: &'c StructurePlan,
        research: Option<&'r mut ResearchCopy>,
    ) -> Self {
        let project = &read.project;
        let mut by_uuid: HashMap<String, usize> = HashMap::new();
        for (index, row) in project.binder.iter().enumerate() {
            if let Some(ItemKey::Uuid(uuid)) = row.key
                && row.owns_key
            {
                by_uuid.insert(uuid.hyphenated().to_string().to_ascii_uppercase(), index);
            }
        }
        Converter {
            tree,
            research,
            read,
            project,
            request,
            plan,
            now: request.now.to_rfc3339(),
            ids: Ids::default(),
            styles: None,
            footnote_marker: None,
            pictures: ProjectPictures {
                tree,
                project,
                store: RefCell::new(DocumentPictures::new()),
                moved: Cell::new(false),
                by_uuid,
            },
            label: Cell::new(0),
            split: HashMap::new(),
            referenced: BTreeSet::new(),
            notices: Vec::new(),
            metadata: Vec::new(),
            undated: RefCell::new(BTreeSet::new()),
            printed: Cell::new(false),
            totals: Totals::default(),
            written: Vec::new(),
        }
    }

    /// Read a member, `Ok(None)` when it cannot be read (said by the caller), a move
    /// stopping everything.
    fn member(&self, member: &MemberRef, limit: u64) -> Result<Option<Vec<u8>>, Stop> {
        match self.tree.read(&member.path, limit) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(ReadError::Changed) => Err(Stop::Moved),
            Err(ReadError::Refused(refusal)) => Err(ConvertError::Refused(refusal).into()),
            Err(_) => Ok(None),
        }
    }

    /// The project's style sheet and footnote marker, read once.
    fn load_shared(&mut self) -> Result<(), Stop> {
        if let Some(styles) = &self.project.styles
            && let Some(bytes) = self.member(styles, MAX_MEMBER_BYTES)?
        {
            self.styles = styles::StyleTable::read(&bytes).ok();
            if let Some(table) = self.styles.as_mut() {
                let (deleted, raw) = metadata::compiled_away(&self.project.compile);
                table.mark_compiled(&deleted, &raw);
            }
        }
        if let Some(preferences) = &self.project.settings.project_preferences
            && let Some(bytes) = self.member(preferences, MAX_MEMBER_BYTES)?
        {
            self.footnote_marker = footnote_marker(&bytes);
        }
        Ok(())
    }

    fn run(&mut self, report: &dyn Fn(f32, &str), cancel: &AtomicBool) -> Result<WorkBundle, Stop> {
        let plan = self.plan;
        let total = (plan.manuscript.len() + plan.notes.len() + plan.templates.len()) as u64;
        report(20.0, "import.converting");
        let mut done = 0u64;
        // Reported by the percent, not by the row, and always at the last row, with the
        // title of the row just converted as data: each report redraws a notice on the
        // interface's thread, and one per row of a large project kept that thread too busy
        // to take the notice's Cancel in time.
        let reported = std::cell::Cell::new(u64::MAX);
        let tick = |done: u64, title: &str| {
            let share = if total == 0 {
                1.0
            } else {
                done as f32 / total as f32
            };
            let percent = (100.0 * share) as u64;
            if percent == reported.get() && done != total {
                return;
            }
            reported.set(percent);
            report(20.0 + 70.0 * share, title);
        };

        // Every row's id first: a tag names the row it creates in, a trash entry the row it
        // trashed, a link the row it points at.
        let manuscript_id = self.ids.take();
        let notes_id = self.ids.take();
        let row_ids: [Vec<u64>; 2] = [
            plan.manuscript.iter().map(|_| self.ids.take()).collect(),
            plan.notes.iter().map(|_| self.ids.take()).collect(),
        ];
        let tag_ids: Vec<u64> = plan.tags.iter().map(|_| self.ids.take()).collect();
        let template_ids: Vec<u64> = plan.templates.iter().map(|_| self.ids.take()).collect();
        // The labels and the status list (D4-3, D4-4), named against the story bible's tags.
        let words = self.request.words;
        let bible_names: Vec<String> = plan
            .tags
            .iter()
            .map(|tag| {
                if tag.name.trim().is_empty() {
                    words.untitled_tag.clone()
                } else {
                    tag.name.clone()
                }
            })
            .collect();
        let vocabularies = metadata::vocabularies(self.project, plan, &bible_names, words);
        let label_ids: Vec<u64> = vocabularies.tags.iter().map(|_| self.ids.take()).collect();
        let rung_ids: Vec<u64> = vocabularies
            .ladder
            .iter()
            .map(|_| self.ids.take())
            .collect();
        // The keywords, fields, collections, targets, bookmarks, deadline and dictionary
        // (D4-6 to D4-20), named against every tag above.
        let language = self.work_language();
        let earlier: Vec<String> = bible_names
            .iter()
            .chain(vocabularies.tags.iter().map(|t| &t.name))
            .cloned()
            .collect();
        let extended = metadata::extended(
            self.project,
            plan,
            &earlier,
            words,
            language.as_deref(),
            self.request.today,
        );
        let extended_ids: Vec<u64> = extended.tags.iter().map(|_| self.ids.take()).collect();

        let mut binders: Vec<BundledBinder> = Vec::new();
        let mut binder_order: Vec<u64> = Vec::new();
        // Links each row's texts hold, to resolve once every row exists.
        let mut links: Vec<(BinderKind, usize, Place, Vec<String>)> = Vec::new();
        for (kind, binder_id, name, index) in [
            (
                BinderKind::Manuscript,
                manuscript_id,
                self.request.words.manuscript_binder.clone(),
                0usize,
            ),
            (
                BinderKind::Notes,
                notes_id,
                self.request.words.notes_binder.clone(),
                1,
            ),
        ] {
            let rows = match kind {
                BinderKind::Manuscript => &plan.manuscript,
                BinderKind::Notes => &plan.notes,
            };
            if rows.is_empty() && kind == BinderKind::Notes {
                continue;
            }
            let dir = binder_dir_name(binders.len(), &name);
            let mut items = Vec::with_capacity(rows.len());
            for (r, row) in rows.iter().enumerate() {
                if cancel.load(Ordering::Relaxed) {
                    return Err(ConvertError::Cancelled.into());
                }
                let at = RowRef {
                    binder: kind,
                    row: r as u32,
                };
                let worn = Worn {
                    tags: row
                        .tag
                        .and_then(|t| tag_ids.get(t as usize).copied())
                        .into_iter()
                        .chain(
                            vocabularies.row_tags[index]
                                .get(r)
                                .copied()
                                .flatten()
                                .and_then(|t| label_ids.get(t as usize).copied()),
                        )
                        .chain(
                            extended.row_tags[index]
                                .get(r)
                                .into_iter()
                                .flatten()
                                .filter_map(|t| extended_ids.get(*t as usize).copied()),
                        )
                        .collect(),
                    status: vocabularies.row_rungs[index]
                        .get(r)
                        .copied()
                        .flatten()
                        .and_then(|k| rung_ids.get(k as usize).copied()),
                    goals: extended.goals.get(&at).copied().unwrap_or((0, 0)),
                    sheet: extended.sheets.get(&at).map(String::as_str),
                    extended: &extended,
                };
                let (item, row_links) = self.row(kind, row, row_ids[index][r], &dir, worn)?;
                self.written.push(WrittenRow {
                    at: RowRef {
                        binder: kind,
                        row: r as u32,
                    },
                    uid: item.item.uid,
                    origin: row.origin,
                });
                if let Some((place, targets)) = row_links {
                    links.push((kind, r, place, targets));
                }
                let title = item.item.title.clone();
                items.push(item);
                done += 1;
                tick(done, &title);
            }
            binders.push(BundledBinder {
                binder: BinderFile {
                    file_id: binder_id,
                    uid: common::uid::new_uid(),
                    created_at: self.now.clone(),
                    updated_at: self.now.clone(),
                    name,
                    activated: true,
                    item_order: items.iter().map(|i| i.item.file_id).collect(),
                },
                items,
            });
            binder_order.push(binder_id);
        }
        self.resolve_links(&mut binders, &row_ids, links);
        // Bookmarks Skribisto shows, beside the links the prose made, each once (X-35).
        let file_id_of = |at: RowRef| match at.binder {
            BinderKind::Manuscript => row_ids[0].get(at.row as usize).copied(),
            BinderKind::Notes => row_ids[1].get(at.row as usize).copied(),
        };
        let edges: Vec<(u64, u64)> = extended
            .edges
            .iter()
            .filter_map(|(owner, target)| Some((file_id_of(*owner)?, file_id_of(*target)?)))
            .collect();
        attach_references(
            binders
                .iter_mut()
                .flat_map(|b| b.items.iter_mut())
                .map(|i| (i.item.file_id, &mut i.item.reference_ids)),
            &edges,
            cancel,
        )?;

        // Note templates (D3.16).
        let mut note_templates = Vec::with_capacity(plan.templates.len());
        let mut note_template_bodies = BTreeMap::new();
        for (t, template) in plan.templates.iter().enumerate() {
            if cancel.load(Ordering::Relaxed) {
                return Err(ConvertError::Cancelled.into());
            }
            let uid = common::uid::new_uid();
            let name = if template.name.trim().is_empty() {
                self.request.words.untitled_template.clone()
            } else {
                template.name.clone()
            };
            let body = match &template.body {
                Some(source) => {
                    let out = self.content(
                        BinderKind::Notes,
                        ContentRole::NoteText,
                        std::slice::from_ref(source),
                        false,
                        None,
                    )?;
                    // A template's body is Djot alone: a note or a comment has nothing to
                    // hang on there.
                    if let Some(out) = &out
                        && (!out.0.comments.is_empty() || !out.0.footnotes.is_empty())
                    {
                        self.notices.push(ProseNotice::Document {
                            at: Place::Text {
                                item: template.item,
                            },
                            key: "footnote-not-carried".to_string(),
                            warning: true,
                            count: (out.0.comments.len() + out.0.footnotes.len()) as u32,
                            detail: String::new(),
                        });
                    }
                    // Nor a reference to a note the body does not hold.
                    out.map(|(content, _)| {
                        let labels: Vec<&str> =
                            content.footnotes.iter().map(|n| n.label.as_str()).collect();
                        without_references(&content.djot, &labels)
                    })
                    .unwrap_or_default()
                }
                None => String::new(),
            };
            let file_id = template_ids[t];
            let title = name.clone();
            note_templates.push(NoteTemplateFile {
                file_id,
                uid,
                created_at: self.now.clone(),
                updated_at: self.now.clone(),
                path: skrib_format::slug::note_template_relpath(uid, &name),
                name,
                starred: false,
            });
            note_template_bodies.insert(file_id, body);
            self.totals.templates += 1;
            done += 1;
            tick(done, &title);
        }

        // Story-bible tags (D3.14), discoverable, each creating its entries where the
        // folder that named it sits.
        let notes_ids = &row_ids[1];
        let mut tags: Vec<BinderTagFile> = plan
            .tags
            .iter()
            .enumerate()
            .map(|(t, tag)| BinderTagFile {
                file_id: tag_ids[t],
                uid: common::uid::new_uid(),
                created_at: self.now.clone(),
                updated_at: self.now.clone(),
                name: bible_names[t].clone(),
                color: PALETTE[t % PALETTE.len()].to_string(),
                details: String::new(),
                discoverable: true,
                creates_in: notes_ids.get(tag.creates_in as usize).copied(),
                note_template: tag
                    .template
                    .and_then(|k| template_ids.get(k as usize).copied()),
            })
            .collect();
        // Labels (D4-3): tags on the rows that wore them, never story-bible material.
        tags.extend(
            vocabularies
                .tags
                .iter()
                .zip(&label_ids)
                .map(|(label, file_id)| BinderTagFile {
                    file_id: *file_id,
                    uid: common::uid::new_uid(),
                    created_at: self.now.clone(),
                    updated_at: self.now.clone(),
                    name: label.name.clone(),
                    color: label.color.clone(),
                    details: label.details.clone(),
                    discoverable: false,
                    creates_in: None,
                    note_template: None,
                }),
        );
        // Keywords, fields and collections (D4-7, D4-8, D4-13): tags on the rows that wore
        // them, never story-bible material.
        tags.extend(
            extended
                .tags
                .iter()
                .zip(&extended_ids)
                .map(|(tag, file_id)| BinderTagFile {
                    file_id: *file_id,
                    uid: common::uid::new_uid(),
                    created_at: self.now.clone(),
                    updated_at: self.now.clone(),
                    name: tag.name.clone(),
                    color: tag.color.clone(),
                    details: tag.details.clone(),
                    discoverable: false,
                    creates_in: None,
                    note_template: None,
                }),
        );
        self.totals.tags = tags.len() as u32;
        // The workflow ladder (D4-4), in the list's order.
        let statuses: Vec<BinderStatusFile> = vocabularies
            .ladder
            .iter()
            .zip(&rung_ids)
            .map(|(rung, file_id)| BinderStatusFile {
                file_id: *file_id,
                // Minted here: the controller that mints a uid for a created row does not
                // run for a status (CLAUDE.md), and a nil one would not survive a reload.
                uid: common::uid::new_uid(),
                created_at: self.now.clone(),
                updated_at: self.now.clone(),
                name: rung.name.clone(),
                category: rung.category.clone(),
                details: rung.details.clone(),
            })
            .collect();
        self.totals.rungs = statuses.len() as u32;
        self.metadata.extend(vocabularies.notices);
        // One line for every tag renamed, whichever half renamed it.
        for notice in extended.notices.iter().cloned() {
            match (
                notice,
                self.metadata
                    .iter_mut()
                    .find(|n| matches!(n, MetadataNotice::TagNameDisambiguated { .. })),
            ) {
                (
                    MetadataNotice::TagNameDisambiguated { names },
                    Some(MetadataNotice::TagNameDisambiguated { names: earlier }),
                ) => earlier.extend(names),
                (notice, _) => self.metadata.push(notice),
            }
        }

        // The project's dictionary: auto-complete words (D4-17).
        let dict_words: Vec<DictWordFile> = extended
            .dictionary
            .iter()
            .map(|word| DictWordFile {
                file_id: self.ids.take(),
                created_at: self.now.clone(),
                updated_at: self.now.clone(),
                word: word.clone(),
            })
            .collect();

        // The deadline's pace plan (D4-12): from the import's day to the deadline's, both
        // midnight UTC as the format stores days.
        let midnight = |day: chrono::NaiveDate| {
            day.and_hms_opt(0, 0, 0)
                .map(|d| d.and_utc().to_rfc3339())
                .unwrap_or_else(|| self.now.clone())
        };
        let paces: Vec<PaceFile> = extended
            .pace
            .iter()
            .filter_map(|pace| {
                let book = file_id_of(pace.book)?;
                Some(PaceFile {
                    file_id: self.ids.take(),
                    created_at: self.now.clone(),
                    updated_at: self.now.clone(),
                    book_item: Some(book),
                    start_date: midnight(pace.start),
                    end_date: midnight(pace.end),
                    weekday_mask: pace.weekday_mask,
                    active: true,
                    holidays: pace
                        .day_off
                        .iter()
                        .map(|label| HolidayFile {
                            file_id: self.ids.take(),
                            created_at: self.now.clone(),
                            updated_at: self.now.clone(),
                            label: label.clone(),
                            start_date: midnight(pace.end),
                            end_date: None,
                        })
                        .collect(),
                    milestones: Vec::new(),
                })
            })
            .collect();

        // One trash entry per trashed subtree, dated the import (X-28).
        let trash_infos: Vec<TrashInfoFile> = plan
            .trash_roots
            .iter()
            .filter_map(|r| notes_ids.get(*r as usize))
            .map(|row| TrashInfoFile {
                file_id: self.ids.take(),
                created_at: self.now.clone(),
                updated_at: self.now.clone(),
                trashed_at: self.now.clone(),
                origin_binder_id: notes_id as i64,
                trashed_binder: None,
                trashed_binder_item: Some(*row),
            })
            .collect();

        if self.pictures.moved.get() {
            return Err(Stop::Moved);
        }
        let pictures = self
            .pictures
            .store
            .replace(DocumentPictures::new())
            .into_pictures();
        let mut assets = Vec::new();
        let mut asset_bytes = BTreeMap::new();
        for picture in pictures {
            // Only what the stored prose names: a picture of a heading that became a title,
            // say, has nowhere to be shown.
            if !self.referenced.contains(&picture.relative_path()) {
                continue;
            }
            assets.push(picture.asset_file(self.ids.take(), &self.now));
            asset_bytes.insert(picture.content_hash.clone(), picture.bytes);
        }
        self.totals.pictures = assets.len() as u32;

        let work = WorkFile {
            file_id: self.ids.take(),
            created_at: self.now.clone(),
            updated_at: self.now.clone(),
            title: self.work_title(),
            author_name: self.work_author(),
            // The project's own targets' unit, else the one New Work would start a project
            // in this language with (X-30).
            goal_unit: extended.goal_unit.clone(),
            dict_language: language.into_iter().collect(),
            tag_ids: tags.iter().map(|t| t.file_id).collect(),
            dict_word_ids: dict_words.iter().map(|w| w.file_id).collect(),
            unique_id: new_unique_id(),
            chapter_flat: plan.chapter_encoding == ChapterEncoding::Flat,
            text_replacement_rule_ids: Vec::new(),
            custom_replacement_rules_enabled: false,
            smart_punctuation: None,
            number_chapters: true,
            part_resets_chapter: false,
        };

        let undated = self.undated.borrow().len() as u32;
        if undated > 0 {
            self.metadata
                .push(MetadataNotice::DatesUnreadable { items: undated });
        }

        Ok(WorkBundle {
            manifest: ProjectManifest {
                format_version: FORMAT_VERSION,
                // The read floor is the writer's to compute when it commits the manifest.
                format_min_read_version: None,
                shape: ShapeTag::Zip,
                work,
                binder_order,
                kind: skrib_format::BundleKind::Regular,
                backup_of: None,
                backup_created_at: None,
            },
            tags,
            dict_words,
            text_replacement_rules: Vec::new(),
            note_templates,
            statuses,
            assets,
            note_template_bodies,
            asset_bytes,
            trash_infos,
            paces,
            progress_snapshots: Vec::new(),
            orphan_comments: Vec::new(),
            orphan_footnotes: Vec::new(),
            // Snapshots are not imported (decision 7): the history starts with the import.
            history: Default::default(),
            binders,
            carried: Default::default(),
        })
    }

    /// The project's title: the Save page's, else the one the project states (D4-15).
    fn work_title(&self) -> String {
        match self.request.title.filter(|t| !t.trim().is_empty()) {
            Some(title) => title.to_string(),
            None => self.read.proposals.title.title.clone(),
        }
    }

    /// The author: the Save page's, else the authors the project names.
    fn work_author(&self) -> String {
        match self.request.author {
            Some(author) => author.trim().to_string(),
            None => self
                .read
                .proposals
                .author
                .as_ref()
                .map(|a| a.names.join(&self.request.words.list_separator))
                .unwrap_or_default(),
        }
    }

    /// The manuscript's language: the Save page's, else the one the project states.
    fn work_language(&self) -> Option<String> {
        match self.request.language {
            Some(language) => {
                Some(language.trim().to_string()).filter(|language| !language.is_empty())
            }
            None => self.read.proposals.language.as_ref().map(|l| l.tag.clone()),
        }
    }

    fn title(&self, title: &PlannedTitle) -> String {
        let words = self.request.words;
        match title {
            PlannedTitle::Own(t) => t.clone(),
            PlannedTitle::NotesOf(t) => fill(&words.notes_of, t),
            PlannedTitle::ProjectTitle(t) => self
                .request
                .title
                .filter(|t| !t.trim().is_empty())
                .map_or_else(|| t.clone(), str::to_string),
            PlannedTitle::ProjectNotes => words.project_notes.clone(),
            PlannedTitle::Recovered => words.recovered.clone(),
            PlannedTitle::FromScrivener => words.metadata.from_scrivener.clone(),
            PlannedTitle::FromScrivenerNote(section) => {
                let w = &words.metadata;
                match section {
                    NoteSection::Project => w.note_project.clone(),
                    NoteSection::ProjectBookmarks => w.note_project_bookmarks.clone(),
                    NoteSection::Replacements => w.note_replacements.clone(),
                    NoteSection::AutoComplete => w.note_auto_complete.clone(),
                    NoteSection::SavedSearches => w.note_saved_searches.clone(),
                    NoteSection::CustomMetadata => w.note_custom_metadata.clone(),
                    NoteSection::Targets => w.note_targets.clone(),
                    NoteSection::Bookmarks => w.note_bookmarks.clone(),
                }
            }
        }
    }

    /// When a row's data was created and last changed: its Scrivener item's dates, else
    /// the import's (D6.9).
    fn dates(&self, origin: &Origin) -> (String, String) {
        let item = match origin {
            Origin::Item { item }
            | Origin::OwnText { item }
            | Origin::DocumentNotes { item }
            | Origin::SplitPart { item, .. }
            | Origin::TemplateLeftover { item }
            | Origin::DraftContent { item } => self
                .project
                .binder
                .get(*item as usize)
                .map(|row| (*item, row)),
            Origin::ProjectNote { note } => {
                let written = self
                    .project
                    .project_notes
                    .get(*note as usize)
                    .and_then(|n| n.modified.as_ref());
                let stamp = written.and_then(crate::dates::rfc3339);
                if written.is_some() && stamp.is_none() {
                    self.undated.borrow_mut().insert((true, *note));
                }
                let at = stamp.unwrap_or_else(|| self.now.clone());
                return (at.clone(), at);
            }
            Origin::Book
            | Origin::BookEnd
            | Origin::ProjectNotes
            | Origin::Recovered
            | Origin::RecoveredText { .. }
            | Origin::FromScrivener
            | Origin::FromScrivenerNote { .. } => None,
        };
        let Some((index, row)) = item else {
            return (self.now.clone(), self.now.clone());
        };
        let created = row.created.as_ref().and_then(crate::dates::rfc3339);
        let modified = row.modified.as_ref().and_then(crate::dates::rfc3339);
        // A date present and unreadable is said once per item, however many rows carry it,
        // and takes the import's instant (D6.9); a change never recorded is the creation.
        if (row.created.is_some() && created.is_none())
            || (row.modified.is_some() && modified.is_none())
        {
            self.undated.borrow_mut().insert((false, index));
        }
        let updated = match (&row.modified, modified) {
            (_, Some(modified)) => modified,
            (None, None) => created.clone().unwrap_or_else(|| self.now.clone()),
            (Some(_), None) => self.now.clone(),
        };
        (created.unwrap_or_else(|| self.now.clone()), updated)
    }

    /// One row of a binder, and the links its texts hold.
    #[allow(clippy::type_complexity)]
    fn row(
        &mut self,
        binder: BinderKind,
        row: &PlannedRow,
        file_id: u64,
        dir: &str,
        worn: Worn,
    ) -> Result<(BundledItem, Option<(Place, Vec<String>)>), Stop> {
        let item_uid = common::uid::new_uid();
        let title = self.title(&row.title);
        let (created, updated) = self.dates(&row.origin);
        let mut inline_contents = Vec::new();
        let mut prose_refs = Vec::new();
        let mut prose_map = BTreeMap::new();
        let mut comments = BTreeMap::new();
        let mut footnotes = BTreeMap::new();
        let mut row_links: Option<(Place, Vec<String>)> = None;
        let item_title = row_item_title(self.project, &row.origin);
        for content in &row.contents {
            if !skribisto_model::content_allowed(&row.role, &row.sub_role, &content.role) {
                continue;
            }
            let content_id = self.ids.take();
            match &content.body {
                Body::Title => inline_contents.push(InlineContent {
                    file_id: content_id,
                    uid: common::uid::new_uid(),
                    created_at: created.clone(),
                    updated_at: updated.clone(),
                    activated: row.activated,
                    role: content.role.clone(),
                    text: title.clone(),
                }),
                Body::From(sources) => {
                    let Some((out, extra)) = self.content(
                        binder,
                        content.role.clone(),
                        sources,
                        row.exportable,
                        item_title.as_deref(),
                    )?
                    else {
                        continue;
                    };
                    if let Some((place, targets)) = extra {
                        match &mut row_links {
                            Some((_, all)) => all.extend(targets),
                            None => row_links = Some((place, targets)),
                        }
                    }
                    if binder == BinderKind::Manuscript
                        && matches!(out.role, ContentRole::SceneText | ContentRole::ParatextText)
                    {
                        self.totals.words += out.words as u64;
                    }
                    let Some(name) = prose_file_name(item_uid, &title, &out.role) else {
                        continue;
                    };
                    if prose_kind(&out.role).is_none() {
                        continue;
                    }
                    self.totals.comments += out.comments.len() as u32;
                    self.totals.footnotes += out.footnotes.len() as u32;
                    prose_refs.push(ProseRef {
                        file_id: content_id,
                        uid: common::uid::new_uid(),
                        created_at: created.clone(),
                        updated_at: updated.clone(),
                        activated: row.activated,
                        role: out.role,
                        path: prose_relpath(dir, &name),
                    });
                    if !out.comments.is_empty() {
                        comments.insert(content_id, out.comments);
                    }
                    if !out.footnotes.is_empty() {
                        footnotes.insert(content_id, out.footnotes);
                    }
                    prose_map.insert(content_id, out.djot);
                }
                Body::Generated(section) => {
                    // A note of the "From Scrivener" folder: the metadata mapping's listing.
                    let Some(djot) = worn.extended.note(*section).filter(|d| !d.is_empty()) else {
                        continue;
                    };
                    let Some(name) = prose_file_name(item_uid, &title, &content.role) else {
                        continue;
                    };
                    prose_refs.push(ProseRef {
                        file_id: content_id,
                        uid: common::uid::new_uid(),
                        created_at: created.clone(),
                        updated_at: updated.clone(),
                        activated: row.activated,
                        role: content.role.clone(),
                        path: prose_relpath(dir, &name),
                    });
                    prose_map.insert(content_id, djot.to_string());
                }
            }
        }
        // A note-like row's own custom metadata and bookmarks, after its text (D4-8,
        // D4-20): its note text grows, or begins with them.
        if let Some(extra) = worn.sheet.filter(|d| !d.is_empty())
            && skribisto_model::content_allowed(&row.role, &row.sub_role, &ContentRole::NoteText)
        {
            let existing = prose_refs
                .iter()
                .find(|p| p.role == ContentRole::NoteText)
                .map(|p| p.file_id);
            match existing.and_then(|id| prose_map.get_mut(&id)) {
                Some(djot) => {
                    djot.push_str("\n\n");
                    djot.push_str(extra);
                }
                None => {
                    if let Some(name) = prose_file_name(item_uid, &title, &ContentRole::NoteText) {
                        let content_id = self.ids.take();
                        prose_refs.push(ProseRef {
                            file_id: content_id,
                            uid: common::uid::new_uid(),
                            created_at: created.clone(),
                            updated_at: updated.clone(),
                            activated: row.activated,
                            role: ContentRole::NoteText,
                            path: prose_relpath(dir, &name),
                        });
                        prose_map.insert(content_id, extra.to_string());
                    }
                }
            }
        }
        self.totals.rows += 1;
        if !row.activated {
            self.totals.trashed += 1;
        }
        let item = BinderItemFile {
            file_id,
            uid: item_uid,
            created_at: created,
            updated_at: updated,
            title,
            sub_title: String::new(),
            role: row.role.clone(),
            sub_role: row.sub_role.clone(),
            label: String::new(),
            activated: row.activated,
            is_favorite: false,
            is_exportable: row.exportable,
            exclude_from_numbering: row.exclude_from_numbering,
            indent: i64::from(row.depth),
            word_count_goal: worn.goals.0,
            char_count_goal: worn.goals.1,
            dict_language: Vec::new(),
            aliases: Vec::new(),
            status_id: worn.status,
            cover_id: None,
            inline_contents,
            prose_refs,
            reference_ids: Vec::new(),
            point_of_view_ids: Vec::new(),
            book_ids: Vec::new(),
            tag_ids: worn.tags,
        };
        Ok((
            BundledItem {
                item,
                prose: prose_map,
                comments,
                footnotes,
            },
            row_links,
        ))
    }

    /// One content of a row, made from `sources`: its Djot, comments and footnotes, and the
    /// links its texts hold. `None` when it holds nothing.
    #[allow(clippy::type_complexity)]
    fn content(
        &mut self,
        binder: BinderKind,
        role: ContentRole,
        sources: &[Source],
        exportable: bool,
        title: Option<&str>,
    ) -> Result<Option<(ContentOut, Option<(Place, Vec<String>)>)>, Stop> {
        let manuscript = binder == BinderKind::Manuscript
            && matches!(role, ContentRole::SceneText | ContentRole::ParatextText);
        let mut pieces: Vec<Piece> = Vec::with_capacity(sources.len());
        self.printed.set(manuscript && exportable);
        for source in sources {
            match self.piece(source, manuscript, title) {
                Ok(Some(piece)) => pieces.push(piece),
                Ok(None) => {}
                Err(stop) => {
                    self.printed.set(false);
                    return Err(stop);
                }
            }
        }
        self.printed.set(false);
        if pieces.is_empty() {
            return Ok(None);
        }
        let place = pieces[0].at;
        let mut links: Vec<String> = Vec::new();
        let mut doc = SourceDocument::new("", "");
        for piece in pieces {
            for (kind, count) in &piece.placeholders {
                self.notices.push(if exportable {
                    ProseNotice::PlaceholdersInExportedText {
                        at: piece.at,
                        kind: kind.clone(),
                        count: *count,
                    }
                } else {
                    ProseNotice::PlaceholdersInOtherText {
                        at: piece.at,
                        kind: kind.clone(),
                        count: *count,
                    }
                });
            }
            links.extend(piece.links);
            append(&mut doc, piece.doc);
        }
        let parts: Vec<RowPart> = (0..doc.blocks.len()).map(RowPart::Prose).collect();
        let assembled = assemble_row(&doc, &parts);
        for diagnostic in &assembled.diagnostics {
            self.notices.push(prose::document_notice(place, diagnostic));
        }
        if assembled.djot.trim().is_empty()
            && assembled.comments.is_empty()
            && assembled.footnotes.is_empty()
        {
            return Ok(None);
        }
        for path in skrib_format::media::referenced_paths(&assembled.djot) {
            self.referenced.insert(path);
        }
        // Every picture the import writes has no description: RTF holds none, and a file's
        // name is not one (X1).
        let undescribed = assembled.djot.matches("![](").count();
        if undescribed > 0 {
            self.notices.push(ProseNotice::PicturesWithoutDescription {
                at: place,
                count: undescribed as u32,
            });
        }
        for note in &assembled.footnotes {
            for path in skrib_format::media::referenced_paths(&note.body) {
                self.referenced.insert(path);
            }
        }
        let comments = self.comments(&assembled.djot, assembled.comments);
        let footnotes = assembled
            .footnotes
            .into_iter()
            .map(|note| FootnoteFile {
                file_id: self.ids.take(),
                uid: common::uid::new_uid(),
                created_at: self.now.clone(),
                updated_at: self.now.clone(),
                label: note.label,
                body: note.body,
            })
            .collect();
        let links = (!links.is_empty()).then_some((place, links));
        Ok(Some((
            ContentOut {
                role,
                djot: assembled.djot,
                comments,
                footnotes,
                words: assembled.word_count,
            },
            links,
        )))
    }

    /// Comments as the bundle stores them. A range that covers whole paragraphs becomes a
    /// comment on those paragraphs, which re-derives its extent as the writer edits them
    /// (D5.9).
    fn comments(&mut self, djot: &str, planned: Vec<PlannedComment>) -> Vec<CommentFile> {
        if planned.is_empty() {
            return Vec::new();
        }
        // The text read once, and its length counted once, for every comment of the row.
        let reading = if planned.iter().any(|c| c.kind == CommentAnchorKind::Range) {
            skrib_format::djot_plain_text(djot)
                .ok()
                .map(|(text, starts)| {
                    let total = text.chars().count();
                    (text, starts, total)
                })
        } else {
            None
        };
        planned
            .into_iter()
            .map(|mut comment| {
                if comment.kind == CommentAnchorKind::Range
                    && !comment.orphaned
                    && let Some((text, starts, total)) = &reading
                    && let Some(anchor) = whole_paragraphs(text, starts, *total, &comment.anchor)
                {
                    comment.kind = CommentAnchorKind::Paragraph;
                    comment.anchor = anchor;
                }
                let anchor = comment.anchor;
                CommentFile {
                    file_id: self.ids.take(),
                    uid: common::uid::new_uid(),
                    created_at: self.now.clone(),
                    updated_at: self.now.clone(),
                    kind: comment.kind,
                    author_name: comment.author,
                    // Never derived from the name: the format says so.
                    author_initials: String::new(),
                    body: comment.body,
                    resolved: comment.resolved,
                    orphaned: comment.orphaned,
                    orphan_reason: comment.orphan_reason,
                    range_start: anchor.start as u64,
                    range_length: anchor.length as u64,
                    quote_prefix: anchor.prefix,
                    quote_exact: anchor.exact,
                    quote_exact_truncated: anchor.exact_truncated,
                    quote_suffix: anchor.suffix,
                    block_ordinal_hint: anchor.block_ordinal as u64,
                    replies: Vec::new(),
                }
            })
            .collect()
    }

    /// One source's text, converted.
    fn piece(
        &mut self,
        source: &Source,
        manuscript: bool,
        title: Option<&str>,
    ) -> Result<Option<Piece>, Stop> {
        let project = self.project;
        match *source {
            Source::Text { item } => self.text(item, None, manuscript, title),
            Source::TextPart { item, part } => self.text(item, Some(part), manuscript, title),
            Source::Notes { item } => {
                let Some(row) = project.binder.get(item as usize) else {
                    return Ok(None);
                };
                let at = Place::Notes { item };
                match (&row.payload.notes, &row.payload.salvaged.notes) {
                    (Some(member), _) => {
                        let list = self.style_list(row.payload.notes_styles.as_ref())?;
                        self.rtf(member, at, list.as_deref(), None, None, &[], false)
                            .map(|mut parts| parts.pop().flatten())
                    }
                    (None, Some(text)) => Ok(Some(plain_piece(text, at))),
                    (None, None) => Ok(None),
                }
            }
            Source::Synopsis { item } => {
                let Some(row) = project.binder.get(item as usize) else {
                    return Ok(None);
                };
                let at = Place::Synopsis { item };
                match (&row.payload.synopsis, &row.payload.salvaged.synopsis) {
                    (Some(member), _) => match self.member(member, MAX_MEMBER_BYTES)? {
                        Some(bytes) => Ok(Some(plain_piece(&String::from_utf8_lossy(&bytes), at))),
                        None => {
                            self.notices.push(ProseNotice::Unreadable { at });
                            Ok(None)
                        }
                    },
                    (None, Some(text)) => Ok(Some(plain_piece(text, at))),
                    (None, None) => Ok(None),
                }
            }
            Source::Media { item } => self.media(item),
            Source::CardImage { item } => {
                let Some(row) = project.binder.get(item as usize) else {
                    return Ok(None);
                };
                let Some(member) = &row.payload.card_image else {
                    return Ok(None);
                };
                let at = Place::CardImage { item };
                let name = file_name(member);
                Ok(self.picture_piece(member, &name, at))
            }
            Source::ProjectNote { note } => {
                let Some(file) = project
                    .project_notes
                    .get(note as usize)
                    .and_then(|n| n.file.clone())
                else {
                    return Ok(None);
                };
                let at = Place::ProjectNote { note };
                self.rtf(&file, at, None, None, None, &[], false)
                    .map(|mut parts| parts.pop().flatten())
            }
            Source::Recovered { text } => {
                let Some(recovered) = project.recovered.get(text as usize).cloned() else {
                    return Ok(None);
                };
                let at = Place::Recovered { text };
                if recovered.role == TextRole::Synopsis {
                    return match self.member(&recovered.member, MAX_MEMBER_BYTES)? {
                        Some(bytes) => Ok(Some(plain_piece(&String::from_utf8_lossy(&bytes), at))),
                        None => {
                            self.notices.push(ProseNotice::Unreadable { at });
                            Ok(None)
                        }
                    };
                }
                self.rtf(&recovered.member, at, None, None, None, &[], false)
                    .map(|mut parts| parts.pop().flatten())
            }
        }
    }

    /// An item's main text, whole or one part of it split at its chapter headings.
    fn text(
        &mut self,
        item: u32,
        part: Option<u32>,
        manuscript: bool,
        title: Option<&str>,
    ) -> Result<Option<Piece>, Stop> {
        if let Some(part) = part
            && self.split.contains_key(&item)
        {
            return Ok(self.take_part(item, part));
        }
        let project = self.project;
        let Some(row) = project.binder.get(item as usize) else {
            return Ok(None);
        };
        let at = Place::Text { item };
        let (member, salvaged) = (&row.payload.text, &row.payload.salvaged.text);
        let Some(member) = member else {
            // A missing file whose words Scrivener's search index kept: that plain text,
            // which the analysis already reported (UQ-12).
            return Ok(match (part, salvaged) {
                (None | Some(0), Some(text)) => Some(plain_piece(text, at)),
                _ => None,
            });
        };
        let list = self.style_list(row.payload.text_styles.as_ref())?;
        let sidecar = self.sidecar(
            row.payload.comments.as_ref(),
            row.payload.links.as_ref(),
            at,
        )?;
        let splits: Vec<u32> = match part {
            Some(_) => self
                .read
                .content
                .items
                .get(item as usize)
                .map(|facts| facts.text.headings.iter().map(|h| h.block).collect())
                .unwrap_or_default(),
            None => Vec::new(),
        };
        let mut parts = self.rtf(
            member,
            at,
            list.as_deref(),
            sidecar.as_ref(),
            title,
            &splits,
            manuscript,
        )?;
        match part {
            None => Ok(parts.pop().flatten()),
            Some(part) => {
                self.split.insert(item, parts);
                Ok(self.take_part(item, part))
            }
        }
    }

    /// Part `part` of a text split at its chapter headings. The plan gives no row to what
    /// comes before the first heading when that holds no writing, yet the import may have
    /// put something there all the same (a comment the text no longer anchors), and the
    /// whole text's links and placeholders travel with that first part: it goes with the
    /// first chapter rather than nowhere.
    fn take_part(&mut self, item: u32, part: u32) -> Option<Piece> {
        let lead = self
            .read
            .content
            .items
            .get(item as usize)
            .is_some_and(|facts| facts.text.lead.counts());
        let parts = self.split.get_mut(&item)?;
        let wanted = parts.get_mut(part as usize).and_then(Option::take);
        let opening = (part == 1 && !lead)
            .then(|| parts.get_mut(0).and_then(Option::take))
            .flatten();
        match (opening, wanted) {
            (Some(mut opening), Some(piece)) => {
                append(&mut opening.doc, piece.doc);
                opening.links.extend(piece.links);
                for (kind, count) in piece.placeholders {
                    *opening.placeholders.entry(kind).or_default() += count;
                }
                Some(opening)
            }
            (opening, wanted) => wanted.or(opening),
        }
    }

    /// A text's style list (`content.styles`), when it has one.
    fn style_list(&mut self, member: Option<&MemberRef>) -> Result<Option<Vec<String>>, Stop> {
        let Some(member) = member else {
            return Ok(None);
        };
        Ok(self
            .member(member, MAX_MEMBER_BYTES)?
            .map(|bytes| styles::read_list(&bytes)))
    }

    /// A text's comments and linked footnotes, from `content.comments` or `<ID>.comments`,
    /// else a format 1.0 `.links` file.
    fn sidecar(
        &mut self,
        comments: Option<&MemberRef>,
        links: Option<&MemberRef>,
        at: Place,
    ) -> Result<Option<sidecar::Sidecar>, Stop> {
        let mut out: Option<sidecar::Sidecar> = None;
        // Format 1.5 keeps its comments in `.comments` and its links to other documents in
        // `.links`; format 1.0 keeps both in `.links`.
        for (member, read) in [
            (comments, sidecar::read_comments as fn(&[u8]) -> _),
            (links, sidecar::read_links),
        ] {
            let Some(member) = member else {
                continue;
            };
            let read = match self.member(member, MAX_MEMBER_BYTES)? {
                Some(bytes) => read(&bytes),
                None => {
                    self.notices.push(ProseNotice::CommentsUnreadable { at });
                    continue;
                }
            };
            match (read, &mut out) {
                (Ok(found), None) => out = Some(found),
                (Ok(found), Some(so_far)) => {
                    // A comment the two files both hold is the `.comments` one.
                    let mut held: HashSet<String> =
                        so_far.ids().into_keys().map(str::to_string).collect();
                    for entry in found.entries {
                        let known = entry
                            .id
                            .as_deref()
                            .is_some_and(|id| !held.insert(id.trim().to_ascii_uppercase()));
                        if !known {
                            so_far.entries.push(entry);
                        }
                    }
                    so_far.links.extend(found.links);
                }
                (Err(_), _) => self.notices.push(ProseNotice::CommentsUnreadable { at }),
            }
        }
        Ok(out)
    }

    /// An RTF text converted into its parts (one when `splits` is empty).
    #[allow(clippy::too_many_arguments)]
    fn rtf(
        &mut self,
        member: &MemberRef,
        at: Place,
        list: Option<&[String]>,
        sidecar: Option<&sidecar::Sidecar>,
        title: Option<&str>,
        splits: &[u32],
        manuscript: bool,
    ) -> Result<Vec<Option<Piece>>, Stop> {
        let Some(bytes) = self.member(member, MAX_MEMBER_BYTES)? else {
            self.notices.push(ProseNotice::Unreadable { at });
            return Ok(Vec::new());
        };
        let stream = prose::Stream {
            at,
            key: &member.path,
            styles: styles::StreamStyles {
                list,
                table: self.styles.as_ref(),
            },
            sidecar,
            title,
            footnote_marker: self.footnote_marker.as_deref(),
            scene_breaks: dialect::SceneBreaks {
                manuscript,
                blank_lines: self.request.prose.blank_lines_as_scene_breaks,
                ornaments: self.request.prose.ornaments_as_scene_breaks,
            },
            splits,
            compiled: self.printed.get(),
            struck_compiled_away: self.project.compile.delete_struck_through,
            older_format_export: older_format_export(&self.project.source),
        };
        let label = &self.label;
        let mint = || {
            label.set(label.get().saturating_add(1));
            format!("fn{}", label.get())
        };
        let converted = prose::rtf_stream(&bytes, &stream, &self.pictures, &mint);
        if self.pictures.moved.get() {
            return Err(Stop::Moved);
        }
        self.notices.extend(converted.notices);
        let mut links = Some(converted.links);
        let mut placeholders = Some(converted.placeholders);
        Ok(converted
            .parts
            .into_iter()
            .map(|doc| {
                Some(Piece {
                    doc,
                    at,
                    // The links and placeholders go with the first part asked for.
                    links: links.take().unwrap_or_default(),
                    placeholders: placeholders.take().unwrap_or_default(),
                })
            })
            .collect())
    }

    /// A picture, PDF or other file an item is (X-31): a picture as an asset shown at its
    /// own size; a text file's words; anything else one line naming the file where it is in
    /// the Scrivener project, which keeps it.
    fn media(&mut self, item: u32) -> Result<Option<Piece>, Stop> {
        let project = self.project;
        let Some(row) = project.binder.get(item as usize) else {
            return Ok(None);
        };
        let Some(member) = &row.payload.file else {
            return Ok(None);
        };
        let at = Place::Media { item };
        // The extension of the file's own name: a path whose file has none has none, never
        // the folders before it.
        let file = file_name(member);
        let extension = file
            .rsplit_once('.')
            .map(|(_, extension)| extension.to_ascii_lowercase())
            .unwrap_or_default();
        let name = match row.title.as_deref().map(str::trim) {
            Some(title) if !title.is_empty() && !extension.is_empty() => {
                format!("{title}.{extension}")
            }
            Some(title) if !title.is_empty() => title.to_string(),
            _ => file,
        };
        let pictures = ["png", "jpg", "jpeg", "gif", "tif", "tiff", "bmp", "webp"];
        let picture = row.kind == ItemKind::Image || pictures.contains(&extension.as_str());
        // A picture the editor cannot show (its format, its size) is a research file it
        // cannot open like any other, and goes where they go.
        if picture && let Some(piece) = self.picture_piece(member, &name, at) {
            return Ok(Some(piece));
        }
        if self.pictures.moved.get() {
            return Err(Stop::Moved);
        }
        if !picture
            && matches!(
                extension.as_str(),
                "txt" | "text" | "md" | "markdown" | "mmd" | "fountain"
            )
            && let Some(bytes) = self.member(member, MAX_MEMBER_BYTES)?
        {
            return Ok(Some(plain_piece(&String::from_utf8_lossy(&bytes), at)));
        } else if self.research.is_some()
            && let Some(bytes) = self.member(member, MAX_MEMBER_BYTES)?
            && let Some(research) = self.research.as_mut()
        {
            // Beside the project, its note naming where (UQ-37).
            let relative = research.copy(&name, &bytes).map_err(ConvertError::Output)?;
            self.notices.push(ProseNotice::MediaFileCopied {
                at,
                path: relative.clone(),
            });
            return Ok(Some(plain_piece(&relative, at)));
        }
        self.notices.push(ProseNotice::MediaFileNotCarried {
            at,
            name: name.clone(),
        });
        let line = format!("{}/{}", project.source.folder_name, member.path);
        Ok(Some(plain_piece(&line, at)))
    }

    /// A content holding one picture from a member of the project, at its own size. `None`
    /// when it stays out, said why.
    fn picture_piece(&mut self, member: &MemberRef, name: &str, at: Place) -> Option<Piece> {
        match self.pictures.member_picture(member, name) {
            Ok(sourced) => {
                if sourced.first_frame_only {
                    self.notices.push(prose::document_notice(
                        at,
                        &ImportDiagnostic::PictureFirstFrameOnly {
                            path: String::new(),
                            target: name.to_string(),
                        },
                    ));
                }
                let rich = RichDocument {
                    blocks: vec![RichBlock::body(vec![Run::picture(
                        "",
                        sourced.placed.src,
                        name,
                        sourced.placed.width,
                        sourced.placed.height,
                    )])],
                    ..RichDocument::default()
                };
                let mut doc = SourceDocument::new("", "");
                if rich::assemble(&rich, &mut doc).is_err() {
                    return None;
                }
                Some(Piece {
                    doc,
                    at,
                    links: Vec::new(),
                    placeholders: BTreeMap::new(),
                })
            }
            Err(reason) => {
                self.notices.push(prose::document_notice(
                    at,
                    &ImportDiagnostic::ImageNotIngested {
                        path: String::new(),
                        target: name.to_string(),
                        reason,
                    },
                ));
                None
            }
        }
    }

    /// Links to other documents, made references where Skribisto shows one (X-35): a note to
    /// a note (its *Links*), or prose to a story-bible entry (its *Cast*). Every other link
    /// keeps its words and is reported.
    fn resolve_links(
        &mut self,
        binders: &mut [BundledBinder],
        row_ids: &[Vec<u64>; 2],
        links: Vec<(BinderKind, usize, Place, Vec<String>)>,
    ) {
        let project = self.project;
        let mut by_key: HashMap<String, usize> = HashMap::new();
        for (index, row) in project.binder.iter().enumerate() {
            if !row.owns_key {
                continue;
            }
            if let Some(ItemKey::Uuid(uuid)) = row.key {
                by_key.insert(uuid.hyphenated().to_string().to_ascii_uppercase(), index);
            }
            if let Some(number) = row.number {
                by_key.entry(number.to_string()).or_insert(index);
            }
        }
        let rows_of = |kind: BinderKind| match kind {
            BinderKind::Manuscript => &self.plan.manuscript,
            BinderKind::Notes => &self.plan.notes,
        };
        let binder_index = |kind: BinderKind| match kind {
            BinderKind::Manuscript => 0usize,
            BinderKind::Notes => 1,
        };
        let mut notices = Vec::new();
        for (kind, r, place, targets) in links {
            let Some(owner) = rows_of(kind).get(r) else {
                continue;
            };
            let owner_id = row_ids[binder_index(kind)].get(r).copied();
            let mut kept: Vec<u64> = Vec::new();
            let (mut dropped, mut unresolved) = (0u32, 0u32);
            for target in targets {
                let target_row = by_key
                    .get(target.trim().to_ascii_uppercase().as_str())
                    .and_then(|item| self.plan.placements.get(*item))
                    .and_then(|placement| placement.item);
                let Some(target_ref) = target_row else {
                    unresolved += 1;
                    continue;
                };
                let Some(target) = rows_of(target_ref.binder).get(target_ref.row as usize) else {
                    unresolved += 1;
                    continue;
                };
                let Some(target_id) = row_ids[binder_index(target_ref.binder)]
                    .get(target_ref.row as usize)
                    .copied()
                else {
                    unresolved += 1;
                    continue;
                };
                if Some(target_id) == owner_id {
                    continue;
                }
                let note = |row: &PlannedRow| row.sub_role == SubRole::Note;
                let shown = (note(owner) && note(target))
                    || (skribisto_model::counts_prose(&owner.role, &owner.sub_role)
                        && target.tag.is_some());
                if shown {
                    if !kept.contains(&target_id) {
                        kept.push(target_id);
                    }
                } else {
                    dropped += 1;
                }
            }
            if !kept.is_empty() {
                notices.push(ProseNotice::DocumentLinksKept {
                    at: place,
                    count: kept.len() as u32,
                });
                if let Some(item) = binders
                    .iter_mut()
                    .flat_map(|b| b.items.iter_mut())
                    .find(|i| Some(i.item.file_id) == owner_id)
                {
                    item.item.reference_ids.extend(kept);
                }
            }
            if dropped > 0 {
                notices.push(ProseNotice::DocumentLinksDropped {
                    at: place,
                    count: dropped,
                });
            }
            if unresolved > 0 {
                notices.push(ProseNotice::DocumentLinkUnresolved {
                    at: place,
                    count: unresolved,
                });
            }
        }
        self.notices.extend(notices);
    }
}

/// The pictures of a project, each once, from its texts and its items.
struct ProjectPictures<'t, 'i, 'd, 'c> {
    tree: &'t Tree<'i, 'd>,
    project: &'c Project,
    store: RefCell<DocumentPictures>,
    /// A member moved while a picture was read: the whole project is read again.
    moved: Cell<bool>,
    /// Items by UUID, for the pictures a text links to.
    by_uuid: HashMap<String, usize>,
}

impl ProjectPictures<'_, '_, '_, '_> {
    /// The picture in a member of the project.
    fn member_picture(
        &self,
        member: &MemberRef,
        name: &str,
    ) -> Result<SourcedPicture, PictureRefusal> {
        let tree = self.tree;
        let moved = &self.moved;
        let read = || match tree.read(&member.path, document_ingest::pictures::MAX_PICTURE_BYTES) {
            Ok(bytes) => Ok(bytes),
            Err(ReadError::Changed) => {
                moved.set(true);
                Err(PictureRefusal::Unreadable)
            }
            Err(ReadError::TooLarge) => Err(PictureRefusal::TooLarge),
            Err(ReadError::Missing) => Err(PictureRefusal::Missing),
            Err(_) => Err(PictureRefusal::Unreadable),
        };
        let brought = self.store.borrow_mut().bring(&member.path, name, read);
        brought.result.map(|placed| SourcedPicture {
            placed,
            first_frame_only: brought.first_frame_only,
        })
    }
}

impl Pictures for ProjectPictures<'_, '_, '_, '_> {
    fn embedded(
        &self,
        key: &str,
        name: &str,
        bytes: &[u8],
    ) -> Result<SourcedPicture, PictureRefusal> {
        let brought = self
            .store
            .borrow_mut()
            .bring(key, name, || Ok(bytes.to_vec()));
        brought.result.map(|placed| SourcedPicture {
            placed,
            first_frame_only: brought.first_frame_only,
        })
    }

    fn linked(&self, uuid: &str) -> Result<SourcedPicture, PictureRefusal> {
        let row = self
            .by_uuid
            .get(uuid)
            .and_then(|index| self.project.binder.get(*index))
            .ok_or(PictureRefusal::Missing)?;
        let member = row.payload.file.as_ref().ok_or(PictureRefusal::Missing)?;
        let name = row
            .title
            .clone()
            .filter(|t| !t.trim().is_empty())
            .unwrap_or_else(|| file_name(member));
        self.member_picture(member, &name)
    }
}

/// `pattern` with `value` in its slot, or before it when it has none.
pub(crate) fn fill(pattern: &str, value: &str) -> String {
    if pattern.contains(TITLE_SLOT) {
        pattern.replace(TITLE_SLOT, value)
    } else if pattern.trim().is_empty() {
        value.to_string()
    } else {
        format!("{value} {pattern}")
    }
}

/// The title of the Scrivener item a row carries, for a heading that repeats it.
fn row_item_title(project: &Project, origin: &Origin) -> Option<String> {
    let item = match origin {
        Origin::Item { item } | Origin::OwnText { item } | Origin::SplitPart { item, .. } => item,
        _ => return None,
    };
    project
        .binder
        .get(*item as usize)
        .and_then(|row| row.title.clone())
}

/// A member's file name, never its folders.
fn file_name(member: &MemberRef) -> String {
    member
        .path
        .rsplit('/')
        .next()
        .unwrap_or(&member.path)
        .to_string()
}

fn plain_piece(text: &str, at: Place) -> Piece {
    Piece {
        doc: prose::plain_document(text),
        at,
        links: Vec::new(),
        placeholders: BTreeMap::new(),
    }
}

/// Append one text's document to a content's: its blocks after the others, its comments
/// rebased onto them, its notes beside the others'.
///
/// A comment its own text had no paragraph for (a text holding no writing at all) goes on
/// the nearest paragraph of the content, the one before it else the one after, as a
/// comment on that paragraph: rebased as it was, it would point past every block and the
/// row would drop it without a word.
fn append(into: &mut SourceDocument, doc: SourceDocument) {
    let base = into.blocks.len();
    let own = doc.blocks.len();
    if own > 0 {
        for annotation in &mut into.annotations {
            if annotation.block_index >= base {
                annotation.block_index = base;
                annotation.unanchored = true;
            }
        }
    }
    into.blocks.extend(doc.blocks);
    into.annotations
        .extend(doc.annotations.into_iter().map(|mut annotation| {
            if annotation.block_index < own {
                annotation.block_index += base;
            } else {
                annotation.block_index = match (own, base) {
                    (0, 0) => usize::MAX,
                    (0, base) => base - 1,
                    (own, base) => base + own - 1,
                };
                annotation.unanchored = true;
            }
            annotation
        }));
    into.footnotes.extend(doc.footnotes);
    into.diagnostics.extend(doc.diagnostics);
}

/// `djot` without its references to the footnotes `labels` name: `[^label]` where the
/// writer's text did not write it (a bracket behind an odd run of backslashes is text).
fn without_references(djot: &str, labels: &[&str]) -> String {
    let mut out = djot.to_string();
    for label in labels {
        let reference = format!("[^{label}]");
        let mut kept = String::with_capacity(out.len());
        let mut rest = out.as_str();
        while let Some(at) = rest.find(&reference) {
            let backslashes = rest[..at].chars().rev().take_while(|c| *c == '\\').count();
            let escaped = backslashes % 2 == 1;
            kept.push_str(&rest[..at]);
            if escaped {
                kept.push_str(&reference);
            }
            rest = &rest[at + reference.len()..];
        }
        kept.push_str(rest);
        out = kept;
    }
    out
}

/// A range comment's anchor as a comment on the whole paragraphs it covers, when it covers
/// whole paragraphs exactly.
fn whole_paragraphs(
    text: &str,
    starts: &[usize],
    total: usize,
    anchor: &comment_anchor::Anchor,
) -> Option<comment_anchor::Anchor> {
    let end = anchor.start + anchor.length;
    let first = starts.binary_search(&anchor.start).ok()?;
    let last = comment_anchor::block_of(starts, end.saturating_sub(1).max(anchor.start));
    let (s, e) = comment_anchor::block_extent(starts, total, first, last);
    if s != anchor.start || e != end || e <= s {
        return None;
    }
    let mut paragraph = comment_anchor::capture(text, s, e, first);
    paragraph.block_span = last - first + 1;
    Some(paragraph)
}

/// Whether a project is in Scrivener's older format and was saved by Scrivener 3 for Mac
/// or Windows: what Scrivener 3's export to that format writes (`gt-writing-v2`, format 1.5
/// from `SCRMAC-3.5.2`), and nothing else does, since Scrivener 3 opens an older project
/// only to upgrade it.
fn older_format_export(source: &SourceFacts) -> bool {
    let older = matches!(
        source.generation,
        Generation::Format10 | Generation::Format15
    );
    let desktop = matches!(source.writer.platform, Platform::Mac | Platform::Windows);
    let scrivener_3 = source
        .writer
        .version
        .as_deref()
        .and_then(|v| v.split('.').next())
        == Some("3");
    older && desktop && scrivener_3
}

/// The glyph a project anchors new footnotes to, when it does
/// (`<FootnoteMarker UseMarker="Yes">*</FootnoteMarker>`).
fn footnote_marker(bytes: &[u8]) -> Option<String> {
    let text = crate::xml::text_of(bytes).ok()?;
    let doc = crate::xml::parse("projectpreferences.xml", text).ok()?;
    let marker = doc
        .descendants()
        .find(|n| n.is_element() && n.tag_name().name() == "FootnoteMarker")?;
    if !marker
        .attribute("UseMarker")
        .is_some_and(|v| v.eq_ignore_ascii_case("yes"))
    {
        return None;
    }
    crate::xml::text(marker)
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
}

/// Add each of `edges` (owner, target, by file id) to its owner's references, once and
/// beside the ones its prose already made, in time linear in the rows and the edges: the
/// rows found by id once, each owner's references gathered once. Checks `cancel` as it
/// goes, as the rows before it did.
fn attach_references<'r>(
    rows: impl Iterator<Item = (u64, &'r mut Vec<u64>)>,
    edges: &[(u64, u64)],
    cancel: &AtomicBool,
) -> Result<(), ConvertError> {
    if edges.is_empty() {
        return Ok(());
    }
    let mut references: HashMap<u64, &'r mut Vec<u64>> = rows.collect();
    let mut held: HashMap<u64, HashSet<u64>> = HashMap::new();
    for (n, (owner, target)) in edges.iter().enumerate() {
        if n % 1024 == 0 && cancel.load(Ordering::Relaxed) {
            return Err(ConvertError::Cancelled);
        }
        let Some(list) = references.get_mut(owner) else {
            continue;
        };
        let seen = held
            .entry(*owner)
            .or_insert_with(|| list.iter().copied().collect());
        if seen.insert(*target) {
            list.push(*target);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Many bookmarks over many rows attach in time linear in both, each once, beside the
    /// references the prose made.
    #[test]
    fn references_attach_in_linear_time_each_once() {
        let n = 60_000u64;
        let mut rows: Vec<(u64, Vec<u64>)> = (0..n).map(|id| (id, Vec::new())).collect();
        rows[5].1.push(6);
        let mut edges: Vec<(u64, u64)> = Vec::new();
        for owner in 0..n {
            for k in 1..=10 {
                edges.push((owner, (owner + k) % n));
            }
        }
        edges.push((5, 6));
        edges.push((5, 7));
        let started = std::time::Instant::now();
        let attached = attach_references(
            rows.iter_mut().map(|(id, refs)| (*id, refs)),
            &edges,
            &AtomicBool::new(false),
        );
        let seconds = started.elapsed().as_secs_f64();
        assert!(attached.is_ok());
        assert!(seconds < 10.0, "{seconds:.1} s for {} edges", edges.len());
        assert_eq!(rows[5].1, vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        assert_eq!(rows[0].1.len(), 10);
        let cancelled = attach_references(
            rows.iter_mut().map(|(id, refs)| (*id, refs)),
            &edges,
            &AtomicBool::new(true),
        );
        assert!(matches!(cancelled, Err(ConvertError::Cancelled)));
    }

    #[test]
    fn a_pattern_puts_the_title_in_its_slot_or_before_it() {
        assert_eq!(fill("\u{E000} (notes)", "Arrival"), "Arrival (notes)");
        assert_eq!(fill("Notes de \u{E000}", "Arrivée"), "Notes de Arrivée");
        assert_eq!(fill("(notes)", "Arrival"), "Arrival (notes)");
        assert_eq!(fill("", "Arrival"), "Arrival");
    }

    #[test]
    fn a_range_over_whole_paragraphs_becomes_a_paragraph_comment() {
        let text = "One two.\nThree four.\nFive.";
        let starts = [0usize, 9, 21];
        let total = text.chars().count();
        let whole = comment_anchor::capture(text, 9, 26, 1);
        let paragraph = whole_paragraphs(text, &starts, total, &whole).expect("whole paragraphs");
        assert_eq!(paragraph.block_span, 2);
        assert_eq!((paragraph.start, paragraph.length), (9, 17));
        let words = comment_anchor::capture(text, 9, 14, 1);
        assert!(whole_paragraphs(text, &starts, total, &words).is_none());
    }

    /// A comment its own text had no paragraph for lands on the nearest paragraph of the
    /// content: the next one when it came first, the one before otherwise. Rebased as it
    /// was, it pointed past every block and the row dropped it.
    #[test]
    fn a_comment_with_no_paragraph_of_its_own_goes_to_the_nearest() {
        let floating = || document_ingest::SourceAnnotation {
            block_index: usize::MAX,
            kind: document_ingest::AnnotationKind::Paragraph,
            anchor: comment_anchor::Anchor::default(),
            unanchored: true,
            uid: None,
            uid_tag: None,
            author: String::new(),
            author_initials: String::new(),
            created: None,
            body: "A remark.".to_string(),
            resolved: false,
            replies: Vec::new(),
        };
        let blank = || {
            let mut doc = SourceDocument::new("", "");
            doc.annotations.push(floating());
            doc
        };
        let words = |text: &str| {
            let mut doc = SourceDocument::new("", "");
            doc.blocks.push(document_ingest::SourceBlock::prose(
                text.to_string(),
                text.to_string(),
            ));
            doc
        };
        let mut first = SourceDocument::new("", "");
        append(&mut first, blank());
        append(&mut first, words("One."));
        append(&mut first, words("Two."));
        assert_eq!(first.annotations[0].block_index, 0);

        let mut last = SourceDocument::new("", "");
        append(&mut last, words("One."));
        append(&mut last, words("Two."));
        append(&mut last, blank());
        assert_eq!(last.annotations[0].block_index, 1);
        assert!(last.annotations[0].unanchored);
    }

    #[test]
    fn a_reference_goes_and_text_that_only_looks_like_one_stays() {
        assert_eq!(
            without_references(r"Goal[^fn1]: \[^fn1\] and[^fn12] \\[^fn1].", &["fn1"]),
            r"Goal: \[^fn1\] and[^fn12] \\."
        );
    }

    #[test]
    fn the_footnote_marker_is_read_only_when_in_use() {
        let on =
            br#"<Preferences><FootnoteMarker UseMarker="Yes">*</FootnoteMarker></Preferences>"#;
        assert_eq!(footnote_marker(on).as_deref(), Some("*"));
        let off =
            br#"<Preferences><FootnoteMarker UseMarker="No">*</FootnoteMarker></Preferences>"#;
        assert_eq!(footnote_marker(off), None);
    }
}
