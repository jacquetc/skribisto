### This file is automatically generated by Qt Design Studio.
### Do not change

set_source_files_properties(Constants.qml
    PROPERTIES
        QT_QML_SINGLETON_TYPE true
)

set_source_files_properties(Globals.qml
    PROPERTIES
        QT_QML_SINGLETON_TYPE true
)

set_source_files_properties(SkrSettings.qml
    PROPERTIES
        QT_QML_SINGLETON_TYPE true
)



qt_add_library(Skribisto STATIC)
qt6_add_qml_module(Skribisto
    URI "Skribisto"
    VERSION 1.0
    QML_FILES
        ItemWordGoalDialog.qml
        SkrViewButtons.qml
        Globals.qml
        TagPad.qml
        NewDictWizardForm.ui.qml
        PropertyPad.qml
        EditView.qml
        SkrSettings.qml
        Constants.qml
        SkrViewManager.qml
        SkrBasePage.qml
        SKRPillarLayout.qml
        CheckableTree.qml
        PluginPage.qml
        ListItemAttributes.qml
        DocumentListView.qml
        SkrShortcut.qml
        SimpleDialog.qml
        NotePadForm.ui.qml
        OutlinePad.qml
        ColorChooser.qml
        NewItemPopup.qml
        TagPadForm.ui.qml
        NotePad.qml
        RelationshipPanelForm.ui.qml
        DocumentListViewForm.ui.qml
        WritingZoneForm.ui.qml
        NewDictWizard.qml
        OutlineWritingZone.qml
        EditViewForm.ui.qml
        TextContextMenuForm.ui.qml
        WritingZone.qml
        SkrBasePageForm.ui.qml
        PluginPageForm.ui.qml
        TextContextMenu.qml
        FindPanel.qml
        NewItemPopupForm.ui.qml
        PropertyPadForm.ui.qml
        SKRTextArea.qml
        RelationshipPanel.qml
        FindPanelForm.ui.qml
        OutlinePadForm.ui.qml
    RESOURCES
        icons/3rdparty/backup/view-split-left-right.svg
        icons/3rdparty/backup/go-previous.svg
        icons/3rdparty/backup/view-list-icons.svg
        icons/3rdparty/backup/format-align-vertical-center.svg
        icons/3rdparty/backup/trash-empty.svg
        icons/3rdparty/backup/remove-link.svg
        icons/3rdparty/backup/accessibility.svg
        icons/3rdparty/backup/process-working.svg
        icons/3rdparty/backup/document-new.svg
        icons/3rdparty/backup/list-remove.svg
        icons/3rdparty/backup/go-top.svg
        icons/3rdparty/backup/configure.svg
        icons/3rdparty/backup/tab-new-background.svg
        icons/3rdparty/backup/edit-delete.svg
        icons/3rdparty/backup/view-pim-notes.svg
        icons/3rdparty/backup/quickopen-file.svg
        icons/3rdparty/backup/artifact.svg
        icons/3rdparty/backup/document-save-as.svg
        icons/3rdparty/backup/roll.svg
        icons/3rdparty/backup/go-next.svg
        icons/3rdparty/backup/view-group.svg
        icons/3rdparty/backup/help-about.svg
        icons/3rdparty/backup/view-right-close.svg
        icons/3rdparty/backup/document-close.svg
        icons/3rdparty/backup/go-next-view.svg
        icons/3rdparty/backup/skribisto-book-beginning.svg
        icons/3rdparty/backup/view-preview.svg
        icons/3rdparty/backup/overflow-menu-left.svg
        icons/3rdparty/backup/tools-media-optical-burn-image.svg
        icons/3rdparty/backup/edit-delete-remove.svg
        icons/3rdparty/backup/go-first.svg
        icons/3rdparty/backup/window-close.svg
        icons/3rdparty/backup/compass.svg
        icons/3rdparty/backup/go-previous-view.svg
        icons/3rdparty/backup/emblem-checked.svg
        icons/3rdparty/backup/go-bottom.svg
        icons/3rdparty/backup/list-add.svg
        icons/3rdparty/backup/go-parent-folder.svg
        icons/3rdparty/backup/edit-select-none.svg
        icons/3rdparty/backup/document-print.svg
        icons/3rdparty/backup/object-rows.svg
        icons/3rdparty/backup/document-edit.svg
        icons/3rdparty/backup/cursor-cross.svg
        icons/3rdparty/backup/go-last.svg
        icons/3rdparty/backup/view-list-text.svg
        icons/3rdparty/backup/go-previous-view-page.svg
        icons/3rdparty/backup/document-open.svg
        icons/3rdparty/backup/go-up-skip.svg
        icons/3rdparty/backup/document-save.svg
        icons/3rdparty/backup/overflow-menu.svg
        icons/3rdparty/backup/tools-check-spelling.svg
        icons/3rdparty/backup/view-fullscreen.svg
        icons/3rdparty/backup/whitebalance.svg
        icons/3rdparty/backup/object-order-raise.svg
        icons/3rdparty/backup/favorite.svg
        icons/3rdparty/backup/go-up.svg
        icons/3rdparty/backup/color-picker-white.svg
        icons/3rdparty/backup/edit-paste.svg
        icons/3rdparty/backup/document-print-direct.svg
        icons/3rdparty/backup/go-next-view-page.svg
        icons/3rdparty/backup/document-print-preview.svg
        icons/3rdparty/backup/view-filter.svg
        icons/3rdparty/backup/tab-new.svg
        icons/3rdparty/backup/format-text-italic.svg
        icons/3rdparty/backup/format-text-strikethrough.svg
        icons/3rdparty/backup/view-hidden.svg
        icons/3rdparty/backup/go-home.svg
        icons/3rdparty/backup/tag-new.svg
        icons/3rdparty/backup/format-text-underline-squiggle.svg
        icons/3rdparty/backup/document-new-from-template.svg
        icons/3rdparty/backup/edit-cut.svg
        icons/3rdparty/backup/go-down-skip.svg
        icons/3rdparty/backup/system-help.svg
        icons/3rdparty/backup/address-book-new.svg
        icons/3rdparty/backup/document-import.svg
        icons/3rdparty/backup/edit-rename.svg
        icons/3rdparty/backup/edit-delete-shred.svg
        icons/3rdparty/backup/go-previous-skip.svg
        icons/3rdparty/backup/overflow-menu-right.svg
        icons/3rdparty/backup/dialog-cancel.svg
        icons/3rdparty/backup/format-font-size-less.svg
        icons/3rdparty/backup/object-order-lower.svg
        icons/3rdparty/backup/skribisto-document-print-none.svg
        icons/3rdparty/backup/format-text-underline.svg
        icons/3rdparty/backup/document-save-all.svg
        icons/3rdparty/backup/dialog-messages.svg
        icons/3rdparty/backup/bookmark-new.svg
        icons/3rdparty/backup/format-text-bold.svg
        icons/3rdparty/backup/story-editor.svg
        icons/3rdparty/backup/go-down.svg
        icons/3rdparty/backup/link.svg
        icons/3rdparty/backup/document-save-as-template.svg
        icons/3rdparty/backup/lock.svg
        icons/3rdparty/backup/edit-copy.svg
        icons/3rdparty/backup/question.svg
        icons/3rdparty/backup/folder-pictures.svg
        icons/3rdparty/backup/quickopen.svg
        icons/3rdparty/backup/edit-find-replace.svg
        icons/3rdparty/backup/view-split-top-bottom.svg
        icons/3rdparty/backup/resizecol.svg
        icons/3rdparty/backup/application-menu.svg
        icons/3rdparty/backup/document-edit-sign.svg
        icons/3rdparty/backup/data-error.svg
        icons/3rdparty/backup/menu_new_sep.svg
        icons/3rdparty/backup/edit-find.svg
        icons/3rdparty/backup/label.svg
        icons/3rdparty/backup/format-font-size-more.svg
        icons/3rdparty/backup/tag.svg
        icons/3rdparty/backup/window-new.svg
        icons/3rdparty/backup/document-open-recent.svg
        icons/3rdparty/backup/skribisto-book-end.svg
        icons/3rdparty/backup/dialog-ok-apply.svg
        icons/3rdparty/backup/smartphone.svg
        icons/3rdparty/backup/view-media-playlist.svg
        icons/3rdparty/backup/view-sort-ascending-name.svg
        icons/3rdparty/backup/edit-clear.svg
        icons/3rdparty/backup/go-next-skip.svg
        icons/3rdparty/backup/document-export.svg
        icons/3rdparty/backup/edit-select-all.svg
        icons/3rdparty/backup/edit-undo.svg
        icons/3rdparty/backup/data-information.svg
        icons/3rdparty/backup/data-warning.svg
        icons/3rdparty/backup/document-edit-verify.svg
        icons/3rdparty/backup/color-picker-black.svg
        icons/3rdparty/backup/view-close.svg
        icons/3rdparty/backup/go-jump.svg
        icons/3rdparty/backup/arrow-down.svg

)
