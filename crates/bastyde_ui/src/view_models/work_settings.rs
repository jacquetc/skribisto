// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `WorkSettingsViewModel` — the two Settings pages that edit the **open project** rather
//! than the application.
//!
//! Everything else under Settings writes to the settings store through
//! [`SettingsViewModel`](super::SettingsViewModel). These two write to the `Work` *entity*:
//! its chapter encoding and its default spell-check language. That difference is why they
//! were the only panes reaching past the view-model layer — they called
//! `SingleWork::set_*` followed by `save()` straight from the pane body, which is the one
//! shape the rest of the settings panel does not have.
//!
//! ## Set-then-save is one operation
//!
//! Neither field is meaningful half-applied: `set_chapter_mode` alone leaves the store
//! disagreeing with disk until something else happens to save, and the panel has no "apply"
//! button to make that visible. Pairing them here means a caller cannot forget the save, and
//! there is one place to change if these ever need to batch.
//!
//! The `stack` is read at call time, not captured: it is the open project's undo stack, and
//! it changes when the project does.

use bastyde::prelude::Signal;

use skribisto_model::ChapterMode;

use crate::singles::SingleWork;

/// Cloneable handle over the open `Work` plus its undo stack.
#[derive(Clone)]
pub struct WorkSettingsViewModel {
    work: SingleWork,
    stack: Signal<Option<u64>>,
}

impl WorkSettingsViewModel {
    pub fn new(work: SingleWork, stack: Signal<Option<u64>>) -> Self {
        Self { work, stack }
    }

    // ── Chapter encoding ─────────────────────────────────────────────────────

    /// The project's live chapter mode — `Folder` (a chapter is a folder with child scenes)
    /// or `Flat` (a chapter is one `ChapterScene` row).
    pub fn chapter_mode(&self) -> Signal<ChapterMode> {
        self.work.chapter_mode()
    }

    /// Is the project on flat chapters? The toggle's own boolean view of [`Self::chapter_mode`].
    pub fn flat_chapters(&self) -> bool {
        matches!(self.chapter_mode().get(), ChapterMode::Flat)
    }

    /// Switch encoding, and persist.
    ///
    /// A no-op when already in that mode — the pane drives this from an effect on a mirrored
    /// `Signal<bool>`, which fires on every rebuild, and an unconditional write would queue a
    /// pointless undo entry and a disk save each time Settings is opened.
    pub fn set_flat_chapters(&self, flat: bool) {
        let want = if flat {
            ChapterMode::Flat
        } else {
            ChapterMode::Folder
        };
        if self.work.chapter_mode().get() == want {
            return;
        }
        self.work.set_chapter_mode(want);
        self.work.save(self.stack.get());
    }

    // ── Default language ─────────────────────────────────────────────────────

    /// The project's default spell-check language list (a `dict_language` tag string).
    pub fn dict_language(&self) -> Signal<String> {
        self.work.dict_language()
    }

    /// Replace the language list, and persist.
    pub fn set_dict_language(&self, languages: String) {
        self.work.set_dict_language(languages);
        self.work.save(self.stack.get());
    }
}
