import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

Dial {
    id: control

    Material.background: SkrTheme.buttonBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent




    property string tip
    hoverEnabled: true

    SkrToolTip {
        text: control.tip
        visible: control.hovered
    }
}
