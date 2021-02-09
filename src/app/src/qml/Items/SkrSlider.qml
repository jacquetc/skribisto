import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

Slider {
    id: control

    Material.accent: SkrTheme.accent


    Rectangle {
        parent: control.background
        anchors.fill: control.background
        color: "transparent"
        border.color: SkrTheme.accent
        border.width: control.activeFocus ? 1 : 0
        radius: 4

    }

    property string tip
    hoverEnabled: true

    SkrToolTip {
        text: control.tip
        visible: control.hovered && tip.length !== 0
    }

    snapMode: Slider.SnapOnRelease



}
