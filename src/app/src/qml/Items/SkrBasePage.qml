import QtQuick 2.15
import QtQuick.Controls 2.15
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

    property list<Component> toolBoxes

    signal closeViewCalled()

    function closeView(){
        closeViewCalled()
        viewManager.closeView(position)
    }


    onActiveFocusChanged: {
        if(activeFocus){
            viewManager.focusedPosition = position
            if(projectId !== -1 & treeItemId !== -1){
                rootWindow.protectedSignals.setBreadcrumbCurrentTreeItemCalled(projectId, treeItemId)
                rootWindow.setNavigationTreeItemIdCalled(projectId, treeItemId)
            }
        }
    }



    DropArea {
        id: dropArea
        anchors.fill: parent



        keys: ["application/skribisto-tree-item"]
        onEntered: {
            dropIndicator.visible = true
        }
        onExited: {
            dropIndicator.visible = false

        }

        onDropped: {
            if(drop.proposedAction === Qt.MoveAction){
                viewManager.loadTreeItemAt(drag.source.projectId, drag.source.treeItemId, position)

            }
            dropIndicator.visible = false
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

    Keys.onPressed: {
        if (event.key === Qt.Key_F2){
            if(control.projectId !== -2){
            renameDialog.projectId = control.projectId
            renameDialog.treeItemId = control.treeItemId
            renameDialog.treeItemTitle = plmData.treeHub().getTitle(control.projectId, control.treeItemId)
            renameDialog.open()
            }
        }
    }


    SimpleDialog {
        id: renameDialog
        property int projectId: -2
        property int treeItemId: -2
        property string treeItemTitle: ""
        title: "Rename an  item"
        //text: qsTr("The project %1 is not saved. Do you want to save it before quiting ?").arg(projectName)
        contentItem: SkrTextField {
            id: renameTextField
                text: renameDialog.treeItemTitle

                onAccepted: {
                    renameDialog.accept()
                }

        }

        standardButtons: Dialog.Ok  | Dialog.Cancel

        onRejected: {
            renameDialog.treeItemTitle = ""

        }

        onDiscarded: {


            renameDialog.treeItemTitle = ""

        }

        onAccepted: {
            plmData.treeHub().setTitle(renameDialog.projectId, renameDialog.treeItemId, renameTextField.text)

            renameDialog.treeItemTitle = ""
        }

        onActiveFocusChanged: {
            if(activeFocus){
                contentItem.forceActiveFocus()
            }

        }

        onOpened: {
                        contentItem.forceActiveFocus()
                        renameTextField.selectAll()
        }

    }
}
