import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

Item {
    id: base
    property alias listView: listView
    property alias goUpToolButton: goUpToolButton
    property alias currentParentToolButton: currentParentToolButton
    property alias addToolButton: addToolButton
    property alias treeMenuToolButton: treeMenuToolButton

    Pane {
        id: pane
        clip: true
        anchors.fill: parent
        padding: 0

        ColumnLayout {
            id: columnLayout
            anchors.fill: parent

            RowLayout {
                id: rowLayout
                width: 100
                height: 100
                spacing: 1
                Layout.maximumHeight: 40
                Layout.preferredHeight: 40
                Layout.fillWidth: true

                ToolButton {
                    id: goUpToolButton
                    display: AbstractButton.IconOnly
                }

                ToolButton {
                    id: currentParentToolButton
                    text: qsTr("current folder name")
                }

                Item {
                    id: element
                    width: 200
                    height: 200
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                }

                ToolButton {
                    id: addToolButton
                    text: qsTr("Add a document")
                    display: AbstractButton.IconOnly
                }

                ToolButton {
                    id: treeMenuToolButton
                    text: qsTr("Navigation menu")
                    display: AbstractButton.IconOnly
                }
            }
            ScrollView {
                focusPolicy: Qt.WheelFocus
                Layout.fillWidth: true
                Layout.fillHeight: true
                focus: true

                TreeListView {
                    id: listView
                    anchors.fill: parent
                    focus: true
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:300;width:300}
}
##^##*/

