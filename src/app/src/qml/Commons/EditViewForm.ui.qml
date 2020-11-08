import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15
import "../Items"
import ".."

Item {
    property alias swipeView: swipeView
    property alias italicToolButton: italicToolButton
    property alias boldToolButton: boldToolButton
    property alias strikeToolButton: strikeToolButton
    property alias sizeToolButton: sizeToolButton
    property alias underlineToolButton: underlineToolButton
    property alias goBackToolButton: goBackToolButton
    property alias textWidthLabel: textWidthLabel
    property alias textWidthSlider: textWidthSlider
    property alias textPointSizeSlider: textPointSizeSlider
    property alias fontFamilyComboBox: fontFamilyComboBox
    property alias textTopMarginSlider: textTopMarginSlider
    property alias textIndentSlider: textIndentSlider
    property alias checkSpellingToolButton: checkSpellingToolButton
    property alias fullScreenToolButton: fullScreenToolButton
    property alias themesToolButton: themesToolButton
    property alias centerTextCursorToolButton: centerTextCursorToolButton

    SwipeView {
        id: swipeView
        interactive: false
        anchors.fill: parent

        Item {
            id: mainPage

            ColumnLayout {
                anchors.fill: parent

                SkrGroupBox {
                    id: groupBox
                    padding: 5
                    Layout.fillWidth: true
                    title: qsTr("Edit text")

                    GridLayout {
                        id: gridLayout
                        columns: gridLayout.width / italicToolButton.width - 1
                        anchors.left: parent.left
                        anchors.right: parent.right
                        columnSpacing: 5
                        rowSpacing: 5

                        SkrToolButton {
                            id: italicToolButton
                            text: qsTr("Italic")
                            Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                            display: AbstractButton.IconOnly

                        }
                        SkrToolButton {
                            id: boldToolButton
                            text: qsTr("Bold")
                            Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                            display: AbstractButton.IconOnly

                        }
                        SkrToolButton {
                            id: strikeToolButton
                            text: qsTr("Strike")
                            Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                            display: AbstractButton.IconOnly

                        }

                        SkrToolButton {
                            id: underlineToolButton
                            text: qsTr("Underline")
                            Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                            display: AbstractButton.IconOnly

                        }
                    }
                }
                SkrGroupBox {
                    id: groupBox2
                    padding: 5
                    Layout.fillWidth: true
                    title: qsTr("Display")

                    GridLayout {
                        id: gridLayout2
                        columnSpacing: 5
                        rowSpacing: 5
                        columns: gridLayout.width / fullScreenToolButton.width - 1
                        anchors.left: parent.left
                        anchors.right: parent.right

                        SkrToolButton {
                            id: fullScreenToolButton
                            text: qsTr("Full screen")
                            Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                            display: AbstractButton.IconOnly
                        }

                        SkrToolButton {
                            id: sizeToolButton
                            text: qsTr("Size")
                            Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                            display: AbstractButton.IconOnly
                        }
                        SkrToolButton {
                            id: themesToolButton
                            text: qsTr("Full screen Colors")
                            Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                            display: AbstractButton.IconOnly
                        }
                        SkrToolButton {
                            id: checkSpellingToolButton
                            text: qsTr("Check spelling")
                            display: AbstractButton.IconOnly
                        }
                        SkrToolButton {
                            id: centerTextCursorToolButton
                            text: qsTr("Center vertically the text cursor ")
                            display: AbstractButton.IconOnly
                        }
                    }
                }

                Item {
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

                SkrToolButton {
                    id: goBackToolButton
                    text: qsTr("Go back")
                    display: AbstractButton.IconOnly
                    icon.source: "qrc:///icons/backup/go-previous.svg"
                }

                SkrLabel {
                    id: textWidthLabel
                    text: qsTr("Text width :")
                }

                Slider {
                    id: textWidthSlider
                    from: 400
                    to: Screen.width
                }

                SkrLabel {
                    text: qsTr("Text size :")
                }

                Slider {
                    id: textPointSizeSlider
                    snapMode: Slider.SnapOnRelease
                    stepSize: 1
                    from: 8
                    to: 40
                }

                SkrComboBox {
                    id: fontFamilyComboBox
                    wheelEnabled: true
                    Layout.fillWidth: true
                }
                SkrLabel {
                    text: qsTr("Text indent :")
                }

                Slider {
                    id: textIndentSlider
                    snapMode: Slider.SnapOnRelease
                    stepSize: 1
                    from: 0
                    to: 200
                }

                SkrLabel {
                    text: qsTr("Top margin :")
                }

                Slider {
                    id: textTopMarginSlider
                    snapMode: Slider.SnapOnRelease
                    stepSize: 1
                    from: 0
                    to: 30
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

