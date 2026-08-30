// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Edit** menu — the application's one Undo, the clipboard, and the
//! find commands.
//!
//! Six of these rows are commands that already existed and had **no menu row at
//! all**: Find, Find and replace, Find next, Find previous, Replace in project,
//! and — worst — Undo and Redo, which were not even registered shortcuts and so
//! appeared in neither Help ▸ Keyboard shortcuts nor Settings ▸ Keymap, and
//! could not be rebound. So this menu is a discoverability fix as much as a
//! structural one.

use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::widgets::{MenuEntry, MenuItems};

use super::ProjectMenuParts;
use crate::edit::{UndoGroupViewModel, UndoTarget};

/// The rows of the menu, in the order they appear.
pub(super) fn menu(m: MenuItems, parts: &ProjectMenuParts) -> MenuItems {
    let group = parts.undo_group.clone();

    m.item(
        MenuEntry::new(undo_label(&group, Step::Undo))
            .enabled(group.can_undo())
            .intent("edit.undo")
            .shortcut("edit.undo"),
    )
    .item(
        MenuEntry::new(undo_label(&group, Step::Redo))
            .enabled(group.can_redo())
            .intent("edit.redo")
            .shortcut("edit.redo"),
    )
    .separator()
    .item(
        MenuEntry::new(tr!(menu_edit_cut()))
            .enabled(group.can_cut())
            .intent("edit.cut")
            .shortcut("edit.cut"),
    )
    .item(
        MenuEntry::new(tr!(menu_edit_copy()))
            .enabled(group.can_copy())
            .intent("edit.copy")
            .shortcut("edit.copy"),
    )
    .item(
        MenuEntry::new(tr!(menu_edit_paste()))
            .enabled(group.can_paste())
            .intent("edit.paste")
            .shortcut("edit.paste"),
    )
    .item(
        MenuEntry::new(tr!(menu_edit_paste_plain()))
            .enabled(group.can_paste())
            .intent("edit.paste_plain")
            .shortcut("edit.paste_plain"),
    )
    .item(
        MenuEntry::new(tr!(menu_edit_select_all()))
            .enabled(group.can_select_all())
            .intent("edit.select_all")
            .shortcut("edit.select_all"),
    )
    .separator()
    // Registered commands that had no menu row until now. Gated by `.enabled`
    // rather than `.visible`, like the scene-break rows: a row that vanishes
    // teaches nobody that the command exists.
    .item(
        MenuEntry::new(tr!(menu_edit_find()))
            .intent("editor.find")
            .shortcut("editor.find"),
    )
    .item(
        MenuEntry::new(tr!(menu_edit_find_next()))
            .intent("editor.find_next")
            .shortcut("editor.find_next"),
    )
    .item(
        MenuEntry::new(tr!(menu_edit_find_prev()))
            .intent("editor.find_prev")
            .shortcut("editor.find_prev"),
    )
    .item(
        MenuEntry::new(tr!(menu_edit_replace()))
            .intent("editor.replace")
            .shortcut("editor.replace"),
    )
    .separator()
    .item(
        MenuEntry::new(tr!(menu_edit_find_in_project()))
            .intent("search.show")
            .shortcut("search.show"),
    )
    .item(
        MenuEntry::new(tr!(menu_edit_replace_in_project()))
            .intent("search.replace")
            .shortcut("search.replace"),
    )
}

#[derive(Clone, Copy)]
enum Step {
    Undo,
    Redo,
}

/// The row's label, following what the command would actually reach.
///
/// A bare "Undo" on a stack where prose and structure both appear leaves the
/// writer guessing whether the next press retypes a word or resurrects a
/// chapter. The label is computed from the *route*, not from the active domain,
/// so it already says "Undo trashing “Chapter 3”" at the moment the writer's
/// own document runs out of history — before anything is pressed.
///
/// Reactive through [`LocalizedString::also_observing`]: the target signal is a
/// dependency of the string, so both the in-window menu and the native macOS
/// bar re-resolve it when the history moves, without rebuilding the menu.
fn undo_label(group: &UndoGroupViewModel, step: Step) -> LocalizedString {
    let target = match step {
        Step::Undo => group.undo_target(),
        Step::Redo => group.redo_target(),
    };
    let read = target.clone();
    teksilo::i18n::localized(move || match read.get() {
        UndoTarget::Frozen => match step {
            Step::Undo => tr!(undo_frozen()),
            Step::Redo => tr!(redo_frozen()),
        }
        .resolve_now(),
        UndoTarget::Nothing => match step {
            Step::Undo => tr!(menu_edit_undo()),
            Step::Redo => tr!(menu_edit_redo()),
        }
        .resolve_now(),
        named => {
            let what = crate::edit::target_text(&named).resolve_now();
            match step {
                Step::Undo => tr!(menu_edit_undo_target(target = what)),
                Step::Redo => tr!(menu_edit_redo_target(target = what)),
            }
            .resolve_now()
        }
    })
    .also_observing(&target)
}
