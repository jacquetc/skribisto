import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Commons"
import ".."
import "../Items"

Item {
    id: base

    readonly property int columnWidth: 550
    property alias themeListView: themeListView
    property alias propertiesListView: propertiesListView
    property alias colorCodeTextField: colorCodeTextField
    property alias addThemeButton: addThemeButton
    property alias removeThemeButton: removeThemeButton

    SkrPane {
        anchors.fill: parent
        padding: 0

        ScrollView {
            id: scrollView
            anchors.fill: parent
            clip: true

            ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
            ScrollBar.vertical.policy: ScrollBar.AsNeeded
            contentWidth: pillarLayout.width
            contentHeight: pillarLayout.implicitHeight

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

                        SkrToolBar {
                            Layout.fillWidth: true

                            SkrToolButton {
                                icon.source: "qrc:///icons/backup/edit-copy.svg"
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
                                    text: qsTr("list item")
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
                            Layout.fillWidth: true
                            Layout.preferredHeight: 50

                            RowLayout {
                                anchors.fill: parent

                                SkrButton {
                                    text: qsTr("button")
                                    icon.source: "qrc:///icons/backup/edit-copy.svg"

                                }
                                SkrButton {
                                    text: qsTr("button")
                                    icon.source: "qrc:///icons/backup/edit-copy.svg"
                                    enabled: false

                                }

                                SkrToolButton {
                                    flat: true
                                    text: qsTr("button")
                                    icon.source: "qrc:///icons/backup/edit-copy.svg"
                                }

                                SkrToolButton {
                                    flat: true
                                    text: qsTr("button")
                                    icon.source: "qrc:///icons/backup/edit-copy.svg"
                                    enabled: false
                                }

                                SkrSwitch {
                                    text: qsTr("Switch")
                                    checked: true

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
                                    spellCheckerKilled: true
                                    leftScrollItemVisible: true
                                    stretch: true
                                    textArea.placeholderText: qsTr("Outline")
                                    textArea.text: qsTr("Primary text")

                                    textAreaStyleBackgroundColor: SkrTheme.mainTextAreaBackground
                                    textAreaStyleForegroundColor: SkrTheme.mainTextAreaForeground
                                    paneStyleBackgroundColor: SkrTheme.pageBackground
                                    textAreaStyleAccentColor: SkrTheme.accent
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
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/
