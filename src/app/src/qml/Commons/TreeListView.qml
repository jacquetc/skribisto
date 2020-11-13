import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import QtQuick.Controls.Material 2.15
import "../Items"
import ".."

TreeListViewForm {
    id: root

    property var proxyModel
    property var model
    onModelChanged: {
        visualModel.model = model
    }

    signal openDocument(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openDocumentInNewTab(int projectId, int paperId)
    signal openDocumentInNewWindow(int projectId, int paperId)
    signal showTrashedList()

    property int currentParent: -2
    property int currentProject: -2
    property int currentPaperId: -2
    property int currentIndex: listView.currentIndex
    property int openedProjectId: -2
    property int openedPaperId: -2
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

    //toolBarPrimaryColor: Material.color(Material.Cyan, Material.Shade200)

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
            source: "qrc:///icons/backup/go-parent-folder.svg"
        }
        //enabled:
        onTriggered: {
            goUpActionToBeTriggered = true
            goUpActionCurrentParent = currentParent

            currentParent = proxyModel.goUp()
            //console.log("currentParent", currentParent)
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

            listView.section.delegate = null

        }
        // clear :
        if (currentParent === -2 & currentProject === -2 ){
            currentParentToolButton.text = ""
            listView.section.delegate = null
        }
        // show "projects" section
        else if (currentParent === -2 ){

            listView.section.delegate = sectionHeading

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
    listView.section.property: "indent"
    listView.section.criteria: ViewSection.FullString
    listView.section.labelPositioning: ViewSection.CurrentLabelAtStart |
                                       ViewSection.InlineLabels
    listView.section.delegate: null

    // The delegate for each section header
    Component {
        id: sectionHeading
        Rectangle {
            width: listView.width
            height: childrenRect.height
            color: "lightsteelblue"

            required property string section

            SkrLabel {
                text: qsTr("Projects")
                font.bold: true
                //font.pixelSize: 20
            }
        }
    }

    //----------------------------------------------------------------------------
    treeMenuToolButton.icon.source: "qrc:///icons/backup/overflow-menu.svg"
    treeMenuToolButton.onClicked: {
        if(navigationMenu.visible){
            navigationMenu.close()
            return
        }
        navigationMenu.open()
    }

    SkrMenu {
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
            enabled: listView.enabled && currentParent !== -2
            shortcut: StandardKey.Paste
            icon.source: "qrc:///icons/backup/edit-paste.svg"
        }
        SkrMenu {

            title: qsTr("Advanced")
            Action {
                text: qsTr("Sort alphabetically")
                icon.source: "qrc:///icons/backup/view-sort-ascending-name.svg"
            }
        }

        MenuSeparator{}

        Action {
            text: qsTr("Trash")
            //shortcut: StandardKey.Paste
            icon.source: "qrc:///icons/backup/edit-delete.svg"
            onTriggered: showTrashedList()
        }

    }

    //----------------------------------------------------------------------------
    // add button :

    Action {
        id: addPaperAction
        text: qsTr("Add")
        shortcut: "Ctrl+T"
        enabled:  listView.enabled && currentParent !== -2
        icon{
            source: "qrc:///icons/backup/document-new.svg"
            height: 100
            width: 100
        }
        onTriggered: {
            proxyModel.addChildItem(currentProject, currentParent,
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

    property bool temporarilyDisableMove: false


    Timer{
        id: temporarilyDisableMoveTimer
        repeat: false
        interval: 300
        onTriggered: {
            temporarilyDisableMove = false
        }

    }



    //-----------------------------------------------------------------------------
    Component.onCompleted: {

    }

    //-----------------------------------------------------------------------------
    listView.onCurrentIndexChanged: {
        contextMenuItemIndex = listView.currentIndex
    }

    property int contextMenuItemIndex: -2

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

            Accessible.name: labelLabel.text.length === 0 ? titleLabel.text  +  ( model.hasChildren ? " " +qsTr("has child") :  "" ):
                                                            titleLabel.text + " " + qsTr("label:") + " " + labelLabel.text + ( model.hasChildren ? " " +qsTr("has child") :  "" )
            Accessible.role: Accessible.ListItem
            Accessible.description: qsTr("navigation item")


            onEntered: {

                content.sourceIndex = drag.source.visualIndex
                visualModel.items.move(drag.source.visualIndex,
                                       content.visualIndex)
            }

            onDropped: {
                //console.log("dropped : ", moveSourceInt, content.visualIndex)
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

            onActiveFocusChanged: {
                if(listView.currentIndex === model.index && model.index !== -1 && activeFocus){
                    root.currentPaperId = model.paperId
                }
            }


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
                titleTextFieldForceActiveFocusTimer.start()
                titleTextField.selectAll()
            }

            Timer{
                id: titleTextFieldForceActiveFocusTimer
                repeat: false
                interval: 100
                onTriggered: {
                    titleTextField.forceActiveFocus()
                }
            }

            function editLabel() {
                state = "edit_label"
                labelTextFieldForceActiveFocusTimer.start()
                labelTextField.selectAll()
            }

            Timer{
                id: labelTextFieldForceActiveFocusTimer
                repeat: false
                interval: 100
                onTriggered: {
                    labelTextField.forceActiveFocus()
                }
            }

            Keys.priority: Keys.AfterItem

            Keys.onShortcutOverride: {
                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_N){
                    event.accepted = true
                }
                if((event.modifiers & Qt.ControlModifier) && (event.modifiers & Qt.ShiftModifier) && event.key === Qt.Key_N){
                    event.accepted = true
                }
                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_C){
                    event.accepted = true
                }
                if((model.isMovable && event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_X){
                    event.accepted = true
                }
                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_V){
                    event.accepted = true
                }
                if( event.key === Qt.Key_Escape  && (delegateRoot.state == "edit_name" || delegateRoot.state == "edit_label")){
                    event.accepted = true
                }
            }

            Keys.onPressed: {
                if (event.key === Qt.Key_Right){
                    console.log("Right key pressed")
                    goToChildAction.trigger()
                    event.accepted = true
                }
                if (event.key === Qt.Key_Backspace || event.key === Qt.Key_Left){
                    console.log("Backspace / Left key pressed")
                    goUpAction.trigger()
                    event.accepted = true
                }
                if (model.isOpenable && (event.modifiers & Qt.AltModifier) && event.key === Qt.Key_Return){
                    console.log("Alt Return key pressed")
                    openDocumentInNewTabAction.trigger()
                    event.accepted = true
                }
                if (model.isOpenable && event.key === Qt.Key_Return && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    console.log("Return key pressed")
                    openDocumentAction.trigger()
                    event.accepted = true
                }
                // rename

                if (model.isRenamable && event.key === Qt.Key_F2 && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    renameAction.trigger()
                    event.accepted = true
                }



                // cut
                if (model.isMovable && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_X && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    cutAction.trigger()
                    event.accepted = true
                }

                // copy
                if (model.isCopyable && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_C && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    copyAction.trigger()
                    event.accepted = true
                }

                // add before
                if (model.canAddPaper && (event.modifiers & Qt.ControlModifier) && (event.modifiers & Qt.ShiftModifier) && event.key === Qt.Key_N && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    addBeforeAction.trigger()
                    event.accepted = true
                }

                // add after
                if (model.canAddPaper && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_N && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    addAfterAction.trigger()
                    event.accepted = true
                }

                // move up
                if (model.isMovable && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Up && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    moveUpAction.trigger()
                    event.accepted = true
                }

                // move down
                if (model.isMovable && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Down && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    moveDownAction.trigger()
                    event.accepted = true
                }

                // send to trash
                if (model.isTrashable && event.key === Qt.Key_Delete && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    sendToTrashAction.trigger()
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
                height: 50

                Drag.active: dragHandler.active
                Drag.source: content
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

                    onSingleTapped: {
                        listView.currentIndex = model.index
                        delegateRoot.forceActiveFocus()
                        eventPoint.accepted = true
                    }

                    onDoubleTapped: {
                        //console.log("double tapped")
                        listView.currentIndex = model.index
                        openDocumentAction.trigger()
                        eventPoint.accepted = true
                    }

                    onLongPressed: { // needed to activate the grab handler
                        enabled = false
                    }

                }

                TapHandler {
                    acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                    acceptedButtons: Qt.RightButton
                    onTapped: {

                        if(menu.visible){
                            menu.close()
                            return
                        }


                        listView.currentIndex = model.index
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
                    acceptedButtons: Qt.NoButton
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
                        if(!model.hasChildren && !model.canAddPaper){
                            return false
                        }

                        if (!listView.enabled){
                            return false
                        }

                        if (listView.enabled && listView.currentIndex === model.index) {
                            return true
                        } else if (hoverHandler.hovered) {
                            return true
                        } else
                            return false
                    }
                    icon.source: model.hasChildren ? "qrc:///icons/backup/go-next.svg" : (model.canAddPaper ? "qrc:///icons/backup/list-add.svg" : "")
                    text: model.hasChildren ? ">" : (model.canAddPaper ? "+" : "")
                    onTriggered: {
                        console.log("goToChildAction triggered")

                        goToChildActionToBeTriggered = true
                        goToChildActionCurrentIndent =  model.indent


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
                            _proxyModel.addChildItem(_currentProject,
                                                     _currentParent, 0)

                            // edit it :
                            _listView.itemAtIndex(0).editName()

                        }



                    }
                }

                property int paperIdToEdit: -2
                Timer{
                    id: editNameTimer
                    repeat: false
                    interval: animationDuration
                    onTriggered: {
                        var index = proxyModel.findVisualIndex(model.projectId, paperIdToEdit)
                        if(index !== -2){
                            listView.itemAtIndex(index).editName()
                        }
                    }
                }

                Action {
                    id: openDocumentAction
                    //shortcut: "Return"
                    enabled: {
                        if(!model.isOpenable){
                            return false
                        }

                        if (listView.enabled && titleTextField.visible === false
                                && listView.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: qsTr("Open document")
                    onTriggered: {
                        //console.log("model.openedProjectId", openedProjectId)
                        //console.log("model.projectId", model.projectId)
                        root.openDocument(openedProjectId, openedPaperId, model.projectId,
                                          model.paperId)
                    }
                }

                Action {
                    id: openDocumentInNewTabAction
                    //shortcut: "Alt+Return"
                    enabled: {
                        if(!model.isOpenable){
                            return false
                        }
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
                        if(!model.isOpenable){
                            return false
                        }
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
                            color:  SkrTheme.accent
                            Layout.fillHeight: true
                            Layout.preferredWidth: 5
                            visible: model.projectId === openedProjectId && model.paperId === openedPaperId
                        }

                        SkrButton {
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
                                source: "qrc:///icons/backup/tools-media-optical-burn-image.svg"
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

                                SkrLabel {
                                    id: titleLabel

                                    Layout.fillWidth: true
                                    Layout.topMargin: 2
                                    Layout.leftMargin: 4
                                    Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                    font.bold: model.projectIsActive && model.indent === -1 ? true : false
                                    text: model.indent === -1 ? model.projectName : model.name
                                    elide: Text.ElideRight
                                }

                                SkrTextField {
                                    id: labelTextField
                                    visible: false


                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    text: labelLabel.text
                                    maximumLength: 50

                                    placeholderText: qsTr("Enter label")


                                    onEditingFinished: {

                                        console.log("editing label finished")
                                        model.label = text
                                        delegateRoot.state = ""


                                        //fix bug while new lone child
                                        titleLabel.visible = true
                                        labelLayout.visible = true
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

                                SkrTextField {
                                    id: titleTextField
                                    visible: false


                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    text: titleLabel.text
                                    maximumLength: 50

                                    placeholderText: qsTr("Enter name")


                                    onEditingFinished: {

                                        console.log("editing finished")
                                        if(model.indent === -1){ //project item
                                            model.projectName = text
                                        }
                                        else {
                                            model.name = text
                                        }

                                        delegateRoot.state = ""


                                        //fix bug while new lone child
                                        titleLabel.visible = true
                                        labelLayout.visible = true
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

                                RowLayout{
                                    id: labelLayout
                                    Layout.fillWidth: true
                                    Layout.leftMargin: 5

                                    ListItemAttributes{
                                        id: attributes
                                        attributes: model.attributes
                                        Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                        Layout.leftMargin: 4
                                        Layout.bottomMargin: 2

                                    }


                                    SkrLabel {
                                        id: labelLabel
                                        text:  model.label === undefined ? "" : model.label
                                        Layout.bottomMargin: 2
                                        Layout.rightMargin: 4
                                        Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                                        elide: Text.ElideRight
                                        horizontalAlignment: Qt.AlignRight
                                        Layout.fillWidth: true


                                    }
                                }
                            }

                        }

                        SkrToolButton {
                            id: menuButton
                            Layout.fillHeight: true
                            Layout.preferredWidth: 30

                            text: "..."
                            flat: true
                            focusPolicy: Qt.NoFocus

                            onClicked: {

                                if(menu.visible){
                                    menu.close()
                                    return
                                }


                                listView.currentIndex = model.index
                                delegateRoot.forceActiveFocus()
                                menu.open()
                            }

                            visible: hoverHandler.hovered | content.isCurrent
                        }

                        SkrToolButton {
                            id: goToChildButton
                            action: goToChildAction

                            flat: true
                            Layout.preferredWidth: 30
                            Layout.fillHeight: true
                            Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                            visible: model.hasChildren ? true : (hoverHandler.hovered | content.isCurrent)
                            focusPolicy: Qt.NoFocus
                        }

                        Rectangle {
                            Layout.fillHeight: true
                            Layout.preferredWidth: 2

                            color: model.indent === 0 ? Material.color(Material.Indigo) :
                                                        (model.indent === 1 ? Material.color(Material.LightBlue) :
                                                                              (model.indent === 2 ? Material.color(Material.LightGreen) :
                                                                                                    (model.indent === 3 ? Material.color(Material.Amber) :
                                                                                                                          (model.indent === 4 ? Material.color(Material.DeepOrange) :
                                                                                                                                                Material.color(Material.Teal)
                                                                                                                           ))))
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
                        target: labelLayout
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
                        target: goToChildButton
                        visible: false
                    }
                    PropertyChanges {
                        target: titleLabel
                        visible: false
                    }
                    PropertyChanges {
                        target: labelLayout
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
            SkrMenu {
                id: menu
                y: menuButton.height

                onOpened: {
                    // necessary to differenciate between all items
                    contextMenuItemIndex = model.index
                }

                onClosed: {

                }


                SkrMenuItem {
                    height: !model.isOpenable || model.paperId === -1 ? 0 : undefined
                    visible: model.isOpenable && model.paperId !== -1
                    action: Action {
                        id: openPaperAction
                        text: qsTr("Open")
                        //shortcut: "Return"
                        icon {
                            source: "qrc:///icons/backup/document-edit.svg"
                        }

                        enabled: contextMenuItemIndex === model.index && titleTextField.visible === false && listView.enabled &&  model.paperId !== -1
                        onTriggered: {
                            console.log("open paper action", model.projectId,
                                        model.paperId)
                            openDocumentAction.trigger()
                        }
                    }
                }
                SkrMenuItem {
                    height: !model.isOpenable ||model.paperId === -1 ? 0 : undefined
                    visible:  model.isOpenable && model.paperId !== -1

                    action: Action {
                        id: openPaperInNewTabAction
                        text: qsTr("Open in new tab")
                        //shortcut: "Alt+Return"
                        icon {
                            source: "qrc:///icons/backup/tab-new.svg"
                        }
                        enabled: contextMenuItemIndex === model.index && titleTextField.visible === false && listView.enabled &&  model.paperId !== -1
                        onTriggered: {
                            console.log("open paper in new tab action", model.projectId,
                                        model.paperId)
                            openDocumentInNewTabAction.trigger()
                        }
                    }
                }


                SkrMenuItem {
                    height: !model.isOpenable || model.paperId === -1 ?  0 : undefined
                    visible: model.isOpenable && model.paperId !== -1

                    action: Action {
                        id: openPaperInNewWindowAction
                        text: qsTr("Open in new window")
                        //shortcut: "Alt+Return"
                        icon {
                            source: "qrc:///icons/backup/window-new.svg"
                        }
                        enabled: contextMenuItemIndex === model.index && titleTextField.visible === false && listView.enabled &&  model.paperId !== -1
                        onTriggered: {
                            console.log("open paper in new window action", model.projectId,
                                        model.paperId)
                            openDocumentInNewWindowAction.trigger()
                        }
                    }
                }

                SkrMenuItem {
                    height: model.paperId === -1 ?  undefined : 0
                    visible: model.paperId === -1
                    enabled: contextMenuItemIndex === model.index && model.projectIsActive === false && listView.enabled &&  model.paperId === -1
                    text: qsTr("Set as active project")
                    icon {
                        source: "qrc:///icons/backup/tab-new.svg"
                    }
                    onTriggered: {
                        console.log("set active project", model.projectId)
                        plmData.projectHub().setActiveProject(model.projectId)
                    }
                }


                MenuSeparator {
                    height: model.isRenamable ? undefined : 0
                    visible: model.isRenamable
                }

                SkrMenuItem {
                    height: model.isRenamable ? undefined : 0
                    visible: model.isRenamable
                    action:
                        Action {
                        id: renameAction
                        text: qsTr("Rename")
                        //shortcut: "F2"
                        icon {
                            source: "qrc:///icons/backup/edit-rename.svg"
                        }
                        enabled: contextMenuItemIndex === model.index && listView.enabled

                        onTriggered: {
                            console.log("rename action", model.projectId,
                                        model.paperId)
                            delegateRoot.editName()
                        }
                    }
                }

                SkrMenuItem {
                    height: !model.isRenamable || model.paperId === -1 ? 0: undefined
                    visible: model.isRenamable && model.paperId !== -1
                    action:
                        Action {
                        id: setLabelAction
                        text: qsTr("Set label")
                        //shortcut: "F2"
                        icon {
                            source: "qrc:///icons/backup/label.svg"
                        }
                        enabled: contextMenuItemIndex === model.index  && listView.enabled
                        onTriggered: {
                            console.log("sel label", model.projectId,
                                        model.paperId)
                            delegateRoot.editLabel()
                        }
                    }
                }

                MenuSeparator {
                    height: !model.isCopyable || model.paperId === -1 ? 0: undefined
                    visible: model.isCopyable && model.paperId !== -1
                }

                SkrMenuItem {
                    height: !model.isCopyable || model.paperId === -1 ? 0: undefined
                    visible: model.isCopyable && model.paperId !== -1
                    action:
                        Action {

                        id: copyAction
                        text: qsTr("Copy")
                        //shortcut: StandardKey.Copy
                        icon {
                            source: "qrc:///icons/backup/edit-copy.svg"
                        }
                        enabled: contextMenuItemIndex === model.index && listView.enabled

                        onTriggered: {
                            console.log("copy action", model.projectId,
                                        model.paperId)
                            proxyModel.copy(model.projectId, model.paperId)
                        }
                    }
                }

                SkrMenuItem {
                    height: !model.isMovable || model.paperId === -1 ? 0: undefined
                    visible: model.isMovable && model.paperId !== -1
                    action:
                        Action {
                        id: cutAction
                        text: qsTr("Cut")
                        //shortcut: StandardKey.Cut
                        icon {
                            source: "qrc:///icons/backup/edit-cut.svg"
                        }
                        enabled: contextMenuItemIndex === model.index && listView.enabled

                        onTriggered: {
                            console.log("cut action", model.projectId,
                                        model.paperId)
                            proxyModel.cut(model.projectId, model.paperId, -2)
                        }
                    }
                }

                MenuSeparator {
                    height: !model.canAddPaper || model.paperId === -1 ? 0: undefined
                    visible: model.canAddPaper && model.paperId !== -1
                }

                SkrMenuItem {
                    height: !model.canAddPaper || model.paperId === -1 ? 0: undefined
                    visible: model.canAddPaper && model.paperId !== -1
                    action:
                        Action {
                        id: addBeforeAction
                        text: qsTr("Add before")
                        //shortcut: "Ctrl+Shift+N"
                        icon {
                            source: "qrc:///icons/backup/document-new.svg"
                        }
                        enabled: contextMenuItemIndex === model.index && listView.enabled
                        onTriggered: {
                            console.log("add before action", model.projectId,
                                        model.paperId)

                            var visualIndex = root.currentIndex
                            proxyModel.addItemAbove(model.projectId, model.paperId, visualIndex)
                            listView.itemAtIndex(visualIndex).editName()
                        }
                    }
                }

                SkrMenuItem {
                    height: !model.canAddPaper || model.paperId === -1 ? 0: undefined
                    visible: model.canAddPaper && model.paperId !== -1
                    action:
                        Action {
                        id: addAfterAction
                        text: qsTr("Add after")
                        //shortcut: "Ctrl+N"
                        icon {
                            source: "qrc:///icons/backup/document-new.svg"
                        }
                        enabled: contextMenuItemIndex === model.index && listView.enabled
                        onTriggered: {
                            console.log("add after action", model.projectId,
                                        model.paperId)

                            var visualIndex = root.currentIndex + 1
                            proxyModel.addItemBelow(model.projectId, model.paperId, visualIndex)
                            listView.itemAtIndex(visualIndex).editName()
                        }
                    }

                }

                MenuSeparator {
                    height: !model.isMovable || model.paperId === -1 ? 0: undefined
                    visible: model.isMovable && model.paperId !== -1
                }

                SkrMenuItem {
                    height: !model.isMovable || model.paperId === -1 ? 0: undefined
                    visible: model.isMovable && model.paperId !== -1
                    action:
                        Action {
                        id: moveUpAction
                        text: qsTr("Move up")
                        //shortcut: "Ctrl+Up"
                        icon {
                            source: "qrc:///icons/backup/object-order-raise.svg"
                        }
                        enabled: contextMenuItemIndex === model.index && listView.enabled
                                 && model.index !== 0
                        onTriggered: {
                            console.log("move up action", model.projectId,
                                        model.paperId)

                            if(temporarilyDisableMove){
                                return
                            }
                            temporarilyDisableMove = true
                            temporarilyDisableMoveTimer.start()

                            proxyModel.moveUp(model.projectId, model.paperId,
                                              model.index)


                        }
                    }
                }

                SkrMenuItem {
                    height: !model.isMovable || model.paperId === -1 ? 0: undefined
                    visible: model.isMovable && model.paperId !== -1
                    action:
                        Action {
                        id: moveDownAction
                        text: qsTr("Move down")
                        //shortcut: "Ctrl+Down"
                        icon {
                            source: "qrc:///icons/backup/object-order-lower.svg"
                        }
                        enabled: contextMenuItemIndex === model.index
                                 && model.index !== visualModel.items.count - 1  && listView.enabled

                        onTriggered: {
                            console.log("move down action", model.projectId,
                                        model.paperId)

                            if(temporarilyDisableMove){
                                return
                            }
                            temporarilyDisableMove = true
                            temporarilyDisableMoveTimer.start()

                            proxyModel.moveDown(model.projectId, model.paperId,
                                                model.index)
                        }
                    }

                }

                MenuSeparator {
                    height: !model.isTrashable || model.paperId === -1 ? 0: undefined
                    visible: model.isTrashable && model.paperId !== -1
                }

                SkrMenuItem {
                    height: !model.isTrashable || model.paperId === -1 ? 0: undefined
                    visible: model.isTrashable && model.paperId !== -1
                    action:
                        Action {
                        id: sendToTrashAction
                        text: qsTr("Send to trash")
                        //shortcut: "Del"
                        icon {
                            source: "qrc:///icons/backup/edit-delete.svg"
                        }
                        enabled: contextMenuItemIndex === model.index  && listView.enabled && model.indent !== -1
                        onTriggered: {
                            console.log("sent to trash action", model.projectId,
                                        model.paperId)

                            removePaperAnimation.start()
                            proxyModel.trashItemWithChildren(model.projectId, model.paperId)
                        }
                    }
                }
            }

            //----------------------------------------------------------

            property int animationDuration: 150

            ListView.onRemove: {


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
                id: removePaperAnimation
                PropertyAction {
                    property: "ListView.delayRemove"
                    value: true
                }

                ScriptAction {
                    script: delegateRoot.state = "unset_anchors"
                }

                NumberAnimation {
                    target: delegateRoot
                    property: "x"
                    to: listView.width
                    duration: 250
                    easing.type: Easing.InBack
                }
                PropertyAction {
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

    onActiveFocusChanged: {
        if (activeFocus) {
            listView.forceActiveFocus()
        }
    }


    listView.addDisplaced:      Transition {
        NumberAnimation { properties: "x,y"; duration: 500 }
    }


    listView.removeDisplaced: Transition {
        SequentialAnimation {
            PauseAnimation{duration: 250}
            NumberAnimation { properties: "x,y"; duration: 250 }
        }

    }


}
