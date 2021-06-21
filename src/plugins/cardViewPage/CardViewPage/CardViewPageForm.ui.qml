﻿import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml 2.15
import QtQuick.Layouts 1.15

import "../../Items"
import "../../Commons"
import "../.."

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

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        //-------------------------------------------------
        //--- Tool bar  ----------------------------------
        //-------------------------------------------------
        SkrPageToolBar {
            Layout.fillWidth: true
            Layout.preferredHeight: 30

            RowLayout {
                anchors.fill: parent

                SkrToolButton {
                    id: goUpToolButton
                    text: qsTr("Go up")
                    icon.source: "qrc:///icons/backup/go-parent-folder.svg"
                    Layout.alignment: Qt.AlignCenter
                    Layout.preferredHeight: 30
                }

                SkrLabel {
                    id: titleLabel
                    verticalAlignment: Qt.AlignVCenter
                    horizontalAlignment: Qt.AlignHCenter
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                }

                //                    SkrToolButton {
                //                        id: pageMenuToolButton
                //                        text: qsTr("Page menu")
                //                        icon.source: "qrc:///icons/backup/overflow-menu.svg"
                //                        Layout.alignment: Qt.AlignCenter
                //                        Layout.preferredHeight: 30
                //                    }
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
