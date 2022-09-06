import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import theme 1.0
import Skribisto

ToolTip {
    id: control

    delay: 1000
    timeout: 5000

    font.pointSize: Application.font.pointSize * SkrSettings.interfaceSettings.zoom

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
