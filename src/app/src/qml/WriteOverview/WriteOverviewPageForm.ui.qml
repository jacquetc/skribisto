import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
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
    property alias overview: overview
    property alias base: base
    property int leftBasePreferredWidth: 0


    ColumnLayout {
        id: columnLayout
        clip: false
        spacing: 0
        anchors.fill: parent
        anchors.leftMargin: Globals.compactSize ? undefined : leftDrawer.width * leftDrawer.position


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
                                focusPolicy: Qt.NoFocus
                                checkable: true
                                Layout.preferredHeight: 30
                                Layout.preferredWidth: 30
                                hoverEnabled: true
                                flat: true
                                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                            }

                            ColumnLayout {
                                id: leftDockMenuGroup
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

                }
            }

            Item {
                id: middleBase
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                Layout.fillHeight: true
                Layout.fillWidth: true


                Button {
                    id: compactLeftDockShowButton
                    anchors.left: parent.left
                    anchors.top: parent.top
                    width: 30
                    height: 30
                    z: 1
                    flat: true
                }

                Rectangle {
                    id: overview
                    anchors.fill: parent
                    color: "orange"
                }

            }
        }
    }
}
