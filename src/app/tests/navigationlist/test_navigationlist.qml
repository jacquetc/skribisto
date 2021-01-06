import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15
import QtQml.Models 2.15
import QtQml 2.15
import Qt.labs.settings 1.1
import eu.skribisto.result 1.0
import eu.skribisto.projecthub 1.0
import eu.skribisto.searchsheetlistproxymodel 1.0
import QtQuick.Controls.Material 2.15
import "qrc:///qml/Commons"
import "qrc:///qml/Items"
import "qrc:///qml/"

ApplicationWindow {

    id: rootWindow
    objectName: "rootWindow"
    minimumHeight: 500
    minimumWidth: 600
    visible: true


    Material.background: SkrTheme.pageBackground

    property url testProjectFileName: "qrc:/testfiles/skribisto_test_project.skrib"

    Component.onCompleted:{
    var result = plmData.projectHub().loadProject(testProjectFileName)
        navigationProxyModel.setCurrentPaperId(1, 2)
    }


    SKRSearchSheetListProxyModel {
        id: navigationProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
        navigateByBranchesEnabled: true
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



        NavigationList {
            id: searchListView
            model: navigationProxyModel
            proxyModel: navigationProxyModel
            //openedPaperId: 1
            width: parent.width
            height: 600



            Accessible.role: Accessible.List

//                    ScrollBar.vertical: ScrollBar {
//                        id: internalScrollBar
//                        parent: searchListView
//                        policy: ScrollBar.AsNeeded
//                    }

        }

    }

}
