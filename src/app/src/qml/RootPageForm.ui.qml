import QtQuick 2.9
import QtQuick.Layouts 1.3
import QtQuick.Window 2.3
import QtQuick.Controls 2.3

//import QtGraphicalEffects 1.0
import "Write"
import "Welcome"
import "Notes"
import "Gallery"
import "Projects"

Item {
    id: base
    //property variant window: none
    property alias root_stack: root_stack
    property alias base: base
    property alias statusBarMenuButtonsLoader: statusBarMenuButtonsLoader
    property alias sideMenuButtonsLoader: sideMenuButtonsLoader
    property alias welcomePage: welcomePage
    property alias writePage: writePage
    property alias notesPage: notesPage
    property alias galleryPage: galleryPage
    property alias projectsPage: projectsPage
    property alias notificationButton: notificationButton

    ColumnLayout {
        id: columnLayout
        spacing: 1
        anchors.fill: parent

        RowLayout {
            id: rowLayout
            spacing: 0
            Layout.fillWidth: true
            Layout.fillHeight: true

            Loader {
                id: sideMenuButtonsLoader
                Layout.preferredWidth: 70
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
                    NotesPage {
                        id: notesPage
                    }
                    GalleryPage {
                        id: galleryPage
                    }
                    ProjectsPage {
                        id: projectsPage
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
            Layout.preferredHeight: 30
            visible: true
            Layout.fillWidth: true
            padding: 0

            RowLayout {
                id: rowLayout1
                anchors.fill: parent
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
                Loader {
                    id: statusBarMenuButtonsLoader
                    Layout.minimumWidth: 100
                    Layout.maximumWidth: 200
                    //Layout.preferredHeight: 30
                    Layout.fillHeight: true
                    Layout.fillWidth: true

                    sourceComponent: flow_comp
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
    D{i:0;autoSize:true;height:480;width:640}D{i:10;anchors_height:100;anchors_width:100}
}
##^##*/

