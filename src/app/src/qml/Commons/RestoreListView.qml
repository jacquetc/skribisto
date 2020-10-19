import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import QtQuick.Controls.Material 2.15
import eu.skribisto.projecthub 1.0


RestoreListViewForm {

    id: root

    property var proxyModel
    property var model
    onModelChanged: {
        listView.visualModel.model = model
        proxyModel.projectIdFilter = currentProjectId

    }
    signal openDocument(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openDocumentInNewTab(int projectId, int paperId)
    signal openDocumentInNewWindow(int projectId, int paperId)
    signal restoreDocumentList(int projectId, var paperIdList)

    property var trashedChildrenList: []
    onTrashedChildrenListChanged: {
        proxyModel.paperIdListFilter = trashedChildrenList
    }

    property int parentPaperIdToBeRestored: -2
    property int treeIndentOffset: listView.treeIndentOffset


    property int currentProjectId: listView.currentProjectId
    property int currentPaperId: listView.currentPaperId


    property int currentIndex: listView.currentIndex
    property int openedProjectId: -2
    property int openedPaperId: -2
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

    toolBarPrimaryColor: Material.color(Material.Cyan, Material.Shade200)

    //-----------------------------------------------------------------------------

    Component.onCompleted: {

        listView.openDocument.connect(root.openDocument)
        listView.openDocumentInNewTab.connect(root.openDocumentInNewTab)
        listView.openDocumentInNewWindow.connect(root.openDocumentInNewWindow)
        listView.goBackCalled.connect( function() {goBackAction.trigger()})
        listView.deleteDefinitivelyCalled.connect(root.prepareDeleteDefinitivelyDialog)

        listView.proxyModel = proxyModel
        listView.treeIndentOffset = treeIndentOffset
        listView.currentProjectId = currentProjectId
        proxyModel.projectIdFilter = currentProjectId
        listView.currentPaperId = parentPaperIdToBeRestored
        listView.currentIndex = currentIndex


        //pre-check same time trashed :
        var idList = proxyModel.findIdsTrashedAtTheSameTimeThan(currentProjectId, parentPaperIdToBeRestored)
        proxyModel.setCheckedIdsList(idList)

    }

    //-----------------------------------------------------------------------------
    // restore button :

    Action {
        id: restoreAction
        text: qsTr("Restore")
        //shortcut: ""
        icon{
            name: "edit-undo"
            height: 100
            width: 100
        }
        onTriggered: {
            var paperIdListToBeFinallyRestored = listView.getCheckedPaperIdList()

            restoreDocumentList(currentProjectId, paperIdListToBeFinallyRestored)
            console.log('finally restore list', currentProjectId, paperIdListToBeFinallyRestored)

        }
    }

    restoreToolButton.action: restoreAction

    //-----------------------------------------------------------------------------


    Connections {

        target: plmData.projectHub()
        function onProjectClosed(projectId){

            goBack()

        }
    }



    //-----------------------------------------------------------------------------
    // go back button :

    signal goBack()

    goBackToolButton.action: Action {
        id: goBackAction
        text: "<"
        icon.name: "go-previous"
        onTriggered:{
            goBack()
        }
    }


    //----------------------------------------------------------------------------

    function prepareDeleteDefinitivelyDialog(projectId, paperId){

        var idList = [paperId]
        idList = idList.concat(proxyModel.getChildrenList(projectId, paperId, true, false))

        //get names
        var nameList = []
        var i
        for(i = 0 ; i < idList.length ; i++){
            var id = idList[i]

            nameList.push(proxyModel.getItemName(projectId, id))

        }

        deleteDefinitivelyDialog.projectId = projectId
        deleteDefinitivelyDialog.projectName = plmData.projectHub().getProjectName(projectId)
        deleteDefinitivelyDialog.paperIdList = idList
        deleteDefinitivelyDialog.paperNamesString = "\n- " + nameList.join("\n- ")
        deleteDefinitivelyDialog.open()

    }


    SimpleDialog {
        property int projectId: -2
        property string projectName: ""
        property var paperIdList: []
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
            for(i = 0 ; i < paperIdList.length ; i++){
                var id = paperIdList[i]

                proxyModel.deleteDefinitively(projectId, id)
            }

        }




    }
    //----------------------------------------------------------------------------
    listMenuToolButton.icon.name: "overflow-menu"
    listMenuToolButton.onClicked: navigationMenu.open()

    Menu {
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
            icon.name: "edit-delete-shred"
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
        icon.name: selectAllAction.checked ? "edit-select-none" : "edit-select-all"
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
