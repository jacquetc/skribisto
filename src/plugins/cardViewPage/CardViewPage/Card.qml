import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml 2.15
import QtQuick.Controls.Material 2.15
import eu.skribisto.projecthub 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import eu.skribisto.taghub 1.0
import eu.skribisto.skr 1.0
import "../../Commons"
import "../.."

CardForm {
    id: root

    TapHandler{
        id: tapHandler

        onSingleTapped: {
            forceActiveFocus()
        }
    }

    onActiveFocusChanged: {
        if(activeFocus){
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
        Globals.touchUsed  = true
        draggableContent.dragging = true
    }



    dropArea.keys: ["application/skribisto-cardview-tree-item"]
    dropArea.onEntered: function(drag) {
        root.GridView.view.visualModel.items.move(drag.source.visualIndex, draggableContent.visualIndex)
    }

    dropArea.onDropped: function(drop)  {
        if (drop.proposedAction === Qt.MoveAction) {
            root.GridView.view.proxyModel.moveItem(root.GridView.view.moveSourceIndex, draggableContent.visualIndex)
        }
    }


    //--------------------------------------------------------------

    mouseDragHandler.onActiveChanged: {
        //        console.log("onActiveChanged",
        //                    mouseDragHandler.active, draggableContent.visualIndex)
        if (mouseDragHandler.active) {
            Globals.touchUsed  = false
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
                                      | PointerHandler.CanTakeOverFromAnything | PointerHandler.TakeOverForbidden



    //--------------------------------------------------------------

    touchDragHandler.onActiveChanged: {
        //        console.log("onActiveChanged",
        //                    touchDragHandler.active, draggableContent.visualIndex)
        if (touchDragHandler.active) {
            Globals.touchUsed  = true
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
                                      | PointerHandler.CanTakeOverFromAnything | PointerHandler.TakeOverForbidden


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

}
