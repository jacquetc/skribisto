import QtQuick 2.12
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import Qt.labs.settings 1.1
import eu.skribisto.notelistproxymodel 1.0
import eu.skribisto.searchnotelistproxymodel 1.0
import eu.skribisto.writedocumentlistmodel 1.0
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
    }
    navigation.deletedListViewProxyModel: deletedNoteProxyModel

    //    Connections {
    //        target: Globals
    //        onOpenSheetCalled: function (projectId, paperId) {

    //           proxyModel.setCurrentPaperId(projectId, paperId)


    //        }
    //    }








    //-----------------------------------------------------------

    //Document List :
    //-----------------------------------------------------------
//    documentView.model: plmModels.noteDocumentListModel()
//    documentView.documentModel: plmModels.noteDocumentListModel()

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
        category: "noteLeftDock"
        //property string dockSplitView: "0"
        property bool dockFolded: false
        property bool navigationFrameFolded: navigationFrame.folded ? true : false
        property bool documentFrameFolded: documentFrame.folded ? true : false
        property int width: fixedWidth
    }

    function setCurrentPaperId(projectId, paperId) {
        proxyModel.setCurrentPaperId(projectId, paperId)
    }
    function setOpenedPaperId(projectId, paperId) {
        navigation.openedProjectId = projectId
        navigation.openedPaperId = paperId
    }


    //    PropertyAnimation {
    //        target: writeNavigationFrame
    //        property: "SplitView.preferredHeight"
    //        duration: 500
    //        easing.type: Easing.InOutQuad
    //    }
    Component.onCompleted: {
        folded ? fold() : unfold()

        navigationFrame.folded = settings.navigationFrameFolded
        documentFrame.folded = settings.documentFrameFolded

        //        splitView.restoreState(settings.dockSplitView
        // navigation :
        navigation.onOpenDocument.connect(Globals.openNoteCalled)
        navigation.onOpenDocumentInNewTab.connect(Globals.openNoteInNewTabCalled)

        //

        fixedWidth = settings.width


    }
    Component.onDestruction: {
        //        settings.dockSplitView = splitView.saveState()
        settings.dockFolded = folded
    }
}
