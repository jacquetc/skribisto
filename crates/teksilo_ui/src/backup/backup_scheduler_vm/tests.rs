// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use std::cell::Cell;

fn test_scheduler() -> BackupSchedulerViewModel {
    let app_ctx = Rc::new(AppContext::new());
    let ids = AppIds::new();
    let settings =
        BackupSettingsViewModel::new(crate::models::BackupSettingsService::in_memory_default());
    let single_work = SingleWork::new(app_ctx.clone());
    let single_work_info = SingleWorkInfo::new(app_ctx.clone());
    let backup_mode = Signal::new(false);
    let workspace_layout = WorkspaceLayoutViewModel::new(
        app_ctx.clone(),
        crate::models::WorkspaceLayoutService::in_memory_default(),
        teksilo::widgets::DockingModel::new(),
        single_work.clone(),
        single_work_info.clone(),
        ids.clone(),
        backup_mode.clone(),
        crate::settings::TreeExpansionViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            crate::models::TreeExpansionService::in_memory_default(),
        ),
        crate::shared::ItemViewStates::new(),
    );
    BackupSchedulerViewModel::new(
        app_ctx,
        ids,
        workspace_layout,
        settings,
        single_work,
        single_work_info,
        backup_mode,
    )
}

// ── Phase 3: per-Work toast identity ──────────────────────────────────────
//
// Two Works can now legitimately back up at the same time (each has its
// own `BackupSchedulerViewModel`, built inside `WorkSession::new`), but
// `teksilo`'s `ToastRegistry` is process-wide (see the module doc's "toast
// routing" section) — a fixed toast id would let Work B's progress/result
// toast silently overwrite Work A's still-in-flight one (`Toast::id`
// reuses the same slot for a repeated id). Folding `self.ids.work_id`
// through [`BACKUP_TOAST_ID`] + `work_scoped_toast_id` closes that.

#[test]
fn two_works_never_share_a_toast_id() {
    // Every production call site scopes `BACKUP_TOAST_ID` against
    // `self.ids.work_id` (F4: the same source every `.target_work(...)`
    // call site right next to it reads) — set that, not `single_work`,
    // which is only a downstream projection of it.
    let a = test_scheduler();
    a.ids.work_id.set(Some(1));
    let b = test_scheduler();
    b.ids.work_id.set(Some(2));

    assert_ne!(
        crate::toast_scope::work_scoped_toast_id(BACKUP_TOAST_ID, a.ids.work_id.get()),
        crate::toast_scope::work_scoped_toast_id(BACKUP_TOAST_ID, b.ids.work_id.get()),
        "two different Works' backup toasts must never collide in the shared registry"
    );
}

#[test]
fn the_same_work_always_gets_the_same_toast_id() {
    // The "update in place" behaviour `Toast::id` exists for (a progress
    // tick replacing the previous percentage) still needs a *stable* id
    // across calls for one Work.
    let a = test_scheduler();
    a.ids.work_id.set(Some(7));
    assert_eq!(
        crate::toast_scope::work_scoped_toast_id(BACKUP_TOAST_ID, a.ids.work_id.get()),
        crate::toast_scope::work_scoped_toast_id(BACKUP_TOAST_ID, a.ids.work_id.get()),
    );
}

/// F4 regression: before the fix, every production toast-id call site read
/// `self.single_work.id()` while `backup_now`'s own `.target_work(...)`
/// read `self.ids.work_id.get()` right next to it — two different
/// "current Work" signals for one toast. This pins them to agree even when
/// `single_work` is stale/unset and only `ids.work_id` is current, proving
/// the toast id is derived from `ids.work_id` alone.
#[test]
fn toast_id_tracks_ids_work_id_even_when_single_work_disagrees() {
    let a = test_scheduler();
    a.ids.work_id.set(Some(1));
    // Deliberately point `single_work` at a DIFFERENT Work (99) — not
    // `None`, which under `--features mocks` isn't even reachable
    // (`SingleWork`'s mock fabricates `Some(1)` by construction). If the
    // toast id was ever again built from `single_work.id()`, it would
    // read "99" here while `target_work` still routed to Work 1.
    a.single_work.set_id(Some(99));
    assert_eq!(
        crate::toast_scope::work_scoped_toast_id(BACKUP_TOAST_ID, a.ids.work_id.get()),
        crate::toast_scope::work_scoped_toast_id(BACKUP_TOAST_ID, Some(1)),
        "the toast id must be derived from ids.work_id, the same source \
             every .target_work(...) call site in this file uses — never \
             single_work.id(), which this test deliberately set to a \
             different Work"
    );
}

/// F1 regression: before the fix, `BackupNowDto.work_id` (the BACKEND
/// operation's own Work identity) was built from `self.single_work.id()`
/// while `Pending::tracked`'s `work_id()`, captured two lines later in [`Self::start`],
/// read `self.ids` via `CapturedWork::now` — two different "current
/// Work" sources for one operation. This pins them to agree even when
/// `single_work` is stale/unset and only `ids.work_id` is current,
/// proving the DTO's `work_id` is derived from `ids.work_id` alone —
/// the same source `Pending::tracked`'s `work_id()` captures right next to it, so the
/// two can never again diverge (a backup dispatched tagged with one Work
/// while its completion toast reports another).
#[test]
fn backup_now_dto_work_id_tracks_ids_work_id_even_when_single_work_disagrees() {
    let a = test_scheduler();
    a.ids.work_id.set(Some(1));
    // Deliberately point `single_work` at a DIFFERENT Work (99) — not
    // `None`, which under `--features mocks` isn't even reachable
    // (`SingleWork`'s mock fabricates `Some(1)` by construction). If the
    // DTO's `work_id` was ever again built from `single_work.id()`, it
    // would read "99" here while the captured `Pending::tracked`'s `work_id()` right
    // next to it still pinned Work 1.
    a.single_work.set_id(Some(99));
    assert_eq!(
        a.backup_now_dto_work_id(),
        1,
        "BackupNowDto.work_id must be derived from ids.work_id, the same \
             source Pending::tracked's work_id() captures via CapturedWork::now right next \
             to it in Self::start — never single_work.id(), which this test \
             deliberately set to a different Work"
    );
    assert_ne!(
        a.backup_now_dto_work_id(),
        a.single_work.id().unwrap_or_default(),
        "the DTO's work_id must now differ from single_work.id() — proving \
             a future regression back to single_work.id() would be caught here"
    );
}

// ── the safety copy taken before a destructive write ──────────────────────
//
// `backup_now` returns early — with nothing but a toast — on three separate
// conditions. A caller that fires a destructive edit after merely *calling*
// it has no safety net on any of those three paths, and would never know.
// `backup_before` exists to make that impossible; these pin the three.

/// A scheduler whose `(unique_id, path)` resolve — i.e. one with a project.
fn opened(uid: &str) -> BackupSchedulerViewModel {
    let s = test_scheduler();
    s.ids.work_id.set(Some(1));
    s.single_work.unique_id().set(uid.to_string());
    s.single_work_info
        .file_name()
        .set(Some("/tmp/novel.skrib".to_string()));
    s
}

#[test]
fn a_backup_already_running_blocks_the_safety_copy_rather_than_queueing_behind_it() {
    let s = opened("ne6qxlag");
    assert_eq!(
        s.safety_backup_blocker(),
        None,
        "a plain open project must be able to take a safety copy",
    );

    s.pending.set(Some(Pending {
        tracked: TrackedOp::start(&s.ids, "in-flight".to_string()),
        uid: "uid".to_string(),
        path: "/tmp/novel.skrib".to_string(),
        dirs: vec![String::new()],
        close: None,
        then: None,
    }));
    assert_eq!(
        s.safety_backup_blocker(),
        Some(SafetyBlocker::AlreadyRunning),
        "the destructive edit must not proceed while the copy is still being made",
    );
}

#[test]
fn a_window_showing_a_backup_file_cannot_take_a_safety_copy() {
    let s = opened("ne6qxlag");
    s.backup_mode.set(true);
    assert_eq!(
        s.safety_backup_blocker(),
        Some(SafetyBlocker::BackupFileOpen),
        "this window never backs itself up, so there is no net here to rely on",
    );
}

/// Every blocker has to be *nameable*, not merely truthy: "a backup is
/// already running, try again in a moment" and "this is a backup file" are
/// different things to tell a writer who just asked to overwrite their text.
#[test]
fn each_reason_a_safety_copy_cannot_run_is_distinguishable() {
    let mut seen = std::collections::BTreeSet::new();
    let running = opened("ne6qxlag");
    running.pending.set(Some(Pending {
        tracked: TrackedOp::start(&running.ids, "op".to_string()),
        uid: "uid".to_string(),
        path: "/tmp/novel.skrib".to_string(),
        dirs: vec![String::new()],
        close: None,
        then: None,
    }));
    seen.insert(format!("{:?}", running.safety_backup_blocker().unwrap()));

    let backup_file = opened("ne6qxlag");
    backup_file.backup_mode.set(true);
    seen.insert(format!(
        "{:?}",
        backup_file.safety_backup_blocker().unwrap()
    ));

    // …and the third. Emptied explicitly rather than left at the default:
    // under `--features mocks` both singles fabricate a project by
    // construction, so "a fresh scheduler" is not "nothing open" there.
    let nothing_open = test_scheduler();
    nothing_open.single_work.unique_id().set(String::new());
    nothing_open.single_work_info.file_name().set(None);
    seen.insert(format!(
        "{:?}",
        nothing_open
            .safety_backup_blocker()
            .expect("with no project there is nothing to copy"),
    ));

    assert_eq!(
        seen.len(),
        3,
        "obstacles collapsed into one reason: {seen:?}"
    );
}

// ── F4: the Work a backup started for must survive a later in-place switch ─
//
// `BackupSchedulerViewModel` is per-Work, but its `ids`/`single_work` are the
// window's own long-lived handles (see `Pending::tracked`'s `work_id()`'s doc) — this
// scheduler outlives any one backup. Before this fix, every `on_long_op_*`
// handler re-read `self.ids.work_id.get()` live, so a
// window that started a backup, then switched to a different Work before it
// finished, would route the completion toast to the NEW Work — the Work
// that actually ran the backup never hearing about it. This pins the fix:
// `Pending::tracked`'s `work_id()`, captured once in `Self::start`, must stay pinned even
// after the window's live `ids.work_id` moves on.

#[test]
fn a_backups_captured_work_id_survives_a_later_in_place_switch() {
    let scheduler = test_scheduler();
    scheduler.ids.work_id.set(Some(1));

    // Simulate what `Self::start` does the instant the long operation
    // begins: snapshot the window's current Work into `Pending`.
    scheduler.pending.set(Some(Pending {
        tracked: TrackedOp::start(&scheduler.ids, "fake-backup-op".to_string()),
        uid: "uid".to_string(),
        path: "/tmp/novel.skrib".to_string(),
        dirs: vec![String::new()],
        close: None,
        then: None,
    }));
    let captured = scheduler.pending.get().unwrap().tracked.work_id();
    assert_eq!(captured, Some(1));

    // An in-place project switch reseeds `ids.work_id` on the SAME `AppIds`
    // this long-lived scheduler holds (`ProjectSwitchViewModel::request`
    // gates only on unsaved edits, never on a backup in flight).
    scheduler.ids.work_id.set(Some(2));

    assert_eq!(
        scheduler.pending.get().unwrap().tracked.work_id(),
        captured,
        "the in-flight backup's own Work must stay pinned to what `start` \
             captured, even after this window switches to a different Work"
    );
    assert_ne!(
        scheduler.pending.get().unwrap().tracked.work_id(),
        scheduler.ids.work_id.get(),
        "the captured Work must now differ from the window's live \
             `ids.work_id` — proving a handler reading `pending.tracked.work_id()` cannot \
             silently be reading the same live value `ids.work_id` would give it"
    );
}

#[test]
fn backup_toast_ids_for_two_captured_works_never_collide() {
    // The other half of the fix (F1/F2's root cause): even with the right
    // Work captured, a bare "backup.now" id shared by every window would let
    // a second Work's backup find this one's still-live toast entry
    // (`ToastRegistry::enqueue` dedups on id alone) and silently
    // retarget/steal it.
    let id_a = crate::toast_scope::work_scoped_toast_id(BACKUP_TOAST_ID, Some(1));
    let id_b = crate::toast_scope::work_scoped_toast_id(BACKUP_TOAST_ID, Some(2));
    assert_ne!(
        id_a, id_b,
        "two different Works' backup toasts must never collide"
    );
}

/// The test above only proves `work_scoped_toast_id` itself is collision-free
/// — it never touches `backup_now`'s actual `.scoped_id(BACKUP_TOAST_ID, ...)`
/// call site, so reverting that call site back to a bare `"backup.now"` would still
/// leave it green. This one drives the real, PUBLIC `backup_now(ctx)` entry
/// point through a real `ToastRegistry`: two schedulers captured for two
/// different Works each raise a toast through a wired `Button` + a
/// dispatched click (a real `EventContext`), then asserts both stay live —
/// `ToastRegistry::enqueue`'s update-in-place merge would collapse them to
/// ONE entry if the id were ever bare again.
///
/// Each scheduler is pre-marked "busy" (`pending` already `Some`) so
/// `backup_now` takes its ctx-only "already running" branch — the one
/// outcome reachable with no real backend Work loaded EITHER under the
/// default `SingleWork`/`SingleWorkInfo` (unset, so `current()` is `None`
/// and `backup_now` would otherwise return via "nothing open" before ever
/// reaching the busy check) OR under `--features mocks` (pre-seeded with a
/// fabricated project, so `current()` is always `Some` and `backup_now`
/// would otherwise fall through into `start(...)`'s real long-operation
/// path). Both toast branches build `.scoped_id(BACKUP_TOAST_ID, self.ids.work_id.get())`
/// identically, so either exercises the exact fix this test is pinning.
#[test]
fn backup_now_toasts_for_two_works_both_stay_live_in_a_real_registry() {
    use teksilo::i18n::lit;
    use teksilo::widgets::{Button, ToastInstallOptions, ToastRegistry};

    let a = test_scheduler();
    a.single_work.set_id(Some(1));
    a.ids.work_id.set(Some(1));
    a.pending.set(Some(Pending {
        tracked: TrackedOp::start(&a.ids, "fake-backup-op-a".to_string()),
        uid: "uid-a".to_string(),
        path: "/tmp/a.skrib".to_string(),
        dirs: vec![String::new()],
        close: None,
        then: None,
    }));
    let b = test_scheduler();
    b.single_work.set_id(Some(2));
    b.ids.work_id.set(Some(2));
    b.pending.set(Some(Pending {
        tracked: TrackedOp::start(&b.ids, "fake-backup-op-b".to_string()),
        uid: "uid-b".to_string(),
        path: "/tmp/b.skrib".to_string(),
        dirs: vec![String::new()],
        close: None,
        then: None,
    }));

    let registry = ToastRegistry::new(ToastInstallOptions {
        archive: None,
        ..ToastInstallOptions::default()
    });
    let mut tree = crate::test_support::tree_with_toast_registry(&a.app_ctx, &registry);

    let sa = a.clone();
    let sb = b.clone();
    let btn_a = tree.add(Button::new(lit!("a")).on_activate_fn(move |ctx| sa.backup_now(ctx)));
    let btn_b = tree.add(Button::new(lit!("b")).on_activate_fn(move |ctx| sb.backup_now(ctx)));
    tree.layout(SizeProposal::exact(200.0, 80.0));

    crate::test_support::click(&mut tree, btn_a);
    crate::test_support::click(&mut tree, btn_b);

    assert_eq!(
        registry.live_count(),
        2,
        "two different Works' backup_now toasts must both stay live — a \
             bare \"backup.now\" id would let Work B's enqueue find Work A's still-live \
             entry (ToastRegistry::enqueue dedups on id alone) and merge into it, \
             leaving only 1"
    );
}

/// F4 regression, driven through the real, public `backup_now` entry
/// point: two schedulers with **different** `ids.work_id`, but neither
/// one's `single_work` ever pointed to match it — exactly the state a
/// fresh `WorkSession` is in for the instant between
/// `ProjectLifecycleViewModel::seed`'s two lines (`ids.seed(...)`, then
/// `single_work.set_id(...)` one line later), or under `--features
/// mocks`, where `SingleWork`'s mock fabricates its own `Some(1)`
/// regardless of what `ids.work_id` is set to. Before the fix, the toast
/// id was built from `self.single_work.id()`: both schedulers would
/// answer the SAME thing there despite genuinely different
/// `ids.work_id`s, so both toasts would collapse onto the same
/// collidable id and `ToastRegistry::enqueue` would merge them into one
/// live entry. This pins that both stay live now that the id is built
/// from `ids.work_id`.
#[test]
fn backup_now_toasts_stay_live_even_when_single_work_does_not_track_ids() {
    use teksilo::i18n::lit;
    use teksilo::widgets::{Button, ToastInstallOptions, ToastRegistry};

    let a = test_scheduler();
    a.ids.work_id.set(Some(1));
    // `single_work` deliberately left at whatever `test_scheduler()`
    // gave it — never pointed to agree with `ids.work_id` — on both `a`
    // and `b`. See the doc above.
    a.pending.set(Some(Pending {
        tracked: TrackedOp::start(&a.ids, "fake-backup-op-a".to_string()),
        uid: "uid-a".to_string(),
        path: "/tmp/a.skrib".to_string(),
        dirs: vec![String::new()],
        close: None,
        then: None,
    }));
    let b = test_scheduler();
    b.ids.work_id.set(Some(2));
    b.pending.set(Some(Pending {
        tracked: TrackedOp::start(&b.ids, "fake-backup-op-b".to_string()),
        uid: "uid-b".to_string(),
        path: "/tmp/b.skrib".to_string(),
        dirs: vec![String::new()],
        close: None,
        then: None,
    }));

    let registry = ToastRegistry::new(ToastInstallOptions {
        archive: None,
        ..ToastInstallOptions::default()
    });
    let mut tree = crate::test_support::tree_with_toast_registry(&a.app_ctx, &registry);

    let sa = a.clone();
    let sb = b.clone();
    let btn_a = tree.add(Button::new(lit!("a")).on_activate_fn(move |ctx| sa.backup_now(ctx)));
    let btn_b = tree.add(Button::new(lit!("b")).on_activate_fn(move |ctx| sb.backup_now(ctx)));
    tree.layout(SizeProposal::exact(200.0, 80.0));

    crate::test_support::click(&mut tree, btn_a);
    crate::test_support::click(&mut tree, btn_b);

    assert_eq!(
        registry.live_count(),
        2,
        "F4: two Works with different ids.work_id but the same unset \
             single_work must both keep their own backup_now toast — a toast \
             id built from single_work.id() would collapse both onto \
             \"backup.now.0\" and merge them into one live entry"
    );
}

// ── T1-2: the flush invariant ────────────────────────────────────────────
//
// `backup_now` and `on_close_flow` also call `self.flush()` as their
// unconditional first statement (see the module doc). A real `EventContext`
// test harness now exists (`crate::test_support`, used by the real-registry
// toast test just above) — but re-deriving `backup_now`'s whole "nothing
// open" branch here would only re-prove the ordering these two ctx-free
// triggers already pin more directly: the exact same private `flush()`
// helper those two call first.

#[test]
fn on_open_flushes_before_anything_else() {
    let scheduler = test_scheduler();
    let flushed = Rc::new(Cell::new(0u32));
    {
        let flushed = flushed.clone();
        scheduler.register_flush_hook(
            TeksiloWindowId::new(1),
            Rc::new(move || flushed.set(flushed.get() + 1)),
        );
    }
    // No project open — `on_open` returns right after the flush, via
    // `current()` returning `None`. The flush must still have happened.
    scheduler.on_open();
    assert_eq!(flushed.get(), 1);
}

#[test]
fn interval_tick_flushes_before_anything_else() {
    let scheduler = test_scheduler();
    let flushed = Rc::new(Cell::new(0u32));
    {
        let flushed = flushed.clone();
        scheduler.register_flush_hook(
            TeksiloWindowId::new(1),
            Rc::new(move || flushed.set(flushed.get() + 1)),
        );
    }
    scheduler.interval_tick();
    assert_eq!(flushed.get(), 1);
}

#[test]
fn flush_hook_defaults_to_a_harmless_no_op() {
    // Constructing the scheduler without installing a hook (the headless-test
    // shape) must not panic — every trigger's `self.flush()` call is safe.
    let scheduler = test_scheduler();
    scheduler.on_open();
    scheduler.interval_tick();
}

#[test]
fn flush_hook_is_visible_on_every_existing_clone() {
    // The hook map is shared (`Rc<RefCell<..>>`), so registering on ONE
    // clone (as `App::build` does) must be visible on clones made earlier —
    // e.g. the project window's close guard clone in `windows.rs`.
    let scheduler = test_scheduler();
    let earlier_clone = scheduler.clone();
    let flushed = Rc::new(Cell::new(0u32));
    {
        let flushed = flushed.clone();
        scheduler.register_flush_hook(
            TeksiloWindowId::new(1),
            Rc::new(move || flushed.set(flushed.get() + 1)),
        );
    }
    earlier_clone.on_open();
    assert_eq!(flushed.get(), 1, "the earlier clone must see the new hook");
}

#[test]
fn every_registered_window_is_flushed_not_just_the_last() {
    // The whole point of the multi-Work migration's fix: a Work with two
    // windows must flush BOTH before a backup, not silently drop the first
    // window's unsaved edits the moment a second window registers.
    let scheduler = test_scheduler();
    let flushed_a = Rc::new(Cell::new(0u32));
    let flushed_b = Rc::new(Cell::new(0u32));
    {
        let flushed_a = flushed_a.clone();
        scheduler.register_flush_hook(
            TeksiloWindowId::new(1),
            Rc::new(move || flushed_a.set(flushed_a.get() + 1)),
        );
    }
    {
        let flushed_b = flushed_b.clone();
        scheduler.register_flush_hook(
            TeksiloWindowId::new(2),
            Rc::new(move || flushed_b.set(flushed_b.get() + 1)),
        );
    }
    scheduler.on_open();
    assert_eq!(
        flushed_a.get(),
        1,
        "window 1's editors must still be flushed"
    );
    assert_eq!(
        flushed_b.get(),
        1,
        "window 2's editors must also be flushed"
    );
}

/// `unregister_flush_hook` — the on_removed-driven inverse of
/// `register_flush_hook` — must drop exactly the closed window's hook and
/// leave a sibling window's alone.
#[test]
fn unregister_flush_hook_drops_only_that_window_and_leaves_a_sibling_alone() {
    let scheduler = test_scheduler();
    let flushed_a = Rc::new(Cell::new(0u32));
    let flushed_b = Rc::new(Cell::new(0u32));
    {
        let flushed_a = flushed_a.clone();
        scheduler.register_flush_hook(
            TeksiloWindowId::new(1),
            Rc::new(move || flushed_a.set(flushed_a.get() + 1)),
        );
    }
    {
        let flushed_b = flushed_b.clone();
        scheduler.register_flush_hook(
            TeksiloWindowId::new(2),
            Rc::new(move || flushed_b.set(flushed_b.get() + 1)),
        );
    }

    scheduler.unregister_flush_hook(TeksiloWindowId::new(1));
    scheduler.on_open();

    assert_eq!(
        flushed_a.get(),
        0,
        "window 1's hook must no longer run once unregistered"
    );
    assert_eq!(flushed_b.get(), 1, "window 2's hook must still run");
}

#[test]
fn unregister_flush_hook_for_an_unknown_window_is_a_safe_no_op() {
    let scheduler = test_scheduler();
    scheduler.unregister_flush_hook(TeksiloWindowId::new(404));
    // Must not panic, and must not disturb an unrelated trigger.
    scheduler.on_open();
}

// ── pure helpers ─────────────────────────────────────────────────────────

/// An unconfigured policy resolves to the app's own backup root, **not** to the
/// project's folder. The mapping itself is unit-tested purely (with an injected
/// root) in `crate::backup_paths`; this asserts the scheduler actually goes
/// through it rather than keeping a second copy of the rule.
#[test]
fn dirs_defaults_to_the_app_backup_root_when_empty() {
    let empty = BackupPolicy {
        destinations: vec![],
        ..BackupPolicy::default()
    };
    let resolved = BackupSchedulerViewModel::dirs(&empty);
    assert_eq!(
        resolved,
        crate::backup_paths::effective_destinations(&[]),
        "the scheduler must delegate to backup_paths, not re-implement the default",
    );
    // On any platform with a data directory this is a real path; where there is
    // none, `backup_paths` deliberately falls back to beside-the-project rather
    // than producing no destination at all.
    assert_eq!(resolved.len(), 1);
}

#[test]
fn dirs_uses_a_configured_list_verbatim() {
    let two = BackupPolicy {
        destinations: vec!["/a".to_string(), "/b".to_string()],
        ..BackupPolicy::default()
    };
    assert_eq!(
        BackupSchedulerViewModel::dirs(&two),
        vec!["/a".to_string(), "/b".to_string()]
    );
}

#[test]
fn build_last_known_bypasses_lookup_when_forced_or_skip_disabled() {
    let dirs = vec!["/a".to_string(), "/b".to_string()];

    let (h, p) = BackupSchedulerViewModel::build_last_known(&dirs, true, true, |_| {
        ("H".to_string(), "P".to_string())
    });
    assert_eq!(
        h,
        vec!["".to_string(), "".to_string()],
        "force bypasses lookup"
    );
    assert_eq!(p, vec!["".to_string(), "".to_string()]);

    let (h, _p) = BackupSchedulerViewModel::build_last_known(&dirs, false, false, |_| {
        ("H".to_string(), "P".to_string())
    });
    assert_eq!(
        h,
        vec!["".to_string(), "".to_string()],
        "skip_if_unchanged=false also bypasses lookup"
    );

    let (h, p) = BackupSchedulerViewModel::build_last_known(&dirs, false, true, |d| {
        (format!("hash-{d}"), format!("path-{d}"))
    });
    assert_eq!(h, vec!["hash-/a".to_string(), "hash-/b".to_string()]);
    assert_eq!(p, vec!["path-/a".to_string(), "path-/b".to_string()]);
}

#[test]
fn classify_result_flags_produced_nothing_only_when_every_destination_failed() {
    let dirs = vec!["/a".to_string(), "/b".to_string()];

    let all_failed = BackupResultDto {
        failed_directories: dirs.clone(),
        ..BackupResultDto::default()
    };
    let c = classify_result(&dirs, &all_failed);
    assert!(c.produced_nothing);
    assert!(c.succeeded_dirs.is_empty());

    let one_written = BackupResultDto {
        succeeded_paths: vec!["/a/x.skrib".to_string()],
        ..BackupResultDto::default()
    };
    let c = classify_result(&dirs, &one_written);
    assert!(!c.produced_nothing);
    // Neither destination is in `skipped`/`failed`, so both count as
    // "succeeded" by this classification (matches `succeeded_paths`' length
    // in the real engine, which only ever pushes for a destination it
    // didn't skip/fail).
    assert_eq!(c.succeeded_dirs, dirs);

    let all_current = BackupResultDto {
        skipped_directories: dirs.clone(),
        ..BackupResultDto::default()
    };
    let c = classify_result(&dirs, &all_current);
    assert!(
        !c.produced_nothing,
        "already-current everywhere is not \"produced nothing\""
    );
    assert_eq!(c.ok, 0);
    assert_eq!(c.skipped, 2);
}

#[test]
fn failure_detail_joins_reasons_and_delete_errors_and_is_none_when_empty() {
    assert_eq!(failure_detail(&[], &[]), None);
    assert_eq!(
        failure_detail(&["disk full".to_string()], &[]),
        Some("disk full".to_string())
    );
    assert_eq!(
        failure_detail(&["disk full".to_string()], &["perm denied".to_string()]),
        Some("disk full · perm denied".to_string())
    );
}

#[test]
fn progress_label_maps_every_machine_key() {
    assert_eq!(
        progress_label("backup.start"),
        String::from(tr!(backup_progress_start()))
    );
    assert_eq!(
        progress_label("backup.retention"),
        String::from(tr!(backup_progress_retention()))
    );
    assert_eq!(
        progress_label("backup.done"),
        String::from(tr!(backup_progress_done()))
    );
    assert_eq!(
        progress_label("backup.destination:2:5"),
        String::from(tr!(backup_progress_destination(i = 2, n = 5)))
    );
    assert_eq!(progress_label("backup.destination:garbage"), "");
    assert_eq!(progress_label("something-unknown"), "");
    assert_eq!(progress_label(""), "");
}

#[test]
fn to_engine_retention_mode_round_trips_both_variants() {
    assert_eq!(
        to_engine_retention_mode(RetentionMode::Tiered),
        EngineRetentionMode::Tiered
    );
    assert_eq!(
        to_engine_retention_mode(RetentionMode::KeepLastN),
        EngineRetentionMode::KeepLastN
    );
}
