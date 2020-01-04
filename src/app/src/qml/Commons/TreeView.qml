import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQml.Models 2.12

TreeViewForm {
    id: root

    property var proxyModel

    property var model
    onModelChanged: {
        visualModel.model = model
    }

    signal open(int projectId, int paperId)
    signal remove(int projectId, int paperId)
    signal clearBin
    signal addAfter(int projectId, int paperId)
    property int currentParent: -2
    property int currentProject: -2

    listView.model: visualModel
    DelegateModel {
        id: visualModel

        delegate: dragDelegate
    }

    //-----------------------------------------------------------------------------
    // go up button :
    goUpToolButton.onClicked: {
        currentParent = proxyModel.goUp()
    }

    //-----------------------------------------------------------------------------
    // current parent button :
    Binding {
        target: root
        property: "currentProject"
        value: proxyModel.projectIdFilter
    }
    Binding {
        target: root
        property: "currentParent"
        value: proxyModel.parentIdFilter
    }
    //currentParent: proxyModel.parentIdFilter
    //currentProject: proxyModel.projectIdFilter
    onCurrentParentChanged: {
        if (currentParent != -2 & currentProject != -2) {
            currentParentToolButton.text = proxyModel.getItemName(
                        currentProject, currentParent)
            //console.log("onCurrentParentChanged")
        }
    }
    onCurrentProjectChanged: {
        if (currentParent != -2 & currentProject != -2) {
            currentParentToolButton.text = proxyModel.getItemName(
                        currentProject, currentParent)
            //console.log("onCurrentProjectChanged")
        }
    }

    currentParentToolButton.onClicked: {

        //currentParent
    }

    //-----------------------------------------------------------------------------
    Component.onCompleted: {

    }

    //-----------------------------------------------------------------------------

    // used to remember the source when moving an item
    property int moveSourceInt: -2

    // TreeView item :
    Component {
        id: dragDelegate

        DropArea {
            id: delegateRoot
            onEntered: {

                content.sourceIndex = drag.source.visualIndex
                visualModel.items.move(drag.source.visualIndex,
                                       content.visualIndex)
            }

            onDropped: {
                console.log("dropped : ", moveSourceInt, content.visualIndex)
                proxyModel.moveItem(moveSourceInt, content.visualIndex)
            }
            property int visualIndex: {
                return DelegateModel.itemsIndex
            }

            Binding {
                target: content
                property: "visualIndex"
                value: visualIndex
            }
            anchors {
                left: parent.left
                right: parent.right
            }
            height: content.height

            //            drag.target: held ? content : undefined
            //            drag.axis: Drag.YAxis

            //            onPressAndHold: held = true
            //            onReleased: held = false
            Rectangle {
                id: content
                property int visualIndex: 0
                property int sourceIndex: -2

                property bool isCurrent: model.index === listView.currentIndex ? true : false

                anchors {
                    horizontalCenter: parent.horizontalCenter
                    verticalCenter: parent.verticalCenter
                }
                width: delegateRoot.width
                height: 40

                Drag.active: dragHandler.active
                Drag.source: content
                Drag.hotSpot.x: width / 2
                Drag.hotSpot.y: height / 2

                color: dragHandler.active | !tapHandler.enabled ? "lightsteelblue" : "white"
                Behavior on color {
                    ColorAnimation {
                        duration: 100
                    }
                }

                DragHandler {
                    id: dragHandler
                    //acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                    //xAxis.enabled: false
                    //grabPermissions: PointerHandler.TakeOverForbidden
                    onActiveChanged: {
                        if (active) {
                            moveSourceInt = content.visualIndex
                        } else {
                            content.Drag.drop()
                            tapHandler.enabled = true
                        }
                    }
                    enabled: !tapHandler.enabled
                }

                //                MouseArea {
                //                    id: mouseArea
                //                    z: 2
                //                    anchors.fill: parent
                //                    enabled: false
                //                    hoverEnabled: true
                //                    //propagateComposedEvents: true
                //                    onClicked: {
                //                        listView.currentIndex = model.index
                //                        console.log("eeeee")
                //                    }
                //                    //                    onPressAndHold: {
                //                    //                        mouse.accepted = false
                //                    //                    }
                //                }
                HoverHandler {
                    id: hoverHandler
                }

                TapHandler {
                    id: tapHandler
                    onTapped: {
                        listView.currentIndex = model.index
                    }
                    onLongPressed: {
                        enabled = false
                    }
                }

                //                WheelHandler {
                //                    id: wheelHandler
                //                    enabled: dragHandler.enabled
                //                }

                /// without MouseArea, it breaks while dragging and scrolling:
                MouseArea {
                    anchors.fill: parent
                    onWheel: {
                        //                        console.log('wheel', wheel.angleDelta.y)
                        //                        listView.flick(wheel.angleDelta.y, 0)
                        wheel.accepted = true
                    }

                    enabled: dragHandler.enabled
                }
                RowLayout {
                    id: rowLayout
                    spacing: 2
                    anchors.fill: parent

                    Rectangle {
                        id: currentItemIndicator
                        color: "#cccccc"
                        Layout.fillHeight: true
                        Layout.preferredWidth: 20
                        visible: listView.currentIndex === model.index
                    }

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

                                //                                text: model.tag
                                text: model.sortOrder
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

                        visible: hoverHandler.hovered | content.isCurrent
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
                        visible: hoverHandler.hovered | content.isCurrent

                        onClicked: {
                            currentProject = model.projectId
                            currentParent = model.paperId
                            proxyModel.setParentFilter(model.projectId,
                                                       model.paperId)
                        }
                    }
                }
            }
            //            DropArea {
            //                id: dropArea
            //                anchors {
            //                    fill: parent
            //                    margins: 10
            //                }
            //                property int sourceIndex: -1
            //                property int targetIndex: -1
            //                onEntered: {
            //                    sourceIndex = drag.source.DelegateModel.itemsIndex
            //                    targetIndex = dragArea.DelegateModel.itemsIndex
            //                    //                    var sourceIndex = drag.source.DelegateModel.itemsIndex
            //                    //                    var targetIndex = dragArea.DelegateModel.itemsIndex
            //                    visualModel.items.move(sourceIndex, targetIndex)

            //                    //                    var sourceModelIndex = drag.source.DelegateModel.modelIndex(
            //                    //                                sourceIndex)
            //                    //                    var targetModelIndex = dragArea.DelegateModel.modelIndex(
            //                    //                                targetIndex)

            //                    //                    console.log("targetIndex : ", sourceModelIndex.name)
            //                }

            //                onDropped: {
            //                    console.log("onDropped")
            //                }
            //            }
            states: State {
                when: content.Drag.active

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
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: true
                }
                NumberAnimation {
                    target: delegateRoot
                    property: "height"
                    to: 0
                    easing.type: Easing.InOutQuad
                }
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: false
                }
            }

            // move :
        }
    }
}
