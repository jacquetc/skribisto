import QtQuick 2.15
import QtQuick.Controls 2.15
import "../Items"




Dialog {
    id: dialog

    anchors.centerIn: Overlay.overlay

    property alias text: contentLabel.text


    modal: true
    implicitWidth: contentLabel.implicitWidth

    header: SkrLabel {
        id: headerLabel
        visible: true
        text: dialog.title
        font.bold: true
        font.pointSize: Qt.application.font.pointSize * 1.2
        horizontalAlignment: Qt.AlignHCenter
        verticalAlignment: Qt.AlignVCenter
    }

    contentItem: SkrLabel {
        id: contentLabel
        wrapMode: Text.WordWrap
    }
    standardButtons: Dialog.Ok

}
