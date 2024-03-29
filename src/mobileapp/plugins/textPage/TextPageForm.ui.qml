import QtQuick
import QtQml
import QtQuick.Layouts

import SkrControls
import Skribisto
import theme

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

    property alias countLabel: countLabel
    property alias relationshipPanel: relationshipPanel
    property int relationshipPanelPreferredHeight: 200

    property alias zoomWheelHandler: zoomWheelHandler

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
                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom

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
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
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

                        WheelHandler {
                            id: zoomWheelHandler
                            acceptedModifiers: Qt.ControlModifier
                            target: SkrSettings.textSettings
                            property: "textPointSize"
                            grabPermissions: PointerHandler.CanTakeOverFromAnything
                            rotationScale: 0.01
                        }
                    }
                }

                Loader {
                    id: minimapLoader
                    asynchronous: true

                    anchors.top: parent.top
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                    anchors.bottomMargin: writingZone.findPanel.visible ? writingZone.findPanel.height : 0
                    width: Qt.isQtObject(
                               minimapLoader.item) ? minimapLoader.item.preferredWidth : -1
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
