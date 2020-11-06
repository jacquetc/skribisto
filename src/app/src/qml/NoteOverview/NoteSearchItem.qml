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

    searchTextField.onEditingFinished: {
        searchListView.forceActiveFocus()
        searchListView.currentIndex = 0
    }




    //----------------------------------------------------------------------------
    // --------------------------------- tagpad ----------------------------------
    //----------------------------------------------------------------------------

    showTagDrawerButton.onClicked: {
        searchDrawer.open()
        searchDrawer.forceActiveFocus()
    }

        //proxy model for tag list :

        SKRSearchTagListProxyModel {
            id: noteOverviewTagProxyModel
        }
        searchTagPad.tagListModel: noteOverviewTagProxyModel

        searchTagPad.onTagTapped: function (projectId, tagId){

            noteOverviewSearchProxyModel.tagIdListFilter = [tagId]
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
