import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import eu.skribisto.projecthub 1.0

import "../../Items"
import "../../Commons"
import "../.."

FavoritesProjectToolboxForm {
    id: root

    iconSource: "qrc:///icons/backup/favorite.svg"
    showButtonText: qsTr( "Show Favorites toolbox")




    navigationProxyModel.projectIdFilter: skrData.projectHub().activeProjectId


    function getIconUrlFromPageType(type){
        return skrTreeManager.getIconUrlFromPageType(type)
    }

    listView.delegate: ItemDelegate {
        text: model.title

        icon.source: getIconUrlFromPageType(model.type)

        onClicked: {
        var viewManager = root.ApplicationWindow.window.viewManager
        var position = viewManager.focusedPosition
            viewManager.loadTreeItemAt(model.projectId, model.treeItemId, position)

        }
    }

}
