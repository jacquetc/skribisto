import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Skribisto
import SkrControls

FocusScope {
    id: base
    property int scrollBarVerticalPolicy: ScrollBar.AlwaysOff
    property alias goUpToolButton: goUpToolButton
    property alias selectToolButton: selectToolButton
    property alias selectAllToolButton: selectAllToolButton
    property alias addToolButton: addToolButton
    property alias treeMenuToolButton: treeMenuToolButton
    property alias navigationListStackView: navigationListStackView

    SkrPane {
        id: pane
        padding: 1
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
                        id: goUpToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                    }

                    Item {
                        id: stretcher
                        Layout.fillWidth: true
                    }

                    SkrToolButton {
                        id: selectAllToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                    }

                    SkrToolButton {
                        id: selectToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                    }

                    SkrToolButton {
                        id: addToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                    }

                    SkrToolButton {
                        id: treeMenuToolButton
                        flat: true
                        text: qsTr("Navigation menu")
                        display: AbstractButton.IconOnly
                        KeyNavigation.tab: navigationListStackView.currentItem
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                    }
                }
            }

            StackView {
                id: navigationListStackView
                Layout.fillWidth: true
                Layout.fillHeight: true
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

