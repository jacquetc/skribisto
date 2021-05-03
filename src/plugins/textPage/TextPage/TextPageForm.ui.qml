import QtQuick 2.15
import QtQml 2.15
import QtQuick.Layouts 1.15

import "../../Items"
import "../../Commons"
import "../.."

SkrBasePage {
    id: base
    width: 1000
    height: 600

    property alias writingZone: writingZone
    property alias loader_previousWritingZone: loader_previousWritingZone
    property alias minimap: minimap
    property alias rightPaneScrollMouseArea: rightPaneScrollMouseArea
    property alias rightPaneScrollTouchArea: rightPaneScrollTouchArea
    property alias leftPaneScrollMouseArea: leftPaneScrollMouseArea
    property alias leftPaneScrollTouchArea: leftPaneScrollTouchArea
    property alias middleBase: middleBase
    property alias viewButtons: viewButtons
    property alias titleLabel: titleLabel

    property alias countLabel: countLabel


    clip: true



    ColumnLayout {
        id: columnLayout
        spacing: 0
        anchors.fill: parent


        //-------------------------------------------------
        //--- Tool bar  ----------------------------------
        //-------------------------------------------------

        SkrPageToolBar {
            Layout.fillWidth: true
            Layout.preferredHeight: 30


            RowLayout {
                anchors.fill: parent

                SkrLabel{
                    id: titleLabel
                    verticalAlignment: Qt.AlignVCenter
                    horizontalAlignment: Qt.AlignHCenter
                    Layout.fillWidth: true
                    Layout.fillHeight: true

                }

                SkrLabel{
                    id: countLabel
                    verticalAlignment: Qt.AlignVCenter
                    horizontalAlignment: Qt.AlignHCenter
                }

                SkrViewButtons {
                    id: viewButtons
                    Layout.fillHeight: true
                }
            }

        }


        //-------------------------------------------------
        //-------------------------------------------------
        //-------------------------------------------------

        RowLayout {
            id: rowLayout
            Layout.fillHeight: true
            Layout.fillWidth: true
            spacing: 0

            Item {
                id: leftBase
                //                Layout.preferredWidth: leftBasePreferredWidth
                //                Layout.maximumWidth: leftBasePreferredWidth
                Layout.minimumWidth: 20
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

                    Loader {
                        id: loader_previousWritingZone

                        Layout.fillWidth: true
                        Layout.preferredHeight: active ? 200 : 0
                        sourceComponent: component_previousWritingZone
                        active: false
                    }


                    WritingZone {
                        id: writingZone
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                        textAreaStyleElevation: true
                        minimalTextAreaWidth: 100
                        textCenteringEnabled: SkrSettings.behaviorSettings.centerTextCursor

                        textAreaStyleBackgroundColor: SkrTheme.mainTextAreaBackground
                        textAreaStyleForegroundColor: SkrTheme.mainTextAreaForeground
                        textAreaStyleAccentColor: SkrTheme.accent
                        paneStyleBackgroundColor: SkrTheme.pageBackground

                    }

                }


            }


            Item {
                id: rightBase
                visible: !Globals.compactMode
                Layout.fillHeight: true
                //                Layout.preferredWidth: rightBasePreferredWidth
                Layout.minimumWidth: 20

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
