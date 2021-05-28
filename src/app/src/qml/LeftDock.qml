import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import QtQml 2.15
import eu.skribisto.searchtreelistproxymodel 1.0
import "Items"
import "Commons"

LeftDockForm {
    id: root


    required property var viewManager
    //-----------------------------------------------------------
    //--------------- toolboxes Behavior------------------------
    //-----------------------------------------------------------

    //    Settings {
    //        id: settings
    //        category: "leftDock"

    //        property bool navigationViewVisible: true
    //        property bool documentViewVisible: true
    //    }

    //    function loadConf(){

    //        navigationViewToolButton.checked = settings.navigationViewVisible
    //        documentViewToolButton.checked = settings.documentViewVisible

    //    }

    //    function resetConf(){
    //        settings.navigationViewVisible = true
    //        settings.documentViewVisible = true
    //    }

    //    Component.onCompleted: {
    //        loadConf()
    //        initNavigationView()
    //        Globals.resetDockConfCalled.connect(resetConf)

    //    }

    //    Component.onDestruction: {
    //        settings.navigationViewVisible = navigationViewToolButton.checked
    //        settings.documentViewVisible = documentViewToolButton.checked

    //    }
    Component.onCompleted: {
        toolButtonModel.clear()
        determineAndAddToolboxPlugins()
        toolboxRepeater.model = toolboxes
    }

    Component {
        id: toolboxLoaderComponent

        Loader{
            id: toolboxLoader
            sourceComponent: modelData


            width: scrollView.width

            onLoaded: {

                //toolboxFlickable.contentHeight = toolboxLayout.childrenRect.height

                var iconSource = toolboxLoader.item.iconSource
                var showButtonText = toolboxLoader.item.showButtonText
                toolButtonModel.append({"iconSource": iconSource, "showButtonText": showButtonText,
                                           "toolbox": toolboxLoader})

            }

            Binding {
                target: toolboxLoader.item
                when: toolboxLoader.status === Loader.Ready
                property: "height"
                value: toolboxLoader.item.implicitHeight

            }

        }
    }
    toolboxRepeater.delegate: toolboxLoaderComponent


    //--------------------------------------------------------------
    //--------------------------------------------------------------
    //--------------------------------------------------------------




    ListModel {
        id: toolButtonModel
    }
    toolButtonRepeater.model: toolButtonModel


    Component {
        id: toolButtonLoaderComponent


        Loader {
            id: toolButtonLoader
            sourceComponent: toolButtonComponent

            onLoaded: {
                toolButtonLoader.item.iconSource = model.iconSource
                toolButtonLoader.item.text = model.showButtonText
                toolButtonLoader.item.toolbox =  model.toolbox
            }


        }
    }
    toolButtonRepeater.delegate: toolButtonLoaderComponent


    Component {
        id: toolButtonComponent
        SkrToolButton {
            id: toolButton
            checkable: true
            icon.source: iconSource

            property var toolbox
            property string iconSource

            onCheckedChanged: {
                toolbox.visible = toolButton.checked
            }

            Binding on checked{
                value: toolbox.visible
                delayed: true
                restoreMode: Binding.RestoreBindingOrValue
            }

        }

    }



    //------------------------------------------------------------------------
    //-----toolboxes------------------------------------------------------------
    //-----------------------------------------------------------------------


    property var toolboxes: [navigationListComponent]

    function determineAndAddToolboxPlugins(){
        var toolboxUrlList = skrTreeManager.findToolboxUrlsForProject()

        for(var i in toolboxUrlList){
            var url = toolboxUrlList[i]
            var pluginComponent = Qt.createComponent(url, Component.PreferSynchronous, root)
            toolboxes.push(pluginComponent)
        }

    }

    //-----------------------------------------------------------
    //------- Navigation List : ---------------------------------
    //-----------------------------------------------------------

    Component {
        id: navigationListComponent

        SkrToolbox{

            showButtonText: qsTr("Show navigation")
            iconSource: "qrc:///icons/backup/compass.svg"


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

            function initNavigationView(){
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
                function onActiveProjectChanged(projectId){
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




            function restoreTreeItemList(projectId, treeItemIdList){
                // restore is difficult to explain : a restored parent will restore its children, even those trashed years before. To avoid that,
                // children trashed at the same minute will be checked to allow restore. The older ones will stay unchecked by default.
                // All that is done in RestoreView.qml

                var i
                for(i = 0 ; i < treeItemIdList.length ; i++){
                    skrData.treeHub().untrashOnlyOneTreeItem(projectId, treeItemIdList[i])
                }


                //console.log("restored: sheet:", sheetIdList)
            }

            Connections{
                target: rootWindow
                function onSetNavigationTreeItemIdCalled(projectId, treeItemId){
                    navigationView.setNavigationTreeItemId(projectId, treeItemId)
                }
            }

            Connections{
                target: rootWindow
                function onSetNavigationTreeItemParentIdCalled(projectId, treeItemParentId){
                    navigationView.setNavigationTreeItemParentId(projectId, treeItemParentId)
                }
            }


            Connections{
                target: viewManager
                function onFocusedChanged(position, projectId, treeItemId, pageType){
                    if(projectId !== -1 && treeItemId !== -1){
                        navigationView.setNavigationTreeItemId(projectId, treeItemId)
                    }
                }
            }

            //----------------------------------------------------------

            Connections {
                target:skrData.projectHub()
                function onProjectLoaded(_projectId) {
                    onProjectLoadedTimer.projectId = _projectId
                    onProjectLoadedTimer.start()
                }

            }
            Timer {
                id: onProjectLoadedTimer
                property int projectId
                interval: 100
                onTriggered: navigationView.setNavigationTreeItemId(projectId, 0)

            }
        }
    }
    //----------------------------------------------------------


    //-----------------------------------------------------------
    //-----------------------------------------------------------
    //-----------------------------------------------------------

    signal hideDockCalled()
    hideDockToolButton.action: hideDockAction

    Action {
        id: hideDockAction
        icon.source: "qrc:///icons/backup/go-previous-view.svg"
        onTriggered: {
            hideDockCalled()
        }

    }


}
