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


}
