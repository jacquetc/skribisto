import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SkrControls
import Skribisto

Item {
    property alias importerButtonRepeater: importerButtonRepeater

    property alias stackView: stackView

    StackView {
        id: stackView
        anchors.fill: parent
        clip: true
        initialItem: Item {
            id: importerMainPage

            ColumnLayout {
                id: columnLayout6
                anchors.fill: parent

                RowLayout {
                    id: rowLayout7
                    Layout.fillWidth: true

                    SkrLabel {
                        id: titleLabel2
                        text: qsTr("<h2>Import</h2>")
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        Layout.fillWidth: true
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    }
                }

                Repeater {
                    id: importerButtonRepeater
                    Layout.minimumHeight: 100
                    Layout.minimumWidth: 200
                    Layout.fillWidth: true
                    Layout.maximumWidth: 500
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
                }

                Item {
                    id: stretcher
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/
