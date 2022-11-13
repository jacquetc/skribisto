import QtQuick
import QtQml
import QtQuick.Layouts
import QtQuick.Controls

import SkrControls
import Skribisto


SkrBasePage {
    id: base
    width: 1000
    height: 600

    property alias viewButtons: viewButtons
    property alias pageMenuToolButton: pageMenuToolButton
    property alias titleLabel: titleLabel
    property alias countLabel: countLabel
    property alias relationshipPanel: relationshipPanel
    property int relationshipPanelPreferredHeight: 200
    readonly property int columnWidth: 550

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
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom

                    }

                    SkrLabel {
                        id: countLabel
                        verticalAlignment: Qt.AlignVCenter
                        horizontalAlignment: Qt.AlignHCenter
                        Layout.alignment: Qt.AlignCenter
                    }

                    SkrViewButtons {
                        id: viewButtons
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                    }
                }
            }

            ScrollView {
                id: scrollView
                clip: true
                Layout.fillWidth: true
                Layout.fillHeight: true

                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded
                contentWidth: pillarLayout.width
                contentHeight: pillarLayout.implicitHeight

                SKRPillarLayout {
                    id: pillarLayout
                    width: scrollView.width
                    columns: ((pillarLayout.width / columnWidth) | 0 )
                    maxColumns: 3



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
}
