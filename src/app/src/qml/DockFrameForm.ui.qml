import QtQuick 2.4
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3

Item {
    id: item1
    property alias dockBody: dockBody
    property bool isCollapsed: false

    //property alias dockEdge: dockEdge

    // Qt.LeftEdge Qt.RightEdge
    ColumnLayout {
        id: columnLayout
        spacing: 0
        anchors.fill: parent

        Rectangle {
            id: dockTop
            color: "#ffffff"
            Layout.preferredHeight: 30
            Layout.fillHeight: false
            Layout.fillWidth: true

            RowLayout {
                id: rowLayout
                anchors.fill: parent

                //                Layout.maximumHeight: 30
                //                Layout.fillHeight: false
                //                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                //                Layout.fillWidth: true
                RoundButton {
                    id: closeButton
                    height: 20
                    text: qsTr("Close")
                    Layout.preferredHeight: 20
                    Layout.fillWidth: false
                    Layout.minimumHeight: 20
                    onClicked: isCollapsed = true
                }
                RoundButton {
                    id: showButton
                    visible: false
                    height: 20
                    text: "O"
                    Layout.preferredHeight: 20
                    Layout.fillWidth: false
                    Layout.minimumHeight: 20
                    onClicked: isCollapsed = false
                }
            }
        }

        Item {
            id: dockBody
            clip: true
            Layout.fillHeight: true
            Layout.fillWidth: true
        }
    }

    states: [
        State {
            name: "LeftEdgeCollapsed"
            when: isCollapsed === true

            PropertyChanges {
                target: dockBody
                visible: false
            }
            PropertyChanges {
                target: showButton
                visible: true
            }

            PropertyChanges {
                target: closeButton
                visible: false
            }

            PropertyChanges {
                target: item1
                width: 50
                height: 30
            }
        }
    ]
}
