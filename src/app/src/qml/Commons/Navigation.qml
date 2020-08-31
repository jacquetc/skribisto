import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import ".."

NavigationForm {
    id: root
    property var treeListViewProxyModel
    property var deletedListViewProxyModel
    property int openedProjectId
    property int openedPaperId


    signal openDocument(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openDocumentInNewTab(int projectId, int paperId)

    Component {
        id: treeListViewComponent

        TreeListView {
            id: treeListView
            proxyModel: root.treeListViewProxyModel
            model: root.treeListViewProxyModel
            openedProjectId: root.openedPaperId
            openedPaperId: root.openedPaperId

            Component.onCompleted: {
                treeListView.openDocument.connect(root.openDocument)
                treeListView.openDocumentInNewTab.connect(root.openDocumentInNewTab)
                treeListView.showDeletedList.connect(root.pushDeletedListView)

            }
        }
    }

    //-----------------------------------------------------------------------
    //------list of deleted items----------------------------------------------------
    //-----------------------------------------------------------------------

    function pushDeletedListView() {
        stackView.push(deletedListViewComponent)
    }

    function popDeletedListView() {
        stackView.pop()
    }

    Component {
        id:deletedListViewComponent

        DeletedListView {
            id: deletedListView
            proxyModel: root.deletedListViewProxyModel
            model: root.deletedListViewProxyModel

            Component.onCompleted: {
                deletedListView.openDocument.connect(root.openDocument)
                deletedListView.openDocumentInNewTab.connect(root.openDocumentInNewTab)
                deletedListView.goBack.connect(root.popDeletedListView)

            }
        }
    }
}
