import QtQuick 2.9
import QtQuick.Layouts 1.3
import QtQuick.Window 2.12
import QtQuick.Controls 2.12

import "WriteOverview"
import "Welcome"
import "Notes"
import "Gallery"
import "Projects"

//import QtGraphicalEffects 1.0
Item {
    id: rootPageBase
    //property variant window: none
    property alias rootPageBase: rootPageBase
    property alias notificationButton: notificationButton
    property alias rootSwipeView: rootSwipeView
    property alias rootTabBar: rootTabBar
    property alias welcomeTab: welcomeTab
    property alias writeTab: writeTab
    property alias projectTab: projectTab
    property alias galleryTab: galleryTab
    property alias notesTab: notesTab
    property alias welcomePage: welcomePage
    property alias notesPage: notesPage
    property alias galleryPage: galleryPage
    property alias projectsPage: projectsPage
    property alias writeOverviewPage: writeOverviewPage

    ColumnLayout {
        id: columnLayout
        spacing: 1
        anchors.fill: parent

        TabBar {
            id: rootTabBar
            Layout.preferredHeight: 30
            Layout.minimumHeight: 30
            Layout.fillWidth: true
                        Tab {
                            id: welcomeTab
                            closable: false

                        }
                        Tab {
                            id: writeTab
                            closable: false

                        }
                        Tab {
                            id: notesTab
                            closable: false

                        }
                        Tab {
                            id: galleryTab
                            closable: false

                        }
                        Tab {
                            id: projectTab
                            closable: false

                       }
        }

        RowLayout {
            id: rowLayout
            spacing: 0
            Layout.fillWidth: true
            Layout.fillHeight: true

            Item {
                Layout.fillHeight: true
                Layout.fillWidth: true


                ColumnLayout {
                    z: 0
                    anchors.fill: parent

//                            TabBar {
//                                id: bar
//                                position: TabBar.Header
//                                wheelEnabled: true
//                                Layout.fillHeight: false
//                                Layout.fillWidth: true
//                                TabButton {
//                                    text: qsTr("Welcome")
//                                    width: implicitWidth
//                                }
//                                TabButton {
//                                    text: qsTr("Write")
//                                    width: implicitWidth
//                                }
//                                TabButton {
//                                    text: qsTr("Notes")
//                                    width: implicitWidth
//                                }
//                                TabButton {
//                                    text: qsTr("Gallery")
//                                    width: implicitWidth
//                                }
//                                TabButton {
//                                    text: qsTr("Informations")
//                                    width: implicitWidth
//                                }
//                            }

                    SwipeView {
                        id: rootSwipeView
                        clip: true
                        Layout.fillHeight: true
                        Layout.fillWidth: true

                        WelcomePage {
                            id: welcomePage

                        }

                        WriteOverviewPage {
                            id: writeOverviewPage

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

