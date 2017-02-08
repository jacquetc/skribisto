import QtQuick 2.5
import QtQuick.Controls 1.4

WelcomePageForm {

    function init(){
        //leftBase.onBaseWidthChanged.connect(changeLeftBaseWidth)
        //rightBase.onBaseWidthChanged.connect(changeRightBaseWidth)
        projectsButton.checked = true
    }

    projectsButton.onClicked: {
        while (stackedView.depth > 1){
            stackedView.pop(stackedView.depth)
        }
        stackedView.get(0)
        //examplesButton.checked = false
        //projectsButton.checked = true
        //console.debug(stackedView.depth)
    }

    examplesButton.onClicked: {
        while (stackedView.depth > 1){
            stackedView.pop(stackedView.depth)
        }
        //projectsButton.checked = false
        stackedView.push("file:gui/qml/WelcomeExamples.qml")
        //examplesButton.checked = true

    }

    //    ListModel {
    //        ListElement { title: "Projects"; source: "file:gui/qml/WelcomeProjects.qml" }
    //        ListElement { title: "Button"; source: "file:gui/qml/WelcomeProjects.qml" }
    //    }

    stackedView.initialItem: "file:gui/qml/WelcomeProjects.qml"

    ExclusiveGroup {id: group}
    examplesButton.exclusiveGroup: group
    projectsButton.exclusiveGroup: group


    Component.onCompleted: init()
}
