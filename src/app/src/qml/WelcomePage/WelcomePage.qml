import QtQuick 2.15
import eu.skribisto.projecthub 1.0
import ".."

WelcomePageForm {
    id: root
    clip: true

    signal closeCalled()


    //--------------------------------------------------------
    //--------------------------------------------
    //--------------------------------------------------------

    // pages :
    goBackButton.onClicked: {
        closeCalled()
    }
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

    goBackTabButton.onClicked: {
        closeCalled()

    }
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
    tabBar.visible: rootWindow.compactMode || width < 800
    mainButtonsPane.visible: !rootWindow.compactMode && width > 800
    separator.visible: !rootWindow.compactMode && width > 800



    Connections {
        target: rootWindow.protectedSignals
        function onShowProjectPageCalled() {
            stackLayout.currentIndex = 0
            stackLayout.itemAt(0).forceActiveFocus()
        }
    }


    Connections {
        target: rootWindow.protectedSignals
        function onShowExamplePageCalled() {
            stackLayout.currentIndex = 1
            stackLayout.itemAt(1).forceActiveFocus()
        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onShowSettingsPageCalled() {
            stackLayout.currentIndex = 2
            stackLayout.itemAt(2).forceActiveFocus()
        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onShowHelpPageCalled() {
            stackLayout.currentIndex = 3
            stackLayout.itemAt(3).forceActiveFocus()
        }
    }






    function init() {

       // rootWindow.rootPage.showWelcomeAction.trigger()


    }


    onActiveFocusChanged: {
        if (activeFocus) {
            if(rootWindow.compactMode){
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
