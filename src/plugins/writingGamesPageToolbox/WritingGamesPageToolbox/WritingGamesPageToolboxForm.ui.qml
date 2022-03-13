import QtQuick
import QtQml
import QtQuick.Layouts

import "../../Items"
import "../../Commons"
import "../.."

SkrToolbox {
    id: base
    width: 1000
    height: 600

    clip: true

    property alias forbidErasingSwitch: forbidErasingSwitch

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
                        text: qsTr("Writing games")
                        verticalAlignment: Qt.AlignVCenter
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                    }
                }



            }
    SkrSwitch{
        id: forbidErasingSwitch

        text: qsTr("Forbid erasing")
        checked: false


    }

    }


}
