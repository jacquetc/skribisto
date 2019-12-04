import QtQuick 2.11
import QtQuick.Controls 2.4
import QtQuick.Layouts 1.3

Item {
    id: base
    property alias listView: listView
    property bool selectionMode: false

    Pane {
        id: pane
        anchors.fill: parent
        padding: 0

        ColumnLayout {
            id: columnLayout
            anchors.fill: parent

            ScrollView {
                id: flickable
                clip: true
                Layout.minimumHeight: 50
                Layout.maximumHeight: 50
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                Layout.fillWidth: true
                Layout.fillHeight: true

                Row {
                    id: row

                    ToolButton {
                        id: toolButton
                        text: qsTr("Tool Button")
                    }

                    ToolButton {
                        id: toolButton2
                        text: qsTr("Tool Button")
                    }

                    ToolButton {
                        id: toolButton1
                        text: qsTr("Tool Button")
                    }
                    ToolButton {
                        id: toolButton3
                        text: qsTr("Tool Button")
                    }
                    ToolButton {
                        id: toolButton4
                        text: qsTr("Tool Button")
                    }
                }
            }

            ListView {
                id: listView
                antialiasing: true
                smooth: true
                clip: true
                Layout.fillWidth: true
                Layout.fillHeight: true
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:300;width:300}
}
##^##*/

