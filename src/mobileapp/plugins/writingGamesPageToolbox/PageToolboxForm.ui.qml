import QtQuick
import QtQml
import QtQuick.Layouts

import SkrControls
import Skribisto


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
                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
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
