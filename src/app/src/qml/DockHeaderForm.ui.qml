import QtQuick 2.4
import QtQuick.Controls 2.3
import QtQuick.Layouts 1.3

Item {
    id: base
    width: 400
    height: 400
    property bool folded: true
    property alias title: dockTitle.text

    Rectangle {
        id: rectangle
        color: "#ffffff"
        anchors.fill: parent

        Text {
            id: dockTitle
            text: qsTr("Text")
            anchors.left: hSwitch.right
            anchors.leftMargin: -6
            anchors.right: parent.right
            anchors.rightMargin: 0
            anchors.bottom: parent.top
            anchors.bottomMargin: -15
            anchors.top: parent.top
            anchors.topMargin: 0
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
            font.pixelSize: 12
        }

        Switch {
            id: hSwitch
            width: 61
            height: 30
            scale: 0.5
            anchors.top: parent.top
            anchors.topMargin: -7
            anchors.left: parent.left
            anchors.leftMargin: -13
            //                implicitHeight: height * scale
            //                implicitWidth: width * scale
            padding: 1
            checked: !folded
            onCheckedChanged: checked ? folded = false : folded = true
        }

        Text {
            id: vDockTitle
            text: dockTitle.text
            anchors.right: hSwitch.left
            anchors.rightMargin: 0
            anchors.bottom: parent.left
            anchors.bottomMargin: -15
            anchors.top: parent.left
            anchors.topMargin: 0
            anchors.left: parent.bottom
            anchors.leftMargin: 0
            antialiasing: true
            transformOrigin: Item.Center
            rotation: 270
            font.pixelSize: 12
        }
    }
    states: [
        State {
            name: "unfolded"
            when: folded === false

            PropertyChanges {
                target: dockTitle
                visible: false
            }
            PropertyChanges {
                target: vDockTitle
                visible: true
            }

            PropertyChanges {
                target: hSwitch
                anchors.topMargin: 1
                anchors.leftMargin: -21
                rotation: 90
            }
        },
        State {
            name: "folded"
            when: folded === true

            PropertyChanges {
                target: vDockTitle
                visible: false
            }
            PropertyChanges {
                target: dockTitle
                anchors.topMargin: 84
                visible: true
            }
        }
    ]
}






/*##^## Designer {
    D{i:13;anchors_x:0;anchors_y:84}D{i:12;anchors_x:0;anchors_y:0}D{i:23;anchors_x:0;anchors_y:192}
}
 ##^##*/
