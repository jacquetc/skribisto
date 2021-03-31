import QtQuick 2.15
import QtQuick.Controls 2.15

ImporterPageForm {


    importFromPlumeToolButton.icon.source: "qrc:/pics/skribisto.svg"
    importFromPlumeToolButton.icon.color: "transparent"
    importFromPlumeToolButton.onClicked: {
        var item = stackView.push("PlumeImporter.qml", StackView.Immediate)
        item.forceActiveFocus()

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
