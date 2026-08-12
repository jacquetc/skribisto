// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Work** menu — the File menu under a project-centric name.
//!
//! Grouping and separators follow the usual desktop document-app File menu:
//!
//!   1. Open / create          New, Open, New Window
//!   2. Persist / ship out     Save, Save As, Import from…, Export
//!   3. Archive                Back up now, Backups…
//!   4. Leave this work        Close Work, Welcome…
//!   5. App preferences        Settings
//!   6. Exit process           Quit
//!
//! Separators sit between those groups only — never inside a group, and never
//! stacked. Import from sits just before Export: both move content across the
//! project boundary (in vs out). Plume still *starts* a work, but Documents land
//! in the open one, so the submenu lives with the project-scoped ship-out group
//! rather than with New/Open. Welcome sits next to Close because both return to
//! the Launcher (`welcome.show` aliases `work.close`).

use export_management::ExportScopeKind;
use frontend::common::entities::WorkShape;
use teksilo::prelude::*;
use teksilo::widgets::{MenuEntry, MenuItems};

use super::ProjectMenuParts;
use crate::intents::AppIntent;
use crate::view_models::scope_label;

/// The rows of the menu, in the order they appear.
pub(super) fn menu(m: MenuItems, parts: &ProjectMenuParts) -> MenuItems {
    let menu_ctx = parts.app_ctx.clone();
    let export_for_menu = parts.export.clone();
    let menu_work = parts.single_work.clone();
    let menu_work_info = parts.single_work_info.clone();
    let menu_ids = parts.ids.clone();
    let menu_autosave = parts.autosave_menu.clone();
    let menu_save_as = parts.save_as.clone();
    let menu_backup_mode = parts.backup_mode.clone();
    let menu_unsaved = parts.unsaved.clone();

    let file_ctx = menu_ctx.clone();
    let folder_ctx = menu_ctx.clone();
    let file_ids = menu_ids.clone();
    let folder_ids = menu_ids.clone();
    let folder_work = menu_work.clone();
    let save_as_file_vm = menu_save_as.clone();
    let save_as_folder_vm = menu_save_as.clone();
    // A work is open iff its WorkInfo shape is known.
    let show_open = menu_work_info.shape().map(|s| s.is_some());
    // Bug 2: offer only the *other* shape — a zip project
    // shows "Save as folder", a folder project shows
    // "Save as single file". Both collapse when no
    // project is open (`shape` is `None`). Reactive via
    // the overlay menu's `visible_when`.
    let show_save_file = menu_work_info
        .shape()
        .map(|s| *s == Some(WorkShape::Folder));
    let show_save_folder = menu_work_info.shape().map(|s| *s == Some(WorkShape::Zip));
    // Autosave hides the manual "Save" item (+ its Ctrl+S
    // accelerator); the save then runs on the debounce timer.
    // Also hidden in backup mode (Save is off there).
    let show_manual_save = menu_autosave
        .zip(&menu_backup_mode)
        .map(|(a, bm)| !*a && !*bm);
    // …and it greys out while there is nothing to save.
    // Same signal as the `editor.save` action/shortcut
    // (app.rs), so the item, Ctrl+S and the intent are
    // enabled or disabled as one.
    let can_save = crate::app::can_save(&menu_unsaved, &menu_backup_mode);
    // "Back up now" shows only for an open, non-backup project.
    let show_backup_now = show_open.zip(&menu_backup_mode).map(|(o, bm)| *o && !*bm);

    // ── 1. Open / create ─────────────────────────────
    // New / Open route through the global `work.new` /
    // `work.open` actions (registered in `App::build`), so
    // the same code path serves the menu and the Ctrl+N /
    // Ctrl+O shortcuts.
    m.item(
        MenuEntry::new(tr!(menu_new_work()))
            .intent("work.new")
            .shortcut("work.new"),
    )
    .item(
        MenuEntry::new(tr!(menu_open_work()))
            .intent("work.open")
            .shortcut("work.open"),
    )
    // A second window onto the Work this one already
    // shows — same project, same edits, its own desk.
    // Hidden with no project open: there is nothing to
    // open a second view of. Same `show_open` signal the
    // Close/Backups entries use, so the whole "needs a
    // project" group appears and disappears together.
    .item(
        MenuEntry::new(tr!(menu_new_window()))
            .visible(show_open.clone())
            .intent("window.new")
            .shortcut("window.new"),
    )
    .separator()
    // The book's cover. It sits in the Work menu rather than with
    // Insert image…, because it belongs to the book and not to
    // whichever scene happens to have the caret — nothing in the prose
    // ever refers to it. Hidden with no project open, like the group
    // above: there is no book to give a cover to.
    .item(
        MenuEntry::new(tr!(cover_choose()))
            .visible(show_open.clone())
            .intent("work.set_cover"),
    )
    .item(
        MenuEntry::new(tr!(cover_clear()))
            .visible(show_open.clone())
            .intent("work.clear_cover"),
    )
    .separator()
    // ── 2. Persist / ship out ────────────────────────
    // Flush editors to the store + write to disk (also Ctrl+S).
    .item(
        MenuEntry::new(tr!(menu_save()))
            .visible(show_manual_save)
            .enabled(can_save)
            .intent("editor.save")
            .shortcut("editor.save"),
    )
    // Convert the open project to a single zipped `.skrib`
    // at a user-chosen location (native save dialog).
    .item(
        MenuEntry::new(tr!(menu_save_as_file()))
            .visible(show_save_file)
            .on_activate(move |ectx| {
                let ctx = file_ctx.clone();
                let vm = save_as_file_vm.clone();
                let req = crate::models::dialog_start_in(
                    ectx,
                    crate::models::FolderPurpose::SaveAs,
                    FileDialogRequest::save_file()
                        .title("Save as single .skrib file")
                        .default_file_name(format!(
                            "{}.skrib",
                            crate::project_stem(&ctx, &file_ids)
                        ))
                        .add_filter("Skribisto work", &["skrib"]),
                );
                let _ = ectx.save_file(req, move |res, ectx2| {
                    if let FileDialogResult::Saved(Some(path)) = res {
                        crate::models::remember_dialog_file(
                            ectx2,
                            crate::models::FolderPurpose::SaveAs,
                            &path,
                        );
                        // `begin` flushes the live editor buffers into the
                        // store first — the background op is read-only, so
                        // without it we would write the pre-edit prose.
                        vm.begin(ectx2, path.to_string_lossy().into_owned(), false);
                    }
                });
            }),
    )
    // Convert the open project to an exploded folder at a
    // user-chosen directory (native folder picker).
    .item(
        MenuEntry::new(tr!(menu_save_as_folder()))
            .visible(show_save_folder)
            .on_activate(move |ectx| {
                let ctx = folder_ctx.clone();
                // Bug 1: the picked folder is the *parent* —
                // write into a subfolder named after the Work
                // title (sanitized), falling back to the
                // project file stem when the title is empty.
                let title = folder_work.title().get();
                let raw = if title.trim().is_empty() {
                    crate::project_stem(&ctx, &folder_ids)
                } else {
                    title
                };
                let name = crate::sanitize_folder_name(&raw);
                let req = crate::models::dialog_start_in(
                    ectx,
                    crate::models::FolderPurpose::SaveAs,
                    FileDialogRequest::pick_folder().title("Choose a parent folder for the work"),
                );
                let vm = save_as_folder_vm.clone();
                let _ = ectx.pick_folder(req, move |res, ectx2| {
                    if let FileDialogResult::Folder(Some(path)) = res {
                        crate::models::remember_dialog_dir(
                            ectx2,
                            crate::models::FolderPurpose::SaveAs,
                            &path,
                        );
                        let target = path.join(&name).to_string_lossy().into_owned();
                        // Flushes the editors first — see the sibling item.
                        vm.begin(ectx2, target, true);
                    }
                });
            }),
    )
    // Import from… sits immediately before Export. Plume starts a new
    // work; Documents land *in* the open project — both move content
    // across the project boundary, the mirror image of Export. Submenu
    // so more importers can slot in later; each opens its own panel
    // via a global action. Documents are hidden with nothing open —
    // the same `show_open` gate the rest of this group uses.
    .submenu(tr!(menu_import_from()), {
        let show_docs = show_open.clone();
        move |s| {
            s.item(MenuEntry::new(tr!(menu_import_plume())).intent("work.import_plume"))
                .item(
                    MenuEntry::new(tr!(menu_import_document()))
                        .visible(show_docs)
                        .intent("work.import_document"),
                )
        }
    })
    // The same focus-adaptive quick-export list the title-bar
    // Export split-button shows — one source (the view-model's
    // `applicable` scopes), two surfaces. Each visible entry fires
    // the data-bearing `export.scope` intent; the disabled hint keeps
    // the submenu from ever being empty. Grouped with Save: both
    // write the current work out (Save keeps Skribisto shape;
    // Export ships another format).
    .submenu(tr!(menu_export()), {
        let ex = export_for_menu.clone();
        move |s| {
            let entry = |scope: ExportScopeKind| {
                let seen = scope.clone();
                let fire = scope.clone();
                MenuEntry::new(scope_label(&scope))
                    .visible(ex.applicable_signal().map(move |v| v.contains(&seen)))
                    .on_activate(move |c| {
                        c.send_intent(AppIntent::ExportScoped {
                            scope: fire.clone(),
                        })
                    })
            };
            s.item(entry(ExportScopeKind::CurrentBook))
                .item(entry(ExportScopeKind::CurrentPart))
                .item(entry(ExportScopeKind::CurrentChapter))
                .item(entry(ExportScopeKind::CurrentScene))
                .item(entry(ExportScopeKind::CurrentNote))
                .item(entry(ExportScopeKind::CurrentFolder))
                // Choose… — the checkbox-tree picker (always
                // available with a project open).
                .item(entry(ExportScopeKind::Custom))
                .item(
                    MenuEntry::new(tr!(menu_export_none()))
                        .visible(ex.applicable_signal().map(|v| v.is_empty()))
                        .enabled(false),
                )
        }
    })
    .separator()
    // ── 3. Archive ───────────────────────────────────
    // Manual backup — routed through the guarded
    // `backup.now` action in `App` (which flushes the
    // editors into the store first, resolves the
    // configured destinations, and drives the toast).
    // Hidden while a project is open in backup mode.
    .item(
        MenuEntry::new(tr!(menu_backup()))
            .visible(show_backup_now)
            .intent("backup.now"),
    )
    // Browse the project's backup files (open / reveal / delete).
    .item(
        MenuEntry::new(tr!(menu_backups_list()))
            .visible(show_open.clone())
            .intent("backups.show"),
    )
    .separator()
    // ── 4. Leave this work ───────────────────────────
    // Close the open work — routed through the guarded
    // `work.close` action (unsaved-changes prompt /
    // autosave-ensure live in `App`). Returns to the
    // Launcher instead of leaving an empty window.
    .item(
        MenuEntry::new(tr!(menu_close_work()))
            .visible(show_open)
            .intent("work.close")
            .shortcut("work.close"),
    )
    // Same terminal path as Close Work (`welcome.show` →
    // `work.close`); kept as its own label so the return
    // to the start screen is discoverable by name.
    .item(MenuEntry::new(tr!(menu_welcome())).intent("welcome.show"))
    .separator()
    // ── 5. App preferences ───────────────────────────
    .item(
        MenuEntry::new(tr!(menu_settings()))
            .intent("app.settings")
            .shortcut("app.settings"),
    )
    .separator()
    // ── 6. Exit process (always last) ────────────────
    .item(
        MenuEntry::new(tr!(menu_quit()))
            .intent("app.quit")
            .shortcut("app.quit"),
    )
}
