import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../../Items"

Item {
    width: 400
    height: 400
    property alias goBackButton: goBackButton


    property alias backUpEveryCheckBox: backUpEveryCheckBox
    property alias backupHoursSpinBox: backupHoursSpinBox
    property alias backupHoursDial: backupHoursDial

    property alias backUpOnceADayCheckBox: backUpOnceADayCheckBox
    property alias backupPathListView: backupPathListView
    property alias removeBackupPathButton: removeBackupPathButton
    property alias addBackupPathButton: addBackupPathButton

    property alias backupGroupBox: backupGroupBox


    ColumnLayout {
        id: pillarLayout
        anchors.fill: parent

        SkrToolButton{
            id: goBackButton
            icon.source: "qrc:///icons/backup/go-previous.svg"
            text: qsTr("Go back")
            Layout.alignment: Qt.AlignLeft
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




    }
}
