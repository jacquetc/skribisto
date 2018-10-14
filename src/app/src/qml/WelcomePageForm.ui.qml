import QtQuick 2.9
import QtQuick.Layouts 1.3
import QtQuick.Controls 2.2

Item {
    property alias open_project_button: open_project_button
    property alias recent_list_view: recent_list_view
    property alias new_project: new_project

    Item {
        anchors.fill: parent
        Grid {
            id: flow1
            columns: 2
            rows: 3
            anchors.fill: parent
            spacing: 10

            Item {
                id: item1
                width: 200
                height: 200

                ColumnLayout {
                    id: columnLayout
                    anchors.fill: parent


                    //                    BreadCrumb {
                    //                        id: breadCrumb
                    //                        Layout.fillWidth: false
                    //                        Layout.fillHeight: false
                    //                    }
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
                        clip: true
                        Layout.preferredHeight: 200
                        Layout.preferredWidth: 300
                        Layout.fillHeight: false
                        Layout.minimumHeight: 200
                        Layout.minimumWidth: 300
                        Layout.fillWidth: false
                        keyNavigationWraps: false
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
                }
            }

            Button {
                id: open_project_button
                text: qsTr("Open project")
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
            }

            Button {
                id: new_project
                text: qsTr("New project")
            }
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
