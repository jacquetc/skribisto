import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ".."

NavigationForm {
    id: root
    property int minimumHeight: 300

    property var navigationListProxyModel
    property var trashedListViewProxyModel
    property var restoreListViewProxyModel
    property int openedProjectId
    property int openedTreeItemId

    signal openDocument(int openedProjectId, int openedTreeItemId, int projectId, int treeItemId)
    signal openDocumentInAnotherView(int projectId, int treeItemId)
    signal openDocumentInNewWindow(int projectId, int treeItemId)
    signal restoreDocumentList(int projectId, var treeItemIdList)

    function setNavigationTreeItemId(projectId, treeItemId){
         stackView.get(0, StackView.DontLoad).setCurrentTreeItemId(projectId, treeItemId)
    }

    function setNavigationTreeItemParentId(projectId, treeItemParentId){
         stackView.get(0, StackView.DontLoad).setCurrentTreeItemParentId(projectId, treeItemParentId)
    }

    Component {
        id: navigationListComponent

        NavigationList {
            id: navigationList
            proxyModel: root.navigationListProxyModel
            openedProjectId: root.openedProjectId
            openedTreeItemId: root.openedTreeItemId

            Component.onCompleted: {
                navigationList.openDocument.connect(root.openDocument)
                navigationList.openDocumentInAnotherView.connect(root.openDocumentInAnotherView)
                navigationList.openDocumentInNewWindow.connect(root.openDocumentInNewWindow)
                navigationList.showTrashedList.connect(root.pushTrashedListView)


            }

        }
    }

    //-----------------------------------------------------------------------
    //------list of trashed items----------------------------------------------------
    //-----------------------------------------------------------------------

    function pushTrashedListView() {
        stackView.push(trashedListViewComponent)
    }

    function popTrashedListView() {
        //console.log("popTrashedListView")
        stackView.pop()
    }

    Component {
        id:trashedListViewComponent

        TrashedListView {
            id: trashedListView
            proxyModel: root.trashedListViewProxyModel

            Component.onCompleted: {
                trashedListView.openDocument.connect(root.openDocument)
                trashedListView.openDocumentInAnotherView.connect(root.openDocumentInAnotherView)
                trashedListView.openDocumentInNewWindow.connect(root.openDocumentInNewWindow)
                trashedListView.restoreDocument.connect(root.defineWhichPossibleDocumentsToRestore)
                trashedListView.goBack.connect(root.popTrashedListView)

            }
        }
    }


    function defineWhichPossibleDocumentsToRestore(projectId, treeItemId){


        // if get children :
        var trashedChildrenList = trashedListViewProxyModel.getChildrenList(projectId, treeItemId, true, false)
        var trashedAncestorsList = trashedListViewProxyModel.getAncestorsList(projectId, treeItemId, true, false)
        // if no children :
        if(trashedChildrenList.length === 0 && trashedAncestorsList === 0){
            restoreDocumentList(projectId, [treeItemId])
            return
        }
        else {

            pushRestoreListView(projectId, treeItemId, trashedChildrenList.concat(trashedAncestorsList))
        }



    }


    //-----------------------------------------------------------------------
    //--------RestoreListView-----------------------------------
    //-----------------------------------------------------------------------

    Component {
        id:restoreListViewComponent

        RestoreListView {
            id: restoreListView
            proxyModel: root.restoreListViewProxyModel

            Component.onCompleted: {
                restoreListView.openDocument.connect(root.openDocument)
                restoreListView.openDocumentInAnotherView.connect(root.openDocumentInAnotherView)
                restoreListView.openDocumentInNewWindow.connect(root.openDocumentInNewWindow)
                restoreListView.restoreDocumentList.connect(root.restoreDocumentList)
                restoreListView.goBack.connect(root.popRestoreListView)

            }
        }
    }

    function pushRestoreListView(projectId, parentTreeItemIdToBeRestored, trashedChildrenList) {

        trashedChildrenList.push(parentTreeItemIdToBeRestored)

        var treeIndentOffset = root.restoreListViewProxyModel.getItemIndent(projectId, parentTreeItemIdToBeRestored)

        stackView.push(restoreListViewComponent, {currentProjectId: projectId,
                           parentTreeItemIdToBeRestored: parentTreeItemIdToBeRestored,
                           trashedChildrenList: trashedChildrenList,
                           treeIndentOffset: treeIndentOffset
                       })
    }

    function popRestoreListView() {
        //console.log("popRestoreListView")
        stackView.pop()
        root.restoreListViewProxyModel.clearCheckedList()
    }




    //-----------------------------------------------------------------------
    //----------------------------------------------------------
    //-----------------------------------------------------------------------




    // focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            stackView.get(0, StackView.DontLoad).forceActiveFocus()
        }
    }


}
