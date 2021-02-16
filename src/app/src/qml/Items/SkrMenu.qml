import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
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
