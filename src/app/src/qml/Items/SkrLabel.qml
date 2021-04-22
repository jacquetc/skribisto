import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

Label {
    id:control
    Material.foreground: SkrTheme.buttonForeground

    SkrFocusIndicator {
        parent: control.background
        anchors.fill: control.background
        visible: control.activeFocus

    }

    elide: Qt.ElideRight
}
