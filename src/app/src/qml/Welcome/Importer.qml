import QtQuick 2.4

ImporterForm {


    goBackToolButton.icon.source: "qrc:///icons/backup/go-previous.svg"
    signal goBackButtonClicked()
    goBackToolButton.onClicked: goBackButtonClicked()

    importFromPlumeToolButton.icon.source: "qrc:/pics/skribisto.svg"
    importFromPlumeToolButton.icon.color: "transparent"
    importFromPlumeToolButton.onClicked: {
        var item = stackView.push("PlumeImporter.qml")

        item.onGoBackButtonClicked.connect(goBackToImporterMainPage)
    }

    function goBackToImporterMainPage(){
        stackView.pop()
    }

    onActiveFocusChanged: {
        if (activeFocus) {
            importFromPlumeToolButton.forceActiveFocus()
        }
    }
}
