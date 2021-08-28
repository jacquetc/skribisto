import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../../Items"

Item {
    width: 400
    height: 400
    property alias goBackButton: goBackButton

    property alias accessibilityGroupBox: accessibilityGroupBox
    property alias accessibilityCheckBox: accessibilityCheckBox

    property alias showMenuButtonCheckBox: showMenuButtonCheckBox

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
            id: accessibilityGroupBox
            Layout.fillWidth: true
            focusPolicy: Qt.TabFocus

            title: qsTr("Accessibility")

            ColumnLayout {
                id: columnLayout3
                anchors.fill: parent

                SkrSwitch {
                    id: accessibilityCheckBox
                    text: qsTr("Help with accessibility")

                }

                SkrSwitch {
                    id: showMenuButtonCheckBox
                    text: qsTr("Show menu button")

                }
            }
        }
    }


}
