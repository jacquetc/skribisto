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

WriteRightDockForm {

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
                    text: qsTr( "&Tags")
                    onTriggered: {

                        if(Globals.compactMode){
                            rightDrawer.open()
                        }
                        tagPadViewToolButton.checked = true
                        tagPadView.forceActiveFocus()
                    }
                }

                SkrMenuItem {
                    text: qsTr( "&Notes")
                    onTriggered: {

                        if(Globals.compactMode){
                            rightDrawer.open()
                        }
                        notePadViewToolButton.checked = true
                        notePadView.forceActiveFocus()
                    }
                }
            }
        }
    ]

    //-----------------------------------------------------------
    //--------------- toolBoxes Behavior------------------------
    //-----------------------------------------------------------


    Settings {
        id: settings
        category: "writeRightDock"

        property bool editViewVisible: true
        property bool propertyPadVisible: true
        property bool tagPadVisible: true
        property bool notePadVisible: true
    }

    function loadConf(){

        editViewToolButton.checked = settings.editViewVisible
        propertyPadViewToolButton.checked = settings.propertyPadVisible
        tagPadViewToolButton.checked = settings.tagPadVisible
        notePadViewToolButton.checked = settings.notePadVisible

    }

    function resetConf(){
        settings.editViewVisible = true
        settings.propertyPadVisible = true
        settings.tagPadVisible = true
        settings.notePadVisible = true
    }

    Component.onCompleted: {
        loadConf()
        Globals.resetDockConfCalled.connect(resetConf)

    }

    Component.onDestruction: {
        settings.editViewVisible = editViewToolButton.checked
        settings.propertyPadVisible = propertyPadViewToolButton.checked
        settings.tagPadVisible = tagPadViewToolButton.checked
        settings.notePadVisible = notePadViewToolButton.checked

    }





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
            editView.visible = checked
        }


        Binding on checked{
            value: editView.visible
            delayed: true
            restoreMode: Binding.RestoreBindingOrValue
        }

    }


    editViewToolButton.action: editViewAction


    editView.skrSettingsGroup: SkrSettings.writeSettings




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

    propertyPadView.propertyHub: plmData.sheetPropertyHub()
    propertyPadView.paperHub: plmData.sheetHub()


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
            tagPadView.visible = checked
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
        sheetIdFilter: paperId
    }
    tagPadView.tagListModel: tagProxyModel
    tagPadView.itemType: SKR.Sheet


    //-----------------------------------------------------------
    //------- Notes------------------------------------------
    //-----------------------------------------------------------

    Action{
        id: notePadViewAction
        checkable: true
        text: qsTr( "Show notes tool box")
        icon {
            source: "qrc:///icons/backup/story-editor.svg"
            height: 50
            width: 50
        }
        onCheckedChanged: {
            notePadView.visible = checked
        }

        Binding on checked{
            value: notePadView.visible
            delayed: true
            restoreMode: Binding.RestoreBindingOrValue
        }
    }
    notePadViewToolButton.action: notePadViewAction



    //-----------------------------------------------------------
    //-----------------------------------------------------------
    //-----------------------------------------------------------


    onProjectIdChanged: {
        editView.projectId = projectId
        notePadView.projectId = projectId
        tagPadView.projectId = projectId
        tagPadView.itemId = -2
        propertyPadView.projectId = projectId
    }
    onPaperIdChanged: {
        editView.paperId = paperId
        notePadView.sheetId = paperId
        notePadView.openSynopsis()
        tagPadView.itemId = paperId
        propertyPadView.paperId = paperId
    }



}
