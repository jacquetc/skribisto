// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `DistractionFreeThemesViewModel` — the library behind Settings ▸ Editor ▸
//! **Distraction-free themes** and the mode's own quick-access picker.
//!
//! **App-local, not a Qleany feature**, exactly like
//! [`crate::view_models::ExportStylesViewModel`], which this mirrors almost
//! line for line: a theme is a machine-wide preference that outlives any `Work`,
//! is not undoable, and never touches the entity store. Single-instance live
//! state (the service over `distraction_free_themes.toml` plus a `changed`
//! bump), created once in `main.rs`, registered as `app_state`, shared by
//! `.clone()`.
//!
//! Built-ins ([`crate::distraction_free::theme::builtin_themes`]) are read-only;
//! the writer works with **duplicates** and portable **JSON** import/export.
//!
//! **The current theme is persisted**, unlike the export panel's chosen preset —
//! which is a deliberately session-lived `Signal`, right for a per-export choice
//! and wrong for the environment somebody writes in.
//!
//! It is **not owned here**, though: it is an ordinary key on
//! `SettingsViewModel`, like every other setting, and this library only knows
//! how to [`resolve`](DistractionFreeThemesViewModel::resolve) an id into a theme. That is not fastidious
//! layering — this view-model is built in `main`, before any widget tree, and a
//! `SettingsStore` handle opened there would have its *own* signals. The
//! settings pane (which reads `ctx.settings()`) would then never see a change
//! the popover made, or the other way round. One key, one live handle, no
//! staleness.

use std::path::Path;
use std::rc::Rc;

use anyhow::Context;
use teksilo::prelude::*;

use crate::distraction_free::theme::{DistractionFreeTheme, builtin_themes};
use crate::models::DistractionFreeThemesService;

#[derive(Clone)]
pub struct DistractionFreeThemesViewModel {
    inner: Rc<Inner>,
}

struct Inner {
    service: DistractionFreeThemesService,
    /// Bumped on every mutation, so the settings lists and the picker re-derive.
    changed: Signal<u64>,
}

impl DistractionFreeThemesViewModel {
    pub fn new(service: DistractionFreeThemesService) -> Self {
        Self {
            inner: Rc::new(Inner {
                service,
                changed: Signal::new(0),
            }),
        }
    }

    // ── read handles ──

    pub fn changed_signal(&self) -> Signal<u64> {
        self.inner.changed.clone()
    }

    /// Resolve a theme id, **always** to something paintable: an id that no
    /// longer names anything — a theme the writer deleted, or one a synced
    /// config names but this machine never imported — falls back to the first
    /// built-in rather than leaving the surface unpainted.
    pub fn resolve(&self, id: &str) -> DistractionFreeTheme {
        self.inner.service.resolve(id)
    }

    pub fn builtin_themes(&self) -> Vec<DistractionFreeTheme> {
        builtin_themes()
    }

    pub fn user_themes(&self) -> Vec<DistractionFreeTheme> {
        self.inner.service.user_themes()
    }

    /// Built-ins first, then the writer's own — the order both the settings pane
    /// and the popover list them in.
    pub fn all_themes(&self) -> Vec<DistractionFreeTheme> {
        self.inner.service.all_themes()
    }

    pub fn as_reloadable(&self) -> Rc<dyn teksilo::settings::Reloadable> {
        self.inner.service.as_reloadable()
    }

    // ── mutators ──

    fn bump(&self) {
        let c = &self.inner.changed;
        c.set(c.get().wrapping_add(1));
    }

    /// Duplicate any theme (typically a built-in) into a fresh editable copy,
    /// appending a localized "copy" suffix to its name.
    pub fn duplicate(
        &self,
        base: &DistractionFreeTheme,
        copy_suffix: &str,
    ) -> Option<DistractionFreeTheme> {
        let mut copy = base.clone();
        copy.builtin = false;
        copy.source = Some(base.id.clone());
        copy.name = format!("{} {copy_suffix}", base.name);
        let stored = self
            .inner
            .service
            .add_fresh(copy, &format!("{}-copy", base.id))
            .ok()?;
        self.bump();
        Some(stored)
    }

    /// Persist an edit to an existing user theme (by id).
    pub fn update(&self, theme: &DistractionFreeTheme) {
        if self.inner.service.upsert(theme).is_ok() {
            self.bump();
        }
    }

    /// Delete a user theme.
    ///
    /// If it was the current one, the selection is **not** rewritten here:
    /// `resolve` already answers a dangling id with a built-in, so leaving it
    /// alone means an accidental delete followed by a re-import puts the
    /// writer's choice back rather than silently having become "Paper".
    pub fn remove(&self, id: &str) {
        if self.inner.service.remove(id).is_ok() {
            self.bump();
        }
    }

    /// Import one theme from a JSON file, stored as a new user theme (fresh id
    /// on collision, `builtin = false`).
    pub fn import_from(&self, path: &Path) -> anyhow::Result<DistractionFreeTheme> {
        let text =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let theme: DistractionFreeTheme = serde_json::from_str(&text)
            .context("this file is not a valid distraction-free theme (JSON)")?;
        let preferred = if theme.id.trim().is_empty() {
            "imported-theme".to_string()
        } else {
            theme.id.clone()
        };
        let stored = self
            .inner
            .service
            .add_fresh(theme, &preferred)
            .context("saving the imported theme")?;
        self.bump();
        Ok(stored)
    }

    /// Export the theme with `id` (built-in or user) to a JSON file.
    pub fn export_to(&self, path: &Path, id: &str) -> anyhow::Result<()> {
        let theme = self
            .all_themes()
            .into_iter()
            .find(|t| t.id == id)
            .with_context(|| format!("no theme with id {id}"))?;
        let json = serde_json::to_string_pretty(&theme).context("serializing the theme")?;
        std::fs::write(path, json).with_context(|| format!("writing {}", path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn vm(dir: &std::path::Path) -> DistractionFreeThemesViewModel {
        DistractionFreeThemesViewModel::new(
            DistractionFreeThemesService::open_at(dir.join("df_themes.toml")).unwrap(),
        )
    }

    #[test]
    fn duplicating_a_builtin_yields_an_editable_copy_that_records_its_source() {
        let d = tempdir().unwrap();
        let vm = vm(d.path());
        let base = vm.builtin_themes()[0].clone();
        let copy = vm.duplicate(&base, "copy").expect("stored");
        assert!(!copy.builtin, "a copy is always editable");
        assert_eq!(copy.source.as_deref(), Some(base.id.as_str()));
        assert_ne!(copy.id, base.id);
        assert!(copy.name.ends_with("copy"));
        assert_eq!(vm.user_themes().len(), 1);
    }

    /// The round trip the pane's Import…/Export… buttons drive. Done on disk
    /// rather than in memory because that is the furthest out it can be checked
    /// at all — the UI path around it goes through a native file dialog, which
    /// no automation probe can drive.
    #[test]
    fn a_theme_exports_and_re_imports_through_a_file() {
        let d = tempdir().unwrap();
        let vm = vm(d.path());
        let path = d.path().join("sepia.json");
        vm.export_to(&path, "sepia").expect("export");

        let back = vm.import_from(&path).expect("import");
        assert_eq!(back.name, "Sepia");
        assert!(!back.builtin, "an imported theme is the writer's, not ours");
        assert_ne!(
            back.id, "sepia",
            "and must not shadow the built-in it came from"
        );
    }

    #[test]
    fn importing_something_that_is_not_a_theme_fails_with_a_readable_message() {
        let d = tempdir().unwrap();
        let vm = vm(d.path());
        let path = d.path().join("nope.json");
        std::fs::write(&path, "{\"hello\": 1}").unwrap();
        let err = vm.import_from(&path).unwrap_err();
        assert!(
            format!("{err:#}").contains("not a valid distraction-free theme"),
            "unhelpful error: {err:#}"
        );
    }

    /// Deleting the theme in force must not strand the surface. The stored
    /// *choice* is left alone — it lives in settings, not here — so an
    /// accidental delete followed by a re-import puts it back rather than having
    /// silently become "Paper"; `resolve` covers the gap meanwhile.
    #[test]
    fn a_deleted_theme_resolves_to_a_builtin() {
        let d = tempdir().unwrap();
        let vm = vm(d.path());
        let copy = vm
            .duplicate(&vm.builtin_themes()[1].clone(), "copy")
            .unwrap();
        assert_eq!(vm.resolve(&copy.id).id, copy.id);

        vm.remove(&copy.id);
        assert_eq!(
            vm.resolve(&copy.id).id,
            builtin_themes()[0].id,
            "a dangling id must resolve to a built-in, never to nothing"
        );
    }

    #[test]
    fn every_mutation_bumps_the_changed_signal() {
        let d = tempdir().unwrap();
        let vm = vm(d.path());
        let before = vm.changed_signal().get();
        let copy = vm
            .duplicate(&vm.builtin_themes()[0].clone(), "copy")
            .unwrap();
        assert_ne!(vm.changed_signal().get(), before);

        let mid = vm.changed_signal().get();
        vm.update(&copy);
        assert_ne!(vm.changed_signal().get(), mid);

        let late = vm.changed_signal().get();
        vm.remove(&copy.id);
        assert_ne!(vm.changed_signal().get(), late);
    }
}
