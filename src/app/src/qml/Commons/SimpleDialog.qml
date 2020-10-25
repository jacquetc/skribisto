import QtQuick 2.15
import QtQuick.Controls 2.15
import "../Items"




Dialog {
    id: dialog

    anchors.centerIn: Overlay.overlay

    property alias dialogTitle: dialog.title
    property alias text: text.text


    modal: true


    SkrLabel {
        id: text
    }
    standardButtons: Dialog.Ok

}
