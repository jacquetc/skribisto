import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts
import QtQml.Models
import QtQuick.Controls.Material
import eu.skribisto.searchtreelistproxymodel 1.0
import "../../Commons"
import "../../Items"
import "../.."

NavigationListForm {
    id: root





    signal openDocument(int projectId, int treeItemId)
    signal openDocumentInAnotherView(int projectId, int treeItemId)
    signal openDocumentInNewWindow(int projectId, int treeItemId)
    signal showTrashedList

    readonly property int currentParentId: priv.currentParentId
    readonly property int currentProjectId: priv.currentProjectId
    readonly property int currentTreeItemId: priv.currentTreeItemId

    readonly property alias selectedTreeItemsIds: priv.selectedTreeItemsIds
    readonly property alias selectedProjectId: priv.selectedProjectId

    QtObject {
        id: priv
        property int currentParentId: -2
        property int currentProjectId: -2
        property int currentTreeItemId: -2
        property bool dragging: false
        onDraggingChanged: if(dragging){
                               sidePopupListModel.clear()
                           }

        property bool renaming: false
        property bool selecting: false
        property bool animationEnabled: SkrSettings.ePaperSettings.animationEnabled
        property int transitionOperation: animationEnabled ? StackView.Transition : StackView.Immediate

        onSelectingChanged: {
            if (!selecting) {
                navigationListStackView.currentItem.checkNone()
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

    }


    Component.onCompleted: {
        var newItem = navigationListStackView.push("NavigationListView.qml", priv.transitionOperation)
        newItem.setCurrentTreeItemParentIdCalled.connect(setCurrentTreeItemParentId)
        newItem.setCurrentTreeItemIdCalled.connect(setCurrentTreeItemId)
        newItem.openDocument.connect(openDocument)
        newItem.openDocumentInAnotherView.connect(openDocumentInAnotherView)
        newItem.openDocumentInNewWindow.connect(openDocumentInNewWindow)

    }




    function setCurrentTreeItemParentId(projectId, treeItemParentId) {

        sidePopupListModel.clear()


        //find parent id
        if (projectId > -1 & treeItemParentId > -1) {
            var ancestorsList = skrData.treeHub().getAllAncestors(
                        projectId, treeItemParentId)
            var currentAncestorsList = skrData.treeHub().getAllAncestors(
                        projectId, root.currentParentId)
        }

        //compare with current parent id
        if (projectId === root.currentProjectId & treeItemParentId === root.currentParentId) {
            console.log("nothing to do")
        }
        else if (projectId < 0 & treeItemParentId < 0) {

            navigationListStackView.pop(null, priv.transitionOperation)
            //project item
            navigationListStackView.get(0).projectId = -2
            navigationListStackView.get(0).parentId = -2
            navigationListStackView.get(0).treeItemId = -2
            navigationListStackView.get(0).init()
            navigationListStackView.get(0).setCurrent()
        }
        else if (projectId > 0 && treeItemParentId >= 0 && root.currentParentId === ancestorsList[0]) {

            var newItem = navigationListStackView.push("NavigationListView.qml", {
                                                           "projectId": projectId,
                                                           "parentId": treeItemParentId
                                                       },
                                                       priv.transitionOperation)


            newItem.setCurrentTreeItemParentIdCalled.connect(setCurrentTreeItemParentId)
            newItem.setCurrentTreeItemIdCalled.connect(setCurrentTreeItemId)
            newItem.openDocument.connect(openDocument)
            newItem.openDocumentInAnotherView.connect(openDocumentInAnotherView)
            newItem.openDocumentInNewWindow.connect(openDocumentInNewWindow)
            newItem.init()
            newItem.setCurrent()
        }
        else {
            navigationListStackView.pop(null, priv.transitionOperation)
            ancestorsList.reverse()
            ancestorsList.push(treeItemParentId)

            //project item
            navigationListStackView.get(0).projectId = projectId
            navigationListStackView.get(0).parentId = -2
            navigationListStackView.get(0).treeItemId = 0
            navigationListStackView.get(0).init()
            navigationListStackView.get(0).setCurrent()

            for (var i = 1; i < ancestorsList.length; i++) {
                var newItem = navigationListStackView.push("NavigationListView.qml", {
                                                               "projectId": projectId,
                                                               "treeItemId": ancestorsList[i]
                                                           },
                                                           StackView.Immediate)


                newItem.setCurrentTreeItemParentIdCalled.connect(setCurrentTreeItemParentId)
                newItem.setCurrentTreeItemIdCalled.connect(setCurrentTreeItemId)
                newItem.openDocument.connect(openDocument)
                newItem.openDocumentInAnotherView.connect(openDocumentInAnotherView)
                newItem.openDocumentInNewWindow.connect(openDocumentInNewWindow)
                newItem.init()
                newItem.setCurrent()
            }

            var lastNewItem = navigationListStackView.push("NavigationListView.qml", {
                                                               "projectId": projectId,
                                                               "parentId": treeItemParentId
                                                           },
                                                           priv.transitionOperation)
            lastNewItem.setCurrentTreeItemParentIdCalled.connect(setCurrentTreeItemParentId)
            lastNewItem.setCurrentTreeItemIdCalled.connect(setCurrentTreeItemId)
            lastNewItem.openDocument.connect(openDocument)
            lastNewItem.openDocumentInAnotherView.connect(openDocumentInAnotherView)
            lastNewItem.openDocumentInNewWindow.connect(openDocumentInNewWindow)
            lastNewItem.init()
            lastNewItem.setCurrent()

        }
        rootWindow.protectedSignals.setBreadcrumbCurrentTreeItemCalled(
                    priv.currentProjectId, priv.currentParentId)
        determineIfGoUpButtonEnabled()
        priv.selecting = false
    }

    function setCurrentTreeItemId(projectId, treeItemId) {
        sidePopupListModel.clear()

        //find parent id
        var ancestorsList = skrData.treeHub().getAllAncestors(
                    projectId, treeItemId)

        var newParentId = ancestorsList[0]

        //compare with current parent id
        if (projectId === root.currentProjectId & newParentId === root.currentParentId) {
            navigationListStackView.currentItem.setCurrentTreeItemId(projectId, treeItemId)
        } //        else if(projectId === root.currentProjectId){

        //        }
        else {
            navigationListStackView.pop(null, priv.transitionOperation)
            ancestorsList.reverse()
            ancestorsList.push(treeItemId)

            //project item
            navigationListStackView.get(0).projectId = projectId
            navigationListStackView.get(0).treeItemId = 0
            navigationListStackView.get(0).init()
            navigationListStackView.get(0).setCurrent()

            for (var i = 1; i < ancestorsList.length; i++) {
                var transition = StackView.Immediate
                if(i === ancestorsList.length - 1){
                    transition = StackView.Transition
                }

                var newItem = navigationListStackView.push("NavigationListView.qml", {
                                                               "projectId": projectId,
                                                               "treeItemId": ancestorsList[i]
                                                           },
                                                           transition)
                newItem.setCurrentTreeItemParentIdCalled.connect(setCurrentTreeItemParentId)
                newItem.setCurrentTreeItemIdCalled.connect(setCurrentTreeItemId)
                newItem.openDocument.connect(openDocument)
                newItem.openDocumentInAnotherView.connect(openDocumentInAnotherView)
                newItem.openDocumentInNewWindow.connect(openDocumentInNewWindow)
                newItem.init()
                newItem.setCurrent()
            }
        }

        if(rootWindow.protectedSignals){
            rootWindow.protectedSignals.setBreadcrumbCurrentTreeItemCalled(
                        priv.currentProjectId, priv.currentParentId)
        }

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
            console.log("goUpAction triggered")

            navigationListStackView.pop(priv.transitionOperation)
            navigationListStackView.currentItem.setCurrent()

            //            if(navigationListStackView.currentItem.listView.currentItem){
            //                navigationListStackView.currentItem.listView.currentItem.forceActiveFocus()
            //            }

            //            var index = navigationListStackView.currentItem.listView.currentIndex
            //            var item = navigationListStackView.currentItem.listView.itemAtIndex(index)

            //            if (item) {
            //                item.forceActiveFocus()
            //            } else {
            //                navigationListStackView.currentItem.listView.forceActiveFocus()
            //            }
            priv.currentProjectId = navigationListStackView.currentItem.projectId
            priv.currentParentId = navigationListStackView.currentItem.parentId

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
    //------------------ select all button :------------------------------------------
    //----------------------------------------------------------------------------
    Action {
        id: selectAllTreeItemsAction
        text: selectAllTreeItemsAction.checked ? qsTr("Select none") : qsTr("Select all")
        enabled: root.enabled && currentParentId !== -2
        checkable: true
        icon {
            source: selectAllTreeItemsAction.checked ? "qrc:///icons/backup/edit-select-none.svg" : "qrc:///icons/backup/edit-select-all.svg"
        }
        onCheckedChanged: {
            if(selectAllTreeItemsAction.checked){
                navigationListStackView.currentItem.checkAll()
            }
            else{
                navigationListStackView.currentItem.checkNone()

            }
        }
    }

    selectAllToolButton.action: selectAllTreeItemsAction
    selectAllToolButton.visible: selectTreeItemAction.checked

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
        }
        onTriggered: {
            addItemMenu.open()
        }
    }

    addToolButton.action: addTreeItemAction

    function addItemAtCurrentParent(type) {

        //console.log(currentProjectId, currentParentId, navigationListStackView.currentItem.visualModel.items.count)

        skrData.treeHub().addChildTreeItem(currentProjectId, currentParentId, type)
        navigationListStackView.currentItem.listView.currentIndex = navigationListStackView.currentItem.listView.count - 1
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

    navigationListStackView.popEnter: Transition {
        onRunningChanged: {
            if(running)
                console.log("popEnter")
        }
        XAnimator {
            from: (navigationListStackView.mirrored ? -1 : 1) * -navigationListStackView.width
            to: 0
            duration: 200
            easing.type: Easing.OutCubic
        }
    }
    navigationListStackView.popExit: Transition {
        onRunningChanged: {
            if(running)
                console.log("popExit")
        }

        XAnimator {
            from: 0
            to: (navigationListStackView.mirrored ? -1 : 1) * navigationListStackView.width
            duration: 200
            easing.type: Easing.OutCubic
        }
    }


    navigationListStackView.pushEnter: Transition {
        onRunningChanged: {
            if(running)
                console.log("pushEnter")
        }
        XAnimator {
            from: (navigationListStackView.mirrored ? -1 : 1) * navigationListStackView.width
            to: 0
            duration: 200
            easing.type: Easing.OutCubic
        }
    }
    navigationListStackView.pushExit: Transition {
        onRunningChanged: {
            if(running)
                console.log("pushExit")
        }
        XAnimator {
            from: 0
            to: (navigationListStackView.mirrored ? -1 : 1) * -navigationListStackView.width
            duration: 200
            easing.type: Easing.OutCubic
        }
    }


    //-------------------------------------------------------------------------------------
    //---------side Navigation Popup---------------------------------------------------------
    //-------------------------------------------------------------------------------------
    Component {
        id: sideNavigationPopupComponent
        SkrPopup {
            id: sideNavigationPopup

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
                source: "NavigationListView.qml"
                visible: status === Loader.Ready

                onStatusChanged: {
                    if (status === Loader.Ready) {
                        //sideNavigationLoader.item.hoverEnabled  = false
                        sideNavigationLoader.item.openDocument.connect(openDocument)
                        sideNavigationLoader.item.openDocumentInAnotherView.connect(openDocumentInAnotherView)
                        sideNavigationLoader.item.openDocumentInNewWindow.connect(openDocumentInNewWindow)
                        sideNavigationLoader.item.setCurrentTreeItemParentIdCalled.connect(setCurrentTreeItemParentId)
                        sideNavigationLoader.item.setCurrentTreeItemIdCalled.connect(setCurrentTreeItemId)
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
