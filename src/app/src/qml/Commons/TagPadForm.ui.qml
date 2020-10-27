import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import ".."

Item {
    id: base
    property alias tagFlow: tagFlow
    property alias addTagMenuToolButton: addTagMenuToolButton
    property alias tagRepeater: tagRepeater
    property bool minimalMode: false

    SkrPane {
        id: pane
        clip: true
        anchors.fill: parent

        ColumnLayout {
            id: columnLayout
            spacing: 0
            anchors.fill: parent

            SkrToolBar {
                id: toolBar
                Layout.maximumHeight: 40
                Layout.preferredHeight: 40
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignTop
                visible: !minimalMode

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

                    SkrToolButton {
                        id: addTagMenuToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                    }

                }
            }

            ScrollView {
                id: scrollView
                Layout.fillHeight: !minimalMode
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
                    Flow {
                        id: tagFlow
                        width: scrollView.width
                        spacing: 10
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

            Item {
                id: stretcher2
                Layout.fillHeight: true
                Layout.fillWidth: true
                visible: !minimalMode
            }

        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/
