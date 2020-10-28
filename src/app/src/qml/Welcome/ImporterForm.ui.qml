import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import "../Commons"
import ".."

Item {
    property alias goBackToolButton: goBackToolButton
    property alias importFromPlumeToolButton: importFromPlumeToolButton
    property alias stackView: stackView

    StackView  {
        id: stackView
        anchors.fill: parent
        clip: true
        initialItem: Item {
            id : importerMainPage


            ColumnLayout {
                id: columnLayout6
                anchors.fill: parent

                RowLayout {
                    id: rowLayout7
                    Layout.fillWidth: true

                    SkrToolButton {
                        id: goBackToolButton
                        text: qsTr("Go back")
                    }

                    SkrLabel {
                        id: titleLabel2
                        text: qsTr("<h2>Import</h2>")
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        Layout.fillWidth: true
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    }
                }



                SkrButton {
                    id: importFromPlumeToolButton
                    text: qsTr("Import from Plume Creator project")

                    icon.height: 90
                    icon.width: 90

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
