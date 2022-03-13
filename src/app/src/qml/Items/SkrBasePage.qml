import QtQuick
import QtQuick.Controls
import ".."
import "../Commons"

FocusScope {
    id: control

    default property alias content: dropArea.data
    required property var viewManager
    required property int position
    property int projectId: -1
    property int treeItemId: -1
    property string pageType: ""
    property var additionalPropertiesForSavingView: ({})
    property bool dropAreaEnabled: true
    property list<Component> toolboxes

    clip: true

    signal closeViewCalled

    function closeView() {
        closeViewCalled()
        viewManager.closeView(position)
    }

    onActiveFocusChanged: {
        if (activeFocus) {
            viewManager.focusedPosition = position
            if (projectId !== -1 & treeItemId !== -1) {
                rootWindow.protectedSignals.setBreadcrumbCurrentTreeItemCalled(
                            projectId, treeItemId)
                rootWindow.setNavigationTreeItemIdCalled(projectId, treeItemId)
            }
        }
    }

    DropArea {
        id: dropArea
        anchors.fill: parent

        keys: dropAreaEnabled ? ["application/skribisto-tree-item"] : []
        onEntered: {
            if (drag.keys.includes("application/skribisto-tree-item")) {
                dropIndicator.visible = true
            }
        }
        onExited: {
            //console.debug("exited page")
            dropIndicator.visible = false
        }

        onDropped: {
            //console.debug("dropped page")
            if (drop.proposedAction === Qt.MoveAction) {
                viewManager.loadTreeItemAt(drag.source.projectId,
                                           drag.source.treeItemId, position)
            }
            dropIndicator.visible = false
        }
    }
    property alias newIdenticalPageAction: newIdenticalPageAction
    Action {
        id: newIdenticalPageAction
        text: skrShortcutManager.description("create-new-identical-page")
        enabled: control.treeItemId > 0
        icon.source: "qrc:///icons/backup/document-new.svg"
        onTriggered: {
            var result = skrData.treeHub().addTreeItemBelow(control.projectId,
                                                            control.treeItemId,
                                                            control.pageType)
            var newTreeItemAdded = result.getData("treeItemId", -1)
            if (newTreeItemAdded === -1) {
                skrData.errorHub().addWarning(
                            qsTr("newIdenticalPageShortcut: Item not created"))
            } else {
                viewManager.loadTreeItem(control.projectId, newTreeItemAdded)
            }
        }
    }
    Shortcut {
        id: newIdenticalPageShortcut
        enabled: control.activeFocus && control.treeItemId > 0
        sequences: skrShortcutManager.shortcuts("create-new-identical-page")
        onActivated: {
            newIdenticalPageAction.trigger()
        }
    }

    Rectangle {
        id: dropIndicator
        anchors.fill: parent
        visible: false
        z: 1
        color: "transparent"
        border.color: SkrTheme.accent
        border.width: 4
    }



    Keys.onPressed: function(event) {
        if (event.key === Qt.Key_F2 && treeItemId != -1) {
            if (control.projectId !== -1) {
                console.log("control.projectId", control.projectId, control.treeItemId)
                renameDialog.projectId = control.projectId
                renameDialog.treeItemId = control.treeItemId
                renameDialog.treeItemTitle = skrData.treeHub().getTitle(
                            control.projectId, control.treeItemId)
                renameDialog.open()
            }
        }
    }

    TapHandler {
        acceptedButtons: Qt.LeftButton | Qt.MiddleButton | Qt.RightButton

        onSingleTapped: function(eventPoint){
            control.forceActiveFocus()
        }

        onDoubleTapped: function(eventPoint){
            control.forceActiveFocus()
        }

        grabPermissions: PointerHandler.TakeOverForbidden
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
}
