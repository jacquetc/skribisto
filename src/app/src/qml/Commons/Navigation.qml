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
    property int openedPaperId

    property string iconUrl


    signal openDocument(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openDocumentInNewTab(int projectId, int paperId)
    signal openDocumentInNewWindow(int projectId, int paperId)
    signal restoreDocumentList(int projectId, var paperIdList)

    function setCurrentPaperId(projectId, paperId){
         stackView.get(0, StackView.DontLoad).setCurrentPaperId(projectId, paperId)
    }

    Component {
        id: navigationListComponent

        NavigationList {
            id: navigationList
            proxyModel: root.navigationListProxyModel
            openedProjectId: root.openedProjectId
            openedPaperId: root.openedPaperId
            iconUrl: root.iconUrl

            Component.onCompleted: {
                navigationList.openDocument.connect(root.openDocument)
                navigationList.openDocumentInNewTab.connect(root.openDocumentInNewTab)
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
        var trashedAncestorsList = trashedListViewProxyModel.getAncestorsList(projectId, paperId, true, false)
        // if no children :
        if(trashedChildrenList.length === 0 && trashedAncestorsList === 0){
            restoreDocumentList(projectId, [paperId])
            return
        }
        else {

            pushRestoreListView(projectId, paperId, trashedChildrenList.concat(trashedAncestorsList))
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
                restoreListView.openDocument.connect(root.openDocument)
                restoreListView.openDocumentInNewTab.connect(root.openDocumentInNewTab)
                restoreListView.openDocumentInNewWindow.connect(root.openDocumentInNewWindow)
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
