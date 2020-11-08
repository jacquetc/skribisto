import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml 2.15
import "../Items"
import ".."


ProjectPageForm {
    id: root


    stackView.initialItem: Qt.createComponent("FileMenuPage.qml")




    Connections {
        target: Globals
        function onShowNewProjectWizardCalled() {
            stackView.pop()
            var item = stackView.push(Qt.createComponent("NewProject.qml"))
            item.goBackButtonClicked.connect(function () {stackView.pop()})
        }
    }


    Connections {
        target: Globals
        function onShowPrintWizardCalled() {
            stackView.pop()
            var item = stackView.push(Qt.createComponent("Print.qml"))
            item.goBackButtonClicked.connect(function () {stackView.pop()})
        }
    }


    Connections {
        target: Globals
        function onShowImportWizardCalled() {
            stackView.pop()
            var item = stackView.push(Qt.createComponent("Importer.qml"))
            item.goBackButtonClicked.connect(function () {stackView.pop()})
        }
    }
    Connections {
        target: Globals
        function onShowExportWizardCalled() {
            stackView.pop()
            var item = stackView.push(Qt.createComponent("Exporter.qml"))
            item.goBackButtonClicked.connect(function () {stackView.pop()})
        }
    }
}
