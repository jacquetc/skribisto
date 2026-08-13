// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! How much text arrived by each route, per open project.
//!
//! ## What this counts, and what it does not
//!
//! Characters, by the channel they came down: typed, pasted, dictated, imported,
//! or inserted by the application. That is a fact about **input**, and it is the
//! only kind of fact this holds.
//!
//! It is emphatically **not** a measure of authorship. A writer who drafts in
//! another application and pastes chapters in has pasted; a writer who dictates
//! has dictated; a writer who types has typed. None of those says anything about
//! who wrote the words, and any reader of these numbers who concludes otherwise
//! has gone beyond what is here. Nothing in this module weighs one route against
//! another, orders them, or has a threshold in it.
//!
//! ## Why the counts live here rather than in the editor
//!
//! An editor is per **window**, and a project can have several open on it. The
//! count that means anything is the project's, so it is kept where the project
//! is — beside the plan store and the lifecycle hook, keyed the same way, and
//! evicted the same way when the project closes.
//!
//! `Send + Sync`, because the save that reads it runs on a worker thread while
//! the editor that writes it runs on the UI thread.
//!
//! ## Read-and-reset, not read
//!
//! [`Arrivals::take`](crate::arrival::Arrivals::take) is the only read, and it
//! empties what it returns. A consumer records
//! *what arrived since it last looked*, which is what a per-save figure is;
//! leaving the totals to accumulate would mean every save reporting the whole
//! session again, and the same characters counted once per save for the rest of
//! the day.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Which route text came down. The application's own vocabulary, mapped from
/// the toolkit's at the point where it is reported.
///
/// Deliberately the same shape as `teksilo`'s `EditSource` but a separate type:
/// the toolkit names input channels, and this names what the *manuscript* cares
/// about — which is why [`Self::Imported`] exists here and has no toolkit
/// equivalent. An import never passes through an editor at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Arrival {
    /// Typed, one key at a time — including a correction the writer made by
    /// hand, and including an autocorrect they undid to put their own
    /// characters back.
    Typed,
    /// Pasted or dropped in from somewhere else.
    Pasted,
    /// Arrived through an accessibility channel: dictation, a braille display.
    ///
    /// **Never folded into [`Self::Typed`].** For some writers this is how
    /// writing happens, and a count that erased the distinction would be
    /// reporting them as something they are not.
    Dictated,
    /// Came in through a document import.
    Imported,
    /// Inserted by the application: a template, an expansion, a typographic
    /// substitution, a spelling suggestion the writer accepted from a menu.
    Programmatic,
}

impl Arrival {
    /// Every route, for a consumer that wants a complete breakdown.
    pub const ALL: [Arrival; 5] = [
        Arrival::Typed,
        Arrival::Pasted,
        Arrival::Dictated,
        Arrival::Imported,
        Arrival::Programmatic,
    ];
}

/// Characters by route, for one project.
///
/// A plain map rather than a struct with five fields, so a route added later
/// does not change this type's shape for every consumer.
pub type Counts = HashMap<Arrival, u64>;

/// One tally per open project, keyed by `Work.unique_id`.
///
/// Cheap to clone; every clone shares one map.
#[derive(Clone, Default)]
pub struct Arrivals {
    inner: Arc<RwLock<HashMap<String, Counts>>>,
}

impl std::fmt::Debug for Arrivals {
    /// The count of projects, never the tallies. A `Debug` that printed them
    /// would put a writer's working habits in a log line.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Arrivals")
            .field("projects", &self.read().len())
            .finish_non_exhaustive()
    }
}

impl Arrivals {
    pub fn new() -> Self {
        Self::default()
    }

    fn read(&self) -> std::sync::RwLockReadGuard<'_, HashMap<String, Counts>> {
        // Poison-safe, like every other lock in this crate: a panic somewhere
        // unrelated must not make a manuscript unsaveable.
        self.inner.read().unwrap_or_else(|e| e.into_inner())
    }

    fn write(&self) -> std::sync::RwLockWriteGuard<'_, HashMap<String, Counts>> {
        self.inner.write().unwrap_or_else(|e| e.into_inner())
    }

    /// Add `chars` of `arrival` to this project's tally.
    ///
    /// The empty uid is what an **unsaved** project carries, and is refused: a
    /// tally under it would pool every unsaved project's typing into one entry
    /// that no save ever asks for. Zero is refused for the same reason it is at
    /// every other layer — no characters arriving is not an event.
    pub fn record(&self, work_unique_id: &str, arrival: Arrival, chars: u64) {
        if work_unique_id.is_empty() || chars == 0 {
            return;
        }
        // ⚠ **One lock, held across the read and the add.** An earlier cut read
        // the old value through `peek` and wrote the sum back, which took the
        // read lock and the write lock separately: correct only because Rust
        // evaluates an assignment's right side first, and *not* atomic — two
        // threads recording at once would each read the same old value and one
        // would overwrite the other's characters. `entry` gives the slot itself,
        // so there is nothing between the read and the add.
        let mut projects = self.write();
        let slot = projects
            .entry(work_unique_id.to_string())
            .or_default()
            .entry(arrival)
            .or_insert(0);
        *slot = slot.saturating_add(chars);
    }

    /// This project's count for one route, without disturbing it. For tests and
    /// for a readout that must not consume.
    pub fn peek(&self, work_unique_id: &str, arrival: Arrival) -> u64 {
        self.read()
            .get(work_unique_id)
            .and_then(|c| c.get(&arrival))
            .copied()
            .unwrap_or(0)
    }

    /// This project's whole tally, without disturbing it.
    pub fn snapshot(&self, work_unique_id: &str) -> Counts {
        self.read().get(work_unique_id).cloned().unwrap_or_default()
    }

    /// This project's tally, **emptied**.
    ///
    /// The read a save makes. Returning and clearing in one locked step is what
    /// stops a character typed between the two from being counted twice or lost
    /// — a snapshot-then-clear pair has a window in it, and the window is
    /// exactly one keystroke wide.
    pub fn take(&self, work_unique_id: &str) -> Counts {
        self.write().remove(work_unique_id).unwrap_or_default()
    }

    /// Forget a project entirely — what a close does.
    pub fn forget(&self, work_unique_id: &str) {
        self.write().remove(work_unique_id);
    }

    /// How many projects currently have a tally.
    pub fn len(&self) -> usize {
        self.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// The application's own tally, for everything that has no other place to keep
/// one.
///
/// A process-wide singleton because the two ends are far apart and neither owns
/// the other: the editor that reports lives in a window, the save that reads
/// runs on a worker thread, and the project they are both about outlives both.
/// Threading a handle from one to the other would mean putting it in the DTOs —
/// the same argument `bundle_contributors` records for being a registry.
pub fn shared() -> Arrivals {
    static SHARED: std::sync::LazyLock<Arrivals> = std::sync::LazyLock::new(Arrivals::new);
    SHARED.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_accumulate_per_route() {
        let a = Arrivals::new();
        a.record("uid", Arrival::Typed, 100);
        a.record("uid", Arrival::Typed, 40);
        a.record("uid", Arrival::Pasted, 900);
        assert_eq!(a.peek("uid", Arrival::Typed), 140);
        assert_eq!(a.peek("uid", Arrival::Pasted), 900);
        assert_eq!(a.peek("uid", Arrival::Dictated), 0);
    }

    /// **The count that means anything is the project's**, and two open at once
    /// must not pool into each other — the same bug `ProjectStore` exists to
    /// make unexpressible, one layer along.
    #[test]
    fn two_projects_never_see_each_others_typing() {
        let a = Arrivals::new();
        a.record("uid-a", Arrival::Typed, 100);
        a.record("uid-b", Arrival::Typed, 7);
        assert_eq!(a.peek("uid-a", Arrival::Typed), 100);
        assert_eq!(a.peek("uid-b", Arrival::Typed), 7);
        assert_eq!(a.len(), 2);
    }

    /// **`take` empties.** A consumer records what arrived since it last looked;
    /// leaving the totals would mean every save reporting the whole session
    /// again, and the same characters counted once per save for the rest of the
    /// day.
    #[test]
    fn taking_a_tally_empties_it() {
        let a = Arrivals::new();
        a.record("uid", Arrival::Typed, 100);
        let first = a.take("uid");
        assert_eq!(first.get(&Arrival::Typed), Some(&100));
        assert!(
            a.take("uid").is_empty(),
            "a second read must find nothing, or the same words are counted twice"
        );

        a.record("uid", Arrival::Typed, 5);
        assert_eq!(a.take("uid").get(&Arrival::Typed), Some(&5));
    }

    /// …and `snapshot` does not, so a readout can show a figure without
    /// consuming the one the save is about to record.
    #[test]
    fn snapshot_leaves_the_tally_alone() {
        let a = Arrivals::new();
        a.record("uid", Arrival::Typed, 100);
        assert_eq!(a.snapshot("uid").get(&Arrival::Typed), Some(&100));
        assert_eq!(a.snapshot("uid").get(&Arrival::Typed), Some(&100));
        assert_eq!(a.take("uid").get(&Arrival::Typed), Some(&100));
    }

    /// The empty uid is an **unsaved** project's. A tally under it would pool
    /// every unsaved project's typing into one entry no save ever asks for.
    #[test]
    fn an_unsaved_project_gets_no_tally() {
        let a = Arrivals::new();
        a.record("", Arrival::Typed, 100);
        assert!(a.is_empty());
        assert_eq!(a.peek("", Arrival::Typed), 0);
    }

    #[test]
    fn zero_characters_is_not_an_event() {
        let a = Arrivals::new();
        a.record("uid", Arrival::Typed, 0);
        assert!(a.is_empty());
    }

    #[test]
    fn a_huge_tally_saturates_rather_than_wrapping() {
        let a = Arrivals::new();
        a.record("uid", Arrival::Typed, u64::MAX);
        a.record("uid", Arrival::Typed, 10);
        assert_eq!(a.peek("uid", Arrival::Typed), u64::MAX);
    }

    #[test]
    fn closing_a_project_forgets_its_tally_and_no_others() {
        let a = Arrivals::new();
        a.record("uid-a", Arrival::Typed, 1);
        a.record("uid-b", Arrival::Typed, 2);
        a.forget("uid-a");
        assert_eq!(a.peek("uid-a", Arrival::Typed), 0);
        assert_eq!(a.peek("uid-b", Arrival::Typed), 2);
    }

    /// Every clone shares one map — the whole reason this type exists is that
    /// the end that writes and the end that reads are in different places.
    #[test]
    fn every_clone_shares_one_tally() {
        let writer = Arrivals::new();
        let reader = writer.clone();
        writer.record("uid", Arrival::Dictated, 42);
        assert_eq!(reader.peek("uid", Arrival::Dictated), 42);
        assert!(!reader.take("uid").is_empty());
        assert_eq!(writer.peek("uid", Arrival::Dictated), 0, "…and one clear");
    }

    /// A `Debug` that printed the tallies would put a writer's working habits
    /// into any log line that happened to format one.
    #[test]
    fn debug_shows_the_project_count_and_no_tallies() {
        let a = Arrivals::new();
        a.record("uid", Arrival::Typed, 12345);
        let text = format!("{a:?}");
        assert!(text.contains('1'), "the project count is shown: {text}");
        assert!(!text.contains("12345"), "the tally is not: {text}");
    }

    /// **Nothing is lost when two threads record at once.** The editor writes
    /// from the UI thread and an import's subscriber can write from another, so
    /// a read-then-write-back pair would drop characters under exactly the load
    /// that matters. Every one of these 8,000 must survive.
    #[test]
    fn concurrent_records_lose_nothing() {
        let a = Arrivals::new();
        std::thread::scope(|s| {
            for _ in 0..8 {
                let a = a.clone();
                s.spawn(move || {
                    for _ in 0..1000 {
                        a.record("uid", Arrival::Typed, 1);
                    }
                });
            }
        });
        assert_eq!(a.peek("uid", Arrival::Typed), 8000);
    }

    #[test]
    fn every_route_is_in_all() {
        assert_eq!(Arrival::ALL.len(), 5);
        for route in Arrival::ALL {
            assert!(Arrival::ALL.iter().filter(|r| **r == route).count() == 1);
        }
    }
}
