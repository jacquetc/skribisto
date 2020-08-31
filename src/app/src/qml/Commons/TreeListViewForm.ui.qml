import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

Item {
    id: base
    property alias listView: listView
    property alias scrollView: scrollView
    property int scrollBarVerticalPolicy: ScrollBar.AlwaysOff
    property alias goUpToolButton: goUpToolButton
    property alias currentParentToolButton: currentParentToolButton
    property alias addToolButton: addToolButton
    property alias treeMenuToolButton: treeMenuToolButton


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



                //                Item {
                //                    id: element
                //                    Layout.fillHeight: true
                //                    Layout.fillWidth: true
                //                }


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


                }


            }
        }
    }

}
