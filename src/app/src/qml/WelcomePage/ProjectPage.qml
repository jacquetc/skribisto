import QtQuick
import QtQuick.Controls
import QtQml
import "../Items"
import ".."


ProjectPageForm {
    id: root


    stackView.initialItem: Qt.createComponent("FileMenuPage.qml")




    Connections {
        target: rootWindow.protectedSignals
        function onShowNewProjectWizardCalled() {
            stackView.pop()
            var item = stackView.push(Qt.createComponent("NewProject.qml"))
            item.forceActiveFocus()
            item.goBackButtonClicked.connect(function () {stackView.pop()})
        }
    }


    Connections {
        target: rootWindow.protectedSignals
        function onShowPrintWizardCalled() {
            stackView.pop()
            var item = stackView.push(Qt.createComponent("Exporter.qml"), {"printEnabled": true})
            item.forceActiveFocus()
            item.goBackButtonClicked.connect(function () {stackView.pop()})
        }
    }


    Connections {
        target: rootWindow.protectedSignals
        function onShowImportWizardCalled() {
            stackView.pop()
            var item = stackView.push(Qt.createComponent("Importer.qml"))
            item.forceActiveFocus()
            item.goBackButtonClicked.connect(function () {stackView.pop()})
        }
    }
    Connections {
        target: rootWindow.protectedSignals
        function onShowExportWizardCalled() {
            stackView.pop()
            var item = stackView.push(Qt.createComponent("Exporter.qml"))
            item.forceActiveFocus()
            item.goBackButtonClicked.connect(function () {stackView.pop()})
        }
    }
}
