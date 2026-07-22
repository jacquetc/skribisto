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
//! ## Holding the `Arc` — no stale-pointer ABA
//!
//! The map is keyed by `Arc::as_ptr(db_context.get_store())`, a raw address —
//! not by anything that keeps that address meaningful on its own. Early drafts
//! of this guard stored only that `usize` and neither borrowed the `DbContext`
//! nor cloned its `Arc`. That is unsound: if the `DbContext` a guard was
//! acquired from is dropped (its last other clone going out of scope) while
//! the guard still lives, the allocator is free to reuse that exact address
//! for an unrelated, brand-new `HashMapStore`. The new store's *first* real
//! write transaction would then find the address already marked as held —
//! tripping the guard for a store that never had a concurrent writer — and
//! worse, when the stale guard eventually dropped it would remove the *new*
//! store's entry, silently re-opening the door the guard exists to close.
//!
//! [`WriteTransactionGuard`] therefore clones the `Arc<HashMapStore>` itself
//! (the `store` field below) and holds that clone for its entire lifetime, in
//! addition to using its address as the map key. Holding the clone keeps the
//! allocation alive — the address cannot be freed, let alone reused for a
//! different store, for as long as any guard referencing it exists. The
//! pointer is still fine to use as a `HashMap` key (it is still unique among
//! *currently live* stores); what changed is that the guard now makes that
//! uniqueness durable for its own lifetime instead of merely observing it at
//! acquire time.
//!
//! ## Diagnosing contention: naming both the current holder and the new claimant
//!
//! Each slot records not just "held", but who holds it: the owning thread's
//! [`ThreadId`](std::thread::ThreadId) and name, plus the `site` string passed
//! to `acquire`. A contending `acquire` call reads that record before failing,
//! so the panic/error names **both** parties — "`{new_site}` on thread {new}
//! collided with `{holder_site}` still held by thread {holder}" — not just the
//! new claimant. That is enough to `grep` straight to the offending call site
//! without attaching a debugger, which is the entire point of a guard whose
//! job is to fail loudly instead of silently.
//!
//! This module deliberately does **not** add any machinery to reclaim a slot
//! left held by a leaked or `mem::forget`-ed guard (a timeout, a generation
//! counter, a "steal the lock" escape hatch). Two things make that
//! unnecessary rather than merely deferred:
//!
//! * The ABA fix above already rules out the scenario that would make a stale
//!   key *dangerous* — a leaked guard cannot cause a *different, live* store to
//!   be wrongly blocked, because it holds the `Arc` that keeps its own store's
//!   address from ever being handed to a different store while the leak
//!   persists. A leak only ever poisons the one store it actually leaked on.
//! * For that one store, "a write-transaction slot that is held and never
//!   released" and "a write transaction that is still legitimately in
//!   progress" are the same observable state from this module's point of view.
//!   Any reclaim heuristic (e.g. "assume held-for-N-seconds means leaked") is a
//!   race against a legitimately long transaction and would trade a loud,
//!   honest failure for an occasional silent one — reintroducing exactly the
//!   failure mode this guard exists to prevent. The recorded thread id/site
//!   above turn that failure into an immediately diagnosable one (which call
//!   site leaked it), which is the correct fix for a bug, not a runtime
//!   work-around for one.
//!
//! ## Re-entrant `begin_transaction` on the same unit of work is diagnosed honestly
//!
//! Every wired `CommandUnitOfWork::begin_transaction` (see "Coverage" above)
//! follows the same shape:
//!
//! ```ignore
//! self.write_guard = Some(WriteTransactionGuard::acquire(&self.context, "close_work")?);
//! ```
//!
//! Assignment evaluates its right-hand side — the whole `acquire` call —
//! *before* dropping whatever `self.write_guard` held previously. So if
//! `begin_transaction` were ever called again on the *same* `CommandUnitOfWork`
//! instance while its own previous guard is still `Some` (a re-entrant call, or
//! a retry loop that calls `begin_transaction` again without going through
//! `commit`/`rollback` first — both of which clear the field), the new
//! `acquire` runs while the old guard is still registered as held. Naively that
//! looks identical to a genuine second writer, and the resulting panic/error
//! would misidentify a single UoW talking to itself as two concurrent writers
//! — plausible-sounding, and wrong, which is worse than no diagnostic at all.
//!
//! `acquire` tells the two apart using exactly the holder record described
//! above: if the slot is already held **by the current thread**, the message
//! says so explicitly — "this thread already holds this store's slot,
//! acquired at `{holder_site}`, this looks like a re-entrant/retried
//! `begin_transaction`, not a second writer" — instead of the generic
//! cross-thread contention wording. It still fails (the underlying invariant
//! violation — two live guards on one store — is exactly as real as in the
//! cross-thread case, and this module cannot see into the caller's struct to
//! silently drop the stale one for it), but the diagnostic now points at the
//! actual bug (a UoW re-entering `begin_transaction` without clearing its own
//! previous guard) instead of a phantom concurrent writer.
//!
//! ## RAII — cannot be defeated by an early return or a panic
//!
//! [`WriteTransactionGuard`] is a plain owned struct with only a `Drop` impl to
//! release its slot; there is no separate "release" method to forget to call.
//! Binding it to a named local for the whole span a write transaction may be open
//! means the slot is freed on every exit path — the happy path, an early `?`
//! return, or an unwinding panic — exactly like `Transaction`'s own `Drop` safety
//! net that it sits beside.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::ThreadId;

use crate::database::db_context::DbContext;
use crate::database::hashmap_store::HashMapStore;

/// Who currently holds a store's write-transaction slot — enough to name them
/// in a contention panic/error (see the module doc's "Diagnosing contention"
/// section) instead of just saying "someone already holds it".
#[derive(Clone, Debug)]
struct Holder {
    thread_id: ThreadId,
    thread_name: Option<String>,
    site: &'static str,
}

impl Holder {
    fn describe(&self) -> String {
        match &self.thread_name {
            Some(name) => format!("{name:?} ({:?})", self.thread_id),
            None => format!("<unnamed> ({:?})", self.thread_id),
        }
    }
}

/// Which stores (keyed by `Arc<HashMapStore>` pointer identity) currently have a
/// write transaction in flight, and who holds each one. `None` until the first
/// acquire; lazily initialised rather than built from a const-unfriendly
/// `HashMap::new()` in a `static` initializer.
static WRITERS: Mutex<Option<HashMap<usize, Holder>>> = Mutex::new(None);

/// RAII token proving "I hold the write-transaction slot for this store." See
/// the module doc for the full rationale.
#[must_use = "the slot is released when this drops — binding it to `_` drops it \
              immediately and defeats the guard; bind it to a named local (e.g. \
              `_write_guard`) that lives for the whole write-transaction span"]
#[derive(Debug)]
pub struct WriteTransactionGuard {
    key: usize,
    /// Never read directly — its only job is to keep the store's allocation
    /// (and hence `key`'s validity as a pointer identity) alive for as long as
    /// this guard exists. See the module doc's "Holding the `Arc`" section.
    #[allow(dead_code)]
    store: Arc<HashMapStore>,
}

impl WriteTransactionGuard {
    /// Claim the write-transaction slot for `db_context`'s store. `site` names
    /// the call site, used only in the panic/error message.
    ///
    /// # Panics
    /// In debug builds, panics immediately if this store's slot is already
    /// held — some other write transaction is already open on it. Turns the
    /// silent-data-loss failure mode this guards against into a loud one, at
    /// the exact call site that introduced the second writer (or, for a
    /// same-thread re-entry, the call site that never released its own
    /// previous guard — see the module doc).
    ///
    /// # Errors
    /// In release builds, returns an error instead of panicking — a shipped
    /// build should degrade to a reported failure, not abort the process.
    pub fn acquire(db_context: &DbContext, site: &'static str) -> anyhow::Result<Self> {
        // Clone (not just dereference) the `Arc`: the guard keeps this clone
        // alive for its whole lifetime so `key` can never be handed out to a
        // different store by the allocator while a guard referencing it
        // exists. See the module doc's "Holding the `Arc`" section.
        let store = Arc::clone(db_context.get_store());
        let key = Arc::as_ptr(&store) as usize;

        let current_thread = std::thread::current();
        let current_thread_id = current_thread.id();
        let current_thread_name = current_thread.name().map(str::to_owned);

        // Take the lock, mutate, and drop it again *before* possibly panicking —
        // panicking while holding a `std::sync::Mutex` poisons it for every
        // other store's acquire/drop for the rest of the process, which would
        // turn one store's legitimate bug into spurious failures for every
        // other, wholly unrelated store.
        let existing_holder = {
            let mut writers = WRITERS.lock().unwrap_or_else(|e| e.into_inner());
            let map = writers.get_or_insert_with(HashMap::new);
            match map.get(&key) {
                Some(holder) => Some(holder.clone()),
                None => {
                    map.insert(
                        key,
                        Holder {
                            thread_id: current_thread_id,
                            thread_name: current_thread_name,
                            site,
                        },
                    );
                    None
                }
            }
        };

        if let Some(holder) = existing_holder {
            let new_thread_desc = current_thread.name().map_or_else(
                || format!("{current_thread_id:?}"),
                |n| format!("{n:?} ({current_thread_id:?})"),
            );

            let message = if holder.thread_id == current_thread_id {
                format!(
                    "WriteTransactionGuard: `{site}` (thread {new_thread_desc}) tried to open a \
                     write transaction on a store THIS SAME THREAD already holds the slot for — \
                     acquired earlier at `{holder_site}` and never released. This is NOT a \
                     second concurrent writer; it is a re-entrant or retried `begin_transaction` \
                     on the same (or a nested) unit of work whose previous \
                     `WriteTransactionGuard` was never dropped — either a missing \
                     commit()/rollback() before the retry, or `self.write_guard = Some(new)` \
                     built `new` (running `acquire` again) before the old guard's `Drop` had a \
                     chance to run. Drop the previous guard (commit/rollback it, or explicitly \
                     `self.write_guard = None;`) before acquiring a new one.",
                    holder_site = holder.site,
                )
            } else {
                format!(
                    "WriteTransactionGuard: a second write transaction started at `{site}` \
                     (thread {new_thread_desc}) while another was already open on this store, \
                     held by thread {holder} since `{holder_site}`. \
                     Transaction::begin_write_transaction takes a WHOLE-STORE savepoint \
                     (common/src/database/transactions.rs) — a second concurrent writer's \
                     rollback/Drop would silently erase the first writer's committed edits. \
                     Fix the caller; do not remove or weaken this guard.",
                    holder = holder.describe(),
                    holder_site = holder.site,
                )
            };

            if cfg!(debug_assertions) {
                panic!("{message}");
            }
            anyhow::bail!(message);
        }

        Ok(WriteTransactionGuard { key, store })
    }
}

impl Drop for WriteTransactionGuard {
    fn drop(&mut self) {
        if let Some(map) = WRITERS.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
            map.remove(&self.key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> DbContext {
        DbContext::new().unwrap()
    }

    /// Extracts the panic payload as a string regardless of whether it was
    /// boxed as `&'static str` (a literal) or `String` (an interpolated
    /// `format!`/`panic!("{message}")` payload, which is what this module's
    /// contention messages are).
    fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
        if let Some(s) = payload.downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.clone()
        } else {
            panic!("panic payload was neither &str nor String");
        }
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

    // ── Finding 2: ABA — the guard must keep the store's `Arc` alive ────────

    #[test]
    fn the_guard_keeps_the_store_allocation_alive_after_the_db_context_drops() {
        let db = ctx();
        let store_ref = Arc::clone(db.get_store());
        // `db` itself + our `store_ref` clone.
        assert_eq!(Arc::strong_count(&store_ref), 2);

        let guard = WriteTransactionGuard::acquire(&db, "test").unwrap();
        // Acquiring must have taken its own clone: three owners now.
        assert_eq!(
            Arc::strong_count(&store_ref),
            3,
            "WriteTransactionGuard::acquire must clone the store's Arc, not just read its pointer"
        );

        drop(db);
        // With the DbContext gone, only the guard's clone (+ ours) keep the
        // allocation alive. If the guard did NOT hold a clone (the finding-2
        // bug), this would drop to 1 and the address would become free for
        // the allocator to hand to an unrelated, brand-new store — the ABA
        // scenario the module doc describes.
        assert_eq!(
            Arc::strong_count(&store_ref),
            2,
            "the guard must still be holding the store alive once the DbContext drops"
        );

        drop(guard);
        assert_eq!(
            Arc::strong_count(&store_ref),
            1,
            "the guard's clone must be released when the guard drops"
        );
    }

    // ── Finding 7: contention diagnostics name both parties ─────────────────

    #[test]
    fn cross_thread_contention_names_both_the_holder_and_the_new_claimant() {
        let db = ctx();
        let _first = WriteTransactionGuard::acquire(&db, "first_site_xyz").unwrap();

        let db_for_thread = db.clone();
        let handle = std::thread::Builder::new()
            .name("contender-thread".to_string())
            .spawn(move || {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    WriteTransactionGuard::acquire(&db_for_thread, "second_site_xyz")
                }))
            })
            .unwrap();

        let result = handle.join().expect("spawned thread must not itself panic outside catch_unwind");
        let payload = result.expect_err("a genuine cross-thread contention must panic in a debug build");
        let message = panic_message(payload.as_ref());

        assert!(
            message.contains("first_site_xyz"),
            "message must name the current holder's call site: {message}"
        );
        assert!(
            message.contains("second_site_xyz"),
            "message must name the new claimant's call site: {message}"
        );
        assert!(
            message.contains("contender-thread"),
            "message must name the new claimant's thread: {message}"
        );
        assert!(
            !message.contains("SAME THREAD"),
            "a genuine cross-thread writer must not be misdiagnosed as same-thread \
             re-entry: {message}"
        );
    }

    // ── Finding 8: same-thread re-entry is diagnosed honestly ───────────────

    #[test]
    fn same_thread_reacquire_is_diagnosed_as_self_reentry_not_a_phantom_second_writer() {
        let db = ctx();
        let _first = WriteTransactionGuard::acquire(&db, "begin_transaction_first_call").unwrap();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            WriteTransactionGuard::acquire(&db, "begin_transaction_retry")
        }));
        let payload =
            result.expect_err("re-acquiring on the same thread while still held must still fail");
        let message = panic_message(payload.as_ref());

        assert!(
            message.contains("begin_transaction_first_call"),
            "message must name the site that still holds the slot: {message}"
        );
        assert!(
            message.contains("begin_transaction_retry"),
            "message must name the re-entrant call site: {message}"
        );
        assert!(
            message.contains("SAME THREAD") || message.contains("re-entrant"),
            "message must say this is the same thread re-entering, not blame a second \
             writer: {message}"
        );
        assert!(
            !message.contains("Fix the caller; do not remove or weaken this guard"),
            "must not reuse the generic cross-thread-second-writer wording for a \
             same-thread self re-entry: {message}"
        );
    }
}
