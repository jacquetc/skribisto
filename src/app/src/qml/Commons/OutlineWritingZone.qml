import QtQuick 2.15

Item {
    id: root
    property alias stretch: writingZone.stretch
    property alias textPointSize: writingZone.textPointSize
    property alias textFontFamily: writingZone.textFontFamily
    property alias textIndent: writingZone.textIndent
    property alias textTopMargin: writingZone.textTopMargin

    property alias textAreaStyleElevation: writingZone.textAreaStyleElevation
    property alias textAreaStyleBackgroundColor: writingZone.textAreaStyleBackgroundColor
    property alias textAreaStyleForegroundColor: writingZone.textAreaStyleForegroundColor
    property alias textAreaStyleAccentColor: writingZone.textAreaStyleAccentColor
    property alias paneStyleBackgroundColor: writingZone.paneStyleBackgroundColor

    property alias projectId: writingZone.projectId
    property alias treeItemId: writingZone.treeItemId
    property int milestone: -2
    property alias spellCheckerKilled: writingZone.spellCheckerKilled
    property alias textCenteringEnabled: writingZone.textCenteringEnabled

    property alias leftScrollItemVisible: writingZone.leftScrollItemVisible
    property alias rightScrollItemVisible: writingZone.rightScrollItemVisible

    property alias placeholderText: writingZone.placeholderText
    required property string name
    property alias writingZone: writingZone

    //---------------------------------------------------------

    clip: true
    WritingZone {
        id: writingZone
        anchors.fill: parent


        textArea.onTextChanged: {

                //avoid first text change, when blank HTML is inserted
                if (writingZone.textArea.length === 0
                        && skrData.projectHub(
                            ).isProjectNotModifiedOnce(
                            projectId)) {
                    return
                }

                if (contentSaveTimer.running) {
                    contentSaveTimer.stop()
                }
                if (documentPrivate.contentSaveTimerAllowedToStart) {
                    contentSaveTimer.start()
                }
            }
    }

    //---------------------------------------------------------


    Component.onCompleted: {
        openOutline(projectId, treeItemId, milestone)
    }
    //---------------------------------------------------------

    // project to be closed :
    Connections {
        target: skrData.projectHub()
        function onProjectToBeClosed(projectId) {

            if (projectId === currentProjectId) {
                // save
                writingZone.clearNoteWritingZone()
            }
        }
    }


    //---------------------------------------------------------

    function clearNoteWritingZone() {
        if (treeItemId !== -2
                && projectId !== -2) {
            contentSaveTimer.stop()
            saveContent()
            saveCurrentPaperCursorPositionAndYTimer.stop()
            saveCurrentPaperCursorPositionAndY()
            var uniqueDocumentReference = projectId + "_"
                    + treeItemId + "_secondary"
            skrTextBridge.unsubscribeTextDocument(
                        uniqueDocumentReference,
                        writingZone.textArea.objectName,
                        writingZone.textArea.textDocument)
        }

        writingZone.setCursorPosition(0)
        writingZone.clear()
    }


//---------------------------------------------------------
    QtObject {
        id: documentPrivate
        property bool contentSaveTimerAllowedToStart: true
        property bool saveCurrentPaperCursorPositionAndYTimerAllowedToStart: true
    }
    function openOutline(_projectId, _treeItemId) {
        // save current
        if (projectId !== _projectId
                && treeItemId !== _treeItemId) {
            //meaning it hasn't just used the constructor
            clearNoteWritingZone()
        }

        documentPrivate.contentSaveTimerAllowedToStart
                = false
        documentPrivate.saveCurrentPaperCursorPositionAndYTimerAllowedToStart = false

        //                                            treeItemId = _treeItemId
        //                                            projectId = _projectId

        //console.log("opening note :", _projectId, _treeItemId)
        writingZone.setCursorPosition(0)
        if (milestone === -2) {
        writingZone.text = skrRootItem.cleanUpHtml(
                    skrData.treeHub(
                        ).getSecondaryContent(
                        _projectId,
                        _treeItemId))
        } else {

            //TODO: if milestone
        }

        if (milestone === -2) {
        var uniqueDocumentReference = _projectId + "_"
                + _treeItemId + "_secondary"
        skrTextBridge.subscribeTextDocument(
                    uniqueDocumentReference,
                    writingZone.textArea.objectName,
                    writingZone.textArea.textDocument)
}
        // apply format
//        writingZone.documentHandler.indentEverywhere = root.textIndent
//        writingZone.documentHandler.topMarginEverywhere = root.textTopMargin

        //restoreCurrentPaperCursorPositionAndY()

        // start the timer for automatic position saving
        if (milestone === -2) {
        documentPrivate.saveCurrentPaperCursorPositionAndYTimerAllowedToStart = true
        if (!saveCurrentPaperCursorPositionAndYTimer.running) {
            saveCurrentPaperCursorPositionAndYTimer.start()
        }
        documentPrivate.contentSaveTimerAllowedToStart
                = true
        }

        determineModifiableTimer.start()
    }

    //---------------------------------------------------------
    // modifiable :
    property bool isModifiable: true

    Connections {
        target: skrData.treePropertyHub(
                    )
        function onPropertyChanged(_projectId, propertyId, _treeItemId, name, value) {
            if (_projectId === writingZone.projectId
                    && _treeItemId
                    === writingZone.treeItemId) {

                if (name === "modifiable") {
                    determineModifiable(
                                )
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

        isModifiable = skrData.treePropertyHub(
                    ).getProperty(
                    writingZone.projectId,
                    writingZone.treeItemId,
                    "modifiable",
                    "true") === "true"

        if (!isModifiable
                !== writingZone.textArea.readOnly) {
            saveCurrentPaperCursorPositionAndY()
            writingZone.textArea.readOnly = !isModifiable
            restoreCurrentPaperCursorPositionAndY()
        }
    }

    //--------------------------------------------------------
    function restoreCurrentPaperCursorPositionAndY() {

        //get cursor position
        var position = skrUserSettings.getFromProjectSettingHash(
                    projectId,
                    name + "PositionHash",
                    treeItemId, 0)
        //get Y
        var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                    projectId,
                    name + "YHash",
                    treeItemId, 0)

        // set positions :
        if (position > writingZone.textArea.length) {
            position = writingZone.textArea.length
        }

        //                                            writingZone.setCursorPosition(position)
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

    function saveCurrentPaperCursorPositionAndY() {

        if (writingZone.treeItemId !== -2
                || writingZone.projectId !== -2) {

            //save cursor position of current document :
            var previousCursorPosition = writingZone.textArea.cursorPosition
            //console.log("previousCursorPosition", previousCursorPosition)
            var previousY = writingZone.flickable.contentY
            //console.log("previousContentY", previousY)
            skrUserSettings.insertInProjectSettingHash(
                        projectId,
                        name + "OulinePositionHash",
                        treeItemId,
                        previousCursorPosition)
            skrUserSettings.insertInProjectSettingHash(
                        projectId,
                        name + "OutlineYHash",
                        treeItemId,
                        previousY)
        }
    }

    Timer {
        id: saveCurrentPaperCursorPositionAndYTimer
        repeat: true
        interval: 10000
        onTriggered: saveCurrentPaperCursorPositionAndY()
    }

    //------------------------------------------------------------
    //------------------------------------------------------------
    //------------------------------------------------------------

    // save content once after writing:

    Timer {
        id: contentSaveTimer
        repeat: false
        interval: 200
        onTriggered: saveContent()
    }

    function saveContent() {
        if (treeItemId === -2
                || projectId === -2) {
            return
        }

        //console.log("saving note")
        var result = skrData.treeHub(
                    ).setSecondaryContent(
                    projectId,
                    treeItemId,
                    skrRootItem.cleanUpHtml(
                        writingZone.text))
        if (!result.success) {
            console.log("saving note failed",
                        projectId,
                        treeItemId)
        } else {

            //console.log("saving note success", projectId, treeItemId)
        }
    }
}

