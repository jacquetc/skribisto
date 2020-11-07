import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

Pane {
    property int elevation: 0

    Material.background: SkrTheme.pageBackground
    Material.elevation: elevation
}
