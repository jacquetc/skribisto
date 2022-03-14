import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import ".."

TextField {
    id: control

    Material.background: SkrTheme.buttonBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent

    selectByMouse: true

    SkrFocusIndicator {
        parent: control.background
        anchors.fill: control.background
        visible: control.activeFocus & Globals.focusVisible

    }
    Keys.onPressed: function(event) {
        if (event.key === Qt.Key_Tab){
            Globals.setFocusTemporarilyVisible()
        }
        if (event.key === Qt.Key_Backtab) {
            Globals.setFocusTemporarilyVisible()
        }
    }



    rightPadding: clearButton.width

    SkrToolButton {
        id: clearButton
        icon.source: "qrc:///icons/backup/edit-clear.svg"
        focusPolicy: Qt.NoFocus

        anchors.right: parent.right
        anchors.top: parent.top
        anchors.bottom: parent.bottom

        width: 30
        visible: control.text

        onClicked: {
            control.text = ""
            control.forceActiveFocus()
        }

    }
    font.pointSize: Application.font.pointSize * SkrSettings.interfaceSettings.zoom


}
