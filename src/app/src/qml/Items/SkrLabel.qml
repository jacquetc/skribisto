import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

Label {
    id:control
    activeFocusOnTab: true

    Material.foreground: SkrTheme.buttonForeground

    SkrFocusIndicator {
        anchors.fill: parent
        visible: control.activeFocus & Globals.focusVisible

    }
    Keys.onPressed: {
        if (event.key === Qt.Key_Tab){
            Globals.setFocusTemporarilyVisible()
        }
    }

    elide: Qt.ElideRight
}
