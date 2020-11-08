import QtQuick 2.4

PrintForm {


    goBackToolButton.icon.source: "qrc:///icons/backup/go-previous.svg"
    signal goBackButtonClicked()
    goBackToolButton.onClicked: goBackButtonClicked()

}
