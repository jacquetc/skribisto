import QtQuick
import QtQuick.Controls
import QtQml
import QtQuick.Layouts

import "../../../../Items"
import "../../../../Commons"
import "../../../.."

SkrBasePage {
    id: base

    width: 1000
    height: 600
    property alias base: base
    property alias cardViewGrid: cardViewGrid
    property alias currentParentId : cardViewGrid.currentParentId
    property alias titleLabel: titleLabel
    property alias goUpToolButton: goUpToolButton
    property alias viewButtons: viewButtons
    property alias addItemToolButton: addItemToolButton

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        //-------------------------------------------------
        //--- Tool bar  ----------------------------------
        //-------------------------------------------------
        SkrPageToolBar {
            Layout.fillWidth: true
            Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom

            RowLayout {
                anchors.fill: parent

                SkrToolButton {
                    id: goUpToolButton
                    text: qsTr("Go up")
                    icon.source: "qrc:///icons/backup/go-parent-folder.svg"
                    Layout.alignment: Qt.AlignCenter
                    Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                    Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                }

                SkrLabel {
                    id: titleLabel
                    verticalAlignment: Qt.AlignVCenter
                    horizontalAlignment: Qt.AlignHCenter
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                }

                SkrToolButton {
                    id: addItemToolButton
                    text: qsTr("Add an item")
                    icon.source: "qrc:///icons/backup/list-add.svg"
                    Layout.alignment: Qt.AlignCenter
                    Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                    Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom

                }
                //                    SkrLabel {
                //                        id: countLabel
                //                        verticalAlignment: Qt.AlignVCenter
                //                        horizontalAlignment: Qt.AlignHCenter
                //                        Layout.alignment: Qt.AlignCenter
                //                    }
                SkrViewButtons {
                    id: viewButtons
                    Layout.fillHeight: true
                }
            }
        }

        CardViewGrid {
            id: cardViewGrid

            Layout.fillHeight: true
            Layout.fillWidth: true
        }

    }
}
