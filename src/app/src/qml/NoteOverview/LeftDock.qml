import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import eu.skribisto.notelistproxymodel 1.0
import eu.skribisto.searchnotelistproxymodel 1.0
import eu.skribisto.skrusersettings 1.0
import ".."

LeftDockForm {


    SkrUserSettings {
        id: skrUserSettings
    }

    splitView.handle: Item {
        implicitHeight: 8
        RowLayout {
            anchors.fill: parent
            Rectangle {
                Layout.preferredWidth: 20
                Layout.preferredHeight: 5
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                color: "lightgrey"
            }
        }
    }



    //Navigation List :
    //-----------------------------------------------------------

    PLMNoteListProxyModel {
        id: proxyModel
    }

    navigation.treeListViewProxyModel: proxyModel

    SKRSearchNoteListProxyModel {
        id: deletedNoteProxyModel
        showDeletedFilter: true
        showNotDeletedFilter: false
    }
    navigation.deletedListViewProxyModel: deletedNoteProxyModel


    //    Connections {
    //        target: Globals
    //        onOpenSheetCalled: function (projectId, paperId) {

    //           //proxyModel.setCurrentPaperId(projectId, paperId)


    //        }
    //    }








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
