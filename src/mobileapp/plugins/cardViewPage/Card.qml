import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQml
import QtQuick.Controls.Material
import eu.skribisto.projecthub 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import eu.skribisto.taghub 1.0
import eu.skribisto.skr 1.0

import Skribisto
import SkrControls
import theme

CardForm {
    id: root

    TapHandler {
        id: tapHandler

        onSingleTapped: {
            forceActiveFocus()
        }
    }

    onActiveFocusChanged: {
        if (activeFocus) {
            root.GridView.view.currentProjectId = model.projectId
            root.GridView.view.currentTreeItemId = model.treeItemId
            root.GridView.view.currentIndex = model.index
        }
    }

    Component {
        id: tagBoxComponent
        TagPad {
            id: tagPad

            minimalMode: true
            projectId: model.projectId
            treeItemId: model.treeItemId

            //proxy model for tag list :
            SKRSearchTagListProxyModel {
                id: tagProxyModel
                projectIdFilter: model.projectId
                treeItemIdFilter: model.treeItemId
            }
            tagListModel: tagProxyModel
        }
    }

    Component {
        id: outlineWritingZoneComponent

        OutlineWritingZone {
            id: writingZone

            property string pageType: model.type

            clip: true
            projectId: model.projectId
            treeItemId: model.treeItemId
            spellCheckerKilled: true
            leftScrollItemVisible: false
            rightScrollItemVisible: Globals.touchUsed
            placeholderText: qsTr("Outline")

            textPointSize: SkrSettings.cardViewOutlineSettings.textPointSize
            textFontFamily: SkrSettings.cardViewOutlineSettings.textFontFamily
            textIndent: SkrSettings.cardViewOutlineSettings.textIndent
            textTopMargin: SkrSettings.cardViewOutlineSettings.textTopMargin

            stretch: true

            textAreaStyleBackgroundColor: SkrTheme.secondaryTextAreaBackground
            textAreaStyleForegroundColor: SkrTheme.secondaryTextAreaForeground
            paneStyleBackgroundColor: SkrTheme.listItemBackground
            textAreaStyleAccentColor: SkrTheme.accent
            name: "cardViewOutline"

            //-----Zoom------------------------------------------------------------
            Shortcut {
                sequences: skrShortcutManager.shortcuts(
                               "plugin-cardview-outline-text-zoom-in")
                context: Qt.WindowShortcut
                enabled: writingZone.activeFocus
                onActivated: {
                    SkrSettings.cardViewOutlineSettings.textPointSize += 1
                }
            }

            Shortcut {
                sequences: skrShortcutManager.shortcuts(
                               "plugin-cardview-outline-text-zoom-out")
                context: Qt.WindowShortcut
                enabled: root.activeFocus
                onActivated: {
                    SkrSettings.cardViewOutlineSettings.textPointSize -= 1
                }
            }
        }
    }

    pageTypeIcon.source: skrTreeManager.getIconUrlFromPageType(model.type)

    function getIconUrlFromPageType(type) {
        return skrTreeManager.getIconUrlFromPageType(type)
    }

    goToChildToolButton.onClicked: root.GridView.view.currentParentId = model.treeItemId

    //-----------------------------------------------------------
    //-----------dropArea----------------------------------------
    //-----------------------------------------------------------
    touchTapHandler.onLongPressed: {
        Globals.touchUsed = true
        draggableContent.dragging = true
    }

    dropArea.keys: ["application/skribisto-cardview-tree-item"]
    dropArea.onEntered: function (drag) {
        root.GridView.view.visualModel.items.move(drag.source.visualIndex,
                                                  draggableContent.visualIndex)
    }

    dropArea.onDropped: function (drop) {
        if (drop.proposedAction === Qt.MoveAction) {
            root.GridView.view.proxyModel.moveItem(
                        root.GridView.view.moveSourceIndex,
                        draggableContent.visualIndex)
        }
    }

    //--------------------------------------------------------------
    mouseDragHandler.onActiveChanged: {
        console.log("onActiveChanged", mouseDragHandler.active,
                    draggableContent.visualIndex)
        if (mouseDragHandler.active) {
            Globals.touchUsed = false
            root.GridView.view.moveSourceIndex = draggableContent.visualIndex
            root.GridView.view.dragging = true
            draggableContent.dragging = true
            draggableContent.Drag.start()
        } else {
            draggableContent.Drag.drop()
            root.GridView.view.dragging = false
            draggableContent.dragging = false
            root.GridView.view.moveSourceIndex = -2
        }
    }
    mouseDragHandler.enabled: true

    mouseDragHandler.onCanceled: {
        console.log("onCanceled")
        root.GridView.view.dragging = false
        draggableContent.dragging = false
        root.GridView.view.moveSourceIndex = -2
    }

    mouseDragHandler.grabPermissions: PointerHandler.CanTakeOverFromItems
                                      | PointerHandler.CanTakeOverFromAnything
                                      | PointerHandler.TakeOverForbidden

    //--------------------------------------------------------------
    touchDragHandler.onActiveChanged: {
        //        console.log("onActiveChanged",
        //                    touchDragHandler.active, draggableContent.visualIndex)
        if (touchDragHandler.active) {
            Globals.touchUsed = true
            root.GridView.view.moveSourceIndex = draggableContent.visualIndex
            root.GridView.view.dragging = true
            draggableContent.dragging = true
            draggableContent.Drag.start()
        } else {
            draggableContent.Drag.drop()
            root.GridView.view.dragging = false
            draggableContent.dragging = false
            root.GridView.view.moveSourceIndex = -2
        }
    }
    touchDragHandler.enabled: draggableContent.dragging

    touchDragHandler.onCanceled: {
        console.log("onCanceled")
        root.GridView.view.dragging = false
        draggableContent.dragging = false
        root.GridView.view.moveSourceIndex = -2
    }

    touchDragHandler.grabPermissions: PointerHandler.CanTakeOverFromItems
                                      | PointerHandler.CanTakeOverFromAnything
                                      | PointerHandler.TakeOverForbidden

    //--------------------------------------------------------------
    titleTapHandler.onSingleTapped: {
        renameDialog.projectId = model.projectId
        renameDialog.treeItemId = model.treeItemId
        renameDialog.treeItemTitle = skrData.treeHub().getTitle(
                    model.projectId, model.treeItemId)
        renameDialog.open()
    }

    SimpleDialog {
        id: renameDialog
        property int projectId: -2
        property int treeItemId: -2
        property string treeItemTitle: ""
        title: qsTr("Rename an item")
        contentItem: SkrTextField {
            id: renameTextField
            text: renameDialog.treeItemTitle

            onAccepted: {
                renameDialog.accept()
            }
        }

        standardButtons: Dialog.Ok | Dialog.Cancel

        onRejected: {
            renameDialog.treeItemTitle = ""
        }

        onDiscarded: {

            renameDialog.treeItemTitle = ""
        }

        onAccepted: {
            skrData.treeHub().setTitle(renameDialog.projectId,
                                       renameDialog.treeItemId,
                                       renameTextField.text)

            renameDialog.treeItemTitle = ""
        }

        onActiveFocusChanged: {
            if (activeFocus) {
                contentItem.forceActiveFocus()
            }
        }

        onOpened: {
            contentItem.forceActiveFocus()
            renameTextField.selectAll()
        }
    }

    //--------------------------------------------------------------
    states: [
        State {
            name: "drag_active"
            when: draggableContent.Drag.active

            ParentChange {
                target: draggableContent
                parent: Overlay.overlay
            }
            AnchorChanges {
                target: draggableContent

                anchors.left: undefined
                anchors.right: undefined
                anchors.top: undefined
                anchors.bottom: undefined
            }

            PropertyChanges {
                target: root
                z: 2
            }
        },

        State {
            name: "unset_anchors"
            AnchorChanges {
                target: draggableContent
                anchors.left: undefined
                anchors.right: undefined
                anchors.top: undefined
                anchors.bottom: undefined
            }
        }
    ]

    //-----------------------------------------------------------
    //---------- Keys----------------------------------------
    //-----------------------------------------------------------
    Keys.onPressed: function (event) {

        if (event.key === Qt.Key_Right) {
            console.log("Right key pressed")
            event.accepted = true
        }
    }
}
