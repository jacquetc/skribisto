// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Shared backup helpers for the UI layer: destination reachability, and the
//! open-a-backup detection (`is_backup_path` / `BackupContext`).

pub(crate) mod banner;
pub(crate) mod choice_panel;
pub(crate) mod list_panel;

use std::path::Path;

use skrib_format::sniff_backup;

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
}
