import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.15
import "../Commons"

Item {
    id: base
    property alias noteFlow: noteFlow
    property alias noteWritingZone: noteWritingZone
    property alias openSynopsisToolButton: openSynopsisToolButton
    property alias openNoteInNewTabToolButton: openNoteInNewTabToolButton
    property alias addNoteMenuToolButton: addNoteMenuToolButton
    property alias noteRepeater: noteRepeater
    property alias toolBar: toolBar

    property bool minimalMode: false
    property var toolBarPrimaryColor


    Pane {
        id: pane
        clip: true
        anchors.fill: parent

        ColumnLayout {
            id: columnLayout
            spacing: 0
            anchors.fill: parent

            ToolBar {
                id: toolBar
                Layout.maximumHeight: 40
                Layout.preferredHeight: 40
                Layout.alignment: Qt.AlignTop
                Layout.fillWidth: true
                visible: !minimalMode

                Material.primary: toolBarPrimaryColor
                Material.elevation: 2
                //                                 Item {
                //                    id: element
                //                    Layout.fillHeight: true
                //                    Layout.fillWidth: true
                //                }
                RowLayout {
                    id: rowLayout
                    spacing: 1
                    anchors.fill: parent

                    Item {
                        id: stretcher
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                    }

                    ToolButton {
                        id: openSynopsisToolButton
                        flat: true
                        text: qsTr("Show outline")
                        display: AbstractButton.IconOnly
                    }

                    ToolButton {
                        id: openNoteInNewTabToolButton
                        text: qsTr("Open note")
                        flat: true
                        display: AbstractButton.IconOnly
                    }




                    ToolButton {
                        id: addNoteMenuToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                    }

                }
            }

            ScrollView {
                id: scrollView
                //Layout.fillHeight: minimalMode ? false : true
                Layout.minimumHeight: 40
                focusPolicy: Qt.StrongFocus
                Layout.fillWidth: true
                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded

                Flickable {
                    boundsBehavior: Flickable.StopAtBounds
                    contentWidth: scrollView.width
                    contentHeight: noteFlow.height
                    Flow {
                        id: noteFlow
                        width: scrollView.width
                        spacing: 10
                        padding: 2
                        antialiasing: true
                        clip: true
                        focus: true

                        Repeater {
                            id: noteRepeater
                            delegate: noteFlowComponent

                        }

                        Accessible.role: Accessible.List
                        Accessible.name: qsTr("Notes list")
                        Accessible.description: "Empty list of related notes"
                    }
                }
            }

            WritingZone {
                id: noteWritingZone
                placeholderText: qsTr("Type your note here...")
                Layout.fillWidth: true
                Layout.fillHeight: true
                stretch: true
                leftScrollItemVisible: false
                visible: !minimalMode
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:300}
}
##^##*/

