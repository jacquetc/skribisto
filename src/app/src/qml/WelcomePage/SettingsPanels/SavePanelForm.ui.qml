import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../../Items"

Item {
    width: 400
    height: 400
    property alias goBackButton: goBackButton


    property alias saveDial: saveDial
    property alias saveSpinBox: saveSpinBox
    property alias saveEveryCheckBox: saveEveryCheckBox

    property alias saveGroupBox: saveGroupBox

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
            id: saveGroupBox
            Layout.rowSpan: 2
            Layout.fillWidth: true
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
