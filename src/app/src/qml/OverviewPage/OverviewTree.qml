import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import QtQml 2.15
import QtQuick.Controls.Material 2.15
import eu.skribisto.projecthub 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import eu.skribisto.taghub 1.0
import "../Commons"
import "../Items"
import ".."

OverviewTreeForm {


    id: root

    signal openDocument(int openedProjectId, int openedTreeItemId, int projectId, int treeItemId)
    signal openDocumentInNewTab(int projectId, int treeItemId)
    signal openDocumentInNewWindow(int projectId, int treeItemId)

    property int currentTreeItemId: -2
    property int currentProjectId: -2
    property int openedProjectId: -2
    property int openedTreeItemId: -2
    property int currentIndex: listView.currentIndex

    property alias visualModel: visualModel
    property var proxyModel
    listView.model: visualModel


    DelegateModel {
        id: visualModel

        delegate: dragDelegate
        model: proxyModel
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

    function getIconUrlFromPageType(type){
       return skrTreeManager.getIconUrlFromPageType(type)
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

                var typeText = skrTreeManager.getPageTypeText(model.type)

                var labelText = ""
                if(labelLabel.text.length > 0){
                    labelText = qsTr("label: %1").arg(labelLabel.text)
                }

                var hasChildrenText = ""
                if(model.hasChildren){
                    hasChildrenText = qsTr("has %1 sub-items").arg(proxyModel.getChildrenCount(model.projectId, model.treeItemId))
                }

                return levelText + " " + titleText + " " + typeText + " " + labelText + " " + hasChildrenText

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
                    root.currentTreeItemId = model.treeItemId
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

            Keys.onShortcutOverride: function(event)  {

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

            Keys.onPressed: function(event) {
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

                // add child
                if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Space && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    addChildAction.trigger()
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
                    enabled: SkrSettings.ePaperSettings.animationEnabled
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
                    onWheel: function(wheel) {
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
                        root.openDocument(openedProjectId, openedTreeItemId, model.projectId,
                                          model.treeItemId)
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
                                                  model.treeItemId)

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
                                                     model.treeItemId)

                    }
                }




                SkrListItemPane{
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

                        onSingleTapped: function(eventPoint) {
                            currentTreeItemId = model.treeItemId
                            currentProjectId = model.projectId
                            listView.currentIndex = model.index
                            delegateRoot.forceActiveFocus()
                            eventPoint.accepted = true

                        }

                        onDoubleTapped: function(eventPoint) {
                            //console.log("double tapped")

                            currentTreeItemId = model.treeItemId
                            currentProjectId = model.projectId
                            listView.currentIndex = model.index
                            openDocumentAction.trigger()
                            eventPoint.accepted = true
                        }


                        onLongPressed: { // needed to activate the grab handler
                            currentTreeItemId = model.treeItemId
                            currentProjectId = model.projectId
                            if(root.dragDropEnabled){
                                enabled = false
                            }
                        }

                        onGrabChanged: function(transition, point) {
                            point.accepted = false

                        }

                    }

                    TapHandler {
                        id: rightClickTapHandler
                        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                        acceptedButtons: Qt.RightButton
                        onSingleTapped: function(eventPoint) {
                            //console.log("right clicked")
                            if(menu.visible){
                                menu.close()
                                return
                            }

                            currentTreeItemId = model.treeItemId
                            currentProjectId = model.projectId

                            // necessary to differenciate between all items
                            contextMenuItemIndex = model.index
                            listView.currentIndex = model.index

                            menu.popup(content, eventPoint.position.x, eventPoint.position.y)
                            eventPoint.accepted = true
                        }
                    }
                    TapHandler {
                        id: middleClickTapHandler
                        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                        acceptedButtons: Qt.MiddleButton
                        onSingleTapped: function(eventPoint) {
                            currentTreeItemId = model.treeItemId
                            currentProjectId = model.projectId
                            listView.currentIndex = model.index
                            delegateRoot.forceActiveFocus()
                            openDocumentInNewTabAction.trigger()
                            eventPoint.accepted = true

                        }
                    }

                    contentItem: RowLayout{
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

                                Item {
                                    Layout.maximumHeight: 30
                                    implicitHeight: treeItemIconIndicator.implicitHeight
                                    implicitWidth: treeItemIconIndicator.implicitWidth

                                    SkrToolButton {
                                        id: treeItemIconIndicator

                                        enabled: true
                                        focusPolicy: Qt.NoFocus
                                        implicitHeight: 32
                                        implicitWidth: 32
                                        padding: 0
                                        rightPadding: 0
                                        bottomPadding: 0
                                        leftPadding: 2
                                        topPadding: 0
                                        flat: true
                                        onDownChanged: down = false

                                        onClicked: {

                                        }


                                        icon {
                                            source: getIconUrlFromPageType(model.type)

                                            height: 22
                                            width: 22
                                        }


                                        hoverEnabled: true
//                                                    ToolTip.delay: 1000
//                                                    ToolTip.timeout: 5000
//                                                    ToolTip.visible: hovered
//                                                    ToolTip.text: qsTr("")




                                        Item {
                                            id: treeItemIconIndicatorHat
                                            anchors.fill: treeItemIconIndicator
                                            z: 1

                                            TapHandler{

                                                onSingleTapped: function(eventPoint) {
                                                    tapHandler.singleTapped(point)
                                                }

                                                onDoubleTapped: function(eventPoint) {
                                                    tapHandler.doubleTapped(point)
                                                }

                                                onGrabChanged: function(transition, point) {
                                                    tapHandler.grabChanged(transition, point)
                                                }


                                            }

                                            TapHandler{
                                                acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                                                acceptedButtons: Qt.RightButton

                                                onSingleTapped: function(eventPoint) {
                                                    rightClickTapHandler.singleTapped(point)
                                                }

                                                onDoubleTapped: function(eventPoint) {
                                                    rightClickTapHandler.doubleTapped(point)
                                                }

                                                onGrabChanged: function(transition, point) {
                                                    rightClickTapHandler.grabChanged(transition, point)
                                                }


                                            }

                                            TapHandler{
                                                acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                                                acceptedButtons: Qt.MiddleButton

                                                onSingleTapped: function(eventPoint) {
                                                    middleClickTapHandler.singleTapped(point)
                                                }

                                                onDoubleTapped: function(eventPoint) {
                                                    middleClickTapHandler.doubleTapped(point)
                                                }

                                                onGrabChanged: function(transition, point) {
                                                    middleClickTapHandler.grabChanged(transition, point)
                                                }


                                            }
                                        }
                                    }

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
                                        Keys.onShortcutOverride: function(event) {
                                            event.accepted = (event.key === Qt.Key_Escape)
                                        }
                                        Keys.onPressed: function(event) {
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
                                        Keys.onShortcutOverride: function(event) { event.accepted = (event.key === Qt.Key_Escape)}
                                        Keys.onPressed: function(event) {
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

                                    RowLayout{
                                        id: labelLayout
                                        Layout.fillWidth: true
                                        Layout.leftMargin: 5

                                        ListItemAttributes{
                                            id: attributes
                                            treeItemId: model.treeItemId
                                            projectId: model.projectId
                                            Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                            Layout.leftMargin: 4
                                            Layout.bottomMargin: 2

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
                                            horizontalAlignment: Qt.AlignRight
                                            Layout.fillWidth: true
                                        }
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
                                            color: "transparent";
                                        }
                                        GradientStop {
                                            position: 0.30;
                                            color: SkrTheme.divider;
                                        }
                                        GradientStop {
                                            position: 0.70;
                                            color: SkrTheme.divider;
                                        }
                                        GradientStop {
                                            position: 1.00;
                                            color: "transparent";
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
                                        paneStyleBackgroundColor: SkrTheme.listItemBackground
                                        textAreaStyleAccentColor: SkrTheme.accent

                                        Component.onCompleted: {
                                            openSynopsisFromSheetId(model.projectId, model.treeItemId)
                                        }


                                        // project to be closed :
                                        Connections{
                                            target: skrData.projectHub()
                                            function onProjectToBeClosed(projectId) {

                                                if (projectId === currentProjectId){
                                                    // save
                                                    writingZone.clearNoteWritingZone()
                                                }
                                            }
                                        }

                                        function clearNoteWritingZone(){
                                            if(treeItemId !== -2 && projectId !== -2){
                                                contentSaveTimer.stop()
                                                saveContent()
                                                saveCurrentPaperCursorPositionAndYTimer.stop()
                                                saveCurrentPaperCursorPositionAndY()
                                                skrTextBridge.unsubscribeTextDocument(pageType, projectId, treeItemId, writingZone.textArea.objectName, writingZone.textArea.textDocument)
                                                projectId = -2
                                                treeItemId = -2

                                            }

                                            writingZone.setCursorPosition(0)
                                            writingZone.clear()
                                        }

                                        //---------------------------------------------------------

                                        function openSynopsisFromSheetId(projectId, sheetId){
                                            var synopsisId = skrData.noteHub().getSynopsisNoteId(projectId, sheetId)

                                            if(synopsisId === -2){ // no synopsis, create one
                                                var result = skrData.noteHub().createSynopsis(projectId, sheetId)
                                                synopsisId = result.getData("noteId", -2);
                                                skrData.noteHub().setTitle(projectId, synopsisId, model.name)
                                                //skrData.notePropertyHub().setProperty(projectId, synopsisId, "label", qsTr("Outline"))
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


                                        function openSynopsis(_projectId, _treeItemId){
                                            // save current
                                            if(projectId !== _projectId && treeItemId !== _treeItemId ){ //meaning it hasn't just used the constructor
                                                clearNoteWritingZone()
                                            }

                                            documentPrivate.contentSaveTimerAllowedToStart = false
                                            documentPrivate.saveCurrentPaperCursorPositionAndYTimerAllowedToStart = false


                                            treeItemId = _treeItemId
                                            projectId = _projectId

                                            //console.log("opening note :", _projectId, _treeItemId)
                                            writingZone.setCursorPosition(0)
                                            writingZone.text = skrData.noteHub().getContent(_projectId, _treeItemId)

                                            skrTextBridge.subscribeTextDocument(writingZone.pageType, projectId, treeItemId, writingZone.textArea.objectName, writingZone.textArea.textDocument)

                                            // apply format
                                            writingZone.documentHandler.indentEverywhere = SkrSettings.overviewTreeNoteSettings.textIndent
                                            writingZone.documentHandler.topMarginEverywhere = SkrSettings.overviewTreeNoteSettings.textTopMargin

                                            //restoreCurrentPaperCursorPositionAndY()

                                            //writingZone.forceActiveFocus()
                                            //save :
                                            skrUserSettings.setProjectSetting(projectId, "overViewTreeNoteCurrentTreeItemId", treeItemId)

                                            // start the timer for automatic position saving
                                            documentPrivate.saveCurrentPaperCursorPositionAndYTimerAllowedToStart = true
                                            if(!saveCurrentPaperCursorPositionAndYTimer.running){
                                                saveCurrentPaperCursorPositionAndYTimer.start()
                                            }
                                            documentPrivate.contentSaveTimerAllowedToStart = true

                                            determineModifiableTimer.start()


                                        }

                                        //---------------------------------------------------------
                                        // modifiable :

                                        property bool isModifiable: true

                                        Connections{
                                            target: skrData.treePropertyHub()
                                            function onPropertyChanged(_projectId, propertyId, _treeItemId, name, value){
                                                if(_projectId === writingZone.projectId && _treeItemId === writingZone.treeItemId){

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

                                            isModifiable = skrData.notePropertyHub().getProperty(writingZone.projectId, writingZone.treeItemId, "modifiable", "true") === "true"

                                            if(!isModifiable !== writingZone.textArea.readOnly){
                                                saveCurrentPaperCursorPositionAndY()
                                                writingZone.textArea.readOnly = !isModifiable
                                                restoreCurrentPaperCursorPositionAndY()
                                            }

                                        }

                                        //--------------------------------------------------------

                                        function restoreCurrentPaperCursorPositionAndY(){

                                            //get cursor position
                                            var position = skrUserSettings.getFromProjectSettingHash(
                                                        projectId, "overViewTreeNotePositionHash", treeItemId, 0)
                                            //get Y
                                            var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                                                        projectId, "overViewTreeNoteYHash", treeItemId, 0)

                                            // set positions :
                                            if(position >= writingZone.textArea.length){
                                                position = writingZone.textArea.length - 1
                                            }

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


                                        function saveCurrentPaperCursorPositionAndY(){

                                            if(treeItemId !== -2 || projectId !== -2){
                                                //save cursor position of current document :

                                                var previousCursorPosition = writingZone.textArea.cursorPosition
                                                //console.log("previousCursorPosition", previousCursorPosition)
                                                var previousY = writingZone.flickable.contentY
                                                //console.log("previousContentY", previousY)
                                                skrUserSettings.insertInProjectSettingHash(
                                                            projectId, "overViewTreeNotePositionHash", treeItemId,
                                                            previousCursorPosition)
                                                skrUserSettings.insertInProjectSettingHash(projectId,
                                                                                           "overViewTreeNoteYHash",
                                                                                           treeItemId,
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
                                                    && skrData.projectHub().isProjectNotModifiedOnce(projectId)){
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
                                            if(treeItemId === -2 || projectId === -2){
                                                return
                                            }


                                            //console.log("saving note")
                                            var result = skrData.noteHub().setContent(projectId, treeItemId, writingZone.text)
                                            if (!result.success){
                                                console.log("saving note failed", projectId, treeItemId)
                                            }
                                            else {
                                                //console.log("saving note success", projectId, treeItemId)

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

                        //-----------------------------------------------------------
                        //-------------- Notes :---------------------------------------------
                        //-----------------------------------------------------------


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
                                        color: "transparent";
                                    }
                                    GradientStop {
                                        position: 0.30;
                                        color: SkrTheme.divider;
                                    }
                                    GradientStop {
                                        position: 0.70;
                                        color: SkrTheme.divider;
                                    }
                                    GradientStop {
                                        position: 1.00;
                                        color: "transparent";
                                    }
                                }

                            }


//                            NotePad {
//                                id: notePad
//                                Layout.fillWidth: true
//                                Layout.fillHeight: true

//                                Layout.alignment: Qt.AlignVCenter

//                                minimalMode: true
//                                projectId: model.projectId
//                                sheetId: model.treeItemId
//                            }



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
                                        color: "transparent";
                                    }
                                    GradientStop {
                                        position: 0.30;
                                        color: SkrTheme.divider;
                                    }
                                    GradientStop {
                                        position: 0.70;
                                        color: SkrTheme.divider;
                                    }
                                    GradientStop {
                                        position: 1.00;
                                        color: "transparent";
                                    }
                                }

                            }


                            TagPad{
                                id: tagPad

                                Layout.fillWidth: true
                                Layout.fillHeight: true
                                Layout.alignment: Qt.AlignVCenter


                                minimalMode: true
                                projectId: model.projectId
                                treeItemId: model.treeItemId


                                //proxy model for tag list :

                                SKRSearchTagListProxyModel {
                                    id: tagProxyModel
                                    projectIdFilter: model.projectId
                                    treeItemIdFilter: model.treeItemId
                                }
                                tagListModel: tagProxyModel

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
                                        color: "transparent";
                                    }
                                    GradientStop {
                                        position: 0.30;
                                        color: SkrTheme.divider;
                                    }
                                    GradientStop {
                                        position: 0.70;
                                        color: SkrTheme.divider;
                                    }
                                    GradientStop {
                                        position: 1.00;
                                        color: "transparent";
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
                        enabled: SkrSettings.ePaperSettings.animationEnabled
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




            property int treeItemIdToEdit: -2
            onTreeItemIdToEditChanged: {
                if(treeItemIdToEdit !== -2){
                    editNameTimer.start()
                }
            }

            Timer{
                id: editNameTimer
                repeat: false
                interval: draggableContent.transitionAnimationDuration
                onTriggered: {
                    var index = proxyModel.findVisualIndex(model.projectId, treeItemIdToEdit)
                    if(index !== -2){
                        listView.itemAtIndex(index).editName()
                    }
                    treeItemIdToEdit = -2
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
        enabled: SkrSettings.ePaperSettings.animationEnabled

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
        enabled: SkrSettings.ePaperSettings.animationEnabled
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
            visible: currentTreeItemId !== -1
            height: currentTreeItemId === -1 ? 0 : undefined
            action: Action {
                id: openPaperAction
                text: qsTr("Open")
                //shortcut: "Return"
                icon {
                    source: "qrc:///icons/backup/document-edit.svg"
                }

                enabled: listView.enabled && currentTreeItemId !== -1
                onTriggered: {
                    console.log("open paper action",currentProjectId, currentTreeItemId)
                    root.openDocument(root.openedProjectId, root.openedTreeItemId, currentProjectId, currentTreeItemId)
                }
            }
        }
        SkrMenuItem {
            visible: currentTreeItemId !== -1
            height: currentTreeItemId === -1 ? 0 : undefined

            action: Action {
                id: openPaperInNewTabAction
                text: qsTr("Open in new tab")
                //shortcut: "Alt+Return"
                icon {
                    source: "qrc:///icons/backup/tab-new.svg"
                }
                enabled: listView.enabled && currentTreeItemId !== -1
                onTriggered: {
                    console.log("open paper in new tab action", currentProjectId, currentTreeItemId)
                    root.openDocumentInNewTab(currentProjectId, currentTreeItemId)
                }
            }
        }


        SkrMenuItem {
            visible: currentTreeItemId !== -1
            height: currentTreeItemId === -1 ? 0 : undefined

            action: Action {
                id: openPaperInNewWindowAction
                text: qsTr("Open in new window")
                //shortcut: "Alt+Return"
                icon {
                    source: "qrc:///icons/backup/window-new.svg"
                }
                enabled: listView.enabled && currentTreeItemId !== -1
                onTriggered: {
                    console.log("open paper in new window action", currentProjectId, currentTreeItemId)
                    root.openDocumentInNewWindow(currentProjectId, currentTreeItemId)
                }
            }
        }


        MenuSeparator {}


        SkrMenuItem {
            visible: currentTreeItemId !== -1
            height: currentTreeItemId === -1 ? 0 : undefined

            action: Action {
                id: focusOnbranchAction
                text: Qt.isQtObject(listView.itemAtIndex(currentIndex)) ? (listView.itemAtIndex(currentIndex).focusOnBranchChecked ? qsTr("Unset focus") : qsTr("Set focus")) : ""
                //shortcut: "Alt+Return"
                icon {
                    source: "qrc:///icons/backup/edit-find.svg"
                }
                enabled: listView.enabled && currentTreeItemId !== -1
                onTriggered: {
                    console.log("focus action", currentProjectId, currentTreeItemId)
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
                        proxyModel.parentIdFilter = currentTreeItemId


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
                console.log("rename action", currentProjectId, currentTreeItemId)
                listView.itemAtIndex(currentIndex).editName()
            }
        }

        SkrMenuItem {
            visible: currentTreeItemId !== -1
            height: currentTreeItemId === -1 ? 0 : undefined
            action: Action {

                id: setLabelAction
                text: qsTr("Set label")
                //shortcut: "F2"
                icon {
                    source: "qrc:///icons/backup/label.svg"
                }
                enabled: listView.enabled && currentTreeItemId !== -1
                onTriggered: {
                    console.log("sel label", currentProjectId, currentTreeItemId)
                    listView.itemAtIndex(currentIndex).editLabel()
                }
            }
        }

        MenuSeparator {}
        SkrMenuItem {
            visible: currentTreeItemId !== -1
            height: currentTreeItemId === -1 ? 0 : undefined
            action: Action {
                id: cutAction
                text: qsTr("Cut")
                //shortcut: StandardKey.Cut
                icon {
                    source: "qrc:///icons/backup/edit-cut.svg"
                }
                enabled: listView.enabled && currentTreeItemId !== -1

                onTriggered: {
                    console.log("cut action", currentProjectId, currentTreeItemId)
                    proxyModel.cut(currentProjectId, currentTreeItemId)
                }
            }
        }

        SkrMenuItem {
            visible: currentTreeItemId !== -1
            height: currentTreeItemId === -1 ? 0 : undefined
            action:
                Action {

                id: copyAction
                text: qsTr("Copy")
                //shortcut: StandardKey.Copy
                icon {
                    source: "qrc:///icons/backup/edit-copy.svg"
                }
                enabled: listView.enabled && currentTreeItemId !== -1

                onTriggered: {
                    console.log("copy action", currentProjectId, currentTreeItemId)
                    proxyModel.copy(currentProjectId, currentTreeItemId)
                }
            }
        }

        SkrMenuItem {
            visible: currentTreeItemId !== -1
            height: currentTreeItemId === -1 ? 0 : undefined
            action:
                Action {

                id: pasteAction
                text: qsTr("Paste")
                //shortcut: StandardKey.Copy
                icon {
                    source: "qrc:///icons/backup/edit-paste.svg"
                }
                enabled: listView.enabled && currentTreeItemId !== -1

                onTriggered: {
                    console.log("copy action", currentProjectId, currentTreeItemId)
                    proxyModel.paste(currentProjectId, currentTreeItemId)
                }
            }
        }

        MenuSeparator {}

        SkrMenuItem {
            visible: currentTreeItemId !== -1
            height: currentTreeItemId === -1 ? 0 : undefined
            action:
                Action {
                id: addBeforeAction
                text: qsTr("Add before")
                //shortcut: "Ctrl+Shift+N"
                icon {
                    source: "qrc:///icons/backup/document-new.svg"
                }
                enabled: listView.enabled && currentTreeItemId !== -1
                onTriggered: {
                    console.log("add before action", currentProjectId, currentTreeItemId)
                    var result = skrData.sheetHub().addPaperAbove(currentProjectId, currentTreeItemId)
                    // edit it :
                    if(result){
                        listView.itemAtIndex(currentIndex).treeItemIdToEdit = result.getData("sheetId", -2) //start when treeItemIdToEdit changes
                    }
                }
            }
        }

        SkrMenuItem {
            visible: currentTreeItemId !== -1
            height: currentTreeItemId === -1 ? 0 : undefined
            action:
                Action {

                id: addAfterAction
                text: qsTr("Add after")
                //shortcut: "Ctrl+N"
                icon {
                    source: "qrc:///icons/backup/document-new.svg"
                }
                enabled: listView.enabled && currentTreeItemId !== -1
                onTriggered: {
                    console.log("add after action", currentProjectId, currentTreeItemId)
                    var result = skrData.sheetHub().addPaperBelow(currentProjectId, currentTreeItemId)
                    // edit it :
                    if(result){
                        listView.itemAtIndex(currentIndex).treeItemIdToEdit = result.getData("sheetId", -2)
                    }

                }
            }
        }

        Action {
            id: addChildAction
            text: qsTr("Add a sub-item")
            //shortcut: "Ctrl+N"
            icon {
                source: "qrc:///icons/backup/document-new.svg"
            }
            enabled: listView.enabled
            onTriggered: {
                console.log("add child action", currentProjectId, currentTreeItemId)

                var result = skrData.sheetHub().addChildPaper(currentProjectId, currentTreeItemId)
                // edit it :
                if(result){
                    listView.itemAtIndex(currentIndex).treeItemIdToEdit = result.getData("sheetId", -2)
                }


            }





        }

        MenuSeparator {}
        SkrMenuItem {
            visible: currentTreeItemId !== -1
            height: currentTreeItemId === -1 ? 0 : undefined
            action: Action {
                id: moveUpAction
                text: qsTr("Move up")
                //shortcut: "Ctrl+Up"
                icon {
                    source: "qrc:///icons/backup/object-order-raise.svg"
                }
                enabled:listView.enabled && currentIndex !== 0 && currentTreeItemId !== -1
                onTriggered: {
                    console.log("move up action", currentProjectId, currentTreeItemId)

                    proxyModel.moveUp(currentProjectId, currentTreeItemId, currentIndex)

                }
            }
        }

        SkrMenuItem {
            visible: currentTreeItemId !== -1
            height: currentTreeItemId === -1 ? 0 : undefined
            action:
                Action {
                id: moveDownAction
                text: qsTr("Move down")
                //shortcut: "Ctrl+Down"
                icon {
                    source: "qrc:///icons/backup/object-order-lower.svg"
                }
                enabled: currentIndex !== visualModel.items.count - 1  && listView.enabled && currentTreeItemId !== -1

                onTriggered: {
                    console.log("move down action", currentProjectId, currentTreeItemId)

                    proxyModel.moveDown(currentProjectId, currentTreeItemId, currentIndex)
                }
            }
        }
        MenuSeparator {}

        SkrMenuItem {
            visible: currentTreeItemId !== -1
            height: currentTreeItemId === -1 ? 0 : undefined
            action:
                Action {
                id: sendToTrashAction
                text: qsTr("Send to trash")
                //shortcut: "Del"
                icon {
                    source: "qrc:///icons/backup/edit-delete.svg"
                }
                enabled: listView.enabled && currentTreeItemId !== -1 && currentTreeItemId !== -1
                onTriggered: {
                    console.log("sent to trash action", currentProjectId, currentTreeItemId)
                    proxyModel.trashItemWithChildren(currentProjectId, currentTreeItemId)

                }
            }
        }
    }








}
