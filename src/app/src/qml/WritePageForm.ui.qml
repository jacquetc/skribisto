import QtQuick 2.11
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3


//import org.kde.kirigami 2.4 as Kirigami
Item {
    id: base
    property alias writingZone: writingZone
    property alias base: base
    width: 1000
    height: 600
    property alias leftDock: leftDock
    property alias resizeHandle: resizeHandle
    property alias resizeHandleMouseArea: resizeHandleMouseArea

    Pane {
        id: pane
        padding: 3
        anchors.fill: parent

        RowLayout {
            id: rowLayout
            spacing: 0
            anchors.fill: parent

            WritingZone {
                id: writingZone
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                Layout.fillHeight: true
                Layout.fillWidth: true
            }
        }

        WriteLeftDock {
            id: leftDock
            contentWidth: writingZone.textAreaLeftPos
            anchors.rightMargin: -writingZone.textAreaLeftPos
            anchors.right: parent.left
            anchors.bottom: parent.bottom
            anchors.left: parent.left
            anchors.top: parent.top


        }

        Item {
            id: rightDock
            anchors.leftMargin: writingZone.textAreaRightPos
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.topMargin: 0
            anchors.right: parent.right
            anchors.rightMargin: 0
            anchors.bottom: parent.bottom
            anchors.bottomMargin: 0

            ScrollView {
                id: scrollView1
                anchors.fill: parent
                contentWidth: writingZone.width - writingZone.textAreaRightPos

                ColumnLayout {
                    id: columnLayout1
                    anchors.fill: parent
                }
            }
        }

        Rectangle {
            id: resizeHandle
            color: "#00000000"
            z: 1
            anchors.leftMargin: writingZone.textAreaLeftPos - 3
            anchors.left: parent.left
            anchors.bottom: parent.bottom
            anchors.bottomMargin: 0
            anchors.top: parent.top
            anchors.topMargin: 0
            anchors.rightMargin: -writingZone.textAreaLeftPos
            anchors.right: parent.left

            MouseArea {
                id: resizeHandleMouseArea
                anchors.fill: parent
                hoverEnabled: true
            }
        }
    }
    states: [
        State {
            name: "hoveringResizeHandle"
            when: resizeHandleMouseArea.containsMouse | resizeHandleMouseArea.pressed

            PropertyChanges {
                target: resizeHandle
                color: "#ffa369"
            }
        }

    ]
}


/*##^## Designer {
    D{i:6;anchors_height:200;anchors_width:200}D{i:9;anchors_height:200;anchors_width:200}
D{i:8;anchors_height:200;anchors_width:200}
}
 ##^##*/
