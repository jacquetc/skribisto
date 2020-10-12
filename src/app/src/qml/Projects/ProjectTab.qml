import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ".."

ProjectTabForm {
    id: root
    property int projectId : -2



    Accessible.name: root.text


    signal onCloseCalled(int index)
    closeButton.onClicked:  Globals.closeProjectCalled(projectId)


    readonly property bool isCurrent:  {
        if (TabBar.tabBar !== null) {
            return TabBar.index === TabBar.tabBar.currentIndex
        }
        return false
    }

    Component.onCompleted:  {

        root.text = plmData.projectHub().getProjectName(projectId)
    }

    Connections {
        target: plmData.projectHub()
        function onProjectNameChanged(projectId, projectName){

            if(projectId === root.projectId){
                root.text = projectName
            }

        }
    }

}
