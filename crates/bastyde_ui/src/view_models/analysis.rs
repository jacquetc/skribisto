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

use std::cell::RefCell;
use std::rc::Rc;

use bastyde::prelude::*;
use frontend::AppContext;
use frontend::commands::analysis_management_commands;
use frontend::common::event::{Event, Origin};

use frontend::analysis_management::{AnalyzeBookDto, BookAnalysisResultDto};

use crate::app_ids::AppIds;
use crate::models::{RepetitionTreeKey, RepetitionTreeModel};
use crate::view_models::long_op::{TrackedOp, event_id, parse_payload};
use bastyde::data::{KeyedSelectionModel, SelectionMode};

/// Which analysis the panel is showing.
///
/// Four, grouped by the question a writer is asking rather than by which measure answered
/// it: Shape absorbs both the per-scene rhythm strands and the chapter-balance chart,
/// because "how is this book distributed" is one question however many numbers answer it.
///
/// Well inside `SegmentedControl`'s documented five-segment ceiling, which matters because
/// this list is the kind that grows. A fifth is fine; a sixth is not a segment any more, it
/// is a different control.
///
/// There is deliberately no Mentions category. Mention coverage is real and useful, but it
/// is the mention feature's data, already surfaced by the Inspector's Cast and backlinks —
/// and `ContentTab` carries no `MentionIndex`, so adding it here would mean threading a
/// whole view-model through the tab to duplicate a view that already exists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnalysisCategory {
    Shape = 0,
    Repetition = 1,
    Synopsis = 2,
    Voice = 3,
}

impl AnalysisCategory {
    pub const ALL: [AnalysisCategory; 4] =
        [Self::Shape, Self::Repetition, Self::Synopsis, Self::Voice];

    /// The category at a bar position. `None` for an out-of-range index, so a widening of
    /// the bar without a matching arm mounts nothing rather than silently showing a
    /// neighbour — the positional trap this codebase has been bitten by before.
    pub fn from_index(i: usize) -> Option<Self> {
        Self::ALL.get(i).copied()
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
    category: Signal<usize>,
    /// `dirty_seq` as of the last completed run. `None` until one completes.
    analysed_at_seq: Signal<Option<u64>>,
    /// The live edit counter, shared with the save indicator.
    dirty_seq: Signal<u64>,
    pending: Rc<RefCell<Option<TrackedOp>>>,
    /// Guards the one automatic first run, so re-selecting the segment does not re-run.
    auto_ran: Rc<std::cell::Cell<bool>>,
    /// Whether Shape leaves texts with no prose out of its charts. On by default: a Part
    /// heading or an unwritten scene is a real row of the binder but a zero-height gap in a
    /// bar chart, and a book outlined ahead of its drafting is mostly gaps.
    ///
    /// A view-model field rather than pane-local state so it survives switching categories
    /// and re-running the analysis — the writer set it once, about this book.
    ignore_empty: Signal<bool>,
    /// The Repetition pane's tree, and the row it has selected.
    ///
    /// Held here rather than in the pane because a `TreeView` built from a source keeps its
    /// expand set on the *source*: parked in the widget it would reset on every rebuild,
    /// closing every row the writer had opened. Populated from each completed run.
    repetition_tree: RepetitionTreeModel,
    repetition_selection: KeyedSelectionModel<RepetitionTreeKey>,
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
            category: Signal::new(0),
            analysed_at_seq: Signal::new(None),
            dirty_seq,
            pending: Rc::new(RefCell::new(None)),
            auto_ran: Rc::new(std::cell::Cell::new(false)),
            ignore_empty: Signal::new(true),
            repetition_tree: RepetitionTreeModel::new(),
            // One row at a time: this tree is a way in, not a multi-select worklist.
            repetition_selection: KeyedSelectionModel::new(SelectionMode::Single),
        }
    }

    /// The Repetition tree's source, for the pane's `TreeView`.
    pub fn repetition_tree(&self) -> RepetitionTreeModel {
        self.repetition_tree.clone()
    }

    pub fn repetition_selection(&self) -> KeyedSelectionModel<RepetitionTreeKey> {
        self.repetition_selection.clone()
    }

    /// Whether Shape hides texts with no prose. See [`Self::ignore_empty`].
    pub fn ignore_empty(&self) -> Signal<bool> {
        self.ignore_empty.clone()
    }

    pub fn state(&self) -> Signal<AnalysisState> {
        self.state.clone()
    }

    pub fn category(&self) -> Signal<usize> {
        self.category.clone()
    }

    pub fn scope_item_id(&self) -> u64 {
        self.scope_item_id
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

    pub fn is_running(&self) -> bool {
        matches!(self.state.get(), AnalysisState::Running)
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
            }
            Err(e) => self.state.set(AnalysisState::Failed(e.to_string())),
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

    /// Feed a `Origin::LongOperation(...)` event in. Ignores anything that is not this
    /// view-model's own in-flight operation.
    pub fn on_long_op_event(&self, event: &Event) {
        let Origin::LongOperation(kind) = &event.origin else {
            return;
        };
        let id = event_id(event);
        let matches_ours = {
            let pending = self.pending.borrow();
            match (pending.as_ref(), id.as_deref()) {
                (Some(op), Some(id)) => op.matches(id),
                _ => false,
            }
        };
        if !matches_ours {
            return;
        }

        use frontend::common::event::LongOperationEvent as L;
        match kind {
            L::Completed => {
                let op_id = id.unwrap_or_default();
                match analysis_management_commands::get_analyze_book_result(&self.ctx, &op_id) {
                    Ok(Some(dto)) => {
                        // Recorded at completion rather than at start: edits made *during*
                        // the pass are not covered by it, and claiming otherwise would show
                        // a fresh badge over a result that predates them.
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
                // Back to whatever was on screen before, not to an error: a cancel is the
                // writer's own decision and is not a failure to report back to them.
                self.state.set(AnalysisState::Idle);
                *self.pending.borrow_mut() = None;
            }
            _ => {}
        }
    }
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
            AnalysisCategory::from_index(3),
            Some(AnalysisCategory::Voice)
        );
        assert_eq!(
            AnalysisCategory::from_index(4),
            None,
            "a segment past the end must mount nothing, not a neighbour"
        );
    }

    /// The bar and the `Switcher` are matched by position, so the count is a contract.
    /// It also stays under `SegmentedControl`'s five-segment ceiling — the point past which
    /// this stops being a segmented control at all.
    #[test]
    fn the_category_count_stays_within_the_segmented_control_ceiling() {
        assert_eq!(AnalysisCategory::ALL.len(), 4);
        assert!(AnalysisCategory::ALL.len() <= 5);
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
            crate::view_models::long_op::CapturedWork::for_test(Some(1)),
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
            crate::view_models::long_op::CapturedWork::for_test(Some(1)),
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
            crate::view_models::long_op::CapturedWork::for_test(Some(1)),
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
}
