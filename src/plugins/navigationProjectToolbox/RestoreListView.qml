import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQml.Models
import QtQuick.Controls.Material
import eu.skribisto.projecthub 1.0
import eu.skribisto.searchtreelistproxymodel 1.0
import "../../Commons"
import "../../Items"


RestoreListViewForm {

    id: root

    SKRSearchTreeListProxyModel {
            id: restoreTreeProxyModel
            showTrashedFilter: true
            showNotTrashedFilter: false
        }

    property var model
    onModelChanged: {
        listView.visualModel.model = model
        restoreTreeProxyModel.projectIdFilter = currentProjectId

    }
    signal openDocument(int openedProjectId, int openedTreeItemId, int projectId, int treeItemId)
    signal openDocumentInAnotherView(int projectId, int treeItemId)
    signal openDocumentInNewWindow(int projectId, int treeItemId)

    property var trashedChildrenList: []
    onTrashedChildrenListChanged: {
        restoreTreeProxyModel.treeItemIdListFilter = trashedChildrenList
    }

    property int parentTreeItemIdToBeRestored: -2
    property int treeIndentOffset: listView.treeIndentOffset


    property int currentProjectId: listView.currentProjectId
    property int currentTreeItemId: listView.currentTreeItemId


    property int currentIndex: listView.currentIndex
    property int openedProjectId: -2
    property int openedTreeItemId: -2
    property bool hoveringChangingTheCurrentItemAllowed: listView.hoveringChangingTheCurrentItemAllowed

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


    //-----------------------------------------------------------------------------

    Component.onCompleted: {

        listView.openDocument.connect(root.openDocument)
        listView.openDocumentInAnotherView.connect(root.openDocumentInAnotherView)
        listView.openDocumentInNewWindow.connect(root.openDocumentInNewWindow)
        listView.escapeKeyPressed.connect( function() {goBackAction.trigger()})
        listView.deleteDefinitivelyCalled.connect(root.prepareDeleteDefinitivelyDialog)

        listView.proxyModel = restoreTreeProxyModel
        listView.treeIndentOffset = treeIndentOffset
        listView.currentProjectId = currentProjectId
        restoreTreeProxyModel.projectIdFilter = currentProjectId
        listView.currentTreeItemId = parentTreeItemIdToBeRestored
        listView.currentIndex = currentIndex


        //pre-check same time trashed :
        var idList = restoreTreeProxyModel.findIdsTrashedAtTheSameTimeThan(currentProjectId, parentTreeItemIdToBeRestored)
        restoreTreeProxyModel.setCheckedIdsList(idList)

    }

    //-----------------------------------------------------------------------------
    // restore button :

    Action {
        id: restoreAction
        text: qsTr("Restore")
        //shortcut: ""
        icon{
            source: "qrc:///icons/backup/edit-undo.svg"
            height: 100
            width: 100
        }
        onTriggered: {
            var treeItemIdListToBeFinallyRestored = listView.getCheckedTreeItemIdList()
            // restore is difficult to explain : a restored parent will restore its children, even those trashed years before. To avoid that,
            // children trashed at the same minute will be checked to allow restore. The older ones will stay unchecked by default.
            // All that is done in RestoreView.qml
            var i
            for (i = 0; i < treeItemIdListToBeFinallyRestored.length; i++) {
                skrData.treeHub().untrashOnlyOneTreeItem(currentProjectId,
                                                         treeItemIdListToBeFinallyRestored[i])
            }
            //console.log('finally restore list', currentProjectId, treeItemIdListToBeFinallyRestored)

        }
    }

    restoreToolButton.action: restoreAction

    //-----------------------------------------------------------------------------


    // go back button :


    Connections {

        target: skrData.projectHub()
        function onProjectClosed(projectId){

            goBack()

        }
    }


    signal goBack()

    goBackToolButton.action: Action {
        id: goBackAction
        text: "<"
        icon.source: "qrc:///icons/backup/go-previous.svg"
        onTriggered:{
            goBack()
        }
    }


    //----------------------------------------------------------------------------

    function prepareDeleteDefinitivelyDialog(projectId, treeItemId){

        var idList = [treeItemId]
        idList = idList.concat(skrData.treeHub().getAllChildren(projectId, treeItemId))

        //get names
        var nameList = []
        var i
        for(i = 0 ; i < idList.length ; i++){
            var id = idList[i]

            nameList.push(skrData.treeHub().getTitle(projectId, id))

        }

        deleteDefinitivelyDialog.projectId = projectId
        deleteDefinitivelyDialog.projectName = skrData.projectHub().getProjectName(projectId)
        deleteDefinitivelyDialog.treeItemIdList = idList
        deleteDefinitivelyDialog.paperNamesString = "\n- " + nameList.join("\n- ")
        deleteDefinitivelyDialog.open()

    }


    SimpleDialog {
        property int projectId: -2
        property string projectName: ""
        property var treeItemIdList: []
        property var paperNamesString: ""

        id: deleteDefinitivelyDialog
        title: "Warning"
        text: qsTr("Do you want to delete definitively the following documents from the \"%1\" project ?\n%2").arg(projectName).arg(paperNamesString)
        standardButtons: Dialog.Yes  |  Dialog.Cancel

        onRejected: {
            deleteDefinitivelyDialog.close()

        }

        onAccepted: {
            var i
            for(i = 0 ; i < treeItemIdList.length ; i++){
                var id = treeItemIdList[i]

                skrData.treeHub().removeTreeItem(projectId, id)
            }

        }




    }
    //----------------------------------------------------------------------------
    listMenuToolButton.icon.name: "overflow-menu"
    listMenuToolButton.onClicked: navigationMenu.open()

    SkrMenu {
        id: navigationMenu
        y: listMenuToolButton.height

        //        Action {
        //            text: qsTr("Rename")
        //        }

        //        MenuSeparator {}
        //        Action {
        //            text: qsTr("Remove")
        //        }
        //        MenuSeparator {}

        Action {
            text: qsTr("Empty the trash")
            enabled: navigationMenu.opened
            //shortcut: "Ctrl+Shift+Del"
            icon.source: "qrc:///icons/backup/edit-delete-shred.svg"
            onTriggered: {
                //TODO: fill that
            }
        }

    }

    //----------------------------------------------------------------------------

    Action {
        id: selectAllAction
        text: selectAllAction.checked ? qsTr("Select none") : qsTr("Select all")
        //enabled: navigationMenu.opened
        //shortcut: "Ctrl+Shift+Del"
        icon.source: selectAllAction.checked ? "qrc:///icons/backup/edit-select-none.svg" : "qrc:///icons/backup/edit-select-all.svg"
        checkable: true
        onTriggered: {

            if(selectAllAction.checked){
                restoreTreeProxyModel.checkAll()
            }
            else {
                restoreTreeProxyModel.checkNone()

            }


        }
    }

    selectAllToolButton.action: selectAllAction


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
        enabled: listView.enabled
        sequences: ["Left", "Backspace"]
        onActivated: goBackAction.trigger()
        //enabled: listView.activeFocus
    }
    //-----------------------------------------------------------------------------


    Binding {
        target: listView
        property: "currentIndex"
        value: restoreTreeProxyModel.forcedCurrentIndex
    }

    //----------------------------------------------------------------------------

    // focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            listView.forceActiveFocus()
        }
    }

    //----------------------------------------------------------------------------





}
