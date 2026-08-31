// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Project window menu model, Format-menu row helpers, and the two
//! platform-standard (macOS) menus every window shares.
//!
//! The full File/Edit/Format/Go/Tools/Help model is built here so
//! `shell::windows` only hosts the window factory and chrome frame.
//! [`app_standard_menu_base`] and [`window_standard_menu`] are `pub(crate)`
//! because the Launcher's much shorter menu ([`crate::shell::launcher_menu`])
//! declares the same two — one declaration each, rather than a second copy that
//! can drift on labels or on the guarded quit route.

use std::rc::Rc;

use frontend::AppContext;
use teksilo::core::menu_item_id::MenuItemId;
use teksilo::prelude::*;
use teksilo::widgets::{MenuEntry, MenuModel, MenuNode};

use crate::binder::OutlineViewModel;
use crate::export::ExportViewModel;
use crate::format::FormatViewModel;
use crate::go::GoAvailability;
use crate::note_templates::NoteTemplatesViewModel;
use crate::save::SaveAsViewModel;
use crate::shared::FocusViewModel;
use crate::singles::{SingleWork, SingleWorkInfo};

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
    /// Mirror of the persisted margin-lane switch, for View ▸ Margin marks.
    ///
    /// The same signal Settings ▸ Editor ▸ Margin marks binds, so the two
    /// surfaces are one switch rather than two that can drift apart.
    pub margin_lane_menu: Signal<bool>,
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
    /// Which history Ctrl+Z means in this window, and what it would take back.
    pub undo_group: crate::edit::UndoGroupViewModel,
    pub save_as: SaveAsViewModel,
    pub backup_mode: Signal<bool>,
    pub unsaved: Signal<bool>,
    pub outline: OutlineViewModel,
    pub focus: FocusViewModel,
    /// Live window placement for the fullscreen checkmark (View ▸ Fullscreen).
    pub placement: Signal<WindowPlacement>,
    /// Whether this project has an active writing plan — Work ▸ "Writing plan…" greys out
    /// without one. Read off the `WorkSession` (Tier 2: it is a fact about the project,
    /// not about this window), and kept in step by `app::wiring::project_events`.
    pub pace_available: Signal<bool>,
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
/// **App**, Work, **Edit**, View, Document, Format, **Image**, Go, Tools, Window,
/// Help.
///
/// The two platform-standard menus (App leading, Window before Help) are
/// top-level nodes like any other and therefore count here, even though neither
/// renders in the in-window bar — which is exactly the trap: a wrong index puts
/// the Image menu one place off on macOS and *nowhere visible* on Linux, since
/// the position is right there in the hamburger either way. Pinned by
/// `the_image_menu_lands_between_format_and_go`, which reads the built model
/// rather than trusting this number.
const IMAGE_MENU_INDEX: usize = 6;

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
mod edit;
mod format;
mod view;
mod work;

/// Whether a row that the macOS App menu also carries should render here.
///
/// Settings and Quit live in the application menu on a Mac and nowhere else —
/// a second copy in Work is not merely redundant, it puts two items in the bar
/// claiming the same key equivalent, and the one AppKit picks is decided by
/// traversal order rather than by us. `MenuEntry::visible` is the right lever:
/// a hidden item is omitted from the native snapshot at build. Everywhere else
/// this is `true` and the rows are exactly as they were — the App menu itself
/// renders on no other platform.
pub(crate) const NOT_ON_MACOS: bool = !cfg!(target_os = "macos");

/// The platform's application menu — the bold app-name submenu macOS puts
/// first, carrying About / Hide / Quit on the standard responder-chain
/// selectors.
///
/// **Every** window's App menu, the Launcher's included
/// ([`crate::shell::launcher_menu`]) — so the two cannot drift apart on the app
/// name, the About/Hide labels, the guarded quit route or the ⌘, that reaches
/// Settings.
///
/// Settings used to be layered on top of this by a project-window-only
/// `app_standard_menu()`, on the reasoning that `app.settings` opens
/// `SettingsPanel` over a `WorkSession` the Launcher has none of. That is no
/// longer true — `SettingsPanel::without_project` is exactly the window with no
/// session — and the reasoning cost the Launcher the one menu a first-run reader
/// needs: on Linux and Windows the app *starts* there, so theme, interface text
/// scale, dictionaries, keybindings and the backup defaults were unreachable
/// until a project had been created or opened.
///
/// Declared rather than left to Teksilo's auto-injection: the bridge adds a
/// default App menu when the model declares none, but with English `lit!`
/// labels, so a French system would read "Skribisto ▸ Quit". The product name
/// itself is **data** and arrives as an argument (same rule as the window
/// titles), which is also what keeps the labels correct for an edition that
/// renames itself.
///
/// The labels carry no `&` mnemonic: macOS has no mnemonics, and the native
/// bridge resolves standard labels without stripping one, so an ampersand
/// would print literally in the menu. Pinned by a test in `shell::windows`.
///
/// **`quit_intent` is load-bearing, not decoration.** Left unset, this menu's
/// Quit is AppKit's `terminate:` on a ⌘Q key equivalent, and AppKit dispatches
/// main-menu key equivalents before the responder chain — so ⌘Q would exit the
/// process without `QuitSequencer` ever running: no unsaved-changes prompt for
/// any open Work, no on-close backup, and not even winit's exit path. Skribisto's
/// own Ctrl+Q shortcut cannot cover for that, because the keystroke never
/// reaches the widget tree to begin with. Routing it hands ⌘Q to the same
/// guarded `app.quit` action Work ▸ Quit fires, which owns the exit from there.
pub(crate) fn app_standard_menu_base() -> teksilo::widgets::StandardMenu {
    let app = crate::identity::display_name();
    teksilo::widgets::StandardMenu::app()
        .title(lit!(app.clone()))
        .about(tr!(native_menu_about(app = app.clone())))
        .hide(tr!(native_menu_hide(app = app.clone())))
        .quit(tr!(native_menu_quit(app = app)))
        .quit_intent("app.quit")
        // The chord comes from the registry, not from AppKit's convention, so
        // the row follows a rebind in Settings > Shortcuts. Hardcoded, ⌘Q would
        // stay live after the writer moved Quit elsewhere *and* shadow wherever
        // they moved it — a main-menu key equivalent is dispatched before the
        // responder chain, so the new chord would never reach the app.
        .quit_shortcut("app.quit")
        // Settings belongs in the App menu on a Mac, at ⌘, — a placement no
        // `MenuEntry` can reach, since the platform fills this menu in. Same
        // intent and same registered shortcut as the in-window Work ▸ Settings
        // row, so the two are one command with one chord rather than two that
        // can disagree. Both windows register the `app.settings` action on
        // their own tree (`app/commands/file.rs` for a project window,
        // `welcome::panel` for the Launcher), because each `WidgetTree` owns its
        // own `global_actions`/`shortcut_registry`.
        .settings(tr!(native_menu_settings()))
        .settings_intent("app.settings")
        .settings_shortcut("app.settings")
}

/// The platform's Window menu (Minimize / Zoom, plus the live window list
/// AppKit maintains).
///
/// Worth declaring for Skribisto specifically: several project windows in one
/// process is the ordinary shape here (Work ▸ New Window, and one window per
/// open project), so the window list is the only surface that names them all.
pub(crate) fn window_standard_menu() -> teksilo::widgets::StandardMenu {
    teksilo::widgets::StandardMenu::window()
        .title(tr!(native_menu_window()))
        .minimize(tr!(native_menu_minimize()))
        .zoom(tr!(native_menu_zoom()))
}

/// Build the full File / Edit / Format / Go / Tools / Help menu model for a project window.
///
/// One module per menu; each takes the parts it needs off [`ProjectMenuParts`]
/// itself rather than through a bespoke argument list, so adding a row to one
/// menu never touches another's signature.
///
/// Two of the top-level nodes are **platform-standard** menus rather than rows
/// of our own — the App menu leading and the Window menu just before Help, the
/// order macOS expects. Both are invisible to the in-window bar
/// (`MenuBar::model_entries` keeps only `Submenu` nodes), so they change
/// nothing on Linux or Windows and exist purely for the mirrored native bar.
pub(crate) fn build_project_menu(parts: ProjectMenuParts) -> MenuModel {
    // The three short menus below still read their pieces here; the four long
    // ones take `&parts` and clone what they need themselves. Nothing is moved
    // out of `parts`, or the borrow the four take would not be available.
    let menu_spellcheck = parts.spellcheck_menu.clone();
    let menu_comments = parts.comments_menu.clone();
    let menu_go = parts.go.clone();

    MenuModel::new()
        .standard_menu(app_standard_menu_base())
        .menu(tr!(menu_work()), |m| work::menu(m, &parts))
        .menu(tr!(menu_edit()), |m| edit::menu(m, &parts))
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
        // Window (macOS only — see `window_standard_menu`). Declared here
        // rather than at the end because the platform's order is Window *then*
        // Help, and the model's order is what the native bar renders.
        .standard_menu(window_standard_menu())
        // Help sits last, as it does on every desktop platform.
        // "About" is fired by name only (no payload), so it needs
        // no `AppIntent` variant — just the global action that
        // `App::build` registers.
        .menu(tr!(menu_help()), |m| {
            // Help Topics leads, and carries the F1 accelerator by *reference*: the
            // chord is declared once on the shortcut and rendered per platform and
            // locale, so a rebind in Settings reaches this row without an edit here.
            m.item(
                MenuEntry::new(tr!(menu_help_topics()))
                    .intent("help.topics")
                    .shortcut("help.topics"),
            )
            .item(
                MenuEntry::new(tr!(menu_command_palette()))
                    .intent("app.command_palette")
                    .shortcut("app.command_palette"),
            )
            .item(
                MenuEntry::new(tr!(menu_help_shortcuts()))
                    .intent("help.shortcuts")
                    .shortcut("help.shortcuts"),
            )
            .separator()
            .item(MenuEntry::new(tr!(menu_help_website())).intent("help.website"))
            .item(MenuEntry::new(tr!(menu_help_report())).intent("help.report"))
            .separator()
            .item(MenuEntry::new(tr!(menu_about())).intent("app.about"))
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

    // ── The platform-standard (macOS) menus ─────────────────────────────

    /// A `ProjectMenuParts` with nothing open — every handle detached or
    /// pointed at an empty `AppContext`.
    ///
    /// The shape of the model does not depend on any of it: the standard menus
    /// are declared unconditionally, and the rows in between are built from
    /// signals whose *values* only decide enabled/visible state. So a blank
    /// project is the cheapest input that still exercises the real builder
    /// rather than a stand-in shaped like it.
    fn empty_parts() -> ProjectMenuParts {
        let app_ctx = Rc::new(frontend::AppContext::new());
        let ids = crate::app_ids::AppIds::new();
        let single_work = SingleWork::new(app_ctx.clone());
        let backup_mode = Signal::new(false);
        let format = FormatViewModel::detached();
        ProjectMenuParts {
            app_ctx: app_ctx.clone(),
            undo_group: crate::edit::UndoGroupViewModel::new(
                format.clone(),
                crate::edit::EntityDomain::new(
                    app_ctx.clone(),
                    ids.stack_id.clone(),
                    crate::save::SaveStateViewModel::new(app_ctx.clone(), ids.clone()),
                ),
            ),
            export: ExportViewModel::new(app_ctx.clone(), ids.clone()),
            single_work: single_work.clone(),
            single_work_info: SingleWorkInfo::new(app_ctx.clone()),
            ids: ids.clone(),
            autosave_menu: Signal::new(false),
            spellcheck_menu: Signal::new(false),
            comments_menu: Signal::new(true),
            margin_lane_menu: Signal::new(true),
            scene_focused: Signal::new(false),
            binder_has_selection: Signal::new(false),
            templates_submenu_id: MenuItemId::next(),
            go: crate::go::GoAvailability::new(),
            format: format.clone(),
            save_as: SaveAsViewModel::new(
                app_ctx.clone(),
                ids.clone(),
                single_work,
                backup_mode.clone(),
                Signal::new(None),
            ),
            backup_mode,
            unsaved: Signal::new(false),
            outline: OutlineViewModel::new_default(app_ctx, ids),
            focus: FocusViewModel::new(),
            placement: Signal::new(teksilo::core::WindowPlacement::default()),
            pace_available: Signal::new(false),
        }
    }

    fn standard_role(node: &MenuNode) -> Option<teksilo::widgets::StandardMenuRole> {
        match node {
            MenuNode::Standard(menu) => Some(menu.role()),
            _ => None,
        }
    }

    /// macOS puts the application menu first — it is where a Mac user reaches
    /// for Quit, and AppKit renders whatever leads the bar in that bold
    /// app-name slot whether or not it belongs there. Teksilo injects a default
    /// one when the model declares none, so the failure this pins is not "no
    /// App menu" but "an App menu with Teksilo's English labels", which reads
    /// as a half-translated app on a French system and is invisible from Linux.
    #[test]
    fn the_application_menu_leads_the_model() {
        let model = build_project_menu(empty_parts());
        model.modify(|nodes| {
            assert_eq!(
                standard_role(&nodes[0]),
                Some(teksilo::widgets::StandardMenuRole::App),
                "the application menu must be the leading top-level node"
            );
        });
    }

    /// The platform order is Window then Help, and the model's order is what
    /// the native bar renders — there is no second sorting step between them.
    #[test]
    fn the_window_menu_sits_just_before_help() {
        let model = build_project_menu(empty_parts());
        model.modify(|nodes| {
            let last = nodes.len() - 1;
            assert!(
                matches!(nodes[last], MenuNode::Submenu { .. }),
                "Help — an ordinary submenu — stays last"
            );
            assert_eq!(
                standard_role(&nodes[last - 1]),
                Some(teksilo::widgets::StandardMenuRole::Window),
                "the Window menu goes immediately before Help"
            );
        });
    }

    /// ⌘Q on macOS must reach `QuitSequencer`, not AppKit's `terminate:`.
    ///
    /// Unrouted, the App menu's Quit exits the process on a key equivalent that
    /// AppKit dispatches before the responder chain — so every open Work's
    /// unsaved prose goes without a prompt, no on-close backup is taken, and
    /// Skribisto's own Ctrl+Q shortcut never even sees the keystroke. None of
    /// that is observable from Linux, which is why it is asserted on the model
    /// rather than left to be noticed on a Mac.
    #[test]
    fn the_macos_quit_is_routed_through_the_guarded_action() {
        assert_eq!(
            app_standard_menu_base().quit_intent_name(),
            Some("app.quit"),
            "the App menu's Quit must fire the same guarded action as Work > Quit"
        );
    }

    /// Only the App menu carries a quit item; the Window menu has no Quit to
    /// route, and claiming one would put a second Command-Q in the bar.
    #[test]
    fn the_window_menu_routes_no_quit() {
        assert_eq!(window_standard_menu().quit_intent_name(), None);
    }

    /// Both routed rows name their registered shortcut, so their chords come
    /// from the same registry the in-window rows read.
    ///
    /// Without this the App menu is the one surface advertising a chord nothing
    /// registered: hardcoded ⌘Q survives a rebind in Settings ▸ Shortcuts, stays
    /// live for a command the writer moved, and shadows wherever they moved it —
    /// AppKit dispatches a main-menu key equivalent before the responder chain,
    /// so the new chord never reaches the app at all.
    #[test]
    fn the_routed_rows_take_their_chords_from_the_registry() {
        let app = app_standard_menu_base();
        assert_eq!(app.quit_shortcut_id(), Some("app.quit"));
        assert_eq!(app.settings_shortcut_id(), Some("app.settings"));
    }

    /// Settings belongs in the App menu on a Mac. Routed to the same intent as
    /// Work ▸ Settings, so the two are one command rather than two that can
    /// drift apart — and routed on the **shared** base, so the Launcher's App
    /// menu carries it too. It did not, and on Linux and Windows the Launcher is
    /// where the app starts: every app-level preference was unreachable in the
    /// first-run state.
    #[test]
    fn settings_is_routed_into_every_windows_application_menu() {
        assert_eq!(
            app_standard_menu_base().settings_intent_name(),
            Some("app.settings")
        );
    }

    /// A row the App menu carries must not also render in Work: two items
    /// claiming one key equivalent leave AppKit to pick by traversal order.
    /// This asserts the *rule*, not the platform — on Linux both rows render and
    /// there is no App menu to collide with.
    #[test]
    fn the_rows_the_app_menu_carries_are_hidden_from_work_on_macos() {
        assert_eq!(
            NOT_ON_MACOS,
            !cfg!(target_os = "macos"),
            "Settings and Quit render in Work exactly where no App menu exists"
        );
    }

    /// Exactly two standard nodes, and every other top-level node an ordinary
    /// submenu. A stray `Standard` would be silently dropped by the in-window
    /// bar (`model_entries` keeps only `Submenu`s), so it would go unnoticed
    /// on every platform but the one it breaks.
    #[test]
    fn the_model_declares_exactly_two_standard_menus() {
        let model = build_project_menu(empty_parts());
        model.modify(|nodes| {
            let standards: Vec<_> = nodes.iter().filter_map(standard_role).collect();
            assert_eq!(
                standards,
                vec![
                    teksilo::widgets::StandardMenuRole::App,
                    teksilo::widgets::StandardMenuRole::Window
                ],
                "App and Window are standard; Help is ours (it carries About)"
            );
            assert!(
                nodes
                    .iter()
                    .filter(|n| standard_role(n).is_none())
                    .all(|n| matches!(n, MenuNode::Submenu { .. })),
                "a bare item or separator at top level renders in neither bar"
            );
        });
    }

    /// Edit sits where every desktop application puts it — right after the
    /// file menu — and everything after it shifts by one.
    ///
    /// Worth pinning for the same reason the Image test is: `IMAGE_MENU_INDEX`
    /// is a raw index into the top-level vector and the two platform-standard
    /// menus count toward it, so inserting a menu anywhere silently moves the
    /// Image menu one place off on macOS and out of sight on Linux.
    #[test]
    fn the_edit_menu_lands_between_work_and_view() {
        let model = build_project_menu(empty_parts());
        model.modify(|nodes| {
            let title_at = |nodes: &[MenuNode], i: usize| match &nodes[i] {
                MenuNode::Submenu { title, .. } => title.resolve_now(),
                _ => panic!("top-level node {i} is not an ordinary submenu"),
            };
            let edit = nodes
                .iter()
                .position(|n| match n {
                    MenuNode::Submenu { title, .. } => {
                        title.resolve_now() == tr!(menu_edit()).resolve_now()
                    }
                    _ => false,
                })
                .expect("the Edit menu is in the model");
            assert_eq!(title_at(nodes, edit - 1), tr!(menu_work()).resolve_now());
            assert_eq!(title_at(nodes, edit + 1), tr!(menu_view()).resolve_now());
        });
    }

    /// `IMAGE_MENU_INDEX` is a raw position into the top-level nodes, and the
    /// two standard menus count towards it — so declaring the App menu shifted
    /// every ordinary menu one place right. Reading the built model is the only
    /// way to notice: the constant is self-consistent whatever it says, and
    /// off-by-one puts the Image menu between Document and Format, which looks
    /// deliberate.
    #[test]
    fn the_image_menu_lands_between_format_and_go() {
        let model = build_project_menu(empty_parts());
        let image = MenuItemId::next();
        sync_image_menu(&model, image, true);
        model.modify(|nodes| {
            let title_at = |nodes: &[MenuNode], i: usize| match &nodes[i] {
                MenuNode::Submenu { title, .. } => title.resolve_now(),
                _ => panic!("top-level node {i} is not an ordinary submenu"),
            };
            // Compared against the same `tr!` calls the model is built from, so
            // renaming a menu re-labels the test rather than breaking it.
            assert_eq!(
                title_at(nodes, IMAGE_MENU_INDEX),
                tr!(menu_image()).resolve_now(),
                "the Image menu goes in at IMAGE_MENU_INDEX"
            );
            assert_eq!(
                title_at(nodes, IMAGE_MENU_INDEX - 1),
                tr!(menu_format()).resolve_now(),
                "Format is the menu it follows"
            );
            assert_eq!(
                title_at(nodes, IMAGE_MENU_INDEX + 1),
                tr!(menu_go()).resolve_now(),
                "Go is the menu it precedes"
            );
        });
    }

    /// The neighbours above only mean something if the menu was absent to begin
    /// with — a model that already carried it would pass that test while
    /// `sync_image_menu` did nothing at all.
    #[test]
    fn the_image_menu_is_absent_until_a_picture_is_selected() {
        let model = build_project_menu(empty_parts());
        let image = MenuItemId::next();
        assert!(!model.contains(image));
        sync_image_menu(&model, image, true);
        assert!(model.contains(image), "selecting a picture adds the menu");
        sync_image_menu(&model, image, false);
        assert!(!model.contains(image), "deselecting takes it away again");
    }
}
