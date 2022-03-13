import QtQuick.Controls
import QtQuick.Controls.Material
import ".."

Pane {
    property int elevation: 0

    Material.background: SkrTheme.pageBackground
    Material.elevation: elevation
}
