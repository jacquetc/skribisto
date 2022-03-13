import QtQuick
import QtQml
import QtQuick.Layouts
import QtQuick.Controls

import "../../Items"
import "../../Commons"
import "../.."

SkrBasePage {
    id: base
    width: 1000
    height: 600

    property alias viewButtons: viewButtons
    property alias pageMenuToolButton: pageMenuToolButton
    property alias titleLabel: titleLabel
    property alias countLabel: countLabel
    property alias relationshipPanel: relationshipPanel
    property int relationshipPanelPreferredHeight: 200
    readonly property int columnWidth: 550


    property alias editTitlebutton: editTitlebutton
    property alias dictComboBox: dictComboBox
    property alias dictNotFoundLabel: dictNotFoundLabel
    property alias titleLabel2: titleLabel2
    property alias editTitleTextFieldLoader: editTitleTextFieldLoader
    property alias locationLabel: locationLabel
    property alias charCountLabel: charCountLabel
    property alias wordCountLabel: wordCountLabel
    property alias installDictButton: installDictButton
    property alias noteFolderComboBox: noteFolderComboBox
    property alias addNewNoteFolderButton: addNewNoteFolderButton



    clip: true

    ColumnLayout {
        id: rowLayout
        spacing: 0
        anchors.fill: parent

        ColumnLayout {
            id: columnLayout
            spacing: 0
            Layout.fillWidth: true
            Layout.fillHeight: true

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

                    SkrToolButton {
                        id: pageMenuToolButton
                        text: qsTr("Page menu")
                        icon.source: "qrc:///icons/backup/overflow-menu.svg"
                        Layout.alignment: Qt.AlignCenter
                        Layout.preferredHeight: 30
                    }

                    SkrLabel {
                        id: countLabel
                        verticalAlignment: Qt.AlignVCenter
                        horizontalAlignment: Qt.AlignHCenter
                        Layout.alignment: Qt.AlignCenter
                    }

                    SkrViewButtons {
                        id: viewButtons
                        Layout.fillHeight: true
                    }
                }
            }

            SkrPane {
                id: pane
                Layout.fillHeight: true
                Layout.fillWidth: true

                ColumnLayout {
                    id: columnLayout2
                    anchors.fill: parent

                    ColumnLayout {
                        id: columnLayout1
                        width: 100
                        height: 100
                        Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                        Layout.margins: 4
                        Layout.fillWidth: true

                        RowLayout {
                            id: rowLayout2

                            SkrLabel {
                                id: titleLabel2
                                font.bold: true
                                font.pointSize: 20
                            }
                            Loader {
                                id: editTitleTextFieldLoader
                                Layout.fillWidth: true
                            }


                            SkrToolButton {
                                id: editTitlebutton
                                text: qsTr("Edit project name")
                                Layout.preferredHeight: 40
                                Layout.preferredWidth: 40
                            }

                        }


                        RowLayout {
                            id: rowLayout3

                            SkrLabel {
                                id: label
                                text: qsTr("Location:")
                            }

                            SkrLabel {
                                id: locationLabel
                                text: "Label"
                                Layout.fillWidth: true
                            }
                        }

                    }

                    SwipeView {
                        id: swipeView
                        Layout.fillHeight: true
                        Layout.fillWidth: true


                        currentIndex: 0
                        interactive: false
                        clip: true

                        ScrollView {
                            id: scrollView
                            clip: true

                            ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                            ScrollBar.vertical.policy: ScrollBar.AsNeeded
                            contentWidth: pillarLayout.width
                            contentHeight: pillarLayout.implicitHeight

                            SKRPillarLayout {
                                id: pillarLayout
                                width: scrollView.width
                                columns: ((pillarLayout.width / columnWidth) | 0 )
                                maxColumns: 3

                                SkrGroupBox {
                                    id: langGroupBox
                                    focusPolicy: Qt.TabFocus
                                    Layout.fillWidth: true
                                    title: qsTr("Language")

                                    ColumnLayout {

                                        RowLayout {
                                            id: rowLayout1
                                            Layout.fillWidth: true

                                            SkrLabel {
                                                id: label1
                                                text: qsTr("Dictionary:")
                                            }

                                            SkrComboBox {
                                                id: dictComboBox
                                            }

                                            SkrLabel {
                                                id: dictNotFoundLabel
                                                color: "#ee0000"
                                                text: qsTr("Selected dictionary not found")
                                            }
                                        }

                                        SkrButton {
                                            id: installDictButton
                                            text: qsTr("Install new dictionaries")
                                        }

                                    }
                                }

                                SkrGroupBox {
                                    id: projectDictGroupBox
                                    focusPolicy: Qt.TabFocus
                                    Layout.fillWidth: true
                                    title: qsTr("Project dictionary")

                                    ColumnLayout {

                                        UserDictPage {
                                            projectId: base.projectId
                                            Layout.fillWidth: true
                                            Layout.fillHeight: true

                                            implicitHeight: 300
                                            implicitWidth: 300

                                        }
                                    }


                                }

                                SkrGroupBox {
                                    id: tagsGroupBox
                                    focusPolicy: Qt.TabFocus
                                    Layout.fillWidth: true
                                    title: qsTr("Tags")

                                    RowLayout {
                                        id: rowLayout4
                                        width: 100
                                        height: 100
                                    }

                                }

                                SkrGroupBox {
                                    id: statsGroupBox
                                    focusPolicy: Qt.TabFocus
                                    Layout.fillWidth: true
                                    title: qsTr("Statistics")

                                    ColumnLayout {

                                        SkrLabel{
                                            id: charCountLabel
                                            Layout.fillWidth: true
                                        }
                                        SkrLabel{
                                            id: wordCountLabel
                                            Layout.fillWidth: true
                                        }

                                    }

                                }

                                SkrGroupBox {
                                    id: noteGroupBox
                                    focusPolicy: Qt.TabFocus
                                    Layout.fillWidth: true
                                    title: qsTr("Notes")

                                    ColumnLayout {

                                        RowLayout {
                                            id: rowLayout5
                                            Layout.fillWidth: true

                                            SkrLabel {
                                                id: label4
                                                text: qsTr("Note folder:")
                                            }

                                            SkrComboBox {
                                                id: noteFolderComboBox
                                            }
                                        }

                                        SkrButton{
                                            id: addNewNoteFolderButton

                                            text: qsTr("Add a \"Notes\" folder")

                                        }

                                    }
                                }
                            }
                        }
                    }
                }
            }




            RelationshipPanel {
                id: relationshipPanel
                Layout.preferredHeight: relationshipPanelPreferredHeight
                Layout.fillWidth: true
                visible: false
            }
        }
    }
}
