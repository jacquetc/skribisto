// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `BackupSchedulerViewModel` — drives every automatic + manual backup trigger.
//!
//! One place that starts a `backup_now` long operation (manual / on-open /
//! interval / on-close), records the per-destination success hash + path on
//! completion, and surfaces a summary toast. Retention now runs **inside** the
//! long operation (the engine resolves the same directories the UI used to
//! sweep — see `work_management::backup_now_uc`), so this view-model no longer
//! touches the filesystem for pruning at all. It holds the reactive singles (to
//! read the open project's `unique_id`/path without a context) and the backup
//! settings, and is registered as `app_state` so `App::build` can route the
//! long-operation events to it and fire its triggers.
//!
//! **T1-2 — the flush invariant.** Editor text lives in a UI widget's buffer
//! until `EditorsViewModel::flush_all()` copies it into the store; a backup
//! only ever sees the store. Every trigger below therefore calls
//! [`Self::flush`] as its very first action, unconditionally — a two-hour
//! typing session in an unfocused tab must never be invisible to skip-if-
//! unchanged. The hook is installed once (`App::build`, as
//! `editors.flush_all()`) via [`Self::set_flush_hook`] and shared through every
//! clone of this view-model (the project window's close guard in `windows.rs`,
//! `App`'s own exit-guard effect, …) via an `Rc<RefCell<..>>` cell, so installing it on one
//! clone is visible everywhere at once. It defaults to a no-op so headless
//! tests can construct the scheduler without an `EditorsViewModel`.
//!
//! **On close** it also orchestrates the "back up before quitting" step: the
//! close guards defer to [`BackupSchedulerViewModel::on_close_flow`], which — if a destination is
//! reachable — runs the backup and only then performs the real (forced) close;
//! if *no* destination is reachable it shows a blocking **Retry / Discard-and-
//! exit** prompt so the user can plug a drive in and retry. The reachability
//! probe itself (T2-3: an untimed `fs::metadata` per destination) runs off the
//! UI thread — a hung network destination must not be able to block the user
//! from quitting the app at all.
//!
//! **T1-6.** The skip-if-unchanged "does the previous backup still exist"
//! check (T1-3) needs both the last-known hash *and* the exact path it was
//! written to. Both now live in [`crate::models::DestinationState`]
//! (`BackupSettingsService::destination_state`) rather than the hash being
//! persisted while the path was only cached in this process's memory — a
//! restart used to forget the path and force one redundant write per
//! destination before the (in-process-only) cache repopulated. Persisting it
//! is safe across the two windows that may share `backup.toml` (one process
//! per project) because the settings service opens the file in cross-process
//! shared mode (see `models::backup_settings_file`'s module docs).

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use bastyde::prelude::*;
use bastyde::widgets::{MessageBox, MessageBoxButtons, StandardButton, Toast, ToastAction};

use frontend::AppContext;
use frontend::common::event::Event;
use frontend::work_management::{
    BackupNowDto, BackupResultDto, RetentionMode as EngineRetentionMode,
};

use crate::app::PendingExit;
use crate::backup::is_destination_available;
use crate::models::{BackupPolicy, RetentionMode, uid_is_usable};
use crate::singles::{SingleWork, SingleWorkInfo};
use crate::view_models::BackupSettingsViewModel;

use super::long_op::{event_id, parse_payload, payload_id};

const BACKUP_TOAST_ID: &str = "backup.now";

/// Context of the in-flight backup, so its completion can record hashes and
/// (on close) perform the deferred close.
#[derive(Clone)]
struct Pending {
    op_id: String,
    uid: String,
    path: String,
    /// The directories passed to the engine (a single `""` means "next to project").
    dirs: Vec<String>,
    /// Set when this backup must be followed by a window/work close.
    close: Option<PendingExit>,
}

#[derive(Clone)]
pub struct BackupSchedulerViewModel {
    app_ctx: Rc<AppContext>,
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
    /// See the module doc's "T1-2 — the flush invariant" section.
    flush_hook: Rc<RefCell<Rc<dyn Fn()>>>,
}

impl BackupSchedulerViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        settings: BackupSettingsViewModel,
        single_work: SingleWork,
        single_work_info: SingleWorkInfo,
        backup_mode: Signal<bool>,
    ) -> Self {
        Self {
            app_ctx,
            settings,
            single_work,
            single_work_info,
            backup_mode,
            pending: Signal::new(None),
            completed_epoch: Signal::new(0),
            flush_hook: Rc::new(RefCell::new(Rc::new(|| {}) as Rc<dyn Fn()>)),
        }
    }

    /// Install the real flush hook (wired once in `App::build`, as
    /// `editors.flush_all()`). Every clone of this scheduler shares the same
    /// cell, so installing it on any one of them is visible on all the others
    /// already handed out.
    pub fn set_flush_hook(&self, hook: Rc<dyn Fn()>) {
        *self.flush_hook.borrow_mut() = hook;
    }

    /// T1-2: flush live editor buffers into the store. Called unconditionally
    /// as the first statement of every trigger — see the module doc.
    fn flush(&self) {
        let hook = self.flush_hook.borrow().clone();
        hook();
    }

    /// Counter of completed backups — the App's interval timer re-arms whenever
    /// this changes (see [`Self::completed_epoch`]).
    pub fn completed_epoch(&self) -> u64 {
        self.completed_epoch.get()
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

    /// True while a backup is running (one at a time per process).
    fn busy(&self) -> bool {
        self.pending.get().is_some()
    }

    /// A backup window never backs itself up — every trigger no-ops.
    fn suppressed(&self) -> bool {
        self.backup_mode.get()
    }

    // ── triggers ────────────────────────────────────────────────────────────

    /// Manual "Back up now": always writes (bypasses skip-if-unchanged).
    pub fn backup_now(&self, ctx: &mut EventContext) {
        self.flush();
        if self.suppressed() {
            return;
        }
        let Some((uid, path)) = self.current() else {
            ctx.show_toast(Toast::warning(tr!(backup_nothing_open())).id(BACKUP_TOAST_ID));
            return;
        };
        if self.busy() {
            ctx.show_toast(Toast::info(tr!(backup_already_running())).id(BACKUP_TOAST_ID));
            return;
        }
        let policy = self.settings.effective_for(&uid);
        self.start(Some(ctx), &uid, &path, &policy, true, None);
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
    /// The reachability probe (T2-3) runs off the UI thread: `is_destination_available`
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
        let dirs = Self::dirs(policy);
        let uid_owned = uid.to_string();
        let settings = self.settings.clone();
        // T1-6: both the hash and the exact written path now come from the
        // same persisted `DestinationState`, so this survives a restart —
        // no more in-process-only path cache.
        let (hashes, paths) = Self::build_last_known(&dirs, force, policy.skip_if_unchanged, |d| {
            match settings.service().destination_state(&uid_owned, d) {
                Some(s) => (s.last_success_hash, s.last_success_path),
                None => (String::new(), String::new()),
            }
        });

        let dto = BackupNowDto {
            work_id: self.single_work.id().unwrap_or_default(),
            directories: dirs.clone(),
            last_known_hashes: hashes,
            last_known_paths: paths,
            // Retention now runs inside the operation, off the UI thread (T1-7):
            // every backup run also sweeps its destinations, exactly like the UI
            // used to after every write/skip.
            prune: true,
            retention_mode: to_engine_retention_mode(policy.retention_mode),
            keep_last_n: policy.keep_last_n as u64,
            gfs_hourly: policy.gfs_hourly as u64,
            gfs_daily: policy.gfs_daily as u64,
            gfs_weekly: policy.gfs_weekly as u64,
            gfs_monthly: policy.gfs_monthly as u64,
            min_keep: policy.min_keep as u64,
        };
        match frontend::commands::work_management_commands::backup_now(&self.app_ctx, &dto) {
            Ok(op_id) => {
                self.pending.set(Some(Pending {
                    op_id,
                    uid: uid.to_string(),
                    path: path.to_string(),
                    dirs,
                    close,
                }));
            }
            Err(e) => {
                if let Some(ctx) = ctx {
                    ctx.show_toast(
                        Toast::error(tr!(backup_error(error = e.to_string()))).id(BACKUP_TOAST_ID),
                    );
                    // A failed *start* must never trap a pending close.
                    if let Some(then) = close {
                        self.do_close(ctx, then);
                    }
                }
            }
        }
    }

    /// Effective destination list (a single empty string ⇒ "next to the project").
    fn dirs(policy: &BackupPolicy) -> Vec<String> {
        if policy.destinations.is_empty() {
            vec![String::new()]
        } else {
            policy.destinations.clone()
        }
    }

    /// Pure: the per-destination `(hash, path)` pair the engine's skip-if-
    /// unchanged check consumes. `force` (manual "Back up now") and
    /// `skip_if_unchanged = false` both bypass `lookup` entirely, forcing a
    /// fresh write everywhere. Split out from [`Self::start`] so it's
    /// unit-testable without a running app (T2-10).
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
    /// `crate::app::close_work_and_return_to_launcher`) or really terminate
    /// the process (see `crate::app::quit_app`) — the two outcomes a guarded
    /// close can end in.
    fn do_close(&self, ctx: &mut EventContext, then: PendingExit) {
        match then {
            PendingExit::ReturnToLauncher => {
                crate::app::close_work_and_return_to_launcher(&self.app_ctx, ctx)
            }
            PendingExit::Quit => crate::app::quit_app(&self.app_ctx, ctx),
            PendingExit::None => {}
        }
    }

    // ── long-operation event handlers (wired in App::build) ───────────────────

    /// A progress tick: update the "backing up…" toast in place. The engine
    /// reports stable machine keys (it has no i18n layer — see
    /// `backup_now_uc.rs`), mapped through `tr!()` here (T2-9) rather than shown
    /// raw.
    pub fn on_long_op_progress(&self, ctx: &mut EventContext, event: &Event) {
        let Some(pending) = self.pending.get() else {
            return;
        };
        let Some(payload) = parse_payload(event) else {
            return;
        };
        if payload_id(&payload) != Some(pending.op_id.as_str()) {
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
        ctx.show_toast(progress_toast(percent, message));
    }

    pub fn on_long_op_completed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(pending) = self.pending.get() else {
            return;
        };
        if event_id(event).as_deref() != Some(pending.op_id.as_str()) {
            return;
        }
        self.pending.set(None);

        let result = frontend::commands::work_management_commands::get_backup_now_result(
            &self.app_ctx,
            &pending.op_id,
        )
        .ok()
        .flatten();

        // Set when the backup produced *nothing*: every destination failed to
        // write. On close this is the last chance to keep a copy, so it blocks.
        let mut produced_nothing = false;

        if let Some(res) = result {
            let now = chrono::Utc::now().to_rfc3339();
            let c = classify_result(&pending.dirs, &res);

            // Record the hash + the exact written path (T1-6 — both persisted
            // now, see `DestinationState::last_success_path`) for every
            // destination this run actually wrote to, in lockstep with
            // `res.succeeded_paths` (built by the engine in the same directory
            // order, skipping skipped/failed).
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
            let (ok, skipped, failed) = (c.ok, c.skipped, c.failed);
            let has_delete_errors = !res.delete_errors.is_empty();

            // A backup really happened (written, or already current) → restart the
            // interval countdown so it measures time since the *last* backup.
            if ok > 0 || skipped > 0 {
                let e = &self.completed_epoch;
                e.set(e.get().wrapping_add(1));
            }

            // T2-8: thread the engine's failure reasons + retention delete
            // errors into the toast as a details line (never the headline —
            // that stays a translated sentence; the raw OS/anyhow strings are
            // data, shown behind a "Details" action).
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
                            .id(BACKUP_TOAST_ID)
                            .auto_dismiss_after(Duration::from_secs(6)),
                        detail,
                    );
                }
            } else if failed > 0 {
                show_result_toast(
                    ctx,
                    Toast::warning(tr!(backup_partial(ok = ok, failed = failed)))
                        .id(BACKUP_TOAST_ID)
                        .auto_dismiss_after(Duration::from_secs(6)),
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
                    .id(BACKUP_TOAST_ID)
                    .auto_dismiss_after(Duration::from_secs(6)),
                    detail,
                );
            } else if pending.close.is_none() && (ok > 0 || skipped > 0) {
                ctx.show_toast(
                    Toast::success(tr!(backup_complete(ok = ok, skipped = skipped)))
                        .id(BACKUP_TOAST_ID)
                        .auto_dismiss_after(Duration::from_secs(4)),
                );
            }
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
        if payload_id(&payload) != Some(pending.op_id.as_str()) {
            return;
        }
        self.pending.set(None);
        let error = payload
            .get("error")
            .and_then(|e| e.as_str())
            .unwrap_or_default()
            .to_string();
        // On close, a failed backup means no copy was made — same backstop as
        // "every destination failed": prompt rather than quit silently.
        if let Some(then) = pending.close {
            return self.prompt_backup_failed(ctx, then);
        }
        // T2-8: headline stays a translated sentence; the raw error string is a
        // detail behind a "Details" action, not the headline itself.
        show_result_toast(
            ctx,
            Toast::error(tr!(backup_failed_title())).id(BACKUP_TOAST_ID),
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
/// re-shown (same id) on every progress tick so the one surface updates in place.
fn progress_toast(percent: f32, message: &str) -> Toast {
    let label = progress_label(message);
    let body = if label.is_empty() {
        format!("{percent:.0}%")
    } else {
        format!("{percent:.0}% · {label}")
    };
    Toast::loading(tr!(backing_up()))
        .id(BACKUP_TOAST_ID)
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
/// stay a detail, never the headline (T2-8).
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
/// running app (T2-10).
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
        let settings =
            BackupSettingsViewModel::new(crate::models::BackupSettingsService::in_memory_default());
        let single_work = SingleWork::new(app_ctx.clone());
        let single_work_info = SingleWorkInfo::new(app_ctx.clone());
        let backup_mode = Signal::new(false);
        BackupSchedulerViewModel::new(
            app_ctx,
            settings,
            single_work,
            single_work_info,
            backup_mode,
        )
    }

    // ── T1-2: the flush invariant ────────────────────────────────────────────
    //
    // `backup_now` and `on_close_flow` also call `self.flush()` as their
    // unconditional first statement (see the module doc), but both require a
    // real `&mut EventContext` to even be called — and this codebase has no
    // `EventContext` test harness (see `restore.rs`'s `safety_copy` tests for
    // the same constraint). The two ctx-free triggers below exercise the exact
    // same private `flush()` helper those two call first, so this is the
    // guarantee the codebase can actually assert.

    #[test]
    fn on_open_flushes_before_anything_else() {
        let scheduler = test_scheduler();
        let flushed = Rc::new(Cell::new(0u32));
        {
            let flushed = flushed.clone();
            scheduler.set_flush_hook(Rc::new(move || flushed.set(flushed.get() + 1)));
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
            scheduler.set_flush_hook(Rc::new(move || flushed.set(flushed.get() + 1)));
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
    fn set_flush_hook_is_visible_on_every_existing_clone() {
        // The hook cell is shared (`Rc<RefCell<..>>`), so installing it on ONE
        // clone (as `App::build` does) must be visible on clones made earlier —
        // e.g. the project window's close guard clone in `windows.rs`.
        let scheduler = test_scheduler();
        let earlier_clone = scheduler.clone();
        let flushed = Rc::new(Cell::new(0u32));
        {
            let flushed = flushed.clone();
            scheduler.set_flush_hook(Rc::new(move || flushed.set(flushed.get() + 1)));
        }
        earlier_clone.on_open();
        assert_eq!(flushed.get(), 1, "the earlier clone must see the new hook");
    }

    // ── pure helpers ─────────────────────────────────────────────────────────

    #[test]
    fn dirs_defaults_to_next_to_project_when_empty() {
        let empty = BackupPolicy {
            destinations: vec![],
            ..BackupPolicy::default()
        };
        assert_eq!(BackupSchedulerViewModel::dirs(&empty), vec![String::new()]);
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
