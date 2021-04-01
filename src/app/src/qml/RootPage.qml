import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import QtQml 2.15
import eu.skribisto.usersettings 1.0
import eu.skribisto.errorhub 1.0

import "Items"
import "Commons"
import "WelcomePage"



RootPageForm {
    id: rootPage



    //---------------------------------------------------------
    //---------------------------------------------------------

    themeColorButton.action: themeColorAction

    Action {
        id: themeColorAction

        icon.source: "qrc:///icons/backup/whitebalance.svg"
        onTriggered: {
            SkrTheme.light = !SkrTheme.light
        }
    }

    //------------------------------------------------------------
    //------------------------------------------------------------
    //------------------------------------------------------------

    Connections{
        target: ApplicationWindow.window
        function onCloseRightDrawerCalled(){
            rightDrawer.close()
        }
    }

    Connections{
        target: ApplicationWindow.window
        function onCloseLeftDrawerCalled(){
            leftDrawer.close()
        }
    }


    property alias leftDock: leftDock
    property int leftDrawerFixedWidth: 300

    SkrDrawer {
        id: leftDrawer
        parent: baseForDrawers
        widthInDockMode: leftDrawerFixedWidth
        widthInDrawerMode: 400
        height: baseForDrawers.height
        interactive: ApplicationWindow.window.compactMode
        dockModeEnabled: !ApplicationWindow.window.compactMode
        settingsCategory: "window_" + ApplicationWindow.window.windowId + "_leftDrawer"
        edge: Qt.LeftEdge



        LeftDock {
            id: leftDock
            anchors.fill: parent
            viewManager: rootPage.viewManager
            
            
            onHideDockCalled: {
                leftDrawer.close()
            }
        }
        onIsVisibleChanged: {
            if(isVisible){
                showLeftDockButtonWidth = 0
            }
            else {
                showLeftDockButtonWidth = 30
            }
        }

    }

    showLeftDockButton.action: showLeftDockAction
    Action {
        id: showLeftDockAction
        icon.source: "qrc:///icons/backup/go-next-view.svg"
        onTriggered: {
            leftDrawer.open()
        }
    }

    showLeftDockButton.visible: showLeftDockButtonWidth !== 0
    Behavior on showLeftDockButtonWidth {
        SpringAnimation {
            spring: 5
            mass: 0.2
            damping: 0.2
        }
    }

    //------------------------------------------------

    property alias rightDock: rightDock
    property int rightDrawerFixedWidth: 300
    SkrDrawer {
        id: rightDrawer
        parent: baseForDrawers
        widthInDockMode: rightDrawerFixedWidth
        widthInDrawerMode: 400
        height: baseForDrawers.height
        interactive: ApplicationWindow.window.compactMode
        dockModeEnabled: !ApplicationWindow.window.compactMode
        settingsCategory: "window_" + ApplicationWindow.window.windowId + "_rightDrawer"
        edge: Qt.RightEdge


        RightDock {
            id: rightDock
            anchors.fill: parent
            viewManager: rootPage.viewManager

            onHideDockCalled: {
                rightDrawer.close()
            }
        }

        onIsVisibleChanged: {
            if(isVisible){
                showRightDockButtonWidth = 0
            }
            else {
                showRightDockButtonWidth = 30
            }
        }

    }

    showRightDockButton.action: showRightDockAction
    Action {
        id: showRightDockAction
        icon.source: "qrc:///icons/backup/go-previous-view.svg"
        onTriggered: {
            rightDrawer.open()
        }
    }

    showRightDockButton.visible: showRightDockButtonWidth !== 0
    Behavior on showRightDockButtonWidth {
        SpringAnimation {
            spring: 5
            mass: 0.2
            damping: 0.2
        }
    }


    //------------------------------------------------------------
    // fullscreen :
    //------------------------------------------------------------

    property bool fullscreen_left_drawer_visible: false
    property bool fullscreen_right_drawer_visible: false
    Connections {
        target: Globals
        function onFullScreenCalled(value) {
            if(value){
                //save previous conf
                fullscreen_left_drawer_visible = leftDrawer.isVisible
                fullscreen_right_drawer_visible = rightDrawer.isVisible
                leftDrawer.close()
                rightDrawer.close()
            }
            else{
                if(fullscreen_left_drawer_visible){
                    leftDrawer.open()
                }
                else {
                    leftDrawer.close()
                }
                if(fullscreen_right_drawer_visible){
                    rightDrawer.open()
                }
                else {
                    rightDrawer.close()
                }
            }

        }
    }



    //---------------------------------------------------------
    //----welcome ----------------------------------------
    //---------------------------------------------------------

    showWelcomeButton.action: showWelcomeAction
    Action {
        id: showWelcomeAction
        text: qsTr("Show the welcome page")
        icon {
            source: "qrc:///pics/skribisto.svg"
            height: 50
            width: 50
            color: "transparent"
        }

        onTriggered: {
            welcomePopup.open()

        }
    }

    SkrPopup {
        id: welcomePopup
        parent: Overlay.overlay
        x: 0
        y: 0
        width: Overlay.overlay.width
        height: Overlay.overlay.height

        modal: true

        background: Rectangle {

            radius: 10
            color: SkrTheme.pageBackground

        }


        contentItem:
            WelcomePage {
            id: welcomePage
            onCloseCalled: welcomePopup.close()

        }

    }


    Connections {
        target: welcomePage
        function onCloseCalled(){
            rootWindow.protectedSignals.closeWelcomePopupCalled()
        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onOpenWelcomePopupCalled(){

            welcomePopup.open()
        }
    }


    Connections {
        target: rootWindow.protectedSignals
        function onCloseWelcomePopupCalled(){
            welcomePopup.close()
        }
    }

    //    Shortcut {
    //        sequence: ""
    //        context: Qt.WindowShortcut
    //        onActivated: goHomeAction.trigger()
    //    }



    //---------------------------------------------------------
    //----main menu ----------------------------------------
    //---------------------------------------------------------


    Connections {
        target: ApplicationWindow.window
        function onOpenMainMenuCalled(){
            mainMenuButton.checked = false
            mainMenuButton.checked = true
        }
    }
    mainMenuButton.objectName: "mainMenuButton"
    mainMenuButton.icon.source: "qrc:///icons/backup/application-menu.svg"

    function subscribeMainMenu() {
        skrEditMenuSignalHub.subscribe(mainMenuButton.objectName)
    }

    mainMenuButton.onCheckedChanged: {
        if(mainMenuButton.checked){
            var coordinates = mainMenuButton.mapToItem(rootPage, mainMenuButton.x, mainMenuButton.y)
            mainMenu.y = coordinates.y + mainMenuButton.height
            mainMenu.x = coordinates.x
            mainMenu.open()


        }
        else {
            mainMenu.dismiss()

        }
    }


    Shortcut {
        id: fileMenuShortcut
        sequence: skrQMLTools.mnemonic(fileMenu.title)
        onActivated: {
            Globals.openMainMenuCalled()
            mainMenu.openSubMenu(fileMenu)

        }
    }

    Shortcut {
        id: editMenuShortcut
        sequence: skrQMLTools.mnemonic(editMenu.title)
        onActivated: {
            Globals.openMainMenuCalled()
            mainMenu.openSubMenu(editMenu)

        }
    }

    Shortcut {
        id: viewMenuShortcut
        sequence: skrQMLTools.mnemonic(viewMenu.title)
        onActivated: {
            Globals.openMainMenuCalled()
            mainMenu.openSubMenu(viewMenu)

        }
    }

    Shortcut {
        id: helpMenuShortcut
        sequence: skrQMLTools.mnemonic(helpMenu.title)
        onActivated: {
            Globals.openMainMenuCalled()
            mainMenu.openSubMenu(helpMenu)

        }
    }

    Connections{
        target: ApplicationWindow.window
        function onOpenSubMenuCalled(menu) {
            Globals.openMainMenuCalled()
            mainMenu.openSubMenu(menu)
        }
    }

    SkrMenu {
        id: mainMenu

        onClosed: {
            mainMenuButton.checked = false
        }

        function findMenuIndex(menu){
            var i
            for(i = 0; i< mainMenu.count ; i++){
                if(menu.title === mainMenu.menuAt(i).title){

                    return i
                }
            }
        }

        function openSubMenu(menu){
            mainMenu.currentIndex = mainMenu.findMenuIndex(menu)
            menu.open()
            menu.currentIndex = 0
        }

        Component.onCompleted: {
            skrEditMenuSignalHub.subscribe(mainMenu.objectName)
        }

        SkrMenu {
            id: fileMenu
            title: qsTr("&File")

            SkrMenuItem{
                action: newProjectAction

            }
            SkrMenuItem{
                action: openProjectAction
            }
            MenuSeparator { }
            SkrMenuItem{
                height: skrRootItem.hasPrintSupport() ? undefined : 0
                visible: skrRootItem.hasPrintSupport()
                action: printAction
            }
            SkrMenuItem{
                action: importAction
            }
            SkrMenuItem{
                action: exportAction
            }

            MenuSeparator { }
            SkrMenuItem{
                action: saveAction
            }
            SkrMenuItem{
                action: saveAsAction
            }
            SkrMenuItem{
                action: saveAllAction
            }
            SkrMenuItem{
                action: saveACopyAction
            }
            SkrMenuItem{
                action: backUpAction
            }

            MenuSeparator { }
            SkrMenuItem{
                action: closeCurrentProjectAction
            }
            SkrMenuItem{
                action: quitAction
            }
        }
        SkrMenu {
            id: editMenu
            objectName: "editMenu"
            title: qsTr("&Edit")



            SkrMenuItem{
                id: cutItem
                objectName: "cutItem"
                action: cutTextAction
            }
            SkrMenuItem{
                id: copyItem
                objectName: "copyItem"
                action: copyTextAction
            }
            SkrMenuItem{
                id: pasteItem
                objectName: "pasteItem"
                action: pasteTextAction
            }

            Component.onCompleted:{
                skrEditMenuSignalHub.subscribe(editMenu.objectName)
                skrEditMenuSignalHub.subscribe(cutItem.objectName)
                skrEditMenuSignalHub.subscribe(copyItem.objectName)
                skrEditMenuSignalHub.subscribe(pasteItem.objectName)
            }

        }
        SkrMenu {
            id: viewMenu
            objectName: "viewMenu"
            title: qsTr("&View")

            SkrMenuItem{
                action: showWelcomeAction
                icon.color: "transparent"
            }

            //            SkrMenuItem{
            //                action: galleryWindowAction
            //            }

            MenuSeparator{}

            SkrMenuItem {
                //action: themePageAction
            }

            MenuSeparator{}

            SkrMenuItem {
                action: fullscreenAction
            }


        }
        SkrMenu {
            id: helpMenu
            objectName: "helpMenu"
            title: qsTr("&Help")

            SkrMenuItem {
                action: showHelpContentAction
            }

            SkrMenuItem {
                action: showFaqAction
            }

            SkrMenuItem {
                action: showAboutAction
            }

            SkrMenuItem {
                action: showAboutQtAction
            }

        }
        SkrMenuItem {
            id: bottomMenuItem
            objectName: "bottomMenuItem"
            focus: false

            onActiveFocusChanged: {
                if(activeFocus){
                    closeCurrentProjectToolButtonInMenuItem.forceActiveFocus()
                }
            }

            background: SkrPane { anchors.fill: parent}
            contentItem:
                RowLayout{
                anchors.fill: parent
                SkrToolButton{
                    id: closeCurrentProjectToolButtonInMenuItem
                    action: closeCurrentProjectAction
                    display: AbstractButton.IconOnly
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                    KeyNavigation.down: quitToolButtonInMenuItem

                }
                SkrToolButton{
                    id: quitToolButtonInMenuItem
                    action: quitAction
                    display: AbstractButton.IconOnly
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                }

            }



        }

    }

}
