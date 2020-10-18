import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import Qt.labs.settings 1.1
import eu.skribisto.sheetlistproxymodel 1.0
import eu.skribisto.searchsheetlistproxymodel 1.0
import eu.skribisto.writedocumentlistmodel 1.0
import eu.skribisto.skrusersettings 1.0
import ".."

LeftDockForm {
    id: root


    SkrUserSettings {
        id: skrUserSettings
    }


    splitView.handle: Item {
        id: handle
        implicitHeight: 8
        property bool hovered: SplitHandle.hovered

        RowLayout {
            anchors.fill: parent
            Rectangle {
                Layout.preferredWidth: 20
                Layout.preferredHeight: 5
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                color: handle.hovered ? Material.accentColor : Material.dividerColor
            }
        }
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
            Menu {
                id: navigationDockMenu
                objectName: "navigationDockMenu"
                title: qsTr("&Navigation dock")

                Component.onCompleted: {

                    navigationMenuShortcut.sequence = skrQMLTools.mnemonic(title)
                    navigationMenuShortcut.activated.connect(function() {
                        Globals.openSubMenuCalled(navigationDockMenu)
                    })
                }


                MenuItem {
                    text: qsTr("&Navigation")
                    onTriggered: {
                        if(Globals.compactSize){
                            leftDrawer.open()
                        }
                        navigationFrame.folded = false
                        navigation.forceActiveFocus()
                    }
                }

                MenuItem {
                    text: qsTr("&Documents")
                    onTriggered: {
                        if(Globals.compactSize){
                            leftDrawer.open()
                        }
                        documentFrame.folded = false
                        documentView.forceActiveFocus()
                    }
                }
            }
        }
    ]

    //Navigation List :
    //-----------------------------------------------------------

    PLMSheetListProxyModel {
        id: proxyModel
    }

    navigation.treeListViewProxyModel: proxyModel


    SKRSearchSheetListProxyModel {
        id: trashedSheetProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false

    }
    navigation.trashedListViewProxyModel: trashedSheetProxyModel

    SKRSearchSheetListProxyModel {
        id: restoreSheetProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false
    }
    navigation.restoreListViewProxyModel: restoreSheetProxyModel






    function restoreSheetList(projectId, sheetIdList){
        // restore is difficult to explain : a restored parent will restore its children, even those trashed years before. To avoid that,
        // children trashed at the same minute will be checked to allow restore. The older ones will stay unchecked by default.
        // All that is done in RestoreView.qml

        var i
        for(i = 0 ; i < sheetIdList.length ; i++){
            plmData.sheetHub().untrashOnlyOnePaper(projectId, sheetIdList[i])
        }


       console.log("restored: sheet:", sheetIdList)
    }






    //Document List :
    //-----------------------------------------------------------
    documentView.model: plmModels.writeDocumentListModel()
    documentView.documentModel: plmModels.writeDocumentListModel()





    //-----------------------------------------------------------

    //-----------------------------------------------------------


    //-----------------------------------------------------------

    //-----------------------------------------------------------
    transitions: [
        Transition {

            PropertyAnimation {
                properties: "implicitWidth"
                easing.type: Easing.InOutQuad
                duration: 500
            }
        }
    ]

    property alias settings: settings

    Settings {
        id: settings
        category: "writeOverviewLeftDock"
        property var dockSplitView
        property bool navigationFrameFolded: navigationFrame.folded
        property bool documentFrameFolded: documentFrame.folded
    }


    function setCurrentPaperId(projectId, paperId) {
        proxyModel.setCurrentPaperId(projectId, paperId)
    }


    PropertyAnimation {
        target: navigationFrame
        property: "SplitView.preferredHeight"
        duration: 500
        easing.type: Easing.InOutQuad
    }

    Connections {
        target: navigation
        function onOpenDocument(openedProjectId, openedPaperId, _projectId, _paperId) {
            Globals.openSheetInNewTabCalled(_projectId, _paperId)
        }
    }

    function loadConf(){
         navigationFrame.folded = settings.navigationFrameFolded
        documentFrame.folded = settings.documentFrameFolded
        splitView.restoreState(settings.dockSplitView)
    }

    function resetConf(){
        navigationFrame.folded = false
        documentFrame.folded = false
        splitView.restoreState("")

    }

    Component.onCompleted: {

        loadConf()
        navigation.openDocumentInNewTab.connect(Globals.openSheetInNewTabCalled)
        navigation.openDocumentInNewWindow.connect(Globals.openSheetInNewWindowCalled)
        navigation.restoreDocumentList.connect(root.restoreSheetList)
        Globals.resetDockConfCalled.connect(resetConf)


    }

    Component.onDestruction: {
        settings.dockSplitView = splitView.saveState()

    }

    onEnabledChanged: {
        if(enabled){

        }
    }
}
