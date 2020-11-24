import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import QtQml 2.15
import QtQuick.Controls.Material 2.15
import eu.skribisto.projecthub 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import eu.skribisto.taghub 1.0
import eu.skribisto.skr 1.0
import "../Commons"
import "../Items"
import ".."

SheetOverviewTreeForm {
    id: root

    signal openDocument(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openDocumentInNewTab(int projectId, int paperId)
    signal openDocumentInNewWindow(int projectId, int paperId)

    property int currentPaperId: -2
    property int currentProjectId: -2
    property int openedProjectId: -2
    property int openedPaperId: -2
    property int currentIndex: listView.currentIndex

    property alias visualModel: visualModel
    property var proxyModel
    property var model
    onModelChanged: {
        visualModel.model = model
    }
    listView.model: visualModel


    DelegateModel {
        id: visualModel

        delegate: dragDelegate
    }



    property int contextMenuItemIndex: -2
    onCurrentIndexChanged: {
        contextMenuItemIndex = listView.currentIndex

    }

    Binding {
        target: listView
        property: "currentIndex"
        value: proxyModel.forcedCurrentIndex
    }


    //-----------------------------------------------------------------------------
    // options :
    property bool treelikeIndentsVisible: true
    property bool dragDropEnabled: false // not yet complemently implemented
    property int displayMode : 0

    //tree-like onTreelikeIndents
    property int treeIndentOffset: 0
    property int treeIndentMultiplier: 10


    // focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            listView.currentItem.forceActiveFocus()
        }
    }
    //----------------------------------------------------------------------------


    Binding {
        target: root
        property: "currentProjectId"
        value: proxyModel.projectIdFilter
    }
    //-----------------------------------------------------------------------------

    // used to remember the source when moving an item
    property int moveSourceInt: -2


    // TreeView item :
    listView.delegate:
        Component {
        id: dragDelegate

        DropArea {
            id: delegateRoot

            Accessible.name: {

                var levelText
                if(treelikeIndentsVisible){
                    levelText = qsTr("Level %1").arg(model.indent)
                }

                var titleText = titleLabel.text


                var labelText = ""
                if(labelLabel.text.length > 0){
                    labelText = qsTr("label: %1").arg(labelLabel.text)
                }

                var hasChildrenText = ""
                if(model.hasChildren){
                    hasChildrenText = qsTr("has children")
                }

                return levelText + " " + titleText + " " + labelText + " " + hasChildrenText

            }
            Accessible.role: Accessible.ListItem
            Accessible.description: qsTr("navigation item")


            onEntered: {

                draggableContent.sourceIndex = drag.source.visualIndex
                visualModel.items.move(drag.source.visualIndex,
                                       draggableContent.visualIndex)
            }

            onDropped: {
                console.log("dropped : ", moveSourceInt, draggableContent.visualIndex)
                proxyModel.moveItem(moveSourceInt, draggableContent.visualIndex)
            }


            property int visualIndex: {
                return DelegateModel.itemsIndex
            }

            Binding {
                target: draggableContent
                property: "visualIndex"
                value: visualIndex
            }

            anchors {
                left: Qt.isQtObject(parent) ? parent.left : undefined
                right: Qt.isQtObject(parent) ? parent.right : undefined
                rightMargin: 5
                leftMargin: treelikeIndentsVisible ? model.indent * root.treeIndentMultiplier - root.treeIndentOffset * root.treeIndentMultiplier : undefined
            }



            height: draggableContent.height


            onActiveFocusChanged: {
                if(listView.currentIndex === model.index){
                    root.currentPaperId = model.paperId
                }
            }


            function editName() {
                titleBox.state = "edit_name"
                titleTextFieldForceActiveFocusTimer.start()
                titleTextField.selectAll()
            }

            Timer{
                id: titleTextFieldForceActiveFocusTimer
                repeat: false
                interval: 100
                onTriggered: {
                    titleTextField.forceActiveFocus()
                }
            }

            function editLabel() {
                titleBox.state = "edit_label"
                labelTextFieldForceActiveFocusTimer.start()
                labelTextField.selectAll()
            }

            Timer{
                id: labelTextFieldForceActiveFocusTimer
                repeat: false
                interval: 100
                onTriggered: {
                    labelTextField.forceActiveFocus()
                }
            }



            Keys.priority: Keys.AfterItem

            Keys.onShortcutOverride: {

                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_C){
                    event.accepted = true
                }
                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_X){
                    event.accepted = true
                }
                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_V){
                    event.accepted = true
                }
                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_N){
                    event.accepted = true
                }
                if(event.key === Qt.Key_Escape && delegateRoot.state == "edit_name"){
                    event.accepted = true
                }
                if( event.key === Qt.Key_Escape){
                    event.accepted = true
                }
            }

            Keys.onPressed: {
                if (event.key === Qt.Key_Return){
                    console.log("Return key pressed")
                    openDocumentAction.trigger()
                    event.accepted = true
                }
                if ((event.modifiers & Qt.AltModifier) && event.key === Qt.Key_Return){
                    console.log("Alt Return key pressed")
                    openDocumentInNewTabAction.trigger()
                    event.accepted = true
                }


                // rename

                if (event.key === Qt.Key_F2 && delegateRoot.state !== "edit_name"){
                    renameAction.trigger()
                    event.accepted = true
                }

                // cut
                if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_X && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    cutAction.trigger()
                    event.accepted = true
                }

                // copy
                if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_C && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    copyAction.trigger()
                    event.accepted = true
                }


                // copy
                if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_V && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    pasteAction.trigger()
                    event.accepted = true
                }

                // add before
                if ((event.modifiers & Qt.ControlModifier) && (event.modifiers & Qt.ShiftModifier) && event.key === Qt.Key_N && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    addBeforeAction.trigger()
                    event.accepted = true
                }

                // add after
                if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_N && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    addAfterAction.trigger()
                    event.accepted = true
                }

                // move up
                if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Up && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    moveUpAction.trigger()
                    event.accepted = true
                }

                // move down
                if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Down && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    moveDownAction.trigger()
                    event.accepted = true
                }

                // send to trash
                if (event.key === Qt.Key_Delete && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    sendToTrashAction.trigger()
                    event.accepted = true
                }

                if (event.key === Qt.Key_Escape){
                    console.log("Escape key pressed")
                    event.accepted = true
                }

            }

            property bool focusOnBranchChecked: false

            Rectangle {
                id: draggableContent
                property int visualIndex: 0
                property int sourceIndex: -2

                property bool isCurrent: model.index === listView.currentIndex ? true : false

                anchors {
                    horizontalCenter: parent.horizontalCenter
                    verticalCenter: parent.verticalCenter
                }
                width: delegateRoot.width  - 2

                height: content.height + 2

                Drag.active: dragHandler.active
                Drag.source: draggableContent
                Drag.hotSpot.x: width / 2
                Drag.hotSpot.y: height / 2

                color: dragHandler.active | !tapHandler.enabled ? SkrTheme.accent : "transparent"

                Behavior on color {
                    ColorAnimation {
                        duration: 200
                    }
                }

                DragHandler {
                    id: dragHandler
                    //acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                    //xAxis.enabled: false
                    //grabPermissions: PointerHandler.TakeOverForbidden
                    onActiveChanged: {
                        if (active) {
                            moveSourceInt = draggableContent.visualIndex
                        } else {
                            draggableContent.Drag.drop()
                            tapHandler.enabled = true
                        }
                    }
                    enabled: !tapHandler.enabled && root.dragDropEnabled
                }






                /// without MouseArea, it breaks while dragging and scrolling:
                MouseArea {
                    anchors.fill: parent
                    onWheel: {
                        listView.flick(0, wheel.angleDelta.y * 50)
                        wheel.accepted = true
                    }

                    enabled: dragHandler.enabled && root.dragDropEnabled
                }

                Action {
                    id: openDocumentAction
                    //shortcut: "Return"
                    enabled: {
                        if (listView.enabled && titleTextField.visible === false
                                && listView.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: qsTr("Open document")
                    onTriggered: {
                        //console.log("model.openedProjectId", openedProjectId)
                        //console.log("model.projectId", model.projectId)
                        root.openDocument(openedProjectId, openedPaperId, model.projectId,
                                          model.paperId)
                    }
                }

                Action {
                    id: openDocumentInNewTabAction
                    //shortcut: "Alt+Return"
                    enabled: {
                        if (listView.enabled && titleTextField.visible === false
                                && listView.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: qsTr("Open document in a new tab")
                    onTriggered: {
                        root.openDocumentInNewTab(model.projectId,
                                                  model.paperId)

                    }
                }


                Action {
                    id: openDocumentInNewWindowAction
                    //shortcut: "Alt+Return"
                    enabled: {
                        if (listView.enabled && titleTextField.visible === false
                                && listView.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: qsTr("Open document in a window")
                    onTriggered: {
                        root.openDocumentInNewWindow(model.projectId,
                                                     model.paperId)

                    }
                }




                SkrPane{
                    id: content

                    property alias tapHandler: tapHandler


                    anchors {
                        horizontalCenter: parent.horizontalCenter
                        verticalCenter: parent.verticalCenter
                    }
                    height: 60
                    width: draggableContent.width

                    padding: 1

                    elevation: 4

                    //Material.backgroundColor: Material.

                    //                    background: Rectangle {
                    //                        color: Material.backgroundColor
                    //                        radius: 5
                    //                    }

                    HoverHandler {
                        id: hoverHandler

                    }


                    TapHandler {
                        id: tapHandler

                        onSingleTapped: {
                            listView.currentIndex = model.index
                            delegateRoot.forceActiveFocus()
                            eventPoint.accepted = true

                        }

                        onDoubleTapped: {
                            //console.log("double tapped")
                            listView.currentIndex = model.index
                            openDocumentAction.trigger()
                            eventPoint.accepted = true
                        }


                        onLongPressed: { // needed to activate the grab handler
                            if(root.dragDropEnabled){
                                enabled = false
                            }
                        }
                    }

                    TapHandler {
                        id: rightClickHandler
                        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                        acceptedButtons: Qt.RightButton
                        onTapped: {
                            //console.log("right clicked")
                            if(menu.visible){
                                menu.close()
                                return
                            }


                            // necessary to differenciate between all items
                            contextMenuItemIndex = model.index
                            listView.currentIndex = model.index

                            menu.popup(content, eventPoint.position.x, eventPoint.position.y)
                            eventPoint.accepted = true
                        }
                    }
                    TapHandler {
                        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                        acceptedButtons: Qt.MiddleButton
                        onTapped: {
                            listView.currentIndex = model.index
                            delegateRoot.forceActiveFocus()
                            openDocumentInNewTabAction.trigger()
                            eventPoint.accepted = true

                        }
                    }

                    RowLayout{
                        id: rowLayout3
                        anchors.fill: parent

                        //---------------------------------------------------------
                        //--------Title----------------------------------------
                        //---------------------------------------------------------

                        Item {
                            id: titleBox
                            clip: true
                            //Layout.minimumWidth: 50
                            Layout.preferredWidth: 200
                            //Layout.maximumWidth: 150
                            Layout.fillHeight: true
                            //Layout.fillWidth: true



                            RowLayout {
                                id: rowLayout
                                anchors.fill: parent
                                spacing: 2

                                Rectangle {
                                    id: currentItemIndicator
                                    color: listView.currentIndex === model.index ? "lightsteelblue" : "transparent"
                                    Layout.fillHeight: true
                                    Layout.preferredWidth: 5
                                    //visible: listView.currentIndex === model.index
                                }



                                ColumnLayout {
                                    id: columnLayout2
                                    spacing: 1
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true

                                    SkrLabel {
                                        id: titleLabel

                                        Layout.topMargin: 2
                                        Layout.leftMargin: 4
                                        Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                        font.bold: model.projectIsActive && model.indent === -1 ? true : false
                                        text: model.indent === -1 ? model.projectName : model.name
                                        elide: Text.ElideRight

                                        Layout.fillWidth: true
                                    }

                                    SkrTextField {
                                        id: labelTextField
                                        visible: false

                                        Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                                        Layout.fillWidth: true
                                        text: labelLabel.text
                                        maximumLength: 50

                                        placeholderText: qsTr("Enter label")

                                        onEditingFinished: {
                                            console.log("editing label finished")
                                            model.label = text
                                            titleBox.state = ""


                                        }

                                        //Keys.priority: Keys.AfterItem
                                        Keys.onShortcutOverride: event.accepted = (event.key === Qt.Key_Escape)
                                        Keys.onPressed: {
                                            if (event.key === Qt.Key_Return){
                                                console.log("Return key pressed title")
                                                editingFinished()
                                                event.accepted = true
                                            }
                                            if ((event.modifiers & Qt.CtrlModifier) && event.key === Qt.Key_Return){
                                                console.log("Ctrl Return key pressed title")
                                                editingFinished()
                                                event.accepted = true
                                            }
                                            if (event.key === Qt.Key_Escape){
                                                console.log("Escape key pressed title")
                                                delegateRoot.state = ""
                                                event.accepted = true
                                            }
                                        }

                                    }

                                    SkrTextField {
                                        id: titleTextField
                                        visible: false

                                        Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                        Layout.fillWidth: true
                                        text: titleLabel.text
                                        maximumLength: 50

                                        placeholderText: qsTr("Enter name")

                                        onEditingFinished: {

                                            console.log("editing finished")
                                            if(model.indent === -1){ //project item
                                                model.projectName = text
                                            }
                                            else {
                                                model.name = text
                                            }

                                            titleBox.state = ""
                                        }

                                        //Keys.priority: Keys.AfterItem
                                        Keys.onShortcutOverride: event.accepted = (event.key === Qt.Key_Escape)
                                        Keys.onPressed: {
                                            if (event.key === Qt.Key_Return){
                                                console.log("Return key pressed title")
                                                editingFinished()
                                                event.accepted = true
                                            }
                                            if ((event.modifiers & Qt.CtrlModifier) && event.key === Qt.Key_Return){
                                                console.log("Ctrl Return key pressed title")
                                                editingFinished()
                                                event.accepted = true
                                            }
                                            if (event.key === Qt.Key_Escape){
                                                console.log("Escape key pressed title")
                                                titleBox.state = ""
                                                event.accepted = true
                                            }
                                        }

                                    }

                                    SkrLabel {
                                        id: labelLabel
                                        text:  model.label === undefined ? "" : model.label
                                        Layout.bottomMargin: 2
                                        Layout.rightMargin: 4
                                        Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                                        elide: Text.ElideRight
                                        visible: text.length === 0 ? false : true
                                        font.italic: true
                                        Layout.fillWidth: true
                                    }
                                }

                                Rectangle {
                                    Layout.fillHeight: true
                                    Layout.preferredWidth: 2
                                    Layout.alignment: Qt.AlignRight | Qt.AlignVCenter

                                    color: model.indent === 0 ? Material.color(Material.Indigo) :
                                                                (model.indent === 1 ? Material.color(Material.LightBlue) :
                                                                                      (model.indent === 2 ? Material.color(Material.LightGreen) :
                                                                                                            (model.indent === 3 ? Material.color(Material.Amber) :
                                                                                                                                  (model.indent === 4 ? Material.color(Material.DeepOrange) :
                                                                                                                                                        Material.color(Material.Teal)
                                                                                                                                   ))))
                                }
                            }

                            states: [
                                State {
                                    name: "edit_name"
                                    PropertyChanges {
                                        target: menuButton
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: titleLabel
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: labelLabel
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: titleTextField
                                        visible: true
                                    }
                                    PropertyChanges {
                                        target: labelTextField
                                        visible: false
                                    }
                                },
                                State {
                                    name: "edit_label"
                                    PropertyChanges {
                                        target: menuButton
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: titleLabel
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: labelLabel
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: titleTextField
                                        visible: false
                                    }
                                    PropertyChanges {
                                        target: labelTextField
                                        visible: true
                                    }
                                }

                            ]
                        }

                        //------------------------------------------------------
                        //--------Outline--------------------------------------
                        //------------------------------------------------------

                        Item {
                            id: synopsisBox
                            Layout.fillHeight: true
                            Layout.fillWidth: true
                            Layout.minimumWidth: 100
                            //Layout.maximumWidth: 600

                            property var writingZone: noteWritingZoneLoader.item

                            onWidthChanged: {
                                if(width === 50 && Component.status === Component.Ready){
                                    SkrSettings.overviewTreeSettings.synopsisBoxVisible = false
                                }
                            }
                            visible: SkrSettings.overviewTreeSettings.synopsisBoxVisible

                            RowLayout {
                                anchors.fill: parent

                                Rectangle {
                                    Layout.preferredWidth: 1
                                    Layout.preferredHeight: content.height / 2
                                    Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                                    gradient: Gradient {
                                        orientation: Qt.Vertical
                                        GradientStop {
                                            position: 0.00;
                                            color: "#ffffff";
                                        }
                                        GradientStop {
                                            position: 0.30;
                                            color: "#9e9e9e";
                                        }
                                        GradientStop {
                                            position: 0.70;
                                            color: "#9e9e9e";
                                        }
                                        GradientStop {
                                            position: 1.00;
                                            color: "#ffffff";
                                        }
                                    }

                                }



                                Component {
                                    id: noteWritingZoneComponent


                                    WritingZone {
                                        id: writingZone

                                        property string pageType: "note"

                                        clip: true
                                        projectId: model.projectId
                                        spellCheckerKilled: true
                                        leftScrollItemVisible: false
                                        textArea.placeholderText: qsTr("Outline")

                                        textPointSize: SkrSettings.overviewTreeNoteSettings.textPointSize
                                        textFontFamily: SkrSettings.overviewTreeNoteSettings.textFontFamily
                                        textIndent: SkrSettings.overviewTreeNoteSettings.textIndent
                                        textTopMargin: SkrSettings.overviewTreeNoteSettings.textTopMargin

                                        stretch: true


                                        textAreaStyleBackgroundColor: SkrTheme.secondaryTextAreaBackground
                                        textAreaStyleForegroundColor: SkrTheme.secondaryTextAreaForeground
                                        paneStyleBackgroundColor: SkrTheme.pageBackground
                                        textAreaStyleAccentColor: SkrTheme.accent

                                        Component.onCompleted: {
                                            openSynopsisFromSheetId(model.projectId, model.paperId)
                                        }


                                        // project to be closed :
                                        Connections{
                                            target: plmData.projectHub()
                                            function onProjectToBeClosed(projectId) {

                                                if (projectId === currentProjectId){
                                                    // save
                                                    writingZone.clearNoteWritingZone()
                                                }
                                            }
                                        }

                                        function clearNoteWritingZone(){
                                            if(paperId !== -2 && projectId !== -2){
                                                contentSaveTimer.stop()
                                                saveContent()
                                                saveCurrentPaperCursorPositionAndYTimer.stop()
                                                saveCurrentPaperCursorPositionAndY()
                                                skrTextBridge.unsubscribeTextDocument(pageType, projectId, paperId, writingZone.textArea.objectName, writingZone.textArea.textDocument)
                                                projectId = -2
                                                paperId = -2

                                            }

                                            writingZone.clear()
                                        }

                                        //---------------------------------------------------------

                                        function openSynopsisFromSheetId(projectId, sheetId){
                                            var synopsisId = plmData.noteHub().getSynopsisNoteId(projectId, sheetId)

                                            if(synopsisId === -2){ // no synopsis, create one
                                                var result = plmData.noteHub().createSynopsis(projectId, sheetId)
                                                synopsisId = result.getData("noteId", -2);
                                                plmData.noteHub().setTitle(projectId, synopsisId, model.name)
                                                //plmData.notePropertyHub().setProperty(projectId, synopsisId, "label", qsTr("Outline"))
                                                if(synopsisId === -2){
                                                    console.warn("can't find synopsis of", projectId, sheetId)
                                                    //TODO: add notification
                                                    return
                                                }
                                            }


                                            openSynopsis(projectId, synopsisId)

                                        }

                                        //---------------------------------------------------------

                                        QtObject {
                                            id: documentPrivate
                                            property bool contentSaveTimerAllowedToStart: true
                                            property bool saveCurrentPaperCursorPositionAndYTimerAllowedToStart: true
                                        }
                                        //---------------------------------------------------------


                                        function openSynopsis(_projectId, _paperId){
                                            // save current
                                            if(projectId !== _projectId && paperId !== _paperId ){ //meaning it hasn't just used the constructor
                                                    clearNoteWritingZone()
                                            }

                                            documentPrivate.contentSaveTimerAllowedToStart = false
                                            documentPrivate.saveCurrentPaperCursorPositionAndYTimerAllowedToStart = false


                                            paperId = _paperId
                                            projectId = _projectId

                                            console.log("opening note :", _projectId, _paperId)
                                            writingZone.text = plmData.noteHub().getContent(_projectId, _paperId)
                                            title = plmData.noteHub().getTitle(_projectId, _paperId)

                                            skrTextBridge.subscribeTextDocument(writingZone.pageType, projectId, paperId, writingZone.textArea.objectName, writingZone.textArea.textDocument)

                                            // apply format
                                            writingZone.documentHandler.indentEverywhere = SkrSettings.overviewTreeNoteSettings.textIndent
                                            writingZone.documentHandler.topMarginEverywhere = SkrSettings.overviewTreeNoteSettings.textTopMargin

                                            //restoreCurrentPaperCursorPositionAndY()

                                            //writingZone.forceActiveFocus()
                                            //save :
                                            skrUserSettings.setProjectSetting(projectId, "overViewTreeNoteCurrentPaperId", paperId)

                                            // start the timer for automatic position saving
                                            documentPrivate.saveCurrentPaperCursorPositionAndYTimerAllowedToStart = true
                                            if(!saveCurrentPaperCursorPositionAndYTimer.running){
                                                saveCurrentPaperCursorPositionAndYTimer.start()
                                            }
                                            documentPrivate.contentSaveTimerAllowedToStart = true


                                        }


                                        function restoreCurrentPaperCursorPositionAndY(){

                                            //get cursor position
                                            var position = skrUserSettings.getFromProjectSettingHash(
                                                        projectId, "overViewTreeNotePositionHash", paperId, 0)
                                            //get Y
                                            var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                                                        projectId, "overViewTreeNoteYHash", paperId, 0)

                                            // set positions :
                                            if(position >= writingZone.textArea.length){
                                                position = writingZone.textArea.length - 1
                                            }

                                            writingZone.setCursorPosition(position)
                                            writingZone.flickable.contentY = visibleAreaY

                                        }

                                        function saveCurrentPaperCursorPositionAndY(){

                                            if(paperId !== -2 || projectId !== -2){
                                                //save cursor position of current document :

                                                var previousCursorPosition = writingZone.textArea.cursorPosition
                                                //console.log("previousCursorPosition", previousCursorPosition)
                                                var previousY = writingZone.flickable.contentY
                                                //console.log("previousContentY", previousY)
                                                skrUserSettings.insertInProjectSettingHash(
                                                            projectId, "overViewTreeNotePositionHash", paperId,
                                                            previousCursorPosition)
                                                skrUserSettings.insertInProjectSettingHash(projectId,
                                                                                           "overViewTreeNoteYHash",
                                                                                           paperId,
                                                                                           previousY)
                                            }
                                        }

                                        Timer{
                                            id: saveCurrentPaperCursorPositionAndYTimer
                                            repeat: true
                                            interval: 10000
                                            onTriggered: saveCurrentPaperCursorPositionAndY()
                                        }



                                        //------------------------------------------------------------
                                        //------------------------------------------------------------
                                        //------------------------------------------------------------


                                        // save content once after writing:
                                        textArea.onTextChanged: {

                                            //avoid first text change, when blank HTML is inserted
                                            if(writingZone.textArea.length === 0
                                                    && plmData.projectHub().isProjectNotModifiedOnce(projectId)){
                                                return
                                            }

                                            if(contentSaveTimer.running){
                                                contentSaveTimer.stop()
                                            }
                                            if(documentPrivate.contentSaveTimerAllowedToStart){
                                            contentSaveTimer.start()
                                            }
                                        }
                                        Timer{
                                            id: contentSaveTimer
                                            repeat: false
                                            interval: 200
                                            onTriggered: saveContent()
                                        }

                                        function saveContent(){
                                            if(paperId === -2 || projectId === -2){
                                                return
                                            }


                                            //console.log("saving note")
                                            var result = plmData.noteHub().setContent(projectId, paperId, writingZone.text)
                                            if (!result.success){
                                                console.log("saving note failed", projectId, paperId)
                                            }
                                            else {
                                                //console.log("saving note success", projectId, paperId)

                                            }
                                        }
                                    }

                                }


                                Loader {
                                    id: noteWritingZoneLoader
                                    sourceComponent: noteWritingZoneComponent
                                    asynchronous: false

                                    Layout.fillHeight: true
                                    Layout.fillWidth: true

                                }

                            }

                        }

                        RowLayout{
                            id: noteBox
                            Layout.minimumWidth: 50
                            Layout.maximumWidth: 400
                            Layout.fillHeight: true
                            Layout.fillWidth: true

                            onWidthChanged: {
                                if(width === 50 && Component.status === Component.Ready){
                                    SkrSettings.overviewTreeSettings.noteBoxVisible = false
                                }
                            }
                            visible: SkrSettings.overviewTreeSettings.noteBoxVisible

                            Rectangle {
                                Layout.preferredWidth: 1
                                Layout.preferredHeight: content.height / 2
                                Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                                gradient: Gradient {
                                    orientation: Qt.Vertical
                                    GradientStop {
                                        position: 0.00;
                                        color: "#ffffff";
                                    }
                                    GradientStop {
                                        position: 0.30;
                                        color: "#9e9e9e";
                                    }
                                    GradientStop {
                                        position: 0.70;
                                        color: "#9e9e9e";
                                    }
                                    GradientStop {
                                        position: 1.00;
                                        color: "#ffffff";
                                    }
                                }

                            }


                            NotePad {
                                id: notePad
                                Layout.fillHeight: true
                                Layout.fillWidth: true


                                minimalMode: true
                                projectId: model.projectId
                                sheetId: model.paperId
                            }



                        }

                        //-----------------------------------------------------------
                        //---------------Tags :---------------------------------------------
                        //-----------------------------------------------------------

                        RowLayout{
                            id: tagBox
                            Layout.minimumWidth: 50
                            Layout.maximumWidth: 400
                            Layout.fillHeight: true
                            Layout.fillWidth: true



                            onWidthChanged: {
                                if(width === 50 && Component.status === Component.Ready){
                                    SkrSettings.overviewTreeSettings.tagBoxVisible = false
                                }
                            }
                            visible: SkrSettings.overviewTreeSettings.tagBoxVisible


                            Rectangle {
                                Layout.preferredWidth: 1
                                Layout.preferredHeight: content.height / 2
                                Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                                gradient: Gradient {
                                    orientation: Qt.Vertical
                                    GradientStop {
                                        position: 0.00;
                                        color: "#ffffff";
                                    }
                                    GradientStop {
                                        position: 0.30;
                                        color: "#9e9e9e";
                                    }
                                    GradientStop {
                                        position: 0.70;
                                        color: "#9e9e9e";
                                    }
                                    GradientStop {
                                        position: 1.00;
                                        color: "#ffffff";
                                    }
                                }

                            }


                            TagPad{
                                id: tagPad

                                Layout.fillHeight: true
                                Layout.fillWidth: true


                                minimalMode: true
                                projectId: model.projectId
                                itemId: model.paperId


                                //proxy model for tag list :

                                SKRSearchTagListProxyModel {
                                    id: tagProxyModel
                                    projectIdFilter: projectId
                                    sheetIdFilter: paperId
                                }
                                tagListModel: tagProxyModel
                                itemType: SKR.Sheet

                            }
                        }


                        //-----------------------------------------------------------
                        //---------------Counts :---------------------------------------------
                        //-----------------------------------------------------------

                        ColumnLayout{
                            id: countBox
                            visible: SkrSettings.overviewTreeSettings.characterCountBoxVisible || SkrSettings.overviewTreeSettings.wordCountBoxVisible
                            //Layout.minimumWidth: 50
                            //Layout.maximumWidth: 100
                            Layout.fillHeight: true
                            Layout.fillWidth: false

                            ColumnLayout{
                                id: characterCountLayout
                                visible: SkrSettings.overviewTreeSettings.characterCountBoxVisible
                                Layout.fillHeight: true
                                Layout.fillWidth: true

                                SkrLabel{
                                    id: characterCountLabel
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true
                                    Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                                    text: qsTr("c: %1").arg(skrRootItem.toLocaleIntString(model.charCount))
                                    verticalAlignment: Qt.AlignVCenter
                                }
                                SkrLabel{
                                    id: characterCountWithChildrenLabel
                                    visible: model.hasChildren
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true
                                    Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                                    text: qsTr("all c: %1").arg(skrRootItem.toLocaleIntString(model.charCountWithChildren))
                                    verticalAlignment: Qt.AlignVCenter
                                }
                            }

                            ColumnLayout{
                                id: wordCountLayout
                                visible: SkrSettings.overviewTreeSettings.wordCountBoxVisible
                                Layout.fillHeight: true
                                Layout.fillWidth: true

                                SkrLabel{
                                    id: wordCountLabel
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true
                                    Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                                    text: qsTr("w: %1").arg(skrRootItem.toLocaleIntString(model.wordCount))
                                    verticalAlignment: Qt.AlignVCenter
                                }
                                SkrLabel{
                                    id: wordCountWithChildrenLabel
                                    visible: model.hasChildren
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true
                                    Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                                    text: qsTr("all w: %1").arg(skrRootItem.toLocaleIntString(model.wordCountWithChildren))
                                    verticalAlignment: Qt.AlignVCenter
                                }
                            }


                        }

                        RowLayout{
                            id: buttonsBox
                            Layout.preferredWidth: 40
                            visible: hoverHandler.hovered | draggableContent.isCurrent

                            Rectangle {
                                Layout.preferredWidth: 1
                                Layout.preferredHeight: content.height / 2
                                Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                                gradient: Gradient {
                                    orientation: Qt.Vertical
                                    GradientStop {
                                        position: 0.00;
                                        color: "#ffffff";
                                    }
                                    GradientStop {
                                        position: 0.30;
                                        color: "#9e9e9e";
                                    }
                                    GradientStop {
                                        position: 0.70;
                                        color: "#9e9e9e";
                                    }
                                    GradientStop {
                                        position: 1.00;
                                        color: "#ffffff";
                                    }
                                }

                            }

                            ColumnLayout {
                                Layout.preferredWidth: 30

                                SkrToolButton {
                                    id: menuButton
                                    Layout.fillHeight: true
                                    Layout.preferredWidth: 30

                                    text: "..."
                                    flat: true
                                    focusPolicy: Qt.NoFocus

                                    onClicked: {
                                        contextMenuItemIndex = model.index
                                        listView.currentIndex = model.index
                                        delegateRoot.forceActiveFocus()


                                        if(menu.visible){
                                          menu.close()
                                            return
                                        }
                                        menu.popup(menuButton, menuButton.x, menuButton.height)

                                    }

                                    visible: hoverHandler.hovered | draggableContent.isCurrent
                                }

                                SkrToolButton {
                                    id: focusOnBranchButton
                                    Layout.fillHeight: true
                                    Layout.preferredWidth: 30

                                    text: "focus"
                                    icon.source: "qrc:///icons/backup/edit-find.svg"
                                    display: AbstractButton.IconOnly
                                    flat: true
                                    visible: false
                                    checkable: true
                                    checked: delegateRoot.focusOnBranchChecked

                                    onCheckedChanged: {

                                        if(focusOnBranchButton.activeFocus){

                                            contextMenuItemIndex = model.index
                                            listView.currentIndex = model.index
                                            delegateRoot.forceActiveFocus()

                                            focusOnbranchAction.trigger()
                                        }

                                    }

                                }
                            }

                        }


                    }
                }
                states: [
                    State {
                        name: "displayMode_1"
                        when: root.displayMode === 1

                        PropertyChanges {
                            target: content
                            height: 100
                        }
                        PropertyChanges {
                            target: focusOnBranchButton
                            visible: hoverHandler.hovered | draggableContent.isCurrent
                        }
                    },
                    State {
                        name: "displayMode_2"
                        when: root.displayMode === 2

                        PropertyChanges {
                            target: content
                            height: 200
                        }
                        PropertyChanges {
                            target: focusOnBranchButton
                            visible: hoverHandler.hovered | draggableContent.isCurrent
                        }


                    }

                ]

                property int transitionAnimationDuration: 150

                transitions: [
                    Transition {
                        SequentialAnimation{
                            PropertyAnimation {
                                properties: "height"
                                duration: draggableContent.transitionAnimationDuration
                                easing.type: Easing.InOutQuad
                            }
                            ScriptAction {
                                script: {
                                    // shakes the writingZone to avoid blanks when resizing
                                    synopsisBox.writingZone.flickable.contentY = 1
                                    synopsisBox.writingZone.flickable.contentY = 0

                                }

                            }

                        }
                    }
                ]






            }
            states: [
                State {
                    name: "drag_active"
                    when: draggableContent.Drag.active

                    ParentChange {
                        target: draggableContent
                        parent: base
                    }
                    AnchorChanges {
                        target: draggableContent
                        anchors {
                            horizontalCenter: undefined
                            verticalCenter: undefined
                        }
                    }
                },

                State {
                    name: "unset_anchors"
                    AnchorChanges {
                        target: delegateRoot
                        anchors.left: undefined
                        anchors.right: undefined

                    }
                }
            ]




            property int paperIdToEdit: -2
            onPaperIdToEditChanged: {
                if(paperIdToEdit !== -2){
                    editNameTimer.start()
                }
            }

            Timer{
                id: editNameTimer
                repeat: false
                interval: draggableContent.transitionAnimationDuration
                onTriggered: {
                    var index = proxyModel.findVisualIndex(model.projectId, paperIdToEdit)
                    if(index !== -2){
                        listView.itemAtIndex(index).editName()
                    }
                    paperIdToEdit = -2
                }

            }





            SequentialAnimation {
                id: removePaperAnimation
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: true
                }
                NumberAnimation {
                    target: delegateRoot
                    property: "height"
                    to: 0
                    duration: 250
                    easing.type: Easing.InOutQuad
                }
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: false
                }
            }


        }
    }

    listView.remove: Transition {

        SequentialAnimation {
            id: removePaperAnimation
            PropertyAction {
                property: "ListView.delayRemove"
                value: true
            }
            PropertyAction {
                property: "state"
                value: "unset_anchors"
            }

            NumberAnimation {
                property: "x"
                to: listView.width
                duration: 250
                easing.type: Easing.InBack
            }
            PropertyAction {
                property: "ListView.delayRemove"
                value: false
            }
        }
    }

    listView.removeDisplaced: Transition {
        SequentialAnimation {
            PauseAnimation{duration: 250}
            NumberAnimation { properties: "x,y"; duration: 250 }
        }

    }


    SkrMenu {
        id: menu


        onOpened: {

        }

        onClosed: {
        }
        SkrMenuItem {
            visible: currentPaperId !== -1
            height: currentPaperId === -1 ? 0 : undefined
            action: Action {
                id: openPaperAction
                text: qsTr("Open")
                //shortcut: "Return"
                icon {
                    source: "qrc:///icons/backup/document-edit.svg"
                }

                enabled: listView.enabled && currentPaperId !== -1
                onTriggered: {
                    console.log("open paper action",currentProjectId, currentPaperId)
                    root.openDocument(root.openedProjectId, root.openedPaperId, currentProjectId, currentPaperId)
                }
            }
        }
        SkrMenuItem {
            visible: currentPaperId !== -1
            height: currentPaperId === -1 ? 0 : undefined

            action: Action {
                id: openPaperInNewTabAction
                text: qsTr("Open in new tab")
                //shortcut: "Alt+Return"
                icon {
                    source: "qrc:///icons/backup/tab-new.svg"
                }
                enabled: listView.enabled && currentPaperId !== -1
                onTriggered: {
                    console.log("open paper in new tab action", currentProjectId, currentPaperId)
                    root.openDocumentInNewTab(currentProjectId, currentPaperId)
                }
            }
        }


        SkrMenuItem {
            visible: currentPaperId !== -1
            height: currentPaperId === -1 ? 0 : undefined

            action: Action {
                id: openPaperInNewWindowAction
                text: qsTr("Open in new window")
                //shortcut: "Alt+Return"
                icon {
                    source: "qrc:///icons/backup/window-new.svg"
                }
                enabled: listView.enabled && currentPaperId !== -1
                onTriggered: {
                    console.log("open paper in new window action", currentProjectId, currentPaperId)
                    root.openDocumentInNewWindow(currentProjectId, currentPaperId)
                }
            }
        }


        MenuSeparator {}


        SkrMenuItem {
            visible: currentPaperId !== -1
            height: currentPaperId === -1 ? 0 : undefined

            action: Action {
                id: focusOnbranchAction
                text: listView.itemAtIndex(currentIndex).focusOnBranchChecked ? qsTr("Unset focus") : qsTr("Set focus")
                //shortcut: "Alt+Return"
                icon {
                    source: "qrc:///icons/backup/edit-find.svg"
                }
                enabled: listView.enabled && currentPaperId !== -1
                onTriggered: {
                    console.log("focus action", currentProjectId, currentPaperId)
                    var checked = listView.itemAtIndex(currentIndex).focusOnBranchChecked

                    // filter to this parent and its children
                    if(checked){
                        listView.itemAtIndex(currentIndex).focusOnBranchChecked = false
                        proxyModel.showParentWhenParentIdFilter = false
                        proxyModel.parentIdFilter = -2
                    }
                    else {
                        listView.itemAtIndex(currentIndex).focusOnBranchChecked = true
                        proxyModel.showParentWhenParentIdFilter = true
                        proxyModel.parentIdFilter = currentPaperId


                    }
                }
            }
        }

        MenuSeparator {}

        Action {
            id: renameAction
            text: qsTr("Rename")
            //shortcut: "F2"
            icon {
                source: "qrc:///icons/backup/edit-rename.svg"
            }
            enabled: listView.enabled

            onTriggered: {
                console.log("rename action", currentProjectId, currentPaperId)
                listView.itemAtIndex(currentIndex).editName()
            }
        }

        SkrMenuItem {
            visible: currentPaperId !== -1
            height: currentPaperId === -1 ? 0 : undefined
            action: Action {

                id: setLabelAction
                text: qsTr("Set label")
                //shortcut: "F2"
                icon {
                    source: "qrc:///icons/backup/label.svg"
                }
                enabled: listView.enabled && currentPaperId !== -1
                onTriggered: {
                    console.log("sel label", currentProjectId, currentPaperId)
                    listView.itemAtIndex(currentIndex).editLabel()
                }
            }
        }

        MenuSeparator {}
        SkrMenuItem {
            visible: currentPaperId !== -1
            height: currentPaperId === -1 ? 0 : undefined
            action: Action {
                id: cutAction
                text: qsTr("Cut")
                //shortcut: StandardKey.Cut
                icon {
                    source: "qrc:///icons/backup/edit-cut.svg"
                }
                enabled: listView.enabled && currentPaperId !== -1

                onTriggered: {
                    console.log("cut action", currentProjectId, currentPaperId)
                    proxyModel.cut(currentProjectId, currentPaperId)
                }
            }
        }

        SkrMenuItem {
            visible: currentPaperId !== -1
            height: currentPaperId === -1 ? 0 : undefined
            action:
                Action {

                id: copyAction
                text: qsTr("Copy")
                //shortcut: StandardKey.Copy
                icon {
                    source: "qrc:///icons/backup/edit-copy.svg"
                }
                enabled: listView.enabled && currentPaperId !== -1

                onTriggered: {
                    console.log("copy action", currentProjectId, currentPaperId)
                    proxyModel.copy(currentProjectId, currentPaperId)
                }
            }
        }

        SkrMenuItem {
            visible: currentPaperId !== -1
            height: currentPaperId === -1 ? 0 : undefined
            action:
                Action {

                id: pasteAction
                text: qsTr("Paste")
                //shortcut: StandardKey.Copy
                icon {
                    source: "qrc:///icons/backup/edit-paste.svg"
                }
                enabled: listView.enabled && currentPaperId !== -1

                onTriggered: {
                    console.log("copy action", currentProjectId, currentPaperId)
                    proxyModel.paste(currentProjectId, currentPaperId)
                }
            }
        }

        MenuSeparator {}

        SkrMenuItem {
            visible: currentPaperId !== -1
            height: currentPaperId === -1 ? 0 : undefined
            action:
                Action {
                id: addBeforeAction
                text: qsTr("Add before")
                //shortcut: "Ctrl+Shift+N"
                icon {
                    source: "qrc:///icons/backup/document-new.svg"
                }
                enabled: listView.enabled && currentPaperId !== -1
                onTriggered: {
                    console.log("add before action", currentProjectId, currentPaperId)
                    var result = plmData.sheetHub().addPaperAbove(currentProjectId, currentPaperId)
                    // edit it :
                    if(result){
                        listView.itemAtIndex(currentIndex).paperIdToEdit = result.getData("sheetId", -2) //start when paperIdToEdit changes
                    }
                }
            }
        }

        SkrMenuItem {
            visible: currentPaperId !== -1
            height: currentPaperId === -1 ? 0 : undefined
            action:
                Action {

                id: addAfterAction
                text: qsTr("Add after")
                //shortcut: "Ctrl+N"
                icon {
                    source: "qrc:///icons/backup/document-new.svg"
                }
                enabled: listView.enabled && currentPaperId !== -1
                onTriggered: {
                    console.log("add after action", currentProjectId, currentPaperId)
                    var result = plmData.sheetHub().addPaperBelow(currentProjectId, currentPaperId)
                    // edit it :
                    if(result){
                        listView.itemAtIndex(currentIndex).paperIdToEdit = result.getData("sheetId", -2)
                    }

                }
            }
        }

        Action {
            id: addChildAction
            text: qsTr("Add child")
            //shortcut: "Ctrl+N"
            icon {
                source: "qrc:///icons/backup/document-new.svg"
            }
            enabled: listView.enabled
            onTriggered: {
                console.log("add child action", currentProjectId, currentPaperId)

                var result = plmData.sheetHub().addChildPaper(currentProjectId, currentPaperId)
                // edit it :
                if(result){
                    listView.itemAtIndex(currentIndex).paperIdToEdit = result.getData("sheetId", -2)
                }


            }





        }

        MenuSeparator {}
        SkrMenuItem {
            visible: currentPaperId !== -1
            height: currentPaperId === -1 ? 0 : undefined
            action: Action {
                id: moveUpAction
                text: qsTr("Move up")
                //shortcut: "Ctrl+Up"
                icon {
                    source: "qrc:///icons/backup/object-order-raise.svg"
                }
                enabled:listView.enabled && currentIndex !== 0 && currentPaperId !== -1
                onTriggered: {
                    console.log("move up action", currentProjectId, currentPaperId)

                    proxyModel.moveUp(currentProjectId, currentPaperId, currentIndex)

                }
            }
        }

        SkrMenuItem {
            visible: currentPaperId !== -1
            height: currentPaperId === -1 ? 0 : undefined
            action:
                Action {
                id: moveDownAction
                text: qsTr("Move down")
                //shortcut: "Ctrl+Down"
                icon {
                    source: "qrc:///icons/backup/object-order-lower.svg"
                }
                enabled: currentIndex !== visualModel.items.count - 1  && listView.enabled && currentPaperId !== -1

                onTriggered: {
                    console.log("move down action", currentProjectId, currentPaperId)

                    proxyModel.moveDown(currentProjectId, currentPaperId, currentIndex)
                }
            }
        }
        MenuSeparator {}

        SkrMenuItem {
            visible: currentPaperId !== -1
            height: currentPaperId === -1 ? 0 : undefined
            action:
                Action {
                id: sendToTrashAction
                text: qsTr("Send to trash")
                //shortcut: "Del"
                icon {
                    source: "qrc:///icons/backup/edit-delete.svg"
                }
                enabled: listView.enabled && currentPaperId !== -1 && currentPaperId !== -1
                onTriggered: {
                    console.log("sent to trash action", currentProjectId, currentPaperId)
                    proxyModel.trashItemWithChildren(currentProjectId, currentPaperId)

                }
            }
        }
    }


}

