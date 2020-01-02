import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQml.Models 2.12

TreeViewForm {

    property var model
    onModelChanged: {
        visualModel.model = model
    }

    signal open(int projectId, int paperId)
    signal remove(int projectId, int paperId)
    signal clearBin
    signal addAfter(int projectId, int paperId)

    listView.model: visualModel
    DelegateModel {
        id: visualModel

        delegate: dragDelegate
    }

    // TreeView item :
    Component {
        id: dragDelegate

        MouseArea {
            id: dragArea

            anchors {
                left: parent.left
                right: parent.right
            }
            height: content.height
            property bool held: false

            drag.target: held ? content : undefined
            drag.axis: Drag.YAxis

            onPressAndHold: held = true
            onReleased: held = false

            Rectangle {
                id: content

                anchors {
                    horizontalCenter: parent.horizontalCenter
                    verticalCenter: parent.verticalCenter
                }
                width: dragArea.width
                height: 40

                Drag.active: dragArea.held
                Drag.source: dragArea
                Drag.hotSpot.x: width / 2
                Drag.hotSpot.y: height / 2

                color: dragArea.held ? "lightsteelblue" : "white"
                Behavior on color {
                    ColorAnimation {
                        duration: 100
                    }
                }

                RowLayout {
                    id: rowLayout
                    spacing: 2
                    anchors.fill: parent

                    Rectangle {
                        color: "transparent"
                        border.width: 1
                        Layout.fillWidth: true
                        Layout.fillHeight: true

                        ColumnLayout {
                            id: columnLayout2
                            spacing: 1
                            anchors.fill: parent
                            Layout.fillHeight: true
                            Layout.fillWidth: true

                            Label {
                                id: titleLabel

                                Layout.fillWidth: true
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter

                                text: model.name
                                Layout.topMargin: 2
                                Layout.leftMargin: 4
                            }
                            Label {
                                id: tagLabel

                                text: model.tag
                                Layout.bottomMargin: 2
                                Layout.rightMargin: 4
                                Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                            }
                        }
                        //                        MouseArea {
                        //                            anchors.fill: parent
                        //                        }
                    }

                    ToolButton {
                        id: menuButton
                        Layout.fillHeight: true
                        Layout.preferredWidth: 30

                        text: "..."
                        flat: true

                        onClicked: menu.open()
                    }

                    ToolButton {
                        id: goToChildButton

                        //                            background: Rectangle {
                        //                                implicitWidth: 30
                        //                                implicitHeight: 30
                        //                                color: Qt.darker(
                        //                                           "#33333333", control.enabled
                        //                                           && (control.checked
                        //                                               || control.highlighted) ? 1.5 : 1.0)
                        //                                opacity: enabled ? 1 : 0.3
                        //                                visible: control.down
                        //                                         || (control.enabled
                        //                                             && (control.checked
                        //                                                 || control.highlighted))
                        //                            }
                        text: "+"
                        flat: false
                        Layout.preferredWidth: 30
                        Layout.fillHeight: true
                        Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                    }
                }
            }
            DropArea {
                id: dropArea
                anchors {
                    fill: parent
                    margins: 10
                }

                onEntered: {
                    visualModel.items.move(
                                drag.source.DelegateModel.itemsIndex,
                                dragArea.DelegateModel.itemsIndex)
                }
            }
            states: State {
                when: dragArea.held

                ParentChange {
                    target: content
                    parent: base
                }
                AnchorChanges {
                    target: content
                    anchors {
                        horizontalCenter: undefined
                        verticalCenter: undefined
                    }
                }
            }

            Menu {
                id: menu
                y: menuButton.height
                Action {
                    text: qsTr("Rename")
                }

                MenuSeparator {}
                Action {
                    text: qsTr("Remove")
                }
                MenuSeparator {}

                Menu {
                    title: qsTr("Advanced")
                    Action {
                        text: qsTr("Reorder alphabetically")
                    }
                }
            }

            ListView.onRemove: SequentialAnimation {
                PropertyAction {
                    target: dragArea
                    property: "ListView.delayRemove"
                    value: true
                }
                NumberAnimation {
                    target: dragArea
                    property: "height"
                    to: 0
                    easing.type: Easing.InOutQuad
                }
                PropertyAction {
                    target: dragArea
                    property: "ListView.delayRemove"
                    value: false
                }
            }

            // move :
        }
    }
}
