import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

Button {
    id: control
    icon.color: control.action === null ? (enabled ? SkrTheme.buttonIcon : SkrTheme.buttonIconDisabled) :
                                          (control.action.icon.color === "transparent" ?
                                               (enabled ? control.action.icon.color: SkrTheme.buttonIconDisabled) :
                                               (enabled ? SkrTheme.buttonIcon : SkrTheme.buttonIconDisabled))

    Material.background: SkrTheme.buttonBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent

    SkrFocusIndicator {
        parent: control.background
        anchors.fill: control.background
        visible: control.activeFocus

    }

    property string tip
    hoverEnabled: true

    SkrToolTip {
        text: control.tip ? control.tip : control.text
        visible: control.hovered && text.length !== 0
    }

}
