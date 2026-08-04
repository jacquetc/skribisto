// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Persistent **user distraction-free themes** — a
//! `SettingsFile<DistractionFreeThemesFile>` at
//! `<config_dir>/distraction_free_themes.toml`.
//!
//! The same shape as [`crate::models::export_styles_file`], which is the right
//! precedent rather than the tags or dictionary panes: it is the only other
//! store with **shipped built-ins plus user entries plus single-item
//! import/export**. One locked read-modify-write per mutation, registered into
//! the app's `SettingsRegistry` so a peer window's write reloads in place. App
//! configuration, orthogonal to the backend — a single implementation, no
//! real/mock seam.
//!
//! ## Why each theme is a JSON string, not a `[[themes]]` table
//!
//! Partly the same reason export styles are — TOML's "values before tables"
//! rule makes a struct-with-enums awkward to grow — and partly containment: a
//! single corrupt entry is skipped rather than failing the whole list, so one
//! hand-edit typo cannot cost a writer every theme they have made. The envelope
//! is TOML (`version` + `Vec<String>`); each theme inside is canonical JSON,
//! which is also exactly the format the pane's Import/Export reads and writes.

use std::collections::HashSet;
use std::path::PathBuf;
use std::rc::Rc;

use bastyde::settings::{
    AppPaths, Migrator, Reloadable, SettingsFile, SettingsFileError, Versioned,
};
use serde::{Deserialize, Serialize};

use crate::distraction_free::theme::{DistractionFreeTheme, builtin_themes};

/// The persisted user themes: a version stamp + each theme as a JSON string.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DistractionFreeThemesFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub themes: Vec<String>,
}

fn default_version() -> u32 {
    DistractionFreeThemesFile::CURRENT_VERSION
}

impl Default for DistractionFreeThemesFile {
    fn default() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            themes: Vec::new(),
        }
    }
}

impl Versioned for DistractionFreeThemesFile {
    const CURRENT_VERSION: u32 = 1;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// v1 is the first version, so there are no upgrade steps; a file with no
/// `version` key defaults forward via `#[serde(default)]`.
fn migrator() -> Migrator<DistractionFreeThemesFile> {
    Migrator::new()
}

/// Persistent user-theme store. `SettingsFile` is `Clone` and shares live state,
/// so every clone is a view over the same configuration.
#[derive(Clone)]
pub struct DistractionFreeThemesService {
    file: SettingsFile<DistractionFreeThemesFile>,
}

impl DistractionFreeThemesService {
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("distraction_free_themes"), migrator())?;
        Ok(Self { file })
    }

    /// Open at an explicit path — used by tests.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn open_at(path: PathBuf) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, migrator())?;
        Ok(Self { file })
    }

    /// Graceful fallback when the config dir is unavailable: a throwaway
    /// per-process temp file, so the app still runs (themes just do not persist).
    /// Shares its retry/uniqueness logic with every sibling via
    /// [`in_memory_settings_file`](super::backup_settings_file::in_memory_settings_file).
    pub fn in_memory_default() -> Self {
        let file = super::backup_settings_file::in_memory_settings_file("df-themes", migrator());
        Self { file }
    }

    /// The `Reloadable` hook for the shared `SettingsRegistry` (keep the handle
    /// alive) so another window's write refreshes this one in place.
    pub fn as_reloadable(&self) -> Rc<dyn Reloadable> {
        Rc::new(self.file.clone())
    }

    /// One user theme by id, without materialising the rest — the list rows ask
    /// for exactly one each, and going through [`Self::user_themes`] for that
    /// would cost N parses per row.
    pub fn user_theme(&self, id: &str) -> Option<DistractionFreeTheme> {
        self.file
            .borrow()
            .themes
            .iter()
            .filter_map(|j| serde_json::from_str::<DistractionFreeTheme>(j).ok())
            .find(|t| t.id == id)
            .map(|mut t| {
                t.builtin = false;
                t
            })
    }

    /// The user's saved themes. A corrupt entry is skipped, not fatal. Each is
    /// stamped `builtin = false` defensively — a hand-edited file cannot pass
    /// one of its own off as read-only and so become undeletable.
    pub fn user_themes(&self) -> Vec<DistractionFreeTheme> {
        self.file
            .borrow()
            .themes
            .iter()
            .filter_map(|j| serde_json::from_str::<DistractionFreeTheme>(j).ok())
            .map(|mut t| {
                t.builtin = false;
                t
            })
            .collect()
    }

    /// Every theme the app knows: the shipped ones, then the writer's.
    pub fn all_themes(&self) -> Vec<DistractionFreeTheme> {
        builtin_themes()
            .into_iter()
            .chain(self.user_themes())
            .collect()
    }

    /// Resolve `id`, **falling back to the first built-in**.
    ///
    /// The fallback is the point: a writer who deletes the theme they were using
    /// (or opens a config that names one they never imported) must not end up
    /// with no theme at all, which would leave the surface unpainted.
    pub fn resolve(&self, id: &str) -> DistractionFreeTheme {
        self.all_themes()
            .into_iter()
            .find(|t| t.id == id)
            .unwrap_or_else(|| {
                builtin_themes()
                    .into_iter()
                    .next()
                    .expect("at least one built-in theme")
            })
    }

    /// Replace the whole user list (it is small; every mutation rewrites it).
    fn write_all(&self, themes: &[DistractionFreeTheme]) -> Result<(), SettingsFileError> {
        let json: Vec<String> = themes
            .iter()
            .filter_map(|t| serde_json::to_string(t).ok())
            .collect();
        self.file.mutate(|f| f.themes = json)
    }

    /// Insert or replace a user theme by id.
    pub fn upsert(&self, theme: &DistractionFreeTheme) -> Result<(), SettingsFileError> {
        let mut all = self.user_themes();
        match all.iter_mut().find(|t| t.id == theme.id) {
            Some(slot) => *slot = theme.clone(),
            None => all.push(theme.clone()),
        }
        self.write_all(&all)
    }

    /// Drop the user theme with `id` (a no-op if absent; built-ins are not
    /// stored here and so cannot be removed).
    pub fn remove(&self, id: &str) -> Result<(), SettingsFileError> {
        let all: Vec<DistractionFreeTheme> = self
            .user_themes()
            .into_iter()
            .filter(|t| t.id != id)
            .collect();
        self.write_all(&all)
    }

    /// Store `theme` as a **new** user theme, with `builtin = false` and a fresh
    /// id derived from `preferred_id`. Used by both duplicate-to-edit and JSON
    /// import. Returns the stored theme, with its final id.
    pub fn add_fresh(
        &self,
        mut theme: DistractionFreeTheme,
        preferred_id: &str,
    ) -> Result<DistractionFreeTheme, SettingsFileError> {
        theme.builtin = false;
        theme.id = unique_id(preferred_id, &self.taken_ids());
        let mut all = self.user_themes();
        all.push(theme.clone());
        self.write_all(&all)?;
        Ok(theme)
    }

    /// Every id in use — built-ins plus saved user themes.
    fn taken_ids(&self) -> HashSet<String> {
        builtin_themes()
            .into_iter()
            .map(|t| t.id)
            .chain(self.user_themes().into_iter().map(|t| t.id))
            .collect()
    }
}

/// `preferred` if free, else `preferred-2`, `preferred-3`, …
fn unique_id(preferred: &str, taken: &HashSet<String>) -> String {
    let base = if preferred.trim().is_empty() {
        "theme"
    } else {
        preferred
    };
    if !taken.contains(base) {
        return base.to_string();
    }
    let mut n = 2u32;
    loop {
        let candidate = format!("{base}-{n}");
        if !taken.contains(&candidate) {
            return candidate;
        }
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn svc(dir: &std::path::Path) -> DistractionFreeThemesService {
        DistractionFreeThemesService::open_at(dir.join("df_themes.toml")).unwrap()
    }

    fn mine(id: &str) -> DistractionFreeTheme {
        DistractionFreeTheme {
            id: id.to_string(),
            name: "Mine".into(),
            builtin: false,
            source: Some("paper".into()),
            base: crate::distraction_free::theme::ThemeBase::Light,
            general_background: "#eeeeee".into(),
            editor_background: "#ffffff".into(),
            editor_text: "#111111".into(),
            widget_text: "#444444".into(),
        }
    }

    #[test]
    fn a_user_theme_round_trips_through_disk() {
        let d = tempdir().unwrap();
        {
            svc(d.path()).upsert(&mine("mine")).unwrap();
        }
        let got = svc(d.path())
            .user_theme("mine")
            .expect("survived the reopen");
        assert_eq!(got.editor_background, "#ffffff");
        assert_eq!(got.source.as_deref(), Some("paper"));
    }

    /// One bad entry costs its owner that theme, not every theme they have.
    #[test]
    fn a_corrupt_entry_is_skipped_not_fatal() {
        let d = tempdir().unwrap();
        let path = d.path().join("df_themes.toml");
        let good = serde_json::to_string(&mine("good")).unwrap();
        std::fs::write(
            &path,
            format!(
                "version = 1\nthemes = [\"{{ not json\", {}]\n",
                serde_json::to_string(&good).unwrap()
            ),
        )
        .unwrap();
        let s = DistractionFreeThemesService::open_at(path).unwrap();
        let all = s.user_themes();
        assert_eq!(all.len(), 1, "the readable theme survived");
        assert_eq!(all[0].id, "good");
    }

    /// A user file cannot pass one of its own off as built-in — which would make
    /// it undeletable in the pane, since built-ins offer only Duplicate.
    #[test]
    fn a_user_theme_can_never_claim_to_be_builtin() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        let mut t = mine("sneaky");
        t.builtin = true;
        s.upsert(&t).unwrap();
        assert!(!s.user_theme("sneaky").unwrap().builtin);
    }

    /// A new theme never takes an id already in use — including a built-in's,
    /// which would otherwise shadow a shipped theme it cannot replace.
    #[test]
    fn add_fresh_dodges_builtin_and_user_ids() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        let a = s.add_fresh(mine("x"), "paper").unwrap();
        assert_ne!(a.id, "paper", "a built-in id must not be taken over");
        let b = s.add_fresh(mine("y"), "paper").unwrap();
        assert_ne!(b.id, a.id);
    }

    /// The writer must never end up with no theme.
    #[test]
    fn a_missing_current_theme_falls_back_to_a_builtin() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        let fallback = s.resolve("a-theme-that-was-deleted");
        assert!(fallback.builtin);
        assert_eq!(fallback.id, builtin_themes()[0].id);
    }

    #[test]
    fn resolve_prefers_a_user_theme_when_it_exists() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        s.upsert(&mine("mine")).unwrap();
        assert_eq!(s.resolve("mine").name, "Mine");
    }

    #[test]
    fn remove_drops_only_the_named_theme() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        s.upsert(&mine("a")).unwrap();
        s.upsert(&mine("b")).unwrap();
        s.remove("a").unwrap();
        let ids: Vec<String> = s.user_themes().into_iter().map(|t| t.id).collect();
        assert_eq!(ids, vec!["b".to_string()]);
    }

    #[test]
    fn all_themes_lists_the_builtins_first() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        s.upsert(&mine("mine")).unwrap();
        let all = s.all_themes();
        assert_eq!(all.len(), builtin_themes().len() + 1);
        assert!(all[0].builtin);
        assert!(!all.last().unwrap().builtin);
    }
}
