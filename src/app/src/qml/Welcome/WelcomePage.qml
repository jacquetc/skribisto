import QtQuick 2.12
import eu.skribisto.projecthub 1.0
import ".."

WelcomePageForm {

    //    Connections {
    //        target: plmData.projectHub()
    //        onProjectLoaded: console.log("loaded !!")
    //    }
    property string testProjectFileName: ":/testfiles/skribisto_test_project.sqlite"

    // pages :
    projectPageButton.onClicked: stackLayout.currentIndex = 0
    examplePageButton.onClicked: stackLayout.currentIndex = 1
    settingsPageButton.onClicked: stackLayout.currentIndex = 2
    helpPageButton.onClicked: stackLayout.currentIndex = 3

    //pages with tabbar :
    projectPageTabButton.onClicked: stackLayout.currentIndex = 0
    examplePageTabButton.onClicked: stackLayout.currentIndex = 1
    settingsPageTabButton.onClicked: stackLayout.currentIndex = 2
    helpPageTabButton.onClicked: stackLayout.currentIndex = 3

    //compact mode :
    tabBar.visible: Globals.compactSize
    mainButtonsPane.visible: !Globals.compactSize
    separator.visible: !Globals.compactSize

    function init() {
        //leftBase.onBaseWidthChanged.connect(changeLeftBaseWidth)
        //rightBase.onBaseWidthChanged.connect(changeRightBaseWidth)

        //show Welcome window


        //        if (!error.success) {
        //            messageDialog.title = qsTr("")
        //            messageDialog.text = qsTr("")
        //            messageDialog.informativeText = "inf"
        //            messageDialog.detailedText = "det"
        //            messageDialog.visible = true
        //        }

        welcomeWindowAction.trigger()
        this.openArgument()

    }

    function openArgument(){


        var arg
        var arguments
        var isTestProject = false
        var oneProjectInArgument = false
        var projectInArgument = ""

        arguments = Qt.application.arguments
        for (arg in arguments) {
            console.log("argument : " , arguments[arg])
            if(arg === 0 ){
                continue
            }

            if (arguments[arg] === "--testProject") {
                var error = plmData.projectHub().loadProject(
                            testProjectFileName)
                console.log("project loaded : " + error.success)
                console.log("projectFileName :", testProjectFileName, "\n")

                //show Write window
                writeWindowAction.trigger()
                isTestProject = true

            }
            else {
                if (arguments[arg][-6] === ".skrib"){
                    oneProjectInArgument = true
                    projectInArgument = plmData.projectHub().loadProject(
                                arguments[arg])

                }
            }
        }
        if(!isTestProject & oneProjectInArgument){
            var error = plmData.projectHub().loadProject(
                        projectInArgument)
            //show Write window
            writeWindowAction.trigger()
        }


        if (!isTestProject & !oneProjectInArgument & plmData.projectHub().getProjectCount() === 0 & SkrSettings.welcomeSettings.createEmptyProjectAtStart === true) {
            plmData.projectHub().loadProject("")

            //show Write window
            writeWindowAction.trigger()

        }
    }

    onActiveFocusChanged: {
        if (activeFocus) {
            projectPageButton.forceActiveFocus()
        }
    }

    Component.onCompleted: {

        init()
    }
}
