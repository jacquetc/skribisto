import QtQuick 2.12
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import Qt.labs.settings 1.1
import eu.skribisto.sheetlistproxymodel 1.0
import eu.skribisto.writedocumentlistmodel 1.0
import eu.skribisto.skrusersettings 1.0
import ".."

WriteRightDockForm {


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


    //-----------------------------------------------------------


    Connections {
        target: Globals
        onOpenSheetCalled: function (projectId, paperId) {




        }
    }








    //-----------------------------------------------------------

    //Document List :
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
        category: "writeRightDock"
        //property string dockSplitView: "0"
        property bool dockFolded: false
        property bool editFrameFolded: editFrame.folded ? true : false
        property bool noteFrameFolded: editFrame.folded ? true : false
//        property bool documentFrameFolded: documentFrame.folded ? true : false
        property int width: fixedWidth
    }




    //    PropertyAnimation {
    //        target: writeTreeViewFrame
    //        property: "SplitView.preferredHeight"
    //        duration: 500
    //        easing.type: Easing.InOutQuad
    //    }
    Component.onCompleted: {
        folded ? fold() : unfold()

        editFrame.folded = settings.editFrameFolded
        noteFrame.folded = settings.noteFrameFolded

        //        splitView.restoreState(settings.dockSplitView)
        //treeView.onOpenDocument.connect(Globals.openSheetCalled)
        fixedWidth = settings.width
    }
    Component.onDestruction: {
        //        settings.dockSplitView = splitView.saveState()
        settings.dockFolded = folded
    }
}
