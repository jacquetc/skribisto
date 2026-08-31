// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Persistent **user export styles** — a `SettingsFile<ExportStylesFile>` at
//! `<config_dir>/export_styles.toml`, the exact shape and cross-process story as
//! [`crate::models::dictionary_settings_file`]: one locked read-modify-write per mutation,
//! registered into the app's `SettingsRegistry` so a peer process's write reloads in place.
//! App configuration, orthogonal to the backend — a single implementation, no real/mock seam.
//!
//! ## Why each preset is a JSON string, not a `[[presets]]` table
//!
//! [`Preset`] carries **data-bearing enums** (`SceneBreak::Glyph("#")`,
//! `HeadingLanguage::Fixed("fr")`) and a nested `Margins` struct sitting *before* later scalar
//! fields — the exact shape a TOML serializer chokes on ("values must be emitted before
//! tables"). The backup settings file flattened its retention fields for the same reason. So
//! the on-disk envelope is TOML (`version` + a `Vec<String>`), but each preset inside it is
//! stored as its **canonical JSON** — the same round-trip-tested wire format
//! [`skribisto_compiler`] already guarantees and that the panel's Import/Export uses. A single
//! corrupt entry is skipped rather than dropping the whole list.

use std::collections::HashSet;
use std::path::PathBuf;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use skribisto_compiler::{Preset, builtin_presets};
use teksilo::settings::{
    AppPaths, Migrator, Reloadable, SettingsFile, SettingsFileError, Versioned,
};

/// The persisted user styles: a version stamp + each user preset as a JSON string.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExportStylesFile {
    #[serde(default = "default_version")]
    pub version: u32,
    /// Each entry is one [`Preset`] serialized with `serde_json` (see the module docs for why
    /// JSON rather than a native `[[presets]]` table).
    #[serde(default)]
    pub presets: Vec<String>,
}

fn default_version() -> u32 {
    ExportStylesFile::CURRENT_VERSION
}

impl Default for ExportStylesFile {
    fn default() -> Self {
        ExportStylesFile {
            version: ExportStylesFile::CURRENT_VERSION,
            presets: Vec::new(),
        }
    }
}

impl Versioned for ExportStylesFile {
    const CURRENT_VERSION: u32 = 1;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// The migrator for `export_styles.toml`. v1 is the first version, so there are no upgrade
/// steps; a file with a missing `version` key defaults forward via `#[serde(default)]`.
fn migrator() -> Migrator<ExportStylesFile> {
    Migrator::new()
}

/// Persistent user-style store. `SettingsFile` is `Clone` (shares the live in-memory state),
/// so every clone is a view over the same configuration.
#[derive(Clone)]
pub struct ExportStylesService {
    file: SettingsFile<ExportStylesFile>,
}

impl ExportStylesService {
    /// Open `export_styles.toml` under `paths` (cross-process safe by default).
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("export_styles"), migrator())?;
        Ok(Self { file })
    }

    /// Open at an explicit path — used by tests.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn open_at(path: PathBuf) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, migrator())?;
        Ok(Self { file })
    }

    /// Graceful fallback when the config dir is unavailable: a throwaway per-process temp file,
    /// so the app still runs (user styles just won't persist across restarts). Shares its
    /// retry/uniqueness logic with every sibling via
    /// [`in_memory_settings_file`](super::backup_settings_file::in_memory_settings_file).
    pub fn in_memory_default() -> Self {
        let file =
            super::backup_settings_file::in_memory_settings_file("export-styles", migrator());
        Self { file }
    }

    /// The `Reloadable` hook for the app's shared `SettingsRegistry` (keep the returned handle
    /// alive) so a peer process's style write refreshes this handle in place.
    pub fn as_reloadable(&self) -> Rc<dyn Reloadable> {
        Rc::new(self.file.clone())
    }

    /// One user preset by id, without materialising the rest.
    ///
    /// [`Self::user_presets`] deserializes **every** stored envelope on each
    /// call, so looking a single style up through it costs N parses — and doing
    /// that once per list row costs N². This stops at the first match.
    pub fn user_preset(&self, id: &str) -> Option<Preset> {
        self.file
            .borrow()
            .presets
            .iter()
            .filter_map(|j| serde_json::from_str::<Preset>(j).ok())
            .find(|p| p.id == id)
            .map(|mut p| {
                p.builtin = false;
                p
            })
    }

    /// The user's saved presets, parsed from their JSON envelopes (a corrupt entry is skipped).
    /// Each is stamped `builtin = false` defensively — a user file can't masquerade a preset as
    /// read-only.
    pub fn user_presets(&self) -> Vec<Preset> {
        self.file
            .borrow()
            .presets
            .iter()
            .filter_map(|j| serde_json::from_str::<Preset>(j).ok())
            .map(|mut p| {
                p.builtin = false;
                p
            })
            .collect()
    }

    /// Replace the whole user-preset list (the store is small; every mutation rewrites it).
    fn write_all(&self, presets: &[Preset]) -> Result<(), SettingsFileError> {
        let json: Vec<String> = presets
            .iter()
            .filter_map(|p| serde_json::to_string(p).ok())
            .collect();
        self.file.mutate(|f| f.presets = json)
    }

    /// Insert or replace a user preset by id (edit-in-place, or first save of a duplicate).
    pub fn upsert(&self, preset: &Preset) -> Result<(), SettingsFileError> {
        let mut all = self.user_presets();
        match all.iter_mut().find(|p| p.id == preset.id) {
            Some(slot) => *slot = preset.clone(),
            None => all.push(preset.clone()),
        }
        self.write_all(&all)
    }

    /// Drop the user preset with `id` (a no-op if absent).
    pub fn remove(&self, id: &str) -> Result<(), SettingsFileError> {
        let all: Vec<Preset> = self
            .user_presets()
            .into_iter()
            .filter(|p| p.id != id)
            .collect();
        self.write_all(&all)
    }

    /// Store `preset` as a **new** user style, giving it `builtin = false` and a fresh unique id
    /// (derived from `preferred`) when that would collide with a built-in or an existing user id.
    /// Used by both duplicate-to-edit and JSON import. Returns the stored preset (with its final id).
    pub fn add_fresh(
        &self,
        mut preset: Preset,
        preferred_id: &str,
    ) -> Result<Preset, SettingsFileError> {
        preset.builtin = false;
        preset.id = unique_id(preferred_id, &self.taken_ids());
        let mut all = self.user_presets();
        all.push(preset.clone());
        self.write_all(&all)?;
        Ok(preset)
    }

    /// Every id currently in use — built-ins plus saved user styles — so a new id can dodge them.
    fn taken_ids(&self) -> HashSet<String> {
        builtin_presets()
            .into_iter()
            .map(|p| p.id)
            .chain(self.user_presets().into_iter().map(|p| p.id))
            .collect()
    }
}

/// `preferred` if free, else `preferred-2`, `preferred-3`, … — a stable, collision-free id.
fn unique_id(preferred: &str, taken: &HashSet<String>) -> String {
    let base = if preferred.trim().is_empty() {
        "style"
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
    use std::sync::atomic::{AtomicU64, Ordering};

    fn temp_service() -> ExportStylesService {
        static N: AtomicU64 = AtomicU64::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "skribisto-stylestest-{}-{n}.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        ExportStylesService::open_at(path).expect("open temp export-styles settings")
    }

    /// A built-in duplicated, edited, and saved must survive a full write→reopen cycle through
    /// the TOML envelope + JSON preset encoding — including its data-bearing enum fields.
    #[test]
    fn user_preset_round_trips_through_disk() {
        static N: AtomicU64 = AtomicU64::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "skribisto-stylesrt-{}-{n}.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);

        let base = builtin_presets()
            .into_iter()
            .find(|p| p.id == "manuscript-shunn")
            .unwrap();
        {
            let svc = ExportStylesService::open_at(path.clone()).unwrap();
            let stored = svc
                .add_fresh(base.clone(), "manuscript-shunn-copy")
                .unwrap();
            assert_eq!(
                stored.id, "manuscript-shunn-copy",
                "fresh id (no collision)"
            );
            assert!(!stored.builtin, "a duplicated style is editable");
            // The data-bearing enum (SceneBreak::Glyph) must survive the JSON-in-TOML envelope.
            assert!(matches!(
                stored.scene_break,
                skribisto_compiler::SceneBreak::Glyph(_)
            ));
            // Both tiers, not just the first: `major_scene_break` carries
            // `#[serde(default)]`, so a round trip that silently dropped it would
            // still deserialize — and quietly reset every user's major break.
            assert_eq!(stored.major_scene_break, base.major_scene_break);
            assert_ne!(
                stored.major_scene_break, stored.scene_break,
                "the Shunn style distinguishes its two tiers"
            );
        }
        // Reopen from disk: the user preset is still there, byte-identical.
        let svc2 = ExportStylesService::open_at(path.clone()).unwrap();
        let users = svc2.user_presets();
        assert_eq!(users.len(), 1, "one user preset persisted");
        assert_eq!(users[0].id, "manuscript-shunn-copy");
        assert_eq!(users[0].font_family, base.font_family);
        assert_eq!(users[0].scene_break, base.scene_break);
        assert_eq!(users[0].major_scene_break, base.major_scene_break);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn add_fresh_dodges_id_collisions() {
        let svc = temp_service();
        let base = builtin_presets().into_iter().next().unwrap();
        // Colliding with a built-in id → suffixed.
        let a = svc.add_fresh(base.clone(), &base.id).unwrap();
        assert_ne!(a.id, base.id, "must not reuse a built-in id");
        // A second duplicate of the same preferred id → next free suffix.
        let b = svc.add_fresh(base.clone(), &base.id).unwrap();
        assert_ne!(b.id, a.id, "two copies get distinct ids");
        assert_eq!(svc.user_presets().len(), 2);
    }

    #[test]
    fn upsert_edits_in_place_and_remove_drops() {
        let svc = temp_service();
        let base = builtin_presets().into_iter().next().unwrap();
        let mut p = svc.add_fresh(base, "my-style").unwrap();
        assert_eq!(svc.user_presets().len(), 1);

        // Edit in place: same id, changed name.
        p.name = "Renamed".to_string();
        svc.upsert(&p).unwrap();
        let users = svc.user_presets();
        assert_eq!(users.len(), 1, "upsert edits, never appends a second");
        assert_eq!(users[0].name, "Renamed");

        svc.remove("my-style").unwrap();
        assert!(svc.user_presets().is_empty());
        // Removing an absent id is a harmless no-op.
        svc.remove("not-there").unwrap();
    }

    #[test]
    fn a_corrupt_entry_is_skipped_not_fatal() {
        let svc = temp_service();
        let base = builtin_presets().into_iter().next().unwrap();
        svc.add_fresh(base, "good").unwrap();
        // Inject a garbage JSON string beside the good one.
        svc.file
            .mutate(|f| f.presets.push("{ not valid preset json".to_string()))
            .unwrap();
        let users = svc.user_presets();
        assert_eq!(
            users.len(),
            1,
            "the good preset survives; the garbage is dropped"
        );
        assert_eq!(users[0].id, "good");
    }
}
