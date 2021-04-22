import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import QtQuick.Controls.Material 2.15
import eu.skribisto.searchtreelistproxymodel 1.0
import "../Items"
import ".."

NavigationListForm {
    id: root

    property var proxyModel

    signal openDocument(int openedProjectId, int openedTreeItemId, int projectId, int treeItemId)
    signal openDocumentInAnotherView(int projectId, int treeItemId)
    signal openDocumentInNewWindow(int projectId, int treeItemId)
    signal showTrashedList()

    readonly property int currentParentId: priv.currentParentId
    readonly property int currentProjectId: priv.currentProjectId
    readonly property int currentTreeItemId: priv.currentTreeItemId
    property int openedProjectId: -2
    property int openedTreeItemId: -2


    QtObject{
        id: priv
        property int currentParentId: -2
        property int currentProjectId: -2
        property int currentTreeItemId: -2
        property bool dragging: false
        property bool renaming: false

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

    Component.onCompleted: {
    }
    function setCurrentTreeItemParentId(projectId, treeItemParentId){


        //find parent id
        var ancestorsList = proxyModel.getAncestorsList(projectId, treeItemParentId, proxyModel.showTrashedFilter, proxyModel.showNotTrashedFilter)



        //compare with current parent id

        if(projectId === root.currentProjectId & treeItemParentId === root.currentParentId){
            navigationListStackView.currentItem.proxyModel.setCurrentTreeItemId(projectId, -1)
        }

        else {
            navigationListStackView.pop(null)
            ancestorsList.reverse()
            ancestorsList.push(treeItemParentId)

            //project item
            navigationListStackView.get(0).projectId = projectId
            navigationListStackView.get(0).treeItemId = 0
            navigationListStackView.get(0).init()
            navigationListStackView.get(0).setCurrent()


            for(var i = 1 ; i < ancestorsList.length ; i++){
                var newItem = navigationListStackView.push(stackViewComponent, {"projectId": projectId, "treeItemId": ancestorsList[i] } )
                newItem.setCurrent()
            }

            var lastNewItem = navigationListStackView.push(stackViewComponent, {"projectId": projectId, "parentId":  treeItemParentId} )
            lastNewItem.setCurrent()
            rootWindow.protectedSignals.setBreadcrumbCurrentTreeItemCalled(priv.currentProjectId, priv.currentParentId)


        }

        sidePopupListModel.clear()
        determineIfGoUpButtonEnabled()
    }

    function setCurrentTreeItemId(projectId, treeItemId){


        //find parent id
        var ancestorsList = proxyModel.getAncestorsList(projectId, treeItemId, proxyModel.showTrashedFilter, proxyModel.showNotTrashedFilter)

        var newParentId = ancestorsList[0]



        //compare with current parent id

        if(projectId === root.currentProjectId & newParentId === root.currentParentId){
            navigationListStackView.currentItem.proxyModel.setCurrentTreeItemId(projectId, treeItemId)
        }
        //        else if(projectId === root.currentProjectId){

        //        }
        else {
            navigationListStackView.pop(null)
            ancestorsList.reverse()
            ancestorsList.push(treeItemId)

            //project item
            navigationListStackView.get(0).projectId = projectId
            navigationListStackView.get(0).treeItemId = 0
            navigationListStackView.get(0).init()
            navigationListStackView.get(0).setCurrent()


            for(var i = 1 ; i < ancestorsList.length ; i++){
                var newItem = navigationListStackView.push(stackViewComponent, {"projectId": projectId, "treeItemId": ancestorsList[i] } )
                newItem.setCurrent()
            }



        }

        sidePopupListModel.clear()
        determineIfGoUpButtonEnabled()


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

            //var parentTreeItemId = proxyModel.getAncestorsList(root.currentProjectId, root.currentTreeItemId, proxyModel.showTrashedFilter, proxyModel.showNotTrashedFilter)[0]
            navigationListStackView.pop()
            navigationListStackView.currentItem.setCurrent()
            rootWindow.protectedSignals.setBreadcrumbCurrentTreeItemCalled(priv.currentProjectId, priv.currentParentId)


        }
    }
    function determineIfGoUpButtonEnabled() {

        if(!root.visible){
            goUpAction.enabled = false
            return
        }

        goUpAction.enabled = (root.currentParentId !== -2)


    }
    goUpToolButton.onPressAndHold: {
        navigationListStackView.pop(null)
        navigationListStackView.currentItem.setCurrent()
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
        id: addTreeItemAction
        text: qsTr("Add")
        shortcut: "Ctrl+T"
        enabled:  root.enabled && currentParentId !== -2
        icon{
            source: "qrc:///icons/backup/list-add.svg"
            height: 100
            width: 100
        }
        onTriggered: {
            addItemMenu.open()
        }
    }

    addToolButton.action: addTreeItemAction

    function addItemAtCurrentParent(type){
        //console.log(currentProjectId, currentParentId, navigationListStackView.currentItem.visualModel.items.count)

        navigationListStackView.currentItem.proxyModel.addChildItem(currentProjectId, currentParentId, type)
        navigationListStackView.currentItem.listView.currentItem.editName()
    }

    function getIconUrlFromPageType(type){
        return skrTreeManager.getIconUrlFromPageType(type)
    }

    function getPageTypeText(type){
        return skrTreeManager.getPageTypeText(type)
    }


    SkrMenu{
        id: addItemMenu
        y: addToolButton.height
        x: addToolButton.x

        Repeater {

            model: skrTreeManager.getPageTypeList(true);

            SkrMenuItem{
                text: getPageTypeText(modelData)
                property string type: modelData
                icon.source: getIconUrlFromPageType(modelData)
                onTriggered: {
                    addItemAtCurrentParent(type)
                }

            }
        }


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
            property int treeItemId: -2
            property int popupId: -1
            property bool hoverEnabled: true

            // used to remember the source when moving an item
            property int moveSourceInt: -2

            focus: false

            Component.onCompleted: {
                init()
            }


            function init(){
                p_section.parentTitle = qsTr("Projects")
                listView.section.delegate = sectionHeading

                if(treeItemId === -2 && parentId !== -2 ){
                    stackViewBaseItem.proxyModel.setParentFilter(projectId, parentId)
                }
                else{
                    stackViewBaseItem.proxyModel.setCurrentTreeItemId(projectId, treeItemId)
                    parentId = stackViewBaseItem.proxyModel.parentIdFilter

                }
                determineSectionTitle()
            }

            Component.onDestruction: {
                delete proxyModel
            }


            function setCurrent(){
                priv.currentProjectId = projectId
                priv.currentParentId = parentId
                priv.currentTreeItemId = treeItemId

                listView.positionViewAtIndex(currentIndex, ListView.Contain)

            }




            onActiveFocusChanged: {
                if(activeFocus){
                    listView.forceActiveFocus()
                }
            }


            function  determineSectionTitle(){

                var projectId = stackViewBaseItem.projectId
                var parentId = stackViewBaseItem.parentId
                if (parentId === 0 && projectId !== -2) {
                    var projectTitle = plmData.projectHub().getProjectName(projectId)

                    p_section.parentTitle = projectTitle
                    listView.section.delegate = sectionHeading

                }
                else if (parentId !== -2 && projectId !== -2) {
                    var parentTitle = proxyModel.getItemName(projectId, parentId)
                    //console.log("onCurrentParentChanged")

                    p_section.parentTitle = parentTitle
                    listView.section.delegate = sectionHeading

                }
                else if (parentId === -2 ) {
                    // show "projects" section

                    p_section.parentTitle = qsTr("Projects")
                    listView.section.delegate = sectionHeading

                }

                // clear :
                else if (parentId === -2 && root.currentProjectId === -2 ){
                    listView.section.delegate = null
                }

            }

            Item {
                id: topDraggingMover
                anchors.top:  scrollView.top
                anchors.right:  scrollView.right
                anchors.left:  scrollView.left
                height: 30
                z: 1

                visible: priv.dragging

                HoverHandler {
                    onHoveredChanged: {
                        if(hovered){
                            topDraggingMoverTimer.start()
                        }
                        else {
                            topDraggingMoverTimer.stop()
                        }
                    }
                }

                Timer {
                    id: topDraggingMoverTimer
                    repeat: true
                    interval: 10
                    onTriggered: {
                        if(listView.atYBeginning){
                            listView.contentY  = 0
                        }
                        else {
                            listView.contentY  = listView.contentY - 2

                        }
                    }

                }

            }

            Item {
                id: bottomDraggingMover
                anchors.bottom:  scrollView.bottom
                anchors.right:  scrollView.right
                anchors.left:  scrollView.left
                height: 30
                z: 1

                visible: priv.dragging

                HoverHandler {
                    onHoveredChanged: {
                        if(hovered){
                            bottomDraggingMoverTimer.start()
                        }
                        else {
                            bottomDraggingMoverTimer.stop()
                        }
                    }
                }

                Timer {
                    id: bottomDraggingMoverTimer
                    repeat: true
                    interval: 10
                    onTriggered: {
                        if(listView.atYEnd){
                            listView.positionViewAtEnd()
                        }
                        else {
                            listView.contentY  = listView.contentY + 2

                        }
                    }

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
                        delegate: listItemComponent
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
                    displaced: Transition {
                             NumberAnimation { properties: "x,y"; duration: 250 }
                         }

                    moveDisplaced: Transition {
                             NumberAnimation { properties: "x,y"; duration: 250 }
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
                        id: listItemComponent


                        SwipeDelegate {
                            id: swipeDelegate
                            property int indent: model.indent
                            focus: true

                            Accessible.name: labelLabel.text.length === 0 ? titleLabel.text  +  ( model.hasChildren ? " " +qsTr("is a folder") :  "" ):
                                                                            titleLabel.text + " " + qsTr("label:") + " " + labelLabel.text + ( model.hasChildren ? " " +qsTr("has child") :  "" )
                            Accessible.role: Accessible.ListItem
                            Accessible.description: qsTr("navigation item")

                            anchors {
                                left: Qt.isQtObject(parent) ? parent.left : undefined
                                right: Qt.isQtObject(parent) ? parent.right : undefined
                                rightMargin: 5
                            }

                            height: 50

                            padding: 0
                            topInset: 0
                            bottomInset: 0

                            property int visualIndex: {
                                return DelegateModel.itemsIndex
                            }

                            Binding {
                                target: content
                                property: "visualIndex"
                                value: visualIndex
                            }

                            onActiveFocusChanged: {
                                if(listView.currentIndex === model.index && model.index !== -1 && activeFocus){
                                    stackViewBaseItem.treeItemId = model.treeItemId
                                    stackViewBaseItem.setCurrent()
                                }
                            }

                            function editName() {
                                priv.renaming = true
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
                                priv.renaming = true
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
                                if( event.key === Qt.Key_Escape  && (swipeDelegate.state == "edit_name" || swipeDelegate.state == "edit_label")){
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
                                    openDocumentInAnotherViewAction.trigger()
                                    event.accepted = true
                                }
                                if (model.isOpenable && event.key === Qt.Key_Return && swipeDelegate.state !== "edit_name" && swipeDelegate.state !== "edit_label"){
                                    console.log("Return key pressed")
                                    openDocumentAction.trigger()
                                    event.accepted = true
                                }
                                // rename

                                if (model.isRenamable && event.key === Qt.Key_F2 && swipeDelegate.state !== "edit_name" && swipeDelegate.state !== "edit_label"){
                                    renameAction.trigger()
                                    event.accepted = true
                                }



                                // cut
                                if (model.isMovable && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_X && swipeDelegate.state !== "edit_name" && swipeDelegate.state !== "edit_label"){
                                    cutAction.trigger()
                                    event.accepted = true
                                }

                                // copy
                                if (model.isCopyable && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_C && swipeDelegate.state !== "edit_name" && swipeDelegate.state !== "edit_label"){
                                    copyAction.trigger()
                                    event.accepted = true
                                }

                                // paste
                                if (model.canAddChildTreeItem && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_V && swipeDelegate.state !== "edit_name" && swipeDelegate.state !== "edit_label"){
                                    copyAction.trigger()
                                    event.accepted = true
                                }

                                // add before
                                if (model.canAddSiblingTreeItem && (event.modifiers & Qt.ControlModifier) && (event.modifiers & Qt.ShiftModifier) && event.key === Qt.Key_N && swipeDelegate.state !== "edit_name" && swipeDelegate.state !== "edit_label"){
                                    addBeforeAction.trigger()
                                    event.accepted = true
                                }

                                // add after
                                if (model.canAddSiblingTreeItem && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_N && swipeDelegate.state !== "edit_name" && swipeDelegate.state !== "edit_label"){
                                    addAfterAction.trigger()
                                    event.accepted = true
                                }

                                // add child
                                if (model.canAddChildTreeItem && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Space && swipeDelegate.state !== "edit_name" && swipeDelegate.state !== "edit_label"){
                                    addChildAction.trigger()
                                    event.accepted = true
                                }

                                // move up
                                if (model.isMovable && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Up && swipeDelegate.state !== "edit_name" && swipeDelegate.state !== "edit_label"){
                                    moveUpAction.trigger()
                                    event.accepted = true
                                }

                                // move down
                                if (model.isMovable && (event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Down && swipeDelegate.state !== "edit_name" && swipeDelegate.state !== "edit_label"){
                                    moveDownAction.trigger()
                                    event.accepted = true
                                }

                                // send to trash
                                if (model.isTrashable && event.key === Qt.Key_Delete && swipeDelegate.state !== "edit_name" && swipeDelegate.state !== "edit_label"){
                                    sendToTrashAction.trigger()
                                    event.accepted = true
                                }
                            }



                            swipe.right: RowLayout {

                                height: parent.height
                                anchors.right: parent.right

                                SkrToolButton {
                                    id: sendToTrashToolButton

                                    focusPolicy: Qt.NoFocus
                                    display: AbstractButton.IconOnly

                                    Layout.fillHeight: true
                                    Layout.fillWidth: true

                                    action: sendToTrashAction
                                }

                                Item {
                                    id: stretcher
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 50

                                }

                                SkrToolButton {
                                    id: openDocumentToolButton
                                    visible: model.hasChildren
                                    focusPolicy: Qt.NoFocus
                                    display: AbstractButton.IconOnly

                                    Layout.fillHeight: true
                                    Layout.fillWidth: true

                                    action: openTreeItemAction
                                }


                                SkrToolButton {
                                    id: openMergedViewToolButton
                                    visible: model.hasChildren
                                    focusPolicy: Qt.NoFocus
                                    display: AbstractButton.IconOnly

                                    Layout.fillHeight: true
                                    Layout.fillWidth: true

                                    icon.source: "qrc:///icons/backup/view-list-text.svg"
                                }
                            }


                            swipe.onOpened: closeSwipeTimer.start()
                            //                            Item {
                            //                                anchors.fill: parent
                            //                                z:1

                            Timer{
                                id: closeSwipeTimer
                                interval: 3000
                                onTriggered: {
                                    swipeDelegate.swipe.close()
                                }
                            }

                            HoverHandler {
                                id: itemHoverHandler
                                acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                                enabled:  stackViewBaseItem.hoverEnabled
                                //grabPermissions:  PointerHandler.CanTakeOverFromItems | PointerHandler.CanTakeOverFromHandlersOfSameType | PointerHandler.ApprovesTakeOverByAnything

                                onHoveredChanged: {
                                    //console.log("item hovered", itemHoverHandler.hovered)
                                    if(hovered && model.hasChildren){

                                        if(!rootWindow.compactMode){

                                            hoveringTimer.start()
                                        }
                                    }
                                    else if(hovered && !model.hasChildren){
                                        if(!addSideNavigationListPopupTimer.running){
                                            hoveringTimer.stop()
                                            addSideNavigationListPopupTimer.stop()
                                            closeSideNavigationListPopupTimer.popupId = stackViewBaseItem.popupId + 1
                                            closeSideNavigationListPopupTimer.start()
                                        }


                                    }
                                    else if(!model.hasChildren){
                                        if(!addSideNavigationListPopupTimer.running){
                                            hoveringTimer.stop()
                                            closeSideNavigationListPopupTimer.popupId = stackViewBaseItem.popupId + 1
                                            closeSideNavigationListPopupTimer.start()
                                        }
                                    }

                                    else{
                                        hoveringTimer.stop()
                                        closeSideNavigationListPopupTimer.stop()
                                        //addSideNavigationListPopupTimer.stop()
                                    }
                                }





                                //                                }

                            }

                            Timer {
                                id: hoveringTimer
                                interval: 200
                                onTriggered: {


                                    addSideNavigationListPopupTimer.projectId = model.projectId
                                    addSideNavigationListPopupTimer.parentId = model.treeItemId
                                    addSideNavigationListPopupTimer.popupId = stackViewBaseItem.popupId
                                    addSideNavigationListPopupTimer.parentItem = swipeDelegate
                                    addSideNavigationListPopupTimer.listView = listView
                                    //                                            console.log("popupId", stackViewBaseItem.popupId)

                                    addSideNavigationListPopupTimer.start()
                                }

                            }



                            contentItem:
                                DropArea {
                                id: dropArea



                                keys: ["application/skribisto-tree-item"]
                                onEntered: {

                                    content.sourceIndex = drag.source.visualIndex
                                    visualModel.items.move(drag.source.visualIndex,
                                                           content.visualIndex)
                                }

                                onDropped: {
                                    if(drop.proposedAction === Qt.MoveAction){

                                        console.log("dropped from :", moveSourceInt, "to :", content.visualIndex)
                                        listView.interactive = true
                                        priv.dragging = false
                                        cancelDragTimer.stop()
                                        proxyModel.moveItem(moveSourceInt, content.visualIndex)

                                    }
                                }




                                SkrListItemPane{
                                    id: content
                                    property int visualIndex: 0
                                    property int sourceIndex: -2
                                    property int projectId: model.projectId
                                    property int treeItemId: model.treeItemId

                                    property bool isCurrent: model.index === listView.currentIndex ? true : false

                                    anchors {
                                        horizontalCenter: parent.horizontalCenter
                                        verticalCenter: parent.verticalCenter
                                    }
                                    width: parent.width
                                    height: swipeDelegate.height

                                    Drag.active: mouseDragHandler.active | touchDragHandler.active
                                    Drag.source: content
                                    Drag.hotSpot.x: width / 2
                                    Drag.hotSpot.y: height / 2
                                    Drag.keys: ["application/skribisto-tree-item"]

                                    Drag.supportedActions: Qt.MoveAction
                                    //Drag.dragType: Drag.Automatic

                                    borderWidth: 2
                                    borderColor: touchDragHandler.active | content.dragging ? SkrTheme.accent : "transparent"
                                    Behavior on borderColor {
                                        ColorAnimation {
                                            duration: 200
                                        }
                                    }

                                    property bool dragging: false
                                    DragHandler {
                                        id: mouseDragHandler
                                        acceptedDevices: PointerDevice.Mouse
                                        //xAxis.enabled: false
                                        //grabPermissions: PointerHandler.TakeOverForbidden

                                        onActiveChanged: {
                                            if (active) {
                                                listView.interactive = false
                                                moveSourceInt = content.visualIndex
                                                priv.dragging = true
                                                cancelDragTimer.stop()
                                            } else {
                                                cancelDragTimer.stop()
                                                priv.dragging = false
                                                content.dragging = false
                                                //content.Drag.drop()
                                            }
                                        }
                                        enabled: true

                                        grabPermissions: PointerHandler.CanTakeOverFromItems |PointerHandler.CanTakeOverFromAnything
                                    }

                                    DragHandler {
                                        id: touchDragHandler
                                        acceptedDevices: PointerDevice.TouchScreen | PointerDevice.Stylus
                                        //xAxis.enabled: false
                                        //grabPermissions: PointerHandler.TakeOverForbidden

                                        onActiveChanged: {
                                            if (active) {
                                                listView.interactive = false
                                                moveSourceInt = content.visualIndex
                                                priv.dragging = true
                                                cancelDragTimer.stop()
                                            } else {
                                                listView.interactive = true
                                                cancelDragTimer.stop()
                                                priv.dragging = false
                                                content.dragging = false
                                                //content.Drag.drop()
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
                                            priv.dragging = false
                                            content.dragging = false
                                        }
                                    }





                                    TapHandler {
                                        id: tapHandler

                                        onSingleTapped: {
                                            if(content.dragging){
                                                eventPoint.accepted = false
                                                return
                                            }
                                            if(eventPoint.event.device.type === PointerDevice.Mouse){
                                                listView.interactive = false
                                            }

                                            if(eventPoint.event.device.type === PointerDevice.TouchScreen | eventPoint.event.device.type === PointerDevice.Stylus ){
                                                listView.interactive = true
                                            }

                                            listView.currentIndex = model.index
                                            priv.currentTreeItemId = model.treeItemId

                                            if(plmData.treePropertyHub().getProperty(model.projectId, model.treeItemId,
                                                                                     "can_add_child_paper", "true") === "true"){
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
                                            if(eventPoint.event.device.type === PointerDevice.Mouse ){
                                                listView.interactive = false
                                            }

                                            if(eventPoint.event.device.type === PointerDevice.TouchScreen | eventPoint.event.device.type === PointerDevice.Stylus ){
                                                listView.interactive = true
                                            }
                                            //console.log("double tapped")
                                            listView.currentIndex = model.index
                                            priv.currentTreeItemId = model.treeItemId


                                            if(model.hasChildren){
                                                //TODO: open folder outline
                                            }
                                            else{
                                                openDocumentAction.trigger()
                                            }

                                            eventPoint.accepted = true
                                        }


                                        onGrabChanged: {
                                            point.accepted = false

                                        }

                                        grabPermissions: PointerHandler.ApprovesTakeOverByHandlersOfDifferentType
                                    }


                                    TapHandler {
                                        id: rightClickTapHandler
                                        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                                        acceptedButtons: Qt.RightButton
                                        onTapped: {
                                            listView.interactive = eventPoint.event.device.type === PointerDevice.Mouse

                                            if(menu.visible){
                                                menu.close()
                                                return
                                            }


                                            listView.currentIndex = model.index
                                            priv.currentTreeItemId = model.treeItemId
                                            menu.open()
                                            eventPoint.accepted = true
                                        }

                                        grabPermissions: PointerHandler.TakeOverForbidden
                                    }

                                    TapHandler {
                                        id: middleClickTapHandler
                                        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                                        acceptedButtons: Qt.MiddleButton
                                        onTapped: {
                                            listView.interactive = eventPoint.event.device.type === PointerDevice.Mouse
                                            listView.currentIndex = model.index
                                            priv.currentTreeItemId = model.treeItemId
                                            swipeDelegate.forceActiveFocus()
                                            openDocumentInAnotherViewAction.trigger()
                                            eventPoint.accepted = true

                                        }
                                        grabPermissions: PointerHandler.TakeOverForbidden
                                    }

                                    /// without MouseArea, it breaks while dragging and scrolling:
                                    MouseArea {
                                        anchors.fill: parent
                                        acceptedButtons: Qt.NoButton
                                        onWheel: {
                                            listView.interactive = false
                                            listView.flick(0, wheel.angleDelta.y * 50)
                                            wheel.accepted = true
                                        }

                                        enabled: listView.interactive === false
                                    }


                                    TapHandler {
                                        acceptedDevices: PointerDevice.TouchScreen | PointerDevice.Stylus

                                        onLongPressed: { // needed to activate the grab handler

                                            //                        if(content.dragging){
                                            //                            eventPoint.accepted = false
                                            //                            return
                                            //                        }


                                            content.dragging = true
                                            listView.interactive = false
                                            cancelDragTimer.start()
                                        }

                                    }


                                    Action {
                                        id: goToChildAction
                                        //shortcut: "Right"
                                        enabled: listView.enabled && listView.currentIndex === model.index && (model.type === "FOLDER" || model.type === "PROJECT")

                                        //icon.source: model.hasChildren ? "qrc:///icons/backup/go-next.svg" : (model.canAddChildTreeItem ? "qrc:///icons/backup/list-add.svg" : "")
                                        text:  qsTr("See sub-items")
                                        onTriggered: {
                                            console.log("goToChildAction triggered")

                                            //                                        goToChildActionToBeTriggered = true
                                            //                                        goToChildActionCurrentIndent =  model.indent


                                            //                                        var _proxyModel = proxyModel

                                            //save current ids:
                                            listView.currentIndex = model.index
                                            navigationListStackView.currentItem.treeItemId = model.treeItemId

                                            sidePopupListModel.clear()

                                            // push new view

                                            var newItem = navigationListStackView.push(stackViewComponent, {"projectId": model.projectId, "parentId":  model.treeItemId} )
                                            newItem.setCurrent()
                                            rootWindow.protectedSignals.setBreadcrumbCurrentTreeItemCalled(priv.currentProjectId, priv.currentParentId)


                                            //                                        currentProject = model.projectId
                                            //                                        currentParent = model.treeItemId
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
                                        property int treeItemIdToEdit: -2
                                        onTreeItemIdToEditChanged: {
                                            if(treeItemIdToEdit !== -2){
                                                editNameTimer.start()
                                            }
                                        }


                                        id: editNameTimer
                                        repeat: false
                                        interval: animationDuration
                                        onTriggered: {
                                            var index = proxyModel.findVisualIndex(model.projectId, treeItemIdToEdit)
                                            if(index !== -2){
                                                listView.itemAtIndex(index).editName()
                                            }
                                            treeItemIdToEdit = -2
                                        }
                                    }


                                    Timer{
                                        property int treeItemIndexToEdit: -2
                                        onTreeItemIndexToEditChanged: {
                                            if(treeItemIndexToEdit !== -2){
                                                editNameWithIndexTimer.start()
                                            }
                                        }


                                        id: editNameWithIndexTimer
                                        repeat: false
                                        interval: animationDuration
                                        onTriggered: {
                                            if(treeItemIndexToEdit !== -2){
                                                visualModel.items.get(treeItemIndexToEdit).editName()
                                                //listView.itemAtIndex(treeItemIndexToEdit)
                                            }
                                            treeItemIndexToEdit = -2
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
                                            root.openDocument(openedProjectId, openedTreeItemId, model.projectId,
                                                              model.treeItemId)
                                        }
                                    }

                                    Action {
                                        id: openDocumentInAnotherViewAction
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
                                            root.openDocumentInAnotherView(model.projectId,
                                                                           model.treeItemId)

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
                                                                         model.treeItemId)
                                            // solve bug where this window's tree disapears
                                            setCurrentTreeItemIdTimer.projectId = model.projectId
                                            setCurrentTreeItemIdTimer.treeItemId = model.treeItemId

                                            setCurrentTreeItemIdTimer.start()

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
                                                visible: model.projectId === openedProjectId && model.treeItemId === openedTreeItemId
                                            }

                                            SkrToolButton {
                                                id: projectIsBackupIndicator
                                                visible: model.projectIsBackup && model.type === 'PROJECT'
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

                                            Item {
                                                Layout.maximumHeight: 30
                                                implicitHeight: treeItemIconIndicator.implicitHeight
                                                implicitWidth: treeItemIconIndicator.implicitWidth

                                                SkrToolButton {
                                                    id: treeItemIconIndicator
                                                    //visible: model.projectIsBackup && model.treeItemId === -1
                                                    enabled: true
                                                    focusPolicy: Qt.NoFocus
                                                    implicitHeight: 32
                                                    implicitWidth: 32
                                                    padding: 0
                                                    rightPadding: 0
                                                    bottomPadding: 0
                                                    leftPadding: 2
                                                    topPadding: 0
                                                    flat: true
                                                    onDownChanged: down = false

                                                    onClicked: {

                                                    }


                                                    icon {
                                                        source: getIconUrlFromPageType(model.type)

                                                        height: 22
                                                        width: 22
                                                    }


                                                    hoverEnabled: true
                                                    //                                                    ToolTip.delay: 1000
                                                    //                                                    ToolTip.timeout: 5000
                                                    //                                                    ToolTip.visible: hovered
                                                    //                                                    ToolTip.text: qsTr("")




                                                    Item {
                                                        id: treeItemIconIndicatorHat
                                                        anchors.fill: treeItemIconIndicator
                                                        z: 1

                                                        TapHandler{

                                                            onSingleTapped: {
                                                                tapHandler.singleTapped(point)
                                                            }

                                                            onDoubleTapped: {
                                                                tapHandler.doubleTapped(point)
                                                            }

                                                            onGrabChanged: {
                                                                tapHandler.grabChanged(transition, point)
                                                            }


                                                        }

                                                        TapHandler{
                                                            acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                                                            acceptedButtons: Qt.RightButton

                                                            onSingleTapped: {
                                                                rightClickTapHandler.singleTapped(point)
                                                            }

                                                            onDoubleTapped: {
                                                                rightClickTapHandler.doubleTapped(point)
                                                            }

                                                            onGrabChanged: {
                                                                rightClickTapHandler.grabChanged(transition, point)
                                                            }


                                                        }

                                                        TapHandler{
                                                            acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                                                            acceptedButtons: Qt.MiddleButton

                                                            onSingleTapped: {
                                                                middleClickTapHandler.singleTapped(point)
                                                            }

                                                            onDoubleTapped: {
                                                                middleClickTapHandler.doubleTapped(point)
                                                            }

                                                            onGrabChanged: {
                                                                middleClickTapHandler.grabChanged(transition, point)
                                                            }


                                                        }
                                                    }
                                                }

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
                                                        text: model.type === 'PROJECT' ? model.projectName : model.title
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
                                                            swipeDelegate.state = ""


                                                            //fix bug while new lone child
                                                            titleLabel.visible = true
                                                            labelLayout.visible = true
                                                            priv.renaming = false
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
                                                                swipeDelegate.state = ""
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

                                                        placeholderText: qsTr("Enter title")


                                                        onEditingFinished: {

                                                            console.log("editing finished")
                                                            if(model.type === "PROJECT"){ //project item
                                                                model.projectName = text
                                                            }
                                                            else {
                                                                model.title = text
                                                            }

                                                            swipeDelegate.state = ""


                                                            //fix bug while new lone child
                                                            titleLabel.visible = true
                                                            labelLayout.visible = true
                                                            priv.renaming = false
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
                                                                swipeDelegate.state = ""
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
                                                            treeItemId: model.treeItemId
                                                            projectId: model.projectId
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
                                                icon.source: "qrc:///icons/backup/overflow-menu.svg"
                                                flat: true
                                                focusPolicy: Qt.NoFocus

                                                onClicked: {

                                                    if(menu.visible){
                                                        menu.close()
                                                        return
                                                    }


                                                    listView.currentIndex = model.index
                                                    swipeDelegate.forceActiveFocus()
                                                    menu.open()
                                                }

                                                visible: itemHoverHandler.hovered || content.isCurrent
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


                                //            Shortcut {
                                //                sequences: ["Ctrl+Shift+N"]
                                //                onActivated: addBeforeAction.trigger()
                                //                enabled: root.visible
                                //            }
                                SkrMenu {
                                    id: menu
                                    y: menuButton.height
                                    width: 300
                                    z: 101

                                    onOpened: {
                                        // necessary to differenciate between all items
                                        listView.contextMenuItemIndex = model.index
                                    }

                                    onClosed: {

                                    }


                                    SkrMenuItem {
                                        height: !model.isOpenable ? 0 : undefined
                                        visible: model.isOpenable
                                        action: Action {
                                            id: openTreeItemAction
                                            text: qsTr("Open")
                                            //shortcut: "Return"
                                            icon {
                                                source: "qrc:///icons/backup/document-edit.svg"
                                            }

                                            enabled: titleTextField.visible === false && listView.enabled
                                            onTriggered: {
                                                console.log("open treeItem action", model.projectId,
                                                            model.treeItemId)
                                                openDocumentAction.trigger()
                                            }
                                        }
                                    }
                                    SkrMenuItem {
                                        height: !model.isOpenable ||model.treeItemId === -1 ? 0 : undefined
                                        visible:  model.isOpenable

                                        action: Action {
                                            id: openTreeItemInAnotherViewAction
                                            text: qsTr("Open in another view")
                                            //shortcut: "Alt+Return"
                                            icon {
                                                source: "qrc:///icons/backup/tab-new.svg"
                                            }
                                            enabled: titleTextField.visible === false && listView.enabled
                                            onTriggered: {
                                                console.log("open treeItem in another view action", model.projectId,
                                                            model.treeItemId)
                                                openDocumentInAnotherViewAction.trigger()
                                            }
                                        }
                                    }


                                    SkrMenuItem {
                                        height: !model.isOpenable ?  0 : undefined
                                        visible: model.isOpenable

                                        action: Action {
                                            id: openTreeItemInNewWindowAction
                                            text: qsTr("Open in new window")
                                            //shortcut: "Alt+Return"
                                            icon {
                                                source: "qrc:///icons/backup/window-new.svg"
                                            }
                                            enabled: titleTextField.visible === false && listView.enabled
                                            onTriggered: {
                                                console.log("open treeItem in new window action", model.projectId,
                                                            model.treeItemId)
                                                // solve bug where this window's tree disapears
                                                setCurrentTreeItemIdTimer.projectId = model.projectId
                                                setCurrentTreeItemIdTimer.treeItemId = model.treeItemId
                                                setCurrentTreeItemIdTimer.start()
                                                root.openDocumentInNewWindow(model.projectId, model.treeItemId)
                                                //                                                setCurrentTreeItemId(model.projectId, model.treeItemId)


                                            }
                                        }

                                    }

                                    SkrMenuItem {
                                        height: model.treeItemId === 0 ?  undefined : 0
                                        visible: model.treeItemId === 0
                                        enabled: model.projectIsActive === false && listView.enabled &&  model.treeItemId === 0
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
                                            enabled: listView.enabled

                                            onTriggered: {
                                                console.log("rename action", model.projectId,
                                                            model.treeItemId)
                                                swipeDelegate.editName()
                                            }
                                        }
                                    }

                                    SkrMenuItem {
                                        height: !model.isRenamable ? 0: undefined
                                        visible: model.isRenamable
                                        action:
                                            Action {
                                            id: setLabelAction
                                            text: qsTr("Set label")
                                            //shortcut: "F2"
                                            icon {
                                                source: "qrc:///icons/backup/label.svg"
                                            }
                                            enabled: listView.enabled
                                            onTriggered: {
                                                console.log("sel label", model.projectId,
                                                            model.treeItemId)
                                                swipeDelegate.editLabel()
                                            }
                                        }
                                    }

                                    MenuSeparator {
                                        height: !model.isMovable || !model.isCopyable || !model.canAddChildTreeItem ? 0: undefined
                                        visible: model.isMovable || model.isCopyable || model.canAddChildTreeItem
                                    }

                                    SkrMenuItem {
                                        height: !model.isMovable ? 0: undefined
                                        visible: model.isMovable
                                        action:
                                            Action {
                                            id: cutAction
                                            text: qsTr("Cut")
                                            //shortcut: StandardKey.Cut
                                            icon {
                                                source: "qrc:///icons/backup/edit-cut.svg"
                                            }
                                            enabled: listView.enabled

                                            onTriggered: {
                                                console.log("cut action", model.projectId,
                                                            model.treeItemId)
                                                proxyModel.cut(model.projectId, model.treeItemId)
                                            }
                                        }
                                    }

                                    SkrMenuItem {
                                        height: !model.isCopyable ? 0: undefined
                                        visible: model.isCopyable
                                        action:
                                            Action {

                                            id: copyAction
                                            text: qsTr("Copy")
                                            //shortcut: StandardKey.Copy
                                            icon {
                                                source: "qrc:///icons/backup/edit-copy.svg"
                                            }
                                            enabled: listView.enabled

                                            onTriggered: {
                                                console.log("copy action", model.projectId,
                                                            model.treeItemId)
                                                proxyModel.copy(model.projectId, model.treeItemId)
                                            }
                                        }
                                    }


                                    SkrMenuItem {
                                        height: !model.canAddChildTreeItem ? 0: undefined
                                        visible: model.canAddChildTreeItem
                                        action:
                                            Action {

                                            id: pasteAction
                                            text: qsTr("Paste")
                                            //shortcut: StandardKey.Copy
                                            icon {
                                                source: "qrc:///icons/backup/edit-paste.svg"
                                            }
                                            enabled: listView.enabled

                                            onTriggered: {
                                                console.log("paste action", model.projectId,
                                                            model.treeItemId)
                                                proxyModel.paste(model.projectId, model.treeItemId)
                                            }
                                        }
                                    }


                                    MenuSeparator {
                                        height: !(model.canAddSiblingTreeItem || model.canAddChildTreeItem) ? 0: undefined
                                        visible: (model.canAddSiblingTreeItem || model.canAddChildTreeItem)
                                    }

                                    SkrMenuItem {
                                        height: !model.canAddSiblingTreeItem ? 0: undefined
                                        visible: model.canAddSiblingTreeItem
                                        action:
                                            Action {
                                            id: addBeforeAction
                                            text: qsTr("Add before")
                                            //shortcut: "Ctrl+Shift+N"
                                            icon {
                                                source: "qrc:///icons/backup/document-new.svg"
                                            }
                                            enabled: listView.enabled
                                            onTriggered: {
                                                console.log("add before action", model.projectId,
                                                            model.treeItemId)




                                                var visualIndex = listView.currentIndex


                                                newItemPopup.projectId = model.projectId
                                                newItemPopup.treeItemId = model.treeItemId
                                                newItemPopup.visualIndex = visualIndex
                                                newItemPopup.createFunction = afterNewItemTypeIsChosen
                                                newItemPopup.open()
                                            }

                                            function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType){
                                                newItemPopup.close()
                                                proxyModel.addItemAbove(projectId, treeItemId, pageType)
                                                listView.itemAtIndex(visualIndex).editName()

                                            }


                                        }
                                    }

                                    SkrMenuItem {
                                        height: !model.canAddSiblingTreeItem ? 0: undefined
                                        visible: model.canAddSiblingTreeItem
                                        action:
                                            Action {
                                            id: addAfterAction
                                            text: qsTr("Add after")
                                            //shortcut: "Ctrl+N"
                                            icon {
                                                source: "qrc:///icons/backup/document-new.svg"
                                            }
                                            enabled: listView.enabled
                                            onTriggered: {
                                                console.log("add after action", model.projectId,
                                                            model.treeItemId)

                                                var visualIndex = listView.currentIndex + 1

                                                newItemPopup.projectId = model.projectId
                                                newItemPopup.treeItemId = model.treeItemId
                                                newItemPopup.visualIndex = visualIndex
                                                newItemPopup.createFunction = afterNewItemTypeIsChosen
                                                newItemPopup.open()



                                                //editNameWithIndexTimer.treeItemIndexToEdit = visualIndex

                                            }


                                            function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType){
                                                newItemPopup.close()
                                                proxyModel.addItemBelow(projectId, treeItemId, pageType)
                                                listView.itemAtIndex(visualIndex).editName()
                                            }


                                        }

                                    }

                                    SkrMenuItem {
                                        height: !model.canAddChildTreeItem ? 0: undefined
                                        visible: model.canAddChildTreeItem
                                        action:
                                            Action {
                                            id: addChildAction
                                            text: qsTr("Add a sub-item")
                                            //shortcut: "Ctrl+N"
                                            icon {
                                                source: "qrc:///icons/backup/document-new.svg"
                                            }
                                            enabled: listView.enabled
                                            onTriggered: {
                                                console.log("add child action", model.projectId,
                                                            model.treeItemId)


                                                //save current ids:
                                                listView.currentIndex = model.index
                                                navigationListStackView.currentItem.treeItemId = model.treeItemId


                                                newItemPopup.projectId = model.projectId
                                                newItemPopup.treeItemId = model.treeItemId
                                                newItemPopup.visualIndex = 0
                                                newItemPopup.createFunction = afterNewItemTypeIsChosen
                                                newItemPopup.open()


                                            }

                                            function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType){

                                                // push new view

                                                var newItem = navigationListStackView.push(stackViewComponent, {"projectId": projectId, "parentId":  treeItemId} )
                                                newItem.setCurrent()

                                                addItemAtCurrentParent(pageType)
                                            }

                                        }

                                    }
                                    MenuSeparator {
                                        height: !model.isMovable ? 0: undefined
                                        visible: model.isMovable
                                    }

                                    SkrMenuItem {
                                        height: !model.isMovable ? 0: undefined
                                        visible: model.isMovable
                                        action:
                                            Action {
                                            id: moveUpAction
                                            text: qsTr("Move up")
                                            //shortcut: "Ctrl+Up"
                                            icon {
                                                source: "qrc:///icons/backup/object-order-raise.svg"
                                            }
                                            enabled: listView.enabled && model.index !== 0
                                            onTriggered: {
                                                console.log("move up action", model.projectId,
                                                            model.treeItemId)

                                                if(temporarilyDisableMove){
                                                    return
                                                }
                                                temporarilyDisableMove = true
                                                temporarilyDisableMoveTimer.start()

                                                proxyModel.moveUp(model.projectId, model.treeItemId,
                                                                  model.index)


                                            }
                                        }
                                    }

                                    SkrMenuItem {
                                        height: !model.isMovable ? 0: undefined
                                        visible: model.isMovable
                                        action:
                                            Action {
                                            id: moveDownAction
                                            text: qsTr("Move down")
                                            //shortcut: "Ctrl+Down"
                                            icon {
                                                source: "qrc:///icons/backup/object-order-lower.svg"
                                            }
                                            enabled: model.index !== visualModel.items.count - 1  && listView.enabled

                                            onTriggered: {
                                                console.log("move down action", model.projectId,
                                                            model.treeItemId)

                                                if(temporarilyDisableMove){
                                                    return
                                                }
                                                temporarilyDisableMove = true
                                                temporarilyDisableMoveTimer.start()

                                                proxyModel.moveDown(model.projectId, model.treeItemId,
                                                                    model.index)
                                            }
                                        }

                                    }

                                    MenuSeparator {
                                        height: !model.isTrashable ? 0: undefined
                                        visible: model.isTrashable
                                    }

                                    SkrMenuItem {
                                        height: !model.isTrashable ? 0: undefined
                                        visible: model.isTrashable
                                        action:
                                            Action {
                                            id: sendToTrashAction
                                            text: qsTr("Send to trash")
                                            //shortcut: "Del"
                                            icon {
                                                source: "qrc:///icons/backup/edit-delete.svg"
                                            }
                                            enabled: listView.enabled && model.indent !== -1
                                            onTriggered: {
                                                console.log("sent to trash action", model.projectId,
                                                            model.treeItemId)

                                                swipeDelegate.swipe.close()
                                                removeTreeItemAnimation.start()
                                                proxyModel.trashItemWithChildren(model.projectId, model.treeItemId)
                                            }
                                        }
                                    }
                                }

                                //----------------------------------------------------------


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

                            }
                            property int animationDuration: 150

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
                                        target: swipeDelegate
                                        anchors.left: undefined
                                        anchors.right: undefined

                                    }
                                }
                            ]

                            SequentialAnimation {
                                id: removeTreeItemAnimation
                                PropertyAction {
                                    property: "ListView.delayRemove"
                                    value: true
                                }

                                ScriptAction {
                                    script: swipeDelegate.state = "unset_anchors"
                                }

                                NumberAnimation {
                                    target: swipeDelegate
                                    property: "x"
                                    to: listView.width
                                    duration: 250
                                    easing.type: Easing.InBack
                                }

                                PropertyAction {
                                    property: "ListView.delayRemove"
                                    value: false
                                }
                                ScriptAction {
                                    script: proxyModel.invalidate()
                                }

                            }
                            SequentialAnimation {
                                id: addTreeItemAtEndAnimation

                                PropertyAction {
                                    target: swipeDelegate
                                    property: "height"
                                    value: 0
                                }
                                NumberAnimation {
                                    target: swipeDelegate
                                    property: "height"
                                    to: swipeDelegate.height
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


    Timer{
        id: setCurrentTreeItemIdTimer
        property int projectId
        property int treeItemId
        interval: 30
        onTriggered: {

            setCurrentTreeItemId(projectId, treeItemId)
        }
    }

    //-------------------------------------------------------------------------------------
    //---------side Navigation Popup---------------------------------------------------------
    //-------------------------------------------------------------------------------------

    Component {
        id: sideNavigationPopupComponent
        SkrPopup {
            id: sideNavigationPopup

            //enableBehavior: false

            property int projectId: -2
            property int parentId: -2
            property int popupId: sidePopupListModel.count - 1
            property int headerHeight: 0
            enabled: false


            width: 200
            z: 100 - popupId
            modal: false
            closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutsideParent
            padding: 0
            // start values:
            x: navigationListStackView.width + sideNavigationPopup.width * popupId
            y: sidePopupListModel.get(popupId - 1) === undefined ? 0 : sidePopupListModel.get(popupId - 1).y


            Binding{
                target: sideNavigationPopup
                property: "height"
                value: sideNavigationLoader.item.listView.contentHeight > navigationListStackView.height ?
                           navigationListStackView.height : sideNavigationLoader.item.listView.contentHeight
            }


            Binding{
                target: sideNavigationPopup
                property: "headerHeight"
                value: sideNavigationLoader.item.listView.contentHeight -
                       sideNavigationLoader.item.listView.count * 50
            }

            //-----------------------------------

            Loader {
                id: sideNavigationLoader
                anchors.fill: parent
                asynchronous: true
                sourceComponent: stackViewComponent
                visible: status === Loader.Ready


                onStatusChanged: {
                    if(status === Loader.Ready){
                        //sideNavigationLoader.item.hoverEnabled  = false
                        sideNavigationLoader.item.projectId = sideNavigationPopup.projectId
                        sideNavigationLoader.item.parentId = sideNavigationPopup.parentId
                        sideNavigationLoader.item.popupId = sideNavigationPopup.popupId
                        sideNavigationLoader.item.init()


                        sideNavigationPopup.enabled = true
                        //enableHoverTimer.start()

                    }
                }
            }
            //            Timer {
            //                id: enableHoverTimer
            //                interval: 50
            //                onTriggered: sideNavigationLoader.item.hoverEnabled = true
            //            }


        }
    }


    ListModel {
        id: sidePopupListModel
    }


    Repeater {

        model: sidePopupListModel

        Loader {
            id: sidePopupLoader
            sourceComponent: sideNavigationPopupComponent
            visible: status === Loader.Ready
            asynchronous: true

            Binding{
                when: sidePopupLoader.status === Loader.Ready
                target: sidePopupLoader.item
                property: "popupId"
                value: model.popupId
            }

            Binding{
                when: sidePopupLoader.status === Loader.Ready
                target: sidePopupLoader.item
                property: "projectId"
                value: model.projectId
            }

            Binding{
                when: sidePopupLoader.status === Loader.Ready
                target: sidePopupLoader.item
                property: "parentId"
                value: model.parentId
            }

            Binding{
                when: sidePopupLoader.status === Loader.Ready
                target: sidePopupLoader.item
                property: "y"
                value:   mapFromItem(model.listView, 0, model.parentItem.y).y - sidePopupLoader.item.headerHeight
            }
            Binding{
                when: sidePopupLoader.status === Loader.Ready
                target: model
                property: "y"
                value:   sidePopupLoader.item.y
            }
            Binding{
                when: sidePopupLoader.status === Loader.Ready
                target: sidePopupLoader.item
                property: "x"
                value: navigationListStackView.width + sidePopupLoader.item.width * (model.index)
            }
            onStatusChanged: {
                if(status === Loader.Ready){
                    sidePopupLoader.item.visible = true
                }
            }
        }
    }

    Timer{
        id: addSideNavigationListPopupTimer
        property int projectId: -2
        property int parentId: -2
        property int popupId: -2
        property var parentItem
        property var listView
        interval: 50
        onTriggered: {
            //closeSideNavigationListPopupTimer.stop()

            //close next popups
            for(var i = sidePopupListModel.count -1 ; i > popupId ; i--){
                sidePopupListModel.remove(i)
            }

            // add
            sidePopupListModel.append({"popupId": popupId + 1 , "projectId": projectId, "parentId": parentId,
                                          "hovered": false, "parentItem": parentItem,
                                          "listView": listView, "y": 0})
            //console.log("sidePopupListModel.count", sidePopupListModel.count)

            //            if(sideNavigationPopup.visible){
            //                sideNavigationPopup.close()
            //            }


            //            sideNavigationPopup.open()

        }

    }


    Timer{
        id: closeSideNavigationListPopupTimer
        property int popupId: -2
        interval: 50
        onTriggered: {

            var hoveredPopupId = -1
            //            for(var i = 0 ; i < sidePopupListModel.count ; i++){
            //                if(sidePopupListModel.get(i).hovered){
            //                    hoveredPopupId = i
            //                }
            //            }



            for(var k = sidePopupListModel.count -1 ; k > popupId ; k--){
                sidePopupListModel.remove(k)
            }
            if(popupId === -1){
                sidePopupListModel.clear()
            }
        }

    }

    function determineIfSideNavigationPopupsHaveToClose(){
        if(itemHoverHandler.hovered){
            closeSideNavigationListPopupTimer.stop()

        }
        else{
            closeSideNavigationListPopupTimer.popupId = stackViewBaseItem.popupId
            closeSideNavigationListPopupTimer.start()
        }


    }



    //-------------------------------------------------------------------------------------
    //-------------------------------------------------------------------------------------
    //-------------------------------------------------------------------------------------


    NewItemPopup {
        id: newItemPopup
    }



    //-------------------------------------------------------------------------------------
    //-------------------------------------------------------------------------------------
    //-------------------------------------------------------------------------------------



    onActiveFocusChanged: {
        if (activeFocus) {
            if(!priv.renaming){
                navigationListStackView.currentItem.forceActiveFocus()
            }
        }
    }




}
