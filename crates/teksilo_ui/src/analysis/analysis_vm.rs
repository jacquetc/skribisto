// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Drives the Analysis segment: runs `analyze_book`, holds its result, and knows whether
//! that result still describes the manuscript on screen.
//!
//! One instance per open container tab, like `StreamViewModel` and `OverviewViewModel` — a
//! Book and a Part opened side by side are two analyses of two different scopes, and sharing
//! one would make the second overwrite the first.
//!
//! ## Manual, not live
//!
//! Every existing long operation in this app is user-triggered and one-shot, and analysis is
//! the one that most needs to stay that way: an all-pairs scene comparison on every keystroke
//! is exactly the runaway recompute the long-operation layer was never built to coalesce. So
//! there is a Run button, plus a single automatic run the first time a scope's Analysis
//! segment is opened with no result yet — mirroring Pace's own empty state rather than making
//! the writer click to see a report they just navigated to.
//!
//! ## Staleness is derived, not tracked
//!
//! Freshness reuses the same monotonic counter the save indicator already trusts: capture
//! `dirty_seq` when a run completes, and the view is stale whenever the live value has moved
//! past it. Inventing a second "has the manuscript changed" signal would give the status bar
//! and this panel two answers that can disagree.
//!
//! ## The footnote-word figure is a second, independent long operation
//!
//! Shape shows how many words live in this book's footnotes, kept apart from the manuscript
//! total for the same reason `progress_management::count_words_uc` keeps the two apart in the
//! first place: a footnote is authored prose, but folding it into the manuscript total would
//! make a heavily annotated chapter look like it made progress the story did not. That figure
//! cannot come from `analyze_book` — `analyze_book_uc` deliberately reads no `Footnote` rows
//! at all, on the grounds that a note is the author's aside on the manuscript, not part of the
//! shape being measured. The only real source is the same `count_words` pass
//! `ProgressRecorder` already fires on every save, which buckets its footnote total **per
//! Book** — exactly this panel's own scope — so `run()` fires it as a second, independently
//! tracked operation alongside `analyze_book`, and [`AnalysisViewModel::footnote_words`] picks this
//! scope's own entry out of the whole-Work result once it lands.

use std::cell::RefCell;
use std::rc::Rc;

use frontend::AppContext;
use frontend::commands::{analysis_management_commands, progress_management_commands};
use frontend::common::event::{Event, Origin};
use teksilo::prelude::*;
use teksilo::widgets::SegmentId;

use frontend::analysis_management::{AnalyzeBookDto, BookAnalysisResultDto};
use frontend::progress_management::{CountWordsDto, WordCountResultDto};

use crate::app_ids::AppIds;
use crate::shared::long_op::{TrackedOp, event_id, parse_payload};

/// Which analysis the panel is showing.
///
/// **One built-in**, and the bar is not built from this list alone: `tabs::analysis`
/// composes it with whatever [`crate::tabs::analysis::register_category`] has been handed,
/// so the enum is the set of categories *this application ships*, not the set the writer
/// sees.
///
/// Shape is that one, and it absorbs both the per-scene rhythm strands and the
/// chapter-balance chart, because "how is this book distributed" is one question however
/// many numbers answer it. Three others — Repetition, Synopsis and Voice — were built in
/// here once and are not any more; the pane that hosts them is the registry, and it does
/// not care whether a category is compiled in or contributed.
///
/// There is deliberately no Mentions category. Mention coverage is real and useful, but it
/// is the mention feature's data, already surfaced by the Inspector's Cast and backlinks —
/// and `ContentTab` carries no `MentionIndex`, so adding it here would mean threading a
/// whole view-model through the tab to duplicate a view that already exists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnalysisCategory {
    Shape = 0,
}

impl AnalysisCategory {
    pub const ALL: [AnalysisCategory; 1] = [Self::Shape];

    /// The category at a bar position. `None` for an out-of-range index, so a widening of
    /// the bar without a matching arm mounts nothing rather than silently showing a
    /// neighbour — the positional trap this codebase has been bitten by before.
    ///
    /// ⚠ Indexes the **built-ins only**. The bar itself is
    /// `tabs::analysis::all_categories()`, which appends registered categories after
    /// these, so an index past the built-ins is a registered one and yields `None` here.
    /// That is the honest answer for a function whose return type is this enum — nothing
    /// in it can name a category it has never heard of.
    pub fn from_index(i: usize) -> Option<Self> {
        Self::ALL.get(i).copied()
    }

    /// Stable identifier, not shown to the writer.
    ///
    /// Deliberately unprefixed where a registered category is namespaced (`"ext.style"`):
    /// the built-ins are the ones a registration may not shadow, so their ids read as the
    /// reserved words they are.
    pub fn id(self) -> &'static str {
        match self {
            Self::Shape => "shape",
        }
    }

    /// The bar label. Resolved per call so a runtime locale switch reaches it.
    pub fn label(self) -> LocalizedString {
        match self {
            Self::Shape => tr!(analysis_shape()),
        }
    }
}

/// What the panel is doing right now.
#[derive(Clone, Debug, PartialEq)]
pub enum AnalysisState {
    /// Never run for this scope.
    Idle,
    Running,
    Ready(Rc<BookAnalysisResultDto>),
    /// The run failed. A failed analysis is stated plainly rather than left as an empty
    /// report, which would read as "your book is fine".
    Failed(String),
}

#[derive(Clone)]
pub struct AnalysisViewModel {
    ctx: Rc<AppContext>,
    ids: AppIds,
    /// The container this analysis is scoped to — the tab's own item.
    scope_item_id: u64,
    state: Signal<AnalysisState>,
    /// Which category the bar has selected, **keyed** rather than positional.
    ///
    /// A category may be contributed by another crate (see
    /// `tabs::analysis::register_category`), so the list can gain and lose entries
    /// between rebuilds. An index would silently re-point at a neighbour the moment one
    /// registered ahead of the selected one; a `SegmentId` derived from the category's
    /// stable string id cannot. `None` is "nothing chosen yet", which the bar resolves to
    /// its first segment.
    category: Signal<Option<SegmentId>>,
    /// `dirty_seq` as of the last completed run. `None` until one completes.
    analysed_at_seq: Signal<Option<u64>>,
    /// The live edit counter, shared with the save indicator.
    dirty_seq: Signal<u64>,
    pending: Rc<RefCell<Option<TrackedOp>>>,
    /// This book's own footnote-word count, picked out of the whole-Work `count_words`
    /// result. `None` covers every state that is not "a real figure is in hand" — never
    /// run yet, still counting, or the companion operation failed — so the pane can show
    /// "still counting" rather than a `0` that would read as a finding about the book
    /// rather than as the panel not knowing yet.
    footnote_words: Signal<Option<i64>>,
    /// The in-flight `count_words` operation backing [`Self::footnote_words`] — tracked
    /// separately from `pending` because it is a genuinely separate long operation (see
    /// the module doc), with its own completion/failure/cancellation and its own
    /// `Origin::LongOperation` events to filter by id.
    footnote_pending: Rc<RefCell<Option<TrackedOp>>>,
    /// Guards the one automatic first run, so re-selecting the segment does not re-run.
    auto_ran: Rc<std::cell::Cell<bool>>,
    /// Whether Shape leaves texts with no prose out of its charts. On by default: a Part
    /// heading or an unwritten scene is a real row of the binder but a zero-height gap in a
    /// bar chart, and a book outlined ahead of its drafting is mostly gaps.
    ///
    /// A view-model field rather than pane-local state so it survives switching categories
    /// and re-running the analysis — the writer set it once, about this book.
    ignore_empty: Signal<bool>,
}

impl std::fmt::Debug for AnalysisViewModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnalysisViewModel")
            .field("scope_item_id", &self.scope_item_id)
            .field("state", &self.state.get())
            .finish()
    }
}

impl AnalysisViewModel {
    pub fn new(
        ctx: Rc<AppContext>,
        ids: AppIds,
        scope_item_id: u64,
        dirty_seq: Signal<u64>,
    ) -> Self {
        Self {
            ctx,
            ids,
            scope_item_id,
            state: Signal::new(AnalysisState::Idle),
            category: Signal::new(None),
            analysed_at_seq: Signal::new(None),
            dirty_seq,
            pending: Rc::new(RefCell::new(None)),
            footnote_words: Signal::new(None),
            footnote_pending: Rc::new(RefCell::new(None)),
            auto_ran: Rc::new(std::cell::Cell::new(false)),
            ignore_empty: Signal::new(true),
        }
    }

    /// Whether Shape hides texts with no prose. See [`Self::ignore_empty`].
    pub fn ignore_empty(&self) -> Signal<bool> {
        self.ignore_empty.clone()
    }

    pub fn state(&self) -> Signal<AnalysisState> {
        self.state.clone()
    }

    /// This book's own footnote-word count — `None` until the companion `count_words`
    /// operation this scope's [`Self::run`] fires has actually landed. See the module
    /// doc for why this is a second operation rather than a field on `analyze_book`'s
    /// own result.
    pub fn footnote_words(&self) -> Signal<Option<i64>> {
        self.footnote_words.clone()
    }

    pub fn category(&self) -> Signal<Option<SegmentId>> {
        self.category.clone()
    }

    /// The backend handle and the app's entity ids.
    ///
    /// `pub` for the `analysis.category` slot: a registered category is handed
    /// this view-model and the finished analysis, so without these it can render
    /// only from its own state. Capturing an `AppContext` at registration time is
    /// not the way round it — the app builds its own inside `run`, so an
    /// extension that made one early would read a second, permanently empty
    /// store. Same reasoning as [`crate::tabs::ContentTab::ids`].
    pub fn app_ctx(&self) -> Rc<AppContext> {
        self.ctx.clone()
    }

    /// See [`Self::app_ctx`].
    pub fn ids(&self) -> &AppIds {
        &self.ids
    }

    pub fn scope_item_id(&self) -> u64 {
        self.scope_item_id
    }

    /// The edit counter the displayed result was produced at, or `None` before any run
    /// has completed.
    ///
    /// `pub` for the `analysis.category` slot, alongside [`Self::app_ctx`] and
    /// [`Self::ids`]. A registered category that measures the manuscript itself — rather
    /// than reading the result it is handed — needs to know *which* manuscript state the
    /// pane is currently reporting on, so that its own measurement can be produced for the
    /// same one and refreshed by the same Run button.
    ///
    /// This value is the honest key for that. It moves **only when a run completes**, so
    /// re-running with nothing edited in between leaves it alone (correctly: nothing
    /// changed), and it never moves backwards. Comparing a stored copy against it answers
    /// "is what I computed still about the book this pane is showing" with no second
    /// freshness notion to disagree with [`Self::is_stale`].
    ///
    /// Deliberately not the live `dirty_seq`: that moves on every keystroke, and a category
    /// keyed to it would re-measure the manuscript continuously — the runaway recompute
    /// this whole panel is manual to avoid.
    pub fn analysed_at_seq(&self) -> Option<u64> {
        self.analysed_at_seq.get()
    }

    /// Whether the manuscript has changed since the displayed result was produced.
    ///
    /// False while nothing has been run: "stale" is a claim about a result, and there is no
    /// result to be stale.
    pub fn is_stale(&self) -> bool {
        match self.analysed_at_seq.get() {
            Some(at) => self.dirty_seq.get() > at,
            None => false,
        }
    }

    /// True while `analyze_book` **or** the companion `count_words` is in flight — a Run
    /// pressed while only the footnote figure is still catching up must not fire a second
    /// full `analyze_book` pass for an answer that is already on the way.
    pub fn is_running(&self) -> bool {
        matches!(self.state.get(), AnalysisState::Running)
            || self.footnote_pending.borrow().is_some()
    }

    /// Start an analysis of this scope, unless one is already in flight.
    ///
    /// Re-entrancy matters here: each concurrent long operation is a full independent
    /// manuscript walk with no request coalescing at the manager level, so a Run button
    /// pressed twice would run the whole pass twice for one answer.
    pub fn run(&self) {
        if self.is_running() {
            return;
        }
        let Some(work_id) = self.ids.work_id.get() else {
            return;
        };
        match analysis_management_commands::analyze_book(
            &self.ctx,
            &AnalyzeBookDto {
                work_id,
                scope_item_id: self.scope_item_id,
            },
        ) {
            Ok(op_id) => {
                // The Work is captured now, not read back on completion: an in-place
                // project switch reseeds the very `AppIds` signal this view-model holds,
                // so a live read at event time answers with whichever Work the window
                // shows then — not the one this analysis actually ran for.
                *self.pending.borrow_mut() = Some(TrackedOp::start(&self.ids, op_id));
                self.state.set(AnalysisState::Running);
                self.start_footnote_count(work_id);
            }
            Err(e) => self.state.set(AnalysisState::Failed(e.to_string())),
        }
    }

    /// Fire the companion `count_words` long operation that supplies
    /// [`Self::footnote_words`] — see the module doc for why this is a second operation
    /// rather than a field `analyze_book` fills in.
    fn start_footnote_count(&self, work_id: u64) {
        // Stale until *this* run's own answer lands — a re-run must not go on showing
        // the previous scope's (or the previous edit's) figure while the new pass reads
        // the manuscript.
        self.footnote_words.set(None);
        // A figure this panel could not even ask for is shown the same way as one
        // still in flight — "still counting" rather than a `0` that would claim to
        // be a finding about the book — so a request failure is silently dropped here.
        if let Ok(op_id) =
            progress_management_commands::count_words(&self.ctx, &CountWordsDto { work_id })
        {
            *self.footnote_pending.borrow_mut() = Some(TrackedOp::start(&self.ids, op_id));
        }
    }

    /// The single automatic run, the first time this scope's panel is shown.
    pub fn run_once_on_open(&self) {
        if self.auto_ran.replace(true) {
            return;
        }
        if matches!(self.state.get(), AnalysisState::Idle) {
            self.run();
        }
    }

    /// Feed a `Origin::LongOperation(...)` event in. Ignores anything that is not one of
    /// this view-model's own two in-flight operations (`analyze_book` or the companion
    /// `count_words` — see the module doc).
    pub fn on_long_op_event(&self, event: &Event) {
        let Origin::LongOperation(kind) = &event.origin else {
            return;
        };
        let id = event_id(event);

        let matches_main = {
            let pending = self.pending.borrow();
            match (pending.as_ref(), id.as_deref()) {
                (Some(op), Some(id)) => op.matches(id),
                _ => false,
            }
        };
        if matches_main {
            use frontend::common::event::LongOperationEvent as L;
            match kind {
                L::Completed => {
                    let op_id = id.unwrap_or_default();
                    match analysis_management_commands::get_analyze_book_result(&self.ctx, &op_id) {
                        Ok(Some(dto)) => {
                            // Recorded at completion rather than at start: edits made
                            // *during* the pass are not covered by it, and claiming
                            // otherwise would show a fresh badge over a result that
                            // predates them.
                            self.analysed_at_seq.set(Some(self.dirty_seq.get()));
                            self.state.set(AnalysisState::Ready(Rc::new(dto)));
                        }
                        Ok(None) => self.state.set(AnalysisState::Failed(String::new())),
                        Err(e) => self.state.set(AnalysisState::Failed(e.to_string())),
                    }
                    *self.pending.borrow_mut() = None;
                }
                L::Failed => {
                    let message = parse_payload(event)
                        .and_then(|p| p.get("error").and_then(|e| e.as_str().map(str::to_string)))
                        .unwrap_or_default();
                    self.state.set(AnalysisState::Failed(message));
                    *self.pending.borrow_mut() = None;
                }
                L::Cancelled => {
                    // Back to whatever was on screen before, not to an error: a cancel is
                    // the writer's own decision and is not a failure to report back to them.
                    self.state.set(AnalysisState::Idle);
                    *self.pending.borrow_mut() = None;
                }
                _ => {}
            }
            return;
        }

        let matches_footnote = {
            let pending = self.footnote_pending.borrow();
            match (pending.as_ref(), id.as_deref()) {
                (Some(op), Some(id)) => op.matches(id),
                _ => false,
            }
        };
        if !matches_footnote {
            return;
        }
        use frontend::common::event::LongOperationEvent as L;
        match kind {
            L::Completed => {
                let op_id = id.unwrap_or_default();
                // Any failure to fetch a real result (an unknown op id, a store error) is
                // folded into `None` alongside "still running" — the pane cannot tell
                // those apart from a figure it simply does not have yet, and should not
                // pretend it can.
                let count = progress_management_commands::get_count_words_result(&self.ctx, &op_id)
                    .ok()
                    .flatten()
                    .map(|result| footnote_words_in_book(&result, self.scope_item_id));
                self.footnote_words.set(count);
                *self.footnote_pending.borrow_mut() = None;
            }
            L::Failed | L::Cancelled => {
                self.footnote_words.set(None);
                *self.footnote_pending.borrow_mut() = None;
            }
            _ => {}
        }
    }
}

/// Pick this book's own footnote-word figure out of a whole-Work `count_words` result.
///
/// `book_item_ids` and `book_footnote_word_counts` are kept aligned position-for-position
/// by `progress_management::count_words_uc::fold_counts` — this is the read half of that
/// contract. A `scope_item_id` absent from the roster is not an unknown figure: per
/// `fold_counts`'s own doc, a book is only left out of the roster when it has neither
/// counted prose nor a counted note, which for the footnote figure specifically just
/// means "no footnotes in this book" — `0`, not "not computed".
fn footnote_words_in_book(result: &WordCountResultDto, scope_item_id: u64) -> i64 {
    result
        .book_item_ids
        .iter()
        .position(|&id| id == scope_item_id)
        .map(|idx| result.book_footnote_word_counts[idx])
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vm(dirty: Signal<u64>) -> AnalysisViewModel {
        AnalysisViewModel::new(Rc::new(AppContext::new()), AppIds::new(), 42, dirty)
    }

    /// On by default. A book outlined ahead of its drafting is mostly unwritten texts, and
    /// a chart that is mostly gaps says nothing about the part that *has* been written.
    #[test]
    fn empty_texts_are_hidden_until_the_writer_says_otherwise() {
        assert!(vm(Signal::new(0)).ignore_empty().get());
    }

    #[test]
    fn categories_map_to_bar_positions_and_stop_at_the_end() {
        assert_eq!(
            AnalysisCategory::from_index(0),
            Some(AnalysisCategory::Shape)
        );
        assert_eq!(
            AnalysisCategory::from_index(1),
            None,
            "a segment past the built-ins is a registered category, and nothing in this \
             enum can name one"
        );
    }

    /// Nothing has been measured, so nothing can be out of date.
    #[test]
    fn a_never_run_analysis_is_not_stale() {
        let vm = vm(Signal::new(7));
        assert!(!vm.is_stale());
        assert_eq!(vm.state().get(), AnalysisState::Idle);
    }

    #[test]
    fn editing_after_a_run_makes_the_result_stale() {
        let dirty = Signal::new(3);
        let vm = vm(dirty.clone());
        vm.analysed_at_seq.set(Some(3));
        assert!(!vm.is_stale(), "no edits since the run");
        dirty.set(4);
        assert!(vm.is_stale(), "an edit landed after the run completed");
    }

    // ── the freshness key a registered category measures against ───────────────

    /// `analysed_at_seq` is `None` until a run lands, and then names the manuscript state
    /// that run described — never the live one.
    ///
    /// A registered category that measures the manuscript itself keys its own cached
    /// result on this. Were it to track `dirty_seq` instead, every keystroke would
    /// invalidate that cache and the category would re-measure the whole book
    /// continuously.
    #[test]
    fn the_freshness_key_is_none_before_a_run_and_pins_the_analysed_state_after_one() {
        let dirty = Signal::new(3);
        let vm = vm(dirty.clone());
        assert_eq!(vm.analysed_at_seq(), None, "nothing has been measured yet");

        vm.analysed_at_seq.set(Some(3));
        assert_eq!(vm.analysed_at_seq(), Some(3));

        dirty.set(9);
        assert_eq!(
            vm.analysed_at_seq(),
            Some(3),
            "editing must not move the key — the displayed result still describes state 3, \
             and a category keyed to this must not re-measure on every keystroke"
        );
    }

    /// The counter is monotonic, so a result can never become fresh again by itself.
    #[test]
    fn staleness_does_not_un_stale_itself() {
        let dirty = Signal::new(10);
        let vm = vm(dirty.clone());
        vm.analysed_at_seq.set(Some(5));
        assert!(vm.is_stale());
        dirty.set(11);
        assert!(vm.is_stale());
    }

    #[test]
    fn the_automatic_first_run_happens_at_most_once() {
        let vm = vm(Signal::new(0));
        assert!(!vm.auto_ran.get());
        vm.run_once_on_open();
        assert!(vm.auto_ran.get(), "the first open arms it");
        // With no work_id the run itself is a no-op, but the guard must still have latched
        // so re-selecting the segment cannot queue a second pass.
        vm.run_once_on_open();
        assert!(vm.auto_ran.get());
    }

    #[test]
    fn an_event_for_another_operation_is_ignored() {
        let vm = vm(Signal::new(0));
        vm.state.set(AnalysisState::Running);
        *vm.pending.borrow_mut() = Some(TrackedOp::given(
            "ours".into(),
            crate::shared::long_op::CapturedWork::for_test(Some(1)),
        ));

        let foreign = Event {
            origin: Origin::LongOperation(frontend::common::event::LongOperationEvent::Cancelled),
            ids: vec![],
            data: Some(r#"{"id":"someone-elses"}"#.to_string()),
        };
        vm.on_long_op_event(&foreign);
        assert_eq!(
            vm.state().get(),
            AnalysisState::Running,
            "another feature's long operation must not touch this panel"
        );
    }

    #[test]
    fn cancelling_returns_to_idle_rather_than_reporting_a_failure() {
        let vm = vm(Signal::new(0));
        vm.state.set(AnalysisState::Running);
        *vm.pending.borrow_mut() = Some(TrackedOp::given(
            "op-1".into(),
            crate::shared::long_op::CapturedWork::for_test(Some(1)),
        ));

        let cancelled = Event {
            origin: Origin::LongOperation(frontend::common::event::LongOperationEvent::Cancelled),
            ids: vec![],
            data: Some(r#"{"id":"op-1"}"#.to_string()),
        };
        vm.on_long_op_event(&cancelled);
        assert_eq!(vm.state().get(), AnalysisState::Idle);
        assert!(
            vm.pending.borrow().is_none(),
            "the slot is released for the next run"
        );
    }

    #[test]
    fn a_failure_is_stated_rather_than_shown_as_an_empty_report() {
        let vm = vm(Signal::new(0));
        vm.state.set(AnalysisState::Running);
        *vm.pending.borrow_mut() = Some(TrackedOp::given(
            "op-2".into(),
            crate::shared::long_op::CapturedWork::for_test(Some(1)),
        ));

        let failed = Event {
            origin: Origin::LongOperation(frontend::common::event::LongOperationEvent::Failed),
            ids: vec![],
            data: Some(r#"{"id":"op-2","error":"the scope vanished"}"#.to_string()),
        };
        vm.on_long_op_event(&failed);
        match vm.state().get() {
            AnalysisState::Failed(msg) => assert_eq!(msg, "the scope vanished"),
            other => panic!("expected a stated failure, got {other:?}"),
        }
    }

    // ── the footnote-word companion operation ──────────────────────────────────

    /// The read half of `count_words_uc::fold_counts`'s alignment contract: the two
    /// vectors line up position for position, so the scope's figure sits at whichever
    /// index its id occupies in `book_item_ids`. Asserted across several entries so a
    /// bug that read the wrong index (an off-by-one, or the *value* vector instead of
    /// the *footnote* one) could not hide behind a single-book fixture.
    #[test]
    fn footnote_words_in_book_picks_the_position_aligned_entry() {
        let result = WordCountResultDto {
            book_item_ids: vec![10, 20, 30],
            book_word_counts: vec![900, 900, 900], // deliberately equal, so a bug that
            // read this vector instead would not be caught by distinct footnote values
            book_footnote_word_counts: vec![0, 44, 7],
            ..Default::default()
        };
        assert_eq!(footnote_words_in_book(&result, 20), 44);
        assert_eq!(footnote_words_in_book(&result, 30), 7);
        assert_eq!(footnote_words_in_book(&result, 10), 0);
    }

    /// A book absent from the roster genuinely has no footnotes to report — per
    /// `fold_counts`'s own doc a book is only left out when it has neither counted
    /// prose nor a counted note — so this must read as `0`, not as an unknown figure.
    #[test]
    fn a_book_missing_from_the_roster_reports_zero_footnote_words() {
        let result = WordCountResultDto {
            book_item_ids: vec![1],
            book_footnote_word_counts: vec![5],
            ..Default::default()
        };
        assert_eq!(footnote_words_in_book(&result, 999), 0);
    }

    #[test]
    fn an_empty_result_reports_zero_for_any_scope() {
        assert_eq!(footnote_words_in_book(&WordCountResultDto::default(), 1), 0);
    }

    /// `is_running` must cover the companion operation too, or a Run pressed while only
    /// the footnote figure is still catching up would fire a whole second `analyze_book`
    /// pass for an answer already on its way.
    #[test]
    fn is_running_reflects_the_footnote_operation_too() {
        let vm = vm(Signal::new(0));
        assert!(!vm.is_running());
        *vm.footnote_pending.borrow_mut() = Some(TrackedOp::given(
            "footnote-op".into(),
            crate::shared::long_op::CapturedWork::for_test(Some(1)),
        ));
        assert!(
            vm.is_running(),
            "the main state is Idle, but the companion op is still in flight"
        );
    }

    #[test]
    fn an_event_for_another_operation_does_not_touch_the_footnote_slot() {
        let vm = vm(Signal::new(0));
        vm.footnote_words.set(Some(12));
        *vm.footnote_pending.borrow_mut() = Some(TrackedOp::given(
            "footnote-op".into(),
            crate::shared::long_op::CapturedWork::for_test(Some(1)),
        ));

        let foreign = Event {
            origin: Origin::LongOperation(frontend::common::event::LongOperationEvent::Cancelled),
            ids: vec![],
            data: Some(r#"{"id":"someone-elses"}"#.to_string()),
        };
        vm.on_long_op_event(&foreign);
        assert_eq!(
            vm.footnote_words().get(),
            Some(12),
            "another feature's long operation must not touch this panel's footnote figure"
        );
        assert!(vm.footnote_pending.borrow().is_some());
    }

    #[test]
    fn cancelling_the_footnote_operation_clears_it_without_a_stale_figure() {
        let vm = vm(Signal::new(0));
        vm.footnote_words.set(Some(9)); // a stale figure from a previous run
        *vm.footnote_pending.borrow_mut() = Some(TrackedOp::given(
            "footnote-op".into(),
            crate::shared::long_op::CapturedWork::for_test(Some(1)),
        ));

        let cancelled = Event {
            origin: Origin::LongOperation(frontend::common::event::LongOperationEvent::Cancelled),
            ids: vec![],
            data: Some(r#"{"id":"footnote-op"}"#.to_string()),
        };
        vm.on_long_op_event(&cancelled);
        assert_eq!(
            vm.footnote_words().get(),
            None,
            "a cancelled companion op must not leave a stale figure on screen"
        );
        assert!(vm.footnote_pending.borrow().is_none());
    }

    /// A completion whose op id names no operation the manager actually knows about
    /// (this test never really ran one) must resolve to "not known" rather than
    /// panicking on the missing result or silently keeping a stale figure.
    #[test]
    fn completing_with_no_real_result_resolves_to_unknown_not_a_guess() {
        let vm = vm(Signal::new(0));
        vm.footnote_words.set(Some(3));
        *vm.footnote_pending.borrow_mut() = Some(TrackedOp::given(
            "footnote-op".into(),
            crate::shared::long_op::CapturedWork::for_test(Some(1)),
        ));

        let completed = Event {
            origin: Origin::LongOperation(frontend::common::event::LongOperationEvent::Completed),
            ids: vec![],
            data: Some(r#"{"id":"footnote-op"}"#.to_string()),
        };
        vm.on_long_op_event(&completed);
        assert_eq!(vm.footnote_words().get(), None);
        assert!(vm.footnote_pending.borrow().is_none());
    }

    /// The main operation's own events must still resolve against `pending` even with a
    /// footnote operation also in flight — the two tracked slots must not shadow each
    /// other just because both can be `Some` at once.
    #[test]
    fn the_main_and_footnote_operations_are_tracked_independently() {
        let vm = vm(Signal::new(0));
        vm.state.set(AnalysisState::Running);
        *vm.pending.borrow_mut() = Some(TrackedOp::given(
            "main-op".into(),
            crate::shared::long_op::CapturedWork::for_test(Some(1)),
        ));
        *vm.footnote_pending.borrow_mut() = Some(TrackedOp::given(
            "footnote-op".into(),
            crate::shared::long_op::CapturedWork::for_test(Some(1)),
        ));

        let cancelled_main = Event {
            origin: Origin::LongOperation(frontend::common::event::LongOperationEvent::Cancelled),
            ids: vec![],
            data: Some(r#"{"id":"main-op"}"#.to_string()),
        };
        vm.on_long_op_event(&cancelled_main);
        assert_eq!(vm.state().get(), AnalysisState::Idle);
        assert!(vm.pending.borrow().is_none(), "the main slot is released");
        assert!(
            vm.footnote_pending.borrow().is_some(),
            "the footnote op is a separate operation and must still be tracked"
        );
    }
}
