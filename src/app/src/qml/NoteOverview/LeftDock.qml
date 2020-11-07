import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import eu.skribisto.notelistproxymodel 1.0
import eu.skribisto.searchnotelistproxymodel 1.0
import eu.skribisto.usersettings 1.0
import "../Items"
import ".."

LeftDockForm {
    id: root


    SKRUserSettings {
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
                color: handle.hovered ? SkrTheme.accent : SkrTheme.divider
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
                        if(Globals.compactSize){
                            leftDrawer.open()
                        }
                        navigationFrame.folded = false
                        navigation.forceActiveFocus()
                    }
                }

                SkrMenuItem {
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

    PLMNoteListProxyModel {
        id: proxyModel
    }

    navigation.treeListViewProxyModel: proxyModel

    SKRSearchNoteListProxyModel {
        id: trashedNoteProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false
    }
    navigation.trashedListViewProxyModel: trashedNoteProxyModel

    SKRSearchNoteListProxyModel {
        id: restoreNoteProxyModel
        showTrashedFilter: true
        showNotTrashedFilter: false
    }
    navigation.restoreListViewProxyModel: restoreNoteProxyModel



    function restoreNoteList(projectId, noteIdList){
        // restore is difficult to explain : a restored parent will restore its children, even those trashed years before. To avoid that,
        // children trashed at the same minute will be checked to allow restore. The older ones will stay unchecked by default.
        // All that is done in RestoreView.qml

        var i
        for(i = 0 ; i < noteIdList.length ; i++){
            plmData.noteHub().untrashOnlyOnePaper(projectId, noteIdList[i])
        }


       console.log("restored: note:", noteIdList)
    }




    //-----------------------------------------------------------

    //Document List :
    //-----------------------------------------------------------
//    documentView.model: plmModels.noteDocumentListModel()
//    documentView.documentModel: plmModels.noteDocumentListModel()




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
        category: "noteOverviewLeftDock"
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
            Globals.openNoteInNewTabCalled(_projectId, _paperId)
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
        navigation.onOpenDocumentInNewTab.connect(Globals.openNoteInNewTabCalled)
        navigation.openDocumentInNewWindow.connect(Globals.openNoteInNewWindowCalled)
        navigation.restoreDocumentList.connect(root.restoreNoteList)
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
