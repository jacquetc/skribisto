import QtQuick
import QtQml
import QtQuick.Controls

import SkrControls
import Skribisto


TextPageForm {
    id: root

    pageType: "TEXT"

    property string title: ""

    function getTitle() {
        var fetchedTitle = skrData.treeHub().getTitle(projectId, treeItemId)

        if (isSecondary) {
            return qsTr("Plan of %1").arg(fetchedTitle)
        } else {
            return fetchedTitle
        }
    }

    Connections {
        target: skrData.treeHub()
        function onTitleChanged(_projectId, _treeItemId, newTitle) {
            if (projectId === _projectId && treeItemId === _treeItemId) {
                title = getTitle()
            }
        }
    }

    titleLabel.text: title

    //--------------------------------------------------------
    additionalPropertiesForSavingView: {
        return {
            "isSecondary": isSecondary,
            "milestone": milestone
        }
    }

    //--------------------------------------------------------
    //---View buttons-----------------------------------------
    //--------------------------------------------------------
    viewButtons.viewManager: root.viewManager
    viewButtons.position: root.position

    viewButtons.onOpenInNewWindowCalled: {
        saveContent(true)
        saveCurrentCursorPositionAndY()
        skrWindowManager.insertAdditionalPropertyForViewManager("isSecondary",
                                                                isSecondary)
        skrWindowManager.addWindowForItemId(projectId, treeItemId)
        rootWindow.setNavigationTreeItemIdCalled(projectId, treeItemId)
    }

    viewButtons.onSplitCalled: function (position) {
        saveContent(true)
        saveCurrentCursorPositionAndY()
        viewManager.insertAdditionalProperty("isSecondary", isSecondary)
        viewManager.loadTreeItemAt(projectId, treeItemId, position)
    }

    //--------------------------------------------------------
    //---Tool bar-----------------------------------------
    //--------------------------------------------------------
    Connections {
        target: skrData.treePropertyHub()
        function onPropertyChanged(projectId, propertyId, treeItemId, name, value) {
            if (projectId === root.projectId
                    && treeItemId === root.treeItemId) {

                if (name === "word_count") {
                    countPriv.wordCount = value
                    updateCountLabel()
                }
                if (name === "char_count") {
                    countPriv.characterCount = value
                    updateCountLabel()
                }
                if (name === "word_count_goal") {
                    countPriv.wordCountGoal = value
                    updateCountLabel()
                }
                if (name === "char_count_goal") {
                    countPriv.characterCountGoal = value
                    updateCountLabel()
                }
            }
        }
    }


    QtObject {
        id: countPriv
        property int wordCount: 0
        property int characterCount: 0
        property int wordCountGoal: 0
        property int characterCountGoal: 0
    }


    Connections{
        target: SkrSettings.interfaceSettings
        function onWordCountVisibleChanged(){
            updateCountLabel()
        }
    }

    Connections{
        target: SkrSettings.interfaceSettings
        function onCharCountVisibleChanged(){
            updateCountLabel()
        }
    }


    function updateCountLabel() {
        var wordCountString = skrRootItem.toLocaleIntString(countPriv.wordCount)
        var wordCountGoalString = skrRootItem.toLocaleIntString(countPriv.wordCountGoal)
        var characterCountString = skrRootItem.toLocaleIntString(
                    countPriv.characterCount)
        var characterCountGoalString = skrRootItem.toLocaleIntString(
                    countPriv.characterCountGoal)

        if(SkrSettings.interfaceSettings.wordCountVisible){
            countLabel.text =  countPriv.wordCountGoal > 0 ? qsTr("%1 / %2 words").arg(wordCountString).arg(wordCountGoalString) : qsTr("%1 words").arg(wordCountString)
        }
        else if(SkrSettings.interfaceSettings.characterCountVisible){
            countLabel.text = countPriv.characterCountGoal > 0 ? qsTr("%1 / %2 characters").arg(characterCountString).arg(characterCountGoalString) :qsTr("%1 characters").arg(characterCountString)
        }
        else {
            countLabel.text = ""
        }
    }

    //-----------------------------------------------------------------
    //----- Page menu ------------------------------------------------
    //-----------------------------------------------------------------
    pageMenuToolButton.onClicked: {
        if (pageMenu.visible) {
            pageMenu.close()
            return
        }
        pageMenu.open()
    }

    SkrMenu {
        id: pageMenu
        y: pageMenuToolButton.height
        x: pageMenuToolButton.x

        SkrMenuItem {
            action: newIdenticalPageAction
        }

        SkrMenuItem {
            action: Action {
                id: showRelationshipPanelAction
                text: skrShortcutManager.description("show-relationship-panel")
                icon.source: "qrc:///icons/backup/link.svg"
                onTriggered: {

                    relationshipPanel.visible = true
                    relationshipPanel.forceActiveFocus()
                }
            }
        }

        SkrMenuItem {
            action: Action {
                id: addQuickNoteAction
                text: skrShortcutManager.description("add-quick-note")
                icon.source: "qrc:///icons/backup/list-add.svg"
                onTriggered: {
                    relationshipPanel.visible = true
                    var result = skrData.treeHub().addQuickNote(projectId,
                                                                treeItemId,
                                                                "TEXT",
                                                                qsTr("Note"))
                    if (result.success) {
                        var newId = result.getData("treeItemId", -2)

                        relationshipPanel.openTreeItemInPanel(projectId, newId)
                        relationshipPanel.forceActiveFocus()
                    }
                }
            }
        }

        SkrMenuItem {
            action: Action {
                id: setGoalAction
                text: skrShortcutManager.description("set-goal")
                icon.source: "qrc:///icons/backup/list-add.svg"
                onTriggered: {
                    itemWordGoalDialog.projectId = projectId
                    itemWordGoalDialog.treeItemId = treeItemId
                    itemWordGoalDialog.open()
                }
            }
        }
    }

    ItemWordGoalDialog{
        id: itemWordGoalDialog
    }


    Shortcut {
        sequences: skrShortcutManager.shortcuts("add-quick-note")
        enabled: root.activeFocus
        onActivated: {
            addQuickNoteAction.trigger()
        }
    }
    Shortcut {
        sequences: skrShortcutManager.shortcuts("show-relationship-panel")
        enabled: root.activeFocus
        onActivated: {
            showRelationshipPanelAction.trigger()
        }
    }

    //--------------------------------------------------------
    //---Writing Zone-----------------------------------------
    //--------------------------------------------------------
    property bool isSecondary: false
    property int milestone: -2

    writingZone.maximumTextAreaWidth: SkrSettings.textSettings.textWidth
    writingZone.textPointSize: SkrSettings.textSettings.textPointSize
    writingZone.textFontFamily: SkrSettings.textSettings.textFontFamily
    writingZone.textIndent: SkrSettings.textSettings.textIndent
    writingZone.textTopMargin: SkrSettings.textSettings.textTopMargin

    writingZone.stretch: root.width < 300 ? true : viewManager.rootWindow.compactMode
    // focus
    Connections {
        enabled: viewManager.focusedPosition === position
        target: viewManager.rootWindow
        function onForceFocusOnEscapePressed() {
            writingZone.forceActiveFocus()
        }
    }

    //---------------------------------------------------------
    Component.onCompleted: {
        openDocument(projectId, treeItemId, isSecondary, milestone)
        determineIfPreviousWritingMustBeVisible()
        determineAndAddToolboxPlugins()
    }

    //---------------------------------------------------------
    function clearWritingZone() {
        if (root.treeItemId !== -2 && root.projectId !== -2
                && milestone === -2) {
            contentSaveTimer.stop()
            saveContent(true)
            saveCurrentCursorPositionAndYTimer.stop()
            saveCurrentCursorPositionAndY()
            var uniqueDocumentReference = projectId + "_" + treeItemId + "_"
                    + (isSecondary ? "secondary" : "primary")
            skrTextBridge.unsubscribeTextDocument(
                        uniqueDocumentReference,
                        writingZone.textArea.objectName,
                        writingZone.textArea.textDocument)

//            root.treeItemId = -2
//            root.projectId = -2
//            writingZone.treeItemId = -2
//            writingZone.projectId = -2
        }

        writingZone.setCursorPosition(0)
        writingZone.clear()
    }

    //---------------------------------------------------------
    function runActionsBeforeDestruction() {
        clearWritingZone()
    }

    Component.onDestruction: {
        runActionsBeforeDestruction()
    }

    //---------------------------------------------------------
    // modifiable :
    property bool isModifiable: true

    Connections {
        target: skrData.treePropertyHub()
        function onPropertyChanged(projectId, propertyId, treeItemId, name, value) {
            if (projectId === root.projectId
                    && treeItemId === root.treeItemId) {

                if (name === "modifiable") {
                    determineModifiable()
                }
            }
        }
    }

    Timer {
        id: determineModifiableTimer
        repeat: false
        interval: 200
        onTriggered: {
            determineModifiable()
        }
    }

    function determineModifiable() {

        root.isModifiable = skrData.treePropertyHub().getProperty(
                    projectId, treeItemId, "modifiable", "true") === "true"

        if (!root.isModifiable !== writingZone.textArea.readOnly) {
            saveCurrentCursorPositionAndY()
            writingZone.textArea.readOnly = !root.isModifiable
            restoreCurrentPaperCursorPositionAndY()
        }
    }

    //--------------------------------------------------------
    //--- stretching if too small-----------------------------------------
    //--------------------------------------------------------
    QtObject {
        id: priv
        property int tempWritingZoneWidth: 0
    }

    onWidthChanged: {
        if (!rootWindow.compactMode && !writingZone.stretch
                && root.width < 450) {
            writingZone.stretch = true
            priv.tempWritingZoneWidth = root.width
        }
        if (!rootWindow.compactMode && writingZone.stretch
                && root.width > priv.tempWritingZoneWidth + 50) {
            writingZone.stretch = false
        }
    }

    //---------------------------------------------------------
    //------Actions----------------------------------------
    //---------------------------------------------------------
    Connections {
        target: italicAction
        function onTriggered() {
            closeRightDrawer()
        }
    }
    Connections {
        target: boldAction
        function onTriggered() {
            closeRightDrawer()
        }
    }
    Connections {
        target: strikeAction
        function onTriggered() {
            closeRightDrawer()
        }
    }
    Connections {
        target: underlineAction
        function onTriggered() {
            closeRightDrawer()
        }
    }

    function closeRightDrawer() {
        if (viewManager.rootWindow.compactMode) {
            viewManager.rootWindow.closeRightDrawerCalled()
        }
    }

    //---------------------------------------------------------
    QtObject {
        id: documentPrivate
        property bool contentSaveTimerAllowedToStart: true
        property bool saveCurrentCursorPositionAndYTimerAllowedToStart: true
    }

    //---------------------------------------------------------
    function openDocument(_projectId, _treeItemId, isSecondary, milestone) {
        // save current
        if (projectId !== _projectId || treeItemId !== _treeItemId) {
            //meaning it hasn't just used the constructor
            clearWritingZone()
        }

        documentPrivate.contentSaveTimerAllowedToStart = false
        documentPrivate.saveCurrentCursorPositionAndYTimerAllowedToStart = false

        treeItemId = _treeItemId
        projectId = _projectId
        writingZone.treeItemId = _treeItemId
        writingZone.projectId = _projectId

        //console.log("opening sheet :", _projectId, _paperId)
        writingZone.setCursorPosition(0)

        if (milestone === -2) {

            if (isSecondary) {
                writingZone.text = skrRootItem.cleanUpHtml(
                            skrData.treeHub().getSecondaryContent(_projectId,
                                                                  _treeItemId))
            } else {
                writingZone.text = skrRootItem.cleanUpHtml(
                            skrData.treeHub().getPrimaryContent(_projectId,
                                                                _treeItemId))
            }
        } else {

            //TODO: if milestone
        }

        title = getTitle()

        if (milestone === -2) {
            var uniqueDocumentReference = projectId + "_" + treeItemId + "_"
                    + (isSecondary ? "secondary" : "primary")
            skrTextBridge.subscribeTextDocument(
                        uniqueDocumentReference,
                        writingZone.textArea.objectName,
                        writingZone.textArea.textDocument)
        }

        writingZone.documentHandler.indentEverywhere = SkrSettings.textSettings.textIndent
        writingZone.documentHandler.topMarginEverywhere = SkrSettings.textSettings.textTopMargin

        restoreCurrentPaperCursorPositionAndY()

        forceActiveFocusTimer.start()

        // start the timer for automatic position saving
        if (milestone === -2) {
            documentPrivate.saveCurrentCursorPositionAndYTimerAllowedToStart = true
            if (!saveCurrentCursorPositionAndYTimer.running) {
                saveCurrentCursorPositionAndYTimer.start()
            }

            documentPrivate.contentSaveTimerAllowedToStart = true
        }

        //        leftDock.setCurrentPaperId(projectId, paperId)
        //        leftDock.setOpenedPaperId(projectId, paperId)
        determineModifiableTimer.start()
    }

    Timer {
        id: forceActiveFocusTimer
        repeat: false
        interval: 100
        onTriggered: writingZone.forceActiveFocus()
    }

    function restoreCurrentPaperCursorPositionAndY() {

        var positionKey
        var yKey

        if (isSecondary) {
            positionKey = "outlineTextPositionHash"
            yKey = "outlineTextYHash"
        } else {
            positionKey = "textPositionHash"
            yKey = "textYHash"
        }

        //get cursor position
        var position = skrUserSettings.getFromProjectSettingHash(projectId,
                                                                 positionKey,
                                                                 treeItemId, 0)

        if (position > writingZone.textArea.length) {
            position = writingZone.textArea.length
        }

        //get Y
        var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                    projectId, yKey, treeItemId, 0)
        //console.log("restoredCursorPosition", position)

        // set positions :
        writingZone.setCursorPosition(position)

        writingZoneFlickableContentYTimer.y = visibleAreaY
        writingZoneFlickableContentYTimer.start()
    }

    Timer {

        property int y: 0
        id: writingZoneFlickableContentYTimer
        repeat: false
        interval: 50
        onTriggered: {
            writingZone.flickable.contentY = y
        }
    }

    function saveCurrentCursorPositionAndY() {

        if (root.treeItemId !== -2 || root.projectId !== -2) {
            var positionKey
            var yKey

            if (isSecondary) {
                positionKey = "outlineTextPositionHash"
                yKey = "outlineTextYHash"
            } else {
                positionKey = "textPositionHash"
                yKey = "textYHash"
            }

            //save cursor position of current document :
            var previousCursorPosition = writingZone.textArea.cursorPosition

            if (previousCursorPosition > writingZone.textArea.length) {
                previousCursorPosition = writingZone.textArea.length
            }
            //console.log("savedCursorPosition", previousCursorPosition)
            var previousY = writingZone.flickable.contentY
            //console.log("previousContentY", previousY)
            skrUserSettings.insertInProjectSettingHash(projectId, positionKey,
                                                       treeItemId,
                                                       previousCursorPosition)
            skrUserSettings.insertInProjectSettingHash(projectId, yKey,
                                                       treeItemId, previousY)
        }
    }

    Timer {
        id: saveCurrentCursorPositionAndYTimer
        repeat: true
        interval: 10000
        onTriggered: saveCurrentCursorPositionAndY()
    }

    //needed to adapt width to a shrinking window
//    Binding on writingZone.textAreaWidth {
//        when: !Globals.compactMode
//              && middleBase.width - 40 < writingZone.maximumTextAreaWidth
//        value: middleBase.width - 40
//        restoreMode: Binding.RestoreBindingOrValue
//    }
    Binding on writingZone.textAreaWidth {
        when: !Globals.compactMode
              && middleBase.width - 40 >= writingZone.maximumTextAreaWidth
        value: writingZone.maximumTextAreaWidth
        restoreMode: Binding.RestoreBindingOrValue
    }

    // save content once after writing:
    writingZone.textArea.onTextChanged: {

        //avoid first text change, when blank HTML is inserted
        if (writingZone.textArea.length === 0 && skrData.projectHub(
                    ).isProjectNotModifiedOnce(projectId)) {
            return
        }

        if (contentSaveTimer.running) {
            contentSaveTimer.stop()
        }
        if (documentPrivate.contentSaveTimerAllowedToStart) {
            contentSaveTimer.start()
        }


    }
    Timer {
        id: contentSaveTimer
        repeat: false
        interval: 50
        onTriggered: saveContent(false)
    }

    function saveContent(sameThread) {
        //console.log("saving text")
        var result

        var text = skrRootItem.cleanUpHtml(writingZone.text)

        if (isSecondary) {
            result = skrData.treeHub().setSecondaryContent(
                        projectId, treeItemId, skrRootItem.cleanUpHtml(text))
        } else {
            result = skrData.treeHub().setPrimaryContent(
                        projectId, treeItemId, skrRootItem.cleanUpHtml(text))
            skrTreeManager.updateCharAndWordCount(projectId, treeItemId,
                                         root.pageType, sameThread)
        }

        if (!result.success) {
            console.log("saving text failed", projectId, treeItemId)
        } else {

            //console.log("saving text success", projectId, treeItemId)
        }
    }



    writingZone.highlighter.spellCheckHighlightColor: SkrTheme.spellCheckHighlight
    writingZone.highlighter.findHighlightColor: SkrTheme.findHighlight
    writingZone.highlighter.otherHighlightColor_1: SkrTheme.otherHighlight_1
    writingZone.highlighter.otherHighlightColor_2: SkrTheme.otherHighlight_2
    writingZone.highlighter.otherHighlightColor_3: SkrTheme.otherHighlight_3

    //------------------------------------------------------------------------
    //----- scrollarea------------------------------------------------------------
    //------------------------------------------------------------------------


    //leftScrollWheel.grabPermissions: PointerHandler.TakeOverForbidden


    //---------------------------


    //---------------------------
    //------------------------------------------------------------------------
    //------------------------------------------------------------------------

    Binding on writingZone.contentY {
        when: !writingZone.active
        value: wholeViewContentY
        restoreMode: Binding.RestoreNone
        //delayed: true
    }


    Binding {
        when: writingZone.active
        target: root
        property: "wholeViewContentY"
        value: writingZone.contentY
        restoreMode: Binding.RestoreNone

    }


    //------------------------------------------------------------------------
    //-----minimap------------------------------------------------------------
    //------------------------------------------------------------------------
    property bool minimapVisibility: SkrSettings.minimapSettings.visible
    minimapLoader.visible: minimapVisibility
    minimapLoader.active: minimapVisibility
    minimapLoader.sourceComponent: minimapComponent
    property int wholeViewContentY: 0

    QtObject{
    id: minimapPriv
        property bool minimapBindingDelayed: true
    }

    writingZone.onWidthChanged: {
        minimapPriv.minimapBindingDelayed = false

        if(minimapBindingDelayedTimer.running){
            minimapBindingDelayedTimer.stop()
        }

        minimapBindingDelayedTimer.start()
    }

    Timer{
        id: minimapBindingDelayedTimer
        interval: 50
        onTriggered: {
            minimapPriv.minimapBindingDelayed = true

        }
    }

    Component{
        id: minimapComponent

        Minimap{
            id: minimap
            sourceViewHeight: writingZone.flickable.height
            sourceTextPointSize: SkrSettings.textSettings.textPointSize
            sourceTextIndent: SkrSettings.textSettings.textIndent
            sourceTextTopMargin: SkrSettings.textSettings.textTopMargin
            sourceTextFontFamily: SkrSettings.textSettings.textFontFamily

            styleBackgroundColor: "transparent"
            styleForegroundColor: SkrTheme.mainTextAreaForeground
            styleAccentColor: SkrTheme.accent

            projectId: root.projectId
            treeItemId: root.treeItemId
            isSecondary: root.isSecondary
            milestone: root.milestone

            divider: SkrSettings.minimapSettings.divider

            Binding on sourceViewWidth {
                value: writingZone.textArea.width
                restoreMode: Binding.RestoreBindingOrValue
                delayed: minimapPriv.minimapBindingDelayed
            }


            //Scrolling minimap

            Binding on value {
                when: !minimap.active
                value: wholeViewContentY
                restoreMode: Binding.RestoreNone
            }

            Binding {
                when: minimap.active
                target: root
                property: "wholeViewContentY"
                value: minimap.value
                restoreMode: Binding.RestoreNone
//                when: !(leftScrollFlickable.flicking || leftScrollFlickable.dragging ||
//                        writingZone.flicking || writingZone.dragging ||
//                        rightScrollFlickable.flicking || rightScrollFlickable.dragging)
//                delayed: true
            }

            onActiveChanged: {
                writingZone.contentYBehaviorEnabled = !minimap.active
            }

        }



    }




    // ScrollBar invisible while minimap is on
    writingZone.scrollBarVerticalPolicy: minimapVisibility ? ScrollBar.AlwaysOff : ScrollBar.AsNeeded

    // focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            writingZone.forceActiveFocus()
        }
    }

    //------------------------------------------------------------------------
    //-----toolboxes------------------------------------------------------------
    //-----------------------------------------------------------------------
    toolboxes: [editViewComponent, propertyPadComponent, outlinePadComponent, tagPadComponent]

    function determineAndAddToolboxPlugins() {
        var toolboxUrlList = skrTreeManager.findToolboxUrlsForPage(
                    root.pageType)

        for (var i in toolboxUrlList) {
            var url = toolboxUrlList[i]
            var pluginComponent = Qt.createComponent(
                        url, Component.PreferSynchronous, root)
            toolboxes.push(pluginComponent)
        }
    }

    Component {
        id: editViewComponent

        SkrToolbox {

            showButtonText: qsTr("Show edit toolbox")
            iconSource: "qrc:///icons/backup/format-text-italic.svg"
            name: "edit"
            EditView {
                id: editView
                anchors.fill: parent
                skrSettingsGroup: SkrSettings.textSettings
                writingZone: root.writingZone
            }
        }
    }

    Component {
        id: propertyPadComponent

        SkrToolbox {

            showButtonText: qsTr("Show properties toolbox")
            iconSource: "qrc:///icons/backup/configure.svg"
            name: "propertyPad"
            PropertyPad {
                id: propertyPad
                anchors.fill: parent

                projectId: root.projectId
                treeItemId: root.treeItemId
            }
        }
    }

    Component {
        id: outlinePadComponent

        SkrToolbox {

            showButtonText: qsTr("Show outline toolbox")
            iconSource: "qrc:///icons/backup/story-editor.svg"
            name: "outlinePad"
            OutlinePad {
                id: outlinePad
                anchors.fill: parent

                projectId: root.projectId
                treeItemId: root.treeItemId
            }
        }
    }

    Component {
        id: tagPadComponent

        SkrToolbox {

            showButtonText: qsTr("Show tags toolbox")
            iconSource: "qrc:///icons/backup/tag.svg"
            name: "tagPad"
            TagPad {
                id: tagPad
                anchors.fill: parent

                projectId: root.projectId
                treeItemId: root.treeItemId
            }
        }
    }

    //-----------------------------------------------------------
    //-------Previous writing zone------------------------------------------
    //-----------------------------------------------------------
    Component {
        id: component_previousWritingZone
        WritingZone {
            id: previousWritingZone

            property int previousTextItemTreeItemId: -2

            textAreaStyleElevation: true
            minimalTextAreaWidth: 100
            textCenteringEnabled: false

            textAreaStyleBackgroundColor: SkrTheme.mainTextAreaBackground
            textAreaStyleForegroundColor: SkrTheme.mainTextAreaForeground
            textAreaStyleAccentColor: SkrTheme.accent
            paneStyleBackgroundColor: SkrTheme.pageBackground

            maximumTextAreaWidth: SkrSettings.textSettings.textWidth
            textPointSize: SkrSettings.textSettings.textPointSize
            textFontFamily: SkrSettings.textSettings.textFontFamily
            textIndent: SkrSettings.textSettings.textIndent
            textTopMargin: SkrSettings.textSettings.textTopMargin

            stretch: root.width < 300 ? true : viewManager.rootWindow.compactMode



            highlighter.spellCheckHighlightColor: SkrTheme.spellCheckHighlight
            highlighter.findHighlightColor: SkrTheme.findHighlight
            highlighter.otherHighlightColor_1: SkrTheme.otherHighlight_1
            highlighter.otherHighlightColor_2: SkrTheme.otherHighlight_2
            highlighter.otherHighlightColor_3: SkrTheme.otherHighlight_3


            //needed to adapt width to a shrinking window
            Binding on textAreaWidth {
                when: !Globals.compactMode
                      && middleBase.width - 40 < maximumTextAreaWidth
                value: width - 40
                restoreMode: Binding.RestoreBindingOrValue
            }
            Binding on textAreaWidth {
                when: !Globals.compactMode
                      && middleBase.width - 40 >= maximumTextAreaWidth
                value: maximumTextAreaWidth
                restoreMode: Binding.RestoreBindingOrValue
            }

            SkrToolButton {
                id: closePreviousWritingZoneButton
                parent: previousWritingZone.textArea
                anchors.right: parent.right
                anchors.bottom: parent.bottom
                focusPolicy: Qt.NoFocus

                width: 30
                height: 30
                text: qsTr("Close quick view")
                icon {
                    source: "qrc:///icons/backup/view-close.svg"
                }

                onClicked: {

                    loader_previousWritingZone.active = false
                }
            }

            Component.onCompleted: {
                var previousTextItemTreeItemId = skrData.treeHub(
                            ).getPreviousTreeItemIdOfTheSameType(
                            root.projectId, root.treeItemId)
                openPreviousDocument(root.projectId,
                                     previousTextItemTreeItemId,
                                     root.isSecondary, root.milestone)

                previousWritingZone.setCursorPosition(
                            previousWritingZone.textArea.length)
            }

            //---------------------------------------------------------
            function clearPreviousWritingZone() {
                if (previousTextItemTreeItemId !== -2 && root.projectId !== -2
                        && milestone === -2) {
                    previousContentSaveTimer.stop()
                    savePreviousContent(true)
                    var uniqueDocumentReference = projectId + "_" + previousTextItemTreeItemId
                            + "_" + (isSecondary ? "secondary" : "primary")
                    skrTextBridge.unsubscribeTextDocument(
                                uniqueDocumentReference,
                                previousWritingZone.textArea.objectName,
                                previousWritingZone.textArea.textDocument)

                    previousTextItemTreeItemId = -2
                }

                previousWritingZone.clear()
            }

            //---------------------------------------------------------
            function runActionsBeforeDestruction() {
                clearPreviousWritingZone()
            }

            Component.onDestruction: {
                runActionsBeforeDestruction()
            }

            //---------------------------------------------------------
            // modifiable :
            property bool isModifiable: true

            Connections {
                target: skrData.treePropertyHub()
                function onPropertyChanged(projectId, propertyId, treeItemId, name, value) {
                    if (projectId === root.projectId
                            && treeItemId === previousWritingZone.previousTextItemTreeItemId) {

                        if (name === "modifiable") {
                            determineModifiable()
                        }
                    }
                }
            }

            Timer {
                id: determineModifiableTimer
                repeat: false
                interval: 200
                onTriggered: {
                    determineModifiable()
                }
            }

            function determineModifiable() {

                previousWritingZone.isModifiable = skrData.treePropertyHub(
                            ).getProperty(projectId,
                                          previousTextItemTreeItemId,
                                          "modifiable", "true") === "true"

                if (!previousWritingZone.isModifiable !== previousWritingZone.textArea.readOnly) {
                    previousWritingZone.textArea.readOnly = !previousWritingZone.isModifiable
                    previousWritingZone.setCursorPosition(
                                previousWritingZone.textArea.length)
                }
            }

            //---------------------------------------------------------
            QtObject {
                id: previousDocumentPrivate
                property bool contentSaveTimerAllowedToStart: true
            }
            //---------------------------------------------------------

            // save content once after writing:
            textArea.onTextChanged: {

                //avoid first text change, when blank HTML is inserted
                if (previousWritingZone.textArea.length === 0
                        && skrData.projectHub().isProjectNotModifiedOnce(
                            projectId)) {
                    return
                }

                if (previousContentSaveTimer.running) {
                    previousContentSaveTimer.stop()
                }
                if (previousDocumentPrivate.contentSaveTimerAllowedToStart) {
                    previousContentSaveTimer.start()
                }
            }
            Timer {
                id: previousContentSaveTimer
                repeat: false
                interval: 200
                onTriggered: savePreviousContent(false)
            }

            function savePreviousContent(sameThread) {
                //console.log("saving text")
                var result

                var text = skrRootItem.cleanUpHtml(previousWritingZone.text)

                if (isSecondary) {
                    result = skrData.treeHub().setSecondaryContent(
                                projectId, previousTextItemTreeItemId, text)
                } else {
                    result = skrData.treeHub().setPrimaryContent(
                                projectId, previousTextItemTreeItemId, text)
                    skrTreeManager.updateCharAndWordCount(projectId, previousTextItemTreeItemId,
                                                          root.pageType, sameThread)

                }

                if (!result.success) {
                    console.log("saving text failed", projectId,
                                previousTextItemTreeItemId)
                } else {

                    //console.log("saving text success", projectId, treeItemId)
                }
            }


            //---------------------------------------------------------
            function openPreviousDocument(_projectId, _treeItemId, isSecondary, milestone) {
                // save current
                if (projectId !== _projectId
                        || previousTextItemTreeItemId !== _treeItemId) {
                    //meaning it hasn't just used the constructor
                    clearPreviousWritingZone()
                }

                previousDocumentPrivate.contentSaveTimerAllowedToStart = false

                previousTextItemTreeItemId = _treeItemId
                previousWritingZone.projectId = _projectId

                previousWritingZone.setCursorPosition(0)

                if (milestone === -2) {

                    if (isSecondary) {
                        previousWritingZone.text = skrRootItem.cleanUpHtml(
                                    skrData.treeHub().getSecondaryContent(
                                        _projectId, _treeItemId))
                    } else {
                        previousWritingZone.text = skrRootItem.cleanUpHtml(
                                    skrData.treeHub().getPrimaryContent(
                                        _projectId, _treeItemId))
                    }
                } else {

                    //TODO: if milestone
                }

                previousWritingZone.setCursorPosition(
                            previousWritingZone.textArea.length)

                //title = getTitle()
                if (milestone === -2) {
                    var uniqueDocumentReference = projectId + "_" + previousTextItemTreeItemId
                            + "_" + (isSecondary ? "secondary" : "primary")
                    skrTextBridge.subscribeTextDocument(
                                uniqueDocumentReference,
                                previousWritingZone.textArea.objectName,
                                previousWritingZone.textArea.textDocument)
                }

                previousWritingZone.documentHandler.indentEverywhere
                        = SkrSettings.textSettings.textIndent
                previousWritingZone.documentHandler.topMarginEverywhere
                        = SkrSettings.textSettings.textTopMargin

                if (milestone === -2) {
                    documentPrivate.contentSaveTimerAllowedToStart = true
                }

                determineModifiableTimer.start()
            }
        }
    }

    SkrToolButton {
        id: openPreviousWritingZoneButton
        parent: writingZone.textArea
        anchors.right: parent.right
        anchors.top: parent.top
        focusPolicy: Qt.NoFocus

        width: 30
        height: 30
        text: qsTr("Open quick view")
        icon {
            source: "qrc:///icons/backup/arrow-down.svg"
        }

        onClicked: {
            if (skrData.treeHub().getPreviousTreeItemIdOfTheSameType(
                        root.projectId, root.treeItemId) > 0) {
                loader_previousWritingZone.active = true
            }
        }
    }

    Connections {
        target: skrData.treeHub()
        function onTrashedChanged(projectId, treeItemId, newTrashedState) {
            determineIfPreviousWritingTreeItemIdChanged()
        }
    }
    Connections {
        target: skrData.treeHub()
        function onTreeItemAdded(projectId, treeItemId) {
            determineIfPreviousWritingTreeItemIdChanged()
        }
    }
    Connections {
        target: skrData.treeHub()
        function onTreeItemRemoved(projectId, treeItemId) {
            determineIfPreviousWritingTreeItemIdChanged()
        }
    }
    Connections {
        target: skrData.treeHub()
        function onTreeItemMoved(sourceProjectId, sourceTreeItemIds, targetProjectId, targetTreeItemId) {
            determineIfPreviousWritingTreeItemIdChanged()
        }
    }

    function determineIfPreviousWritingTreeItemIdChanged() {
        if (loader_previousWritingZone.active) {
            var newTreeItemId = skrData.treeHub(
                        ).getPreviousTreeItemIdOfTheSameType(root.projectId,
                                                             root.treeItemId)
            if (loader_previousWritingZone.item.previousTextItemTreeItemId !== newTreeItemId) {

                if (newTreeItemId === -1) {
                    determineIfPreviousWritingMustBeVisible()
                } else {
                    loader_previousWritingZone.active = false
                    loader_previousWritingZone.active = true
                }
            }
        }
    }
    function determineIfPreviousWritingMustBeVisible() {
        var newTreeItemId = skrData.treeHub(
                    ).getPreviousTreeItemIdOfTheSameType(root.projectId,
                                                         root.treeItemId)
        if (newTreeItemId === -1) {
            loader_previousWritingZone.active = false
            openPreviousWritingZoneButton.visible = false
        } else {
            openPreviousWritingZoneButton.visible = true
        }
    }




    //-----------------------------------------------------------------
    //----- Related Panel------------------------------------------------
    //-----------------------------------------------------------------
    relationshipPanel.projectId: root.projectId
    relationshipPanel.treeItemId: root.treeItemId

    relationshipPanel.viewManager: viewManager

    relationshipPanel.onExtendedChanged: {
        if (relationshipPanel.extended) {
            relationshipPanelPreferredHeight = root.height / 2
        } else {
            relationshipPanelPreferredHeight = 200
        }
    }

    Behavior on relationshipPanelPreferredHeight {
        enabled: SkrSettings.ePaperSettings.animationEnabled
        SpringAnimation {
            spring: 5
            mass: 0.2
            damping: 0.2
        }
    }



    //----------------------------------------------------------------------------
    //-----Zoom------------------------------------------------------------
    //----------------------------------------------------------------------------

    Shortcut {
        sequences: skrShortcutManager.shortcuts("plugin-text-page-zoom-in")
        context: Qt.WindowShortcut
        enabled: writingZone.activeFocus
        onActivated: {SkrSettings.textSettings.textPointSize += 1}
    }

    Shortcut {
        sequences: skrShortcutManager.shortcuts("plugin-text-page-zoom-out")
        context: Qt.WindowShortcut
        enabled: writingZone.activeFocus
        onActivated: {SkrSettings.textSettings.textPointSize -= 1}
    }




}
