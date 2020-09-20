import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15

TreeListViewForm {
    id: root

    property var proxyModel
    property var model
    onModelChanged: {
        visualModel.model = model
    }

    signal openDocument(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openDocumentInNewTab(int projectId, int paperId)
    signal showDeletedList()

    property int currentParent: -2
    property int currentProject: -2
    property int currentPaperId: -2
    property int currentIndex: listView.currentIndex
    property int openedProjectId: -2
    property int openedPaperId: -2
    property bool hoveringChangingTheCurrentItemAllowed: true
    listView.model: visualModel
    DelegateModel {
        id: visualModel

        delegate: dragDelegate
    }


    // scrollBar interactivity :

    listView.onContentHeightChanged: {
        //fix scrollbar visible at start
        if(scrollView.height === 0){
            scrollBarVerticalPolicy = ScrollBar.AlwaysOff
            return
        }

        if(listView.contentHeight > scrollView.height){
            scrollBarVerticalPolicy = ScrollBar.AlwaysOn
        }
        else {
            scrollBarVerticalPolicy = ScrollBar.AlwaysOff
        }
    }


    //-----------------------------------------------------------------------------
    // go up button :
    property bool goUpButtonEnabled: true
    property bool goUpActionToBeTriggered :false
    property int goUpActionCurrentParent: -2
    goUpToolButton.action: goUpAction

    Action {
        id: goUpAction
        text: qsTr("Go up")
        //shortcut: "Left,Backspace" Doesn't work well
        icon {
            name: "go-parent-folder"
        }
        //enabled:
        onTriggered: {
            goUpActionToBeTriggered = true
            goUpActionCurrentParent = currentParent

            currentParent = proxyModel.goUp()
            listView.currentIndex = proxyModel.getLastOfHistory(currentProject)
            proxyModel.removeLastOfHistory(currentProject)

            goUpActionToBeTriggered = false
            goUpActionCurrentParent = -2

        }
    }
    function determineIfGoUpButtonEnabled() {

        if(!root.visible){
            goUpAction.enabled = false
            return
        }

        goUpAction.enabled = (currentParent !== -2)


    }

    onVisibleChanged: {
        determineIfGoUpButtonEnabled()
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
        determineIfGoUpButtonEnabled()


        if (currentParent !== -2 & currentProject !== -2) {
            currentParentToolButton.text = proxyModel.getItemName(
                        currentProject, currentParent)
            //console.log("onCurrentParentChanged")
        }
        // clear :
        if (currentParent === -2 & currentProject === -2 ){
            currentParentToolButton.text = ""
        }
    }
    onCurrentProjectChanged: {
        if (currentParent !== -2 & currentProject !== -2) {
            currentParentToolButton.text = proxyModel.getItemName(
                        currentProject, currentParent)
            //console.log("onCurrentProjectChanged")
        }
        // clear :
        if (currentParent === -2 & currentProject === -2 ){
            currentParentToolButton.text = ""
        }
    }

    currentParentToolButton.onClicked: {

        //currentParent
    }

    //----------------------------------------------------------------------------


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

        MenuSeparator{}

        Action {
            text: qsTr("Trash")
            //shortcut: StandardKey.Paste
            icon.name: "edit-delete"
            onTriggered: showDeletedList()
        }

    }

    //----------------------------------------------------------------------------
    // add button :

    Action {
        id: addPaperAction
        text: qsTr("Add")
        enabled: currentParent !== -2
        shortcut: "Ctrl+T"
        //enabled: listView.focus === true
        icon{
            name: "document-new"
            height: 100
            width: 100
        }
        onTriggered: {
            proxyModel.addItemAtEnd(currentProject, currentParent,
                                    visualModel.items.count)
            listView.currentItem.editName()
        }
    }

    addToolButton.action: addPaperAction

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
        id: goUpShortcut
        sequences: ["Left", "Backspace"]
        onActivated: goUpAction.trigger()
        enabled: listView.activeFocus

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
    }

    //----------------------------------------------------------------------------

    // focus :
    listView.onActiveFocusChanged: {
        if (activeFocus) {
            listView.currentItem.forceActiveFocus()
        }
    }

    //----------------------------------------------------------------------------

    // used to remember the source when moving an item
    property int moveSourceInt: -2

    //used to differenciante tapCount between ItemSelectionModel
    property int tapCountIndex: -2


    property bool goToChildActionToBeTriggered :false
    property int goToChildActionCurrentIndent: -2
    property bool newPaperAdded :false

    // TreeView item :
    Component {
        id: dragDelegate

        DropArea {
            id: delegateRoot
            property int indent: model.indent




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
                left: Qt.isQtObject(parent) ? parent.left : undefined
                right: Qt.isQtObject(parent) ? parent.right : undefined
                rightMargin: 5
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
            Keys.priority: Keys.AfterItem

            Keys.onPressed: {
                if (event.key === Qt.Key_Right){
                    console.log("Right key pressed")
                    goToChildAction.trigger()
                    event.accepted = true
                }
                if (event.key === Qt.Key_Return && delegateRoot.state !== "edit_name"){
                    console.log("Return key pressed")
                    openDocumentAction.trigger()
                    event.accepted = true
                }
                if ((event.modifiers & Qt.AltModifier) && event.key === Qt.Key_Return){
                    console.log("Alt Return key pressed")
                    openDocumentInNewTabAction.trigger()
                    event.accepted = true
                }
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
                    //                    onHoveredChanged: {
                    //                        if (hoverHandler.hovered & hoveringChangingTheCurrentItemAllowed) {
                    //                            listView.currentIndex = model.index
                    //                        }
                    //                    }
                }

                TapHandler {
                    id: tapHandler
                    onTapCountChanged: {
                        //                        console.log("tap", tapHandler.tapCount)
                        //                        console.log("tap", tapCountTimer.running)


                        //                        if(tapCount == 2 & tapCountTimer.running & tapCountIndex === model.index){
                        //                            //tripleTap
                        //                            console.log("tripletap")
                        //                            tapCountTimer.stop()
                        //                            delegateRoot.editName()
                        //                            return
                        //                        }
                        //                        if(tapCount == 1){
                        //                            tapCountIndex = model.index
                        //                            tapCountTimer.start()
                        //                            return

                        //                        }


                    }

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
                        enabled = false
                    }
                }
                Timer{
                    id: tapCountTimer
                    interval: 200
                    repeat: false
                }
                TapHandler {
                    acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                    acceptedButtons: Qt.RightButton
                    onTapped: {
                        listView.currentIndex = model.index
                        delegateRoot.forceActiveFocus()
                        menu.open()
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
                /// without MouseArea, it breaks while dragging and scrolling:
                MouseArea {
                    anchors.fill: parent
                    onWheel: {
                        listView.flick(0, wheel.angleDelta.y * 50)
                        wheel.accepted = true
                    }

                    enabled: dragHandler.enabled
                }



                Action {
                    id: goToChildAction
                    //shortcut: "Right"
                    enabled: {
                        if (!listView.enabled){
                            return false
                        }

                        if (listView.focus === true && listView.currentIndex === model.index) {
                            return true
                        } else if (hoverHandler.hovered) {
                            return true
                        } else
                            return false
                    }

                    text: model.hasChildren ? ">" : "+"
                    onTriggered: {
                        console.log("goToChildAction triggered")

                        goToChildActionToBeTriggered = true
                        goToChildActionCurrentIndent =  model.indent

                        //var _delegateRoot = delegateRoot
                        //                        var _titleTextField = titleTextField
                        var _proxyModel = proxyModel
                        currentProject = model.projectId
                        currentParent = model.paperId
                        var _currentProject = currentProject
                        var _currentParent = currentParent
                        var _index = model.index

                        var _listView = listView


                        // change level
                        _proxyModel.setParentFilter(_currentProject,
                                                    _currentParent)
                        _proxyModel.addHistory(_currentProject, _index)

                        // create a child if none present
                        if (!_proxyModel.hasChildren(_currentProject,
                                                     _currentParent)) {
                            newPaperAdded = true
                            _proxyModel.addItemAtEnd(_currentProject,
                                                     _currentParent, 0)

                            // edit it :
                            _listView.itemAtIndex(0).editName()
                            //_listView.itemAt(0, 0).editName()


                        }



                    }
                }
                Action {
                    id: openDocumentAction
                    //shortcut: "Return"
                    enabled: {
                        if (listView.focus === true && titleTextField.visible === false
                                && listView.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: "Open document"
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
                        if (listView.focus === true && titleTextField.visible === false
                                && listView.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: "Open document in a new tab"
                    onTriggered: {
                        console.log("model.projectId", model.projectId)
                        root.openDocumentInNewTab(model.projectId,
                                                  model.paperId)

                    }
                }
                ColumnLayout{
                    id: columnLayout3
                    anchors.fill: parent


                    RowLayout {
                        id: rowLayout
                        spacing: 2
                        Layout.fillHeight: true
                        Layout.fillWidth: true


                        Rectangle {
                            id: currentItemIndicator
                            color: "lightsteelblue"
                            Layout.fillHeight: true
                            Layout.preferredWidth: 5
                            visible: listView.currentIndex === model.index
                        }
                        Rectangle {
                            id: openedItemIndicator
                            color: "#2ba200"
                            Layout.fillHeight: true
                            Layout.preferredWidth: 5
                            visible: model.projectId === openedProjectId && model.paperId === openedPaperId
                        }

                        Button {
                            id: projectIsBackupIndicator
                            visible: model.projectIsBackup && model.paperId === -1
                            enabled: true
                            focusPolicy: Qt.NoFocus
                            implicitHeight: 32
                            implicitWidth: 32
                            Layout.maximumHeight: 30
                            padding: 0
                            rightPadding: 0
                            bottomPadding: 0
                            leftPadding: 2
                            topPadding: 0
                            flat: true
                            onDownChanged: down = false


                            icon {
                                name: "tools-media-optical-burn-image"
                                height: 32
                                width: 32
                            }


                            hoverEnabled: true
                            ToolTip.delay: 1000
                            ToolTip.timeout: 5000
                            ToolTip.visible: hovered
                            ToolTip.text: qsTr("This project is a backup")
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
                                    font.bold: model.projectIsActive && model.indent === -1 ? true : false
                                    text: model.indent === -1 ? model.projectName : model.name
                                }

                                TextField {
                                    id: titleTextField
                                    visible: false


                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    text: titleLabel.text
                                    maximumLength: 50

                                    placeholderText: qsTr("Enter name")


                                    onEditingFinished: {
                                        //if (!activeFocus) {
                                        //accepted()
                                        //}
                                        console.log("editing finished")
                                        if(model.indent === -1){ //project item
                                            model.projectName = text
                                        }
                                        else {
                                            model.name = text
                                        }

                                        delegateRoot.state = ""
                                    }

                                    //Keys.priority: Keys.AfterItem

                                    Keys.onPressed: {
                                        if (event.key === Qt.Key_Return){
                                            console.log("Return key pressed title")
                                            editingFinished()
                                            event.accepted = true
                                        }
                                        if ((event.modifiers & Qt.CtrlModifier) && event.key === Qt.Key_Return){
                                            console.log("Ctrl Return key pressed title")
                                            event.accepted = true
                                        }
                                    }

                                }

                                Label {
                                    id: labelLabel

                                    //                                text: model.label
                                    text:  model.label === undefined ? "" : model.label
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
                                listView.currentIndex = model.index
                                delegateRoot.forceActiveFocus()
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
                        Label {
                            id: hasChildrenLabel
                            Layout.preferredWidth: 30
                            Layout.fillHeight: true
                            Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                            text: ">"
                            visible: model.hasChildren & !(hoverHandler.hovered | content.isCurrent)

                        }


                    }
                    Rectangle {
                        id: separator
                        Layout.preferredHeight: 1
                        Layout.preferredWidth: content.width / 2
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignBottom
                        gradient: Gradient {
                            orientation: Qt.Horizontal
                            GradientStop {
                                position: 0.00;
                                color: "#ffffff";
                            }
                            GradientStop {
                                position: 0.30;
                                color: "#9e9e9e";
                            }
                            GradientStop {
                                position: 0.70;
                                color: "#9e9e9e";
                            }
                            GradientStop {
                                position: 1.00;
                                color: "#ffffff";
                            }
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
                        target: labelLabel
                        visible: false
                    }
                    PropertyChanges {
                        target: titleTextField
                        visible: true
                    }
                },

                State {
                    name: "unset_anchors"
                    AnchorChanges {
                        target: delegateRoot
                        anchors.left: undefined
                        anchors.right: undefined

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
                    hoveringChangingTheCurrentItemAllowed = false
                    // necessary to differenciate between all items
                    contextMenuItemIndex = model.index
                }

                onClosed: {
                    hoveringChangingTheCurrentItemAllowed = true

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

                        enabled: contextMenuItemIndex === model.index && titleTextField.visible === false && listView.focus === true &&  model.paperId !== -1
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
                        enabled: contextMenuItemIndex === model.index && titleTextField.visible === false && listView.focus === true &&  model.paperId !== -1
                        onTriggered: {
                            console.log("open paper in new tab action", model.projectId,
                                        model.paperId)
                            openDocumentInNewTabAction.trigger()
                        }
                    }
                }


                MenuItem {
                    height: model.paperId === -1 ? undefined : 0
                    visible: model.paperId === -1
                    enabled: contextMenuItemIndex === model.index && model.projectIsActive === false &&  model.paperId === -1
                    text: qsTr("Set as active project")
                    icon {
                        name: "tab-new"
                    }
                    onTriggered: {
                        console.log("set active project", model.projectId)
                        plmData.projectHub().setActiveProject(model.projectId)
                    }
                }


                MenuSeparator {}

                Action {
                    id: renameAction
                    text: qsTr("Rename")
                    shortcut: "F2"
                    icon {
                        name: "edit-rename"
                    }
                    enabled: contextMenuItemIndex === model.index && listView.focus === true
                    onTriggered: {
                        console.log("rename action", model.projectId,
                                    model.paperId)
                        delegateRoot.editName()
                    }
                }

                MenuSeparator {}

                Action {

                    text: qsTr("Copy")
                    shortcut: StandardKey.Copy
                    icon {
                        name: "edit-copy"
                    }
                    enabled: contextMenuItemIndex === model.index && listView.focus === true

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
                    enabled: contextMenuItemIndex === model.index && listView.focus === true

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
                    enabled: contextMenuItemIndex === model.index && listView.focus === true
                    onTriggered: {
                        //TODO: fill that
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
                    enabled: contextMenuItemIndex === model.index && listView.focus === true
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
                    shortcut: "Ctrl+Up"
                    icon {
                        name: "object-order-raise"
                    }
                    enabled: contextMenuItemIndex === model.index && listView.focus === true
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
                    enabled: contextMenuItemIndex === model.index && listView.focus === true
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
                    enabled: contextMenuItemIndex === model.index && listView.focus === true && model.indent !== -1
                    onTriggered: {
                        console.log("delete action", model.projectId,
                                    model.paperId)
                        model.deleted = true

                    }
                }
                MenuSeparator {}
            }

            //----------------------------------------------------------

            property int animationDuration: 150

            ListView.onRemove: {
                console.log("onRemove")


                var goUpActionCurrentParentIndent = proxyModel.getItemIndent(currentProject, goUpActionCurrentParent)

                if(goUpActionToBeTriggered && goUpActionCurrentParentIndent + 1 === delegateRoot.indent){
                    paperGoesRightAnimation.start()
                }


                if(goToChildActionToBeTriggered && goToChildActionCurrentIndent === delegateRoot.indent){
                    paperGoesLeftAnimation.start()

                }
            }

            //goUpActionToBeTriggered
            ListView.onAdd: {
                console.log("onAdd")

                var goUpActionCurrentParentIndent = proxyModel.getItemIndent(currentProject, goUpActionCurrentParent)

                if(goUpActionToBeTriggered && goUpActionCurrentParentIndent === model.indent){
                    paperAppearsFromLeftAnimation.start()
                }



                if(goToChildActionToBeTriggered && goToChildActionCurrentIndent + 1 === delegateRoot.indent){
                    paperAppearsFromRightAnimation.start()
                }
            }
            SequentialAnimation {
                id: paperAppearsFromLeftAnimation
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: true
                }
                PropertyAction {
                    target: delegateRoot
                    property: "visible"
                    value: false
                }
                PauseAnimation {
                    duration: animationDuration
                }
                PropertyAction {
                    target: delegateRoot
                    property: "visible"
                    value: true
                }

                ScriptAction {
                    script: delegateRoot.state = "unset_anchors"
                }

                PropertyAction {
                    target: delegateRoot
                    property: "x"
                    value: - delegateRoot.width
                }
                NumberAnimation {
                    target: delegateRoot
                    property: "x"
                    to: 0
                    duration: animationDuration
                    easing.type: Easing.InOutQuad
                }
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: false
                }

                ScriptAction {
                    script: delegateRoot.state = ""
                }

            }

            SequentialAnimation {
                id: paperAppearsFromRightAnimation
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: true
                }
                PropertyAction {
                    target: delegateRoot
                    property: "visible"
                    value: false
                }
                PauseAnimation {
                    duration: animationDuration
                }
                PropertyAction {
                    target: delegateRoot
                    property: "visible"
                    value: true
                }
                ScriptAction {
                    script: delegateRoot.state = "unset_anchors"
                }

                PropertyAction {
                    target: delegateRoot
                    property: "x"
                    value: delegateRoot.width
                }
                NumberAnimation {
                    target: delegateRoot
                    property: "x"
                    to: 0
                    duration: animationDuration
                    easing.type: Easing.InOutQuad
                }
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: false
                }

                ScriptAction {
                    script: delegateRoot.state = ""
                }


                ScriptAction {
                    script: if(newPaperAdded === true){
                                listView.itemAtIndex(0).editName()
                                newPaperAdded = false
                            }
                }

                ScriptAction {
                    script: {
                        goToChildActionToBeTriggered = false
                        goToChildActionCurrentIndent =  -2
                    }

                }
            }

            SequentialAnimation {
                id: paperGoesLeftAnimation
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: true
                }
                ScriptAction {
                    script: delegateRoot.state = "unset_anchors"
                }

                NumberAnimation {
                    target: delegateRoot
                    property: "x"
                    to: - delegateRoot.width
                    duration: animationDuration
                    easing.type: Easing.InOutQuad
                }
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: false
                }

                ScriptAction {
                    script: delegateRoot.state = ""
                }
            }

            SequentialAnimation {
                id: paperGoesRightAnimation
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: true
                }

                ScriptAction {
                    script: delegateRoot.state = "unset_anchors"
                }

                NumberAnimation {
                    target: delegateRoot
                    property: "x"
                    to: delegateRoot.width
                    duration: animationDuration
                    easing.type: Easing.InOutQuad
                }
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: false
                }

                ScriptAction {
                    script: delegateRoot.state = ""
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
            // move :
        }
    }


}
