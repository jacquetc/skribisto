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
/// refuses what takes text *away*, which the commands that delete ask about
/// themselves (`EditCommandKind::Cut`, `EditCommandKind::DeleteNext`).
pub fn accepts_writes(handle: &EditorHandle) -> bool {
    handle.command_filter().accepts(EditCommandKind::InsertChar)
}
