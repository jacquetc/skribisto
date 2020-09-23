import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import eu.skribisto.sheetlistproxymodel 1.0
import eu.skribisto.writedocumentlistmodel 1.0
import eu.skribisto.skrusersettings 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import eu.skribisto.taghub 1.0
import ".."

WriteRightDockForm {

    property int projectId : -2
    property int paperId : -2

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


    //-----------------------------------------------------------

    //Menu :
    property list<Component> menuComponents:  [
        Component{
        id:  toolMenuComponent
        Menu {
            id: toolDockMenu
            title: qsTr("&Tools dock")

            MenuItem {
                text: qsTr( "&Edit")
                onTriggered: {

                    if(Globals.compactSize){
                        rightDrawer.open()
                    }
                    editFrame.folded = false
                    editView.forceActiveFocus()
                }
            }


            MenuItem {
                text: qsTr( "&Tags")
                onTriggered: {

                    if(Globals.compactSize){
                        rightDrawer.open()
                    }
                    tagPadFrame.folded = false
                    tagPadView.forceActiveFocus()
                }
            }

            MenuItem {
                text: qsTr( "&Notes")
                onTriggered: {

                    if(Globals.compactSize){
                        rightDrawer.open()
                    }
                    notePadFrame.folded = false
                    notePadView.forceActiveFocus()
                }
            }
        }
    }
]




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
            plmData.tagHub().removeTagRelationship(projectId, SKRTagHub.Sheet , itemId, tagId)
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
            error = plmData.tagHub().setTagRelationship(projectId, SKRTagHub.Sheet, itemId, tagId)
            if (!error.success){
                console.log("error onCallAddTagRelationship")
                //TODO: add notification
                return
            }
        }
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
        category: "writeRightDock"
        property var dockSplitView
        property bool editFrameFolded: editFrame.folded
        property bool notePadFrameFolded: notePadFrame.folded
        property bool tagPadFrameFolded: tagPadFrame.folded
        //        property bool documentFrameFolded: documentFrame.folded ? true : false
    }


    onProjectIdChanged: {
        notePadView.projectId = projectId
        tagPadView.projectId = projectId
    }
    onPaperIdChanged: {
        notePadView.sheetId = paperId
        tagPadView.itemId = paperId
    }


    PropertyAnimation {
        target: editFrame
        property: "SplitView.preferredHeight"
        duration: 500
        easing.type: Easing.InOutQuad
    }
    PropertyAnimation {
        target: notePadFrame
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

    function loadConf(){

        editFrame.folded = settings.editFrameFolded
        notePadFrame.folded = settings.notePadFrameFolded
        tagPadFrame.folded = settings.tagPadFrameFolded

        var result = splitView.restoreState(settings.dockSplitView)

    }

    function resetConf(){
        editFrame.folded = false
        notePadFrame.folded = false
        tagPadFrame.folded = false
        splitView.restoreState("")

    }

    Component.onCompleted: {
        loadConf()
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
