import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Commons"
import "../Items"
import ".."

Item {

    property alias saveDial: saveDial
    property alias saveSpinBox: saveSpinBox
    property alias saveEveryCheckBox: saveEveryCheckBox
    property alias backUpEveryCheckBox: backUpEveryCheckBox
    property alias backupHoursSpinBox: backupHoursSpinBox
    property alias backupHoursDial: backupHoursDial

    property alias accessibilityGroupBox: accessibilityGroupBox
    property alias accessibilityCheckBox: accessibilityCheckBox

    property alias showMenuButtonCheckBox: showMenuButtonCheckBox
    property alias backUpOnceADayCheckBox: backUpOnceADayCheckBox
    property alias backupPathListView: backupPathListView
    property alias removeBackupPathButton: removeBackupPathButton
    property alias addBackupPathButton: addBackupPathButton
    property alias setTextCursorUnblinkingCheckBox: setTextCursorUnblinkingCheckBox
    property alias animationEnabledCheckBox: animationEnabledCheckBox
    property alias langComboBox: langComboBox
    property alias checkSpellingCheckBox: checkSpellingCheckBox
    property alias checkSpellingComboBox: checkSpellingComboBox
    property alias openThemePageButton: openThemePageButton
    property alias createEmpyProjectAtStartSwitch: createEmpyProjectAtStartSwitch
    property alias centerTextCursorSwitch: centerTextCursorSwitch
    property alias textPointSizeSlider: textPointSizeSlider
    property alias fontFamilyComboBox: fontFamilyComboBox
    property alias textTopMarginSlider: textTopMarginSlider
    property alias textIndentSlider: textIndentSlider
    property alias devModeCheckBox: devModeCheckBox
    property alias pluginPageButton: pluginPageButton
    property alias firstStepsButton: firstStepsButton
    property alias showMinimapCheckBox: showMinimapCheckBox
    property alias minimapDividerSpinBox: minimapDividerSpinBox
    property alias minimapDividerDial: minimapDividerDial

    SkrPane {
        id: pane2
        anchors.fill: parent

        ScrollView {
            id: scrollView
            anchors.fill: parent
            clip: true

            ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
            ScrollBar.vertical.policy: ScrollBar.AsNeeded
            contentWidth: scrollView.width
            contentHeight: pillarLayout.implicitHeight

            ColumnLayout {
                id: pillarLayout
                width: scrollView.width

                SkrGroupBox {
                    id: accessibilityGroupBox
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus

                    title: qsTr("Accessibility")

                    ColumnLayout {
                        id: columnLayout3
                        anchors.fill: parent

                        SkrSwitch {
                            id: accessibilityCheckBox
                            text: qsTr("Help with accessibility")

                        }

                        SkrSwitch {
                            id: showMenuButtonCheckBox
                            text: qsTr("Show menu button")

                        }
                    }
                }

                SkrGroupBox {
                    id: appearanceGroupBox
                    width: 200
                    height: 200
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus
                    title: qsTr("Appearance")

                    ColumnLayout {
                        id: columnLayout5
                        anchors.fill: parent

                        ColumnLayout {
                            id: rowLayout4

                            SkrLabel {
                                id: langLabel
                                text: qsTr("Interface language :")
                            }

                            SkrComboBox {
                                id: langComboBox
                                wheelEnabled: true
                            }
                            SkrButton {
                                id: openThemePageButton
                                text: qsTr("Manage themes")
                            }
                        }

                    }
                }

                SkrGroupBox {
                    id: behaviorGroupBox
                    focusPolicy: Qt.TabFocus
                    Layout.fillWidth: true
                    title: qsTr("Behavior")

                    ColumnLayout {
                        id: columnLayout10
                        anchors.fill: parent

                        SkrSwitch {
                            id: createEmpyProjectAtStartSwitch
                            text: qsTr("Create an empty project at start")
                        }

                        SkrSwitch {
                            id: centerTextCursorSwitch
                            text: qsTr("Center vertically the text cursor")
                        }


                        SkrButton {
                            id: pluginPageButton
                            text: qsTr("Manage plugins")
                        }

                        SkrButton{
                            id: firstStepsButton
                            text: qsTr("Show the first steps dialog")
                        }
                    }
                }

                SkrGroupBox {
                    id: spellCheckingGroupBox
                    width: 200
                    height: 200
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus
                    title: qsTr("Spell checking")

                    ColumnLayout {
                        id: columnLayout6
                        anchors.fill: parent

                        SkrSwitch {
                            id: checkSpellingCheckBox
                            text: qsTr("Check spelling")
                        }

                        RowLayout {
                            id: rowLayout5

                            SkrLabel {
                                id: label
                                text: qsTr("Default dictionary :")
                            }

                            SkrComboBox {
                                id: checkSpellingComboBox

                            }
                        }
                    }
                }

                SkrGroupBox {
                    id: minimapGroupBox
                    Layout.rowSpan: 2
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus
                    title: qsTr("Minimap scrollbar")

                    ColumnLayout {
                        anchors.fill: parent

                        SkrSwitch {
                            id: showMinimapCheckBox
                            text: qsTr("Show the minimap scrollbar")

                        }
                        RowLayout {
                            Layout.fillHeight: true
                            Layout.fillWidth: true


                            SkrSpinBox {
                                id: minimapDividerSpinBox
                                to: 10
                                from: 3

                            }

                            SkrDial {
                                id: minimapDividerDial
                                to: 10
                                from: 3
                                wheelEnabled: true
                            }
                        }
                    }
                }

                SkrGroupBox {
                    id: backupGroupBox
                    Layout.rowSpan: 3
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus
                    title: qsTr("Backup")

                    ColumnLayout {
                        id: columnLayout
                        anchors.fill: parent

                        SkrGroupBox {
                            id: groupBox1
                            title: qsTr("Backup paths :")
                            focusPolicy: Qt.TabFocus
                            bigTitleEnabled: false

                            ColumnLayout {
                                id: columnLayout1
                                anchors.fill: parent
                                anchors.topMargin: 5

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
                                            id: removeBackupPathButton
                                            flat: true
                                            display: AbstractButton.IconOnly
                                        }

                                        SkrToolButton {
                                            id: addBackupPathButton
                                            flat: true
                                            display: AbstractButton.IconOnly

                                        }
                                    }
                                }

                                ScrollView {
                                    id: backupPathScrollView
                                    focusPolicy: Qt.StrongFocus
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    Layout.preferredHeight: 200
                                    Layout.preferredWidth: 300
                                    clip: true
                                    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                                    ScrollBar.vertical.policy: ScrollBar.AsNeeded

                                    ListView {
                                        id: backupPathListView
                                        width: 110
                                        height: 160
                                        Layout.preferredHeight: 200
                                        Layout.preferredWidth: 300
                                        clip: true
                                        focus: true
                                        smooth: true
                                        boundsBehavior: Flickable.StopAtBounds
                                    }
                                }
                            }
                        }

                        RowLayout {
                            id: rowLayout

                            SkrSwitch {
                                id: backUpEveryCheckBox
                                text: qsTr("Back up every")
                            }

                            SkrSpinBox {
                                id: backupHoursSpinBox
                                wheelEnabled: true
                                editable: true

                                to: 60
                                from: 1

                            }

                            SkrLabel {
                                id: backupHours
                                text: qsTr("hours")

                            }

                            SkrDial {
                                id: backupHoursDial
                                wheelEnabled: true

                                to: 60
                                from: 1
                            }
                        }

                        SkrSwitch {
                            id: backUpOnceADayCheckBox
                            text: qsTr("Back up once a day")
                        }
                    }
                }


                SkrGroupBox {
                    id: saveGroupBox
                    Layout.rowSpan: 2
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus
                    title: qsTr("Save")

                    ColumnLayout {
                        id: columnLayout2
                        anchors.fill: parent

                        RowLayout {
                            id: rowLayout2
                            Layout.fillHeight: true
                            Layout.fillWidth: true

                            SkrSwitch {
                                id: saveEveryCheckBox
                                text: qsTr("Save every")

                            }

                            SkrSpinBox {
                                id: saveSpinBox

                            }

                            SkrLabel {
                                id: saveMinutes
                                text: qsTr("minutes")

                            }

                            SkrDial {
                                id: saveDial
                                to: 60
                                from: 1
                                wheelEnabled: true
                            }
                        }
                    }
                }


                SkrGroupBox {
                    id: quickPrintGroupBox
                    Layout.rowSpan: 2
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus
                    title: qsTr("Quick print")

                    ColumnLayout {
                        id: columnLayout4
                        anchors.fill: parent

                        RowLayout {
                            id: rowLayout8
                            Layout.fillHeight: true
                            Layout.fillWidth: true

                            SkrSwitch {
                                id: includeSynopsisCheckBox
                                text: qsTr("Include outline")

                            }
                            SkrSwitch {
                                id: tagsEnabledCheckBox
                                text: qsTr("Add tags")

                            }

                        }
                        ColumnLayout {
                            Layout.fillHeight: true
                            Layout.fillWidth: true


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

                SkrGroupBox {
                    id: specialEPaperGroupBox
                    width: 200
                    height: 200
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus
                    title: qsTr("Special E-Paper")

                    ColumnLayout {
                        Layout.fillHeight: true
                        Layout.fillWidth: true

                        SkrSwitch {
                            id: setTextCursorUnblinkingCheckBox
                            text: qsTr("Set the text cursor unblinking")

                        }

                        SkrSwitch {
                            id: animationEnabledCheckBox
                            text: qsTr("Enable animations")

                        }

                    }

                }


                SkrGroupBox {
                    id: advancedGroupBox
                    width: 200
                    height: 200
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus
                    title: qsTr("Advanced")

                    RowLayout {
                        id: rowLayout3
                        anchors.fill: parent

                        ColumnLayout {
                            id: columnLayout7
                            width: 100
                            height: 100

                            SkrSwitch {
                                id: devModeCheckBox
                                text: qsTr("Development mode")
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
    D{i:0;autoSize:true;height:1500;width:600}
}
##^##*/

