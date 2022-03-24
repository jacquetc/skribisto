import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts
import "../../Items"

SkrSettingsPanel {


    property alias langComboBox: langComboBox
    property alias openThemePageButton: openThemePageButton
    property alias appearanceGroupBox: appearanceGroupBox
    property alias zoomSpinBox: zoomSpinBox

    ColumnLayout {
        id: pillarLayout
        anchors.fill: parent


        SkrGroupBox {
            id: appearanceGroupBox
            width: 200
            height: 200
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignTop
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

                    RowLayout {
                        id: rowLayout

                        SkrLabel {
                            text: qsTr("Zoom")
                        }
                    SkrSpinBox {
                        id: zoomSpinBox
                        wheelEnabled: true
                        editable: true

                        to: 300
                        from: 70

                    }
                    }
                }

            }
        }
    }

}
