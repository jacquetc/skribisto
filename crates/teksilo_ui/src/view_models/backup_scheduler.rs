// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `BackupSchedulerViewModel` — drives every automatic + manual backup trigger.
//!
//! One place that starts a `backup_now` long operation (manual / on-open /
//! interval / on-close), records the per-destination success hash + path on
//! completion, and surfaces a summary toast. Retention runs **inside** the long
//! operation itself (`work_management::backup_now_uc`); this view-model never
//! touches the filesystem for pruning. It holds the reactive singles (to read
//! the open project's `unique_id`/path without a context) and the backup
//! settings, and is registered as `app_state` so `App::build` can route
//! long-operation events to it.
//!
//! **The flush invariant.** Editor text lives in a UI widget's buffer until
//! `EditorsViewModel::flush_all()` copies it into the store; a backup only ever
//! sees the store. Every trigger below therefore calls [`BackupSchedulerViewModel::flush`] first,
//! unconditionally — a typing session in an unfocused tab must never be
//! invisible to skip-if-unchanged. The hook is installed once (`App::build`)
//! via [`BackupSchedulerViewModel::register_flush_hook`] and shared through every clone of this
//! view-model via an `Rc<RefCell<..>>` cell; it defaults to a no-op so headless
//! tests can construct the scheduler without an `EditorsViewModel`.
//!
//! **On close**, the close guards defer to
//! [`BackupSchedulerViewModel::on_close_flow`]: if a destination is reachable
//! it runs the backup before the real (forced) close; if none is, it shows a
//! blocking Retry/Discard-and-exit prompt. The reachability probe (`fs::metadata`
//! per destination) runs off the UI thread, so a hung network destination
//! can't block quitting.
//!
//! **Skip-if-unchanged** needs both the last-known hash and the exact path it
//! was written to — both live in `DestinationState`
//! (`BackupSettingsService::destination_state`), persisted rather than cached
//! only in this process's memory, so a restart doesn't force one redundant
//! write per destination. Safe across windows sharing `backup.toml` (the
//! settings service opens it in cross-process shared mode).
//!
//! **Per-Work, not shared.** Constructed inside `sessions::WorkSession::new`
//! using that Work's own `single_work`/`single_work_info`/`ids`, so two Works
//! can back up concurrently. Every toast is dedup-scoped to this Work's
//! `work_id` ([`BACKUP_TOAST_ID`] + `crate::toast_scope::ToastWorkExt::scoped_id`,
//! never a fixed string) and routed with `.target_work(self.ids.work_id.get())`,
//! which resolves to teksilo's window-scoped toast routing — so a Work B
//! backup toast renders only in windows currently showing Work B.
//!
//! **Capture, don't re-read live.** `ids`/`single_work` are the *window's* own
//! long-lived handles (this scheduler outlives any one backup), and an
//! in-place project switch can reseed those same signals mid-flight.
//! [`Pending::tracked`] is a [`super::long_op::TrackedOp`], bundling the op id
//! with a [`super::long_op::CapturedWork`] captured once in [`BackupSchedulerViewModel::start`] —
//! every `on_long_op_*` handler below routes and scopes its toast on
//! `tracked.work_id()`, never `self.ids.work_id.get()`.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use teksilo::prelude::*;
use teksilo::widgets::{MessageBox, MessageBoxButtons, StandardButton, Toast, ToastAction};

use frontend::AppContext;
use frontend::common::event::Event;
use frontend::work_management::{
    BackupNowDto, BackupResultDto, RetentionMode as EngineRetentionMode,
};

use crate::app::PendingExit;
use crate::app_ids::AppIds;
use crate::backup::is_destination_available;
use crate::models::{BackupPolicy, RetentionMode, uid_is_usable};
use crate::singles::{SingleWork, SingleWorkInfo};
use crate::toast_scope::ToastWorkExt;
use crate::view_models::{BackupSettingsViewModel, WorkspaceLayoutViewModel};

use super::long_op::{TrackedOp, event_id, parse_payload, payload_id};

/// Per-window flush hooks, keyed by window: a Work with two windows must flush
/// both editors before a backup. Aliased so the field type stays legible.
type FlushHooks = Rc<RefCell<HashMap<TeksiloWindowId, Rc<dyn Fn()>>>>;

/// Toast id base for every backup toast this view-model shows (progress,
/// success, partial, failure) — folded through
/// [`crate::toast_scope::ToastWorkExt::scoped_id`] with `self.ids.work_id`/
/// `pending.tracked.work_id()` at every use, never bare: two windows backing up two
/// different Works at once must not collide in the shared `ToastRegistry`
/// (see the module doc's "toast routing" section).
const BACKUP_TOAST_ID: &str = "backup.now";

/// Context of the in-flight backup, so its completion can record hashes and
/// (on close) perform the deferred close.
#[derive(Clone)]
struct Pending {
    /// The long-operation id bundled with the Work it was captured for (F4)
    /// — see `long_op::TrackedOp`'s doc for why every toast this struct's own
    /// operation raises after it started must route on `tracked.work_id()`,
    /// never a live `self.ids.work_id.get()`.
    tracked: TrackedOp,
    uid: String,
    path: String,
    /// The directories passed to the engine (a single `""` means "next to project").
    dirs: Vec<String>,
    /// Set when this backup must be followed by a window/work close.
    close: Option<PendingExit>,
    /// Set when this backup is a **safety copy taken before a destructive
    /// write**, and the write must happen only if a copy actually exists.
    ///
    /// Called exactly once on every terminal path — completed, failed, or failed
    /// to start — with `true` only when at least one destination holds a copy.
    /// A caller that fired the destructive edit itself and merely *called*
    /// `backup_now` first would have no safety net at all: that function returns
    /// early, with nothing but a toast, on three separate conditions.
    then: Option<SafetyOutcome>,
}

/// Told whether a safety backup produced a copy. See [`Pending::then`].
pub type SafetyOutcome = Rc<dyn Fn(&mut EventContext, bool)>;

/// Why a safety copy cannot be taken right now.
///
/// Named rather than collapsed into one failure, because the three read very
/// differently to a writer: one is temporary and worth retrying in a moment, one
/// is a state they chose, and one means there is nothing to copy at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyBlocker {
    /// A backup file is open here. This window never backs itself up.
    BackupFileOpen,
    /// No project is open, so there is nothing to copy.
    NoProject,
    /// A backup is already in flight.
    AlreadyRunning,
}

#[derive(Clone)]
pub struct BackupSchedulerViewModel {
    app_ctx: Rc<AppContext>,
    /// This session's own id-only state — needed so [`Self::do_close`] can name
    /// the *right* Work when it calls `close_work_and_return_to_launcher`
    /// (multi-Work migration: it used to resolve "which Work" via
    /// `ctx.app_state::<AppIds>()`, a single process-wide slot that can only
    /// ever answer with whichever window's `AppIds` the builder happened to
    /// register — wrong the instant a second Work's window exists).
    ids: AppIds,
    /// This session's own `WorkspaceLayoutViewModel` — needed for the same reason as
    /// `ids`: [`Self::do_close`] hands it straight to
    /// `close_work_and_return_to_launcher` so it captures *this* window's desk,
    /// not whichever window's instance last won `ctx.app_state`'s single slot.
    workspace_layout: WorkspaceLayoutViewModel,
    settings: BackupSettingsViewModel,
    single_work: SingleWork,
    single_work_info: SingleWorkInfo,
    /// `true` while a backup file is open here — every automatic/manual trigger
    /// no-ops (a backup window never backs itself up).
    backup_mode: Signal<bool>,
    pending: Signal<Option<Pending>>,
    /// Bumped every time a backup actually happens (written or already-current).
    /// The App's interval timer watches this and restarts its countdown, so
    /// "every N hours" means N hours since the *last backup*, whatever triggered
    /// it — not N hours since the timer last armed.
    completed_epoch: Signal<u64>,
    /// See the module doc's "the flush invariant" section. A Work with two
    /// windows must flush *both* before a backup, so this is keyed per window.
    flush_hooks: FlushHooks,
    /// The app-global quit sequencer, if one has been injected
    /// ([`Self::set_quit_sequencer`]). `None` in tests and in the throwaway
    /// bootstrap session `main` builds before any project window exists.
    quit: Rc<RefCell<Option<crate::view_models::QuitSequencer>>>,
}

impl BackupSchedulerViewModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        workspace_layout: WorkspaceLayoutViewModel,
        settings: BackupSettingsViewModel,
        single_work: SingleWork,
        single_work_info: SingleWorkInfo,
        backup_mode: Signal<bool>,
    ) -> Self {
        Self {
            app_ctx,
            ids,
            workspace_layout,
            settings,
            single_work,
            single_work_info,
            backup_mode,
            pending: Signal::new(None),
            completed_epoch: Signal::new(0),
            flush_hooks: Rc::new(RefCell::new(HashMap::new())),
            quit: Rc::new(RefCell::new(None)),
        }
    }

    /// This session's own `WorkspaceLayoutViewModel` — see the field's doc.
    /// Exposed so `guard_unsaved_exit`'s immediate-discard branch (which calls
    /// `close_work_and_return_to_launcher` directly, bypassing
    /// [`Self::do_close`]) can pass the *right* one too.
    pub(crate) fn workspace_layout(&self) -> &WorkspaceLayoutViewModel {
        &self.workspace_layout
    }

    /// Register (or replace) this `window_id`'s flush hook — wired in
    /// `App::build`, as `editors.flush_all()`, once per window. Every clone of
    /// this scheduler shares the same map, so registering on any one of them is
    /// visible on all the others already handed out — the same sharing
    /// `set_flush_hook` used to rely on, now keyed so a second window's editors
    /// don't silently displace the first's.
    ///
    /// Unregistered by [`Self::unregister_flush_hook`], called from the
    /// window's own `on_removed`-driven teardown (see
    /// `sessions::WorkRegistry::remove_window`) — teksilo's window-teardown
    /// hook that closed the framework gap this doc used to describe (a
    /// per-widget destroy pass on window close, so there is now a reliable
    /// "this window just closed" moment to call it from). Deliberately NOT
    /// unregistered on an in-place Work switch (`register_window`'s replace
    /// path): this hook is keyed on the window, not the Work it happens to be
    /// showing, and the same window keeps needing its buffers flushed for
    /// whatever Work it shows next.
    pub fn register_flush_hook(&self, window_id: TeksiloWindowId, hook: Rc<dyn Fn()>) {
        self.flush_hooks.borrow_mut().insert(window_id, hook);
    }

    /// Drop `window_id`'s flush hook — the inverse of
    /// [`Self::register_flush_hook`], called once teksilo's `on_removed` hook
    /// confirms that window is gone. Without this, every closed window's hook
    /// stayed in the map for the rest of the process, each future backup
    /// trigger calling `editors.flush_all()` on a torn-down `EditorsViewModel`
    /// forever — harmless (the call is inert, not unsound) but a real,
    /// unbounded leak over a long session that opens and closes many windows.
    /// A safe no-op for a `window_id` with no hook registered.
    pub fn unregister_flush_hook(&self, window_id: TeksiloWindowId) {
        self.flush_hooks.borrow_mut().remove(&window_id);
    }

    /// Flush every registered window's live editor buffers into the store.
    /// Called unconditionally as the first statement of every trigger — see
    /// the module doc.
    fn flush(&self) {
        let hooks: Vec<Rc<dyn Fn()>> = self.flush_hooks.borrow().values().cloned().collect();
        for hook in hooks {
            hook();
        }
    }

    /// [`Self::flush`], for a caller outside this type.
    ///
    /// The hook registry is per-Work and keyed by window, which makes it the only
    /// thing in the crate that can answer "put *this Work's* live editor buffers
    /// into the store" without holding one particular window's
    /// `EditorsViewModel`. [`crate::view_models::QuitSequencer`] needs exactly
    /// that: it saves Works it is not the window for, and `request_save` records
    /// an edit sequence that only means anything once the store holds everything
    /// up to it.
    pub fn flush_all_windows(&self) {
        self.flush();
    }

    /// Counter of completed backups — the App's interval timer re-arms whenever
    /// this changes (see [`Self::completed_epoch`]).
    pub fn completed_epoch(&self) -> u64 {
        self.completed_epoch.get()
    }

    /// The same counter, as something a peer can *react* to.
    ///
    /// [`Self::completed_epoch`] returns a snapshot, which is all the interval
    /// timer needs. The version surfaces need the other half: a backup that
    /// lands mid-session is a new version of every row in the project, and a
    /// pane that cached its answer before it must be told to look again. Handed
    /// out as a signal so `project_shell` can bump the project's revision from an
    /// effect rather than polling.
    pub fn completed_epoch_signal(&self) -> Signal<u64> {
        self.completed_epoch.clone()
    }

    /// The open project's `(unique_id, path)`, or `None` when nothing is open /
    /// the project has no id or path yet.
    fn current(&self) -> Option<(String, String)> {
        let uid = self.single_work.unique_id().get();
        let path = self.single_work_info.file_name().get()?;
        if !uid_is_usable(&uid) || path.trim().is_empty() {
            return None;
        }
        Some((uid, path))
    }

    /// True while a backup is running for **this Work** (one at a time per
    /// Work — this scheduler is itself per-Work, see the module doc, so a
    /// second, simultaneously-open Work's own backup never counts as "busy"
    /// here).
    fn busy(&self) -> bool {
        self.pending.get().is_some()
    }

    /// A backup window never backs itself up — every trigger no-ops.
    fn suppressed(&self) -> bool {
        self.backup_mode.get()
    }

    // ── triggers ────────────────────────────────────────────────────────────

    /// Manual "Back up now": always writes (bypasses skip-if-unchanged).
    ///
    /// Every toast below is scoped with [`BACKUP_TOAST_ID`] +
    /// [`crate::toast_scope::ToastWorkExt::scoped_id`], reading `self.ids.work_id`
    /// — the SAME source `.target_work(...)` reads right next to it.
    ///
    /// **One source, not two.** `self.ids.work_id` is the authoritative answer
    /// to "which Work" throughout this file — it's what every captured
    /// `Pending::tracked`'s `work_id()` derives from
    /// (`CapturedWork::now(&self.ids)`) and what every post-capture toast
    /// routes on — so every toast here, including the dedup id, reads that one
    /// signal, never `self.single_work.id()` (a *different* "current Work"
    /// signal that only agrees with `ids.work_id` because
    /// `ProjectLifecycleViewModel::seed` happens to set both, in order, with
    /// nothing enforcing it).
    ///
    /// **No id at all when no Work is open.** `scoped_id` skips the
    /// `.id(...)` call entirely when `work_id` is `None` (see
    /// [`crate::toast_scope::work_scoped_toast_id`]'s doc): two windows with
    /// no Work open yet (both have this globally-registered action live
    /// before their own `LoadWork`/`NewWork` resolves a `work_id`) must not
    /// share one collidable toast id.
    pub fn backup_now(&self, ctx: &mut EventContext) {
        self.flush();
        if self.suppressed() {
            return;
        }
        let Some((uid, path)) = self.current() else {
            ctx.show_toast(
                Toast::warning(tr!(backup_nothing_open()))
                    .scoped_id(BACKUP_TOAST_ID, self.ids.work_id.get())
                    .target_work(self.ids.work_id.get()),
            );
            return;
        };
        if self.busy() {
            ctx.show_toast(
                Toast::info(tr!(backup_already_running()))
                    .scoped_id(BACKUP_TOAST_ID, self.ids.work_id.get())
                    .target_work(self.ids.work_id.get()),
            );
            return;
        }
        let policy = self.settings.effective_for(&uid);
        self.start(Some(ctx), &uid, &path, &policy, true, None);
    }

    /// Take a safety copy, then hand the verdict to `then`.
    ///
    /// The contract that matters is that `then` is **always** called, exactly
    /// once, and with `false` whenever no copy was made — including the three
    /// early returns [`Self::backup_now`] takes silently (a backup file is open,
    /// no project, a backup already running) and a long operation that fails to
    /// start at all. A destructive edit fired from anywhere but the `true` branch
    /// is an edit with no safety net, which is exactly the shape this exists to
    /// make impossible.
    ///
    /// Forces a write like the manual trigger does: a deliberate safety copy is
    /// worth one write even when the content matches the last backup.
    pub fn backup_before(&self, ctx: &mut EventContext, then: SafetyOutcome) {
        self.flush();
        if self.safety_backup_blocker().is_some() {
            return then(ctx, false);
        }
        let Some((uid, path)) = self.current() else {
            return then(ctx, false);
        };
        let policy = self.settings.effective_for(&uid);
        self.start_with(Some(ctx), &uid, &path, &policy, true, None, Some(then));
    }

    /// Why [`Self::backup_before`] would refuse right now, or `None` if it would
    /// go ahead.
    ///
    /// Split out from the call so a caller can say *which* obstacle it hit
    /// before doing anything destructive — and so the three early returns
    /// `backup_now` takes in silence are testable without an event loop.
    pub fn safety_backup_blocker(&self) -> Option<SafetyBlocker> {
        if self.suppressed() {
            return Some(SafetyBlocker::BackupFileOpen);
        }
        if self.current().is_none() {
            return Some(SafetyBlocker::NoProject);
        }
        if self.busy() {
            return Some(SafetyBlocker::AlreadyRunning);
        }
        None
    }

    /// On project open: a quiet, fire-and-forget backup when the policy asks for it.
    pub fn on_open(&self) {
        self.flush();
        if self.suppressed() {
            return;
        }
        let Some((uid, path)) = self.current() else {
            return;
        };
        if self.busy() {
            return;
        }
        let policy = self.settings.effective_for(&uid);
        if !policy.on_open {
            return;
        }
        self.start(None, &uid, &path, &policy, false, None);
    }

    /// Interval tick: back up if enough time has elapsed (the App timer decides
    /// *when*; this just runs one, honouring skip-if-unchanged).
    pub fn interval_tick(&self) {
        self.flush();
        if self.suppressed() {
            return;
        }
        let Some((uid, path)) = self.current() else {
            return;
        };
        if self.busy() {
            return;
        }
        let policy = self.settings.effective_for(&uid);
        if !policy.interval_enabled {
            return;
        }
        self.start(None, &uid, &path, &policy, false, None);
    }

    /// The interval between automatic backups for the open project, in seconds —
    /// `None` when nothing is open or the interval trigger is off. Drives the App
    /// timer that calls [`Self::interval_tick`].
    pub fn interval_secs(&self) -> Option<u64> {
        if self.suppressed() {
            return None;
        }
        let (uid, _) = self.current()?;
        let policy = self.settings.effective_for(&uid);
        if policy.interval_enabled {
            Some(policy.interval_hours.max(1) as u64 * 3600)
        } else {
            None
        }
    }

    /// The on-close orchestration: back up (if a destination is reachable), then
    /// perform `then`. Called from the close guards / the SaveWork-completion
    /// handler once the project is in a saved-consistent state.
    ///
    /// The reachability probe runs off the UI thread: `is_destination_available`
    /// is an untimed `fs::metadata` per destination, and this runs during app
    /// exit — a hung destination must not be able to block quitting.
    pub fn on_close_flow(&self, ctx: &mut EventContext, then: PendingExit) {
        self.flush();
        // A backup window never backs up on close — just perform the close.
        if self.suppressed() {
            return self.do_close(ctx, then);
        }
        let Some((uid, path)) = self.current() else {
            return self.do_close(ctx, then);
        };
        let policy = self.settings.effective_for(&uid);
        if !policy.on_close {
            return self.do_close(ctx, then);
        }
        // A backup already running: attach the close to it instead of starting a
        // second one; its completion will close.
        if let Some(mut p) = self.pending.get() {
            p.close = Some(then);
            self.pending.set(Some(p));
            return;
        }

        let dirs = Self::dirs(&policy);
        let me = self.clone();
        ctx.spawn_local_with(
            async move {
                spawn_blocking(move || dirs.iter().any(|d| is_destination_available(d)))
                    .await
                    .unwrap_or(false)
            },
            move |available, ctx2| {
                if !available {
                    // No reachable destination — the one case that blocks the exit.
                    return me.prompt_no_destination(ctx2, then);
                }
                me.start(Some(ctx2), &uid, &path, &policy, false, Some(then));
            },
        )
        .detach();
    }

    fn prompt_no_destination(&self, ctx: &mut EventContext, then: PendingExit) {
        let me = self.clone();
        MessageBox::warning(tr!(backup_no_destination_title()))
            .text(tr!(backup_no_destination_text()))
            .buttons(MessageBoxButtons::Custom(vec![
                StandardButton::Retry.into(),
                StandardButton::Discard.into(),
            ]))
            .default_button(StandardButton::Retry)
            .escape_button(StandardButton::Discard)
            .on_result(move |r, c| match r.button {
                // Retry: re-check availability (a drive may now be plugged in).
                StandardButton::Retry => me.on_close_flow(c, then),
                // Discard and exit: close without a backup.
                _ => me.do_close(c, then),
            })
            .present(ctx);
    }

    // ── engine kickoff ────────────────────────────────────────────────────────

    /// The Work id the BACKEND `BackupNowDto` itself is tagged with.
    ///
    /// **One source, not two.** `self.ids.work_id` is the authoritative
    /// "current Work" signal throughout this file, so this reads that one
    /// signal, never `self.single_work.id()` — a *different* signal that
    /// [`Self::start`]'s `Pending::tracked`'s `work_id()` (which captures
    /// `self.ids` via `CapturedWork::now` for the exact same operation) only
    /// agrees with because `ProjectLifecycleViewModel::seed` happens to set
    /// both, in order, with nothing enforcing it — reading the wrong one here
    /// could dispatch a backup tagged with one Work while its own completion
    /// toast reports a different one. Split out so a test can pin it without
    /// driving the real long-operation command.
    fn backup_now_dto_work_id(&self) -> u64 {
        self.ids.work_id.get().unwrap_or_default()
    }

    /// Build the DTO and start the long op. `ctx` is present only when a UI
    /// surface (toast / close) should react; on-open / interval pass `None`.
    fn start(
        &self,
        ctx: Option<&mut EventContext>,
        uid: &str,
        path: &str,
        policy: &BackupPolicy,
        force: bool,
        close: Option<PendingExit>,
    ) {
        self.start_with(ctx, uid, path, policy, force, close, None);
    }

    /// [`Self::start`] plus the safety-copy completion hook.
    #[allow(clippy::too_many_arguments)]
    fn start_with(
        &self,
        ctx: Option<&mut EventContext>,
        uid: &str,
        path: &str,
        policy: &BackupPolicy,
        force: bool,
        close: Option<PendingExit>,
        then: Option<SafetyOutcome>,
    ) {
        let dirs = Self::dirs(policy);
        let uid_owned = uid.to_string();
        let settings = self.settings.clone();
        // Both the hash and the exact written path come from the same
        // persisted `DestinationState`, so this survives a restart.
        let (hashes, paths) = Self::build_last_known(&dirs, force, policy.skip_if_unchanged, |d| {
            match settings.service().destination_state(&uid_owned, d) {
                Some(s) => (s.last_success_hash, s.last_success_path),
                None => (String::new(), String::new()),
            }
        });

        let dto = BackupNowDto {
            media_root: crate::media_paths::media_root_string(),
            work_id: self.backup_now_dto_work_id(),
            directories: dirs.clone(),
            last_known_hashes: hashes,
            last_known_paths: paths,
            // Retention runs inside the operation, off the UI thread: every
            // backup run also sweeps its destinations.
            prune: true,
            retention_mode: to_engine_retention_mode(policy.retention_mode),
            keep_last_n: policy.keep_last_n as u64,
            gfs_hourly: policy.gfs_hourly as u64,
            gfs_daily: policy.gfs_daily as u64,
            gfs_weekly: policy.gfs_weekly as u64,
            gfs_monthly: policy.gfs_monthly as u64,
            min_keep: policy.min_keep as u64,
            // Read fresh at dispatch, and pruned of files that no longer exist:
            // a pin is a promise about a file, and a file the writer deleted by
            // hand is not one this can keep.
            pinned_paths: {
                let _ = settings.service().forget_missing_pins(&uid_owned);
                settings.service().pinned(&uid_owned)
            },
        };
        match frontend::commands::work_management_commands::backup_now(&self.app_ctx, &dto) {
            Ok(op_id) => {
                self.pending.set(Some(Pending {
                    // Captured NOW, bundled with the op id — see
                    // `long_op::TrackedOp`'s doc.
                    tracked: TrackedOp::start(&self.ids, op_id),
                    uid: uid.to_string(),
                    path: path.to_string(),
                    dirs,
                    close,
                    then,
                }));
            }
            Err(e) => {
                if let Some(ctx) = ctx {
                    ctx.show_toast(
                        Toast::error(tr!(backup_error(error = e.to_string())))
                            .scoped_id(BACKUP_TOAST_ID, self.ids.work_id.get())
                            .target_work(self.ids.work_id.get()),
                    );
                    // A failed *start* must never trap a pending close…
                    if let Some(exit) = close {
                        self.do_close(ctx, exit);
                    }
                    // …nor leave a caller waiting for a verdict that will never
                    // come. No operation was dispatched, so no copy exists.
                    if let Some(hook) = then {
                        hook(ctx, false);
                    }
                }
            }
        }
    }

    /// Effective destination list.
    ///
    /// An unconfigured policy resolves to the app's own backup root rather than to
    /// the project's folder — see [`crate::backup_paths`] for why, and for the rule
    /// that an explicit `""` entry still means "next to the project".
    fn dirs(policy: &BackupPolicy) -> Vec<String> {
        crate::backup_paths::effective_destinations(&policy.destinations)
    }

    /// Pure: the per-destination `(hash, path)` pair the engine's skip-if-
    /// unchanged check consumes. `force` (manual "Back up now") and
    /// `skip_if_unchanged = false` both bypass `lookup` entirely, forcing a
    /// fresh write everywhere. Split out from [`Self::start`] so it's
    /// unit-testable without a running app.
    fn build_last_known(
        dirs: &[String],
        force: bool,
        skip_if_unchanged: bool,
        lookup: impl Fn(&str) -> (String, String),
    ) -> (Vec<String>, Vec<String>) {
        dirs.iter()
            .map(|d| {
                if force || !skip_if_unchanged {
                    (String::new(), String::new())
                } else {
                    lookup(d)
                }
            })
            .unzip()
    }

    /// Perform the deferred close: either return to the Launcher (see
    /// `crate::app::close_work_and_return_to_launcher`) or hand control back to
    /// the quit sequencer ([`Self::set_quit_sequencer`]) — the two outcomes a
    /// guarded close can end in.
    fn do_close(&self, ctx: &mut EventContext, then: PendingExit) {
        match then {
            PendingExit::ReturnToLauncher => crate::app::close_work_and_return_to_launcher(
                &self.app_ctx,
                &self.ids,
                &self.workspace_layout,
                ctx,
            ),
            // A quit spans every open Work, so "close" here cannot mean "close my
            // own window" — that would end the process with other projects still
            // unaccounted for. It means "this Work's on-close backup is done;
            // carry on with the quit", and the sequencer owns what that is.
            PendingExit::Quit => {
                if let Some(quit) = self.quit.borrow().as_ref() {
                    quit.on_backup_done(ctx);
                }
            }
            PendingExit::None => {}
        }
    }

    /// Hand this Work's scheduler the app-global quit sequencer, so its on-close
    /// backup can hand control back when it finishes.
    ///
    /// Injected after construction rather than taken by `WorkSession::new`
    /// because the dependency runs the other way round for everything else here:
    /// the sequencer reaches *into* per-Work schedulers to flush and save, and
    /// making the constructor require it would put the app-global handle in the
    /// signature of every per-Work test fixture in the crate.
    pub fn set_quit_sequencer(&self, quit: crate::view_models::QuitSequencer) {
        *self.quit.borrow_mut() = Some(quit);
    }

    // ── long-operation event handlers (wired in App::build) ───────────────────

    /// A progress tick: update the "backing up…" toast in place. The engine
    /// reports stable machine keys (it has no i18n layer — see
    /// `backup_now_uc.rs`), mapped through `tr!()` here rather than shown raw.
    pub fn on_long_op_progress(&self, ctx: &mut EventContext, event: &Event) {
        let Some(pending) = self.pending.get() else {
            return;
        };
        let Some(payload) = parse_payload(event) else {
            return;
        };
        if payload_id(&payload) != Some(pending.tracked.op_id()) {
            return;
        }
        let percent = payload
            .get("percentage")
            .and_then(|p| p.as_f64())
            .unwrap_or(0.0) as f32;
        let message = payload
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("");
        ctx.show_toast(
            progress_toast(pending.tracked.work_id(), percent, message)
                .target_work(pending.tracked.work_id()),
        );
    }

    pub fn on_long_op_completed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(pending) = self.pending.get() else {
            return;
        };
        if event_id(event).as_deref() != Some(pending.tracked.op_id()) {
            return;
        }
        self.pending.set(None);

        let result = frontend::commands::work_management_commands::get_backup_now_result(
            &self.app_ctx,
            pending.tracked.op_id(),
        )
        .ok()
        .flatten();

        // Set when the backup produced *nothing*: every destination failed to
        // write. On close this is the last chance to keep a copy, so it blocks.
        let mut produced_nothing = false;
        // Set when at least one destination now holds a copy — either freshly
        // written, or already current and therefore still a copy. This is the
        // verdict a safety backup's caller acts on.
        let mut copy_exists = false;

        if let Some(res) = result {
            let now = chrono::Utc::now().to_rfc3339();
            let c = classify_result(&pending.dirs, &res);

            // Record the hash + the exact written path (see
            // `DestinationState::last_success_path`) for every destination this
            // run actually wrote to, in lockstep with `res.succeeded_paths`
            // (built by the engine in the same directory order, skipping
            // skipped/failed).
            for (d, p) in c.succeeded_dirs.iter().zip(res.succeeded_paths.iter()) {
                self.settings.record_destination_success(
                    &pending.uid,
                    &pending.path,
                    d,
                    &res.content_hash,
                    p,
                    &now,
                );
            }

            produced_nothing = c.produced_nothing;
            copy_exists = c.ok > 0 || c.skipped > 0;
            let (ok, skipped, failed) = (c.ok, c.skipped, c.failed);
            let has_delete_errors = !res.delete_errors.is_empty();

            // A backup really happened (written, or already current) → restart the
            // interval countdown so it measures time since the *last* backup.
            if ok > 0 || skipped > 0 {
                let e = &self.completed_epoch;
                e.set(e.get().wrapping_add(1));
                // …and what the app's backup root holds has just moved: a run
                // writes a bundle and prunes old ones. Settings ▸ Backup measures
                // that root once and caches it, so without this the "N backups,
                // M MB, oldest …" line would keep quoting a number from before
                // this run for the rest of the session.
                self.settings.invalidate_root_usage();
            }

            // Thread the engine's failure reasons + retention delete errors into
            // the toast as a details line (never the headline — that stays a
            // translated sentence; the raw OS/anyhow strings are data, shown
            // behind a "Details" action).
            let detail = failure_detail(&res.failed_reasons, &res.delete_errors);

            // Quiet on a pure no-op close (nothing written, nothing failed); noisy
            // enough to reassure on a manual/successful write, and warn on failure
            // (incl. a write success whose retention prune failed). When
            // *everything* failed on close we prompt below instead of toasting.
            if produced_nothing {
                if pending.close.is_none() {
                    show_result_toast(
                        ctx,
                        Toast::error(tr!(backup_partial(ok = ok, failed = failed)))
                            .scoped_id(BACKUP_TOAST_ID, pending.tracked.work_id())
                            .auto_dismiss_after(Duration::from_secs(6))
                            .target_work(pending.tracked.work_id()),
                        detail,
                    );
                }
            } else if failed > 0 {
                show_result_toast(
                    ctx,
                    Toast::warning(tr!(backup_partial(ok = ok, failed = failed)))
                        .scoped_id(BACKUP_TOAST_ID, pending.tracked.work_id())
                        .auto_dismiss_after(Duration::from_secs(6))
                        .target_work(pending.tracked.work_id()),
                    detail,
                );
            } else if has_delete_errors {
                // Every destination wrote fine — only the post-write prune
                // failed somewhere. Still a warning: silently swallowing this
                // would leave the user believing everything is clean.
                show_result_toast(
                    ctx,
                    Toast::warning(tr!(backup_complete_prune_warning(
                        ok = ok,
                        skipped = skipped
                    )))
                    .scoped_id(BACKUP_TOAST_ID, pending.tracked.work_id())
                    .auto_dismiss_after(Duration::from_secs(6))
                    .target_work(pending.tracked.work_id()),
                    detail,
                );
            } else if pending.close.is_none() && (ok > 0 || skipped > 0) {
                ctx.show_toast(
                    Toast::success(tr!(backup_complete(ok = ok, skipped = skipped)))
                        .scoped_id(BACKUP_TOAST_ID, pending.tracked.work_id())
                        .auto_dismiss_after(Duration::from_secs(4))
                        .target_work(pending.tracked.work_id()),
                );
            }
        }

        // Before the close branch below, which can return early: a safety
        // backup's caller is waiting on this and must hear the verdict on every
        // path out of this function.
        if let Some(hook) = pending.then {
            hook(ctx, copy_exists);
        }

        if let Some(then) = pending.close {
            // Every destination failed at write time (drive yanked mid-write, disk
            // full, permissions). The availability pre-check couldn't catch this, so
            // this is the backstop: don't silently quit without a backup — let the
            // user fix it and retry, or knowingly discard.
            if produced_nothing {
                return self.prompt_backup_failed(ctx, then);
            }
            self.do_close(ctx, then);
        }
    }

    pub fn on_long_op_failed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(pending) = self.pending.get() else {
            return;
        };
        let Some(payload) = parse_payload(event) else {
            return;
        };
        if payload_id(&payload) != Some(pending.tracked.op_id()) {
            return;
        }
        self.pending.set(None);
        let error = payload
            .get("error")
            .and_then(|e| e.as_str())
            .unwrap_or_default()
            .to_string();
        // The operation failed outright, so nothing was written anywhere.
        if let Some(hook) = pending.then {
            hook(ctx, false);
        }
        // On close, a failed backup means no copy was made — same backstop as
        // "every destination failed": prompt rather than quit silently.
        if let Some(then) = pending.close {
            return self.prompt_backup_failed(ctx, then);
        }
        // Headline stays a translated sentence; the raw error string is a
        // detail behind a "Details" action, not the headline itself.
        show_result_toast(
            ctx,
            Toast::error(tr!(backup_failed_title()))
                .scoped_id(BACKUP_TOAST_ID, pending.tracked.work_id())
                .target_work(pending.tracked.work_id()),
            Some(error),
        );
    }

    /// The backup ran but nothing was written (or it failed outright) and a close
    /// is pending. Offer to retry — the retry re-scans the destinations, so a
    /// freshly-plugged drive is picked up — or to quit without a backup.
    fn prompt_backup_failed(&self, ctx: &mut EventContext, then: PendingExit) {
        let me = self.clone();
        MessageBox::warning(tr!(backup_failed_close_title()))
            .text(tr!(backup_failed_close_text()))
            .buttons(MessageBoxButtons::Custom(vec![
                StandardButton::Retry.into(),
                StandardButton::Discard.into(),
            ]))
            .default_button(StandardButton::Retry)
            .escape_button(StandardButton::Discard)
            .on_result(move |r, c| match r.button {
                // Re-run the whole on-close flow (re-checks reachability, re-writes).
                StandardButton::Retry => me.on_close_flow(c, then),
                // Discard and exit: quit without a backup.
                _ => me.do_close(c, then),
            })
            .present(ctx);
    }
}

/// Adapt the app's persisted [`RetentionMode`] to the engine DTO's own copy of
/// the same unit enum (kept separate so `work_management`'s generated `dtos.rs`
/// never depends on the UI's settings-file type).
fn to_engine_retention_mode(mode: RetentionMode) -> EngineRetentionMode {
    match mode {
        RetentionMode::Tiered => EngineRetentionMode::Tiered,
        RetentionMode::KeepLastN => EngineRetentionMode::KeepLastN,
    }
}

/// Map a backup long-operation's stable machine progress key (the engine has
/// no i18n layer) to a localized phrase. Unrecognized/empty keys map to an
/// empty string rather than falling back to raw text — never show unlocalized
/// engine internals.
fn progress_label(message: &str) -> String {
    if message == "backup.start" {
        String::from(tr!(backup_progress_start()))
    } else if message == "backup.retention" {
        String::from(tr!(backup_progress_retention()))
    } else if message == "backup.done" {
        String::from(tr!(backup_progress_done()))
    } else if let Some(rest) = message.strip_prefix("backup.destination:") {
        let mut parts = rest.splitn(2, ':');
        match (
            parts.next().and_then(|s| s.parse::<u32>().ok()),
            parts.next().and_then(|s| s.parse::<u32>().ok()),
        ) {
            (Some(i), Some(n)) => String::from(tr!(backup_progress_destination(i = i, n = n))),
            _ => String::new(),
        }
    } else {
        String::new()
    }
}

/// The loading toast shown while a backup runs: title + `NN% · phase` body,
/// re-shown (same id) on every progress tick so the one surface updates in
/// place. `work_id` is the backup's own captured `Pending::tracked`'s `work_id()` — never a
/// fixed string — folded into [`BACKUP_TOAST_ID`] via
/// [`crate::toast_scope::ToastWorkExt::scoped_id`], so a second Work's own
/// progress toast can never land in this one's slot.
fn progress_toast(work_id: impl Into<Option<u64>>, percent: f32, message: &str) -> Toast {
    let label = progress_label(message);
    let body = if label.is_empty() {
        format!("{percent:.0}%")
    } else {
        format!("{percent:.0}% · {label}")
    };
    Toast::loading(tr!(backing_up()))
        .scoped_id(BACKUP_TOAST_ID, work_id)
        .body(lit!(body))
}

/// Join the engine's per-destination failure reasons and retention delete
/// errors into one details string, or `None` if there's nothing to show.
fn failure_detail(reasons: &[String], delete_errors: &[String]) -> Option<String> {
    let mut parts: Vec<String> = Vec::with_capacity(reasons.len() + delete_errors.len());
    parts.extend(reasons.iter().cloned());
    parts.extend(delete_errors.iter().cloned());
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" · "))
    }
}

/// Show `toast`, attaching a **Details** action (opening a plain message box
/// with the raw content) when `detail` is `Some` — the raw OS/anyhow strings
/// stay a detail, never the headline.
fn show_result_toast(ctx: &mut EventContext, toast: Toast, detail: Option<String>) {
    let toast = match detail {
        Some(d) => toast.action(ToastAction::primary(tr!(backup_details()), move |c| {
            MessageBox::warning(tr!(backup_issues_title()))
                .text(lit!(d.clone()))
                .buttons(MessageBoxButtons::Ok)
                .present(c);
        })),
        None => toast,
    };
    ctx.show_toast(toast);
}

/// Classification of a completed backup result against the destinations this
/// run targeted: which were actually written vs skipped vs failed, and
/// whether the whole run produced nothing at all (every destination failed —
/// the on-close backstop cares about exactly this). Pure — takes only the
/// plain lists a [`BackupResultDto`] carries, so it's unit-testable without a
/// running app.
struct ResultClassification {
    succeeded_dirs: Vec<String>,
    ok: usize,
    skipped: usize,
    failed: usize,
    produced_nothing: bool,
}

fn classify_result(dirs: &[String], res: &BackupResultDto) -> ResultClassification {
    let succeeded_dirs: Vec<String> = dirs
        .iter()
        .filter(|d| !res.skipped_directories.contains(*d) && !res.failed_directories.contains(*d))
        .cloned()
        .collect();
    let ok = res.succeeded_paths.len();
    let skipped = res.skipped_directories.len();
    let failed = res.failed_directories.len();
    ResultClassification {
        succeeded_dirs,
        ok,
        skipped,
        failed,
        produced_nothing: ok == 0 && skipped == 0 && failed > 0,
    }
}

#[cfg(test)]
mod tests {
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
            crate::view_models::TreeExpansionViewModel::new(
                app_ctx.clone(),
                ids.clone(),
                crate::models::TreeExpansionService::in_memory_default(),
            ),
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
}
