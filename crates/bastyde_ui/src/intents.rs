//! App-wide commands as typed intents — the scriptable command surface.
//!
//! An intent is a named command any handler can fire (`ctx.send_intent(...)`),
//! consumed by an `Action` registered in `App::build`. Unit intents
//! (`ToggleOutline`) react on name alone, so a menu (`MenuEntry::intent(..)`) or
//! a `Shortcut` can trigger them without constructing the variant; data-bearing
//! intents (`OpenItem`) carry a typed payload recovered via `from_intent`.
//!
//! The variants are the command catalog: fired by name from menus/shortcuts, and
//! constructed by handler-driven callers added incrementally (a command palette,
//! recents, search results), so not all are constructed in app code yet.

use bastyde::IntentKind; // derive macro

use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

#[allow(dead_code)]
#[derive(Debug, IntentKind)]
pub enum AppIntent {
    /// Toggle the binder/outline dock's visibility.
    #[name = "outline.toggle"]
    ToggleOutline,

    /// Show the Welcome modal (start screen). Fired at startup (unless a work
    /// path was passed), from the File ▸ Welcome… menu, and from the brand
    /// icon button. Consumed by a global `welcome.show` action in `App::build`.
    #[name = "welcome.show"]
    ShowWelcome,

    /// Create a new work — presents the New Work modal. Fired from File ▸ New
    /// Work, Ctrl+N, and the Welcome panel's "New Work" button. Consumed by a
    /// global `work.new` action in `App::build`.
    #[name = "work.new"]
    NewWork,

    /// Import a Plume Creator (.plume) project — presents the Import Plume modal.
    /// Fired from File ▸ Import from ▸ Plume Creator. Consumed by a global
    /// `work.import_plume` action in `App::build`.
    #[name = "work.import_plume"]
    ImportPlumeCreator,

    /// Open (or focus) the editor tab for a binder item.
    #[name = "editor.open_item"]
    OpenItem { item_id: u64, title: String },

    /// Create a new binder item. A "folder" is just `role = Folder` — there is
    /// no separate `NewFolder` (the *New Folder* affordance fires this with
    /// `role = Folder`).
    #[name = "binder.new_item"]
    NewItem {
        role: BinderItemRole,
        sub_role: BinderItemSubRole,
    },

    /// Rename the selected binder/item (presents an input dialog).
    #[name = "binder.rename"]
    Rename,

    /// Duplicate the selected item subtrees.
    #[name = "binder.duplicate"]
    Duplicate,

    /// Move the selected binders/items to trash.
    #[name = "binder.trash_selected"]
    TrashSelected,

    /// Move one specific binder to trash — fired from the switcher popover's
    /// context menu (after a confirmation), carrying the binder id (not the
    /// current selection). Consumed by the `binder.trash` global action.
    #[name = "binder.trash"]
    TrashBinder { binder_id: i64 },

    /// Indent / outdent the selected items.
    #[name = "binder.indent"]
    Indent,
    #[name = "binder.outdent"]
    Outdent,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::prelude::{Intent, IntentKind as _};

    #[test]
    fn open_item_round_trips_its_payload() {
        let intent: Intent = AppIntent::OpenItem {
            item_id: 7,
            title: "Scene".into(),
        }
        .into();
        match AppIntent::from_intent(&intent) {
            Some(AppIntent::OpenItem { item_id, title }) => {
                assert_eq!(*item_id, 7);
                assert_eq!(title, "Scene");
            }
            other => panic!("expected OpenItem, got {other:?}"),
        }
    }

    #[test]
    fn trash_binder_round_trips_its_payload() {
        let intent: Intent = AppIntent::TrashBinder { binder_id: 42 }.into();
        match AppIntent::from_intent(&intent) {
            Some(AppIntent::TrashBinder { binder_id }) => assert_eq!(*binder_id, 42),
            other => panic!("expected TrashBinder, got {other:?}"),
        }
    }

    #[test]
    fn toggle_outline_unit_intent_bridges() {
        let intent: Intent = AppIntent::ToggleOutline.into();
        assert!(matches!(
            AppIntent::from_intent(&intent),
            Some(AppIntent::ToggleOutline)
        ));
    }

    #[test]
    fn show_welcome_unit_intent_bridges() {
        let intent: Intent = AppIntent::ShowWelcome.into();
        assert!(matches!(
            AppIntent::from_intent(&intent),
            Some(AppIntent::ShowWelcome)
        ));
    }
}
