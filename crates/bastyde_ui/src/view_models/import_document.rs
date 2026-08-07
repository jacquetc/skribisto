// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ImportDocumentViewModel` — the Import documents wizard's business logic.
//!
//! Two steps and two use cases. Step one collects files; `analyze_document_import`
//! turns them into a plan without writing anything. Step two shows that plan as a
//! tree the writer can retype, exclude from and re-anchor, and only then does
//! `apply_document_import` create what is left, as one undo entry.
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
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

use bastyde::data::ListModel;
use bastyde::prelude::*;
use bastyde::widgets::{MessageBox, MessageBoxButtons, Toast, ToastAction};

use frontend::AppContext;
use frontend::commands::{long_operation_commands, undo_redo_commands};
use frontend::common::event::Event;
use frontend::import_management::{
    AnalyzeDocumentImportDto, ApplyDocumentImportDto, ApplyImportRow, ApplyImportRows,
    DocumentImportRow, DocumentImportRows, ImportDiagnosticRow, ImportDiagnosticRows,
    ImportRowKind,
};

use document_ingest::plan::PlannedRow;
use document_ingest::{ImportPlan, ScannerRegistry};
use skribisto_model::CreateType;

use super::long_op::{TrackedOp, event_id, parse_payload, payload_id};
use crate::app_ids::AppIds;
use crate::models::import_plan_source::{ImportPlanSource, PlanRowKey};
use crate::toast_scope::ToastWorkExt;
use crate::widgets::DestinationPicker;

/// Which step of the wizard is on screen. A plain index, because the panel binds
/// a `Switcher` to it — keep these in lock-step with the panel's `Switcher`
/// children, which are positional.
pub const STEP_FILES: usize = 0;
pub const STEP_REVIEW: usize = 1;
/// The analysis is running. Its own step rather than a toast over the modal: the
/// wizard is already the writer's whole attention, and a progress surface behind
/// a dialog that blocks it would be chrome nobody can act on.
pub const STEP_ANALYSING: usize = 2;

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
    step: Signal<usize>,

    /// The analysed plan, as a tree.
    plan: ImportPlanSource,
    diagnostics: Signal<Vec<Diagnostic>>,

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
}

impl ImportDocumentViewModel {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        let destination = DestinationPicker::new(app_ctx.clone(), ids.work_id.clone());
        Self {
            app_ctx,
            ids,
            files: ListModel::new(),
            file_count: Signal::new(0),
            step: Signal::new(STEP_FILES),
            plan: ImportPlanSource::empty(),
            diagnostics: Signal::new(Vec::new()),
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
    pub fn step(&self) -> Signal<usize> {
        self.step.clone()
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
        self.step.set(STEP_FILES);
        self.plan.set_plan(&ImportPlan::default());
        self.diagnostics.set(Vec::new());
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
        !self.files.is_empty() && self.ids.work_id.get().is_some() && !self.busy.get()
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

        let dto = AnalyzeDocumentImportDto {
            work_id,
            binder_id: self
                .destination
                .selected()
                .map(|d| d.binder_id)
                .unwrap_or(0),
            source_paths: self
                .file_paths()
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect(),
            start_kind: ImportRowKind::Book,
            base_indent: 0,
        };
        let op = frontend::commands::import_management_commands::analyze_document_import(
            &self.app_ctx,
            &dto,
        )?;
        *self.active.borrow_mut() = Some(TrackedOp::start(&self.ids, op.clone()));
        self.busy.set(true);
        self.progress.set(0.0);
        self.progress_message.set(String::new());
        self.step.set(STEP_ANALYSING);
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
        self.step.set(STEP_FILES);
    }

    // ── step two: the plan ──────────────────────────────────────────────────

    /// Take a freshly analysed plan and move to the review step.
    pub fn on_plan_ready(&self, plan: &ImportPlan, levels: Vec<u8>, rules: Vec<(u8, CreateType)>) {
        self.plan.set_plan(plan);
        self.row_levels.set(levels);
        self.level_rules.set(rules);
        self.pinned.set(Vec::new());
        self.diagnostics.set(Vec::new());
        *self.active.borrow_mut() = None;
        self.busy.set(false);
        self.progress.set(0.0);
        self.progress_message.set(String::new());
        self.step.set(STEP_REVIEW);
    }

    pub fn set_diagnostics(&self, diagnostics: Vec<Diagnostic>) {
        self.diagnostics.set(diagnostics);
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
            all.iter().filter(|d| d.is_warning()).count(),
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
    }

    pub fn is_pinned(&self, key: PlanRowKey) -> bool {
        self.pinned.get().contains(&key)
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

    pub fn can_apply(&self) -> bool {
        self.included_count() > 0
            && self.ids.work_id.get().is_some()
            && self.destination.selected().is_some()
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
                row: ApplyImportRow::Empty,
                rows: ApplyImportRows::Create(rows),
            },
        )?;
        Ok(result.created_ids)
    }

    /// The accepted rows, in plan order, carrying whatever the writer retyped.
    ///
    /// Public so a test can assert on exactly what would be created without
    /// needing a store behind it — the thing that is expensive to check any other
    /// way, and the thing a mistake here would silently get wrong.
    pub fn rows_to_create(&self) -> Vec<ApplyImportRow> {
        self.plan
            .keys_in_order()
            .into_iter()
            .filter(|k| self.is_included(*k))
            .filter_map(|key| {
                let row = self.plan.row(key)?;
                let kind = self.plan.type_of(key)?;
                Some(ApplyImportRow::Create {
                    indent: row.indent,
                    kind: create_type_to_kind(kind),
                    title: row.title,
                    djot: row.djot,
                })
            })
            .collect()
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
        self.step.set(STEP_FILES);
    }
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
                origin,
                included,
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
                origin: origin.clone(),
                included: *included,
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
        use bastyde::i18n::config::I18nConfig;
        use bastyde::i18n::manager::I18nManager;
        use bastyde::i18n::thread_local::{clear, install};

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
            diagnostics: Vec::new(),
        }
    }

    /// Book / Chapter One (Scene A, Scene B) / Chapter Two, from heading levels
    /// 1, 2, 3, 3, 2.
    fn vm() -> ImportDocumentViewModel {
        let vm = ImportDocumentViewModel::new(Rc::new(AppContext::new()), AppIds::default());
        let plan = ImportPlan {
            rows: vec![
                planned(0, "Book", CreateType::Book),
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
                ApplyImportRow::Empty => None,
            })
            .collect()
    }

    #[test]
    fn a_ready_plan_moves_to_the_review_step() {
        let vm = vm();
        assert_eq!(vm.step().get(), STEP_REVIEW);
        assert_eq!(vm.included_count(), 5);
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

    /// The picker and the drop zone must offer exactly what a scanner can read.
    #[test]
    fn the_accepted_extensions_come_from_the_scanners_themselves() {
        let extensions = ImportDocumentViewModel::accepted_extensions();
        for expected in ["md", "markdown", "txt"] {
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
                planned(0, "Book", CreateType::Book),
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
        tree.run_with_event_context(&mut bastyde::core::NoopWindowOps, |ctx| {
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

    /// **Every** diagnostic `document_ingest` can raise must reach a translated
    /// sentence.
    ///
    /// The list is built from the real `ImportDiagnostic` variants, mapped
    /// through the real DTO mapper, so a fifteenth variant added upstream fails
    /// here rather than silently landing in the fallback arm — which is what a
    /// hand-written list of key strings would have allowed. Fourteen collected
    /// diagnostics that nothing rendered is the bug this whole surface exists to
    /// close; a fifteenth quietly joining them would be the same bug again.
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
        ];

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
        assert_eq!(vm.diagnostic_counts(), (1, 1));
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
        assert_eq!(vm.step().get(), STEP_ANALYSING);

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
        tree.run_with_event_context(&mut bastyde::core::NoopWindowOps, |ctx| {
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
}
