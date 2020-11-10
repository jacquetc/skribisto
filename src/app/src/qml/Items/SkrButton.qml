import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

Button {
    id: control
    icon.color: control.action === null ? SkrTheme.buttonIcon : control.action.icon.color
    Material.background: SkrTheme.buttonBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent



    property string tip
    hoverEnabled: true

    SkrToolTip {
        text: control.tip ? control.tip : control.text
        visible: control.hovered && text.length !== 0
    }

}
