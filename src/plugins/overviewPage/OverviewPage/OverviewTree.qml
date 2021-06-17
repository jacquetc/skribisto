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
import "../../Commons"
import "../../Items"
import "../.."

OverviewTreeForm {
    id: root

    readonly property int currentTreeItemId: priv.currentTreeItemId
    readonly property int currentProjectId: priv.currentProjectId
    property int currentIndex: listView.currentIndex

    property alias visualModel: visualModel
    property var proxyModel
    required property var viewManager

    DelegateModel {
        id: visualModel

        delegate: dragDelegate
        model: proxyModel
    }
    listView.model: visualModel

    property int contextMenuItemIndex: -2
    onCurrentIndexChanged: {
        contextMenuItemIndex = listView.currentIndex
    }

    Binding {
        target: listView
        property: "currentIndex"
        value: proxyModel.forcedCurrentIndex
    }

    QtObject {
        id: priv
        property int currentProjectId: -2
        property int currentTreeItemId: -2
        property bool dragging: false
        property bool renaming: false
        property bool selecting: false
        property bool animationEnabled: SkrSettings.ePaperSettings.animationEnabled

        property var selectedTreeItemsIds: []
        property int selectedProjectId: -2

        property bool devModeEnabled: SkrSettings.devSettings.devModeEnabled
    }

    //-----------------------------------------------------------------------------
    // options :
    property bool treelikeIndentsVisible: true
    property bool dragDropEnabled: false // not yet completly implemented
    property int displayMode: SkrSettings.overviewTreeSettings.treeItemDisplayMode

    //tree-like onTreelikeIndents
    property int treeIndentOffset: 0
    property int treeIndentMultiplier: SkrSettings.overviewTreeSettings.treeIndentation

    // focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            listView.currentItem.forceActiveFocus()
        }
    }

    //-----------------------------------------------------------------------------

    // used to remember the source when moving an item
    property int moveSourceInt: -2

    // TreeView item :
    listView.delegate: Component {
        id: dragDelegate

        DropArea {
            id: delegateRoot

            property int treeItemId: model.treeItemId
            property int projectId: model.projectId
            property bool isOpenable: model.isOpenable
            property bool canAddChildTreeItem: model.canAddChildTreeItem
            property bool canAddSiblingTreeItem: model.canAddSiblingTreeItem
            property bool isCopyable: model.isCopyable
            property bool isMovable: model.isMovable
            property bool isRenamable: model.isRenamable
            property bool isTrashable: model.isTrashable

            Accessible.name: {

                var levelText
                if (treelikeIndentsVisible) {
                    levelText = qsTr("Level %1").arg(model.indent)
                }

                var titleText = titleLabel.text

                var labelText = ""
                if (labelLabel.text.length > 0) {
                    labelText = qsTr("label: %1").arg(labelLabel.text)
                }

                var hasChildrenText = ""
                if (model.hasChildren) {
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
                console.log("dropped : ", moveSourceInt,
                            draggableContent.visualIndex)
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
                leftMargin: treelikeIndentsVisible ? model.indent * root.treeIndentMultiplier
                                                     - root.treeIndentOffset
                                                     * root.treeIndentMultiplier : undefined
            }

            height: draggableContent.height

            onActiveFocusChanged: {
                if (listView.currentIndex === model.index) {
                    priv.currentTreeItemId = model.treeItemId
                }
            }

            function editName() {
                titleBox.state = "edit_name"
                titleTextFieldForceActiveFocusTimer.start()
                titleTextField.selectAll()
            }

            Timer {
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
            }

            Timer {
                id: labelTextFieldForceActiveFocusTimer
                repeat: false
                interval: 100
                onTriggered: {
                    labelTextField.forceActiveFocus()
                    labelTextField.selectAll()
                }
            }

            Keys.priority: Keys.AfterItem

            Keys.onShortcutOverride: {

                if ((event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_C) {
                    event.accepted = true
                }
                if ((event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_X) {
                    event.accepted = true
                }
                if ((event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_V) {
                    event.accepted = true
                }
                if ((event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_N) {
                    event.accepted = true
                }
                if (event.key === Qt.Key_Escape
                        && delegateRoot.state == "edit_name") {
                    event.accepted = true
                }
                if (event.key === Qt.Key_Escape && priv.renaming) {
                    event.accepted = true
                }
            }

            Keys.onPressed: {
                if (model.isOpenable && !priv.renaming
                        && event.key === Qt.Key_Return) {
                    console.log("Return key pressed")
                    openDocumentAction.trigger()
                    event.accepted = true
                }
                if (model.isOpenable && !priv.renaming
                        && (event.modifiers & Qt.AltModifier)
                        && event.key === Qt.Key_Return) {
                    console.log("Alt Return key pressed")
                    openDocumentInNewTabAction.trigger()
                    event.accepted = true
                }

                // rename
                if (model.isRenamable && !priv.renaming
                        && event.key === Qt.Key_F2
                        && delegateRoot.state !== "edit_name") {
                    renameAction.trigger()
                    event.accepted = true
                }

                // cut
                if (model.isMovable && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_X
                        && delegateRoot.state !== "edit_name"
                        && delegateRoot.state !== "edit_label") {
                    cutAction.trigger()
                    event.accepted = true
                }

                // copy
                if (model.isCopyable && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_C
                        && delegateRoot.state !== "edit_name"
                        && delegateRoot.state !== "edit_label") {
                    copyAction.trigger()
                    event.accepted = true
                }

                // paste
                if (model.canAddChildTreeItem && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_V
                        && delegateRoot.state !== "edit_name"
                        && delegateRoot.state !== "edit_label") {
                    pasteAction.trigger()
                    event.accepted = true
                }

                // add before
                if (model.canAddSiblingTreeItem && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && (event.modifiers & Qt.ShiftModifier)
                        && event.key === Qt.Key_N
                        && delegateRoot.state !== "edit_name"
                        && delegateRoot.state !== "edit_label") {
                    addBeforeAction.trigger()
                    event.accepted = true
                }

                // add after
                if (model.canAddSiblingTreeItem && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_N
                        && delegateRoot.state !== "edit_name"
                        && delegateRoot.state !== "edit_label") {
                    addAfterAction.trigger()
                    event.accepted = true
                }

                // add child
                if (model.canAddChildTreeItem && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_Space
                        && delegateRoot.state !== "edit_name"
                        && delegateRoot.state !== "edit_label") {
                    addChildAction.trigger()
                    event.accepted = true
                }

                // move up
                if (model.isMovable && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_Up
                        && delegateRoot.state !== "edit_name"
                        && delegateRoot.state !== "edit_label") {
                    moveUpAction.trigger()
                    event.accepted = true
                }

                // move down
                if (model.isMovable && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_Down
                        && delegateRoot.state !== "edit_name"
                        && delegateRoot.state !== "edit_label") {
                    moveDownAction.trigger()
                    event.accepted = true
                }

                // send to trash
                if (model.isTrashable && !priv.renaming
                        && event.key === Qt.Key_Delete
                        && delegateRoot.state !== "edit_name"
                        && delegateRoot.state !== "edit_label") {
                    sendToTrashAction.trigger()
                    event.accepted = true
                }

                if (event.key === Qt.Key_Escape) {
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
                width: delegateRoot.width - 2

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

                SkrListItemPane {
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
                            priv.currentTreeItemId = model.treeItemId
                            priv.currentProjectId = model.projectId
                            listView.currentIndex = model.index
                            delegateRoot.forceActiveFocus()
                            eventPoint.accepted = true
                        }

                        onDoubleTapped: {

                            //console.log("double tapped")
                            priv.currentTreeItemId = model.treeItemId
                            priv.currentProjectId = model.projectId
                            listView.currentIndex = model.index
                            openDocumentAction.trigger()
                            eventPoint.accepted = true
                        }

                        onLongPressed: {
                            // needed to activate the grab handler
                            priv.currentTreeItemId = model.treeItemId
                            priv.currentProjectId = model.projectId
                            if (root.dragDropEnabled) {
                                enabled = false
                            }
                        }

                        onGrabChanged: {
                            point.accepted = false
                        }
                    }

                    TapHandler {
                        id: rightClickHandler
                        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                        acceptedButtons: Qt.RightButton
                        onTapped: {

                            //console.log("right clicked")
                            if (loader_menu.active) {
                                if (loader_menu.item.visible) {
                                    loader_menu.item.close()
                                    return
                                }
                            }

                            priv.currentTreeItemId = model.treeItemId
                            priv.currentProjectId = model.projectId

                            // necessary to differenciate between all items
                            contextMenuItemIndex = model.index
                            listView.currentIndex = model.index

                            loader_menu.active = true
                            loader_menu.item.popup(content,
                                                   eventPoint.position.x,
                                                   eventPoint.position.y)
                            eventPoint.accepted = true
                        }
                    }
                    TapHandler {
                        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                        acceptedButtons: Qt.MiddleButton
                        onTapped: {
                            priv.currentTreeItemId = model.treeItemId
                            priv.currentProjectId = model.projectId
                            listView.currentIndex = model.index
                            delegateRoot.forceActiveFocus()
                            openDocumentInNewTabAction.trigger()
                            eventPoint.accepted = true
                        }
                    }

                    contentItem: RowLayout {
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
                                    color: listView.currentIndex
                                           === model.index ? "lightsteelblue" : "transparent"
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
                                        font.bold: model.projectIsActive
                                                   && model.indent === -1 ? true : false
                                        text: model.indent === 0 ? model.projectName : model.title
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
                                        Keys.onShortcutOverride: event.accepted
                                                                 = (event.key === Qt.Key_Escape)
                                        Keys.onPressed: {
                                            if (event.key === Qt.Key_Return) {
                                                console.log("Return key pressed title")
                                                editingFinished()
                                                event.accepted = true
                                            }
                                            if ((event.modifiers & Qt.CtrlModifier)
                                                    && event.key === Qt.Key_Return) {
                                                console.log("Ctrl Return key pressed title")
                                                editingFinished()
                                                event.accepted = true
                                            }
                                            if (event.key === Qt.Key_Escape) {
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
                                            if (model.indent === 0) {
                                                //project item
                                                model.projectName = text
                                            } else {
                                                model.title = text
                                            }

                                            titleBox.state = ""
                                        }

                                        //Keys.priority: Keys.AfterItem
                                        Keys.onShortcutOverride: event.accepted
                                                                 = (event.key === Qt.Key_Escape)
                                        Keys.onPressed: {
                                            if (event.key === Qt.Key_Return) {
                                                console.log("Return key pressed title")
                                                editingFinished()
                                                event.accepted = true
                                            }
                                            if ((event.modifiers & Qt.CtrlModifier)
                                                    && event.key === Qt.Key_Return) {
                                                console.log("Ctrl Return key pressed title")
                                                editingFinished()
                                                event.accepted = true
                                            }
                                            if (event.key === Qt.Key_Escape) {
                                                console.log("Escape key pressed title")
                                                titleBox.state = ""
                                                event.accepted = true
                                            }
                                        }
                                    }

                                    RowLayout {
                                        id: labelLayout
                                        Layout.fillWidth: true
                                        Layout.leftMargin: 5

                                        ListItemAttributes {
                                            id: attributes
                                            treeItemId: model.treeItemId
                                            projectId: model.projectId
                                            Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                            Layout.leftMargin: 4
                                            Layout.bottomMargin: 2
                                        }

                                        SkrLabel {
                                            id: labelLabel
                                            text: model.label === undefined ? "" : model.label
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

                                    color: model.indent === 0 ? Material.color(
                                                                    Material.Indigo) : (model.indent === 1 ? Material.color(Material.LightBlue) : (model.indent === 2 ? Material.color(Material.LightGreen) : (model.indent === 3 ? Material.color(Material.Amber) : (model.indent === 4 ? Material.color(Material.DeepOrange) : Material.color(Material.Teal)))))
                                }
                            }

                            states: [
                                State {
                                    name: "edit_name"
                                    PropertyChanges {
                                        target: priv
                                        renaming: true
                                    }
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
                                        target: priv
                                        renaming: true
                                    }
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
                            id: outlineBox
                            Layout.fillHeight: true
                            Layout.fillWidth: true
                            Layout.minimumWidth: 100

                            //Layout.maximumWidth: 600
                            property var writingZone: noteWritingZoneLoader.item

                            onWidthChanged: {
                                if (width === 50
                                        && Component.status === Component.Ready) {
                                    SkrSettings.overviewTreeSettings.outlineBoxVisible = false
                                }
                            }
                            visible: SkrSettings.overviewTreeSettings.outlineBoxVisible

                            RowLayout {
                                anchors.fill: parent

                                Rectangle {
                                    Layout.preferredWidth: 1
                                    Layout.preferredHeight: content.height / 2
                                    Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                                    gradient: Gradient {
                                        orientation: Qt.Vertical
                                        GradientStop {
                                            position: 0.00
                                            color: "transparent"
                                        }
                                        GradientStop {
                                            position: 0.30
                                            color: SkrTheme.divider
                                        }
                                        GradientStop {
                                            position: 0.70
                                            color: SkrTheme.divider
                                        }
                                        GradientStop {
                                            position: 1.00
                                            color: "transparent"
                                        }
                                    }
                                }

                                Component {
                                    id: noteWritingZoneComponent

                                    WritingZone {
                                        id: writingZone

                                        property string pageType: model.type
                                        clip: true
                                        projectId: model.projectId
                                        treeItemId: model.treeItemId
                                        spellCheckerKilled: true
                                        leftScrollItemVisible: false
                                        textArea.placeholderText: qsTr(
                                                                      "Outline")

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
                                            openOutline(model.projectId,
                                                        model.treeItemId)
                                        }

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

                                        //---------------------------------------------------------
                                        function openOutline(_projectId, _treeItemId) {
                                            // save current
                                            if (projectId !== _projectId
                                                    && treeItemId !== _treeItemId) {
                                                //meaning it hasn't just used the constructor
                                                clearNoteWritingZone()
                                            }

                                            documentPrivate.contentSaveTimerAllowedToStart = false
                                            documentPrivate.saveCurrentPaperCursorPositionAndYTimerAllowedToStart = false

                                            //                                            treeItemId = _treeItemId
                                            //                                            projectId = _projectId

                                            //console.log("opening note :", _projectId, _treeItemId)
                                            writingZone.setCursorPosition(0)
                                            writingZone.text = skrData.treeHub(
                                                        ).getSecondaryContent(
                                                        _projectId, _treeItemId)

                                            var uniqueDocumentReference = _projectId + "_"
                                                    + _treeItemId + "_secondary"
                                            skrTextBridge.subscribeTextDocument(
                                                        uniqueDocumentReference,
                                                        writingZone.textArea.objectName,
                                                        writingZone.textArea.textDocument)

                                            // apply format
                                            writingZone.documentHandler.indentEverywhere = SkrSettings.overviewTreeNoteSettings.textIndent
                                            writingZone.documentHandler.topMarginEverywhere = SkrSettings.overviewTreeNoteSettings.textTopMargin

                                            //restoreCurrentPaperCursorPositionAndY()

                                            //writingZone.forceActiveFocus()
                                            //save :
                                            skrUserSettings.setProjectSetting(
                                                        projectId,
                                                        "overViewTreeNoteCurrentTreeItemId",
                                                        treeItemId)

                                            // start the timer for automatic position saving
                                            documentPrivate.saveCurrentPaperCursorPositionAndYTimerAllowedToStart = true
                                            if (!saveCurrentPaperCursorPositionAndYTimer.running) {
                                                saveCurrentPaperCursorPositionAndYTimer.start()
                                            }
                                            documentPrivate.contentSaveTimerAllowedToStart = true

                                            determineModifiableTimer.start()
                                        }

                                        //---------------------------------------------------------
                                        // modifiable :
                                        property bool isModifiable: true

                                        Connections {
                                            target: skrData.treePropertyHub()
                                            function onPropertyChanged(_projectId, propertyId, _treeItemId, name, value) {
                                                if (_projectId === writingZone.projectId
                                                        && _treeItemId === writingZone.treeItemId) {

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

                                            isModifiable = skrData.treePropertyHub(
                                                        ).getProperty(
                                                        writingZone.projectId,
                                                        writingZone.treeItemId,
                                                        "modifiable",
                                                        "true") === "true"

                                            if (!isModifiable !== writingZone.textArea.readOnly) {
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
                                                        "overViewTreeNotePositionHash",
                                                        treeItemId, 0)
                                            //get Y
                                            var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                                                        projectId,
                                                        "overViewTreeNoteYHash",
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
                                                            "overViewTreeNotePositionHash",
                                                            treeItemId,
                                                            previousCursorPosition)
                                                skrUserSettings.insertInProjectSettingHash(
                                                            projectId,
                                                            "overViewTreeNoteYHash",
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
                                        Timer {
                                            id: contentSaveTimer
                                            repeat: false
                                            interval: 200
                                            onTriggered: saveContent()
                                        }

                                        function saveContent() {
                                            if (model.treeItemId === -2
                                                    || model.projectId === -2) {
                                                return
                                            }

                                            //console.log("saving note")
                                            var result = skrData.treeHub(
                                                        ).setSecondaryContent(
                                                        model.projectId,
                                                        model.treeItemId,
                                                        writingZone.text)
                                            if (!result.success) {
                                                console.log("saving note failed",
                                                            model.projectId,
                                                            model.treeItemId)
                                            } else {

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

                        //                        RowLayout{
                        //                            id: noteBox
                        //                            Layout.minimumWidth: 50
                        //                            Layout.maximumWidth: 400
                        //                            Layout.fillHeight: true
                        //                            Layout.fillWidth: true

                        //                            onWidthChanged: {
                        //                                if(width === 50 && Component.status === Component.Ready){
                        //                                    SkrSettings.overviewTreeSettings.noteBoxVisible = false
                        //                                }
                        //                            }
                        //                            visible: SkrSettings.overviewTreeSettings.noteBoxVisible

                        //                            Rectangle {
                        //                                Layout.preferredWidth: 1
                        //                                Layout.preferredHeight: content.height / 2
                        //                                Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                        //                                gradient: Gradient {
                        //                                    orientation: Qt.Vertical
                        //                                    GradientStop {
                        //                                        position: 0.00;
                        //                                        color: "transparent";
                        //                                    }
                        //                                    GradientStop {
                        //                                        position: 0.30;
                        //                                        color: SkrTheme.divider;
                        //                                    }
                        //                                    GradientStop {
                        //                                        position: 0.70;
                        //                                        color: SkrTheme.divider;
                        //                                    }
                        //                                    GradientStop {
                        //                                        position: 1.00;
                        //                                        color: "transparent";
                        //                                    }
                        //                                }

                        //                            }

                        //                            NotePad {
                        //                                id: notePad
                        //                                Layout.fillWidth: true
                        //                                Layout.fillHeight: true

                        //                                Layout.alignment: Qt.AlignVCenter

                        //                                minimalMode: true
                        //                                projectId: model.projectId
                        //                                sheetId: model.treeItemId
                        //                            }

                        //                        }

                        //-----------------------------------------------------------
                        //---------------Tags :---------------------------------------------
                        //-----------------------------------------------------------
                        RowLayout {
                            id: tagBox
                            Layout.minimumWidth: 50
                            Layout.maximumWidth: 400
                            Layout.fillHeight: true
                            Layout.fillWidth: true

                            onWidthChanged: {
                                if (width === 50
                                        && Component.status === Component.Ready) {
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
                                        position: 0.00
                                        color: "transparent"
                                    }
                                    GradientStop {
                                        position: 0.30
                                        color: SkrTheme.divider
                                    }
                                    GradientStop {
                                        position: 0.70
                                        color: SkrTheme.divider
                                    }
                                    GradientStop {
                                        position: 1.00
                                        color: "transparent"
                                    }
                                }
                            }

                            TagPad {
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
                        ColumnLayout {
                            id: countBox
                            visible: SkrSettings.overviewTreeSettings.characterCountBoxVisible
                                     || SkrSettings.overviewTreeSettings.wordCountBoxVisible
                            //Layout.minimumWidth: 50
                            //Layout.maximumWidth: 100
                            Layout.fillHeight: true
                            Layout.fillWidth: false

                            ColumnLayout {
                                id: characterCountLayout
                                visible: SkrSettings.overviewTreeSettings.characterCountBoxVisible
                                Layout.fillHeight: true
                                Layout.fillWidth: true

                                SkrLabel {
                                    id: characterCountLabel
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true
                                    Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                                    text: qsTr("c: %1").arg(
                                              skrRootItem.toLocaleIntString(
                                                  model.charCount))
                                    verticalAlignment: Qt.AlignVCenter
                                }
                                SkrLabel {
                                    id: characterCountWithChildrenLabel
                                    visible: model.hasChildren
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true
                                    Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                                    text: qsTr("all c: %1").arg(
                                              skrRootItem.toLocaleIntString(
                                                  model.charCountWithChildren))
                                    verticalAlignment: Qt.AlignVCenter
                                }
                            }

                            ColumnLayout {
                                id: wordCountLayout
                                visible: SkrSettings.overviewTreeSettings.wordCountBoxVisible
                                Layout.fillHeight: true
                                Layout.fillWidth: true

                                SkrLabel {
                                    id: wordCountLabel
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true
                                    Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                                    text: qsTr("w: %1").arg(
                                              skrRootItem.toLocaleIntString(
                                                  model.wordCount))
                                    verticalAlignment: Qt.AlignVCenter
                                }
                                SkrLabel {
                                    id: wordCountWithChildrenLabel
                                    visible: model.hasChildren
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true
                                    Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                                    text: qsTr("all w: %1").arg(
                                              skrRootItem.toLocaleIntString(
                                                  model.wordCountWithChildren))
                                    verticalAlignment: Qt.AlignVCenter
                                }
                            }
                        }

                        RowLayout {
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
                                        position: 0.00
                                        color: "transparent"
                                    }
                                    GradientStop {
                                        position: 0.30
                                        color: SkrTheme.divider
                                    }
                                    GradientStop {
                                        position: 0.70
                                        color: SkrTheme.divider
                                    }
                                    GradientStop {
                                        position: 1.00
                                        color: "transparent"
                                    }
                                }
                            }

                            ColumnLayout {
                                Layout.preferredWidth: 30

                                SkrToolButton {
                                    id: menuButton
                                    Layout.fillHeight: true
                                    Layout.preferredWidth: 30

                                    text: qsTr("Item menu")
                                    icon.source: "qrc:///icons/backup/overflow-menu.svg"
                                    focusPolicy: Qt.NoFocus

                                    onClicked: {
                                        priv.currentTreeItemId = model.treeItemId
                                        priv.currentProjectId = model.projectId
                                        contextMenuItemIndex = model.index
                                        listView.currentIndex = model.index
                                        delegateRoot.forceActiveFocus()

                                        if (loader_menu.active) {
                                            if (loader_menu.item.visible) {
                                                loader_menu.item.close()
                                                return
                                            }
                                        }

                                        loader_menu.active = true

                                        loader_menu.item.popup(
                                                    menuButton, menuButton.x,
                                                    menuButton.height)
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

                                        if (focusOnBranchButton.activeFocus) {

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
                        enabled: priv.animationEnabled
                        SequentialAnimation {
                            PropertyAnimation {
                                properties: "height"
                                duration: draggableContent.transitionAnimationDuration
                                easing.type: Easing.InOutQuad
                            }
                            ScriptAction {
                                script: {
                                    // shakes the writingZone to avoid blanks when resizing
                                    outlineBox.writingZone.flickable.contentY = 1
                                    outlineBox.writingZone.flickable.contentY = 0
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
                if (treeItemIdToEdit !== -2) {
                    editNameTimer.start()
                }
            }

            Timer {
                id: editNameTimer
                repeat: false
                interval: draggableContent.transitionAnimationDuration
                onTriggered: {
                    var index = proxyModel.findVisualIndex(model.projectId,
                                                           treeItemIdToEdit)
                    if (index !== -2) {
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
        enabled: priv.animationEnabled
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
        enabled: priv.animationEnabled
        SequentialAnimation {
            PauseAnimation {
                duration: 250
            }
            NumberAnimation {
                properties: "x,y"
                duration: 250
            }
        }
    }

    Component {
        id: component_menu
        SkrMenu {
            id: menu

            onOpened: {

            }

            onClosed: {
                loader_menu.active = false
            }
            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined
                action: openPaperAction
            }
            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined

                action: openPaperInNewTabAction
            }

            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined

                action: openPaperInNewWindowAction
            }

            MenuSeparator {}

            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined

                action: focusOnbranchAction
            }

            MenuSeparator {}

            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined

                action: renameAction
            }

            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined
                action: setLabelAction
            }

            MenuSeparator {}
            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined
                action: cutAction
            }

            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined
                action: copyAction
            }

            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined
                action: pasteAction
            }

            MenuSeparator {}

            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined
                action: addBeforeAction
            }

            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined
                action: addAfterAction
            }
            SkrMenuItem {

                height: !listView.currentItem.canAddChildTreeItem ? 0 : undefined
                visible: listView.currentItem.canAddChildTreeItem

                action: addChildAction
            }

            MenuSeparator {}
            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined
                action: moveUpAction
            }

            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined
                action: moveDownAction
            }
            MenuSeparator {}

            SkrMenuItem {
                visible: currentTreeItemId !== -1
                height: currentTreeItemId === -1 ? 0 : undefined
                action: sendToTrashAction
            }
        }
    }
    Loader {
        id: loader_menu
        sourceComponent: component_menu
        active: false
    }

    //-------------------------------------------------------------------------------------
    //------Actions------------------------------------------------------------------------
    //-------------------------------------------------------------------------------------
    Action {
        id: openPaperAction
        text: qsTr("Open")
        //shortcut: "Return"
        icon {
            source: "qrc:///icons/backup/document-edit.svg"
        }

        enabled: currentTreeItemId !== -1
        onTriggered: {
            if (!listView.currentItem.isOpenable) {
                return
            }

            console.log("open paper action", currentProjectId,
                        currentTreeItemId)

            viewManager.loadTreeItem(currentProjectId, currentTreeItemId)
        }
    }

    //-------------------------------------------------------------------------------------
    Action {
        id: openPaperInNewTabAction
        text: qsTr("Open in new tab")
        //shortcut: "Alt+Return"
        icon {
            source: "qrc:///icons/backup/tab-new.svg"
        }
        enabled: currentTreeItemId !== -1
        onTriggered: {
            if (!listView.currentItem.isOpenable) {
                return
            }
            console.log("open paper in new view action", currentProjectId,
                        currentTreeItemId)
            viewManager.loadTreeItemAtAnotherView(currentProjectId,
                                                  currentTreeItemId)
        }
    }
    //-------------------------------------------------------------------------------------
    Action {
        id: openPaperInNewWindowAction
        text: qsTr("Open in new window")
        //shortcut: "Alt+Return"
        icon {
            source: "qrc:///icons/backup/window-new.svg"
        }
        enabled: currentTreeItemId !== -1
        onTriggered: {
            if (!listView.currentItem.isOpenable) {
                return
            }
            console.log("open paper in new window action", currentProjectId,
                        currentTreeItemId)
            skrWindowManager.addWindowForItemId(currentProjectId,
                                                currentTreeItemId)
        }
    }

    //-------------------------------------------------------------------------------------
    Action {
        id: focusOnbranchAction
        text: Qt.isQtObject(
                  listView.itemAtIndex(
                      currentIndex)) ? (listView.itemAtIndex(
                                            currentIndex).focusOnBranchChecked ? qsTr("Unset focus") : qsTr("Set focus")) : ""
        //shortcut: "Alt+Return"
        icon {
            source: "qrc:///icons/backup/edit-find.svg"
        }
        enabled: currentTreeItemId !== -1
        onTriggered: {
            if (!listView.currentItem.canAddChildTreeItem) {
                return
            }
            //console.log("focus action", currentProjectId, currentTreeItemId)
            var checked = listView.itemAtIndex(
                        currentIndex).focusOnBranchChecked

            // filter to this parent and its children
            if (checked) {
                listView.itemAtIndex(currentIndex).focusOnBranchChecked = false
                proxyModel.showParentWhenParentIdFilter = false
                proxyModel.parentIdFilter = -2
            } else {
                listView.itemAtIndex(currentIndex).focusOnBranchChecked = true
                proxyModel.showParentWhenParentIdFilter = true
                proxyModel.parentIdFilter = currentTreeItemId
            }
        }
    }

    //-------------------------------------------------------------------------------------
    Action {

        id: renameAction
        text: qsTr("Rename")
        //shortcut: "F2"
        icon {
            source: "qrc:///icons/backup/edit-rename.svg"
        }
        enabled: listView.enabled

        onTriggered: {
            if (!listView.currentItem.isRenamable) {
                return
            }
            console.log("rename action", currentProjectId, currentTreeItemId)
            listView.itemAtIndex(currentIndex).editName()
        }
    }

    //-------------------------------------------------------------------------------------
    Action {

        id: setLabelAction
        text: qsTr("Set label")
        //shortcut: "F2"
        icon {
            source: "qrc:///icons/backup/label.svg"
        }
        enabled: currentTreeItemId !== -1
        onTriggered: {
            if (!listView.currentItem.isRenamable) {
                return
            }
            console.log("sel label", currentProjectId, currentTreeItemId)
            listView.itemAtIndex(currentIndex).editLabel()
        }
    }

    //-------------------------------------------------------------------------------------
    Action {
        id: cutAction
        text: qsTr("Cut")
        //shortcut: StandardKey.Cut
        icon {
            source: "qrc:///icons/backup/edit-cut.svg"
        }
        enabled: currentTreeItemId !== -1

        onTriggered: {
            if (!listView.currentItem.isMovable) {
                return
            }
            console.log("cut action", currentProjectId, currentTreeItemId)
            skrData.treeHub().cut(currentProjectId, [currentTreeItemId])
        }
    }
    //-------------------------------------------------------------------------------------
    Action {

        id: copyAction
        text: qsTr("Copy")
        //shortcut: StandardKey.Copy
        icon {
            source: "qrc:///icons/backup/edit-copy.svg"
        }
        enabled: currentTreeItemId !== -1

        onTriggered: {
            if (!listView.currentItem.isCopyable) {
                return
            }
            console.log("copy action", currentProjectId, currentTreeItemId)
            skrData.treeHub().copy(currentProjectId, [currentTreeItemId])
        }
    }

    //-------------------------------------------------------------------------------------
    Action {

        id: pasteAction
        text: qsTr("Paste")
        //shortcut: StandardKey.Copy
        icon {
            source: "qrc:///icons/backup/edit-paste.svg"
        }
        enabled: currentTreeItemId !== -1

        onTriggered: {
            if (!listView.currentItem.canAddChildTreeItem) {
                return
            }
            console.log("paste action", currentProjectId, currentTreeItemId)
            skrData.treeHub().paste(currentProjectId, currentTreeItemId)
        }
    }

    //-------------------------------------------------------------------------------------
    Action {
        id: addBeforeAction
        text: qsTr("Add before")
        //shortcut: "Ctrl+Shift+N"
        icon {
            source: "qrc:///icons/backup/document-new.svg"
        }
        enabled: currentTreeItemId !== -1
        onTriggered: {
            if (!listView.currentItem.canAddSiblingTreeItem) {
                return
            }
            console.log("add before action", currentProjectId,
                        currentTreeItemId)

            var visualIndex = listView.currentIndex

            newItemPopup.projectId = currentProjectId
            newItemPopup.treeItemId = currentTreeItemId
            newItemPopup.visualIndex = visualIndex
            newItemPopup.createFunction = afterNewItemTypeIsChosen
            newItemPopup.open()
        }
        function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType) {
            newItemPopup.close()
            var result = skrData.treeHub().addTreeItemAbove(projectId,
                                                            treeItemId,
                                                            pageType)

            if (result.success) {
                var newId = result.getData("treeItemId", -2)

                priv.currentTreeItemId = newId
                listView.currentIndex = proxyModel.findVisualIndex(projectId,
                                                                   newId)
                listView.currentItem.editName()
            }
        }
    }

    //-------------------------------------------------------------------------------------
    Action {

        id: addAfterAction
        text: qsTr("Add after")
        //shortcut: "Ctrl+N"
        icon {
            source: "qrc:///icons/backup/document-new.svg"
        }
        enabled: currentTreeItemId !== -1
        onTriggered: {
            if (!listView.currentItem.canAddSiblingTreeItem) {
                return
            }
            console.log("add after action", currentProjectId, currentTreeItemId)

            var visualIndex = listView.currentIndex + 1

            newItemPopup.projectId = currentProjectId
            newItemPopup.treeItemId = currentTreeItemId
            newItemPopup.visualIndex = visualIndex
            newItemPopup.createFunction = afterNewItemTypeIsChosen
            newItemPopup.open()
        }

        function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType) {
            newItemPopup.close()
            var result = skrData.treeHub().addTreeItemBelow(projectId,
                                                            treeItemId,
                                                            pageType)
            if (result.success) {
                var newId = result.getData("treeItemId", -2)

                priv.currentTreeItemId = newId
                listView.currentIndex = proxyModel.findVisualIndex(projectId,
                                                                   newId)
                listView.currentItem.editName()
            }
        }
    }

    //-------------------------------------------------------------------------------------
    Action {
        id: addChildAction
        text: qsTr("Add a sub-item")
        //shortcut: "Ctrl+N"
        icon {
            source: "qrc:///icons/backup/document-new.svg"
        }
        enabled: listView.enabled
        onTriggered: {
            if (!listView.currentItem.canAddChildTreeItem) {
                return
            }
            console.log("add child action", currentProjectId, currentTreeItemId)

            newItemPopup.projectId = currentProjectId
            newItemPopup.treeItemId = currentTreeItemId
            newItemPopup.visualIndex = 0
            newItemPopup.createFunction = afterNewItemTypeIsChosen
            newItemPopup.open()
        }

        function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType) {
            newItemPopup.close()

            var result = skrData.treeHub().addChildTreeItem(projectId,
                                                            treeItemId,
                                                            pageType)
            if (result.success) {
                var newId = result.getData("treeItemId", -2)

                priv.currentTreeItemId = newId
                listView.currentIndex = proxyModel.findVisualIndex(projectId,
                                                                   newId)

                listView.currentItem.editName()
            }
        }
    }

    //-------------------------------------------------------------------------------------
    Action {
        id: moveUpAction
        text: qsTr("Move up")
        //shortcut: "Ctrl+Up"
        icon {
            source: "qrc:///icons/backup/object-order-raise.svg"
        }
        enabled: currentIndex !== 0 && currentTreeItemId !== -1
        onTriggered: {
            if (!listView.currentItem.isMovable) {
                return
            }
            console.log("move up action", currentProjectId, currentTreeItemId)

            proxyModel.moveUp(currentProjectId, currentTreeItemId, currentIndex)
        }
    }

    //-------------------------------------------------------------------------------------
    Action {
        id: moveDownAction
        text: qsTr("Move down")
        //shortcut: "Ctrl+Down"
        icon {
            source: "qrc:///icons/backup/object-order-lower.svg"
        }
        enabled: currentIndex !== visualModel.items.count - 1
                 && currentTreeItemId !== -1

        onTriggered: {
            if (!listView.currentItem.isMovable) {
                return
            }
            console.log("move down action", currentProjectId, currentTreeItemId)

            proxyModel.moveDown(currentProjectId, currentTreeItemId,
                                currentIndex)
        }
    }
    //-------------------------------------------------------------------------------------
    Action {
        id: sendToTrashAction
        text: qsTr("Send to trash")
        //shortcut: "Del"
        icon {
            source: "qrc:///icons/backup/edit-delete.svg"
        }
        enabled: currentTreeItemId !== -1 && currentTreeItemId !== -1
        onTriggered: {
            if (!listView.currentItem.isTrashable) {
                return
            }
            console.log("sent to trash action", currentProjectId,
                        currentTreeItemId)
            skrData.treeHub().setTrashedWithChildren(currentProjectId,
                                                     currentTreeItemId, true)
        }
    }

    //-------------------------------------------------------------------------------------
    //-------------------------------------------------------------------------------------
    //-------------------------------------------------------------------------------------
    NewItemPopup {
        id: newItemPopup
    }
}
