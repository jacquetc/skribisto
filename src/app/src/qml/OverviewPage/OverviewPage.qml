import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15

import QtQml 2.15
import eu.skribisto.projecthub 1.0
import eu.skribisto.searchtreelistproxymodel 1.0
import "../Commons"
import "../Items"
import ".."

OverviewPageForm {
    id: root

    pageType: "OVERVIEW"


    //--------------------------------------------------------
    //---View buttons-----------------------------------------
    //--------------------------------------------------------

    viewButtons.viewManager: root.viewManager
    viewButtons.position: root.position

    viewButtons.onOpenInNewWindowCalled: {
        skrWindowManager.addWindowForProjectIndependantPageType(pageType)
    }

    viewButtons.onSplitCalled: function(position){
        viewManager.loadProjectIndependantPageAt(pageType, position)
    }


    //--------------------------------------------------------
    //--- Tree -----------------------------------------
    //--------------------------------------------------------


    SKRSearchTreeListProxyModel {
        id: overviewProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
        navigateByBranchesEnabled: false
        projectIdFilter: plmData.projectHub().activeProjectId
    }

    overviewTree.proxyModel: overviewProxyModel

}
