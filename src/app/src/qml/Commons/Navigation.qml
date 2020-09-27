import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ".."

NavigationForm {
    id: root
    property int minimumHeight: 300

    property var treeListViewProxyModel
    property var trashedListViewProxyModel
    property var restoreListViewProxyModel
    property int openedProjectId
    property int openedPaperId


    signal openDocument(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openDocumentInNewTab(int projectId, int paperId)
    signal openDocumentInNewWindow(int projectId, int paperId)
    signal restoreDocumentList(int projectId, var paperIdList)

    Component {
        id: treeListViewComponent

        TreeListView {
            id: treeListView
            proxyModel: root.treeListViewProxyModel
            model: root.treeListViewProxyModel
            openedProjectId: root.openedProjectId
            openedPaperId: root.openedPaperId

            Component.onCompleted: {
                treeListView.openDocument.connect(root.openDocument)
                treeListView.openDocumentInNewTab.connect(root.openDocumentInNewTab)
                treeListView.openDocumentInNewWindow.connect(root.openDocumentInNewWindow)
                treeListView.showTrashedList.connect(root.pushTrashedListView)

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
        console.log("popTrashedListView")
        stackView.pop()
    }

    Component {
        id:trashedListViewComponent

        TrashedListView {
            id: trashedListView
            proxyModel: root.trashedListViewProxyModel
            model: root.trashedListViewProxyModel

            Component.onCompleted: {
                trashedListView.openDocument.connect(root.openDocument)
                trashedListView.openDocumentInNewTab.connect(root.openDocumentInNewTab)
                trashedListView.openDocumentInNewWindow.connect(root.openDocumentInNewWindow)
                trashedListView.restoreDocument.connect(root.defineWhichPossibleDocumentsToRestore)
                trashedListView.goBack.connect(root.popTrashedListView)

            }
        }
    }


    function defineWhichPossibleDocumentsToRestore(projectId, paperId){


        // if get children :
        var trashedChildrenList = trashedListViewProxyModel.getChildrenList(projectId, paperId, true, false)

        // if no children :
        if(trashedChildrenList.length === 0){
            restoreDocumentList(projectId, [paperId])
            return
        }
        else {

            pushRestoreListView(projectId, paperId, trashedChildrenList)
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
            model: root.restoreListViewProxyModel

            Component.onCompleted: {
                restoreListView.restoreDocumentList.connect(root.restoreDocumentList)
                restoreListView.goBack.connect(root.popRestoreListView)

            }
        }
    }

    function pushRestoreListView(projectId, parentPaperIdToBeRestored, trashedChildrenList) {

        trashedChildrenList.push(parentPaperIdToBeRestored)

        var treeIndentOffset = root.restoreListViewProxyModel.getItemIndent(projectId, parentPaperIdToBeRestored)

        stackView.push(restoreListViewComponent, {currentProjectId: projectId,
                           parentPaperIdToBeRestored: parentPaperIdToBeRestored,
                           trashedChildrenList: trashedChildrenList,
                           treeIndentOffset: treeIndentOffset
                       })
    }

    function popRestoreListView() {
        console.log("popRestoreListView")
        stackView.pop()
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
