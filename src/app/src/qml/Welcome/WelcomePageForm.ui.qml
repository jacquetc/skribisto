import QtQuick 2.9
import QtQuick.Layouts 1.3
import QtQuick.Controls 2.2

Item {
    property alias projectPageButton: projectPageButton
    property alias examplePageButton: examplePageButton
    property alias helpPageButton: helpPageButton
    property alias settingsPageButton: settingsPageButton
    property alias examplePageTabButton: examplePageTabButton
    property alias helpPageTabButton: helpPageTabButton
    property alias projectPageTabButton: projectPageTabButton
    property alias settingsPageTabButton: settingsPageTabButton


    property alias stackLayout: stackLayout
    property alias tabBar: tabBar
    property alias separator: separator
    property alias mainButtonsPane: mainButtonsPane

    ColumnLayout {
        id: columnLayout2
        spacing: 0
        anchors.fill: parent

        TabBar {
            id: tabBar
            width: 240
            Layout.preferredHeight: 40
            Layout.fillWidth: true

            TabButton {
                id: projectPageTabButton
                text: qsTr("Project")
            }
            TabButton {
                id: examplePageTabButton
                text: qsTr("Examples")
            }
            TabButton {
                id: settingsPageTabButton
                text: qsTr("Settings")
            }

            TabButton {
                id: helpPageTabButton
                text: qsTr("Help")
            }
        }

        RowLayout {
            id: rowLayout
            Layout.fillWidth: true
            Layout.fillHeight: true

            Pane {
                id: mainButtonsPane
                width: 200
                height: 200
                Layout.minimumWidth: 200
                Layout.maximumWidth: 300
                Layout.preferredWidth: 200
                Layout.fillHeight: true

                ColumnLayout {
                    id: columnLayout1
                    anchors.fill: parent

                    Button {
                        id: projectPageButton
                        text: qsTr("Project")
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    }

                    Button {
                        id: examplePageButton
                        text: qsTr("Examples")
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    }

                    Button {
                        id: settingsPageButton
                        text: qsTr("Settings")
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    }

                    Button {
                        id: helpPageButton
                        text: qsTr("Help")
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    }

                    Item {
                        id: element
                        width: 200
                        height: 200
                        Layout.preferredWidth: 10
                        Layout.fillHeight: true
                    }
                }
            }

            Rectangle {
                id: separator
                width: 200
                height: 200
                color: "#4e4e4e"
                Layout.maximumWidth: 2
                Layout.fillHeight: true
            }

            StackLayout {
                id: stackLayout
                width: 100
                height: 100
                Layout.fillWidth: true
                Layout.fillHeight: true

                Item {
                    id: projectPage
                    ProjectPage {
                        anchors.fill: parent
                    }
                }

                Item {
                    id: examplePage

                    ExamplePage {

                    }
                }

                Item {
                    id: settingsPage

                    SettingsPage{
                        anchors.fill: parent
                    }
                }

                Item {
                    id: helpPage

                    HelpPage {
                        anchors.fill: parent
                    }
                }
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

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}D{i:3;anchors_height:100;anchors_width:100}
D{i:12;anchors_height:200;anchors_width:200}D{i:13;anchors_height:100;anchors_width:100;anchors_x:0;anchors_y:0}
D{i:14;anchors_height:100;anchors_width:100;anchors_x:0;anchors_y:0}D{i:9;anchors_height:100;anchors_width:100}
D{i:30;anchors_height:200;anchors_width:200}D{i:31;anchors_height:100;anchors_width:100}
D{i:34;anchors_height:100;anchors_width:100}D{i:19;anchors_height:100;anchors_width:100;anchors_x:0;anchors_y:0}
D{i:18;anchors_height:200;anchors_width:200}D{i:38;anchors_height:100;anchors_width:100}
D{i:37;anchors_height:200;anchors_width:200}D{i:7;anchors_height:100;anchors_width:100}
D{i:1;anchors_height:100;anchors_width:100}
}
##^##*/

