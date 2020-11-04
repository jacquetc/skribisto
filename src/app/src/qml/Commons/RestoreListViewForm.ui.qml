import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import ".."

Item {
    id: base

    property alias listView: listView
    property alias scrollView: scrollView
    property int scrollBarVerticalPolicy: ScrollBar.AlwaysOff
    property alias goBackToolButton: goBackToolButton
    property alias restoreToolButton: restoreToolButton
    property alias listMenuToolButton: listMenuToolButton
    property alias selectAllToolButton: selectAllToolButton
    property var toolBarPrimaryColor

    SkrPane {
        id: pane
        clip: true
        anchors.fill: parent

        ColumnLayout {
            id: columnLayout
            spacing: 0
            anchors.fill: parent

            SkrToolBar {
                id: toolBar
                Layout.maximumHeight: 40
                Layout.preferredHeight: 40
                Layout.fillWidth: true

                RowLayout {
                    id: rowLayout
                    spacing: 1
                    anchors.fill: parent

                    SkrToolButton {
                        id: goBackToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                    }

                    SkrToolButton {
                        id: restoreToolButton
                        flat: true
                        text: qsTr("Restore a document")
                        display: AbstractButton.IconOnly
                    }

                    SkrToolButton {
                        id: selectAllToolButton
                        flat: true
                        text: qsTr("Select all")
                        display: AbstractButton.IconOnly
                    }

                    SkrToolButton {
                        id: listMenuToolButton
                        flat: true
                        text: qsTr("Trashed menu")
                        display: AbstractButton.IconOnly
                    }
                }
            }

            SkrLabel {
                id: text1
                text: qsTr("The checked documents are those which were trashed at the same time")
                font.pixelSize: 12
                wrapMode: Text.WordWrap
                Layout.fillWidth: true
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
                    smooth: true
                    boundsBehavior: Flickable.StopAtBounds

                    treelikeIndentsVisible: true
                    checkButtonsVisible: true
                    openActionsEnabled: true
                    renameActionEnabled: true
                    copyActionEnabled: true
                    deleteActionEnabled: true

                    Accessible.name: "List of trashed items to be restored"
                    Accessible.role: Accessible.List
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:600;width:400}
}
##^##*/

