import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

ToolButton {
    id: control
    icon.color: control.action ? (control.action.icon.color === "transparent"?
                                      (enabled ? control.action.icon.color: SkrTheme.buttonIconDisabled) :
                                      (enabled ? SkrTheme.buttonIcon : SkrTheme.buttonIconDisabled)) :
                                        (enabled ? SkrTheme.buttonIcon : SkrTheme.buttonIconDisabled)

    Material.background: SkrTheme.buttonBackground
    Material.foreground: control.activeFocus ?  SkrTheme.accent : SkrTheme.buttonForeground
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
    flat: true

    display: AbstractButton.IconOnly

    SkrToolTip {
        text: control.tip ? control.tip + priv.finalShortcutText : control.text + priv.finalShortcutText
        visible: control.hovered && text.length !== 0
    }

    Rectangle {
        id: checkedIndicator

        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right

        height: 3
        color: SkrTheme.accent
        visible: control.checked
    }

    QtObject{
        id: priv
        property string finalShortcutText: shortcutText ? " (" + shortcutText +")" : ""
    }

    property string shortcutText:  action ? (action.shortcutText ? action.shortcutText : "") : ""


}
