import QtQuick
import QtQml
import QtQuick.Controls

import "../../../../Items"
import "../../../../Commons"
import "../../../.."

FolderPageForm {
    id: root

    pageType: "FOLDER"

    property string title: {
        return getTitle()
    }

    function getTitle() {
        return skrData.treeHub().getTitle(projectId, treeItemId)

    }

    Connections {
        target: skrData.treeHub()
        function onTitleChanged(_projectId, _treeItemId, newTitle) {
            if (projectId === _projectId && treeItemId === _treeItemId) {
                title = getTitle()
            }
        }
    }

    titleLabel.text: title

    //--------------------------------------------------------
    additionalPropertiesForSavingView: {
        return {}
    }

    //--------------------------------------------------------
    //---View buttons-----------------------------------------
    //--------------------------------------------------------
    viewButtons.viewManager: root.viewManager
    viewButtons.position: root.position

    viewButtons.onOpenInNewWindowCalled: {
//        skrWindowManager.insertAdditionalPropertyForViewManager("isSecondary",
//                                                                isSecondary)
        skrWindowManager.addWindowForItemId(projectId, treeItemId)
        rootWindow.setNavigationTreeItemIdCalled(projectId, treeItemId)
    }

    viewButtons.onSplitCalled: function (position) {
//        viewManager.insertAdditionalProperty("isSecondary", isSecondary)
        viewManager.loadTreeItemAt(projectId, treeItemId, position)
    }

    //--------------------------------------------------------
    //---Tool bar-----------------------------------------
    //--------------------------------------------------------
    Connections {
        target: skrData.treePropertyHub()
        function onPropertyChanged(projectId, propertyId, treeItemId, name, value) {
            if (projectId === root.projectId
                    && treeItemId === root.treeItemId) {

                if (name === "word_count_with_children") {
                    countPriv.wordCount = value
                    updateCountLabel()
                }
            }
        }
    }

    Connections {
        target: skrData.treePropertyHub()
        function onPropertyChanged(projectId, propertyId, treeItemId, name, value) {
            if (projectId === root.projectId
                    && treeItemId === root.treeItemId) {

                if (name === "char_count_with_children") {
                    countPriv.characterCount = value
                    updateCountLabel()
                }
            }
        }
    }

    QtObject {
        id: countPriv
        property string wordCount: ""
        property string characterCount: ""
    }

    function updateCountLabel() {
        var wordCountString = skrRootItem.toLocaleIntString(countPriv.wordCount)
        var characterCountString = skrRootItem.toLocaleIntString(
                    countPriv.characterCount)

        countLabel.text = qsTr("%1 words, %2 characters (all)").arg(
                    wordCountString).arg(characterCountString)
    }

    //-----------------------------------------------------------------
    //----- Page menu ------------------------------------------------
    //-----------------------------------------------------------------
    pageMenuToolButton.onClicked: {
        if (pageMenu.visible) {
            pageMenu.close()
            return
        }
        pageMenu.open()
    }

    SkrMenu {
        id: pageMenu
        y: pageMenuToolButton.height
        x: pageMenuToolButton.x

        SkrMenuItem {
            action: newIdenticalPageAction
        }

        SkrMenuItem {
            action: Action {
                id: showRelationshipPanelAction
                text: skrShortcutManager.description("show-relationship-panel")
                icon.source: "qrc:///icons/backup/link.svg"
                onTriggered: {

                    relationshipPanel.visible = true
                    relationshipPanel.forceActiveFocus()
                }
            }
        }

        SkrMenuItem {
            action: Action {
                id: addQuickNoteAction
                text: skrShortcutManager.description("add-quick-note")
                icon.source: "qrc:///icons/backup/list-add.svg"
                onTriggered: {
                    relationshipPanel.visible = true
                    var result = skrData.treeHub().addQuickNote(projectId,
                                                                treeItemId,
                                                                "TEXT",
                                                                qsTr("Note"))
                    if (result.success) {
                        var newId = result.getData("treeItemId", -2)

                        relationshipPanel.openTreeItemInPanel(projectId, newId)
                        relationshipPanel.forceActiveFocus()
                    }
                }
            }
        }
    }

    Shortcut {
        sequences: skrShortcutManager.shortcuts("add-quick-note")
        enabled: root.activeFocus
        onActivated: {
            addQuickNoteAction.trigger()
        }
    }
    Shortcut {
        sequences: skrShortcutManager.shortcuts("show-relationship-panel")
        enabled: root.activeFocus
        onActivated: {
            showRelationshipPanelAction.trigger()
        }
    }


    //-----------------------------------------------------------------
    //----- Related Panel------------------------------------------------
    //-----------------------------------------------------------------
    relationshipPanel.projectId: root.projectId
    relationshipPanel.treeItemId: root.treeItemId

    relationshipPanel.viewManager: viewManager

    relationshipPanel.onExtendedChanged: {
        if (relationshipPanel.extended) {
            relationshipPanelPreferredHeight = root.height / 2
        } else {
            relationshipPanelPreferredHeight = 200
        }
    }

    Behavior on relationshipPanelPreferredHeight {
        enabled: SkrSettings.ePaperSettings.animationEnabled
        SpringAnimation {
            spring: 5
            mass: 0.2
            damping: 0.2
        }
    }
}
