import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Controls.Material
import "../Commons"
import "../Items"
import ".."

Item {
    id: base
    property alias noteFlow: noteFlow
    property alias noteFlickable: noteFlickable
    property alias noteWritingZone: noteWritingZone
    property alias openSynopsisToolButton: openSynopsisToolButton
    property alias openNoteInNewTabToolButton: openNoteInNewTabToolButton
    property alias addNoteMenuToolButton: addNoteMenuToolButton
    property alias noteRepeater: noteRepeater
    property alias toolBar: toolBar
    property alias currentNoteTitleLabel: currentNoteTitleLabel
    property alias noteFlowFocusScope: noteFlowFocusScope

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
                Layout.maximumHeight: 40
                Layout.preferredHeight: 40
                Layout.alignment: Qt.AlignTop
                Layout.fillWidth: true
                visible: !minimalMode

                //                                 Item {
                //                    id: element
                //                    Layout.fillHeight: true
                //                    Layout.fillWidth: true
                //                }
                RowLayout {
                    id: rowLayout
                    spacing: 1
                    anchors.fill: parent


                    SkrLabel {
                        id: currentNoteTitleLabel
                        elide: Text.ElideRight
                        verticalAlignment: Qt.AlignVCenter
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                    }

                    SkrToolButton {
                        id: openSynopsisToolButton
                        flat: true
                        text: qsTr("Show outline")
                        display: AbstractButton.IconOnly
                    }

                    SkrToolButton {
                        id: openNoteInNewTabToolButton
                        text: qsTr("Open note")
                        flat: true
                        display: AbstractButton.IconOnly
                    }




                    SkrToolButton {
                        id: addNoteMenuToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                    }

                }
            }

            FocusScope {
                id: noteFlowFocusScope
                Layout.fillHeight: true
                Layout.fillWidth: true
                property int flowHeight: noteFlow.height + noteFlickable.topMargin + noteFlickable.bottomMargin
                Layout.preferredHeight: minimalMode ? -1 : (flowHeight > 200 ? 200 : flowHeight)

                ScrollView {
                    id: scrollView
                    anchors.fill: parent
                    focusPolicy: Qt.StrongFocus
                    clip: true
                    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                    ScrollBar.vertical.policy: ScrollBar.AsNeeded

                    Flickable {
                        id: noteFlickable
                        boundsBehavior: Flickable.StopAtBounds
                        contentWidth: scrollView.width
                        contentHeight: noteFlow.height

                        //needed to center vertically in overview tree
                        interactive: noteFlow.height > noteFlickable.height


                        Flow {
                            id: noteFlow
                            width: scrollView.width
                            spacing: 5
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
            }

            WritingZone {
                id: noteWritingZone
                placeholderText: qsTr("Type your note here …")
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.preferredHeight: 400
                stretch: true
                leftScrollItemVisible: false
                visible: !minimalMode

                textAreaStyleBackgroundColor: SkrTheme.secondaryTextAreaBackground
                textAreaStyleForegroundColor: SkrTheme.secondaryTextAreaForeground
                paneStyleBackgroundColor: SkrTheme.pageBackground
                textAreaStyleAccentColor: SkrTheme.accent
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:300}
}
##^##*/

