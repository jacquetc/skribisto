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
//! (which serialises unit enums as plain strings); `BackupPolicy::retention()`
//! rebuilds the `skrib_format::retention::RetentionPolicy` the engine consumes.

use std::time::Duration;

use bastyde::settings::{AppPaths, Migrator, SettingsFile, SettingsFileError, Versioned};
use serde::{Deserialize, Serialize};
use skrib_format::retention::RetentionPolicy;

/// Debounce for backup-settings writes (same default as the framework's stores).
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
    /// Destination directories. Empty ⇒ back up next to the project.
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
        // The user-confirmed defaults: back up on close, tiered retention.
        BackupPolicy {
            on_close: true,
            on_open: false,
            interval_enabled: false,
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
    /// The `skrib_format` retention policy this configuration resolves to.
    pub fn retention(&self) -> RetentionPolicy {
        match self.retention_mode {
            RetentionMode::KeepLastN => RetentionPolicy::KeepLastN {
                n: self.keep_last_n,
            },
            RetentionMode::Tiered => RetentionPolicy::Gfs {
                hourly: self.gfs_hourly,
                daily: self.gfs_daily,
                weekly: self.gfs_weekly,
                monthly: self.gfs_monthly,
            },
        }
    }

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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    /// Open `backup.toml` under `paths`, default debounce.
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        Self::open_with_delay(paths, SETTINGS_DEBOUNCE)
    }

    pub fn open_with_delay(paths: &AppPaths, delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("backup"), delay, &Migrator::new())?;
        Ok(Self { file })
    }

    /// Open at an explicit path — used by tests.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn open_at(path: std::path::PathBuf, delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, delay, &Migrator::new())?;
        Ok(Self { file })
    }

    /// Graceful fallback when the config dir is unavailable: a throwaway
    /// per-process temp file, so the app still runs (backups just won't persist
    /// their settings across restarts).
    pub fn in_memory_default() -> Self {
        let path =
            std::env::temp_dir().join(format!("skribisto-backup-{}.toml", std::process::id()));
        SettingsFile::load(path, Duration::from_millis(500), &Migrator::new())
            .map(|file| Self { file })
            .unwrap_or_else(|_| {
                // Even the temp path failed; use a last-ditch in-cwd name.
                let file = SettingsFile::load(
                    std::path::PathBuf::from(".skribisto-backup.toml"),
                    Duration::from_millis(500),
                    &Migrator::new(),
                )
                .expect("in-memory backup settings fallback");
                Self { file }
            })
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

    pub fn record_destination_success(
        &self,
        work_uid: &str,
        last_path: &str,
        dir: &str,
        hash: &str,
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
                d.last_success_at = at.to_string();
            } else {
                ps.destination_states.push(DestinationState {
                    directory: dir.to_string(),
                    last_success_hash: hash.to_string(),
                    last_success_at: at.to_string(),
                });
            }
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn svc(dir: &std::path::Path) -> BackupSettingsService {
        BackupSettingsService::open_at(dir.join("backup.toml"), Duration::ZERO).unwrap()
    }

    #[test]
    fn defaults_are_on_close_tiered() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        let g = s.general();
        assert!(g.on_close && !g.on_open && !g.interval_enabled);
        assert_eq!(g.retention_mode, RetentionMode::Tiered);
        assert_eq!(g.min_keep, 3);
        assert!(matches!(
            g.retention(),
            RetentionPolicy::Gfs { monthly: 12, .. }
        ));
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
            s.effective_for("uid-A").retention(),
            RetentionPolicy::KeepLastN { n: 5 }
        );
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
                "2026-06-01T10:00:00Z",
            )
            .unwrap();
            s.record_destination_success(
                "uid-A",
                "/x/a.skrib",
                "/usb",
                "hash2",
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
            s.last_backup_at("uid-A").as_deref(),
            Some("2026-06-02T10:00:00Z")
        );
        assert!(s.was_nudged("uid-A"));
    }
}
