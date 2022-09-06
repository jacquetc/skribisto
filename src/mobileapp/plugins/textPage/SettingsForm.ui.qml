import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts

import SkrControls
import Skribisto


SkrSettingsPanel {
    property alias textGroupBox: textGroupBox

    property alias centerTextCursorSwitch: centerTextCursorSwitch

    property alias showMinimapCheckBox: showMinimapCheckBox
    property alias minimapDividerSpinBox: minimapDividerSpinBox
    property alias minimapDividerDial: minimapDividerDial
    property alias textWidthLabel: textWidthLabel
    property alias textWidthSlider: textWidthSlider
    property alias textPointSizeSlider: textPointSizeSlider
    property alias fontFamilyComboBox: fontFamilyComboBox
    property alias textTopMarginSlider: textTopMarginSlider
    property alias textIndentSlider: textIndentSlider

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


                SkrLabel {
                    id: textWidthLabel
                    text: qsTr("Text width :")
                }

                SkrSlider {
                    id: textWidthSlider
                    from: 400
                    to: rootWindow.width
                    Layout.fillWidth: true
                }

                SkrLabel {
                    text: qsTr("Text size :")
                }

                SkrSlider {
                    id: textPointSizeSlider
                    stepSize: 1
                    from: 8
                    to: 40
                    Layout.fillWidth: true
                }

                SkrComboBox {
                    id: fontFamilyComboBox
                    wheelEnabled: true
                    Layout.fillWidth: true
                }
                SkrLabel {
                    text: qsTr("Text indent :")
                }

                SkrSlider {
                    id: textIndentSlider
                    stepSize: 1
                    from: 0
                    to: 200
                    Layout.fillWidth: true
                }

                SkrLabel {
                    text: qsTr("Top margin :")
                }

                SkrSlider {
                    id: textTopMarginSlider
                    stepSize: 1
                    from: 0
                    to: 30
                    Layout.fillWidth: true
                }

            }
        }




    }
}
