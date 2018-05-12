import QtQuick 2.9
import QtQuick.Dialogs 1.2
import eu.plumecreator.qml 1.0

WelcomePageForm {

    Connections {
        target: plmData.projectHub()
        onProjectLoaded: console.log("loaded !!")
    }


    function init(){
        //leftBase.onBaseWidthChanged.connect(changeLeftBaseWidth)
        //rightBase.onBaseWidthChanged.connect(changeRightBaseWidth)
        var error = plmData.projectHub().loadProject("/home/cyril/Devel/plume/plume-creator/resources/test/plume_test_project.sqlite");
        console.log(error.success);
//        if (!error.success) {
//            messageDialog.title = qsTr("")
//            messageDialog.text = qsTr("")
//            messageDialog.informativeText = "inf"
//            messageDialog.detailedText = "det"
//            messageDialog.visible = true
//        }

    }

    Component.onCompleted: init()


    MessageDialog {
        id: messageDialog
        title: ""
        onAccepted: {
            visible = false
        }
    }


}
