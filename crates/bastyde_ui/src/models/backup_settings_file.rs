// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Persistent backup ("Copies de secours") configuration.
//!
//! A `SettingsFile<BackupSettingsFile>` at `<config_dir>/backup.toml` holding one
//! **general** policy plus optional **per-project overrides** and per-project
//! bookkeeping (last-success hashes/timestamps for dedup + the "last backup"
//! indicator, and the one-time no-backups nudge flag).
//!
//! Per-project entries are keyed by the project's stable **`Work.unique_id`**
//! (not its path — that survives rename/move, and is the same key
//! `skrib_format::retention` correlates backups on). The raw path + title are
//! kept only for display. This is app configuration, orthogonal to the backend,
//! so there is a single implementation (no real/mock seam) — it opens a real
//! `SettingsFile` in every build, exactly like `WindowStateService`.
//!
//! Retention is stored as **flat fields** + a unit `RetentionMode` enum rather
//! than a nested data-carrying enum, so it round-trips cleanly through TOML
//! (which serialises unit enums as plain strings). Those same flat fields go
//! straight into `BackupNowDto`, and the **backend** rebuilds the
//! `skrib_format::retention::RetentionPolicy` from them — one conversion, in one
//! place, next to the engine that consumes it.
//!
//! **T1-6 — cross-process safety.** Skribisto is single-instance: normally one
//! process hosts every open project window, sharing this same
//! `<config_dir>/backup.toml` — two windows editing overrides for two different
//! projects (or the general policy) must never clobber each other. That is now
//! `SettingsFile<T>`'s **only** mode (`load_shared` is gone — `load` performs the locked
//! read-modify-write unconditionally, see `bastyde_settings::file`'s module
//! docs), so this service just calls [`SettingsFile::load`] like any other
//! persisted type.
//!
//! **Reads no longer eagerly reload.** The old per-read `reload_if_stale()`
//! poll before every getter is gone: cross-process *write* safety was always
//! the point of that call's sibling machinery, but eagerly re-`stat`ing the
//! file on every single read was a workaround for not having a real
//! change-notification path. Now there is one: this handle is registered
//! into the app's shared `bastyde::settings::SettingsRegistry` (see
//! `App::build` in `app.rs`), and the app's `SettingsWatcher` calls
//! [`Reloadable::reload_from_disk`]
//! on it the moment a peer's write lands on disk — no polling, and reads in
//! between two writes are just plain in-memory reads. [`as_reloadable`](BackupSettingsService::as_reloadable)
//! exposes the hook that registration needs. Tests that stand in for two
//! processes (no live app, no watcher) call `as_reloadable().reload_from_disk()`
//! directly to simulate the watcher firing.

use std::rc::Rc;
use std::time::Duration;

use bastyde::settings::{
    AppPaths, Migrator, Reloadable, SettingsFile, SettingsFileError, Versioned,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Debounce parameter accepted by [`open_with_delay`](BackupSettingsService::open_with_delay)/[`open_at`](BackupSettingsService::open_at) for call-site
/// stability only. `SettingsFile::load`'s writes are always a synchronous
/// locked read-modify-write now (see the module docs) — there is no debounce
/// left to configure, exactly like `bastyde-settings`' own `WindowStateService`.
const SETTINGS_DEBOUNCE: Duration = Duration::from_millis(500);

/// Which retention strategy a policy uses.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RetentionMode {
    /// Keep 1 per bucket across thinning time tiers (grandfather-father-son).
    #[default]
    Tiered,
    /// Keep the N most recent backups.
    KeepLastN,
}

/// A complete backup policy — the general default, or a per-project override.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BackupPolicy {
    // Triggers.
    pub on_close: bool,
    pub on_open: bool,
    pub interval_enabled: bool,
    pub interval_hours: u32,
    /// Destination directories.
    ///
    /// **Empty is the symbolic default**, resolved at use time by
    /// [`crate::backup_paths`] to the app's own backup root — deliberately not
    /// persisted as an absolute path, which would not survive a Flatpak/native
    /// switch. An explicit `""` entry still means "next to the project".
    pub destinations: Vec<String>,
    // Retention (flat for TOML friendliness; see `retention()`).
    pub retention_mode: RetentionMode,
    pub keep_last_n: u32,
    pub gfs_hourly: u32,
    pub gfs_daily: u32,
    pub gfs_weekly: u32,
    pub gfs_monthly: u32,
    /// Absolute floor: the newest `min_keep` backups are never pruned.
    pub min_keep: u32,
    /// Skip a write when the content matches the destination's last-known hash.
    pub skip_if_unchanged: bool,
}

impl Default for BackupPolicy {
    fn default() -> Self {
        // The user-confirmed defaults: back up on close *and* every two hours,
        // tiered retention, into the app's own backup root.
        //
        // `interval_enabled` is on because closing is not a writing rhythm. This
        // process hosts every window and every open project, so "on close" fires
        // when the *app* exits — which for a writer who leaves it running is days
        // apart, not sittings. On its own that yields a history too sparse to
        // answer "what did this scene say this morning".
        //
        // Two hours, and not less, because `write_zip` rebuilds the whole archive
        // on every write: the cost is proportional to the project's assets, not to
        // the prose that changed. `skip_if_unchanged` already suppresses the write
        // entirely when nothing moved, so an idle app still costs nothing.
        //
        // `destinations` stays **empty on purpose** — that is the symbolic default
        // resolved at use time by `crate::backup_paths`, never a persisted absolute
        // path (which would break across a Flatpak/native switch).
        BackupPolicy {
            on_close: true,
            on_open: false,
            interval_enabled: true,
            interval_hours: 2,
            destinations: Vec::new(),
            retention_mode: RetentionMode::Tiered,
            keep_last_n: 25,
            gfs_hourly: 24,
            gfs_daily: 7,
            gfs_weekly: 4,
            gfs_monthly: 12,
            min_keep: 3,
            skip_if_unchanged: true,
        }
    }
}

impl BackupPolicy {
    /// True when this policy has no way to actually produce a backup — the
    /// "no backups configured" condition the nudge/hint keys on.
    pub fn is_effectively_off(&self) -> bool {
        !self.on_close && !self.on_open && !self.interval_enabled
    }
}

/// Per-(project, destination) bookkeeping. Not user-facing config — powers
/// skip-if-unchanged and the "last backup" indicator.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct DestinationState {
    pub directory: String,
    pub last_success_hash: String,
    /// The on-disk path of the most recent successful backup written to this
    /// destination. Persisted (T1-6) so the engine's skip-if-unchanged check
    /// (T1-3 — "does the previously-recorded backup still exist?") survives a
    /// restart, not just the current process's in-memory run.
    #[serde(default)]
    pub last_success_path: String,
    /// RFC3339.
    pub last_success_at: String,
}

/// Per-project bookkeeping, keyed by `work_uid`.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct ProjectBackupState {
    pub work_uid: String,
    /// Last known on-disk path (display/debug only).
    pub last_path: String,
    /// The one-time "no backups configured" toast has already been shown.
    pub nudged: bool,
    pub destination_states: Vec<DestinationState>,
    /// Backup files this project has asked never to have swept away.
    ///
    /// Retention is a policy about *how much past to keep in general*, and it has
    /// no way to know that one of those files is the draft that went to an
    /// editor. Without this, a feature whose whole purpose is not losing things
    /// eventually deletes the one copy that mattered most.
    ///
    /// Absolute paths, by identity — the same thing
    /// [`skrib_format::retention::plan_deletions`]'s `protected` argument
    /// guarantees, and deliberately not timestamps, which a backwards clock
    /// correction can reorder.
    ///
    /// `#[serde(default)]`, so every `backup.toml` written before this field
    /// existed still loads — the same additive rule every other field here
    /// follows.
    #[serde(default)]
    pub pinned: Vec<String>,
}

/// A per-project override of the general policy (all-or-nothing).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PerProjectBackupOverride {
    pub work_uid: String,
    /// Display/debug only (the key is `work_uid`).
    pub last_path: String,
    pub title: String,
    pub policy: BackupPolicy,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BackupSettingsFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub general: BackupPolicy,
    #[serde(default)]
    pub overrides: Vec<PerProjectBackupOverride>,
    #[serde(default)]
    pub project_states: Vec<ProjectBackupState>,
}

fn default_version() -> u32 {
    BackupSettingsFile::CURRENT_VERSION
}

impl Default for BackupSettingsFile {
    fn default() -> Self {
        BackupSettingsFile {
            version: BackupSettingsFile::CURRENT_VERSION,
            general: BackupPolicy::default(),
            overrides: Vec::new(),
            project_states: Vec::new(),
        }
    }
}

impl Versioned for BackupSettingsFile {
    const CURRENT_VERSION: u32 = 1;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// Persistent backup-settings service. `SettingsFile` is `Clone` (shares the
/// in-memory state + debounced writer), so cloning hands out views over the same
/// live configuration — the settings VM and the scheduler share one.
#[derive(Clone)]
pub struct BackupSettingsService {
    file: SettingsFile<BackupSettingsFile>,
}

impl BackupSettingsService {
    /// Open `backup.toml` under `paths` (T1-6: cross-process safe by default).
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        Self::open_with_delay(paths, SETTINGS_DEBOUNCE)
    }

    /// `delay` is accepted for call-site stability but has no effect: writes
    /// are always a synchronous locked read-modify-write (see the module docs).
    pub fn open_with_delay(paths: &AppPaths, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("backup"), Migrator::new())?;
        Ok(Self { file })
    }

    /// Open at an explicit path — used by tests. `delay` is likewise accepted
    /// only for signature stability (see [`open_with_delay`](Self::open_with_delay)).
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn open_at(path: std::path::PathBuf, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, Migrator::new())?;
        Ok(Self { file })
    }

    /// Graceful fallback when the config dir is unavailable: a throwaway
    /// per-process temp file, so the app still runs (backups just won't persist
    /// their settings across restarts). See [`in_memory_settings_file`] for what
    /// "graceful" means when even the temp dir turns out to be unwritable.
    pub fn in_memory_default() -> Self {
        let file = in_memory_settings_file("backup", Migrator::new());
        Self { file }
    }

    /// The `Reloadable` hook for the app's shared `SettingsRegistry` — register
    /// this (and keep the returned handle alive) so the settings-file watcher
    /// picks up a peer process's write and refreshes this handle in place, with
    /// no per-read polling. See the module docs.
    pub fn as_reloadable(&self) -> Rc<dyn Reloadable> {
        Rc::new(self.file.clone())
    }

    // ── general policy ──
    pub fn general(&self) -> BackupPolicy {
        self.file.borrow().general.clone()
    }

    pub fn set_general(&self, policy: BackupPolicy) -> Result<(), SettingsFileError> {
        self.file.mutate(|f| f.general = policy)
    }

    // ── per-project overrides ──
    pub fn has_override(&self, work_uid: &str) -> bool {
        self.file
            .borrow()
            .overrides
            .iter()
            .any(|o| o.work_uid == work_uid)
    }

    /// The policy in effect for `work_uid`: its override if set, else general.
    pub fn effective_for(&self, work_uid: &str) -> BackupPolicy {
        let f = self.file.borrow();
        f.overrides
            .iter()
            .find(|o| o.work_uid == work_uid)
            .map(|o| o.policy.clone())
            .unwrap_or_else(|| f.general.clone())
    }

    pub fn set_override(
        &self,
        work_uid: &str,
        last_path: &str,
        title: &str,
        policy: BackupPolicy,
    ) -> Result<(), SettingsFileError> {
        self.file.mutate(|f| {
            if let Some(existing) = f.overrides.iter_mut().find(|o| o.work_uid == work_uid) {
                existing.last_path = last_path.to_string();
                existing.title = title.to_string();
                existing.policy = policy;
            } else {
                f.overrides.push(PerProjectBackupOverride {
                    work_uid: work_uid.to_string(),
                    last_path: last_path.to_string(),
                    title: title.to_string(),
                    policy,
                });
            }
        })
    }

    pub fn clear_override(&self, work_uid: &str) -> Result<(), SettingsFileError> {
        self.file
            .mutate(|f| f.overrides.retain(|o| o.work_uid != work_uid))
    }

    // ── per-destination bookkeeping (dedup + "last backup") ──
    pub fn destination_state(&self, work_uid: &str, dir: &str) -> Option<DestinationState> {
        self.file
            .borrow()
            .project_states
            .iter()
            .find(|p| p.work_uid == work_uid)
            .and_then(|p| p.destination_states.iter().find(|d| d.directory == dir))
            .cloned()
    }

    /// Most recent successful-backup timestamp across this project's destinations.
    pub fn last_backup_at(&self, work_uid: &str) -> Option<String> {
        self.file
            .borrow()
            .project_states
            .iter()
            .find(|p| p.work_uid == work_uid)
            .and_then(|p| {
                p.destination_states
                    .iter()
                    .map(|d| d.last_success_at.clone())
                    .filter(|s| !s.is_empty())
                    .max()
            })
    }

    /// Record a successful write: the hash (dedup), the exact on-disk path
    /// (T1-6 — `backup_path`, so the engine's "does the previous backup still
    /// exist" check (T1-3) survives a restart), and the timestamp.
    pub fn record_destination_success(
        &self,
        work_uid: &str,
        last_path: &str,
        dir: &str,
        hash: &str,
        backup_path: &str,
        at: &str,
    ) -> Result<(), SettingsFileError> {
        self.file.mutate(|f| {
            let ps = project_state_mut(f, work_uid);
            ps.last_path = last_path.to_string();
            if let Some(d) = ps
                .destination_states
                .iter_mut()
                .find(|d| d.directory == dir)
            {
                d.last_success_hash = hash.to_string();
                d.last_success_path = backup_path.to_string();
                d.last_success_at = at.to_string();
            } else {
                ps.destination_states.push(DestinationState {
                    directory: dir.to_string(),
                    last_success_hash: hash.to_string(),
                    last_success_path: backup_path.to_string(),
                    last_success_at: at.to_string(),
                });
            }
        })
    }

    // ── pinned backups ──

    /// Every backup this project has pinned.
    pub fn pinned(&self, work_uid: &str) -> Vec<String> {
        self.file
            .borrow()
            .project_states
            .iter()
            .find(|p| p.work_uid == work_uid)
            .map(|p| p.pinned.clone())
            .unwrap_or_default()
    }

    pub fn is_pinned(&self, work_uid: &str, path: &str) -> bool {
        self.file
            .borrow()
            .project_states
            .iter()
            .find(|p| p.work_uid == work_uid)
            .is_some_and(|p| p.pinned.iter().any(|x| x == path))
    }

    /// Pin or unpin one backup file. Idempotent either way.
    pub fn set_pinned(
        &self,
        work_uid: &str,
        last_path: &str,
        path: &str,
        pinned: bool,
    ) -> Result<(), SettingsFileError> {
        self.file.mutate(|f| {
            let ps = project_state_mut(f, work_uid);
            ps.last_path = last_path.to_string();
            if pinned {
                if !ps.pinned.iter().any(|x| x == path) {
                    ps.pinned.push(path.to_string());
                }
            } else {
                ps.pinned.retain(|x| x != path);
            }
        })
    }

    /// Drop pins whose file is gone.
    ///
    /// A pin is a promise about a file, and a file a writer deleted by hand is
    /// not one this can keep. Without this the list grows forever with names of
    /// things that no longer exist, and `protected` starts carrying paths that
    /// protect nothing.
    pub fn forget_missing_pins(&self, work_uid: &str) -> Result<(), SettingsFileError> {
        // One read of the list, not two: this runs on every backup dispatch, and
        // the common case is that nothing has gone missing.
        let pinned = self.pinned(work_uid);
        let live: Vec<String> = pinned
            .iter()
            .filter(|p| std::path::Path::new(p).exists())
            .cloned()
            .collect();
        if live.len() == pinned.len() {
            return Ok(());
        }
        self.file
            .mutate(|f| project_state_mut(f, work_uid).pinned = live)
    }

    // ── one-time no-backups nudge ──
    pub fn was_nudged(&self, work_uid: &str) -> bool {
        self.file
            .borrow()
            .project_states
            .iter()
            .find(|p| p.work_uid == work_uid)
            .map(|p| p.nudged)
            .unwrap_or(false)
    }

    pub fn mark_nudged(&self, work_uid: &str, last_path: &str) -> Result<(), SettingsFileError> {
        self.file.mutate(|f| {
            let ps = project_state_mut(f, work_uid);
            ps.last_path = last_path.to_string();
            ps.nudged = true;
        })
    }

    pub fn flush_now(&self) -> Result<(), SettingsFileError> {
        self.file.flush_now()
    }
}

fn project_state_mut<'a>(
    f: &'a mut BackupSettingsFile,
    work_uid: &str,
) -> &'a mut ProjectBackupState {
    if let Some(pos) = f.project_states.iter().position(|p| p.work_uid == work_uid) {
        &mut f.project_states[pos]
    } else {
        f.project_states.push(ProjectBackupState {
            work_uid: work_uid.to_string(),
            ..Default::default()
        });
        f.project_states.last_mut().unwrap()
    }
}

/// Ignore an empty UUID as a key: a brand-new unsaved project has none yet, and
/// keying by "" would collide across unrelated projects. Callers guard on this.
pub fn uid_is_usable(work_uid: &str) -> bool {
    !work_uid.trim().is_empty()
}

/// Every sibling `*Service::in_memory_default()`'s graceful-degradation path, shared here so
/// the retry/uniqueness logic exists exactly once. Called by
/// [`crate::models::search_settings_file`], [`crate::models::tree_expansion_file`],
/// [`crate::models::workspace_layout_file`], [`crate::models::dictionary_settings_file`],
/// [`crate::models::export_styles_file`], [`crate::models::distraction_free_themes_file`] and
/// [`crate::models::paratext_presets`] as `super::backup_settings_file::in_memory_settings_file`
/// (this module is where [`uid_is_usable`] already lives as the other shared free function, so
/// it is the natural home for this one too — not `models.rs`, which stays generated-shape and
/// isn't where any of the eight callers' own logic lives).
///
/// **There is no non-persisting `SettingsFile` constructor to reach for instead.** Checked
/// directly against `bastyde-settings::file`: `SettingsFile::load` and `load_strict` both call
/// `FileLock::acquire_exclusive` unconditionally — real disk I/O — before they ever produce a
/// value, so a settings file that skips storage entirely is not something the framework
/// currently offers, and this crate does not modify `bastyde` to add one (out of scope, and
/// unnecessary for what follows). What this function *can* guarantee, working only within
/// that constraint, is that reaching it never aborts the caller: `prefix` names the caller
/// (`"backup"`, `"search"`, …) for both the file names it tries and the diagnostics it prints
/// past a failed one; `migrator` is the caller's own, forwarded unchanged.
///
/// Delegates to [`in_memory_settings_file_under`] with the two real-world candidate roots (see
/// there for the actual attempt/retry shape); split out so a test can hand
/// [`in_memory_settings_file_under`] a root that is guaranteed to fail without needing to
/// sabotage the real OS temp dir that every other test in this process also reads.
pub(crate) fn in_memory_settings_file<T>(prefix: &str, migrator: Migrator<T>) -> SettingsFile<T>
where
    T: Versioned + Serialize + DeserializeOwned + Default + Clone + 'static,
{
    // Root 1: the OS temp dir — succeeds in every ordinary launch, including a Flatpak
    // sandbox (which gives the app its own private, always-writable `/tmp`).
    let mut roots = vec![std::env::temp_dir()];
    // Root 2: a single, recognisably-named folder under the *home* dir — reached only once
    // the whole temp filesystem has refused, so a genuinely different mount/permission domain
    // gets a real chance instead of retrying the one that already said no. Deliberately not
    // the caller's arbitrary current directory: writing there (the original bug) litters
    // whatever folder the app happened to be launched from with a bare dotfile a writer would
    // never think to look for; a single well-known home-relative folder is at least
    // findable and deletable on purpose.
    if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
        roots.push(std::path::PathBuf::from(home).join(".skribisto-emergency-settings"));
    }
    in_memory_settings_file_under(prefix, migrator, &roots)
}

/// The testable core of [`in_memory_settings_file`]: try each of `roots` in turn — a flat file
/// first, then (only if that fails) a freshly-created subdirectory of it, since a flat-file
/// failure and a whole-root failure are different failure domains (some sandboxes gate
/// specific filenames but still allow directory creation) — before moving on to the next root.
///
/// Both attempts under a root use a name unique to *this specific attempt* (a nanosecond
/// timestamp plus a per-process counter, see [`unique_suffix`]), never just the caller's pid
/// the way the original, panicking version of this fallback did: a relaunch the OS happens to
/// hand the same pid a crashed run had could otherwise collide with that run's leftover
/// `<path>.lock` sidecar — `FileLock` deliberately never deletes it (see
/// `bastyde-settings::lock`'s module docs) — and wedge on a lock nobody holds any more.
///
/// If every root is exhausted, there is no writable storage anywhere this process can see — a
/// state in which nothing else the app needs (autosave, the backup engine, the single-instance
/// socket) could function either. That is not "this one setting failed to load," it is "this
/// environment cannot run Skribisto," so it is reported as a clean, logged
/// [`std::process::exit`] rather than a panic: no unwind, so no `catch_unwind` boundary
/// elsewhere in the app (`common::long_operation`) ever sees it, and no lock anywhere in the
/// process is left poisoned by it — the two hazards a bare `.expect()` here would reintroduce
/// now that the release profile's `panic = "abort"` is gone and panics unwind again. This
/// branch cannot be exercised by a test without spawning a subprocess (it ends the process by
/// design); the two real disk attempts above are what the tests below cover.
fn in_memory_settings_file_under<T>(
    prefix: &str,
    migrator: Migrator<T>,
    roots: &[std::path::PathBuf],
) -> SettingsFile<T>
where
    T: Versioned + Serialize + DeserializeOwned + Default + Clone + 'static,
{
    let pid = std::process::id();
    for root in roots {
        let flat = root.join(format!("skribisto-{prefix}-{pid}-{}.toml", unique_suffix()));
        match SettingsFile::load(flat, migrator.clone()) {
            Ok(file) => return file,
            Err(e) => eprintln!(
                "skribisto: {prefix} in-memory settings: {} (flat attempt under {})",
                e,
                root.display()
            ),
        }
        let sub = root.join(format!("skribisto-{prefix}-{pid}-{}", unique_suffix()));
        if std::fs::create_dir_all(&sub).is_ok() {
            match SettingsFile::load(sub.join("settings.toml"), migrator.clone()) {
                Ok(file) => return file,
                Err(e) => eprintln!(
                    "skribisto: {prefix} in-memory settings: {} (fresh-subdirectory attempt under {})",
                    e,
                    root.display()
                ),
            }
        }
    }
    eprintln!(
        "skribisto: {prefix} settings: no writable location found across {} candidate root(s); \
         this environment cannot run Skribisto",
        roots.len()
    );
    std::process::exit(70); // EX_SOFTWARE — a deliberate, logged, non-panicking exit.
}

/// A per-attempt identifier unique enough that two [`in_memory_settings_file_under`] attempts
/// — even two in the same nanosecond, from the same process — never collide on a path: a
/// nanosecond timestamp plus a monotonic per-process counter as a tiebreaker.
fn unique_suffix() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{nanos}-{n}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn svc(dir: &std::path::Path) -> BackupSettingsService {
        BackupSettingsService::open_at(dir.join("backup.toml"), Duration::ZERO).unwrap()
    }

    // ── pinned backups ──────────────────────────────────────────────────────

    /// A pin is a promise that has to outlive the session that made it — the
    /// whole point is surviving a retention sweep weeks later.
    #[test]
    fn a_pin_round_trips_through_the_settings_file() {
        let dir = tempdir().unwrap();
        let kept = dir.path().join("Novel-20260701-090000.skrib");
        std::fs::write(&kept, b"x").unwrap();

        {
            let s = svc(dir.path());
            s.set_pinned("uid", "/tmp/Novel.skrib", kept.to_str().unwrap(), true)
                .unwrap();
            s.flush_now().unwrap();
        }
        // A fresh service over the same file — i.e. the next launch.
        let s = svc(dir.path());
        assert!(s.is_pinned("uid", kept.to_str().unwrap()));
        assert_eq!(s.pinned("uid"), vec![kept.to_string_lossy().to_string()]);
    }

    #[test]
    fn pinning_twice_records_one_pin_and_unpinning_removes_it() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        s.set_pinned("uid", "", "/backups/a.skrib", true).unwrap();
        s.set_pinned("uid", "", "/backups/a.skrib", true).unwrap();
        assert_eq!(s.pinned("uid").len(), 1, "a pin is a state, not a counter");

        s.set_pinned("uid", "", "/backups/a.skrib", false).unwrap();
        assert!(s.pinned("uid").is_empty());
        assert!(!s.is_pinned("uid", "/backups/a.skrib"));
    }

    #[test]
    fn two_projects_pins_never_mix() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        s.set_pinned("one", "", "/backups/one.skrib", true).unwrap();
        s.set_pinned("two", "", "/backups/two.skrib", true).unwrap();
        assert_eq!(s.pinned("one"), vec!["/backups/one.skrib".to_string()]);
        assert!(!s.is_pinned("two", "/backups/one.skrib"));
    }

    /// A pin names a file. A file the writer deleted by hand is a promise this
    /// cannot keep, and carrying its name forever would grow `protected` into a
    /// list of things that protect nothing.
    #[test]
    fn a_pin_whose_file_is_gone_is_forgotten() {
        let dir = tempdir().unwrap();
        let here = dir.path().join("still-here.skrib");
        std::fs::write(&here, b"x").unwrap();
        let gone = dir.path().join("deleted-by-hand.skrib");

        let s = svc(dir.path());
        s.set_pinned("uid", "", here.to_str().unwrap(), true)
            .unwrap();
        s.set_pinned("uid", "", gone.to_str().unwrap(), true)
            .unwrap();
        assert_eq!(s.pinned("uid").len(), 2);

        s.forget_missing_pins("uid").unwrap();
        assert_eq!(
            s.pinned("uid"),
            vec![here.to_string_lossy().to_string()],
            "only the pin whose file still exists survives",
        );
    }

    // ── the shipped capture policy ──────────────────────────────────────────

    /// Pinned so changing what a writer gets out of the box is always a deliberate
    /// act with a failing test to justify, never a drive-by edit.
    ///
    /// The two that carry the most weight: `interval_enabled` (without it the only
    /// automatic trigger is app *exit*, which for a long-running session is days
    /// apart — far too sparse to be a version history), and `destinations` staying
    /// empty (the symbolic default `crate::backup_paths` resolves at use time; a
    /// persisted absolute path would not survive a Flatpak/native switch).
    #[test]
    fn the_default_policy_captures_on_close_and_every_two_hours() {
        let p = BackupPolicy::default();
        assert!(p.on_close, "closing the project must still take a backup");
        assert!(
            p.interval_enabled,
            "periodic capture is what makes the history dense enough to be useful",
        );
        assert_eq!(
            p.interval_hours, 2,
            "shorter re-deflates the whole archive too often"
        );
        assert!(
            p.destinations.is_empty(),
            "the default destination is symbolic and resolved at use time, never persisted",
        );
        assert!(
            p.skip_if_unchanged,
            "an idle app must not write identical backups"
        );
        assert!(
            !p.is_effectively_off(),
            "the shipped default must actually produce backups"
        );
    }

    // ── in-memory fallback: infallible, never panics ────────────────────────

    /// The primary candidate can be entirely unusable (a stale lock, a read-only mount, a
    /// sandbox denial) without the caller ever seeing a panic: `in_memory_settings_file_under`
    /// must fall through to the next root and hand back a genuinely usable, disk-backed file
    /// — this is exactly the path every sibling `in_memory_default()` takes when the config
    /// dir it actually wants is unavailable.
    ///
    /// The "unusable" root is a path that walks *through* a plain file
    /// (`<blocker-file>/nested`), which makes every `create_dir_all` under it fail
    /// deterministically — no env var mutation (`TMPDIR`/`HOME` are process-global and would
    /// race every other test in this binary that also calls `std::env::temp_dir()`), and no
    /// dependence on actual filesystem permissions (which a CI runner as root would ignore
    /// anyway).
    #[test]
    fn the_shared_fallback_survives_an_unusable_first_root() {
        let scratch = tempdir().unwrap();
        let blocker = scratch.path().join("not-a-directory");
        std::fs::write(&blocker, b"").unwrap();
        let unusable_root = blocker.join("nested");
        let usable_root = scratch.path().join("actually-writable");

        let file: SettingsFile<BackupSettingsFile> = in_memory_settings_file_under(
            "fallback-test",
            Migrator::new(),
            &[unusable_root.clone(), usable_root.clone()],
        );
        assert!(
            file.path().starts_with(&usable_root),
            "must have fallen through to the second, usable root instead of the first: {}",
            file.path().display()
        );
        // And it must be genuinely writable, not a fluke default that happens to compare
        // equal — round-trip a real mutation through it.
        file.mutate(|f| f.general.on_open = true).unwrap();
        assert!(file.borrow().general.on_open);
    }

    /// The real, non-injected entry point every sibling service's `in_memory_default()`
    /// reaches for: it must never panic in the ordinary case (a writable OS temp dir), and the
    /// handle it hands back must be immediately usable.
    #[test]
    fn in_memory_default_is_infallible_and_usable() {
        let svc = BackupSettingsService::in_memory_default();
        let mut policy = svc.general();
        policy.on_open = true;
        svc.set_general(policy).unwrap();
        assert!(
            svc.general().on_open,
            "the fallback handle is really writable"
        );
    }

    /// A freshly-created settings file hands back the shipped policy — the same one
    /// `the_default_policy_captures_on_close_and_every_two_hours` pins, checked here
    /// through the service so a bad `Default` *and* a bad round-trip both fail.
    #[test]
    fn defaults_are_on_close_interval_tiered() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        let g = s.general();
        assert!(g.on_close && !g.on_open && g.interval_enabled);
        assert_eq!(g.interval_hours, 2);
        assert_eq!(g.retention_mode, RetentionMode::Tiered);
        assert_eq!(g.min_keep, 3);
        assert_eq!(g.gfs_monthly, 12);
    }

    #[test]
    fn override_wins_and_is_isolated_per_uid() {
        let d = tempdir().unwrap();
        let s = svc(d.path());

        // Project A gets an override; project B still sees the general policy.
        let mut custom = s.general();
        custom.on_open = true;
        custom.retention_mode = RetentionMode::KeepLastN;
        custom.keep_last_n = 5;
        s.set_override("uid-A", "/x/a.skrib", "A", custom).unwrap();

        assert!(s.has_override("uid-A"));
        assert!(!s.has_override("uid-B"));
        assert!(s.effective_for("uid-A").on_open);
        assert_eq!(
            s.effective_for("uid-A").retention_mode,
            RetentionMode::KeepLastN
        );
        assert_eq!(s.effective_for("uid-A").keep_last_n, 5);
        assert!(!s.effective_for("uid-B").on_open, "B inherits general");

        s.clear_override("uid-A").unwrap();
        assert!(!s.has_override("uid-A"));
        assert!(!s.effective_for("uid-A").on_open, "A back to general");
    }

    #[test]
    fn destination_state_and_last_backup_roundtrip_on_disk() {
        let d = tempdir().unwrap();
        {
            let s = svc(d.path());
            s.record_destination_success(
                "uid-A",
                "/x/a.skrib",
                "/backups",
                "hash1",
                "/backups/a-20260601-100000.skrib",
                "2026-06-01T10:00:00Z",
            )
            .unwrap();
            s.record_destination_success(
                "uid-A",
                "/x/a.skrib",
                "/usb",
                "hash2",
                "/usb/a-20260602-100000.skrib",
                "2026-06-02T10:00:00Z",
            )
            .unwrap();
            s.mark_nudged("uid-A", "/x/a.skrib").unwrap();
            s.flush_now().unwrap();
        }
        // Reopen from disk — everything survived (proves TOML round-trip of the
        // nested structs + the flat retention fields).
        let s = svc(d.path());
        assert_eq!(
            s.destination_state("uid-A", "/backups")
                .unwrap()
                .last_success_hash,
            "hash1"
        );
        assert_eq!(
            s.destination_state("uid-A", "/backups")
                .unwrap()
                .last_success_path,
            "/backups/a-20260601-100000.skrib",
            "T1-6: the exact written path round-trips too, not just the hash"
        );
        assert_eq!(
            s.last_backup_at("uid-A").as_deref(),
            Some("2026-06-02T10:00:00Z")
        );
        assert!(s.was_nudged("uid-A"));
    }

    // ── T1-6: cross-process shared mode ─────────────────────────────────────

    #[test]
    fn two_shared_mode_services_over_one_file_do_not_clobber_each_others_overrides() {
        // Mirrors bastyde-settings' own
        // `shared_mode_two_concurrent_handles_both_writes_survive` headline test,
        // at this crate's own level: two `BackupSettingsService`s standing in for
        // two windows sharing one `backup.toml`, each writing a *different*
        // project's override. Without
        // `SettingsFile::load`'s locked read-modify-write (the only mode —
        // `load_shared` no longer exists as a separate opt-in), the second
        // write's stale in-memory snapshot would silently drop the first.
        let d = tempdir().unwrap();
        let path = d.path().join("backup.toml");

        let a = BackupSettingsService::open_at(path.clone(), Duration::ZERO).unwrap();
        let b = BackupSettingsService::open_at(path.clone(), Duration::ZERO).unwrap();

        let mut policy_a = a.general();
        policy_a.on_open = true;
        a.set_override("uid-A", "/x/a.skrib", "A", policy_a)
            .unwrap();

        let mut policy_b = b.general();
        policy_b.interval_enabled = true;
        policy_b.interval_hours = 4;
        b.set_override("uid-B", "/x/b.skrib", "B", policy_b)
            .unwrap();

        // A third, fresh handle proves both writes actually landed on disk
        // together, not just in `a`'s or `b`'s own memory.
        let c = BackupSettingsService::open_at(path, Duration::ZERO).unwrap();
        assert!(c.has_override("uid-A"), "a's override must survive");
        assert!(c.effective_for("uid-A").on_open);
        assert!(c.has_override("uid-B"), "b's override must survive");
        assert!(c.effective_for("uid-B").interval_enabled);
        assert_eq!(c.effective_for("uid-B").interval_hours, 4);
    }

    #[test]
    fn a_peers_write_is_not_seen_until_reload_from_disk_is_called() {
        // Reads no longer eagerly poll the file (that per-read `reload_if_stale`
        // call is gone — see the module docs): in the real app, the app's
        // `SettingsWatcher` calls `Reloadable::reload_from_disk()` on this
        // handle the moment a peer's write lands on disk. Here (no live app, no
        // watcher) we drive that same hook by hand, proving both halves: a bare
        // read really doesn't see the peer's write, and the hook — once
        // invoked — makes it visible.
        let d = tempdir().unwrap();
        let path = d.path().join("backup.toml");

        let a = BackupSettingsService::open_at(path.clone(), Duration::ZERO).unwrap();
        let b = BackupSettingsService::open_at(path, Duration::ZERO).unwrap();

        assert!(!a.has_override("uid-A"));
        b.record_destination_success(
            "uid-A",
            "/x/a.skrib",
            "/backups",
            "hash1",
            "/backups/a.skrib",
            "2026-06-01T10:00:00Z",
        )
        .unwrap();

        // `a` never wrote anything itself and never re-checks disk on its own.
        assert!(
            a.destination_state("uid-A", "/backups").is_none(),
            "no eager reload: a bare read must not see a peer's write yet"
        );

        // Simulate the watcher noticing the peer's write and firing the hook.
        let changed = a
            .as_reloadable()
            .reload_from_disk()
            .expect("reload_from_disk should succeed");
        assert!(
            changed,
            "reload_from_disk must report the peer's real change"
        );

        assert_eq!(
            a.destination_state("uid-A", "/backups")
                .unwrap()
                .last_success_path,
            "/backups/a.skrib"
        );
        assert!(a.last_backup_at("uid-A").is_some());
    }

    #[test]
    fn registering_with_the_settings_registry_reloads_on_dispatch() {
        // The actual integration point `App::build` relies on: register this
        // handle's `Reloadable` into a `SettingsRegistry` (as the app does with
        // the real, watcher-backed registry), then a path-keyed `dispatch` —
        // exactly what `SettingsWatcher`'s sink triggers — must reload it.
        use bastyde::settings::SettingsRegistry;

        let d = tempdir().unwrap();
        let path = d.path().join("backup.toml");

        let a = BackupSettingsService::open_at(path.clone(), Duration::ZERO).unwrap();
        let b = BackupSettingsService::open_at(path.clone(), Duration::ZERO).unwrap();

        let registry = SettingsRegistry::new();
        // Keep the returned handle alive for the registration to stay live.
        let _keep_alive = registry.register(a.as_reloadable());

        let mut policy = b.general();
        policy.on_open = true;
        b.set_override("uid-X", "/x/x.skrib", "X", policy).unwrap();

        assert!(
            !a.has_override("uid-X"),
            "no eager reload before dispatch fires"
        );
        let changed = registry.dispatch(&path).expect("dispatch should not error");
        assert!(changed, "dispatch must report a's state actually changed");
        assert!(
            a.has_override("uid-X"),
            "registry dispatch must apply the peer's write to a's live state"
        );
    }
}
