import QtQuick
import QtQml
import QtQuick.Layouts
import QtQuick.Controls

import "../../Items"
import "../../Commons"
import "../.."

Item {
    property alias goBackToolButton: goBackToolButton
    property alias importPlumeProjectButton: importPlumeProjectButton
    property alias plumeProjectDetailPathLabel: plumeProjectDetailPathLabel
    property alias selectPlumeProjectFileToolButton: selectPlumeProjectFileToolButton
    property alias plumeProjectFileTextField: plumeProjectFileTextField
    readonly property int columnWidth: 550

    clip: true
    ColumnLayout {
        id: columnLayout6
        anchors.fill: parent

        RowLayout {
            id: rowLayout7
            Layout.fillWidth: true

            SkrToolButton {
                id: goBackToolButton
                text: qsTr("Go back")
            }

            SkrLabel {
                id: titleLabel2
                text: qsTr("<h2>Import Plume Creator project</h2>")
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
            }
        }

        ScrollView {
            id: scrollView
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
            ScrollBar.vertical.policy: ScrollBar.AsNeeded
            contentWidth: pillarLayout.width
            contentHeight: pillarLayout.implicitHeight



            SKRPillarLayout {
                id: pillarLayout
                columns: ((pillarLayout.width / columnWidth) | 0 )
                width: scrollView.width
                maxColumns: 1

                RowLayout {
                    id: rowLayout
                    Layout.fillWidth: true
                    Layout.maximumWidth: 800

                    SkrLabel {
                        id: plumeProjectFileLabel
                        text: qsTr("Plume project file (*.plume) :")
                    }
                    SkrTextField {

                        id: plumeProjectFileTextField
                        placeholderText: qsTr("plume project file")
                        Layout.columnSpan: 2
                        Layout.fillWidth: true
                    }
                    SkrButton {
                        id: selectPlumeProjectFileToolButton
                        text: qsTr("Select")
                    }
                }
                ColumnLayout {
                    id: columnLayout8
                    SkrLabel {
                        id: plumeProjectDetailLabel
                        text: qsTr(
                                  "This project will be imported as : ")
                    }
                    SkrLabel {
                        id: plumeProjectDetailPathLabel
                    }
                }
                RowLayout {
                    id: rowLayout8
                    Layout.alignment: Qt.AlignHCenter

                    SkrButton {
                        id: importPlumeProjectButton
                        text: qsTr("Import")
                        icon.color: SkrTheme.buttonIcon
                    }
                }

            }
        }


    }
}

