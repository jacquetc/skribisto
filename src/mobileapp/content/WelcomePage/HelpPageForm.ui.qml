import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQml

import SkrControls
import Skribisto

Item {

    property alias stackView: stackView
    property alias tabBar: tabBar
    property alias helpTabButton: helpTabButton
    property alias aboutTabButton: aboutTabButton
    property alias aboutQtTabButton: aboutQtTabButton

    SkrPane {
        id: pane1
        anchors.fill: parent

        ColumnLayout {
            anchors.fill: parent

            SkrTabBar {
                id: tabBar
                Layout.fillWidth: true

                SkrTabButton {
                    id: helpTabButton
                    text: qsTr("Help")
                    closable: false
                }
                SkrTabButton {
                    id: aboutTabButton
                    text: qsTr("About")
                    closable: false
                }
                SkrTabButton {
                    id: aboutQtTabButton
                    text: qsTr("About Qt")
                    closable: false
                }
            }

            StackView {
                id: stackView
                Layout.fillWidth: true
                Layout.fillHeight: true

                clip: true
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

