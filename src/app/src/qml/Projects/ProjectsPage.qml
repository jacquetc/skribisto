import QtQuick 2.12
import ".."

ProjectsPageForm {
    property string pageType: "projects"

    Connections {
        target: plmData.projectHub()
        // @disable-check M16
        onProjectCountChanged: function (count){
            if(count === 0){
                tabBar.visible = false
            }
            else {
                tabBar.visible = true
            }

        }
    }

    Connections {
        target: plmData.projectHub()
        // @disable-check M16
        onProjectOpened: function (){
            if(count === 0){
                tabBar.visible = false
            }
            else {
                tabBar.visible = true
            }

        }
    }
}
