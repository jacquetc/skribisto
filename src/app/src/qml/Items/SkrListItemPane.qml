import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

Pane {
    property int elevation: 0
    property int borderWidth: 0
    property string borderColor: "transparent"

    Material.elevation: elevation
    padding: 0

    contentItem: Rectangle{
        color: SkrTheme.listItemBackground
        border.width: borderWidth
        border.color: borderColor
    }

}
