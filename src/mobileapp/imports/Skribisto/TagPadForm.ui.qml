import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Controls.Material
import SkrControls
import ".."

Item {
    id: base
    property alias tagFlow: tagFlow
    property alias tagFlickable: tagFlickable
    property alias addTagMenuToolButton: addTagMenuToolButton
    property alias tagRepeater: tagRepeater
    property alias tagFlowFocusScope: tagFlowFocusScope
    property bool minimalMode: false
    implicitHeight: columnLayout.childrenRect.height

    SkrPane {
        id: pane
        clip: true
        anchors.fill: parent
        padding: minimalMode ? 0 : undefined
        Material.background: "transparent"

        ColumnLayout {
            id: columnLayout
            spacing: 0
            anchors.top: parent.top
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.bottom:  minimalMode ? parent.bottom : undefined

            SkrToolBar {
                id: toolBar
                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
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

                    SkrLabel {
                        id: tagPadLabel
                        text: qsTr("Tags")
                        elide: Text.ElideRight
                        verticalAlignment: Qt.AlignVCenter
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                    }


                    SkrToolButton {
                        id: addTagMenuToolButton
                        flat: true
                        Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                    }

                }
            }

            FocusScope {
                id: tagFlowFocusScope
                Layout.fillHeight: true
                Layout.fillWidth: true
                property int flowHeight: tagFlow.height + tagFlickable.topMargin + tagFlickable.bottomMargin
                Layout.preferredHeight: minimalMode ? -1 : (flowHeight  * SkrSettings.interfaceSettings.zoom > 200   * SkrSettings.interfaceSettings.zoom ? 200   * SkrSettings.interfaceSettings.zoom : flowHeight   * SkrSettings.interfaceSettings.zoom)

                ScrollView {
                    id: scrollView
                    anchors.fill: parent
                    focusPolicy: Qt.StrongFocus
                    clip: true
                    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                    ScrollBar.vertical.policy: ScrollBar.AsNeeded

                    Flickable {
                        id: tagFlickable
                        boundsBehavior: Flickable.StopAtBounds
                        contentWidth: scrollView.width
                        contentHeight: tagFlow.height

                        //needed to center vertically in overview tree
                        interactive: tagFlow.height > tagFlickable.height

                        Flow {
                            id: tagFlow
                            width: scrollView.width
                            spacing: 5
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
