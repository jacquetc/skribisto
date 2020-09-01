import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

Item {
    id: base

    Pane {
        id: pane
        clip: true
        anchors.fill: parent

        ColumnLayout {
            id: columnLayout
            spacing: 0
            anchors.fill: parent

            ToolBar {
                id: toolBar
                Layout.maximumHeight: 40
                Layout.preferredHeight: 40
                Layout.fillWidth: true

                //                Item {
                //                    id: element
                //                    Layout.fillHeight: true
                //                    Layout.fillWidth: true
                //                }
                RowLayout {
                    id: rowLayout
                    spacing: 1
                    anchors.fill: parent

                    ToolButton {
                        id: goBackToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                    }

                    ToolButton {
                        id: restoreToolButton
                        flat: true
                        text: qsTr("Restore a document")
                        display: AbstractButton.IconOnly
                    }

                    ToolButton {
                        id: listMenuToolButton
                        flat: true
                        text: qsTr("Deleted menu")
                        display: AbstractButton.IconOnly
                    }
                }
            }

            ScrollView {
                id: scrollView
                focusPolicy: Qt.StrongFocus
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: scrollBarVerticalPolicy

                Flow {
                    id: noteFlow
                    anchors.fill: parent
                    spacing: 15
                    padding: 0
                    antialiasing: true
                    clip: true

                    Text {
                        text: "Text"
                        font.pixelSize: 40
                    }
                    Text {
                        text: "items"
                        font.pixelSize: 40
                    }
                    Text {
                        text: "flowing"
                        font.pixelSize: 40
                    }
                    Text {
                        text: "inside"
                        font.pixelSize: 40
                    }
                    Text {
                        text: "a"
                        font.pixelSize: 40
                    }
                    Text {
                        text: "Flow"
                        font.pixelSize: 40
                    }
                    Text {
                        text: "item"
                        font.pixelSize: 40
                    }
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:300}
}
##^##*/
