import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import Skribisto
import SkrControls
import theme

SkrBasePage {
    id: base

    readonly property int columnWidth: 550 * SkrSettings.interfaceSettings.zoom
    property alias themeListView: themeListView
    property alias propertiesListView: propertiesListView
    property alias colorCodeTextField: colorCodeTextField
    property alias addThemeButton: addThemeButton
    property alias removeThemeButton: removeThemeButton

    property alias viewButtons: viewButtons
    property alias titleLabel: titleLabel

    property alias forceLightButton: forceLightButton
    property alias forceDarkButton: forceDarkButton
    property alias forceDistractionFreeButton: forceDistractionFreeButton
    property alias primaryTextAreaSample: primaryTextAreaSample

    property alias saveThemeButton: saveThemeButton
    property alias resetThemeButton: resetThemeButton

    clip: true

    ColumnLayout {
        id: columnLayout
        spacing: 0
        anchors.fill: parent

        //-------------------------------------------------
        //--- Tool bar  ----------------------------------
        //-------------------------------------------------
        SkrPageToolBar {
            Layout.fillWidth: true
            Layout.preferredHeight: 30

            RowLayout {
                anchors.fill: parent

                SkrLabel {
                    id: titleLabel
                    verticalAlignment: Qt.AlignVCenter
                    horizontalAlignment: Qt.AlignHCenter
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                }

                SkrViewButtons {
                    id: viewButtons
                    Layout.fillHeight: true
                }
            }
        }
        SkrPane {
            Layout.fillHeight: true
            Layout.fillWidth: true

            padding: 0

            ScrollView {
                id: scrollView
                anchors.fill: parent
                clip: true

                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded
                contentWidth: pillarLayout.width
                contentHeight: pillarLayout.implicitHeight

                Flickable {

                    SKRPillarLayout {
                        id: pillarLayout
                        width: scrollView.width
                        columns: ((pillarLayout.width / columnWidth) | 0)
                        maxColumns: 3

                        SkrGroupBox {
                            id: themeListGroupBox
                            Layout.fillWidth: true
                            focusPolicy: Qt.TabFocus

                            title: qsTr("Themes")

                            ColumnLayout {
                                id: columnLayout3
                                anchors.fill: parent

                                SkrToolBar {
                                    Layout.fillWidth: true

                                    RowLayout {
                                        id: rowLayout1
                                        spacing: 1
                                        anchors.fill: parent

                                        Item {
                                            id: stretcher
                                            Layout.fillHeight: true
                                            Layout.fillWidth: true
                                        }
                                        SkrToolButton {
                                            id: removeThemeButton
                                            flat: true
                                            display: AbstractButton.IconOnly
                                        }

                                        SkrToolButton {
                                            id: addThemeButton
                                            flat: true
                                            display: AbstractButton.IconOnly
                                        }
                                    }
                                }

                                ListView {
                                    id: themeListView
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 400
                                    clip: true
                                    Layout.preferredHeight: 400
                                    Layout.minimumHeight: 200
                                    Layout.fillHeight: false
                                    keyNavigationWraps: false
                                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                                    //                            KeyNavigation.tab: groupBox2
                                    //                            KeyNavigation.backtab: groupBox
                                }
                            }
                        }
                        SkrGroupBox {
                            id: exampleGroupBox
                            Layout.fillWidth: true
                            focusPolicy: Qt.TabFocus

                            title: qsTr("Example")

                            ColumnLayout {
                                id: columnLayout5
                                anchors.fill: parent

                                RowLayout {
                                    Layout.fillWidth: true

                                    SkrLabel {
                                        text: qsTr("Force a color mode:")
                                    }
                                    SkrButton {
                                        id: forceLightButton
                                        text: qsTr("Light")
                                        icon.source: "qrc:///icons/backup/color-picker-white.svg"
                                    }
                                    SkrButton {
                                        id: forceDarkButton
                                        text: qsTr("Dark")
                                        icon.source: "qrc:///icons/backup/color-picker-black.svg"
                                    }
                                    SkrButton {
                                        id: forceDistractionFreeButton
                                        text: qsTr("Distraction free")
                                        icon.source: "qrc:///icons/backup/view-fullscreen.svg"
                                    }
                                }

                                SkrToolBar {
                                    Layout.fillWidth: true

                                    RowLayout {
                                        anchors.fill: parent
                                        SkrLabel {
                                            text: qsTr("Tool bar")
                                        }
                                        Item {
                                            Layout.fillWidth: true
                                        }
                                        SkrToolButton {
                                            icon.source: "qrc:///icons/backup/edit-copy.svg"
                                        }
                                        SkrToolButton {
                                            icon.source: "qrc:///icons/backup/edit-copy.svg"
                                            enabled: false
                                        }
                                    }
                                }

                                SkrPageToolBar {
                                    Layout.fillWidth: true

                                    RowLayout {
                                        anchors.fill: parent

                                        SkrLabel {
                                            text: qsTr("Page tool bar")
                                        }
                                        Item {
                                            Layout.fillWidth: true
                                        }

                                        SkrToolButton {
                                            icon.source: "qrc:///icons/backup/edit-copy.svg"
                                        }
                                        SkrToolButton {
                                            icon.source: "qrc:///icons/backup/edit-copy.svg"
                                            enabled: false
                                        }
                                    }
                                }
                                ListView {
                                    id: exampleListView
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 400
                                    clip: true
                                    Layout.preferredHeight: 100
                                    Layout.minimumHeight: 100
                                    Layout.fillHeight: false
                                    keyNavigationWraps: false
                                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                                    model: ListModel {
                                        ListElement {
                                            text: qsTr("List item")
                                        }
                                    }

                                    delegate: SkrPane {
                                        height: 40

                                        SkrLabel {
                                            anchors.fill: parent
                                            text: model.text
                                        }
                                    }
                                }

                                SkrPane {
                                    //Layout.fillWidth: true
                                    Layout.preferredHeight: 100
                                    Layout.preferredWidth: 500

                                    GridLayout {
                                        anchors.fill: parent
                                        columns: 4

                                        SkrButton {
                                            text: qsTr("Button")
                                            icon.source: "qrc:///icons/backup/edit-copy.svg"
                                        }
                                        SkrButton {
                                            text: qsTr("Button (disabled)")
                                            icon.source: "qrc:///icons/backup/edit-copy.svg"
                                            enabled: false
                                        }

                                        SkrToolButton {
                                            text: qsTr("Tool Button")
                                            icon.source: "qrc:///icons/backup/edit-copy.svg"
                                        }

                                        SkrToolButton {
                                            text: qsTr("Tool Button")
                                            display: AbstractButton.TextBesideIcon
                                            icon.source: "qrc:///icons/backup/edit-copy.svg"
                                        }

                                        SkrToolButton {
                                            text: qsTr("Tool Button (disabled)")
                                            icon.source: "qrc:///icons/backup/edit-copy.svg"
                                            enabled: false
                                        }

                                        SkrRoundButton {
                                            text: qsTr("Round button")
                                            icon.source: "qrc:///icons/backup/edit-copy.svg"
                                        }

                                        SkrRoundButton {
                                            text: qsTr("Round button")
                                            display: AbstractButton.IconOnly
                                            icon.source: "qrc:///icons/backup/edit-copy.svg"
                                        }

                                        SkrRoundButton {
                                            text: qsTr(
                                                      "Round button (disabled)")
                                            icon.source: "qrc:///icons/backup/edit-copy.svg"
                                            enabled: false
                                        }
                                        SkrSwitch {
                                            text: qsTr("Switch (checked)")
                                            display: AbstractButton.IconOnly
                                            checked: true
                                        }

                                        SkrSwitch {
                                            text: qsTr("Switch")
                                            checked: false
                                        }
                                    }
                                }

                                SkrPane {
                                    id: pageBackgroundExample
                                    Layout.fillWidth: true
                                    Layout.preferredHeight: 200

                                    ColumnLayout {

                                        WritingZone {
                                            id: primaryTextAreaSample
                                            Layout.preferredHeight: 200
                                            Layout.preferredWidth: pageBackgroundExample.width / 2

                                            clip: true
                                            spellCheckerKilled: false
                                            leftScrollItemVisible: true
                                            stretch: true
                                            textArea.placeholderText: qsTr("Outline")

                                            textAreaStyleBackgroundColor: SkrTheme.mainTextAreaBackground
                                            textAreaStyleForegroundColor: SkrTheme.mainTextAreaForeground
                                            paneStyleBackgroundColor: SkrTheme.pageBackground
                                            textAreaStyleAccentColor: SkrTheme.accent

                                            Component.onCompleted: {
                                                primaryTextAreaSample.determineSpellCheckerLanguageCode()
                                            }
                                        }
                                    }
                                }

                                WritingZone {
                                    id: secondaryTextAreaSample

                                    Layout.preferredHeight: 200
                                    Layout.fillWidth: true

                                    clip: true
                                    spellCheckerKilled: true
                                    leftScrollItemVisible: false
                                    stretch: true
                                    textArea.placeholderText: qsTr("Outline")
                                    textArea.text: qsTr("Secondary text")

                                    textAreaStyleBackgroundColor: SkrTheme.secondaryTextAreaBackground
                                    textAreaStyleForegroundColor: SkrTheme.secondaryTextAreaForeground
                                    paneStyleBackgroundColor: SkrTheme.pageBackground
                                    textAreaStyleAccentColor: SkrTheme.accent
                                }
                            }
                        }

                        SkrGroupBox {
                            id: colorPropertiesGroupBox
                            Layout.preferredWidth: 400
                            //Layout.fillWidth: true
                            focusPolicy: Qt.TabFocus

                            title: qsTr("Color properties")

                            ColumnLayout {
                                id: columnLayout4
                                anchors.fill: parent

                                ScrollView {
                                    id: scrollView2
                                    focusPolicy: Qt.StrongFocus
                                    Layout.fillWidth: true
                                    Layout.preferredHeight: 800
                                    clip: true
                                    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                                    ScrollBar.vertical.policy: ScrollBar.AsNeeded

                                    ListView {
                                        id: propertiesListView
                                        anchors.fill: parent
                                        clip: true
                                        keyNavigationWraps: false
                                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                                    }
                                }
                            }
                        }

                        SkrGroupBox {
                            id: colorsGroupBox
                            Layout.fillWidth: true
                            focusPolicy: Qt.TabFocus

                            title: qsTr("Colors")

                            ColumnLayout {
                                id: columnLayout6
                                anchors.fill: parent

                                SkrTextField {
                                    id: colorCodeTextField
                                    Layout.fillWidth: true
                                    Layout.preferredHeight: 50
                                }
                            }
                        }
                        SkrGroupBox {
                            id: saveGroupBox
                            Layout.fillWidth: true
                            focusPolicy: Qt.TabFocus

                            title: qsTr("Save")

                            RowLayout {
                                id: columnLayout7
                                anchors.fill: parent
                                SkrButton {
                                    id: resetThemeButton
                                    text: qsTr("Reset theme")
                                }
                                SkrButton {
                                    id: saveThemeButton
                                    text: qsTr("Save theme")
                                }
                            }
                        }
                    }
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

