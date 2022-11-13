import QtQuick
import QtQuick.Controls
import QtQuick.Layouts


NavigationForm {
    id: root
    property int minimumHeight: 300

    property var navigationListProxyModel
    property int openedProjectId
    property int openedTreeItemId

    signal openDocument(int projectId, int treeItemId)
    signal openDocumentInAnotherView(int projectId, int treeItemId)
    signal openDocumentInNewWindow(int projectId, int treeItemId)

    function setNavigationTreeItemId(projectId, treeItemId) {
        stackView.get(0, StackView.DontLoad).setCurrentTreeItemId(projectId,
                                                                  treeItemId)
    }

    function setNavigationTreeItemParentId(projectId, treeItemParentId) {
        stackView.get(0, StackView.DontLoad).setCurrentTreeItemParentId(
                    projectId, treeItemParentId)
    }

    Component {
        id: navigationListComponent

        NavigationList {
            id: navigationList

            Component.onCompleted: {
                navigationList.openDocument.connect(root.openDocument)
                navigationList.openDocumentInAnotherView.connect(
                            root.openDocumentInAnotherView)
                navigationList.openDocumentInNewWindow.connect(
                            root.openDocumentInNewWindow)
            }
        }
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
