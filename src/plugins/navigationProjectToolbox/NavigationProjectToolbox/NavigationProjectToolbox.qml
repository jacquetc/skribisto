import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import eu.skribisto.projecthub 1.0
import eu.skribisto.searchtreelistproxymodel 1.0

import "../../Items"
import "../../Commons"
import "../.."

SkrToolbox {
    id: root

    iconSource: "qrc:///icons/backup/compass.svg"
    showButtonText: qsTr("Show navigation toolbox")

    Navigation {
        id: navigationView
        clip: true

        width: scrollView.width
        height: navigationView.implicitHeight

        navigationListProxyModel: navigationProxyModel
        trashedListViewProxyModel: trashedTreeProxyModel
        restoreListViewProxyModel: restoreTreeProxyModel
    }

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

    function initNavigationView() {
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

    Connections {
        target: skrData.projectHub()
        function onActiveProjectChanged(projectId) {
            navigationProxyModel.projectIdFilter = projectId
            navigationProxyModel.parentIdFilter = -1
        }
    }

    SKRSearchTreeListProxyModel {
        id: trashedTreeProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false
    }

    SKRSearchTreeListProxyModel {
        id: restoreTreeProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false
    }

    function restoreTreeItemList(projectId, treeItemIdList) {



        //console.log("restored: sheet:", sheetIdList)
    }

    Connections {
        target: rootWindow
        function onSetNavigationTreeItemIdCalled(projectId, treeItemId) {
            navigationView.setNavigationTreeItemId(projectId, treeItemId)
        }
    }

    Connections {
        target: rootWindow
        function onSetNavigationTreeItemParentIdCalled(projectId, treeItemParentId) {
            navigationView.setNavigationTreeItemParentId(projectId,
                                                         treeItemParentId)
        }
    }

    Connections {
        target: viewManager
        function onFocusedChanged(position, projectId, treeItemId, pageType) {
            if (projectId !== -1 && treeItemId !== -1) {
                navigationView.setNavigationTreeItemId(projectId, treeItemId)
            }
        }
    }

    //----------------------------------------------------------
    Connections {
        target: skrData.projectHub()
        function onProjectLoaded(_projectId) {
            navigationView.setNavigationTreeItemId(_projectId, 0)
        }
    }
}
