// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Document** menu — the binder rows, and what goes in at the caret.
//!
//! The five binder rows had no menu-bar home at all before this: they were
//! reachable only from the outline's context menu, which is a real
//! discoverability and a11y gap (a context menu is hard to reach by keyboard and
//! by screen reader). Every one of them is an already-registered
//! *selection-based* global action, so these rows fire the same intent the
//! context menu does — no second code path.
//!
//! Rows stay visible when disabled, exactly as the Format menu's do: a greyed row
//! still teaches that the feature exists.

use teksilo::prelude::*;
use teksilo::widgets::{MenuEntry, MenuItems};

use super::ProjectMenuParts;

/// The rows of the menu, in the order they appear.
pub(super) fn menu(m: MenuItems, parts: &ProjectMenuParts) -> MenuItems {
    let menu_binder_selection = parts.binder_has_selection.clone();
    let templates_submenu_id = parts.templates_submenu_id.clone();
    let menu_format_vm = parts.format.clone();

    let on_selection = menu_binder_selection.clone();
    // Any editor will do. Templates started out note-only; that restriction is
    // gone, so the gate is simply "is there somewhere to type" — which
    // `has_target` already answers, and answers *stickily*, surviving the focus
    // loss that opening this very menu causes.
    let on_editor = menu_format_vm.has_target();
    // Inserting happens *at the caret*, so this row needs more than
    // "a document tab is open" (which is all `on_editor` above means —
    // see `has_target`'s doc). Its siblings act on a whole document and
    // are right to use the broader gate.
    let on_caret = menu_format_vm.has_caret_target();

    let m = m
        .item(
            MenuEntry::new(tr!(ctx_rename()))
                .enabled(on_selection.clone())
                .intent("binder.rename"),
        )
        .item(
            MenuEntry::new(tr!(ctx_duplicate()))
                .enabled(on_selection.clone())
                .intent("binder.duplicate")
                .shortcut("binder.duplicate"),
        )
        .separator()
        .item(
            MenuEntry::new(tr!(ctx_indent()))
                .enabled(on_selection.clone())
                .intent("binder.indent"),
        )
        .item(
            MenuEntry::new(tr!(ctx_outdent()))
                .enabled(on_selection.clone())
                .intent("binder.outdent"),
        )
        .separator()
        // Work-scoped, so no selection gate: it asks before it touches
        // anything, and says so when there is nothing to tidy.
        .item(MenuEntry::new(tr!(menu_document_tidy_titles())).intent("numbering.tidy_titles"))
        .separator();

    // Insert template ▸ — the one menu in this app whose item list is DATA,
    // not a fixed set of commands.
    //
    // `MenuItems::submenu`'s builder is `FnOnce` and runs right here, at
    // window construction — long before any project is loaded. So the list
    // cannot be produced by reading the catalogue from inside it: it would
    // bake in whatever existed at that instant, which is nothing, forever.
    // (It did exactly that.)
    //
    // The framework's answer is the `Open Recent` pattern from
    // `teksilo/docs/native-menu.md` §"Dynamic structure": give the submenu a
    // **pre-allocated id**, leave it empty here, and repopulate it through the
    // model's `&self` mutators whenever the data changes. Each mutation bumps
    // `MenuModel::version`, which a `from_model` bar binds at `Rebuild` level,
    // so the dropdown re-derives (and the native menu re-installs) on its own.
    // `sync_insert_template_submenu` below is the repopulation; `App::build`
    // drives it from the catalogue's own change signal.
    let m = m.submenu_with_id(templates_submenu_id, tr!(menu_insert_template()), |t| t);

    // Inserting a picture is not a template action. It shares the
    // "put something at the caret" idea with the submenu above, but
    // a template is prose the writer wrote and an image is a file
    // from outside the project — its own row, on its own side of a
    // separator, and it stays here rather than moving to the Image
    // menu because that menu only exists once an image is selected,
    // which is exactly when you are not inserting one.
    // A footnote goes at the caret like the two rows above it, and
    // belongs beside them for that reason — but it is neither a
    // template nor a file: it is a piece of the book being written
    // here, whose words live in the project and whose number the
    // manuscript decides.
    let m = m.separator().item(
        MenuEntry::new(tr!(footnotes_insert()))
            .enabled(on_caret.clone())
            .shortcut("editor.insert_footnote")
            .intent("editor.insert_footnote"),
    );

    let m = m.item(
        MenuEntry::new(tr!(image_insert()))
            .enabled(on_caret.clone())
            .intent("editor.insert_image"),
    );

    let m = m.separator();

    m.item(
        MenuEntry::new(tr!(menu_save_as_template()))
            .enabled(on_editor.clone())
            .intent("templates.save_as"),
    )
    .separator()
    .item(
        MenuEntry::new(tr!(ctx_trash()))
            .enabled(on_selection.clone())
            .intent("binder.trash_selected"),
    )
}
