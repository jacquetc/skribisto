import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml 2.15
import "../Items"
import ".."

Item {

    property alias stackView: stackView
    property alias tabBar: tabBar
    property alias helpTabButton: helpTabButton
    property alias faqTabButton: faqTabButton
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
                    text: qsTr("Contents")
                    closable: false
                }
                SkrTabButton {
                    id: faqTabButton
                    text: qsTr("FAQ")
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
