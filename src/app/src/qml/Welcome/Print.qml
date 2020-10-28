import QtQuick 2.4

PrintForm {


    goBackToolButton.icon.name: "go-previous"
    signal goBackButtonClicked()
    goBackToolButton.onClicked: goBackButtonClicked()

}
