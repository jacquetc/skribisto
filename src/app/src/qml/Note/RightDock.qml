import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import eu.skribisto.notelistproxymodel 1.0
import eu.skribisto.writedocumentlistmodel 1.0
import eu.skribisto.skrusersettings 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import eu.skribisto.taghub 1.0
import "../Commons"
import ".."

RightDockForm {

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


    Shortcut {
        id: toolMenuShortcut
        enabled: root.enabled

    }


    //-----------------------------------------------------------


    //Menu :
    property list<Component> menuComponents:  [
        Component{
        id:  toolDockMenuComponent
        Menu {
            id: toolDockMenu
            objectName: "toolDockMenu"
            title: qsTr("&Tools dock")


            Component.onCompleted: {

                toolMenuShortcut.sequence = skrQMLTools.mnemonic(title)
                toolMenuShortcut.activated.connect(function() {
                    Globals.openSubMenuCalled(toolDockMenu)
                })
            }

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
        }
    }
]



    //-----------------------------------------------------------



    //-----------------------------------------------------------
    //---------------Edit---------------------------------------------
    //-----------------------------------------------------------


    editView.skrSettingsGroup: SkrSettings.noteSettings




    //-----------------------------------------------------------
    //---------------Tags :---------------------------------------------
    //-----------------------------------------------------------


    //proxy model for tag list :

    SKRSearchTagListProxyModel {
        id: tagProxyModel
        projectIdFilter: projectId
        noteIdFilter: paperId
    }
    tagPadView.tagListModel: tagProxyModel
    tagPadView.itemType: SKRTagHub.Note


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
        property bool editFrameFolded: editFrame.folded
        property bool tagPadFrameFolded: tagPadFrame.folded
        //        property bool documentFrameFolded: documentFrame.folded ? true : false
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

    function loadConf(){

        editFrame.folded = settings.editFrameFolded
        tagPadFrame.folded = settings.tagPadFrameFolded

        splitView.restoreState(settings.dockSplitView)

    }

    function resetConf(){
        editFrame.folded = false
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
