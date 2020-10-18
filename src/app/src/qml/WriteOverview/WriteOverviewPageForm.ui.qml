import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ".."

Item {
    id: base
    width: 1000
    height: 600
    property alias topfilteringBannerLabel: topfilteringBannerLabel
    property alias topfilteringBanner: topfilteringBanner
    property alias compactLeftDockShowButton: compactLeftDockShowButton
    property alias compactRightDockShowButton: compactRightDockShowButton
    property alias leftDockMenuGroup: leftDockMenuGroup
    property alias rightDockMenuGroup: rightDockMenuGroup
    property alias leftDockResizeButton: leftDockResizeButton
    property alias rightDockResizeButton: rightDockResizeButton
    property alias leftDockMenuButton: leftDockMenuButton
    property alias rightDockMenuButton: rightDockMenuButton
    property alias leftDockShowButton: leftDockShowButton
    property alias rightDockShowButton: rightDockShowButton
    property alias middleBase: middleBase
    property alias base: base
    property alias sheetOverviewTree: sheetOverviewTree
    property int leftBasePreferredWidth: 0
    property int rightBasePreferredWidth: 0


    ColumnLayout {
        id: columnLayout
        clip: false
        spacing: 0
        anchors.fill: parent
        anchors.leftMargin: Globals.compactSize ? undefined : leftDrawer.width * leftDrawer.position
        anchors.rightMargin: Globals.compactSize ? undefined : rightDrawer.width * rightDrawer.position + 10


        RowLayout {
            id: rowLayout
            Layout.fillHeight: true
            Layout.fillWidth: true
            spacing: 0

            Item {
                id: leftBase
                Layout.preferredWidth: leftBasePreferredWidth
                Layout.maximumWidth: leftBasePreferredWidth
                Layout.minimumWidth: 30
                visible: !Globals.compactSize
                Layout.fillHeight: true

                z: 1

                RowLayout {
                    spacing: 0
                    anchors.fill: parent

                    Pane {
                        id: leftPane
                        Layout.fillHeight: true
                        Layout.fillWidth: true

                        MultiPointTouchArea {
                            id: leftPaneScrollTouchArea
                            z: 1
                            anchors.fill: parent
                            mouseEnabled: false
                            maximumTouchPoints: 1
                            touchPoints: [
                                TouchPoint {
                                    id: leftTouch1
                                }
                            ]
                        }

                        MouseArea {
                            id: leftPaneScrollMouseArea
                            z: 0
                            anchors.fill: parent
                        }

                        ColumnLayout {
                            anchors.fill: parent
                            z: 2
                            ToolButton {
                                id: leftDockShowButton
                                focusPolicy: Qt.NoFocus
                                Layout.preferredHeight: 40
                                Layout.preferredWidth: 40
                                flat: true
                                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                            }

                            ToolButton {
                                id: leftDockMenuButton
                                focusPolicy: Qt.NoFocus
                                checkable: true
                                Layout.preferredHeight: 40
                                Layout.preferredWidth: 40
                                hoverEnabled: true
                                flat: true
                                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                            }

                            ColumnLayout {
                                id: leftDockMenuGroup
                                Layout.alignment: Qt.AlignLeft | Qt.AlignTop

                                ToolButton {
                                    id: leftDockResizeButton
                                    focusPolicy: Qt.NoFocus
                                    Layout.preferredHeight: 40
                                    Layout.preferredWidth: 40
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

                }
            }

            Item {
                id: middleBase
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                Layout.fillHeight: true
                Layout.fillWidth: true

                ColumnLayout {
                    anchors.fill: parent

                    Pane {
                        id: topfilteringBanner
                        Layout.preferredHeight: 50
                        Layout.fillWidth: true

                        RowLayout{
                            anchors.fill: parent
                            Label {
                                id: topfilteringBannerLabel
                                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                            }
                        }

                    }

                    Item {
                        Layout.fillHeight: true
                        Layout.fillWidth: true

                        ToolButton {
                            id: compactLeftDockShowButton
                            anchors.left: parent.left
                            anchors.top: parent.top
                            width: 40
                            height: 40
                            z: 1
                            flat: true
                        }


                        SheetOverviewTree{
                            id: sheetOverviewTree
                            anchors.fill: parent
                            anchors.leftMargin: 50
                            anchors.rightMargin: 50

                        }

                        ToolButton {
                            id: compactRightDockShowButton
                            anchors.right: parent.right
                            anchors.top: parent.top
                            width: 40
                            height: 40
                            z: 1
                            flat: true
                        }
                    }


                }

            }


            Item {
                id: rightBase
                visible: !Globals.compactSize
                Layout.fillHeight: true
                //Layout.fillWidth: true
                Layout.preferredWidth: rightBasePreferredWidth
                Layout.minimumWidth: 30
                //Layout.maximumWidth: 300

                z: 1

                RowLayout {
                    spacing: 0
                    anchors.fill: parent

                    Pane {
                        id: rightPane
                        Layout.fillHeight: true
                        Layout.fillWidth: true

                        MultiPointTouchArea {
                            id: rightPaneScrollTouchArea
                            z: 1
                            anchors.fill: parent
                            mouseEnabled: false
                            maximumTouchPoints: 1
                            touchPoints: [
                                TouchPoint {
                                    id: leftTouch2
                                }
                            ]
                        }

                        MouseArea {
                            id: rightPaneScrollMouseArea
                            z: 0
                            anchors.fill: parent
                        }

                        ColumnLayout {
                            anchors.fill: parent
                            z: 2
                            ToolButton {
                                id: rightDockShowButton
                                focusPolicy: Qt.NoFocus
                                Layout.preferredHeight: 40
                                Layout.preferredWidth: 40
                                flat: true
                                Layout.alignment: Qt.AlignRight | Qt.AlignTop
                            }

                            ToolButton {
                                id: rightDockMenuButton
                                focusPolicy: Qt.NoFocus
                                checkable: true
                                Layout.preferredHeight: 40
                                Layout.preferredWidth: 40
                                flat: true
                                Layout.alignment: Qt.AlignRight | Qt.AlignTop
                            }

                            ColumnLayout {
                                id: rightDockMenuGroup
                                Layout.alignment: Qt.AlignRight | Qt.AlignTop

                                ToolButton {
                                    id: rightDockResizeButton
                                    focusPolicy: Qt.NoFocus
                                    Layout.preferredHeight: 40
                                    Layout.preferredWidth: 40
                                    flat: true
                                    Layout.alignment: Qt.AlignRight | Qt.AlignTop
                                }
                            }
                            Item {
                                id: stretcher2
                                Layout.fillHeight: true
                                Layout.fillWidth: true
                            }
                        }
                    }
                }
            }
        }
    }
}
