import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../Commons"
import "../../Items"
import "../.."

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
                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                Layout.fillWidth: true

                RowLayout {
                    id: rowLayout
                    spacing: 1
                    anchors.fill: parent

                    SkrToolButton {
                        id: goBackToolButton
                        flat: true
                        display: AbstractButton.IconOnly                
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                    }

                    Item {
                        id: stretcher2
                        Layout.fillWidth: true
                        Layout.fillHeight: true

                    }

                    SkrToolButton {
                        id: restoreToolButton
                        flat: true
                        text: qsTr("Restore a document")
                        display: AbstractButton.IconOnly
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                    }

                    SkrToolButton {
                        id: selectAllToolButton
                        flat: true
                        text: qsTr("Select all")
                        display: AbstractButton.IconOnly
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                    }

                    SkrToolButton {
                        id: listMenuToolButton
                        flat: true
                        text: qsTr("Trashed menu")
                        display: AbstractButton.IconOnly
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                    }
                }
            }

            SkrLabel {
                id: text1
                text: qsTr("The checked documents are those which were trashed at the same time")
                font.pointSize: 12
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

                    Accessible.name: qsTr("List of trashed items to be restored")
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

