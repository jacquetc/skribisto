// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ExportStylesViewModel` — the business logic behind Settings ▸ Compile & Export ▸ **Export
//! Formats** and the Export panel's style picker.
//!
//! **App-local, not a Qleany feature.** An export style is a machine-wide preference that
//! outlives any `Work`, is not undoable, and never touches the entity store — so this mirrors
//! [`crate::view_models::DictionariesViewModel`]: single-instance live state
//! ([`ExportStylesService`] over `export_styles.toml` + a `changed` bump), created once in
//! `main.rs`, registered as `app_state`, and shared by `.clone()`. The Settings pane binds
//! `changed` to re-derive its lists; the Export panel unions the built-ins with the user's.
//!
//! Built-in presets ([`skribisto_compiler::builtin_presets`]) are read-only; the user works
//! with **duplicates** (duplicate-to-edit) and portable **JSON** import/export.

use std::path::Path;
use std::rc::Rc;

use anyhow::Context;
use bastyde::prelude::*;
use skribisto_compiler::{Preset, builtin_presets};

use crate::models::ExportStylesService;

#[derive(Clone)]
pub struct ExportStylesViewModel {
    inner: Rc<Inner>,
}

struct Inner {
    service: ExportStylesService,
    /// Bumped on every mutation (duplicate / edit / delete / import) so the Settings lists and
    /// the panel's picker re-derive.
    changed: Signal<u64>,
}

impl ExportStylesViewModel {
    pub fn new(service: ExportStylesService) -> Self {
        Self { inner: Rc::new(Inner { service, changed: Signal::new(0) }) }
    }

    // ── read handles ──

    /// Bumped on any style mutation.
    pub fn changed_signal(&self) -> Signal<u64> {
        self.inner.changed.clone()
    }

    /// The read-only shipped styles.
    pub fn builtin_presets(&self) -> Vec<Preset> {
        builtin_presets()
    }

    /// The user's editable styles.
    pub fn user_presets(&self) -> Vec<Preset> {
        self.inner.service.user_presets()
    }

    /// Built-ins ∪ user styles — the full catalogue the Export panel's style picker offers.
    pub fn all_presets(&self) -> Vec<Preset> {
        let mut v = builtin_presets();
        v.extend(self.inner.service.user_presets());
        v
    }

    /// Whether `id` names a read-only built-in (so the pane hides Edit/Delete for it).
    pub fn is_builtin(&self, id: &str) -> bool {
        builtin_presets().iter().any(|p| p.id == id)
    }

    /// The user preset with `id`, if any (the editor reads this to seed its fields).
    pub fn user_preset(&self, id: &str) -> Option<Preset> {
        self.inner.service.user_preset(id)
    }

    /// The `Reloadable` hook for the app's shared `SettingsRegistry` (keep the returned handle
    /// alive) so a peer process's style write refreshes this handle in place.
    pub fn settings_reloadable(&self) -> Rc<dyn bastyde::settings::Reloadable> {
        self.inner.service.as_reloadable()
    }

    // ── mutators ──

    fn bump(&self) {
        let c = &self.inner.changed;
        c.set(c.get().wrapping_add(1));
    }

    /// Duplicate any preset (typically a built-in) into a fresh editable user copy, appending a
    /// localized "copy" suffix to its name. Returns the stored copy (with its final id), or
    /// `None` if the write failed.
    pub fn duplicate(&self, base: &Preset, copy_suffix: &str) -> Option<Preset> {
        let mut copy = base.clone();
        copy.builtin = false;
        copy.source = Some(base.id.clone());
        copy.name = format!("{} {copy_suffix}", base.name);
        let stored = self.inner.service.add_fresh(copy, &format!("{}-copy", base.id)).ok()?;
        self.bump();
        Some(stored)
    }

    /// Persist an edit to an existing user preset (by id).
    pub fn update(&self, preset: &Preset) {
        if self.inner.service.upsert(preset).is_ok() {
            self.bump();
        }
    }

    /// Delete a user preset.
    pub fn remove(&self, id: &str) {
        if self.inner.service.remove(id).is_ok() {
            self.bump();
        }
    }

    /// Import a single preset from a JSON file, storing it as a new user style (fresh id on
    /// collision, `builtin = false`). Returns the stored preset.
    pub fn import_from(&self, path: &Path) -> anyhow::Result<Preset> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading {}", path.display()))?;
        let preset: Preset =
            serde_json::from_str(&text).context("this file is not a valid export style (JSON)")?;
        let preferred = if preset.id.trim().is_empty() { "imported-style" } else { &preset.id };
        let stored = self
            .inner
            .service
            .add_fresh(preset.clone(), preferred)
            .context("saving the imported style")?;
        self.bump();
        Ok(stored)
    }

    /// Export the preset with `id` (built-in or user) to a JSON file.
    pub fn export_to(&self, path: &Path, id: &str) -> anyhow::Result<()> {
        let preset = self
            .all_presets()
            .into_iter()
            .find(|p| p.id == id)
            .with_context(|| format!("no style with id {id}"))?;
        let json = serde_json::to_string_pretty(&preset).context("serializing the style")?;
        std::fs::write(path, json).with_context(|| format!("writing {}", path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vm() -> ExportStylesViewModel {
        use std::sync::atomic::{AtomicU64, Ordering};
        static N: AtomicU64 = AtomicU64::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir()
            .join(format!("skribisto-stylesvm-{}-{n}.toml", std::process::id()));
        let _ = std::fs::remove_file(&path);
        ExportStylesViewModel::new(ExportStylesService::open_at(path).unwrap())
    }

    #[test]
    fn duplicate_creates_an_editable_copy_and_bumps() {
        let vm = vm();
        let before = vm.changed_signal().get();
        let base = vm.builtin_presets().into_iter().next().unwrap();
        let copy = vm.duplicate(&base, "(copy)").expect("duplicate");
        assert!(!copy.builtin);
        assert_eq!(copy.source.as_deref(), Some(base.id.as_str()));
        assert!(copy.name.ends_with("(copy)"));
        assert_ne!(copy.id, base.id);
        assert_eq!(vm.user_presets().len(), 1);
        assert!(vm.changed_signal().get() > before, "a mutation bumps changed");
        // all_presets unions built-ins with the new user copy.
        assert_eq!(vm.all_presets().len(), vm.builtin_presets().len() + 1);
        assert!(vm.is_builtin(&base.id) && !vm.is_builtin(&copy.id));
    }

    #[test]
    fn json_export_then_import_round_trips() {
        let vm = vm();
        let path = std::env::temp_dir().join(format!("skribisto-style-export-{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let base = vm.builtin_presets().into_iter().find(|p| p.id == "manuscript-fr").unwrap();

        vm.export_to(&path, &base.id).expect("export");
        let imported = vm.import_from(&path).expect("import");
        assert!(!imported.builtin, "an imported style is editable");
        assert_eq!(imported.font_family, base.font_family);
        assert_eq!(imported.scene_break, base.scene_break, "data-enum field survives JSON");
        // Importing the same id again dodges the collision.
        let again = vm.import_from(&path).expect("second import");
        assert_ne!(again.id, imported.id);
        assert_eq!(vm.user_presets().len(), 2);
        let _ = std::fs::remove_file(&path);
    }
}
