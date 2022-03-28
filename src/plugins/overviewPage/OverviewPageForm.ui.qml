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
    property alias unsetFilteringParentToolButton: unsetFilteringParentToolButton
    property alias topFilteringBanner: topFilteringBanner
    property alias topFilteringBannerLabel: topFilteringBannerLabel
    property alias base: base
    property alias overviewTree: overviewTree
    property alias titleLabel: titleLabel
    property alias viewButtons: viewButtons

    ColumnLayout {
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

            SkrPane {
                id: topFilteringBanner
                Layout.preferredHeight: 50
                Layout.fillWidth: true

                RowLayout {
                    anchors.fill: parent

                    RowLayout {
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                        SkrLabel {
                            id: topFilteringBannerLabel
                        }

                        SkrToolButton {
                            id: unsetFilteringParentToolButton
                            text: qsTr("Unfocus")
                            display: AbstractButton.IconOnly
                        }
                    }
                }
            }
            OverviewTree {
                id: overviewTree

                viewManager: base.viewManager

                Layout.fillHeight: true
                Layout.fillWidth: true
            }
        }
    }
}
