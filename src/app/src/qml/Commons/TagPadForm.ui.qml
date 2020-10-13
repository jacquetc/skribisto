import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
    id: base
    property alias tagFlow: tagFlow
    property alias addTagMenuToolButton: addTagMenuToolButton
    property alias tagRepeater: tagRepeater
    
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

                    Item {
                        id: stretcher
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                    }

                    ToolButton {
                        id: addTagMenuToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                    }

                }
            }

            ScrollView {
                id: scrollView
                Layout.fillHeight: true
                Layout.minimumHeight: 40
                focusPolicy: Qt.StrongFocus
                Layout.fillWidth: true
                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded

                Flickable {
                    boundsBehavior: Flickable.StopAtBounds
                    contentWidth: scrollView.width
                    contentHeight: tagFlow.height
                    //contentHeight: leftDockColumnLayout.height
                    Flow {
                        id: tagFlow
                        width: scrollView.width
                        //                    width: parent.width
                        //                    anchors.left: parent.left
                        //                    anchors.top: parent.top
                        //                    anchors.topMargin: 0
                        //                    anchors.leftMargin: 0
                        spacing: 20
                        padding: 2
                        antialiasing: true
                        clip: true
                        focus: true

                        Repeater {
                            id: tagRepeater

                            delegate: tagFlowComponent


                        }

                        Accessible.role: Accessible.List
                        Accessible.name: qsTr("Tags list")
                        Accessible.description: "Empty list of tags"
                    }
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/
