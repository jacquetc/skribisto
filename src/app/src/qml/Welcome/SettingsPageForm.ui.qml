import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {

    property alias saveDial: saveDial
    property alias saveSpinBox: saveSpinBox
    property alias saveCheckBox: saveCheckBox
    property alias backUpEveryCheckBox: backUpEveryCheckBox
    property alias backupHoursSpinBox: backupHoursSpinBox
    property alias backupHoursDial: backupHoursDial

    property alias menuButtonsInStatusBarSwitch: menuButtonsInStatusBarSwitch
    property alias disallowSwipeBetweenTabsCheckBox: disallowSwipeBetweenTabsCheckBox
    property alias showMenuBarCheckBox: showMenuBarCheckBox

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
                 columns:  width/ 500
                Switch {
                    id: menuButtonsInStatusBarSwitch
                    text: qsTr("Set main menu in status bar")
                }

                GroupBox {
                    id: accessibilityGroupBox

                    title: qsTr("Accessibility")

                    ColumnLayout {
                        id: columnLayout3
                        anchors.fill: parent

                        CheckBox {
                            id: disallowSwipeBetweenTabsCheckBox
                            text: qsTr("Disallow swipe gesture between tabs")
                        }

                        CheckBox {
                            id: showMenuBarCheckBox
                            text: qsTr("Show menu bar")
                        }
                    }
                }

                GroupBox {
                    id: backupGroupBox
                    title: qsTr("Backup")

                    ColumnLayout {
                        id: columnLayout
                        anchors.fill: parent

                        GroupBox {
                            id: groupBox1
                            topPadding: 20
                            title: qsTr("Backup paths :")

                            ColumnLayout {
                                id: columnLayout1
                                anchors.fill: parent

                                ListView {
                                    id: listView
                                    width: 110
                                    height: 160
                                    Layout.preferredHeight: 200
                                    Layout.preferredWidth: 300
                                    clip: true
                                    delegate: Item {
                                        x: 5
                                        width: 80
                                        height: 40
                                        Row {
                                            id: row1
                                            spacing: 10
                                            Rectangle {
                                                width: 40
                                                height: 40
                                                color: colorCode
                                            }

                                            Text {
                                                text: name
                                                anchors.verticalCenter: parent.verticalCenter
                                                font.bold: true
                                            }
                                        }
                                    }
                                    model: ListModel {
                                        id: listModel

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
                            id: backUpOneADayCheckBox
                            text: qsTr("Back up once a day")
                        }
                    }
                }


                GroupBox {
                    id: saveGroupBox
                    title: qsTr("Save")

                    ColumnLayout {
                        id: columnLayout2
                        anchors.fill: parent

                        RowLayout {
                            id: rowLayout1
                            Layout.fillHeight: true
                            Layout.fillWidth: true

                            CheckBox {
                                id: saveCheckBox
                                text: qsTr("Save every")
                            }

                            SpinBox {
                                id: saveSpinBox
                            }

                            Dial {
                                id: saveDial
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
    D{i:0;height:800;width:1000}
}
##^##*/

