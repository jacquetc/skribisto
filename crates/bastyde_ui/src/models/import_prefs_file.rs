// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet
//! Where each project's last import landed.
//!
//! A `SettingsFile<ImportPrefsFile>` at `<config_dir>/import_prefs.toml`, holding one
//! row per project (keyed by `Work.unique_id`) carrying the destination its last
//! document import was applied to.
//!
//! **Per project, unlike its folder-memory neighbour.** Where a writer keeps their
//! source documents is a habit that spans every book (`folder_memory_file`); *which
//! chapter an import lands in* is a fact about one manuscript, and offering the last
//! project's answer in this one would be worse than offering nothing.
//!
//! **A `BinderTreeKey`, never a store id or a position.** `EntityId` is re-minted by
//! every `load_work` and a position moves whenever anything above it does, so either
//! would restore onto whatever row happened to inherit the number — silently importing
//! into the wrong chapter, which is the one mistake this feature could make that a
//! writer might not notice until much later. The key is a durable uid and already
//! derives `Serialize`, so it persists as itself.
//!
//! **Its own file rather than a section of another.** The desk layout, the expand state
//! and this each have their own read/write cadence, and a separate file keeps a corrupt
//! or future-schema'd blob on one side from quarantining the others — the reasoning
//! `tree_expansion_file` gives for the same choice.
//!
//! **Single implementation (no real/mock seam)** — app configuration, not backend data.

use std::time::Duration;

use bastyde::settings::{AppPaths, Migrator, SettingsFile, SettingsFileError, Versioned};
use serde::{Deserialize, Serialize};

use super::BinderTreeKey;

/// Accepted for call-site stability only; `SettingsFile`'s writes are a synchronous
/// locked read-modify-write with no debounce (see the siblings).
const SETTINGS_DEBOUNCE: Duration = Duration::from_millis(500);

/// Cap on remembered projects, evicted oldest-first by touch order. The same backstop
/// `tree_expansion_file` carries, and for the same reasons: deleted projects, and
/// legacy uid-less `.skrib`s that mint a fresh `unique_id` on every open.
const MAX_PROJECTS: usize = 128;

/// One project's remembered import destination.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PerProjectImportPrefs {
    /// `Work.unique_id` — the key. A `String`, not a `Uuid`, because that field is one.
    pub work_uid: String,
    /// The row the last import landed on, as a durable key.
    pub destination: Option<BinderTreeKey>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImportPrefsFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub projects: Vec<PerProjectImportPrefs>,
}

fn default_version() -> u32 {
    ImportPrefsFile::CURRENT_VERSION
}

impl Default for ImportPrefsFile {
    fn default() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            projects: Vec::new(),
        }
    }
}

impl Versioned for ImportPrefsFile {
    const CURRENT_VERSION: u32 = 1;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// Remembered-import-destination service. `SettingsFile` is `Clone` (it shares the
/// in-memory state + writer), so cloning hands out views over the same live file.
#[derive(Clone)]
pub struct ImportPrefsService {
    file: SettingsFile<ImportPrefsFile>,
}

impl ImportPrefsService {
    /// Open `import_prefs.toml` under `paths` (cross-process safe).
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        Self::open_with_delay(paths, SETTINGS_DEBOUNCE)
    }

    /// `delay` is accepted for call-site stability but has no effect.
    pub fn open_with_delay(paths: &AppPaths, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("import_prefs"), Migrator::new())?;
        Ok(Self { file })
    }

    /// Open at an explicit path. Test-only.
    #[cfg(test)]
    pub fn open_at(path: std::path::PathBuf) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, Migrator::new())?;
        Ok(Self { file })
    }

    /// Graceful fallback when the config dir is unavailable — the wizard just opens with
    /// nothing chosen, exactly as it did before this existed.
    pub fn in_memory_default() -> Self {
        let file =
            super::backup_settings_file::in_memory_settings_file("import-prefs", Migrator::new());
        Self { file }
    }

    /// Where this project's last import landed, if anything is remembered.
    pub fn last_destination(&self, work_uid: &str) -> Option<BinderTreeKey> {
        if !super::uid_is_usable(work_uid) {
            return None;
        }
        self.file
            .borrow()
            .projects
            .iter()
            .find(|p| p.work_uid == work_uid)
            .and_then(|p| p.destination)
    }

    /// Record where an import just landed.
    ///
    /// Guarded on `uid_is_usable`: a project that has never been saved has no
    /// `unique_id`, and keying by `""` would make every such project share one answer.
    pub fn remember_destination(&self, work_uid: &str, destination: BinderTreeKey) {
        if !super::uid_is_usable(work_uid) {
            return;
        }
        let _ = self.file.mutate(|f| {
            f.version = ImportPrefsFile::CURRENT_VERSION;
            // Moved to the end before the cap is applied, so the project being used is
            // never the one evicted — the touch-order rule `tree_expansion_file` uses.
            if let Some(i) = f.projects.iter().position(|p| p.work_uid == work_uid) {
                let mut row = f.projects.remove(i);
                row.destination = Some(destination);
                f.projects.push(row);
            } else {
                f.projects.push(PerProjectImportPrefs {
                    work_uid: work_uid.to_string(),
                    destination: Some(destination),
                });
            }
            if f.projects.len() > MAX_PROJECTS {
                let excess = f.projects.len() - MAX_PROJECTS;
                f.projects.drain(0..excess);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service() -> (ImportPrefsService, tempfile::TempDir) {
        let dir = tempfile::tempdir().expect("tempdir");
        let svc = ImportPrefsService::open_at(dir.path().join("import_prefs.toml"))
            .expect("open settings");
        (svc, dir)
    }

    fn key(n: u128) -> BinderTreeKey {
        BinderTreeKey::Item(uuid::Uuid::from_u128(n))
    }

    #[test]
    fn a_project_with_no_import_yet_remembers_nothing() {
        let (svc, _d) = service();
        assert_eq!(svc.last_destination("project-a"), None);
    }

    /// The round trip that matters: a durable key must come back as itself, including
    /// which *kind* of row it named — a binder and an item can share a uuid without
    /// being the same thing.
    #[test]
    fn a_destination_survives_a_reopen_as_the_same_row() {
        let (svc, d) = service();
        svc.remember_destination("project-a", key(7));
        svc.remember_destination("project-b", BinderTreeKey::Binder(uuid::Uuid::from_u128(7)));

        let reopened = ImportPrefsService::open_at(d.path().join("import_prefs.toml"))
            .expect("reopen settings");
        assert_eq!(reopened.last_destination("project-a"), Some(key(7)));
        assert_eq!(
            reopened.last_destination("project-b"),
            Some(BinderTreeKey::Binder(uuid::Uuid::from_u128(7))),
            "a binder row and an item row with the same uuid are different destinations"
        );
    }

    #[test]
    fn importing_again_replaces_the_projects_answer() {
        let (svc, _d) = service();
        svc.remember_destination("project-a", key(1));
        svc.remember_destination("project-a", key(2));

        assert_eq!(svc.last_destination("project-a"), Some(key(2)));
        assert_eq!(
            svc.file.borrow().projects.len(),
            1,
            "one project is one row, however many times it is imported into"
        );
    }

    /// An unsaved project has no `unique_id`, and keying by `""` would hand every such
    /// project the same destination.
    #[test]
    fn a_project_with_no_uid_is_neither_written_nor_read() {
        let (svc, _d) = service();
        svc.remember_destination("", key(1));
        assert_eq!(svc.last_destination(""), None);
        assert!(
            svc.file.borrow().projects.is_empty(),
            "an unusable uid must not create a row at all"
        );
    }

    /// Bounded growth, with the row in use protected: the project just imported into is
    /// the one most likely to be imported into again.
    #[test]
    fn the_project_in_use_is_never_the_one_evicted() {
        let (svc, _d) = service();
        for i in 0..MAX_PROJECTS {
            svc.remember_destination(&format!("project-{i}"), key(i as u128));
        }
        // Touch the oldest, then push the cap over by one.
        svc.remember_destination("project-0", key(999));
        svc.remember_destination("one-more", key(1000));

        assert_eq!(svc.file.borrow().projects.len(), MAX_PROJECTS);
        assert_eq!(
            svc.last_destination("project-0"),
            Some(key(999)),
            "the project just used must survive the cap"
        );
        assert_eq!(
            svc.last_destination("project-1"),
            None,
            "the genuinely oldest row is the one that goes"
        );
    }
}
