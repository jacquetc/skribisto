import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts

import "../../Items"
import "../../Commons"
import "../.."

SkrSettingsPanel {
    property alias textGroupBox: textGroupBox

    property alias centerTextCursorSwitch: centerTextCursorSwitch

    property alias showMinimapCheckBox: showMinimapCheckBox
    property alias minimapDividerSpinBox: minimapDividerSpinBox
    property alias minimapDividerDial: minimapDividerDial

    ColumnLayout {
        id: pillarLayout
        anchors.fill: parent



        SkrGroupBox {
            id: textGroupBox
            focusPolicy: Qt.TabFocus
            Layout.fillWidth: true
            title: qsTr("Text")

            ColumnLayout {
                id: columnLayout10
                anchors.fill: parent


                SkrSwitch {
                    id: centerTextCursorSwitch
                    text: qsTr("Center vertically the text cursor")
                }
            }
        }


        SkrGroupBox {
            id: minimapGroupBox
            Layout.rowSpan: 2
            Layout.fillWidth: true
            focusPolicy: Qt.TabFocus
            title: qsTr("Minimap scrollbar")

            ColumnLayout {
                anchors.fill: parent

                SkrSwitch {
                    id: showMinimapCheckBox
                    text: qsTr("Show the minimap scrollbar")

                }

                SkrLabel {
                    text: qsTr("Scale: 1/%1").arg(minimapDividerSpinBox.value)

                }
                RowLayout {
                    Layout.fillHeight: true
                    Layout.fillWidth: true


                    SkrLabel {
                        text: qsTr("Set scale:")
                    }

                    SkrSpinBox {
                        id: minimapDividerSpinBox
                        to: 10
                        from: 3

                    }

                    SkrDial {
                        id: minimapDividerDial
                        to: 10
                        from: 3
                        wheelEnabled: true
                    }
                }
            }
        }




    }
}
