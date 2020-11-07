import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import eu.skribisto.projecthub 1.0
import QtQuick.Controls.Material 2.15
import "../Items"
import ".."

TrashedListViewForm {
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
    signal restoreDocument(int projectId, int paperId)



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

    Component.onCompleted: {
        trashProjectComboBox.textRole = "name"
        trashProjectComboBox.valueRole = "projectId"
        populateProjectComboBoxModel()

        listView.openDocument.connect(root.openDocument)
        listView.openDocumentInNewTab.connect(root.openDocumentInNewTab)
        listView.openDocumentInNewWindow.connect(root.openDocumentInNewWindow)
        listView.escapeKeyPressed.connect( function() {goBackAction.trigger()})
        listView.deleteDefinitivelyCalled.connect(root.prepareDeleteDefinitivelyDialog)

        listView.proxyModel = proxyModel
        listView.currentProjectId = currentProjectId
        proxyModel.projectIdFilter = currentProjectId
        listView.currentIndex = currentIndex

    }

    //-----------------------------------------------------------------------------
    // project comboBox :



    trashProjectComboBox.model: ListModel {
        id: projectComboBoxModel
    }


    Connections {

        target: plmData.projectHub()
        function onProjectLoaded(projectId){


            var name =  plmData.projectHub().getProjectName(projectId)

            projectComboBoxModel.append({projectId: projectId, name: name})

            trashProjectComboBox.currentIndex = trashProjectComboBox.count -1
        }
    }



    trashProjectComboBox.displayText: qsTr("Trash: %1").arg(trashProjectComboBox.currentText)

    function populateProjectComboBoxModel(){

        projectComboBoxModel.clear()

        // populate

        var projectList = plmData.projectHub().getProjectIdList()

        var i;
        for(i = 0 ; i < projectList.length ; i++ ){
            var projectId = projectList[i]

            var name =  plmData.projectHub().getProjectName(projectId)

            projectComboBoxModel.append({projectId: projectId, name: name})
            console.log("projectList")

        }

        // select last :
        if (projectList.length > 0){
            trashProjectComboBox.currentIndex = projectList.length - 1;
            var _projectId = trashProjectComboBox.valueAt(projectList.length - 1)
            proxyModel.projectIdFilter = _projectId
            currentProjectId = _projectId
        }
    }
    trashProjectComboBox.onCurrentIndexChanged: {
        console.log("onCurrentIndexChanged")

        if(trashProjectComboBox.currentValue === undefined){

            var projectList = plmData.projectHub().getProjectIdList()
            trashProjectComboBox.currentIndex = projectList.length - 1;
            var _projectId = trashProjectComboBox.valueAt(projectList.length - 1)
            proxyModel.projectIdFilter = _projectId
            currentProjectId = _projectId
        }
        else {
            proxyModel.projectIdFilter = trashProjectComboBox.currentValue
            currentProjectId = trashProjectComboBox.currentValue

        }


    }


    //-----------------------------------------------------------------------------
    // go back button :
    signal goBack()

    Connections {

        target: plmData.projectHub()
        function onProjectClosed(projectId){

            goBack()

        }
    }


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
            icon.name: "edit-delete-shred"
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
            name: "edit-undo"
            height: 100
            width: 100
        }
        onTriggered: {

            console.log('restore', currentProjectId, currentPaperId)
            restoreDocument(currentProjectId, currentPaperId)

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
        value: proxyModel.forcedCurrentIndex
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
