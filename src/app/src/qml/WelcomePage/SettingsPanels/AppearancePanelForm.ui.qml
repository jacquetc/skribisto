import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../../Items"

Item {
    width: 400
    height: 400

    property alias goBackButton: goBackButton

    property alias langComboBox: langComboBox
    property alias openThemePageButton: openThemePageButton
    property alias appearanceGroupBox: appearanceGroupBox

    property alias showMinimapCheckBox: showMinimapCheckBox
    property alias minimapDividerSpinBox: minimapDividerSpinBox
    property alias minimapDividerDial: minimapDividerDial

    ColumnLayout {
        id: pillarLayout
        anchors.fill: parent

        SkrToolButton{
            id: goBackButton
            icon.source: "qrc:///icons/backup/go-previous.svg"
            text: qsTr("Go back")
            Layout.alignment: Qt.AlignLeft
        }



        SkrGroupBox {
            id: appearanceGroupBox
            width: 200
            height: 200
            Layout.fillWidth: true
            focusPolicy: Qt.TabFocus
            title: qsTr("Appearance")

            ColumnLayout {
                id: columnLayout5
                anchors.fill: parent

                ColumnLayout {
                    id: rowLayout4

                    SkrLabel {
                        id: langLabel
                        text: qsTr("Interface language :")
                    }

                    SkrComboBox {
                        id: langComboBox
                        wheelEnabled: true
                    }
                    SkrButton {
                        id: openThemePageButton
                        text: qsTr("Manage themes")
                    }
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
