import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Skribisto
import SkrControls

Item {
    id: base

    property alias listView: listView
    property alias scrollView: scrollView
    property int scrollBarVerticalPolicy: ScrollBar.AlwaysOff
    property alias goBackToolButton: goBackToolButton
    property alias restoreToolButton: restoreToolButton
    property alias listMenuToolButton: listMenuToolButton
    property alias trashProjectComboBox: trashProjectComboBox
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

                    SkrComboBox {
                        id: trashProjectComboBox
                        Layout.fillWidth: true
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
                        id: listMenuToolButton
                        flat: true
                        text: qsTr("Trashed menu")
                        display: AbstractButton.IconOnly
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
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
                    smooth: true
                    boundsBehavior: Flickable.StopAtBounds

                    treelikeIndentsVisible: true
                    checkButtonsVisible: false
                    openActionsEnabled: true
                    renameActionEnabled: true
                    copyActionEnabled: true
                    deleteActionEnabled: true

                    Accessible.name: "List of trashed items"
                    Accessible.role: Accessible.List
                }
            }
        }
    }
}
