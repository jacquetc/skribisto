import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Commons"
import "../Items"
import ".."

Item {
    id: base
    width: 1000
    height: 600
    property alias compactLeftDockShowButton: compactLeftDockShowButton
    property alias leftDockMenuGroup: leftDockMenuGroup
    property alias leftDockResizeButton: leftDockResizeButton
    property alias leftDockMenuButton: leftDockMenuButton
    property alias leftDockShowButton: leftDockShowButton
    property alias middleBase: middleBase
    property alias base: base
    property int leftBasePreferredWidth: 0

    readonly property int columnWidth: 550

    ColumnLayout {
        id: columnLayout
        clip: false
        spacing: 0
        anchors.fill: parent
        anchors.leftMargin: Globals.compactMode ? undefined : leftDrawer.width * leftDrawer.position - 10


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
                visible: !Globals.compactMode
                Layout.fillHeight: true

                z: 1

                RowLayout {
                    spacing: 0
                    anchors.fill: parent

                    SkrPane {
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
                            SkrToolButton {
                                id: leftDockShowButton
                                focusPolicy: Qt.NoFocus
                                Layout.preferredHeight: 40
                                Layout.preferredWidth: 40
                                flat: true
                                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                            }

                            SkrToolButton {
                                id: leftDockMenuButton
                                focusPolicy: Qt.NoFocus
                                checkable: true
                                Layout.preferredHeight: 40
                                Layout.preferredWidth: 40
                                flat: true
                                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                            }

                            ColumnLayout {
                                id: leftDockMenuGroup
                                Layout.alignment: Qt.AlignLeft | Qt.AlignTop

                                SkrToolButton {
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

                SkrToolButton {
                    id: compactLeftDockShowButton
                    anchors.left: parent.left
                    anchors.top: parent.top
                    width: 40
                    height: 40

                    z: 1
                    flat: true
                }

                ScrollView {
                    id: scrollView
                    anchors.fill: parent
                    anchors.leftMargin: 50
                    clip: true

                    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                    ScrollBar.vertical.policy: ScrollBar.AsNeeded
                    contentWidth: gridLayout.width
                    contentHeight: gridLayout.implicitHeight

//                    SKRPillarLayout {
//                        id: pillarLayout
//                        width: scrollView.width
//                        columns: ((pillarLayout.width / columnWidth) | 0 )
//                        maxColumns: 2

                    GridLayout{
                        id: gridLayout
                        columns: ((gridLayout.width / columnWidth) | 0 )
                        width: scrollView.width

//                        RecentNotesItem {
//                            id: recentNotesItem
//                            Layout.fillWidth: true
//                        }

                        NoteSearchItem {
                            id: noteSearchItem
                            Layout.fillWidth: true
                            Layout.preferredHeight: scrollView.height
                        }

                        Item {
                            id: placeHolder
                            Layout.fillWidth: true
                        }

                    }
                }

            }
        }
    }
}
