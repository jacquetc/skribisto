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
    projectPageButton.onClicked: {
        stackLayout.currentIndex = 0
        stackLayout.itemAt(0).forceActiveFocus()

    }
    examplePageButton.onClicked: {
        stackLayout.currentIndex = 1
        stackLayout.itemAt(1).forceActiveFocus()

    }
    settingsPageButton.onClicked: {
        stackLayout.currentIndex = 2
        stackLayout.itemAt(2).forceActiveFocus()

    }
    helpPageButton.onClicked: {
        stackLayout.currentIndex = 3
        stackLayout.itemAt(3).forceActiveFocus()

    }

    //pages with tabbar :
    projectPageTabButton.onClicked: {
        stackLayout.currentIndex = 0
        stackLayout.itemAt(0).forceActiveFocus()

    }
    examplePageTabButton.onClicked: {
        stackLayout.currentIndex = 1
        stackLayout.itemAt(1).forceActiveFocus()

    }
    settingsPageTabButton.onClicked: {
        stackLayout.currentIndex = 2
        stackLayout.itemAt(2).forceActiveFocus()

    }
    helpPageTabButton.onClicked: {
        stackLayout.currentIndex = 3
        stackLayout.itemAt(3).forceActiveFocus()

    }
    //compact mode :
    tabBar.visible: Globals.compactSize
    mainButtonsPane.visible: !Globals.compactSize
    separator.visible: !Globals.compactSize



    Connections {
        target: Globals
        function onShowWelcomeProjectPageCalled() {
            stackLayout.currentIndex = 0
            stackLayout.itemAt(0).forceActiveFocus()
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
