import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import "../.."
import "../../Items"

SkrToolbox {
    property alias displayGroupBox: displayGroupBox
    property alias cardSizeMultiplierSlider: cardSizeMultiplierSlider
    property alias showNotePadSwitch: showNotePadSwitch
    property alias showTagPadSwitch: showTagPadSwitch
    property alias showOutlineSwitch: showOutlineSwitch
    property alias showCharacterCountSwitch: showCharacterCountSwitch
    property alias showWordCountSwitch: showWordCountSwitch

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
                            visible: false
                        }

                        SkrSlider {
                            id: treeItemDisplayModeSlider
                            stepSize: 1
                            from: 0
                            to: 2
                            visible: false
                        }

                        SkrLabel {
                            text: qsTr("Card size:")
                        }
                        SkrSlider {
                            id: cardSizeMultiplierSlider
                            stepSize: 0.1
                            from: 1
                            to: 3
                        }

                        SkrLabel {
                            text: qsTr("Tree indentation:")
                            visible: false
                        }

                        SkrSlider {
                            id: treeIndentationSlider
                            stepSize: 1
                            from: 0
                            to: 200
                            visible: false
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
                        SkrSwitch {
                            id: showCharacterCountSwitch
                            text: qsTr("Show character count")
                            visible: false
                        }
                        SkrSwitch {
                            id: showWordCountSwitch
                            text: qsTr("Show word count")
                            visible: false
                        }
                    }
                }
            }

        }
    }
}
