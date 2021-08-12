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
    property alias minimapLoader: minimapLoader
    property alias middleBase: middleBase
    property alias viewButtons: viewButtons
    property alias pageMenuToolButton: pageMenuToolButton
    property alias titleLabel: titleLabel
    property alias leftScrollFlickable: leftScrollFlickable
    property alias rightScrollFlickable: rightScrollFlickable


    property alias countLabel: countLabel
    property alias relationshipPanel: relationshipPanel
    property int relationshipPanelPreferredHeight: 200

    clip: true

    ColumnLayout {
        id: rowLayout
        spacing: 0
        anchors.fill: parent

        ColumnLayout {
            id: columnLayout
            spacing: 0
            Layout.fillWidth: true
            Layout.fillHeight: true

            //-------------------------------------------------
            //--- Tool bar  ----------------------------------
            //-------------------------------------------------
            SkrPageToolBar {
                Layout.fillWidth: true
                Layout.preferredHeight: 30

                RowLayout {
                    anchors.fill: parent

                    SkrLabel {
                        id: titleLabel
                        verticalAlignment: Qt.AlignVCenter
                        horizontalAlignment: Qt.AlignHCenter
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                    }

                    SkrToolButton {
                        id: pageMenuToolButton
                        text: qsTr("Page menu")
                        icon.source: "qrc:///icons/backup/overflow-menu.svg"
                        Layout.alignment: Qt.AlignCenter
                        Layout.preferredHeight: 30
                    }

                    SkrLabel {
                        id: countLabel
                        verticalAlignment: Qt.AlignVCenter
                        horizontalAlignment: Qt.AlignHCenter
                        Layout.alignment: Qt.AlignCenter
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
                id: rowLayout2
                Layout.fillHeight: true
                Layout.fillWidth: true
                spacing: 0

                Item {
                    id: leftBase
                    //color: "blue"
                    //                Layout.preferredWidth: leftBasePreferredWidth
                    //                Layout.maximumWidth: leftBasePreferredWidth
                    //Layout.minimumWidth: rightBase.Layout.minimumWidth
                    //visible: !Globals.compactMode
                    Layout.fillHeight: true
                    Layout.minimumWidth:  viewManager.rootWindow.compactMode ? 0 : rightBaseLayout.childrenRect.width

                    z: 1


                    SkrFlickable{
                        id: leftScrollFlickable
                        anchors.fill: parent
                        flickableDirection: Qt.Vertical
                        //contentHeight: writingZone.textArea.height


                        contentHeight: writingZone.flickable.contentHeight
                        contentWidth: width

                        Binding {
                            target: leftScrollFlickable.contentItem
                            property: "height"
                            value: writingZone.flickable.contentHeight
                            restoreMode: Binding.RestoreNone
                        }


                        Binding {
                            target: leftScrollFlickable.contentItem
                            property: "width"
                            value: writingZone.flickable.contentWidth
                            restoreMode: Binding.RestoreNone
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
                    //visible: !Globals.compactMode
                    Layout.fillHeight: true
                    //                Layout.preferredWidth: rightBasePreferredWidth
                    Layout.minimumWidth: rightBaseLayout.childrenRect.width

                    //Layout.maximumWidth: 300
                    z: 1

                    SkrFlickable{
                        id: rightScrollFlickable
                        anchors.fill: parent
                        anchors.rightMargin: rightBaseLayout.width
                        flickableDirection: Qt.Vertical
                        contentHeight: writingZone.flickable.contentHeight
                        contentWidth: width
                    }

                    RowLayout {
                        id: rightBaseLayout
                        spacing: 0

                        anchors.top: parent.top
                        anchors.bottom: parent.bottom
                        anchors.left: parent.left
                        //width: childrenRect.width
                        Loader {
                            id: minimapLoader
                            asynchronous : true

                            Layout.fillHeight: true
                            Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                            Layout.preferredWidth: Qt.isQtObject(minimapLoader.item) ? minimapLoader.item.preferredWidth : -1


                        }

                    }
                }


            }
        }

        RelationshipPanel {
            id: relationshipPanel
            Layout.preferredHeight: relationshipPanelPreferredHeight
            Layout.fillWidth: true
            visible: false
        }
    }
}
