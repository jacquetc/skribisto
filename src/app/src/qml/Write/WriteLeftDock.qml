import QtQuick 2.12
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import Qt.labs.settings 1.1
import eu.skribisto.sheetlistproxymodel 1.0
import eu.skribisto.writedocumentlistmodel 1.0
import eu.skribisto.skrusersettings 1.0
import ".."

WriteLeftDockForm {


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

    PLMSheetListProxyModel {
        id: proxyModel
    }

    treeView.model: proxyModel
    treeView.proxyModel: proxyModel

    Connections {
        target: Globals
        onOpenSheetCalled: function (projectId, paperId) {

           proxyModel.setCurrentPaperId(projectId, paperId)


        }
    }








    //-----------------------------------------------------------

    //Document List :
    //-----------------------------------------------------------
    documentView.model: plmModels.writeDocumentListModel()
    documentView.documentModel: plmModels.writeDocumentListModel()

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
        category: "writeLeftDock"
        //property string dockSplitView: "0"
        property bool dockFolded: false
        property bool treeViewFrameFolded: treeViewFrame.folded ? true : false
        property bool documentFrameFolded: documentFrame.folded ? true : false
    }




    //    PropertyAnimation {
    //        target: writeTreeViewFrame
    //        property: "SplitView.preferredHeight"
    //        duration: 500
    //        easing.type: Easing.InOutQuad
    //    }
    Component.onCompleted: {
        folded ? fold() : unfold()

        treeViewFrame.folded = settings.treeViewFrameFolded
        documentFrame.folded = settings.documentFrameFolded

        //        splitView.restoreState(settings.dockSplitView)
        treeView.onOpenDocument.connect(Globals.openSheetCalled)
    }
    Component.onDestruction: {
        //        settings.dockSplitView = splitView.saveState()
        settings.dockFolded = folded
    }
}
