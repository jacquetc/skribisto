import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import "../Items"
import ".."



Dialog {
    id: dialog

    anchors.centerIn: Overlay.overlay

    property alias text: contentLabel.text

    background: Rectangle {
        color: SkrTheme.menuBackground
    }

    Material.background: SkrTheme.menuBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent

    modal: true
    implicitWidth: Math.max(Math.max(contentLabel.implicitWidth, footer.implicitWidth), header.implicitWidth) + 20

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


    TapHandler {
        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus | PointerDevice.TouchScreen
        grabPermissions: PointerHandler.CanTakeOverFromAnything

        onGrabChanged: function(transition, point) {
            point.accepted = true
        }
    }



}
