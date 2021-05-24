import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import QtQml 2.15
import eu.skribisto.writedocumentlistmodel 1.0
import eu.skribisto.usersettings 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import eu.skribisto.taghub 1.0
import eu.skribisto.skr 1.0
import ".."
import "../Items"

RightDockForm {

    property int projectId : -2
    property int paperId : -2

    SKRUserSettings {
        id: skrUserSettings
    }


    //-----------------------------------------------------------


    Shortcut {
        id: toolMenuShortcut
        enabled: root.enabled

    }


    //-----------------------------------------------------------


    //Menu :
    property list<Component> menuComponents:  [
        Component{
        id:  toolDockMenuComponent
        SkrMenu {
            id: toolDockMenu
            objectName: "toolDockMenu"
            title: qsTr("&Tools dock")


            Component.onCompleted: {

                toolMenuShortcut.sequence = skrQMLTools.mnemonic(title)
                toolMenuShortcut.activated.connect(function() {
                    Globals.openSubMenuCalled(toolDockMenu)
                })
            }


            SkrMenuItem {
                text: qsTr( "&Overview")
                onTriggered: {

                    if(Globals.compactMode){
                        rightDrawer.open()
                    }
                    sheetOverviewToolViewToolButton.checked = true
                    sheetOverviewToolView.forceActiveFocus()
                }
            }

            SkrMenuItem {
                text: qsTr( "&Edit")
                onTriggered: {

                    if(Globals.compactMode){
                        rightDrawer.open()
                    }
                    editViewToolButton.checked = true
                    editView.forceActiveFocus()
                }
            }

            SkrMenuItem {
                text: qsTr( "&Properties")
                onTriggered: {

                    if(Globals.compactMode){
                        rightDrawer.open()
                    }
                    propertyPadViewToolButton.checked = true
                    propertyPadView.forceActiveFocus()
                }
            }


            SkrMenuItem {
                text: qsTr( "&Tags")
                onTriggered: {

                    if(Globals.compactMode){
                        rightDrawer.open()
                    }
                    tagPadViewToolButton.checked = true
                    tagPadView.forceActiveFocus()
                }
            }
        }
    }
]


    //-----------------------------------------------------------
    //--------------- toolboxes Behavior------------------------
    //-----------------------------------------------------------


    Settings {
        id: settings
        category: "writeOverviewRightDock"

        property bool sheetOverviewToolViewVisible: true
        property bool editViewVisible: true
        property bool tagPadVisible: true
    }

    function loadConf(){

        sheetOverviewToolViewToolButton.checked = settings.sheetOverviewToolViewVisible
        editViewToolButton.checked = settings.editViewVisible
        tagPadViewToolButton.checked = settings.tagPadVisible


    }

    function resetConf(){
        settings.sheetOverviewToolViewVisible = true
        settings.editViewVisible = true
        settings.tagPadVisible = true
    }

    Component.onCompleted: {
        loadConf()
        Globals.resetDockConfCalled.connect(resetConf)

    }

    Component.onDestruction: {
        settings.sheetOverviewToolViewVisible = sheetOverviewToolViewToolButton.checked
        settings.editViewVisible = editViewToolButton.checked
        settings.tagPadVisible = tagPadViewToolButton.checked

    }



    //-----------------------------------------------------------

    //-----------------------------------------------------------
    //---------------Overview---------------------------------------
    //-----------------------------------------------------------

    Action{
        id: sheetOverviewToolViewAction
        checkable: true
        text: qsTr( "Show overview tool box")
        icon {
            source: "qrc:///icons/backup/configure.svg"
            height: 50
            width: 50
        }
        onCheckedChanged: {
            sheetOverviewToolView.visible = sheetOverviewToolViewAction.checked

        }


        Binding on checked{
            value: sheetOverviewToolView.visible
            delayed: true
            restoreMode: Binding.RestoreBindingOrValue
        }

    }


    sheetOverviewToolViewToolButton.action: sheetOverviewToolViewAction


    //-----------------------------------------------------------
    //---------------Edit---------------------------------------------
    //-----------------------------------------------------------

    editView.paperType: SKR.Sheet

    Action{
        id: editViewAction
        checkable: true
        text: qsTr( "Show edit tool box")
        icon {
            source: "qrc:///icons/backup/format-text-italic.svg"
            height: 50
            width: 50
        }
        onCheckedChanged: {
            editView.visible = editViewAction.checked

        }


        Binding on checked{
            value: editView.visible
            delayed: true
            restoreMode: Binding.RestoreBindingOrValue
        }

    }


    editViewToolButton.action: editViewAction

    editView.skrSettingsGroup: SkrSettings.overviewTreeNoteSettings
    editView.textWidthSliderVisible: false


    //-----------------------------------------------------------
    //---------------properties :--------------------------------
    //-----------------------------------------------------------



    Action{
        id: propertyPadViewAction
        checkable: true
        text: qsTr( "Show properties tool box")
        icon {
            source: "qrc:///icons/backup/configure.svg"
            height: 50
            width: 50
        }
        onCheckedChanged: {
            propertyPadView.visible = checked
        }

        Binding on checked{
            value: propertyPadView.visible
            delayed: true
            restoreMode: Binding.RestoreBindingOrValue
        }
    }
    propertyPadViewToolButton.action: propertyPadViewAction

    propertyPadView.propertyHub: skrData.sheetPropertyHub()
    propertyPadView.paperHub: skrData.sheetHub()


    //-----------------------------------------------------------
    //---------------Tags :---------------------------------------------
    //-----------------------------------------------------------

    Action{
        id: tagPadViewAction
        checkable: true
        text: qsTr( "Show tags tool box")
        icon {
            source: "qrc:///icons/backup/tag.svg"
            height: 50
            width: 50
        }
        onCheckedChanged: {
            tagPadView.visible = tagPadViewAction.checked
        }

        Binding on checked{
            value: tagPadView.visible
            delayed: true
            restoreMode: Binding.RestoreBindingOrValue
        }
    }
    tagPadViewToolButton.action: tagPadViewAction

    //proxy model for tag list :

    SKRSearchTagListProxyModel {
        id: tagProxyModel
        projectIdFilter: projectId
        noteIdFilter: paperId
    }
    tagPadView.tagListModel: tagProxyModel

    Connections{
        target: tagPadView
        function onCallRemoveTagRelationship(projectId, itemId, tagId){
            skrData.tagHub().removeTagRelationship(projectId, SKR.Note , itemId, tagId)
        }
    }

    Connections{
        target: tagPadView
        function onCallAddTagRelationship(projectId, itemId, tagName){
            var result;
            // verify if name doesn't already exist :
            var tagId = skrData.tagHub().getTagIdWithName(projectId, tagName)

            if(tagId === -2){
                //if not, create tag
                result = skrData.tagHub().addTag(projectId, tagName)
                tagId = skrData.tagHub().getLastAddedId()
            }

            // set relationship
            result = skrData.tagHub().setTagRelationship(projectId, SKR.Note, itemId, tagId)
            if (!result){
                console.log("result onCallAddTagRelationship")
                //TODO: add notification
                return
            }
        }
    }

    onProjectIdChanged: {
        editView.projectId = projectId
        tagPadView.projectId = projectId
        tagPadView.itemId = -2
        propertyPadView.projectId = projectId
    }
    onPaperIdChanged: {
        editView.paperId = paperId
        tagPadView.itemId = paperId
        propertyPadView.paperId = paperId
    }

    //-----------------------------------------------------------
    //-----------------------------------------------------------
    //-----------------------------------------------------------

}
