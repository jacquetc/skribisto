// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Project window menu model and Format-menu row helpers.
//!
//! The full File/Edit/Format/Go/Tools/Help model is built here so
//! `shell::windows` only hosts the window factory and chrome frame.

use std::rc::Rc;

use bastyde::core::menu_item_id::MenuItemId;
use bastyde::prelude::*;
use bastyde::widgets::{DockSide, MenuEntry, MenuModel, MenuNode};
use export_management::ExportScopeKind;
use frontend::AppContext;
use frontend::common::entities::WorkShape;

use crate::intents::AppIntent;
use crate::singles::{SingleWork, SingleWorkInfo};
use crate::view_models::{
    ALIGN_CENTER, ALIGN_LEFT, DIR_AUTO, DIR_LTR, DIR_RTL, ExportViewModel, FocusViewModel,
    FormatViewModel, GoAvailability, NoteTemplatesViewModel, OutlineViewModel, SaveAsViewModel,
    scope_label,
};

/// A Format-menu row that reflects document state: a reflect-only checkmark
/// mirroring the same signal the dock's button binds, so the two surfaces
/// cannot disagree about whether the selection is bold.
///
/// `checked` and not `checkable`: the mark mirrors the document read-only, and
/// the command is what changes it. A checkable row would write the signal on
/// click and fight the value the editor reports back.
pub(crate) fn mark(
    vm: &FormatViewModel,
    label: bastyde::i18n::LocalizedString,
    enabled: Signal<bool>,
    state: Signal<bool>,
    run: fn(&FormatViewModel),
) -> MenuEntry {
    let vm = vm.clone();
    MenuEntry::new(label)
        .enabled(enabled)
        .checked(state)
        .on_activate(move |c| {
            run(&vm);
            c.request_frame();
            vm.refocus(c);
        })
}

/// A Format-menu row that just runs a command.
///
/// Like the dock's buttons it requests a frame: the pointer is on the menu
/// overlay and the editor is unfocused, so nothing else schedules the repaint
/// that shows the edit. It then puts focus back where the writer was typing —
/// reaching a menu item took it away, and the dock and context menu do not have
/// that problem (the dock's buttons are non-focusable, and dismissing the
/// context menu restores focus by itself).
pub(crate) fn command(
    vm: &FormatViewModel,
    label: bastyde::i18n::LocalizedString,
    enabled: Signal<bool>,
    run: fn(&FormatViewModel),
) -> MenuEntry {
    let vm = vm.clone();
    MenuEntry::new(label)
        .enabled(enabled)
        .on_activate(move |c| {
            run(&vm);
            c.request_frame();
            vm.refocus(c);
        })
}

/// Inputs for the project window's hamburger menu model.
pub(crate) struct ProjectMenuParts {
    pub app_ctx: Rc<AppContext>,
    pub export: ExportViewModel,
    pub single_work: SingleWork,
    pub single_work_info: SingleWorkInfo,
    pub ids: crate::app_ids::AppIds,
    pub autosave_menu: Signal<bool>,
    pub spellcheck_menu: Signal<bool>,
    /// Mirror of the persisted Tools ▸ Comments switch, for its checkmark.
    pub comments_menu: Signal<bool>,
    pub scene_focused: Signal<bool>,
    /// Whether this window's binder has a selection — the Document menu's per-item rows
    /// grey out without one.
    pub binder_has_selection: Signal<bool>,
    /// The pre-allocated id of the Document menu's "Insert template" submenu, so its
    /// contents can be repopulated at runtime — see the submenu's own comment, and
    /// [`sync_insert_template_submenu`].
    pub templates_submenu_id: MenuItemId,
    pub go: GoAvailability,
    pub format: FormatViewModel,
    pub save_as: SaveAsViewModel,
    pub backup_mode: Signal<bool>,
    pub unsaved: Signal<bool>,
    pub outline: OutlineViewModel,
    pub focus: FocusViewModel,
    /// Live window placement for the fullscreen checkmark (View ▸ Fullscreen).
    pub placement: Signal<WindowPlacement>,
}

/// Refill the Document ▸ **Insert template** submenu from the current catalogue.
///
/// Called once when the window's menu is built and again on every change to the
/// catalogue (`App::build` drives it off `NoteTemplatesViewModel::changed_signal`).
/// `MenuModel::modify` is the doc's sanctioned escape hatch for a bulk structural edit;
/// it bumps `version`, and a `MenuBar::from_model` bar binds that at `Rebuild` level, so
/// the dropdown re-derives on its own.
///
/// Replaces the submenu's children wholesale rather than diffing: the list is a handful
/// of rows, and rebuilding it is the only way to honour a reorder or a rename without
/// tracking a per-row id map that would have to stay in step with the writer's
/// arrangement.
pub(crate) fn sync_insert_template_submenu(
    model: &MenuModel,
    submenu_id: MenuItemId,
    templates: &NoteTemplatesViewModel,
    has_editor: &Signal<bool>,
) {
    let rows = templates.menu_rows();
    let has_editor = has_editor.clone();
    model.modify(|nodes| {
        let Some(children) = find_submenu_children(nodes, submenu_id) else {
            return;
        };
        children.clear();
        if rows.is_empty() {
            // An empty submenu reads as broken. Say why instead.
            children.push(MenuNode::Item(
                MenuEntry::new(tr!(menu_insert_template_none())).enabled(false),
            ));
            return;
        }
        for row in &rows {
            let id = row.id;
            children.push(MenuNode::Item(
                MenuEntry::new(lit!(row.name.clone()))
                    .enabled(has_editor.clone())
                    .on_activate(move |c| {
                        c.send_intent(crate::intents::AppIntent::InsertTemplate {
                            template_id: id,
                        });
                    }),
            ));
        }
    });
}

/// Where the Image menu slots in among the top-level menus, counting from zero:
/// Work, View, Document, Format, **Image**, Go, Tools, Help.
const IMAGE_MENU_INDEX: usize = 4;

/// Show or hide the whole **Image** menu, following the selection.
///
/// A top-level menu that comes and goes, which `MenuNode` has no declarative
/// visibility for — `MenuEntry::visible` hides an item, not a menu, and
/// bastyde's native backend says so in as many words: a hidden item is omitted
/// from the snapshot, and "for fully-dynamic native menus use `MenuModel::remove`
/// / `push_item`". So it is inserted and removed, which is the same mechanism
/// the Insert template submenu already uses to stay in step with its data.
///
/// Idempotent: called on every change of the selection, and does nothing when
/// the menu is already in the state asked for. That matters — each real mutation
/// bumps the model's version, which re-installs the native menu bar, and doing
/// that on every click would make the bar flicker.
pub(crate) fn sync_image_menu(model: &MenuModel, id: MenuItemId, image_selected: bool) {
    let present = model.contains(id);
    if image_selected == present {
        return;
    }
    if !image_selected {
        model.remove(id);
        return;
    }
    // No "Insert image" here: this menu exists only while a picture is
    // selected, which is exactly when the writer is not inserting one. It lives
    // in the Document menu, beside the other things that go in at the caret.
    // Between Format and Go: the writer reached for a picture, and the menus
    // either side are the ones that act on what is selected.
    model.insert_menu_at(IMAGE_MENU_INDEX, id, tr!(menu_image()), |m| {
        m.item(MenuEntry::new(tr!(image_menu_describe())).intent("image.describe"))
            .item(MenuEntry::new(tr!(image_menu_resize())).intent("image.resize"))
            .item(MenuEntry::new(tr!(image_menu_reset_size())).intent("image.reset_size"))
    });
}

/// The children vector of the submenu with `id`, searched recursively.
///
/// `MenuModel::modify` hands out the top-level nodes, and the target sits one level down
/// inside the Document menu, so the walk is this function's job rather than the model's.
fn find_submenu_children(nodes: &mut [MenuNode], id: MenuItemId) -> Option<&mut Vec<MenuNode>> {
    for node in nodes.iter_mut() {
        if let MenuNode::Submenu {
            id: node_id,
            children,
            ..
        } = node
        {
            if *node_id == id {
                return Some(children);
            }
            if let Some(found) = find_submenu_children(children, id) {
                return Some(found);
            }
        }
    }
    None
}

/// Build the full File / Edit / Format / Go / Tools / Help menu model for a project window.
pub(crate) fn build_project_menu(parts: ProjectMenuParts) -> MenuModel {
    let menu_ctx = parts.app_ctx;
    let export_for_menu = parts.export;
    let menu_work = parts.single_work;
    let menu_work_info = parts.single_work_info;
    let menu_ids = parts.ids;
    let menu_autosave = parts.autosave_menu;
    let menu_spellcheck = parts.spellcheck_menu;
    let menu_comments = parts.comments_menu;
    let menu_scene_focused = parts.scene_focused;
    let menu_binder_selection = parts.binder_has_selection;
    let templates_submenu_id = parts.templates_submenu_id;
    let menu_go = parts.go;
    let menu_format_vm = parts.format;
    let menu_save_as = parts.save_as;
    let menu_backup_mode = parts.backup_mode;
    let menu_unsaved = parts.unsaved;
    let outline = parts.outline;
    let focus = parts.focus;
    let placement = parts.placement;

    // Work menu = the File menu under a project-centric name. Grouping and
    // separators follow the usual desktop document-app File menu:
    //
    //   1. Open / create          New, Open, New Window, Import
    //   2. Persist / ship out     Save, Save As, Export
    //   3. Archive                Back up now, Backups…
    //   4. Leave this work        Close Work, Welcome…
    //   5. App preferences        Settings
    //   6. Exit process           Quit
    //
    // Separators sit between those groups only — never inside a group, and
    // never stacked. Import lives with Open (foreign formats that start a
    // work); Export lives with Save (write the current work out). Welcome
    // sits next to Close because both return to the Launcher (`welcome.show`
    // aliases `work.close`).
    MenuModel::new()
        .menu(tr!(menu_work()), move |m| {
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
            // Import from another writing app. Open-adjacent: it
            // starts a work from a foreign format. Submenu so
            // more importers can slot in later; each opens its
            // own panel via a global action.
            .submenu(tr!(menu_import_from()), |s| {
                s.item(MenuEntry::new(tr!(menu_import_plume())).intent("work.import_plume"))
            })
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
                        let req = FileDialogRequest::save_file()
                            .title("Save as single .skrib file")
                            .default_file_name(format!(
                                "{}.skrib",
                                crate::project_stem(&ctx, &file_ids)
                            ))
                            .add_filter("Skribisto work", &["skrib"]);
                        let _ = ectx.save_file(req, move |res, ectx2| {
                            if let FileDialogResult::Saved(Some(path)) = res {
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
                        let req = FileDialogRequest::pick_folder()
                            .title("Choose a parent folder for the work");
                        let vm = save_as_folder_vm.clone();
                        let _ = ectx.pick_folder(req, move |res, ectx2| {
                            if let FileDialogResult::Folder(Some(path)) = res {
                                let target = path.join(&name).to_string_lossy().into_owned();
                                // Flushes the editors first — see the sibling item.
                                vm.begin(ectx2, target, true);
                            }
                        });
                    }),
            )
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
        })
        .menu(tr!(menu_view()), {
            // Reflect-only checkmarks: they mirror each dock's truth
            // without writing it; the toggles are driven by the
            // `outline.toggle` (F9) / `preview.toggle` (F10) intents.
            let outline = outline.clone();
            // Increment 1 of distraction-free (plain fullscreen): a
            // reflect-only checkmark straight off THIS window's own
            // `WindowState::placement()` — the same round-tripping
            // signal `view.fullscreen`'s action writes and the OS
            // reads back (see `FullscreenViewModel`'s doc), so the
            // mark stays correct even if fullscreen is left behind
            // the app's back (KDE's own shortcut, the titlebar).
            let is_fullscreen = placement.clone().map(|p| *p == WindowPlacement::Fullscreen);
            // Increment 2 of distraction-free: this checkmark reflects
            // `FocusViewModel::active_signal()` directly — unlike
            // `is_fullscreen` above, this is this window's own live
            // state, not something round-tripped off the OS, so there
            // is no fresher source to read it from.
            let is_focus_mode = focus.active_signal();
            move |m| {
                // The bottom band has no persistent reveal affordance
                // of its own — a hidden top/bottom side collapses its
                // rail with it (unlike leading/trailing, whose rail
                // survives as the way back). So the menu entry is not
                // a convenience here, it is the only way to bring the
                // preview back once it is closed.
                let preview_visible = outline.docking().side_visible_signal(DockSide::Bottom);
                m.item(
                    MenuEntry::new(tr!(menu_outline()))
                        .checked(outline.is_visible())
                        .intent("outline.toggle")
                        .shortcut("outline.toggle"),
                )
                // Reveal the leading search & replace dock. A plain
                // action (not a reflect-only checkbox): the search
                // dock is one of two switchable leading tabs, not a
                // side that is simply shown or hidden.
                .item(
                    MenuEntry::new(tr!(menu_search()))
                        .intent("search.show")
                        .shortcut("search.show"),
                )
                // Reveal the trash dock — like Search, a plain action
                // (a switchable leading tab, not a shown/hidden side).
                .item(MenuEntry::new(tr!(menu_trash())).intent("trash.show"))
                // Reveal the footnotes dock. Not a convenience either: a dock
                // reachable only from its rail has no way back once a saved
                // desk stops mounting it, and this is the door that recovers it.
                .item(MenuEntry::new(tr!(menu_footnotes())).intent("footnotes.show"))
                .item(
                    MenuEntry::new(tr!(menu_search_preview()))
                        .checked(preview_visible)
                        .intent("preview.toggle")
                        .shortcut("preview.toggle"),
                )
                .separator()
                // Increment 1 of distraction-free: plain fullscreen,
                // not the collapsed-chrome mode itself (that is a
                // later increment). F11, the platform convention.
                .item(
                    MenuEntry::new(tr!(menu_fullscreen()))
                        .checked(is_fullscreen)
                        .intent("view.fullscreen")
                        .shortcut("view.fullscreen"),
                )
                // Increment 2 of distraction-free: chrome
                // collapse + docks disabled + fullscreen,
                // together, as one per-window mode. Shift+F11.
                .item(
                    MenuEntry::new(tr!(menu_focus_mode()))
                        .checked(is_focus_mode)
                        .intent("view.focus_mode")
                        .shortcut("view.focus_mode"),
                )
            }
        })
        // Format — marks the author places in the prose itself, as
        // opposed to Tools, which processes the manuscript. A scene
        // break lives here because it is something you *write*, not
        // something the compiler infers from binder structure.
        // `MenuEntry` carries no tooltip — the menubar model has no
        // such affordance — so the two tiers are explained by the
        // rich tooltips on their glyph pickers in
        // Settings ▸ Compile & Export, which is where a writer
        // decides what each one prints as.
        // Document — everything scoped to the *item* you are in, sitting between the
        // window-scoped View menu and the run-scoped Format menu so the bar reads
        // outward-in: project, window, document, text run.
        //
        // The five binder rows had no menu-bar home at all before this: they were
        // reachable only from the outline's context menu, which is a real discoverability
        // and a11y gap (a context menu is hard to reach by keyboard and by screen reader).
        // Every one of them is an already-registered *selection-based* global action, so
        // these rows fire the same intent the context menu does — no second code path.
        //
        // Rows stay visible when disabled, exactly as the Format menu's do: a greyed row
        // still teaches that the feature exists.
        .menu(tr!(menu_document()), {
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
            move |m| {
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
                    .item(
                        MenuEntry::new(tr!(menu_document_tidy_titles()))
                            .intent("numbering.tidy_titles"),
                    )
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
                // `bastyde/docs/native-menu.md` §"Dynamic structure": give the submenu a
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
        })
        .menu(tr!(menu_format()), {
            // Enabled on the *sticky* target, not on live focus:
            // opening this menu moves focus to the menu overlay,
            // so an enablement keyed on focus would grey every
            // row out at the instant the user reached for one.
            // Scene breaks keep their own narrower gate — the
            // same predicate the compiler uses, so the menu can
            // never offer a mark the exporter would ignore.
            //
            // Rows stay visible when disabled: a greyed row still
            // teaches that the feature exists and what its
            // shortcut is, and still reaches the a11y tree. That
            // is the opposite of the dock, which hides what does
            // not apply — a menu is a map of what exists, a dock
            // is a set of what applies right now.
            //
            // Text-only, with no glyphs: `MenuEntry` has no
            // `.icon()`. Parity with the dock means the same
            // commands and the same state, not the same look.
            let on_scene = menu_scene_focused.clone();
            let f = menu_format_vm.clone();
            move |m| {
                let on = f.has_target();
                // Bold/Italic/Underline are handled inside
                // `RichTextEditor`'s own key dispatch, not the
                // shortcut registry, so there is no id to bind —
                // the chord travels in the label instead.
                let mut m = m
                    .item(mark(
                        &f,
                        tr!(menu_format_marks_bold()),
                        on.clone(),
                        f.bold(),
                        FormatViewModel::toggle_bold,
                    ))
                    .item(mark(
                        &f,
                        tr!(menu_format_marks_italic()),
                        on.clone(),
                        f.italic(),
                        FormatViewModel::toggle_italic,
                    ))
                    .item(mark(
                        &f,
                        tr!(menu_format_marks_underline()),
                        on.clone(),
                        f.underline(),
                        FormatViewModel::toggle_underline,
                    ))
                    .item(mark(
                        &f,
                        tr!(menu_format_marks_strike()),
                        on.clone(),
                        f.strikethrough(),
                        FormatViewModel::toggle_strikethrough,
                    ))
                    .item(mark(
                        &f,
                        tr!(menu_format_marks_superscript()),
                        on.clone(),
                        f.superscript(),
                        FormatViewModel::toggle_superscript,
                    ))
                    .item(mark(
                        &f,
                        tr!(menu_format_marks_subscript()),
                        on.clone(),
                        f.subscript(),
                        FormatViewModel::toggle_subscript,
                    ))
                    .item(command(
                        &f,
                        tr!(menu_format_marks_clear()),
                        on.clone(),
                        FormatViewModel::clear_formatting,
                    ))
                    .separator();

                // Seven levels, one exclusive choice — a radio
                // group over the index the caret already reports.
                m = m.submenu(tr!(menu_format_heading()), {
                    let f = f.clone();
                    let on = on.clone();
                    move |s| {
                        let mut s = s;
                        for (level, label) in [
                            (0usize, tr!(menu_format_heading_normal())),
                            (1, tr!(menu_format_heading_1())),
                            (2, tr!(menu_format_heading_2())),
                            (3, tr!(menu_format_heading_3())),
                            (4, tr!(menu_format_heading_4())),
                            (5, tr!(menu_format_heading_5())),
                            (6, tr!(menu_format_heading_6())),
                        ] {
                            let f = f.clone();
                            s = s.item(
                                MenuEntry::new(label)
                                    .enabled(on.clone())
                                    .radio(level, f.heading())
                                    .on_activate(move |c| {
                                        f.set_heading(level);
                                        c.request_frame();
                                        f.refocus(c);
                                    }),
                            );
                        }
                        s
                    }
                });

                m = m.submenu(tr!(menu_format_alignment()), {
                    let f = f.clone();
                    let on = on.clone();
                    move |s| {
                        let mut s = s;
                        for (idx, label) in [
                            (ALIGN_LEFT, tr!(menu_format_align_left())),
                            (ALIGN_CENTER, tr!(menu_format_align_center())),
                        ] {
                            let f = f.clone();
                            s = s.item(
                                MenuEntry::new(label)
                                    .enabled(on.clone())
                                    .radio(idx, f.alignment())
                                    .on_activate(move |c| {
                                        f.set_alignment(idx);
                                        c.request_frame();
                                        f.refocus(c);
                                    }),
                            );
                        }
                        s
                    }
                });

                // Paragraph direction gets the full three-way
                // radio the dock's single toggle cannot express:
                // "automatic" is a real state, distinct from a
                // pinned left-to-right, and worth reaching.
                m = m.submenu(tr!(menu_format_direction()), {
                    let f = f.clone();
                    let on = on.clone();
                    move |s| {
                        let mut s = s;
                        for (idx, label) in [
                            (DIR_AUTO, tr!(menu_format_direction_auto())),
                            (DIR_LTR, tr!(menu_format_direction_ltr())),
                            (DIR_RTL, tr!(menu_format_direction_rtl())),
                        ] {
                            let f = f.clone();
                            s = s.item(
                                MenuEntry::new(label)
                                    .enabled(on.clone())
                                    .radio(idx, f.direction())
                                    .on_activate(move |c| {
                                        f.set_direction(idx);
                                        c.request_frame();
                                        f.refocus(c);
                                    }),
                            );
                        }
                        s
                    }
                });

                m = m
                    .item(mark(
                        &f,
                        tr!(menu_format_blockquote()),
                        on.clone(),
                        f.blockquote(),
                        FormatViewModel::toggle_blockquote,
                    ))
                    .submenu(tr!(menu_format_lists()), {
                        let f = f.clone();
                        let on = on.clone();
                        move |s| {
                            s.item(command(
                                &f,
                                tr!(menu_format_list_bullet()),
                                on.clone(),
                                FormatViewModel::insert_bullet_list,
                            ))
                            .item(command(
                                &f,
                                tr!(menu_format_list_numbered()),
                                on.clone(),
                                FormatViewModel::insert_numbered_list,
                            ))
                            .separator()
                            .item(command(
                                &f,
                                tr!(menu_format_indent()),
                                on.clone(),
                                FormatViewModel::indent,
                            ))
                            .item(command(
                                &f,
                                tr!(menu_format_outdent()),
                                on.clone(),
                                FormatViewModel::outdent,
                            ))
                        }
                    })
                    .submenu(tr!(menu_format_table()), {
                        let f = f.clone();
                        let on = on.clone();
                        move |s| {
                            // `insert_table` takes two runtime
                            // numbers and `MenuEntry` has no
                            // payload slot, so the sizes are
                            // spelled out rather than prompted.
                            let sizes = s.submenu(tr!(menu_format_table_insert()), {
                                let f = f.clone();
                                let on = on.clone();
                                move |t| {
                                    let mut t = t;
                                    for (n, label) in [
                                        (2usize, tr!(menu_format_table_2x2())),
                                        (3, tr!(menu_format_table_3x3())),
                                        (4, tr!(menu_format_table_4x4())),
                                    ] {
                                        let f = f.clone();
                                        t = t.item(
                                            MenuEntry::new(label).enabled(on.clone()).on_activate(
                                                move |c| {
                                                    f.insert_table(n, n);
                                                    c.request_frame();
                                                    f.refocus(c);
                                                },
                                            ),
                                        );
                                    }
                                    t
                                }
                            });
                            // The row/column commands are gated
                            // on the caret actually being in a
                            // table — the dock hides them, a menu
                            // greys them.
                            let in_table = f.in_table();
                            sizes
                                .separator()
                                .item(command(
                                    &f,
                                    tr!(menu_format_table_row_above()),
                                    in_table.clone(),
                                    FormatViewModel::insert_row_above,
                                ))
                                .item(command(
                                    &f,
                                    tr!(menu_format_table_row_below()),
                                    in_table.clone(),
                                    FormatViewModel::insert_row_below,
                                ))
                                .item(command(
                                    &f,
                                    tr!(menu_format_table_col_before()),
                                    in_table.clone(),
                                    FormatViewModel::insert_column_before,
                                ))
                                .item(command(
                                    &f,
                                    tr!(menu_format_table_col_after()),
                                    in_table.clone(),
                                    FormatViewModel::insert_column_after,
                                ))
                                .separator()
                                .item(command(
                                    &f,
                                    tr!(menu_format_table_row_delete()),
                                    in_table.clone(),
                                    FormatViewModel::remove_row,
                                ))
                                .item(command(
                                    &f,
                                    tr!(menu_format_table_col_delete()),
                                    in_table.clone(),
                                    FormatViewModel::remove_column,
                                ))
                                .item(command(
                                    &f,
                                    tr!(menu_format_table_remove()),
                                    in_table,
                                    FormatViewModel::remove_table,
                                ))
                        }
                    })
                    .separator()
                    .item(command(
                        &f,
                        tr!(menu_format_undo()),
                        f.can_undo(),
                        FormatViewModel::undo,
                    ))
                    .item(command(
                        &f,
                        tr!(menu_format_redo()),
                        f.can_redo(),
                        FormatViewModel::redo,
                    ))
                    .separator();

                m.item(
                    MenuEntry::new(tr!(menu_scene_break()))
                        .enabled(on_scene.clone())
                        .intent("format.scene_break")
                        .shortcut("format.scene_break"),
                )
                .item(
                    MenuEntry::new(tr!(menu_major_scene_break()))
                        .enabled(on_scene.clone())
                        .intent("format.major_scene_break")
                        .shortcut("format.major_scene_break"),
                )
                .separator()
                // Gated by `.enabled()`, never `.visible()`, for the
                // same reason the scene-break rows above are: a row
                // that vanishes teaches nobody the shortcut exists.
                .item(
                    MenuEntry::new(tr!(comments_menu_add()))
                        .enabled(on_scene.clone())
                        .intent("comments.add")
                        .shortcut("comments.add"),
                )
                .item(
                    MenuEntry::new(tr!(comments_menu_add_paragraph()))
                        .enabled(on_scene.clone())
                        .intent("comments.add_paragraph")
                        .shortcut("comments.add_paragraph"),
                )
            }
        })
        // Go — Increment 4 of distraction-free: prev/next Scene/
        // Chapter/Note, scoped to one binder, deliberately crossing
        // Chapter/Part/Book boundaries within it (uninterrupted
        // drafting), never wrapping around. Six STATIC rows, gated
        // by `.enabled()` — never `.visible()` — for the same
        // reason the Format menu's scene-break rows stay visible
        // when disabled (see that menu's own comment above): a
        // greyed row still teaches the feature exists and what its
        // shortcut namespace is, and it still reaches the a11y
        // tree. No individual shortcuts here — the generic
        // `go.next`/`go.prev` pair (Alt+Down/Alt+Up), registered in
        // `app::commands::go`, delegates to whichever of these six
        // answers the focused tab's own kind resolves to; the
        // distraction-free strip's own Next/Previous buttons fire
        // that same generic pair, never a reimplementation.
        .menu(tr!(menu_go()), {
            let go = menu_go.clone();
            move |m| {
                use skribisto_model::{
                    GoDirection::{Next, Previous},
                    GoKind::{Chapter, Note, Scene},
                };
                m.item(
                    MenuEntry::new(tr!(menu_go_next_scene()))
                        .enabled(go.signal(Scene, Next))
                        .intent("go.next_scene"),
                )
                .item(
                    MenuEntry::new(tr!(menu_go_prev_scene()))
                        .enabled(go.signal(Scene, Previous))
                        .intent("go.prev_scene"),
                )
                .item(
                    MenuEntry::new(tr!(menu_go_next_chapter()))
                        .enabled(go.signal(Chapter, Next))
                        .intent("go.next_chapter"),
                )
                .item(
                    MenuEntry::new(tr!(menu_go_prev_chapter()))
                        .enabled(go.signal(Chapter, Previous))
                        .intent("go.prev_chapter"),
                )
                .item(
                    MenuEntry::new(tr!(menu_go_next_note()))
                        .enabled(go.signal(Note, Next))
                        .intent("go.next_note"),
                )
                .item(
                    MenuEntry::new(tr!(menu_go_prev_note()))
                        .enabled(go.signal(Note, Previous))
                        .intent("go.prev_note"),
                )
                .separator()
                // "Go to…" — the six rows above step relative to
                // where you are; this one jumps anywhere. Always
                // enabled: unlike the stepping rows there is no
                // "is there a target" question to answer, and an
                // empty binder simply opens an empty list.
                // The action behind it belongs to the `GoToButton`
                // widget (`PopoverWidget::open_action`), because
                // presenting a popover needs an `EventContext`.
                .item(
                    MenuEntry::new(tr!(menu_go_to()))
                        .intent("go.to")
                        .shortcut("go.to"),
                )
            }
        })
        // Tools — where every office suite keeps spell-check. Its own
        // top-level section rather than a View entry: View toggles what a
        // *dock* shows, whereas this changes how the manuscript is *processed*.
        .menu(tr!(menu_tools()), move |m| {
            // The master spell-check switch. `checked(..)` is Bastyde's
            // **reflect-only** mark — it mirrors the setting read-only and the
            // intent is what drives it. NOT `.checkable()`, which would write
            // the signal on click and fight the store-backed value.
            m.item(
                MenuEntry::new(tr!(menu_spellcheck()))
                    .checked(menu_spellcheck.clone())
                    .intent("spellcheck.toggle")
                    .shortcut("spellcheck.toggle"),
            )
            // Show or hide the anchored-comment marks and their
            // margin. Same reflect-only `checked(..)` as its
            // neighbour above, for the same reason: the persisted
            // setting is the truth and the intent is its only
            // writer, so `.checkable()` — which writes the bound
            // signal on click — would fight it. It hides the
            // presentation, not the data: both comment docks keep
            // listing every thread, and a screen reader keeps
            // announcing them.
            .item(
                MenuEntry::new(tr!(menu_comments()))
                    .checked(menu_comments.clone())
                    .intent("comments.toggle"),
            )
        })
        // Help sits last, as it does on every desktop platform.
        // "About" is fired by name only (no payload), so it needs
        // no `AppIntent` variant — just the global action that
        // `App::build` registers.
        .menu(tr!(menu_help()), |m| {
            m.item(MenuEntry::new(tr!(menu_about())).intent("app.about"))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A model shaped like the real Document menu: a top-level menu with a couple of items
    /// and the addressable template submenu nested inside it.
    fn model_with_submenu(id: MenuItemId) -> MenuModel {
        MenuModel::new().menu(lit!("Document"), move |m| {
            m.item(MenuEntry::new(lit!("Rename")))
                .submenu_with_id(id, lit!("Insert template"), |t| t)
                .item(MenuEntry::new(lit!("Save as template…")))
        })
    }

    fn submenu_len(model: &MenuModel, id: MenuItemId) -> usize {
        let mut n = 0;
        model.modify(|nodes| {
            n = find_submenu_children(nodes, id)
                .map(|c| c.len())
                .unwrap_or(0);
        });
        n
    }

    /// The submenu is one level down inside the Document menu, so the lookup has to
    /// recurse — `MenuModel::modify` only hands out the top-level nodes.
    #[test]
    fn a_nested_submenu_is_found_by_id() {
        let id = MenuItemId::next();
        let model = model_with_submenu(id);
        model.modify(|nodes| {
            assert!(
                find_submenu_children(nodes, id).is_some(),
                "the nested submenu must be reachable by its pre-allocated id"
            );
        });
    }

    #[test]
    fn an_unknown_id_finds_nothing() {
        let model = model_with_submenu(MenuItemId::next());
        model.modify(|nodes| {
            assert!(find_submenu_children(nodes, MenuItemId::next()).is_none());
        });
    }

    /// Refilling must **replace** the children, never append to them.
    ///
    /// This is the failure mode a live-updating menu invites: the sync runs on every
    /// catalogue change, so an appending implementation would show each template twice
    /// after the second change, three times after the third. Asserted on the empty case
    /// — built explicitly, not merely by having no project loaded — since the count is
    /// what matters, not the contents.
    #[test]
    fn refilling_replaces_the_children_rather_than_appending() {
        let id = MenuItemId::next();
        let model = model_with_submenu(id);
        let templates = NoteTemplatesViewModel::new(
            crate::models::WorkNoteTemplatesListModel::empty(
                std::rc::Rc::new(frontend::AppContext::new()),
                crate::app_ids::AppIds::default(),
            ),
            crate::app_ids::AppIds::default(),
        );
        let note_focused = Signal::new(false);

        sync_insert_template_submenu(&model, id, &templates, &note_focused);
        let after_one = submenu_len(&model, id);
        sync_insert_template_submenu(&model, id, &templates, &note_focused);
        let after_two = submenu_len(&model, id);

        assert_eq!(
            after_one, after_two,
            "a second refill must not grow the submenu (got {after_one} then {after_two})"
        );
    }

    /// Every refill must bump `version`: a `MenuBar::from_model` bar binds that at
    /// `Rebuild` level, and it is the only thing that makes the dropdown re-derive. Without
    /// the bump the model would be correct and the menu would still render stale — which is
    /// indistinguishable, from the writer's side, from the bug this whole path fixes.
    #[test]
    fn refilling_bumps_the_model_version() {
        let id = MenuItemId::next();
        let model = model_with_submenu(id);
        let templates = NoteTemplatesViewModel::new(
            crate::models::WorkNoteTemplatesListModel::empty(
                std::rc::Rc::new(frontend::AppContext::new()),
                crate::app_ids::AppIds::default(),
            ),
            crate::app_ids::AppIds::default(),
        );
        let before = model.version().get();
        sync_insert_template_submenu(&model, id, &templates, &Signal::new(false));
        assert!(
            model.version().get() > before,
            "the bar re-derives on a version bump; without one the menu renders stale"
        );
    }

    /// An empty catalogue yields the explanatory placeholder, not a blank submenu — a
    /// submenu that opens onto nothing reads as broken.
    #[test]
    fn an_empty_catalogue_yields_one_placeholder_row() {
        let id = MenuItemId::next();
        let model = model_with_submenu(id);
        let templates = NoteTemplatesViewModel::new(
            crate::models::WorkNoteTemplatesListModel::empty(
                std::rc::Rc::new(frontend::AppContext::new()),
                crate::app_ids::AppIds::default(),
            ),
            crate::app_ids::AppIds::default(),
        );
        sync_insert_template_submenu(&model, id, &templates, &Signal::new(false));
        assert_eq!(
            submenu_len(&model, id),
            1,
            "exactly the placeholder while the project has no templates"
        );
    }
}
