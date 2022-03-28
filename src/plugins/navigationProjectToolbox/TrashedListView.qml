import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQml.Models
import eu.skribisto.projecthub 1.0
import QtQuick.Controls.Material
import eu.skribisto.searchtreelistproxymodel 1.0
import "../../../../Commons"
import "../../../../Items"
import "../../../.."

TrashedListViewForm {
    id: root

    SKRSearchTreeListProxyModel {
            id: trashedTreeProxyModel
            showTrashedFilter: true
            showNotTrashedFilter: false
        }


    property var model
    onModelChanged: {
        listView.visualModel.model = model
        trashedTreeProxyModel.projectIdFilter = currentProjectId
    }
    signal openDocument(int openedProjectId, int openedTreeItemId, int projectId, int treeItemId)
    signal openDocumentInAnotherView(int projectId, int treeItemId)
    signal openDocumentInNewWindow(int projectId, int treeItemId)
    signal restoreDocument(int projectId, int treeItemId)



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

    Component.onCompleted: {
        trashProjectComboBox.textRole = "name"
        trashProjectComboBox.valueRole = "projectId"
        populateProjectComboBoxModel()

        listView.openDocument.connect(root.openDocument)
        listView.openDocumentInAnotherView.connect(root.openDocumentInAnotherView)
        listView.openDocumentInNewWindow.connect(root.openDocumentInNewWindow)
        listView.escapeKeyPressed.connect( function() {goBackAction.trigger()})
        listView.deleteDefinitivelyCalled.connect(root.prepareDeleteDefinitivelyDialog)

        listView.proxyModel = trashedTreeProxyModel
        listView.currentProjectId = currentProjectId
        trashedTreeProxyModel.projectIdFilter = currentProjectId
        listView.currentIndex = currentIndex

    }

    //-----------------------------------------------------------------------------
    // project comboBox :



    trashProjectComboBox.model: ListModel {
        id: projectComboBoxModel
    }


    Connections {

        target: skrData.projectHub()
        function onProjectLoaded(projectId){


            var name =  skrData.projectHub().getProjectName(projectId)

            projectComboBoxModel.append({projectId: projectId, name: name})

            trashProjectComboBox.currentIndex = trashProjectComboBox.count -1
        }
    }



    trashProjectComboBox.displayText: qsTr("Trash: %1").arg(trashProjectComboBox.currentText)

    function populateProjectComboBoxModel(){

        projectComboBoxModel.clear()

        // populate

        var projectList = skrData.projectHub().getProjectIdList()

        var i;
        for(i = 0 ; i < projectList.length ; i++ ){
            var projectId = projectList[i]

            var name =  skrData.projectHub().getProjectName(projectId)

            projectComboBoxModel.append({projectId: projectId, name: name})
            //console.log("projectList")

        }

        // select last :
        if (projectList.length > 0){
            trashProjectComboBox.currentIndex = projectList.length - 1;
            var _projectId = trashProjectComboBox.valueAt(projectList.length - 1)
            trashedTreeProxyModel.projectIdFilter = _projectId
            currentProjectId = _projectId
        }
    }
    trashProjectComboBox.onCurrentIndexChanged: {
        //console.log("onCurrentIndexChanged")

        if(trashProjectComboBox.currentValue === undefined){

            var projectList = skrData.projectHub().getProjectIdList()
            trashProjectComboBox.currentIndex = projectList.length - 1;
            var _projectId = trashProjectComboBox.valueAt(projectList.length - 1)
            trashedTreeProxyModel.projectIdFilter = _projectId
            currentProjectId = _projectId
        }
        else {
            trashedTreeProxyModel.projectIdFilter = trashProjectComboBox.currentValue
            currentProjectId = trashProjectComboBox.currentValue

        }


    }


    //-----------------------------------------------------------------------------
    // go back button :
    signal goBack()

    Connections {

        target: skrData.projectHub()
        function onProjectClosed(projectId){

            goBack()

        }
    }


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


    //----------------------------------------------------------------------------
    listMenuToolButton.icon.source: "qrc:///icons/backup/overflow-menu.svg"
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
    // restore button :

    Action {
        id: restoreAction
        text: qsTr("Restore")
        //shortcut: ""
        enabled: listView.focus === true
        icon{
            source: "qrc:///icons/backup/edit-undo.svg"
            height: 100
            width: 100
        }
        onTriggered: {

            console.log('restore', currentProjectId, currentTreeItemId)
            restoreDocument(currentProjectId, currentTreeItemId)

        }
    }

    restoreToolButton.action: restoreAction

    //----------------------------------------------------------------------------

    // shortcuts



    Shortcut {
        enabled: listView.enabled
        sequences: ["Left", "Backspace"]
        onActivated: goBackAction.trigger()
        //enabled: listView.activeFocus
    }
    //-----------------------------------------------------------------------------

    property int contextMenuItemIndex: -2

    Binding {
        target: listView
        property: "currentIndex"
        value: trashedTreeProxyModel.forcedCurrentIndex
    }

    //----------------------------------------------------------------------------

    // focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            listView.forceActiveFocus()
        }
    }

    //---------------------------------------------------------------------------

}
