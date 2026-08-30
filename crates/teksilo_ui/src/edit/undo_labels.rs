// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Turning the backend's machine keys into something a writer reads.
//!
//! The generated undo commands name themselves with an [`UndoLabel`] — a pair
//! of `&'static str`s such as `("binder_item", "remove")`. That is deliberately
//! *not* a sentence: the backend is generated, knows nothing about locales, and
//! must not carry the application's wording. The translation happens here, once,
//! so every surface that names an undo says the same thing in the same language.
//!
//! An unrecognised key falls back to the generic phrase rather than showing the
//! key itself. A row reading *"Undo binder_item/remove"* would be worse than one
//! reading *"Undo the last change to this project"* — the second is vague, the
//! first is broken.

use frontend::common::undo_redo::UndoLabel;
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;

use super::UndoTarget;

/// What the Edit menu should call this target, in the reader's language.
pub(crate) fn target_text(target: &UndoTarget) -> LocalizedString {
    match target {
        UndoTarget::Typing => tr!(undo_target_typing()),
        UndoTarget::Entity(Some(label)) => entity_text(label),
        UndoTarget::Entity(None) | UndoTarget::Nothing | UndoTarget::Frozen => {
            tr!(undo_target_project())
        }
    }
}

/// One machine key → one phrase.
///
/// Matched on the *action* first and the subject second, because the verb is
/// what a writer recognises: "deleting" reads the same whether the row was a
/// scene or a note, and a table with one entry per (entity × action) pair would
/// be forty rows of near-duplicates that drift apart in translation.
fn entity_text(label: &UndoLabel) -> LocalizedString {
    match (label.subject, label.action) {
        // The feature use cases name the whole act, so they are their own row.
        ("trash_binder_items" | "trash_binder" | "trash_selection", _) => tr!(undo_target_trash()),
        ("restore_items" | "restore_items_to", _) => tr!(undo_target_restore()),
        ("empty_trash" | "delete_trash_entries", _) => tr!(undo_target_delete_forever()),
        ("replace_in_project", _) => tr!(undo_target_replace_all()),
        ("apply_document_import", _) => tr!(undo_target_import()),
        ("duplicate", _) => tr!(undo_target_duplicate()),
        ("move_items", _) => tr!(undo_target_move()),
        ("merge_two_scenes", _) => tr!(undo_target_merge()),
        ("split_scene", _) => tr!(undo_target_split()),
        ("promote", _) => tr!(undo_target_promote()),
        ("clear_titles", _) => tr!(undo_target_tidy_titles()),
        ("import_tags", _) => tr!(undo_target_import_tags()),
        ("import_note_templates", _) => tr!(undo_target_import_templates()),

        // Entity CRUD: the verb carries it.
        (_, "create") => tr!(undo_target_create()),
        (_, "remove") => tr!(undo_target_remove()),
        ("binder_item" | "binder", "update") => tr!(undo_target_rename()),
        (_, "update" | "update_with_relationships") => tr!(undo_target_edit()),
        (_, "set_relationship" | "move_relationship") => tr!(undo_target_move()),

        _ => tr!(undo_target_project()),
    }
}
