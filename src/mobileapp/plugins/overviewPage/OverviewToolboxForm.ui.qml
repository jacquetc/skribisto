import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

import SkrControls

SkrToolbox {
    property alias displayGroupBox: displayGroupBox
    property alias showNotePadSwitch: showNotePadSwitch
    property alias showTagPadSwitch: showTagPadSwitch
    property alias showOutlineSwitch: showOutlineSwitch
    property alias treeItemDisplayModeSlider: treeItemDisplayModeSlider
    property alias treeIndentationSlider: treeIndentationSlider

    implicitHeight: columnLayout.childrenRect.height + columnLayout.spacing

    SkrPane {
        id: pane
        anchors.fill: parent
        padding: 5


        ColumnLayout {
            id: columnLayout
            anchors.top: parent.top
            anchors.left: parent.left
            anchors.right: parent.right

            SkrGroupBox {
                id: displayGroupBox
                focusPolicy: Qt.TabFocus
                padding: 5
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignTop
                title: qsTr("Display")
                bigTitleEnabled: false

                ColumnLayout {
                    anchors.fill: parent

                    ColumnLayout {

                        Layout.fillHeight: true
                        Layout.fillWidth: true

                        SkrLabel {
                            text: qsTr("Display mode:")
                        }

                        SkrSlider {
                            id: treeItemDisplayModeSlider
                            stepSize: 1
                            from: 0
                            to: 2
                        }

                        SkrLabel {
                            text: qsTr("Tree indentation:")
                        }

                        SkrSlider {
                            id: treeIndentationSlider
                            stepSize: 1
                            from: 0
                            to: 200
                        }

                        SkrSwitch {
                            id: showOutlineSwitch
                            text: qsTr("Show outline")
                        }
                        SkrSwitch {
                            id: showNotePadSwitch
                            text: qsTr("Show notes")
                            visible: false
                        }
                        SkrSwitch {
                            id: showTagPadSwitch
                            text: qsTr("Show tags")
                        }
                    }
                }
            }

        }
    }
}
