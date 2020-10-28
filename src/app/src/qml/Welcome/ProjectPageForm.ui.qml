import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import "../Commons"
import ".."

Item {
    width: 400
    height: 400

    property alias printButton: printButton
    property alias importButton: importButton
    property alias exportButton: exportButton
    property alias projectFileTextField: projectFileTextField
    property alias createNewProjectButton: createNewProjectButton
    property alias partSpinBox: partSpinBox
    property alias projectDetailPathLabel: projectDetailPathLabel
    property alias selectProjectPathToolButton: selectProjectPathToolButton
    property alias projectPathTextField: projectPathTextField
    property alias projectTitleTextField: projectTitleTextField
    property alias goBackToolButton: goBackToolButton
    property alias createEmpyProjectAtStartSwitch: createEmpyProjectAtStartSwitch
    property alias gridLayout1: gridLayout1
    property alias swipeView: swipeView

    property alias saveButton: saveButton
    property alias saveAsButton: saveAsButton
    property alias saveAllButton: saveAllButton
    property alias saveACopyButton: saveACopyButton
    property alias backUpButton: backUpButton

    property alias openProjectButton: openProjectButton
    property alias recentListView: recentListView
    property alias newProjectButton: newProjectButton

    property alias importerItem: importerItem
    property alias printItem: printItem
    property alias exporterItem: exporterItem

    readonly property int columnWidth: 550

    SkrPane {
        id: pane1
        anchors.fill: parent

        SwipeView {
            id: swipeView
            currentIndex: 0
            anchors.fill: parent
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
                    columns: ((pillarLayout.width / columnWidth) | 0)
                    maxColumns: 3

                    SkrGroupBox {
                        id: groupBox1
                        focusPolicy: Qt.TabFocus
                        Layout.fillWidth: true
                        title: qsTr("Projects")

                        ColumnLayout {
                            id: columnLayout
                            anchors.fill: parent
                            anchors.topMargin: 5

                            RowLayout {
                                id: rowLayout
                                width: 100
                                height: 100

                                SkrButton {
                                    id: newProjectButton
                                    text: qsTr("New project")
                                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                                }

                                SkrButton {
                                    id: openProjectButton
                                    text: qsTr("Open project")
                                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                                }
                            }
                        }
                    }

                    SkrGroupBox {
                        id: groupBox
                        focusPolicy: Qt.TabFocus
                        Layout.fillWidth: true
                        title: qsTr("Recent projects")
                        KeyNavigation.tab: recentListView

                        ColumnLayout {
                            id: columnLayout5
                            anchors.fill: parent

                            ListView {
                                id: recentListView
                                Layout.fillWidth: true
                                Layout.minimumWidth: 400
                                clip: true
                                Layout.preferredHeight: 400
                                Layout.minimumHeight: 200
                                Layout.fillHeight: false
                                keyNavigationWraps: false
                                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                                KeyNavigation.tab: groupBox2
                                KeyNavigation.backtab: groupBox
                            }
                        }
                    }

                    SkrGroupBox {
                        id: groupBox2
                        focusPolicy: Qt.TabFocus
                        Layout.fillWidth: true
                        title: qsTr("Save")

                        ColumnLayout {
                            id: columnLayout2
                            anchors.fill: parent
                            anchors.topMargin: 5

                            RowLayout {
                                id: rowLayout1
                                width: 100
                                height: 100

                                SkrButton {
                                    id: saveButton
                                    text: qsTr("Save")
                                    icon {
                                        name: "document-save"
                                    }
                                }

                                SkrButton {
                                    id: saveAsButton
                                    text: qsTr("Save as")
                                    icon {
                                        name: "document-save-as"
                                    }
                                }
                                SkrButton {
                                    id: saveAllButton
                                    text: qsTr("Save all")
                                    icon {
                                        name: "document-save-all"
                                    }
                                }
                            }

                            RowLayout {
                                id: rowLayout6
                                width: 100
                                height: 100

                                SkrButton {
                                    id: saveACopyButton
                                    text: qsTr("Save a copy")
                                    icon {
                                        name: "document-save-as-template"
                                    }
                                }

                                SkrButton {
                                    id: backUpButton
                                    text: qsTr("Back up")
                                    icon {
                                        name: "tools-media-optical-burn-image"
                                    }
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
                            id: columnLayout1
                            anchors.fill: parent

                            SkrSwitch {
                                id: createEmpyProjectAtStartSwitch
                                text: qsTr("Create an empty project at start")
                                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                            }
                        }
                    }

                    SkrGroupBox {
                        id: groupBox3
                        focusPolicy: Qt.TabFocus
                        Layout.fillWidth: true
                        title: qsTr("")

                        ColumnLayout {
                            id: columnLayout4
                            anchors.fill: parent
                            anchors.topMargin: 5

                            RowLayout {
                                id: rowLayout5
                                width: 100
                                height: 100

                                SkrButton {
                                    id: printButton
                                    text: qsTr("Print")
                                }

                                SkrButton {
                                    id: importButton
                                    text: qsTr("Import")
                                }

                                SkrButton {
                                    id: exportButton
                                    text: qsTr("Export")
                                }
                            }
                        }
                    }

                    Item {
                        id: stretcher1
                        Layout.columnSpan: 1
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                    }
                }
            }

            Item {
                id: newProjectItem
                enabled: false
                ColumnLayout {
                    id: columnLayout3
                    anchors.fill: parent

                    RowLayout {
                        id: rowLayout2
                        Layout.fillWidth: true

                        SkrToolButton {
                            id: goBackToolButton
                            text: qsTr("Go back")
                        }

                        SkrLabel {
                            id: titleLabel
                            text: qsTr("<h2>New project</h2>")
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                            Layout.fillWidth: true
                            Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        }
                    }
                    GridLayout {
                        id: gridLayout2
                        rows: -1
                        columns: 2
                        flow: GridLayout.TopToBottom
                        Layout.alignment: Qt.AlignHCenter

                        GridLayout {
                            id: gridLayout1
                            rows: 2
                            columns: 3
                            Layout.fillWidth: true
                            Layout.maximumWidth: 600

                            SkrLabel {
                                id: projectTitleLabel
                                text: qsTr("Project title :")
                            }
                            SkrTextField {

                                id: projectTitleTextField
                                placeholderText: qsTr("project title")
                                Layout.columnSpan: 2
                                Layout.fillWidth: true
                            }
                            SkrLabel {
                                id: projectFileLabel
                                text: qsTr("Project file :")
                            }
                            SkrTextField {

                                id: projectFileTextField
                                placeholderText: qsTr("project file")
                                Layout.columnSpan: 2
                                Layout.fillWidth: true
                            }
                            SkrLabel {
                                id: projectPathLabel
                                text: qsTr("Project path :")
                            }

                            SkrTextField {
                                id: projectPathTextField
                                placeholderText: qsTr("project path")
                                Layout.fillWidth: true
                            }

                            SkrToolButton {
                                id: selectProjectPathToolButton
                                text: qsTr("Select")
                            }
                        }
                        ColumnLayout {
                            id: columnLayout6s
                            SkrLabel {
                                id: projectDetailLabel
                                text: qsTr("This project will be created as : ")
                            }
                            SkrLabel {
                                id: projectDetailPathLabel
                            }
                        }
                        RowLayout {
                            id: rowLayout3
                            Layout.alignment: Qt.AlignHCenter
                            SkrLabel {
                                id: partLabel
                                text: qsTr("Number of parts :")
                            }

                            SkrSpinBox {
                                id: partSpinBox
                                value: 1
                                from: 1
                                to: 40
                                editable: true
                            }
                        }
                        RowLayout {
                            id: rowLayout4
                            Layout.alignment: Qt.AlignHCenter

                            SkrButton {
                                id: createNewProjectButton
                                text: qsTr("Create")
                                icon.color: SkrTheme.buttonIcon
                            }
                        }
                    }

                    Item {
                        id: stretcher2
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                    }
                }
            }

            Print {
                id: printItem
                enabled: false


            }

            Importer {
                id: importerItem
                enabled: false

            }

            Exporter {
                id: exporterItem
                enabled: false
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;height:800;width:800}
}
##^##*/

