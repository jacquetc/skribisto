import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import QtQml 2.15
import eu.skribisto.searchtreelistproxymodel 1.0
import eu.skribisto.writedocumentlistmodel 1.0
import "Items"
import "Commons"

LeftDockForm {
    id: root


    required property var viewManager
    //-----------------------------------------------------------
    //--------------- toolboxes Behavior------------------------
    //-----------------------------------------------------------

    Settings {
        id: settings
        category: "leftDock"

        property bool navigationViewVisible: true
        property bool documentViewVisible: true
    }

    function loadConf(){

        navigationViewToolButton.checked = settings.navigationViewVisible
        documentViewToolButton.checked = settings.documentViewVisible

    }

    function resetConf(){
        settings.navigationViewVisible = true
        settings.documentViewVisible = true
    }

    Component.onCompleted: {
        loadConf()
        initNavigationView()
        Globals.resetDockConfCalled.connect(resetConf)

    }

    Component.onDestruction: {
        settings.navigationViewVisible = navigationViewToolButton.checked
        settings.documentViewVisible = documentViewToolButton.checked

    }



    //-----------------------------------------------------------
    //------- Navigation List : ---------------------------------
    //-----------------------------------------------------------

    Action{
        id: navigationViewAction
        checkable: true
        text: qsTr( "Show navigation")
        icon {
            source: "qrc:///icons/backup/compass.svg"
            height: 50
            width: 50
        }
        onCheckedChanged: {
            navigationView.visible = navigationViewAction.checked
        }

        Binding on checked{
            value: navigationView.visible
            delayed: true
            restoreMode: Binding.RestoreBindingOrValue
        }
    }
    navigationViewToolButton.action: navigationViewAction

    //    Connections {
    //        target: navigationView
    //        function onOpenDocument(openedProjectId, openedPaperId, _projectId, _treeItemId) {
    //            Globals.openTreeInAnotherViewCalled(_projectId, _treeItemId)
    //        }
    //    }

    Connections {
        target: navigationView
        function onOpenDocument(openedProjectId, openedPaperId, _projectId, _treeItemId) {
            viewManager.loadTreeItem(_projectId, _treeItemId)
        }
    }
    Connections {
        target: navigationView
        function onOpenDocumentInAnotherView(_projectId, _treeItemId) {
            viewManager.loadTreeItemAtAnotherView(_projectId, _treeItemId)
        }
    }
    Connections {
        target: navigationView
        function onOpenDocumentInNewWindow(_projectId, _treeItemId) {
            skrWindowManager.addWindowForItemId(_projectId, _treeItemId)
        }
    }

    function initNavigationView(){
        //navigationView.openDocumentInAnotherView.connect(Globals.openTreeInAnotherViewCalled)
        //navigationView.openDocumentInNewWindow.connect(Globals.openTreeInNewWindowCalled)
        navigationView.restoreDocumentList.connect(root.restoreTreeItemList)
    }

    SKRSearchTreeListProxyModel {
        id: navigationProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
        navigateByBranchesEnabled: true
    }

    navigationView.navigationListProxyModel: navigationProxyModel

    Connections {
        target: plmData.projectHub()
        function onActiveProjectChanged(projectId){
            navigationProxyModel.projectIdFilter = projectId
            navigationProxyModel.parentIdFilter = -1
        }
    }


    SKRSearchTreeListProxyModel {
        id: trashedTreeProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false

    }
    navigationView.trashedListViewProxyModel: trashedTreeProxyModel

    SKRSearchTreeListProxyModel {
        id: restoreTreeProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false
    }
    navigationView.restoreListViewProxyModel: restoreTreeProxyModel





    function restoreTreeItemList(projectId, treeItemIdList){
        // restore is difficult to explain : a restored parent will restore its children, even those trashed years before. To avoid that,
        // children trashed at the same minute will be checked to allow restore. The older ones will stay unchecked by default.
        // All that is done in RestoreView.qml

        var i
        for(i = 0 ; i < treeItemIdList.length ; i++){
            plmData.treeHub().untrashOnlyOneTreeItem(projectId, treeItemIdList[i])
        }


        //console.log("restored: sheet:", sheetIdList)
    }

    Connections{
        target: rootWindow
        function onSetNavigationTreeItemIdCalled(projectId, treeItemId){
            navigationView.setNavigationTreeItemId(projectId, treeItemId)
        }
    }

    Connections{
        target: rootWindow
        function onSetNavigationTreeItemParentIdCalled(projectId, treeItemParentId){
            navigationView.setNavigationTreeItemParentId(projectId, treeItemParentId)
        }
    }


    Connections{
        target: viewManager
        function onFocusedChanged(position, projectId, treeItemId, pageType){
            if(projectId !== -1 && treeItemId !== -1){
            navigationView.setNavigationTreeItemId(projectId, treeItemId)
            }
        }
    }

    //----------------------------------------------------------

    Connections {
        target:plmData.projectHub()
        function onProjectLoaded(_projectId) {
            onProjectLoadedTimer.projectId = _projectId
            onProjectLoadedTimer.start()
        }

    }
    Timer {
        id: onProjectLoadedTimer
        property int projectId
        interval: 100
        onTriggered: navigationView.setNavigationTreeItemId(projectId, 0)

    }
    //----------------------------------------------------------

    //-----------------------------------------------------------
    //---------Document List : ----------------------------------
    //-----------------------------------------------------------

    Action{
        id: documentViewAction
        checkable: true
        text: qsTr( "Show recent sheets")
        icon {
            source: "qrc:///icons/backup/document-open-recent.svg"
            height: 50
            width: 50
        }
        onCheckedChanged: {
            documentView.visible = documentViewAction.checked
        }

        Binding on checked{
            value: documentView.visible
            delayed: true
            restoreMode: Binding.RestoreBindingOrValue
        }
    }
    documentViewToolButton.action: documentViewAction

    documentView.model: skrModels.writeDocumentListModel()
    documentView.documentModel: skrModels.writeDocumentListModel()





    //-----------------------------------------------------------
    //-----------------------------------------------------------
    //-----------------------------------------------------------

    signal hideDockCalled()
    hideDockToolButton.action: hideDockAction

    Action {
        id: hideDockAction
        icon.source: "qrc:///icons/backup/go-previous-view.svg"
        onTriggered: {
            hideDockCalled()
        }

    }


}
