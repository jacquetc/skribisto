import QtQuick 2.9
import eu.skribisto.projecthub 1.0
import ".."

WelcomePageForm {

//    Connections {
//        target: plmData.projectHub()
//        onProjectLoaded: console.log("loaded !!")
//    }
    property string testProjectFileName: ":/testfiles/skribisto_test_project.sqlite"


    function init(){
        //leftBase.onBaseWidthChanged.connect(changeLeftBaseWidth)
        //rightBase.onBaseWidthChanged.connect(changeRightBaseWidth)
var arg;
        var arguments;
        arguments = Qt.application.arguments;
        for(arg in arguments){
            if(arguments[arg] === "--testProject"){
        var error = plmData.projectHub().loadProject(testProjectFileName);
        console.log("project loaded : " + error.success);
        console.log("projectFileName :", testProjectFileName, "\n");
            }
        }
//        if (!error.success) {
//            messageDialog.title = qsTr("")
//            messageDialog.text = qsTr("")
//            messageDialog.informativeText = "inf"
//            messageDialog.detailedText = "det"
//            messageDialog.visible = true
//        }



    }

    testSwitch.checked:  SkrSettings.interfaceSettings.menuButtonsInStatusBar
    Binding{
        target: SkrSettings.interfaceSettings
        property: "menuButtonsInStatusBar"
        value: testSwitch.checked
    }


    Component.onCompleted: {



        init()



    }





}
