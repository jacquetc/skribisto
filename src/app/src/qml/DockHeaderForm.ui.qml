import QtQuick 2.4
import QtQuick.Controls 2.3
import QtQuick.Layouts 1.3

Item {
    id: base
    width: 400
    height: 400
    property bool folded: !hSwitch.checked
    property alias dockTitle: dockTitle
    property alias vDockTitle: vDockTitle

    Rectangle {
        id: rectangle
        color: "#ffffff"
        anchors.fill: parent

        RowLayout {
            id: rowLayout
            spacing: 1
            anchors.fill: parent

            Switch {
                id: hSwitch
                padding: 1
                scale: 0.5
                Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
            }

            Text {
                id: dockTitle
                text: qsTr("Text")
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                font.pixelSize: 12
            }
        }

        ColumnLayout {
            id: columnLayout
            spacing: 1
            anchors.fill: parent

            Item {
                id: item1
                width: hSwitch.width
                height: hSwitch.height
            }

            Text {
                id: vDockTitle
                text: qsTr("Text")
                rotation: 270
                font.pixelSize: 12
            }

        }
    }
    states: [
        State {
            name: "folded"
            when: hSwitch.checked === false

            PropertyChanges {
                target: vDockTitle
                visible: false
            }
            PropertyChanges {
                target: base
                implicitHeight: 30
                implicitWidth: 0x0
            }


        },
        State {
            name: "unfolded"
            when: hSwitch.checked === true

            PropertyChanges {
                target: dockTitle
                visible: false
            }

            PropertyChanges {
                target: hSwitch
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
            }
            PropertyChanges {
                target: base
                implicitHeight: 0x0
                implicitWidth: 30
            }

        }
    ]
}

/*##^## Designer {
    D{i:4;anchors_height:100;anchors_width:100}D{i:7;anchors_height:100;anchors_width:100}
D{i:1;anchors_height:200;anchors_width:200}
}
 ##^##*/
