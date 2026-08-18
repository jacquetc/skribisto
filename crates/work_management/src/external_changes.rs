// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! An extension saying, from any thread, that a project holds something not on
//! disk.
//!
//! ## The gap this closes
//!
//! [`crate::bundle_contributors`] lets an extension put a file into a save, and
//! [`crate::project_store`] gives it somewhere `Send + Sync` to keep that file's
//! contents between saves. Together they cover everything an extension changes
//! *in response to the writer*, because the writer's own action is already
//! marking the Work dirty through `WorkHandle::mark_changed`, and the save that
//! follows carries the extension's bytes with it.
//!
//! They do not cover an extension whose state changes **on its own**, on a
//! thread of its own, after the save that started the work has already
//! finished. That is not a hypothetical shape: an answer from a network service
//! arrives when it arrives, and the thread it arrives on cannot touch
//! `WorkHandle` at all, because that type is `Rc`-backed and belongs to the UI
//! thread by construction.
//!
//! What happened without this is the failure the whole dirty-tracking seam
//! exists to prevent, one level further out. The extension put the answer in
//! its `ProjectStore` slot, correctly. Nothing marked the Work dirty, so
//! `is_unsaved` stayed false, so Close took the `Proceed` branch **with no
//! prompt at all**, so the slot was dropped and the answer was gone. Not one
//! error anywhere, and for an answer that cannot be asked for again the loss is
//! permanent.
//!
//! ## What it is
//!
//! A generation counter per `Work.unique_id`, behind a lock, reachable from any
//! thread. An extension bumps it; the UI side compares it against the
//! generation its last save covered, exactly as it already compares `dirty_seq`
//! against `saved_seq`. Two counters rather than a flag for the same reason
//! that pair is two counters: an answer that arrives **while a save is in
//! flight** may or may not have made it into the bytes being written, and a
//! flag cleared on completion would decide that question the wrong way half the
//! time. A generation cannot: the save records the number it started from, and
//! anything bumped since is still outstanding.
//!
//! ## What it deliberately is not
//!
//! **Not a way to ask for a save.** It says a project is unsaved, which is the
//! honest thing to say, and leaves every decision about *when* to write to the
//! save queue and to the writer. An extension that could trigger a disk write
//! from a worker thread would be an extension that could defeat
//! [`crate::bundle_contributors`]'s coalescing and autosave's timing at once.
//!
//! **Not per window, and not per document.** The unit is the Work, because that
//! is the unit a save writes and the unit a Close prompt asks about.
//!
//! ## The uid, and the one value that is refused
//!
//! The empty uid belongs to an **unsaved** project, which has no file to write
//! into and no durable identity to key anything by. It is refused here rather
//! than stored, so that every unsaved project cannot pool into one entry that
//! no save ever asks about.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// One generation per project, keyed by `Work.unique_id`.
///
/// A process-wide singleton, like [`crate::bundle_contributors`]'s registry and
/// for the same reason: the end that reports and the end that reads cannot hand
/// it to each other. The reporter is an extension's worker thread, several
/// layers below any UI object; the reader is a view-model built long
/// afterwards, in another crate.
fn generations() -> &'static RwLock<HashMap<String, u64>> {
    static GENERATIONS: OnceLock<RwLock<HashMap<String, u64>>> = OnceLock::new();
    GENERATIONS.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Poison-safe, like every other lock in this crate: a panic somewhere
/// unrelated must never be what makes a manuscript unsaveable.
fn read() -> std::sync::RwLockReadGuard<'static, HashMap<String, u64>> {
    generations().read().unwrap_or_else(|e| e.into_inner())
}

fn write() -> std::sync::RwLockWriteGuard<'static, HashMap<String, u64>> {
    generations().write().unwrap_or_else(|e| e.into_inner())
}

/// Say that this project holds an extension change that is not on disk.
///
/// Callable from **any thread**, which is the whole point of it.
///
/// Returns the generation this bump produced, so a caller that wants to know
/// whether its own change has been written can hold on to the number. Most
/// callers ignore it: the interesting comparison is made on the UI side.
pub fn mark_changed(work_unique_id: &str) -> u64 {
    if work_unique_id.is_empty() {
        return 0;
    }
    let mut generations = write();
    let slot = generations.entry(work_unique_id.to_string()).or_insert(0);
    *slot += 1;
    *slot
}

/// This project's current generation. `0` for a project nothing has marked.
pub fn generation(work_unique_id: &str) -> u64 {
    if work_unique_id.is_empty() {
        return 0;
    }
    read().get(work_unique_id).copied().unwrap_or(0)
}

/// Forget a project entirely, as a close does.
///
/// Nothing is lost by it: a Work that is closing has either been written or has
/// been abandoned deliberately, and both of those are answers the writer has
/// already given.
pub fn forget(work_unique_id: &str) {
    write().remove(work_unique_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unique per test, because the registry is process-wide and the test
    /// harness is not.
    fn uid(name: &str) -> String {
        format!("external-changes-test-{name}")
    }

    #[test]
    fn a_project_nothing_marked_is_at_zero() {
        assert_eq!(generation(&uid("untouched")), 0);
    }

    #[test]
    fn marking_advances_the_generation_and_never_repeats_one() {
        let uid = uid("advances");
        assert_eq!(mark_changed(&uid), 1);
        assert_eq!(mark_changed(&uid), 2);
        assert_eq!(generation(&uid), 2);
        forget(&uid);
    }

    /// The reason this is a generation and not a flag. An answer that arrives
    /// while a save is in flight must still read as outstanding afterwards,
    /// because the bytes that save wrote may predate it.
    #[test]
    fn a_change_during_a_save_is_still_outstanding_when_that_save_lands() {
        let uid = uid("during-a-save");
        mark_changed(&uid);
        // A save starts here and records what it covers.
        let covered_by_the_save_in_flight = generation(&uid);
        // …and the answer arrives before it finishes.
        mark_changed(&uid);
        assert!(
            generation(&uid) > covered_by_the_save_in_flight,
            "a flag cleared on completion would have called this written"
        );
        forget(&uid);
    }

    /// An unsaved project has no durable identity, so there is nothing to key
    /// and nothing on disk to write into either.
    #[test]
    fn an_unsaved_project_is_refused_rather_than_pooled() {
        assert_eq!(mark_changed(""), 0);
        assert_eq!(generation(""), 0);
    }

    #[test]
    fn forgetting_a_project_puts_it_back_to_zero() {
        let uid = uid("forgotten");
        mark_changed(&uid);
        assert_eq!(generation(&uid), 1);
        forget(&uid);
        assert_eq!(generation(&uid), 0);
    }

    /// Two projects open at once keep their own counts, which is the same rule
    /// every other per-project thing in this crate follows.
    #[test]
    fn two_projects_keep_their_own_generations() {
        let a = uid("two-a");
        let b = uid("two-b");
        mark_changed(&a);
        mark_changed(&a);
        mark_changed(&b);
        assert_eq!(generation(&a), 2);
        assert_eq!(generation(&b), 1);
        forget(&a);
        forget(&b);
    }
}
