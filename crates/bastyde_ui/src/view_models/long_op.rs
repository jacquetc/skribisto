// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Shared helpers for consuming `Origin::LongOperation(...)` events in view-models.
//!
//! The long-operation manager emits its lifecycle events with a JSON `data`
//! payload (`{"id": …, "percentage": …, "message": …, "error": …}`). Several
//! view-models (import, save-as, …) filter these by the in-flight operation id,
//! so the payload parsing lives here once rather than being copied per feature.

use frontend::common::event::Event;

use crate::app_ids::AppIds;

/// The Work a long operation is running for, pinned the instant it starts.
///
/// Every long-operation view-model in this crate is minted once per window
/// (`shell::windows::ProjectWindowFactory::window_config`) and outlives any
/// one operation it drives — including an in-place project switch in the
/// SAME window while that operation is still running
/// (`ProjectSwitchViewModel::request` gates a switch only on unsaved
/// edits/autosave, never on "is a long operation running"). A switch reseeds
/// `AppIds.work_id` on the SAME `Signal` every long-operation view-model
/// already holds as its `ids` field. An `on_long_op_*` handler that re-reads
/// `ids.work_id.get()` live therefore silently answers with whichever Work
/// the window shows *at event time*, not the Work the operation actually
/// started for — misrouting (and, via a shared static toast id, mis-deduping
/// — see [`crate::toast_scope::work_scoped_toast_id`]) the
/// progress/completion/error toast onto a Work that has nothing to do with
/// it, while the Work that actually ran the operation never hears it
/// finished.
///
/// This exact bug was independently hand-fixed four times
/// (`ExportViewModel::active_work_id`, `BackupSchedulerViewModel::Pending::work_id`,
/// `BackupRestoreViewModel::BackupRestorePending::work_id`,
/// `ProjectSwitchViewModel::pending_work_id`) — near-identical private fields,
/// near-identical doc comments explaining the same hazard — and the drift
/// those four separate copies predicted then happened for real:
/// `SaveAsViewModel` was built the same way, one field short, and read
/// `self.ids.work_id` live on completion/failure. `CapturedWork` centralizes
/// the mechanism: one door in ([`Self::now`]), one door out (`Into<Option<u64>>`),
/// so a new long-operation view-model that copies this idiom copies a *type*
/// it cannot produce from a live `AppIds` read by accident — there is no
/// `From<&AppIds>`-free way to mint a "real" one outside a test.
///
/// # Usage
/// Store one alongside whatever else your `Pending`/`active` state already
/// records for the in-flight op — a struct field, a parallel `Signal`, a
/// `Cell`. This crate's existing storage shapes (a `HashMap<String, Pending>`
/// keyed by op id, a `Signal<Option<Pending>>`, an `Rc<RefCell<Option<Pending>>>`,
/// a bare `Rc<Cell<_>>`) are all still valid — only the *type* of the "which
/// Work" slot is shared, not its container, because forcing four different
/// op-tracking shapes into one wrapper would be a worse fit than any of them.
/// [`Self::now`] the instant the operation starts, in the same call that
/// records its op id; every later handler reads the value back — `Toast::
/// target_work` and [`crate::toast_scope::work_scoped_toast_id`] both accept
/// it directly (`impl Into<Option<u64>>`) — and never touches `ids.work_id`
/// again for that operation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct CapturedWork(Option<u64>);

impl CapturedWork {
    /// Capture `ids.work_id` NOW. Call this — and only this, for a
    /// view-model that holds its own `ids: AppIds` — the instant a long
    /// operation starts.
    pub(crate) fn now(ids: &AppIds) -> Self {
        Self(ids.work_id.get())
    }

    /// Wrap an already-captured Work id handed in by the caller. For a
    /// view-model that does not hold its own `AppIds`:
    /// `ProjectSwitchViewModel` (a single shared instance — see its module
    /// doc) resolves `outgoing_work_id` from *its own* window's `AppIds`
    /// before calling in, at each of its four switch doors; `BackupsListViewModel`
    /// (F7 — see its `work_id` field's doc) is handed its caller's
    /// `ids.work_id.get()` at construction, once, and never holds `AppIds`
    /// at all. Either way, the capture already happened one layer up — this
    /// just stores that pre-captured value through the same shared type,
    /// instead of a bare `Option<u64>` that would need its own hand-written
    /// "don't re-read live" doc comment — exactly the drift this type exists
    /// to stop.
    pub(crate) fn given(work_id: Option<u64>) -> Self {
        Self(work_id)
    }

    /// The placeholder meaning "no operation is in flight" — what a
    /// `Pending`-holding cell/signal is initialized to, and reset to on
    /// completion/failure/cancellation. Never use this to start an
    /// operation — that's [`Self::now`]/[`Self::given`]'s job.
    pub(crate) fn none() -> Self {
        Self(None)
    }

    /// Test-only escape hatch: mint an arbitrary sentinel value decoupled
    /// from any real `AppIds`, for tests that only need two distinct "Work"
    /// values to prove a routing/dedup property (not to exercise the capture
    /// timing itself — those tests use [`Self::now`] against a real `AppIds`,
    /// same as production code).
    #[cfg(test)]
    pub(crate) fn for_test(work_id: Option<u64>) -> Self {
        Self(work_id)
    }
}

impl From<CapturedWork> for Option<u64> {
    fn from(w: CapturedWork) -> Self {
        w.0
    }
}

/// So a test can `assert_eq!(captured, Some(1))` / `assert_eq!(captured, None)`
/// directly against the plain `Option<u64>` it conceptually wraps, without an
/// explicit `.into()` at every call site.
impl PartialEq<Option<u64>> for CapturedWork {
    fn eq(&self, other: &Option<u64>) -> bool {
        self.0 == *other
    }
}

/// An in-flight long operation's id, bundled with the Work it was captured
/// for.
///
/// # Why this exists (F4)
/// [`CapturedWork`] stops a handler from re-reading `ids.work_id` LIVE once
/// an operation has captured it — it does not stop a future view-model from
/// forgetting to capture it AT ALL. That is exactly how `SaveAsViewModel`
/// shipped (see [`CapturedWork`]'s own doc): a hand-written `Pending { op_id:
/// String, work_id: CapturedWork, … }` compiles fine even with the `work_id`
/// field simply left out of the literal — an op id and a captured Work are
/// two independent struct fields, and nothing ties them together. Four
/// near-identical view-models (`ExportViewModel`, `BackupSchedulerViewModel`,
/// `BackupRestoreViewModel`, `SaveAsViewModel`) each independently hand-rolled
/// "the op id an event is for" + "the Work that op is for" as such a pair;
/// `SaveAsViewModel` was the one that dropped the second half.
///
/// `TrackedOp` makes that pair one value with one constructor
/// ([`Self::start`]) that captures `ids.work_id` in the *same call* that
/// records the op id — there is no struct-literal spelling that produces a
/// `TrackedOp` without the capture happening. Embed ONE `TrackedOp` field in
/// your own `Pending`-shaped struct for whatever extra data your operation
/// needs to remember (destination path, backup id, retention dirs, …),
/// instead of a bare `op_id: String` + `work_id: CapturedWork` pair.
///
/// # Why not go further and own the map/slot too
/// Deliberately NOT a generic `PendingOp<T>` that also owns the container
/// (`HashMap`/`Cell`/`RefCell`/`Signal`) itself: the four op-id-keyed
/// view-models above use four different containers for good reason — see
/// [`CapturedWork`]'s own doc's "Usage" section for why forcing one container
/// shape on all of them would be a worse fit than any of them (a `HashMap`
/// where several ops can genuinely run at once vs. a single `Option` slot
/// where only one can). A fifth view-model that also holds a
/// [`CapturedWork`], `ProjectSwitchViewModel`, isn't op-id-keyed at all — its
/// parked switch waits on an *edit sequence* (`on_saved(ctx, saved_seq: u64)`,
/// `saved_seq >= pending_seq`), never an `Origin::LongOperation` event, so
/// `TrackedOp`'s "does this event's id match" contract does not apply to it;
/// it keeps its own standalone `CapturedWork` field, documented in place. Only
/// the op-id+Work *pairing* is shared here; the surrounding struct and its
/// container are still each view-model's own.
#[derive(Clone, Debug)]
pub(crate) struct TrackedOp {
    op_id: String,
    work_id: CapturedWork,
}

impl TrackedOp {
    /// Start tracking `op_id`: captures `ids.work_id` right here, in the same
    /// call that records the id — see the type's doc. Call this the instant a
    /// long operation starts, exactly where [`CapturedWork::now`] alone used
    /// to be called.
    pub(crate) fn start(ids: &AppIds, op_id: String) -> Self {
        Self {
            op_id,
            work_id: CapturedWork::now(ids),
        }
    }

    /// Wrap an already-captured `(op_id, work_id)` pair. For a tracker whose
    /// view-model does not hold its own `AppIds` — mirrors
    /// [`CapturedWork::given`]; see that method's doc for when this applies.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn given(op_id: String, work_id: CapturedWork) -> Self {
        Self { op_id, work_id }
    }

    /// The tracked operation's own id.
    pub(crate) fn op_id(&self) -> &str {
        &self.op_id
    }

    /// Does `id` (an event's own op id) name this tracked operation?
    pub(crate) fn matches(&self, id: &str) -> bool {
        self.op_id == id
    }

    /// The Work this operation was captured for.
    pub(crate) fn work_id(&self) -> CapturedWork {
        self.work_id
    }
}

/// Parse a `LongOperation` event's JSON payload (`{"id":…, "percentage":…, …}`).
pub(crate) fn parse_payload(event: &Event) -> Option<serde_json::Value> {
    event
        .data
        .as_ref()
        .and_then(|s| serde_json::from_str(s).ok())
}

/// The operation id inside an already-parsed payload.
pub(crate) fn payload_id(payload: &serde_json::Value) -> Option<&str> {
    payload.get("id").and_then(|i| i.as_str())
}

/// The operation id carried by an event (parse + extract in one step).
pub(crate) fn event_id(event: &Event) -> Option<String> {
    parse_payload(event)?
        .get("id")
        .and_then(|i| i.as_str())
        .map(str::to_string)
}

#[cfg(test)]
mod captured_work_tests {
    use super::*;

    #[test]
    fn now_captures_the_live_work_id_at_the_instant_it_is_called() {
        let ids = AppIds::new();
        ids.work_id.set(Some(7));
        let captured = CapturedWork::now(&ids);
        assert_eq!(captured, Some(7));
    }

    #[test]
    fn now_survives_a_later_change_to_the_same_appids() {
        // The entire point of the type: a capture must not silently track a
        // later mutation of the `AppIds` it was captured from.
        let ids = AppIds::new();
        ids.work_id.set(Some(1));
        let captured = CapturedWork::now(&ids);
        ids.work_id.set(Some(2));
        assert_eq!(
            captured,
            Some(1),
            "the captured value must stay pinned to what `now` read, not follow \
             the live `AppIds` it was captured from"
        );
        assert_ne!(
            Option::<u64>::from(captured),
            ids.work_id.get(),
            "the captured value must now differ from the live one"
        );
    }

    #[test]
    fn none_captures_as_none_when_no_work_is_open() {
        let ids = AppIds::new();
        assert_eq!(CapturedWork::now(&ids), None);
    }

    #[test]
    fn the_placeholder_reads_as_none() {
        assert_eq!(CapturedWork::none(), None);
    }

    #[test]
    fn given_wraps_an_already_captured_value_unchanged() {
        assert_eq!(CapturedWork::given(Some(42)), Some(42));
        assert_eq!(CapturedWork::given(None), None);
    }
}

#[cfg(test)]
mod tracked_op_tests {
    use super::*;

    /// F4: `TrackedOp::start` is the only constructor a view-model that owns
    /// its own `AppIds` needs — it must capture `ids.work_id` in the same
    /// call, not merely accept one handed in (that would leave open the exact
    /// "forgot the work_id half" hole `TrackedOp` exists to close).
    #[test]
    fn start_bundles_the_op_id_with_a_capture_of_the_live_work_id() {
        let ids = AppIds::new();
        ids.work_id.set(Some(3));
        let tracked = TrackedOp::start(&ids, "op-1".to_string());
        assert_eq!(tracked.op_id(), "op-1");
        assert_eq!(tracked.work_id(), Some(3));
    }

    #[test]
    fn start_survives_a_later_change_to_the_same_appids() {
        let ids = AppIds::new();
        ids.work_id.set(Some(1));
        let tracked = TrackedOp::start(&ids, "op-1".to_string());
        ids.work_id.set(Some(2));
        assert_eq!(
            tracked.work_id(),
            Some(1),
            "a TrackedOp must stay pinned to the Work it captured, not follow \
             a later mutation of the AppIds it was captured from"
        );
    }

    #[test]
    fn matches_compares_against_the_tracked_op_id_only() {
        let tracked = TrackedOp::given("op-a".to_string(), CapturedWork::for_test(Some(1)));
        assert!(tracked.matches("op-a"));
        assert!(!tracked.matches("op-b"));
    }

    #[test]
    fn given_wraps_an_already_captured_op_id_and_work_id_pair_unchanged() {
        let tracked = TrackedOp::given("op-a".to_string(), CapturedWork::for_test(Some(7)));
        assert_eq!(tracked.op_id(), "op-a");
        assert_eq!(tracked.work_id(), Some(7));
    }
}
