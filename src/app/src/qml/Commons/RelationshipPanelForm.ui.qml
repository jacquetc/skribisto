import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../Commons"
import "../Items"
import ".."

SkrPane {
    id: base
    property alias extendPanelButton: extendPanelButton
    property alias closeSubPanelButton: closeSubPanelButton
    property alias openTreeItemButton: openTreeItemButton
    property alias addQuickNoteItemButton: addQuickNoteItemButton
    property alias closePanelButton: closePanelButton
    property alias subPanelLoader: subPanelLoader
    property alias gridView: gridView
    property alias allTabButton: allTabButton
    property alias notesTabButton: notesTabButton
    property alias proposedTabButton: proposedTabButton
    property alias subPanel: subPanel

    RowLayout {
        id: rowLayout
        anchors.fill: parent

        RowLayout {
            id: rowLayout5
            Layout.fillHeight: true

            ColumnLayout {
                id: columnLayout
                Layout.fillHeight: true
                Layout.fillWidth: true

                SkrTabBar {
                    id: tabBar
                    Layout.fillWidth: true
                    SkrTabButton{
                        id: proposedTabButton
                        text: qsTr("Proposed")
                        visible: false
                        closable: false
                        height: 30
                        width: visible ? implicitWidth : 0

                    }
                    SkrTabButton{
                        id: notesTabButton
                        text: qsTr("Notes")
                        closable: false
                        height: 30
                        width: implicitWidth
                    }
                    SkrTabButton{
                        id: allTabButton
                        text: qsTr("All")
                        closable: false
                        height: 30
                        width: implicitWidth
                    }
                }

                ScrollView {
                    id: scrollView
                    clip: true
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                    focusPolicy: Qt.StrongFocus
                    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                    ScrollBar.vertical.policy: ScrollBar.AsNeeded //scrollBarVerticalPolicy

                    GridView {
                        id: gridView
                        smooth: true
                        boundsBehavior: Flickable.StopAtBounds

                        cellHeight: 70
                        cellWidth: 70

                        focus: true
                        clip: true
                        Accessible.name: qsTr("Relationship list")
                        Accessible.role: Accessible.List
                    }
                }
            }

            SkrPane {
                id: pane
                Layout.preferredWidth: childrenRect.width
                Layout.fillHeight: true
                elevation: 1
                padding: 0

                ColumnLayout {
                    id: columnLayout1
                    anchors.fill: parent

                    SkrToolButton {
                        id: extendPanelButton
                        text: qsTr("Extend this panel")
                        icon.source: extended ? "qrc:///icons/backup/go-down.svg" : "qrc:///icons/backup/go-up.svg"
                        Layout.preferredHeight: 30
                        Layout.preferredWidth: 30
                    }

                    SkrToolButton {
                        id: closeSubPanelButton
                        text: qsTr("Hide the sub panel")
                        icon.source: subPanel.visible ? "qrc:///icons/backup/go-next.svg" : "qrc:///icons/backup/go-previous.svg"
                        Layout.preferredHeight: 30
                        Layout.preferredWidth: 30
                    }

                    SkrToolButton {
                        id: openTreeItemButton
                        text: qsTr("Open item")
                        icon.source: "qrc:///icons/backup/quickopen-file.svg"
                        Layout.preferredHeight: 30
                        Layout.preferredWidth: 30
                    }

                    SkrToolButton {
                        id: addQuickNoteItemButton
                        icon.source: "qrc:///icons/backup/list-add.svg"
                        Layout.preferredHeight: 30
                        Layout.preferredWidth: 30
                    }


                    Item {
                        id: stretcher
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                    }
                    SkrToolButton {
                        id: closePanelButton
                        text: qsTr("Close this panel")
                        icon.source: "qrc:///icons/backup/go-down.svg"
                        Layout.preferredHeight: 30
                        Layout.preferredWidth: 30
                    }
                }
            }
        }

        Item {
            id: subPanel
            visible: false
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.minimumWidth: 200
            Layout.maximumWidth: 400

            ColumnLayout{
                anchors.fill: parent

                Loader {
                    id: subPanelLoader
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                    active: false
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}D{i:2}D{i:1}
}
##^##*/

