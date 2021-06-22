import QtQuick 2.15
import QtQuick.Controls 2.15
import eu.skribisto.projecthub 1.0
import ".."

WelcomePageForm {
    id: welcomePage
    clip: true

    signal closeCalled()


    //--------------------------------------------------------
    //--------------------------------------------
    //--------------------------------------------------------

    // pages :
    goBackButton.onClicked: {
        closeCalled()
    }

    newButton.onClicked: {
        priv.mainButtonsPaneIsVisible  = false
        var item = stackView.push("NewProjectPage.qml", StackView.Immediate)
        item.closeCalled.connect(welcomePage.closeCalled)
        item.forceActiveFocus()

    }
    openButton.action: openProjectAction
    saveButton.action: saveAction
    saveAsButton.action: saveAsAction
    saveACopyButton.action: saveACopyAction
    backUpButton.action: backUpAction
    settingsButton.action: showSettingsAction

    recentButton.onClicked: {
        priv.mainButtonsPaneIsVisible  = false
        var item = stackView.push("RecentPage.qml")
        item.closeCalled.connect(welcomePage.closeCalled)
        item.forceActiveFocus()

    }

    exampleButton.onClicked: {
        priv.mainButtonsPaneIsVisible  = false
        var item = stackView.push("ExamplePage.qml")
        item.closeCalled.connect(welcomePage.closeCalled)
        item.forceActiveFocus()

    }

    importButton.onClicked: {
        priv.mainButtonsPaneIsVisible  = false
        var item = stackView.push("ImporterPage.qml")
        item.closeCalled.connect(welcomePage.closeCalled)
        item.forceActiveFocus()

    }

    exportButton.onClicked: {
        priv.mainButtonsPaneIsVisible  = false
        var item = stackView.push("ExporterPage.qml")
        item.forceActiveFocus()

    }

    printButton.onClicked: {
        priv.mainButtonsPaneIsVisible  = false
        var item = stackView.push("ExporterPage.qml", {"printEnabled": true})
        item.forceActiveFocus()
    }
    settingsButton.onClicked: {
        priv.mainButtonsPaneIsVisible  = false
        var item = stackView.push("SettingsPage.qml")
        item.closeCalled.connect(welcomePage.closeCalled)
        item.forceActiveFocus()

    }
    helpButton.onClicked: {
        priv.mainButtonsPaneIsVisible  = false
        var item = stackView.push("HelpPage.qml")
        item.forceActiveFocus()

    }

    //--------------------------------------------------------------

    //compact mode :
    mainButtonsPane.visible: !rootWindow.compactMode || priv.mainButtonsPaneIsVisible
    goBackToMenuButton.visible: rootWindow.compactMode && !priv.mainButtonsPaneIsVisible
    separator.visible: !rootWindow.compactMode

    goBackToMenuButton.onClicked: {
        stackView.pop()
        priv.mainButtonsPaneIsVisible = true
    }

    Connections {
        target: rootWindow.protectedSignals
        function onShowHelpPageCalled() {
            stackView.pop(StackView.Immediate)
            var item = stackView.push("HelpPage.qml", StackView.Immediate)
            item.forceActiveFocus()
        }
    }


    QtObject{
        id: priv
        property bool mainButtonsPaneIsVisible: true

    }


    //--------------------------------------------------------------

    Connections {
        target: rootWindow.protectedSignals
        function onShowNewProjectPageCalled() {
            stackView.pop(StackView.Immediate)
            var item = stackView.push("NewProjectPage.qml", StackView.Immediate)
            item.closeCalled.connect(welcomePage.closeCalled)
            item.forceActiveFocus()

        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onShowRecentPageCalled() {
            stackView.pop(StackView.Immediate)
            var item = stackView.push("RecentPage.qml", StackView.Immediate)
            item.closeCalled.connect(welcomePage.closeCalled)
            item.forceActiveFocus()
        }
    }


    Connections {
        target: rootWindow.protectedSignals
        function onShowExamplePageCalled() {
            stackView.pop(StackView.Immediate)
            var item = stackView.push("ExamplePage.qml", StackView.Immediate)
            item.closeCalled.connect(welcomePage.closeCalled)
            item.forceActiveFocus()
        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onShowImportPageCalled() {
            stackView.pop(StackView.Immediate)
            var item = stackView.push("ImporterPage.qml", StackView.Immediate)
            item.closeCalled.connect(welcomePage.closeCalled)
            item.forceActiveFocus()
        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onShowExportPageCalled() {
            stackView.pop(StackView.Immediate)
            var item = stackView.push("ExporterPage.qml", StackView.Immediate)
            item.forceActiveFocus()
        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onShowPrintPageCalled() {
            stackView.pop(StackView.Immediate)
            var item = stackView.push("ExporterPage.qml", {"printEnabled": true}, StackView.Immediate)
            item.forceActiveFocus()
        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onShowSettingsPageCalled() {
            stackView.pop(StackView.Immediate)
            var item = stackView.push("SettingsPage.qml", StackView.Immediate)
            item.closeCalled.connect(welcomePage.closeCalled)
            item.forceActiveFocus()
        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onShowHelpPageCalled() {
            stackView.pop(StackView.Immediate)
            var item = stackView.push("HelpPage.qml", StackView.Immediate)
            item.forceActiveFocus()
        }
    }






    function init() {

        // rootWindow.rootPage.showWelcomeAction.trigger()

    }
    onVisibleChanged: {
        if (visible) {
            newButton.forceActiveFocus()

            if(rootWindow.compactMode){
                stackView.pop()
            }
            else {
                var item = stackView.push("RecentPage.qml", StackView.Immediate)
                item.closeCalled.connect(welcomePage.closeCalled)
            }

        }

    }

    versionLabel.text: qsTr("Skribisto %1 created by Cyril Jacquet").arg(skrRootItem.skribistoVersion())



    onActiveFocusChanged: {
        if (activeFocus) {
            recentButton.forceActiveFocus()
        }
    }


    Component.onCompleted: {

        init()
    }



//    TapHandler{
//        acceptedButtons: Qt.RightButton | Qt.MiddleButton | Qt.LeftButton

//        //grabPermissions: PointerHandler.CanTakeOverFromAnything | PointerHandler.TakeOverForbidden
//        gesturePolicy: TapHandler.WithinBounds
//    }


}
