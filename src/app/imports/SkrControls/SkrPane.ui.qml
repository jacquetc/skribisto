import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import theme 1.0

Pane {
    property int elevation: 0

    Material.background: SkrTheme.pageBackground
    Material.elevation: elevation
}
