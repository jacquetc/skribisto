// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Project window menu model and Format-menu row helpers.
//!
//! The full File/Edit/Format/Go/Tools/Help model is built here so
//! `shell::windows` only hosts the window factory and chrome frame.

use std::rc::Rc;

use frontend::AppContext;
use teksilo::core::menu_item_id::MenuItemId;
use teksilo::prelude::*;
use teksilo::widgets::{MenuEntry, MenuModel, MenuNode};

use crate::singles::{SingleWork, SingleWorkInfo};
use crate::view_models::{
    ExportViewModel, FocusViewModel, FormatViewModel, GoAvailability, NoteTemplatesViewModel,
    OutlineViewModel, SaveAsViewModel,
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
    label: teksilo::i18n::LocalizedString,
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
    label: teksilo::i18n::LocalizedString,
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
/// teksilo's native backend says so in as many words: a hidden item is omitted
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

mod document;
mod format;
mod view;
mod work;

/// Build the full File / Edit / Format / Go / Tools / Help menu model for a project window.
///
/// One module per menu; each takes the parts it needs off [`ProjectMenuParts`]
/// itself rather than through a bespoke argument list, so adding a row to one
/// menu never touches another's signature.
pub(crate) fn build_project_menu(parts: ProjectMenuParts) -> MenuModel {
    // The three short menus below still read their pieces here; the four long
    // ones take `&parts` and clone what they need themselves. Nothing is moved
    // out of `parts`, or the borrow the four take would not be available.
    let menu_spellcheck = parts.spellcheck_menu.clone();
    let menu_comments = parts.comments_menu.clone();
    let menu_go = parts.go.clone();

    MenuModel::new()
        .menu(tr!(menu_work()), |m| work::menu(m, &parts))
        .menu(tr!(menu_view()), |m| view::menu(m, &parts))
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
        .menu(tr!(menu_document()), |m| document::menu(m, &parts))
        .menu(tr!(menu_format()), |m| format::menu(m, &parts))
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
        .menu(tr!(menu_tools()), move |mut m| {
            // The master spell-check switch. `checked(..)` is Teksilo's
            // **reflect-only** mark — it mirrors the setting read-only and the
            // intent is what drives it. NOT `.checkable()`, which would write
            // the signal on click and fight the store-backed value.
            m = m
                .item(
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
                );
            // Anything an extension registered, after the app's own rows and
            // behind a separator so the two groups read as what they are. A
            // snapshot taken as this window's menu is assembled — which is also
            // what makes it idempotent: the rows are built with the menu, never
            // pushed into it afterwards, so an `App` rebuild cannot double them.
            if crate::commands_ext::has_menu_rows() {
                m = crate::commands_ext::menu_rows(m.separator());
            }
            m
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
