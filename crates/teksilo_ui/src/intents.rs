// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

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

use teksilo::IntentKind; // derive macro

use export_management::ExportScopeKind;
use skribisto_model::{CreateType, Relation};

#[allow(dead_code)]
#[derive(Debug, IntentKind)]
pub enum AppIntent {
    /// Toggle the binder/outline dock's visibility.
    #[name = "outline.toggle"]
    ToggleOutline,

    /// Clear the chapter and part titles that say nothing but their own number.
    ///
    /// Every project this app creates starts with its chapters titled "Chapter 1".."Chapter
    /// N", because until the ordinal became visible in the binder that was the only place a
    /// writer could see it. Now that the badge shows it, those titles read "3. Chapter 3".
    /// Fired from the Document menu; consumed by the global `numbering.tidy_titles` action
    /// in `App::build`, which previews the change and asks before touching anything.
    #[name = "numbering.tidy_titles"]
    TidyNumberTitles,

    /// Toggle THIS window between fullscreen and whatever placement it had
    /// before (Maximized/Floating) — Increment 1 of distraction-free (plain
    /// fullscreen; not distraction-free mode itself). Fired by F11 and the
    /// View ▸ Fullscreen menu entry. Consumed by the global `view.fullscreen`
    /// action in `App::build`, which resolves the firing window from the
    /// `EventContext` rather than any captured handle — correct with several
    /// project windows open.
    #[name = "view.fullscreen"]
    ToggleFullscreen,

    /// Toggle distraction-free mode on THIS window — Increment 2 of
    /// distraction-free: chrome collapses (menu bar, backup banner, status
    /// bar, the binder/inspector/preview docks), the editor fills the window,
    /// and the window goes fullscreen, all together. Independent of
    /// [`Self::ToggleFullscreen`] — see `FocusViewModel`'s module doc for why
    /// the two keep separate placement memory. Fired by Shift+F11, the
    /// View ▸ Distraction-free Mode menu entry, and the mode's own strip
    /// Exit button. Consumed by the global `view.focus_mode` action in
    /// `App::build`, which resolves the firing window from the
    /// `EventContext` rather than any captured handle — correct with several
    /// project windows open.
    #[name = "view.focus_mode"]
    ToggleFocusMode,

    /// Start or pause the status-bar writing session. Fired by name (palette /
    /// automation); the play/pause button is the primary control. Consumed by a
    /// global `session.toggle` action in `App::build`.
    #[name = "session.toggle"]
    ToggleWritingSession,

    /// Show the Welcome modal (start screen). Fired at startup (unless a work
    /// path was passed) and from the File ▸ Welcome… menu. Consumed by a
    /// global `welcome.show` action in `App::build`.
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

    /// Import Markdown / plain-text documents **into the open project** —
    /// presents the Import documents wizard. Fired from File ▸ Import from ▸
    /// Documents. Consumed by a global `work.import_document` action.
    ///
    /// Unlike [`Self::ImportPlumeCreator`], which produces a brand-new `.skrib`
    /// nobody has opened, this one writes into the Work this window is showing —
    /// which is why its view-model is per-window (Tier 3) and threaded through
    /// `CommandDeps` rather than resolved from `ctx.app_state`.
    #[name = "work.import_document"]
    ImportDocument,

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
    /// relative to an anchor. The header "Create" SplitButton fires this with the
    /// recommended type + relation; the concrete `(role, sub_role)` is resolved
    /// from the project's chapter mode at execution time.
    ///
    /// `anchor_item_id` names the item the new one is placed relative to.
    /// `None` = the current Outline selection (the outline dock's own "Create").
    /// The corkboard passes `Some(current_container)` because its drilled-into
    /// container is independent of the Outline's selection.
    #[name = "binder.new_item"]
    NewItem {
        create_type: CreateType,
        relation: Relation,
        anchor_item_id: Option<u64>,
    },

    /// Import documents straight into a place the writer has already pointed at, from
    /// the binder's own context menu — the wizard opens with its destination step
    /// already answered.
    ///
    /// Carries a [`BinderTreeKey`](crate::models::BinderTreeKey), the durable uid, and
    /// not a store id: an intent is dispatched a frame or more after the click, and an
    /// `EntityId` is only meaningful until the next `load_work`. Consumed by the
    /// `binder.import_here` global action.
    #[name = "binder.import_here"]
    ImportHere {
        destination: crate::models::BinderTreeKey,
    },

    /// Reveal a binder item in the outline dock: show the dock and select the row.
    /// Fired from the Overview table's context menu ("where does this sit in the
    /// project?"), carrying the item id rather than relying on any shared selection.
    ///
    /// Over the bus rather than the Overview holding an `OutlineViewModel`, for the same
    /// reason as `work.open_path`: peer view-models do not import each other, so `App`
    /// mediates and the graph stays a DAG.
    #[name = "binder.reveal_in_outline"]
    RevealInOutline { item_id: u64 },

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

    /// Insert one note template's body at the caret of the focused note's prose editor.
    ///
    /// Data-bearing, so the Document menu's per-template rows can each carry their own id
    /// rather than the command having to re-derive "which one was clicked". Consumed by the
    /// `editor.insert_template` global action.
    #[name = "editor.insert_template"]
    InsertTemplate { template_id: u64 },

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

    /// Restore one or more trashed roots in place (from the trash dock's context
    /// menu / automation). If the backend reports `orphaned`, the destination
    /// picker opens automatically. Consumed by the global `trash.restore` action.
    #[name = "trash.restore"]
    RestoreTrashed { trash_info_ids: Vec<u64> },

    /// Restore a single trashed item to a chosen destination — fired by the
    /// editor's trash banner (which has the item id, not a TrashInfo). Opens the
    /// destination picker. Consumed by the global `trash.restore_item` action.
    #[name = "trash.restore_item"]
    RestoreTrashedItem { item_id: u64 },

    /// Permanently delete one or more trashed roots (behind a confirmation +
    /// undo-grace toast). Consumed by the global `trash.delete_forever` action.
    #[name = "trash.delete_forever"]
    DeleteTrashForever { trash_info_ids: Vec<u64> },

    /// Add word(s) to the open project's personal dictionary — fired from the
    /// editor's "Add to dictionary" context-menu item with the resolved
    /// selection/caret words. The menu mounts at the arena root, so it reaches
    /// only a **global** action (`editor.add_to_dictionary`), which delegates to
    /// `UserDictionaryViewModel::add_words` + shows the added-toast.
    #[name = "editor.add_to_dictionary"]
    AddWordsToDictionary { words: Vec<String> },
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::prelude::{Intent, IntentKind as _};

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
    fn add_words_to_dictionary_round_trips_its_payload() {
        // The editor menu carries the resolved words across the bus to the global
        // action; a lost payload would add nothing (or the wrong words).
        let intent: Intent = AppIntent::AddWordsToDictionary {
            words: vec!["Gandalf".into(), "Skribisto".into()],
        }
        .into();
        match AppIntent::from_intent(&intent) {
            Some(AppIntent::AddWordsToDictionary { words }) => {
                assert_eq!(words, &["Gandalf".to_string(), "Skribisto".to_string()]);
            }
            other => panic!("expected AddWordsToDictionary, got {other:?}"),
        }
    }

    #[test]
    fn restore_trashed_round_trips_its_payload() {
        let intent: Intent = AppIntent::RestoreTrashed {
            trash_info_ids: vec![7, 9],
        }
        .into();
        match AppIntent::from_intent(&intent) {
            Some(AppIntent::RestoreTrashed { trash_info_ids }) => {
                assert_eq!(trash_info_ids, &[7, 9])
            }
            other => panic!("expected RestoreTrashed, got {other:?}"),
        }
    }

    #[test]
    fn restore_trashed_item_round_trips_its_payload() {
        let intent: Intent = AppIntent::RestoreTrashedItem { item_id: 8200 }.into();
        match AppIntent::from_intent(&intent) {
            Some(AppIntent::RestoreTrashedItem { item_id }) => assert_eq!(*item_id, 8200),
            other => panic!("expected RestoreTrashedItem, got {other:?}"),
        }
    }

    #[test]
    fn delete_trash_forever_round_trips_its_payload() {
        let intent: Intent = AppIntent::DeleteTrashForever {
            trash_info_ids: vec![3],
        }
        .into();
        match AppIntent::from_intent(&intent) {
            Some(AppIntent::DeleteTrashForever { trash_info_ids }) => {
                assert_eq!(trash_info_ids, &[3])
            }
            other => panic!("expected DeleteTrashForever, got {other:?}"),
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

    #[test]
    fn toggle_fullscreen_unit_intent_bridges() {
        let intent: Intent = AppIntent::ToggleFullscreen.into();
        assert!(matches!(
            AppIntent::from_intent(&intent),
            Some(AppIntent::ToggleFullscreen)
        ));
    }

    #[test]
    fn toggle_focus_mode_unit_intent_bridges() {
        let intent: Intent = AppIntent::ToggleFocusMode.into();
        assert!(matches!(
            AppIntent::from_intent(&intent),
            Some(AppIntent::ToggleFocusMode)
        ));
    }
}
