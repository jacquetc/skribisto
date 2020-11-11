import QtQuick 2.4

ExporterForm {



    goBackToolButton.icon.source: "qrc:///icons/backup/go-previous.svg"
    signal goBackButtonClicked()
    goBackToolButton.onClicked: goBackButtonClicked()

    onActiveFocusChanged: {
        if (activeFocus) {

        }
    }


}
