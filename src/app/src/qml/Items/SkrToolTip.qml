import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import ".."

ToolTip {
    id: control

    delay: 1000
    timeout: 5000


    contentItem: Label {
        text: control.text
        font: control.font
        color: SkrTheme.buttonForeground
    }

    background: Rectangle {
        border.color: SkrTheme.accent
        color: SkrTheme.buttonBackground
    }

}
