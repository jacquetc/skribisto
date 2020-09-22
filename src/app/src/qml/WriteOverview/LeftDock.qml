import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import eu.skribisto.sheetlistproxymodel 1.0
import eu.skribisto.searchsheetlistproxymodel 1.0
import eu.skribisto.writedocumentlistmodel 1.0
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
        navigation.onOpenDocumentInNewTab.connect(Globals.openSheetInNewTabCalled)
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
