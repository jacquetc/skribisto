import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml 2.15
import ".."

ProjectsMainPageForm {
    property string pageType: "projects"

    clip: true

    tabBar.currentIndex: projectSwipeView.currentIndex

    //---------------------------------------------------------

    Binding {
        //when: rootTabBar.currentIndex !== rootSwipeView.currentIndex
        when: projectSwipeView.currentIndexChanged
        target: projectSwipeView
        property: "currentIndex"
        value: tabBar.currentIndex
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }
    //---------------------------------------------------------

    projectSwipeView.onCurrentItemChanged: {
        var i;
        for(i = 0; i < projectSwipeView.count; i++ ){

            var item = projectSwipeView.itemAt(i)
            if(item === projectSwipeView.currentItem){
                item.enabled = true
            }
            else{
                item.enabled = false
            }
        }
    }

    //---------------------------------------------------------

    Connections {
        target: plmData.projectHub()
        function onProjectLoaded(projectId){



            var comp = Qt.createComponent("ProjectSubPage.qml");
            var incubator = comp.incubateObject(projectSwipeView, {projectId: projectId});
            console.debug("debug : ", comp.errorString())
            if (incubator.status !== Component.Ready) {
                incubator.onStatusChanged = function(status) {
                    if (status === Component.Ready) {


                        var tabComp = Qt.createComponent("ProjectTab.qml");
                        if (tabComp.status === Component.Ready)
                            var tab = tabComp.createObject(tabBar, {projectId: projectId});


                    }
                }
            } else {

                var tabComp = Qt.createComponent("ProjectTab.qml");
                if (tabComp.status === Component.Ready)
                    var tab = tabComp.createObject(tabBar, {projectId: projectId});


            }


        }
    }


    Connections {
        target: plmData.projectHub()
        function onProjectClosed(projectId){

            var i;
            for(i = 0; i < tabBar.count; i++){

                if(tabBar.itemAt(i).projectId === projectId){

                    projectSwipeView.removeItem(projectSwipeView.itemAt(i))
                    tabBar.removeItem(tabBar.itemAt(i))
                }
            }

        }
    }




}
