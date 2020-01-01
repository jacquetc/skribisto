import QtQuick 2.4

import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

Item {
    id: baseDelegateItem

    width: listView.width
    height: 50
    property alias menuButton: menuButton
    property alias goToChildButton: goToChildButton
    property alias baseDelegateItem: baseDelegateItem

    RowLayout {
        id: rowLayout
        spacing: 2
        anchors.fill: parent

        Rectangle {
            color: "transparent"
            border.width: 1
            Layout.fillWidth: true
            Layout.fillHeight: true
            ColumnLayout {
                id: columnLayout2
                anchors.fill: parent
                Layout.fillHeight: true
                Layout.fillWidth: true

                Label {
                    id: titleLabel

                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter

                    text: model.name
                }
                Label {
                    id: tagLabel

                    text: model.tag
                    Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                }
            }
            MouseArea {
                anchors.fill: parent
            }
        }
        ToolButton {
            id: menuButton
            Layout.fillHeight: true
            Layout.preferredWidth: 30

            text: "..."
            flat: true
        }

        ToolButton {
            id: goToChildButton

            //                            background: Rectangle {
            //                                implicitWidth: 30
            //                                implicitHeight: 30
            //                                color: Qt.darker(
            //                                           "#33333333", control.enabled
            //                                           && (control.checked
            //                                               || control.highlighted) ? 1.5 : 1.0)
            //                                opacity: enabled ? 1 : 0.3
            //                                visible: control.down
            //                                         || (control.enabled
            //                                             && (control.checked
            //                                                 || control.highlighted))
            //                            }
            text: "+"
            flat: false
            Layout.preferredWidth: 30
            Layout.fillHeight: true
            Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:300}
}
##^##*/

