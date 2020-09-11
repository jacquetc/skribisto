import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Commons"

Item {
    id: base
    property alias noteFlow: noteFlow
    property alias noteWritingZone: noteWritingZone
    property alias openSynopsisToolButton: openSynopsisToolButton
    property alias openNoteInNewTabToolButton: openNoteInNewTabToolButton
    property alias addNoteMenuToolButton: addNoteMenuToolButton
    property alias noteRepeater: noteRepeater

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
                Layout.fillWidth: true

                //                Item {
                //                    id: element
                //                    Layout.fillHeight: true
                //                    Layout.fillWidth: true
                //                }
                RowLayout {
                    id: rowLayout
                    spacing: 1
                    anchors.fill: parent

                    ToolButton {
                        id: openNoteInNewTabToolButton
                        text: qsTr("Open note")
                        flat: true
                        display: AbstractButton.IconOnly
                    }

                    ToolButton {
                        id: openSynopsisToolButton
                        flat: true
                        text: qsTr("Show synopsis")
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
                Layout.maximumHeight: 100
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
                    //contentHeight: leftDockColumnLayout.height
                    Flow {
                        id: noteFlow
                        width: scrollView.width
                        //                    width: parent.width
                        //                    anchors.left: parent.left
                        //                    anchors.top: parent.top
                        //                    anchors.topMargin: 0
                        //                    anchors.leftMargin: 0
                        spacing: 20
                        padding: 0
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
                placeholderText: "Type your note here..."
                Layout.fillWidth: true
                Layout.fillHeight: true
                stretch: true
                minimapVisibility: false
                leftScrollItemVisible: false
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:300}
}
##^##*/

