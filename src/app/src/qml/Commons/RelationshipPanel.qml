import QtQuick 2.15
import eu.skribisto.searchtreelistproxymodel 1.0
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml 2.15
import "../Commons"
import "../Items"
import ".."




RelationshipPanelForm {
    id: root

    property int treeItemId: -2
    property int projectId: -2


    property bool extended: false
    extendPanelButton.onClicked: {
        extended= !extended
    }

    closeSubPanelButton.onClicked: {

        subPanel.visible = false
        subPanelLoader.active = false

    }
    closeSubPanelButton.visible: subPanel.visible


    closePanelButton.onClicked: {
        subPanel.visible = false
        subPanelLoader.active = false
        root.visible = false
    }


    openTreeItemButton.onClicked: {
        viewManager.loadTreeItem(subPanelLoader.item.projectId, subPanelLoader.item.treeItemId)

    }
    Component.onCompleted: {
        populateTreeItemIdListFilter()
    }

    addQuickNoteItemButton.text: skrShortcutManager.description("add-quick-note")
    addQuickNoteItemButton.onClicked: {
        var result = skrData.treeHub().addQuickNote(projectId, treeItemId, "TEXT", qsTr("Note"))
        if(result.success){
            var newId = result.getData("treeItemId", -2)

            openTreeItemInPanel(projectId, newId)
        }
    }

    function openTreeItemInPanel(projectId, treeItemId){

        priv.currentTreeItemId = treeItemId

        if(skrData.treeHub().getType(projectId, treeItemId) === "TEXT"){

            subPanel.visible = true
            subPanelLoader.active = true
            subPanelLoader.item.clearWritingZone()
            subPanelLoader.item.openDocument(projectId, treeItemId, false, -2)


        }
    }


    //---------------------------------------------------------
    // ------------------grid view-----------------------------
    //---------------------------------------------------------

    property string viewMode: SkrSettings.relationshipPanelSettings.viewMode
    required property var viewManager

    onViewModeChanged: {
        populateTreeItemIdListFilter()
        if(viewMode === "notes"){
            notesTabButton.checked = true
        }
        if(viewMode === "all"){
            allTabButton.checked = true
        }
        if(viewMode === "proposed"){
            proposedTabButton.checked = true
        }
    }

    SKRSearchTreeListProxyModel {
        id: proxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
        navigateByBranchesEnabled: false
        projectIdFilter: root.projectId
    }

    Binding {
        target: SkrSettings.relationshipPanelSettings
        property: "viewMode"
        value: viewMode
        restoreMode: Binding.RestoreBindingOrValue
    }


    function populateTreeItemIdListFilter() {
        var list = skrData.treeHub().getTreeRelationshipSourcesFromReceiverId(root.projectId, root.treeItemId)

        if(list.length === 0){
            proxyModel.treeItemIdListFilter = [-2]
            return
        }

        if(viewMode === "notes"){

            var note_folder_id = -2

            var foldersList = skrData.treeHub().getIdsWithInternalTitle(projectId, "note_folder")
            for(var f in foldersList){
                if(!skrData.treeHub().getTrashed(projectId, foldersList[f])){
                    note_folder_id = foldersList[f]
                    break
                }
            }

            var notesList = []

            for(var i in list){
                var ancestors = skrData.treeHub().getAllAncestors(projectId, list[i])
                for(var j in ancestors){
                    if(ancestors[j] === note_folder_id){
                        if(!skrData.treeHub().getTrashed(projectId, list[i])){
                            notesList.push(list[i])
                            break
                        }
                    }
                }

            }


            if(notesList.length === 0){
                proxyModel.treeItemIdListFilter = [-2]
                return
            }


            proxyModel.treeItemIdListFilter = notesList
        }
        if(viewMode === "all"){

            var allNotesList = []
            for(var m in list){
                if(!skrData.treeHub().getTrashed(projectId, list[m])){
                    allNotesList.push(list[m])
                }

            }

            if(allNotesList.length === 0){
                proxyModel.treeItemIdListFilter = [-2]
                return
            }

            proxyModel.treeItemIdListFilter = allNotesList
        }
    }

    Connections{
        target: skrData.treeHub()

        function onTreeRelationshipAdded(projectId, sourceTreeItemId, receiverTreeItemId){

            if(projectId === root.projectId && receiverTreeItemId === root.treeItemId){
                populateTreeItemIdListFilter()
            }
        }
    }

    Connections{
        target: skrData.treeHub()

        function onTreeRelationshipRemoved(projectId, sourceTreeItemId, receiverTreeItemId){

            if(projectId === root.projectId && receiverTreeItemId === root.treeItemId){
                populateTreeItemIdListFilter()
            }
        }
    }

    Connections{
        target: skrData.treeHub()

        function onTrashedChanged(projectId, treeItemId, newTrashedState){

            if(projectId === root.projectId){
                populateTreeItemIdListFilter()
            }
        }
    }

    function getIconUrlFromPageType(type) {
        return skrTreeManager.getIconUrlFromPageType(type)
    }



    proposedTabButton.onClicked: {
        viewMode = "proposed"
    }

    allTabButton.onClicked: {
        viewMode = "all"
    }

    notesTabButton.onClicked: {
        viewMode = "notes"
    }


    gridView.model: proxyModel

    QtObject {
        id: priv
        property int currentTreeItemId: -2
    }

    //-----------------------------------------------------------------
    //----------WritingZone --------------------------------------------
    //-----------------------------------------------------------------

    Component  {
        id: writingZoneComponent

        Item {
            id: componentRoot

            property alias writingZone: writingZone
            property alias projectId: writingZone.projectId
            property alias treeItemId: writingZone.treeItemId

            WritingZone {
                id: writingZone
                placeholderText: qsTr("Type your text here …")
                stretch: true
                leftScrollItemVisible: false
                anchors.fill: parent


                textAreaStyleBackgroundColor: SkrTheme.secondaryTextAreaBackground
                textAreaStyleForegroundColor: SkrTheme.secondaryTextAreaForeground
                paneStyleBackgroundColor: SkrTheme.pageBackground
                textAreaStyleAccentColor: SkrTheme.accent

            }

            writingZone.maximumTextAreaWidth: SkrSettings.relatedTextSettings.textWidth
            writingZone.textPointSize: SkrSettings.relatedTextSettings.textPointSize
            writingZone.textFontFamily: SkrSettings.relatedTextSettings.textFontFamily
            writingZone.textIndent: SkrSettings.relatedTextSettings.textIndent
            writingZone.textTopMargin: SkrSettings.relatedTextSettings.textTopMargin

            //---------------------------------------------------------
            property bool isSecondary: false
            property int milestone: -2

            Component.onCompleted: {
            }
            //---------------------------------------------------------

            function clearWritingZone(){
                if(componentRoot.treeItemId !== -2 && componentRoot.projectId !== -2 && milestone === -2){
                    contentSaveTimer.stop()
                    saveContent()
                    saveCurrentCursorPositionAndYTimer.stop()
                    saveCurrentCursorPositionAndY()
                    var uniqueDocumentReference = projectId + "_" + treeItemId + "_" + (isSecondary ? "secondary" : "primary")
                    skrTextBridge.unsubscribeTextDocument(uniqueDocumentReference, writingZone.textArea.objectName, writingZone.textArea.textDocument)

                    componentRoot.projectId = -2
                    componentRoot.treeItemId = -2
                }

                writingZone.setCursorPosition(0)
                writingZone.clear()
            }


            //---------------------------------------------------------
            // modifiable :

            property bool isModifiable: true

            Connections{
                target: skrData.treePropertyHub()
                function onPropertyChanged(projectId, propertyId, treeItemId, name, value){
                    if(projectId === componentRoot.projectId && treeItemId === componentRoot.treeItemId){

                        if(name === "modifiable"){
                            determineModifiable()
                        }
                    }
                }
            }

            Timer{
                id: determineModifiableTimer
                repeat: false
                interval: 200
                onTriggered: {
                    determineModifiable()
                }
            }




            function determineModifiable(){

                isModifiable = skrData.treePropertyHub().getProperty(componentRoot.projectId, componentRoot.treeItemId, "modifiable", "true") === "true"

                if(!isModifiable !== writingZone.textArea.readOnly){
                    saveCurrentCursorPositionAndY()
                    writingZone.textArea.readOnly = !isModifiable
                    restoreCurrentPaperCursorPositionAndY()
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
                if(projectId !== _projectId || treeItemId !== _treeItemId ){ //meaning it hasn't just used the constructor
                    clearWritingZone()
                }

                documentPrivate.contentSaveTimerAllowedToStart = false
                documentPrivate.saveCurrentCursorPositionAndYTimerAllowedToStart = false

                componentRoot.treeItemId = _treeItemId
                componentRoot.projectId = _projectId

                //console.log("opening sheet :", _projectId, _paperId)
                writingZone.setCursorPosition(0)

                if(milestone === -2){

                    if(isSecondary){
                        writingZone.text = skrRootItem.cleanUpHtml(skrData.treeHub().getSecondaryContent(_projectId, _treeItemId))
                    }
                    else {
                        writingZone.text = skrRootItem.cleanUpHtml(skrData.treeHub().getPrimaryContent(_projectId, _treeItemId))
                    }

                }
                else {
                    //TODO: if milestone
                }


                //title = getTitle()

                if(milestone === -2){
                    var uniqueDocumentReference = projectId + "_" + treeItemId + "_" + (isSecondary ? "secondary" : "primary")
                    skrTextBridge.subscribeTextDocument(uniqueDocumentReference,
                                                        writingZone.textArea.objectName,
                                                        writingZone.textArea.textDocument)
                }

                writingZone.documentHandler.indentEverywhere = SkrSettings.relatedTextSettings.textIndent
                writingZone.documentHandler.topMarginEverywhere = SkrSettings.relatedTextSettings.textTopMargin

                restoreCurrentPaperCursorPositionAndY()

                forceActiveFocusTimer.start()

                // start the timer for automatic position saving
                if(milestone === -2){
                    documentPrivate.saveCurrentCursorPositionAndYTimerAllowedToStart = true
                    if(!saveCurrentCursorPositionAndYTimer.running){
                        saveCurrentCursorPositionAndYTimer.start()
                    }

                    documentPrivate.contentSaveTimerAllowedToStart = true
                }

                //        leftDock.setCurrentPaperId(projectId, paperId)
                //        leftDock.setOpenedPaperId(projectId, paperId)

                determineModifiableTimer.start()

            }

            Timer{
                id: forceActiveFocusTimer
                repeat: false
                interval: 100
                onTriggered:  writingZone.forceActiveFocus()
            }

            function restoreCurrentPaperCursorPositionAndY(){

                var positionKey
                var yKey

                if(isSecondary){
                    positionKey = "outlineRelatedTextPositionHash"
                    yKey = "outlineRelatedTextYHash"
                }
                else {
                    positionKey = "relatedTextPositionHash"
                    yKey = "relatedTextYHash"
                }

                //get cursor position
                var position = skrUserSettings.getFromProjectSettingHash(
                            componentRoot.projectId, positionKey, componentRoot.treeItemId, 0)

                if(position > writingZone.textArea.length){
                    position = writingZone.textArea.length
                }


                //get Y
                var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                            componentRoot.projectId, yKey, componentRoot.treeItemId, 0)
                //console.log("restoredCursorPosition", position)

                // set positions :
                writingZone.setCursorPosition(position)

                writingZoneFlickableContentYTimer.y = visibleAreaY
                writingZoneFlickableContentYTimer.start()

            }

            Timer{

                property int y: 0
                id: writingZoneFlickableContentYTimer
                repeat: false
                interval: 50
                onTriggered: {
                    writingZone.flickable.contentY = y
                }
            }


            function saveCurrentCursorPositionAndY(){

                if(componentRoot.treeItemId !== -2 || componentRoot.projectId !== -2){
                    var positionKey
                    var yKey

                    if(isSecondary){
                        positionKey = "outlineRelatedTextPositionHash"
                        yKey = "outlineRelatedTextYHash"
                    }
                    else {
                        positionKey = "relatedTextPositionHash"
                        yKey = "relatedTextYHash"
                    }

                    //save cursor position of current document :

                    var previousCursorPosition = writingZone.textArea.cursorPosition
                    if(previousCursorPosition > writingZone.textArea.length){
                        previousCursorPosition = writingZone.textArea.length
                    }

                    //console.log("savedCursorPosition", previousCursorPosition)
                    var previousY = writingZone.flickable.contentY
                    //console.log("previousContentY", previousY)
                    skrUserSettings.insertInProjectSettingHash(
                                componentRoot.projectId, positionKey, componentRoot.treeItemId,
                                previousCursorPosition)
                    skrUserSettings.insertInProjectSettingHash(componentRoot.projectId,
                                                               yKey,
                                                               componentRoot.treeItemId,
                                                               previousY)
                }
            }

            Timer{
                id: saveCurrentCursorPositionAndYTimer
                repeat: true
                interval: 10000
                onTriggered: saveCurrentCursorPositionAndY()
            }

            // save content once after writing:

            Connections{
                target:  writingZone.textArea
                function onTextChanged(){
                    //avoid first text change, when blank HTML is inserted
                    if(writingZone.textArea.length === 0
                            && skrData.projectHub().isProjectNotModifiedOnce(componentRoot.projectId)){
                        return
                    }

                    if(contentSaveTimer.running){
                        contentSaveTimer.stop()
                    }
                    if(documentPrivate.contentSaveTimerAllowedToStart){
                        contentSaveTimer.start()
                    }
                }
            }



            Timer{
                id: contentSaveTimer
                repeat: false
                interval: 200
                onTriggered: saveContent()
            }

            function saveContent(){
                //console.log("saving text")
                var result

                var text = skrRootItem.cleanUpHtml(writingZone.text)


                if(isSecondary){
                    result = skrData.treeHub().setSecondaryContent(componentRoot.projectId,  componentRoot.treeItemId, text)
                }
                else {
                    result = skrData.treeHub().setPrimaryContent(componentRoot.projectId, componentRoot.treeItemId, text)
                    if(!contentSaveTimer.running)
                        skrTreeManager.updateCharAndWordCount(componentRoot.projectId, componentRoot.treeItemId, "TEXT", true)

                }

                if (!result.success){
                    console.log("saving text failed", componentRoot.projectId, componentRoot.treeItemId)
                }
                else {
                    //console.log("saving text success", projectId, treeItemId)

                }
            }

        }

    }
    subPanelLoader.sourceComponent: writingZoneComponent


    //-----------------------------------------------------------------
    //----------Delegate --------------------------------------------
    //-----------------------------------------------------------------


    gridView.delegate: Item {
        id: control

        width: gridView.cellWidth
        height: gridView.cellHeight

        focus: true



        SkrFocusIndicator {
            parent: control
            anchors.fill: control
            visible: control.activeFocus & Globals.focusVisible
        }
        Keys.onPressed: function(event) {
            if (event.key === Qt.Key_Tab) {
                Globals.setFocusTemporarilyVisible()
            }
        }

        property string tip

        SkrToolTip {
            text: control.tip ? control.tip : text
            visible: hoverHandler.hovered && text.length !== 0
        }


        HoverHandler{
            id: hoverHandler
        }


        TapHandler{
            id: tapHandler

            onSingleTapped: function(eventPoint) {
                control.forceActiveFocus()
                gridView.currentIndex = model.index

                root.openTreeItemInPanel(projectId, model.treeItemId)

            }



            onDoubleTapped: function(eventPoint) {
                control.forceActiveFocus()
                gridView.currentIndex = model.index
                priv.currentTreeItemId = model.treeItemId
                viewManager.loadTreeItem(root.projectId, model.treeItemId)
            }

            grabPermissions: PointerHandler.TakeOverForbidden

        }
        TapHandler{
            id: middleClickTapHandler
            acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
            acceptedButtons: Qt.MiddleButton

            onSingleTapped: function(eventPoint) {
                control.forceActiveFocus()
                gridView.currentIndex = model.index
                priv.currentTreeItemId = model.treeItemId
                viewManager.loadTreeItemAtAnotherView(root.projectId, model.treeItemId)
            }

            grabPermissions: PointerHandler.TakeOverForbidden

        }

        TapHandler {
            id: rightClickTapHandler
            acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
            acceptedButtons: Qt.RightButton
            onSingleTapped: function(eventPoint) {
                control.forceActiveFocus()
                gridView.currentIndex = model.index
                priv.currentTreeItemId = model.treeItemId

                if (menu.visible) {
                    menu.close()
                    return
                }

                menu.open()
                eventPoint.accepted = true
            }

            grabPermissions: PointerHandler.TakeOverForbidden
        }

        //----------------------------------------------------------------------
        //-----------touch handler--------------------------------------------------
        //----------------------------------------------------------------------
        TapHandler {
            id: touchHandler
            acceptedDevices: PointerDevice.TouchScreen
            acceptedPointerTypes: PointerDevice.Finger

            onLongPressed: {
                control.forceActiveFocus()
                gridView.currentIndex = model.index
                priv.currentTreeItemId = model.treeItemId

                if (menu.visible) {
                    menu.close()
                    return
                }

                menu.open()
                eventPoint.accepted = true
            }
        }



        //----------------------------------------
        //----------menu--------------------------
        //----------------------------------------
        SkrMenu {
            id: menu
            //y: menuButton.height
            width: 300

            SkrMenuItem {
                height: !model.isOpenable ? 0 : undefined
                visible: model.isOpenable
                action: Action {
                    id: openTreeItemAction
                    text: qsTr("Open")
                    //shortcut: "Return"
                    icon {
                        source: "qrc:///icons/backup/document-edit.svg"
                    }

                    onTriggered: {
                        console.log("open treeItem action",
                                    model.projectId,
                                    model.treeItemId)
                        viewManager.loadTreeItem(root.projectId, model.treeItemId)
                    }
                }
            }
            SkrMenuItem {
                height: !model.isOpenable
                        || model.treeItemId === -1 ? 0 : undefined
                visible: model.isOpenable

                action: Action {
                    id: openTreeItemInAnotherViewAction
                    text: qsTr("Open in another view")
                    //shortcut: "Alt+Return"
                    icon {
                        source: "qrc:///icons/backup/tab-new.svg"
                    }
                    onTriggered: {
                        console.log("open treeItem in another view action",
                                    model.projectId,
                                    model.treeItemId)
                        viewManager.loadTreeItemAtAnotherView(root.projectId, model.treeItemId)
                    }
                }
            }

            SkrMenuItem {
                height: !model.isOpenable ? 0 : undefined
                visible: model.isOpenable

                action: Action {
                    id: openTreeItemInNewWindowAction
                    text: qsTr("Open in new window")
                    //shortcut: "Alt+Return"
                    icon {
                        source: "qrc:///icons/backup/window-new.svg"
                    }
                    onTriggered: {
                        console.log("open treeItem in new window action",
                                    model.projectId,
                                    model.treeItemId)

                        skrWindowManager.addWindowForItemId(model.projectId, model.treeItemId)

                    }
                }
            }

            MenuSeparator {
            }
            SkrMenuItem {
                action: Action {
                    text: qsTr("Dissociate")
                    icon {
                        source: "qrc:///icons/backup/remove-link.svg"
                    }

                    onTriggered: {
                        skrData.treeHub().removeTreeRelationship(model.projectId, model.treeItemId, root.treeItemId)
                    }
                }

            }


            MenuSeparator {
                height: model.isRenamable ? undefined : 0
                visible: model.isRenamable
            }

            SkrMenuItem {
                height: model.isRenamable ? undefined : 0
                visible: model.isRenamable
                action: Action {
                    id: renameAction
                    text: qsTr("Rename")
                    shortcut: "F2"
                    icon {
                        source: "qrc:///icons/backup/edit-rename.svg"
                    }

                    onTriggered: {
                        loader_renameDialog.active = true
                    }
                }
            }


            MenuSeparator {
                height: !model.isTrashable ? 0 : undefined
                visible: model.isTrashable
            }

            SkrMenuItem {
                height: !model.isTrashable ? 0 : undefined
                visible: model.isTrashable
                action: Action {
                    id: sendToTrashAction
                    text: qsTr("Send to trash")
                    //shortcut: "Del"
                    icon {
                        source: "qrc:///icons/backup/edit-delete.svg"
                    }
                    enabled: model.indent !== -1
                    onTriggered: {
                        console.log("sent to trash action",
                                    model.projectId,
                                    model.treeItemId)

                        proxyModel.trashItemWithChildren(
                                    model.projectId,
                                    model.treeItemId)
                    }
                }
            }


        }

        //----------------------------------------
        //--------Rename -------------------------
        //----------------------------------------

        Component {
            id: component_renameDialog
            SimpleDialog {
                property alias renameTextField: inner_renameTextField

                id: renameDialog
                property int projectId: model.projectId
                property int treeItemId: model.treeItemId
                property string treeItemTitle: model.title
                title: qsTr("Rename an item")
                contentItem: SkrTextField {
                    id: inner_renameTextField
                    text: renameDialog.treeItemTitle

                    onAccepted: {
                        renameDialog.accept()
                    }

                }

                Component.onCompleted: {
                    renameDialog.open()
                    inner_renameTextField.selectAll()
                    inner_renameTextField.forceActiveFocus()
                }

                onClosed: loader_renameDialog.active = false

                standardButtons: Dialog.Ok  | Dialog.Cancel

                onRejected: {
                    renameDialog.treeItemTitle = ""

                }

                onDiscarded: {


                    renameDialog.treeItemTitle = ""

                }

                onAccepted: {
                    skrData.treeHub().setTitle(renameDialog.projectId, renameDialog.treeItemId, renameTextField.text)

                    renameDialog.treeItemTitle = ""
                }

                onActiveFocusChanged: {
                    if(activeFocus){
                        contentItem.forceActiveFocus()
                    }

                }

                onOpened: {
                    contentItem.forceActiveFocus()
                    renameTextField.selectAll()
                }

            }
        }
        Loader {
            id: loader_renameDialog
            sourceComponent: component_renameDialog
            active: false
        }


        //----------------------------------------
        //----------UI--------------------------
        //----------------------------------------


        ColumnLayout {
            anchors.fill: parent
            focus: true

            Image {
                source: skrTreeManager.getIconUrlFromPageType(type)
                Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
                Layout.preferredHeight: 30
                Layout.preferredWidth: 30
            }
            SkrLabel {
                text: title
                Layout.alignment: Qt.AlignHCenter
                Layout.maximumWidth: control.width
                activeFocusOnTab: false
                maximumLineCount: 2
                wrapMode: Text.Wrap
            }


            Rectangle {
                id: currentItemIndicator
                Layout.alignment: Qt.AlignBottom
                Layout.fillWidth: true
                Layout.preferredHeight: 5
                color: priv.currentTreeItemId === model.treeItemId ? "lightsteelblue" : "transparent"


            }

        }

    }


    //----------------------------------------
    //---------- drop--------------------------
    //----------------------------------------




    Rectangle {
        id: dropIndicator
        anchors.fill: parent
        visible: false
        z: 1
        color: "transparent"
        border.color: SkrTheme.accent
        border.width: 4

    }

    DropArea {
        id: dropArea
        anchors.fill: parent



        keys: ["application/skribisto-tree-item"]
        onEntered: {
            dropIndicator.visible = true
        }
        onExited: {
            dropIndicator.visible = false

        }

        onDropped: {
            if(drop.proposedAction === Qt.MoveAction){
                skrData.treeHub().setTreeRelationship(drag.source.projectId, drag.source.treeItemId, root.treeItemId)
            }
            dropIndicator.visible = false
        }



    }



    onActiveFocusChanged: {
        if (activeFocus) {
            notesTabButton.forceActiveFocus()
        }
    }


}
