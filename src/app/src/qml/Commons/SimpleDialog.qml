import QtQuick 2.12
import QtQuick.Controls 2.12




Dialog {
    id: dialog

    anchors.centerIn: Overlay.overlay

    property alias dialogTitle: dialog.title
    property alias text: text.text


    modal: true


    Text {
        id: text
    }
    standardButtons: Dialog.Ok

}
