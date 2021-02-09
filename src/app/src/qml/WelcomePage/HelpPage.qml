import QtQuick 2.15
import QtQml 2.15
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


    faqTabButton.onClicked: showFaq()
    Connections {
        target: rootWindow.protectedSignals
        function onShowFaqCalled() {showFaq() }
    }

    function showFaq(){
        stackView.pop()
        tabBar.currentIndex = 1
        var item = stackView.push(Qt.createComponent("FAQ.qml"))
    }

    aboutTabButton.onClicked: showAbout()
    Connections {
        target: rootWindow.protectedSignals
        function onShowAboutCalled() {showAbout()}
    }

    function showAbout(){
        stackView.pop()
        tabBar.currentIndex = 2
        var item = stackView.push(Qt.createComponent("About.qml"))
    }

    aboutQtTabButton.onClicked: showAboutQt()
    Connections {
        target: rootWindow.protectedSignals
        function onShowAboutQtCalled() {showAboutQt()}
    }

    function showAboutQt(){
        stackView.pop()
        tabBar.currentIndex = 3
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
                showFaq()
                break;
            case 2:
                showAbout()
                break;
            case 3:
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
