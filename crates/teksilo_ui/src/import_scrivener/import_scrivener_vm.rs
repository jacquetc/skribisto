// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ImportScrivenerViewModel`: the Import a Scrivener project wizard's business logic.
//!
//! **Tier 3, one per window** (D7-11): a project window's `App` and the Launcher's
//! `WelcomePanel` each build one, and each wires its inspection to its own widget tree, so
//! two windows can each hold a wizard. What it starts, the import itself, belongs to the
//! process's one `shared::project_import::ProjectImportJob` (Tier 1): one import at a time,
//! the progress notice, Cancel, the result with **Open now**, a failure worded in the
//! writer's language. The job starts the conversion and reads its result through
//! `models::scrivener_import_run`, the import's data seam, so the mocks build walks the same
//! path to the report on a fabricated run.
//!
//! Three steps, as the panel shows them:
//!
//! * **Project**: the path, typed, picked or dropped; the inspection
//!   (`models::scrivener_inspection`, the data seam) starts as soon as one is set. Its
//!   answer is a refusal, a question to choose, or a project that reads, with notices to
//!   know and questions to accept. A choice with a right answer is answered at once and
//!   the project read again with it; the radio stays, so the writer can change it.
//! * **Review**: the structure the converter's own mapping plans, with the choices it
//!   offers. Every change recomputes the plan synchronously, through the seam, from the
//!   analysis's own review data: the preview and the import cannot disagree (D7-5).
//! * **Save**: where the new project goes and what it is called, its title, author and
//!   language as the project states them (else the interface's language), and whether the
//!   research files Skribisto cannot open are copied beside it.
//!
//! Every word the import writes into the project is resolved here, in the interface
//! language, once ([`ImportScrivenerViewModel::request`]).
//!
//! **Never log or `{:?}` an analysis**: its `review_data` can hold a sync copy's file name.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use teksilo::core::signal::ObserverHandle;
use teksilo::data::ListModel;
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::widgets::{MessageBox, MessageBoxButtons, StepperController, ValidationState};

use frontend::AppContext;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use frontend::import_management::scrivener::ScrivenerReviewError;
use frontend::import_management::scrivener::{METADATA_WORD_ORDER, VALUE_SLOT};
use frontend::import_management::{
    AnalyzeScrivenerProjectDto, ImportScrivenerProjectDto, ImportScrivenerProjectResultDto,
    ScrivenerAnalysisDto, ScrivenerAnalysisOutcome, ScrivenerBinder, ScrivenerBooksChoice,
    ScrivenerChoice, ScrivenerComputers, ScrivenerContainer, ScrivenerContainerRung,
    ScrivenerControl, ScrivenerDefaultRung, ScrivenerDestination, ScrivenerFinding,
    ScrivenerGeneration, ScrivenerItemRef, ScrivenerManifestCandidate, ScrivenerPlatform,
    ScrivenerPreviewRow, ScrivenerQuestion, ScrivenerSiblingChoice, ScrivenerStructure,
    ScrivenerTextKind, ScrivenerTitleKind, ScrivenerTotals, ScrivenerTrashChoice,
};
use skrib_format::importing::TITLE_SLOT;
use skribisto_model::CreateType;

use super::messages::{self, Tier};
use super::preview_source::{PreviewIdentity, PreviewKey, PreviewRowView, PreviewSource};
use super::report::ImportReport;
use super::report_panel::present_import_report;
use super::report_vm::{ImportReportViewModel, ItemFinder};
use crate::binder::create_labels::recommendation_label;
use crate::intents::AppIntent;
use crate::models::scrivener_import_run::ScrivenerImportRun;
use crate::models::scrivener_inspection::{InspectState, ScrivenerInspection};
use crate::shared::form_checks::{CachedValidation, DiskChecked, FolderMessages, folder_state};
use crate::shared::import_destination::{DestinationMessages, ImportDestination};
use crate::shared::import_warnings::SCRIVENER as NOTICE;
use crate::shared::project_import::{
    ImportOutcome, Importer, ProjectImportJob, ReviewAgain, SeeReport,
};

/// The wizard's three steps: Project, Review, Save.
const STEPS: usize = 3;

/// What the Project page shows below the path, by [`ImportScrivenerViewModel::inspection_page`].
pub(crate) const PAGE_EMPTY: usize = 0;
pub(crate) const PAGE_READING: usize = 1;
pub(crate) const PAGE_CARD: usize = 2;
pub(crate) const PAGE_REFUSED: usize = 3;
pub(crate) const PAGE_FAILED: usize = 4;

/// No answer chosen yet, in a radio group's selection.
pub(crate) const NO_ANSWER: usize = usize::MAX;

/// The keys of the two questions to accept, as the request carries their answers.
pub(crate) const UNMERGED_MOBILE_EDITS: &str = "scrivener-question-unmerged-mobile-edits";
pub(crate) const NO_TEXT_IN_COPY: &str = "scrivener-question-no-text-in-copy";

/// The help topic the wizard's **Help** opens: "Coming from Scrivener".
pub(crate) const HELP_TOPIC: &str = "help-import-scrivener";

/// The registered action that reopens a window's last Scrivener import report: the
/// command palette's entry.
pub(crate) const REPORT_ACTION: &str = "work.import_scrivener_report";

/// What the name of an archived **See report** starts with, the path its import wrote
/// following it. Every Scrivener import's notice shares one id, so the notification log
/// keeps one row, which shows the latest import: the name says which one, and the row
/// opens that import's report, not the one its window showed last. Stable: it is persisted
/// in the notification archive.
const ARCHIVED_REPORT_PREFIX: &str = "work.import_scrivener_report_of:";

/// The name an archived **See report** of the import that wrote `output_path` keeps.
pub(crate) fn archived_report_name(output_path: &str) -> String {
    format!("{ARCHIVED_REPORT_PREFIX}{output_path}")
}

/// The path of the import an archived action's `name` opens the report of, if it is an
/// archived **See report**.
pub(crate) fn archived_report_path(name: &str) -> Option<&str> {
    name.strip_prefix(ARCHIVED_REPORT_PREFIX)
        .filter(|path| !path.is_empty())
}

/// The Scrivener importer, as the shared job drives it.
pub(crate) static SCRIVENER: Importer<ImportScrivenerProjectDto> = Importer {
    toast_id: "import.scrivener",
    notice: &NOTICE,
    progress_title: || tr!(import_scrivener_progress_title()),
    own_words: messages::own_failure,
    target: |request| request.output_path.clone(),
    allow_overwrite: |request| ImportScrivenerProjectDto {
        overwrite: true,
        ..request
    },
    launch: ScrivenerImportRun::start,
    outcome,
    source: |request| request.source_path.clone(),
    review_again: Some(ReviewAgain {
        applies: messages::reviewing_again_helps,
        label: || tr!(import_scrivener_review_again()),
        intent: |path| AppIntent::ImportScrivenerPath { path },
    }),
    // A project brought across from another program is a new project beside the one
    // being written, never in its place.
    open_now: |path| AppIntent::OpenWorkPathInNewWindow { path },
};

// ── the import's result, for the report ──────────────────────────────────────────────────

/// What the writer chose in the wizard, kept for the import's report: the report is the
/// writer's record of the import, and an accepted notice or a folder left out is part of
/// what happened.
#[derive(Debug, Clone)]
pub struct ImportRecord {
    /// The project the import read: the one the analysis found, not the path picked (a
    /// folder around it, a file inside it, the copy beside it the writer chose).
    pub source: String,
    /// The Project page's answers, as the report states them: which project, binder file
    /// or copy the import read.
    pub project_answers: Vec<LocalizedString>,
    /// The keys of the questions the writer accepted.
    pub accepted: Vec<String>,
    /// The structure choices, each a change from what was detected (the Trash's always).
    pub choices: Vec<ScrivenerChoice>,
    /// The same choices, and the prose ones, as the Review page worded them when Import
    /// was pressed: what the choice was about, and the answer.
    pub worded: Vec<ChoiceLine>,
    pub blank_lines_as_scene_breaks: bool,
    pub ornaments_as_scene_breaks: bool,
    pub copy_research_files: bool,
    /// Whether the project held research files Skribisto cannot open, so the copy
    /// choice was offered at all.
    pub research_files: bool,
    /// The new project's title, author and language (as the language field names it),
    /// as the Save page sent them.
    pub title: String,
    pub author: String,
    pub language: String,
    /// Whether the writer replaced what the Save page proposed for each: a hint about
    /// the proposal then says so in the report.
    pub title_changed: bool,
    pub author_changed: bool,
    pub language_changed: bool,
}

/// One choice of the Review page, as the report states it.
#[derive(Debug, Clone)]
pub struct ChoiceLine {
    /// What the choice was about: a level, a folder, the Trash.
    pub subject: LocalizedString,
    /// The answer the import used.
    pub answer: LocalizedString,
}

/// A finished Scrivener import, typed: what the report dialog reads.
#[derive(Clone)]
pub struct ScrivenerImportResult {
    pub output_path: String,
    /// The folder the research files were copied into, or empty.
    pub research_folder: String,
    pub totals: ScrivenerTotals,
    /// One row per kind and place, as the converter reports them.
    pub findings: Vec<ScrivenerFinding>,
    /// What the writer chose, when the import was started from this process's wizard.
    pub record: Option<ImportRecord>,
}

impl ScrivenerImportResult {
    fn new(result: ImportScrivenerProjectResultDto, record: Option<ImportRecord>) -> Self {
        Self {
            output_path: result.output_path,
            research_folder: result.research_folder,
            totals: result.totals,
            findings: result.findings,
            record,
        }
    }

    /// The rows the new project holds: what came across from Scrivener and the rows the
    /// import made for it (a document's notes, the project notes), which is why the result
    /// says what the project holds rather than what came across.
    pub fn items(&self) -> i64 {
        match &self.totals {
            ScrivenerTotals::Counted { rows, .. } => *rows,
            ScrivenerTotals::Empty => 0,
        }
    }

    /// How many lines of the report say something could not be read or needs checking
    /// (Q7): counted on the report itself, so the notice and the dialog agree. What came
    /// across differently or stayed behind raises no alarm.
    pub fn findings_to_check(&self) -> usize {
        ImportReport::from_result(self).lines_to_check()
    }
}

/// How many finished imports the session keeps for their reports: the newest, and enough
/// before them to reopen any report a window showed. A report is kept until Skribisto
/// closes, never in the project (D7 Q2).
const KEPT_REPORTS: usize = 16;

thread_local! {
    /// The imports this process finished, oldest first, for their reports. One list per
    /// process (Tier 1), like the job that ran them; the UI thread is the only one that
    /// reads or writes it.
    static FINISHED: RefCell<Vec<Rc<ScrivenerImportResult>>> = const { RefCell::new(Vec::new()) };
    /// What the writer chose for each import a wizard handed to the job, by the file it
    /// writes, until its result is read. By file, not one slot: a wizard's import can wait
    /// on its overwrite question while another window's runs and finishes, and each result
    /// must find its own record.
    static PENDING_RECORDS: RefCell<HashMap<String, ImportRecord>> = RefCell::new(HashMap::new());
    /// What each import a wizard handed to the job asked for, by the project it read: what
    /// **Review again** reads the project with, whichever window's wizard it opens.
    static SENT: RefCell<HashMap<String, Sent>> = RefCell::new(HashMap::new());
}

/// An import as a wizard handed it to the job: the request, and what the request does not
/// say of the Project and Save pages.
#[derive(Clone)]
struct Sent {
    request: ImportScrivenerProjectDto,
    /// The questions to choose met on the way, so their radios come back with the answers.
    asked: Vec<ScrivenerQuestion>,
    /// The Save page's folder and name, as the writer left them.
    location: String,
    name: String,
}

/// The last Scrivener import this process finished: its totals, every finding, and what
/// the writer chose. What the report dialog reads.
pub(crate) fn last_import() -> Option<Rc<ScrivenerImportResult>> {
    FINISHED.with(|finished| finished.borrow().last().cloned())
}

/// The latest finished import that wrote `output_path`, compared as the open-project
/// registry compares two projects, whatever the spelling.
pub(crate) fn import_of(output_path: &str) -> Option<Rc<ScrivenerImportResult>> {
    FINISHED.with(|finished| {
        finished
            .borrow()
            .iter()
            .rev()
            .find(|result| same_project(&result.output_path, output_path))
            .cloned()
    })
}

/// Keep `result` as the newest finished import, letting the oldest go past
/// [`KEPT_REPORTS`].
fn remember_finished(result: Rc<ScrivenerImportResult>) {
    FINISHED.with(|finished| {
        let mut finished = finished.borrow_mut();
        finished.push(result);
        let excess = finished.len().saturating_sub(KEPT_REPORTS);
        finished.drain(..excess);
    });
}

/// The finished import's result, in the writer's words: how many items the new project
/// holds and how many kinds of thing need a look. Keeps the whole result for the report.
fn outcome(app_ctx: &AppContext, op_id: &str) -> Option<ImportOutcome> {
    Some(record_outcome(ScrivenerImportRun::result(app_ctx, op_id)?))
}

/// Keep `result` as the last import, with the record of the wizard that started it, and
/// word its headline.
fn record_outcome(result: ImportScrivenerProjectResultDto) -> ImportOutcome {
    let record = PENDING_RECORDS.with(|pending| pending.borrow_mut().remove(&result.output_path));
    let imported = ScrivenerImportResult::new(result, record);
    let to_check = imported.findings_to_check();
    let items = imported.items();
    let output_path = imported.output_path.clone();
    // **See report** only when the report has a line to read beyond the counts (D7-8);
    // the command palette reopens it either way.
    let has_lines = !imported.findings.is_empty();
    remember_finished(Rc::new(imported));
    // A notice carrying **See report** stays until the writer closes it: in the Launcher
    // it is the only way back to the report, which a timer must not take away.
    ImportOutcome {
        output_path,
        result: if to_check == 0 {
            tr!(import_scrivener_done(items = items))
        } else {
            tr!(import_scrivener_done_check(
                items = items,
                findings = to_check as i64
            ))
        },
        report: Vec::new(),
        persistent: to_check > 0 || has_lines,
        headline: Some(tr!(import_scrivener_done_title())),
        see_report: has_lines.then_some(SeeReport {
            label: || tr!(import_scrivener_see_report()),
            intent: |output_path| AppIntent::ImportScrivenerReportOf { output_path },
            archived_as: archived_report_name,
        }),
    }
}

// ── what the pages show ──────────────────────────────────────────────────────────────────

/// The destination fields' words.
static DESTINATION: DestinationMessages = DestinationMessages {
    folder: FOLDER,
    name_required: || tr!(project_import_name_required()),
    name_exists: || tr!(project_import_name_exists()),
    name_is_folder: || tr!(project_import_name_is_folder()),
};

const FOLDER: FolderMessages = FolderMessages {
    required: || tr!(project_import_location_required()),
    missing: || tr!(project_import_location_missing()),
    not_folder: || tr!(project_import_location_not_folder()),
    readonly: || tr!(project_import_location_readonly()),
};

/// The project card: who last saved the project, when, and what it holds.
#[derive(Clone)]
pub struct CardView {
    /// The project's name, as the writer knows it.
    pub name: String,
    pub made_with: LocalizedString,
    pub saved_on: Option<LocalizedString>,
    pub counts: LocalizedString,
}

/// A notice on the Project page.
#[derive(Clone)]
pub struct NoticeView {
    pub title: LocalizedString,
    pub body: Option<LocalizedString>,
    /// A warning rather than an information.
    pub warning: bool,
}

/// A question on the Project page: chosen with a radio group, or accepted with a
/// checkbox.
#[derive(Clone)]
pub struct QuestionView {
    /// Its position among the page's questions, which the view hands back.
    pub index: usize,
    /// The question's key, which the view hands back to accept it.
    pub key: String,
    pub title: LocalizedString,
    pub body: LocalizedString,
    /// The answers, for a question to choose.
    pub options: Vec<LocalizedString>,
    /// The answer chosen, [`NO_ANSWER`] for none.
    pub chosen: Signal<usize>,
    /// The checkbox's label, for a question to accept.
    pub accept: Option<LocalizedString>,
    pub accepted: Signal<bool>,
}

/// Why the project cannot be imported, as the Project page says it.
#[derive(Clone)]
pub struct RefusalView {
    pub title: LocalizedString,
    pub body: Option<LocalizedString>,
    /// The cause may go away by itself: **Check again**.
    pub retry: bool,
    /// The technical account, for **Details**.
    pub details: Option<String>,
}

/// What a choice on the Review page changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickTarget {
    /// The folders at this depth of the manuscript.
    Containers(i64),
    /// The documents at this depth of the manuscript.
    Texts(i64),
    /// A folder beside the manuscript, by its Scrivener binder row.
    Root(i64),
    /// One book or several.
    Books,
}

/// What a checkbox on the Review page changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleTarget {
    /// Split a one-document manuscript at its headings.
    Split(i64),
    Trash,
    BlankLines,
    Ornaments,
}

/// The control a row of the Review page's choices carries. Its value is a signal the
/// view binds: a new plan that offers the same controls writes its values into them, so
/// the control the writer just used stays, and keeps the focus.
#[derive(Clone)]
pub enum ReviewControl {
    /// A section heading.
    Heading,
    /// One answer among several.
    Pick {
        target: PickTarget,
        options: Vec<LocalizedString>,
        selected: Signal<Option<usize>>,
    },
    /// On or off.
    Toggle {
        target: ToggleTarget,
        on: Signal<bool>,
    },
}

/// One row of the Review page's choices.
#[derive(Clone)]
pub struct ReviewRow {
    pub label: LocalizedString,
    /// A second line: how many items it concerns, an example.
    pub detail: Option<LocalizedString>,
    pub control: ReviewControl,
}

/// What a row's control is about, as a new plan's row is matched with the one on screen.
#[derive(PartialEq)]
enum RowSubject {
    Heading(String),
    Pick(PickTarget, Vec<String>),
    Toggle(ToggleTarget),
}

impl ReviewRow {
    fn subject(&self) -> RowSubject {
        match &self.control {
            ReviewControl::Heading => RowSubject::Heading(self.label.resolve_now()),
            ReviewControl::Pick {
                target, options, ..
            } => RowSubject::Pick(*target, options.iter().map(|o| o.resolve_now()).collect()),
            ReviewControl::Toggle { target, .. } => RowSubject::Toggle(*target),
        }
    }

    /// Its words, as they read now.
    fn words(&self) -> (String, Option<String>) {
        (
            self.label.resolve_now(),
            self.detail.as_ref().map(|d| d.resolve_now()),
        )
    }

    /// Take the value `newer`'s control holds, keeping this row's control.
    fn take_value_of(&self, newer: &ReviewRow) {
        match (&self.control, &newer.control) {
            (
                ReviewControl::Pick { selected, .. },
                ReviewControl::Pick {
                    selected: value, ..
                },
            ) => {
                selected.set_if_changed(value.get());
            }
            (ReviewControl::Toggle { on, .. }, ReviewControl::Toggle { on: value, .. }) => {
                on.set_if_changed(value.get());
            }
            _ => {}
        }
    }
}

/// The answers to the questions a project asks before it can be read.
#[derive(Clone, Default, PartialEq)]
struct Answers {
    project: String,
    manifest: String,
    use_enclosing_project: bool,
    upgraded_sibling: ScrivenerSiblingChoice,
}

#[derive(Clone)]
pub struct ImportScrivenerViewModel {
    inner: Rc<Inner>,
}

struct Inner {
    job: ProjectImportJob,
    inspection: ScrivenerInspection,
    controller: StepperController,

    // Project
    source_text: Signal<String>,
    /// The path last submitted, the one the path field's check looks at on the disk: a
    /// keystroke never does (a network share that does not answer would stop the window).
    submitted: Rc<RefCell<String>>,
    source_check: CachedValidation,
    /// The source the current inspection reads, as it was submitted.
    inspected: RefCell<String>,
    /// The project the analysis on the page read, as it says it (else the source
    /// submitted): what the new project goes beside, and what a destination may not be
    /// inside.
    read_source: Signal<String>,
    /// The structure choices an import asked for, applied again once the project reads:
    /// **Review again**.
    carried: RefCell<Option<Vec<ScrivenerChoice>>>,
    /// The folder remembered for new projects, the default destination's second choice.
    remembered_folder: RefCell<Option<PathBuf>>,
    answers: RefCell<Answers>,
    /// The questions to choose met so far, in the order they were asked.
    asked: RefCell<Vec<ScrivenerQuestion>>,
    /// The keys of the questions to accept the writer accepted.
    accepted: RefCell<BTreeSet<String>>,
    /// The analysis on the page, when one reads.
    analysis: RefCell<Option<Rc<ScrivenerAnalysisDto>>>,
    page: Signal<usize>,
    progress: Signal<f32>,
    card: Signal<Option<CardView>>,
    notices: ListModel<NoticeView>,
    questions: ListModel<QuestionView>,
    /// The refusal on the page: none, or one.
    refusal: ListModel<RefusalView>,
    failure: Signal<Option<String>>,
    can_leave_project: Signal<bool>,

    // Review
    choices: RefCell<Vec<ScrivenerChoice>>,
    structure: RefCell<Option<ScrivenerStructure>>,
    review_rows: ListModel<ReviewRow>,
    preview: PreviewSource,
    before: ListModel<LocalizedString>,
    /// Whether the strip has a line: it is hidden when it has none.
    has_before: Signal<bool>,
    review_error: Signal<Option<LocalizedString>>,
    blank_lines: Signal<bool>,
    ornaments: Signal<bool>,
    can_leave_review: Signal<bool>,

    // Save
    destination: ImportDestination,
    /// Whether the folder is inside the Scrivener project: Scrivener keeps only its own
    /// files in there.
    inside_check: CachedValidation,
    title: Signal<String>,
    author: Signal<String>,
    language: Signal<Option<String>>,
    /// The tags the language field offers.
    languages: ListModel<String>,
    copy_research: Signal<bool>,
    research_files: Signal<i64>,
    save_hints: ListModel<LocalizedString>,
    /// The writer edited the name, the folder, the title, the author or the language: a
    /// later reading no longer replaces it.
    name_touched: Cell<bool>,
    location_touched: Cell<bool>,
    title_touched: Cell<bool>,
    author_touched: Cell<bool>,
    language_touched: Cell<bool>,
    /// The view-model is writing a default, not the writer: no field is touched by it.
    defaulting: Cell<bool>,
    /// The import report this window showed last, or whose import this window's wizard
    /// started: what the command palette's entry reopens here.
    report: RefCell<Option<Rc<ScrivenerImportResult>>>,
    /// The output of the import this window's wizard started last, until its result is in.
    started: RefCell<Option<String>>,
    _watch: RefCell<Vec<ObserverHandle>>,
}

impl ImportScrivenerViewModel {
    pub(crate) fn new(app_ctx: Rc<AppContext>, job: ProjectImportJob) -> Self {
        let source_text = Signal::new(String::new());
        let submitted = Rc::new(RefCell::new(String::new()));
        let source_check = {
            let (source, submitted) = (source_text.clone(), submitted.clone());
            CachedValidation::new(&[&source_text], move || {
                source_state(&source.get(), &submitted.borrow())
            })
        };
        let destination = ImportDestination::new(&DESTINATION);
        let read_source = Signal::new(String::new());
        let inside_check = {
            let (location, source) = (destination.location(), read_source.clone());
            CachedValidation::new(&[&destination.location(), &read_source], move || {
                inside_state(&location.get(), &source.get())
            })
        };
        let inner = Rc::new(Inner {
            job,
            inspection: ScrivenerInspection::new(app_ctx),
            controller: StepperController::new(STEPS),
            source_text,
            submitted,
            source_check,
            inspected: RefCell::new(String::new()),
            read_source,
            carried: RefCell::new(None),
            remembered_folder: RefCell::new(None),
            answers: RefCell::new(Answers::default()),
            asked: RefCell::new(Vec::new()),
            accepted: RefCell::new(BTreeSet::new()),
            analysis: RefCell::new(None),
            page: Signal::new(PAGE_EMPTY),
            progress: Signal::new(0.0),
            card: Signal::new(None),
            notices: ListModel::new(),
            questions: ListModel::new(),
            refusal: ListModel::new(),
            failure: Signal::new(None),
            can_leave_project: Signal::new(false),
            choices: RefCell::new(Vec::new()),
            structure: RefCell::new(None),
            review_rows: ListModel::new(),
            preview: PreviewSource::new(),
            before: ListModel::new(),
            has_before: Signal::new(false),
            review_error: Signal::new(None),
            blank_lines: Signal::new(true),
            ornaments: Signal::new(false),
            can_leave_review: Signal::new(false),
            destination,
            inside_check,
            title: Signal::new(String::new()),
            author: Signal::new(String::new()),
            language: Signal::new(None),
            languages: ListModel::from_vec(language_tags(None)),
            copy_research: Signal::new(true),
            research_files: Signal::new(0),
            save_hints: ListModel::new(),
            name_touched: Cell::new(false),
            location_touched: Cell::new(false),
            title_touched: Cell::new(false),
            author_touched: Cell::new(false),
            language_touched: Cell::new(false),
            defaulting: Cell::new(false),
            report: RefCell::new(None),
            started: RefCell::new(None),
            _watch: RefCell::new(Vec::new()),
        });
        let vm = Self { inner };
        vm.watch();
        vm
    }

    /// Observe what the view-model reacts to: the inspection's state, and the writer's own
    /// edits of the Save page's fields. Weak: the observers live in signals the view-model
    /// owns, and must not keep it alive.
    fn watch(&self) {
        let weak = Rc::downgrade(&self.inner);
        let on_state = self.inner.inspection.state().observe(move |state| {
            if let Some(inner) = weak.upgrade() {
                ImportScrivenerViewModel { inner }.on_state(state);
            }
        });
        let inner = &self.inner;
        let handles = [
            on_state,
            self.touched_by_writer(&inner.destination.name(), |i| &i.name_touched),
            self.touched_by_writer(&inner.destination.location(), |i| &i.location_touched),
            self.touched_by_writer(&inner.title, |i| &i.title_touched),
            self.touched_by_writer(&inner.author, |i| &i.author_touched),
            self.touched_by_writer(&inner.language, |i| &i.language_touched),
        ];
        self.inner._watch.borrow_mut().extend(handles);
    }

    /// Mark `flag` once the writer, not the view-model, sets `field`.
    fn touched_by_writer<T: Clone + 'static>(
        &self,
        field: &Signal<T>,
        flag: fn(&Inner) -> &Cell<bool>,
    ) -> ObserverHandle {
        let weak = Rc::downgrade(&self.inner);
        field.observe(move |_| {
            if let Some(inner) = weak.upgrade()
                && !inner.defaulting.get()
            {
                flag(&inner).set(true);
            }
        })
    }

    /// Empty every page, for a wizard being opened afresh. A reading under way stops; an
    /// import running is the job's, and goes on.
    pub fn reset(&self) {
        let inner = &self.inner;
        inner.inspection.stop();
        inner.source_text.set(String::new());
        inner.submitted.borrow_mut().clear();
        inner.inspected.borrow_mut().clear();
        inner.read_source.set(String::new());
        *inner.carried.borrow_mut() = None;
        *inner.answers.borrow_mut() = Answers::default();
        inner.asked.borrow_mut().clear();
        inner.accepted.borrow_mut().clear();
        inner.title_touched.set(false);
        inner.author_touched.set(false);
        inner.language_touched.set(false);
        self.clear_read();
        inner.page.set(PAGE_EMPTY);
        inner.blank_lines.set(true);
        inner.ornaments.set(false);
        inner.copy_research.set(true);
        inner.defaulting.set(true);
        inner.destination.clear();
        inner.defaulting.set(false);
        inner.name_touched.set(false);
        inner.location_touched.set(false);
        inner.controller.reset();
    }

    /// Forget what the last analysis read.
    fn clear_read(&self) {
        let inner = &self.inner;
        *inner.analysis.borrow_mut() = None;
        inner.card.set(None);
        inner.notices.replace_all(Vec::new());
        inner.questions.replace_all(Vec::new());
        inner.refusal.replace_all(Vec::new());
        inner.failure.set(None);
        inner.choices.borrow_mut().clear();
        *inner.structure.borrow_mut() = None;
        inner.review_rows.replace_all(Vec::new());
        inner.preview.clear();
        inner.before.replace_all(Vec::new());
        inner.has_before.set_if_changed(false);
        inner.review_error.set(None);
        inner.save_hints.replace_all(Vec::new());
        inner.research_files.set(0);
        // What the writer typed stays; what the last project proposed goes with it.
        inner.defaulting.set(true);
        if !inner.title_touched.get() {
            inner.title.set(String::new());
        }
        if !inner.author_touched.get() {
            inner.author.set(String::new());
        }
        if !inner.language_touched.get() {
            inner.language.set(None);
        }
        inner.defaulting.set(false);
        self.refresh_gates();
    }

    /// The folder the writer last created a project in, the default destination when the
    /// one beside the Scrivener project will not do. Read where an `EventContext` is at
    /// hand, when the wizard opens.
    pub fn set_remembered_folder(&self, folder: Option<PathBuf>) {
        *self.inner.remembered_folder.borrow_mut() = folder;
    }

    pub fn controller(&self) -> StepperController {
        self.inner.controller.clone()
    }

    /// The inspection, for the window that owns the wizard to wire to its tree.
    pub fn inspection(&self) -> ScrivenerInspection {
        self.inner.inspection.clone()
    }

    /// Subscribe this window's tree to the inspection's long-operation events. Called on
    /// every build of the widget that owns the view-model.
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.inner.inspection.wire(ctx);
    }

    // ── Project ──────────────────────────────────────────────────────────────────────────

    pub fn source_text(&self) -> Signal<String> {
        self.inner.source_text.clone()
    }

    /// The path field's verdict: "nothing is at this path", or a name that cannot be read,
    /// worked out when the path is submitted; an edit clears it. Reading it never touches
    /// the disk.
    pub fn source_validation(&self) -> Signal<ValidationState> {
        self.inner.source_check.signal()
    }

    /// The hint under the path: a `.scriv` project is a folder on Windows and Linux, and a
    /// single file on a Mac where Scrivener is installed.
    pub fn source_hint(&self) -> LocalizedString {
        source_hint(cfg!(target_os = "macos"))
    }

    /// Read the project the path names, unless that is the project already read: the
    /// field submits on Enter and when it loses focus, and the second must not read the
    /// same project twice.
    pub fn submit_source(&self) {
        let source = self.inner.source_text.get().trim().to_string();
        if source.is_empty() {
            return;
        }
        let same = *self.inner.inspected.borrow() == source;
        let settled = matches!(
            self.inner.inspection.state().get(),
            InspectState::Reading { .. } | InspectState::Read(_)
        );
        if same && settled {
            return;
        }
        *self.inner.submitted.borrow_mut() = source.clone();
        if !self.inner.source_check.recheck() {
            return;
        }
        *self.inner.inspected.borrow_mut() = source;
        *self.inner.answers.borrow_mut() = Answers::default();
        self.inner.asked.borrow_mut().clear();
        self.inner.accepted.borrow_mut().clear();
        *self.inner.carried.borrow_mut() = None;
        self.read();
    }

    /// **Review again** after an import that failed on `source`: read it again with what
    /// the writer answered, accepted and chose for that import, and keep the Save page as
    /// they left it. The project may have changed: a choice its new plan no longer offers
    /// is dropped, an acceptance it no longer asks for goes. An import this process did
    /// not send reads the project afresh.
    pub fn review_again(&self, source: &str) {
        let inner = &self.inner;
        let sent = SENT.with(|sent| sent.borrow().get(source).cloned());
        inner.source_text.set(source.to_string());
        let Some(sent) = sent else {
            self.submit_source();
            return;
        };
        *inner.submitted.borrow_mut() = source.to_string();
        if !inner.source_check.recheck() {
            return;
        }
        let request = &sent.request;
        *inner.inspected.borrow_mut() = source.to_string();
        *inner.answers.borrow_mut() = Answers {
            project: request.chosen_project.clone(),
            manifest: request.chosen_manifest.clone(),
            use_enclosing_project: request.use_enclosing_project,
            upgraded_sibling: request.upgraded_sibling.clone(),
        };
        *inner.asked.borrow_mut() = sent.asked.clone();
        {
            let mut accepted = inner.accepted.borrow_mut();
            accepted.clear();
            if request.accept_unmerged_mobile_edits {
                accepted.insert(UNMERGED_MOBILE_EDITS.to_string());
            }
            if request.accept_no_text {
                accepted.insert(NO_TEXT_IN_COPY.to_string());
            }
        }
        inner.blank_lines.set(request.blank_lines_as_scene_breaks);
        inner.ornaments.set(request.ornaments_as_scene_breaks);
        inner.copy_research.set(request.copy_research_files);
        // The Save page as the writer confirmed it by importing: theirs, whatever the
        // project proposes when it reads again.
        inner.destination.location().set(sent.location.clone());
        inner.destination.name().set(sent.name.clone());
        inner.title.set(request.title.clone().unwrap_or_default());
        inner.author.set(request.author.clone().unwrap_or_default());
        inner.language.set(
            request
                .language
                .clone()
                .filter(|language| !language.trim().is_empty()),
        );
        for touched in [
            &inner.name_touched,
            &inner.location_touched,
            &inner.title_touched,
            &inner.author_touched,
            &inner.language_touched,
        ] {
            touched.set(true);
        }
        *inner.carried.borrow_mut() = Some(request.choices.clone());
        self.read();
    }

    /// Read the project again, as it is now, with the answers already given: **Check
    /// again** after a refusal whose cause may have gone away.
    pub fn check_again(&self) {
        if !self.inner.inspected.borrow().is_empty() {
            self.read();
        }
    }

    /// Start the inspection with the current source and answers. The steps after this
    /// one were about the project read before, so they start over.
    fn read(&self) {
        self.inner.controller.reset();
        let request = {
            let answers = self.inner.answers.borrow();
            AnalyzeScrivenerProjectDto {
                source_path: self.inner.inspected.borrow().clone(),
                chosen_project: answers.project.clone(),
                chosen_manifest: answers.manifest.clone(),
                use_enclosing_project: answers.use_enclosing_project,
                upgraded_sibling: answers.upgraded_sibling.clone(),
                accept_unmerged_mobile_edits: false,
                accept_no_text: false,
                today: writer_today(),
            }
        };
        self.inner.inspection.start(request);
    }

    /// Open the file picker: a `.scriv` package on a Mac, a `.scrivx`, a backup `.zip` or a
    /// `.scrivtemplate`.
    pub fn pick_file(&self, ctx: &mut EventContext) {
        let vm = self.clone();
        let request = crate::models::dialog_start_in(
            ctx,
            crate::models::FolderPurpose::ImportScrivener,
            // Resolved at the click, in the language the writer is reading the wizard in.
            FileDialogRequest::pick_file()
                .title(tr!(import_scrivener_dialog_file_title()).resolve_now())
                .add_filter(
                    tr!(import_scrivener_dialog_filter()).resolve_now(),
                    &["scriv", "scrivx", "zip", "scrivtemplate"],
                ),
        );
        let _ = ctx.pick_file(request, move |res, c| {
            crate::models::remember_pick(c, crate::models::FolderPurpose::ImportScrivener, &res);
            vm.apply_pick(&res);
        });
    }

    /// Open the folder picker: a `.scriv` folder, or a folder holding projects.
    pub fn pick_folder(&self, ctx: &mut EventContext) {
        let vm = self.clone();
        let request = crate::models::dialog_start_in(
            ctx,
            crate::models::FolderPurpose::ImportScrivener,
            FileDialogRequest::pick_folder()
                .title(tr!(import_scrivener_dialog_folder_title()).resolve_now()),
        );
        let _ = ctx.pick_folder(request, move |res, c| {
            crate::models::remember_pick(c, crate::models::FolderPurpose::ImportScrivener, &res);
            vm.apply_pick(&res);
        });
    }

    /// What either picker chose: the path, read at once.
    pub fn apply_pick(&self, res: &FileDialogResult) {
        let picked = match res {
            FileDialogResult::File(Some(path)) | FileDialogResult::Folder(Some(path)) => {
                Some(path.clone())
            }
            _ => None,
        };
        if let Some(path) = picked {
            self.set_source(&path);
        }
    }

    /// What was dropped on the page: the first path, read at once.
    pub fn accept_dropped(&self, paths: &[PathBuf]) {
        if let Some(path) = paths.first() {
            self.set_source(path);
        }
    }

    fn set_source(&self, path: &Path) {
        self.inner
            .source_text
            .set(path.to_string_lossy().into_owned());
        self.submit_source();
    }

    /// Which part of the Project page shows: nothing chosen, reading, the card, a refusal,
    /// a reading that failed.
    pub fn inspection_page(&self) -> Signal<usize> {
        self.inner.page.clone()
    }

    /// How far the reading has got, from 0 to 1.
    pub fn inspect_progress(&self) -> Signal<f32> {
        self.inner.progress.clone()
    }

    pub fn stop_reading(&self) {
        self.inner.inspection.stop();
    }

    /// Whether the project is being read now.
    pub fn is_reading(&self) -> bool {
        self.inner.inspection.is_reading()
    }

    pub fn card(&self) -> Signal<Option<CardView>> {
        self.inner.card.clone()
    }

    /// Whether a project read, so the card has lines to show: a question asked before
    /// anything could be read leaves it empty.
    pub fn has_card(&self) -> Signal<usize> {
        self.inner.card.map(|card| usize::from(card.is_some()))
    }

    /// The card's first line, the project's name.
    pub fn card_name(&self) -> Signal<String> {
        self.inner
            .card
            .map(|card| card.as_ref().map(|c| c.name.clone()).unwrap_or_default())
    }

    pub fn card_made_with(&self) -> Signal<String> {
        self.inner.card.map(|card| {
            card.as_ref()
                .map(|c| c.made_with.resolve_now())
                .unwrap_or_default()
        })
    }

    pub fn card_saved_on(&self) -> Signal<String> {
        self.inner.card.map(|card| {
            card.as_ref()
                .and_then(|c| c.saved_on.as_ref().map(|s| s.resolve_now()))
                .unwrap_or_default()
        })
    }

    pub fn card_counts(&self) -> Signal<String> {
        self.inner.card.map(|card| {
            card.as_ref()
                .map(|c| c.counts.resolve_now())
                .unwrap_or_default()
        })
    }

    pub fn notices(&self) -> ListModel<NoticeView> {
        self.inner.notices.clone()
    }

    pub fn questions(&self) -> ListModel<QuestionView> {
        self.inner.questions.clone()
    }

    /// The refusal the page shows: none, or one.
    pub fn refusal(&self) -> ListModel<RefusalView> {
        self.inner.refusal.clone()
    }

    /// The technical account of a reading that failed.
    pub fn failure(&self) -> Signal<Option<String>> {
        self.inner.failure.clone()
    }

    /// Whether Next may leave the Project page: a project that reads, and every question
    /// on it answered or accepted.
    pub fn can_leave_project_step(&self) -> Signal<bool> {
        self.inner.can_leave_project.clone()
    }

    /// The writer chose answer `option` to question `index` on the Project page: read
    /// the project again with it. Questions asked after this one were about what this one
    /// chose, so their answers go, and so does every acceptance: it was given for the
    /// project read before, and another project, or another copy of it, has to be accepted
    /// for itself.
    pub fn choose(&self, index: usize, option: usize) {
        let question = self.inner.asked.borrow().get(index).cloned();
        let Some(question) = question else {
            return;
        };
        if !answer(&mut self.inner.answers.borrow_mut(), &question, option) {
            return;
        }
        {
            let mut asked = self.inner.asked.borrow_mut();
            let mut answers = self.inner.answers.borrow_mut();
            for later in asked.drain(index + 1..) {
                forget_answer(&mut answers, &later);
            }
        }
        self.inner.accepted.borrow_mut().clear();
        self.read();
    }

    /// The writer accepted, or took back, the question to accept `key`.
    pub fn accept(&self, key: &str, yes: bool) {
        {
            let mut accepted = self.inner.accepted.borrow_mut();
            if yes {
                accepted.insert(key.to_string());
            } else {
                accepted.remove(key);
            }
        }
        self.refresh_gates();
    }

    /// The inspection moved on: show where it stands.
    fn on_state(&self, state: &InspectState) {
        let inner = &self.inner;
        match state {
            InspectState::Idle => {
                self.clear_read();
                inner.page.set(PAGE_EMPTY);
            }
            InspectState::Reading { percent } => {
                inner.progress.set(*percent);
                if inner.page.get() != PAGE_READING {
                    self.clear_read();
                    inner.page.set(PAGE_READING);
                }
            }
            InspectState::Failed(details) => {
                self.clear_read();
                inner.failure.set(Some(details.to_string()));
                // The answers that led here stay on the page, to choose another.
                self.show_questions(&[]);
                inner.page.set(PAGE_FAILED);
            }
            InspectState::Read(analysis) => {
                self.clear_read();
                match analysis.outcome {
                    ScrivenerAnalysisOutcome::Refused => {
                        inner
                            .refusal
                            .replace_all(vec![refusal_view(&analysis.refusal)]);
                        // A refusal reached through an answer keeps the question that led
                        // to it, so the writer can choose another project or copy.
                        self.show_questions(&[]);
                        self.show_notices(analysis);
                        inner.page.set(PAGE_REFUSED);
                    }
                    ScrivenerAnalysisOutcome::Choose => {
                        self.remember_question(&analysis.question);
                        // A question with a right answer is answered at once; the radio
                        // stays, so the writer can change it. Only once: asked again
                        // with that answer given, it is the writer's to answer.
                        let unanswered =
                            chosen_answer(&inner.answers.borrow(), &analysis.question) == NO_ANSWER;
                        if unanswered
                            && let Some(option) = preselection(&analysis.question)
                            && answer(&mut inner.answers.borrow_mut(), &analysis.question, option)
                        {
                            self.read();
                            return;
                        }
                        self.show_questions(&analysis.pending);
                        self.show_notices(analysis);
                        inner.page.set(PAGE_CARD);
                    }
                    ScrivenerAnalysisOutcome::Read => {
                        *inner.analysis.borrow_mut() = Some(analysis.clone());
                        // An acceptance answers a question this project asks, or none.
                        inner
                            .accepted
                            .borrow_mut()
                            .retain(|key| analysis.pending.iter().any(|q| q.key() == key));
                        inner.card.set(Some(card_view(analysis)));
                        self.show_questions(&analysis.pending);
                        self.show_notices(analysis);
                        self.install_plan(analysis.structure.clone());
                        *inner.choices.borrow_mut() = planned_choices(&analysis.structure);
                        self.apply_carried_choices();
                        // The project the analysis read, not the path picked: a folder
                        // around it or inside it, a link, a copy beside it.
                        inner
                            .read_source
                            .set(read_project(analysis, &inner.inspected.borrow()));
                        self.fill_save_page(analysis);
                        inner.page.set(PAGE_CARD);
                    }
                }
            }
        }
        self.refresh_gates();
    }

    /// Keep a question to choose among the ones met, replacing an earlier question of the
    /// same kind.
    fn remember_question(&self, question: &ScrivenerQuestion) {
        let mut asked = self.inner.asked.borrow_mut();
        match asked.iter().position(|q| q.key() == question.key()) {
            Some(at) => asked[at] = question.clone(),
            None => asked.push(question.clone()),
        }
    }

    /// The questions on the page: every question to choose met so far, with its answer,
    /// then `pending`, the ones the analysis waits to have accepted.
    fn show_questions(&self, pending: &[ScrivenerQuestion]) {
        let answers = self.inner.answers.borrow().clone();
        let accepted = self.inner.accepted.borrow().clone();
        let asked = self.inner.asked.borrow().clone();
        let mut views = Vec::new();
        for question in asked.iter().chain(pending.iter()) {
            let Some(words) = messages::question(question) else {
                continue;
            };
            let index = views.len();
            views.push(QuestionView {
                index,
                key: question.key().to_string(),
                title: words.title,
                body: words.body,
                options: words.options,
                chosen: Signal::new(chosen_answer(&answers, question)),
                accepted: Signal::new(accepted.contains(question.key())),
                accept: words.accept,
            });
        }
        self.inner.questions.replace_all(views);
    }

    /// The notices the Project page shows, and the Save page's hints.
    fn show_notices(&self, analysis: &ScrivenerAnalysisDto) {
        let mut card = Vec::new();
        let mut hints = Vec::new();
        for finding in &analysis.notices {
            let Some(entry) = messages::entry(messages::key_of(finding)) else {
                continue;
            };
            let Some(worded) = messages::notice(finding, &analysis.computers) else {
                continue;
            };
            match entry.tier {
                Tier::Card => card.push(NoticeView {
                    title: worded.title,
                    body: worded.body,
                    warning: worded.warning,
                }),
                Tier::SaveHint => hints.push(worded.title),
                _ => {}
            }
        }
        self.inner.notices.replace_all(card);
        self.inner.save_hints.replace_all(hints);
    }

    /// Recompute the gates the steps' Next and Import read.
    fn refresh_gates(&self) {
        let inner = &self.inner;
        let readable = inner
            .analysis
            .borrow()
            .as_ref()
            .is_some_and(|a| a.outcome == ScrivenerAnalysisOutcome::Read);
        let all_accepted = {
            let accepted = inner.accepted.borrow();
            inner
                .analysis
                .borrow()
                .as_ref()
                .is_none_or(|a| a.pending.iter().all(|q| accepted.contains(q.key())))
        };
        let project_ok = readable && all_accepted;
        inner.can_leave_project.set_if_changed(project_ok);
        let planned = inner.structure.borrow().is_some() && inner.review_error.get().is_none();
        inner.can_leave_review.set_if_changed(project_ok && planned);
    }

    // ── Review ───────────────────────────────────────────────────────────────────────────

    pub fn review_rows(&self) -> ListModel<ReviewRow> {
        self.inner.review_rows.clone()
    }

    pub fn preview(&self) -> PreviewSource {
        self.inner.preview.clone()
    }

    /// The "before you import" lines: what the import does with the binder, or leaves
    /// behind, one line per kind.
    pub fn before_you_import(&self) -> ListModel<LocalizedString> {
        self.inner.before.clone()
    }

    /// Whether the "before you import" strip has a line.
    pub fn has_before_you_import(&self) -> Signal<bool> {
        self.inner.has_before.clone()
    }

    /// Why the plan could not be recomputed with the writer's last choice, if it could
    /// not.
    pub fn review_error(&self) -> Signal<Option<LocalizedString>> {
        self.inner.review_error.clone()
    }

    pub fn can_leave_review_step(&self) -> Signal<bool> {
        self.inner.can_leave_review.clone()
    }

    /// The structure choices as they stand: each a change from what was detected.
    pub fn choices(&self) -> Vec<ScrivenerChoice> {
        self.inner.choices.borrow().clone()
    }

    /// The writer chose answer `index` of a pick on the Review page.
    pub fn pick(&self, target: PickTarget, index: usize) {
        let choice = match target {
            PickTarget::Containers(depth) => {
                RUNGS.get(index).map(|rung| ScrivenerChoice::Containers {
                    depth,
                    rung: rung.clone(),
                })
            }
            PickTarget::Texts(depth) => TEXT_KINDS.get(index).map(|kind| ScrivenerChoice::Texts {
                depth,
                kind: kind.clone(),
            }),
            PickTarget::Root(item) => {
                DESTINATIONS
                    .get(index)
                    .map(|destination| ScrivenerChoice::Root {
                        item,
                        destination: destination.clone(),
                    })
            }
            PickTarget::Books => BOOKS.get(index).map(|choice| ScrivenerChoice::Books {
                choice: choice.clone(),
            }),
        };
        if let Some(choice) = choice {
            self.set_choice(choice);
        }
    }

    /// The writer ticked or cleared a checkbox on the Review page.
    pub fn toggle(&self, target: ToggleTarget, on: bool) {
        match target {
            ToggleTarget::Split(item) => {
                self.set_choice(ScrivenerChoice::Split { item, split: on })
            }
            ToggleTarget::Trash => self.set_choice(ScrivenerChoice::Trash {
                choice: if on {
                    ScrivenerTrashChoice::Import
                } else {
                    ScrivenerTrashChoice::LeaveOut
                },
            }),
            // The prose choices change the request, not the plan: their checkboxes hold the
            // view-model's own signals, so nothing on the page is rebuilt.
            ToggleTarget::BlankLines => {
                self.inner.blank_lines.set_if_changed(on);
            }
            ToggleTarget::Ornaments => {
                self.inner.ornaments.set_if_changed(on);
            }
        }
    }

    /// Replace the choice about the same thing as `choice`, and plan the structure again,
    /// now, with the converter's own mapping.
    pub fn set_choice(&self, choice: ScrivenerChoice) {
        let choices = {
            let mut choices = self.inner.choices.borrow_mut();
            choices.retain(|c| !same_subject(c, &choice));
            choices.push(choice);
            choices.clone()
        };
        match self.inner.inspection.plan(&choices) {
            Ok(structure) => {
                self.inner.review_error.set(None);
                self.install_plan(structure);
            }
            Err(error) => self.inner.review_error.set(Some(review_error(&error))),
        }
        self.refresh_gates();
    }

    /// Apply again the choices **Review again** carried, one at a time on the plan the
    /// project now gives: one its new plan does not offer is left out.
    fn apply_carried_choices(&self) {
        let Some(carried) = self.inner.carried.borrow_mut().take() else {
            return;
        };
        for choice in carried {
            let tentative = {
                let mut choices = self.inner.choices.borrow().clone();
                choices.retain(|c| !same_subject(c, &choice));
                choices.push(choice);
                choices
            };
            if let Ok(structure) = self.inner.inspection.plan(&tentative) {
                *self.inner.choices.borrow_mut() = tentative;
                self.install_plan(structure);
            }
        }
    }

    /// Show `structure`: its choices, its preview and what it does before the import.
    fn install_plan(&self, structure: ScrivenerStructure) {
        *self.inner.structure.borrow_mut() = Some(structure);
        self.refill_review_rows();
        self.refill_preview();
        self.refill_before();
    }

    /// Show the plan's choices. A plan that offers the same controls as the one on screen
    /// (any choice but one that changes which levels or folders there are) only writes
    /// their values and words into the rows already there: rebuilding them all would
    /// destroy the control the writer just used, and the focus with it.
    fn refill_review_rows(&self) {
        let rows = {
            let structure = self.inner.structure.borrow();
            match structure.as_ref() {
                Some(ScrivenerStructure::Planned { controls, .. }) => {
                    review_rows(controls, &self.inner.blank_lines, &self.inner.ornaments)
                }
                _ => Vec::new(),
            }
        };
        let model = &self.inner.review_rows;
        let same_controls = model.len() == rows.len()
            && rows.iter().enumerate().all(|(i, row)| {
                model
                    .with_item(i, |old| old.subject() == row.subject())
                    .unwrap_or(false)
            });
        if !same_controls {
            model.replace_all(rows);
            return;
        }
        for (i, row) in rows.into_iter().enumerate() {
            // A clone shares the row's signals: its control is the one on screen.
            let Some(on_screen) = model.with_item(i, ReviewRow::clone) else {
                continue;
            };
            on_screen.take_value_of(&row);
            if on_screen.words() != row.words() {
                model.set(
                    i,
                    ReviewRow {
                        label: row.label,
                        detail: row.detail,
                        control: on_screen.control,
                    },
                );
            }
        }
    }

    fn refill_preview(&self) {
        let structure = self.inner.structure.borrow();
        let rows = match structure.as_ref() {
            Some(ScrivenerStructure::Planned { rows, .. }) => preview_rows(rows),
            _ => Vec::new(),
        };
        self.inner.preview.set_rows(rows);
    }

    fn refill_before(&self) {
        let analysis = self.inner.analysis.borrow();
        let structure = self.inner.structure.borrow();
        let mut findings: Vec<&ScrivenerFinding> = Vec::new();
        if let Some(analysis) = analysis.as_ref() {
            findings.extend(analysis.notices.iter().filter(|f| {
                messages::entry(messages::key_of(f)).is_some_and(|e| e.tier == Tier::BeforeImport)
            }));
        }
        if let Some(ScrivenerStructure::Planned { notices, .. }) = structure.as_ref() {
            findings.extend(notices.iter());
        }
        let lines = before_lines(&findings);
        self.inner.has_before.set_if_changed(!lines.is_empty());
        self.inner.before.replace_all(lines);
    }

    pub fn blank_lines_as_scene_breaks(&self) -> Signal<bool> {
        self.inner.blank_lines.clone()
    }

    pub fn ornaments_as_scene_breaks(&self) -> Signal<bool> {
        self.inner.ornaments.clone()
    }

    // ── Save ─────────────────────────────────────────────────────────────────────────────

    pub fn location(&self) -> Signal<String> {
        self.inner.destination.location()
    }

    pub fn name(&self) -> Signal<String> {
        self.inner.destination.name()
    }

    /// The folder field's verdict: inside the Scrivener project first, then whatever the
    /// shared destination check said. Both cached; reading touches nothing.
    pub fn location_validation(&self) -> Signal<ValidationState> {
        self.inner
            .inside_check
            .signal()
            .zip(&self.inner.destination.location_validation())
            .map(|(inside, folder)| match inside {
                ValidationState::Error(_) => inside.clone(),
                _ => folder.clone(),
            })
    }

    pub fn name_validation(&self) -> Signal<ValidationState> {
        self.inner.destination.name_validation()
    }

    /// The reactive "Will create …" preview.
    pub fn target_path(&self) -> Signal<String> {
        self.inner.destination.target_path()
    }

    pub fn title(&self) -> Signal<String> {
        self.inner.title.clone()
    }

    pub fn author(&self) -> Signal<String> {
        self.inner.author.clone()
    }

    /// The manuscript's language, a tag.
    pub fn language(&self) -> Signal<Option<String>> {
        self.inner.language.clone()
    }

    /// The languages the Save page offers, by tag: the dictionaries Skribisto knows, and
    /// the one the project proposes when it is not among them. A model, so the field
    /// follows a project read after it was built.
    pub fn languages(&self) -> ListModel<String> {
        self.inner.languages.clone()
    }

    /// How the Save page names a language: the dictionary's own name and its tag, or the
    /// tag alone for one Skribisto has no dictionary for. Names are data, never translated.
    pub fn language_label(tag: &str) -> String {
        crate::spellcheck::dictionary_registry::entries()
            .iter()
            .find(|e| e.id == tag)
            .map(|e| format!("{} ({})", e.display_name, e.id))
            .unwrap_or_else(|| tag.to_string())
    }

    pub fn copy_research_files(&self) -> Signal<bool> {
        self.inner.copy_research.clone()
    }

    /// How many research files Skribisto cannot open the project holds: the copy
    /// checkbox shows only when there are some.
    pub fn research_files(&self) -> Signal<i64> {
        self.inner.research_files.clone()
    }

    /// The hints about what the project states, under the title, author and language.
    pub fn save_hints(&self) -> ListModel<LocalizedString> {
        self.inner.save_hints.clone()
    }

    /// Whether an import is running, here or in another window.
    pub fn job_running(&self) -> Signal<bool> {
        self.inner.job.running()
    }

    /// Whether Import may fire: a project that reads with everything accepted, a plan, a
    /// destination, and no import running. Derived from cached verdicts only.
    pub fn can_import(&self) -> Signal<bool> {
        let idle = self.inner.job.running().map(|running| !running);
        self.inner
            .can_leave_review
            .and(&self.inner.destination.is_ready())
            .and(&self.inner.inside_check.passes())
            .and(&idle)
    }

    /// Fill the Save page from what the project states: its name, beside it, and its
    /// title, author and language. A field the writer edited keeps what they wrote.
    fn fill_save_page(&self, analysis: &ScrivenerAnalysisDto) {
        let inner = &self.inner;
        inner.research_files.set(analysis.research_files);
        inner.defaulting.set(true);
        let proposed = proposals(analysis);
        if !inner.title_touched.get() {
            inner.title.set(proposed.title);
        }
        if !inner.author_touched.get() {
            inner.author.set(proposed.author);
        }
        let language = if inner.language_touched.get() {
            inner.language.get()
        } else {
            proposed.language
        };
        // The list first, so the field finds the value it is given among its items.
        inner
            .languages
            .replace_all(language_tags(language.as_deref()));
        if !inner.language_touched.get() {
            inner.language.set(language);
        }
        if !inner.name_touched.get() {
            inner.destination.name().set(default_name(analysis));
        }
        if !inner.location_touched.get() {
            let project = PathBuf::from(inner.read_source.get());
            let folder = default_location(
                &project,
                is_backup(analysis),
                &|path| {
                    matches!(
                        folder_state(&path.to_string_lossy(), &FOLDER),
                        ValidationState::None
                    )
                },
                inner.remembered_folder.borrow().clone(),
                home_folder(),
            );
            if let Some(folder) = folder {
                inner
                    .destination
                    .location()
                    .set(folder.to_string_lossy().into_owned());
            }
        }
        inner.defaulting.set(false);
    }

    /// The request the import sends, with every word it writes into the project resolved
    /// in the interface language; `None` until a project reads with everything accepted.
    pub fn request(&self) -> Option<ImportScrivenerProjectDto> {
        let inner = &self.inner;
        if !inner.can_leave_project.get() {
            return None;
        }
        let analysis = inner.analysis.borrow().clone()?;
        let answers = inner.answers.borrow().clone();
        let accepted = inner.accepted.borrow().clone();
        let words = Words::resolve();
        Some(ImportScrivenerProjectDto {
            source_path: inner.inspected.borrow().clone(),
            chosen_project: answers.project,
            chosen_manifest: answers.manifest,
            use_enclosing_project: answers.use_enclosing_project,
            upgraded_sibling: answers.upgraded_sibling,
            accept_unmerged_mobile_edits: accepted.contains(UNMERGED_MOBILE_EDITS),
            accept_no_text: accepted.contains(NO_TEXT_IN_COPY),
            fingerprint: analysis.fingerprint.clone(),
            choices: inner.choices.borrow().clone(),
            blank_lines_as_scene_breaks: inner.blank_lines.get(),
            ornaments_as_scene_breaks: inner.ornaments.get(),
            output_path: inner.destination.target(),
            overwrite: false,
            title: Some(inner.title.get()),
            author: Some(inner.author.get()),
            language: Some(inner.language.get().unwrap_or_default()),
            copy_research_files: analysis.research_files > 0 && inner.copy_research.get(),
            manuscript_binder_name: words.manuscript_binder,
            notes_binder_name: words.notes_binder,
            notes_of_pattern: words.notes_of,
            project_notes_name: words.project_notes,
            recovered_name: words.recovered,
            untitled_template_name: words.untitled_template,
            research_folder_pattern: words.research_folder,
            list_separator: words.list_separator,
            label_details_pattern: words.label_details,
            status_details_pattern: words.status_details,
            unnamed_label_pattern: words.unnamed_label,
            unnamed_status_pattern: words.unnamed_status,
            label_tag_pattern: words.label_tag,
            untitled_tag_name: words.untitled_tag,
            default_statuses: words.default_statuses,
            metadata_words: words.metadata,
            today: writer_today(),
        })
    }

    /// **Import**: check the destination against the disk again, then hand the request to
    /// the shared job, which refuses a second import and a target that is open, asks
    /// before replacing an existing file, then starts and closes the wizard. `false` keeps
    /// the wizard open on the Save page.
    pub fn import(&self, ctx: &mut EventContext) -> bool {
        // The verdicts on screen were worked out when the fields last changed, and the
        // disk may have moved on since: both run, so every field shows its fresh verdict.
        let destination_ok = self.inner.destination.recheck();
        let outside = self.inner.inside_check.recheck();
        if !(destination_ok && outside) || self.inner.job.is_running() {
            return false;
        }
        let Some(request) = self.request() else {
            return false;
        };
        let analysis = self.inner.analysis.borrow().clone();
        let proposed = analysis.as_deref().map(proposals).unwrap_or_default();
        let title = request.title.clone().unwrap_or_default();
        let author = request.author.clone().unwrap_or_default();
        let language = request.language.clone().unwrap_or_default();
        let read = self.inner.read_source.get();
        let record = ImportRecord {
            source: if read.trim().is_empty() {
                request.source_path.clone()
            } else {
                read
            },
            project_answers: self.project_answers(),
            accepted: self.inner.accepted.borrow().iter().cloned().collect(),
            choices: request.choices.clone(),
            worded: self.worded_choices(&request.choices),
            blank_lines_as_scene_breaks: request.blank_lines_as_scene_breaks,
            ornaments_as_scene_breaks: request.ornaments_as_scene_breaks,
            copy_research_files: request.copy_research_files,
            research_files: analysis_research_files(&analysis),
            title_changed: title.trim() != proposed.title.trim(),
            author_changed: author.trim() != proposed.author.trim(),
            language_changed: language != proposed.language.clone().unwrap_or_default(),
            language: if language.is_empty() {
                String::new()
            } else {
                Self::language_label(&language)
            },
            title,
            author,
        };
        PENDING_RECORDS.with(|pending| {
            pending
                .borrow_mut()
                .insert(request.output_path.clone(), record);
        });
        let sent = Sent {
            request: request.clone(),
            asked: self.inner.asked.borrow().clone(),
            location: self.inner.destination.location().get(),
            name: self.inner.destination.name().get(),
        };
        SENT.with(|all| {
            all.borrow_mut().insert(request.source_path.clone(), sent);
        });
        *self.inner.started.borrow_mut() = Some(request.output_path.clone());
        self.inner.job.import(ctx, &SCRIVENER, request);
        true
    }

    /// The Project page's answers as the report states them, in the order they were asked:
    /// which project, binder file or copy the import read.
    fn project_answers(&self) -> Vec<LocalizedString> {
        let answers = self.inner.answers.borrow();
        self.inner
            .asked
            .borrow()
            .iter()
            .filter_map(|question| match chosen_answer(&answers, question) {
                NO_ANSWER => None,
                option => messages::answered(question, option),
            })
            .collect()
    }

    /// The Review page's choices as the report states them: each structure choice the
    /// import sends, by the row that offered it and the answer it shows, then the two prose
    /// choices, which the import always applies.
    fn worded_choices(&self, choices: &[ScrivenerChoice]) -> Vec<ChoiceLine> {
        let model = &self.inner.review_rows;
        let rows: Vec<ReviewRow> = (0..model.len())
            .filter_map(|i| model.with_item(i, ReviewRow::clone))
            .collect();
        let mut lines: Vec<ChoiceLine> = choices
            .iter()
            .filter_map(|choice| {
                let row = rows.iter().find(|row| row_is_about(row, choice))?;
                Some(ChoiceLine {
                    subject: row.label.clone(),
                    answer: row_answer(row)?,
                })
            })
            .collect();
        for (target, on) in [
            (ToggleTarget::BlankLines, self.inner.blank_lines.get()),
            (ToggleTarget::Ornaments, self.inner.ornaments.get()),
        ] {
            let subject = rows.iter().find_map(|row| match &row.control {
                ReviewControl::Toggle { target: t, .. } if *t == target => Some(row.label.clone()),
                _ => None,
            });
            if let Some(subject) = subject {
                lines.push(ChoiceLine {
                    subject,
                    answer: yes_no(on),
                });
            }
        }
        lines
    }

    /// The Stepper's **Cancel**: close the wizard, stopping a reading under way. Always
    /// both: pressing Cancel takes the focus off the path field, which reads what it holds
    /// first, so a Cancel that only stopped a reading would have to be pressed twice. The
    /// reading page has its own **Stop reading**.
    pub fn cancel_or_dismiss(&self, ctx: &mut EventContext) {
        self.stop_reading();
        ctx.dismiss_modal();
    }

    /// The last Scrivener import this process finished, for its report: totals, every
    /// finding, and what the writer chose.
    pub fn last_import(&self) -> Option<Rc<ScrivenerImportResult>> {
        last_import()
    }

    /// The report this window reopens (the command palette's entry): this window's own, the newer of the one it showed last and the one of
    /// the import its wizard started; else the one about the project it shows (`project`,
    /// the window's open project); else the session's last import. `None` when no import
    /// has finished since Skribisto started.
    pub fn report_for_window(&self, project: Option<&str>) -> Option<Rc<ScrivenerImportResult>> {
        let shown = self.inner.report.borrow().clone();
        let started = self.inner.started.borrow().clone();
        let started = started.as_deref().and_then(import_of);
        let own = match (shown, started) {
            (Some(shown), Some(started)) => Some(if finished_after(&started, &shown) {
                started
            } else {
                shown
            }),
            (shown, started) => shown.or(started),
        };
        own.or_else(|| project.and_then(import_of))
            .or_else(last_import)
    }

    /// Remember `result` as the report this window showed last.
    pub fn showed_report(&self, result: Rc<ScrivenerImportResult>) {
        *self.inner.report.borrow_mut() = Some(result);
    }

    /// The command palette's entry: show this window's report ([`Self::report_for_window`]),
    /// or say that no import has finished since Skribisto started. `project` is the project
    /// the window shows; `find_item` finds a row of it by uid.
    pub(crate) fn show_window_report(
        &self,
        ctx: &mut EventContext,
        project: Option<&str>,
        find_item: Option<ItemFinder>,
    ) {
        match self.report_for_window(project) {
            Some(result) => self.present_report(ctx, result, project, find_item),
            None => say_no_report(ctx, tr!(import_scrivener_report_none())),
        }
    }

    /// A result notice's **See report**: show the report of the import that wrote
    /// `output_path`, or say it is no longer kept.
    pub(crate) fn show_report_of(
        &self,
        ctx: &mut EventContext,
        output_path: &str,
        project: Option<&str>,
        find_item: Option<ItemFinder>,
    ) {
        match report_of(output_path) {
            Ok(result) => self.present_report(ctx, result, project, find_item),
            Err(why) => say_no_report(ctx, why),
        }
    }

    /// Show `result`'s report over this window, its items linked to their rows when this
    /// window shows the project it wrote, and remember it as this window's.
    fn present_report(
        &self,
        ctx: &mut EventContext,
        result: Rc<ScrivenerImportResult>,
        project: Option<&str>,
        find_item: Option<ItemFinder>,
    ) {
        let shows_it = project.is_some_and(|project| same_project(project, &result.output_path));
        let report = ImportReport::from_result(&result);
        self.showed_report(result);
        present_import_report(
            ctx,
            ImportReportViewModel::new(report, find_item.filter(|_| shows_it)),
        );
    }
}

/// Whether two paths name the same project, whatever the spelling.
fn same_project(a: &str, b: &str) -> bool {
    a == b
        || crate::shell::open_registry::project_key(a)
            == crate::shell::open_registry::project_key(b)
}

/// The report of the import that wrote `output_path`, or why there is none: no import has
/// finished since Skribisto started, or this one is older than the reports it keeps.
pub(crate) fn report_of(output_path: &str) -> Result<Rc<ScrivenerImportResult>, LocalizedString> {
    match import_of(output_path) {
        Some(result) => Ok(result),
        None if last_import().is_none() => Err(tr!(import_scrivener_report_none())),
        None => Err(tr!(import_scrivener_report_gone(
            count = KEPT_REPORTS as i64
        ))),
    }
}

/// Say why there is no report to show.
fn say_no_report(ctx: &mut EventContext, why: LocalizedString) {
    MessageBox::information(tr!(import_scrivener_report_none_title()))
        .text(why)
        .buttons(MessageBoxButtons::Ok)
        .present(ctx);
}

/// Whether `later` finished after `earlier`: later in the session's list, or `earlier` is
/// no longer kept in it.
fn finished_after(later: &Rc<ScrivenerImportResult>, earlier: &Rc<ScrivenerImportResult>) -> bool {
    FINISHED.with(|finished| {
        let finished = finished.borrow();
        let at = |wanted: &Rc<ScrivenerImportResult>| {
            finished.iter().position(|r| Rc::ptr_eq(r, wanted))
        };
        match (at(later), at(earlier)) {
            (Some(l), Some(e)) => l > e,
            (_, None) => true,
            (None, Some(_)) => false,
        }
    })
}

/// What the Save page proposes for a project: its title, else its name; its authors; its
/// language, else the interface's.
#[derive(Default)]
struct Proposals {
    title: String,
    author: String,
    language: Option<String>,
}

fn proposals(analysis: &ScrivenerAnalysisDto) -> Proposals {
    Proposals {
        title: if analysis.proposed_title.trim().is_empty() {
            default_name(analysis)
        } else {
            analysis.proposed_title.clone()
        },
        author: analysis
            .proposed_authors
            .join(&tr!(import_scrivener_list_separator()).resolve_now()),
        language: if analysis.proposed_language.trim().is_empty() {
            interface_language()
        } else {
            Some(analysis.proposed_language.clone())
        },
    }
}

/// Whether the analysis on the page found research files Skribisto cannot open.
fn analysis_research_files(analysis: &Option<Rc<ScrivenerAnalysisDto>>) -> bool {
    analysis.as_ref().is_some_and(|a| a.research_files > 0)
}

/// Whether `row` is the Review page row that offers `choice`.
fn row_is_about(row: &ReviewRow, choice: &ScrivenerChoice) -> bool {
    use ScrivenerChoice as C;
    match (&row.control, choice) {
        (ReviewControl::Pick { target, .. }, C::Containers { depth, .. }) => {
            *target == PickTarget::Containers(*depth)
        }
        (ReviewControl::Pick { target, .. }, C::Texts { depth, .. }) => {
            *target == PickTarget::Texts(*depth)
        }
        (ReviewControl::Pick { target, .. }, C::Root { item, .. }) => {
            *target == PickTarget::Root(*item)
        }
        (ReviewControl::Pick { target, .. }, C::Books { .. }) => *target == PickTarget::Books,
        (ReviewControl::Toggle { target, .. }, C::Split { item, .. }) => {
            *target == ToggleTarget::Split(*item)
        }
        (ReviewControl::Toggle { target, .. }, C::Trash { .. }) => *target == ToggleTarget::Trash,
        _ => false,
    }
}

/// The answer a Review page row shows: its pick's label, or yes or no.
fn row_answer(row: &ReviewRow) -> Option<LocalizedString> {
    match &row.control {
        ReviewControl::Pick {
            options, selected, ..
        } => selected.get().and_then(|i| options.get(i).cloned()),
        ReviewControl::Toggle { on, .. } => Some(yes_no(on.get())),
        ReviewControl::Heading => None,
    }
}

/// "Yes" or "No", as the report answers a checkbox.
fn yes_no(on: bool) -> LocalizedString {
    if on {
        tr!(import_scrivener_report_yes())
    } else {
        tr!(import_scrivener_report_no())
    }
}

/// The form's disk-checked fields, looked at again while the wizard is on screen and one
/// of them is refused: the destination folder only. The project's path is looked at when
/// it is submitted, never behind the writer's back: once a second on a network share that
/// does not answer would stop the window for as long as each look waits.
impl DiskChecked for ImportScrivenerViewModel {
    fn disk_verdicts(&self) -> Vec<Signal<ValidationState>> {
        vec![self.inner.destination.location_validation()]
    }

    fn refused_on_disk(&self) -> bool {
        self.inner.destination.folder_refused()
    }

    fn retry_refused(&self) {
        self.inner.destination.retry_refused_folder();
    }
}

// ── the words the import writes into the project ─────────────────────────────────────────

/// Every word the import writes into the project, resolved in the interface language at
/// the moment of importing, and stored literally from then on.
struct Words {
    manuscript_binder: String,
    notes_binder: String,
    notes_of: String,
    project_notes: String,
    recovered: String,
    untitled_template: String,
    research_folder: String,
    list_separator: String,
    label_details: String,
    status_details: String,
    unnamed_label: String,
    unnamed_status: String,
    label_tag: String,
    untitled_tag: String,
    default_statuses: Vec<ScrivenerDefaultRung>,
    /// The second half of the metadata mapping's words, in the converter's order.
    metadata: Vec<String>,
}

impl Words {
    fn resolve() -> Self {
        let slot = || TITLE_SLOT.to_string();
        Self {
            manuscript_binder: tr!(import_scrivener_manuscript_binder()).resolve_now(),
            notes_binder: tr!(import_scrivener_notes_binder()).resolve_now(),
            notes_of: tr!(import_scrivener_notes_of(title = slot())).resolve_now(),
            project_notes: tr!(import_scrivener_project_notes()).resolve_now(),
            recovered: tr!(import_scrivener_recovered()).resolve_now(),
            untitled_template: tr!(import_scrivener_untitled_template()).resolve_now(),
            research_folder: tr!(import_scrivener_research_folder(name = slot())).resolve_now(),
            list_separator: tr!(import_scrivener_list_separator()).resolve_now(),
            label_details: tr!(import_scrivener_label_details(field = slot())).resolve_now(),
            status_details: tr!(import_scrivener_status_details(field = slot())).resolve_now(),
            unnamed_label: tr!(import_scrivener_unnamed_label(number = slot())).resolve_now(),
            unnamed_status: tr!(import_scrivener_unnamed_status(number = slot())).resolve_now(),
            label_tag: tr!(import_scrivener_label_tag(name = slot())).resolve_now(),
            untitled_tag: tr!(import_scrivener_untitled_tag()).resolve_now(),
            default_statuses: crate::statuses::Preset::DEFAULT
                .rows()
                .into_iter()
                .map(|row| ScrivenerDefaultRung::Rung {
                    name: row.name.resolve_now(),
                    category: row.category,
                })
                .collect(),
            metadata: metadata_words(),
        }
    }
}

/// The writer's calendar day, at midnight UTC as a pace plan's days are kept: the day a
/// deadline is ahead or behind, and the day an imported pace plan starts. The backend
/// knows only UTC, whose date is a day early or late for part of every day.
fn writer_today() -> chrono::DateTime<chrono::Utc> {
    crate::shared::stamps::today()
        .and_hms_opt(0, 0, 0)
        .map(|midnight| midnight.and_utc())
        .unwrap_or_default()
}

/// The word of the second half of the metadata mapping `key` names, resolved now; `None`
/// for a key this interface does not know, which the converter then writes empty.
fn metadata_word(key: &str) -> Option<LocalizedString> {
    let slot = || TITLE_SLOT.to_string();
    Some(match key {
        "keyword-details" => tr!(import_scrivener_keyword_details(path = slot())),
        "unnamed-keyword" => tr!(import_scrivener_unnamed_keyword(number = slot())),
        "keyword-tag" => tr!(import_scrivener_keyword_tag(name = slot())),
        "collection-details" => tr!(import_scrivener_collection_details()),
        "collection-tag" => tr!(import_scrivener_collection_tag(name = slot())),
        "field-details" => tr!(import_scrivener_field_details(field = slot())),
        "field-value" => tr!(import_scrivener_field_value(
            field = slot(),
            option = VALUE_SLOT.to_string()
        )),
        "field-tag" => tr!(import_scrivener_field_tag(name = slot())),
        "deadline-day" => tr!(import_scrivener_deadline_day()),
        "from-scrivener" => tr!(import_scrivener_from_scrivener()),
        // The folder's notes, titled as the report names them.
        note if note.starts_with("note-") => return messages::note_title(&note["note-".len()..]),
        "abbreviated-title" => tr!(import_scrivener_abbreviated_title()),
        "subjects" => tr!(import_scrivener_subjects()),
        "company" => tr!(import_scrivener_company()),
        "session-target" => tr!(import_scrivener_session_target()),
        "session-target-calculated" => tr!(import_scrivener_session_target_calculated()),
        "writing-days" => tr!(import_scrivener_writing_days()),
        "weekday-monday" => tr!(import_scrivener_weekday_monday()),
        "weekday-tuesday" => tr!(import_scrivener_weekday_tuesday()),
        "weekday-wednesday" => tr!(import_scrivener_weekday_wednesday()),
        "weekday-thursday" => tr!(import_scrivener_weekday_thursday()),
        "weekday-friday" => tr!(import_scrivener_weekday_friday()),
        "weekday-saturday" => tr!(import_scrivener_weekday_saturday()),
        "weekday-sunday" => tr!(import_scrivener_weekday_sunday()),
        "draft-target" => tr!(import_scrivener_draft_target()),
        "deadline" => tr!(import_scrivener_deadline()),
        "unit-words" => tr!(import_scrivener_unit_words()),
        "unit-characters" => tr!(import_scrivener_unit_characters()),
        "unit-pages" => tr!(import_scrivener_unit_pages()),
        "search-query" => tr!(import_scrivener_search_query()),
        "search-in" => tr!(import_scrivener_search_in()),
        "search-operator" => tr!(import_scrivener_search_operator()),
        "search-in-all" => tr!(import_scrivener_search_in_all()),
        "search-in-title" => tr!(import_scrivener_search_in_title()),
        "search-in-text" => tr!(import_scrivener_search_in_text()),
        "search-in-notes" => tr!(import_scrivener_search_in_notes()),
        "search-in-synopsis" => tr!(import_scrivener_search_in_synopsis()),
        "search-in-keywords" => tr!(import_scrivener_search_in_keywords()),
        "search-in-label" => tr!(import_scrivener_search_in_label()),
        "search-in-status" => tr!(import_scrivener_search_in_status()),
        "search-in-section-type" => tr!(import_scrivener_search_in_section_type()),
        "search-in-custom-metadata" => tr!(import_scrivener_search_in_custom_metadata()),
        "search-any" => tr!(import_scrivener_search_any()),
        "search-all" => tr!(import_scrivener_search_all()),
        "search-exact" => tr!(import_scrivener_search_exact()),
        "label-value" => tr!(import_scrivener_label_value(
            label = slot(),
            value = VALUE_SLOT.to_string()
        )),
        "replacement-regex" => tr!(import_scrivener_replacement_regex()),
        "replacement-off" => tr!(import_scrivener_replacement_off()),
        "in-own-note" => tr!(import_scrivener_in_own_note()),
        _ => return None,
    })
}

/// Every word of the second half, resolved in the interface's language, one per key in the
/// converter's order; a key this interface does not know is left empty.
fn metadata_words() -> Vec<String> {
    METADATA_WORD_ORDER
        .iter()
        .map(|key| {
            metadata_word(key)
                .map(|word| word.resolve_now())
                .unwrap_or_default()
        })
        .collect()
}

/// The name of a note of the "From Scrivener" folder, by the token the plan gives it; a
/// note this version does not know is named by its token rather than by nothing.
fn import_note_title(token: &str) -> LocalizedString {
    messages::note_title(token).unwrap_or_else(|| lit!(token.to_string()))
}

// ── pure helpers ─────────────────────────────────────────────────────────────────────────

/// The path field's verdict on `source`, looked at on the disk only when it is the path
/// last `submitted`: a path being typed says nothing (nor does a blank one: the page says
/// what to choose). A path with nothing at it is refused, and so is one whose name is not
/// Unicode, which reaches the field with its unreadable bytes replaced and so is never
/// there: that one is said for what it is, not called missing.
fn source_state(source: &str, submitted: &str) -> ValidationState {
    let trimmed = source.trim();
    if trimmed.is_empty() || trimmed != submitted.trim() || Path::new(trimmed).exists() {
        return ValidationState::None;
    }
    if trimmed.contains(char::REPLACEMENT_CHARACTER) {
        ValidationState::Error(tr!(import_scrivener_source_unreadable_name()))
    } else {
        ValidationState::Error(tr!(import_scrivener_source_missing()))
    }
}

/// Whether `location` is inside the Scrivener project `project`: the folders themselves
/// compared, whatever the spelling (a `..`, a link, a case or a Unicode normalisation the
/// filesystem does not tell apart), as the open-project registry compares two projects.
fn inside_state(location: &str, project: &str) -> ValidationState {
    let (location, project) = (location.trim(), project.trim());
    if location.is_empty() || project.is_empty() {
        return ValidationState::None;
    }
    let project = project_folder(Path::new(project));
    let key = |path: &Path| crate::shell::open_registry::project_key(&path.to_string_lossy());
    let (location_key, project_key) = (key(Path::new(location)), key(&project));
    if Path::new(&location_key).starts_with(Path::new(&project_key))
        || Path::new(location).starts_with(&project)
    {
        ValidationState::Error(tr!(import_scrivener_location_inside_project()))
    } else {
        ValidationState::None
    }
}

/// The project `analysis` read, as it says it, else the source submitted.
fn read_project(analysis: &ScrivenerAnalysisDto, submitted: &str) -> String {
    if analysis.project_path.trim().is_empty() {
        submitted.to_string()
    } else {
        analysis.project_path.clone()
    }
}

/// Whether the analysis found the project to be one of Scrivener's backups: the folder
/// holding it is then a backup folder, whatever it is called.
fn is_backup(analysis: &ScrivenerAnalysisDto) -> bool {
    analysis
        .notices
        .iter()
        .any(|n| messages::key_of(n) == "scrivener-notice-source-is-backup")
}

/// The hint under the path, by platform.
pub(crate) fn source_hint(mac: bool) -> LocalizedString {
    if mac {
        tr!(import_scrivener_source_hint_mac())
    } else {
        tr!(import_scrivener_source_hint())
    }
}

/// The folder (or the archive) that is the project: the `.scrivx` picked inside a project
/// stands for the folder that holds it.
pub(crate) fn project_folder(source: &Path) -> PathBuf {
    let is_manifest = source
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("scrivx"));
    match source.parent() {
        Some(parent) if is_manifest => parent.to_path_buf(),
        _ => source.to_path_buf(),
    }
}

/// Whether `folder` is, or is inside, a folder of Scrivener's automatic backups, where
/// Scrivener's own manual says never to work.
fn in_backups(folder: &Path) -> bool {
    folder.components().any(|c| {
        c.as_os_str()
            .to_string_lossy()
            .eq_ignore_ascii_case("backups")
    })
}

/// Where the new project goes by default: beside the Scrivener project the analysis read
/// (`project`), unless that folder cannot be written to or holds Scrivener's backups (the
/// project is a backup, or the folder is named for them), or cannot be named in a
/// string; then the folder remembered for new projects; then the writer's home folder.
pub(crate) fn default_location(
    project: &Path,
    project_is_backup: bool,
    is_writable: &dyn Fn(&Path) -> bool,
    remembered: Option<PathBuf>,
    home: Option<PathBuf>,
) -> Option<PathBuf> {
    let project = project_folder(project);
    let beside = project
        .parent()
        .filter(|beside| !beside.as_os_str().is_empty() && beside.to_str().is_some())
        .map(Path::to_path_buf)
        .filter(|beside| !project_is_backup && !in_backups(beside));
    [beside, remembered, home]
        .into_iter()
        .flatten()
        .find(|folder| is_writable(folder))
}

/// The writer's home folder.
fn home_folder() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

/// The tags the language field offers: every dictionary's, and `proposed` first when no
/// dictionary has it.
fn language_tags(proposed: Option<&str>) -> Vec<String> {
    let mut tags: Vec<String> = crate::spellcheck::dictionary_registry::entries()
        .iter()
        .map(|e| e.id.clone())
        .collect();
    if let Some(proposed) = proposed
        && !tags.iter().any(|t| t == proposed)
    {
        tags.insert(0, proposed.to_string());
    }
    tags
}

/// The interface's language, the manuscript's when the project states none.
fn interface_language() -> Option<String> {
    teksilo::i18n::current_locale().map(|locale| locale.get().to_string())
}

/// The new project's file name by default: the project's name as the writer knows it (a
/// template's own title for a template), without the characters a file name cannot hold.
pub(crate) fn default_name(analysis: &ScrivenerAnalysisDto) -> String {
    let name = match analysis.container {
        ScrivenerContainer::Template if !analysis.template_title.trim().is_empty() => {
            analysis.template_title.as_str()
        }
        _ => analysis.folder_title.as_str(),
    };
    let cleaned = file_name_from(name);
    if cleaned.is_empty() {
        tr!(import_scrivener_default_name()).resolve_now()
    } else {
        cleaned
    }
}

/// `name` as a file name every platform accepts: the characters one of them refuses become
/// spaces, runs of spaces one, and no leading or trailing space or dot.
pub(crate) fn file_name_from(name: &str) -> String {
    let replaced: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => ' ',
            c if c.is_control() => ' ',
            c => c,
        })
        .collect();
    replaced
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches(|c: char| c == '.' || c.is_whitespace())
        .to_string()
}

/// "Last saved by Scrivener 3.3.6 for macOS", from what the manifest says of the program
/// that saved it; the format alone when it names no version.
pub(crate) fn made_with(
    platform: &ScrivenerPlatform,
    version: &str,
    generation: &ScrivenerGeneration,
) -> LocalizedString {
    let version = version.trim().to_string();
    if version.is_empty() {
        return tr!(import_scrivener_made_with_unknown(
            format = format_of(generation)
        ));
    }
    match platform {
        ScrivenerPlatform::Mac => tr!(import_scrivener_made_with_mac(version = version)),
        ScrivenerPlatform::Windows => tr!(import_scrivener_made_with_windows(version = version)),
        ScrivenerPlatform::Linux => tr!(import_scrivener_made_with_linux(version = version)),
        ScrivenerPlatform::Ios => tr!(import_scrivener_made_with_ios(version = version)),
        ScrivenerPlatform::Unknown => tr!(import_scrivener_made_with_version(version = version)),
    }
}

/// A Scrivener format, as the project names it.
fn format_of(generation: &ScrivenerGeneration) -> String {
    match generation {
        ScrivenerGeneration::Format20 => "2.0",
        ScrivenerGeneration::Format15 => "1.5",
        ScrivenerGeneration::Format10 => "1.0",
        ScrivenerGeneration::IosStub => "iOS",
    }
    .to_string()
}

/// The project card of an analysis that reads.
fn card_view(analysis: &ScrivenerAnalysisDto) -> CardView {
    let name = match analysis.container {
        ScrivenerContainer::Template if !analysis.template_title.trim().is_empty() => {
            analysis.template_title.clone()
        }
        _ => analysis.folder_title.clone(),
    };
    // A template's device is the computer of whoever made the template, not the writer's.
    let device = match (&analysis.container, &analysis.computers) {
        (ScrivenerContainer::Template, _) => None,
        (_, ScrivenerComputers::Named { device, .. }) => device.clone(),
        (_, ScrivenerComputers::Unknown) => None,
    };
    let saved_on = analysis
        .modified
        .and_then(|m| chrono::DateTime::from_timestamp(m, 0))
        .map(|date| {
            let date = crate::shared::stamps::iso_stamp(date);
            match device.clone() {
                Some(device) => tr!(import_scrivener_saved_on(date = date, device = device)),
                None => tr!(import_scrivener_saved_on_no_device(date = date)),
            }
        });
    CardView {
        name,
        made_with: made_with(
            &analysis.platform,
            &analysis.writer_version,
            &analysis.generation,
        ),
        saved_on,
        counts: tr!(import_scrivener_card_counts(
            documents = analysis.documents,
            manuscript = analysis.manuscript_documents
        )),
    }
}

/// A refusal as the Project page shows it; a key no table row words still says that the
/// project cannot be imported, with the key behind **Details**.
fn refusal_view(finding: &ScrivenerFinding) -> RefusalView {
    match messages::refusal(finding) {
        Some(worded) => RefusalView {
            title: worded.title,
            body: worded.body,
            retry: worded.retry,
            details: worded.details,
        },
        None => RefusalView {
            title: tr!(import_scrivener_refusal_unknown_title()),
            body: Some(tr!(import_scrivener_refusal_unknown_body())),
            retry: false,
            details: Some(messages::key_of(finding).to_string()),
        },
    }
}

/// Record answer `option` to `question`; `false` when it names no answer.
fn answer(answers: &mut Answers, question: &ScrivenerQuestion, option: usize) -> bool {
    match question {
        ScrivenerQuestion::SeveralProjects { candidates } => match candidates.get(option) {
            Some(candidate) => {
                answers.project = candidate.clone();
                true
            }
            None => false,
        },
        ScrivenerQuestion::SeveralManifests { candidates, .. } => match candidates.get(option) {
            Some(ScrivenerManifestCandidate::Candidate { name, .. }) => {
                answers.manifest = name.clone();
                true
            }
            _ => false,
        },
        ScrivenerQuestion::EnclosingProject { .. } => {
            answers.use_enclosing_project = option == 0;
            option == 0
        }
        ScrivenerQuestion::UpgradedSibling { .. } => {
            answers.upgraded_sibling = match option {
                0 => ScrivenerSiblingChoice::Sibling,
                1 => ScrivenerSiblingChoice::Backup,
                _ => return false,
            };
            true
        }
        ScrivenerQuestion::UnmergedMobileEdits { .. }
        | ScrivenerQuestion::NoTextInCopy { .. }
        | ScrivenerQuestion::Empty => false,
    }
}

/// Forget the answer to `question`.
fn forget_answer(answers: &mut Answers, question: &ScrivenerQuestion) {
    match question {
        ScrivenerQuestion::SeveralProjects { .. } => answers.project.clear(),
        ScrivenerQuestion::SeveralManifests { .. } => answers.manifest.clear(),
        ScrivenerQuestion::EnclosingProject { .. } => answers.use_enclosing_project = false,
        ScrivenerQuestion::UpgradedSibling { .. } => {
            answers.upgraded_sibling = ScrivenerSiblingChoice::Unanswered
        }
        _ => {}
    }
}

/// Which answer of `question` is the chosen one, [`NO_ANSWER`] for none.
fn chosen_answer(answers: &Answers, question: &ScrivenerQuestion) -> usize {
    let found = match question {
        ScrivenerQuestion::SeveralProjects { candidates } => {
            candidates.iter().position(|c| *c == answers.project)
        }
        ScrivenerQuestion::SeveralManifests { candidates, .. } => candidates.iter().position(|c| {
            matches!(c, ScrivenerManifestCandidate::Candidate { name, .. } if *name == answers.manifest)
        }),
        ScrivenerQuestion::EnclosingProject { .. } => answers.use_enclosing_project.then_some(0),
        ScrivenerQuestion::UpgradedSibling { .. } => match answers.upgraded_sibling {
            ScrivenerSiblingChoice::Sibling => Some(0),
            ScrivenerSiblingChoice::Backup => Some(1),
            ScrivenerSiblingChoice::Unanswered => None,
        },
        _ => None,
    };
    found.unwrap_or(NO_ANSWER)
}

/// The answer a question has when evidence singles one out, else `None`: the writer then
/// chooses before anything is read.
pub(crate) fn preselection(question: &ScrivenerQuestion) -> Option<usize> {
    match question {
        ScrivenerQuestion::SeveralManifests {
            candidates,
            preselected: Some(name),
            ..
        } => candidates.iter().position(
            |c| matches!(c, ScrivenerManifestCandidate::Candidate { name: n, .. } if n == name),
        ),
        // A pick inside a project: importing the whole project is the only import there is.
        ScrivenerQuestion::EnclosingProject { .. } => Some(0),
        ScrivenerQuestion::UpgradedSibling {
            preselect_sibling: true,
            ..
        } => Some(0),
        _ => None,
    }
}

/// The choices a plan was made with.
fn planned_choices(structure: &ScrivenerStructure) -> Vec<ScrivenerChoice> {
    match structure {
        ScrivenerStructure::Planned { choices, .. } => choices.clone(),
        ScrivenerStructure::Empty => Vec::new(),
    }
}

/// Whether two choices are about the same thing, so the later replaces the earlier.
fn same_subject(a: &ScrivenerChoice, b: &ScrivenerChoice) -> bool {
    use ScrivenerChoice as C;
    match (a, b) {
        (C::Containers { depth: x, .. }, C::Containers { depth: y, .. }) => x == y,
        (C::Texts { depth: x, .. }, C::Texts { depth: y, .. }) => x == y,
        (C::Root { item: x, .. }, C::Root { item: y, .. }) => x == y,
        (C::Split { item: x, .. }, C::Split { item: y, .. }) => x == y,
        (C::Books { .. }, C::Books { .. }) | (C::Trash { .. }, C::Trash { .. }) => true,
        _ => false,
    }
}

/// Why the plan could not be recomputed, in the writer's words.
fn review_error(error: &ScrivenerReviewError) -> LocalizedString {
    match error {
        ScrivenerReviewError::Unreadable => tr!(import_scrivener_review_error_unreadable()),
        ScrivenerReviewError::DepthNotOffered { .. }
        | ScrivenerReviewError::RootNotOffered { .. } => {
            tr!(import_scrivener_review_error_choice())
        }
        ScrivenerReviewError::Unmappable { .. } => tr!(import_scrivener_review_error_unmappable()),
    }
}

/// The answers a pick offers, in order.
const RUNGS: [ScrivenerContainerRung; 3] = [
    ScrivenerContainerRung::Part,
    ScrivenerContainerRung::Chapter,
    ScrivenerContainerRung::Plain,
];
const TEXT_KINDS: [ScrivenerTextKind; 2] = [ScrivenerTextKind::Scenes, ScrivenerTextKind::Chapters];
const DESTINATIONS: [ScrivenerDestination; 4] = [
    ScrivenerDestination::Notes,
    ScrivenerDestination::StoryBible,
    ScrivenerDestination::Manuscript,
    ScrivenerDestination::LeaveOut,
];
const BOOKS: [ScrivenerBooksChoice; 2] = [ScrivenerBooksChoice::One, ScrivenerBooksChoice::Several];

fn rung_label(rung: &ScrivenerContainerRung) -> LocalizedString {
    match rung {
        ScrivenerContainerRung::Part => tr!(import_scrivener_rung_part()),
        ScrivenerContainerRung::Chapter => tr!(import_scrivener_rung_chapter()),
        ScrivenerContainerRung::Plain => tr!(import_scrivener_rung_plain()),
    }
}

fn text_kind_label(kind: &ScrivenerTextKind) -> LocalizedString {
    match kind {
        ScrivenerTextKind::Scenes => tr!(import_scrivener_texts_scenes()),
        ScrivenerTextKind::Chapters => tr!(import_scrivener_texts_chapters()),
    }
}

fn destination_label(destination: &ScrivenerDestination) -> LocalizedString {
    match destination {
        ScrivenerDestination::Notes => tr!(import_scrivener_dest_notes()),
        ScrivenerDestination::StoryBible => tr!(import_scrivener_dest_story_bible()),
        ScrivenerDestination::Manuscript => tr!(import_scrivener_dest_manuscript()),
        ScrivenerDestination::LeaveOut => tr!(import_scrivener_dest_leave_out()),
    }
}

fn books_label(choice: &ScrivenerBooksChoice) -> LocalizedString {
    match choice {
        ScrivenerBooksChoice::One => tr!(import_scrivener_books_one()),
        ScrivenerBooksChoice::Several => tr!(import_scrivener_books_several()),
    }
}

/// An item a control names, by the last part of its binder path.
fn item_title(item: &ScrivenerItemRef) -> String {
    match item {
        ScrivenerItemRef::Item { binder_path, .. } => {
            binder_path.last().cloned().unwrap_or_default()
        }
        ScrivenerItemRef::Empty => String::new(),
    }
}

fn item_row(item: &ScrivenerItemRef) -> i64 {
    match item {
        ScrivenerItemRef::Item { item, .. } => *item,
        ScrivenerItemRef::Empty => -1,
    }
}

/// The Review page's choices, from the controls the plan offers: the levels of the
/// manuscript, the books, the splits, then the folders beside the manuscript and the
/// Trash, then the prose. A control the plan does not offer is not shown.
pub(crate) fn review_rows(
    controls: &[ScrivenerControl],
    blank_lines: &Signal<bool>,
    ornaments: &Signal<bool>,
) -> Vec<ReviewRow> {
    let heading = |label: LocalizedString| ReviewRow {
        label,
        detail: None,
        control: ReviewControl::Heading,
    };
    let mut structure = Vec::new();
    let mut roots = Vec::new();
    for control in controls {
        match control {
            ScrivenerControl::Containers {
                depth,
                containers,
                detected,
                chosen,
                examples,
                ..
            } => {
                let current = chosen.as_ref().unwrap_or(detected);
                structure.push(ReviewRow {
                    label: tr!(import_scrivener_choice_containers(depth = *depth)),
                    detail: Some(examples_detail(*containers, examples)),
                    control: ReviewControl::Pick {
                        target: PickTarget::Containers(*depth),
                        options: RUNGS.iter().map(rung_label).collect(),
                        selected: Signal::new(Some(
                            RUNGS.iter().position(|r| r == current).unwrap_or(0),
                        )),
                    },
                });
            }
            ScrivenerControl::Texts {
                depth,
                texts,
                detected,
                chosen,
                examples,
                ..
            } => {
                let current = chosen.as_ref().unwrap_or(detected);
                structure.push(ReviewRow {
                    label: tr!(import_scrivener_choice_texts(depth = *depth)),
                    detail: Some(examples_detail(*texts, examples)),
                    control: ReviewControl::Pick {
                        target: PickTarget::Texts(*depth),
                        options: TEXT_KINDS.iter().map(text_kind_label).collect(),
                        selected: Signal::new(Some(
                            TEXT_KINDS.iter().position(|k| k == current).unwrap_or(0),
                        )),
                    },
                });
            }
            ScrivenerControl::Books {
                claimed,
                detected,
                chosen,
            } => {
                let current = chosen.as_ref().unwrap_or(detected);
                structure.push(ReviewRow {
                    label: tr!(import_scrivener_choice_books()),
                    detail: Some(examples_detail(claimed.len() as i64, claimed)),
                    control: ReviewControl::Pick {
                        target: PickTarget::Books,
                        options: BOOKS.iter().map(books_label).collect(),
                        selected: Signal::new(Some(
                            BOOKS.iter().position(|b| b == current).unwrap_or(0),
                        )),
                    },
                });
            }
            ScrivenerControl::Split {
                item,
                headings,
                detected,
                chosen,
            } => structure.push(ReviewRow {
                label: tr!(import_scrivener_choice_split(
                    title = item_title(item),
                    count = *headings
                )),
                detail: None,
                control: ReviewControl::Toggle {
                    target: ToggleTarget::Split(item_row(item)),
                    on: Signal::new(chosen.unwrap_or(*detected)),
                },
            }),
            ScrivenerControl::Root {
                item,
                items,
                detected,
                chosen,
                looks_like_manuscript,
            } => {
                let current = chosen.as_ref().unwrap_or(detected);
                roots.push(ReviewRow {
                    label: lit!(item_title(item)),
                    detail: Some(if *looks_like_manuscript {
                        tr!(import_scrivener_root_items_manuscript(count = *items))
                    } else {
                        tr!(import_scrivener_root_items(count = *items))
                    }),
                    control: ReviewControl::Pick {
                        target: PickTarget::Root(item_row(item)),
                        options: DESTINATIONS.iter().map(destination_label).collect(),
                        selected: Signal::new(Some(
                            DESTINATIONS.iter().position(|d| d == current).unwrap_or(0),
                        )),
                    },
                });
            }
            ScrivenerControl::Trash {
                items,
                with_writing,
                chosen,
            } => roots.push(ReviewRow {
                label: tr!(import_scrivener_choice_trash(count = *items)),
                detail: Some(tr!(import_scrivener_trash_detail(count = *with_writing))),
                control: ReviewControl::Toggle {
                    target: ToggleTarget::Trash,
                    on: Signal::new(*chosen == ScrivenerTrashChoice::Import),
                },
            }),
            ScrivenerControl::Empty => {}
        }
    }
    let mut rows = Vec::new();
    if !structure.is_empty() {
        rows.push(heading(tr!(import_scrivener_choices_structure())));
        rows.extend(structure);
    }
    if !roots.is_empty() {
        rows.push(heading(tr!(import_scrivener_top_level())));
        rows.extend(roots);
    }
    rows.push(heading(tr!(import_scrivener_choices_text())));
    rows.push(ReviewRow {
        label: tr!(import_scrivener_choice_blank_lines()),
        detail: None,
        control: ReviewControl::Toggle {
            target: ToggleTarget::BlankLines,
            on: blank_lines.clone(),
        },
    });
    rows.push(ReviewRow {
        label: tr!(import_scrivener_choice_ornaments()),
        detail: None,
        control: ReviewControl::Toggle {
            target: ToggleTarget::Ornaments,
            on: ornaments.clone(),
        },
    });
    rows
}

/// "4 folders, such as “Chapter 1”": how many items a level holds and the first example.
fn examples_detail(count: i64, examples: &[ScrivenerItemRef]) -> LocalizedString {
    match examples.first().map(item_title).filter(|t| !t.is_empty()) {
        Some(example) => tr!(import_scrivener_choice_count_example(
            count = count,
            example = example
        )),
        None => tr!(import_scrivener_choice_count(count = count)),
    }
}

/// What a planned row becomes, in the binder's own words.
pub(crate) fn becomes(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> LocalizedString {
    match CreateType::of(role, sub_role) {
        Some(kind) => recommendation_label(kind),
        None => match role {
            BinderItemRole::Folder => tr!(create_folder()),
            BinderItemRole::Item => tr!(import_scrivener_becomes_item()),
        },
    }
}

/// The preview's rows: one band per binder, its rows under it, worded.
pub(crate) fn preview_rows(rows: &[ScrivenerPreviewRow]) -> Vec<PreviewRowView> {
    let mut out = Vec::with_capacity(rows.len() + 2);
    let mut occurrences: std::collections::HashMap<(u8, i64), u32> = Default::default();
    for (binder_index, binder) in [ScrivenerBinder::Manuscript, ScrivenerBinder::Notes]
        .iter()
        .enumerate()
    {
        let binder_index = binder_index as u8;
        let in_binder: Vec<&ScrivenerPreviewRow> = rows
            .iter()
            .filter(|r| matches!(r, ScrivenerPreviewRow::Row { binder: b, .. } if b == binder))
            .collect();
        if in_binder.is_empty() {
            continue;
        }
        let key = PreviewKey(out.len() as u32);
        out.push(PreviewRowView {
            key,
            identity: PreviewIdentity {
                binder: binder_index,
                item: -1,
                occurrence: u32::MAX,
            },
            depth: 0,
            band: true,
            title: match binder {
                ScrivenerBinder::Manuscript => tr!(import_scrivener_manuscript_binder()),
                ScrivenerBinder::Notes => tr!(import_scrivener_notes_binder()),
            },
            title_is_placeholder: false,
            becomes: lit!(""),
            in_exports: false,
            trashed: false,
            folded_by_default: false,
        });
        for row in in_binder {
            let ScrivenerPreviewRow::Row {
                depth,
                role,
                sub_role,
                title,
                title_kind,
                exportable,
                activated,
                item,
                ..
            } = row
            else {
                continue;
            };
            let occurrence = occurrences.entry((binder_index, *item)).or_insert(0);
            let identity = PreviewIdentity {
                binder: binder_index,
                item: *item,
                occurrence: *occurrence,
            };
            *occurrence += 1;
            let becomes_label = if *activated {
                becomes(role, sub_role)
            } else {
                tr!(import_scrivener_becomes_trashed())
            };
            let (title, placeholder) = match title_kind {
                ScrivenerTitleKind::Own | ScrivenerTitleKind::ProjectTitle
                    if title.trim().is_empty() =>
                {
                    (becomes(role, sub_role), true)
                }
                ScrivenerTitleKind::Own | ScrivenerTitleKind::ProjectTitle => {
                    (lit!(title.clone()), false)
                }
                ScrivenerTitleKind::NotesOf => {
                    (tr!(import_scrivener_notes_of(title = title.clone())), false)
                }
                ScrivenerTitleKind::ProjectNotes => (tr!(import_scrivener_project_notes()), false),
                ScrivenerTitleKind::Recovered => (tr!(import_scrivener_recovered()), false),
                ScrivenerTitleKind::FromScrivener => {
                    (tr!(import_scrivener_from_scrivener()), false)
                }
                ScrivenerTitleKind::FromScrivenerNote => (import_note_title(title), false),
            };
            let depth = usize::try_from(*depth).unwrap_or(0) + 1;
            let folded_by_default = match binder {
                // The manuscript opens to its chapters, the rest to its folders.
                ScrivenerBinder::Manuscript => depth >= 3,
                ScrivenerBinder::Notes => true,
            };
            out.push(PreviewRowView {
                key: PreviewKey(out.len() as u32),
                identity,
                depth,
                band: false,
                title,
                title_is_placeholder: placeholder,
                becomes: becomes_label,
                in_exports: *exportable && *activated,
                trashed: !*activated,
                folded_by_default,
            });
        }
    }
    out
}

/// One line per kind of finding, in the report groups' order, for the "before you import"
/// strip. A kind that says one sentence per field, collection or note (`line_variant`)
/// gives one line for each, as the report does: the custom metadata of two fields left
/// without a row are two facts, which one line about the first would hide.
pub(crate) fn before_lines(findings: &[&ScrivenerFinding]) -> Vec<LocalizedString> {
    type Identity = (String, Option<String>);
    let identity = |f: &ScrivenerFinding| -> Identity {
        (messages::key_of(f).to_string(), messages::line_variant(f))
    };
    let mut lines: Vec<(messages::Group, Identity)> = Vec::new();
    for finding in findings {
        let found = identity(finding);
        let group = messages::group_of(finding).unwrap_or(messages::Group::LeftBehind);
        if !lines.iter().any(|(_, known)| *known == found) {
            lines.push((group, found));
        }
    }
    lines.sort_by_key(|(group, _)| *group);
    lines
        .iter()
        .filter_map(|(_, line)| {
            let of_line: Vec<&ScrivenerFinding> = findings
                .iter()
                .copied()
                .filter(|f| identity(f) == *line)
                .collect();
            messages::summary_line(&line.0, &of_line)
        })
        .collect()
}

#[cfg(test)]
mod tests;
