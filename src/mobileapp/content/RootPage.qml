import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SkrControls
import Skribisto
import theme
import eu.skribisto.result

import "WelcomePage"

RootPageForm {
    id: rootPage

    Component.onCompleted: {
        populateProjectPageModel()
    }

    //---------------------------------------------------------
    //---------------------------------------------------------
    themeColorButton.action: themeColorAction

    Action {
        id: themeColorAction

        icon.source: "icons/3rdparty/backup/whitebalance.svg"
        onTriggered: {
            if (SkrTheme.light) {
                SkrTheme.setDark()
            } else {
                SkrTheme.setLight()
            }
        }
    }

    distractionFreeButton.action: fullscreenAction

    //-------------------------------------------------------------------------
    //---- notification ----------------------------------------------------------
    //-------------------------------------------------------------------------
    SkrToolButton {
        id: notificationButton

        width: 20 * SkrSettings.interfaceSettings.zoom
        height: 20 * SkrSettings.interfaceSettings.zoom
        x: rootPage.width - 20 * SkrSettings.interfaceSettings.zoom
        y: rootPage.height - 20 * SkrSettings.interfaceSettings.zoom

        padding: 0
        action: notificationButtonAction
    }

    Action {
        id: notificationButtonAction
        icon {
            source: "icons/3rdparty/backup/dialog-messages.svg"
            width: 50 * SkrSettings.interfaceSettings.zoom
            height: 50 * SkrSettings.interfaceSettings.zoom
        }
        checkable: true

        onTriggered: {
            //show notification list
            if (notificationPopup.visible
                    && !notificationButtonAction.checked) {
                notificationPopup.close()
                return
            }
            if (!notificationPopup.visible) {
                notificationPopup.open()
            }
        }
    }

    SkrPopup {
        id: notificationPopup
        property point windowPoint: rootPage.mapFromItem(
                                        notificationButton.parent,
                                        notificationButton.x,
                                        notificationButton.y)
        x: windowPoint.x - width + notificationButton.width
        y: windowPoint.y - height
        width: 300
        height: notificationListModel.count > 4 ? 4 * 120 : notificationListModel.count * 120
        modal: false
        focus: false
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
        clip: true
        enabled: visible

        onVisibleChanged: {
            if (!notificationPopup.visible) {
                notificationButtonAction.checked = false
            }
        }

        background: Item {}

        Connections {
            target: skrData.errorHub()
            function onSendNotification(errorType, content) {
                notificationListModel.append({
                                                 "errorType": errorType,
                                                 "content": content
                                             })

                if (!notificationPopup.visible) {
                    notificationPopup.open()
                    notificationButtonAction.checked = true
                }

                if (notificationCloseTimer.running) {
                    notificationCloseTimer.stop()
                    notificationCloseTimer.start()
                }
                notificationListView.positionViewAtEnd()
            }
        }
        Timer {
            id: notificationCloseTimer
            repeat: false
            interval: 5000
            onTriggered: {
                notificationButtonAction.checked = false
            }
        }

        ListModel {
            id: notificationListModel
        }

        ListView {
            id: notificationListView
            anchors.fill: parent
            model: notificationListModel
            delegate: Rectangle {

                height: 120
                width: notificationListView.width
                color: SkrTheme.pageBackground
                border.width: 1
                border.color: SkrTheme.buttonBackground
                radius: 10

                ColumnLayout {
                    anchors.fill: parent
                    spacing: 1

                    RowLayout {
                        Layout.fillWidth: true

                        SkrToolButton {
                            id: errorTypeButton
                            icon.source: {
                                var icon
                                switch (model.errorType) {
                                case 0:
                                    icon = "icons/3rdparty/backup/data-information.svg"
                                    break
                                case skrResult.Warning:
                                    icon = "icons/3rdparty/backup/data-warning.svg"
                                    break
                                case skrResult.Critical:
                                    icon = "icons/3rdparty/backup/data-error.svg"
                                    break
                                case skrResult.Fatal:
                                    icon = "icons/3rdparty/backup/data-error.svg"
                                    break
                                default:
                                    icon = "Unknown"
                                    break
                                }
                                return icon
                            }

                            icon.color: "transparent"
                        }

                        SkrLabel {
                            id: errorTypeLabel
                            Layout.fillWidth: true
                            text: {
                                var errorType
                                switch (model.errorType) {
                                case 0:
                                    errorType = qsTr("Ok")
                                    break
                                case skrResult.Warning:
                                    errorType = qsTr("Warning")
                                    break
                                case skrResult.Critical:
                                    errorType = qsTr("Critical")
                                    break
                                case skrResult.Fatal:
                                    errorType = qsTr("Fatal")
                                    break
                                default:
                                    errorType = "Unknown"
                                    break
                                }
                                return errorType
                            }
                        }
                        SkrToolButton {
                            id: copyErrorButton
                            icon.source: "icons/3rdparty/backup/edit-copy.svg"

                            onClicked: {
                                textArea.selectAll()
                                textArea.copy()
                                textArea.deselect()
                            }
                        }
                        SkrToolButton {
                            id: deleteErrorButton
                            icon.source: "icons/3rdparty/backup/window-close.svg"

                            onClicked: {
                                if (notificationListModel.count === 1) {
                                    notificationPopup.close()
                                }

                                notificationListModel.remove(model.index)
                            }
                        }
                    }
                    ScrollView {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        id: scrollView
                        padding: 1
                        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                        ScrollBar.vertical.policy: ScrollBar.AsNeeded

                        TextArea {
                            id: textArea
                            //width:  scrollView.width
                            readOnly: true
                            color: SkrTheme.buttonForeground
                            wrapMode: TextEdit.Wrap
                            text: model.content
                            topPadding: 0
                            bottomPadding: 0
                            background: SkrPane {}
                            selectByMouse: true

                            onTextChanged: {
                                textArea.cursorPosition = textArea.text.length
                            }
                        }
                    }
                }
            }
        }
    }

    //-------------------------------------------------------------------------
    //--------------------------------------------------------------------------
    //-------------------------------------------------------------------------
    Connections {
        target: rootWindow.protectedSignals
        function onFullScreenCalled(value) {
            if (value) {
                hideHeaderRowLayout()
            } else {
                showHeaderRowLayout()
            }
        }
    }

    Item {
        id: headerShowZone

        anchors.leftMargin: 0
        anchors.rightMargin: 50
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: 10
        visible: !headerRowLayout.visible && rootWindow.isDistractionFree
        TapHandler {
            onTapped: function (eventPoint) {

                if (eventPoint.device.type === PointerDevice.Mouse) {
                    Globals.touchUsed = false
                }
                if (eventPoint.device.type === PointerDevice.TouchScreen
                        | eventPoint.device.type === PointerDevice.Stylus) {
                    Globals.touchUsed = true

                    if (headerStayVisibleTimer.running) {
                        headerStayVisibleTimer.stop()
                    }
                    headerStayVisibleTimer.start()
                }

                showHeaderRowLayout()
            }
        }
        HoverHandler {
            acceptedDevices: PointerDevice.Mouse

            onHoveredChanged: {
                if (hovered) {
                    showHeaderRowLayout()
                }
            }
        }
    }

    headerStayVisibleHoverHandler.enabled: headerRowLayout.visible
                                           && rootWindow.isDistractionFree
    headerStayVisibleHoverHandler.onHoveredChanged: {
        if (!headerStayVisibleHoverHandler.hovered) {
            // leaving
            hideHeaderRowLayout()
        }
    }

    headerStayVisibleTapHandler.enabled: headerRowLayout.visible
                                         && rootWindow.isDistractionFree
    headerStayVisibleTapHandler.onTapped: function (eventPoint) {
        if (headerStayVisibleTimer.running) {
            headerStayVisibleTimer.stop()
        }
        headerStayVisibleTimer.start()
    }

    Timer {
        id: headerStayVisibleTimer
        interval: 5000
        onTriggered: {
            hideHeaderRowLayout()
        }
    }

    function showHeaderRowLayout() {
        //        if(SkrSettings.ePaperSettings.animationEnabled){
        //            showHeaderAnimation.start()
        //        }
        //        else{
        headerRowLayout.visible = true
        //        }
    }

    function hideHeaderRowLayout() {
        if (!rootWindow.isDistractionFree) {
            return
        }

        //        if(SkrSettings.ePaperSettings.animationEnabled){
        //            hideHeaderAnimation.start()
        //        }
        //        else{
        headerRowLayout.visible = false
        //        }
    }

    //    SequentialAnimation {
    //        id: hideHeaderAnimation

    //        NumberAnimation{
    //            //target: headerRowLayout
    //            property: "headerRowLayoutPreferredHeight"
    //            from: 30
    //            to: 0
    //            easing.type: Easing.InQuad
    //            duration: 10000
    //        }
    //        PropertyAction {
    //            target: headerRowLayout
    //            property: "visible"
    //            value: false
    //        }
    //    }
    //    SequentialAnimation {
    //        id: showHeaderAnimation

    //        PropertyAction {
    //            target: headerRowLayout
    //            property: "visible"
    //            value: true
    //        }
    //        NumberAnimation{
    //            //target: headerRowLayout
    //            property: "headerRowLayoutPreferredHeight"
    //            from: 0
    //            to: 30
    //            easing.type: Easing.InQuad
    //            duration: 10000
    //        }

    //    }

    //------------------------------------------------------------
    //------------------------------------------------------------
    //------------------------------------------------------------
    Connections {
        target: ApplicationWindow.window
        function onCloseRightDrawerCalled() {
            rightDrawer.close()
        }
    }

    Connections {
        target: ApplicationWindow.window
        function onCloseLeftDrawerCalled() {
            leftDrawer.close()
        }
    }

    property alias leftDock: leftDock
    property int leftDrawerFixedWidth: 300 * SkrSettings.interfaceSettings.zoom

    SkrDrawer {
        id: leftDrawer
        parent: baseForDrawers
        widthInDockMode: leftDrawerFixedWidth
        widthInDrawerMode: 400 * SkrSettings.interfaceSettings.zoom
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
            if (isVisible) {
                showLeftDockButtonWidth = 0
            } else {
                showLeftDockButtonWidth = 30 * SkrSettings.interfaceSettings.zoom
            }
        }
    }

    showLeftDockButton.action: showLeftDockAction
    Action {
        id: showLeftDockAction
        icon.source: "icons/3rdparty/backup/go-next-view.svg"
        onTriggered: {
            leftDrawer.open()
        }
    }

    showLeftDockButton.visible: showLeftDockButtonWidth !== 0
    Behavior on showLeftDockButtonWidth {
        enabled: SkrSettings.ePaperSettings.animationEnabled
        SpringAnimation {
            spring: 5
            mass: 0.2
            damping: 0.2
        }
    }

    //------------------------------------------------
    property alias rightDock: rightDock
    property int rightDrawerFixedWidth: 300 * SkrSettings.interfaceSettings.zoom
    SkrDrawer {
        id: rightDrawer
        parent: baseForDrawers
        widthInDockMode: rightDrawerFixedWidth
        widthInDrawerMode: 400 * SkrSettings.interfaceSettings.zoom
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
            if (isVisible) {
                showRightDockButtonWidth = 0
            } else {
                showRightDockButtonWidth = 30 * SkrSettings.interfaceSettings.zoom
            }
        }
    }

    showRightDockButton.action: showRightDockAction
    Action {
        id: showRightDockAction
        icon.source: "icons/3rdparty/backup/go-previous-view.svg"
        onTriggered: {
            rightDrawer.open()
        }
    }

    showRightDockButton.visible: showRightDockButtonWidth !== 0
    Behavior on showRightDockButtonWidth {
        enabled: SkrSettings.ePaperSettings.animationEnabled
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
        target: rootWindow.protectedSignals
        function onFullScreenCalled(value) {
            if (value) {
                //save previous conf
                fullscreen_left_drawer_visible = leftDrawer.isVisible
                fullscreen_right_drawer_visible = rightDrawer.isVisible
                leftDrawer.close()
                rightDrawer.close()
            } else {
                if (fullscreen_left_drawer_visible) {
                    leftDrawer.open()
                } else {
                    leftDrawer.close()
                }
                if (fullscreen_right_drawer_visible) {
                    rightDrawer.open()
                } else {
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
            source: "icons/skribisto.svg"
            height: 50 * SkrSettings.interfaceSettings.zoom
            width: 50 * SkrSettings.interfaceSettings.zoom
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
        width: Overlay.overlay.width >= 1000 * SkrSettings.interfaceSettings.zoom ? 1000 * SkrSettings.interfaceSettings.zoom : Overlay.overlay.width
        height: Overlay.overlay.height

        modal: true

        background: Rectangle {

            radius: 10
            color: SkrTheme.pageBackground
        }

        contentItem: WelcomePage {
            id: welcomePage
            onCloseCalled: welcomePopup.close()
        }
    }

    Connections {
        target: welcomePage
        function onCloseCalled() {
            rootWindow.protectedSignals.closeWelcomePopupCalled()
        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onOpenWelcomePopupCalled() {

            welcomePopup.open()
        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onCloseWelcomePopupCalled() {
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
        target: rootWindow
        function onOpenMainMenuCalled() {
            mainMenuButton.checked = false
            mainMenuButton.checked = true
        }
    }
    mainMenuButton.objectName: "mainMenuButton"
    mainMenuButton.icon.source: "icons/3rdparty/backup/application-menu.svg"

    function subscribeMainMenu() {
        skrEditMenuSignalHub.subscribe(mainMenuButton.objectName)
    }

    mainMenuButton.onCheckedChanged: {
        if (mainMenuButton.checked) {
            var coordinates = mainMenuButton.mapToItem(rootPage,
                                                       mainMenuButton.x,
                                                       mainMenuButton.y)
            mainMenu.y = coordinates.y + mainMenuButton.height
            mainMenu.x = coordinates.x
            mainMenu.open()
        } else {
            mainMenu.dismiss()
        }
    }

    Shortcut {
        id: fileMenuShortcut
        sequence: skrQMLTools.mnemonic(fileMenu.title)
        onActivated: {
            rootWindow.openMainMenuCalled()
            mainMenu.openSubMenu(fileMenu)
        }
    }

    Shortcut {
        id: editMenuShortcut
        sequence: skrQMLTools.mnemonic(editMenu.title)
        onActivated: {
            rootWindow.openMainMenuCalled()
            mainMenu.openSubMenu(editMenu)
        }
    }

    Shortcut {
        id: viewMenuShortcut
        sequence: skrQMLTools.mnemonic(viewMenu.title)
        onActivated: {
            rootWindow.openMainMenuCalled()
            mainMenu.openSubMenu(viewMenu)
        }
    }

    Shortcut {
        id: helpMenuShortcut
        sequence: skrQMLTools.mnemonic(helpMenu.title)
        onActivated: {
            rootWindow.openMainMenuCalled()
            mainMenu.openSubMenu(helpMenu)
        }
    }

    Connections {
        target: ApplicationWindow.window
        function onOpenSubMenuCalled(menu) {
            rootWindow.openMainMenuCalled()
            mainMenu.openSubMenu(menu)
        }
    }

    SkrMenu {
        id: mainMenu

        onClosed: {
            mainMenuButton.checked = false
        }

        function findMenuIndex(menu) {
            var i
            for (i = 0; i < mainMenu.count; i++) {
                if (mainMenu.menuAt(i)) {
                    if (menu.title === mainMenu.menuAt(i).title) {

                        return i
                    }
                }
            }
        }

        function openSubMenu(menu) {
            mainMenu.currentIndex = mainMenu.findMenuIndex(menu)
            menu.open()
            menu.currentIndex = 0
        }

        Component.onCompleted: {
            skrEditMenuSignalHub.subscribe(mainMenu.objectName)
        }

        SkrMenu {
            id: fileMenu
            objectName: "fileMenu"
            title: qsTr("&File")

            SkrMenuItem {
                action: newProjectAction
            }
            SkrMenuItem {
                action: openProjectAction
            }
            MenuSeparator {}
            SkrMenuItem {
                height: skrRootItem.hasPrintSupport() ? undefined : 0
                visible: skrRootItem.hasPrintSupport()
                action: printAction
            }
            SkrMenuItem {
                action: importAction
            }
            SkrMenuItem {
                action: exportAction
            }

            MenuSeparator {}
            SkrMenuItem {
                action: saveAction
            }
            SkrMenuItem {
                action: saveAsAction
            }
            SkrMenuItem {
                action: saveAllAction
            }
            SkrMenuItem {
                action: saveACopyAction
            }
            SkrMenuItem {
                action: backUpAction
            }

            MenuSeparator {}
            SkrMenuItem {
                action: closeCurrentProjectAction
            }
            SkrMenuItem {
                action: quitAction
            }
        }
        SkrMenu {
            id: editMenu
            objectName: "editMenu"
            title: qsTr("&Edit")

            SkrMenuItem {
                id: cutItem
                objectName: "cutItem"
                action: cutTextAction
            }
            SkrMenuItem {
                id: copyItem
                objectName: "copyItem"
                action: copyTextAction
            }
            SkrMenuItem {
                id: pasteItem
                objectName: "pasteItem"
                action: pasteTextAction
            }

            Component.onCompleted: {
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

            SkrMenuItem {
                action: showWelcomeAction
                icon.color: "transparent"
            }

            //            SkrMenuItem{
            //                action: galleryWindowAction
            //            }
            MenuSeparator {}

            SkrMenuItem {
                action: themesColorAction
            }
            SkrMenuItem {
                action: firstStepsAction
            }

            MenuSeparator {}

            SkrMenuItem {
                action: fullscreenAction
            }
        }

        SkrMenuItem {
            action: showSettingsAction
        }

        SkrMenu {
            id: helpMenu
            objectName: "helpMenu"
            title: qsTr("&Help")

            SkrMenuItem {
                action: showUserManualAction
            }

            SkrMenuItem {
                action: showFaqAction
            }

            SkrMenuItem {
                action: showDiscordAction
            }

            SkrMenuItem {
                action: showGitHubAction
            }

            SkrMenuItem {
                action: showSkribistoWebsiteAction
            }

            SkrMenuItem {
                action: showAboutAction
            }

            SkrMenuItem {
                action: showAboutQtAction
            }
        }

        SkrMenuItem {
            action: closeCurrentProjectAction
        }

        SkrMenuItem {
            action: quitAction
        }
    }

    //--------------------------------------------------------------
    //----- Top toolbar-----------------------------------------------
    //--------------------------------------------------------------
    ListModel {
        id: projectPageModel
    }

    function populateProjectPageModel() {
        projectPageModel.clear()

        var list = skrTreeManager.findProjectPageNamesForLocation("top-toolbar")

        for (var i in list) {
            var name = list[i]
            var type = skrTreeManager.findProjectPageType(name)
            var iconSource = skrTreeManager.findProjectPageIconSource(name)
            var showButtonText = skrTreeManager.findProjectPageShowButtonText(
                        name)
            var shortcutSequences = skrTreeManager.findProjectPageShortcutSequences(
                        name)

            projectPageModel.append({
                                        "name": name,
                                        "type": type,
                                        "iconSource": iconSource,
                                        "showButtonText": showButtonText,
                                        "shortcutSequences": shortcutSequences
                                    })
        }
    }

    topToolBarRepeater.model: projectPageModel

    topToolBarRepeater.delegate: SkrToolButton {
        property string name: model.name
        text: model.showButtonText

        icon.source: model.iconSource

        onClicked: {

            var activeProjectId = skrData.projectHub().activeProjectId
            if (activeProjectId === -2) {
                return
            }
            viewManager.loadProjectDependantPage(activeProjectId, model.type)
        }

        Shortcut {

            sequences: model.shortcutSequences
            onActivated: {

            }
        }
    }
}
