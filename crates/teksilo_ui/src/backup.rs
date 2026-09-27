// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The backup feature: business logic, plus the banner/choice/list views.
//!
//! Four view-models: [`BackupSchedulerViewModel`] drives every automatic + manual backup
//! trigger and is the one place that starts a `backup_now` long operation;
//! [`BackupRestoreViewModel`] is "restore this project to this backup", reusing `save_as`
//! under an atomic-replace swap; [`BackupSettingsViewModel`] is a cloneable handle over the
//! persisted policy (general + per-project overrides); [`BackupsListViewModel`] finds,
//! opens, reveals and deletes the backup **files** for the open project, distinct from the
//! destinations `BackupSettingsViewModel` configures. [`banner`]/[`choice_panel`]/
//! [`list_panel`] are the views: the permanent warning strip shown while a backup file is
//! open, the modal offering to open-or-restore it, and the browsable list of backup files.
//! [`is_backup_path`]/[`BackupContext`]/[`is_destination_available`] are the shared plumbing
//! underneath all four: open-a-backup detection and destination reachability.

mod backup_restore_vm;
mod backup_scheduler_vm;
mod backup_settings_vm;
mod backups_list_vm;
pub(crate) mod banner;
pub(crate) mod choice_panel;
pub(crate) mod list_panel;

use std::path::Path;

use skrib_format::sniff_backup;

pub use backup_restore_vm::BackupRestoreViewModel;
pub use backup_scheduler_vm::{BackupSchedulerViewModel, SafetyBlocker};
pub use backup_settings_vm::BackupSettingsViewModel;
pub use backups_list_vm::{BackupRow, BackupsListViewModel};

/// State carried while a **backup file** is open in this window: enough to drive
/// the permanent banner and the restore flow. Held in `App`'s `backup_context`
/// signal (`Some` ⇒ this window shows a backup in "backup mode").
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackupContext {
    /// This loaded project's own on-disk path (the backup file).
    pub path: String,
    /// The original project's path (from the manifest, or a filename guess).
    pub backup_of: Option<String>,
    /// The backup's creation timestamp (RFC3339), when the manifest carried it.
    pub backup_created_at: Option<String>,
    /// `true` when the manifest's marker decided it; `false` for a filename guess.
    pub authoritative: bool,
}

impl BackupContext {
    /// When the backup was taken, as the writer reads it: on their own clock.
    ///
    /// The manifest keeps the moment as RFC 3339 text in UTC, and that text used
    /// to reach the screen as stored ("Backup taken
    /// 2026-01-01T12:00:00.482913+00:00."). Text that does not parse is shown as
    /// it is, since it is still the only account of the moment there is.
    /// `None` when the manifest recorded no moment at all.
    pub fn taken_label(&self) -> Option<String> {
        let raw = self.backup_created_at.as_deref()?;
        Some(crate::shared::stamps::iso_stamp_from_rfc3339(raw).unwrap_or_else(|| raw.to_string()))
    }
}

/// Is `path` a backup file? (Authoritative manifest marker, or the filename
/// fallback for pre-marker backups — see `skrib_format::sniff_backup`.)
pub fn is_backup_path(path: &str) -> bool {
    sniff_backup(path).is_backup
}

/// The [`BackupContext`] for `path` if it is a backup, else `None`.
pub fn backup_context_for(path: &str) -> Option<BackupContext> {
    let s = sniff_backup(path);
    if !s.is_backup {
        return None;
    }
    Some(BackupContext {
        path: path.to_string(),
        backup_of: s.backup_of,
        backup_created_at: s.backup_created_at,
        authoritative: s.authoritative,
    })
}

/// Is `dir` a usable backup destination *right now* — an existing, writable
/// directory?
///
/// An empty string means "next to the project" (the default destination), which
/// is considered available whenever a project is open. A path that does not
/// exist (e.g. an unplugged USB drive / unmounted share) is unavailable, which is
/// exactly what the settings badge and the on-close "no destination" check key on.
///
/// Writability is inferred from directory metadata (side-effect-free); it is a
/// best-effort signal, not a guarantee the eventual write succeeds — the backup
/// engine still reports a genuine write failure per destination.
pub fn is_destination_available(dir: &str) -> bool {
    if dir.trim().is_empty() {
        return true; // "next to the project" — reachable while the project is open
    }
    match std::fs::metadata(Path::new(dir)) {
        Ok(meta) => meta.is_dir() && !meta.permissions().readonly(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn existing_dir_is_available() {
        let d = tempdir().unwrap();
        assert!(is_destination_available(d.path().to_str().unwrap()));
    }

    #[test]
    fn missing_dir_is_unavailable() {
        let d = tempdir().unwrap();
        let missing = d.path().join("not-there");
        assert!(!is_destination_available(missing.to_str().unwrap()));
    }

    #[test]
    fn a_file_is_not_a_valid_destination() {
        let d = tempdir().unwrap();
        let f = d.path().join("f.txt");
        std::fs::write(&f, b"x").unwrap();
        assert!(!is_destination_available(f.to_str().unwrap()));
    }

    #[test]
    fn empty_is_the_default_destination() {
        assert!(is_destination_available(""));
    }

    fn context_taken_at(stamp: Option<&str>) -> BackupContext {
        BackupContext {
            path: "/b/novel-20260303-233000.skrib".into(),
            backup_of: Some("/b/novel.skrib".into()),
            backup_created_at: stamp.map(str::to_string),
            authoritative: true,
        }
    }

    /// The backup choice card's "Backup taken …" used to print the manifest's
    /// RFC 3339 text verbatim: UTC, with its offset, seconds and fraction.
    #[test]
    fn a_backup_says_when_it_was_taken_on_the_writers_clock() {
        use crate::shared::stamps::{Zone, override_writer_zone};
        let _tokyo = override_writer_zone(Zone::tokyo());
        assert_eq!(
            context_taken_at(Some("2026-03-03T23:30:41.482913+00:00"))
                .taken_label()
                .as_deref(),
            Some("2026-03-04 08:30"),
        );
    }

    #[test]
    fn a_stamp_that_does_not_parse_is_still_shown_and_a_missing_one_is_not() {
        assert_eq!(
            context_taken_at(Some("last Tuesday"))
                .taken_label()
                .as_deref(),
            Some("last Tuesday"),
            "the only account of the moment there is",
        );
        assert_eq!(context_taken_at(None).taken_label(), None);
    }
}
