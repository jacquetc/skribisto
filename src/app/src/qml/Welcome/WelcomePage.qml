import QtQuick 2.15
import eu.skribisto.projecthub 1.0
import ".."

WelcomePageForm {
    property string pageType: "welcome"

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
    tabBar.visible: Globals.compactMode
    mainButtonsPane.visible: !Globals.compactMode
    separator.visible: !Globals.compactMode



    Connections {
        target: Globals
        function onShowWelcomeProjectPageCalled() {
            stackLayout.currentIndex = 0
            stackLayout.itemAt(0).forceActiveFocus()
        }
    }





    function init() {

        welcomeWindowAction.trigger()


    }


    onActiveFocusChanged: {
        if (activeFocus) {
            if(Globals.compactMode){
                projectPageTabButton.forceActiveFocus()
            }
            else{
                projectPageButton.forceActiveFocus()
            }
        }
    }

    Component.onCompleted: {

        init()
    }
}
