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

    signal openDocument(int projectId, int paperId)
    signal removeDocument(int projectId, int paperId)
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
    goUpToolButton.action: goUpAction

    Action {
        id: goUpAction
        text: qsTr("Go up")
        //shortcut: "Left,Backspace" Doesn't work well
        icon {
            name: "go-parent-folder"
        }
        enabled: root.visible
        onTriggered: {
            currentParent = proxyModel.goUp()
            listView.currentIndex = proxyModel.getLastOfHistory(currentProject)
            proxyModel.removeLastOfHistory(currentProject)
            //proxyModel.setForcedCurrentIndex(0)
            console.log("go up action")
        }
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

    //----------------------------------------------------------------------------
    treeMenuToolButton.icon.name: "overflow-menu"
    treeMenuToolButton.onClicked: navigationMenu.open()

    Menu {
        id: navigationMenu
        y: treeMenuToolButton.height

        //        Action {
        //            text: qsTr("Rename")
        //        }

        //        MenuSeparator {}
        //        Action {
        //            text: qsTr("Remove")
        //        }
        //        MenuSeparator {}
        Action {
            text: qsTr("Paste")
            shortcut: StandardKey.Paste
            icon.name: "edit-paste"
        }
        Menu {

            title: qsTr("Advanced")
            Action {
                text: qsTr("Sort alphabetically")
                icon.name: "view-sort-ascending-name"
            }
        }
    }

    //----------------------------------------------------------------------------
    addToolButton.icon.name: "document-new"
    addToolButton.onClicked: {
        proxyModel.addItemAtEnd(currentProject, currentParent,
                                visualModel.items.count - 1)
        listView.currentItem.editName()
    }

    //----------------------------------------------------------------------------

    // shortcuts

    //listView.focus: true
    //    listView.Keys.onLeftPressed: {

    //        console.log("onLeftPressed")
    //        goUpAction.trigger()
    //    }
    //    listView.Keys.onBackPressed: {

    //        console.log("onBackPressed")
    //        goUpAction.trigger()

    //    }
    Shortcut {
        sequences: ["Left", "Backspace"]
        onActivated: goUpAction.trigger()
        enabled: root.visible
    }
    //-----------------------------------------------------------------------------
    Component.onCompleted: {

    }

    //-----------------------------------------------------------------------------
    listView.onCurrentIndexChanged: {
        contextMenuItemIndex = listView.currentIndex
    }

    property int contextMenuItemIndex: -2
    property int itemButtonsIndex: -2

    Binding {
        target: listView
        property: "currentIndex"
        value: proxyModel.forcedCurrentIndex
        //when: proxyModel.onForcedCurrentIndexChanged
    }

    //----------------------------------------------------------------------------

    //----------------------------------------------------------------------------

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
            //            Shortcut {
            //                sequence: "Ctrl+Up"
            //                onActivated: moveUpAction.trigger(delegateRoot)
            //            }
            //            Keys.onShortcutOverride: {
            //                if (event.key === Qt.Key_Backspace) {
            //                    console.log("onShortcutOverride")
            //                    event.accepted = true
            //                }
            //            }
            //            Keys.onBackPressed: {
            //                console.log("eee")
            //            }
            function editName() {
                state = "edit_name"
                titleTextField.forceActiveFocus()
                titleTextField.selectAll()
            }

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

                HoverHandler {
                    id: hoverHandler
                }

                TapHandler {
                    id: tapHandler
                    onTapped: {

                        listView.currentIndex = model.index
                        delegateRoot.forceActiveFocus()
                    }
                    onDoubleTapped: {
                        openDocumentAction.trigger()
                    }

                    onLongPressed: {
                        enabled = false
                    }
                }
                TapHandler {
                    acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                    acceptedButtons: Qt.RightButton
                    onTapped: menu.open()
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

                Action {
                    id: goToChildAction
                    shortcut: "Right"
                    enabled: {
                        if (listView.currentIndex === model.index) {
                            return true
                        } else if (hoverHandler.hovered) {
                            return true
                        } else
                            return false
                    }

                    text: model.hasChildren ? ">" : "+"
                    onTriggered: {
                        //var _delegateRoot = delegateRoot
                        //                        var _titleTextField = titleTextField
                        var _proxyModel = proxyModel
                        currentProject = model.projectId
                        currentParent = model.paperId
                        var _currentProject = currentProject
                        var _currentParent = currentParent
                        var _index = model.index
                        _proxyModel.setParentFilter(model.projectId,
                                                    model.paperId)
                        _proxyModel.addHistory(_currentProject, _index)

                        // create a child if none present
                        if (!_proxyModel.hasChildren(_currentProject,
                                                     _currentParent)) {
                            _proxyModel.addItemAtEnd(_currentProject,
                                                     _currentParent, 0)

                            // edit it :
                            //_delegateRoot.editName()
                            //FIXIT: editName makes the app crash
                        }
                    }
                }
                Action {
                    id: openDocumentAction
                    shortcut: "Return"
                    enabled: {
                        if (titleTextField.visible === false
                                && listView.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: "Open document"
                    onTriggered: root.openDocument(model.projectId,
                                                   model.paperId)
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
                        //border.width: 1
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
                                Layout.topMargin: 2
                                Layout.leftMargin: 4
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter

                                text: model.name
                            }
                            TextField {
                                id: titleTextField
                                visible: false

                                Layout.fillWidth: true
                                Layout.fillHeight: true
                                text: titleLabel.text
                                maximumLength: 50

                                placeholderText: qsTr("Enter name")

                                onAccepted: {
                                    model.name = text
                                    delegateRoot.state = ""
                                }

                                onEditingFinished: {
                                    if (!activeFocus) {
                                        accepted()
                                    }
                                }
                            }

                            Label {
                                id: tagLabel

                                //                                text: model.tag
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
                        focusPolicy: Qt.NoFocus

                        onClicked: {
                            menu.open()
                        }

                        visible: hoverHandler.hovered | content.isCurrent
                    }

                    ToolButton {
                        id: goToChildButton
                        action: goToChildAction

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
                        flat: true
                        Layout.preferredWidth: 30
                        Layout.fillHeight: true
                        Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                        visible: hoverHandler.hovered | content.isCurrent
                        focusPolicy: Qt.NoFocus
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
            states: [
                State {
                    name: "drag_active"
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
                },
                State {
                    name: "edit_name"
                    PropertyChanges {
                        target: menuButton
                        visible: false
                    }
                    PropertyChanges {
                        target: goToChildButton
                        visible: false
                    }
                    PropertyChanges {
                        target: titleLabel
                        visible: false
                    }
                    PropertyChanges {
                        target: tagLabel
                        visible: false
                    }
                    PropertyChanges {
                        target: titleTextField
                        visible: true
                    }
                }
            ]

            //            Shortcut {
            //                sequences: ["Ctrl+Shift+N"]
            //                onActivated: addBeforeAction.trigger()
            //                enabled: root.visible
            //            }
            Menu {
                id: menu
                y: menuButton.height

                onOpened: {
                    // necessary to differenciate between all items
                    contextMenuItemIndex = model.index
                }

                Action {
                    id: renameAction
                    text: qsTr("Rename")
                    shortcut: "F2"
                    icon {
                        name: "edit-rename"
                    }
                    enabled: contextMenuItemIndex === model.index
                    onTriggered: {
                        console.log("rename action", model.projectId,
                                    model.paperId)
                        delegateRoot.editName()
                        //forceActiveFocus(titleTextField)
                    }
                }

                MenuSeparator {}

                Action {

                    text: qsTr("Copy")
                    shortcut: StandardKey.Copy
                    icon {
                        name: "edit-copy"
                    }
                    enabled: contextMenuItemIndex === model.index

                    onTriggered: {
                        console.log("copy action", model.projectId,
                                    model.paperId)
                        proxyModel.copy(model.projectId, model.paperId)
                    }
                }
                Action {

                    text: qsTr("Cut")
                    shortcut: StandardKey.Cut
                    icon {
                        name: "edit-cut"
                    }
                    enabled: contextMenuItemIndex === model.index

                    onTriggered: {
                        console.log("cut action", model.projectId,
                                    model.paperId)
                        proxyModel.cut(model.projectId, model.paperId, -2)
                    }
                }
                MenuSeparator {}
                Action {
                    id: addBeforeAction
                    text: qsTr("Add before")
                    shortcut: "Ctrl+Shift+N"
                    icon {
                        name: "document-new"
                    }
                    enabled: contextMenuItemIndex === model.index
                    onTriggered: {
                        console.log("add before action", model.projectId,
                                    model.paperId)
                    }
                }

                Action {
                    id: addAfterAction
                    text: qsTr("Add after")
                    shortcut: "Ctrl+N"
                    icon {
                        name: "document-new"
                    }
                    enabled: contextMenuItemIndex === model.index
                    onTriggered: {
                        console.log("add after action", model.projectId,
                                    model.paperId)
                    }
                }

                MenuSeparator {}

                Action {
                    id: moveUpAction
                    text: qsTr("Move up")
                    shortcut: "Ctrl+Up"
                    icon {
                        name: "object-order-raise"
                    }
                    enabled: contextMenuItemIndex === model.index
                             && model.index !== 0
                    onTriggered: {
                        console.log("move up action", model.projectId,
                                    model.paperId)
                        proxyModel.moveUp(model.projectId, model.paperId,
                                          model.index)
                    }
                }

                Action {

                    text: qsTr("Move down")
                    shortcut: "Ctrl+Down"
                    icon {
                        name: "object-order-lower"
                    }
                    enabled: contextMenuItemIndex === model.index
                             && model.index !== visualModel.items.count - 1

                    onTriggered: {
                        console.log("move down action", model.projectId,
                                    model.paperId)
                        proxyModel.moveDown(model.projectId, model.paperId,
                                            model.index)
                    }
                }
                MenuSeparator {}
                Action {
                    text: qsTr("Delete")
                    shortcut: "Del"
                    icon {
                        name: "edit-delete"
                    }
                    enabled: contextMenuItemIndex === model.index
                    onTriggered: {
                        console.log("delete action", model.projectId,
                                    model.paperId)
                        model.deleted = true
                    }
                }
                MenuSeparator {}
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

            ListView.onAdd: SequentialAnimation {
                PropertyAction {
                    target: delegateRoot
                    property: "height"
                    value: 0
                }
                NumberAnimation {
                    target: delegateRoot
                    property: "height"
                    to: 80
                    duration: 250
                    easing.type: Easing.InOutQuad
                }
            }

            // move :
        }
    }
}
