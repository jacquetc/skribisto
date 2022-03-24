import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import ".."

Menu {
    //Material.background: SkrTheme.menuBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent



    background: Rectangle {
        color: SkrTheme.menuBackground
        border.color: SkrTheme.buttonForeground
        implicitWidth: 250 * SkrSettings.interfaceSettings.zoom
        implicitHeight: 30 * SkrSettings.interfaceSettings.zoom
    }
    topPadding: 2
    bottomPadding: 2

    font.pointSize: Application.font.pointSize * SkrSettings.interfaceSettings.zoom
}
