import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Commons"
import ".."
import "../Items"

Item {
    id: base
    width: 1000
    height: 600
    property alias rightPaneScrollMouseArea: rightPaneScrollMouseArea
    property alias rightPaneScrollTouchArea: rightPaneScrollTouchArea
    property alias leftDockMenuGroup: leftDockMenuGroup
    property alias compactRightDockShowButton: compactRightDockShowButton
    property alias compactLeftDockShowButton: compactLeftDockShowButton
    property alias rightDockMenuGroup: rightDockMenuGroup
    property alias leftDockResizeButton: leftDockResizeButton
    property alias rightDockResizeButton: rightDockResizeButton
    property alias leftDockMenuButton: leftDockMenuButton
    property alias rightDockMenuButton: rightDockMenuButton
    property alias leftDockShowButton: leftDockShowButton
    property alias rightDockShowButton: rightDockShowButton
    property alias minimap: minimap
    property alias middleBase: middleBase
    property alias writingZone: writingZone
    property alias base: base
    property alias leftPaneScrollMouseArea: leftPaneScrollMouseArea
    property alias leftPaneScrollTouchArea: leftPaneScrollTouchArea
    property int leftBasePreferredWidth: 0
    property int rightBasePreferredWidth: 0

    ColumnLayout {
        id: columnLayout
        clip: false
        spacing: 0
        anchors.fill: parent
        anchors.leftMargin: Globals.compactSize ? undefined : leftDrawer.width
                                                  * leftDrawer.position + 10
        anchors.rightMargin: Globals.compactSize ? undefined : rightDrawer.width
                                                   * rightDrawer.position + 10

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

                WritingZone {
                    id: writingZone
                    anchors.fill: parent
                    textAreaStyleElevation: true
                    minimalTextAreaWidth: 100

                    textAreaStyleBackgroundColor: SkrTheme.mainTextAreaBackground
                    textAreaStyleForegroundColor: SkrTheme.mainTextAreaForeground
                    textAreaStyleAccentColor: SkrTheme.accent
                    paneStyleBackgroundColor: SkrTheme.pageBackground

                }

                SkrToolButton {
                    id: compactRightDockShowButton
                    anchors.right: parent.right
                    anchors.top: parent.top
                    width: 40
                    height: 40
                    z: 1
                    flat: true
                }
            }

            Item {
                id: rightBase
                visible: !Globals.compactSize
                Layout.fillHeight: true
                Layout.preferredWidth: rightBasePreferredWidth
                Layout.minimumWidth: 30

                //Layout.maximumWidth: 300
                z: 1

                RowLayout {
                    spacing: 0
                    anchors.fill: parent

                    SkrPane {
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

                            SkrToolButton {
                                id: rightDockShowButton
                                focusPolicy: Qt.NoFocus
                                Layout.preferredHeight: 40
                                Layout.preferredWidth: 40
                                flat: true
                                Layout.alignment: Qt.AlignRight | Qt.AlignTop
                            }

                            SkrToolButton {
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

                                SkrToolButton {
                                    id: rightDockResizeButton
                                    focusPolicy: Qt.NoFocus
                                    Layout.preferredHeight: 40
                                    Layout.preferredWidth: 40
                                    flat: true
                                    Layout.alignment: Qt.AlignRight | Qt.AlignTop
                                }

                                SkrToolButton {
                                    id: showMinimapButton
                                    visible: false
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
                    Minimap {
                        Layout.fillHeight: true
                        Layout.alignment: Qt.AlignRight | Qt.AlignVCenter

                        id: minimap
                        //width: minimapFlickable.width
                        //pageSize: (writingZone.flickable.height) / (writingZone.textArea.contentHeight + 16)
                        //textScale: minimapFlickable.height / writingZone.flickable.contentHeight
                        sourceWidth: writingZone.textArea.width
                        sourcePointSize: writingZone.textArea.font.pointSize
                    }
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;width:1300}
}
##^##*/

