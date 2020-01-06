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
        var arg
        var arguments
        arguments = Qt.application.arguments
        for (arg in arguments) {
            if (arguments[arg] === "--testProject") {
                var error = plmData.projectHub().loadProject(
                            testProjectFileName)
                console.log("project loaded : " + error.success)
                console.log("projectFileName :", testProjectFileName, "\n")
            }
        }
        if (plmData.projectHub().getProjectCount() === 0) {
            plmData.projectHub().loadProject("")
        }

        //        if (!error.success) {
        //            messageDialog.title = qsTr("")
        //            messageDialog.text = qsTr("")
        //            messageDialog.informativeText = "inf"
        //            messageDialog.detailedText = "det"
        //            messageDialog.visible = true
        //        }
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
