import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../../Items"
import "../.."

FocusScope {
    id: base
    property int scrollBarVerticalPolicy: ScrollBar.AlwaysOff
    property alias goUpToolButton: goUpToolButton
    property alias selectToolButton: selectToolButton
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
                Layout.maximumHeight: 40
                Layout.preferredHeight: 40
                Layout.fillWidth: true

                RowLayout {
                    id: rowLayout
                    spacing: 1
                    anchors.fill: parent

                    SkrToolButton {
                        id: goUpToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                    }

                    Item {
                        id: stretcher
                        Layout.fillWidth: true
                    }

                    SkrToolButton {
                        id: selectToolButton
                        flat: true
                        text: qsTr("Select")
                        display: AbstractButton.IconOnly
                    }

                    SkrToolButton {
                        id: addToolButton
                        flat: true
                        text: qsTr("Add a document")
                        display: AbstractButton.IconOnly
                    }

                    SkrToolButton {
                        id: treeMenuToolButton
                        flat: true
                        text: qsTr("Navigation menu")
                        display: AbstractButton.IconOnly
                        KeyNavigation.tab: navigationListStackView.currentItem
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

