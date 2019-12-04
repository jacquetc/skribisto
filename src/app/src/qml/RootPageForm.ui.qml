import QtQuick 2.9
import QtQuick.Layouts 1.3
import QtQuick.Window 2.3
import QtQuick.Controls 2.3

//import QtGraphicalEffects 1.0
Item {
    id: base
    //property variant window: none
    property alias root_stack: root_stack
    property alias base: base
    property alias statusBarMenuButtonsLoader: statusBarMenuButtonsLoader

    ColumnLayout {
        id: columnLayout
        anchors.fill: parent

        RowLayout {
            id: rowLayout
            Layout.fillHeight: true
            Layout.fillWidth: true

            Loader {
                Layout.minimumHeight: 100
                Layout.minimumWidth: 50
                Layout.fillHeight: true
                sourceComponent: flow_comp
            }
            ColumnLayout {
                Layout.fillHeight: true
                Layout.fillWidth: true

                //        TabBar {
                //            id: bar
                //            position: TabBar.Header
                //            wheelEnabled: true
                //            Layout.fillHeight: false
                //            Layout.fillWidth: true
                //            TabButton {
                //                text: qsTr("Welcome")
                //                width: implicitWidth
                //            }
                //            TabButton {
                //                text: qsTr("Write")
                //                width: implicitWidth
                //            }
                //            TabButton {
                //                text: qsTr("Notes")
                //                width: implicitWidth
                //            }
                //            TabButton {
                //                text: qsTr("Gallery")
                //                width: implicitWidth
                //            }
                //            TabButton {
                //                text: qsTr("Informations")
                //                width: implicitWidth
                //            }
                //        }
                StackLayout {
                    id: root_stack
                    clip: true
                    //            Layout.minimumHeight: 50
                    //            Layout.minimumWidth: 50
                    //            Layout.preferredHeight: 50
                    //            Layout.preferredWidth: 50
                    Layout.fillHeight: true
                    Layout.fillWidth: true

                    WelcomePage {
                        id: welcomePage
                    }
                    WritePage {
                        id: writePage
                    }
                }
                //        PageIndicator {
                //            id: indicator
                //                        Layout.fillHeight: false
                //                        Layout.fillWidth: true
                //            count: view.count
                //            currentIndex: view.currentIndex

                //        }
            }
            //    Rectangle {
            //        color: "transparent"
            //        border.color: "darkorange"
            //        anchors.fill: parent
            //    }
        }

        Pane {
            id: statusBar
            visible: true
            Layout.fillWidth: true
            Layout.preferredHeight: 30

            RowLayout {
                id: rowLayout1
                anchors.fill: parent
                spacing: 1

                Label {
                    id: statusLeftLabel
                    text: qsTr("Label")
                    verticalAlignment: Text.AlignVCenter
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                }

                Label {
                    id: statusRightLabel
                    text: qsTr("Label")
                    verticalAlignment: Text.AlignVCenter
                    Layout.fillHeight: true
                }
                Loader {
                    id: statusBarMenuButtonsLoader
                    Layout.minimumWidth: 100
                    Layout.maximumWidth: 200
                    Layout.fillHeight: true
                    Layout.fillWidth: true

                    sourceComponent: flow_comp
                }
            }
        }
    }
    states: [
        State {
            name: "toolbar"

            PropertyChanges {
                target: pane
                visible: true
            }
        }
    ]
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}D{i:9;anchors_height:100;anchors_width:100}
}
##^##*/

