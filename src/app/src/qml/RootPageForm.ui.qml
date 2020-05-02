import QtQuick 2.9
import QtQuick.Layouts 1.3
import QtQuick.Window 2.12
import QtQuick.Controls 2.12

//import QtGraphicalEffects 1.0
Item {
    id: rootPageBase
    //property variant window: none
    property alias rootPageBase: rootPageBase
    property alias notificationButton: notificationButton
    property alias rootLeftDock: rootLeftDock
//    property alias compactLeftDockShowButton: compactLeftDockShowButton
    property alias leftDockMenuGroup: leftDockMenuGroup
    property alias leftDockResizeButton: leftDockResizeButton
    property alias leftDockMenuButton: leftDockMenuButton
    property alias leftDockShowButton: leftDockShowButton
    property alias rootSwipeView: rootSwipeView
    property alias rootTabBar: rootTabBar

    ColumnLayout {
        id: columnLayout
        spacing: 1
        anchors.fill: parent

        TabBar {
            id: rootTabBar
            width: 240
            Layout.preferredHeight: 30
            Layout.minimumHeight: 30
            Layout.fillWidth: true
        }

        RowLayout {
            id: rowLayout
            spacing: 0
            Layout.fillWidth: true
            Layout.fillHeight: true
            RootLeftDock {
                id: rootLeftDock
                visible: !Globals.compactSize
                Layout.fillHeight: true

            }
            Item {
                Layout.fillHeight: true
                Layout.fillWidth: true
                Item {
                    z: 2
                    id: dockMenuItem
                    anchors.left: parent.left
                    anchors.top: parent.top
                    width: 30
                    height: 200


                    ColumnLayout {
                        anchors.fill: parent
                        Button {
                            id: leftDockShowButton
                            focusPolicy: Qt.NoFocus
                            Layout.preferredHeight: 30
                            Layout.preferredWidth: 30
                            flat: true
                            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                        }

                        Button {
                            id: leftDockMenuButton
                            visible: !Globals.compactSize
                            focusPolicy: Qt.NoFocus
                            checkable: true
                            Layout.preferredHeight: 30
                            Layout.preferredWidth: 30
                            flat: true
                            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                        }

                        ColumnLayout {
                            id: leftDockMenuGroup
                            visible: !Globals.compactSize
                            Layout.alignment: Qt.AlignLeft | Qt.AlignTop

                            Button {
                                id: leftDockResizeButton
                                focusPolicy: Qt.NoFocus
                                Layout.preferredHeight: 30
                                Layout.preferredWidth: 30
                                flat: true
                                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                            }
                        }

                        Item {
                            id: stretcher
                            Layout.fillHeight: true
                            Layout.fillWidth: true
                        }
                    }
                }

                ColumnLayout {
                    z: 0
                    anchors.fill: parent

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

                    SwipeView {
                        id: rootSwipeView
                        clip: true
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                    }
                    //        PageIndicator {
                    //            id: indicator
                    //                        Layout.fillHeight: false
                    //                        Layout.fillWidth: true
                    //            count: view.count
                    //            currentIndex: view.currentIndex

                    //        }
                }
            }


            //    Rectangle {
            //        color: "transparent"
            //        border.color: "darkorange"
            //        anchors.fill: parent
            //    }
        }


        Pane {
            id: statusBar
            Layout.preferredHeight: 30
            visible: true
            Layout.fillWidth: true
            padding: 0

            RowLayout {
                id: rowLayout1
                anchors.top: parent.top
                anchors.right: parent.right
                anchors.bottom: parent.bottom
                anchors.left: parent.left
                anchors.topMargin: 0
                spacing: 0

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


                Button{
                    id: notificationButton
                    flat: true
                    Layout.preferredWidth: 40
                    Layout.fillHeight: true
                    padding: 0


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

