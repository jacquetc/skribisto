import QtQuick 2.0
import QtQuick.Controls 1.2
import QtQuick.Layouts 1.0
import QtQuick.Window 2.1

Rectangle {
    id: sideBar
    width: 70
    height: 500
    color: "#797979"
    radius: 0
    border.width: 1
    rotation: 0
    clip: false
    visible: true

    ToolButton {
        id: toolButton1
        height: 70
        text: qsTr("Write")
        isDefault: false
        checked: false
        anchors.right: parent.right
        anchors.rightMargin: 0
        anchors.left: parent.left
        anchors.leftMargin: 0
        z: 2
        checkable: true
        iconSource: "qrc:///pics/48x48/scribus.png"
        iconName: qsTr("Write")
    }
}
