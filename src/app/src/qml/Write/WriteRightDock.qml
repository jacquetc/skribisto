import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import eu.skribisto.sheetlistproxymodel 1.0
import eu.skribisto.writedocumentlistmodel 1.0
import eu.skribisto.usersettings 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import eu.skribisto.taghub 1.0
import ".."
import "../Items"

WriteRightDockForm {

    property int projectId : -2
    property int paperId : -2

    SKRUserSettings {
        id: skrUserSettings
    }


    splitView.handle: Item {
        id: handle
        implicitHeight: 8
        property bool hovered: SplitHandle.hovered

        RowLayout {
            anchors.fill: parent
            Rectangle {
                Layout.preferredWidth: 20
                Layout.preferredHeight: 5
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                color: handle.hovered ? SkrTheme.accent : SkrTheme.divider
            }
        }
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

                        if(Globals.compactSize){
                            rightDrawer.open()
                        }
                        editFrame.folded = false
                        editView.forceActiveFocus()
                    }
                }


                SkrMenuItem {
                    text: qsTr( "&Tags")
                    onTriggered: {

                        if(Globals.compactSize){
                            rightDrawer.open()
                        }
                        tagPadFrame.folded = false
                        tagPadView.forceActiveFocus()
                    }
                }

                SkrMenuItem {
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
    //---------------Edit---------------------------------------------
    //-----------------------------------------------------------


    editView.skrSettingsGroup: SkrSettings.writeSettings





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
    tagPadView.itemType: SKRTagHub.Sheet



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
        tagPadView.itemId = -2
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
