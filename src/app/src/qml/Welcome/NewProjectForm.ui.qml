import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import "../Commons"
import ".."

Item {

    property alias projectFileTextField: projectFileTextField
    property alias createNewProjectButton: createNewProjectButton
    property alias partSpinBox: partSpinBox
    property alias projectDetailPathLabel: projectDetailPathLabel
    property alias selectProjectPathToolButton: selectProjectPathToolButton
    property alias projectPathTextField: projectPathTextField
    property alias projectTitleTextField: projectTitleTextField
    property alias goBackToolButton: goBackToolButton


    readonly property int columnWidth: 550

    Item {
        id: newProjectItem
        anchors.fill: parent

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
                columns: 1
                flow: GridLayout.TopToBottom
                Layout.alignment: Qt.AlignHCenter
                Layout.fillWidth: true

                GridLayout {
                    id: gridLayout1
                    rows: 2
                    columns: 3
                    Layout.fillWidth: true
                    Layout.maximumWidth: 800

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

                    SkrButton {
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
}


/*##^##
Designer {
    D{i:0;height:800;width:800}
}
##^##*/

