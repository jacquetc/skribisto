import QtQuick
import QtQml
import ".."

HelpPageForm {



    stackView.initialItem: Qt.createComponent("HelpContents.qml")

    helpTabButton.onClicked: {showHelpContents()}
    Connections {
        target: rootWindow.protectedSignals
        function onShowHelpContentsCalled() {showHelpContents() }
    }
    function showHelpContents(){
        tabBar.currentIndex = 0
        stackView.pop()
    }

    aboutTabButton.onClicked: showAbout()
    Connections {
        target: rootWindow.protectedSignals
        function onShowAboutCalled() {showAbout()}
    }

    function showAbout(){
        stackView.pop()
        tabBar.currentIndex = 1
        var item = stackView.push(Qt.createComponent("About.qml"))
    }

    aboutQtTabButton.onClicked: showAboutQt()
    Connections {
        target: rootWindow.protectedSignals
        function onShowAboutQtCalled() {showAboutQt()}
    }

    function showAboutQt(){
        stackView.pop()
        tabBar.currentIndex = 2
        var item = stackView.push(Qt.createComponent("About.qml"), {"qt": true})
    }

    //--------------------------------------------------

    Connections{
        target: tabBar
        enabled: tabBar.activeFocus
        function onCurrentIndexChanged(){
            switch(tabBar.currentIndex){
            case 0:
                showHelpContents()
                break;
            case 1:
                showAbout()
                break;
            case 2:
                showAboutQt()
                break;

        }
        }
    }

    //--------------------------------------------------


    onActiveFocusChanged: {
        if (activeFocus) {
            helpTabButton.forceActiveFocus()
        }
    }


}
