import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

Item {
    width: 400
    height: 400
    property alias projectFileTextField: projectFileTextField
    property alias createNewProjectButton: createNewProjectButton
    property alias partSpinBox: partSpinBox
    property alias projectDetailPathLabel: projectDetailPathLabel
    property alias selectProjectPathToolButton: selectProjectPathToolButton
    property alias projectPathTextField: projectPathTextField
    property alias projectTitleTextField: projectTitleTextField
    property alias goBackToolButton: goBackToolButton
    property alias createEmpyProjectAtStartSwitch: createEmpyProjectAtStartSwitch
    property alias gridLayout: gridLayout
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
    property alias folderDialogLoader: folderDialogLoader
    property string fileName: fileName

    Pane {
        id: pane1
        anchors.fill: parent

        SwipeView {
            id: swipeView
            currentIndex: 0
            anchors.fill: parent
            interactive: false
            clip: true

            Item {

                GridLayout {
                    id: gridLayout
                    anchors.fill: parent
                    flow: GridLayout.TopToBottom

                    GroupBox {
                        id: groupBox2
                        title: qsTr("Group Box")

                        ColumnLayout {
                            id: columnLayout2
                            anchors.fill: parent

                            RowLayout {
                                id: rowLayout1
                                width: 100
                                height: 100

                                Button {
                                    id: saveButton
                                    text: qsTr("Save")
                                    icon {
                                        name: "document-save"
                                    }
                                }

                                Button {
                                    id: saveAsButton
                                    text: qsTr("Save as")
                                    icon {
                                        name: "document-save-as"
                                    }
                                }
                                Button {
                                    id: saveACopyButton
                                    text: qsTr("Save a copy")
                                    icon {
                                        name: "document-save-as-template"
                                    }
                                }
                                Button {
                                    id: backUpButton
                                    text: qsTr("Back up")
                                    icon {
                                        name: "tools-media-optical-burn-image"
                                    }
                                }
                                Button {
                                    id: saveAllButton
                                    text: qsTr("Save all")
                                    icon {
                                        name: "document-save-all"
                                    }
                                }
                            }
                        }
                    }

                    GroupBox {
                        id: groupBox1
                        width: 200
                        height: 200
                        title: qsTr("Projects")

                        ColumnLayout {
                            id: columnLayout
                            anchors.fill: parent

                            RowLayout {
                                id: rowLayout
                                width: 100
                                height: 100

                                Button {
                                    id: newProjectButton
                                    text: qsTr("New project")
                                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                                }

                                Button {
                                    id: openProjectButton
                                    text: qsTr("Open project")
                                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                                }
                            }

                            ListView {
                                id: recentListView
                                width: 110
                                height: 160
                                clip: true
                                Layout.preferredHeight: 200
                                Layout.preferredWidth: 300
                                Layout.fillHeight: false
                                Layout.minimumHeight: 200
                                Layout.minimumWidth: 300
                                Layout.fillWidth: false
                                keyNavigationWraps: false
                                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
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
                                            font.bold: true
                                            anchors.verticalCenter: parent.verticalCenter
                                        }
                                    }
                                }
                            }
                        }
                    }

                    GroupBox {
                        id: groupBox
                        width: 200
                        height: 200
                        title: qsTr("")

                        ColumnLayout {
                            id: columnLayout1
                            anchors.fill: parent

                            Switch {
                                id: createEmpyProjectAtStartSwitch
                                text: qsTr("Create an empty project at start")
                                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                            }
                        }
                    }

                    GroupBox {
                        id: groupBox3
                        width: 200
                        height: 200
                        title: qsTr("Group Box")

                        ColumnLayout {
                            id: columnLayout4
                            anchors.fill: parent

                            RowLayout {
                                id: rowLayout5
                                width: 100
                                height: 100

                                Button {
                                    id: printButton
                                    text: qsTr("Print")
                                }

                                Button {
                                    id: exportButton
                                    text: qsTr("Export")
                                }
                            }
                        }
                    }

                    Item {
                        id: stretcher1
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                    }

                }
            }

            Item {
                ColumnLayout {
                    id: columnLayout3
                    anchors.fill: parent

                    RowLayout {
                        id: rowLayout2
                        Layout.fillWidth: true

                        ToolButton {
                            id: goBackToolButton
                            text: qsTr("Go back")
                        }

                        Label {
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

                            Label {
                                id: projectTitleLabel
                                text: qsTr("Project title :")
                            }
                            TextField {

                                id: projectTitleTextField
                                placeholderText: qsTr("project title")
                                Layout.columnSpan: 2
                                Layout.fillWidth: true
                            }
                            Label {
                                id: projectFileLabel
                                text: qsTr("Project file :")
                            }
                            TextField {

                                id: projectFileTextField
                                placeholderText: qsTr("project file")
                                Layout.columnSpan: 2
                                Layout.fillWidth: true
                            }
                            Label {
                                id: projectPathLabel
                                text: qsTr("Project path :")
                            }

                            TextField {
                                id: projectPathTextField
                                placeholderText: qsTr("project path")
                                Layout.fillWidth: true
                            }

                            ToolButton {
                                id: selectProjectPathToolButton
                                text: qsTr("Select")
                            }
                        }
                        ColumnLayout {
                            id: columnLayout6s
                            Label {
                                id: projectDetailLabel
                                text: qsTr("This project will be created as : ")
                            }
                            Label {
                                id: projectDetailPathLabel
                                text: fileName
                            }
                        }
                        RowLayout {
                            id: rowLayout3
                            Layout.alignment: Qt.AlignHCenter
                            Label {
                                id: partLabel
                                text: qsTr("Number of parts :")
                            }

                            SpinBox {
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

                            Button {
                                id: createNewProjectButton
                                text: qsTr("Create")
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
            Item {

                Loader {
                    id: folderDialogLoader
                    anchors.fill: parent
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;height:800;width:800}D{i:11}D{i:2}
}
##^##*/

