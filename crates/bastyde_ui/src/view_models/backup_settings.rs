//! `BackupSettingsViewModel` — a cloneable handle over [`BackupSettingsService`].
//!
//! Registered as `app_state`; the settings panes and the backup scheduler each
//! hold a clone. Every clone shares the same underlying `SettingsFile`, so a
//! change made in one is visible to all. It stays a thin wrapper (the service
//! does the persistence) plus small conveniences for the two callers.

use crate::models::{BackupPolicy, BackupSettingsService};

#[derive(Clone)]
pub struct BackupSettingsViewModel {
    service: BackupSettingsService,
}

impl BackupSettingsViewModel {
    pub fn new(service: BackupSettingsService) -> Self {
        Self { service }
    }

    /// The underlying service (read-only queries the scheduler may prefer direct).
    pub fn service(&self) -> &BackupSettingsService {
        &self.service
    }

    // ── general policy ──
    pub fn general(&self) -> BackupPolicy {
        self.service.general()
    }

    pub fn set_general(&self, policy: BackupPolicy) {
        if let Err(e) = self.service.set_general(policy) {
            eprintln!("backup settings: set_general failed: {e}");
        }
    }

    // ── per-project overrides ──
    pub fn has_override(&self, uid: &str) -> bool {
        self.service.has_override(uid)
    }

    pub fn effective_for(&self, uid: &str) -> BackupPolicy {
        self.service.effective_for(uid)
    }

    pub fn set_override(&self, uid: &str, last_path: &str, title: &str, policy: BackupPolicy) {
        if let Err(e) = self.service.set_override(uid, last_path, title, policy) {
            eprintln!("backup settings: set_override failed: {e}");
        }
    }

    pub fn clear_override(&self, uid: &str) {
        if let Err(e) = self.service.clear_override(uid) {
            eprintln!("backup settings: clear_override failed: {e}");
        }
    }

    // ── bookkeeping (dedup + "last backup" + nudge) ──
    pub fn last_backup_at(&self, uid: &str) -> Option<String> {
        self.service.last_backup_at(uid)
    }

    pub fn record_destination_success(
        &self,
        uid: &str,
        last_path: &str,
        dir: &str,
        hash: &str,
        backup_path: &str,
        at: &str,
    ) {
        if let Err(e) =
            self.service
                .record_destination_success(uid, last_path, dir, hash, backup_path, at)
        {
            eprintln!("backup settings: record_destination_success failed: {e}");
        }
    }

    pub fn was_nudged(&self, uid: &str) -> bool {
        self.service.was_nudged(uid)
    }

    pub fn mark_nudged(&self, uid: &str, last_path: &str) {
        if let Err(e) = self.service.mark_nudged(uid, last_path) {
            eprintln!("backup settings: mark_nudged failed: {e}");
        }
    }

    pub fn flush_now(&self) {
        if let Err(e) = self.service.flush_now() {
            eprintln!("backup settings: flush failed: {e}");
        }
    }
}
