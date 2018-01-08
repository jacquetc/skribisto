import QtQuick 2.9
import QtQuick.Layouts 1.3
import QtQuick.Controls 2.2

Item {
    property alias open_project_button: open_project_button
    property alias recent_list_view: recent_list_view
    property alias new_project: new_project
    anchors.fill: parent

    Rectangle {
        id: rectangle1
        color: "#ffffff"
        anchors.rightMargin: 0
        anchors.bottomMargin: 0
        anchors.leftMargin: 0
        anchors.topMargin: 0
        anchors.fill: parent

        GridLayout {
            id: grid1
            anchors.rightMargin: 0
            anchors.bottomMargin: 0
            anchors.leftMargin: 0
            anchors.topMargin: 0
            rows: 5
            columns: 3
            anchors.fill: parent
            flow: GridLayout.TopToBottom

            Item {
                id: item1
                width: 200
                height: 200
                Layout.columnSpan: 3
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                Layout.maximumHeight: 100
                Layout.maximumWidth: 100
                Layout.minimumHeight: 50
                Layout.minimumWidth: 50
            }

            Button {
                id: open_project_button
                text: qsTr("Open project")
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
            }

            Text {
                id: text1
                text: qsTr("Recent projects")
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                horizontalAlignment: Text.AlignHCenter
                font.pixelSize: 12
            }

            ListView {
                id: recent_list_view
                width: 110
                height: 160
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                model: ListModel {
                    ListElement {
                        name: "Grey"
                        colorCode: "grey"
                    }

                    ListElement {
                        name: "Red"
                        colorCode: "red"
                    }

                    ListElement {
                        name: "Blue"
                        colorCode: "blue"
                    }

                    ListElement {
                        name: "Green"
                        colorCode: "green"
                    }
                }
                delegate: Item {
                    x: 5
                    width: 80
                    height: 40
                    Row {
                        id: row1
                        spacing: 10
                        Rectangle {
                            width: 40
                            height: 40
                            color: colorCode
                        }

                        Text {
                            text: name
                            font.bold: true
                            anchors.verticalCenter: parent.verticalCenter
                        }
                    }
                }
            }

            Item {
                id: item2
                width: 200
                height: 200
                Layout.fillHeight: true
                Layout.columnSpan: 3
                Layout.maximumHeight: 200
                Layout.maximumWidth: 200
                Layout.minimumHeight: 100
                Layout.minimumWidth: 100
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
            }

            Button {
                id: new_project
                text: qsTr("New project")
            }
        }

        RadioButton {
            id: radioButton
            x: 255
            y: 273
            text: qsTr("Radio Button")
        }
    }

    //    ListView {
    //        id: listView
    //        anchors.fill: parent
    //        model: projectListModel
    //        delegate: projectItemDelegate

    //    }

    //    ListModel {
    //        id: projectListModel
    //        ListElement {
    //            name: "Grey"
    //            colorCode: "grey"
    //        }

    //        ListElement {
    //            name: "Red"
    //            colorCode: "red"
    //        }

    //        ListElement {
    //            name: "Blue"
    //            colorCode: "blue"
    //        }

    //        ListElement {
    //            name: "Green"
    //            colorCode: "green"
    //        }
    //    }
    //    Component {
    //        id: projectItemDelegate
    //        Row {
    //            spacing: 10
    //            Text { text: name }
    //            Text { text: '$' + cost }
    //        }
    //    }
}
