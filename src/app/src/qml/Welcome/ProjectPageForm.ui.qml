import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

Item {
    width: 400
    height: 400
    property alias createEmpyProjectAtStartSwitch: createEmpyProjectAtStartSwitch
    property alias gridLayout: gridLayout

    property alias open_project_button: open_project_button
    property alias recent_list_view: recent_list_view
    property alias new_project: new_project

    Pane {
        id: pane1
        anchors.fill: parent

        SwipeView {
            id: swipeView
            anchors.fill: parent

            Item {
                GridLayout {
                    id: gridLayout
                    anchors.fill: parent

                    GroupBox {
                        id: groupBox1
                        width: 200
                        height: 200
                        title: qsTr("Recent projects")

                        ColumnLayout {
                            id: columnLayout
                            width: 100
                            height: 100

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

                    GroupBox {
                        id: groupBox
                        width: 200
                        height: 200
                        title: qsTr("")

                        ColumnLayout {
                            id: columnLayout1
                            width: 100
                            height: 100


                            Button {
                                id: new_project
                                text: qsTr("New project")
                            }

                            Button {
                                id: open_project_button
                                text: qsTr("Open project")
                                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                            }
                            
                            Switch {
                                id: createEmpyProjectAtStartSwitch
                                text: qsTr("Switch")
                            }

                        }
                    }

                    Item {
                        id: element
                        width: 200
                        height: 200
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                    }
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:2;anchors_height:200;anchors_width:200}
}
##^##*/

