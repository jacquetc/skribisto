import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.15
import ".."


Item {
    id: handle
    implicitHeight:  orientation ===  Qt.Horizontal ? 4 : 0
    implicitWidth: orientation ===  Qt.Horizontal ? 0 : 4
    readonly property bool hovered: SplitHandle.hovered
    property var orientation: Qt.Horizontal

        Rectangle {
            visible: orientation === Qt.Horizontal

            color: hovered ? SkrTheme.buttonBackground : SkrTheme.divider
            height: 1
            anchors.bottom: parent.bottom
            anchors.left: parent.left
            anchors.right: parent.right


                Rectangle {
                    width: 20
                    height: 6
                    anchors.centerIn: parent
                    color: hovered ? SkrTheme.buttonBackground : "transparent"
                }


        }
        Rectangle {
            visible: orientation === Qt.Vertical

            color: hovered ? SkrTheme.buttonBackground : SkrTheme.divider
            width: 1
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            anchors.left: parent.left


                Rectangle {
                    width: 6
                    height: 20
                    anchors.centerIn: parent
                    color: hovered ? SkrTheme.buttonBackground : "transparent"
                }


        }

}
