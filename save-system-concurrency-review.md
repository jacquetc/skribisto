# Save-system concurrency review

**Scope:** correctness of Skribisto's save/backup/export path under concurrency — specifically the fear that *"other actions corrupt the internal storage while a save has not fetched/frozen all the needed entities."*

**Date:** 2026-07-10 · **Branch:** `bastyde-migration`

---

## TL;DR

The fear is **justified**, and it splits into two independent root causes plus a robustness gap:

| # | Finding | Severity | Can corrupt in-memory store? | Status |
|---|---------|----------|------------------------------|--------|
| **F1** | `save_work` / `save_as` read the Work tree **non-atomically** (table-by-table, no snapshot) → torn/corrupt `.skrib` **or** the save aborts with *"vanished mid-read"* | High | No (read-only) — bad **file** / failed op | Code-evident |
| **F2** | `save_as` holds a **whole-store savepoint on a background thread**; on failure/panic its rollback reverts the **entire** store, erasing edits the UI committed meanwhile | **Critical — data loss** | **Yes** | **Empirically reproduced** |
| **F3** | Backup design = **save first, then copy the zip** (sound); but the Backup menu doesn't save first → **stale** backup if there are unsaved edits. (Folder-shape source, if supported, also needs a consistent zip source.) | Medium | No — stale **backup file** | Missing save-first, code-confirmed |
| **F4** | A **panic** while holding a store write-lock poisons it; the `Drop` rollback then re-panics → **process abort**. `LongOperation` has no `catch_unwind` → panicked op stuck `Running` | Medium | Process crash / poison spread | Reasoned from code + std semantics |

**Bounded / non-issues:** `save_work` is read-only so it cannot corrupt the store (only the file it writes); `export_work` is an `unimplemented!()` stub ([export_work_uc.rs:76](crates/export_management/src/use_cases/export_work_uc.rs#L76)) — inert today but would inherit F1 if built the same way; `import_plume_creator_file` never touches the store (pure file→file).

The reason this is reachable at all: long operations run on a **`thread::spawn`ed background thread** ([long_operation.rs:276](crates/common/src/long_operation.rs#L276)) with **zero cross-operation serialization**, while every ordinary edit runs **synchronously on the UI thread** against the same live store. Keeping the UI responsive during a save is the *designed* mode — so concurrency between a save and an edit is normal, not exotic. Debounced **autosave-to-disk** ([tabs.rs:83-86](crates/bastyde_ui/src/tabs.rs#L83), `AUTOSAVE_KEY` [main.rs:132](crates/bastyde_ui/src/main.rs#L132)) makes a background save fire *while the user keeps typing/moving items*.

---

## Architecture recap

- **Store:** one `im::HashMap` per entity table, **each behind its own `RwLock`** ([hashmap_store.rs:20-58](crates/common/src/database/hashmap_store.rs#L20)). There is **no** global store lock. `im::HashMap` clone is O(1) (structural sharing).
- **Reads** (`begin_read_transaction`) take **no snapshot** — they just clone the live `Arc<HashMapStore>` ([transactions.rs:28-34](crates/common/src/database/transactions.rs#L28)). Each generated `get_*` locks only its own table, briefly.
- **Writes** (`begin_write_transaction`) take a **whole-store savepoint** at begin ([transactions.rs:18-26](crates/common/src/database/transactions.rs#L18)); a savepoint snapshots **every table + id counters** ([hashmap_store.rs:66-135](crates/common/src/database/hashmap_store.rs#L66)).
- **Rollback is usually implicit:** feature use cases early-return with `?` and rely on `Transaction::Drop` → `restore_savepoint(T0)` ([transactions.rs:95-103](crates/common/src/database/transactions.rs#L95)). `move_items_uc` documents this: *"On any early `return Err`, the uow is dropped and its transaction auto-rolls back."* Events are **buffered** during the txn and discarded on rollback, flushed on commit ([move_items_uow.rs:45-63](crates/binder_item_management/src/units_of_work/move_items_uow.rs#L45)) — so the UI never observes reverted state.
- **Dispatch split** ([work_management_controller.rs](crates/work_management/src/work_management_controller.rs)):
  - Synchronous, UI thread (`uc.execute()`): `load_work`, `close_work`, `new_work`, and every direct-access / binder-item / trash mutation.
  - Background thread (`start_operation` → `thread::spawn`): `save_work` (read), **`save_as` (write)**, `backup_now` (read), plus `export_work` / `import_plume` in their own controllers.

The qleany undo doc's savepoint guidance is explicit that a savepoint is *"the nuclear option… reverts the entire database… keep your finger away from it,"* and that it is only meant as a **rollback-on-failure net for a single in-flight transaction** (*"long operations… rely on transaction rollback"*). That advice is sound **for a serialized command model** — which Skribisto honours everywhere **except** `save_as`.

---

## Findings in detail

### F1 — Non-atomic tree read (torn / failed save) · High

`work_io::gather` ([work_io.rs:90-146](crates/work_management/src/work_io.rs#L90)) reads `all_work` → `all_work_info` → `work_rel` → `binder_multi` → `binder_rel` → `item_multi` → `item_rel` → `content_multi`, **each acquiring and releasing only its own table's lock**. No snapshot, no lock spanning the walk. Because the save runs on a background thread while the UI thread mutates the store, an edit landing *between* two of those reads yields one of:

- **Aborted save:** a junction read earlier now references an entity a concurrent delete/move removed → `fetch_multi` errors *"entity {id} vanished mid-read"* ([work_io.rs:215](crates/work_management/src/work_io.rs#L215)) and the whole save fails.
- **Torn file:** a junction read before a mutation is serialized against entities read after it → a `.skrib` whose `items.ron` references content that no longer matches, an item added after its binder's child-list was read is silently missing, etc.

**Trigger:** autosave (or manual Save) fires; user keeps typing / moves an item / undoes while the background gather is walking.

**Blast radius:** the on-disk file only. `save_work` never writes the store, so the in-memory model stays intact. Still: silent corruption of the saved project, or a save that "randomly" fails.

> Note: even `HashMapStore::snapshot()` ([hashmap_store.rs:66](crates/common/src/database/hashmap_store.rs#L66)) is itself **not** atomic across tables — it clones each table under a separate lock. So "just call snapshot()" does not fix F1 unless the snapshot is made atomic (see fix).

---

### F2 — `save_as` background rollback destroys concurrent edits · Critical (data loss)

`save_as` is the **only** write use case (`begin_write_transaction` → whole-store savepoint) dispatched on a **background thread**. On failure it explicitly `uow.rollback()` ([save_as_uc.rs:133-136](crates/work_management/src/use_cases/save_as_uc.rs#L133)); on panic/early-drop the `Transaction::Drop` net does the same. Either way it calls `restore_savepoint(T0)`, which overwrites **every table** with the snapshot taken when `save_as` began — **including rows the UI thread committed in the meantime.**

**Empirically reproduced** against the real `Transaction` + `HashMapStore` (throwaway test, since removed):

```
PART1: serial Drop-rollback reverted the partial write — CLAIM HOLDS (serial).
PART2: after background rollback, work1 = Some("original")
PART2: the committed UI edit was ERASED by the background op's rollback — CLAIM BREAKS under concurrency.
```

PART1 confirms rollback is correct in the serial single-writer model. PART2 sets up the real interleaving — background `save_as` takes savepoint T0, the UI thread commits an edit, `save_as` fails and rolls back — and shows the committed edit reverted to its T0 value.

**Concrete interleaving:**
1. User picks *Save As…*; `save_as` starts on a background thread, `begin_write_transaction` snapshots the whole store at **T0**.
2. Gather + file write take time (the reason it's a long op). Meanwhile the user edits a scene / moves an item; that runs synchronously on the UI thread through its own write txn and **commits** into the live store.
3. `save_as` hits an error (disk full, path perms, cancel, a panic) → `rollback()` / `Drop` → `restore_savepoint(T0)` → the whole store snaps back to T0, **silently discarding the user's edit.**

This is the "nuclear option" the undo doc warns about, fired across threads. It is the one place where *"the transaction rollback handles failures"* is not merely insufficient but **actively destructive**.

---

### F3 — backup relies on a "save first" invariant it doesn't enforce · Medium

**Intended design (confirmed):** the in-memory store is the source of truth; a backup is always a zip, and **a backup saves the work first**, so copying the freshly-written on-disk zip is a sound shortcut for a zip serialization of the store. Given that invariant, `backup_now`'s `copy_bundle(source, backup_path)` ([backup_now_uc.rs:49-65](crates/work_management/src/use_cases/backup_now_uc.rs#L49)) is correct for zip-mode projects: `fs::copy` of a fully-written zip is atomic (saves write via `NamedTempFile` + rename), and after a save the on-disk zip *is* the current store. No concurrency issue, no reimplementation needed. **My earlier "serialize the store in `backup_now`" recommendation was over-engineering — retracted.**

The actual gaps are narrower:

1. **The save-first invariant is not enforced in the flow.** The Backup menu item calls `backup_now` directly with no preceding save ([main.rs:556-561](crates/bastyde_ui/src/main.rs#L556)). So a backup taken with unsaved in-memory edits copies a **stale** zip (missing the latest work). Fix: route Backup through a save-first step (the app already has an "autosave-ensure" guard on `work.close` — reuse it).
2. **Folder-shape source (only if that path is intended).** A folder-mode project has no on-disk zip to copy, so `copy_bundle` `zip_dir`s the **live** folder ([lib.rs:85](crates/skrib_format/src/lib.rs#L85)); that walk is not atomic, so a concurrent autosave during it can tear the backup. If folder-mode projects should be backable, produce their zip from the store / a just-saved consistent source rather than walking the live tree. (If folder-mode backup is out of scope, this is moot.)

---

### F4 — Panic robustness gap · Medium

*"Transaction rollback handles failures"* covers `Result::Err` only. Under a **panic**:

- The store's `RwLock`s use plain `.unwrap()` throughout ([hashmap_store.rs](crates/common/src/database/hashmap_store.rs)). A panic **while a write guard is held** poisons that lock. The `Transaction::Drop` net then calls `restore_savepoint` → `restore()` → `self.<table>.write().unwrap()` on the poisoned lock → **double-panic → process abort** instead of a clean rollback. `restore_savepoint` also `.expect("savepoint not found")`s.
- `LongOperationManager::start_operation` runs `operation.execute(...)` with **no `catch_unwind`** ([long_operation.rs:299](crates/common/src/long_operation.rs#L299)). A panicked long-op unwinds its thread; the `final_status` code never runs, so the op is stuck reporting `Running`, and any lock it poisoned breaks the UI thread's next store access.

---

## Root causes

1. **Reads are not snapshot-isolated (and can't be made so cheaply as written).** Per-table locks + no atomic freeze → F1, and the read half of F2.
2. **A whole-store savepoint is held by a writer on a background thread.** Savepoint rollback is whole-store by design; safe only when commands are serialized. `save_as` breaks that assumption → F2.
3. **Backup's "save first" invariant isn't enforced** — the Backup menu copies the on-disk zip without saving first → stale backup → F3. (Copying the zip itself is sound once a save precedes it.)
4. **Locks/threads are not panic-hardened** → F4.

Causes 1 and 2 are orthogonal; `save_as` is unlucky enough to hit both.

---

## Fix design

Constraint: the store / transaction / long-op machinery is **qleany-generated** (`hashmap_store.tera`, `transactions.tera`, `long_operation.tera`) — do not hand-edit; changes belong in the templates + regenerate. But `save_work_uc`, `save_as_uc`, `backup_now_uc`, `work_io.rs` are **hand-maintained** (their headers say so) and can be edited directly. So the plan is tiered: stop the critical bleeding in hand-written code now, then do the proper isolation via templates.

### Tier 1 — Stop the data loss (hand-written only, ship first)

**F2 fix — make `save_as` read-only on the background thread.** It should never hold a write savepoint off the UI thread. Split it:

- Background op: gather (read) + write the file. **No `begin_write_transaction`, no rollback.** (Reuse the read UoW like `save_work` does.)
- The one mutation it needs — recording the new path/shape in `WorkInfo` — is applied **synchronously on the UI thread** in the completion handler, via the ordinary `update_work_info` command. This is exactly what `save_work_uc`'s own header already describes (*"a 'Save As' path/shape change is applied by the UI via update_work_info"*).

```rust
// save_as_uc (background): read-only, returns the resolved path + intended shape
struct SaveAsResultDto { output_path: String, new_shape: WorkShape /* Folder|Zip */ }
// ...gather via begin_read_transaction; serialize_and_write(...); return Ok(result)

// bastyde_ui completion handler (UI thread), on SaveAs "Completed":
work_info_commands::update_work_info(ctx, &WorkInfo { file_name: Some(out), shape, .. });
```

Result: no background thread ever holds a whole-store savepoint → **F2 eliminated**, entirely in hand-written code.

**F3 fix — enforce save-first before backup (hand-written, `bastyde_ui`).** Route the Backup menu action through the same save/autosave-ensure step used by `work.close`, then call `backup_now`. For a zip-mode project the copy is then a current, atomic snapshot — no change to `backup_now` itself. (Only if folder-mode backup is in scope: additionally produce that zip from the store / a just-saved source instead of walking the live folder.)

### Tier 2 — Snapshot-isolated reads for all long-ops (templates + regen)

Give long operations a **consistent, isolated** view so the slow file I/O runs against frozen data and torn reads are impossible. Two viable mechanisms:

| Option | Approach | Pros | Cons |
|--------|----------|------|------|
| **A. Atomic freeze (recommended)** | Add `HashMapStore::freeze()` that acquires **all** table write-locks at once, clones every table (O(1) `im` Arc-bumps), releases — a microsecond "stop-the-world". Long-op read use cases gather from a `DbContext` wrapping a store restored from that frozen snapshot; **no locks held during file I/O.** | Fixes F1 for `save_work`/`save_as`/future `export` with one primitive; read path stays lock-free during slow I/O; fully isolated (concurrent edits don't affect the frozen copy). | Touches `hashmap_store.tera`; brief all-locks acquisition (still O(1) work, microseconds). |
| **B. Global store RwLock** | One `RwLock<()>` gate in the store: writers hold `write()` for their txn, `freeze` takes `write()` just to clone. | Also makes writes mutually exclusive (defuses any future multi-writer). | Serializes writers globally; more invasive to the write path; still needs the freeze to isolate the long I/O. |
| **C. Serialize saves vs writes** | A single lock: saves take it shared, writes exclusive, held for the whole op. | Simplest to reason about. | Reintroduces the UI stalls the long-op design exists to avoid. Rejected. |

**Recommended: Option A.** Sketch:

```rust
// hashmap_store.tera — atomic freeze (all locks, then O(1) clones)
pub fn freeze(&self) -> HashMapStoreSnapshot {
    let _g_roots   = self.roots.write().unwrap();
    let _g_binders = self.binders.write().unwrap();
    /* ...acquire every table + junction write guard in a fixed order... */
    self.snapshot() // clones are O(1); guards drop at end of fn
}
```
```rust
// save_work_uc / save_as_uc (background) — read from the frozen copy
let frozen: DbContext = self.uow_factory.freeze_context()?; // wraps a store restored from freeze()
let g = work_io::gather(&read_uow_over(frozen), progress, cancel)?; // now torn-read-free
work_io::serialize_and_write(&g, target, shape, tag)?; // slow I/O, nothing locked
```

**F3** needs no isolation work at all — enforcing save-first (above) makes the zip copy a current, atomic snapshot. Only a folder-mode backup path (if in scope) would benefit from producing its zip via the freeze.

### Tier 3 — Panic hardening (templates)

- Wrap `operation.execute(...)` in `std::panic::catch_unwind` in `long_operation.tera`, mapping a caught panic to `OperationStatus::Failed` so a panicked long-op reports failure instead of hanging at `Running`.
- Replace store-lock `.unwrap()` with poison-recovery (the `lock_or_recover` pattern already used in [long_operation.rs:171](crates/common/src/long_operation.rs#L171)) in `hashmap_store.tera`, so a mid-write panic can't turn a rollback into a process abort.

### Suggested regression tests

- **Atomicity:** spawn a writer thread hammering binder-item add/move/delete while `gather`/`freeze` runs; assert the produced bundle is internally consistent and never errors *"vanished mid-read"*.
- **No clobber (F2 guard):** the PART2 interleaving from this review — background op begins, UI commits an edit, background op fails — must leave the UI edit intact. (Keep a permanent version of the throwaway test.)
- **Backup saves first (F3):** edit in memory without saving, then trigger Backup → the produced zip must contain that edit (proving a save ran before the copy).

---

## Priority

1. **F2 (Tier 1)** — critical, silent data loss; small, hand-written fix. Do first.
2. **F3 (hand-written)** — enforce save-first before backup; small `bastyde_ui` change. Fixes the stale-backup gap.
3. **F1 (Tier 2, Option A)** — the atomic-freeze primitive gives consistent reads to `save_work` / `save_as` / future `export` at once.
4. **F4 (Tier 3)** — robustness hardening; do alongside Tier 2 since both are template edits.
