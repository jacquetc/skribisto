import QtQuick 2.15
import QtQml 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import eu.skribisto.searchtreelistproxymodel 1.0

import "../../Items"
import "../../Commons"
import "../.."

SkrToolbox {
    id: base
    width: 1000
    height: 600

    property alias navigationProxyModel: navigationProxyModel
    property alias listView: listView

    clip: true

    ColumnLayout{
        anchors.fill: parent

            SkrToolBar {
                id: toolBar
                Layout.maximumHeight: 40
                Layout.preferredHeight: 40
                Layout.alignment: Qt.AlignTop
                Layout.fillWidth: true

                RowLayout {
                    id: rowLayout
                    spacing: 1
                    anchors.fill: parent


                    SkrLabel {
                        id: titleLabel
                        text: qsTr("Favorites")
                        verticalAlignment: Qt.AlignVCenter
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                    }
                }



            }


            ScrollView {
                id: scrollView
                clip: true
                Layout.fillWidth: true
                Layout.preferredHeight: 500
                focusPolicy: Qt.StrongFocus
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded//scrollBarVerticalPolicy





                ListView {
                    id: listView
                    smooth: true
                    boundsBehavior: Flickable.StopAtBounds
                    spacing: 1

                    Accessible.name: qsTr("Favorites list")
                    Accessible.role: Accessible.List

                    model: SKRSearchTreeListProxyModel {
                        id: navigationProxyModel
                        showTrashedFilter: false
                        showNotTrashedFilter: true
                        showOnlyWithPropertiesFilter: ["favorite"]
                    }




                }
            }



    }


}
