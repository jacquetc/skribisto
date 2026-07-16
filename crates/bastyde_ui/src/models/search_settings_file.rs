// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Persistent search & replace configuration.
//!
//! A `SettingsFile<SearchSettingsFile>` at `<config_dir>/search.toml` holding one
//! **general** preference set plus optional **per-project overrides** and
//! per-project query/replacement history. It is the persistence half of the
//! search feature: the `Search`/`SearchResult` entities are the live, reactive,
//! re-derivable surface, but they survive nothing — `WorkInfo` is recreated on
//! every load and torn down on close, and the store is an ephemeral `HashMap`. So
//! the toggles a writer set, the scopes and facets they narrowed to, and what
//! they searched for last time all live here instead.
//!
//! Per-project entries are keyed by the project's stable **`Work.unique_id`**
//! (not its path — that survives rename/move, and is the same key
//! `backup.toml` correlates on). The raw path + title are kept only for display.
//! This is app configuration, orthogonal to the backend, so there is a **single
//! implementation (no real/mock seam)** — it opens a real `SettingsFile` in every
//! build, exactly like [`BackupSettingsService`](super::BackupSettingsService)
//! and `WindowStateService`.
//!
//! **Cross-process safety.** Skribisto runs **one process per project**, and
//! every instance shares this same `<config_dir>/search.toml`. `SettingsFile`'s
//! locked read-modify-write is the only write mode, so two windows persisting two
//! different projects' preferences never clobber each other; [`as_reloadable`]
//! lets the app's `SettingsWatcher` refresh this handle when a peer writes.

use std::rc::Rc;
use std::time::Duration;

use bastyde::settings::{
    AppPaths, Migrator, Reloadable, SettingsFile, SettingsFileError, Versioned,
};
use serde::{Deserialize, Serialize};

/// Accepted for call-site stability only. `SettingsFile::load`'s writes are a
/// synchronous locked read-modify-write — there is no debounce to configure
/// (see [`BackupSettingsService`](super::BackupSettingsService)).
const SETTINGS_DEBOUNCE: Duration = Duration::from_millis(500);

/// How many past queries / replacements to keep per project.
const HISTORY_CAP: usize = 20;

/// One complete search preference set — the general default, or a per-project
/// override. Toggles + scopes + the facet filter.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SearchPrefs {
    // Matching toggles.
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub diacritic_sensitive: bool,
    // Field scopes (which parts of an item to search).
    pub search_body: bool,
    pub search_titles: bool,
    pub search_synopsis: bool,
    pub search_labels: bool,
    /// Include trashed items in the scan.
    pub include_trashed: bool,
    /// The ticked facet chips as `skribisto_model::SearchFacet::code()` values.
    /// **Empty ⇒ no filter (all kinds)** — the same convention the backend's
    /// `run_search` reads (see its `wanted_facets`).
    pub facets: Vec<i64>,
}

impl Default for SearchPrefs {
    fn default() -> Self {
        // Body + synopsis + titles + labels on; whole-word on (a rename must not
        // half-match `Elena` inside `Elenavich`); diacritics folded; trashed
        // excluded; no facet filter.
        SearchPrefs {
            case_sensitive: false,
            whole_word: true,
            diacritic_sensitive: false,
            search_body: true,
            search_titles: true,
            search_synopsis: true,
            search_labels: true,
            include_trashed: false,
            facets: Vec::new(),
        }
    }
}

/// A per-project override of the general preferences (all-or-nothing) plus that
/// project's query/replacement history.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct PerProjectSearch {
    pub work_uid: String,
    /// Display/debug only (the key is `work_uid`).
    pub last_path: String,
    pub title: String,
    /// The project's saved preferences. `None` ⇒ inherit general (history can
    /// exist without an override, and vice versa).
    pub prefs: Option<SearchPrefs>,
    /// Past queries, most-recent-first, deduped, capped at [`HISTORY_CAP`].
    #[serde(default)]
    pub query_history: Vec<String>,
    /// Past replacement strings, most-recent-first, deduped, capped.
    #[serde(default)]
    pub replacement_history: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SearchSettingsFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub general: SearchPrefs,
    #[serde(default)]
    pub projects: Vec<PerProjectSearch>,
}

fn default_version() -> u32 {
    SearchSettingsFile::CURRENT_VERSION
}

impl Default for SearchSettingsFile {
    fn default() -> Self {
        SearchSettingsFile {
            version: SearchSettingsFile::CURRENT_VERSION,
            general: SearchPrefs::default(),
            projects: Vec::new(),
        }
    }
}

impl Versioned for SearchSettingsFile {
    const CURRENT_VERSION: u32 = 1;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// Persistent search-settings service. `SettingsFile` is `Clone` (shares the
/// in-memory state + writer), so cloning hands out views over the same live
/// configuration.
#[derive(Clone)]
pub struct SearchSettingsService {
    file: SettingsFile<SearchSettingsFile>,
}

// Some methods (general/override/history CRUD) are the service's complete,
// tested surface but are wired incrementally — a search-settings *panel* is a
// natural follow-on that will consume the general/override setters. Same
// convention as `view_models::outline`.
#[allow(dead_code)]
impl SearchSettingsService {
    /// Open `search.toml` under `paths` (cross-process safe).
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        Self::open_with_delay(paths, SETTINGS_DEBOUNCE)
    }

    /// `delay` is accepted for call-site stability but has no effect.
    pub fn open_with_delay(paths: &AppPaths, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("search"), Migrator::new())?;
        Ok(Self { file })
    }

    /// Open at an explicit path — used by tests.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn open_at(path: std::path::PathBuf, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, Migrator::new())?;
        Ok(Self { file })
    }

    /// Graceful fallback when the config dir is unavailable: a throwaway
    /// per-process temp file, so the app still runs (search preferences just
    /// won't persist across restarts).
    pub fn in_memory_default() -> Self {
        let path =
            std::env::temp_dir().join(format!("skribisto-search-{}.toml", std::process::id()));
        SettingsFile::load(path, Migrator::new())
            .map(|file| Self { file })
            .unwrap_or_else(|_| {
                let file = SettingsFile::load(
                    std::path::PathBuf::from(".skribisto-search.toml"),
                    Migrator::new(),
                )
                .expect("in-memory search settings fallback");
                Self { file }
            })
    }

    /// The `Reloadable` hook for the app's shared `SettingsRegistry` — register
    /// this (and keep the returned handle alive) so a peer process's write is
    /// picked up with no per-read polling.
    pub fn as_reloadable(&self) -> Rc<dyn Reloadable> {
        Rc::new(self.file.clone())
    }

    // ── general preferences ──
    pub fn general(&self) -> SearchPrefs {
        self.file.borrow().general.clone()
    }

    pub fn set_general(&self, prefs: SearchPrefs) -> Result<(), SettingsFileError> {
        self.file.mutate(|f| f.general = prefs)
    }

    // ── per-project overrides ──
    pub fn has_override(&self, work_uid: &str) -> bool {
        self.file
            .borrow()
            .projects
            .iter()
            .any(|p| p.work_uid == work_uid && p.prefs.is_some())
    }

    /// The preferences in effect for `work_uid`: its override if set, else general.
    pub fn effective_for(&self, work_uid: &str) -> SearchPrefs {
        let f = self.file.borrow();
        f.projects
            .iter()
            .find(|p| p.work_uid == work_uid)
            .and_then(|p| p.prefs.clone())
            .unwrap_or_else(|| f.general.clone())
    }

    /// Persist `prefs` as `work_uid`'s override. A no-op when `work_uid` is empty
    /// (a brand-new unsaved project has no uid yet — see [`super::uid_is_usable`]).
    pub fn set_override(
        &self,
        work_uid: &str,
        last_path: &str,
        title: &str,
        prefs: SearchPrefs,
    ) -> Result<(), SettingsFileError> {
        if !super::uid_is_usable(work_uid) {
            return Ok(());
        }
        self.file.mutate(|f| {
            let p = project_mut(f, work_uid);
            p.last_path = last_path.to_string();
            p.title = title.to_string();
            p.prefs = Some(prefs);
        })
    }

    pub fn clear_override(&self, work_uid: &str) -> Result<(), SettingsFileError> {
        self.file.mutate(|f| {
            if let Some(p) = f.projects.iter_mut().find(|p| p.work_uid == work_uid) {
                p.prefs = None;
            }
        })
    }

    // ── query / replacement history ──
    pub fn query_history(&self, work_uid: &str) -> Vec<String> {
        self.file
            .borrow()
            .projects
            .iter()
            .find(|p| p.work_uid == work_uid)
            .map(|p| p.query_history.clone())
            .unwrap_or_default()
    }

    pub fn replacement_history(&self, work_uid: &str) -> Vec<String> {
        self.file
            .borrow()
            .projects
            .iter()
            .find(|p| p.work_uid == work_uid)
            .map(|p| p.replacement_history.clone())
            .unwrap_or_default()
    }

    /// Record a used query at the front of `work_uid`'s history (dedup, capped).
    /// No-op for an empty/whitespace query or an unusable uid.
    pub fn push_query(
        &self,
        work_uid: &str,
        last_path: &str,
        query: &str,
    ) -> Result<(), SettingsFileError> {
        if !super::uid_is_usable(work_uid) || query.trim().is_empty() {
            return Ok(());
        }
        self.file.mutate(|f| {
            let p = project_mut(f, work_uid);
            p.last_path = last_path.to_string();
            push_front_dedup(&mut p.query_history, query);
        })
    }

    /// Record a used replacement at the front of `work_uid`'s history. An empty
    /// replacement is legitimate ("delete every match"), so — unlike a query —
    /// only the uid is guarded.
    pub fn push_replacement(
        &self,
        work_uid: &str,
        last_path: &str,
        replacement: &str,
    ) -> Result<(), SettingsFileError> {
        if !super::uid_is_usable(work_uid) {
            return Ok(());
        }
        self.file.mutate(|f| {
            let p = project_mut(f, work_uid);
            p.last_path = last_path.to_string();
            push_front_dedup(&mut p.replacement_history, replacement);
        })
    }

    pub fn flush_now(&self) -> Result<(), SettingsFileError> {
        self.file.flush_now()
    }
}

fn project_mut<'a>(f: &'a mut SearchSettingsFile, work_uid: &str) -> &'a mut PerProjectSearch {
    if let Some(pos) = f.projects.iter().position(|p| p.work_uid == work_uid) {
        &mut f.projects[pos]
    } else {
        f.projects.push(PerProjectSearch {
            work_uid: work_uid.to_string(),
            ..Default::default()
        });
        f.projects.last_mut().unwrap()
    }
}

/// Move `value` to the front, removing any earlier copy, and cap the length.
/// Most-recently-used-first, no duplicates — the shape `SearchField`'s
/// suggestion list wants.
fn push_front_dedup(history: &mut Vec<String>, value: &str) {
    history.retain(|v| v != value);
    history.insert(0, value.to_string());
    history.truncate(HISTORY_CAP);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn svc(dir: &std::path::Path) -> SearchSettingsService {
        SearchSettingsService::open_at(dir.join("search.toml"), Duration::ZERO).unwrap()
    }

    #[test]
    fn defaults_search_everything_whole_word_folded() {
        let d = tempdir().unwrap();
        let g = svc(d.path()).general();
        assert!(g.search_body && g.search_titles && g.search_synopsis && g.search_labels);
        assert!(g.whole_word, "whole-word is on by default (rename safety)");
        assert!(!g.case_sensitive && !g.diacritic_sensitive && !g.include_trashed);
        assert!(g.facets.is_empty(), "empty facets means no filter");
    }

    #[test]
    fn override_wins_and_is_isolated_per_uid() {
        let d = tempdir().unwrap();
        let s = svc(d.path());

        let mut custom = s.general();
        custom.case_sensitive = true;
        custom.facets = vec![1, 3];
        s.set_override("uid-A", "/x/a.skrib", "A", custom).unwrap();

        assert!(s.has_override("uid-A"));
        assert!(!s.has_override("uid-B"));
        assert!(s.effective_for("uid-A").case_sensitive);
        assert_eq!(s.effective_for("uid-A").facets, vec![1, 3]);
        assert!(!s.effective_for("uid-B").case_sensitive, "B inherits general");

        s.clear_override("uid-A").unwrap();
        assert!(!s.has_override("uid-A"));
        assert!(!s.effective_for("uid-A").case_sensitive, "A back to general");
    }

    #[test]
    fn an_empty_uid_never_persists_an_override() {
        // A brand-new unsaved project has no unique_id; keying "" would collide
        // across unrelated projects, so a set on it is silently dropped.
        let d = tempdir().unwrap();
        let s = svc(d.path());
        let mut custom = s.general();
        custom.case_sensitive = true;
        s.set_override("", "/x/new.skrib", "Untitled", custom).unwrap();
        assert!(!s.has_override(""), "empty uid must not persist");
    }

    #[test]
    fn history_is_mru_deduped_capped_and_roundtrips() {
        let d = tempdir().unwrap();
        {
            let s = svc(d.path());
            for q in ["alpha", "beta", "alpha", "gamma"] {
                s.push_query("uid-A", "/x/a.skrib", q).unwrap();
            }
            // Empty / whitespace queries are not recorded.
            s.push_query("uid-A", "/x/a.skrib", "   ").unwrap();
            // An empty replacement IS legitimate ("delete every match").
            s.push_replacement("uid-A", "/x/a.skrib", "").unwrap();
            s.push_replacement("uid-A", "/x/a.skrib", "Elena").unwrap();
            s.flush_now().unwrap();
        }
        // Reopen from disk — history survived the TOML round-trip.
        let s = svc(d.path());
        assert_eq!(
            s.query_history("uid-A"),
            vec!["gamma", "alpha", "beta"],
            "most-recent-first, deduped (re-added alpha moved to front)"
        );
        assert_eq!(s.replacement_history("uid-A"), vec!["Elena", ""]);
    }

    #[test]
    fn history_is_capped_at_the_limit() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        for i in 0..(HISTORY_CAP + 10) {
            s.push_query("uid-A", "/x/a.skrib", &format!("q{i}")).unwrap();
        }
        assert_eq!(s.query_history("uid-A").len(), HISTORY_CAP);
        // The newest is at the front, the oldest fell off the back.
        assert_eq!(s.query_history("uid-A")[0], format!("q{}", HISTORY_CAP + 9));
    }

    #[test]
    fn two_shared_services_over_one_file_do_not_clobber_each_others_projects() {
        // Two processes (one per project) sharing one search.toml, each writing a
        // different project's override. Without the locked read-modify-write, the
        // second write's stale snapshot would drop the first.
        let d = tempdir().unwrap();
        let path = d.path().join("search.toml");
        let a = SearchSettingsService::open_at(path.clone(), Duration::ZERO).unwrap();
        let b = SearchSettingsService::open_at(path.clone(), Duration::ZERO).unwrap();

        let mut pa = a.general();
        pa.case_sensitive = true;
        a.set_override("uid-A", "/x/a.skrib", "A", pa).unwrap();
        let mut pb = b.general();
        pb.diacritic_sensitive = true;
        b.set_override("uid-B", "/x/b.skrib", "B", pb).unwrap();

        let c = SearchSettingsService::open_at(path, Duration::ZERO).unwrap();
        assert!(c.has_override("uid-A") && c.effective_for("uid-A").case_sensitive);
        assert!(c.has_override("uid-B") && c.effective_for("uid-B").diacritic_sensitive);
    }
}
