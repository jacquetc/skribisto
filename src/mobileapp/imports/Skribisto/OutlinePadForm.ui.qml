import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Controls.Material
import Skribisto
import SkrControls

Item {
    id: base
    property alias outlineWritingZone: outlineWritingZone
    property bool minimalMode: false
    property alias openOutlineToolButton: openOutlineToolButton
    property alias addOutlineToolButton: addOutlineToolButton

    implicitHeight: columnLayout.childrenRect.height

    SkrPane {
        id: pane
        clip: true
        anchors.fill: parent
        padding: minimalMode ? 0 : undefined

        Material.background: "transparent"

        ColumnLayout {
            id: columnLayout
            spacing: 0
            anchors.top: parent.top
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.bottom: minimalMode ? parent.bottom : undefined

            SkrToolBar {
                id: toolBar
                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                Layout.alignment: Qt.AlignTop
                Layout.fillWidth: true
                visible: !minimalMode

                RowLayout {
                    id: rowLayout
                    spacing: 1
                    anchors.fill: parent

                    SkrLabel {
                        id: outlineTitleLabel
                        text: qsTr("Outline")
                        elide: Text.ElideRight
                        verticalAlignment: Qt.AlignVCenter
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                    }

                    SkrToolButton {
                        id: openOutlineToolButton
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                    }

                    SkrToolButton {
                        id: addOutlineToolButton
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                    }
                }
            }

            OutlineWritingZone {
                id: outlineWritingZone
                clip: true
                Layout.fillWidth: true
                Layout.preferredHeight: 0
                stretch: true
                placeholderText: qsTr("Type your outline here …")
                leftScrollItemVisible: false
                rightScrollItemVisible: Globals.touchUsed
                visible: !minimalMode

                textPointSize: SkrSettings.outlinePadSettings.textPointSize
                textFontFamily: SkrSettings.outlinePadSettings.textFontFamily
                textIndent: SkrSettings.outlinePadSettings.textIndent
                textTopMargin: SkrSettings.outlinePadSettings.textTopMargin

                textAreaStyleBackgroundColor: SkrTheme.secondaryTextAreaBackground
                textAreaStyleForegroundColor: SkrTheme.secondaryTextAreaForeground
                paneStyleBackgroundColor: SkrTheme.pageBackground
                textAreaStyleAccentColor: SkrTheme.accent

                name: "outlinePad"

                Behavior on Layout.preferredHeight {
                    enabled: SkrSettings.ePaperSettings.animationEnabled
                    SpringAnimation {
                        spring: 5
                        mass: 0.2
                        damping: 0.2
                    }
                }
            }
        }
    }
}
