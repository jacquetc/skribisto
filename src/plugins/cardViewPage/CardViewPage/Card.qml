import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml 2.15
import QtQuick.Controls.Material 2.15
import eu.skribisto.projecthub 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import eu.skribisto.taghub 1.0
import eu.skribisto.skr 1.0
import "../../Commons"
import "../.."

CardForm {
    id: root

    TapHandler{
        id: tapHandler

        onSingleTapped: {
            forceActiveFocus()
        }
    }

    onActiveFocusChanged: {
        if(activeFocus){
        root.GridView.view.currentProjectId = model.projectId
        root.GridView.view.currentTreeItemId = model.treeItemId
        root.GridView.view.currentIndex = model.index
        }
    }


    Component {
        id: noteWritingZoneComponent

        OutlineWritingZone {
            id: writingZone

            property string pageType: model.type

            clip: true
            projectId: model.projectId
            treeItemId: model.treeItemId
            spellCheckerKilled: true
            leftScrollItemVisible: false
            rightScrollItemVisible: Globals.touchUsed
            placeholderText: qsTr("Outline")

            textPointSize: SkrSettings.cardViewOutlineSettings.textPointSize
            textFontFamily: SkrSettings.cardViewOutlineSettings.textFontFamily
            textIndent: SkrSettings.cardViewOutlineSettings.textIndent
            textTopMargin: SkrSettings.cardViewOutlineSettings.textTopMargin

            stretch: true

            textAreaStyleBackgroundColor: SkrTheme.secondaryTextAreaBackground
            textAreaStyleForegroundColor: SkrTheme.secondaryTextAreaForeground
            paneStyleBackgroundColor: SkrTheme.listItemBackground
            textAreaStyleAccentColor: SkrTheme.accent
            name: "cardViewOutline"

        }
    }

    pageTypeIcon.source: skrTreeManager.getIconUrlFromPageType(model.type)


    function getIconUrlFromPageType(type) {
        return skrTreeManager.getIconUrlFromPageType(type)
    }

    goToChildToolButton.onClicked: root.GridView.view.currentParentId = model.treeItemId

}
