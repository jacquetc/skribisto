import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

Slider {
    id: control

    Material.accent: SkrTheme.accent


    SkrFocusIndicator {
        parent: control.background
        anchors.fill: control.background
        visible: control.activeFocus

    }


    property string tip
    hoverEnabled: true

    SkrToolTip {
        text: control.tip
        visible: control.hovered && tip.length !== 0
    }

    snapMode: Slider.SnapOnRelease



}
