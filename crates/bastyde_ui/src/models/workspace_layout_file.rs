//! Per-work **workspace layout**: the open editor tabs (both panes + selection +
//! split) and the dock layout, so re-opening a project restores the desk exactly
//! as it was left.
//!
//! A `SettingsFile<WorkspaceLayoutFile>` at `<config_dir>/workspace.toml` holding
//! one entry per project, keyed by the project's stable **`Work.unique_id`** (not
//! its path — that survives rename/move, and is the same key `backup.toml` /
//! `search.toml` correlate on). The raw path is kept only for display/debug. This
//! is app configuration, orthogonal to the `.skrib` document, so — exactly like
//! [`SearchSettingsService`](super::SearchSettingsService) and
//! [`BackupSettingsService`](super::BackupSettingsService) — there is a **single
//! implementation (no real/mock seam)**: it opens a real `SettingsFile` in every
//! build.
//!
//! **Why ordinals, not item ids.** A tab is a `BinderItem`, but the backend store
//! is an ephemeral `HashMap` whose ids are **remapped on every load** (a `.skrib`
//! stores `file_id`s, `load_work` mints fresh store ids in binder order). Those
//! ids are not stable across a save→load cycle — an id can even shift when an
//! unrelated tag or dictionary word is added earlier in the load. So a tab is
//! persisted by its **position** in the work's ordered binder-item stream (its
//! flat, binder-major ordinal), which *does* round-trip: the save writes the
//! stream order, the load reproduces it, and the ordinal maps 1:1 to the new id.
//! See [`WorkspaceLayoutViewModel`](crate::view_models::WorkspaceLayoutViewModel)
//! for the capture/restore that translates ordinals ↔ store ids.
//!
//! **Cross-process safety.** Skribisto runs **one process per project**, and every
//! instance shares this same `workspace.toml`. Two instances only ever touch two
//! *different* projects' rows, and `SettingsFile`'s locked read-modify-write
//! ([`mutate`](bastyde::settings::SettingsFile::mutate)) keeps those writes from
//! clobbering each other. A process never needs another's live layout, so — unlike
//! `search.toml` — this file is not registered as a `Reloadable`.

use std::time::Duration;

use bastyde::settings::{AppPaths, Migrator, SettingsFile, SettingsFileError, Versioned};
use bastyde::widgets::{DockLayoutState, SplitterState};
use serde::{Deserialize, Serialize};

/// Accepted for call-site stability only. `SettingsFile`'s writes are a
/// synchronous locked read-modify-write — there is no debounce to configure
/// (see [`SearchSettingsService`](super::SearchSettingsService)).
const SETTINGS_DEBOUNCE: Duration = Duration::from_millis(500);

/// One editor pane's persisted tabs.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PaneLayout {
    /// The flat, binder-major ordinals of the open items, in tab order. An ordinal
    /// is the item's index in the work's ordered binder-item stream (see the module
    /// docs on why position rather than store id).
    #[serde(default)]
    pub tabs: Vec<usize>,
    /// The ordinal of the selected tab (one of [`Self::tabs`]), or `None` when the
    /// pane is empty.
    #[serde(default)]
    pub selected: Option<usize>,
}

impl PaneLayout {
    /// Nothing worth restoring (no open tabs).
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
}

/// One project's complete workspace layout.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PerProjectLayout {
    pub work_uid: String,
    /// Display/debug only (the key is [`Self::work_uid`]).
    #[serde(default)]
    pub last_path: String,
    /// The primary (always-present) editor pane.
    #[serde(default)]
    pub primary: PaneLayout,
    /// The secondary (side) editor pane.
    #[serde(default)]
    pub secondary: PaneLayout,
    /// The side pane was shown.
    #[serde(default)]
    pub split_active: bool,
    /// The secondary pane was the focused one (drives the open-item marker).
    #[serde(default)]
    pub focus_secondary: bool,
    /// The two editor panes' splitter ratio, if the split was shown.
    #[serde(default)]
    pub editor_splitter: Option<SplitterState>,
    /// The dock layout (per-side size / presentation / selected-tab / arrangement /
    /// corners). `None` for a legacy entry written before docks were persisted, or
    /// a blob this build can't read (see [`lenient_docks`]).
    #[serde(default, deserialize_with = "lenient_docks")]
    pub docks: Option<DockLayoutState>,
}

/// Deserialize the embedded [`DockLayoutState`] **tolerantly**: on any error, drop
/// it to `None` rather than failing the whole `WorkspaceLayoutFile`.
///
/// `DockLayoutState` is an *external*, independently-`Versioned` type with its own
/// migrator — but embedded here as a plain field it bypasses that migrator (only
/// the outer file's version is walked). Serde fails a whole container if any one
/// element fails, so without this a future non-additive `DockLayoutState` schema
/// change (its own tests show a required-field rename is a real precedent) would
/// fail-load *every* project's row, silently disabling all workspace persistence
/// until the file is hand-fixed. Parsing through a `toml::Value` and swallowing the
/// error contains the blast radius to that one project's dock layout — its tabs and
/// every other project's desk still load.
fn lenient_docks<'de, D>(d: D) -> Result<Option<DockLayoutState>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = toml::Value::deserialize(d)?;
    Ok(DockLayoutState::deserialize(value).ok())
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WorkspaceLayoutFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub projects: Vec<PerProjectLayout>,
}

fn default_version() -> u32 {
    WorkspaceLayoutFile::CURRENT_VERSION
}

impl Default for WorkspaceLayoutFile {
    fn default() -> Self {
        WorkspaceLayoutFile {
            version: WorkspaceLayoutFile::CURRENT_VERSION,
            projects: Vec::new(),
        }
    }
}

impl Versioned for WorkspaceLayoutFile {
    const CURRENT_VERSION: u32 = 1;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// Persistent workspace-layout service. `SettingsFile` is `Clone` (shares the
/// in-memory state + writer), so cloning hands out views over the same live file.
#[derive(Clone)]
pub struct WorkspaceLayoutService {
    file: SettingsFile<WorkspaceLayoutFile>,
}

impl WorkspaceLayoutService {
    /// Open `workspace.toml` under `paths` (cross-process safe).
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        Self::open_with_delay(paths, SETTINGS_DEBOUNCE)
    }

    /// `delay` is accepted for call-site stability but has no effect (writes are a
    /// synchronous locked read-modify-write, no debounce).
    pub fn open_with_delay(paths: &AppPaths, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("workspace"), Migrator::new())?;
        Ok(Self { file })
    }

    /// Open at an explicit path — used by tests.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn open_at(path: std::path::PathBuf, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, Migrator::new())?;
        Ok(Self { file })
    }

    /// Graceful fallback when the config dir is unavailable: a throwaway per-process
    /// temp file, so the app still runs (the layout just won't persist across
    /// restarts) — mirrors [`SearchSettingsService::in_memory_default`](super::SearchSettingsService).
    pub fn in_memory_default() -> Self {
        let path =
            std::env::temp_dir().join(format!("skribisto-workspace-{}.toml", std::process::id()));
        SettingsFile::load(path, Migrator::new())
            .map(|file| Self { file })
            .unwrap_or_else(|_| {
                let file = SettingsFile::load(
                    std::path::PathBuf::from(".skribisto-workspace.toml"),
                    Migrator::new(),
                )
                .expect("in-memory workspace layout fallback");
                Self { file }
            })
    }

    /// This project's saved layout, if any.
    pub fn get(&self, work_uid: &str) -> Option<PerProjectLayout> {
        if !super::uid_is_usable(work_uid) {
            return None;
        }
        self.file
            .borrow()
            .projects
            .iter()
            .find(|p| p.work_uid == work_uid)
            .cloned()
    }

    /// Persist `layout` as `layout.work_uid`'s entry (upsert). A no-op when the uid
    /// is empty (a brand-new unsaved project has no uid yet — see
    /// [`super::uid_is_usable`]): keying `""` would collide across unrelated
    /// projects.
    pub fn set(&self, layout: PerProjectLayout) -> Result<(), SettingsFileError> {
        if !super::uid_is_usable(&layout.work_uid) {
            return Ok(());
        }
        self.file.mutate(|f| {
            if let Some(pos) = f.projects.iter().position(|p| p.work_uid == layout.work_uid) {
                f.projects[pos] = layout;
            } else {
                f.projects.push(layout);
            }
        })
    }

    /// Drop `work_uid`'s entry (project deleted / no longer wanted). No-op if absent.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn forget(&self, work_uid: &str) -> Result<(), SettingsFileError> {
        self.file.mutate(|f| f.projects.retain(|p| p.work_uid != work_uid))
    }

    pub fn flush_now(&self) -> Result<(), SettingsFileError> {
        self.file.flush_now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn svc(dir: &std::path::Path) -> WorkspaceLayoutService {
        WorkspaceLayoutService::open_at(dir.join("workspace.toml"), Duration::ZERO).unwrap()
    }

    fn sample(uid: &str) -> PerProjectLayout {
        PerProjectLayout {
            work_uid: uid.to_string(),
            last_path: "/x/a.skrib".to_string(),
            primary: PaneLayout {
                tabs: vec![3, 7, 1],
                selected: Some(7),
            },
            secondary: PaneLayout {
                tabs: vec![9],
                selected: Some(9),
            },
            split_active: true,
            focus_secondary: false,
            editor_splitter: None,
            docks: Some(DockLayoutState::default()),
        }
    }

    #[test]
    fn round_trips_through_toml() {
        let d = tempdir().unwrap();
        {
            let s = svc(d.path());
            s.set(sample("uid-A")).unwrap();
            s.flush_now().unwrap();
        }
        // Reopen from disk — the layout survived the TOML round-trip.
        let s = svc(d.path());
        let got = s.get("uid-A").expect("entry present");
        assert_eq!(got.primary.tabs, vec![3, 7, 1]);
        assert_eq!(got.primary.selected, Some(7));
        assert_eq!(got.secondary.tabs, vec![9]);
        assert!(got.split_active);
        assert!(got.docks.is_some());
    }

    #[test]
    fn upsert_replaces_and_isolates_per_uid() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        s.set(sample("uid-A")).unwrap();
        s.set(sample("uid-B")).unwrap();

        // Overwrite A with a different desk.
        let mut a2 = sample("uid-A");
        a2.primary.tabs = vec![42];
        a2.primary.selected = Some(42);
        s.set(a2).unwrap();

        assert_eq!(s.get("uid-A").unwrap().primary.tabs, vec![42]);
        assert_eq!(s.get("uid-B").unwrap().primary.tabs, vec![3, 7, 1], "B untouched");
        assert_eq!(s.file.borrow().projects.len(), 2, "no duplicate row for A");
    }

    #[test]
    fn an_empty_uid_never_persists() {
        // A brand-new unsaved project has no unique_id; keying "" would collide
        // across unrelated projects, so a set on it is silently dropped.
        let d = tempdir().unwrap();
        let s = svc(d.path());
        s.set(sample("")).unwrap();
        assert!(s.get("").is_none(), "empty uid must not persist");
        assert!(s.file.borrow().projects.is_empty());
    }

    #[test]
    fn forget_removes_only_the_named_entry() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        s.set(sample("uid-A")).unwrap();
        s.set(sample("uid-B")).unwrap();
        s.forget("uid-A").unwrap();
        assert!(s.get("uid-A").is_none());
        assert!(s.get("uid-B").is_some());
    }

    #[test]
    fn two_shared_services_over_one_file_do_not_clobber_each_others_projects() {
        // Two processes (one per project) sharing one workspace.toml, each writing
        // a different project's layout. The locked read-modify-write keeps the
        // second write's stale snapshot from dropping the first.
        let d = tempdir().unwrap();
        let path = d.path().join("workspace.toml");
        let a = WorkspaceLayoutService::open_at(path.clone(), Duration::ZERO).unwrap();
        let b = WorkspaceLayoutService::open_at(path.clone(), Duration::ZERO).unwrap();
        a.set(sample("uid-A")).unwrap();
        b.set(sample("uid-B")).unwrap();

        let c = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
        assert!(c.get("uid-A").is_some());
        assert!(c.get("uid-B").is_some());
    }

    #[test]
    fn an_unreadable_docks_blob_drops_to_none_without_failing_the_file() {
        // A project row whose embedded `docks` is a garbage/future-incompatible
        // table must load with `docks: None` and its tabs intact — and must not
        // take down a sibling project's row (blast-radius containment).
        let d = tempdir().unwrap();
        let path = d.path().join("workspace.toml");
        std::fs::write(
            &path,
            r#"
version = 1

[[projects]]
work_uid = "uid-good"
[projects.primary]
tabs = [1, 4]
selected = 4

[[projects]]
work_uid = "uid-bad-docks"
[projects.primary]
tabs = [2]
[projects.docks]
this_is_not = "a valid DockLayoutState"
leading = 12345
"#,
        )
        .unwrap();
        let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
        // The good row is untouched.
        let good = s.get("uid-good").expect("good row present");
        assert_eq!(good.primary.tabs, vec![1, 4]);
        assert!(good.docks.is_none());
        // The bad-docks row still loads; only its docks dropped to None.
        let bad = s.get("uid-bad-docks").expect("bad-docks row still loaded");
        assert_eq!(bad.primary.tabs, vec![2], "tabs survive an unreadable docks blob");
        assert!(bad.docks.is_none(), "unreadable docks blob -> None, not a load failure");
    }

    #[test]
    fn missing_optional_fields_default_cleanly() {
        // A minimal legacy-shaped row (only work_uid + a couple of tabs) must load,
        // the rest defaulting — additive schema evolution safety.
        let d = tempdir().unwrap();
        let path = d.path().join("workspace.toml");
        std::fs::write(
            &path,
            "version = 1\n\n[[projects]]\nwork_uid = \"uid-A\"\n[projects.primary]\ntabs = [0, 2]\n",
        )
        .unwrap();
        let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
        let got = s.get("uid-A").unwrap();
        assert_eq!(got.primary.tabs, vec![0, 2]);
        assert_eq!(got.primary.selected, None);
        assert!(!got.split_active);
        assert!(got.docks.is_none());
        assert!(got.secondary.is_empty());
    }
}
