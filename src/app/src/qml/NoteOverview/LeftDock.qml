import QtQuick 2.12
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import Qt.labs.settings 1.1
import eu.skribisto.notelistproxymodel 1.0
import eu.skribisto.searchnotelistproxymodel 1.0
//import eu.skribisto.writedocumentlistmodel 1.0
import eu.skribisto.skrusersettings 1.0
import ".."

LeftDockForm {


    SkrUserSettings {
        id: skrUserSettings
    }

    // fold :
    property bool folded: settings.dockFolded
    onFoldedChanged: folded ? fold() : unfold()

    function fold() {
        state = "dockFolded"
        settings.dockFolded = true
        folded = true
    }
    function unfold() {
        state = "dockUnfolded"
        settings.dockFolded = false
        folded = false
    }

    //    splitView.handle: Rectangle {
    //        implicitWidth: 4
    //        implicitHeight: 4
    //    }


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

    Settings {
        id: settings
        category: "noteOverviewLeftDock"
        //property string dockSplitView: "0"
        property bool dockFolded: false
        property bool navigationFrameFolded: navigationFrame.folded ? true : false
        property int width: fixedWidth
    }


    function setCurrentPaperId(projectId, paperId) {
        proxyModel.setCurrentPaperId(projectId, paperId)
    }


    //    PropertyAnimation {
    //        target: writeNavigationFrame
    //        property: "SplitView.preferredHeight"
    //        duration: 500
    //        easing.type: Easing.InOutQuad
    //    }

    Connections {
        target: navigation
        onOpenDocument: function (openedProjectId, openedPaperId, _projectId, _paperId) {
            Globals.openNoteInNewTabCalled(_projectId, _paperId)
        }
        }

    Component.onCompleted: {
        folded ? fold() : unfold()

        navigationFrame.folded = settings.navigationFrameFolded

        //        splitView.restoreState(settings.dockSplitView)

        // navigation :

        navigation.onOpenDocumentInNewTab.connect(Globals.openNoteInNewTabCalled)

        fixedWidth = settings.width
    }
    Component.onDestruction: {
        //        settings.dockSplitView = splitView.saveState()
        settings.dockFolded = folded
    }
}
