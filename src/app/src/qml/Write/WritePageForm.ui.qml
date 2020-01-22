import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import ".."

Item {
    id: base
    width: 1000
    height: 600
    property alias compactHeaderPane: compactHeaderPane
    property alias compactRightDockShowButton: compactRightDockShowButton
    property alias compactLeftDockShowButton: compactLeftDockShowButton
    property alias leftDockMenuGroup: leftDockMenuGroup
    property alias rightDockMenuGroup: rightDockMenuGroup
    property alias leftDockResizeButton: leftDockResizeButton
    property alias rightDockResizeButton: rightDockResizeButton
    property alias leftDockMenuButton: leftDockMenuButton
    property alias rightDockMenuButton: rightDockMenuButton
    property alias leftDock: leftDock
    property alias rightDock: rightDock
    property alias leftDockShowButton: leftDockShowButton
    property alias rightDockShowButton: rightDockShowButton
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
                Layout.minimumWidth: 400
                visible: !Globals.compactSize
                Layout.fillHeight: true
                Layout.preferredWidth: Globals.compactSize ? -1 : base.width / 6
                z: 1

                RowLayout {
                    anchors.fill: parent

                    WriteLeftDock {
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
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                Layout.fillHeight: true
                Layout.fillWidth: true

                WritingZone {
                    id: writingZone
                    anchors.fill: parent
                }
            }

            Item {
                id: rightBase
                Layout.minimumWidth: 400
                visible: !Globals.compactSize
                Layout.fillHeight: true
                Layout.preferredWidth: Globals.compactSize ? -1 : base.width / 6
                z: 1

                RowLayout {
                    anchors.fill: parent
                    ColumnLayout {
                        Layout.alignment: Qt.AlignRight | Qt.AlignTop
                        Layout.maximumWidth: 60
                        Button {
                            id: rightDockShowButton
                            Layout.preferredHeight: 50
                            Layout.preferredWidth: 50
                            flat: true
                            Layout.alignment: Qt.AlignRight | Qt.AlignTop
                        }

                        Button {
                            id: rightDockMenuButton
                            checkable: true
                            Layout.preferredHeight: 50
                            Layout.preferredWidth: 50
                            flat: true
                            Layout.alignment: Qt.AlignRight | Qt.AlignTop
                        }

                        ColumnLayout {
                            id: rightDockMenuGroup
                            Layout.alignment: Qt.AlignRight | Qt.AlignTop

                            Button {
                                id: rightDockResizeButton
                                Layout.preferredHeight: 50
                                Layout.preferredWidth: 50
                                flat: true
                                Layout.alignment: Qt.AlignRight | Qt.AlignTop
                            }
                        }
                    }
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
                    WriteRightDock {
                        id: rightDock
                        Layout.fillHeight: true
                    }
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;width:1200}D{i:17;anchors_height:100;anchors_width:100}
}
##^##*/

