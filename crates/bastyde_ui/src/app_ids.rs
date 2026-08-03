// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The only mutable state the app itself holds: entity **ids**.
//!
//! Per the writing-model architecture, the UI reads entity *data* reactively
//! through [`singles`](crate::singles) and [`models`](crate::models); the app
//! itself keeps only the handful of ids those handles point at — the open
//! `Work`, its `WorkInfo`, and the per-`Work` undo stack.
//!
//! Seeded once per `LoadWork` (see `App::build`), shared everywhere by clone, and
//! registered as `app_state` so any widget can reach it via
//! `ctx.app_state::<AppIds>()`. Singles/models read these signals to know what to
//! point at and refresh themselves on fine-grained backend events thereafter.
//!
//! **`root_id` lives on [`crate::sessions::WorkRegistry`], not here.** Every
//! field below is per-open-`Work` (Tier 2 in the multi-Work migration's terms
//! — see `crate::sessions`'s module doc): a second simultaneously-open Work
//! needs its *own* `work_id`/`work_info_id`/`stack_id`. `root_id` does not —
//! there is exactly one shared `Root` per process, so it is genuinely Tier 1
//! (app-global). `AppIds` itself is the type `WorkSession` bundles as its
//! `ids` field.

use bastyde::prelude::Signal;

use frontend::AppContext;
use frontend::commands::{undo_redo_commands, work_info_commands};

/// The app's id-only per-Work state. Cloneable (every field is an `Rc`-backed
/// `Signal`), so all clones share one live state.
#[derive(Clone)]
pub struct AppIds {
    pub work_id: Signal<Option<u64>>,
    pub work_info_id: Signal<Option<u64>>,
    /// Per-`Work` undo stack id — one Ctrl+Z history for the whole undoable trunk.
    pub stack_id: Signal<Option<u64>>,
}

impl Default for AppIds {
    fn default() -> Self {
        Self {
            work_id: Signal::new(None),
            work_info_id: Signal::new(None),
            stack_id: Signal::new(None),
        }
    }
}

impl AppIds {
    pub fn new() -> Self {
        Self::default()
    }

    /// Bootstrap the entity ids from the freshly-loaded project, given the
    /// `work_id` the triggering `LoadWork`/`NewWork` event itself carries.
    ///
    /// Deliberately **not** "the first `Work`/`WorkInfo` the store returns":
    /// with a second Work simultaneously open, `get_all_work(ctx).first()` can
    /// answer with *either* open Work (whichever the backend's `HashMap`
    /// happens to iterate first) — silently seeding this window at someone
    /// else's project. `work_id` is supplied by the caller (the event that
    /// fired this seed), so `work_info_id` is resolved by walking `WorkInfo.work`
    /// back to exactly that id, not by taking a guess.
    pub fn seed(&self, ctx: &AppContext, work_id: u64) {
        self.work_id.set(Some(work_id));
        self.work_info_id.set(
            work_info_commands::get_all_work_info(ctx)
                .ok()
                .and_then(|list| list.into_iter().find(|wi| wi.work == Some(work_id)))
                .map(|wi| wi.id),
        );
    }

    /// Open a fresh per-`Work` undo stack and record its id. Call on `LoadWork`.
    pub fn open_stack(&self, ctx: &AppContext) {
        let id = undo_redo_commands::create_new_stack(ctx);
        self.stack_id.set(Some(id));
    }

    /// Forget all ids — no work is open. Call on `CloseWork`.
    pub fn clear(&self) {
        self.work_id.set(None);
        self.work_info_id.set(None);
        self.stack_id.set(None);
    }

    /// "Is this `LoadWork`/`NewWork` event about my Work?" — loose form, for the
    /// two events that **seed** [`Self::work_id`] in the first place.
    ///
    /// Every open window shares one process-wide event hub, so a second
    /// simultaneously-open Work's `LoadWork` fires this window's subscribers
    /// too. For a window whose Work is already known, that's exactly
    /// `event_ids.contains(&mine)`. But the window whose *own* bootstrap load
    /// fired this very event still has `work_id == None` at the instant its
    /// subscribers run — seeding is that subscriber's own job. Treating "not
    /// yet seeded" as "mine" is exact, not a guess: event dispatch is
    /// synchronous and single-threaded, so at most one not-yet-seeded window
    /// can be mid-subscribe at any instant.
    ///
    /// Do NOT use this for `CloseWork` or for anything that must never fire
    /// against a window with nothing open yet — see [`Self::is_event_for_my_work`].
    pub fn is_bootstrap_or_own(&self, event_ids: &[u64]) -> bool {
        match self.work_id.get() {
            Some(mine) => event_ids.contains(&mine),
            None => true,
        }
    }

    /// "Is this event about my Work?" — strict form. `None` (nothing open yet)
    /// never matches: a `CloseWork`/entity event cannot legitimately be about a
    /// window that has not finished loading anything.
    pub fn is_event_for_my_work(&self, event_ids: &[u64]) -> bool {
        self.work_id
            .get()
            .is_some_and(|mine| event_ids.contains(&mine))
    }
}

/// A view-model that holds its own [`AppIds`] and wants the open Work's id
/// directly, rather than spelling out `self.ids.work_id.get()` at every call
/// site. `app_ids` is infrastructure every view-model already imports (never
/// a peer view-model importing another peer), so this default method gives
/// the accessor one shared home without breaking the cross-VM-talk DAG rule.
pub trait HasWorkId {
    /// This view-model's own [`AppIds`].
    fn app_ids(&self) -> &AppIds;

    /// The open Work this view-model's own reads/toasts should route to.
    /// `self.ids.work_id` is the authoritative "current Work" source — see
    /// this module's doc — so implementors need only hand back their `ids`
    /// field from [`Self::app_ids`]; this default does the rest.
    fn work_id(&self) -> Option<u64> {
        self.app_ids().work_id.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_or_own_accepts_any_event_before_seeding() {
        let ids = AppIds::new();
        assert!(ids.is_bootstrap_or_own(&[42]));
        assert!(ids.is_bootstrap_or_own(&[]));
    }

    #[test]
    fn bootstrap_or_own_matches_only_my_work_once_seeded() {
        let ids = AppIds::new();
        ids.work_id.set(Some(7));
        assert!(ids.is_bootstrap_or_own(&[7]));
        assert!(!ids.is_bootstrap_or_own(&[8]));
        assert!(!ids.is_bootstrap_or_own(&[]));
    }

    #[test]
    fn strict_match_never_accepts_before_seeding() {
        let ids = AppIds::new();
        assert!(!ids.is_event_for_my_work(&[42]));
        assert!(!ids.is_event_for_my_work(&[]));
    }

    #[test]
    fn strict_match_matches_only_my_work_once_seeded() {
        let ids = AppIds::new();
        ids.work_id.set(Some(7));
        assert!(ids.is_event_for_my_work(&[7]));
        assert!(!ids.is_event_for_my_work(&[8]));
    }

    struct Holder(AppIds);
    impl HasWorkId for Holder {
        fn app_ids(&self) -> &AppIds {
            &self.0
        }
    }

    #[test]
    fn has_work_id_default_reads_through_to_the_live_app_ids() {
        let ids = AppIds::new();
        let holder = Holder(ids.clone());
        assert_eq!(holder.work_id(), None);
        ids.work_id.set(Some(9));
        assert_eq!(
            holder.work_id(),
            Some(9),
            "the default method must read the live signal each call, not a snapshot \
             taken when `app_ids()` first ran"
        );
    }
}
