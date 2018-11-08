import QtQuick 2.11
import QtQuick.Controls 2.4
import QtQuick.Layouts 1.3

Item {
    id: item1

    Pane {
        id: pane
        anchors.fill: parent
        padding: 0

        ColumnLayout {
            id: columnLayout
            anchors.fill: parent

            ScrollView {
                id: flickable
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
                        id: toolButton1
                        text: qsTr("Tool Button")
                    }
                    ToolButton {
                        id: toolButton2
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
                clip: true
                Layout.fillWidth: true
                Layout.fillHeight: true
                delegate: ItemDelegate {
                    x: 5
                    width: 80
                    height: 40
                    Row {
                        id: row1
                        spacing: 10
                        Image {
                            id: image
                            width: 40
                            height: 40
                            fillMode: Image.PreserveAspectFit
                            source: "qrc:/pics/author.svg"
                        }

                        Item {
                            id: item3
                            width: 200
                            height: 50

                            Column {
                                id: column
                                anchors.fill: parent

                                Text {
                                    text: name
                                    font.bold: true
                                }

                                Text {
                                    id: text1
                                    text: tag
                                    font.pixelSize: 12
                                }
                            }
                        }
                    }
                }
                model: ListModel {
                    ListElement {
                        name: "Grey"
                        colorCode: "grey"
                        tag: 'ee'
                    }

                    ListElement {
                        name: "Red"
                        colorCode: "red"
                        tag: 'ee'
                    }

                    ListElement {
                        name: "Blue"
                        colorCode: "blue"
                        tag: 'ee'
                    }

                    ListElement {
                        name: "Green"
                        colorCode: "green"
                        tag: 'ee'
                    }
                }
            }
        }
    }
}


/*##^## Designer {
    D{i:0;autoSize:true;height:300;width:300}
}
 ##^##*/
