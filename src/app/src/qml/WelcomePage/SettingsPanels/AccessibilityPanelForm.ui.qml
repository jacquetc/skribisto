import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts
import "../../Items"

SkrSettingsPanel{

    property alias accessibilityGroupBox: accessibilityGroupBox
    property alias accessibilityCheckBox: accessibilityCheckBox

    property alias showMenuButtonCheckBox: showMenuButtonCheckBox

    ColumnLayout {
        id: pillarLayout
        anchors.fill: parent



        SkrGroupBox {
            id: accessibilityGroupBox
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignTop
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
