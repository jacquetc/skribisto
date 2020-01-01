import QtQuick 2.11
import QtQuick.Controls 2.4
import QtQuick.Layouts 1.3
import ".."

Item {
    id: base
    width: 1000
    height: 600
    property alias compactHeaderPane: compactHeaderPane
    property alias compactRightDockShowButton: compactRightDockShowButton
    property alias compactLeftDockShowButton: compactLeftDockShowButton
    property alias leftDockMenuGroup: leftDockMenuGroup
    property alias leftDockResizeButton: leftDockResizeButton
    property alias leftDockMenuButton: leftDockMenuButton
    property alias leftDock: leftDock
    property alias leftDockShowButton: leftDockShowButton
    property alias minimap: minimap
    property alias middleBase: middleBase
    property alias writingZone: writingZone
    property alias base: base
    property int middleBasePreferredWidth

    ColumnLayout {
        id: columnLayout
        spacing: 0
        anchors.fill: parent

        Pane {
            id: compactHeaderPane
            width: 200
            height: 200
            padding: 2
            Layout.fillHeight: false
            Layout.preferredHeight: 50
            Layout.fillWidth: true

            RowLayout {
                id: rowLayout1
                spacing: 2
                anchors.fill: parent
                RowLayout {
                    id: rowLayout2
                    spacing: 2
                    Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                    Button {
                        id: compactLeftDockShowButton
                        Layout.preferredHeight: 50
                        Layout.preferredWidth: 50
                        flat: true
                        Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                    }
                }
                RowLayout {
                    id: rowLayout3
                    spacing: 2
                    Layout.alignment: Qt.AlignRight | Qt.AlignTop
                    Button {
                        id: compactRightDockShowButton
                        Layout.preferredHeight: 50
                        Layout.preferredWidth: 50
                        flat: true
                        Layout.alignment: Qt.AlignRight | Qt.AlignTop
                    }
                }
            }
        }

        RowLayout {
            id: rowLayout
            Layout.fillHeight: true
            Layout.fillWidth: true
            spacing: 0

            Item {
                id: leftBase
                Layout.minimumWidth: 200
                visible: !Globals.compactSize
                Layout.fillHeight: true
                Layout.preferredWidth: Globals.compactSize ? -1 : base.width / 6
                z: 1

                RowLayout {
                    anchors.fill: parent

                    NotesLeftDock {
                        id: leftDock
                        Layout.fillHeight: true
                    }
                    ColumnLayout {
                        Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                        Layout.maximumWidth: 60
                        Button {
                            id: leftDockShowButton
                            Layout.preferredHeight: 50
                            Layout.preferredWidth: 50
                            flat: true
                            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                        }

                        Button {
                            id: leftDockMenuButton
                            checkable: true
                            Layout.preferredHeight: 50
                            Layout.preferredWidth: 50
                            flat: true
                            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                        }

                        ColumnLayout {
                            id: leftDockMenuGroup
                            Layout.alignment: Qt.AlignLeft | Qt.AlignTop

                            Button {
                                id: leftDockResizeButton
                                Layout.preferredHeight: 50
                                Layout.preferredWidth: 50
                                flat: true
                                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                            }
                        }
                    }
                }
            }

            Item {
                id: middleBase
                width: 400
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                Layout.fillHeight: true
                Layout.minimumWidth: 200
                Layout.fillWidth: Globals.compactSize
                Layout.preferredWidth: Globals.compactSize ? -1 : base.width / 3 * 2

                WritingZone {
                    id: writingZone
                    anchors.fill: parent
                }
            }

            Item {

                id: rightBase
                z: 1
                Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                Layout.minimumWidth: 200
                Layout.fillHeight: true
                Layout.fillWidth: false
                Layout.preferredWidth: Globals.compactSize ? -1 : base.width / 6

                RowLayout {
                    visible: false
                    anchors.fill: parent

                    Minimap {
                        Layout.fillHeight: true
                        Layout.alignment: Qt.AlignRight | Qt.AlignVCenter

                        id: minimap
                        //width: minimapFlickable.width
                        //pageSize: (writingZone.flickable.height) / (writingZone.textArea.contentHeight + 16)
                        //textScale: minimapFlickable.height / writingZone.flickable.contentHeight
                        sourceWidth: writingZone.textArea.width
                        sourcePointSize: writingZone.textArea.font.pointSize
                    }

                    //                ScrollBar {
                    //                    id: minimapScrollBar
                    //                    visible: true
                    //                    active: true
                    //                    background:ScrollView {
                    //                        TextEdit {
                    //                            scale: minimapFlickable.height / writingZone.flickable.contentHeight
                    //                            textFormat: TextEdit.RichText
                    //                            visible: true
                    //                            readOnly: true
                    //                            wrapMode: TextEdit.WrapAtWordBoundaryOrAnywhere

                    //                            text: writingZone.textArea.text
                    //                        }
                    //}

                    //                    contentItem: Rectangle {
                    //                        id: content
                    //                        implicitWidth: 6
                    //                        implicitHeight: 20
                    //                        radius: 5
                    //                        visible: true
                    //                        border.color: "red"
                    //                        color: minimapScrollBar.pressed ? "#81e889" : "transparent"
                    //                    }
                    //                }
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:17;anchors_height:100;anchors_width:100}
}
##^##*/

