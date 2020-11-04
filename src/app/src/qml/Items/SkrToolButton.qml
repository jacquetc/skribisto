import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

ToolButton {
    id: control
    icon.color: SkrTheme.buttonIcon
    Material.background: SkrTheme.buttonBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent


    property string tip
    hoverEnabled: true

    SkrToolTip {
        text: control.tip ? control.tip : control.text
        visible: control.hovered
    }
}
