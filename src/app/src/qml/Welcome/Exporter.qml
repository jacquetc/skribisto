import QtQuick 2.4

ExporterForm {



    goBackToolButton.icon.name: "go-previous"
    signal goBackButtonClicked()
    goBackToolButton.onClicked: goBackButtonClicked()
}
