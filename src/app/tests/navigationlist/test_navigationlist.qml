import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15
import QtQml.Models 2.15
import QtQml 2.15
import Qt.labs.settings 1.1
import eu.skribisto.result 1.0
import eu.skribisto.projecthub 1.0
import eu.skribisto.searchtreelistproxymodel 1.0
import QtQuick.Controls.Material 2.15
import "qrc:///qml/Commons"
import "qrc:///qml/Items"
import "qrc:///qml/"

ApplicationWindow {

    id: rootWindow
    objectName: "rootWindow"
    minimumHeight: 800
    minimumWidth: 600
    visible: true


    Material.background: SkrTheme.pageBackground

    property url testProjectFileName: "qrc:/testfiles/skribisto_test_project.skrib"

    Component.onCompleted:{
    var result = plmData.projectHub().loadProject(testProjectFileName)
        navigationView.setNavigationTreeItemId(1, 21)
    }


    SKRSearchTreeListProxyModel {
        id: navigationProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
        navigateByBranchesEnabled: true
    }

    SKRSearchTreeListProxyModel {
        id: trashedSheetProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false

    }

    SKRSearchTreeListProxyModel {
        id: restoreSheetProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false
    }

    Item{
        id: base
        anchors.fill: parent


//    ScrollView {
//        id: scrollView
//        anchors.fill: parent
//        focusPolicy: Qt.StrongFocus
//        clip: true
//        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
//        ScrollBar.vertical.policy: ScrollBar.AsNeeded

//        contentWidth: scrollView.width


        Navigation{
            id: navigationView
        navigationListProxyModel: navigationProxyModel

        trashedListViewProxyModel: trashedSheetProxyModel
        restoreListViewProxyModel: restoreSheetProxyModel

        width: parent.width
        height: 600
        }


        function initNavigationView(){
            navigationView.restoreDocumentList.connect(root.restoreTreeIdList)
        }

        function restoreTreeIdList(projectId, treeIdList){
            // restore is difficult to explain : a restored parent will restore its children, even those trashed years before. To avoid that,
            // children trashed at the same minute will be checked to allow restore. The older ones will stay unchecked by default.
            // All that is done in RestoreView.qml

            var i
            for(i = 0 ; i < treeIdList.length ; i++){
                plmData.sheetHub().untrashOnlyOnePaper(projectId, treeIdList[i])
            }


           //console.log("restored: sheet:", sheetIdList)
        }



//                    ScrollBar.vertical: ScrollBar {
//                        id: internalScrollBar
//                        parent: searchListView
//                        policy: ScrollBar.AsNeeded
//                    }





    }

}
