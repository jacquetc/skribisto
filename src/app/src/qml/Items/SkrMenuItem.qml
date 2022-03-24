import QtQuick.Controls
import QtQuick.Controls.Material
import ".."

MenuItem {
    id: control
    icon.color: control.action ? (control.action.icon.color === "transparent"?
                                      (enabled ? control.action.icon.color: SkrTheme.buttonIconDisabled) :
                                      (enabled ? SkrTheme.buttonIcon : SkrTheme.buttonIconDisabled)) :
                                        (enabled ? SkrTheme.buttonIcon : SkrTheme.buttonIconDisabled)

    //Material.background: SkrTheme.menuBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent

    implicitHeight: 30 * SkrSettings.interfaceSettings.zoom
    topPadding: 0
    bottomPadding: 0
}
