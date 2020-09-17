import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import eu.skribisto.sheetlistproxymodel 1.0
import eu.skribisto.searchsheetlistproxymodel 1.0
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

    PLMSheetListProxyModel {
        id: proxyModel
    }

    navigation.treeListViewProxyModel: proxyModel


    SKRSearchSheetListProxyModel {
        id: deletedSheetProxyModel
        showDeletedFilter: true
        showNotDeletedFilter: false
    }
    navigation.deletedListViewProxyModel: deletedSheetProxyModel




    //    Connections {
    //        target: Globals
    //        onOpenSheetCalled: function (projectId, paperId) {

    //           proxyModel.setCurrentPaperId(projectId, paperId)


    //        }
    //    }








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

    property alias settings: settings

    Settings {
        id: settings
        category: "writeLeftDock"
        property var dockSplitView
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


        PropertyAnimation {
            target: navigationFrame
            property: "SplitView.preferredHeight"
            duration: 500
            easing.type: Easing.InOutQuad
        }


        function init(){
            if(folded){
                fold()
            }
            else{
                unfold()
            }

            navigationFrame.folded = settings.navigationFrameFolded
            documentFrame.folded = settings.documentFrameFolded

            splitView.restoreState(settings.dockSplitView)

            // navigation :
            navigation.onOpenDocument.connect(Globals.openSheetCalled)
            navigation.onOpenDocumentInNewTab.connect(Globals.openSheetInNewTabCalled)

            //

            fixedWidth = settings.width
        }
    Component.onCompleted: {


        init()

    }
    Component.onDestruction: {

        settings.dockSplitView = splitView.saveState()
            settings.dockFolded = folded

        }

    onEnabledChanged: {
        if(enabled){

        }
    }
}
