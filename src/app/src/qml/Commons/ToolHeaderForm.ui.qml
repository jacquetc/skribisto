import QtQuick 2.4
import QtQuick.Controls 2.3
import QtQuick.Layouts 1.3
import QtQuick.Controls.Material 2.15
import "../Items"
import ".."

Item {
    id: base
    width: 400
    height: 400
    property alias vDockTitle: vDockTitle
    property alias dockTitle: dockTitle
    property bool folded: true
    property alias title: dockTitle.text
    property alias base: base

    property var edge: Qt.LeftEdge

    GridLayout {
        id: gridBase
        anchors.fill: parent
        rows: 2
        columns: 2

        SkrSwitch {
            id: hSwitch
            scale: 0.5

            Layout.row: 0
            Layout.column: edge === Qt.LeftEdge ? 0 : 1

            Layout.preferredHeight: 30
            Layout.preferredWidth: 30
            Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
            padding: 1
            checked: !folded
            onCheckedChanged: checked ? folded = false : folded = true
            focusPolicy: Qt.NoFocus

        }
        SkrLabel {
            id: dockTitle
            //width: gridBase.width - hSwitch.width
            height: 30
            text: "Text"

            Layout.fillWidth: true
            Layout.alignment: Qt.AlignTop | Qt.AlignHCenter

            Layout.row: 0
            Layout.column: edge === Qt.LeftEdge ? 1 : 0

            padding: 2
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
            font.pixelSize: 12
        }

        SkrLabel {
            id: vDockTitle
            text: dockTitle.text
            verticalAlignment: Text.AlignVCenter
            horizontalAlignment: Text.AlignHCenter
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.preferredHeight: 30

            Layout.row: 1
            Layout.column: 0
            Layout.columnSpan: 2

            padding: 2
            antialiasing: true
            rotation: 270
            font.pixelSize: 12

            visible: false
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
                target: dockTitle
                opacity: 0.0
            }

            PropertyChanges {
                target: vDockTitle
                visible: true
            }
            PropertyChanges {
                target: vDockTitle
                opacity: 1.0
            }

            //            PropertyChanges {
            //                target: hSwitch
            //                rotation: 90
            //            }
            PropertyChanges {
                target: base
                width: 30
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
                target: vDockTitle
                opacity: 0.0
            }
            PropertyChanges {
                target: dockTitle
                visible: true
            }
            PropertyChanges {
                target: dockTitle
                opacity: 1.0
            }

            PropertyChanges {
                target: base
                height: 30
            }
        }
    ]
}
