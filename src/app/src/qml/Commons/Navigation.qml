import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ".."

NavigationForm {
    id: root
    property int minimumHeight: 300

    property var treeListViewProxyModel
    property var trashedListViewProxyModel
    property int openedProjectId
    property int openedPaperId


    signal openDocument(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openDocumentInNewTab(int projectId, int paperId)
    signal openDocumentInNewWindow(int projectId, int paperId)

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
                trashedListView.goBack.connect(root.popTrashedListView)

            }
        }
    }




    // focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            stackView.get(0, StackView.DontLoad).forceActiveFocus()
        }
    }


}
