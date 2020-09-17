import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import eu.skribisto.notelistproxymodel 1.0
import eu.skribisto.writedocumentlistmodel 1.0
import eu.skribisto.skrusersettings 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import eu.skribisto.taghub 1.0
import ".."

RightDockForm {

    property int projectId : -2
    property int paperId : -2

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

    //-----------------------------------------------------------


    Connections {
        target: Globals
        function onOpenNoteCalled(projectId, paperId) {




        }
    }







    //-----------------------------------------------------------
    //---------------Tags :---------------------------------------------
    //-----------------------------------------------------------


    //proxy model for tag list :

    SKRSearchTagListProxyModel {
        id: tagProxyModel
        projectIdFilter: projectId
        sheetIdFilter: paperId
    }
    tagPadView.tagListModel: tagProxyModel

    Connections{
        target: tagPadView
        function onCallRemoveTagRelationship(projectId, itemId, tagId){
            plmData.tagHub().removeTagRelationship(projectId, SKRTagHub.Note , itemId, tagId)
        }
    }

    Connections{
        target: tagPadView
        function onCallAddTagRelationship(projectId, itemId, tagName){
            var error;
            // verify if name doesn't already exist :
            var tagId = plmData.tagHub().getTagIdWithName(projectId, tagName)

            if(tagId === -2){
                //if not, create tag
                error = plmData.tagHub().addTag(projectId, tagName)
                tagId = plmData.tagHub().getLastAddedId()
            }

            // set relationship
            error = plmData.tagHub().setTagRelationship(projectId, SKRTagHub.Note, itemId, tagId)
            if (!error.success){
                console.log("error onCallAddTagRelationship")
                //TODO: add notification
                return
            }
        }
    }

    onProjectIdChanged: {
        tagPadView.projectId = projectId
    }
    onPaperIdChanged: {
        tagPadView.itemId = paperId
    }

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
        category: "noteRightDock"
        property var dockSplitView
        property bool dockFolded: false
        property bool editFrameFolded: editFrame.folded ? true : false
        property bool tagPadFrameFolded: tagPadFrame.folded ? true : false
        //        property bool documentFrameFolded: documentFrame.folded ? true : false
        property int width: fixedWidth
    }




    PropertyAnimation {
        target: editFrame
        property: "SplitView.preferredHeight"
        duration: 500
        easing.type: Easing.InOutQuad
    }
    PropertyAnimation {
        target: tagPadFrame
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

        editFrame.folded = settings.editFrameFolded
        noteFrame.folded = settings.noteFrameFolded
        tagPadFrame.folded = settings.tagPadFrameFolded

        fixedWidth = settings.width
        splitView.restoreState(settings.dockSplitView)

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
