import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

Item {
    id: base
    property alias listView: listView
    property bool selectionMode: false

    Pane {
        id: pane
        clip: true
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
                        id: goUpToolButton
                        text: qsTr("^")
                    }

                    ToolButton {
                        id: currentFolderToolButton
                        text: qsTr("current folder name")
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

                //                model: ListModel {
                //                    ListElement {
                //                        name: "Bill Smith"
                //                    }
                //                    ListElement {
                //                        name: "John Brown"
                //                    }
                //                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:300;width:300}
}
##^##*/

