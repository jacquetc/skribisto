import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
    id: base



    property alias listView: listView
    property alias scrollView: scrollView
    property int scrollBarVerticalPolicy: ScrollBar.AlwaysOff
    property alias goBackToolButton: goBackToolButton
    property alias restoreToolButton: restoreToolButton
    property alias listMenuToolButton: listMenuToolButton
    property alias selectAllToolButton: selectAllToolButton


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
                        id: goBackToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                    }


                    ToolButton {
                        id: restoreToolButton
                        flat: true
                        text: qsTr("Restore a document")
                        display: AbstractButton.IconOnly
                    }

                    ToolButton {
                        id: selectAllToolButton
                        flat: true
                        text: qsTr("Select all")
                        display: AbstractButton.IconOnly
                    }

                    ToolButton {
                        id: listMenuToolButton
                        flat: true
                        text: qsTr("Trashed menu")
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


                CheckableTree {
                    id: listView
                    anchors.fill: parent
                    antialiasing: true
                    clip: true
                    smooth: true
                    boundsBehavior: Flickable.StopAtBounds


                    treelikeIndentsVisible: true
                    checkButtonsVisible: true
                    openActionsEnabled: true
                    renameActionEnabled: true
                    copyActionEnabled: true
                    deleteActionEnabled: true


                }


            }
        }
    }
}
