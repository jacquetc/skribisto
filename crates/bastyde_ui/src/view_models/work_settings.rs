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

use frontend::common::entities::QuoteStyle;
use skribisto_model::ChapterMode;

use crate::singles::{SingleSmartPunctuation, SingleWork};

/// Cloneable handle over the open `Work` plus its undo stack.
#[derive(Clone)]
pub struct WorkSettingsViewModel {
    work: SingleWork,
    /// The project's punctuation house style. A sibling entity rather than a
    /// field of `Work`, so it saves through its own handle — and on the same
    /// undo stack, so flipping a switch by accident comes back with Ctrl+Z like
    /// any other project edit.
    punctuation: SingleSmartPunctuation,
    stack: Signal<Option<u64>>,
}

impl WorkSettingsViewModel {
    pub fn new(
        work: SingleWork,
        punctuation: SingleSmartPunctuation,
        stack: Signal<Option<u64>>,
    ) -> Self {
        Self {
            work,
            punctuation,
            stack,
        }
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

    // ── Numbering ────────────────────────────────────────────────────────────

    /// Does this book number its chapters and parts at all?
    ///
    /// Manuscript data, not a machine preference — it rides in the `.skrib`, so a co-author
    /// opening the project gets the same answer, exactly as `chapter_mode` does. It also
    /// reaches the *exported file*: the compiler clamps an export style's heading scheme
    /// when this is off, so "off" cannot mean "off in the app but numbered on disk".
    pub fn number_chapters(&self) -> Signal<bool> {
        self.work.number_chapters()
    }

    /// Whether a new Part restarts chapter numbering.
    ///
    /// Off by default, which is the trade convention: chapters run continuously across the
    /// parts of one book, so "Part Two" opens on "Chapter Eleven". Exposed rather than
    /// hardcoded because the in-world "Book Two, Chapter One" framing is real.
    pub fn part_resets_chapter(&self) -> Signal<bool> {
        self.work.part_resets_chapter()
    }

    /// Persist the numbering master switch. No-op guarded like [`Self::set_flat_chapters`],
    /// and for the same reason: the pane drives this from an effect that fires on every
    /// rebuild, and an unconditional write would queue an undo entry and a disk save each
    /// time Settings is opened.
    pub fn set_number_chapters(&self, on: bool) {
        if self.work.number_chapters().get() == on {
            return;
        }
        self.work.set_number_chapters(on);
        self.work.save(self.stack.get());
    }

    /// Persist the part-reset rule. Same no-op guard as above.
    pub fn set_part_resets_chapter(&self, on: bool) {
        if self.work.part_resets_chapter().get() == on {
            return;
        }
        self.work.set_part_resets_chapter(on);
        self.work.save(self.stack.get());
    }

    // ── Author ───────────────────────────────────────────────────────────────

    /// The writer's name, as it appears on the compiled title page and in the
    /// exported metadata (EPUB `dc:creator`, DOCX `creator`, the PDF author).
    pub fn author_name(&self) -> Signal<String> {
        self.work.author_name()
    }

    /// Replace the author name, and persist.
    ///
    /// Trimmed, and a no-op when unchanged — the pane commits on blur/submit, so
    /// tabbing through the field without editing it must not queue an undo entry
    /// and a disk save. Empty is a legal value: the name is optional, and clearing
    /// it must be as saveable as setting it.
    pub fn set_author_name(&self, name: String) {
        let name = name.trim().to_string();
        if self.work.author_name().get() == name {
            return;
        }
        self.work.set_author_name(name);
        self.work.save(self.stack.get());
    }

    // ── Default language ─────────────────────────────────────────────────────

    /// The project's default spell-check language list.
    pub fn dict_language(&self) -> Signal<Vec<String>> {
        self.work.dict_language()
    }

    /// Replace the language list, and persist.
    pub fn set_dict_language(&self, languages: Vec<String>) {
        self.work.set_dict_language(languages);
        self.work.save(self.stack.get());
    }

    // ── Punctuation house style ──────────────────────────────────────────────

    /// Whether this project overrides the application's punctuation preference.
    ///
    /// While false the five switches below are inert — kept, not cleared, so
    /// turning the override back on restores what the writer had configured
    /// rather than making them set it all up again.
    pub fn punctuation_override(&self) -> Signal<bool> {
        self.punctuation.override_app_default()
    }
    pub fn smart_dashes(&self) -> Signal<bool> {
        self.punctuation.dashes()
    }
    pub fn smart_ellipsis(&self) -> Signal<bool> {
        self.punctuation.ellipsis()
    }
    pub fn smart_quotes(&self) -> Signal<bool> {
        self.punctuation.quotes()
    }
    pub fn quote_style(&self) -> Signal<QuoteStyle> {
        self.punctuation.quote_style()
    }
    pub fn pre_punctuation_spacing(&self) -> Signal<bool> {
        self.punctuation.pre_punctuation_spacing()
    }
    pub fn dialogue_marker(&self) -> Signal<bool> {
        self.punctuation.dialogue_marker()
    }

    /// Write one punctuation switch and persist.
    ///
    /// Every setter here carries the same no-op guard as `set_flat_chapters`,
    /// and for the same reason: the pane drives these from effects over mirrored
    /// signals, which fire on every rebuild. Without the guard, merely opening
    /// Settings would queue six undo entries and six disk saves.
    pub fn set_punctuation_override(&self, on: bool) {
        if self.punctuation.override_app_default().get() == on {
            return;
        }
        self.punctuation.set_override_app_default(on);
        self.punctuation.save(self.stack.get());
    }
    pub fn set_smart_dashes(&self, on: bool) {
        if self.punctuation.dashes().get() == on {
            return;
        }
        self.punctuation.set_dashes(on);
        self.punctuation.save(self.stack.get());
    }
    pub fn set_smart_ellipsis(&self, on: bool) {
        if self.punctuation.ellipsis().get() == on {
            return;
        }
        self.punctuation.set_ellipsis(on);
        self.punctuation.save(self.stack.get());
    }
    pub fn set_smart_quotes(&self, on: bool) {
        if self.punctuation.quotes().get() == on {
            return;
        }
        self.punctuation.set_quotes(on);
        self.punctuation.save(self.stack.get());
    }
    pub fn set_quote_style(&self, style: QuoteStyle) {
        if self.punctuation.quote_style().get() == style {
            return;
        }
        self.punctuation.set_quote_style(style);
        self.punctuation.save(self.stack.get());
    }
    pub fn set_pre_punctuation_spacing(&self, on: bool) {
        if self.punctuation.pre_punctuation_spacing().get() == on {
            return;
        }
        self.punctuation.set_pre_punctuation_spacing(on);
        self.punctuation.save(self.stack.get());
    }
    pub fn set_dialogue_marker(&self, on: bool) {
        if self.punctuation.dialogue_marker().get() == on {
            return;
        }
        self.punctuation.set_dialogue_marker(on);
        self.punctuation.save(self.stack.get());
    }
}
