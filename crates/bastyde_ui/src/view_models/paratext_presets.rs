// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Settings ▸ Paratext pane's logic: the preset catalogue, and editing it.
//!
//! Mirrors [`ExportStylesViewModel`](crate::view_models::ExportStylesViewModel) in shape,
//! because it is the same problem — a list of shipped, read-only presets alongside a list
//! of the writer's own, with the pane reacting to writes through a bumped signal.
//!
//! What differs is the editing surface. An export style is a form of typed fields; a
//! paratext preset is a small TOML document whose whole content is a name and two lists,
//! so it is edited **as text**, in a modal code editor. That also means this view-model
//! validates: the pane must refuse to save something the loader would later skip, because
//! a preset that vanishes from the picker with no explanation is worse than one that
//! refuses to save with a parse error attached.

use std::rc::Rc;

use bastyde::core::signal::Signal;

use crate::models::{ParatextPreset, ParatextPresetsService};

/// One row of the pane's list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresetRow {
    /// The parsed preset, when it parses. `None` for a user entry that no longer does —
    /// which must still be listed, or a writer whose file broke sees the preset silently
    /// disappear with nowhere to go and fix it.
    pub preset: Option<ParatextPreset>,
    /// Its index among the *user* presets, or `None` for a bundled one.
    pub user_index: Option<usize>,
    /// The raw TOML, which is what the editor edits.
    pub source: String,
    /// The parse error, for a user entry that does not parse.
    pub error: Option<String>,
}

impl PresetRow {
    pub fn name(&self) -> String {
        self.preset
            .as_ref()
            .map(|p| p.name.clone())
            .unwrap_or_default()
    }

    pub fn editable(&self) -> bool {
        self.user_index.is_some()
    }
}

#[derive(Clone)]
pub struct ParatextPresetsViewModel {
    inner: Rc<Inner>,
}

struct Inner {
    service: ParatextPresetsService,
    /// Bumped on every mutation, so the pane's list and the row count re-derive.
    changed: Signal<u64>,
}

impl ParatextPresetsViewModel {
    pub fn new(service: ParatextPresetsService) -> Self {
        Self {
            inner: Rc::new(Inner {
                service,
                changed: Signal::new(0),
            }),
        }
    }

    pub fn changed_signal(&self) -> Signal<u64> {
        self.inner.changed.clone()
    }

    /// Every row the pane lists: the shipped presets first, then the writer's own —
    /// including any that no longer parse, each carrying its error so the pane can say
    /// what is wrong and let them open it.
    pub fn rows(&self) -> Vec<PresetRow> {
        let mut rows: Vec<PresetRow> = ParatextPresetsService::bundled()
            .into_iter()
            .map(|p| PresetRow {
                source: String::new(),
                preset: Some(p),
                user_index: None,
                error: None,
            })
            .collect();

        for (i, source) in self.inner.service.raw_user_presets().into_iter().enumerate() {
            match ParatextPreset::parse(&format!("user-{i}"), &source, true) {
                Ok(p) => rows.push(PresetRow {
                    preset: Some(p),
                    user_index: Some(i),
                    source,
                    error: None,
                }),
                Err(e) => rows.push(PresetRow {
                    preset: None,
                    user_index: Some(i),
                    source,
                    error: Some(e),
                }),
            }
        }
        rows
    }

    pub fn len(&self) -> usize {
        ParatextPresetsService::bundled().len() + self.inner.service.raw_user_presets().len()
    }

    /// The TOML a bundled preset is made of, so *duplicate and edit* has something to
    /// start from. Bundled presets are read-only, and this is how a writer bases their
    /// own on one rather than retyping it.
    pub fn bundled_source(&self, id: &str) -> Option<String> {
        ParatextPresetsService::bundled_source(id).map(str::to_string)
    }

    /// Validate without saving — what the modal's Save button asks before it commits.
    ///
    /// The pane refuses rather than writing something the loader would skip: a preset
    /// that quietly vanishes from the picker is a worse failure than one that will not
    /// save and says why.
    pub fn validate(&self, source: &str) -> Result<(), String> {
        ParatextPreset::parse("draft", source, true).map(|_| ())
    }

    /// Save a new user preset. Refuses invalid TOML.
    pub fn add(&self, source: &str) -> Result<(), String> {
        self.validate(source)?;
        self.inner
            .service
            .add_user_preset(source)
            .map_err(|e| e.to_string())?;
        self.bump();
        Ok(())
    }

    /// Overwrite a user preset in place. Refuses invalid TOML.
    pub fn replace(&self, index: usize, source: &str) -> Result<(), String> {
        self.validate(source)?;
        self.inner
            .service
            .replace_user_preset(index, source)
            .map_err(|e| e.to_string())?;
        self.bump();
        Ok(())
    }

    /// Delete a user preset. Bundled presets have no index and cannot reach this.
    pub fn remove(&self, index: usize) -> Result<(), String> {
        self.inner
            .service
            .remove_user_preset(index)
            .map_err(|e| e.to_string())?;
        self.bump();
        Ok(())
    }

    fn bump(&self) {
        let c = &self.inner.changed;
        c.set(c.get().wrapping_add(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::NEW_PRESET_TEMPLATE;

    /// A private directory per call. Keyed on a counter, not a stack address: `&0u8`
    /// resolves to the same pointer in two threads' frames often enough that the tests
    /// raced each other's files and one of them failed about one run in three.
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

    fn vm() -> (ParatextPresetsViewModel, std::path::PathBuf) {
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir = std::env::temp_dir()
            .join(format!("skrib-paratext-vm-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("p.toml");
        let svc = ParatextPresetsService::open_at(path.clone()).unwrap();
        (ParatextPresetsViewModel::new(svc), dir)
    }

    /// Invalid TOML is refused rather than written. The alternative is a preset that
    /// disappears from the picker on next load with nothing to explain it.
    #[test]
    fn an_invalid_preset_is_refused_not_saved() {
        let (vm, dir) = vm();
        let before = vm.len();
        let err = vm.add("this is not toml [[[").unwrap_err();
        assert!(!err.is_empty(), "the parse error must reach the pane");
        assert_eq!(vm.len(), before, "nothing was written");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A user preset that broke since it was written is still listed, with its error, so
    /// there is somewhere to go and fix it.
    #[test]
    fn a_broken_user_preset_is_listed_with_its_error() {
        let (vm, dir) = vm();
        // Written past `add`'s validation, as a hand-edited file could be.
        vm.inner.service.add_user_preset("nonsense [[[").unwrap();

        let rows = vm.rows();
        let broken = rows.last().unwrap();
        assert!(broken.preset.is_none());
        assert!(broken.error.is_some(), "its error must be shown");
        assert_eq!(broken.user_index, Some(0), "and it must be editable");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Bundled presets are listed but never editable — the pane offers duplicate instead.
    #[test]
    fn bundled_presets_are_listed_and_not_editable() {
        let (vm, dir) = vm();
        let rows = vm.rows();
        assert_eq!(rows.len(), ParatextPresetsService::bundled().len());
        assert!(rows.iter().all(|r| !r.editable()));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Duplicate-and-edit needs the shipped source to start from.
    #[test]
    fn a_bundled_presets_source_is_available_to_duplicate() {
        let (vm, dir) = vm();
        let src = vm.bundled_source("roman-francais").expect("its source");
        assert!(src.contains("Roman français"));
        assert!(vm.validate(&src).is_ok(), "and it round-trips");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The full edit cycle: add, change, delete.
    #[test]
    fn a_user_preset_can_be_added_edited_and_deleted() {
        let (vm, dir) = vm();
        vm.add(NEW_PRESET_TEMPLATE).unwrap();
        assert_eq!(vm.rows().last().unwrap().name(), "My structure");

        let edited = NEW_PRESET_TEMPLATE.replace("My structure", "Mine");
        vm.replace(0, &edited).unwrap();
        assert_eq!(vm.rows().last().unwrap().name(), "Mine");

        vm.remove(0).unwrap();
        assert_eq!(vm.rows().len(), ParatextPresetsService::bundled().len());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
