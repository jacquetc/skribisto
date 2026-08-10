// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ImportDocumentViewModel` — the Import documents wizard's business logic.
//!
//! Three Stepper steps and two use cases. **Files** collects paths; Next runs
//! `analyze_document_import`. **Review** shows progress while that runs, then the
//! plan as a tree the writer can retype and exclude from. **Destination** is the
//! binder outline target alone — kept off Review so the plan tree has the card.
//! Finish on Destination runs `apply_document_import`. The panel owns the
//! [`teksilo::widgets::Stepper`]; this view-model owns the
//! [`teksilo::widgets::StepperController`] so long-op handlers can jump steps.
//!
//! That shape is the whole point: every importer surveyed while designing this
//! lands its guesses straight in the manuscript and leaves the writer to find the
//! damage. Nothing here reaches the store until the tree on screen is the tree
//! they want.
//!
//! ## Tier 3, and constructor-threaded
//!
//! Per window, created in `shell::windows::build_window` and passed to whoever
//! needs it — **never** `ctx.app_state`. `ImportPlumeViewModel` is registered that
//! way and gets away with it because a Plume import needs no open project at all;
//! this one imports *into* one, so a process-wide slot could only ever answer for
//! whichever window happened to be built first.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

use teksilo::data::{ListModel, TreeDataSource};
use teksilo::prelude::*;
use teksilo::widgets::{MessageBox, MessageBoxButtons, StepperController, Toast, ToastAction};

use frontend::AppContext;
use frontend::commands::{long_operation_commands, undo_redo_commands};
use frontend::common::event::Event;
use frontend::import_management::{
    AnalyzeDocumentImportDto, ApplyDocumentImportDto, ApplyImportRow, ApplyImportRows,
    DocumentImportRow, DocumentImportRows, DropPosition, ImportComment, ImportCommentKind,
    ImportDiagnosticRow, ImportDiagnosticRows, ImportOrphanReason, ImportReply, ImportRowKind,
};

use document_ingest::plan::{PlannedComment, PlannedRow};
use document_ingest::{ImportPlan, ScannerRegistry};
use skribisto_model::CreateType;
use skribisto_model::reconcile::{self, ExistingRow, IncomingRow, RowAction, RowStatus};

use super::long_op::{TrackedOp, event_id, parse_payload, payload_id};
use crate::app_ids::AppIds;
use crate::models::import_plan_source::{ImportPlanSource, PlanRowKey};
use crate::toast_scope::ToastWorkExt;
use crate::widgets::DestinationPicker;

/// Indices into the wizard's [`StepperController`] — keep these in lock-step with
/// the panel's `Stepper` steps (Files → Review → Destination). Analysis progress
/// is shown *inside* Review while `busy`, not as its own indicator step.
pub const STEP_FILES: usize = 0;
pub const STEP_REVIEW: usize = 1;
pub const STEP_DESTINATION: usize = 2;
/// How many steps the import [`StepperController`] owns.
pub const STEP_COUNT: usize = 3;

/// Update-in-place key for the toast the *apply* raises. Work-scoped (see
/// [`crate::toast_scope`]) rather than a bare static: two windows on two
/// different projects must not share one entry.
const IMPORT_TOAST_ID: &str = "import.document";

/// How long the "imported — Undo" offer stays up.
///
/// The undo entry itself is **not** cleared when it lapses: an import is an
/// ordinary undoable edit like a comment delete (`view_models::comments`), not
/// Empty Trash. Dropping the project's undo history because a toast timed out
/// would be a far larger act than the one the toast was offering to reverse.
const UNDO_GRACE: Duration = Duration::from_secs(12);

/// What a row may be retyped to.
///
/// Narrower than the whole `CreateType` vocabulary on purpose: `EndOfBook` marks a
/// book's end rather than holding imported material, and the two folder types are
/// organisational containers a document's headings never mean. Everything a
/// heading can plausibly be is here.
pub const ROW_TYPES: &[CreateType] = &[
    CreateType::Book,
    CreateType::Part,
    CreateType::Chapter,
    CreateType::Scene,
    CreateType::Note,
    CreateType::Paratext,
];

/// What a heading *level* may be mapped to — the containment ladder, and only it.
/// `document_ingest`'s own inference walks the same four.
pub const LEVEL_TYPES: &[CreateType] = &[
    CreateType::Book,
    CreateType::Part,
    CreateType::Chapter,
    CreateType::Scene,
];

/// `ImportDiagnostic::IllegalCombination`'s key, as `document_ingest` spells it.
///
/// Named rather than inlined because it is matched in two directions — the row-scoped
/// entries carrying it are cleared and rebuilt on every retype — and a typo in either
/// would silently stop marking the rows it is there to mark.
const ILLEGAL_COMBINATION: &str = "illegal-combination";

/// A diagnostic, translated at the boundary rather than carried as a sentence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub key: String,
    pub severity: String,
    pub path: String,
    pub detail: String,
    pub count: i64,
    /// The row it belongs to, or `None` for one about the import as a whole.
    pub row: Option<PlanRowKey>,
}

impl Diagnostic {
    /// The sentence the writer reads, in their own locale.
    ///
    /// One `tr!` arm per `ImportDiagnostic::key()`, so a variant added to
    /// `document_ingest` without a translation here falls through to a visible
    /// fallback rather than disappearing — and every arm is compile-validated
    /// against `en-US.ftl` like any other key.
    ///
    /// `title` and `kind` are passed in by the caller, read off the row this
    /// belongs to: a row-scoped diagnostic names its row, and the row already
    /// knows what it is called and what type it resolved to. Sending them over
    /// the wire as well would only give them a second place to disagree.
    pub fn message(&self, title: &str, kind: Option<CreateType>) -> LocalizedString {
        let path = self.short_path();
        let detail = self.detail.clone();
        let count = self.count;
        match self.key.as_str() {
            "file-unreadable" => tr!(import_diagnostic_file_unreadable(
                path = path,
                detail = detail
            )),
            "lossy-decode" => tr!(import_diagnostic_lossy_decode(path = path, count = count)),
            "decoded-from-bom" => tr!(import_diagnostic_decoded_from_bom(
                path = path,
                detail = detail
            )),
            "empty-file" => tr!(import_diagnostic_empty_file(path = path)),
            "no-headings" => tr!(import_diagnostic_no_headings(path = path)),
            "unsupported-format" => tr!(import_diagnostic_unsupported_format(
                path = path,
                detail = detail
            )),
            "front-matter-not-flat" => tr!(import_diagnostic_front_matter_not_flat(
                path = path,
                detail = detail
            )),
            "footnotes-degraded" => tr!(import_diagnostic_footnotes_degraded(
                path = path,
                count = count
            )),
            "raw-html-dropped" => tr!(import_diagnostic_raw_html_dropped(
                path = path,
                count = count
            )),
            "nested-break-dropped" => tr!(import_diagnostic_nested_break_dropped(
                path = path,
                count = count
            )),
            "image-not-ingested" => tr!(import_diagnostic_image_not_ingested(
                path = path,
                detail = detail
            )),
            "duplicate-title" => tr!(import_diagnostic_duplicate_title(
                title = detail,
                count = count
            )),
            "heading-level-jump" => tr!(import_diagnostic_heading_level_jump(
                title = title.to_string(),
                from = detail,
                to = count
            )),
            "illegal-combination" => tr!(import_diagnostic_illegal_combination(
                title = title.to_string(),
                kind = kind
                    .map(|k| crate::binder::create_labels::recommendation_label(k).resolve_now())
                    .unwrap_or_default()
            )),
            "tracked-changes-flattened" => tr!(import_diagnostic_tracked_changes_flattened(
                path = path,
                count = count
            )),
            "text-box-dropped" => tr!(import_diagnostic_text_box_dropped(
                path = path,
                count = count
            )),
            "embedded-object-dropped" => tr!(import_diagnostic_embedded_object_dropped(
                path = path,
                count = count
            )),
            "field-flattened" => tr!(import_diagnostic_field_flattened(
                path = path,
                count = count
            )),
            "unknown-style-level" => tr!(import_diagnostic_unknown_style_level(
                path = path,
                detail = detail
            )),
            "comment-unanchored" => tr!(import_diagnostic_comment_unanchored(
                path = path,
                detail = detail
            )),
            "comment-replies-flattened" => tr!(import_diagnostic_comment_replies_flattened(
                path = path,
                count = count
            )),
            // Not silence: a diagnostic nobody translated is still a diagnostic,
            // and the writer would rather read a raw key than lose the warning.
            other => lit!(format!("{other}: {} ({})", self.detail, self.path)),
        }
    }

    /// Just the file's own name. A column of absolute paths would bury the one
    /// part that distinguishes them — the same reasoning as the file list's.
    fn short_path(&self) -> String {
        std::path::Path::new(&self.path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| self.path.clone())
    }

    pub fn is_error(&self) -> bool {
        self.severity == "error"
    }

    pub fn is_warning(&self) -> bool {
        self.severity == "warning"
    }
}

#[derive(Clone)]
pub struct ImportDocumentViewModel {
    app_ctx: Rc<AppContext>,
    ids: AppIds,

    /// Files chosen so far, in the order they will be imported. A data model
    /// rather than a `Signal<Vec<_>>` because the panel renders it with a
    /// `ListView`, and because reorder and removal are exactly what it is for.
    /// The writer fixes order here; the analyser sorts names naturally, but
    /// `interlude.md` carries none.
    files: ListModel<PathBuf>,
    /// How many files are chosen, mirrored as a signal.
    ///
    /// `ListModel` reports changes through `observe_changes`, not a version
    /// signal, so a button that must grey out on an empty list needs its own
    /// reactive count. Written by every method that touches `files`.
    file_count: Signal<usize>,
    /// Drives the panel's `Stepper` (active step, Back/Next, indicator strip).
    /// Owned here rather than on the panel so long-op handlers can
    /// `go_to(Review)` / `go_to(Files)` after analysis lands or is abandoned —
    /// the panel is torn down around this view-model, not the other way round.
    controller: StepperController,

    /// The analysed plan, as a tree.
    plan: ImportPlanSource,
    diagnostics: Signal<Vec<Diagnostic>>,
    /// The same diagnostics, already turned into the writer's sentences, as the
    /// list model the strip binds.
    ///
    /// Owned here and written eagerly by [`Self::set_diagnostics`] rather than
    /// mirrored in the view: the panel used to fill its own `ListModel` from a
    /// side effect inside a mapped signal, read by a zero-width label that
    /// existed only to force that map to run. Layer B owns the data; the view
    /// binds it.
    diagnostic_rows: ListModel<(String, LocalizedString)>,

    /// Heading level → type. Editing one entry retypes every row that came from
    /// that level and has not been individually pinned.
    level_rules: Signal<Vec<(u8, CreateType)>>,
    /// Rows the writer retyped by hand. A level rule must not silently undo a
    /// per-row decision, so a pinned row is left alone when the rule changes.
    pinned: Signal<Vec<PlanRowKey>>,
    /// Which heading level each row came from, parallel to the plan's rows.
    row_levels: Signal<Vec<u8>>,

    destination: DestinationPicker,

    /// The analyse operation in flight, if any — set on start, cleared on
    /// completion, cancellation or failure. Filters the shared long-operation
    /// event stream down to this one job.
    ///
    /// A [`TrackedOp`], not a bare id: it pins the Work the analysis started
    /// for in the same call that records the op id, so a project switch in this
    /// window mid-analysis cannot make the completion toast address the wrong
    /// project. See that type's doc for the four times this was hand-fixed.
    active: Rc<RefCell<Option<TrackedOp>>>,
    busy: Signal<bool>,
    /// 0.0–1.0, for the analysing step's bar.
    progress: Signal<f32>,
    /// The backend's own progress line — which file it is reading.
    progress_message: Signal<String>,

    /// The plan lined up against what the destination already holds, rebuilt whenever the
    /// destination changes.
    ///
    /// Empty until the writer leaves the destination step, because it *cannot* be computed
    /// before then: matching a returning file against the project is a question about a
    /// particular subtree, and there is no answer until one is chosen.
    merge: Rc<RefCell<Vec<MergeRowView>>>,
    /// What the writer decided for each merge row, keyed by the row's own durable key.
    ///
    /// Keyed by [`MergeRowKey`] and never by position: going Back and choosing a different
    /// destination rebuilds the whole sequence, and decisions keyed by an index would be
    /// silently reassigned to other rows.
    merge_actions: Rc<RefCell<HashMap<MergeRowKey, RowAction>>>,
    /// Bumped whenever the merge is rebuilt, so the panel's table re-sources.
    merge_version: Signal<u64>,
}

/// A merge row's durable identity — what a decision is remembered against.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MergeRowKey {
    /// A row the project holds: its `BinderItem.uid`.
    Current(uuid::Uuid),
    /// A row only the returning file has: its key in the plan.
    Incoming(PlanRowKey),
}

/// One line of the merge, as the reconcile step shows it.
#[derive(Clone, Debug)]
pub struct MergeRowView {
    pub key: MergeRowKey,
    /// Depth in the current tree, so the first column can draw a real outline.
    pub indent: i64,
    /// The project's row: its title and the item it is. `None` when the editor added this row.
    pub current_title: Option<String>,
    pub current_item_id: Option<u64>,
    /// The returning file's row. `None` when the file no longer has it.
    pub incoming_title: Option<String>,
    pub incoming_key: Option<PlanRowKey>,
    pub status: RowStatus,
    /// The editor moved it — see [`skribisto_model::reconcile::MergeRow::moved`].
    pub moved: bool,
    pub actions: Vec<RowAction>,
}

impl MergeRowView {
    /// Whether a side-by-side comparison is meaningful: both sides have prose to show.
    pub fn can_compare(&self) -> bool {
        self.current_item_id.is_some() && self.incoming_key.is_some()
    }
}

impl ImportDocumentViewModel {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        let destination = DestinationPicker::new(app_ctx.clone(), ids.work_id.clone());
        Self {
            app_ctx,
            ids,
            merge: Rc::new(RefCell::new(Vec::new())),
            merge_actions: Rc::new(RefCell::new(HashMap::new())),
            merge_version: Signal::new(0),
            files: ListModel::new(),
            file_count: Signal::new(0),
            controller: StepperController::new(STEP_COUNT),
            plan: ImportPlanSource::empty(),
            diagnostics: Signal::new(Vec::new()),
            diagnostic_rows: ListModel::new(),
            level_rules: Signal::new(Vec::new()),
            pinned: Signal::new(Vec::new()),
            row_levels: Signal::new(Vec::new()),
            destination,
            active: Rc::new(RefCell::new(None)),
            busy: Signal::new(false),
            progress: Signal::new(0.0),
            progress_message: Signal::new(String::new()),
        }
    }

    // ── what the view binds to ──────────────────────────────────────────────

    pub fn files(&self) -> ListModel<PathBuf> {
        self.files.clone()
    }
    pub fn file_count(&self) -> Signal<usize> {
        self.file_count.clone()
    }

    /// The chosen files, in order — for the analyse call and for tests.
    pub fn file_paths(&self) -> Vec<PathBuf> {
        (0..self.files.len())
            .filter_map(|i| self.files.with_item(i, Clone::clone))
            .collect()
    }
    /// Active wizard step — the `Stepper`'s `Switcher` and any test that asks
    /// "which page is on screen" both read this.
    pub fn step(&self) -> Signal<usize> {
        self.controller.current_step_signal()
    }

    /// The controller the panel's `Stepper` is driven by — also the handle
    /// long-op handlers use to jump to Review / back to Files.
    pub fn controller(&self) -> StepperController {
        self.controller.clone()
    }

    pub fn plan(&self) -> ImportPlanSource {
        self.plan.clone()
    }
    pub fn diagnostics(&self) -> Signal<Vec<Diagnostic>> {
        self.diagnostics.clone()
    }
    pub fn level_rules(&self) -> Signal<Vec<(u8, CreateType)>> {
        self.level_rules.clone()
    }
    /// `Work.unique_id`, or `None` when there is no project or it has never been saved
    /// — the key this project's remembered import destination is filed under.
    pub fn work_uid(&self) -> Option<String> {
        let work_id = self.ids.work_id.get()?;
        let work = frontend::commands::work_commands::get_work(&self.app_ctx, &work_id)
            .ok()
            .flatten()?;
        crate::models::uid_is_usable(&work.unique_id).then_some(work.unique_id)
    }

    /// The destination the writer settled on, as the durable key worth remembering.
    pub fn chosen_destination_key(&self) -> Option<crate::models::BinderTreeKey> {
        self.destination.selected_key()
    }

    /// Open the wizard already pointing at `key`.
    ///
    /// Forwards to the picker, which holds the request until its tree can satisfy it —
    /// see `DestinationPicker::preselect`. Used by the binder's "Import here…", where
    /// the writer has already said where by right-clicking a row.
    pub fn preselect_destination(&self, key: crate::models::BinderTreeKey) {
        self.destination.preselect(key);
    }

    pub fn destination(&self) -> DestinationPicker {
        self.destination.clone()
    }
    /// The Work this import targets — what a toast scopes itself to, so a second
    /// window on another project never shows it.
    pub fn work_id(&self) -> Option<u64> {
        self.ids.work_id.get()
    }
    pub fn busy(&self) -> Signal<bool> {
        self.busy.clone()
    }
    pub fn progress(&self) -> Signal<f32> {
        self.progress.clone()
    }
    pub fn progress_message(&self) -> Signal<String> {
        self.progress_message.clone()
    }
    /// The in-flight analysis's id, or `None`. For tests and for the panel's
    /// "is there something to cancel" gate.
    pub fn active_op_id(&self) -> Option<String> {
        self.active
            .borrow()
            .as_ref()
            .map(|op| op.op_id().to_string())
    }

    /// Everything the writer chose, cleared — called when the panel opens, so a
    /// previous import's files and plan never appear under a fresh one.
    pub fn reset(&self) {
        self.files.clear();
        self.file_count.set(0);
        self.controller.reset();
        self.plan.set_plan(&ImportPlan::default());
        self.diagnostics.set(Vec::new());
        self.refill_diagnostic_rows();
        self.level_rules.set(Vec::new());
        self.pinned.set(Vec::new());
        self.row_levels.set(Vec::new());
        *self.active.borrow_mut() = None;
        self.busy.set(false);
        self.progress.set(0.0);
        self.progress_message.set(String::new());
    }

    // ── step one: the files ─────────────────────────────────────────────────

    /// Extensions the file picker and the drop zone should accept, asked of the
    /// scanners themselves so the two can never offer something no scanner reads.
    pub fn accepted_extensions() -> Vec<String> {
        ScannerRegistry::with_builtin_scanners()
            .accepted_extensions()
            .into_iter()
            .map(str::to_string)
            .collect()
    }

    /// Add files, keeping the order they arrived in and dropping duplicates.
    ///
    /// A drop or a multi-select hands over an arbitrary order; the analyser sorts
    /// naturally afterwards, so this only has to preserve what the writer can see
    /// and let them fix it.
    pub fn add_files(&self, paths: impl IntoIterator<Item = PathBuf>) {
        let mut existing = self.file_paths();
        for path in paths {
            if !existing.contains(&path) {
                self.files.push(path.clone());
                existing.push(path);
            }
        }
        self.file_count.set(self.files.len());
    }

    pub fn remove_file(&self, index: usize) {
        if index < self.files.len() {
            self.files.remove(index);
            self.file_count.set(self.files.len());
        }
    }

    /// Move one file up or down. Out-of-range moves are no-ops rather than
    /// errors: the buttons at either end of the list are simply inert.
    pub fn move_file(&self, index: usize, delta: isize) {
        let target = index as isize + delta;
        if index < self.files.len() && target >= 0 && (target as usize) < self.files.len() {
            self.files.move_item(index, target as usize);
        }
    }

    pub fn can_analyse(&self) -> bool {
        if cfg!(feature = "mocks") {
            // See [`Self::can_analyse_signal`] — file list is optional under mocks.
            return !self.busy.get();
        }
        !self.files.is_empty() && self.ids.work_id.get().is_some() && !self.busy.get()
    }

    /// Files → Review.
    ///
    /// Real builds start `analyze_document_import`. With `--features mocks` the
    /// long op is skipped and a small plan is planted so the Stepper's Next can
    /// walk Review and Destination for layout / automation without a file drop
    /// (the bridge cannot synthesize one).
    pub fn try_advance_from_files(&self) -> bool {
        if cfg!(feature = "mocks") {
            self.seed_mock_review_plan();
            return true;
        }
        self.start_analysis().is_ok()
    }

    /// Plant a tiny review plan without reading disk. Used by the `mocks` Next
    /// bypass and available to headless layout tests.
    ///
    /// Does **not** jump the Stepper: Files' `validate_on_next` returns true and
    /// the footer then runs `next()` once into Review. Calling
    /// [`on_plan_ready`](Self::on_plan_ready) here would `go_to(Review)` *and*
    /// then `next()`, landing on Destination in one click.
    pub fn seed_mock_review_plan(&self) {
        use document_ingest::plan::PlannedRow;
        let plan = ImportPlan {
            rows: vec![
                PlannedRow {
                    indent: 0,
                    create_type: CreateType::Book,
                    title: "Mock Book".into(),
                    stripped_ordinal: None,
                    djot: "Opening.".into(),
                    scene_breaks: 0,
                    word_count: 1,
                    origin: "mock.md".into(),
                    included: true,
                    comments: Vec::new(),
                    source_uid_tag: None,
                    source_digest: None,
                    diagnostics: Vec::new(),
                },
                PlannedRow {
                    indent: 1,
                    create_type: CreateType::Chapter,
                    title: "Mock Chapter".into(),
                    stripped_ordinal: None,
                    djot: "Prose.".into(),
                    scene_breaks: 1,
                    word_count: 1,
                    origin: "mock.md".into(),
                    included: true,
                    comments: Vec::new(),
                    source_uid_tag: None,
                    source_digest: None,
                    diagnostics: Vec::new(),
                },
                PlannedRow {
                    indent: 2,
                    create_type: CreateType::Scene,
                    title: "Mock Scene".into(),
                    stripped_ordinal: None,
                    djot: "More prose.".into(),
                    scene_breaks: 0,
                    word_count: 2,
                    origin: "mock.md".into(),
                    included: true,
                    comments: Vec::new(),
                    source_uid_tag: None,
                    source_digest: None,
                    diagnostics: Vec::new(),
                },
            ],
            diagnostics: Vec::new(),
        };
        self.install_plan(
            &plan,
            vec![1, 2, 3],
            vec![
                (1, CreateType::Book),
                (2, CreateType::Chapter),
                (3, CreateType::Scene),
            ],
        );
        // Two of the fourteen things an import can have to admit to. Seeded so the
        // diagnostics strip — a third of the review step, and the reason the
        // feature exists — is actually on screen in a mocks build; without them it
        // is a surface no mock run can ever see.
        self.set_diagnostics(vec![
            Diagnostic {
                key: "footnotes-degraded".into(),
                severity: "warning".into(),
                path: "mock.md".into(),
                detail: String::new(),
                count: 3,
                row: None,
            },
            Diagnostic {
                key: "no-headings".into(),
                severity: "info".into(),
                path: "mock-notes.md".into(),
                detail: String::new(),
                count: 0,
                row: None,
            },
        ]);
    }

    /// Start the read-only analysis. Returns the long-operation id.
    ///
    /// Nothing is written; the result comes back through the long-op event stream
    /// and lands in [`Self::on_plan_ready`].
    pub fn start_analysis(&self) -> anyhow::Result<String> {
        let work_id = self
            .ids
            .work_id
            .get()
            .ok_or_else(|| anyhow::anyhow!("import: no project is open"))?;

        // The destination, verbatim — never a depth or a kind worked out here.
        //
        // This used to send `(Book, 0)` whatever the writer had pointed at, so an
        // import into a chapter arrived as a top-level book spliced between that
        // chapter and its own scenes, which then re-parented onto it. What a
        // destination *means* is `binder_ordering`'s question, and both halves of the
        // use case now ask it there; the wizard's job is only to say where the writer
        // pointed.
        let destination = self.destination.selected();
        let dto = AnalyzeDocumentImportDto {
            work_id,
            binder_id: destination.as_ref().map(|d| d.binder_id).unwrap_or(0),
            source_paths: self
                .file_paths()
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect(),
            anchor_item_id: destination
                .as_ref()
                .and_then(|d| d.anchor_item_id)
                .unwrap_or(0),
            drop_position: destination
                .as_ref()
                .map(|d| to_dto_position(d.position.clone()))
                .unwrap_or(DropPosition::Into),
        };
        let op = frontend::commands::import_management_commands::analyze_document_import(
            &self.app_ctx,
            &dto,
        )?;
        *self.active.borrow_mut() = Some(TrackedOp::start(&self.ids, op.clone()));
        self.busy.set(true);
        self.progress.set(0.0);
        self.progress_message.set(String::new());
        // Does **not** advance the Stepper: Files' `validate_on_next` calls this,
        // returns `true`, and the footer then runs `controller.next()` once into
        // Review (which shows the progress UI while `busy`).
        Ok(op)
    }

    /// Ask the running analysis to stop. The backend halts at its next
    /// checkpoint and emits `Cancelled`, which [`Self::on_long_op_cancelled`]
    /// turns back into the file step.
    pub fn cancel_analysis(&self) {
        if let Some(op) = self.active.borrow().as_ref() {
            long_operation_commands::cancel_operation(&self.app_ctx, op.op_id());
        }
    }

    // ── the long-operation event stream ─────────────────────────────────────
    // Every background job in the app reports through the same four events, so
    // each handler first checks the id against the analysis this wizard started.
    // An export finishing elsewhere must not move this wizard's step.

    /// Is `event` about the analysis this view-model started?
    fn is_mine(&self, event: &Event) -> bool {
        let Some(id) = event_id(event) else {
            return false;
        };
        self.active
            .borrow()
            .as_ref()
            .is_some_and(|op| op.matches(&id))
    }

    pub fn on_long_op_progress(&self, _ctx: &mut EventContext, event: &Event) {
        let Some(payload) = parse_payload(event) else {
            return;
        };
        if self
            .active
            .borrow()
            .as_ref()
            .is_none_or(|op| payload_id(&payload) != Some(op.op_id()))
        {
            return;
        }
        let percent = payload
            .get("percentage")
            .and_then(|p| p.as_f64())
            .unwrap_or(0.0) as f32;
        self.progress.set((percent / 100.0).clamp(0.0, 1.0));
        self.progress_message.set(
            payload
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("")
                .to_string(),
        );
    }

    /// The analysis finished: read its plan out of the operation manager and
    /// move the wizard to the review step.
    pub fn on_long_op_completed(&self, ctx: &mut EventContext, event: &Event) {
        if !self.is_mine(event) {
            return;
        }
        let op_id = self.active_op_id().unwrap_or_default();
        *self.active.borrow_mut() = None;

        match frontend::commands::import_management_commands::get_analyze_document_import_result(
            &self.app_ctx,
            &op_id,
        ) {
            Ok(Some(dto)) => {
                let (plan, levels, diagnostics) = plan_from_dto(&dto.rows, &dto.diagnostics);
                let rules = infer_level_rules(&plan, &levels);
                self.on_plan_ready(&plan, levels, rules);
                self.set_diagnostics(diagnostics);
            }
            // Completed with nothing recoverable, or the result could not be
            // read back. Either way the wizard must not sit on a spinner
            // forever: return to the files, and say why.
            Ok(None) | Err(_) => {
                self.abandon_analysis();
                ctx.show_toast(
                    Toast::error(tr!(import_document_analyse_failed()))
                        .scoped_id(IMPORT_TOAST_ID, self.work_id())
                        .target_work(self.work_id()),
                );
            }
        }
    }

    pub fn on_long_op_cancelled(&self, _ctx: &mut EventContext, event: &Event) {
        if !self.is_mine(event) {
            return;
        }
        // No toast: the writer is looking at the wizard and just pressed
        // Cancel on it. Returning to their file list *is* the acknowledgement.
        self.abandon_analysis();
    }

    pub fn on_long_op_failed(&self, ctx: &mut EventContext, event: &Event) {
        if !self.is_mine(event) {
            return;
        }
        let detail = parse_payload(event)
            .and_then(|p| p.get("error").and_then(|e| e.as_str()).map(str::to_string))
            .unwrap_or_default();
        self.abandon_analysis();

        let mut toast = Toast::error(tr!(import_document_analyse_failed()))
            .scoped_id(IMPORT_TOAST_ID, self.work_id())
            .target_work(self.work_id());
        if !detail.is_empty() {
            toast = toast.action(ToastAction::primary(
                tr!(import_document_details()),
                move |c| {
                    MessageBox::warning(tr!(import_document_analyse_failed()))
                        .text(lit!(detail.clone()))
                        .buttons(MessageBoxButtons::Ok)
                        .present(c);
                },
            ));
        }
        ctx.show_toast(toast);
    }

    /// Stop waiting and put the writer back on their file list, with whatever
    /// they had chosen intact.
    fn abandon_analysis(&self) {
        *self.active.borrow_mut() = None;
        self.busy.set(false);
        self.progress.set(0.0);
        self.progress_message.set(String::new());
        // Clear busy *before* leaving Review so the panel's "left Review while
        // busy → cancel" effect does not re-enter `cancel_analysis`.
        self.controller.go_to(STEP_FILES);
    }

    // ── step two: the plan ──────────────────────────────────────────────────

    /// Install plan state without moving the Stepper.
    fn install_plan(&self, plan: &ImportPlan, levels: Vec<u8>, rules: Vec<(u8, CreateType)>) {
        self.plan.set_plan(plan);
        self.row_levels.set(levels);
        self.level_rules.set(rules);
        self.pinned.set(Vec::new());
        self.diagnostics.set(Vec::new());
        self.refill_diagnostic_rows();
        *self.active.borrow_mut() = None;
        // Busy off before any step jump — see `abandon_analysis`.
        self.busy.set(false);
        self.progress.set(0.0);
        self.progress_message.set(String::new());
    }

    /// Take a freshly analysed plan and put the writer on the review step.
    ///
    /// Already on Review when analysis was started via Next (Files → Review);
    /// `go_to` still runs so a test that only calls this lands on the right page.
    /// The mocks Files→Review path uses [`seed_mock_review_plan`] instead, so the
    /// footer's single `next()` is the only advance.
    pub fn on_plan_ready(&self, plan: &ImportPlan, levels: Vec<u8>, rules: Vec<(u8, CreateType)>) {
        self.install_plan(plan, levels, rules);
        self.controller.go_to(STEP_REVIEW);
    }

    pub fn set_diagnostics(&self, diagnostics: Vec<Diagnostic>) {
        self.diagnostics.set(diagnostics);
        // The analyser's own illegal-combination entries are recomputed here too, so
        // there is exactly one rule deciding which rows are marked — this one.
        // `recheck_types` writes `diagnostics` again and refills the rows.
        self.recheck_types();
    }

    /// The sentences the diagnostics strip lists — bind this, do not mirror it.
    pub fn diagnostic_rows(&self) -> ListModel<(String, LocalizedString)> {
        self.diagnostic_rows.clone()
    }

    /// Re-render [`Self::diagnostic_messages`] into the bound list model.
    ///
    /// Every write to `diagnostics` goes through here, so the list can never
    /// disagree with the headline that counts the same entries.
    fn refill_diagnostic_rows(&self) {
        self.diagnostic_rows.replace_all(self.diagnostic_messages());
    }

    /// Every diagnostic, worst first, already turned into the writer's sentence.
    ///
    /// Sorted by severity because the list is short and read top-down: a file
    /// that could not be opened at all must not sit below three notes about
    /// footnotes. Stable within a severity, so the file order the writer chose
    /// survives.
    pub fn diagnostic_messages(&self) -> Vec<(String, LocalizedString)> {
        let mut all = self.diagnostics.get();
        all.sort_by_key(|d| match d.severity.as_str() {
            "error" => 0,
            "warning" => 1,
            _ => 2,
        });
        all.into_iter()
            .map(|d| {
                let (title, kind) = self.row_context(d.row);
                let severity = d.severity.clone();
                (severity, d.message(&title, kind))
            })
            .collect()
    }

    /// How many diagnostics name this row — what the review tree's marker column
    /// shows, and what its tooltip counts.
    pub fn diagnostics_for_row(&self, key: PlanRowKey) -> Vec<Diagnostic> {
        self.diagnostics
            .get()
            .into_iter()
            .filter(|d| d.row == Some(key))
            .collect()
    }

    /// `(errors, warnings)`. Drives the strip's own headline, and nothing else:
    /// an import with neither shows no strip at all rather than an empty box
    /// announcing that nothing is wrong.
    pub fn diagnostic_counts(&self) -> (usize, usize) {
        let all = self.diagnostics.get();
        (
            all.iter().filter(|d| d.is_error()).count(),
            // Everything that is not an error, not only the `warning` severity:
            // the headline counts what the list below it shows, and an `info`
            // entry is a line in that list. Counting warnings alone said "1
            // thing to know" over three visible sentences.
            all.iter().filter(|d| !d.is_error()).count(),
        )
    }

    /// The title and type of the row a diagnostic names, for its sentence.
    fn row_context(&self, key: Option<PlanRowKey>) -> (String, Option<CreateType>) {
        let Some(key) = key else {
            return (String::new(), None);
        };
        let title = self.plan.row(key).map(|r| r.title).unwrap_or_default();
        (title, self.plan.type_of(key))
    }

    /// Retype one row by hand, and remember that the writer did.
    ///
    /// The pin is what stops a later level-rule change from silently reverting a
    /// deliberate decision — the failure mode that makes a bulk rule and a
    /// per-row override unusable together.
    pub fn retype_row(&self, key: PlanRowKey, kind: CreateType) {
        self.plan.set_type(key, kind);
        let mut pinned = self.pinned.get();
        if !pinned.contains(&key) {
            pinned.push(key);
            self.pinned.set(pinned);
        }
        self.recheck_types();
    }

    pub fn is_pinned(&self, key: PlanRowKey) -> bool {
        self.pinned.get().contains(&key)
    }

    /// Insert a container above every analysed row (indent 0), and push the rest
    /// one level deeper.
    ///
    /// The case this exists for: several Markdown files that are chapters of one
    /// book, with no `# Book` heading anywhere. The writer adds the Book here;
    /// apply then creates it as the parent of everything that was analysed.
    ///
    /// Defaults to [`CreateType::Book`] with the same default title Create uses.
    /// The plan tree's type combo can retype it afterwards.
    pub fn add_top_level_header(&self) {
        self.add_top_level_header_as(CreateType::Book);
    }

    /// As [`Self::add_top_level_header`], with an explicit container type.
    pub fn add_top_level_header_as(&self, kind: CreateType) {
        if self.plan.is_empty() {
            return;
        }
        let title = crate::binder::create_labels::default_title(kind).resolve_now();
        self.plan.prepend_root(kind, title);
        // Synthetic root is not from a document heading — level 0 never matches
        // a level-rule retype. Existing rows shift one index.
        let mut levels = self.row_levels.get();
        levels.insert(0, 0);
        self.row_levels.set(levels);
        let pinned: Vec<PlanRowKey> = self
            .pinned
            .get()
            .into_iter()
            .map(|k| PlanRowKey(k.0.saturating_add(1)))
            .collect();
        self.pinned.set(pinned);
        self.recheck_types();
    }

    /// Map a heading level to a type, retyping every row that came from it —
    /// except the ones the writer has already retyped themselves.
    ///
    /// This is the affordance that makes a two-hundred-chapter import survivable:
    /// one change instead of two hundred.
    pub fn set_level_rule(&self, level: u8, kind: CreateType) {
        let mut rules = self.level_rules.get();
        match rules.iter_mut().find(|(l, _)| *l == level) {
            Some(entry) => entry.1 = kind,
            None => {
                rules.push((level, kind));
                rules.sort_by_key(|(l, _)| *l);
            }
        }
        self.level_rules.set(rules);

        let levels = self.row_levels.get();
        let pinned = self.pinned.get();
        for (index, row_level) in levels.iter().enumerate() {
            let key = PlanRowKey(index as u32);
            if *row_level == level && !pinned.contains(&key) {
                self.plan.set_type(key, kind);
            }
        }
        self.recheck_types();
    }

    // ── what apply would refuse ─────────────────────────────────────────────

    /// Re-derive the "this row's type cannot hold its prose" diagnostic against the
    /// types the rows carry **now**.
    ///
    /// `IllegalCombination` is computed once, inside `build_plan`, from the types the
    /// *analyser* inferred. But the review step exists precisely so the writer can
    /// change those types — and retyping a row that carries prose into a Book, a Part
    /// or a folder makes an import that `apply_document_import` refuses outright,
    /// aborting the whole transaction. Before this, the strip went on reporting the
    /// state the plan arrived in, no marker appeared on the offending row, and Import
    /// stayed enabled: the writer pressed it and lost the lot.
    ///
    /// Row-scoped `illegal-combination` entries are rebuilt wholesale rather than
    /// patched, so a row retyped *back* to something legal loses its marker too.
    fn recheck_types(&self) {
        let mut diagnostics: Vec<Diagnostic> = self
            .diagnostics
            .get()
            .into_iter()
            .filter(|d| !(d.key == ILLEGAL_COMBINATION && d.row.is_some()))
            .collect();

        for key in self.plan.keys_in_order() {
            if self.holds_its_prose(key) {
                continue;
            }
            diagnostics.push(Diagnostic {
                key: ILLEGAL_COMBINATION.to_string(),
                severity: "warning".to_string(),
                path: String::new(),
                detail: String::new(),
                count: 0,
                row: Some(key),
            });
        }
        self.diagnostics.set(diagnostics);
        self.refill_diagnostic_rows();
    }

    /// False when this row carries prose that its current type cannot store.
    ///
    /// Asks `skribisto_model` rather than deciding: the constraint matrix is the one
    /// authority, and `apply_document_import` asks it the same question at write time
    /// (`prose_role_for`). A second opinion here would only tell the writer the import
    /// was fine and let the backend refuse it.
    fn holds_its_prose(&self, key: PlanRowKey) -> bool {
        let Some(row) = self.plan.row(key) else {
            return true;
        };
        if row.djot.trim().is_empty() {
            return true;
        }
        let Some(kind) = self.plan.type_of(key) else {
            return true;
        };
        let (role, sub_role) = kind.combo(self.chapter_mode());
        skribisto_model::allowed_content(&role, &sub_role)
            .iter()
            .any(|r| {
                matches!(
                    r,
                    frontend::common::entities::ContentRole::SceneText
                        | frontend::common::entities::ContentRole::NoteText
                        | frontend::common::entities::ContentRole::ParatextText
                )
            })
    }

    /// The rows the chosen destination already holds, in stream order.
    ///
    /// The *subtree*, not the whole binder: dropping into a chapter reconciles against that
    /// chapter's own rows, and matching a returning chapter against the entire manuscript
    /// would let a title guess reach across the book. A binder destination is the whole
    /// binder, which is the same rule with the root as the anchor.
    ///
    /// Read live from the backend rather than from a cached model: the binder can change
    /// between the writer analysing an import and accepting it, and a merge computed against a
    /// stale tree would offer to update rows that have moved.
    fn destination_rows(&self) -> Vec<(DestinationItem, ExistingRow)> {
        use frontend::commands::{binder_commands, binder_item_commands};
        use frontend::common::direct_access::binder::BinderRelationshipField;

        let Some(dest) = self.destination.selected() else {
            return Vec::new();
        };
        let ids = binder_commands::get_binder_relationship(
            &self.app_ctx,
            &dest.binder_id,
            &BinderRelationshipField::BinderItems,
        )
        .unwrap_or_default();
        let by_id: HashMap<u64, _> =
            binder_item_commands::get_binder_item_multi(&self.app_ctx, &ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .map(|it| (it.id, it))
                .collect();

        // "X plus every following item with a strictly greater indent" — the binder has no
        // parent/child graph, only an ordered stream and a depth, and this is what a subtree
        // means in it. Same rule `binder_ordering` states for move and restore.
        let ordered: Vec<_> = ids.iter().filter_map(|id| by_id.get(id)).collect();
        let scope: Vec<_> = match dest.anchor_item_id {
            Some(anchor) => match ordered.iter().position(|it| it.id == anchor) {
                Some(at) => {
                    let depth = ordered[at].indent;
                    ordered[at + 1..]
                        .iter()
                        .take_while(|it| it.indent > depth)
                        .copied()
                        .collect()
                }
                None => Vec::new(),
            },
            None => ordered.iter().copied().collect(),
        };

        scope
            .into_iter()
            // A trashed row is not part of the manuscript, and offering to update one would
            // quietly bring it back.
            .filter(|it| it.activated && !it.uid.is_nil())
            // A row whose (role, sub_role) is not in the constraint matrix cannot be
            // named by the vocabulary the plan speaks, so it cannot be paired with anything
            // the file brings — it is left out rather than guessed at.
            .filter_map(|it| {
                let create_type = CreateType::of(&it.role, &it.sub_role)?;
                Some((
                    DestinationItem {
                        id: it.id,
                        uid: it.uid,
                        title: it.title.clone(),
                        indent: it.indent,
                    },
                    ExistingRow {
                        uid_tag: skribisto_model::round_trip::uid_tag(&it.uid),
                        title: it.title.clone(),
                        create_type,
                        digest: digest_of_djot(&self.item_prose(it.id).unwrap_or_default()),
                    },
                ))
            })
            .collect()
    }

    /// The prose stored on one binder item, if it has any.
    fn item_prose(&self, item_id: u64) -> Option<String> {
        use frontend::commands::{binder_item_commands, content_commands};
        use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
        use frontend::common::entities::ContentRole;

        let ids = binder_item_commands::get_binder_item_relationship(
            &self.app_ctx,
            &item_id,
            &BinderItemRelationshipField::Contents,
        )
        .ok()?;
        content_commands::get_content_multi(&self.app_ctx, &ids)
            .ok()?
            .into_iter()
            .flatten()
            .find(|c| {
                matches!(
                    c.role,
                    ContentRole::SceneText | ContentRole::NoteText | ContentRole::ParatextText
                )
            })
            .map(|c| c.data)
    }

    /// How this project encodes a Chapter. Read live rather than cached: it is a
    /// per-`Work` setting, and this view-model outlives any one analysis.
    fn chapter_mode(&self) -> frontend::common::entities::ChapterMode {
        self.ids
            .work_id
            .get()
            .and_then(|id| frontend::commands::work_commands::get_work(&self.app_ctx, &id).ok())
            .flatten()
            .map(|w| w.chapter_mode)
            .unwrap_or(frontend::common::entities::ChapterMode::Folder)
    }

    /// The included rows `apply_document_import` would refuse, by title.
    ///
    /// Drives both the Import button's enabled state and the sentence that says why
    /// it is off — a disabled button with no reason is worse than one that fails.
    pub fn blocking_rows(&self) -> Vec<String> {
        self.plan
            .keys_in_order()
            .into_iter()
            .filter(|k| self.is_included(*k))
            .filter(|k| !self.holds_its_prose(*k))
            .filter_map(|k| self.plan.row(k).map(|r| r.title))
            .collect()
    }

    // ── inclusion ───────────────────────────────────────────────────────────

    /// Whether this row will actually be created: its own tick **and** every
    /// ancestor's.
    ///
    /// Effective rather than cascaded, because a checkbox owns the signal it is
    /// given and there is no toggle hook to cascade from. It is also the better
    /// reading: unticking a chapter and ticking it again restores exactly the
    /// scenes the writer had chosen underneath, where a destructive cascade would
    /// have forgotten them. The panel disables a descendant's checkbox while an
    /// ancestor is unticked, so a ticked row under an unticked chapter reads as
    /// "fine, but its chapter is not coming" rather than as a contradiction.
    pub fn is_included(&self, key: PlanRowKey) -> bool {
        self.plan.is_ticked(key) && self.ancestors_included(key)
    }

    /// Whether every ancestor of `key` is ticked — what the panel binds a
    /// descendant checkbox's `enabled` to.
    pub fn ancestors_included(&self, key: PlanRowKey) -> bool {
        self.plan
            .ancestors(key)
            .into_iter()
            .all(|a| self.plan.is_ticked(a))
    }

    /// Tick or untick one row. Descendants are not touched — they are governed by
    /// [`Self::is_included`] instead.
    pub fn set_included(&self, key: PlanRowKey, included: bool) {
        self.plan.set_ticked(key, included);
    }

    pub fn included_count(&self) -> usize {
        self.plan
            .keys_in_order()
            .into_iter()
            .filter(|k| self.is_included(*k))
            .count()
    }

    /// Whether Import may be pressed.
    ///
    /// Includes "no included row would be refused by the backend". `apply_document_import`
    /// runs in one transaction and returns `Err` on the first row whose type cannot hold
    /// its prose — so one bad row does not lose one row, it loses the whole import. The
    /// review step is where that is still fixable, so it is refused here.
    pub fn can_apply(&self) -> bool {
        self.included_count() > 0
            && self.ids.work_id.get().is_some()
            && self.destination.selected().is_some()
            && self.blocking_rows().is_empty()
    }

    // ── commit ──────────────────────────────────────────────────────────────

    /// Create the accepted rows. One transaction, one undo entry.
    pub fn apply(&self) -> anyhow::Result<Vec<u64>> {
        let work_id = self
            .ids
            .work_id
            .get()
            .ok_or_else(|| anyhow::anyhow!("import: no project is open"))?;
        let destination = self
            .destination
            .selected()
            .ok_or_else(|| anyhow::anyhow!("import: no destination chosen"))?;

        let rows = self.rows_to_create();
        if rows.is_empty() {
            return Err(anyhow::anyhow!("import: every row is excluded"));
        }

        let result = frontend::commands::import_management_commands::apply_document_import(
            &self.app_ctx,
            self.ids.stack_id.get(),
            &ApplyDocumentImportDto {
                work_id,
                binder_id: destination.binder_id,
                anchor_item_id: destination.anchor_item_id.unwrap_or(0),
                drop_position: to_dto_position(destination.position.clone()),
                row: ApplyImportRow::Empty,
                rows: ApplyImportRows::Create(rows),
            },
        )?;
        Ok(result.created_ids)
    }

    /// The rows the import would write, in plan order, carrying whatever the writer decided.
    ///
    /// Two shapes come out of here, and which one a row takes is the reconcile step's whole
    /// output: a row the returning file brings home to one the project already has becomes an
    /// `Update` naming it; everything else becomes a `Create`. A row the writer chose to leave
    /// alone becomes nothing at all — the same way an unticked row does.
    ///
    /// Public so a test can assert on exactly what would be written without needing a store
    /// behind it — the thing that is expensive to check any other way, and the thing a mistake
    /// here would silently get wrong.
    pub fn rows_to_create(&self) -> Vec<ApplyImportRow> {
        // What the reconcile step decided, if the writer got that far. Keyed by the plan row
        // each decision is about, so the walk below stays in plan order.
        let decided: HashMap<PlanRowKey, RowAction> = self
            .merge
            .borrow()
            .iter()
            .filter_map(|m| Some((m.incoming_key?, self.action_for(m))))
            .collect();

        self.plan
            .keys_in_order()
            .into_iter()
            .filter(|k| self.is_included(*k))
            .filter_map(|key| {
                let row = self.plan.row(key)?;
                let kind = self.plan.type_of(key)?;
                // Handed straight back as they arrived. The writer reviews *rows*, and
                // retyping a title or a type cannot move a comment: its quote was measured
                // against this row's prose, and the prose is what the review step never edits.
                let comments: Vec<_> = row.comments.iter().map(comment_to_dto).collect();
                let tag = row.source_uid_tag.clone().unwrap_or_default();

                match decided.get(&key).copied() {
                    // Bring it home to the row it came from. `TakeImport` wants the editor's
                    // wording as well as their remarks; `CommentsOnly` wants only the remarks,
                    // which is the case this whole feature exists for.
                    Some(action @ (RowAction::TakeImport | RowAction::CommentsOnly))
                        if !tag.is_empty() =>
                    {
                        Some(ApplyImportRow::Update {
                            target_uid_tag: tag,
                            replace_prose: action == RowAction::TakeImport,
                            djot: row.djot,
                            comments,
                        })
                    }
                    // Nothing is written for a row the writer is keeping as it is. There is no
                    // "leave it alone" instruction to send, and there does not need to be.
                    Some(RowAction::KeepCurrent | RowAction::Ignore) => None,
                    // `CreateNew`, or no decision at all — a first import, or a plan the writer
                    // accepted without ever reaching the reconcile step.
                    _ => Some(ApplyImportRow::Create {
                        indent: row.indent,
                        kind: create_type_to_kind(kind),
                        title: row.title,
                        djot: row.djot,
                        comments,
                        // The row's own identity, handed back untouched for the same reason its
                        // comments are.
                        source_uid_tag: tag,
                    }),
                }
            })
            .collect()
    }

    // ── reconcile ───────────────────────────────────────────────────────────

    /// Line the plan up against what the chosen destination already holds.
    ///
    /// Called on the way *into* the reconcile step, and rebuilt on every entry: the writer may
    /// go back and choose a different destination, and the whole question is about a
    /// particular subtree. Decisions already made survive, because they are keyed by
    /// [`MergeRowKey`] rather than by position.
    pub fn rebuild_merge(&self) {
        let existing = self.destination_rows();
        let incoming: Vec<(PlanRowKey, IncomingRow)> = self
            .plan
            .keys_in_order()
            .into_iter()
            .filter(|k| self.is_included(*k))
            .filter_map(|key| {
                let row = self.plan.row(key)?;
                let kind = self.plan.type_of(key)?;
                Some((
                    key,
                    IncomingRow {
                        source_uid_tag: row.source_uid_tag.clone(),
                        source_digest: row.source_digest.clone(),
                        title: row.title.clone(),
                        create_type: kind,
                        digest: digest_of_djot(&row.djot),
                    },
                ))
            })
            .collect();

        let incoming_rows: Vec<IncomingRow> = incoming.iter().map(|(_, r)| r.clone()).collect();
        let existing_rows: Vec<ExistingRow> = existing.iter().map(|(_, r)| r.clone()).collect();
        let aligned = reconcile::align(&existing_rows, &incoming_rows);

        let mut out = Vec::with_capacity(aligned.len());
        for m in aligned {
            let cur = m.current.and_then(|j| existing.get(j));
            let inc = m.incoming.and_then(|i| incoming.get(i));
            let key = match (cur, inc) {
                (Some((item, _)), _) => MergeRowKey::Current(item.uid),
                (None, Some((k, _))) => MergeRowKey::Incoming(*k),
                (None, None) => continue,
            };
            out.push(MergeRowView {
                key,
                indent: cur.map(|(item, _)| item.indent).unwrap_or(0),
                current_title: cur.map(|(item, _)| item.title.clone()),
                current_item_id: cur.map(|(item, _)| item.id),
                incoming_title: inc.map(|(_, r)| r.title.clone()),
                incoming_key: inc.map(|(k, _)| *k),
                status: m.status,
                moved: m.moved,
                actions: m.actions,
            });
        }

        *self.merge.borrow_mut() = out;
        // Decisions for rows that are no longer in the sequence are dropped, so a destination
        // the writer tried and abandoned cannot leave an instruction behind.
        let live: std::collections::HashSet<MergeRowKey> =
            self.merge.borrow().iter().map(|m| m.key).collect();
        self.merge_actions
            .borrow_mut()
            .retain(|k, _| live.contains(k));
        self.merge_version.set(self.merge_version.get() + 1);
    }

    /// The merge sequence, for the reconcile step's table.
    pub fn merge_rows(&self) -> Vec<MergeRowView> {
        self.merge.borrow().clone()
    }

    /// Bumped whenever [`rebuild_merge`](Self::rebuild_merge) runs.
    pub fn merge_version(&self) -> Signal<u64> {
        self.merge_version.clone()
    }

    /// What will happen to this row: the writer's choice, or the safe default for its shape.
    pub fn action_for(&self, row: &MergeRowView) -> RowAction {
        self.merge_actions
            .borrow()
            .get(&row.key)
            .copied()
            .unwrap_or_else(|| row.actions.first().copied().unwrap_or(RowAction::Ignore))
    }

    /// Record what the writer chose for one row.
    pub fn set_action(&self, key: MergeRowKey, action: RowAction) {
        self.merge_actions.borrow_mut().insert(key, action);
        self.merge_version.set(self.merge_version.get() + 1);
    }

    /// Whether the returning file has anything to reconcile against at all.
    ///
    /// False for a first import into an empty destination, where every row is new and a table
    /// of identical "create it" dropdowns would be a page of ceremony saying nothing.
    pub fn has_anything_to_reconcile(&self) -> bool {
        self.merge
            .borrow()
            .iter()
            .any(|m| m.current_item_id.is_some())
    }

    /// How many rows the returning file brings home rather than adds.
    pub fn matched_count(&self) -> usize {
        self.merge
            .borrow()
            .iter()
            .filter(|m| m.current_item_id.is_some() && m.incoming_key.is_some())
            .count()
    }

    /// The prose on either side of one row, for the compare view.
    ///
    /// `(what the project holds, what the file brings)`. Either may be empty — a row present on
    /// only one side has nothing to show on the other, and the view says so rather than
    /// pretending the passage was deleted or invented.
    pub fn compare_prose(&self, row: &MergeRowView) -> (String, String) {
        let current = row
            .current_item_id
            .and_then(|id| self.item_prose(id))
            .unwrap_or_default();
        let incoming = row
            .incoming_key
            .and_then(|k| self.plan.row(k))
            .map(|r| r.djot)
            .unwrap_or_default();
        (current, incoming)
    }

    /// Say that `created` rows landed, and offer to take them back.
    ///
    /// The whole import is one undo entry, so one Undo press reverses it —
    /// which is what makes landing an inferred structure straight in a live
    /// manuscript a reasonable thing to do at all. The offer lapses after
    /// [`UNDO_GRACE`]; the entry itself does **not**, so a writer who notices
    /// an hour later can still reach it through the ordinary undo history.
    /// (Empty Trash clears the stack when its toast lapses; an import must not
    /// — see [`UNDO_GRACE`]'s own note.)
    ///
    /// Separate from [`Self::apply`] so the panel can dismiss itself *between*
    /// the two: a toast raised first becomes the topmost overlay, and the
    /// dismissal would take it instead of the wizard.
    pub fn offer_undo(&self, ctx: &mut EventContext, created: usize) {
        let work_id = self.work_id();
        let app_ctx = self.app_ctx.clone();
        let stack = self.ids.stack_id.get();
        ctx.show_toast(
            Toast::success(tr!(import_document_done(count = created as i64)))
                .scoped_id(IMPORT_TOAST_ID, work_id)
                .target_work(work_id)
                .auto_dismiss_after(UNDO_GRACE)
                .action(ToastAction::primary(
                    tr!(import_document_undo()),
                    move |_c| {
                        let _ = undo_redo_commands::undo(&app_ctx, stack);
                    },
                )),
        );
    }

    /// The import was refused. Scoped to this Work like the success toast, so
    /// two projects never overwrite each other's answer.
    pub fn report_failure(&self, ctx: &mut EventContext, error: &anyhow::Error) {
        ctx.show_toast(
            Toast::error(lit!(format!("{error:#}")))
                .scoped_id(IMPORT_TOAST_ID, self.work_id())
                .target_work(self.work_id()),
        );
    }

    pub fn back_to_files(&self) {
        self.controller.go_to(STEP_FILES);
    }

    /// Reactive Next gate for the Files step — at least one file, a project open,
    /// and no analysis already in flight.
    ///
    /// Under `mocks`, files are not required: Next must stay enabled so layout
    /// and automation can walk the Stepper without a real drop.
    pub fn can_analyse_signal(&self) -> Signal<bool> {
        let busy = self.busy.clone();
        if cfg!(feature = "mocks") {
            return busy.map(|b| !*b);
        }
        let files = self.file_count.clone();
        let work = self.ids.work_id.clone();
        files
            .zip(&busy)
            .zip(&work)
            .map(|((n, b), w)| *n > 0 && !*b && w.is_some())
    }

    /// Reactive Next gate for the Review step — analysis finished (progress UI
    /// has cleared). Destination is chosen on the next step, so it is not required
    /// here.
    ///
    /// Under `mocks`, Next stays enabled once not busy (the mock plan is planted
    /// synchronously on the Files→Review advance).
    ///
    /// **Also refuses while a row would be refused by the backend.** That rule
    /// used to live only on Finish, two steps later, where nothing on screen
    /// could explain it: a writer who picked a destination watched Import stay
    /// grey with the cause — a Book row carrying prose — on a page they had
    /// already left. `blocking_rows` is fixable *here* (retype the row, or
    /// untick it) and the diagnostics strip right below the tree says which row
    /// and why, so this is where the flow has to stop.
    pub fn can_proceed_from_review_signal(&self) -> Signal<bool> {
        let me = self.clone();
        let plan_v = self.plan.version_signal();
        self.busy
            .zip(&plan_v)
            .map(move |(busy, _)| !*busy && me.blocking_rows().is_empty())
    }

    /// Reactive Finish gate for the Destination step — same rules as
    /// [`Self::can_apply`], re-derived whenever the plan or the destination
    /// selection changes.
    pub fn can_apply_signal(&self) -> Signal<bool> {
        let me = self.clone();
        let plan_v = self.plan.version_signal();
        let has_dest: Prop<bool> = self.destination.has_selection().into();
        let has_dest = has_dest.as_signal();
        plan_v.zip(&has_dest).map(move |_| me.can_apply())
    }
}

/// One row of the destination, as the merge view needs to name it.
#[derive(Clone, Debug)]
pub struct DestinationItem {
    pub id: u64,
    pub uid: uuid::Uuid,
    pub title: String,
    pub indent: i64,
}

/// The digest of a row's prose, over its plain reading.
///
/// Through the same `djot_plain_text` the exporter used when it wrote the mark, so the two
/// sides of a three-way comparison are measured the same way. A row whose Djot will not parse
/// digests as empty rather than failing the merge — an unreadable row is one the writer needs
/// to see, not one the wizard should refuse to show them.
fn digest_of_djot(djot: &str) -> String {
    let plain = skrib_format::djot_plain_text(djot)
        .map(|(t, _)| t)
        .unwrap_or_default();
    skribisto_model::round_trip::digest(&plain)
}

/// The level → type table the analysis's own rows imply.
///
/// The plan DTO carries no rules — it carries the *result* of applying them —
/// so the table the writer edits is read back off the rows: each heading level
/// takes the type the analyser gave the first row that came from it. Rebuilding
/// it here rather than widening the DTO keeps one source of truth: a rule that
/// disagreed with the rows it supposedly produced would be worse than no table.
fn infer_level_rules(plan: &ImportPlan, levels: &[u8]) -> Vec<(u8, CreateType)> {
    let mut rules: Vec<(u8, CreateType)> = Vec::new();
    for (row, level) in plan.rows.iter().zip(levels) {
        if !rules.iter().any(|(l, _)| l == level) {
            rules.push((*level, row.create_type));
        }
    }
    rules.sort_by_key(|(l, _)| *l);
    rules
}

/// The picker's own `DropPosition` → the import feature's DTO enum.
///
/// Two enums for one idea, because a Qleany DTO enum cannot be shared across crates —
/// `trash_management` carries the identical pair and maps it the same way
/// (`restore_items_to_uc::to_drop_place`). Exhaustive, so a fourth position added to
/// the widget fails to compile here rather than defaulting to something plausible.
fn to_dto_position(place: frontend::trash_management::DropPosition) -> DropPosition {
    use frontend::trash_management::DropPosition as Picked;
    match place {
        Picked::Before => DropPosition::Before,
        Picked::After => DropPosition::After,
        Picked::Into => DropPosition::Into,
    }
}

/// The DTO's flat kind ↔ the model's create vocabulary.
///
/// A second copy of `import_management::kind_mapping`, and deliberately so: that
/// one lives behind a use case and the UI cannot reach it without depending on a
/// feature crate. Both are exhaustive matches, so adding a `CreateType` fails to
/// compile in both places rather than defaulting one of them to Scene.
fn create_type_to_kind(kind: CreateType) -> ImportRowKind {
    match kind {
        CreateType::Book => ImportRowKind::Book,
        CreateType::Part => ImportRowKind::Part,
        CreateType::Chapter => ImportRowKind::Chapter,
        CreateType::Scene => ImportRowKind::Scene,
        CreateType::Note => ImportRowKind::Note,
        CreateType::NoteFolder => ImportRowKind::NoteFolder,
        CreateType::Folder => ImportRowKind::Folder,
        CreateType::Paratext => ImportRowKind::Paratext,
        CreateType::ParatextFolder => ImportRowKind::ParatextFolder,
        CreateType::EndOfBook => ImportRowKind::EndOfBook,
    }
}

/// One imported comment, DTO → plan.
///
/// A pure, exhaustive field copy in both directions ([`comment_to_dto`] is the
/// twin). Nothing here decides anything: the anchor was proved against this row's
/// Djot back in `document_ingest::plan`, and the review step edits titles and types
/// but never prose — so a comment that arrives correct stays correct.
///
/// Returns `None` for `ImportComment::Empty`, the default variant nothing populates.
fn comment_from_dto(comment: &ImportComment) -> Option<PlannedComment> {
    let ImportComment::Found {
        kind,
        uid,
        uid_tag,
        author_name,
        author_initials,
        created_at,
        body,
        resolved,
        orphaned,
        orphan_reason,
        range_start,
        range_length,
        quote_prefix,
        quote_exact,
        quote_exact_truncated,
        quote_suffix,
        block_ordinal_hint,
        replies,
    } = comment
    else {
        return None;
    };
    Some(PlannedComment {
        kind: match kind {
            ImportCommentKind::Range => frontend::common::entities::CommentAnchorKind::Range,
            ImportCommentKind::Paragraph => {
                frontend::common::entities::CommentAnchorKind::Paragraph
            }
            ImportCommentKind::Document => frontend::common::entities::CommentAnchorKind::Document,
        },
        anchor: skribisto_model::comment_anchor::Anchor {
            start: (*range_start).max(0) as usize,
            length: (*range_length).max(0) as usize,
            prefix: quote_prefix.clone(),
            exact: quote_exact.clone(),
            exact_truncated: *quote_exact_truncated,
            suffix: quote_suffix.clone(),
            block_ordinal: (*block_ordinal_hint).max(0) as usize,
            block_span: 1,
        },
        orphaned: *orphaned,
        orphan_reason: match orphan_reason {
            ImportOrphanReason::NotOrphaned => {
                frontend::common::entities::CommentOrphanReason::NotOrphaned
            }
            ImportOrphanReason::TextNotFound => {
                frontend::common::entities::CommentOrphanReason::TextNotFound
            }
            ImportOrphanReason::Ambiguous => {
                frontend::common::entities::CommentOrphanReason::Ambiguous
            }
            ImportOrphanReason::TargetDeleted => {
                frontend::common::entities::CommentOrphanReason::TargetDeleted
            }
        },
        uid: *uid,
        // Empty on the wire means "this file carried no mark for this comment".
        uid_tag: (!uid_tag.is_empty()).then(|| uid_tag.clone()),
        author: author_name.clone(),
        author_initials: author_initials.clone(),
        created: parse_rfc3339(created_at),
        body: body.clone(),
        resolved: *resolved,
        replies: replies
            .iter()
            .filter_map(|reply| {
                let ImportReply::Found {
                    uid,
                    author_name,
                    author_initials,
                    created_at,
                    body,
                } = reply
                else {
                    return None;
                };
                Some(document_ingest::SourceAnnotationReply {
                    uid: *uid,
                    author: author_name.clone(),
                    author_initials: author_initials.clone(),
                    created: parse_rfc3339(created_at),
                    body: body.clone(),
                })
            })
            .collect(),
    })
}

/// One imported comment, plan → DTO. The twin of [`comment_from_dto`].
fn comment_to_dto(comment: &PlannedComment) -> ImportComment {
    ImportComment::Found {
        uid_tag: comment.uid_tag.clone().unwrap_or_default(),
        kind: match comment.kind {
            frontend::common::entities::CommentAnchorKind::Range => ImportCommentKind::Range,
            frontend::common::entities::CommentAnchorKind::Paragraph => {
                ImportCommentKind::Paragraph
            }
            frontend::common::entities::CommentAnchorKind::Document => ImportCommentKind::Document,
        },
        uid: comment.uid,
        author_name: comment.author.clone(),
        author_initials: comment.author_initials.clone(),
        created_at: comment.created.map(|d| d.to_rfc3339()).unwrap_or_default(),
        body: comment.body.clone(),
        resolved: comment.resolved,
        orphaned: comment.orphaned,
        orphan_reason: match comment.orphan_reason {
            frontend::common::entities::CommentOrphanReason::NotOrphaned => {
                ImportOrphanReason::NotOrphaned
            }
            frontend::common::entities::CommentOrphanReason::TextNotFound => {
                ImportOrphanReason::TextNotFound
            }
            frontend::common::entities::CommentOrphanReason::Ambiguous => {
                ImportOrphanReason::Ambiguous
            }
            frontend::common::entities::CommentOrphanReason::TargetDeleted => {
                ImportOrphanReason::TargetDeleted
            }
        },
        range_start: comment.anchor.start as i64,
        range_length: comment.anchor.length as i64,
        quote_prefix: comment.anchor.prefix.clone(),
        quote_exact: comment.anchor.exact.clone(),
        quote_exact_truncated: comment.anchor.exact_truncated,
        quote_suffix: comment.anchor.suffix.clone(),
        block_ordinal_hint: comment.anchor.block_ordinal as i64,
        replies: comment
            .replies
            .iter()
            .map(|reply| ImportReply::Found {
                uid: reply.uid,
                author_name: reply.author.clone(),
                author_initials: reply.author_initials.clone(),
                created_at: reply.created.map(|d| d.to_rfc3339()).unwrap_or_default(),
                body: reply.body.clone(),
            })
            .collect(),
    }
}

/// An empty string means the source carried no date, which is not an error.
fn parse_rfc3339(value: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(value.trim())
        .ok()
        .map(|d| d.with_timezone(&chrono::Utc))
}

/// Read a plan and its diagnostics back out of the analyse DTO.
pub fn plan_from_dto(
    rows: &DocumentImportRows,
    diagnostics: &ImportDiagnosticRows,
) -> (ImportPlan, Vec<u8>, Vec<Diagnostic>) {
    let mut plan = ImportPlan::default();
    let mut levels = Vec::new();

    if let DocumentImportRows::Found(found) = rows {
        for row in found {
            let DocumentImportRow::Found {
                indent,
                kind,
                title,
                stripped_ordinal,
                djot,
                scene_breaks,
                word_count,
                comments,
                origin,
                included,
                source_uid_tag,
                source_digest,
            } = row
            else {
                continue;
            };
            // The heading level a row came from is its indent plus one: the plan
            // is built by walking levels down into indents, so the inverse is
            // exact for every row a heading produced.
            levels.push((indent + 1).clamp(1, u8::MAX as i64) as u8);
            plan.rows.push(PlannedRow {
                indent: *indent,
                create_type: kind_to_create_type(kind),
                title: title.clone(),
                stripped_ordinal: (!stripped_ordinal.is_empty()).then(|| stripped_ordinal.clone()),
                djot: djot.clone(),
                scene_breaks: *scene_breaks as usize,
                word_count: *word_count as usize,
                comments: comments.iter().filter_map(comment_from_dto).collect(),
                origin: origin.clone(),
                included: *included,
                // Empty on the wire means the file carried no mark for this row.
                source_uid_tag: (!source_uid_tag.is_empty()).then(|| source_uid_tag.clone()),
                source_digest: (!source_digest.is_empty()).then(|| source_digest.clone()),
                diagnostics: Vec::new(),
            });
        }
    }

    let mut out = Vec::new();
    if let ImportDiagnosticRows::Reported(reported) = diagnostics {
        for entry in reported {
            let ImportDiagnosticRow::Reported {
                key,
                severity,
                path,
                detail,
                count,
                row_index,
            } = entry
            else {
                continue;
            };
            out.push(Diagnostic {
                key: key.clone(),
                severity: severity.clone(),
                path: path.clone(),
                detail: detail.clone(),
                count: *count,
                row: (*row_index >= 0).then_some(PlanRowKey(*row_index as u32)),
            });
        }
    }

    (plan, levels, out)
}

fn kind_to_create_type(kind: &ImportRowKind) -> CreateType {
    match kind {
        ImportRowKind::Book => CreateType::Book,
        ImportRowKind::Part => CreateType::Part,
        ImportRowKind::Chapter => CreateType::Chapter,
        ImportRowKind::Scene => CreateType::Scene,
        ImportRowKind::Note => CreateType::Note,
        ImportRowKind::NoteFolder => CreateType::NoteFolder,
        ImportRowKind::Folder => CreateType::Folder,
        ImportRowKind::Paratext => CreateType::Paratext,
        ImportRowKind::ParatextFolder => CreateType::ParatextFolder,
        ImportRowKind::EndOfBook => CreateType::EndOfBook,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use document_ingest::plan::PlannedRow;

    /// Run `f` with the **real shipped** `en-US` messages installed.
    ///
    /// Not `I18nConfig::test_only` with a hand-copied list of patterns, which is
    /// the other precedent in this crate (`view_models::open_failure`): that
    /// proves a copy agrees with itself, and the whole point here is to catch a
    /// diagnostic whose key never reached `main.ftl`.
    ///
    /// It also has to exist at all: with no manager installed, a message
    /// carrying a `{ $count -> … }` plural selector resolves to its own id —
    /// which would have made this test pass for the wrong reason on every
    /// plural diagnostic.
    fn with_real_messages(f: impl FnOnce()) {
        use teksilo::i18n::config::I18nConfig;
        use teksilo::i18n::manager::I18nManager;
        use teksilo::i18n::thread_local::{clear, install};

        clear();
        let cfg = I18nConfig::new()
            .source_locale("en-US".parse().unwrap())
            .supported_locales(["en-US".parse().unwrap()])
            .compile_in(&[(
                "en-US",
                &[
                    include_str!("../../locales/en-US/main.ftl"),
                    include_str!("../../locales/en-US/tooltips.ftl"),
                    include_str!("../../locales/en-US/tags.ftl"),
                    include_str!("../../locales/en-US/templates.ftl"),
                ],
            )])
            .auto_detect_os_locale(false)
            .fallback_locale("en-US".parse().unwrap());
        install(I18nManager::from_config(&cfg));
        f();
        clear();
    }

    /// A container row: a real one carries no prose of its own.
    ///
    /// The fixture used to give *every* row prose, including its Book — a shape the
    /// analyser cannot produce (`infer_rules` keeps the deepest level prose-bearing)
    /// and one `apply_document_import` refuses outright. It went unnoticed while
    /// nothing checked; the live type check now does, which is the point of it.
    fn container(indent: i64, title: &str, kind: CreateType) -> PlannedRow {
        PlannedRow {
            djot: String::new(),
            word_count: 0,
            ..planned(indent, title, kind)
        }
    }

    fn planned(indent: i64, title: &str, kind: CreateType) -> PlannedRow {
        PlannedRow {
            indent,
            create_type: kind,
            title: title.into(),
            stripped_ordinal: None,
            djot: format!("{title} prose."),
            scene_breaks: 0,
            word_count: 2,
            origin: "a.md".into(),
            included: true,
            comments: Vec::new(),
            source_uid_tag: None,
            source_digest: None,
            diagnostics: Vec::new(),
        }
    }

    /// Book / Chapter One (Scene A, Scene B) / Chapter Two, from heading levels
    /// 1, 2, 3, 3, 2.
    fn vm() -> ImportDocumentViewModel {
        let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
        let plan = ImportPlan {
            rows: vec![
                container(0, "Book", CreateType::Book),
                planned(1, "Chapter One", CreateType::Chapter),
                planned(2, "Scene A", CreateType::Scene),
                planned(2, "Scene B", CreateType::Scene),
                planned(1, "Chapter Two", CreateType::Chapter),
            ],
            diagnostics: Vec::new(),
        };
        vm.on_plan_ready(
            &plan,
            vec![1, 2, 3, 3, 2],
            vec![
                (1, CreateType::Book),
                (2, CreateType::Chapter),
                (3, CreateType::Scene),
            ],
        );
        vm
    }

    fn created_titles(vm: &ImportDocumentViewModel) -> Vec<String> {
        vm.rows_to_create()
            .into_iter()
            .filter_map(|r| match r {
                ApplyImportRow::Create { title, .. } => Some(title),
                // An update names no title — it points at a row that already has one.
                ApplyImportRow::Update { .. } | ApplyImportRow::Empty => None,
            })
            .collect()
    }

    #[test]
    fn a_ready_plan_moves_to_the_review_step() {
        let vm = vm();
        assert_eq!(vm.step().get(), STEP_REVIEW);
        assert_eq!(vm.included_count(), 5);
    }

    /// Chapter files without a Book heading — wrap them under a writer-added root.
    #[test]
    fn a_top_level_header_wraps_every_analysed_row() {
        let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
        vm.on_plan_ready(
            &ImportPlan {
                rows: vec![
                    planned(0, "Chapter One", CreateType::Chapter),
                    planned(0, "Chapter Two", CreateType::Chapter),
                ],
                diagnostics: Vec::new(),
            },
            vec![1, 1],
            vec![(1, CreateType::Chapter)],
        );
        vm.add_top_level_header();

        assert_eq!(vm.plan().visible_count(), 3);
        assert_eq!(vm.plan().type_of(PlanRowKey(0)), Some(CreateType::Book));
        assert_eq!(vm.plan().row(PlanRowKey(1)).map(|r| r.indent), Some(1));
        assert_eq!(vm.plan().row(PlanRowKey(2)).map(|r| r.indent), Some(1));
        // The synthetic root is included and carries no prose, so Import is not blocked.
        assert!(vm.blocking_rows().is_empty());
        let titles = created_titles(&vm);
        assert_eq!(titles.len(), 3, "root + two chapters");
        assert_eq!(titles[1], "Chapter One");
        assert_eq!(titles[2], "Chapter Two");
    }

    /// One rule change instead of two hundred corrections — the reason the rule
    /// table exists at all.
    #[test]
    fn a_level_rule_retypes_every_row_that_came_from_that_level() {
        let vm = vm();
        vm.set_level_rule(3, CreateType::Note);
        assert_eq!(vm.plan().type_of(PlanRowKey(2)), Some(CreateType::Note));
        assert_eq!(vm.plan().type_of(PlanRowKey(3)), Some(CreateType::Note));
        assert_eq!(
            vm.plan().type_of(PlanRowKey(1)),
            Some(CreateType::Chapter),
            "a different level is untouched"
        );
    }

    /// The failure that would make the rule table and the per-row combo unusable
    /// together: a bulk change quietly reverting a deliberate one.
    #[test]
    fn a_level_rule_never_overrules_a_row_the_writer_retyped() {
        let vm = vm();
        vm.retype_row(PlanRowKey(2), CreateType::Paratext);
        vm.set_level_rule(3, CreateType::Note);

        assert_eq!(
            vm.plan().type_of(PlanRowKey(2)),
            Some(CreateType::Paratext),
            "the pinned row keeps what the writer chose"
        );
        assert_eq!(
            vm.plan().type_of(PlanRowKey(3)),
            Some(CreateType::Note),
            "its unpinned sibling still follows the rule"
        );
    }

    #[test]
    fn excluding_a_chapter_excludes_its_scenes() {
        let vm = vm();
        vm.set_included(PlanRowKey(1), false);
        assert_eq!(created_titles(&vm), vec!["Book", "Chapter Two"]);
        assert_eq!(vm.included_count(), 2);
    }

    #[test]
    fn including_it_again_brings_the_whole_subtree_back() {
        let vm = vm();
        vm.set_included(PlanRowKey(1), false);
        vm.set_included(PlanRowKey(1), true);
        assert_eq!(vm.included_count(), 5);
        assert_eq!(created_titles(&vm).len(), 5);
    }

    /// What effective inclusion buys over a destructive cascade: a scene the
    /// writer unticked on its own stays unticked when its chapter comes back. A
    /// cascade would have forgotten that decision and re-included it.
    #[test]
    fn re_including_a_chapter_does_not_resurrect_a_scene_the_writer_unticked() {
        let vm = vm();
        vm.set_included(PlanRowKey(2), false); // Scene A, on its own
        vm.set_included(PlanRowKey(1), false); // then the whole chapter
        vm.set_included(PlanRowKey(1), true); // and back

        assert!(!vm.is_included(PlanRowKey(2)), "Scene A stays out");
        assert!(vm.is_included(PlanRowKey(3)), "Scene B comes back");
        assert_eq!(
            created_titles(&vm),
            vec!["Book", "Chapter One", "Scene B", "Chapter Two"]
        );
    }

    /// What the panel binds a descendant checkbox's `enabled` to, so a ticked row
    /// under an unticked chapter reads as "fine, but its chapter is not coming"
    /// rather than as a contradiction.
    #[test]
    fn a_descendant_knows_when_an_ancestor_is_holding_it_back() {
        let vm = vm();
        assert!(vm.ancestors_included(PlanRowKey(2)));
        vm.set_included(PlanRowKey(1), false);
        assert!(!vm.ancestors_included(PlanRowKey(2)));
        assert!(
            vm.plan().is_ticked(PlanRowKey(2)),
            "its own tick is untouched — only its effect is suspended"
        );
    }

    #[test]
    fn what_would_be_created_carries_the_retyped_kind_not_the_analysed_one() {
        let vm = vm();
        vm.retype_row(PlanRowKey(4), CreateType::Note);
        let rows = vm.rows_to_create();
        let ApplyImportRow::Create { kind, title, .. } = &rows[4] else {
            panic!("expected a create row");
        };
        assert_eq!(title, "Chapter Two");
        assert_eq!(*kind, ImportRowKind::Note);
    }

    #[test]
    fn indent_and_prose_survive_into_what_gets_created() {
        let vm = vm();
        let rows = vm.rows_to_create();
        let ApplyImportRow::Create { indent, djot, .. } = &rows[2] else {
            panic!("expected a create row");
        };
        assert_eq!(*indent, 2);
        assert_eq!(djot, "Scene A prose.");
    }

    #[test]
    fn nothing_can_be_applied_with_every_row_excluded() {
        let vm = vm();
        vm.set_included(PlanRowKey(0), false);
        assert_eq!(vm.included_count(), 0);
        assert!(!vm.can_apply());
        assert!(vm.apply().is_err());
    }

    #[test]
    fn files_keep_the_order_they_arrived_in_and_never_double_up() {
        let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
        vm.add_files([PathBuf::from("b.md"), PathBuf::from("a.md")]);
        vm.add_files([PathBuf::from("b.md")]);
        assert_eq!(
            vm.file_paths(),
            vec![PathBuf::from("b.md"), PathBuf::from("a.md")]
        );

        vm.move_file(1, -1);
        assert_eq!(
            vm.file_paths(),
            vec![PathBuf::from("a.md"), PathBuf::from("b.md")]
        );

        vm.remove_file(0);
        assert_eq!(vm.file_paths(), vec![PathBuf::from("b.md")]);
    }

    #[test]
    fn moving_a_file_off_either_end_does_nothing() {
        let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
        vm.add_files([PathBuf::from("a.md"), PathBuf::from("b.md")]);
        vm.move_file(0, -1);
        vm.move_file(1, 1);
        assert_eq!(
            vm.file_paths(),
            vec![PathBuf::from("a.md"), PathBuf::from("b.md")]
        );
    }

    /// The picker and the drop zone must offer exactly what a scanner can read —
    /// Markdown, plain text, Word and ODT (default `document_ingest` features).
    #[test]
    fn the_accepted_extensions_come_from_the_scanners_themselves() {
        let extensions = ImportDocumentViewModel::accepted_extensions();
        for expected in ["md", "markdown", "txt", "docx", "odt"] {
            assert!(
                extensions.iter().any(|e| e == expected),
                "missing {expected} in {extensions:?}"
            );
        }
    }

    #[test]
    fn resetting_clears_the_files_the_plan_and_the_step() {
        let vm = vm();
        vm.add_files([PathBuf::from("a.md")]);
        vm.reset();
        assert!(vm.file_paths().is_empty());
        assert!(vm.plan().is_empty());
        assert_eq!(vm.step().get(), STEP_FILES);
        assert_eq!(vm.included_count(), 0);
    }

    /// One drop can carry the same path twice. The dedup has to see what this
    /// very call already added, not only what was there when it started.
    #[test]
    fn one_call_cannot_add_the_same_file_twice() {
        let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
        vm.add_files([PathBuf::from("a.md"), PathBuf::from("a.md")]);
        assert_eq!(vm.file_paths(), vec![PathBuf::from("a.md")]);
        assert_eq!(vm.file_count().get(), 1);
    }

    /// What the footer's Next button greys out on — the count has to be a
    /// signal, because `ListModel` reports through observers and not a version.
    #[test]
    fn the_file_count_follows_every_way_the_list_changes() {
        let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
        assert_eq!(vm.file_count().get(), 0);
        vm.add_files([PathBuf::from("a.md"), PathBuf::from("b.md")]);
        assert_eq!(vm.file_count().get(), 2);
        vm.remove_file(0);
        assert_eq!(vm.file_count().get(), 1);
        vm.reset();
        assert_eq!(vm.file_count().get(), 0);
    }

    /// The rule table is read back off the rows, so it can never claim a level
    /// produced something the plan disagrees with.
    #[test]
    fn the_level_rules_are_what_the_analysed_rows_actually_say() {
        let plan = ImportPlan {
            rows: vec![
                container(0, "Book", CreateType::Book),
                planned(1, "One", CreateType::Chapter),
                planned(2, "A", CreateType::Scene),
                planned(1, "Two", CreateType::Chapter),
            ],
            diagnostics: Vec::new(),
        };
        assert_eq!(
            infer_level_rules(&plan, &[1, 2, 3, 2]),
            vec![
                (1, CreateType::Book),
                (2, CreateType::Chapter),
                (3, CreateType::Scene),
            ]
        );
    }

    /// Every background job in the app reports through the same four events.
    /// A wizard that moved its step on somebody else's export finishing would
    /// be unusable — and the failure would look like a random UI jump.
    #[test]
    fn an_event_for_another_operation_is_ignored() {
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportDocumentViewModel::new(app_ctx.clone(), AppIds::default());
        vm.add_files([PathBuf::from("a.md")]);

        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        tree.run_with_event_context(&mut teksilo::core::NoopWindowOps, |ctx| {
            let event = long_op_event("someone-elses-op");
            vm.on_long_op_progress(ctx, &event);
            vm.on_long_op_completed(ctx, &event);
            vm.on_long_op_cancelled(ctx, &event);
            vm.on_long_op_failed(ctx, &event);
        });

        assert_eq!(vm.step().get(), STEP_FILES, "the step must not have moved");
        assert_eq!(vm.file_paths().len(), 1, "the chosen files survive");
        assert_eq!(vm.progress().get(), 0.0);
    }

    /// A `LongOperation` event carrying `id`, shaped like the manager's own
    /// JSON payload (`view_models::long_op` parses it).
    fn long_op_event(id: &str) -> Event {
        use frontend::common::event::{LongOperationEvent, Origin};
        Event {
            origin: Origin::LongOperation(LongOperationEvent::Completed),
            ids: Vec::new(),
            data: Some(format!(
                r#"{{"id":"{id}","percentage":50.0,"message":"reading"}}"#
            )),
        }
    }

    /// A match with no wildcard arm, so a new `ImportDiagnostic` variant upstream
    /// stops this crate from compiling until the sample list below, the `message`
    /// arms, and both `.ftl` files have caught up.
    ///
    /// It does nothing at run time on purpose — the compiler is the assertion.
    fn exhaustive_over_every_variant(d: &document_ingest::ImportDiagnostic) {
        use document_ingest::ImportDiagnostic::*;
        match d {
            FileUnreadable { .. }
            | LossyDecode { .. }
            | DecodedFromBom { .. }
            | EmptyFile { .. }
            | NoHeadings { .. }
            | UnsupportedFormat { .. }
            | FrontMatterNotFlat { .. }
            | FootnotesDegraded { .. }
            | RawHtmlDropped { .. }
            | NestedBreakDropped { .. }
            | ImageNotIngested { .. }
            | DuplicateTitle { .. }
            | HeadingLevelJump { .. }
            | IllegalCombination { .. }
            | TrackedChangesFlattened { .. }
            | TextBoxDropped { .. }
            | EmbeddedObjectDropped { .. }
            | FieldFlattened { .. }
            | UnknownStyleLevel { .. }
            | CommentUnanchored { .. }
            | CommentRepliesFlattened { .. } => {}
        }
    }

    /// **Every** diagnostic `document_ingest` can raise must reach a translated
    /// sentence.
    ///
    /// The samples are real `ImportDiagnostic` variants, mapped through the real
    /// DTO mapper, so a variant added upstream fails here rather than silently
    /// landing in the fallback arm. Collected diagnostics that nothing rendered is
    /// the bug this whole surface exists to close; a new one quietly joining them
    /// would be the same bug again.
    ///
    /// [`exhaustive_over_every_variant`] is what keeps the list honest: a `vec!` of
    /// samples is only as complete as whoever last edited it, and this test's own
    /// note used to claim it was built from the variants when it was not. That
    /// claim is now enforced by the compiler.
    #[test]
    fn every_diagnostic_the_importer_can_raise_has_a_sentence() {
        use document_ingest::ImportDiagnostic as D;

        let every = vec![
            D::FileUnreadable {
                path: "/tmp/a.md".into(),
                reason: "permission denied".into(),
            },
            D::LossyDecode {
                path: "/tmp/a.md".into(),
                replacements: 3,
            },
            D::DecodedFromBom {
                path: "/tmp/a.md".into(),
                encoding: "UTF-16LE",
            },
            D::EmptyFile {
                path: "/tmp/a.md".into(),
            },
            D::NoHeadings {
                path: "/tmp/a.md".into(),
            },
            D::UnsupportedFormat {
                path: "/tmp/a.odt".into(),
                extension: "odt".into(),
            },
            D::FrontMatterNotFlat {
                path: "/tmp/a.md".into(),
                key: "tags".into(),
            },
            D::FootnotesDegraded {
                path: "/tmp/a.md".into(),
                count: 2,
            },
            D::RawHtmlDropped {
                path: "/tmp/a.md".into(),
                count: 1,
            },
            D::NestedBreakDropped {
                path: "/tmp/a.md".into(),
                count: 4,
            },
            D::ImageNotIngested {
                path: "/tmp/a.md".into(),
                target: "cover.png".into(),
            },
            D::DuplicateTitle {
                title: "Later".into(),
                occurrences: 2,
            },
            D::HeadingLevelJump {
                title: "Deep".into(),
                from: 1,
                to: 4,
            },
            D::IllegalCombination {
                title: "A part".into(),
                kind: CreateType::Part,
            },
            D::TrackedChangesFlattened {
                path: "/tmp/a.docx".into(),
                count: 6,
            },
            D::TextBoxDropped {
                path: "/tmp/a.docx".into(),
                count: 1,
            },
            D::EmbeddedObjectDropped {
                path: "/tmp/a.odt".into(),
                count: 2,
            },
            D::FieldFlattened {
                path: "/tmp/a.docx".into(),
                count: 9,
            },
            D::UnknownStyleLevel {
                path: "/tmp/a.docx".into(),
                style: "HeadingChapter".into(),
            },
            D::CommentUnanchored {
                path: "/tmp/a.docx".into(),
                quote: "Is this the right word?".into(),
            },
            D::CommentRepliesFlattened {
                path: "/tmp/a.odt".into(),
                count: 3,
            },
        ];

        for raised in &every {
            exhaustive_over_every_variant(raised);
        }

        with_real_messages(|| {
            for raised in &every {
                let dto = frontend::import_management::diagnostic_to_dto(raised, 0);
                let (_, _, parsed) = plan_from_dto(
                    &DocumentImportRows::Empty,
                    &ImportDiagnosticRows::Reported(vec![dto]),
                );
                let d = parsed.first().expect("the DTO round-trips");
                let text = d.message("A part", Some(CreateType::Part)).resolve_now();

                assert!(
                    !text.contains(raised.key()),
                    "{} fell through to the untranslated fallback: {text:?}",
                    raised.key()
                );
                assert!(!text.trim().is_empty(), "{} rendered nothing", raised.key());
                // A Fluent argument the message references but nobody supplied is
                // rendered as `{$name}` rather than failing — which reads as a
                // corrupt sentence and is exactly the mistake a plural selector
                // invites.
                assert!(
                    !text.contains("{$") && !text.contains("{ $"),
                    "{} left an argument unfilled: {text:?}",
                    raised.key()
                );
            }
        });
    }

    /// Worst first. A file that could not be opened at all must not sit under
    /// three notes about footnotes.
    /// **Regression.** Retyping a row into something that cannot hold its prose must
    /// stop the import, mark the row, and say so.
    ///
    /// `apply_document_import` runs in one transaction and returns `Err` on the first
    /// such row — so pressing Import lost the *whole* import, however many files it
    /// covered. The plan's own `illegal-combination` diagnostic was computed once at
    /// analysis time and never recomputed, so the strip reported the pre-retype state,
    /// no marker appeared, and the button stayed enabled. The review step is where this
    /// is still fixable.
    #[test]
    fn retyping_a_row_so_it_cannot_hold_its_prose_stops_the_import() {
        let vm = vm();
        assert!(
            vm.blocking_rows().is_empty(),
            "the fixture starts out applyable"
        );

        // Scene A carries prose; a Part cannot hold any.
        vm.retype_row(PlanRowKey(2), CreateType::Part);

        assert_eq!(
            vm.blocking_rows(),
            vec!["Scene A".to_string()],
            "the offending row must be named, so the writer can find it"
        );
        assert!(
            !vm.can_apply(),
            "Import must refuse rather than lose the batch"
        );
        assert_eq!(
            vm.diagnostics_for_row(PlanRowKey(2)).len(),
            1,
            "and the row must be marked in the tree"
        );

        // Retyping it back clears all three.
        vm.retype_row(PlanRowKey(2), CreateType::Scene);
        assert!(vm.blocking_rows().is_empty());
        assert!(
            vm.diagnostics_for_row(PlanRowKey(2)).is_empty(),
            "a row put right must lose its marker too"
        );
    }

    /// **Regression.** A blocking row must stop the writer on Review, not two
    /// steps later on a page that cannot explain it.
    ///
    /// What the writer met: a Book row carrying prose — the ordinary shape of
    /// "import a book" — walked through Review, through Destination, and then sat
    /// there with a chosen destination and a grey Import button. The cause was
    /// real and the refusal correct, but it was stated on a page they had left,
    /// so the button simply looked broken. Review is where a row can be retyped
    /// or unticked, so Review is where the flow stops.
    #[test]
    fn a_blocking_row_holds_the_writer_on_the_review_step() {
        let vm = vm();
        let gate = vm.can_proceed_from_review_signal();
        assert!(gate.get(), "an applyable plan may go on to Destination");

        vm.retype_row(PlanRowKey(2), CreateType::Part);
        assert!(
            !gate.get(),
            "a row the backend would refuse must hold Next, where it is fixable"
        );

        // Both ways out of it re-open the gate, reactively.
        vm.set_included(PlanRowKey(2), false);
        assert!(gate.get(), "unticking the row is one way out");
        vm.set_included(PlanRowKey(2), true);
        assert!(!gate.get());
        vm.retype_row(PlanRowKey(2), CreateType::Scene);
        assert!(gate.get(), "retyping it is the other");
    }

    /// The strip's rows are the view-model's own, written on every diagnostics
    /// change — the view used to mirror them from a side effect inside a mapped
    /// signal, which is how a headline said "3 things to know" over an empty box.
    #[test]
    fn the_diagnostic_rows_follow_every_change() {
        let vm = vm();
        let rows = vm.diagnostic_rows();
        assert_eq!(rows.len(), vm.diagnostic_messages().len());

        // A retype adds an illegal-combination entry; the rows must gain it
        // without anyone reading a signal to make it happen.
        let before = rows.len();
        vm.retype_row(PlanRowKey(2), CreateType::Part);
        assert_eq!(rows.len(), before + 1);
        assert_eq!(rows.len(), vm.diagnostic_messages().len());

        // And the headline counts exactly what the list shows — infos included.
        let (errors, others) = vm.diagnostic_counts();
        assert_eq!(errors + others, rows.len());

        vm.reset();
        assert_eq!(rows.len(), 0, "a reset empties the strip too");
    }

    /// A row the writer unticked is never created, so it cannot block anything.
    #[test]
    fn an_excluded_row_does_not_block_the_import() {
        let vm = vm();
        vm.retype_row(PlanRowKey(2), CreateType::Part);
        assert!(!vm.blocking_rows().is_empty());

        vm.set_included(PlanRowKey(2), false);
        assert!(
            vm.blocking_rows().is_empty(),
            "excluding the row removes the reason to refuse"
        );
    }

    /// The same trap reached the other way: a *bulk* level rule can make a whole
    /// level illegal at once, and it must be caught the same way a single retype is.
    #[test]
    fn a_bulk_level_rule_that_breaks_rows_stops_the_import_too() {
        let vm = vm();
        vm.set_level_rule(3, CreateType::Part);
        assert_eq!(
            vm.blocking_rows(),
            vec!["Scene A".to_string(), "Scene B".to_string()],
            "every row the rule touched"
        );
        assert!(!vm.can_apply());
    }

    #[test]
    fn diagnostics_are_read_worst_first() {
        let vm = vm();
        vm.set_diagnostics(vec![
            Diagnostic {
                key: "no-headings".into(),
                severity: "info".into(),
                path: "/tmp/c.md".into(),
                detail: String::new(),
                count: 0,
                row: None,
            },
            Diagnostic {
                key: "footnotes-degraded".into(),
                severity: "warning".into(),
                path: "/tmp/b.md".into(),
                detail: String::new(),
                count: 1,
                row: None,
            },
            Diagnostic {
                key: "file-unreadable".into(),
                severity: "error".into(),
                path: "/tmp/a.md".into(),
                detail: "denied".into(),
                count: 0,
                row: None,
            },
        ]);

        let severities: Vec<String> = vm
            .diagnostic_messages()
            .into_iter()
            .map(|(s, _)| s)
            .collect();
        assert_eq!(severities, vec!["error", "warning", "info"]);
        // One error, and two entries that are not errors. The second number is
        // everything the list shows below the headline, not the `warning`
        // severity alone — a headline that counted warnings only said "1 thing to
        // know" above three visible sentences.
        assert_eq!(vm.diagnostic_counts(), (1, 2));
    }

    /// A row-scoped diagnostic names its row, and the sentence is built from the
    /// row's own title and type rather than from a second copy on the wire.
    #[test]
    fn a_row_scoped_diagnostic_reads_its_row_for_the_words_it_needs() {
        let vm = vm();
        vm.retype_row(PlanRowKey(1), CreateType::Part);
        vm.set_diagnostics(vec![Diagnostic {
            key: "illegal-combination".into(),
            severity: "warning".into(),
            path: String::new(),
            detail: String::new(),
            count: 0,
            row: Some(PlanRowKey(1)),
        }]);

        assert_eq!(vm.diagnostics_for_row(PlanRowKey(1)).len(), 1);
        assert!(vm.diagnostics_for_row(PlanRowKey(0)).is_empty());

        with_real_messages(|| {
            let (_, message) = vm.diagnostic_messages().remove(0);
            let text = message.resolve_now();
            assert!(
                text.contains("Chapter One"),
                "the sentence must name the row: {text:?}"
            );
            assert!(
                text.contains("Part"),
                "…and the type it was retyped to: {text:?}"
            );
        });
    }

    /// The whole analysis half, end to end against the real backend: two
    /// Markdown files on disk → `analyze_document_import` → the plan on the
    /// review step, with the structure the headings imply.
    ///
    /// The one test that would catch the wiring being wrong rather than the
    /// logic: the DTO round-trip, the level inference, the step move and the
    /// event filter all have to agree, and each of them compiles fine alone
    /// while disagreeing with the others.
    #[test]
    fn a_real_analysis_lands_a_plan_on_the_review_step() {
        use frontend::commands::{handling_app_lifecycle_commands, work_management_commands};
        use frontend::work_management::{NewWorkDto, NewWorkTemplate};

        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("01-one.md"),
            "# The Book\n\n## Chapter One\n\nIt began.\n\n* * *\n\nAnd continued.\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("02-two.md"),
            "## Chapter Two\n\nIt ended.\n",
        )
        .unwrap();

        let app_ctx = Rc::new(AppContext::new());
        handling_app_lifecycle_commands::initialize_app(&app_ctx).unwrap();
        work_management_commands::new_work(
            &app_ctx,
            &NewWorkDto {
                file_name: dir.path().join("p.skrib").to_string_lossy().into_owned(),
                is_folder: false,
                template_kind: NewWorkTemplate::Novel,
                labels: vec![],
                language: vec!["en".to_string()],
                author_name: String::new(),
                chapter_scene_mode: false,
                paratext_front: Vec::new(),
                paratext_back: Vec::new(),
            },
        )
        .unwrap();
        let work_id = frontend::commands::work_commands::get_all_work(&app_ctx)
            .unwrap()
            .first()
            .map(|w| w.id)
            .expect("the work the test just created");

        let ids = AppIds::default();
        ids.work_id.set(Some(work_id));
        let vm = ImportDocumentViewModel::new(app_ctx.clone(), ids);
        vm.add_files([dir.path().join("01-one.md"), dir.path().join("02-two.md")]);

        let op = vm.start_analysis().expect("the analysis starts");
        // The Stepper footer's Next advances after `validate_on_next` returns true;
        // tests that call `start_analysis` directly must do the same jump — into
        // Review, which shows the progress UI while `busy`.
        vm.controller().next();
        assert_eq!(vm.step().get(), STEP_REVIEW);
        assert!(vm.busy().get(), "analysis is in flight on the review step");

        // A long operation runs on its own thread; poll for its result rather
        // than sleeping a guessed interval.
        let deadline = std::time::Instant::now() + Duration::from_secs(20);
        loop {
            let done =
                frontend::commands::import_management_commands::get_analyze_document_import_result(
                    &app_ctx, &op,
                )
                .ok()
                .flatten()
                .is_some();
            if done {
                break;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "analysis never finished"
            );
            std::thread::sleep(Duration::from_millis(20));
        }

        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        tree.run_with_event_context(&mut teksilo::core::NoopWindowOps, |ctx| {
            vm.on_long_op_completed(ctx, &long_op_event(&op));
        });

        assert_eq!(vm.step().get(), STEP_REVIEW, "the plan must be on screen");
        assert!(!vm.busy().get());
        assert_eq!(vm.active_op_id(), None, "the op is no longer in flight");

        let titles: Vec<String> = vm.plan().rows().iter().map(|r| r.title.clone()).collect();
        assert_eq!(titles, vec!["The Book", "Chapter One", "Chapter Two"]);

        // The heading ladder became indents, and the rule table agrees with it.
        let indents: Vec<i64> = vm.plan().rows().iter().map(|r| r.indent).collect();
        assert_eq!(indents, vec![0, 1, 1]);
        assert_eq!(
            vm.level_rules().get(),
            vec![(1, CreateType::Book), (2, CreateType::Chapter)]
        );

        // The `* * *` is preserved inside its chapter's prose and counted, not
        // split into a second row — the design decision this whole feature
        // rests on.
        let breaks: usize = vm.plan().rows().iter().map(|r| r.scene_breaks).sum();
        assert_eq!(
            breaks, 1,
            "the scene break is reported, not silently dropped"
        );
    }

    /// Build a `.skrib` holding an imported `.docx`, comments and all, at
    /// `$SKRIBISTO_IMPORT_FIXTURE_OUT`.
    ///
    /// `#[ignore]` because it is a tool, not an assertion: it exists so a real
    /// project carrying imported comments can be opened in the running app. The
    /// store-level correctness is asserted by `skribisto-import-management`'s
    /// integration tests; what only the live app can show is whether an imported
    /// comment *renders* — in the margin beside the sentence it names, and in the
    /// comments dock — which is exactly the failure the anchor module's own doc
    /// calls invisible until someone's comment has moved to the wrong sentence.
    ///
    ///     SKRIBISTO_IMPORT_FIXTURE_OUT=/tmp/imported.skrib \
    ///       cargo test -p teksilo_ui a_docx_import_saved_as_a_project -- --ignored --nocapture
    #[test]
    #[ignore = "a fixture builder for live inspection, not a check"]
    fn a_docx_import_saved_as_a_project() {
        use frontend::commands::{handling_app_lifecycle_commands, work_management_commands};
        use frontend::work_management::{NewWorkDto, NewWorkTemplate, SaveWorkDto};

        let out = std::env::var("SKRIBISTO_IMPORT_FIXTURE_OUT")
            .expect("set SKRIBISTO_IMPORT_FIXTURE_OUT to the .skrib to write");
        let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../document_ingest/tests/fixtures/word-shaped.docx");
        assert!(
            source.exists(),
            "run document_ingest's fixtures/generate.py"
        );

        let app_ctx = Rc::new(AppContext::new());
        handling_app_lifecycle_commands::initialize_app(&app_ctx).unwrap();
        work_management_commands::new_work(
            &app_ctx,
            &NewWorkDto {
                file_name: out.clone(),
                is_folder: false,
                // The emptiest template there is: the point of this fixture is to
                // look at what the *import* produced, and a dozen seeded chapters
                // above it are a dozen rows of noise.
                template_kind: NewWorkTemplate::EmptyNovel,
                labels: vec![],
                language: vec!["en".to_string()],
                author_name: "Cyril".into(),
                chapter_scene_mode: false,
                paratext_front: Vec::new(),
                paratext_back: Vec::new(),
            },
        )
        .unwrap();
        let work_id = frontend::commands::work_commands::get_all_work(&app_ctx)
            .unwrap()
            .first()
            .map(|w| w.id)
            .expect("the work just created");

        let ids = AppIds::default();
        ids.work_id.set(Some(work_id));
        let vm = ImportDocumentViewModel::new(app_ctx.clone(), ids);
        vm.add_files([source]);

        let op = vm.start_analysis().expect("the analysis starts");
        let deadline = std::time::Instant::now() + Duration::from_secs(30);
        loop {
            if frontend::commands::import_management_commands::get_analyze_document_import_result(
                &app_ctx, &op,
            )
            .ok()
            .flatten()
            .is_some()
            {
                break;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "analysis never finished"
            );
            std::thread::sleep(Duration::from_millis(20));
        }

        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        tree.run_with_event_context(&mut teksilo::core::NoopWindowOps, |ctx| {
            vm.on_long_op_completed(ctx, &long_op_event(&op));
        });
        assert_eq!(vm.step().get(), STEP_REVIEW);

        let carried: usize = vm.plan().rows().iter().map(|r| r.comments.len()).sum();
        assert_eq!(carried, 2, "two comment threads reached the review step");

        // Land it in the first binder, at the end. The picker is keyed by durable
        // uid, so the binder's own uid is the destination.
        let binder_id = frontend::commands::work_commands::get_work_relationship(
            &app_ctx,
            &work_id,
            &frontend::common::direct_access::work::WorkRelationshipField::Binders,
        )
        .unwrap()
        .first()
        .copied()
        .expect("the template made a binder");
        let binder_uid = frontend::commands::binder_commands::get_binder(&app_ctx, &binder_id)
            .unwrap()
            .expect("binder row")
            .uid;
        vm.destination().reload();
        vm.destination()
            .preselect(crate::models::BinderTreeKey::Binder(binder_uid));
        let created = vm.apply().expect("apply");
        assert!(!created.is_empty());

        let path = work_management_commands::save_work(
            &app_ctx,
            &SaveWorkDto {
                media_root: crate::media_paths::media_root_string(),
                work_id,
                file_name: out.clone(),
                overwrite: true,
            },
        )
        .expect("start save");
        // `save_work` is a long operation; wait for the file to appear.
        let deadline = std::time::Instant::now() + Duration::from_secs(30);
        while !std::path::Path::new(&out).exists() {
            assert!(std::time::Instant::now() < deadline, "save never finished");
            std::thread::sleep(Duration::from_millis(50));
        }
        println!("WROTE {out} (op {path})");
    }
}
