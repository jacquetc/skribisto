import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import QtQuick.Controls.Material 2.15
import eu.skribisto.searchtreelistproxymodel 1.0
import "../../Commons"
import "../../Items"
import "../.."

NavigationListForm {
    id: root

    property var proxyModel

    signal openDocument(int openedProjectId, int openedTreeItemId, int projectId, int treeItemId)
    signal openDocumentInAnotherView(int projectId, int treeItemId)
    signal openDocumentInNewWindow(int projectId, int treeItemId)
    signal showTrashedList

    readonly property int currentParentId: priv.currentParentId
    readonly property int currentProjectId: priv.currentProjectId
    readonly property int currentTreeItemId: priv.currentTreeItemId
    property int openedProjectId: -2
    property int openedTreeItemId: -2

    readonly property alias selectedTreeItemsIds: priv.selectedTreeItemsIds
    readonly property alias selectedProjectId: priv.selectedProjectId

    QtObject {
        id: priv
        property int currentParentId: -2
        property int currentProjectId: -2
        property int currentTreeItemId: -2
        property bool dragging: false
        property bool renaming: false
        property bool selecting: false
        property bool animationEnabled: SkrSettings.ePaperSettings.animationEnabled
        property int transitionOperation: animationEnabled ? StackView.Transition : StackView.Immediate

        onSelectingChanged: {
            if (!selecting) {
                navigationListStackView.currentItem.proxyModel.checkNone()
            }
        }
        property var selectedTreeItemsIds: []
        property int selectedProjectId: -2

        property bool devModeEnabled: SkrSettings.devSettings.devModeEnabled
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
    function setCurrentTreeItemParentId(projectId, treeItemParentId) {

        //find parent id
        if (projectId > -1 & treeItemParentId > -1) {
            var ancestorsList = proxyModel.getAncestorsList(
                        projectId, treeItemParentId,
                        proxyModel.showTrashedFilter,
                        proxyModel.showNotTrashedFilter)
        }

        //compare with current parent id
        if (projectId === root.currentProjectId & treeItemParentId === root.currentParentId) {

            //            navigationListStackView.currentItem.proxyModel.setCurrentTreeItemId(
            //                        projectId, -1)
        } else if (projectId === -1 & treeItemParentId === -1) {
            navigationListStackView.pop(null, priv.transitionOperation)
            //project item
            navigationListStackView.get(0).projectId = -2
            navigationListStackView.get(0).parentId = -2
            navigationListStackView.get(0).treeItemId = -2
            navigationListStackView.get(0).init()
            navigationListStackView.get(0).setCurrent()
        } else {
            navigationListStackView.pop(null, priv.transitionOperation)
            ancestorsList.reverse()
            ancestorsList.push(treeItemParentId, priv.transitionOperation)

            if (ancestorsList[ancestorsList.length - 1] === -1) {
                ancestorsList.pop()
            }
            //project item
            navigationListStackView.get(0).projectId = projectId
            navigationListStackView.get(0).treeItemId = 0
            navigationListStackView.get(0).init()
            navigationListStackView.get(0).setCurrent()

            for (var i = 1; i < ancestorsList.length; i++) {
                var newItem = navigationListStackView.push(stackViewComponent, {
                                                               "projectId": projectId,
                                                               "treeItemId": ancestorsList[i]
                                                           },
                                                           priv.transitionOperation)
                newItem.setCurrent()
            }

            var lastNewItem = navigationListStackView.push(stackViewComponent, {
                                                               "projectId": projectId,
                                                               "parentId": treeItemParentId
                                                           },
                                                           priv.transitionOperation)
            lastNewItem.setCurrent()
            rootWindow.protectedSignals.setBreadcrumbCurrentTreeItemCalled(
                        priv.currentProjectId, priv.currentParentId)
        }

        sidePopupListModel.clear()
        determineIfGoUpButtonEnabled()
        priv.selecting = false
    }

    function setCurrentTreeItemId(projectId, treeItemId) {

        //find parent id
        var ancestorsList = proxyModel.getAncestorsList(
                    projectId, treeItemId, proxyModel.showTrashedFilter,
                    proxyModel.showNotTrashedFilter)

        var newParentId = ancestorsList[0]

        //compare with current parent id
        if (projectId === root.currentProjectId & newParentId === root.currentParentId) {
            navigationListStackView.currentItem.proxyModel.setCurrentTreeItemId(
                        projectId, treeItemId)
        } //        else if(projectId === root.currentProjectId){

        //        }
        else {
            navigationListStackView.pop(null, priv.transitionOperation)
            ancestorsList.reverse()
            ancestorsList.push(treeItemId, priv.transitionOperation)

            if (ancestorsList[ancestorsList.length - 1] === -1) {
                ancestorsList.pop()
            }

            //project item
            navigationListStackView.get(0).projectId = projectId
            navigationListStackView.get(0).treeItemId = 0
            navigationListStackView.get(0).init()
            navigationListStackView.get(0).setCurrent()

            for (var i = 1; i < ancestorsList.length; i++) {
                var newItem = navigationListStackView.push(stackViewComponent, {
                                                               "projectId": projectId,
                                                               "treeItemId": ancestorsList[i]
                                                           },
                                                           priv.transitionOperation)
                newItem.setCurrent()
            }
        }

        sidePopupListModel.clear()
        determineIfGoUpButtonEnabled()
        priv.selecting = false
    }

    //-----------------------------------------------------------------------------

    //toolBarPrimaryColor: Material.color(Material.Cyan, Material.Shade200)

    //-----------------------------------------------------------------------------
    // go up button :
    goUpToolButton.action: goUpAction

    Action {
        id: goUpAction
        text: qsTr("Go up")
        icon {
            source: "qrc:///icons/backup/go-parent-folder.svg"
        }
        //enabled:
        onTriggered: {

            //var parentTreeItemId = proxyModel.getAncestorsList(root.currentProjectId, root.currentTreeItemId, proxyModel.showTrashedFilter, proxyModel.showNotTrashedFilter)[0]
            navigationListStackView.pop(priv.transitionOperation)
            navigationListStackView.currentItem.setCurrent()
            //console.log("index", navigationListStackView.currentItem.currentIndex)
            navigationListStackView.currentItem.listView.currentItem.forceActiveFocus()

            var index = navigationListStackView.currentItem.listView.currentIndex
            var item = navigationListStackView.currentItem.listView.itemAtIndex(
                        index)
            if (item) {
                item.forceActiveFocus()
            } else {
                navigationListStackView.currentItem.listView.forceActiveFocus()
            }
            priv.currentProjectId = navigationListStackView.currentItem.projectId
            priv.currentParentId = navigationListStackView.currentItem.parentId
//            console.log("priv.currentProjectId", priv.currentProjectId)
//            console.log("priv.currentParentId", priv.currentParentId)
//            console.log("priv.currentTreeItemId", priv.currentTreeItemId)

            rootWindow.protectedSignals.setBreadcrumbCurrentTreeItemCalled(
                        priv.currentProjectId, priv.currentParentId)
            priv.selecting = false
        }
    }
    function determineIfGoUpButtonEnabled() {

        if (!root.visible) {
            goUpAction.enabled = false
            return
        }

        goUpAction.enabled = (root.currentParentId !== -2)
    }
    goUpToolButton.onPressAndHold: {
        navigationListStackView.pop(null, priv.transitionOperation)
        navigationListStackView.currentItem.setCurrent()
        priv.selecting = false
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
        if (navigationMenu.visible) {
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
            onTriggered: {
                console.log("paste action", currentProjectId, currentParentId)
                skrData.treeHub().paste(currentProjectId, currentParentId)
            }
        }
        SkrMenu {

            title: qsTr("Advanced")
            Action {
                text: qsTr("Sort alphabetically")
                icon.source: "qrc:///icons/backup/view-sort-ascending-name.svg"
                enabled: currentParentId !== -2

                onTriggered: {
                    var result = skrData.treeHub().sortAlphabetically(
                                currentProjectId, currentParentId)
                    if (result.isSuccess()) {
                        console.log("sorted")
                    }
                }
            }
        }

        MenuSeparator {}

        Action {
            text: qsTr("Trash")
            //shortcut: StandardKey.Paste
            icon.source: "qrc:///icons/backup/edit-delete.svg"
            onTriggered: showTrashedList()
        }
    }

    //----------------------------------------------------------------------------
    //------------------ select button :------------------------------------------
    //----------------------------------------------------------------------------
    Action {
        id: selectTreeItemAction
        text: qsTr("Select")
        enabled: root.enabled && currentParentId !== -2
        checkable: true
        icon {
            source: "qrc:///icons/backup/dialog-ok-apply.svg"
            height: 100
            width: 100
        }
        onCheckedChanged: {
            priv.selecting = selectTreeItemAction.checked
        }
    }

    selectToolButton.action: selectTreeItemAction

    //----------------------------------------------------------------------------
    //------------------ add button :---------------------------------------------
    //----------------------------------------------------------------------------
    Action {
        id: addTreeItemAction
        text: qsTr("Add")
        shortcut: "Ctrl+T"
        enabled: root.enabled && currentParentId !== -2
        icon {
            source: "qrc:///icons/backup/list-add.svg"
            height: 100
            width: 100
        }
        onTriggered: {
            addItemMenu.open()
        }
    }

    addToolButton.action: addTreeItemAction

    function addItemAtCurrentParent(type) {

        //console.log(currentProjectId, currentParentId, navigationListStackView.currentItem.visualModel.items.count)
        navigationListStackView.currentItem.proxyModel.addChildItem(
                    currentProjectId, currentParentId, type)
        navigationListStackView.currentItem.listView.positionViewAtEnd()
        navigationListStackView.currentItem.listView.currentItem.editName()
    }

    function getIconUrlFromPageType(type, projectId, treeItemId) {
        return skrTreeManager.getIconUrlFromPageType(type, projectId, treeItemId)
    }

    function getPageTypeText(type) {
        return skrTreeManager.getPageTypeText(type)
    }

    SkrMenu {
        id: addItemMenu
        y: addToolButton.height
        x: addToolButton.x

        Repeater {

            model: skrTreeManager.getPageTypeList(true)

            SkrMenuItem {
                text: getPageTypeText(modelData)
                property string type: modelData
                icon.source: getIconUrlFromPageType(modelData, -1, -1)
                onTriggered: {
                    if(skrQMLTools.isURLValid(skrTreeManager.getCreationParametersQmlUrlFromPageType(type))){
                        creationParameterDialog.pageType = type
                        creationParameterDialog.open()
                    }
                    else{
                        addItemAtCurrentParent(type)
                    }
                }
            }
        }
    }

    //----------------------------------------------------------------------------



    SimpleDialog {
        id: creationParameterDialog
        property string pageType: ""
        title: qsTr("New item's parameters")

        contentItem: Loader{
            id: loader
            source: skrTreeManager.getCreationParametersQmlUrlFromPageType(creationParameterDialog.pageType)
        }


        standardButtons: Dialog.Ok | Dialog.Cancel

        onRejected: {
            creationParameterDialog.pageType = ""
        }

        onDiscarded: {

            creationParameterDialog.pageType = ""
        }

        onAccepted: {
            addItemAtCurrentParent(creationParameterDialog.pageType)

            creationParameterDialog.pageType = ""
        }

        onActiveFocusChanged: {
            if (activeFocus) {
                contentItem.forceActiveFocus()
            }
        }

        onOpened: {
            contentItem.forceActiveFocus()
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

    Timer {
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
    Component {
        id: stackViewComponent

        Item {
            id: stackViewBaseItem
            property alias listView: listView
            property alias visualModel: visualModel
            property var proxyModel: root.proxyModel.clone()
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

            focus: false

            Component.onCompleted: {
                init()
            }

            function init() {
                p_section.parentTitle = qsTr("Projects")
                listView.section.delegate = sectionHeading

                if (treeItemId < 0 && parentId >= -1) {
                    stackViewBaseItem.proxyModel.setParentFilter(projectId,
                                                                 parentId)
                } else {
                    stackViewBaseItem.proxyModel.setCurrentTreeItemId(
                                projectId, treeItemId)
                    parentId = stackViewBaseItem.proxyModel.parentIdFilter
                }
                determineSectionTitle()
            }

            Component.onDestruction: {
                delete proxyModel
            }

            function setCurrent() {
                priv.currentProjectId = projectId
                priv.currentParentId = parentId
                priv.currentTreeItemId = treeItemId

                listView.positionViewAtIndex(currentIndex, ListView.Contain)
            }

            onActiveFocusChanged: {
                if (activeFocus) {
                    listView.forceActiveFocus()
                }
            }

            function determineSectionTitle() {

                var projectId = stackViewBaseItem.projectId
                var parentId = stackViewBaseItem.parentId
                if (parentId === 0 && projectId !== -2) {
                    var projectTitle = skrData.projectHub().getProjectName(
                                projectId)

                    p_section.parentTitle = projectTitle
                    listView.section.delegate = sectionHeading
                } else if (parentId !== -2 && projectId !== -2) {
                    var parentTitle = proxyModel.getItemName(projectId,
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

            Item {
                id: focusZone
                anchors.bottom: parent.bottom
                anchors.left: parent.left
                anchors.right: parent.right
                height: listView.height - listView.contentHeight
                        > 0 ? listView.height - listView.contentHeight : 0
                z: 1

                TapHandler {
                    onSingleTapped: function(eventPoint) {
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
                        model: stackViewBaseItem.proxyModel
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


                    Binding {
                        target: listView
                        property: "currentIndex"
                        value: proxyModel.forcedCurrentIndex
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
                            property alias dropArea: dropArea
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

                            height: 40

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

                            Timer {
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

                            Timer {
                                id: labelTextFieldForceActiveFocusTimer
                                repeat: false
                                interval: 100
                                onTriggered: {
                                    labelTextField.forceActiveFocus()
                                }
                            }

                            Keys.priority: Keys.AfterItem

                            Keys.onShortcutOverride: function(event)  {
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
                                    openDocumentInAnotherViewAction.trigger()
                                    event.accepted = true
                                }
                                if (model.isOpenable
                                        && event.key === Qt.Key_Return
                                        && swipeDelegate.state !== "edit_name"
                                        && swipeDelegate.state !== "edit_label") {
                                    console.log("Return key pressed")
                                    openDocumentAction.trigger()
                                    event.accepted = true
                                }

                                // rename
                                if (model.isRenamable && event.key === Qt.Key_F2
                                        && swipeDelegate.state !== "edit_name"
                                        && swipeDelegate.state !== "edit_label") {
                                    renameAction.trigger()
                                    event.accepted = true
                                }

                                // cut
                                if (model.isMovable
                                        && (event.modifiers & Qt.ControlModifier)
                                        && event.key === Qt.Key_X
                                        && swipeDelegate.state !== "edit_name"
                                        && swipeDelegate.state !== "edit_label") {
                                    cutAction.trigger()
                                    event.accepted = true
                                }

                                // copy
                                if (model.isCopyable
                                        && (event.modifiers & Qt.ControlModifier)
                                        && event.key === Qt.Key_C
                                        && swipeDelegate.state !== "edit_name"
                                        && swipeDelegate.state !== "edit_label") {
                                    copyAction.trigger()
                                    event.accepted = true
                                }

                                // paste
                                if (model.canAddChildTreeItem
                                        && (event.modifiers & Qt.ControlModifier)
                                        && event.key === Qt.Key_V
                                        && swipeDelegate.state !== "edit_name"
                                        && swipeDelegate.state !== "edit_label") {
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
                                    moveUpAction.trigger()
                                    event.accepted = true
                                }

                                // move down
                                if (model.isMovable
                                        && (event.modifiers & Qt.ControlModifier)
                                        && event.key === Qt.Key_Down
                                        && swipeDelegate.state !== "edit_name"
                                        && swipeDelegate.state !== "edit_label") {
                                    moveDownAction.trigger()
                                    event.accepted = true
                                }

                                // send to trash
                                if (model.isTrashable
                                        && event.key === Qt.Key_Delete
                                        && swipeDelegate.state !== "edit_name"
                                        && swipeDelegate.state !== "edit_label") {
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
                                acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                                enabled: stackViewBaseItem.hoverEnabled

                                //grabPermissions:  PointerHandler.CanTakeOverFromItems | PointerHandler.CanTakeOverFromHandlersOfSameType | PointerHandler.ApprovesTakeOverByAnything
                                onHoveredChanged: {
                                    //console.log("item hovered", itemHoverHandler.hovered)
                                    if (priv.dragging || priv.selecting) {
                                        hoveringTimer.stop()
                                        closeSideNavigationListPopupTimer.popupId
                                                = stackViewBaseItem.popupId + 1
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
                                                    = stackViewBaseItem.popupId + 1
                                            closeSideNavigationListPopupTimer.start()
                                        }
                                    } else if (!model.hasChildren) {
                                        if (!addSideNavigationListPopupTimer.running) {
                                            hoveringTimer.stop()
                                            closeSideNavigationListPopupTimer.popupId
                                                    = stackViewBaseItem.popupId + 1
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
                                interval: 200
                                onTriggered: {

                                    addSideNavigationListPopupTimer.projectId = model.projectId
                                    addSideNavigationListPopupTimer.parentId = model.treeItemId
                                    addSideNavigationListPopupTimer.popupId
                                            = stackViewBaseItem.popupId
                                    addSideNavigationListPopupTimer.parentItem = swipeDelegate
                                    addSideNavigationListPopupTimer.listView = listView

                                    //                                            console.log("popupId", stackViewBaseItem.popupId)
                                    addSideNavigationListPopupTimer.start()
                                }
                            }

                            contentItem: DropArea {
                                id: dropArea

                                keys: ["application/skribisto-tree-item"]
                                onEntered: {

                                    //console.log("entered")
                                    //content.sourceIndex = drag.source.visualIndex
                                    visualModel.items.move(
                                                drag.source.visualIndex,
                                                content.visualIndex)
                                }
                                onExited: {

                                }
                                onDropped: {
                                    //                                console.log("dropped")
                                    if (drop.proposedAction === Qt.MoveAction) {

                                        //console.log("dropped from :", moveSourceInt, "to :", content.visualIndex)
                                        cancelDragTimer.stop()
                                        listView.interactive = true
                                        priv.dragging = false
                                        proxyModel.moveItem(moveSourceInt,
                                                            content.visualIndex)
                                        //proxyModel.moveItemById(moveSourceProjectId, moveSourceTreeItemId, content.treeItemId)
                                    }
                                }

                                SkrListItemPane {
                                    id: content
                                    property int visualIndex: model.index
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

                                    opacity: mouseDragHandler.active | touchDragHandler.active ? 0.2 : 1.0
                                    //sDrag.dragType: Drag.Internal
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
                                        acceptedDevices: PointerDevice.Mouse

                                        //xAxis.enabled: false
                                        //grabPermissions: PointerHandler.TakeOverForbidden
                                        onActiveChanged: {
                                            if (active) {
                                                Globals.touchUsed  = false
                                                listView.interactive = false
                                                moveSourceInt = content.visualIndex
                                                moveSourceTreeItemId = content.treeItemId
                                                moveSourceProjectId = content.projectId
                                                priv.dragging = true
                                                cancelDragTimer.stop()
                                            } else {
                                                cancelDragTimer.stop()
                                                priv.dragging = false
                                                content.dragging = false

                                                content.Drag.drop()
                                                proxyModel.invalidate()
                                            }
                                        }
                                        enabled: true

                                        onCanceled: {
                                            cancelDragTimer.stop()
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
                                                Globals.touchUsed  = true
                                                listView.interactive = false
                                                moveSourceInt = content.visualIndex
                                                moveSourceTreeItemId = content.treeItemId
                                                moveSourceProjectId = content.projectId
                                                priv.dragging = true
                                                cancelDragTimer.stop()
                                            } else {
                                                listView.interactive = true
                                                cancelDragTimer.stop()
                                                priv.dragging = false
                                                content.dragging = false
                                                content.Drag.drop()
                                                proxyModel.invalidate()
                                            }
                                        }
                                        enabled: content.dragging

                                        onCanceled: {
                                            cancelDragTimer.stop()
                                            priv.dragging = false
                                            content.dragging = false
                                        }
                                        grabPermissions: PointerHandler.CanTakeOverFromItems
                                                         | PointerHandler.CanTakeOverFromAnything
                                    }

                                    Timer {
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

                                        onSingleTapped: function(eventPoint) {
                                            priv.selecting = false
                                            //console.log("dragThreshold", mouseDragHandler.dragThreshold)

                                            if (content.dragging) {
                                                eventPoint.accepted = false
                                                return
                                            }
                                            if (eventPoint.event.device.type
                                                    === PointerDevice.Mouse) {
                                                Globals.touchUsed  = false
                                                listView.interactive = false
                                            }

                                            if (eventPoint.event.device.type
                                                    === PointerDevice.TouchScreen
                                                    | eventPoint.event.device.type
                                                    === PointerDevice.Stylus) {
                                                Globals.touchUsed  = true
                                                listView.interactive = true
                                            }

                                            listView.currentIndex = model.index
                                            priv.currentTreeItemId = model.treeItemId

                                            //console.log("tapped")
                                            if (skrData.treePropertyHub(
                                                        ).getProperty(
                                                        model.projectId,
                                                        model.treeItemId,
                                                        "can_add_child_paper",
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
                                            if (content.dragging) {
                                                eventPoint.accepted = false
                                                return
                                            }
                                            if (eventPoint.event.device.type
                                                    === PointerDevice.Mouse) {
                                                Globals.touchUsed  = false
                                                listView.interactive = false
                                            }

                                            if (eventPoint.event.device.type
                                                    === PointerDevice.TouchScreen
                                                    | eventPoint.event.device.type
                                                    === PointerDevice.Stylus) {
                                                Globals.touchUsed  = true
                                                listView.interactive = true
                                            }
                                            //console.log("double tapped")
                                            listView.currentIndex = model.index
                                            priv.currentTreeItemId = model.treeItemId

                                            openDocumentTimer.start()

                                            eventPoint.accepted = true
                                        }

                                        onGrabChanged: function(transition, point) {
                                            point.accepted = false
                                        }

                                        grabPermissions: PointerHandler.TakeOverForbidden
                                    }
                                    Timer {
                                        id: openDocumentTimer
                                        interval: 150
                                        onTriggered: {
                                            openDocumentAction.trigger()
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
                                        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                                        acceptedButtons: Qt.RightButton
                                        onSingleTapped: function(eventPoint) {
                                            listView.interactive = eventPoint.event.device.type
                                                    === PointerDevice.Mouse
                                            Globals.touchUsed  = false

                                            if (menu.visible) {
                                                menu.close()
                                                return
                                            }

                                            listView.currentIndex = model.index
                                            priv.currentTreeItemId = model.treeItemId


                                            menu.index = model.index
                                            menu.treeItemId = model.treeItemId
                                            menu.projectId = model.projectId
                                            menu.isOpenable = model.isOpenable
                                            menu.canAddChildTreeItem = model.canAddChildTreeItem
                                            menu.canAddSiblingTreeItem = model.canAddSiblingTreeItem
                                            menu.isCopyable = model.isCopyable
                                            menu.isMovable = model.isMovable
                                            menu.isRenamable = model.isRenamable
                                            menu.isTrashable = model.isTrashable



                                            menu.open()
                                            eventPoint.accepted = true
                                        }

                                        grabPermissions: PointerHandler.TakeOverForbidden
                                    }

                                    TapHandler {
                                        id: middleClickTapHandler
                                        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                                        acceptedButtons: Qt.MiddleButton
                                        onSingleTapped: function(eventPoint) {
                                            listView.interactive = eventPoint.event.device.type
                                                    === PointerDevice.Mouse
                                            Globals.touchUsed  = true
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
                                        onWheel: function(wheel) {
                                            listView.interactive = false
                                            listView.flick(
                                                        0,
                                                        wheel.angleDelta.y * 50)
                                            wheel.accepted = true
                                        }

                                        enabled: listView.interactive === false
                                    }

                                    TapHandler {
                                        acceptedDevices: PointerDevice.TouchScreen
                                                         | PointerDevice.Stylus

                                        onLongPressed: {
                                            Globals.touchUsed  = true

                                            // needed to activate the grab handler

                                            //                        if(content.dragging){
                                            //                            eventPoint.accepted = false
                                            //                            return
                                            //                        }
                                            content.dragging = true
                                            listView.interactive = false
                                            cancelDragTimer.start()
                                            priv.selecting = false
                                        }
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

                                            //                                        goToChildActionToBeTriggered = true
                                            //                                        goToChildActionCurrentIndent =  model.indent

                                            //                                        var _proxyModel = proxyModel

                                            //save current ids:
                                            listView.currentIndex = model.index
                                            navigationListStackView.currentItem.treeItemId
                                                    = model.treeItemId

                                            sidePopupListModel.clear()

                                            // push new view
                                            var newItem = navigationListStackView.push(
                                                        stackViewComponent, {
                                                            "projectId": model.projectId,
                                                            "parentId": model.treeItemId
                                                        },
                                                        priv.transitionOperation)
                                            newItem.setCurrent()
                                            newItem.listView.currentIndex = 0
                                            newItem.forceActiveFocus()

                                            rootWindow.protectedSignals.setBreadcrumbCurrentTreeItemCalled(
                                                        priv.currentProjectId,
                                                        priv.currentParentId)
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
                                            var index = proxyModel.findVisualIndex(
                                                        model.projectId,
                                                        treeItemIdToEdit)
                                            if (index !== -2) {
                                                listView.itemAtIndex(
                                                            index).editName()
                                            }
                                            treeItemIdToEdit = -2
                                        }
                                    }

                                    Action {
                                        id: openDocumentAction
                                        //shortcut: "Return"
                                        enabled: {
                                            if (!model.isOpenable) {
                                                return false
                                            }

                                            if (listView.enabled
                                                    && titleTextField.visible === false
                                                    && listView.currentIndex === model.index) {
                                                return true
                                            } else
                                                return false
                                        }

                                        text: qsTr("Open document")
                                        onTriggered: {
                                            //console.log("model.openedProjectId", openedProjectId)
                                            //console.log("model.projectId", model.projectId)
                                            root.openDocument(openedProjectId,
                                                              openedTreeItemId,
                                                              model.projectId,
                                                              model.treeItemId)
                                        }
                                    }

                                    Action {
                                        id: openDocumentInAnotherViewAction
                                        //shortcut: "Alt+Return"
                                        enabled: {
                                            if (!model.isOpenable) {
                                                return false
                                            }
                                            if (listView.enabled
                                                    && titleTextField.visible === false
                                                    && listView.currentIndex === model.index) {
                                                return true
                                            } else
                                                return false
                                        }

                                        text: qsTr("Open document in a new tab")
                                        onTriggered: {
                                            root.openDocumentInAnotherView(
                                                        model.projectId,
                                                        model.treeItemId)
                                        }
                                    }

                                    Action {
                                        id: openDocumentInNewWindowAction
                                        //shortcut: "Alt+Return"
                                        enabled: {
                                            if (!model.isOpenable) {
                                                return false
                                            }
                                            if (listView.enabled
                                                    && titleTextField.visible === false
                                                    && listView.currentIndex === model.index) {
                                                return true
                                            } else
                                                return false
                                        }

                                        text: qsTr("Open document in a window")
                                        onTriggered: {
                                            root.openDocumentInNewWindow(
                                                        model.projectId,
                                                        model.treeItemId)
                                            // solve bug where this window's tree disapears
                                            setCurrentTreeItemIdTimer.projectId = model.projectId
                                            setCurrentTreeItemIdTimer.treeItemId = model.treeItemId

                                            setCurrentTreeItemIdTimer.start()
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
                                                color: "lightsteelblue"
                                                Layout.fillHeight: true
                                                Layout.preferredWidth: 5
                                                visible: listView.currentIndex === model.index
                                            }
                                            Rectangle {
                                                id: openedItemIndicator
                                                color: SkrTheme.accent
                                                Layout.fillHeight: true
                                                Layout.preferredWidth: 5
                                                visible: model.projectId === openedProjectId
                                                         && model.treeItemId === openedTreeItemId
                                            }

                                            SkrToolButton {
                                                id: projectIsBackupIndicator
                                                visible: model.projectIsBackup
                                                         && model.type === 'PROJECT'
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

                                            SkrCheckBox {
                                                id: selectionCheckBox
                                                visible: priv.selecting
                                                implicitHeight: 32
                                                implicitWidth: 32
                                                Layout.maximumHeight: 30
                                                padding: 0
                                                rightPadding: 0
                                                bottomPadding: 0
                                                leftPadding: 0
                                                topPadding: 0
                                                focusPolicy: Qt.NoFocus

                                                onCheckStateChanged: {
                                                    model.checkState = selectionCheckBox.checkState
                                                    determineSelectedTreeItems()
                                                }

                                                Binding on checkState {
                                                    value: model.checkState
                                                    delayed: true
                                                    restoreMode: Binding.RestoreBindingOrValue
                                                }

                                                function determineSelectedTreeItems() {
                                                    priv.selectedTreeItemsIds = []
                                                    priv.selectedTreeItemsIds
                                                            = proxyModel.getCheckedIdsList()
                                                    priv.selectedProjectId = currentProjectId

                                                    console.log(selectedTreeItemsIds)
                                                }
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
                                                    implicitHeight: 36
                                                    implicitWidth: 36
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
                                                        source: getIconUrlFromPageType(
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
                                                            acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
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
                                                Layout.preferredWidth: 30

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




                                                    menu.index = model.index
                                                    menu.treeItemId = model.treeItemId
                                                    menu.projectId = model.projectId
                                                    menu.isOpenable = model.isOpenable
                                                    menu.canAddChildTreeItem = model.canAddChildTreeItem
                                                    menu.canAddSiblingTreeItem = model.canAddSiblingTreeItem
                                                    menu.isCopyable = model.isCopyable
                                                    menu.isMovable = model.isMovable
                                                    menu.isRenamable = model.isRenamable
                                                    menu.isTrashable = model.isTrashable



                                                    menu.open()
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
                                                orientation: Qt.Horizontal
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
                                SkrMenu {
                                    id: menu
                                    y: menuButton.height
                                    width: 300
                                    z: 101
                                    closePolicy: Popup.CloseOnPressOutside | Popup.CloseOnEscape

                                    property int index
                                    property int treeItemId
                                    property int projectId
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

                                            enabled: titleTextField.visible === false
                                                     && listView.enabled
                                            onTriggered: {
                                                console.log("open treeItem action",
                                                             menu.projectId,
                                                             menu.treeItemId)
                                                openDocumentAction.trigger()
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
                                            enabled: titleTextField.visible === false
                                                     && listView.enabled
                                            onTriggered: {
                                                console.log("open treeItem in another view action",
                                                             menu.projectId,
                                                             menu.treeItemId)
                                                openDocumentInAnotherViewAction.trigger()
                                            }
                                        }
                                    }

                                    SkrMenuItem {
                                        height: ! menu.isOpenable ? 0 : undefined
                                        visible:  menu.isOpenable

                                        action: Action {
                                            id: openTreeItemInNewWindowAction
                                            text: qsTr("Open in new window")
                                            //shortcut: "Alt+Return"
                                            icon {
                                                source: "qrc:///icons/backup/window-new.svg"
                                            }
                                            enabled: titleTextField.visible === false
                                                     && listView.enabled
                                            onTriggered: {
                                                console.log("open treeItem in new window action",
                                                             menu.projectId,
                                                             menu.treeItemId)
                                                // solve bug where this window's tree disapears
                                                setCurrentTreeItemIdTimer.projectId
                                                        =  menu.projectId
                                                setCurrentTreeItemIdTimer.treeItemId
                                                        =  menu.treeItemId
                                                setCurrentTreeItemIdTimer.start(
                                                            )
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
                                            //shortcut: "F2"
                                            icon {
                                                source: "qrc:///icons/backup/edit-rename.svg"
                                            }
                                            enabled: listView.enabled

                                            onTriggered: {
                                                console.log("rename action",
                                                             menu.projectId,
                                                             menu.treeItemId)
                                                swipeDelegate.editName()
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
                                                swipeDelegate.editLabel()
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

                                                if (selectedTreeItemsIds.length > 0) {
                                                    console.log("cut action",
                                                                 menu.projectId,
                                                                selectedTreeItemsIds)
                                                    skrData.treeHub().cut(
                                                                 menu.projectId,
                                                                selectedTreeItemsIds)
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
                                                if (selectedTreeItemsIds.length > 0) {
                                                    console.log("copy action",
                                                                 menu.projectId,
                                                                selectedTreeItemsIds)
                                                    skrData.treeHub().copy(
                                                                 menu.projectId,
                                                                selectedTreeItemsIds)
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
                                                             menu.treeItemId)

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
                                                    proxyModel.addItemAbove(
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
                                                    proxyModel.addItemBelow(
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
                                                listView.currentIndex =  menu.index
                                                navigationListStackView.currentItem.treeItemId
                                                        =  menu.treeItemId

                                                newItemPopup.projectId =  menu.projectId
                                                newItemPopup.treeItemId =  menu.treeItemId
                                                newItemPopup.visualIndex = 0
                                                newItemPopup.createFunction
                                                        = afterNewItemTypeIsChosen
                                                newItemPopup.open()
                                            }

                                            function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType, quantity) {

                                                // push new view
                                                var newItem = navigationListStackView.push(
                                                            stackViewComponent,
                                                            {
                                                                "projectId": projectId,
                                                                "parentId": treeItemId
                                                            },
                                                            priv.transitionOperation)
                                                newItem.setCurrent()

                                                for(var i = 0; i < quantity ; i++){
                                                    addItemAtCurrentParent(pageType)
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

                                                proxyModel.moveUp(
                                                             menu.projectId,
                                                             menu.treeItemId,
                                                             menu.index)
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

                                                proxyModel.moveDown(
                                                             menu.projectId,
                                                             menu.treeItemId,
                                                             menu.index)
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

                                                swipeDelegate.swipe.close()
                                                removeTreeItemAnimation.start()
                                                proxyModel.trashItemWithChildren(
                                                             menu.projectId,
                                                             menu.treeItemId)
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
                                        parent: Overlay.overlay
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

    Timer {
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
            enableBehavior: false

            width: 200
            z: 100 - popupId
            modal: false
            closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutsideParent
            padding: 0
            // start values:
            x: navigationListStackView.width + sideNavigationPopup.width * popupId
            y: sidePopupListModel.get(
                   popupId - 1) === undefined ? 0 : sidePopupListModel.get(
                                                    popupId - 1).y

            Binding {
                target: sideNavigationPopup
                property: "height"
                value: sideNavigationLoader.item.listView.contentHeight
                       > navigationListStackView.height ? navigationListStackView.height : sideNavigationLoader.item.listView.contentHeight
            }

            Binding {
                target: sideNavigationPopup
                property: "headerHeight"
                value: sideNavigationLoader.item.listView.contentHeight
                       - sideNavigationLoader.item.listView.count * 40
            }

            //-----------------------------------
            Loader {
                id: sideNavigationLoader
                anchors.fill: parent
                asynchronous: true
                sourceComponent: stackViewComponent
                visible: status === Loader.Ready

                onStatusChanged: {
                    if (status === Loader.Ready) {
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

            Binding {
                when: sidePopupLoader.status === Loader.Ready
                target: sidePopupLoader.item
                property: "popupId"
                value: model.popupId
            }

            Binding {
                when: sidePopupLoader.status === Loader.Ready
                target: sidePopupLoader.item
                property: "projectId"
                value: model.projectId
            }

            Binding {
                when: sidePopupLoader.status === Loader.Ready
                target: sidePopupLoader.item
                property: "parentId"
                value: model.parentId
            }

            Binding {
                when: sidePopupLoader.status === Loader.Ready
                target: sidePopupLoader.item
                property: "y"
                value: mapFromItem(
                           model.listView, 0,
                           model.parentItem.y).y - sidePopupLoader.item.headerHeight
            }
            Binding {
                when: sidePopupLoader.status === Loader.Ready
                target: model
                property: "y"
                value: sidePopupLoader.item.y
            }
            Binding {
                when: sidePopupLoader.status === Loader.Ready
                target: sidePopupLoader.item
                property: "x"
                value: navigationListStackView.width + sidePopupLoader.item.width * (model.index)
            }
            onStatusChanged: {
                if (status === Loader.Ready) {
                    sidePopupLoader.item.visible = true
                }
            }
        }
    }

    Timer {
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
            for (var i = sidePopupListModel.count - 1; i > popupId; i--) {
                sidePopupListModel.remove(i)
            }

            // add
            sidePopupListModel.append({
                                          "popupId": popupId + 1,
                                          "projectId": projectId,
                                          "parentId": parentId,
                                          "hovered": false,
                                          "parentItem": parentItem,
                                          "listView": listView,
                                          "y": 0
                                      })
            //console.log("sidePopupListModel.count", sidePopupListModel.count)

            //            if(sideNavigationPopup.visible){
            //                sideNavigationPopup.close()
            //            }

            //            sideNavigationPopup.open()
        }
    }

    Timer {
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
            for (var k = sidePopupListModel.count - 1; k > popupId; k--) {
                sidePopupListModel.remove(k)
            }
            if (popupId === -1) {
                sidePopupListModel.clear()
            }
        }
    }

    function determineIfSideNavigationPopupsHaveToClose() {
        if (itemHoverHandler.hovered) {
            closeSideNavigationListPopupTimer.stop()
        } else {
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
            if (!priv.renaming) {
                navigationListStackView.currentItem.forceActiveFocus()
            }
        }
    }
}
