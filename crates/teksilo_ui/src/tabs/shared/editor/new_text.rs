// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Editor ▸ Scene / Synopsis / Notes ▸ *Start justified*: the paragraph a
//! writer begins an empty text with is justified.
//!
//! Only the first paragraph is set. Enter copies a paragraph's alignment into the one it
//! opens, so the text stays justified as it grows, and the writer can still set any
//! paragraph otherwise.
//!
//! **At the first keystroke, not when the text opens.** Justifying an empty text as it
//! is shown would write into every empty item the writer only looked at: the item
//! turns modified and is saved, and its undo history opens on a change the writer never
//! made. Waiting for the text to stop being empty touches only what the writer writes.
//! The alignment is then its own step in the history, after the first letter, so Undo
//! right away takes the justification back off and leaves the letter, the way an
//! autocorrection is undone.

use std::cell::Cell;

use teksilo::prelude::Signal;
use teksilo::text_document::{Alignment, TextDocument};
use teksilo::widgets::rich_text::EditorHandle;

/// Watches one editor for the moment its text stops being empty.
///
/// Driven from the frame tick (see `bound_editor`), never from `on_change`: that
/// callback runs while the editor's state is borrowed, and every `EditorHandle`
/// method panics there.
///
/// It reads the document itself, not the editor's signals, for the history: the
/// editor's `can_redo` signal is debounced, so for a few frames after an Undo it
/// still says there is nothing to redo.
pub(super) struct NewTextJustifier {
    doc: TextDocument,
    /// The Scene, Synopsis or Notes switch, read live so turning it on reaches the
    /// editors already open.
    wanted: Signal<bool>,
    /// The content revision last looked at, so a frame with no edit costs one read.
    seen: Cell<u64>,
    /// Whether the text was empty at that revision.
    empty: Cell<bool>,
}

impl NewTextJustifier {
    pub(super) fn new(doc: TextDocument, wanted: Signal<bool>) -> Self {
        Self {
            seen: Cell::new(doc.content_revision()),
            empty: Cell::new(doc.is_empty()),
            doc,
            wanted,
        }
    }

    /// Justify the paragraph if this edit is the one that began the text.
    pub(super) fn tick(&self, handle: &EditorHandle) {
        let revision = self.doc.content_revision();
        if self.seen.get() == revision {
            return;
        }
        // An IME still composing holds provisional text: decide once it commits,
        // which is a later revision, so the transition is still there to see.
        if handle.is_composing() {
            return;
        }
        self.seen.set(revision);
        let empty = self.doc.is_empty();
        let began = self.empty.replace(empty) && !empty;
        if began && self.wanted.get() && self.begun_by_the_writer(handle) && plain_paragraph(handle)
        {
            handle.set_alignment(Alignment::Justify);
        }
    }

    /// The writer typed or pasted it into this editor.
    ///
    /// Focus rules out the other ways an empty text fills: a document loaded or
    /// restored under the editor, and the other view of a text open in two places,
    /// which sees the same change but is not where the writer is. Something left
    /// to redo rules out an Undo or a Redo, which bring back a text as it was: an
    /// edit made then would also throw away what was left to redo.
    fn begun_by_the_writer(&self, handle: &EditorHandle) -> bool {
        handle.focused_signal().get() && !self.doc.can_redo()
    }
}

/// A single body paragraph whose alignment nobody has chosen.
///
/// A heading, a list item or a table cell is left alone. So is a paste of several
/// paragraphs, which brings its own layout and would otherwise come out justified
/// in its last paragraph only. `Left` is also what an unset alignment reads as, so
/// a paragraph already centred or justified (a template's) keeps its own.
fn plain_paragraph(handle: &EditorHandle) -> bool {
    handle.get_alignment() == Alignment::Left
        && handle.get_heading_level() == 0
        && handle.current_list_style().is_none()
        && !handle.is_in_table()
        && !handle.to_plain_text().contains('\n')
}

#[cfg(test)]
#[path = "new_text_tests.rs"]
mod tests;
