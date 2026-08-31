// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Per-Work **item view-state roster**: where the writer was in an item, that
//! is caret, scroll, segment and Corkboard navigation, keyed by its durable
//! `BinderItem.uid` and independent of whether a tab on it is open right now.
//!
//! This is what lets reopening an item the writer had **closed**, this
//! session or after a full restart, come back to where they left it.
//!
//! It is deliberately *not* `PaneLayout::view_states`
//! ([`crate::models::PaneLayout`]). That roster is scoped to a *pane*'s
//! currently-open tabs, and stays the authority for restoring one of those (the
//! same item open in both split panes legitimately has two carets, which only
//! a per-pane list can tell apart). But it only ever knows about tabs that are
//! open: close a tab and its entry is simply gone, however recently the writer
//! was looking at it. This roster is the one place a remembered position
//! outlives the tab that recorded it.
//!
//! **Tier 2** (per open `Work`, in the multi-Work migration's terms, see
//! [`crate::sessions`]'s module doc): `Clone`, sharing one `Rc<RefCell<…>>`
//! between clones, so every window on the same project agrees on one roster
//! rather than each keeping its own diverging copy. Held on
//! `sessions::WorkSession` alongside the crate's other per-Work handles.
//!
//! **Not `Send`.** Like every other `Rc`-based handle in this crate, it never
//! crosses a thread; the UI is single-threaded end to end, and nothing here
//! needs to be anything else.
//!
//! The upsert/evict/prune policy itself is not reimplemented here: `record` and
//! `prune` below are thin wrappers over
//! [`crate::models::touch`]/[`crate::models::prune`], so the cap on how many
//! items are ever remembered lives in exactly the one place that also owns the
//! on-disk roster's shape.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use uuid::Uuid;

use crate::models::{TabViewState, prune, touch};

/// Cloneable handle over one Work's item view-state roster. See the module
/// docs for what this is for and why it exists beside `PaneLayout::view_states`.
#[derive(Clone, Default)]
pub struct ItemViewStates {
    inner: Rc<RefCell<Vec<TabViewState>>>,
}

impl ItemViewStates {
    /// An empty roster: nothing recorded until [`Self::load`] seeds it.
    pub fn new() -> Self {
        Self::default()
    }

    /// Seed the roster from a project's saved
    /// `PerProjectLayout::item_view_states`, replacing whatever was here before
    /// (a previous project's entries, or nothing). Called once, on project open.
    pub fn load(&self, states: Vec<TabViewState>) {
        *self.inner.borrow_mut() = states;
    }

    /// This item's remembered view, if anything was ever recorded for it.
    pub fn get(&self, uid: Uuid) -> Option<TabViewState> {
        self.inner.borrow().iter().find(|s| s.uid == uid).cloned()
    }

    /// Record `state` as this item's latest view: upserted by uid and moved to
    /// the front of the roster (see [`crate::models::touch`]), so an item the
    /// writer keeps coming back to never ages out ahead of one they haven't
    /// touched in a while.
    pub fn record(&self, state: TabViewState) {
        touch(&mut self.inner.borrow_mut(), state);
    }

    /// Drop every entry whose uid is not in `live` (see
    /// [`crate::models::prune`]): an item trashed or deleted since it was last
    /// recorded. No cascade reaches this roster on its own; a caller must run
    /// this itself, wherever the binder's own trash/delete path runs.
    pub fn prune(&self, live: &HashSet<Uuid>) {
        prune(&mut self.inner.borrow_mut(), live);
    }

    /// The whole roster, newest-first: for capture into
    /// `PerProjectLayout::item_view_states`.
    pub fn snapshot(&self) -> Vec<TabViewState> {
        self.inner.borrow().clone()
    }

    /// Empty the roster (project close), so the next [`Self::load`] seeds a
    /// fresh project rather than inheriting a closed one's entries into an
    /// unrelated one.
    pub fn clear(&self) {
        self.inner.borrow_mut().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn u(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    fn state(uid: Uuid, caret: usize) -> TabViewState {
        TabViewState {
            uid,
            caret,
            ..Default::default()
        }
    }

    #[test]
    fn seeded_state_is_readable_by_uid() {
        let states = ItemViewStates::new();
        states.load(vec![state(u(1), 10), state(u(2), 20)]);
        assert_eq!(states.get(u(1)).unwrap().caret, 10);
        assert_eq!(states.get(u(2)).unwrap().caret, 20);
        assert!(states.get(u(3)).is_none(), "never recorded");
    }

    #[test]
    fn record_upserts_rather_than_duplicating() {
        let states = ItemViewStates::new();
        states.record(state(u(1), 10));
        states.record(state(u(1), 42));
        let snap = states.snapshot();
        assert_eq!(snap.len(), 1, "one entry per uid, not a second row");
        assert_eq!(snap[0].caret, 42, "the latest recording wins");
    }

    #[test]
    fn prune_drops_a_dead_uid_and_keeps_the_live_one() {
        let states = ItemViewStates::new();
        states.load(vec![state(u(1), 0), state(u(2), 0)]);
        states.prune(&[u(1)].into_iter().collect());
        assert!(states.get(u(1)).is_some(), "still live");
        assert!(
            states.get(u(2)).is_none(),
            "trashed or deleted since recorded"
        );
    }

    #[test]
    fn snapshot_is_newest_first() {
        let states = ItemViewStates::new();
        states.record(state(u(1), 0));
        states.record(state(u(2), 0));
        states.record(state(u(3), 0));
        let uids: Vec<Uuid> = states.snapshot().iter().map(|s| s.uid).collect();
        assert_eq!(uids, vec![u(3), u(2), u(1)], "most recently recorded first");
    }

    #[test]
    fn two_clones_observe_one_shared_roster() {
        // The Tier-2 sharing property this handle exists for: two windows on
        // the same Work must see each other's writes rather than each keeping
        // its own diverging roster.
        let a = ItemViewStates::new();
        let b = a.clone();
        a.record(state(u(1), 7));
        assert_eq!(b.get(u(1)).unwrap().caret, 7, "b sees a's write");
        b.record(state(u(2), 9));
        assert_eq!(a.get(u(2)).unwrap().caret, 9, "and a sees b's");
    }

    #[test]
    fn clear_empties_the_roster_for_the_next_project() {
        let states = ItemViewStates::new();
        states.load(vec![state(u(1), 0)]);
        states.clear();
        assert!(states.snapshot().is_empty());
    }
}
