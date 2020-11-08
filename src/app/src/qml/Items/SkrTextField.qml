import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

TextField {
    id: control

    Material.background: SkrTheme.buttonBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent

    selectByMouse: true





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


}
