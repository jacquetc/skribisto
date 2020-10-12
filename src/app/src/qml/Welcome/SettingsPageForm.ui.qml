import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {

    property alias saveDial: saveDial
    property alias saveSpinBox: saveSpinBox
    property alias saveEveryCheckBox: saveEveryCheckBox
    property alias backUpEveryCheckBox: backUpEveryCheckBox
    property alias backupHoursSpinBox: backupHoursSpinBox
    property alias backupHoursDial: backupHoursDial

    property alias accessibilityGroupBox: accessibilityGroupBox

    property alias menuButtonsInStatusBarSwitch: menuButtonsInStatusBarSwitch
    property alias showMenuBarCheckBox: showMenuBarCheckBox
    property alias backUpOnceADayCheckBox: backUpOnceADayCheckBox
    property alias backupPathListView: backupPathListView
    property alias removeBackupPathButton: removeBackupPathButton
    property alias addBackupPathButton: addBackupPathButton
    property alias showPropertiesCheckBox: showPropertiesCheckBox
    property alias resetDockConfButton: resetDockConfButton
    property alias allowSwipeBetweenTabsCheckBox: allowSwipeBetweenTabsCheckBox
    property alias setTextCursorUnblinkingCheckBox: setTextCursorUnblinkingCheckBox
    readonly property int columnWidth: 550
    property alias langComboBox: langComboBox
    property alias checkSpellingCheckBox: checkSpellingCheckBox
    property alias checkSpellingComboBox: checkSpellingComboBox

    Pane {
        id: pane2
        anchors.fill: parent

        ScrollView {
            id: scrollView
            anchors.fill: parent
            clip: true

            ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
            ScrollBar.vertical.policy: ScrollBar.AsNeeded
            contentWidth: gridLayout1.width
            contentHeight: gridLayout1.height

            GridLayout {
                id: gridLayout1
                width: scrollView.width
                columns: width / columnWidth

                GroupBox {
                    id: accessibilityGroupBox
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus

                    title: qsTr("Accessibility")

                    ColumnLayout {
                        id: columnLayout3
                        anchors.fill: parent

                        CheckBox {
                            id: allowSwipeBetweenTabsCheckBox
                            text: qsTr("Allow swipe gesture between tabs")
                        }

                        CheckBox {
                            id: showMenuBarCheckBox
                            visible: false
                            text: qsTr("Show menu bar")
                        }
                    }
                }

                GroupBox {
                    id: appearanceGroupBox
                    width: 200
                    height: 200
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus
                    title: qsTr("Appearance")

                    ColumnLayout {
                        id: columnLayout5
                        anchors.fill: parent

                        RowLayout {
                            id: rowLayout4
                            width: 100
                            height: 100

                            Label {
                                id: langLabel
                                text: qsTr("Interface language :")
                            }

                            ComboBox {
                                id: langComboBox
                                wheelEnabled: true
                            }
                        }
                        Switch {
                            id: menuButtonsInStatusBarSwitch
                            text: qsTr("Set main menu in status bar")
                        }
                    }
                }

                GroupBox {
                    id: backupGroupBox
                    Layout.rowSpan: 3
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus
                    title: qsTr("Backup")

                    ColumnLayout {
                        id: columnLayout
                        anchors.fill: parent

                        GroupBox {
                            id: groupBox1
                            title: qsTr("Backup paths :")
                            focusPolicy: Qt.TabFocus

                            ColumnLayout {
                                id: columnLayout1
                                anchors.fill: parent
                                anchors.topMargin: 5

                                ToolBar {
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
                                        ToolButton {
                                            id: removeBackupPathButton
                                            flat: true
                                            display: AbstractButton.IconOnly
                                        }

                                        ToolButton {
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

                            CheckBox {
                                id: backUpEveryCheckBox
                                text: qsTr("Back up every")
                            }

                            SpinBox {
                                id: backupHoursSpinBox
                                wheelEnabled: true
                                editable: true

                                to: 60
                                from: 1
                            }

                            Label {
                                id: backupHours
                                text: qsTr("hours")
                            }

                            Dial {
                                id: backupHoursDial
                                wheelEnabled: true

                                to: 60
                                from: 1
                            }
                        }

                        CheckBox {
                            id: backUpOnceADayCheckBox
                            text: qsTr("Back up once a day")
                        }
                    }
                }

                GroupBox {
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

                            CheckBox {
                                id: saveEveryCheckBox
                                text: qsTr("Save every")
                            }

                            SpinBox {
                                id: saveSpinBox
                            }

                            Label {
                                id: saveMinutes
                                text: qsTr("minutes")
                            }

                            Dial {
                                id: saveDial
                                to: 60
                                from: 1
                                wheelEnabled: true
                            }
                        }
                    }
                }

                GroupBox {
                    id: specialEPaperGroupBox
                    width: 200
                    height: 200
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus
                    title: qsTr("Special E-Paper")

                    CheckBox {
                        id: setTextCursorUnblinkingCheckBox
                        text: qsTr("Set the text cursor unblinking")
                    }
                }

                GroupBox {
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
                            id: columnLayout4
                            width: 100
                            height: 100

                            CheckBox {
                                id: showPropertiesCheckBox
                                text: qsTr("Show properties tool box")
                            }

                            Button {
                                id: resetDockConfButton
                                text: qsTr("Reset dock configuration")
                            }
                        }
                    }
                }

                GroupBox {
                    id: spellCheckingGroupBox
                    width: 200
                    height: 200
                    Layout.fillWidth: true
                    focusPolicy: Qt.TabFocus
                    title: qsTr("Spell checking")

                    ColumnLayout {
                        id: columnLayout6
                        anchors.fill: parent

                        CheckBox {
                            id: checkSpellingCheckBox
                            text: qsTr("Check spelling")
                        }

                        ComboBox {
                            id: checkSpellingComboBox
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

