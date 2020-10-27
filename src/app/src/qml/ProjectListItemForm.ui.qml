import QtQuick 2.15
import QtQuick.Controls 2.15

Item {
    width: 400
    height: 400

    Flow {
        id: flow1
        x: 97
        y: 96
        anchors.rightMargin: 10
        anchors.leftMargin: 10
        anchors.bottomMargin: 10
        anchors.topMargin: 10
        spacing: 20
        anchors.fill: parent

        SkrLabel {
            id: projectNameLabel
            text: qsTr("Text")
        }
    }
}
