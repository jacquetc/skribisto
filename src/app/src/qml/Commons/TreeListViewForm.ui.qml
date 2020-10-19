import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.15

Item {
    id: base
    property alias listView: listView
    property alias scrollView: scrollView
    property int scrollBarVerticalPolicy: ScrollBar.AlwaysOff
    property alias goUpToolButton: goUpToolButton
    property alias currentParentToolButton: currentParentToolButton
    property alias addToolButton: addToolButton
    property alias treeMenuToolButton: treeMenuToolButton
    property var toolBarPrimaryColor

    Pane {
        id: pane
        clip: true
        anchors.fill: parent

        ColumnLayout {
            id: columnLayout
            spacing: 0
            anchors.fill: parent

            ToolBar {
                id: toolBar
                Layout.maximumHeight: 40
                Layout.preferredHeight: 40
                Layout.fillWidth: true

                Material.primary: toolBarPrimaryColor

                RowLayout {
                    id: rowLayout
                    spacing: 1
                    anchors.fill: parent

                    ToolButton {
                        id: goUpToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                    }

                    ToolButton {
                        id: currentParentToolButton
                        flat: true
                        Layout.fillWidth: true
                        text: qsTr("current folder name")
                    }
                    ToolButton {
                        id: addToolButton
                        flat: true
                        text: qsTr("Add a document")
                        display: AbstractButton.IconOnly
                    }

                    ToolButton {
                        id: treeMenuToolButton
                        flat: true
                        text: qsTr("Navigation menu")
                        display: AbstractButton.IconOnly
                    }
                }
            }

            ScrollView {
                id: scrollView
                focusPolicy: Qt.StrongFocus
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: scrollBarVerticalPolicy

                ListView {
                    id: listView
                    anchors.fill: parent
                    antialiasing: true
                    clip: true
                    smooth: true
                    boundsBehavior: Flickable.StopAtBounds
                    focus: true

                    Accessible.name: "Navigation list"
                    Accessible.role: Accessible.List
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
