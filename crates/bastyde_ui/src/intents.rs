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

use export_management::ExportScopeKind;
use skribisto_model::{CreateType, Relation};

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

    /// Open an **already-chosen** `.skrib` over the project in this window.
    ///
    /// Fired by the doors that pick the path themselves and live outside `App`:
    /// the project switcher's "Open here" and the Plume import toast's "Open
    /// now". Consumed by the global `work.open_path` action, which runs it
    /// through the unsaved-changes guard (`ProjectSwitchViewModel`) — replacing
    /// the open project destroys its unsaved edits otherwise. Distinct from
    /// `work.open`, which *picks* a file first.
    ///
    /// The intent bus (rather than either caller reaching for the guard directly)
    /// is what keeps the view-model graph a DAG — same shape as `binder.trash`.
    #[name = "work.open_path"]
    OpenWorkPath { path: String },

    /// Open (or focus) the editor tab for a binder item (primary pane).
    #[name = "editor.open_item"]
    OpenItem { item_id: u64, title: String },

    /// Open (or focus) a binder item in the **side** pane, revealing the split.
    /// Fired from the outline's "Open to the Side" (context menu / Ctrl+Enter /
    /// middle-click). Consumed by the `editor.open_item_to_side` global action.
    #[name = "editor.open_item_to_side"]
    OpenItemToSide { item_id: u64, title: String },

    /// Create a new binder item of a logical `CreateType`, placed by `relation`
    /// relative to the current selection. The header "Create" SplitButton fires
    /// this with the recommended type + relation; the concrete `(role, sub_role)`
    /// is resolved from the project's chapter mode at execution time.
    #[name = "binder.new_item"]
    NewItem {
        create_type: CreateType,
        relation: Relation,
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

    /// Open the Export panel pre-scoped to a quick scope resolved from the
    /// current focus (Export Book / Part / Chapter / Scene / Note / Folder).
    /// Fired by name+payload from the focus-adaptive Export split-button in the
    /// title bar and the matching File ▸ Export submenu; the concrete anchor
    /// (the focused item) is read by the `export.scope` global action at
    /// dispatch time, so the payload carries only the scope. Consumed in
    /// `App::build`.
    #[name = "export.scope"]
    ExportScoped { scope: ExportScopeKind },
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
    fn open_work_path_round_trips_its_payload() {
        // The switcher's "Open here" and the import toast's "Open now" carry the
        // path across the bus to the guard; a lost payload would mean opening
        // nothing (or, worse, the wrong project) after the guard's save.
        let intent: Intent = AppIntent::OpenWorkPath {
            path: "/tmp/novel.skrib".into(),
        }
        .into();
        match AppIntent::from_intent(&intent) {
            Some(AppIntent::OpenWorkPath { path }) => assert_eq!(path, "/tmp/novel.skrib"),
            other => panic!("expected OpenWorkPath, got {other:?}"),
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
    fn export_scoped_round_trips_its_scope() {
        // The split-button/menu fire the scope across the bus; a lost payload would
        // export the wrong extent (a whole book instead of one scene).
        let intent: Intent = AppIntent::ExportScoped {
            scope: ExportScopeKind::CurrentScene,
        }
        .into();
        match AppIntent::from_intent(&intent) {
            Some(AppIntent::ExportScoped { scope }) => {
                assert_eq!(*scope, ExportScopeKind::CurrentScene)
            }
            other => panic!("expected ExportScoped, got {other:?}"),
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
