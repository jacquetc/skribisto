// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `updates.toml` — what the last update check found, and when it ran.
//!
//! App-global rather than per-project, like `dictionaries.toml`: which release
//! is current is a fact about the installation, not about a manuscript.
//!
//! ## Why the answer is persisted at all
//!
//! Because the surfaces that show it are places a reader *arrives at*, not
//! places that are open when the check completes. The Launcher is dismissed
//! within seconds of a project opening, and About is opened on purpose, days
//! later. A discovery kept only in memory would be shown to almost nobody: the
//! one session that learned the news is the one least likely to be looking.
//!
//! Persisting it also means the surfaces work with no network at all. A reader
//! who was told about 3.0.2 last week still sees it while offline, which is the
//! honest thing to show, since it is still true.
//!
//! ## Why `checked_on` is written *before* the request, not after
//!
//! It is a "do not ask again today" mark, not a record of success. Writing it
//! only on success would make a machine that is offline all week try again on
//! every single launch, which is the retry storm the cadence exists to prevent.
//! Writing it up front costs a day's delay after a failure and nothing else.

use std::collections::BTreeMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use teksilo::settings::{AppPaths, Migrator, SettingsFile, SettingsFileError, Versioned};

/// Accepted for call-site stability only; `SettingsFile`'s writes are a
/// synchronous locked read-modify-write with no debounce (see the siblings this
/// mirrors).
const SETTINGS_DEBOUNCE: Duration = Duration::from_millis(500);

/// What the last check learned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UpdatesFile {
    #[serde(default = "default_version")]
    pub version: u32,
    /// UTC date of the last *attempt*, `YYYY-MM-DD`. Empty when none has run.
    #[serde(default)]
    pub checked_on: String,
    /// The newest release the last successful check saw. Empty when none has
    /// succeeded.
    #[serde(default)]
    pub latest_version: String,
    /// Its publication date, `YYYY-MM-DD`.
    #[serde(default)]
    pub latest_date: String,
    /// Release-notes page by language code, as the feed published it.
    #[serde(default)]
    pub notes: BTreeMap<String, String>,
    /// Download page by language code, as the feed published it.
    #[serde(default)]
    pub download: BTreeMap<String, String>,
}

fn default_version() -> u32 {
    UpdatesFile::CURRENT_VERSION
}

impl Default for UpdatesFile {
    fn default() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            checked_on: String::new(),
            latest_version: String::new(),
            latest_date: String::new(),
            notes: BTreeMap::new(),
            download: BTreeMap::new(),
        }
    }
}

impl Versioned for UpdatesFile {
    const CURRENT_VERSION: u32 = 1;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// Handle on `updates.toml`. `SettingsFile` is `Clone` and shares live state, so
/// cloning hands out views over the same file.
#[derive(Clone)]
pub struct UpdatesService {
    file: SettingsFile<UpdatesFile>,
}

impl UpdatesService {
    /// Open `updates.toml` under `paths` (cross-process safe).
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        Self::open_with_delay(paths, SETTINGS_DEBOUNCE)
    }

    pub fn open_with_delay(paths: &AppPaths, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("updates"), Migrator::new())?;
        Ok(Self { file })
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn open_at(path: std::path::PathBuf) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, Migrator::new())?;
        Ok(Self { file })
    }

    /// An in-memory stand-in for a launch with no usable config directory. The
    /// feature goes quiet rather than blocking startup: the application still
    /// runs, it simply forgets between sessions what it found.
    pub fn in_memory_default() -> Self {
        Self {
            file: super::backup_settings_file::in_memory_settings_file("updates", Migrator::new()),
        }
    }

    /// The whole record.
    pub fn get(&self) -> UpdatesFile {
        self.file.snapshot()
    }

    /// Mark that a check was attempted on `today` (`YYYY-MM-DD`, UTC).
    ///
    /// Called before the request goes out; see the module docs.
    pub fn mark_attempt(&self, today: &str) {
        let today = today.to_string();
        let _ = self.file.mutate(|f| f.checked_on = today.clone());
    }

    /// Record what a successful check found.
    pub fn record(
        &self,
        latest_version: &str,
        latest_date: &str,
        notes: BTreeMap<String, String>,
        download: BTreeMap<String, String>,
    ) {
        let (v, d) = (latest_version.to_string(), latest_date.to_string());
        let _ = self.file.mutate(|f| {
            f.latest_version = v.clone();
            f.latest_date = d.clone();
            f.notes = notes.clone();
            f.download = download.clone();
        });
    }

    /// Whether a check has already been attempted on `today`.
    pub fn checked_today(&self, today: &str) -> bool {
        !today.is_empty() && self.get().checked_on == today
    }

    /// Forget the last discovery, leaving the attempt date alone.
    ///
    /// Used when the reader turns the check off: the surfaces must go dark at
    /// once, and a stale "3.0.2 is available" left in the file would come back
    /// the moment they turned it on again, without any check having run.
    pub fn forget_discovery(&self) {
        let _ = self.file.mutate(|f| {
            f.latest_version = String::new();
            f.latest_date = String::new();
            f.notes.clear();
            f.download.clear();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service() -> (UpdatesService, tempfile::TempDir) {
        let dir = tempfile::tempdir().expect("temp dir");
        let svc = UpdatesService::open_at(dir.path().join("updates.toml")).expect("open");
        (svc, dir)
    }

    fn links() -> BTreeMap<String, String> {
        BTreeMap::from([
            (
                "en".to_string(),
                "https://www.skribisto.eu/news/".to_string(),
            ),
            (
                "fr".to_string(),
                "https://www.skribisto.eu/fr/news/".to_string(),
            ),
        ])
    }

    #[test]
    fn a_fresh_install_has_never_checked_and_knows_nothing() {
        let (svc, _d) = service();
        let f = svc.get();
        assert_eq!(f.checked_on, "");
        assert_eq!(f.latest_version, "");
        assert!(!svc.checked_today("2026-09-04"));
    }

    #[test]
    fn an_attempt_is_remembered_even_when_it_learns_nothing() {
        // The retry-storm guard: a machine that is offline must not try again on
        // every launch for the rest of the day.
        let (svc, _d) = service();
        svc.mark_attempt("2026-09-04");
        assert!(svc.checked_today("2026-09-04"));
        assert_eq!(svc.get().latest_version, "", "nothing was learned");
        assert!(!svc.checked_today("2026-09-05"), "tomorrow it tries again");
    }

    #[test]
    fn a_discovery_survives_a_restart() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("updates.toml");
        {
            let svc = UpdatesService::open_at(path.clone()).expect("open");
            svc.mark_attempt("2026-09-04");
            svc.record("3.0.2", "2026-09-18", links(), links());
        }
        // A second handle on the same path is what a later launch sees.
        let reopened = UpdatesService::open_at(path).expect("reopen");
        let f = reopened.get();
        assert_eq!(f.latest_version, "3.0.2");
        assert_eq!(f.latest_date, "2026-09-18");
        assert_eq!(
            f.notes.get("fr").map(String::as_str),
            Some("https://www.skribisto.eu/fr/news/")
        );
    }

    #[test]
    fn turning_the_check_off_forgets_what_it_found() {
        let (svc, _d) = service();
        svc.mark_attempt("2026-09-04");
        svc.record("3.0.2", "2026-09-18", links(), links());
        svc.forget_discovery();
        let f = svc.get();
        assert_eq!(f.latest_version, "");
        assert!(f.notes.is_empty());
        // The attempt mark stays: turning the setting off is not a reason to
        // re-ask the moment it is turned back on.
        assert_eq!(f.checked_on, "2026-09-04");
    }

    #[test]
    fn an_empty_date_never_counts_as_checked_today() {
        let (svc, _d) = service();
        assert!(
            !svc.checked_today(""),
            "a clock that gave nothing is not a day"
        );
    }

    #[test]
    fn the_in_memory_fallback_works_without_a_config_directory() {
        let svc = UpdatesService::in_memory_default();
        svc.mark_attempt("2026-09-04");
        assert!(svc.checked_today("2026-09-04"));
    }
}
