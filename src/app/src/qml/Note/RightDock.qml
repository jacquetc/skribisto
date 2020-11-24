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
import "../Commons"
import "../Items"
import ".."

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
            }
        }
    ]





    //-----------------------------------------------------------
    //--------------- toolBoxes Behavior------------------------
    //-----------------------------------------------------------


    Settings {
        id: settings
        category: "noteRightDock"

        property bool editViewVisible: true
        property bool tagPadVisible: true
    }

    function loadConf(){

        editViewToolButton.checked = settings.editViewVisible
        tagPadViewToolButton.checked = settings.tagPadVisible

    }

    function resetConf(){
        settings.editViewVisible = true
        settings.tagPadVisible = true
    }

    Component.onCompleted: {
        loadConf()
        Globals.resetDockConfCalled.connect(resetConf)

    }

    Component.onDestruction: {
        settings.editViewVisible = editViewToolButton.checked
        settings.tagPadVisible = tagPadViewToolButton.checked

    }



    //-----------------------------------------------------------
    //---------------Edit---------------------------------------------
    //-----------------------------------------------------------
    editView.paperType: SKR.Note

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

    editView.skrSettingsGroup: SkrSettings.noteSettings




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
    tagPadView.itemType: SKR.Note


    //-----------------------------------------------------------
    //-----------------------------------------------------------
    //-----------------------------------------------------------

    onProjectIdChanged: {
        editView.projectId = projectId
        tagPadView.projectId = projectId
        tagPadView.itemId = -2
    }
    onPaperIdChanged: {
        editView.paperId = paperId
        tagPadView.itemId = paperId
    }



}
