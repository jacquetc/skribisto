import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import eu.skribisto.searchnotelistproxymodel 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import "../Commons"
import "../Items"
import ".."

NoteSearchItemForm {


    Connections{
        target: plmData.projectHub()
        function onActiveProjectChanged(projectId){

            noteOverviewSearchProxyModel.projectIdFilter = projectId
            noteOverviewTagProxyModel.projectIdFilter =  projectId
            searchTagPad.projectId =  projectId
        }
    }

    //----------------------------------------------------------------------------
    // --------------------------------- text field ------------------------------
    //----------------------------------------------------------------------------

    searchTextField.onAccepted: {
        searchListView.forceActiveFocus()
        searchListView.currentIndex = 0
    }




    //----------------------------------------------------------------------------
    // --------------------------------- tagpad ----------------------------------
    //----------------------------------------------------------------------------

    showTagDrawerButton.onClicked: {
        searchDrawer.open()
        searchTagPad.forceActiveFocus()
    }

        //proxy model for tag list :

        SKRSearchTagListProxyModel {
            id: noteOverviewTagProxyModel
        }
        searchTagPad.tagListModel: noteOverviewTagProxyModel

        searchTagPad.onSelectedListModified: function (selectedList){

            noteOverviewSearchProxyModel.tagIdListFilter = selectedList
            deselectTagsButton.visible = selectedList.length === 0 ? false : true

        }

        deselectTagsButton.onClicked: {

            noteOverviewSearchProxyModel.tagIdListFilter = []
            searchTagPad.clearSelection()
            deselectTagsButton.visible = false
        }

        searchTagPad.onEscapeKeyPressed: {
            searchListView.forceActiveFocus()
            searchDrawer.close()
        }

    //----------------------------------------------------------------------------
    // --------------------------------- list view ----------------------------------
    //----------------------------------------------------------------------------

    //proxy model for search :

    SKRSearchNoteListProxyModel {
        id: noteOverviewSearchProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
        textFilter: searchTextField.text
    }
    searchListView.model: noteOverviewSearchProxyModel
    searchListView.proxyModel: noteOverviewSearchProxyModel

    searchListView.treelikeIndentsVisible: searchTextField.text.length === 0


    Component.onCompleted: {
        searchListView.openDocumentInNewTab.connect(Globals.openNoteInNewTabCalled)
        searchListView.openDocumentInNewWindow.connect(Globals.openNoteInNewWindowCalled)
    }

    Connections {
        target: searchListView
        function onOpenDocument(openedProjectId, openedPaperId, _projectId, _paperId) {
            Globals.openNoteInNewTabCalled(_projectId, _paperId)
        }
    }


}
