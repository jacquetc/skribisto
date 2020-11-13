import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15

import "Items"
import "WriteOverview"
import "Welcome"
import "NoteOverview"
import "Gallery"
import "Projects"

//import "Write"
//import "Note"

//import QtGraphicalEffects 1.0
Item {
    id: rootPageBase
    //property variant window: none
    property alias rootPageBase: rootPageBase
    property alias notificationButton: notificationButton
    property alias rootSwipeView: rootSwipeView
    property alias rootTabBar: rootTabBar
    property alias welcomeTab: welcomeTab
    property alias writeOverviewTab: writeOverviewTab
    property alias projectTab: projectTab
    //NOTE: waiting to be implemented
    //property alias galleryTab: galleryTab
    property alias noteOverviewTab: noteOverviewTab
    property alias welcomePage: welcomePage
    property alias noteOverviewPage: noteOverviewPage
    //NOTE: waiting to be implemented
    //property alias galleryPage: galleryPage
    property alias projectsMainPage: projectsMainPage
    property alias writeOverviewPage: writeOverviewPage
    property alias saveButton: saveButton
    property alias tabBarRevealer: tabBarRevealer
    property alias mainMenuButton: mainMenuButton
    property alias showTabListButton: showTabListButton
    property alias headerRowLayout: headerRowLayout
    property alias statusLeftLabel: statusLeftLabel
    property alias statusRightLabel: statusRightLabel
    property alias welcomeStatusBarButton: welcomeStatusBarButton
    property alias writeOverviewStatusBarButton: writeOverviewStatusBarButton
    property alias noteOverviewStatusBarButton: noteOverviewStatusBarButton
    property alias projectStatusBarButton: projectStatusBarButton

    ColumnLayout {
        id: columnLayout
        spacing: 1
        anchors.fill: parent

        RowLayout {
            id: headerRowLayout
            width: 100
            height: 100
            spacing: 0
            Layout.preferredHeight: 30
            Layout.fillWidth: true


            SkrToolButton {
                id: mainMenuButton
                text: qsTr("Main menu")
                focusPolicy: Qt.NoFocus
                padding: 2
                display: AbstractButton.IconOnly
                Layout.preferredHeight: 30
                Layout.preferredWidth: 30
                checkable: true
                flat: true
            }




            SkrTabBar {
                id: rootTabBar
                //Layout.preferredHeight: 40
                Layout.minimumHeight: 30
                Layout.fillWidth: true

                SkrTabButton {
                    id: welcomeTab
                    closable: false
                    pageType: welcomePage.pageType
                    visible: !SkrSettings.interfaceSettings.menuButtonsInStatusBar
                    width: SkrSettings.interfaceSettings.menuButtonsInStatusBar ? 0 : implicitWidth
                    textVisible: !SkrSettings.interfaceSettings.minimalistMenuTabs
                }
                SkrTabButton {
                    id: writeOverviewTab
                    closable: false
                    pageType: writeOverviewPage.pageType
                    visible: !SkrSettings.interfaceSettings.menuButtonsInStatusBar
                    width: SkrSettings.interfaceSettings.menuButtonsInStatusBar ? 0 : implicitWidth
                    textVisible: !SkrSettings.interfaceSettings.minimalistMenuTabs
                }
                SkrTabButton {
                    id: noteOverviewTab
                    closable: false
                    pageType: noteOverviewPage.pageType
                    visible: !SkrSettings.interfaceSettings.menuButtonsInStatusBar
                    width: SkrSettings.interfaceSettings.menuButtonsInStatusBar ? 0 : implicitWidth
                    textVisible: !SkrSettings.interfaceSettings.minimalistMenuTabs
                }
                //NOTE: waiting to be implemented
//                SkrTabButton {
//                    id: galleryTab
//                    closable: false
//                    pageType: galleryPage.pageType
//                    visible: !SkrSettings.interfaceSettings.menuButtonsInStatusBar
//                    textVisible: !SkrSettings.interfaceSettings.minimalistMenuTabs
// }
                SkrTabButton {
                    id: projectTab
                    closable: false
                    pageType: projectsMainPage.pageType
                    visible: !SkrSettings.interfaceSettings.menuButtonsInStatusBar
                    width: SkrSettings.interfaceSettings.menuButtonsInStatusBar ? 0 : implicitWidth
                    textVisible: !SkrSettings.interfaceSettings.minimalistMenuTabs
                }
            }


            SkrToolButton {
                id: showTabListButton
                text: qsTr("Show list of tabs")
                flat: true
                checkable: true
                focusPolicy: Qt.NoFocus
                padding: 2
                display: AbstractButton.IconOnly
                Layout.preferredHeight: 30
                Layout.preferredWidth: 30
            }
        }

        Item {
            id: tabBarRevealer
            visible: false
            Layout.minimumHeight: 5
            Layout.preferredHeight: 5
            Layout.fillWidth: true
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
                    //                                SkrTabButton {
                    //                                    text: qsTr("Welcome")
                    //                                    width: implicitWidth
                    //                                }
                    //                                SkrTabButton {
                    //                                    text: qsTr("Write")
                    //                                    width: implicitWidth
                    //                                }
                    //                                SkrTabButton {
                    //                                    text: qsTr("Note")
                    //                                    width: implicitWidth
                    //                                }
                    //                                SkrTabButton {
                    //                                    text: qsTr("Gallery")
                    //                                    width: implicitWidth
                    //                                }
                    //                                SkrTabButton {
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
                            enabled: false
                        }

                        NoteOverviewPage {
                            id: noteOverviewPage
                            enabled: false
                        }

                        //NOTE: waiting to be implemented
//                        GalleryPage {
//                            id: galleryPage
//                            enabled: false
//                            visible: false
//                        }

                        ProjectsMainPage {
                            id: projectsMainPage
                            enabled: false
                        }

                        //                        WritePage {
                        //                            id: writePage
                        //                        }

                        //                                                NotePage {
                        //                                                    id: notePage

                        //                                                }
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

        SkrPane {
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

                SkrToolButton {
                    id: saveButton
                    flat: true
                    text: qsTr("Save")
                    padding: 0
                    Layout.preferredHeight: 30
                    Layout.preferredWidth: 30
                    focusPolicy: Qt.NoFocus
                    display: AbstractButton.IconOnly
                }

                SkrLabel {
                    id: statusLeftLabel
                    verticalAlignment: Text.AlignVCenter
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                }

                Item {
                    id: stretcher
                    Layout.fillWidth: true

                }

                SkrLabel {
                    id: statusRightLabel
                    text: qsTr("Label")
                    verticalAlignment: Text.AlignVCenter
                    Layout.fillHeight: true
                }

                SkrToolButton {
                    id: welcomeStatusBarButton
                    Layout.preferredWidth: 40
                    Layout.fillHeight: true
                    padding: 0
                    display: AbstractButton.IconOnly
                    visible: SkrSettings.interfaceSettings.menuButtonsInStatusBar
                }
                SkrToolButton {
                    id: writeOverviewStatusBarButton
                    Layout.preferredWidth: 40
                    Layout.fillHeight: true
                    padding: 0
                    display: AbstractButton.IconOnly
                    visible: SkrSettings.interfaceSettings.menuButtonsInStatusBar
                }
                SkrToolButton {
                    id: noteOverviewStatusBarButton
                    Layout.preferredWidth: 40
                    Layout.fillHeight: true
                    padding: 0
                    display: AbstractButton.IconOnly
                    visible: SkrSettings.interfaceSettings.menuButtonsInStatusBar
                }
                //NOTE: waiting to be implemented
//                SkrToolButton {
//                    id: galleryStatusBarButton
//                    Layout.preferredWidth: 40
//                    Layout.fillHeight: true
//                    padding: 0
//                             display: AbstractButton.IconOnly
//       visible: SkrSettings.interfaceSettings.menuButtonsInStatusBar
//  }
                SkrToolButton {
                    id: projectStatusBarButton
                    Layout.preferredWidth: 40
                    Layout.fillHeight: true
                    padding: 0
                    display: AbstractButton.IconOnly
                    visible: SkrSettings.interfaceSettings.menuButtonsInStatusBar
                }

                SkrToolButton {
                    id: notificationButton
                    Layout.preferredWidth: 40
                    Layout.fillHeight: true
                    padding: 0
                }
            }
        }
    }

}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

