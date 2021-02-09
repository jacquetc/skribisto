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
        }
    }



        DropArea {
            id: dropArea
            anchors.fill: parent



            keys: ["application/skribisto-tree-item"]
            onEntered: {
                dropIndicator.border.color = SkrTheme.accent
            }
            onExited: {
                dropIndicator.border.color = "transparent"

            }

            onDropped: {
                if(drop.proposedAction === Qt.MoveAction){

                    viewManager.loadTreeItemAt(drag.source.projectId, drag.source.treeItemId, position)
                    dropIndicator.border.color = "transparent"

                }
            }




        }

        Rectangle {
            id: dropIndicator
            anchors.fill: parent
            enabled: border.color === SkrTheme.accent
            z: 1
            color: "transparent"
            border.color: "transparent"
            border.width: 4

    }


}
