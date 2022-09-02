import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts
import QtQml.Models
import QtQuick.Controls.Material
import eu.skribisto.searchtreelistproxymodel 1.0
import Skribisto
import SkrControls



Item {
    id: root
    property alias listView: listView
    property alias visualModel: visualModel

    SKRSearchTreeListProxyModel {
        id: navigationProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
        navigateByBranchesEnabled: true
    }

    property int currentIndex: listView.currentIndex
    property int parentId: -2
    property int projectId: -2
    property int treeItemId: -2
    property int popupId: -1
    property bool hoverEnabled: true

    // used to remember the source when moving an item
    property int moveSourceInt: -2
    property int moveSourceTreeItemId: -2
    property int moveSourceProjectId: -2
    signal setCurrentTreeItemParentIdCalled(int currentProjectId, int currentParentId)
    signal setCurrentTreeItemIdCalled(int currentProjectId, int currentTreeItemId)
    signal openDocument(int projectId, int treeItemId)
    signal openDocumentInAnotherView(int projectId, int treeItemId)
    signal openDocumentInNewWindow(int projectId, int treeItemId)

    focus: false

    function init() {
        p_section.parentTitle = qsTr("Projects")
        listView.section.delegate = sectionHeading


        if (projectId > 0 && treeItemId === 0 && parentId === -2) {
            //list of project items

            navigationProxyModel.setCurrentTreeItemId(projectId,
                                                      treeItemId)
        }
        else if (treeItemId === -2 && parentId === -2) {
            //list of project items
            navigationProxyModel.setParentFilter(projectId,
                                                 parentId)
        }
        else if (projectId > 0 && parentId >= 0) {
            navigationProxyModel.setParentFilter(projectId,
                                                 parentId)
            listView.currentIndex = 0

        }
        else if (projectId > 0 && treeItemId >= 0){
            navigationProxyModel.setCurrentTreeItemId(
                        projectId, treeItemId)
            parentId = navigationProxyModel.parentIdFilter

        }
        determineSectionTitle()
    }

    Component.onDestruction: {
        delete navigationProxyModel
    }

    function setCurrent() {
        priv.currentProjectId = projectId
        priv.currentParentId = parentId

        listView.positionViewAtIndex(currentIndex, ListView.Contain)
    }

    function setCurrentTreeItemId(projectId, treeItemId) {
        navigationProxyModel.setCurrentTreeItemId(projectId, treeItemId)
    }


    onActiveFocusChanged: {
        if (activeFocus) {
            listView.forceActiveFocus()
        }
    }

    function checkAll(){
        navigationProxyModel.checkAll()
    }

    function checkNone(){
        navigationProxyModel.checkNone()
    }


    NewItemPopup {
        id: newItemPopup
    }

    function determineSectionTitle() {

        var projectId = root.projectId
        var parentId = root.parentId
        if (parentId === 0 && projectId !== -2) {
            var projectTitle = skrData.projectHub().getProjectName(
                        projectId)

            p_section.parentTitle = projectTitle
            listView.section.delegate = sectionHeading
        } else if (parentId !== -2 && projectId !== -2) {
            var parentTitle = navigationProxyModel.getItemName(projectId,
                                                               parentId)

            //console.log("onCurrentParentChanged")
            p_section.parentTitle = parentTitle
            listView.section.delegate = sectionHeading
        } else if (parentId === -2) {

            // show "projects" section
            p_section.parentTitle = qsTr("Projects")
            listView.section.delegate = sectionHeading
        } // clear :
        else if (parentId === -2 && root.currentProjectId === -2) {
            listView.section.delegate = null
        }
    }

    Item {
        id: topDraggingMover
        anchors.top: scrollView.top
        anchors.right: scrollView.right
        anchors.left: scrollView.left
        height: 30
        z: 1

        visible: priv.dragging

        HoverHandler {
            onHoveredChanged: {
                if (hovered) {
                    topDraggingMoverTimer.start()
                } else {
                    topDraggingMoverTimer.stop()
                }
            }
        }

        Timer {
            id: topDraggingMoverTimer
            repeat: true
            interval: 10
            onTriggered: {
                if (listView.atYBeginning) {
                    listView.contentY = 0
                } else {
                    listView.contentY = listView.contentY - 2
                }
            }
        }
    }

    Item {
        id: bottomDraggingMover
        anchors.bottom: scrollView.bottom
        anchors.right: scrollView.right
        anchors.left: scrollView.left
        height: 30
        z: 1

        visible: priv.dragging

        HoverHandler {
            onHoveredChanged: {
                if (hovered) {
                    bottomDraggingMoverTimer.start()
                } else {
                    bottomDraggingMoverTimer.stop()
                }
            }
        }

        Timer {
            id: bottomDraggingMoverTimer
            repeat: true
            interval: 10
            onTriggered: {
                if (listView.atYEnd) {
                    listView.positionViewAtEnd()
                } else {
                    listView.contentY = listView.contentY + 2
                }
            }
        }
    }

    DropArea {
        id: focusZone
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        height: listView.height - listView.contentHeight
                > 0 ? listView.height - listView.contentHeight : 0
        z: 1


        keys: ["application/skribisto-tree-item"]
        onEntered: {
            if(!model.canAddChildTreeItem){
                drag.accepted = false
                return
            }

            if(skrData.treeHub().getAllAncestors(projectId, parentId).includes(drag.source.treeItemId)){
                drag.accepted = false
                return
            }

        }
        onExited: {

        }
        onDropped: function(drop){
            if (drop.proposedAction === Qt.MoveAction) {

                listView.interactive = true
                priv.dragging = false
                console.log("onDropped", drag.source.projectId, drag.source.treeItemId, projectId, parentId)
                focusZoneMoveTreeItemAsChildOfTimer.sourceProjectId = drag.source.projectId
                focusZoneMoveTreeItemAsChildOfTimer.sourceTreeItemId = drag.source.treeItemId
                focusZoneMoveTreeItemAsChildOfTimer.start()
                //skrData.treeHub().moveTreeItemAsChildOf(drag.source.projectId, drag.source.treeItemId, projectId, parentId)
            }
        }
        Timer{
            id: focusZoneMoveTreeItemAsChildOfTimer

            property int sourceProjectId: -1
            property int sourceTreeItemId: -1
            interval: 1000
            onTriggered: {
                skrData.treeHub().moveTreeItemAsChildOf(sourceProjectId, sourceTreeItemId, projectId, parentId)
            }
        }




        TapHandler {
            onSingleTapped: function(eventPoint) {
                if(priv.renaming){
                    return
                }


                listView.forceActiveFocus()
                console.log("focusZone", "forceActiveFocus")

                var index = listView.currentIndex
                var item = listView.itemAtIndex(index)
                if (item) {
                    item.forceActiveFocus()
                } else {
                    listView.forceActiveFocus()
                }

                eventPoint.accepted = false
            }
            grabPermissions: PointerHandler.ApprovesTakeOverByAnything
        }



    }

    ScrollView {
        id: scrollView
        clip: true
        anchors.fill: parent
        focusPolicy: Qt.StrongFocus
        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
        ScrollBar.vertical.policy: ScrollBar.AsNeeded //scrollBarVerticalPolicy

        ListView {
            id: listView
            smooth: true
            boundsBehavior: Flickable.StopAtBounds
            spacing: 1

            Accessible.name: qsTr("Navigation list")
            Accessible.role: Accessible.List

            DelegateModel {
                id: visualModel
                delegate: listItemComponent
                model: navigationProxyModel
            }
            model: visualModel

            // scrollBar interactivity :
            onContentHeightChanged: {
                //fix scrollbar visible at start
                if (scrollView.height === 0) {
                    scrollBarVerticalPolicy = ScrollBar.AlwaysOff
                    return
                }

                if (listView.contentHeight > scrollView.height) {
                    scrollBarVerticalPolicy = ScrollBar.AlwaysOn
                } else {
                    scrollBarVerticalPolicy = ScrollBar.AlwaysOff
                }
            }

            //-----------------------------------------------------------------------------
            property int popupId: root.popupId
            property alias proxyModel: navigationProxyModel



            Binding {
                target: listView
                property: "currentIndex"
                value: navigationProxyModel.forcedCurrentIndex
            }

            //----------------------------------------------------------------------------

            // move :
            addDisplaced: Transition {
                enabled: SkrSettings.ePaperSettings.animationEnabled
                NumberAnimation {
                    properties: "x,y"
                    duration: 250
                }
            }

            removeDisplaced: Transition {
                enabled: SkrSettings.ePaperSettings.animationEnabled
                SequentialAnimation {
                    PauseAnimation {
                        duration: 250
                    }
                    NumberAnimation {
                        properties: "x,y"
                        duration: 250
                    }
                }
            }
            displaced: Transition {
                enabled: SkrSettings.ePaperSettings.animationEnabled
                NumberAnimation {
                    properties: "x,y"
                    duration: 250
                }
            }

            moveDisplaced: Transition {
                enabled: SkrSettings.ePaperSettings.animationEnabled
                NumberAnimation {
                    properties: "x,y"
                    duration: 100
                }
            }
            QtObject {
                id: p_section
                property string parentTitle: ""
            }

            //----------------------------------------------------------------------------


            function enableSection(value){
                if(value){
                    determineSectionTitle()
                }
                else{
                    listView.section.delegate = null
                }
            }

            section.property: "indent"
            section.criteria: ViewSection.FullString
            section.labelPositioning: ViewSection.CurrentLabelAtStart
                                      | ViewSection.InlineLabels
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
                        activeFocusOnTab: false
                        text: p_section.parentTitle
                        font.bold: true
                        horizontalAlignment: Qt.AlignHCenter
                        color: SkrTheme.buttonForeground
                    }
                }
            }

            //----------------------------------------------------------------------
            //---  listview keys ------------------------------------------
            //----------------------------------------------------------------------
            Keys.onShortcutOverride: function(event)  {
                if ((event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_N) {
                    event.accepted = true
                }
                if ((event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_V) {
                    event.accepted = true
                }
            }
            Keys.onPressed: function(event) {
                //                        if (event.key === Qt.Key_Up) {
                //                            listView.currentItem.forceActiveFocus()
                //                            event.accepted = false
                //                                                    }
                if (event.key === Qt.Key_Backspace
                        || event.key === Qt.Key_Left) {
                    console.log("Backspace / Left key pressed")
                    goUpAction.trigger()
                    event.accepted = true
                }
                // paste
                if ((event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_V) {
                    skrData.treeHub().paste(currentProjectId,
                                            currentParentId)
                    event.accepted = true
                }

                // add
                if ((event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_N) {

                    newItemPopup.projectId = currentProjectId
                    newItemPopup.treeItemId = currentParentId
                    newItemPopup.visualIndex = 0
                    newItemPopup.createFunction = afterNewItemTypeIsChosen
                    newItemPopup.open()

                    event.accepted = true
                }
            }

            function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType, quantity) {

                for(var i = 0; i < quantity ; i++){
                    addItemAtCurrentParent(pageType)
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
                    property alias content: content
                    property alias topDropIndicator: topDropIndicator
                    property alias middleDropIndicator: middleDropIndicator
                    property alias bottomDropIndicator: bottomDropIndicator
                    property alias checkState: selectionCheckBox.checkState
                    focus: true

                    Accessible.name: labelLabel.text.length
                                     === 0 ? titleLabel.text + (model.hasChildren ? " " + qsTr("is a folder") : "") : titleLabel.text + " " + qsTr(
                                                 "label:") + " " + labelLabel.text
                                             + (model.hasChildren ? " " + qsTr(
                                                                        "has child") : "")
                    Accessible.role: Accessible.ListItem
                    Accessible.description: qsTr("navigation item")

                    anchors {
                        left: Qt.isQtObject(
                                  parent) ? parent.left : undefined
                        right: Qt.isQtObject(
                                   parent) ? parent.right : undefined
                        rightMargin: 5
                    }

                    height: 40 * SkrSettings.interfaceSettings.zoom

                    padding: 0
                    topPadding: 0
                    bottomPadding: 0
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
                        if (listView.currentIndex === model.index
                                && model.index !== -1 && activeFocus) {

                            //                            root.treeItemId = model.treeItemId
                        }
                    }

                    function editName() {
                        priv.renaming = true
                        titleTextFieldForceActiveFocusTimer.start()
                    }

                    Timer {
                        id: titleTextFieldForceActiveFocusTimer
                        repeat: false
                        interval: 100
                        onTriggered: {
                            state = "edit_name"
                            titleTextField.forceActiveFocus()
                            titleTextField.selectAll()
                        }
                    }

                    function editLabel() {
                        priv.renaming = true
                        labelTextFieldForceActiveFocusTimer.start()
                    }

                    Timer {
                        id: labelTextFieldForceActiveFocusTimer
                        repeat: false
                        interval: 100
                        onTriggered: {
                            state = "edit_label"
                            labelTextField.forceActiveFocus()
                            labelTextField.selectAll()
                        }
                    }

                    function configureMenu(){
                        menu.index = model.index
                        menu.treeItemId = model.treeItemId
                        menu.projectId = model.projectId
                        menu.type = model.type
                        menu.isOpenable = model.isOpenable
                        menu.projectIsActive = model.projectIsActive
                        menu.canAddChildTreeItem = model.canAddChildTreeItem
                        menu.canAddSiblingTreeItem = model.canAddSiblingTreeItem
                        menu.isCopyable = model.isCopyable
                        menu.isMovable = model.isMovable
                        menu.isRenamable = model.isRenamable
                        menu.isTrashable = model.isTrashable
                    }

                    Keys.priority: Keys.AfterItem

                    Keys.onShortcutOverride: function(event)  {
                        if (event.key === Qt.Key_F2) {
                            event.accepted = true
                        }
                        if ((event.modifiers & Qt.ControlModifier)
                                && event.key === Qt.Key_N) {
                            event.accepted = true
                        }
                        if ((event.modifiers & Qt.ControlModifier)
                                && (event.modifiers & Qt.ShiftModifier)
                                && event.key === Qt.Key_N) {
                            event.accepted = true
                        }
                        if ((event.modifiers & Qt.ControlModifier)
                                && event.key === Qt.Key_C) {
                            event.accepted = true
                        }
                        if (model.isMovable
                                && (event.modifiers & Qt.ControlModifier)
                                && event.key === Qt.Key_X) {
                            event.accepted = true
                        }
                        if ((event.modifiers & Qt.ControlModifier)
                                && event.key === Qt.Key_V) {
                            event.accepted = true
                        }
                        if (event.key === Qt.Key_Escape
                                && (swipeDelegate.state == "edit_name"
                                    || swipeDelegate.state == "edit_label")) {
                            event.accepted = true
                        }
                    }

                    Keys.onPressed: function(event) {
                        if (event.key === Qt.Key_Right) {
                            console.log("Right key pressed")
                            goToChildAction.trigger()
                            event.accepted = true
                        }
                        if (event.key === Qt.Key_Backspace
                                || event.key === Qt.Key_Left) {
                            console.log("Backspace / Left key pressed")
                            goUpAction.trigger()
                            event.accepted = true
                        }
                        if (model.isOpenable
                                && (event.modifiers & Qt.AltModifier)
                                && event.key === Qt.Key_Return) {
                            console.log("Alt Return key pressed")
                            configureMenu()
                            openTreeItemInAnotherViewAction.trigger()
                            event.accepted = true
                        }
                        if (model.isOpenable
                                && event.key === Qt.Key_Return
                                && swipeDelegate.state !== "edit_name"
                                && swipeDelegate.state !== "edit_label") {
                            console.log("Return key pressed")
                            swipeDelegate.configureMenu()
                            openTreeItemAction.trigger()
                            event.accepted = true
                        }

                        // rename
                        if (model.isRenamable && event.key === Qt.Key_F2
                                && swipeDelegate.state !== "edit_name"
                                && swipeDelegate.state !== "edit_label") {
                            swipeDelegate.configureMenu()
                            renameAction.trigger()
                            event.accepted = true
                        }

                        // cut
                        if (model.isMovable
                                && (event.modifiers & Qt.ControlModifier)
                                && event.key === Qt.Key_X
                                && swipeDelegate.state !== "edit_name"
                                && swipeDelegate.state !== "edit_label") {
                            swipeDelegate.configureMenu()
                            cutAction.trigger()
                            event.accepted = true
                        }

                        // copy
                        if (model.isCopyable
                                && (event.modifiers & Qt.ControlModifier)
                                && event.key === Qt.Key_C
                                && swipeDelegate.state !== "edit_name"
                                && swipeDelegate.state !== "edit_label") {
                            swipeDelegate.configureMenu()
                            copyAction.trigger()
                            event.accepted = true
                        }

                        // paste
                        if (model.canAddChildTreeItem
                                && (event.modifiers & Qt.ControlModifier)
                                && event.key === Qt.Key_V
                                && swipeDelegate.state !== "edit_name"
                                && swipeDelegate.state !== "edit_label") {
                            swipeDelegate.configureMenu()
                            pasteAction.trigger()
                            event.accepted = true
                        }

                        // add before
                        if (model.canAddSiblingTreeItem
                                && (event.modifiers & Qt.ControlModifier)
                                && (event.modifiers & Qt.ShiftModifier)
                                && event.key === Qt.Key_N
                                && swipeDelegate.state !== "edit_name"
                                && swipeDelegate.state !== "edit_label") {
                            addBeforeAction.trigger()
                            event.accepted = true
                        }

                        // add after
                        if (model.canAddSiblingTreeItem
                                && (event.modifiers & Qt.ControlModifier)
                                && event.key === Qt.Key_N
                                && swipeDelegate.state !== "edit_name"
                                && swipeDelegate.state !== "edit_label") {
                            addAfterAction.trigger()
                            event.accepted = true
                        }

                        // add child
                        if (model.canAddChildTreeItem
                                && (event.modifiers & Qt.ControlModifier)
                                && event.key === Qt.Key_Space
                                && swipeDelegate.state !== "edit_name"
                                && swipeDelegate.state !== "edit_label") {
                            addChildAction.trigger()
                            event.accepted = true
                        }

                        // move up
                        if (model.isMovable
                                && (event.modifiers & Qt.ControlModifier)
                                && event.key === Qt.Key_Up
                                && swipeDelegate.state !== "edit_name"
                                && swipeDelegate.state !== "edit_label") {
                            configureMenu()
                            moveUpAction.trigger()
                            event.accepted = true
                        }

                        // move down
                        if (model.isMovable
                                && (event.modifiers & Qt.ControlModifier)
                                && event.key === Qt.Key_Down
                                && swipeDelegate.state !== "edit_name"
                                && swipeDelegate.state !== "edit_label") {
                            swipeDelegate.configureMenu()
                            moveDownAction.trigger()
                            event.accepted = true
                        }
                        // go up
                        if (event.key === Qt.Key_Up
                                && swipeDelegate.state !== "edit_name"
                                && swipeDelegate.state !== "edit_label") {
                            swipeDelegate.ListView.view.decrementCurrentIndex()
                            event.accepted = true
                        }

                        // go down
                        if (event.key === Qt.Key_Down
                                && swipeDelegate.state !== "edit_name"
                                && swipeDelegate.state !== "edit_label") {
                            swipeDelegate.ListView.view.incrementCurrentIndex()
                            event.accepted = true
                        }

                        // send to trash
                        if (model.isTrashable
                                && event.key === Qt.Key_Delete
                                && swipeDelegate.state !== "edit_name"
                                && swipeDelegate.state !== "edit_label") {
                            swipeDelegate.configureMenu()
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
                    Timer {
                        id: closeSwipeTimer
                        interval: 3000
                        onTriggered: {
                            swipeDelegate.swipe.close()
                        }
                    }

                    HoverHandler {
                        id: itemHoverHandler
                        acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad | PointerDevice.Stylus
                        enabled: root.hoverEnabled

                        //grabPermissions:  PointerHandler.CanTakeOverFromItems | PointerHandler.CanTakeOverFromHandlersOfSameType | PointerHandler.ApprovesTakeOverByAnything
                        onHoveredChanged: {
                            //console.log("item hovered", itemHoverHandler.hovered)
                            if (priv.dragging || priv.selecting) {
                                hoveringTimer.stop()
                                closeSideNavigationListPopupTimer.popupId
                                        = root.popupId + 1
                                closeSideNavigationListPopupTimer.start(
                                            )
                            } else if (hovered && model.hasChildren) {

                                if (!rootWindow.compactMode) {

                                    hoveringTimer.start()
                                }
                            } else if (hovered && !model.hasChildren) {
                                if (!addSideNavigationListPopupTimer.running) {
                                    hoveringTimer.stop()
                                    addSideNavigationListPopupTimer.stop()
                                    closeSideNavigationListPopupTimer.popupId
                                            = root.popupId + 1
                                    closeSideNavigationListPopupTimer.start()
                                }
                            } else if (!model.hasChildren) {
                                if (!addSideNavigationListPopupTimer.running) {
                                    hoveringTimer.stop()
                                    closeSideNavigationListPopupTimer.popupId
                                            = root.popupId + 1
                                    closeSideNavigationListPopupTimer.start()
                                }
                            } else {
                                hoveringTimer.stop()
                                closeSideNavigationListPopupTimer.stop()
                                //addSideNavigationListPopupTimer.stop()
                            }
                        }

                        //                                }
                    }

                    Timer {
                        id: hoveringTimer
                        interval: 50
                        onTriggered: {

                            addSideNavigationListPopupTimer.projectId = model.projectId
                            addSideNavigationListPopupTimer.parentId = model.treeItemId
                            addSideNavigationListPopupTimer.popupId
                                    = root.popupId
                            addSideNavigationListPopupTimer.parentItem = swipeDelegate
                            addSideNavigationListPopupTimer.listView = listView

                            //                                            console.log("popupId", root.popupId)
                            addSideNavigationListPopupTimer.start()
                        }
                    }

                    contentItem: Item {
                        id: dropAreaHolder

                        property int dropAreaSizeDivider: 4

                        Rectangle{
                            id: topDropIndicator
                            z: 1


                            anchors.top: parent.top
                            anchors.left: parent.left
                            anchors.right: parent.right
                            height: parent.height / dropAreaHolder.dropAreaSizeDivider
                            visible : false
                            onVisibleChanged: {
                                var previousItem = swipeDelegate.ListView.view.itemAtIndex(model.index -1)
                                if(previousItem){
                                    previousItem.bottomDropIndicator.visible = visible
                                }
                            }

                            gradient: Gradient {
                                orientation: Gradient.Vertical
                                GradientStop {
                                    position: 0.00;
                                    color: SkrTheme.accent;
                                }
                                GradientStop {
                                    position: 1.00;
                                    color: "transparent";
                                }
                            }

                        }

                        DropArea {
                            id: topDropArea
                            anchors.top: parent.top
                            anchors.left: parent.left
                            anchors.right: parent.right
                            height: parent.height / dropAreaHolder.dropAreaSizeDivider


                            keys:["application/skribisto-tree-item"]
                            onEntered: function(drag) {

                                if(!model.canAddSiblingTreeItem){
                                    drag.accepted = false
                                    return
                                }

                                if(drag.source.treeItemId === model.treeItemId){
                                    drag.accepted = false
                                    return
                                }

                                if(skrData.treeHub().getAllAncestors(projectId, parentId).includes(drag.source.treeItemId)){
                                    drag.accepted = false
                                    return
                                }


                                topOnEnteredTimer.start()
                            }

                            Timer{
                                id: topOnEnteredTimer
                                property bool visible: false
                                interval: 10
                                onTriggered: {


                                    topDropIndicator.visible = true

                                }
                            }

                            onExited: {
                                topOnExitedTimer.start()

                            }

                            Timer{
                                id: topOnExitedTimer
                                property bool visible: false
                                interval: 10
                                onTriggered: {
                                    topDropIndicator.visible = false

                                }
                            }

                            onDropped: function(drop){
                                topDropIndicator.visible = false

                                topMoveTreeItemTimer.sourceTreeItemId = drag.source.treeItemId
                                topMoveTreeItemTimer.targetTreeItemId = model.treeItemId
                                topMoveTreeItemTimer.projectId = model.projectId
                                topMoveTreeItemTimer.start()

                            }

                            Timer{
                                id: topMoveTreeItemTimer
                                property int sourceTreeItemId: -1
                                property int targetTreeItemId: -1
                                property int projectId: -1
                                interval: 600
                                onTriggered: {
                                    skrData.treeHub().moveTreeItem(projectId, sourceTreeItemId, targetTreeItemId, false)

                                }
                            }
                        }


                        DropArea {
                            id: middleDropArea
                            anchors.fill: parent
                            anchors.topMargin: topDropArea.height
                            anchors.bottomMargin: bottomDropArea.height

                            keys: ["application/skribisto-tree-item"]


                            onEntered: function(drag) {
                                console.log("entered", content.visualIndex)

                                if(!model.canAddChildTreeItem){
                                    middleDropArea.visible = false
                                    dropAreaHolder.dropAreaSizeDivider = 2
                                    drag.accepted = false
                                    return
                                }

                                if(drag.source.treeItemId === model.treeItemId){
                                    drag.accepted = false
                                    return
                                }


                                if(skrData.treeHub().getAllAncestors(projectId, parentId).includes(drag.source.treeItemId)){
                                    drag.accepted = false
                                    return
                                }

                                middleDropIndicator.visible = true

                            }
                            onExited: {
                                middleDropIndicator.visible = false

                            }

                            onDropped: function(drop) {
                                middleDropIndicator.visible = false



                                if (drop.proposedAction === Qt.MoveAction) {

                                    middleMoveTreeItemTimer.sourceProjectId = drag.source.projectId
                                    middleMoveTreeItemTimer.sourceTreeItemId = drag.source.treeItemId
                                    middleMoveTreeItemTimer.targetProjectId = model.projectId
                                    middleMoveTreeItemTimer.targetTreeItemId = model.treeItemId
                                    middleMoveTreeItemTimer.start()

                                }
                            }

                            Timer{
                                id: middleMoveTreeItemTimer
                                property int sourceProjectId: -1
                                property int sourceTreeItemId: -1
                                property int targetProjectId: -1
                                property int targetTreeItemId: -1
                                interval: 600
                                onTriggered: {
                                    skrData.treeHub().moveTreeItemAsChildOf(sourceProjectId, sourceTreeItemId, targetProjectId, targetTreeItemId)

                                }
                            }


                        }

                        Rectangle{
                            id: middleDropIndicator
                            z: 1


                            anchors.fill: parent
                            anchors.topMargin: topDropArea.height
                            anchors.bottomMargin: bottomDropArea.height
                            visible : false

                            gradient: Gradient {
                                orientation: Gradient.Vertical
                                GradientStop {
                                    position: 0.00;
                                    color: "transparent";
                                }
                                GradientStop {
                                    position: 0.33;
                                    color: SkrTheme.accent;
                                }
                                GradientStop {
                                    position: 0.66;
                                    color: SkrTheme.accent;
                                }
                                GradientStop {
                                    position: 1.00;
                                    color: "transparent";
                                }
                            }

                        }
                        DropArea {
                            id: bottomDropArea

                            anchors.bottom: parent.bottom
                            anchors.left: parent.left
                            anchors.right: parent.right
                            height: parent.height / dropAreaHolder.dropAreaSizeDivider

                            keys: ["application/skribisto-tree-item"]
                            onEntered: function(drag) {

                                if(!model.canAddSiblingTreeItem){
                                    drag.accepted = false
                                    return
                                }

                                if(drag.source.treeItemId === model.treeItemId){
                                    drag.accepted = false
                                    return
                                }


                                if(skrData.treeHub().getAllAncestors(projectId, parentId).includes(drag.source.treeItemId)){
                                    drag.accepted = false
                                    return
                                }

                                bottomOnEnteredTimer.start()
                            }


                            Timer{
                                id: bottomOnEnteredTimer
                                property bool visible: false
                                interval: 10
                                onTriggered: {
                                    bottomDropIndicator.visible = true

                                }
                            }

                            onExited: {
                                bottomOnExitedTimer.start()

                            }

                            Timer{
                                id: bottomOnExitedTimer
                                property bool visible: false
                                interval: 10
                                onTriggered: {
                                    bottomDropIndicator.visible = false

                                }
                            }
                            onDropped: function(drop){
                                bottomDropIndicator.visible = false
                                bottomMoveTreeItemTimer.sourceTreeItemId = drag.source.treeItemId
                                bottomMoveTreeItemTimer.targetTreeItemId = model.treeItemId
                                bottomMoveTreeItemTimer.projectId = model.projectId
                                bottomMoveTreeItemTimer.start()

                            }


                            Timer{
                                id: bottomMoveTreeItemTimer
                                property int sourceTreeItemId: -1
                                property int targetTreeItemId: -1
                                property int projectId: -1
                                interval: 600
                                onTriggered: {
                                    skrData.treeHub().moveTreeItem(projectId, sourceTreeItemId, targetTreeItemId, true)

                                }
                            }

                        }

                        Rectangle{
                            id: bottomDropIndicator
                            z: 1


                            anchors.bottom: parent.bottom
                            anchors.left: parent.left
                            anchors.right: parent.right
                            height: parent.height / dropAreaHolder.dropAreaSizeDivider
                            visible : false
                            onVisibleChanged: {
                                var nextItem = swipeDelegate.ListView.view.itemAtIndex(model.index + 1)
                                if(nextItem){
                                    nextItem.topDropIndicator.visible = visible
                                }
                            }
                            gradient: Gradient {
                                orientation: Gradient.Vertical
                                GradientStop {
                                    position: 0.00;
                                    color: "transparent";
                                }
                                GradientStop {
                                    position: 1.00;
                                    color: SkrTheme.accent;
                                }
                            }

                        }


                        SkrListItemPane {
                            id: content
                            property int visualIndex: model.index
                            property int sourceIndex: -2
                            property int projectId: model.projectId
                            property int treeItemId: model.treeItemId
                            property point dragPoint: Qt.point(width / 2, height / 2)

                            property bool isCurrent: model.index === listView.currentIndex ? true : false

                            anchors.fill: parent
                            opacity: model.cutCopy ? 0.2 : 1.0

                            Drag.active: mouseDragHandler.active | touchDragHandler.active
                            Drag.source: content
                            Drag.hotSpot.x: dragPoint.x  / 1.5
                            Drag.hotSpot.y: dragPoint.y  / 1.5
                            Drag.keys: ["application/skribisto-tree-item"]

                            Drag.supportedActions: Qt.MoveAction

                            Drag.dragType: Drag.Automatic
                            Drag.mimeData: {
                                "application/skribisto-tree-item": "Copied text"
                            }


                            borderWidth: 2
                            borderColor: mouseDragHandler.active | content.dragging ? SkrTheme.accent : "transparent"
                            Behavior on borderColor {
                                enabled: SkrSettings.ePaperSettings.animationEnabled
                                ColorAnimation {
                                    duration: 200
                                }
                            }

                            property bool dragging: false
                            DragHandler {
                                id: mouseDragHandler
                                acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                                cursorShape: Qt.ClosedHandCursor

                                //xAxis.enabled: false
                                //grabPermissions: PointerHandler.TakeOverForbidden
                                onActiveChanged: {
                                    if (active) {
                                        //swipeDelegate.ListView.view.enableSection(false)


                                        Globals.touchUsed  = false
                                        listView.interactive = false
                                        moveSourceInt = content.visualIndex
                                        moveSourceTreeItemId = content.treeItemId
                                        moveSourceProjectId = content.projectId
                                        priv.dragging = true


                                        //cancelDragTimer.stop()
                                    } else {
                                        //cancelDragTimer.stop()
                                        priv.dragging = false
                                        content.dragging = false

                                        content.Drag.drop()
                                        //swipeDelegate.ListView.view.enableSection(true)
                                    }

                                }
                                enabled: swipeDelegate.ListView.view.popupId === -1

                                onCanceled: {
                                    console.log("drag cancelled")
                                    //cancelDragTimer.stop()
                                    priv.dragging = false
                                    content.dragging = false
                                }

                                grabPermissions: PointerHandler.CanTakeOverFromItems
                                                 | PointerHandler.CanTakeOverFromAnything | PointerHandler.TakeOverForbidden


                            }

                            DragHandler {
                                id: touchDragHandler
                                acceptedDevices: PointerDevice.TouchScreen
                                                 | PointerDevice.Stylus

                                //xAxis.enabled: false
                                //grabPermissions: PointerHandler.TakeOverForbidden
                                onActiveChanged: {
                                    if (active) {
                                        //swipeDelegate.ListView.view.enableSection(false)
                                        Globals.touchUsed  = true
                                        listView.interactive = false
                                        moveSourceInt = content.visualIndex
                                        moveSourceTreeItemId = content.treeItemId
                                        moveSourceProjectId = content.projectId
                                        priv.dragging = true
                                        //cancelDragTimer.stop()
                                    } else {
                                        listView.interactive = true
                                        //cancelDragTimer.stop()
                                        priv.dragging = false
                                        content.dragging = false
                                        content.Drag.drop()
                                        //swipeDelegate.ListView.view.enableSection(true)
                                    }
                                }
                                enabled: content.dragging && swipeDelegate.ListView.view.popupId === -1

                                onCanceled: {
                                    //cancelDragTimer.stop()
                                    priv.dragging = false
                                    content.dragging = false
                                }
                                grabPermissions: PointerHandler.CanTakeOverFromItems
                                                 | PointerHandler.CanTakeOverFromAnything
                            }

                            //                            Timer {
                            //                                id: cancelDragTimer
                            //                                repeat: false
                            //                                interval: 3000
                            //                                onTriggered: {
                            //                                    priv.dragging = false
                            //                                    content.dragging = false
                            //                                }
                            //                            }

                            TapHandler {
                                id: tapHandler
                                acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad

                                onSingleTapped: function(eventPoint) {
                                    priv.selecting = false

                                    if (content.dragging || priv.renaming) {
                                        eventPoint.accepted = false
                                        return
                                    }

                                    Globals.touchUsed  = false
                                    swipeDelegate.ListView.view.interactive = false


                                    if (skrData.treePropertyHub(
                                                ).getProperty(
                                                model.projectId,
                                                model.treeItemId,
                                                "can_add_child_tree_item",
                                                "true") === "true") {

                                        if( root.popupId >= 0){
                                            setCurrentTreeItemParentIdCalled(model.projectId , model.treeItemId)
                                        }
                                        else{
                                            swipeDelegate.ListView.view.currentIndex = model.index
                                            goToChildTimer.start()
                                        }

                                    } else {
                                        if( root.popupId >= 0){
                                            openDocumentTimer.start()
                                        }
                                        else {
                                            swipeDelegate.ListView.view.currentIndex = model.index
                                        }
                                    }

                                    swipeDelegate.forceActiveFocus()
                                    eventPoint.accepted = true
                                }

                                onDoubleTapped: function(eventPoint) {
                                    openDocumentTimer.stop()
                                    goToChildTimer.stop()
                                    if (content.dragging || priv.renaming) {
                                        eventPoint.accepted = false
                                        return
                                    }

                                    Globals.touchUsed  = false
                                    swipeDelegate.ListView.view.interactive = false


                                    //console.log("double tapped")
                                    swipeDelegate.ListView.view.currentIndex = model.index

                                    openDocumentTimer.start()

                                    eventPoint.accepted = true
                                }

                                onGrabChanged: function(transition, point) {
                                    point.accepted = false
                                }

                                onPressedChanged: {
                                    content.grabToImage(function(result) {
                                        content.Drag.imageSource = result.url
                                    }, Qt.size(content.width / 1.5, content.height / 1.5))

                                    content.dragPoint = point.pressPosition
                                }

                                //grabPermissions: PointerHandler.TakeOverForbidden
                            }

                            TapHandler {
                                id: touchTapHandler
                                acceptedDevices: PointerDevice.TouchScreen | PointerDevice.Stylus


                                onSingleTapped: function(eventPoint) {
                                    priv.selecting = false

                                    if (content.dragging || priv.renaming) {
                                        eventPoint.accepted = false
                                        return
                                    }

                                    Globals.touchUsed  = true
                                    swipeDelegate.ListView.view.interactive = true

                                    swipeDelegate.ListView.view.currentIndex = model.index

                                    if (skrData.treePropertyHub(
                                                ).getProperty(
                                                model.projectId,
                                                model.treeItemId,
                                                "can_add_child_tree_item",
                                                "true") === "true") {
                                        goToChildTimer.start()
                                    } else {
                                        openDocumentTimer.start()
                                    }

                                    //delegateRoot.forceActiveFocus()
                                    eventPoint.accepted = true
                                }

                                onDoubleTapped: function(eventPoint) {
                                    openDocumentTimer.stop()
                                    goToChildTimer.stop()
                                    if (content.dragging || priv.renaming) {
                                        eventPoint.accepted = false
                                        return
                                    }

                                    Globals.touchUsed  = true
                                    swipeDelegate.ListView.view.interactive = true

                                    //console.log("double tapped")
                                    swipeDelegate.ListView.view.currentIndex = model.index

                                    openDocumentTimer.start()

                                    eventPoint.accepted = true
                                }

                                onLongPressed: {
                                    Globals.touchUsed  = true

                                    // needed to activate the grab handler

                                    content.dragging = true
                                    listView.interactive = false
                                    priv.selecting = false
                                }

                                onGrabChanged: function(transition, point) {
                                    point.accepted = true
                                }

                                onPressedChanged: {

                                    content.grabToImage(function(result) {
                                        content.Drag.imageSource = result.url
                                    }, Qt.size(content.width / 1.5, content.height / 1.5))

                                    content.dragPoint = point.pressPosition
                                }

                               // grabPermissions: PointerHandler.TakeOverForbidden
                            }
                            Timer {
                                id: openDocumentTimer
                                interval: 150
                                onTriggered: {
                                    swipeDelegate.configureMenu()
                                    openTreeItemAction.trigger()
                                }
                            }

                            Timer {
                                id: goToChildTimer
                                interval: 150
                                onTriggered: {
                                    goToChildAction.trigger()
                                }
                            }

                            TapHandler {
                                id: rightClickTapHandler
                                acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                                acceptedButtons: Qt.RightButton
                                onSingleTapped: function(eventPoint) {
                                    listView.interactive = eventPoint.device.type
                                            === PointerDevice.Mouse
                                    Globals.touchUsed  = false

                                    if (menu.visible) {
                                        menu.close()
                                        return
                                    }

                                    listView.currentIndex = model.index



                                    swipeDelegate.configureMenu()

                                    menu.popup(swipeDelegate, 0, swipeDelegate.height)

                                    eventPoint.accepted = true
                                }

                                grabPermissions: PointerHandler.TakeOverForbidden
                            }

                            TapHandler {
                                id: middleClickTapHandler
                                acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                                acceptedButtons: Qt.MiddleButton
                                onSingleTapped: function(eventPoint) {
                                    listView.interactive = eventPoint.device.type
                                            === PointerDevice.Mouse
                                    Globals.touchUsed  = false
                                    listView.currentIndex = model.index
                                    swipeDelegate.configureMenu()
                                    openTreeItemInAnotherViewAction.trigger()
                                    eventPoint.accepted = true
                                }
                                grabPermissions: PointerHandler.TakeOverForbidden
                            }

                            /// without MouseArea, it breaks while dragging and scrolling:
                            MouseArea {
                                anchors.fill: parent
                                acceptedButtons: Qt.NoButton
                                onWheel: function(wheel) {
                                    listView.interactive = false
                                    listView.flick(
                                                0,
                                                wheel.angleDelta.y * 50)
                                    wheel.accepted = true
                                }

                                enabled: listView.interactive === false
                            }




                            Action {
                                id: goToChildAction
                                //shortcut: "Right"
                                enabled: listView.enabled
                                         && listView.currentIndex === model.index
                                         && model.canAddChildTreeItem

                                //icon.source: model.hasChildren ? "qrc:///icons/backup/go-next.svg" : (model.canAddChildTreeItem ? "qrc:///icons/backup/list-add.svg" : "")
                                text: qsTr("See sub-items")
                                onTriggered: {
                                    console.log("goToChildAction triggered")


                                    root.setCurrentTreeItemParentIdCalled(model.projectId, model.treeItemId)



                                }
                            }

                            Timer {
                                property int treeItemIdToEdit: -2
                                onTreeItemIdToEditChanged: {
                                    if (treeItemIdToEdit !== -2) {
                                        editNameTimer.start()
                                    }
                                }

                                id: editNameTimer
                                repeat: false
                                interval: animationDuration
                                onTriggered: {
                                    var index = swipeDelegate.ListView.view.proxyModel.findVisualIndex(
                                                model.projectId,
                                                treeItemIdToEdit)
                                    if (index !== -2) {
                                        listView.itemAtIndex(
                                                    index).editName()
                                    }
                                    treeItemIdToEdit = -2
                                }
                            }


                            contentItem: ColumnLayout {
                                id: columnLayout3
                                anchors.fill: parent

                                RowLayout {
                                    id: rowLayout
                                    spacing: 2
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true

                                    Rectangle {
                                        id: currentItemIndicator
                                        color: listView.currentIndex === model.index ? "lightsteelblue" : "transparent"
                                        Layout.fillHeight: true
                                        Layout.preferredWidth: 3
                                    }
                                    Rectangle {
                                        id: openedItemIndicator
                                        color: model.projectId === rootWindow.viewManager.focusedPage.projectId
                                               && model.treeItemId === rootWindow.viewManager.focusedPage.treeItemId ? SkrTheme.accent : "transparent"
                                        Layout.fillHeight: true
                                        Layout.preferredWidth: 3
                                    }

                                    SkrToolButton {
                                        id: projectIsBackupIndicator
                                        visible: model.projectIsBackup
                                                 && model.type === 'PROJECT'
                                        enabled: true
                                        focusPolicy: Qt.NoFocus
                                        implicitHeight: 32 * SkrSettings.interfaceSettings.zoom
                                        implicitWidth: 32 * SkrSettings.interfaceSettings.zoom
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
                                            height: 32 * SkrSettings.interfaceSettings.zoom
                                            width: 32 * SkrSettings.interfaceSettings.zoom
                                        }

                                        hoverEnabled: true
                                        ToolTip.delay: 1000
                                        ToolTip.timeout: 5000
                                        ToolTip.visible: hovered
                                        ToolTip.text: qsTr("This project is a backup")
                                    }

                                    SkrCheckBox {
                                        id: selectionCheckBox
                                        visible: priv.selecting
                                        implicitHeight: 32 * SkrSettings.interfaceSettings.zoom
                                        implicitWidth: 32 * SkrSettings.interfaceSettings.zoom
                                        Layout.maximumHeight: 32 * SkrSettings.interfaceSettings.zoom
                                        padding: 0
                                        rightPadding: 0
                                        bottomPadding: 0
                                        leftPadding: 0
                                        topPadding: 0
                                        focusPolicy: Qt.NoFocus

                                        onCheckStateChanged: {
                                            model.checkState = selectionCheckBox.checkState
                                            determineSelectedTreeItems()
                                            console.log("onCheckStateChanged")
                                        }

                                        Binding on checkState {
                                            value: model.checkState
                                            delayed: true
                                            restoreMode: Binding.RestoreBindingOrValue
                                        }

                                        function determineSelectedTreeItems() {
                                            priv.selectedTreeItemsIds = []
                                            priv.selectedTreeItemsIds
                                                    = navigationProxyModel.getCheckedIdsList()
                                            priv.selectedProjectId = currentProjectId

                                            console.log(priv.selectedTreeItemsIds)
                                        }
                                    }

                                    Item {
                                        Layout.maximumHeight: 30 * SkrSettings.interfaceSettings.zoom
                                        implicitHeight: treeItemIconIndicator.implicitHeight
                                        implicitWidth: treeItemIconIndicator.implicitWidth

                                        SkrToolButton {
                                            id: treeItemIconIndicator
                                            //visible: model.projectIsBackup && model.treeItemId === -1
                                            enabled: true
                                            focusPolicy: Qt.NoFocus
                                            implicitHeight: 30 * SkrSettings.interfaceSettings.zoom
                                            implicitWidth: 30 * SkrSettings.interfaceSettings.zoom
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

                                                height: 30 * SkrSettings.interfaceSettings.zoom
                                                width: 30 * SkrSettings.interfaceSettings.zoom
                                                source: model.otherProperties ? getIconUrlFromPageType(
                                                                                    model.type, model.projectId, model.treeItemId) : getIconUrlFromPageType(
                                                                                    model.type, model.projectId, model.treeItemId)

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

                                                TapHandler {

                                                    onSingleTapped: function(eventPoint) {
                                                        tapHandler.singleTapped(
                                                                    eventPoint)
                                                    }

                                                    onDoubleTapped: function(eventPoint) {
                                                        tapHandler.doubleTapped(
                                                                    eventPoint)
                                                    }

                                                    onGrabChanged: function(transition, point) {
                                                        tapHandler.grabChanged(
                                                                    transition,
                                                                    point)
                                                    }
                                                }

                                                TapHandler {
                                                    acceptedDevices: PointerDevice.Mouse| PointerDevice.Stylus
                                                    acceptedButtons: Qt.RightButton

                                                    onSingleTapped: function(eventPoint) {
                                                        rightClickTapHandler.singleTapped(
                                                                    eventPoint)
                                                    }

                                                    onDoubleTapped: function(eventPoint) {
                                                        rightClickTapHandler.doubleTapped(
                                                                    eventPoint)
                                                    }

                                                    onGrabChanged: function(transition, point) {
                                                        rightClickTapHandler.grabChanged(
                                                                    transition,
                                                                    point)
                                                    }
                                                }

                                                TapHandler {
                                                    acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                                                    acceptedButtons: Qt.MiddleButton

                                                    onSingleTapped: function(eventPoint) {
                                                        middleClickTapHandler.singleTapped(
                                                                    eventPoint)
                                                    }

                                                    onDoubleTapped: function(eventPoint) {
                                                        middleClickTapHandler.doubleTapped(
                                                                    eventPoint)
                                                    }

                                                    onGrabChanged: function(transition, point) {
                                                        middleClickTapHandler.grabChanged(
                                                                    transition,
                                                                    point)
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
                                                activeFocusOnTab: false

                                                Layout.fillWidth: true
                                                Layout.topMargin: 2
                                                Layout.leftMargin: 4
                                                Layout.alignment: Qt.AlignLeft
                                                                  | Qt.AlignVCenter
                                                font.bold: model.projectIsActive
                                                           && model.indent
                                                           === -1 ? true : false
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
                                                Keys.onShortcutOverride: function(event) { event.accepted = (event.key === Qt.Key_Escape)}
                                                Keys.onPressed: function(event) {
                                                    if (event.key === Qt.Key_Return) {
                                                        console.log("Return key pressed title")
                                                        editingFinished(
                                                                    )
                                                        event.accepted = true
                                                    }
                                                    if ((event.modifiers & Qt.CtrlModifier)
                                                            && event.key
                                                            === Qt.Key_Return) {
                                                        console.log("Ctrl Return key pressed title")
                                                        editingFinished(
                                                                    )
                                                        event.accepted = true
                                                    }
                                                    if (event.key === Qt.Key_Escape) {
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
                                                    if (model.type === "PROJECT") {
                                                        //project item
                                                        model.projectName = text
                                                    } else {
                                                        model.title = text
                                                    }

                                                    swipeDelegate.state = ""

                                                    //fix bug while new lone child
                                                    titleLabel.visible = true
                                                    labelLayout.visible = true
                                                    priv.renaming = false
                                                }

                                                //Keys.priority: Keys.AfterItem
                                                Keys.onShortcutOverride: function(event) { event.accepted = (event.key === Qt.Key_Escape)}
                                                Keys.onPressed: function(event) {
                                                    if (event.key === Qt.Key_Return) {
                                                        console.log("Return key pressed title")
                                                        editingFinished(
                                                                    )
                                                        event.accepted = true
                                                    }
                                                    if ((event.modifiers & Qt.CtrlModifier)
                                                            && event.key
                                                            === Qt.Key_Return) {
                                                        console.log("Ctrl Return key pressed title")
                                                        editingFinished(
                                                                    )
                                                        event.accepted = true
                                                    }
                                                    if (event.key === Qt.Key_Escape) {
                                                        console.log("Escape key pressed title")
                                                        swipeDelegate.state = ""
                                                        event.accepted = true
                                                    }
                                                }
                                            }

                                            RowLayout {
                                                id: labelLayout
                                                Layout.fillWidth: true
                                                Layout.leftMargin: 5

                                                ListItemAttributes {
                                                    id: attributes
                                                    treeItemId: model.treeItemId
                                                    projectId: model.projectId
                                                    Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                                    Layout.leftMargin: 4
                                                    Layout.bottomMargin: 2
                                                }

                                                SkrLabel {
                                                    id: labelLabel
                                                    activeFocusOnTab: false
                                                    text: model.label
                                                          === undefined ? "" : model.label
                                                    visible: text.length
                                                             === 0 ? false : true
                                                    Layout.bottomMargin: 2
                                                    Layout.rightMargin: 4
                                                    Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                                                    elide: Text.ElideRight
                                                    font.italic: true
                                                    horizontalAlignment: Qt.AlignRight
                                                    Layout.fillWidth: true
                                                }
                                            }
                                        }
                                    }

                                    SkrLabel {
                                        id: devLabel
                                        activeFocusOnTab: false
                                        text: model.index + "-" + model.treeItemId
                                              + "-" + model.sortOrder
                                        visible: +priv.devModeEnabled
                                        elide: Text.ElideNone
                                        Layout.bottomMargin: 2
                                        Layout.rightMargin: 4
                                        Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                                        horizontalAlignment: Qt.AlignRight
                                        Layout.fillHeight: true
                                        //Layout.preferredWidth: 30
                                    }

                                    SkrToolButton {
                                        id: menuButton
                                        Layout.fillHeight: true
                                        Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom

                                        text: qsTr("Item menu")
                                        icon.source: "qrc:///icons/backup/overflow-menu.svg"
                                        flat: true
                                        focusPolicy: Qt.NoFocus

                                        onClicked: {

                                            if (menu.visible) {
                                                menu.close()
                                                return
                                            }

                                            listView.currentIndex = model.index
                                            swipeDelegate.forceActiveFocus()

                                            swipeDelegate.configureMenu()

                                            menu.popup(menuButton, 0, menuButton.height)
                                        }

                                        visible: itemHoverHandler.hovered
                                                 || content.isCurrent
                                    }

                                    Rectangle {
                                        Layout.fillHeight: true
                                        Layout.preferredWidth: 2

                                        color: model.indent === 0 ? Material.color(Material.Indigo) : (model.indent === 1 ? Material.color(Material.LightBlue) : (model.indent === 2 ? Material.color(Material.LightGreen) : (model.indent === 3 ? Material.color(Material.Amber) : (model.indent === 4 ? Material.color(Material.DeepOrange) : Material.color(Material.Teal)))))
                                    }
                                }
                                Rectangle {
                                    id: separator
                                    Layout.preferredHeight: 1
                                    Layout.preferredWidth: content.width / 2
                                    Layout.alignment: Qt.AlignHCenter | Qt.AlignBottom
                                    gradient: Gradient {
                                        orientation: Gradient.Horizontal
                                        GradientStop {
                                            position: 0.00
                                            color: "transparent"
                                        }
                                        GradientStop {
                                            position: 0.30
                                            color: SkrTheme.divider
                                        }
                                        GradientStop {
                                            position: 0.70
                                            color: SkrTheme.divider
                                        }
                                        GradientStop {
                                            position: 1.00
                                            color: "transparent"
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
                        //                        State {
                        //                            name: "drag_active"
                        //                            when: content.Drag.active

                        //                            //                            ParentChange {
                        //                            //                                target: swipeDelegate
                        //                            //                                parent: Overlay.overlay
                        //                            //                            }
                        //                            PropertyChanges {
                        //                                target: swipeDelegate
                        //                                z: 3
                        //                            }
                        //                            AnchorChanges {
                        //                                target: content
                        //                                anchors {
                        //                                    horizontalCenter: undefined
                        //                                    verticalCenter: undefined
                        //                                }
                        //                            }
                        //                        },
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
                                target: swipeDelegate
                                height: 50
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
                                target: swipeDelegate
                                height: 50
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

                    property alias removeTreeItemAnimation: removeTreeItemAnimation

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
                            script: swipeDelegate.ListView.view.proxyModel.invalidate()
                        }
                    }
                    property alias addTreeItemAtEndAnimation: addTreeItemAtEndAnimation
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

    SkrMenu {
        id: menu
        //y: menuButton.height
        width: 300
        z: 101
        closePolicy: Popup.CloseOnPressOutside | Popup.CloseOnEscape

        property int index
        property int treeItemId
        property int projectId
        property string type
        property bool projectIsActive
        property bool isOpenable
        property bool canAddChildTreeItem
        property bool canAddSiblingTreeItem
        property bool isCopyable
        property bool isMovable
        property bool isRenamable
        property bool isTrashable


        onOpened: {

        }

        onClosed: {

        }

        SkrMenuItem {
            height: !menu.isOpenable ? 0 : undefined
            visible: menu.isOpenable
            action: Action {
                id: openTreeItemAction
                text: qsTr("Open")
                //shortcut: "Return"
                icon {
                    source: "qrc:///icons/backup/document-edit.svg"
                }
                enabled: menu.isOpenable && listView.enabled

                onTriggered: {
                    console.log("open treeItem action",
                                menu.projectId,
                                menu.treeItemId)
                    root.openDocument(menu.projectId,
                                      menu.treeItemId)
                }
            }
        }
        SkrMenuItem {
            height: ! menu.isOpenable
                    ||  menu.treeItemId === -1 ? 0 : undefined
            visible:  menu.isOpenable

            action: Action {
                id: openTreeItemInAnotherViewAction
                text: qsTr("Open in another view")
                //shortcut: "Alt+Return"
                icon {
                    source: "qrc:///icons/backup/tab-new.svg"
                }
                enabled: menu.isOpenable && listView.enabled
                onTriggered: {
                    console.log("open treeItem in another view action",
                                menu.projectId,
                                menu.treeItemId)
                    root.openDocumentInAnotherView(
                                menu.projectId,
                                menu.treeItemId)
                }
            }
        }

        SkrMenuItem {
            height: ! menu.isOpenable ? 0 : undefined
            visible:  menu.isOpenable

            action: Action {
                id: openTreeItemInNewWindowAction
                text: qsTr("Open in a new window")
                //shortcut: "Alt+Return"
                icon {
                    source: "qrc:///icons/backup/window-new.svg"
                }
                enabled: menu.isOpenable && listView.enabled
                onTriggered: {
                    console.log("open treeItem in new window action",
                                menu.projectId,
                                menu.treeItemId)

                    root.openDocumentInNewWindow(
                                menu.projectId,
                                menu.treeItemId)
                    //                                                setCurrentTreeItemId( menu.projectId,  menu.treeItemId)
                }



            }
        }

        SkrMenuItem {
            height:  menu.treeItemId === 0 ? undefined : 0
            visible:  menu.treeItemId === 0
            enabled:  menu.projectIsActive === false
                      && listView.enabled
                      &&  menu.treeItemId === 0
            text: qsTr("Set as active project")
            icon {
                source: "qrc:///icons/backup/tab-new.svg"
            }
            onTriggered: {
                console.log("set active project",
                            menu.projectId)
                skrData.projectHub(
                            ).setActiveProject(
                            menu.projectId)
            }
        }

        MenuSeparator {
            height:  menu.isRenamable ? undefined : 0
            visible:  menu.isRenamable
        }

        SkrMenuItem {
            height:  menu.isRenamable ? undefined : 0
            visible:  menu.isRenamable
            action: Action {
                id: renameAction
                text: qsTr("Rename")
                icon {
                    source: "qrc:///icons/backup/edit-rename.svg"
                }
                enabled: listView.enabled

                onTriggered: {
                    console.log("rename action",
                                menu.projectId,
                                menu.treeItemId)
                    listView.currentItem.editName()
                }
            }
        }

        SkrMenuItem {
            height: ! menu.isRenamable ? 0 : undefined
            visible:  menu.isRenamable
            action: Action {
                id: setLabelAction
                text: qsTr("Set label")
                //shortcut: "F2"
                icon {
                    source: "qrc:///icons/backup/label.svg"
                }
                enabled: listView.enabled
                onTriggered: {
                    console.log("sel label",
                                menu.projectId,
                                menu.treeItemId)
                    listView.currentItem.editLabel()
                }
            }
        }

        MenuSeparator {
            height: ! menu.isMovable
                    || ! menu.isCopyable
                    || ! menu.canAddChildTreeItem ? 0 : undefined
            visible:  menu.isMovable
                      ||  menu.isCopyable
                      ||  menu.canAddChildTreeItem
        }

        SkrMenuItem {
            height: ! menu.isMovable ? 0 : undefined
            visible:  menu.isMovable
            action: Action {
                id: cutAction
                text: qsTr("Cut")
                //shortcut: StandardKey.Cut
                icon {
                    source: "qrc:///icons/backup/edit-cut.svg"
                }
                enabled: listView.enabled

                onTriggered: {

                    if (priv.selectedTreeItemsIds.length > 0) {
                        console.log("cut action",
                                    menu.projectId,
                                    priv.selectedTreeItemsIds)
                        skrData.treeHub().cut(
                                    menu.projectId,
                                    priv.selectedTreeItemsIds)
                    } else {
                        console.log("cut action",
                                    menu.projectId,
                                    menu.treeItemId)
                        skrData.treeHub().cut(
                                    menu.projectId,
                                    [ menu.treeItemId])
                    }
                }
            }
        }

        SkrMenuItem {
            height: ! menu.isCopyable ? 0 : undefined
            visible:  menu.isCopyable
            action: Action {

                id: copyAction
                text: qsTr("Copy")
                //shortcut: StandardKey.Copy
                icon {
                    source: "qrc:///icons/backup/edit-copy.svg"
                }
                enabled: listView.enabled

                onTriggered: {
                    if (priv.selectedTreeItemsIds.length > 0) {
                        console.log("copy action",
                                    menu.projectId,
                                    priv.selectedTreeItemsIds)
                        skrData.treeHub().copy(
                                    menu.projectId,
                                    priv.selectedTreeItemsIds)
                    } else {
                        console.log("copy action",
                                    menu.projectId,
                                    menu.treeItemId)
                        skrData.treeHub().copy(
                                    menu.projectId,
                                    [ menu.treeItemId])
                    }
                }
            }
        }

        SkrMenuItem {
            height: ! menu.canAddChildTreeItem ? 0 : undefined
            visible:  menu.canAddChildTreeItem
            action: Action {

                id: pasteAction
                text: qsTr("Paste")
                //shortcut: StandardKey.Paste
                icon {
                    source: "qrc:///icons/backup/edit-paste.svg"
                }
                enabled: listView.enabled

                onTriggered: {
                    console.log("paste action",
                                menu.projectId,
                                menu.treeItemId)
                    var result = skrData.treeHub(
                                ).paste(
                                menu.projectId,
                                menu.treeItemId,
                                true)

                    if (!result.success) {
                        console.debug(
                                    "paste action: error")
                    }
                }
            }
        }

        MenuSeparator {
            height: !( menu.canAddSiblingTreeItem
                      ||  menu.canAddChildTreeItem) ? 0 : undefined
            visible: ( menu.canAddSiblingTreeItem
                      ||  menu.canAddChildTreeItem)
        }

        SkrMenuItem {
            height: ! menu.canAddSiblingTreeItem ? 0 : undefined
            visible:  menu.canAddSiblingTreeItem
            action: Action {
                id: addBeforeAction
                text: qsTr("Add before")
                icon {
                    source: "qrc:///icons/backup/document-new.svg"
                }
                enabled: listView.enabled
                onTriggered: {
                    console.log("add before action",
                                menu.projectId,
                                menu.treeItemId)

                    var visualIndex = listView.currentIndex

                    newItemPopup.projectId =  menu.projectId
                    newItemPopup.treeItemId =  menu.treeItemId
                    newItemPopup.visualIndex = visualIndex
                    newItemPopup.createFunction
                            = afterNewItemTypeIsChosen
                    newItemPopup.open()
                }

                function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType, quantity) {

                    for(var i = 0; i < quantity ; i++){
                        skrData.treeHub().addTreeItemAbove(
                                    projectId,
                                    treeItemId,
                                    pageType)

                    }
                    listView.itemAtIndex(
                                visualIndex).editName()
                }
            }
        }

        SkrMenuItem {
            height: ! menu.canAddSiblingTreeItem ? 0 : undefined
            visible:  menu.canAddSiblingTreeItem
            action: Action {
                id: addAfterAction
                text: qsTr("Add after")
                icon {
                    source: "qrc:///icons/backup/document-new.svg"
                }
                enabled: listView.enabled
                onTriggered: {
                    console.log("add after action",
                                menu.projectId,
                                menu.treeItemId)

                    var visualIndex = listView.currentIndex + 1

                    newItemPopup.projectId =  menu.projectId
                    newItemPopup.treeItemId =  menu.treeItemId
                    newItemPopup.visualIndex = visualIndex
                    newItemPopup.createFunction
                            = afterNewItemTypeIsChosen
                    newItemPopup.open()
                }

                function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType, quantity) {

                    for(var i = 0; i < quantity ; i++){
                        skrData.treeHub().addTreeItemBelow(
                                    projectId,
                                    treeItemId,
                                    pageType)

                    }
                    listView.itemAtIndex(
                                visualIndex).editName()
                }

            }
        }

        SkrMenuItem {
            height: ! menu.canAddChildTreeItem ? 0 : undefined
            visible:  menu.canAddChildTreeItem
            action: Action {
                id: addChildAction
                text: qsTr("Add a sub-item")
                icon {
                    source: "qrc:///icons/backup/document-new.svg"
                }
                enabled: listView.enabled
                onTriggered: {
                    console.log("add child action",
                                menu.projectId,
                                menu.treeItemId)

                    //save current ids:

                    newItemPopup.projectId =  menu.projectId
                    newItemPopup.treeItemId =  menu.treeItemId
                    newItemPopup.visualIndex = 0
                    newItemPopup.createFunction
                            = afterNewItemTypeIsChosen
                    newItemPopup.open()
                }

                function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType, quantity) {

                    // push new view
                    setCurrentTreeItemParentIdCalled(projectId, treeItemId)

                    for(var i = 0; i < quantity ; i++){
                        skrData.treeHub().addChildTreeItem(projectId, treeItemId, pageType)
                    }
                }
            }
        }
        MenuSeparator {
            height: ! menu.isMovable ? 0 : undefined
            visible:  menu.isMovable
        }

        SkrMenuItem {
            height: ! menu.isMovable ? 0 : undefined
            visible:  menu.isMovable
            action: Action {
                id: moveUpAction
                text: qsTr("Move up")
                //shortcut: "Ctrl+Up"
                icon {
                    source: "qrc:///icons/backup/object-order-raise.svg"
                }
                enabled: listView.enabled
                         &&  menu.index !== 0
                onTriggered: {
                    console.log("move up action",
                                menu.projectId,
                                menu.treeItemId)

                    if (temporarilyDisableMove) {
                        return
                    }
                    temporarilyDisableMove = true
                    temporarilyDisableMoveTimer.start()

                    skrData.treeHub().moveTreeItemUp(
                                menu.projectId,
                                menu.treeItemId)
                    listView.decrementCurrentIndex()

                }
            }
        }

        SkrMenuItem {
            height: ! menu.isMovable ? 0 : undefined
            visible:  menu.isMovable
            action: Action {
                id: moveDownAction
                text: qsTr("Move down")
                //shortcut: "Ctrl+Down"
                icon {
                    source: "qrc:///icons/backup/object-order-lower.svg"
                }
                enabled:  menu.index !== visualModel.items.count - 1
                          && listView.enabled

                onTriggered: {
                    console.log("move down action",
                                menu.projectId,
                                menu.treeItemId)

                    if (temporarilyDisableMove) {
                        return
                    }
                    temporarilyDisableMove = true
                    temporarilyDisableMoveTimer.start()

                    skrData.treeHub().moveTreeItemDown(
                                menu.projectId,
                                menu.treeItemId)
                    listView.incrementCurrentIndex()
                }
            }
        }

        MenuSeparator {
            height: ! menu.isTrashable ? 0 : undefined
            visible:  menu.isTrashable
        }

        SkrMenuItem {
            height: ! menu.isTrashable ? 0 : undefined
            visible:  menu.isTrashable
            action: Action {
                id: sendToTrashAction
                text: qsTr("Send to trash")
                //shortcut: "Del"
                icon {
                    source: "qrc:///icons/backup/edit-delete.svg"
                    color: "transparent"
                }
                enabled: listView.enabled
                         &&  menu.indent !== -1
                onTriggered: {
                    console.log("sent to trash action",
                                menu.projectId,
                                menu.treeItemId)

                    listView.currentItem.swipe.close()
                    listView.currentItem.removeTreeItemAnimation.start()
                    skrData.treeHub().setTrashedWithChildren(
                                menu.projectId,
                                menu.treeItemId, true)
                }
            }
        }

        MenuSeparator {
            height:  menu.treeItemId !== 0 ? 0 : undefined
            visible:  menu.treeItemId === 0
        }

        SkrMenuItem {
            height:  menu.treeItemId === 0 ? undefined : 0
            visible:  menu.treeItemId === 0
            enabled: listView.enabled
                     &&  menu.treeItemId === 0
            text: qsTr("Close this project")
            icon {
                source: "qrc:///icons/backup/document-close.svg"
                color: "transparent"
            }
            onTriggered: {
                console.log("close this project",
                            menu.projectId)

                sidePopupListModel.clear()
                Globals.closeProjectCalled(
                            menu.projectId)
            }
        }
    }


}

