import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ".."

OutlinePadForm {
    id: root
    property int projectId: -2
    property int treeItemId: -2
    property int milestone: -2

    //---------------------------------------------------------
    Component.onCompleted: {

        if (outlineWritingZone.textArea.length === 0) {
            addOutlineToolButton.visible = true
            outlineWritingZone.Layout.preferredHeight = 0
        }
    }

    //---------------------------------------------------------
    Action {
        id: openOutlineAction
        text: qsTr("Open outline")
        icon.source: "qrc:///icons/backup/quickopen-file.svg"
        onTriggered: {
            saveContent()
            saveCurrentCursorPositionAndY()
            rootWindow.viewManager.insertAdditionalProperty("isSecondary", true)
            rootWindow.viewManager.loadTreeItemAtAnotherView(projectId,
                                                             treeItemId)
        }
    }
    openOutlineToolButton.action: openOutlineAction

    //---------------------------------------------------------
    Action {
        id: addOutlineAction
        text: qsTr("Add outline")
        icon.source: "qrc:///icons/backup/list-add.svg"
        onTriggered: {
            outlineWritingZone.visible = true
            outlineWritingZone.forceActiveFocus()
            outlineWritingZone.Layout.preferredHeight = 400
        }
    }
    addOutlineToolButton.action: addOutlineAction

    //---------------------------------------------------------

    outlineWritingZone.projectId: projectId
    outlineWritingZone.treeItemId: treeItemId
    outlineWritingZone.milestone: milestone
}
