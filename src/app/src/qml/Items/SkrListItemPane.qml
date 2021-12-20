import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import ".."

Item {

    Material.elevation: elevation
    id: control
    property int elevation: 0
    property int borderWidth: 0
    property int padding: 0
    property string borderColor: "transparent"
    property alias contentItem : mainRectangle.data

    Rectangle {
        id: mainRectangle
        anchors.fill: parent
        anchors.margins: padding
        color: SkrTheme.listItemBackground
        border.width: borderWidth
        border.color: borderColor

    }

//    DropShadow {
//        anchors.fill: parent
//        horizontalOffset: elevation
//        verticalOffset: 0
//        cached: true
//        radius: 8
//        samples: 17
//        color: "#33000000"
//        source: mainRectangle
//    }

//    DropShadow {
//        anchors.fill: parent
//        horizontalOffset: - elevation
//        verticalOffset: 0
//        cached: true
//        radius: 8
//        samples: 17
//        color: "#33000000"
//        source: mainRectangle
//    }

//    DropShadow {
//        anchors.fill: parent
//        horizontalOffset: elevation
//        verticalOffset: elevation
//        cached: true
//        radius: 8
//        samples: 17
//        color: "#33000000"
//        source: mainRectangle
//    }

//    DropShadow {
//        anchors.fill: parent
//        horizontalOffset: - elevation
//        verticalOffset: elevation
//        cached: true
//        radius: 8
//        samples: 17
//        color: "#33000000"
//        source: mainRectangle
//    }
}
