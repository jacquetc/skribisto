// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Launcher window's menu model.
//!
//! The Launcher used to mount no `MenuBar` at all, which cost it two things:
//! on Linux and Windows its two "start a project from something that isn't a
//! `.skrib`" doors were reachable by pointer only, and on macOS a focused
//! Launcher left the *last project window's* menu sitting in the global bar —
//! or, on a first launch with nothing else open, no menu bar at all.
//!
//! So it now builds a model of its own, mounted exactly like the project
//! window's ([`crate::shell::launcher_window`]): collapsed to a hamburger
//! in-window, mirrored into the native bar on macOS and suppressed in-window
//! there. Two top-level nodes are **platform-standard** menus rather than rows
//! of ours — the App menu leading and the Window menu last, the order macOS
//! expects — and both are invisible to the in-window bar
//! (`MenuBar::model_entries` keeps only `Submenu` nodes), so on Linux the
//! hamburger opens onto exactly one menu: Work.
//!
//! Both are declared by [`crate::shell::project_menus`] and shared, not copied:
//! a second declaration of the App menu is a second place for the app name, the
//! About/Hide labels and the guarded ⌘Q route to drift.
//!
//! ## Where its words come from
//!
//! Every label is a key the project window's menu bar already carries — `menu-work`,
//! `menu-new-work`, `menu-open-work`, `menu-import-document`, `menu-import-plume`,
//! `menu-settings`, `menu-quit`. One row, one name, wherever the writer meets it,
//! and no second translation of a string that already exists in both locales. The
//! single exception is the wrapper `menu-create-from`, which exists precisely because
//! it is *not* the project window's "Import from": these two rows produce a brand-new
//! project, where that menu's importers land content **in** the one already open.
//!
//! ## What it is deliberately *not*
//!
//! Not a shorter copy of the project menu. Every row here has to mean something
//! with **no project open**, which rules out most of Work (Save, Export, Close,
//! New Window) and all of View/Document/Format/Go. What is left is the four ways
//! *into* a project — New, Open, and the two Create-from importers — the app's
//! preferences, and the way out.
//!
//! ## Settings, and why it is here
//!
//! It was withheld, on the reasoning that `app.settings` opens `SettingsPanel`
//! over a `WorkSession` this window has none of. That was true of the panel and
//! wrong about the app: on Linux and Windows this window is where Skribisto
//! *starts*, so theme, interface text scale, dictionaries, keybindings and the
//! backup defaults were all unreachable until the writer had created or opened a
//! project — every one of them a thing a first-run reader is likely to want
//! *first*. `SettingsPanel::without_project` is the window with no session: every
//! app-level page live, the Work section absent from its tree. The row fires the
//! same `app.settings` action name the project window's Work ▸ Settings does,
//! registered here on this window's own tree by `WelcomePanel::build`.
//!
//! New Work and Open Work are buttons on the Welcome content as well, and
//! deliberately so: the menu is where a keyboard user and a screen reader look
//! for a command, the button is what a first-time reader sees, and both fire the
//! same named action rather than a second copy of the flow.

use teksilo::prelude::*;
use teksilo::widgets::{MenuEntry, MenuModel};

use super::project_menus::{NOT_ON_MACOS, app_standard_menu_base, window_standard_menu};

/// "Start a blank project" — the Welcome pane's New Work button, and the same
/// action name the project window registers for Work ▸ New Work. Both windows
/// register it on their own tree, over bodies that differ: the project window's
/// asks its unsaved-changes guard first, because creating a project there
/// *replaces* the one on screen. Here there is nothing to replace.
pub(crate) const ACTION_NEW: &str = "work.new";

/// "Open an existing `.skrib`" — the Welcome pane's Open button, and the project
/// window's Work ▸ Open Work. Same split as [`ACTION_NEW`]: one name, one
/// meaning, two bodies, because only one of the two windows has a project to
/// guard.
pub(crate) const ACTION_OPEN: &str = "work.open";

/// "Create a project from a set of documents" — the Welcome pane's own
/// `WelcomeViewModel::new_work_from_documents` flow.
///
/// A constant rather than a literal in two places: this module names it in the
/// menu row, `WelcomePanel::build` names it registering the action, and the
/// Welcome pane's "Create from…" button names it again. A typo in any one of
/// them is a row that silently opens nothing, which no compiler and no test on
/// the model can see.
pub(crate) const ACTION_NEW_FROM_DOCUMENTS: &str = "work.new_from_documents";

/// "Create a project from a Plume Creator project" — the same action name the
/// project window's Work ▸ Import from ▸ Plume Creator uses, registered
/// separately on each window's own tree (see [`build_launcher_menu`]).
pub(crate) const ACTION_IMPORT_PLUME: &str = "work.import_plume";

/// "Create a project from a Manuskript project" — the same action name the
/// project window's Work ▸ Import from ▸ Manuskript uses, registered separately
/// on each window's own tree (see [`build_launcher_menu`]).
pub(crate) const ACTION_IMPORT_MANUSKRIPT: &str = "work.import_manuskript";

/// "Open the preferences window" — the same action name the project window's
/// Work ▸ Settings row and the App menu's ⌘, both fire, over a body that opens
/// [`SettingsPanel::without_project`](crate::settings::SettingsPanel::without_project)
/// instead of one over this window's session, because this window has none.
///
/// A constant for the same reason [`ACTION_NEW_FROM_DOCUMENTS`] is: two places
/// here name it (this module's row, `WelcomePanel::build`'s registration) and a
/// typo in either is a row that silently opens nothing.
pub(crate) const ACTION_SETTINGS: &str = "app.settings";

/// Build the Launcher's menu model — one Work menu between the two
/// platform-standard ones.
///
/// Every row fires a **named global action**, registered on the Launcher's own
/// widget tree by `WelcomePanel::build`. It has to be registered there and not
/// reused from `App`: each `WidgetTree` (one per OS window) owns its
/// `global_actions`/`shortcut_registry`, and this window never builds an `App`.
/// `intent(..)` and not an inline `on_activate` closure, because the model is
/// built before the window's `WelcomeViewModel` exists (it needs a
/// `SettingsStore`, so `WelcomePanel::build` mints it) — and because the Welcome
/// pane's own New Work / Open / "Create from…" controls then fire the very same
/// actions rather than a second copy of the logic.
pub(crate) fn build_launcher_menu() -> MenuModel {
    MenuModel::new()
        .standard_menu(app_standard_menu_base())
        // Named "Work" like the project window's, and holding the same kind of
        // thing: what you can do to a project. Here that is the five ways in and
        // the way out — blank project first, exactly as the project window's own
        // Work menu opens.
        .menu(tr!(menu_work()), |m| {
            m.item(
                MenuEntry::new(tr!(menu_new_work()))
                    .intent(ACTION_NEW)
                    .shortcut(ACTION_NEW),
            )
            .item(
                MenuEntry::new(tr!(menu_open_work()))
                    .intent(ACTION_OPEN)
                    .shortcut(ACTION_OPEN),
            )
            .separator()
            // "Create from", not the project window's "Import from": both of
            // these produce a brand-new `.skrib`, where that menu's importers
            // land content *in* the project already open. A submenu, so a third
            // importer slots in without a second top-level row.
            .submenu(tr!(menu_create_from()), |s| {
                // The **project window's own row labels**, reused verbatim: the two
                // importers are the same two here, and naming them differently in
                // the two menus would read as two different features. Only the
                // wrapper differs — "Create from" against Work ▸ "Import from" —
                // because that is the one thing that really is different.
                s.item(
                    MenuEntry::new(tr!(menu_import_document())).intent(ACTION_NEW_FROM_DOCUMENTS),
                )
                .item(MenuEntry::new(tr!(menu_import_plume())).intent(ACTION_IMPORT_PLUME))
                .item(
                    MenuEntry::new(tr!(menu_import_manuskript())).intent(ACTION_IMPORT_MANUSKRIPT),
                )
            })
            .separator()
            // App preferences — in the same slot the project window's Work menu
            // puts them, just above Quit, and hidden on macOS for the same
            // reason: the App menu carries Settings at ⌘, there, and two items
            // claiming one key equivalent leave AppKit to pick by traversal
            // order.
            .item(
                MenuEntry::new(tr!(menu_settings()))
                    .visible(NOT_ON_MACOS)
                    .intent(ACTION_SETTINGS)
                    .shortcut(ACTION_SETTINGS),
            )
            .separator()
            // Hidden on macOS: Quit lives in the App menu there, and two items
            // claiming ⌘Q leave AppKit to pick by traversal order. Same rule and
            // same `NOT_ON_MACOS` lever as the project window's Work ▸ Quit.
            .item(
                MenuEntry::new(tr!(menu_quit()))
                    .visible(NOT_ON_MACOS)
                    .intent("app.quit")
                    .shortcut("app.quit"),
            )
        })
        // Worth declaring even for a window that cannot be resized: this process
        // holds every open project's window, so on macOS the Window menu's live
        // window list is the only surface from the Launcher that names them —
        // and Minimize is a real command here. Zoom is inert on a fixed-size
        // window, which is AppKit's business, not a reason to withhold the menu.
        .standard_menu(window_standard_menu())
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::widgets::{MenuNode, StandardMenuRole};

    fn standard_role(node: &MenuNode) -> Option<StandardMenuRole> {
        match node {
            MenuNode::Standard(menu) => Some(menu.role()),
            _ => None,
        }
    }

    /// The App menu leads and the Window menu closes, the order macOS expects —
    /// AppKit renders whatever leads the bar in the bold app-name slot whether
    /// or not it belongs there.
    #[test]
    fn the_standard_menus_bracket_the_model() {
        let model = build_launcher_menu();
        let nodes = model.nodes();
        assert_eq!(
            standard_role(&nodes[0]),
            Some(StandardMenuRole::App),
            "the application menu must lead"
        );
        assert_eq!(
            standard_role(&nodes[nodes.len() - 1]),
            Some(StandardMenuRole::Window),
            "the Window menu closes the bar"
        );
    }

    /// Exactly one ordinary submenu, so the Linux hamburger opens onto one menu.
    /// A stray `Standard` node would be silently dropped by the in-window bar
    /// (`model_entries` keeps only `Submenu`s) and a stray top-level `Item`
    /// renders in neither bar — both invisible failures on the platform they do
    /// not break.
    #[test]
    fn the_in_window_bar_shows_exactly_the_work_menu() {
        let model = build_launcher_menu();
        let nodes = model.nodes();
        let submenus: Vec<String> = nodes
            .iter()
            .filter_map(|n| match n {
                MenuNode::Submenu { title, .. } => Some(title.resolve_now()),
                _ => None,
            })
            .collect();
        assert_eq!(submenus, vec![tr!(menu_work()).resolve_now()]);
        assert!(
            nodes
                .iter()
                .all(|n| standard_role(n).is_some() || matches!(n, MenuNode::Submenu { .. })),
            "a bare item or separator at top level renders in neither bar"
        );
    }

    /// The Work menu's shape: New Work, Open Work, the Create-from submenu,
    /// Settings, Quit.
    ///
    /// `MenuEntry`'s title and intent are `pub(crate)` to teksilo, so a test here
    /// can see a row's *kind* and *position* but not its label or the action it
    /// fires. Neither half is left to a test: every intent name is one of the
    /// [`ACTION_NEW`]/[`ACTION_OPEN`]/[`ACTION_NEW_FROM_DOCUMENTS`]/
    /// [`ACTION_IMPORT_PLUME`] constants this module and `WelcomePanel` both read,
    /// and every label is a `tr!` key the project window's own menu already uses,
    /// so a row cannot disagree with either its action or its twin.
    #[test]
    fn the_work_menu_offers_five_ways_in_the_preferences_and_one_way_out() {
        let model = build_launcher_menu();
        let nodes = model.nodes();
        let MenuNode::Submenu {
            title, children, ..
        } = &nodes[1]
        else {
            panic!("the Work menu is the model's second node");
        };
        assert_eq!(title.resolve_now(), tr!(menu_work()).resolve_now());
        assert!(
            matches!(
                children.as_slice(),
                [
                    MenuNode::Item(_),
                    MenuNode::Item(_),
                    MenuNode::Separator,
                    MenuNode::Submenu { .. },
                    MenuNode::Separator,
                    MenuNode::Item(_),
                    MenuNode::Separator,
                    MenuNode::Item(_)
                ]
            ),
            "New Work · Open Work · separator · Create from ▸ … · separator · \
             Settings · separator · Quit"
        );
        let MenuNode::Submenu {
            title, children, ..
        } = &children[3]
        else {
            unreachable!("matched just above");
        };
        assert_eq!(title.resolve_now(), tr!(menu_create_from()).resolve_now());
        assert_eq!(
            children.len(),
            3,
            "three ways in — a document set, a Plume Creator project, a Manuskript project"
        );
        assert!(
            children.iter().all(|n| matches!(n, MenuNode::Item(_))),
            "Create from holds leaf rows only"
        );
    }

    /// ⌘Q on macOS must reach the app's guarded quit, not AppKit's `terminate:`
    /// — which skips winit's exit path entirely, so `QuitSequencer` would never
    /// run and a project open in another window would be closed over its unsaved
    /// prose with no prompt and no on-close backup. Asserted on the model
    /// because no test on this machine can reach the NSMenu construction.
    #[test]
    fn the_macos_quit_is_routed_through_the_guarded_action() {
        let app = app_standard_menu_base();
        assert_eq!(app.quit_intent_name(), Some("app.quit"));
        assert_eq!(app.quit_shortcut_id(), Some("app.quit"));
    }

    /// ⌘, on macOS reaches the preferences window from the Launcher too.
    ///
    /// It did not: the App menu's Settings row was layered on by a
    /// project-window-only variant, on the reasoning that `SettingsPanel` needs
    /// a `WorkSession`. On Linux and Windows that left every app-level
    /// preference unreachable in the state the app *starts* in; on macOS it left
    /// the App menu without its standard ⌘, whenever the Launcher held focus.
    /// The route is now on the shared base, so neither window can lose it
    /// without the other noticing.
    #[test]
    fn the_launcher_app_menu_reaches_settings() {
        let app = app_standard_menu_base();
        assert_eq!(app.settings_intent_name(), Some("app.settings"));
        assert_eq!(
            app.settings_shortcut_id(),
            Some("app.settings"),
            "the chord comes from the registry, so the row follows a rebind"
        );
    }
}
