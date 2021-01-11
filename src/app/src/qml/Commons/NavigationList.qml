import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import QtQuick.Controls.Material 2.15
import eu.skribisto.searchsheetlistproxymodel 1.0
import eu.skribisto.searchnotelistproxymodel 1.0
import "../Items"
import ".."

NavigationListForm {
    id: root

    property var proxyModel

    signal openDocument(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openDocumentInNewTab(int projectId, int paperId)
    signal openDocumentInNewWindow(int projectId, int paperId)
    signal showTrashedList()

    readonly property int currentParentId: priv.currentParentId
    readonly property int currentProjectId: priv.currentProjectId
    readonly property int currentPaperId: priv.currentPaperId
    property int openedProjectId: -2
    property int openedPaperId: -2

    property string iconUrl

    QtObject{
        id: priv
        property int currentParentId: -2
        property int currentProjectId: -2
        property int currentPaperId: -2

    }

    onCurrentParentIdChanged: {
        determineIfGoUpButtonEnabled()
    }

    onCurrentProjectIdChanged: {
        //        if (currentParent !== -2 & currentProject !== -2) {
        //            p_section.parentTitle = proxyModel.getItemName(
        //                        currentProject, currentParent)
        //            listView.section.delegate = sectionHeading
        //            //console.log("onCurrentProjectChanged")
        //        }
        //        // clear :
        //        if (currentParent === -2 & currentProject === -2 ){
        //            listView.section.delegate = null
        //        }
    }

    function setCurrentPaperId(projectId, paperId){


        //find parent id
        var ancestorsList = proxyModel.getAncestorsList(projectId, paperId, proxyModel.showTrashedFilter, proxyModel.showNotTrashedFilter)

        var newParentId = -1
        if(ancestorsList.length === 0){
            ancestorsList.push(-2)
        }
        else {
            newParentId = ancestorsList[0]
        }


        //compare with current parent id

        if(projectId === root.currentProjectId & newParentId === root.currentParentId){
            navigationListStackView.currentItem.proxyModel.setCurrentPaperId(projectId, paperId)
        }
        else if(projectId === root.currentProjectId){

        }
        else {
            navigationListStackView.pop(null)
            ancestorsList.reverse()
            ancestorsList.push(paperId)
            for(var i = 0 ; i < ancestorsList.length ; i++){
                var newItem = navigationListStackView.push(stackViewComponent, {"projectId": projectId, "paperId": ancestorsList[i] } )
                newItem.setCurrent()

            }

        }

        sideNavigationPopup.close()

        //
    }




    //-----------------------------------------------------------------------------

    //toolBarPrimaryColor: Material.color(Material.Cyan, Material.Shade200)

    //-----------------------------------------------------------------------------
    // go up button :

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

            var parentPaperId = proxyModel.getAncestorsList(root.currentProjectId, root.currentPaperId, proxyModel.showTrashedFilter, proxyModel.showNotTrashedFilter)[0]
            navigationListStackView.pop()
            navigationListStackView.currentItem.setCurrent()


        }
    }
    function determineIfGoUpButtonEnabled() {

        if(!root.visible){
            goUpAction.enabled = false
            return
        }

        goUpAction.enabled = (root.currentParentId !== -2)


    }

    onVisibleChanged: {
        determineIfGoUpButtonEnabled()
    }






    //-----------------------------------------------------------------------------
    // current parent button :
    //    Binding {
    //        target: root
    //        property: "currentProjectId"
    //        value: proxyModel.projectIdFilter
    //    }
    //    Binding {
    //        target: root
    //        property: "currentParentId"
    //        value: proxyModel.parentIdFilter
    //    }
    //currentParent: proxyModel.parentIdFilter
    //currentProject: proxyModel.projectIdFilter




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
            enabled: root.enabled && currentParentId !== -2
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
        enabled:  root.enabled && currentParentId !== -2
        icon{
            source: "qrc:///icons/backup/document-new.svg"
            height: 100
            width: 100
        }
        onTriggered: {
            console.log(currentProjectId, currentParentId, navigationListStackView.currentItem.visualModel.items.count)

            navigationListStackView.currentItem.proxyModel.addChildItem(currentProjectId, currentParentId,
                                                                        navigationListStackView.currentItem.visualModel.items.count)
            navigationListStackView.currentItem.listView.currentItem.editName()
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



    //----------------------------------------------------------------------------






    //----------------------------------------------------------------------------
    //----------------------------------------------------------------------------
    //----------------------------------------------------------------------------

    Component {
        id: stackViewComponent

        Item {
            id: stackViewBaseItem
            property alias listView: listView
            property alias visualModel: visualModel
            property var proxyModel : root.proxyModel.clone()
            property int currentIndex: listView.currentIndex
            property int parentId: -2
            property int projectId: -2
            property int paperId: -2

            // used to remember the source when moving an item
            property int moveSourceInt: -2

            focus: false

            Component.onCompleted: {
                init()

            }

            function init(){
                p_section.parentTitle = qsTr("Projects")
                listView.section.delegate = sectionHeading

                if(paperId === -2 & parentId !== -2 ){
                    stackViewBaseItem.proxyModel.setParentFilter(projectId, parentId)
                }
                else{
                    stackViewBaseItem.proxyModel.setCurrentPaperId(projectId, paperId)
                    parentId = stackViewBaseItem.proxyModel.parentIdFilter
                    console.log("parentId", parentId)

                }

                setCurrent()

                determineSectionTitle()
            }

            Component.onDestruction: {
                //    proxyModel.destroy()
            }


            function setCurrent(){
                priv.currentProjectId = projectId
                priv.currentParentId = parentId
                priv.currentPaperId = paperId

            }


            onActiveFocusChanged: {
                if(activeFocus){
                    listView.forceActiveFocus()
                }
            }


            function  determineSectionTitle(){

                if (parentId !== -2 && root.currentProjectId !== -2) {
                    var parentTitle = proxyModel.getItemName(projectId, parentId)
                    //console.log("onCurrentParentChanged")

                    p_section.parentTitle = parentTitle
                    listView.section.delegate = sectionHeading

                }
                else if (parentId === -2 && root.currentProjectId !== -2) {
                    var projectTitle = plmData.getProjectName(projectId)
                    //console.log("onCurrentParentChanged")

                    p_section.parentTitle = projectTitle
                    listView.section.delegate = sectionHeading

                }
                // clear :
                else if (parentId === -2 && root.currentProjectId === -2 ){
                    listView.section.delegate = null
                }
                // show "projects" section
                else if (parentId === -2 && parentId === -2){

                    p_section.parentTitle = qsTr("Projects")
                    listView.section.delegate = sectionHeading


                }
            }

            ScrollView {
                id: scrollView
                clip: true
                anchors.fill: parent
                focusPolicy: Qt.StrongFocus
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded//scrollBarVerticalPolicy





                ListView {
                    id: listView
                    smooth: true
                    boundsBehavior: Flickable.StopAtBounds
                    spacing: 1

                    Accessible.name: "Navigation list"
                    Accessible.role: Accessible.List



                    DelegateModel {
                        id: visualModel
                        delegate: dragDelegate
                        model: stackViewBaseItem.proxyModel
                    }
                    model: visualModel
                    // scrollBar interactivity :

                    onContentHeightChanged: {
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
                    onCurrentIndexChanged: {
                        contextMenuItemIndex = listView.currentIndex
                    }

                    property int contextMenuItemIndex: -2

                    Binding {
                        target: listView
                        property: "currentIndex"
                        value: proxyModel.forcedCurrentIndex
                    }

                    //----------------------------------------------------------------------------


                    // move :
                    addDisplaced:      Transition {
                        NumberAnimation { properties: "x,y"; duration: 500 }
                    }


                    removeDisplaced: Transition {
                        SequentialAnimation {
                            PauseAnimation{duration: 250}
                            NumberAnimation { properties: "x,y"; duration: 250 }
                        }

                    }


                    QtObject{
                        id: p_section
                        property string parentTitle: ""
                    }

                    //----------------------------------------------------------------------------
                    section.property: "indent"
                    section.criteria: ViewSection.FullString
                    section.labelPositioning: ViewSection.CurrentLabelAtStart |
                                              ViewSection.InlineLabels
                    section.delegate: null


                    // The delegate for each section header
                    Component {
                        id: sectionHeading
                        Rectangle {
                            width: listView.width
                            height: childrenRect.height
                            color: SkrTheme.buttonBackground

                            required property string section

                            SkrLabel {
                                anchors.left: parent.left
                                anchors.right: parent.right
                                text: p_section.parentTitle
                                font.bold: true
                                horizontalAlignment: Qt.AlignHCenter
                                color: SkrTheme.buttonForeground
                            }
                        }
                    }

                    //----------------------------------------------------------------------
                    //--- Start list item component ------------------------------------------
                    //----------------------------------------------------------------------



                    Component {
                        id: dragDelegate

                        DropArea {
                            id: delegateRoot
                            property int indent: model.indent

                            Accessible.name: labelLabel.text.length === 0 ? titleLabel.text  +  ( model.hasChildren ? " " +qsTr("is a folder") :  "" ):
                                                                            titleLabel.text + " " + qsTr("label:") + " " + labelLabel.text + ( model.hasChildren ? " " +qsTr("has child") :  "" )
                            Accessible.role: Accessible.ListItem
                            Accessible.description: qsTr("navigation item")

                            focus: true


                            keys: ["application/skribisto-paper"]
                            onEntered: {

                                content.sourceIndex = drag.source.visualIndex
                                visualModel.items.move(drag.source.visualIndex,
                                                       content.visualIndex)
                            }

                            onDropped: {
                                if(drop.proposedAction === Qt.MoveAction){

                                    console.log("dropped from :", moveSourceInt, "to :", content.visualIndex)
                                    proxyModel.moveItem(moveSourceInt, content.visualIndex)

                                }
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
                                    stackViewBaseItem.paperId = model.paperId
                                    stackViewBaseItem.setCurrent()
                                }
                            }


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
                                if(model.isMovable && ( event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_X){
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

                                // paste
                                if (model.canAddChildPaper && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_V && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                                    copyAction.trigger()
                                    event.accepted = true
                                }

                                // add before
                                if (model.canAddSiblingPaper && (event.modifiers & Qt.ControlModifier) && (event.modifiers & Qt.ShiftModifier) && event.key === Qt.Key_N && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                                    addBeforeAction.trigger()
                                    event.accepted = true
                                }

                                // add after
                                if (model.canAddSiblingPaper && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_N && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                                    addAfterAction.trigger()
                                    event.accepted = true
                                }

                                // add child
                                if (model.canAddChildPaper && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Space && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                                    addChildAction.trigger()
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

                            SkrListItemPane{
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
                                Drag.keys: ["application/skribisto-paper"]

                                Drag.supportedActions: Qt.MoveAction
                                //Drag.dragType: Drag.Automatic

                                borderWidth: 2
                                borderColor: dragHandler.active | content.dragging ? SkrTheme.accent : "transparent"
                                Behavior on borderColor {
                                    ColorAnimation {
                                        duration: 200
                                    }
                                }

                                property bool dragging: false
                                DragHandler {
                                    id: dragHandler
                                    //acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                                    //xAxis.enabled: false
                                    //grabPermissions: PointerHandler.TakeOverForbidden

                                    onActiveChanged: {
                                        if (active) {
                                            moveSourceInt = content.visualIndex
                                            cancelDragTimer.stop()
                                        } else {
                                            content.Drag.drop()
                                            content.dragging = false
                                        }
                                    }
                                    enabled: content.dragging

                                    grabPermissions: PointerHandler.CanTakeOverFromItems |PointerHandler.CanTakeOverFromAnything
                                }

                                Timer{
                                    id: cancelDragTimer
                                    repeat: false
                                    interval: 3000
                                    onTriggered: {
                                        content.dragging = false
                                    }
                                }

                                HoverHandler {
                                    id: hoverHandler
                                    acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus

                                    onHoveredChanged: {
                                        if(hovered && model.hasChildren){
                                            sideNavigationPopup.projectId = model.projectId
                                            sideNavigationPopup.parentId = model.paperId
                                            closeSideNavigationListPopupTimer.start()

                                        }
                                        else if(!model.hasChildren){
                                            sideNavigationPopup.close()

                                        }
                                        else{
                                            sideNavigationPopup.close()

                                        }
                                    }

                                }



                                TapHandler {
                                    id: tapHandler

                                    onSingleTapped: {
                                        if(content.dragging){
                                            eventPoint.accepted = false
                                            return
                                        }

                                        listView.currentIndex = model.index

                                        if(model.hasChildren){
                                            goToChildAction.trigger()
                                        }
                                        else{
                                            openDocumentAction.trigger()
                                        }




                                        //delegateRoot.forceActiveFocus()
                                        eventPoint.accepted = true
                                    }

                                    onDoubleTapped: {
                                        if(content.dragging){
                                            eventPoint.accepted = false
                                            return
                                        }
                                        //console.log("double tapped")
                                        listView.currentIndex = model.index


                                        if(model.hasChildren){
                                            //TODO: open folder outline
                                        }
                                        else{
                                            openDocumentAction.trigger()
                                        }

                                        eventPoint.accepted = true
                                    }

                                    onLongPressed: { // needed to activate the grab handler

                                        //                        if(content.dragging){
                                        //                            eventPoint.accepted = false
                                        //                            return
                                        //                        }


                                        content.dragging = true
                                        cancelDragTimer.start()
                                    }


                                    onGrabChanged: {
                                        point.accepted = false

                                    }

                                    grabPermissions: PointerHandler.ApprovesTakeOverByHandlersOfDifferentType
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

                                    grabPermissions: PointerHandler.TakeOverForbidden
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
                                    grabPermissions: PointerHandler.TakeOverForbidden
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
                                        //                                        if(!model.hasChildren && !model.canAddChildPaper){
                                        //                                            return false
                                        //                                        }

                                        if (!root.enabled){
                                            return false
                                        }

                                        if (listView.enabled && listView.currentIndex === model.index) {
                                            return true
                                        }

                                        return true
                                    }
                                    icon.source: model.hasChildren ? "qrc:///icons/backup/go-next.svg" : (model.canAddChildPaper ? "qrc:///icons/backup/list-add.svg" : "")
                                    text: model.hasChildren ? ">" : (model.canAddChildPaper ? qsTr("Add a sub-item") : qsTr("See sub-items"))
                                    onTriggered: {
                                        console.log("goToChildAction triggered")

                                        //                                        goToChildActionToBeTriggered = true
                                        //                                        goToChildActionCurrentIndent =  model.indent


                                        //                                        var _proxyModel = proxyModel

                                        //save current ids:
                                        listView.currentIndex = model.index
                                        navigationListStackView.currentItem.paperId = model.paperId

                                        sideNavigationPopup.close()

                                        // push new view

                                        var newItem = navigationListStackView.push(stackViewComponent, {"projectId": model.projectId, "parentId":  model.paperId} )
                                        newItem.setCurrent()


                                        //                                        currentProject = model.projectId
                                        //                                        currentParent = model.paperId
                                        //                                        var _currentProject = currentProject
                                        //                                        var _currentParent = currentParent
                                        //                                        var _index = model.index

                                        //                                        var _listView = listView


                                        //                                        // change level
                                        //                                        _proxyModel.setParentFilter(_currentProject,
                                        //                                                                    _currentParent)
                                        //                                        _proxyModel.addHistory(_currentProject, _index)

                                        //                                        // create a child if none present
                                        //                                        if (!_proxyModel.hasChildren(_currentProject,
                                        //                                                                     _currentParent)) {
                                        //                                            newPaperAdded = true
                                        //                                            _proxyModel.addChildItem(_currentProject,
                                        //                                                                     _currentParent, 0)

                                        //                                            // edit it :
                                        //                                            _listView.itemAtIndex(0).editName()

                                        //                                        }



                                    }
                                }

                                Timer{
                                    property int paperIdToEdit: -2
                                    onPaperIdToEditChanged: {
                                        if(paperIdToEdit !== -2){
                                            editNameTimer.start()
                                        }
                                    }


                                    id: editNameTimer
                                    repeat: false
                                    interval: animationDuration
                                    onTriggered: {
                                        var index = proxyModel.findVisualIndex(model.projectId, paperIdToEdit)
                                        if(index !== -2){
                                            listView.itemAtIndex(index).editName()
                                        }
                                        paperIdToEdit = -2
                                    }
                                }


                                Timer{
                                    property int paperIndexToEdit: -2
                                    onPaperIndexToEditChanged: {
                                        if(paperIndexToEdit !== -2){
                                            editNameWithIndexTimer.start()
                                        }
                                    }


                                    id: editNameWithIndexTimer
                                    repeat: false
                                    interval: animationDuration
                                    onTriggered: {
                                        if(paperIndexToEdit !== -2){
                                            visualModel.items.get(paperIndexToEdit).editName()
                                            //listView.itemAtIndex(paperIndexToEdit)
                                        }
                                        paperIndexToEdit = -2
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

                                contentItem: ColumnLayout{
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

                                        SkrToolButton {
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

                                        SkrToolButton {
                                            id: paperIconIndicator
                                            //visible: model.projectIsBackup && model.paperId === -1
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
                                                source: model.hasChildren ? "qrc:///icons/backup/document-open.svg" : root.iconUrl

                                                height: 22
                                                width: 22
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
                                                        visible: text.length === 0 ? false : true
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

                                            visible: hoverHandler.hovered || content.isCurrent
                                        }

                                        SkrToolButton {
                                            id: goToChildButton
                                            action: goToChildAction

                                            flat: true
                                            Layout.preferredWidth: 30
                                            Layout.fillHeight: true
                                            Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                                            visible: model.hasChildren ? true : (hoverHandler.hovered || content.isCurrent)
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
                                                color: "transparent";
                                            }
                                            GradientStop {
                                                position: 0.30;
                                                color: SkrTheme.divider;
                                            }
                                            GradientStop {
                                                position: 0.70;
                                                color: SkrTheme.divider;
                                            }
                                            GradientStop {
                                                position: 1.00;
                                                color: "transparent";
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
                                        target: labelTextField
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: titleTextField
                                        visible: true
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
                                width: 300

                                onOpened: {
                                    // necessary to differenciate between all items
                                    listView.contextMenuItemIndex = model.index
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

                                        enabled: listView.contextMenuItemIndex === model.index && titleTextField.visible === false && listView.enabled &&  model.paperId !== -1
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
                                        enabled: listView.contextMenuItemIndex === model.index && titleTextField.visible === false && listView.enabled &&  model.paperId !== -1
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
                                        enabled: listView.contextMenuItemIndex === model.index && titleTextField.visible === false && listView.enabled &&  model.paperId !== -1
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
                                    enabled: listView.contextMenuItemIndex === model.index && model.projectIsActive === false && listView.enabled &&  model.paperId === -1
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
                                        enabled: listView.contextMenuItemIndex === model.index && listView.enabled

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
                                        enabled: listView.contextMenuItemIndex === model.index  && listView.enabled
                                        onTriggered: {
                                            console.log("sel label", model.projectId,
                                                        model.paperId)
                                            delegateRoot.editLabel()
                                        }
                                    }
                                }

                                MenuSeparator {
                                    height: !model.isCopyable || !model.canAddChildPaper || model.paperId === -1 ? 0: undefined
                                    visible: model.isCopyable && model.canAddChildPaper && model.paperId !== -1
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
                                        enabled: listView.contextMenuItemIndex === model.index && listView.enabled

                                        onTriggered: {
                                            console.log("cut action", model.projectId,
                                                        model.paperId)
                                            proxyModel.cut(model.projectId, model.paperId)
                                        }
                                    }
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
                                        enabled: listView.contextMenuItemIndex === model.index && listView.enabled

                                        onTriggered: {
                                            console.log("copy action", model.projectId,
                                                        model.paperId)
                                            proxyModel.copy(model.projectId, model.paperId)
                                        }
                                    }
                                }


                                SkrMenuItem {
                                    height: !model.canAddChildPaper || model.paperId === -1 ? 0: undefined
                                    visible: model.canAddChildPaper && model.paperId !== -1
                                    action:
                                        Action {

                                        id: pasteAction
                                        text: qsTr("Paste")
                                        //shortcut: StandardKey.Copy
                                        icon {
                                            source: "qrc:///icons/backup/edit-paste.svg"
                                        }
                                        enabled: listView.contextMenuItemIndex === model.index && listView.enabled

                                        onTriggered: {
                                            console.log("paste action", model.projectId,
                                                        model.paperId)
                                            proxyModel.paste(model.projectId, model.paperId)
                                        }
                                    }
                                }

                                MenuSeparator {
                                    height: !(model.canAddSiblingPaper && model.canAddChildPaper) || model.paperId === -1 ? 0: undefined
                                    visible: (model.canAddSiblingPaper || model.canAddChildPaper) && model.paperId !== -1
                                }

                                SkrMenuItem {
                                    height: !model.canAddSiblingPaper || model.paperId === -1 ? 0: undefined
                                    visible: model.canAddSiblingPaper && model.paperId !== -1
                                    action:
                                        Action {
                                        id: addBeforeAction
                                        text: qsTr("Add before")
                                        //shortcut: "Ctrl+Shift+N"
                                        icon {
                                            source: "qrc:///icons/backup/document-new.svg"
                                        }
                                        enabled: listView.contextMenuItemIndex === model.index && listView.enabled
                                        onTriggered: {
                                            console.log("add before action", model.projectId,
                                                        model.paperId)

                                            var visualIndex = listView.currentIndex
                                            proxyModel.addItemAbove(model.projectId, model.paperId, visualIndex)
                                            listView.itemAtIndex(visualIndex).editName()
                                        }
                                    }
                                }

                                SkrMenuItem {
                                    height: !model.canAddSiblingPaper || model.paperId === -1 ? 0: undefined
                                    visible: model.canAddSiblingPaper && model.paperId !== -1
                                    action:
                                        Action {
                                        id: addAfterAction
                                        text: qsTr("Add after")
                                        //shortcut: "Ctrl+N"
                                        icon {
                                            source: "qrc:///icons/backup/document-new.svg"
                                        }
                                        enabled: listView.contextMenuItemIndex === model.index && listView.enabled
                                        onTriggered: {
                                            console.log("add after action", model.projectId,
                                                        model.paperId)

                                            var visualIndex = listView.currentIndex + 1
                                            proxyModel.addItemBelow(model.projectId, model.paperId, visualIndex)
                                            listView.itemAtIndex(visualIndex).editName()
                                            //editNameWithIndexTimer.paperIndexToEdit = visualIndex

                                        }
                                    }

                                }

                                SkrMenuItem {
                                    height: !model.canAddChildPaper || model.paperId === -1 ? 0: undefined
                                    visible: model.canAddChildPaper && model.paperId !== -1
                                    action:
                                        Action {
                                        id: addChildAction
                                        text: qsTr("Add a sub-item")
                                        //shortcut: "Ctrl+N"
                                        icon {
                                            source: "qrc:///icons/backup/document-new.svg"
                                        }
                                        enabled: listView.contextMenuItemIndex === model.index && listView.enabled
                                        onTriggered: {
                                            console.log("add child action", model.projectId,
                                                        model.paperId)


                                            //save current ids:
                                            listView.currentIndex = model.index
                                            navigationListStackView.currentItem.paperId = model.paperId

                                            // push new view

                                            var newItem = navigationListStackView.push(stackViewComponent, {"projectId": model.projectId, "parentId":  model.paperId} )
                                            newItem.setCurrent()

                                            addPaperAction.trigger()

                                            //                                            var result = proxyModel.addChildItem(currentProjectId, currentPaperId, listView.count)

                                            //                                            var paperId = result.getData("paperId", -2)
                                            //                                            root.setCurrentPaperId(currentProjectId, paperId)

                                            //                                            goToChildActionToBeTriggered = true
                                            //                                            goToChildActionCurrentIndent =  model.indent


                                            //                                            var _proxyModel = proxyModel
                                            //                                            currentProject = model.projectId
                                            //                                            currentParent = model.paperId
                                            //                                            var _currentProject = currentProject
                                            //                                            var _currentParent = currentParent
                                            //                                            var _index = model.index

                                            //                                            var _listView = listView


                                            //                                            // change level
                                            //                                            _proxyModel.setParentFilter(_currentProject,
                                            //                                                                        _currentParent)
                                            //                                            _proxyModel.addHistory(_currentProject, _index)

                                            //                                            newPaperVisualIndex = _listView.count
                                            //                                            newPaperAdded = true // triggers editName()
                                            //                                            _proxyModel.addChildItem(_currentProject,
                                            //                                                                     _currentParent, _listView.count)

                                            //                                            // edit it :




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
                                        enabled: listView.contextMenuItemIndex === model.index && listView.enabled
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
                                        enabled: listView.contextMenuItemIndex === model.index
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
                                        enabled: listView.contextMenuItemIndex === model.index  && listView.enabled && model.indent !== -1
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

                            //                            ListView.onRemove: {


                            //                                var goUpActionCurrentParentIndent = proxyModel.getItemIndent(currentProject, goUpActionCurrentParent)

                            //                                if(goUpActionToBeTriggered && goUpActionCurrentParentIndent + 1 === delegateRoot.indent){
                            //                                    paperGoesRightAnimation.start()
                            //                                }


                            //                                if(goToChildActionToBeTriggered && goToChildActionCurrentIndent === delegateRoot.indent){
                            //                                    paperGoesLeftAnimation.start()

                            //                                }
                            //                            }

                            //goUpActionToBeTriggered
                            //                            ListView.onAdd: {

                            //                                var goUpActionCurrentParentIndent = proxyModel.getItemIndent(currentProject, goUpActionCurrentParent)

                            //                                if(goUpActionToBeTriggered && goUpActionCurrentParentIndent === model.indent){
                            //                                    paperAppearsFromLeftAnimation.start()
                            //                                }



                            //                                if(goToChildActionToBeTriggered && goToChildActionCurrentIndent + 1 === delegateRoot.indent){
                            //                                    paperAppearsFromRightAnimation.start()
                            //                                }
                            //                            }


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
                        }
                    }













                    //----------------------------------------------------------------------
                    //--- End list item component ------------------------------------------
                    //----------------------------------------------------------------------
                }

            }
        }
    }



    //-------------------------------------------------------------------------------------
    //---------side Navigation Popup---------------------------------------------------------
    //-------------------------------------------------------------------------------------

    SkrPopup {
        id: sideNavigationPopup
        property int projectId: -2
        property int parentId: -2

        x: root.width
        y: navigationListStackView.y
        width: 200
        height: navigationListStackView.height
        modal: false
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutsideParent
        padding: 0

        Loader {
            id: sideNavigationLoader
            anchors.fill: parent
            asynchronous: true
            sourceComponent: stackViewComponent
            visible: status == Loader.Ready

            onLoaded: {
                item.init()
            }

            Binding {
                target: sideNavigationLoader.item
                property: "projectId"
                value: sideNavigationPopup.projectId
            }
            Binding {
                target: sideNavigationLoader.item
                property: "parentId"
                value: sideNavigationPopup.parentId
            }

        }

    }

    Timer{
        id: closeSideNavigationListPopupTimer
        interval: 20
        onTriggered: {

            if(sideNavigationPopup.visible){
                sideNavigationPopup.close()
            }


            sideNavigationPopup.open()

        }

    }
    //-------------------------------------------------------------------------------------
    //-------------------------------------------------------------------------------------
    //-------------------------------------------------------------------------------------





    onActiveFocusChanged: {
        if (activeFocus) {
            navigationListStackView.currentItem.forceActiveFocus()
        }
    }




}
