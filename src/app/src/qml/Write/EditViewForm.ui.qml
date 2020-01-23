import QtQuick 2.12
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

Item {
    property alias swipeView: swipeView
    property alias italicToolButton: italicToolButton
    property alias boldToolButton: boldToolButton
    property alias strikeToolButton: strikeToolButton
    property alias sizeToolButton: sizeToolButton
    property alias underlineToolButton: underlineToolButton
    property alias fullScreenToolButton: fullScreenToolButton
    property alias goBackToolButton: goBackToolButton
    property alias textWidthSlider: textWidthSlider
    property alias textPointSizeSlider: textPointSizeSlider
    property alias fontFamilyComboBox: fontFamilyComboBox

    SwipeView {
        id: swipeView
        interactive: false
        anchors.fill: parent

        Item {
            id: mainPage


            ColumnLayout {
                anchors.fill: parent


                GroupBox {
                    id: groupBox
                    Layout.fillWidth: true
                    title: qsTr("Edit text")

                    GridLayout {
                        id: gridLayout
                        columns: groupBox.contentItem.width / italicToolButton.width -1
                        anchors.left: parent.left
                        anchors.right: parent.right

                        ToolButton {
                            id: italicToolButton
                            text: qsTr("Italic")
                            display: AbstractButton.IconOnly
                        }
                        ToolButton {
                            id: boldToolButton
                            text: qsTr("Bold")
                            display: AbstractButton.IconOnly
                        }
                        ToolButton {
                            id: strikeToolButton
                            text: qsTr("Strike")
                            display: AbstractButton.IconOnly
                        }

                        ToolButton {
                            id: underlineToolButton
                            text: qsTr("Underline")
                            display: AbstractButton.IconOnly
                        }

                        ToolButton {
                            id: fullScreenToolButton
                            text: qsTr("Fullscreen")
                            display: AbstractButton.IconOnly
                        }

                        ToolButton {
                            id: sizeToolButton
                            text: qsTr("Size")
                            display: AbstractButton.IconOnly

                        }

                    }
                }

            Item{
                id: stretcher
                Layout.fillHeight: true
                Layout.fillWidth: true
            }
            }

        }

        Item {
            id: sizePage
            ColumnLayout {
                anchors.fill: parent

                ToolButton {
                    id: goBackToolButton
                    text: qsTr("Go back")
                    display: AbstractButton.IconOnly
                }


                Label {
                    text: qsTr("Text width :")
                }

                Slider {
                    id: textWidthSlider
                    from: writingZone.width / 4
                    to: Screen.width
                }

                Label {
                    text: qsTr("Text size :")
                }

                Slider {
                    id: textPointSizeSlider
                    snapMode: Slider.SnapOnRelease
                    stepSize: 1
                    from: 8
                    to: 40
                }

                ComboBox {
                    id: fontFamilyComboBox
                    Layout.fillWidth: true
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

