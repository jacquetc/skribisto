// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Whether a command may write into an editor's text.
//!
//! A trashed item's text is shown in a read-only editor, so the writer can still
//! read, select and copy it. teksilo enforces read-only on the editor's *input*
//! paths (keys, the input method, drops), not on `EditorHandle`: its
//! `insert_text`, `insert_djot`, `toggle_bold`, `cut` and `paste` write straight to
//! the cursor, because that is the API a version restore goes through (see the
//! `read_only` parameter of `tabs::shared::editor`). Every command that writes
//! through a handle therefore asks
//! [`accepts_writes`](crate::shared::editor_writes::accepts_writes) first, and the
//! row, button or menu item that runs it is enabled on the same answer. Before this, the Format
//! menu, the Format dock, the editor's context menu, Insert image, Insert footnote,
//! the note templates and the scene breaks all wrote into a trashed scene's text
//! through its handle.
//!
//! The answer is only as good as the editor's filter. The writing games used to
//! push their own (`All` while no game is played) onto every editor, a read-only
//! one included, which handed the trashed text to the keyboard as well; a
//! read-only editor now keeps `ReadOnly` (see `TypographyBoundEditor`'s games
//! wiring).

use teksilo::widgets::rich_text::{EditCommandKind, EditorHandle};

/// Whether `handle`'s editor takes commands that change its text.
///
/// `false` only for a read-only editor. "Always forward" still takes them: it
/// refuses what takes text *away*, which a command that deletes asks about with
/// [`accepts_deletions`], and one that inserts over a selection answers with
/// [`collapse_selection_before_insert`].
pub fn accepts_writes(handle: &EditorHandle) -> bool {
    handle.command_filter().accepts(EditCommandKind::InsertChar)
}

/// Whether `handle`'s editor takes commands that take text *away*: deleting a table
/// row, replacing a misspelled word with a correction, renaming a link's words.
///
/// `false` for a read-only editor and under "Always forward", whose whole rule is
/// that nothing the writer committed can be taken back. The same question its
/// keyboard asks of Delete.
pub fn accepts_deletions(handle: &EditorHandle) -> bool {
    handle.command_filter().accepts(EditCommandKind::DeleteNext)
}

/// Before inserting through `handle`, move a selection's caret to its end when the
/// editor's filter forbids replacing it, so the insertion lands after the selected
/// words instead of in their place.
///
/// Every insertion through an `EditorHandle` replaces the selection, a deletion no
/// command filter sees. teksilo collapses first at its own insert sites
/// (`CommandFilter::collapses_selection_before_insert`); a command that inserts
/// through the handle (a formatting mark, a picture, a scene break, a template)
/// has to ask for itself, or "Always forward" loses the selected words to it.
pub fn collapse_selection_before_insert(handle: &EditorHandle) {
    if !handle.command_filter().collapses_selection_before_insert() {
        return;
    }
    let (anchor, position) = handle.selection();
    if anchor != position {
        let end = anchor.max(position);
        handle.select_range(end, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::text_document::TextDocument;
    use teksilo::widgets::rich_text::{CommandFilter, RichTextEditor};

    fn editor(filter: CommandFilter) -> (EditorHandle, RichTextEditor) {
        let doc = TextDocument::new();
        doc.set_plain_text("Roses are red").unwrap();
        let editor = RichTextEditor::editor(doc);
        let handle = editor.handle();
        handle.set_command_filter(filter);
        (handle, editor)
    }

    /// What each filter lets a command do: write, and take away.
    #[test]
    fn the_three_filters_answer_as_their_keys_do() {
        for (filter, writes, deletes) in [
            (CommandFilter::All, true, true),
            (CommandFilter::ForwardOnly, true, false),
            (CommandFilter::ReadOnly, false, false),
        ] {
            let (handle, _editor) = editor(filter);
            assert_eq!(accepts_writes(&handle), writes, "{filter:?} writes");
            assert_eq!(accepts_deletions(&handle), deletes, "{filter:?} deletes");
        }
    }

    /// Only "Always forward" moves the caret to the selection's end, whichever way
    /// the selection was dragged; elsewhere an insertion replaces it, as typing does.
    #[test]
    fn a_selection_collapses_to_its_end_under_always_forward_only() {
        let (handle, _editor) = editor(CommandFilter::ForwardOnly);
        handle.select_range(9, 6);
        collapse_selection_before_insert(&handle);
        assert_eq!(handle.selection(), (9, 9), "dragged right to left");
        handle.select_range(6, 9);
        collapse_selection_before_insert(&handle);
        assert_eq!(handle.selection(), (9, 9), "dragged left to right");

        let (handle, _editor) = editor(CommandFilter::All);
        handle.select_range(6, 9);
        collapse_selection_before_insert(&handle);
        assert_eq!(handle.selection(), (6, 9), "left alone with no game");
    }
}
