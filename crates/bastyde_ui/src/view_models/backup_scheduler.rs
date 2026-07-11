//! `BackupSchedulerViewModel` — drives every automatic + manual backup trigger.
//!
//! One place that starts a `backup_now` long operation (manual / on-open /
//! interval / on-close), applies retention + records the per-destination success
//! hash on completion, and surfaces a summary toast. It holds the reactive
//! singles (to read the open project's `unique_id`/path without a context) and
//! the backup settings, and is registered as `app_state` so `App::build` can
//! route the long-operation events to it and fire its triggers.
//!
//! **On close** it also orchestrates the "back up before quitting" step: the
//! close guards defer to [`BackupSchedulerViewModel::on_close_flow`], which — if a destination is
//! reachable — runs the backup and only then performs the real (forced) close;
//! if *no* destination is reachable it shows a blocking **Retry / Discard-and-
//! exit** prompt so the user can plug a drive in and retry.

use std::rc::Rc;
use std::time::Duration;

use bastyde::prelude::*;
use bastyde::widgets::{MessageBox, MessageBoxButtons, StandardButton, Toast};

use frontend::AppContext;
use frontend::common::event::Event;
use frontend::work_management::BackupNowDto;
use skrib_format::retention::{self, RetentionPolicy};

use crate::app::PendingExit;
use crate::backup::is_destination_available;
use crate::models::{BackupPolicy, uid_is_usable};
use crate::singles::{SingleWork, SingleWorkInfo};
use crate::view_models::BackupSettingsViewModel;

use super::long_op::{event_id, parse_payload, payload_id};

const BACKUP_TOAST_ID: &str = "backup.now";

/// Context of the in-flight backup, so its completion can record hashes, prune,
/// and (on close) perform the deferred close.
#[derive(Clone)]
struct Pending {
    op_id: String,
    uid: String,
    path: String,
    /// The directories passed to the engine (a single `""` means "next to project").
    dirs: Vec<String>,
    retention: RetentionPolicy,
    min_keep: u32,
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
        }
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

    /// Whether the currently-open project wants an on-close backup (the close
    /// guards consult this before deferring to [`Self::on_close_flow`]).
    pub fn wants_on_close(&self) -> bool {
        if self.suppressed() {
            return false;
        }
        self.current()
            .map(|(uid, _)| self.settings.effective_for(&uid).on_close)
            .unwrap_or(false)
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
    pub fn on_close_flow(&self, ctx: &mut EventContext, then: PendingExit) {
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
        if !dirs.iter().any(|d| is_destination_available(d)) {
            // No reachable destination — the one case that blocks the exit.
            return self.prompt_no_destination(ctx, then);
        }
        self.start(Some(ctx), &uid, &path, &policy, false, Some(then));
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
        let hashes: Vec<String> = dirs
            .iter()
            .map(|d| {
                if force || !policy.skip_if_unchanged {
                    String::new()
                } else {
                    self.settings
                        .service()
                        .destination_state(uid, d)
                        .map(|s| s.last_success_hash)
                        .unwrap_or_default()
                }
            })
            .collect();

        let dto = BackupNowDto {
            directories: dirs.clone(),
            last_known_hashes: hashes,
        };
        match frontend::commands::work_management_commands::backup_now(&self.app_ctx, &dto) {
            Ok(op_id) => {
                self.pending.set(Some(Pending {
                    op_id,
                    uid: uid.to_string(),
                    path: path.to_string(),
                    dirs,
                    retention: policy.retention(),
                    min_keep: policy.min_keep,
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

    /// Perform the deferred close (window forced-close, or the Close-Work command).
    fn do_close(&self, ctx: &mut EventContext, then: PendingExit) {
        match then {
            PendingExit::CloseWindow => ctx.close_window_forced(),
            PendingExit::CloseWork => {
                let _ = frontend::commands::work_management_commands::close_work(&self.app_ctx);
            }
            PendingExit::None => {}
        }
    }

    // ── long-operation event handlers (wired in App::build) ───────────────────

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
            // Directories that were actually written = all minus skipped minus failed.
            let succeeded_dirs: Vec<String> = pending
                .dirs
                .iter()
                .filter(|d| {
                    !res.skipped_directories.contains(*d) && !res.failed_directories.contains(*d)
                })
                .cloned()
                .collect();

            for d in &succeeded_dirs {
                self.settings.record_destination_success(
                    &pending.uid,
                    &pending.path,
                    d,
                    &res.content_hash,
                    &now,
                );
            }
            // Retention: prune every destination we wrote to or already had current.
            for d in succeeded_dirs.iter().chain(res.skipped_directories.iter()) {
                if let Some(dir) = retention_dir(d, &pending.path) {
                    let _ = retention::apply_retention(
                        &dir,
                        &pending.uid,
                        &pending.path,
                        &pending.retention,
                        pending.min_keep,
                    );
                }
            }

            let ok = res.succeeded_paths.len();
            let skipped = res.skipped_directories.len();
            let failed = res.failed_directories.len();
            produced_nothing = ok == 0 && skipped == 0 && failed > 0;

            // A backup really happened (written, or already current) → restart the
            // interval countdown so it measures time since the *last* backup.
            if ok > 0 || skipped > 0 {
                let e = &self.completed_epoch;
                e.set(e.get().wrapping_add(1));
            }

            // Quiet on a pure no-op close (nothing written, nothing failed); noisy
            // enough to reassure on a manual/successful write, and warn on failure.
            // When *everything* failed on close we prompt below instead of toasting.
            if produced_nothing {
                if pending.close.is_none() {
                    ctx.show_toast(
                        Toast::error(tr!(backup_partial(ok = ok, failed = failed)))
                            .id(BACKUP_TOAST_ID)
                            .auto_dismiss_after(Duration::from_secs(6)),
                    );
                }
            } else if failed > 0 {
                ctx.show_toast(
                    Toast::warning(tr!(backup_partial(ok = ok, failed = failed)))
                        .id(BACKUP_TOAST_ID)
                        .auto_dismiss_after(Duration::from_secs(6)),
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
        ctx.show_toast(Toast::error(tr!(backup_error(error = error))).id(BACKUP_TOAST_ID));
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

/// The directory retention should sweep for destination `dir`: the dir itself,
/// or (for the empty "next to project" destination) the project's parent folder.
fn retention_dir(dir: &str, project_path: &str) -> Option<std::path::PathBuf> {
    if dir.trim().is_empty() {
        std::path::Path::new(project_path)
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .map(|p| p.to_path_buf())
    } else {
        Some(std::path::PathBuf::from(dir))
    }
}
