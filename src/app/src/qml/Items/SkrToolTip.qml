import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
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
