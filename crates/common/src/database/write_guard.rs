// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Enforces the single-write-transaction-per-store invariant.
//!
//! ## Why this exists
//!
//! `Transaction::begin_write_transaction` ([`transactions.rs`](super::transactions),
//! Qleany-generated — do not edit) takes an unconditional WHOLE-STORE savepoint via
//! `HashMapStore::create_savepoint`, whose own doc comment states the assumption
//! directly: taken "on the single writer thread with no concurrent writer".
//! `rollback()` and the `Drop` safety net on `Transaction` both restore that
//! savepoint wholesale — not just the rows the transaction itself touched. This is
//! current, intended Qleany behaviour (regenerating both files from qleany 1.8.0
//! with `--temp` reproduces them byte-for-byte); it is not a bug to fix in the
//! generated layer.
//!
//! Skribisto holds "single writer, no concurrent writer" today only by
//! construction: every write-transaction call site in the whole workspace —
//! `work_management`'s `load_work`/`new_work`/`close_work`, the editing use
//! cases in `binder_item_management`/`trash_management`/`tag_management`/
//! `search_management`/`progress_management`/`handling_app_lifecycle`, and the
//! per-entity CRUD `WriteUoW`s generated into `direct_access` (one per entity,
//! e.g. editing a Content's title or a DictWord) — runs synchronously on the
//! UI thread. None is wired through a `LongOperation` background thread the way
//! `save_work`/`save_as`/`backup_now` are (those use
//! `begin_frozen_read_transaction` specifically so they *can* run in the
//! background). Nothing before this guard enforced that; it was an accident of
//! "nobody has moved these off-thread yet". If a second write transaction on
//! the same store were ever opened concurrently with one already in flight —
//! e.g. a future change moves `load_work` to a background thread to stop
//! freezing every other open Work's window while it unzips + parses — the
//! second transaction's own `commit`/`rollback`/`Drop` would silently roll back
//! to a savepoint taken *before* the first transaction's edits ever existed,
//! erasing them with no error and no event.
//!
//! [`WriteTransactionGuard`] turns that assumption into an assertion. Acquire
//! one for the entire span a write transaction may be open. A second
//! concurrent acquisition **on the same store** panics in debug builds — loud,
//! immediate, naming the call site — and returns an error in release builds,
//! so a shipped build degrades to a reported failure instead of silently
//! corrupting the store.
//!
//! ## Coverage — every real write-transaction call site is wired, by hand
//!
//! `Transaction::begin_write_transaction` and the code that calls it
//! (`begin_transaction` on each `CommandUnitOfWork` impl) are Qleany-generated;
//! this crate's house rule is not to hand-edit generated files. There is,
//! however, no single generated chokepoint shared by every write-transaction
//! call site to hook the guard into once — each use case's/entity's `WriteUoW`
//! calls `Transaction::begin_write_transaction` directly in its own generated
//! file. Closing the gap for real therefore meant hand-adding the
//! `write_guard` field/acquire to **every one of those files**, each marked
//! "Modify-and-protect scaffold" (or folded into an existing "Custom
//! implementation" marker) so a future blanket regeneration does not silently
//! drop the guard. As of this writing that is the three `work_management`
//! sites named above, the 17 feature-use-case write UoWs in
//! `binder_item_management`/`trash_management`/`tag_management`/
//! `search_management`/`progress_management`/`handling_app_lifecycle`, and the
//! 17 per-entity `direct_access` write UoWs — every `grep -rl
//! begin_write_transaction crates/` hit outside this module and the generated
//! `transactions.rs`/`hashmap_store.rs` themselves.
//!
//! **This list is not self-maintaining.** Qleany's use-case and entity
//! scaffolds (`feature_use_case_uow.tera`, `entity_units_of_work.tera`) do not
//! know about this guard, so a *new* use case or entity generated after this
//! comment was written starts out unguarded — `qleany generate` has no reason
//! to add the `write_guard` field, and nothing else will remind you to. Adding
//! one: acquire a `WriteTransactionGuard` as the first statement of
//! `begin_transaction` (before `Transaction::begin_write_transaction`), hold it
//! in an `Option<WriteTransactionGuard>` struct field, and clear it (`= None`)
//! in both `commit()` and `rollback()` — copy the pattern from any file listed
//! above, e.g. `work_management/src/units_of_work/close_work_uow.rs`. The
//! guard's own contention check does not care who forgot: a genuinely
//! concurrent write transaction from an unguarded site simply proceeds
//! unnoticed instead of panicking, which is exactly the silent-rollback failure
//! mode this module exists to turn loud — so treat "does this new write
//! transaction have a `write_guard` field" as a mandatory review question, not
//! an optional hardening step.
//!
//! ## Why scoped per store, not one bare global flag
//!
//! Production constructs exactly one `DbContext` (hence one store) for the whole
//! process, so a single process-wide flag would be an equally correct proxy for
//! "a write transaction is open on THE store" there. But `work_management`'s own
//! tests (`save_load_test.rs`) and several of `frontend`'s integration tests each
//! build a fresh, independent `DbContext`/store per `#[test]` fn, and `cargo
//! test`'s default runner executes those fns concurrently on separate threads —
//! two fns opening a write transaction on their OWN unrelated stores can
//! legitimately overlap in wall-clock time. A bare process-wide flag would make
//! those spuriously trip each other's guard. Keying by the store's `Arc` pointer
//! identity models the real invariant (one writer *per store*, not one writer
//! *ever*) without that flakiness — and needs no test-only carve-out to stay
//! correct.
//!
//! ## RAII — cannot be defeated by an early return or a panic
//!
//! [`WriteTransactionGuard`] is a plain owned struct with only a `Drop` impl to
//! release its slot; there is no separate "release" method to forget to call.
//! Binding it to a named local for the whole span a write transaction may be open
//! means the slot is freed on every exit path — the happy path, an early `?`
//! return, or an unwinding panic — exactly like `Transaction`'s own `Drop` safety
//! net that it sits beside.

use std::collections::HashSet;
use std::sync::Mutex;

use crate::database::db_context::DbContext;

/// Which stores (keyed by `Arc<HashMapStore>` pointer identity) currently have a
/// write transaction in flight. `None` until the first acquire; lazily
/// initialised rather than built from a const-unfriendly `HashSet::new()` in a
/// `static` initializer.
static WRITERS: Mutex<Option<HashSet<usize>>> = Mutex::new(None);

/// RAII token proving "I hold the write-transaction slot for this store." See
/// the module doc for the full rationale.
#[must_use = "the slot is released when this drops — binding it to `_` drops it \
              immediately and defeats the guard; bind it to a named local (e.g. \
              `_write_guard`) that lives for the whole write-transaction span"]
pub struct WriteTransactionGuard {
    key: usize,
}

impl WriteTransactionGuard {
    /// Claim the write-transaction slot for `db_context`'s store. `site` names
    /// the call site, used only in the panic/error message.
    ///
    /// # Panics
    /// In debug builds, panics immediately if this store's slot is already
    /// held — some other write transaction is already open on it. Turns the
    /// silent-data-loss failure mode this guards against into a loud one, at
    /// the exact call site that introduced the second writer.
    ///
    /// # Errors
    /// In release builds, returns an error instead of panicking — a shipped
    /// build should degrade to a reported failure, not abort the process.
    pub fn acquire(db_context: &DbContext, site: &'static str) -> anyhow::Result<Self> {
        let key = std::sync::Arc::as_ptr(db_context.get_store()) as usize;

        // Take the lock, mutate, and drop it again *before* possibly panicking —
        // panicking while holding a `std::sync::Mutex` poisons it for every
        // other store's acquire/drop for the rest of the process, which would
        // turn one store's legitimate bug into spurious failures for every
        // other, wholly unrelated store.
        let already_held = {
            let mut writers = WRITERS.lock().unwrap_or_else(|e| e.into_inner());
            let set = writers.get_or_insert_with(HashSet::new);
            !set.insert(key)
        };

        if already_held {
            if cfg!(debug_assertions) {
                panic!(
                    "WriteTransactionGuard: a second write transaction started at \
                     `{site}` while another was already open on this store. \
                     Transaction::begin_write_transaction takes a WHOLE-STORE savepoint \
                     (common/src/database/transactions.rs) — a second concurrent \
                     writer's rollback/Drop would silently erase the first writer's \
                     committed edits. Fix the caller; do not remove or weaken this guard."
                );
            }
            anyhow::bail!(
                "internal error: a second write transaction started at `{site}` while \
                 another was already open on this store (see \
                 common::database::write_guard's module doc)"
            );
        }

        Ok(WriteTransactionGuard { key })
    }
}

impl Drop for WriteTransactionGuard {
    fn drop(&mut self) {
        if let Some(set) = WRITERS
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_mut()
        {
            set.remove(&self.key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> DbContext {
        DbContext::new().unwrap()
    }

    #[test]
    fn sequential_acquire_release_never_trips_the_guard() {
        let db = ctx();
        for _ in 0..3 {
            let g = WriteTransactionGuard::acquire(&db, "test").unwrap();
            drop(g);
        }
    }

    #[test]
    fn two_different_stores_can_each_acquire_independently() {
        let a = ctx();
        let b = ctx();
        let _ga = WriteTransactionGuard::acquire(&a, "a").unwrap();
        let _gb = WriteTransactionGuard::acquire(&b, "b").unwrap();
        // No panic reaching here is the assertion: two unrelated stores must
        // never contend on each other's slot.
    }

    #[test]
    fn dropping_the_first_guard_frees_the_slot_for_a_second_acquire() {
        let db = ctx();
        let first = WriteTransactionGuard::acquire(&db, "first").unwrap();
        drop(first);
        let second = WriteTransactionGuard::acquire(&db, "second");
        assert!(second.is_ok(), "the slot must be free once the first guard dropped");
    }

    #[test]
    fn a_second_concurrent_acquire_on_the_same_store_panics() {
        let db = ctx();
        let _first = WriteTransactionGuard::acquire(&db, "first").unwrap();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            WriteTransactionGuard::acquire(&db, "second")
        }));
        assert!(
            result.is_err(),
            "a concurrent second acquire on the same store must panic in a debug build"
        );

        // The panic must not have poisoned the global map for anyone else: a
        // fresh, unrelated store can still acquire immediately afterward.
        let other = ctx();
        assert!(WriteTransactionGuard::acquire(&other, "other").is_ok());
    }
}
