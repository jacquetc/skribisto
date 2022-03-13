import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import QtQuick.Window
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
    property alias quickPrintToolButton: quickPrintToolButton
    property alias findToolButton: findToolButton
    property alias minimapToolButton: minimapToolButton


    readonly property int sizePageLayoutHeight: sizePageLayout.childrenRect.height + sizePage.padding * 2
    readonly property int mainPageLayoutHeight: mainPageLayout.childrenRect.height + mainPage.padding * 2

    implicitHeight:  sizePageLayoutHeight > mainPageLayoutHeight ? sizePageLayoutHeight : mainPageLayoutHeight

        SwipeView {
            id: swipeView
            interactive: false
            anchors.fill: parent


            SkrPane {
                id: mainPage
                padding: 2


                ColumnLayout {
                    id: mainPageLayout
                    anchors.top: parent.top
                    anchors.left: parent.left
                    anchors.right: parent.right

                    SkrGroupBox {
                        id: groupBox
                        padding: 5
                        Layout.fillWidth: true
                        title: qsTr("Edit text")
                        bigTitleEnabled: false

                        GridLayout {
                            id: gridLayout
                            columns: groupBox.width / italicToolButton.width - 1
                            columnSpacing: 3
                            rowSpacing: 3

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
                        bigTitleEnabled: false

                        GridLayout {
                            id: gridLayout2
                            columnSpacing: 3
                            rowSpacing: 3
                            columns: groupBox2.width / fullScreenToolButton.width - 1

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
                                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                                display: AbstractButton.IconOnly
                            }
                            SkrToolButton {
                                id: centerTextCursorToolButton
                                text: qsTr("Center vertically the text cursor")
                                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                                display: AbstractButton.IconOnly
                            }
                            SkrToolButton {
                                id: findToolButton
                                text: qsTr("Find")
                                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                                display: AbstractButton.IconOnly
                            }
                            SkrToolButton {
                                id: minimapToolButton
                                text: qsTr("Show the minimap scrollbar")
                                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                                display: AbstractButton.IconOnly
                            }
                        }
                    }

                    SkrGroupBox {
                        id: groupBox3
                        padding: 5
                        Layout.fillWidth: true
                        title: qsTr("Share")
                        bigTitleEnabled: false

                        GridLayout {
                            id: gridLayout3
                            columnSpacing: 3
                            rowSpacing: 3
                            columns: groupBox3.width / quickPrintToolButton.width - 1

                            SkrToolButton {
                                id: quickPrintToolButton
                                text: qsTr("Quick print")
                                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                                display: AbstractButton.IconOnly
                            }

                        }
                    }

//                    Item {
//                        id: stretcher
//                        Layout.fillHeight: true
//                        Layout.fillWidth: true
//                    }
                }
            }

            SkrPane {
                id: sizePage
                padding: 2
                ColumnLayout {
                    id: sizePageLayout
                    anchors.top: parent.top
                    anchors.left: parent.left
                    anchors.right: parent.right

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

                    SkrSlider {
                        id: textWidthSlider
                        from: 400
                        to: rootWindow.width
                        Layout.fillWidth: true
                    }

                    SkrLabel {
                        text: qsTr("Text size :")
                    }

                    SkrSlider {
                        id: textPointSizeSlider
                        stepSize: 1
                        from: 8
                        to: 40
                        Layout.fillWidth: true
                    }

                    SkrComboBox {
                        id: fontFamilyComboBox
                        wheelEnabled: true
                        Layout.fillWidth: true
                    }
                    SkrLabel {
                        text: qsTr("Text indent :")
                    }

                    SkrSlider {
                        id: textIndentSlider
                        stepSize: 1
                        from: 0
                        to: 200
                        Layout.fillWidth: true
                    }

                    SkrLabel {
                        text: qsTr("Top margin :")
                    }

                    SkrSlider {
                        id: textTopMarginSlider
                        stepSize: 1
                        from: 0
                        to: 30
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

