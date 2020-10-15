import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import QtQml 2.15
import QtQuick.Controls.Material 2.15
import eu.skribisto.projecthub 1.0

SheetOverviewTreeForm {
    id: root

    signal openDocument(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openDocumentInNewTab(int projectId, int paperId)
    signal openDocumentInNewWindow(int projectId, int paperId)

    property int currentPaperId: -2
    property int currentProjectId: -2
    property int openedProjectId: -2
    property int openedPaperId: -2
    property int currentIndex: listView.currentIndex



    property alias visualModel: visualModel
    property var proxyModel
    property var model
    onModelChanged: {
        visualModel.model = model
    }
    listView.model: visualModel


    DelegateModel {
        id: visualModel

        delegate: dragDelegate
    }



    property int contextMenuItemIndex: -2
    onCurrentIndexChanged: {
        contextMenuItemIndex = listView.currentIndex
    }

    Binding {
        target: listView
        property: "currentIndex"
        value: proxyModel.forcedCurrentIndex
    }


    //-----------------------------------------------------------------------------
    // options :
    property bool treelikeIndentsVisible: true
    property bool dragDropEnabled: false // not yet complemently implemented
    property int displayMode : 0

    //tree-like onTreelikeIndents
    property int treeIndentOffset: 0
    property int treeIndentMultiplier: 10


    // focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            listView.currentItem.forceActiveFocus()
        }
    }
    //----------------------------------------------------------------------------


    Binding {
        target: root
        property: "currentProjectId"
        value: proxyModel.projectIdFilter
    }
    //-----------------------------------------------------------------------------

    // used to remember the source when moving an item
    property int moveSourceInt: -2


    // TreeView item :
    listView.delegate:     Component {
        id: dragDelegate

        DropArea {
            id: delegateRoot



            Accessible.name: {

                var levelText
                if(treelikeIndentsVisible){
                    levelText = qsTr("Level %1").arg(model.indent)
                }

                var titleText = titleLabel.text


                var labelText = ""
                if(labelLabel.text.length > 0){
                    labelText = qsTr("label: %1").arg(labelLabel.text)
                }

                var hasChildrenText = ""
                if(model.hasChildren){
                    hasChildrenText = qsTr("has children")
                }

                return levelText + " " + titleText + " " + labelText + " " + hasChildrenText

            }
            Accessible.role: Accessible.ListItem
            Accessible.description: qsTr("navigation item")


            onEntered: {

                draggableContent.sourceIndex = drag.source.visualIndex
                visualModel.items.move(drag.source.visualIndex,
                                       draggableContent.visualIndex)
            }

            onDropped: {
                console.log("dropped : ", moveSourceInt, draggableContent.visualIndex)
                proxyModel.moveItem(moveSourceInt, draggableContent.visualIndex)
            }


            property int visualIndex: {
                return DelegateModel.itemsIndex
            }

            Binding {
                target: draggableContent
                property: "visualIndex"
                value: visualIndex
            }

            anchors {
                left: Qt.isQtObject(parent) ? parent.left : undefined
                right: Qt.isQtObject(parent) ? parent.right : undefined
                rightMargin: 5
                leftMargin: treelikeIndentsVisible ? model.indent * root.treeIndentMultiplier - root.treeIndentOffset * root.treeIndentMultiplier : undefined
            }



            height: draggableContent.height


            onActiveFocusChanged: {
                if(listView.currentIndex === model.index){
                    root.currentPaperId = model.paperId
                }
            }



            function editName() {
                titleBox.state = "edit_name"
                titleTextField.forceActiveFocus()
                titleTextField.selectAll()
            }

            function editLabel() {
                titleBox.state = "edit_label"
                labelTextField.forceActiveFocus()
                labelTextField.selectAll()
            }



            Keys.priority: Keys.AfterItem

            Keys.onShortcutOverride: {

                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_C){
                    event.accepted = true
                }
                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_X){
                    event.accepted = true
                }
                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_V){
                    event.accepted = true
                }
                if(event.key === Qt.Key_Escape && delegateRoot.state == "edit_name"){
                    event.accepted = true
                }
                if( event.key === Qt.Key_Escape){
                    event.accepted = true
                }
            }

            Keys.onPressed: {
                if (event.key === Qt.Key_Return){
                    console.log("Return key pressed")
                    openDocumentAction.trigger()
                    event.accepted = true
                }
                if ((event.modifiers & Qt.AltModifier) && event.key === Qt.Key_Return){
                    console.log("Alt Return key pressed")
                    openDocumentInNewTabAction.trigger()
                    event.accepted = true
                }


                // rename

                if (event.key === Qt.Key_F2 && delegateRoot.state !== "edit_name"){
                    renameAction.trigger()
                    event.accepted = true
                }

                // cut
                if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_X && delegateRoot.state !== "edit_name"){
                    cutAction.trigger()
                    event.accepted = true
                }

                // copy
                if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_C && delegateRoot.state !== "edit_name"){
                    copyAction.trigger()
                    event.accepted = true
                }

                // add before
                if ((event.modifiers & Qt.ControlModifier) && (event.modifiers & Qt.ShiftModifier) && event.key === Qt.Key_N && delegateRoot.state !== "edit_name"){
                    addBeforeAction.trigger()
                    event.accepted = true
                }

                // add after
                if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_N && delegateRoot.state !== "edit_name"){
                    addAfterAction.trigger()
                    event.accepted = true
                }

                // move up
                if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Up && delegateRoot.state !== "edit_name"){
                    moveUpAction.trigger()
                    event.accepted = true
                }

                // move down
                if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Down && delegateRoot.state !== "edit_name"){
                    moveDownAction.trigger()
                    event.accepted = true
                }

                // send to trash
                if (event.key === Qt.Key_Delete && delegateRoot.state !== "edit_name"){
                    sendToTrashAction.trigger()
                    event.accepted = true
                }

                if (event.key === Qt.Key_Escape){
                    console.log("Escape key pressed")
                    event.accepted = true
                }

            }

            property bool editBugWorkaround: false

            Rectangle {
                id: draggableContent
                property int visualIndex: 0
                property int sourceIndex: -2

                property bool isCurrent: model.index === listView.currentIndex ? true : false

                anchors {
                    horizontalCenter: parent.horizontalCenter
                    verticalCenter: parent.verticalCenter
                }
                width: delegateRoot.width  - 2

                height: content.height + 2

                Drag.active: dragHandler.active
                Drag.source: draggableContent
                Drag.hotSpot.x: width / 2
                Drag.hotSpot.y: height / 2

                color: dragHandler.active | !tapHandler.enabled ? "lightsteelblue" : "transparent"

                Behavior on color {
                    ColorAnimation {
                        duration: 200
                    }
                }

                DragHandler {
                    id: dragHandler
                    //acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                    //xAxis.enabled: false
                    //grabPermissions: PointerHandler.TakeOverForbidden
                    onActiveChanged: {
                        if (active) {
                            moveSourceInt = draggableContent.visualIndex
                        } else {
                            draggableContent.Drag.drop()
                            tapHandler.enabled = true
                        }
                    }
                    enabled: !tapHandler.enabled && root.dragDropEnabled
                }






                /// without MouseArea, it breaks while dragging and scrolling:
                MouseArea {
                    anchors.fill: parent
                    onWheel: {
                        listView.flick(0, wheel.angleDelta.y * 50)
                        wheel.accepted = true
                    }

                    enabled: dragHandler.enabled && root.dragDropEnabled
                }

                Action {
                    id: openDocumentAction
                    //shortcut: "Return"
                    enabled: {
                        if (listView.enabled && titleTextField.visible === false
                                && listView.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: qsTr("Open document")
                    onTriggered: {
                        console.log("model.openedProjectId", openedProjectId)
                        console.log("model.projectId", model.projectId)
                        root.openDocument(openedProjectId, openedPaperId, model.projectId,
                                          model.paperId)
                    }
                }

                Action {
                    id: openDocumentInNewTabAction
                    //shortcut: "Alt+Return"
                    enabled: {
                        if (listView.enabled && titleTextField.visible === false
                                && listView.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: qsTr("Open document in a new tab")
                    onTriggered: {
                        root.openDocumentInNewTab(model.projectId,
                                                  model.paperId)

                    }
                }


                Action {
                    id: openDocumentInNewWindowAction
                    //shortcut: "Alt+Return"
                    enabled: {
                        if (listView.enabled && titleTextField.visible === false
                                && listView.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: qsTr("Open document in a window")
                    onTriggered: {
                        root.openDocumentInNewWindow(model.projectId,
                                                     model.paperId)

                    }
                }




                Pane{
                    id: content

                    property alias tapHandler: tapHandler

                    anchors {
                        horizontalCenter: parent.horizontalCenter
                        verticalCenter: parent.verticalCenter
                    }
                    height: 60
                    width: draggableContent.width




                    HoverHandler {
                        id: hoverHandler

                    }


                    TapHandler {
                        id: tapHandler

                        onSingleTapped: {
                            listView.currentIndex = model.index
                            delegateRoot.forceActiveFocus()
                            eventPoint.accepted = true
                        }

                        onDoubleTapped: {
                            console.log("double tapped")
                            listView.currentIndex = model.index
                            openDocumentAction.trigger()
                            eventPoint.accepted = true
                        }


                        onLongPressed: { // needed to activate the grab handler
                            if(root.dragDropEnabled){
                                enabled = false
                            }
                        }
                    }

                    TapHandler {
                        id: rightClickHandler
                        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                        acceptedButtons: Qt.RightButton
                        onTapped: {
                            listView.currentIndex = model.index
                            //delegateRoot.forceActiveFocus()

                            menu.popup(content, eventPoint.position.x, eventPoint.position.y)

                            eventPoint.accepted = true
                        }
                    }
                    TapHandler {
                        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                        acceptedButtons: Qt.MiddleButton
                        onTapped: {
                            listView.currentIndex = model.index
                            delegateRoot.forceActiveFocus()
                            openDocumentInNewTabAction.trigger()
                            eventPoint.accepted = true

                        }
                    }

                    Material.elevation: 4


                    RowLayout{
                        id: rowLayout3
                        anchors.fill: parent


                        Item {
                            id: titleBox
                            Layout.fillHeight: true
                            Layout.fillWidth: true



                            RowLayout {
                                id: rowLayout
                                anchors.fill: parent
                                spacing: 2

                                Rectangle {
                                    id: currentItemIndicator
                                    color: listView.currentIndex === model.index ? "lightsteelblue" : "transparent"
                                    Layout.fillHeight: true
                                    Layout.preferredWidth: 5
                                    //visible: listView.currentIndex === model.index
                                }



                                ColumnLayout {
                                    id: columnLayout2
                                    spacing: 1
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true

                                    Label {
                                        id: titleLabel

                                        Layout.fillWidth: true
                                        Layout.topMargin: 2
                                        Layout.leftMargin: 4
                                        Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                        font.bold: model.projectIsActive && model.indent === -1 ? true : false
                                        text: model.indent === -1 ? model.projectName : model.name
                                    }

                                    TextField {
                                        id: labelTextField
                                        visible: false


                                        Layout.fillWidth: true
                                        Layout.fillHeight: true
                                        text: labelLabel.text
                                        maximumLength: 50

                                        placeholderText: qsTr("Enter label")

                                        //workaround for bug losing focus after using rename from context menu
                                        onActiveFocusChanged: {
                                            if(!activeFocus && editBugWorkaround){
                                                delegateRoot.editLabel()
                                                delegateRoot.editBugWorkaround = false
                                            }
                                        }

                                        onEditingFinished: {
                                            console.log("editing label finished")
                                            model.label = text
                                            titleBox.state = ""

                                            //fix bug while new lone child
                                            titleLabel.visible = true
                                            labelLabel.visible = true

                                        }

                                        //Keys.priority: Keys.AfterItem
                                        Keys.onShortcutOverride: event.accepted = (event.key === Qt.Key_Escape)
                                        Keys.onPressed: {
                                            if (event.key === Qt.Key_Return){
                                                console.log("Return key pressed title")
                                                editingFinished()
                                                event.accepted = true
                                            }
                                            if ((event.modifiers & Qt.CtrlModifier) && event.key === Qt.Key_Return){
                                                console.log("Ctrl Return key pressed title")
                                                editingFinished()
                                                event.accepted = true
                                            }
                                            if (event.key === Qt.Key_Escape){
                                                console.log("Escape key pressed title")
                                                delegateRoot.state = ""
                                                event.accepted = true
                                            }
                                        }

                                    }

                                    TextField {
                                        id: titleTextField
                                        visible: false


                                        Layout.fillWidth: true
                                        Layout.fillHeight: true
                                        text: titleLabel.text
                                        maximumLength: 50

                                        placeholderText: qsTr("Enter name")

                                        //workaround for bug losing focus after using rename from context menu
                                        onActiveFocusChanged: {
                                            if(!activeFocus && editBugWorkaround){
                                                delegateRoot.editName()
                                                delegateRoot.editBugWorkaround = false
                                            }
                                        }

                                        onEditingFinished: {

                                            console.log("editing finished")
                                            if(model.indent === -1){ //project item
                                                model.projectName = text
                                            }
                                            else {
                                                model.name = text
                                            }

                                            titleBox.state = ""
                                            //fix bug while new lone child
                                            titleLabel.visible = true
                                            labelLabel.visible = true
                                        }

                                        //Keys.priority: Keys.AfterItem
                                        Keys.onShortcutOverride: event.accepted = (event.key === Qt.Key_Escape)
                                        Keys.onPressed: {
                                            if (event.key === Qt.Key_Return){
                                                console.log("Return key pressed title")
                                                editingFinished()
                                                event.accepted = true
                                            }
                                            if ((event.modifiers & Qt.CtrlModifier) && event.key === Qt.Key_Return){
                                                console.log("Ctrl Return key pressed title")
                                                editingFinished()
                                                event.accepted = true
                                            }
                                            if (event.key === Qt.Key_Escape){
                                                console.log("Escape key pressed title")
                                                titleBox.state = ""
                                                event.accepted = true
                                            }
                                        }

                                    }

                                    Label {
                                        id: labelLabel
                                        text:  model.label === undefined ? "" : model.label
                                        Layout.bottomMargin: 2
                                        Layout.rightMargin: 4
                                        Layout.alignment: Qt.AlignRight | Qt.AlignVCenter


                                    }
                                }
                            }

                            states: [
                                State {
                                    name: "edit_name"
                                    PropertyChanges {
                                        target: menuButton
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: titleLabel
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: labelLabel
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: titleTextField
                                        visible: true
                                    }
                                    PropertyChanges {
                                        target: labelTextField
                                        visible: false
                                    }
                                },
                                State {
                                    name: "edit_label"
                                    PropertyChanges {
                                        target: menuButton
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: titleLabel
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: labelLabel
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: titleTextField
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: labelTextField
                                        visible: true
                                    }
                                }

                            ]
                        }

                        ColumnLayout {
                            ToolButton {
                                id: menuButton
                                Layout.fillHeight: true
                                Layout.preferredWidth: 30

                                text: "..."
                                flat: true
                                focusPolicy: Qt.NoFocus

                                onClicked: {
                                    listView.currentIndex = model.index
                                    delegateRoot.forceActiveFocus()
                                    menu.open()
                                    menu.popup(menuButton, menuButton.x, menuButton.height)

                                }

                                visible: hoverHandler.hovered | draggableContent.isCurrent
                            }

                            ToolButton {
                                id: focusOnBranchButton
                                Layout.fillHeight: true
                                Layout.preferredWidth: 30

                                text: "focus"
                                icon.name: "edit-find"
                                display: AbstractButton.IconOnly
                                flat: true
                                visible: false

                                onClicked: {
                                    listView.currentIndex = model.index
                                    delegateRoot.forceActiveFocus()

                                    // filter to this parent and its children


                                }

                            }

                        }


                    }
                }
                states: [
                    State {
                        name: "displayMode_1"
                        when: root.displayMode === 1

                        PropertyChanges {
                            target: content
                            height: 100
                        }
                        PropertyChanges {
                            target: focusOnBranchButton
                            visible: true
                        }
                    },
                    State {
                        name: "displayMode_2"
                        when: root.displayMode === 2

                        PropertyChanges {
                            target: content
                            height: 200
                        }
                    }

                ]

                transitions: [
                    Transition {
                        PropertyAnimation {
                            //target: content
                            properties: "height"
                            duration: 250
                            easing.type: Easing.InOutQuad
                        }
                    }
                ]






            }
            states: [
                State {
                    name: "drag_active"
                    when: draggableContent.Drag.active

                    ParentChange {
                        target: draggableContent
                        parent: base
                    }
                    AnchorChanges {
                        target: draggableContent
                        anchors {
                            horizontalCenter: undefined
                            verticalCenter: undefined
                        }
                    }
                }
            ]





            Menu {
                id: menu


                onOpened: {
                    // necessary to differenciate between all items
                    contextMenuItemIndex = model.index
                }

                onClosed: {

                }
                MenuItem {
                    visible: model.paperId !== -1
                    height: model.paperId === -1 ? 0 : undefined
                    action: Action {
                        id: openPaperAction
                        text: qsTr("Open")
                        //shortcut: "Return"
                        icon {
                            name: "document-edit"
                        }

                        enabled: contextMenuItemIndex === model.index && titleTextField.visible === false && listView.enabled &&  model.paperId !== -1
                        onTriggered: {
                            console.log("open paper action", model.projectId,
                                        model.paperId)
                            openDocumentAction.trigger()
                        }
                    }
                }
                MenuItem {
                    visible: model.paperId !== -1
                    height: model.paperId === -1 ? 0 : undefined

                    action: Action {
                        id: openPaperInNewTabAction
                        text: qsTr("Open in new tab")
                        //shortcut: "Alt+Return"
                        icon {
                            name: "tab-new"
                        }
                        enabled: contextMenuItemIndex === model.index && titleTextField.visible === false && listView.enabled &&  model.paperId !== -1
                        onTriggered: {
                            console.log("open paper in new tab action", model.projectId,
                                        model.paperId)
                            openDocumentInNewTabAction.trigger()
                        }
                    }
                }


                MenuItem {
                    visible: model.paperId !== -1
                    height: model.paperId === -1 ? 0 : undefined

                    action: Action {
                        id: openPaperInNewWindowAction
                        text: qsTr("Open in new window")
                        //shortcut: "Alt+Return"
                        icon {
                            name: "window-new"
                        }
                        enabled: contextMenuItemIndex === model.index && titleTextField.visible === false && listView.enabled &&  model.paperId !== -1
                        onTriggered: {
                            console.log("open paper in new window action", model.projectId,
                                        model.paperId)
                            openDocumentInNewWindowAction.trigger()
                        }
                    }
                }



                MenuSeparator {}

                Action {
                    id: renameAction
                    text: qsTr("Rename")
                    //shortcut: "F2"
                    icon {
                        name: "edit-rename"
                    }
                    enabled: contextMenuItemIndex === model.index && listView.enabled

                    onTriggered: {
                        console.log("rename action", model.projectId,
                                    model.paperId)
                        delegateRoot.editBugWorkaround = true
                        delegateRoot.editName()
                    }
                }

                Action {
                    id: setLabelAction
                    text: qsTr("Set label")
                    //shortcut: "F2"
                    icon {
                        name: "label"
                    }
                    enabled: contextMenuItemIndex === model.index  && listView.enabled
                    onTriggered: {
                        console.log("from deleted: sel label", model.projectId,
                                    model.paperId)
                        delegateRoot.editBugWorkaround = true
                        delegateRoot.editLabel()
                    }
                }

                MenuSeparator {}
                Action {
                    id: cutAction
                    text: qsTr("Cut")
                    //shortcut: StandardKey.Cut
                    icon {
                        name: "edit-cut"
                    }
                    enabled: contextMenuItemIndex === model.index && listView.enabled

                    onTriggered: {
                        console.log("cut action", model.projectId,
                                    model.paperId)
                        proxyModel.cut(model.projectId, model.paperId, -2)
                    }
                }

                Action {

                    id: copyAction
                    text: qsTr("Copy")
                    //shortcut: StandardKey.Copy
                    icon {
                        name: "edit-copy"
                    }
                    enabled: contextMenuItemIndex === model.index && listView.enabled

                    onTriggered: {
                        console.log("copy action", model.projectId,
                                    model.paperId)
                        proxyModel.copy(model.projectId, model.paperId)
                    }
                }

                MenuSeparator {}
                Action {
                    id: addBeforeAction
                    text: qsTr("Add before")
                    //shortcut: "Ctrl+Shift+N"
                    icon {
                        name: "document-new"
                    }
                    enabled: contextMenuItemIndex === model.index && listView.enabled
                    onTriggered: {
                        //TODO: fill that
                        console.log("add before action", model.projectId,
                                    model.paperId)
                    }
                }

                Action {
                    id: addAfterAction
                    text: qsTr("Add after")
                    //shortcut: "Ctrl+N"
                    icon {
                        name: "document-new"
                    }
                    enabled: contextMenuItemIndex === model.index && listView.enabled
                    onTriggered: {
                        //TODO: fill that
                        console.log("add after action", model.projectId,
                                    model.paperId)
                    }
                }

                MenuSeparator {}

                Action {
                    id: moveUpAction
                    text: qsTr("Move up")
                    //shortcut: "Ctrl+Up"
                    icon {
                        name: "object-order-raise"
                    }
                    enabled: contextMenuItemIndex === model.index && listView.enabled
                             && model.index !== 0
                    onTriggered: {
                        console.log("move up action", model.projectId,
                                    model.paperId)
                        proxyModel.moveUp(model.projectId, model.paperId,
                                          model.index)
                    }
                }

                Action {
                    id: moveDownAction
                    text: qsTr("Move down")
                    //shortcut: "Ctrl+Down"
                    icon {
                        name: "object-order-lower"
                    }
                    enabled: contextMenuItemIndex === model.index
                             && model.index !== visualModel.items.count - 1  && listView.enabled

                    onTriggered: {
                        console.log("move down action", model.projectId,
                                    model.paperId)
                        proxyModel.moveDown(model.projectId, model.paperId,
                                            model.index)
                    }
                }
                MenuSeparator {}
                Action {
                    id: sendToTrashAction
                    text: qsTr("Send to trash")
                    //shortcut: "Del"
                    icon {
                        name: "edit-delete"
                    }
                    enabled: contextMenuItemIndex === model.index  && listView.enabled && model.indent !== -1
                    onTriggered: {
                        console.log("sent to trash action", model.projectId,
                                    model.paperId)
                        model.trashed = true

                    }
                }
            }




            SequentialAnimation {
                id: removePaperAtEndAnimation
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: true
                }
                NumberAnimation {
                    target: delegateRoot
                    property: "height"
                    to: 0
                    duration: 250
                    easing.type: Easing.InOutQuad
                }
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: false
                }
            }
            SequentialAnimation {
                id: addPaperAtEndAnimation

                PropertyAction {
                    target: delegateRoot
                    property: "height"
                    value: 0
                }
                NumberAnimation {
                    target: delegateRoot
                    property: "height"
                    to: delegateRoot.height
                    duration: 250
                    easing.type: Easing.InOutQuad
                }
            }

        }
    }

}

