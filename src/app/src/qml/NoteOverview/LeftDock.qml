import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import QtQml 2.15
import eu.skribisto.searchnotelistproxymodel 1.0
import eu.skribisto.usersettings 1.0
import "../Items"
import ".."

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
    //--------------- toolboxes Behavior------------------------
    //-----------------------------------------------------------

    Settings {
        id: settings
        category: "noteOverviewLeftDock"

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
            source: "qrc:///icons/backup/compass.svg"
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
            Globals.openNoteInNewTabCalled(_projectId, _paperId)
        }
    }


    function initNavigationView(){
        navigationView.openDocumentInNewTab.connect(Globals.openNoteInNewTabCalled)
        navigationView.openDocumentInNewWindow.connect(Globals.openNoteInNewWindowCalled)
        navigationView.restoreDocumentList.connect(root.restoreNoteList)
    }

    SKRSearchNoteListProxyModel {
        id: navigationProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
        navigateByBranchesEnabled: true
    }

    navigationView.navigationListProxyModel: navigationProxyModel

    Connections {
        target: skrData.projectHub()
        function onActiveProjectChanged(projectId){
            navigationProxyModel.projectIdFilter = projectId
            navigationProxyModel.parentIdFilter = -1
        }
    }


    SKRSearchNoteListProxyModel {
        id: trashedNoteProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false
    }
    navigationView.trashedListViewProxyModel: trashedNoteProxyModel

    SKRSearchNoteListProxyModel {
        id: restoreNoteProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false
    }
    navigationView.restoreListViewProxyModel: restoreNoteProxyModel

    navigationView.iconUrl: "qrc:///icons/backup/view-pim-notes.svg"


    function restoreNoteList(projectId, noteIdList){
        // restore is difficult to explain : a restored parent will restore its children, even those trashed years before. To avoid that,
        // children trashed at the same minute will be checked to allow restore. The older ones will stay unchecked by default.
        // All that is done in RestoreView.qml

        var i
        for(i = 0 ; i < noteIdList.length ; i++){
            skrData.noteHub().untrashOnlyOnePaper(projectId, noteIdList[i])
        }


        //console.log("restored: note:", noteIdList)
    }




    //-----------------------------------------------------------
    //---------Document List : ----------------------------------
    //-----------------------------------------------------------

    Action{
        id: documentViewAction
        checkable: true
        text: qsTr( "Show recent notes")
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


    //    documentView.model: plmModels.noteDocumentListModel()
    //    documentView.documentModel: plmModels.noteDocumentListModel()




    //-----------------------------------------------------------
    //-----------------------------------------------------------
    //-----------------------------------------------------------




    function setCurrentPaperId(projectId, paperId) {
        navigationView.setCurrentPaperId(projectId, paperId)
    }



}
