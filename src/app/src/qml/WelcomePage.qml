import QtQuick 2.9
import eu.skribisto.projecthub 1.0
import Qt.labs.settings 1.1

WelcomePageForm {

//    Connections {
//        target: plmData.projectHub()
//        onProjectLoaded: console.log("loaded !!")
//    }


    function init(){
        //leftBase.onBaseWidthChanged.connect(changeLeftBaseWidth)
        //rightBase.onBaseWidthChanged.connect(changeRightBaseWidth)
        var error = plmData.projectHub().loadProject("c:/users/jacqu/Devel/skribisto/resources/test/skribisto_test_project.sqlite");
        console.log("project loaded : " + error.success);
//        if (!error.success) {
//            messageDialog.title = qsTr("")
//            messageDialog.text = qsTr("")
//            messageDialog.informativeText = "inf"
//            messageDialog.detailedText = "det"
//            messageDialog.visible = true
//        }

    }

    Settings{
        id: settings
        property bool menuButtonsInStatusBar: false
    }

    testSwitch.onCheckedChanged: {
        testSwitch.checked ? settings.menuButtonsInStatusBar = true :  settings.menuButtonsInStatusBar = false
        Globals.loadAllSettings()
    }

    Component.onCompleted: {

        init()


    }





}
