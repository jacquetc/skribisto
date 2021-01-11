import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import QtQml 2.15
import eu.skribisto.searchsheetlistproxymodel 1.0
import eu.skribisto.writedocumentlistmodel 1.0
import eu.skribisto.usersettings 1.0
import ".."
import "../Items"

LeftDockForm {
    id: root


    SKRUserSettings {
        id: skrUserSettings
    }



    //-----------------------------------------------------------



    Shortcut {
        id: navigationMenuShortcut
        enabled: root.enabled

    }



    //-----------------------------------------------------------



    //Menu :
    property list<Component> menuComponents:  [
        Component{
            id:  navigationDockMenuComponent
            SkrMenu {
                id: navigationDockMenu
                objectName: "navigationDockMenu"
                title: qsTr("&Navigation dock")

                Component.onCompleted: {

                    navigationMenuShortcut.sequence = skrQMLTools.mnemonic(title)
                    navigationMenuShortcut.activated.connect(function() {
                        Globals.openSubMenuCalled(navigationDockMenu)
                    })
                }


                SkrMenuItem {
                    text: qsTr("&Navigation")
                    onTriggered: {
                        if(Globals.compactMode){
                            leftDrawer.open()
                        }
                        navigationViewToolButton.checked = true
                        navigationView.forceActiveFocus()
                    }
                }

                SkrMenuItem {
                    text: qsTr("&Documents")
                    onTriggered: {
                        if(Globals.compactMode){
                            leftDrawer.open()
                        }
                        documentViewToolButton.checked = true
                        documentView.forceActiveFocus()
                    }
                }
            }
        }
    ]


    //-----------------------------------------------------------
    //--------------- toolBoxes Behavior------------------------
    //-----------------------------------------------------------

    Settings {
        id: settings
        category: "writeOverviewLeftDock"

        property bool navigationViewVisible: true
        property bool documentViewVisible: true
    }

    function loadConf(){

        navigationViewToolButton.checked = settings.navigationViewVisible
        documentViewToolButton.checked = settings.documentViewVisible

    }

    function resetConf(){
        settings.navigationViewVisible = true
        settings.documentViewVisible = true
    }

    Component.onCompleted: {
        loadConf()
        initNavigationView()
        Globals.resetDockConfCalled.connect(resetConf)

    }

    Component.onDestruction: {
        settings.navigationViewVisible = navigationViewToolButton.checked
        settings.documentViewVisible = documentViewToolButton.checked

    }


    //-----------------------------------------------------------
    //------- Navigation List : ---------------------------------
    //-----------------------------------------------------------

    Action{
        id: navigationViewAction
        checkable: true
        text: qsTr( "Show navigation")
        icon {
            source: "qrc:///icons/backup/object-rows.svg"
            height: 50
            width: 50
        }
        onCheckedChanged: {
            navigationView.visible = navigationViewAction.checked
        }

        Binding on checked{
            value: navigationView.visible
            delayed: true
            restoreMode: Binding.RestoreBindingOrValue
        }
    }
    navigationViewToolButton.action: navigationViewAction

    Connections {
        target: navigationView
        function onOpenDocument(openedProjectId, openedPaperId, _projectId, _paperId) {
            Globals.openSheetInNewTabCalled(_projectId, _paperId)
        }
    }

    function initNavigationView(){
        navigationView.openDocumentInNewTab.connect(Globals.openSheetInNewTabCalled)
        navigationView.openDocumentInNewWindow.connect(Globals.openSheetInNewWindowCalled)
        navigationView.restoreDocumentList.connect(root.restoreSheetList)
    }

    SKRSearchSheetListProxyModel {
        id: navigationProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
        navigateByBranchesEnabled: true
    }

    navigationView.navigationListProxyModel: navigationProxyModel

    Connections {
        target: plmData.projectHub()
        function onActiveProjectChanged(projectId){
            navigationProxyModel.projectIdFilter = projectId
            navigationProxyModel.parentIdFilter = -1
        }
    }


    SKRSearchSheetListProxyModel {
        id: trashedSheetProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false

    }
    navigationView.trashedListViewProxyModel: trashedSheetProxyModel

    SKRSearchSheetListProxyModel {
        id: restoreSheetProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false
    }
    navigationView.restoreListViewProxyModel: restoreSheetProxyModel

    navigationView.iconUrl: "qrc:///icons/backup/story-editor.svg"





    function restoreSheetList(projectId, sheetIdList){
        // restore is difficult to explain : a restored parent will restore its children, even those trashed years before. To avoid that,
        // children trashed at the same minute will be checked to allow restore. The older ones will stay unchecked by default.
        // All that is done in RestoreView.qml

        var i
        for(i = 0 ; i < sheetIdList.length ; i++){
            plmData.sheetHub().untrashOnlyOnePaper(projectId, sheetIdList[i])
        }


       //console.log("restored: sheet:", sheetIdList)
    }






    //-----------------------------------------------------------
    //---------Document List : ----------------------------------
    //-----------------------------------------------------------

    Action{
        id: documentViewAction
        checkable: true
        text: qsTr( "Show recent sheets")
        icon {
            source: "qrc:///icons/backup/document-open-recent.svg"
            height: 50
            width: 50
        }
        onCheckedChanged: {
            documentView.visible = documentViewAction.checked
        }

        Binding on checked{
            value: documentView.visible
            delayed: true
            restoreMode: Binding.RestoreBindingOrValue
        }
    }
    documentViewToolButton.action: documentViewAction

    documentView.model: plmModels.writeDocumentListModel()
    documentView.documentModel: plmModels.writeDocumentListModel()





    //-----------------------------------------------------------
    //-----------------------------------------------------------
    //-----------------------------------------------------------


    function setCurrentPaperId(projectId, paperId) {
        navigationView.setCurrentPaperId(projectId, paperId)
    }


}
