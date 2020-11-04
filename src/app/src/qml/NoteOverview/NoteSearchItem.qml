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
    // --------------------------------- tagpad ----------------------------------
    //----------------------------------------------------------------------------

        //proxy model for tag list :

        SKRSearchTagListProxyModel {
            id: noteOverviewTagProxyModel
        }
        searchTagPad.tagListModel: noteOverviewTagProxyModel

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
    searchListView.treelikeIndentsVisible: searchTextField.text.length === 0
    searchListView.treeIndentMultiplier: 20
    searchListView.elevationEnabled: true


}
