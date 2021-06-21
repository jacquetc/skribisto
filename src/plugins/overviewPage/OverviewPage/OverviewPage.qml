import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import eu.skribisto.searchtreelistproxymodel 1.0

import "../../Items"
import "../../Commons"
import "../.."

OverviewPageForm {
    id: root

    pageType: "OVERVIEW"

    toolboxes: [overviewToolboxComponent, tagPadComponent]

    Component {
        id: overviewToolboxComponent

        OverviewToolbox {
            projectId: root.projectId
        }
    }

    Component {
        id: tagPadComponent

        SkrToolbox {
            showButtonText: qsTr("Show tags toolbox")
            iconSource: "qrc:///icons/backup/tag.svg"

            TagPad {
                id: tagPad
                anchors.fill: parent
                projectId: root.projectId
                treeItemId: overviewTree.currentTreeItemId
            }
        }
    }

    //--------------------------------------------------------
    additionalPropertiesForSavingView: {
        return {}
    }

    //--------------------------------------------------------
    //---View buttons-----------------------------------------
    //--------------------------------------------------------
    titleLabel.text: qsTr("Overview of %1").arg(
                         skrData.projectHub().getProjectName(
                             skrData.projectHub().activeProjectId))

    //--------------------------------------------------------
    viewButtons.viewManager: root.viewManager
    viewButtons.position: root.position

    viewButtons.onOpenInNewWindowCalled: {
        //skrWindowManager.insertAdditionalPropertyForViewManager("isSecondary", isSecondary)
        skrWindowManager.addWindowForProjectDependantPageType(projectId,
                                                              pageType)
    }

    viewButtons.onSplitCalled: function (position) {
        //viewManager.insertAdditionalProperty("isSecondary", isSecondary)
        viewManager.loadProjectDependantPageAt(projectId, pageType, position)
    }

    //-------------------------------------------------------------
    //-------Overview------------------------------------------
    //-------------------------------------------------------------
    overviewTree.proxyModel: SKRSearchTreeListProxyModel {
        id: overviewProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
        projectIdFilter: root.projectId

        onParentIdFilterChanged: {
            if (overviewProxyModel.parentIdFilter === -2) {
                topFilteringBanner.visible = false
                return
            }

            topFilteringBanner.visible = true

            var title = skrData.treeHub().getTitle(
                        root.projectId, overviewProxyModel.parentIdFilter)
            topFilteringBannerLabel.text = qsTr(
                        "The focus is currently on %1").arg(title)
        }
    }

    unsetFilteringParentToolButton.icon.source: "qrc:///icons/backup/window-close.svg"
    unsetFilteringParentToolButton.onClicked: {
        overviewProxyModel.parentIdFilter = -2
    }

    dropAreaEnabled: !overviewTree.dragging
}
