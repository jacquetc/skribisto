import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import ".."

SpinBox {
    id: control

    Material.background: SkrTheme.buttonBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent


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


    property string tip
    hoverEnabled: true

    SkrToolTip {
        text: control.tip
        visible: control.hovered && tip.length !== 0
    }

    font.pointSize: Application.font.pointSize * SkrSettings.interfaceSettings.zoom

}
