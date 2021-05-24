import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import QtQuick.Controls.Material 2.15
import eu.skribisto.projecthub 1.0
import "../Items"


RestoreListViewForm {

    id: root

    property var proxyModel
    property var model
    onModelChanged: {
        listView.visualModel.model = model
        proxyModel.projectIdFilter = currentProjectId

    }
    signal openDocument(int openedProjectId, int openedTreeItemId, int projectId, int treeItemId)
    signal openDocumentInAnotherView(int projectId, int treeItemId)
    signal openDocumentInNewWindow(int projectId, int treeItemId)
    signal restoreDocumentList(int projectId, var treeItemIdList)

    property var trashedChildrenList: []
    onTrashedChildrenListChanged: {
        proxyModel.treeItemIdListFilter = trashedChildrenList
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

        listView.proxyModel = proxyModel
        listView.treeIndentOffset = treeIndentOffset
        listView.currentProjectId = currentProjectId
        proxyModel.projectIdFilter = currentProjectId
        listView.currentTreeItemId = parentTreeItemIdToBeRestored
        listView.currentIndex = currentIndex


        //pre-check same time trashed :
        var idList = proxyModel.findIdsTrashedAtTheSameTimeThan(currentProjectId, parentTreeItemIdToBeRestored)
        proxyModel.setCheckedIdsList(idList)

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

            restoreDocumentList(currentProjectId, treeItemIdListToBeFinallyRestored)
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
        idList = idList.concat(proxyModel.getChildrenList(projectId, treeItemId, true, false))

        //get names
        var nameList = []
        var i
        for(i = 0 ; i < idList.length ; i++){
            var id = idList[i]

            nameList.push(proxyModel.getItemName(projectId, id))

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

                proxyModel.deleteDefinitively(projectId, id)
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
                proxyModel.checkAll()
            }
            else {
                proxyModel.checkNone()

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
        value: proxyModel.forcedCurrentIndex
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
