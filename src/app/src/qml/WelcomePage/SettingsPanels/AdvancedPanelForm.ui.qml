import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../../Items"

SkrSettingsPanel {
    property alias devModeCheckBox: devModeCheckBox
    property alias advancedGroupBox: advancedGroupBox

    ColumnLayout {
        id: pillarLayout
        anchors.fill: parent


        SkrGroupBox {
            id: advancedGroupBox
            width: 200
            height: 200
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignTop
            focusPolicy: Qt.TabFocus
            title: qsTr("Advanced")

            RowLayout {
                id: rowLayout3
                anchors.fill: parent

                ColumnLayout {
                    id: columnLayout7
                    width: 100
                    height: 100

                    SkrSwitch {
                        id: devModeCheckBox
                        text: qsTr("Development mode")
                    }
                }
            }
        }


    }
}
