import QtQuick 2.15
import eu.skribisto.projecthub 1.0
import ".."

WelcomePageForm {
    property string pageType: "welcome"

    //    Connections {
    //        target: plmData.projectHub()
    //        onProjectLoaded: console.log("loaded !!")
    //    }

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



    Connections {
        target: Globals
        function onShowProjectPage() {
            stackLayout.currentIndex = 0
        }
    }





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
