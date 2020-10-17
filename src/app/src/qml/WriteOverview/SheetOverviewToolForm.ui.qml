import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15

Item {
    width: 400
    height: 400
    property alias showNotePadSwitch: showNotePadSwitch
    property alias showTagPadSwitch: showTagPadSwitch
    property alias showOutlineSwitch: showOutlineSwitch

    property alias treeItemDisplayModeSlider: treeItemDisplayModeSlider
    property alias treeIndentationSlider: treeIndentationSlider

    Pane {
        id: pane
        anchors.fill: parent
        padding: 5

        ColumnLayout {
            anchors.fill: parent

            GroupBox {
                id: groupBox
                focusPolicy: Qt.TabFocus
                padding: 5
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignTop
                title: qsTr("Display")

                ColumnLayout {
                    anchors.fill: parent

                    ColumnLayout {

                        Layout.fillHeight: true
                        Layout.fillWidth: true

                        Label {
                            text: qsTr("Display mode :")
                        }

                        Slider {
                            id: treeItemDisplayModeSlider
                            snapMode: Slider.SnapOnRelease
                            stepSize: 1
                            from: 0
                            to: 2
                        }

                        Label {
                            text: qsTr("Tree indentation :")
                        }

                        Slider {
                            id: treeIndentationSlider
                            snapMode: Slider.SnapOnRelease
                            stepSize: 1
                            from: 0
                            to: 200
                        }

                        Switch {
                            id: showOutlineSwitch
                            text: qsTr("Show outline")
                        }
                        Switch {
                            id: showNotePadSwitch
                            text: qsTr("Show notes")
                        }
                        Switch {
                            id: showTagPadSwitch
                            text: qsTr("Show tags")
                        }
                    }
                }
            }
        }
    }
}