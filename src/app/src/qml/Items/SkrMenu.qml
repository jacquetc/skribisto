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
        implicitWidth: 200
        implicitHeight: 40
    }
}
