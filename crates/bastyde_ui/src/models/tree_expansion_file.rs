// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet
//! Remembered expand/collapse state for the app's binder trees.
//!
//! A `SettingsFile<TreeExpansionFile>` at `<config_dir>/tree_expansion.toml`, holding one
//! row per project (keyed by `Work.unique_id`), each holding one entry per container whose
//! Overview table the writer has expanded or collapsed.
//!
//! **Why a durable uid and not a store id.** The chevrons a writer sets are worth keeping
//! precisely *because* they outlive the session, and nothing else about a `BinderItem`
//! does: `EntityId` is a position in an ephemeral `HashMap` that `load_work` re-mints on
//! every open, and a `.skrib`'s `file_id`s are only those ids at save time. Keyed by
//! either, a remembered set would restore onto whatever items happened to inherit those
//! numbers — silently expanding the wrong chapters. `BinderItem.uid` (`.skrib` v3) is the
//! only identity that survives a save → load, which is why B0 added it before this.
//!
//! **A fourth sibling**, alongside `workspace_layout_file` / `backup_settings_file` /
//! `search_settings_file`, and deliberately *not* another section inside `workspace.toml`:
//! the desk layout is written once per project-leave and read once per project-open, while
//! this is written per project-leave but read per *container tab opened*. Separate files
//! keep a corrupt or future-schema'd blob on one side from quarantining the other.
//!
//! **Single implementation (no real/mock seam)** — it is a config file, not backend data,
//! exactly like the three siblings.

use std::time::Duration;

use bastyde::settings::{AppPaths, Migrator, SettingsFile, SettingsFileError, Versioned};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Accepted for call-site stability only; `SettingsFile`'s writes are a synchronous
/// locked read-modify-write with no debounce (see the siblings).
const SETTINGS_DEBOUNCE: Duration = Duration::from_millis(500);

/// Cap on remembered projects — a backstop against unbounded growth from deleted projects
/// and from legacy uid-less `.skrib`s that mint a fresh `unique_id` on every open.
/// Newest kept, oldest evicted. Mirrors `workspace_layout_file`.
const MAX_PROJECTS: usize = 128;

/// Cap on remembered containers per project. A writer works in a handful of Books, Parts
/// and chapter folders; well past that the oldest entries are stale by definition.
const MAX_FOLDERS_PER_PROJECT: usize = 64;

/// Cap on remembered expanded rows inside one container. A 4096-row expanded subtree is
/// already past what any table is usefully showing, and the cap bounds one project's row
/// at roughly 150 kB of uuids rather than letting a pathological manuscript grow it
/// without limit.
const MAX_EXPANDED_PER_FOLDER: usize = 4096;

/// One container's remembered expand set.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FolderExpansionState {
    /// The container whose Overview this describes (`BinderItem.uid`).
    pub container_uid: Uuid,
    /// The rows expanded inside it, by `BinderItem.uid`.
    #[serde(default)]
    pub expanded_uids: Vec<Uuid>,
}

/// One project's remembered expand state.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PerProjectTreeExpansion {
    /// `Work.unique_id` — the key. A `String`, not a `Uuid`, because that field still is
    /// one (unlike the row-level uids this file otherwise stores).
    pub work_uid: String,
    /// Display/debug only; the key is [`Self::work_uid`].
    #[serde(default)]
    pub last_path: String,
    /// One entry per container whose Overview has remembered state.
    #[serde(default)]
    pub folders: Vec<FolderExpansionState>,
    /// The **outline** dock's expanded rows — binder rows and item rows in one flat set,
    /// which is why `Binder` needed a uid of its own alongside `BinderItem`.
    ///
    /// Not keyed by container like [`Self::folders`]: the outline is one tree over the
    /// whole project, not one per container.
    #[serde(default)]
    pub outline_expanded: Vec<crate::models::BinderTreeKey>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TreeExpansionFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub projects: Vec<PerProjectTreeExpansion>,
}

fn default_version() -> u32 {
    TreeExpansionFile::CURRENT_VERSION
}

impl Default for TreeExpansionFile {
    fn default() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            projects: Vec::new(),
        }
    }
}

impl Versioned for TreeExpansionFile {
    const CURRENT_VERSION: u32 = 1;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// Persistent tree-expansion service. `SettingsFile` is `Clone` (it shares the in-memory
/// state + writer), so cloning hands out views over the same live file.
#[derive(Clone)]
pub struct TreeExpansionService {
    file: SettingsFile<TreeExpansionFile>,
}

impl TreeExpansionService {
    /// Open `tree_expansion.toml` under `paths` (cross-process safe).
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        Self::open_with_delay(paths, SETTINGS_DEBOUNCE)
    }

    /// `delay` is accepted for call-site stability but has no effect.
    pub fn open_with_delay(paths: &AppPaths, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("tree_expansion"), Migrator::new())?;
        Ok(Self { file })
    }

    /// Open at an explicit path. Test-only: production reaches the file through
    /// [`open`](Self::open) (the config dir) or [`in_memory_default`](Self::in_memory_default).
    #[cfg(test)]
    pub fn open_at(path: std::path::PathBuf, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, Migrator::new())?;
        Ok(Self { file })
    }

    /// Graceful fallback when the config dir is unavailable: a throwaway per-process temp
    /// file, so the app still runs (the chevrons just won't survive a restart). Mirrors
    /// [`WorkspaceLayoutService::in_memory_default`](super::WorkspaceLayoutService) —
    /// which is also why every call site can register this unconditionally rather than
    /// carrying an `Option` around.
    pub fn in_memory_default() -> Self {
        let path = std::env::temp_dir().join(format!(
            "skribisto-tree-expansion-{}.toml",
            std::process::id()
        ));
        SettingsFile::load(path, Migrator::new())
            .map(|file| Self { file })
            .unwrap_or_else(|_| {
                let file = SettingsFile::load(
                    std::path::PathBuf::from(".skribisto-tree-expansion.toml"),
                    Migrator::new(),
                )
                .expect("in-memory tree expansion fallback");
                Self { file }
            })
    }

    /// The remembered expand set for one container, or empty when there is none.
    pub fn expanded(&self, work_uid: &str, container_uid: Uuid) -> Vec<Uuid> {
        if !super::uid_is_usable(work_uid) {
            return Vec::new();
        }
        let f = self.file.borrow();
        f.projects
            .iter()
            .find(|p| p.work_uid == work_uid)
            .and_then(|p| {
                p.folders
                    .iter()
                    .find(|d| d.container_uid == container_uid)
                    .map(|d| d.expanded_uids.clone())
            })
            .unwrap_or_default()
    }

    /// Record the expand state of every container captured in one go.
    ///
    /// **One batched write per door**, not one per container and emphatically not one per
    /// chevron: `SettingsFile::mutate` is a synchronous locked read-modify-write, and a
    /// writer expanding their way down a book would otherwise rewrite the whole file on
    /// every click.
    ///
    /// A container that reports an **empty** set is removed rather than stored, so
    /// "collapsed everything" costs no row and a project whose folders are all collapsed
    /// eventually drops out of the file entirely.
    pub fn set_folders(
        &self,
        work_uid: &str,
        last_path: &str,
        folders: &[(Uuid, Vec<Uuid>)],
    ) -> Result<(), SettingsFileError> {
        if !super::uid_is_usable(work_uid) {
            return Ok(());
        }
        self.file.mutate(|f| {
            f.version = TreeExpansionFile::CURRENT_VERSION;
            let pos = match f.projects.iter().position(|p| p.work_uid == work_uid) {
                Some(pos) => pos,
                None => {
                    f.projects.push(PerProjectTreeExpansion {
                        work_uid: work_uid.to_string(),
                        ..Default::default()
                    });
                    f.projects.len() - 1
                }
            };
            let row = &mut f.projects[pos];
            row.last_path = last_path.to_string();
            for (container_uid, expanded) in folders {
                let existing = row
                    .folders
                    .iter()
                    .position(|d| d.container_uid == *container_uid);
                if expanded.is_empty() {
                    if let Some(i) = existing {
                        row.folders.remove(i);
                    }
                    continue;
                }
                let mut expanded = expanded.clone();
                expanded.truncate(MAX_EXPANDED_PER_FOLDER);
                match existing {
                    Some(i) => row.folders[i].expanded_uids = expanded,
                    None => row.folders.push(FolderExpansionState {
                        container_uid: *container_uid,
                        expanded_uids: expanded,
                    }),
                }
            }
            let len = row.folders.len();
            if len > MAX_FOLDERS_PER_PROJECT {
                row.folders.drain(0..len - MAX_FOLDERS_PER_PROJECT); // evict the oldest
            }
            // Touch-order eviction: the project just written is the newest, so move its
            // row to the end before capping — otherwise the cap would evict whichever
            // project happened to be added first, including the one in use.
            let row = f.projects.remove(pos);
            f.projects.push(row);
            let len = f.projects.len();
            if len > MAX_PROJECTS {
                f.projects.drain(0..len - MAX_PROJECTS);
            }
        })
    }

    /// The outline's remembered expanded rows for a project.
    pub fn outline(&self, work_uid: &str) -> Vec<crate::models::BinderTreeKey> {
        if !super::uid_is_usable(work_uid) {
            return Vec::new();
        }
        self.file
            .borrow()
            .projects
            .iter()
            .find(|p| p.work_uid == work_uid)
            .map(|p| p.outline_expanded.clone())
            .unwrap_or_default()
    }

    /// Record the outline's expanded rows.
    ///
    /// Separate from [`set_folders`](Self::set_folders) so a project with an outline but
    /// no open container tabs still persists, and vice versa — but both land in the same
    /// project row, so the file stays one row per project.
    pub fn set_outline(
        &self,
        work_uid: &str,
        last_path: &str,
        expanded: &[crate::models::BinderTreeKey],
    ) -> Result<(), SettingsFileError> {
        if !super::uid_is_usable(work_uid) {
            return Ok(());
        }
        let mut expanded = expanded.to_vec();
        expanded.truncate(MAX_EXPANDED_PER_FOLDER);
        self.file.mutate(|f| {
            f.version = TreeExpansionFile::CURRENT_VERSION;
            let pos = match f.projects.iter().position(|p| p.work_uid == work_uid) {
                Some(pos) => pos,
                None => {
                    f.projects.push(PerProjectTreeExpansion {
                        work_uid: work_uid.to_string(),
                        ..Default::default()
                    });
                    f.projects.len() - 1
                }
            };
            f.projects[pos].last_path = last_path.to_string();
            f.projects[pos].outline_expanded = expanded;
            // Touch order, as in `set_folders`: the row just written is the newest, so
            // the cap can never evict the project in use.
            let row = f.projects.remove(pos);
            f.projects.push(row);
            let len = f.projects.len();
            if len > MAX_PROJECTS {
                f.projects.drain(0..len - MAX_PROJECTS);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn svc(dir: &std::path::Path) -> TreeExpansionService {
        TreeExpansionService::open_at(dir.join("tree_expansion.toml"), Duration::ZERO).unwrap()
    }

    fn uid(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    #[test]
    fn a_captured_set_round_trips() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        s.set_folders("work-1", "/p.skrib", &[(uid(10), vec![uid(1), uid(2)])])
            .unwrap();
        assert_eq!(s.expanded("work-1", uid(10)), vec![uid(1), uid(2)]);
    }

    #[test]
    fn an_unknown_project_or_container_reads_as_empty() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        s.set_folders("work-1", "", &[(uid(10), vec![uid(1)])])
            .unwrap();
        assert!(s.expanded("other-work", uid(10)).is_empty());
        assert!(s.expanded("work-1", uid(999)).is_empty());
    }

    /// A blank `Work.unique_id` must not become a shared key — a brand-new unsaved project
    /// has none, and every one of them would otherwise read and write the same row.
    #[test]
    fn a_blank_work_uid_is_neither_written_nor_read() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        s.set_folders("", "/p.skrib", &[(uid(10), vec![uid(1)])])
            .unwrap();
        assert!(s.expanded("", uid(10)).is_empty());
        s.set_folders("  ", "", &[(uid(10), vec![uid(1)])]).unwrap();
        assert!(s.expanded("  ", uid(10)).is_empty());
    }

    /// Collapsing everything in a container drops its row rather than storing an empty
    /// vector — otherwise the file only ever grows.
    #[test]
    fn an_empty_set_removes_the_container_row() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        s.set_folders("w", "", &[(uid(10), vec![uid(1)])]).unwrap();
        s.set_folders("w", "", &[(uid(10), vec![])]).unwrap();
        assert!(s.expanded("w", uid(10)).is_empty());
        assert!(
            s.file.borrow().projects[0].folders.is_empty(),
            "the row is gone, not empty"
        );
    }

    /// Capturing several containers at once is one write, and each keeps its own set.
    #[test]
    fn containers_are_independent_within_a_project() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        s.set_folders(
            "w",
            "",
            &[(uid(10), vec![uid(1)]), (uid(20), vec![uid(2), uid(3)])],
        )
        .unwrap();
        assert_eq!(s.expanded("w", uid(10)), vec![uid(1)]);
        assert_eq!(s.expanded("w", uid(20)), vec![uid(2), uid(3)]);
    }

    /// A later capture replaces a container's set rather than appending to it — otherwise
    /// a collapsed row would stay "expanded" forever.
    #[test]
    fn recapturing_replaces_rather_than_merges() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        s.set_folders("w", "", &[(uid(10), vec![uid(1), uid(2)])])
            .unwrap();
        s.set_folders("w", "", &[(uid(10), vec![uid(3)])]).unwrap();
        assert_eq!(s.expanded("w", uid(10)), vec![uid(3)]);
    }

    #[test]
    fn the_per_container_expanded_set_is_capped() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        let many: Vec<Uuid> = (0..(MAX_EXPANDED_PER_FOLDER as u128 + 50))
            .map(uid)
            .collect();
        s.set_folders("w", "", &[(uid(9999), many)]).unwrap();
        assert_eq!(s.expanded("w", uid(9999)).len(), MAX_EXPANDED_PER_FOLDER);
    }

    #[test]
    fn containers_per_project_are_capped() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        for i in 0..(MAX_FOLDERS_PER_PROJECT as u128 + 20) {
            s.set_folders("w", "", &[(uid(i), vec![uid(1)])]).unwrap();
        }
        let f = s.file.borrow();
        assert_eq!(f.projects[0].folders.len(), MAX_FOLDERS_PER_PROJECT);
        // The newest survives; the oldest were evicted.
        assert!(
            f.projects[0]
                .folders
                .iter()
                .any(|d| d.container_uid == uid(MAX_FOLDERS_PER_PROJECT as u128 + 19))
        );
    }

    /// Projects are capped by **touch order**, so the project being written can never be
    /// the one evicted — the bug a naive "drain the front" cap has.
    #[test]
    fn projects_are_capped_newest_first() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        for i in 0..(MAX_PROJECTS + 20) {
            s.set_folders(&format!("w-{i}"), "", &[(uid(1), vec![uid(2)])])
                .unwrap();
        }
        assert_eq!(s.file.borrow().projects.len(), MAX_PROJECTS);
        assert_eq!(
            s.expanded(&format!("w-{}", MAX_PROJECTS + 19), uid(1)),
            vec![uid(2)],
            "the most recently written project survives the cap"
        );
    }
}
