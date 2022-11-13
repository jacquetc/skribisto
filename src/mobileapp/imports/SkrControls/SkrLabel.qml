import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import theme 1.0
import Skribisto

Label {
    id: control
    activeFocusOnTab: true
    maximumLineCount: 1

    Material.foreground: SkrTheme.buttonForeground

    SkrFocusIndicator {
        anchors.fill: parent
        visible: control.activeFocus & Globals.focusVisible
    }
    Keys.onPressed: function(event) {
        if (event.key === Qt.Key_Tab) {
            Globals.setFocusTemporarilyVisible()
        }
        if (event.key === Qt.Key_Backtab) {
            Globals.setFocusTemporarilyVisible()
        }
    }

    elide: Qt.ElideRight

    font.pointSize: Application.font.pointSize * SkrSettings.interfaceSettings.zoom
}
