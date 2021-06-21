import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
//import QtQuick.Controls.Material 2.15

import "Items"

Item {
    id: rootPageBase
    property alias viewManager: multiViewArea.viewManager
//    property int leftBasePreferredWidth: 0
//    property int rightBasePreferredWidth: 0
    property alias mainMenuButton: mainMenuButton
    property alias showWelcomeButton: showWelcomeButton
    property alias baseForDrawers: baseForDrawers
    property alias distractionFreeButton: distractionFreeButton
    property alias themeColorButton: themeColorButton
    property alias showLeftDockButton: showLeftDockButton
    property alias showRightDockButton: showRightDockButton
    property alias topToolBarRepeater: topToolBarRepeater

    property int showLeftDockButtonWidth: 30
    property int showRightDockButtonWidth: 30

    ColumnLayout {
        id: baseColumnLayout
        anchors.fill: parent


        RowLayout {
            id: headerRowLayout
            width: 100
            height: 100
            spacing: 0
            Layout.preferredHeight: 30
            Layout.fillWidth: true

            SkrToolButton {
                id: showLeftDockButton

                Layout.preferredHeight: 30
                Layout.preferredWidth: showLeftDockButtonWidth

                focusPolicy: Qt.NoFocus

            }

            SkrToolButton {
                id: mainMenuButton
                text: qsTr("Main menu")
                visible: SkrSettings.accessibilitySettings.showMenuButton
                focusPolicy: Qt.NoFocus
                padding: 2
                Layout.preferredHeight: 30
                Layout.preferredWidth: 30
                checkable: true
                flat: true
            }


            SkrToolButton {
                id: showWelcomeButton
                icon.color: "transparent"

                Layout.preferredHeight: 30
                Layout.preferredWidth: 30

            }

            Breadcrumb {
                id: breadcrumb
                Layout.preferredHeight: 30
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                Layout.fillWidth: true
            }

            RowLayout{
                Repeater {
                    id: topToolBarRepeater

                }

            }


            Item {
                id: stretcher
                Layout.fillWidth: true

            }

            SkrToolButton {
                id: distractionFreeButton

                Layout.preferredHeight: 30
                Layout.preferredWidth: 30

            }

            SkrToolButton {
                id: themeColorButton

                Layout.preferredHeight: 30
                Layout.preferredWidth: 30

            }

            SkrToolButton {
                id: showRightDockButton

                Layout.preferredHeight: 30
                Layout.preferredWidth: showRightDockButtonWidth

                focusPolicy: Qt.NoFocus
            }

        }


        Item {
            id: baseForDrawers
            Layout.fillHeight: true
            Layout.fillWidth: true

            Item {
                id: columnLayout
                anchors.fill: parent
                anchors.leftMargin: ApplicationWindow.window.compactMode ? 0 : leftDrawer.width
                                                                    * leftDrawer.position
                anchors.rightMargin: ApplicationWindow.window.compactMode ? 0 : rightDrawer.width
                                                                     * rightDrawer.position


//                RowLayout {
//                    id: rowLayout
//                    Layout.fillHeight: true
//                    Layout.fillWidth: true
//                    spacing: 0

//                    Item {
//                        id: leftBase
//                        Layout.preferredWidth: leftBasePreferredWidth
//                        Layout.maximumWidth: leftBasePreferredWidth
//                        visible: !Globals.compactMode
//                        Layout.fillHeight: true


//                    }


                    Item {
                        id: middleBase
                        anchors.fill: parent

                        MultiViewArea{
                            id: multiViewArea

                            anchors.fill: parent
                        }

                    }


//                    Item {
//                        id: rightBase
//                        Layout.preferredWidth: rightBasePreferredWidth
//                        visible: !Globals.compactMode
//                        Layout.fillHeight: true

//                    }


               // }
            }

        }



    }

}
