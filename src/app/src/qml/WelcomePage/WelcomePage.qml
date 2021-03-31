import QtQuick 2.15
import QtQuick.Controls 2.15
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
    newButton.onClicked: {
        var item = stackView.push("NewProjectPage.qml", StackView.Immediate)
        item.forceActiveFocus()

    }
    openButton.onClicked: {
        openProjectAction.triggered()
    }

    recentButton.onClicked: {
        var item = stackView.push("RecentPage.qml")
        item.forceActiveFocus()

    }

    exampleButton.onClicked: {
        var item = stackView.push("ExamplePage.qml")
        item.forceActiveFocus()

    }

    importButton.onClicked: {
        var item = stackView.push("ImporterPage.qml", StackView.Immediate)
        item.forceActiveFocus()

    }

    exportButton.onClicked: {
        var item = stackView.push("ExporterPage.qml", StackView.Immediate)
        item.forceActiveFocus()

    }

    printButton.onClicked: {
        var item = stackView.push("ExporterPage.qml", {"printEnabled": true}, StackView.Immediate)
        item.forceActiveFocus()
    }
    settingsButton.onClicked: {
        var item = stackView.push("SettingsPage.qml", StackView.Immediate)
        item.forceActiveFocus()

    }
    helpButton.onClicked: {
        var item = stackView.push("HelpPage.qml", StackView.Immediate)
        item.forceActiveFocus()

    }

    //compact mode :
    mainButtonsPane.visible: !rootWindow.compactMode && width > 800
    separator.visible: !rootWindow.compactMode && width > 800



    Connections {
        target: rootWindow.protectedSignals
        function onShowNewProjectPageCalled() {
            stackView.pop(StackView.Immediate)
            var item = stackView.push("NewProjectPage.qml", StackView.Immediate)
            item.forceActiveFocus()

        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onShowRecentPageCalled() {
            stackView.pop(StackView.Immediate)
            var item = stackView.push("RecentPage.qml", StackView.Immediate)
            item.forceActiveFocus()
        }
    }


    Connections {
        target: rootWindow.protectedSignals
        function onShowExamplePageCalled() {
            stackView.pop(StackView.Immediate)
            var item = stackView.push("ExamplePage.qml", StackView.Immediate)
            item.forceActiveFocus()
        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onShowImportPageCalled() {
            stackView.pop(StackView.Immediate)
            var item = stackView.push("ImporterPage.qml", StackView.Immediate)
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
        var item = stackView.push("RecentPage.qml", StackView.Immediate)

    }
    onVisibleChanged: {
        if (visible) {
        newButton.forceActiveFocus()
        }
    }

    onActiveFocusChanged: {
        if (activeFocus) {
            newButton.forceActiveFocus()
        }
    }


    Component.onCompleted: {

        init()
    }






}
