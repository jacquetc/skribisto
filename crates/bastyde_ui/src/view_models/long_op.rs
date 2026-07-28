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

    /// Wrap an already-captured Work id handed in by the caller. For the one
    /// long-operation view-model that does not hold its own `AppIds`
    /// (`ProjectSwitchViewModel` is a single shared instance — see its module
    /// doc): each of its four switch doors resolves `outgoing_work_id` from
    /// *its own* window's `AppIds` before calling in, so the capture already
    /// happened one layer up. This just stores that pre-captured value
    /// through the same shared type, instead of a bare `Option<u64>` that
    /// would need its own hand-written "don't re-read live" doc comment —
    /// exactly the drift this type exists to stop.
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
            captured, Some(1),
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
