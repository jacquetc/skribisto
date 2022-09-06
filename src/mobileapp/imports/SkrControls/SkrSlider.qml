import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import theme 1.0
import Skribisto

Slider {
    id: control

    Material.accent: SkrTheme.accent


    SkrFocusIndicator {
        parent: control
        anchors.fill: control
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

    snapMode: Slider.SnapOnRelease


    font.pointSize: Application.font.pointSize * SkrSettings.interfaceSettings.zoom

}
