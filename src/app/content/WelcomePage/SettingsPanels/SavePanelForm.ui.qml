import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts

import SkrControls
import Skribisto

SkrSettingsPanel {

    property alias saveDial: saveDial
    property alias saveSpinBox: saveSpinBox
    property alias saveEveryCheckBox: saveEveryCheckBox

    property alias saveGroupBox: saveGroupBox

    ColumnLayout {
        id: pillarLayout
        anchors.fill: parent

        SkrGroupBox {
            id: saveGroupBox
            Layout.rowSpan: 2
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignTop
            focusPolicy: Qt.TabFocus
            title: qsTr("Save")

            ColumnLayout {
                id: columnLayout2
                anchors.fill: parent

                RowLayout {
                    id: rowLayout2
                    Layout.fillHeight: true
                    Layout.fillWidth: true

                    SkrSwitch {
                        id: saveEveryCheckBox
                        text: qsTr("Save every")

                    }

                    SkrSpinBox {
                        id: saveSpinBox

                    }

                    SkrLabel {
                        id: saveMinutes
                        text: qsTr("minutes")

                    }

                    SkrDial {
                        id: saveDial
                        to: 60
                        from: 1
                        wheelEnabled: true
                    }
                }
            }
        }


    }
}
