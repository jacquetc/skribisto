import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQml.Models
import QtQml
import QtQuick.Controls.Material
import eu.skribisto.projecthub 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import eu.skribisto.taghub 1.0
import eu.skribisto.skr 1.0
import "../../../../Commons"
import "../../../../Items"
import "../../../.."

OverviewTreeForm {
    id: root

    readonly property int currentTreeItemId: priv.currentTreeItemId
    readonly property int currentProjectId: priv.currentProjectId
    readonly property bool dragging: priv.dragging
    property int currentIndex: listView.currentIndex

    property alias visualModel: visualModel
    property var proxyModel

    readonly property alias selectedTreeItemsIds: priv.selectedTreeItemsIds
    readonly property alias selectedProjectId: priv.selectedProjectId
    required property var viewManager

    DelegateModel {
        id: visualModel

        delegate: swipeDelegateComponent
        model: proxyModel
    }
    listView.model: visualModel

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

        property bool isOpenable
        property bool canAddChildTreeItem
        property bool canAddSiblingTreeItem
        property bool isCopyable
        property bool isMovable
        property bool isRenamable
        property bool isTrashable

    }

    //-----------------------------------------------------------------------------
    // options :
    property bool treelikeIndentsVisible: true
    property bool dragDropEnabled: false // not yet completly implemented
    property int displayMode: SkrSettings.overviewTreeSettings.treeItemDisplayMode

    //tree-like onTreelikeIndents
    property int treeIndentOffset: 1
    property int treeIndentMultiplier: SkrSettings.overviewTreeSettings.treeIndentation

    // focus :
    listView.onActiveFocusChanged: {
        if (activeFocus) {
            listView.currentItem.forceActiveFocus()
        }
    }

    //-----------------------------------------------------------------------------

    // used to remember the source when moving an item
    property int moveSourceInt: -2
    property int moveSourceTreeItemId: -2
    property int moveSourceProjectId: -2

    //-----------------------------------------------------------------------------



    function getIconUrlFromPageType(type, projectId, treeItemId) {
        return skrTreeManager.getIconUrlFromPageType(type, projectId, treeItemId)
    }


    //-----------------------------------------------------------------------------

    // TreeView item :
    listView.delegate: Component {
        id: swipeDelegateComponent

        SwipeDelegate {
            id: swipeDelegate
            property int indent: model.indent
            property alias dropArea: dropArea
            //property alias checkState: selectionCheckBox.checkState
            focus: true

            property int treeItemId: model.treeItemId
            property int projectId: model.projectId
            property bool isOpenable: model.isOpenable
            property bool canAddChildTreeItem: model.canAddChildTreeItem
            property bool canAddSiblingTreeItem: model.canAddSiblingTreeItem
            property bool isCopyable: model.isCopyable
            property bool isMovable: model.isMovable
            property bool isRenamable: model.isRenamable
            property bool isTrashable: model.isTrashable

            Material.background: SkrTheme.buttonBackground
            Material.foreground: SkrTheme.buttonForeground
            Material.accent: SkrTheme.accent

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

            height: draggableContent.height + 4

            //            padding: 0
            //            topPadding: 0
            //            bottomPadding: 0
            //            topInset: 0
            //            bottomInset: 0
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

            Keys.onShortcutOverride: function(event)  {

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
                        && swipeDelegate.state == "edit_name") {
                    event.accepted = true
                }
                if (event.key === Qt.Key_Escape && priv.renaming) {
                    event.accepted = true
                }
            }

            Keys.onPressed: function(event) {

                if (event.key === Qt.Key_Right) {
                    console.log("Right key pressed")
                    event.accepted = true
                }

                if ((event.modifiers ^ Qt.ControlModifier)
                        && event.key === Qt.Key_Up) {
                    console.log("Up key pressed")
                    listView.decrementCurrentIndex()
                    event.accepted = true
                }

                if ((event.modifiers ^ Qt.ControlModifier)
                        && event.key === Qt.Key_Down) {
                    console.log("Down key pressed")
                    listView.incrementCurrentIndex()
                    event.accepted = true
                }
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
                        && swipeDelegate.state !== "edit_name") {
                    renameAction.trigger()
                    event.accepted = true
                }

                // cut
                if (model.isMovable && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_X
                        && swipeDelegate.state !== "edit_name"
                        && swipeDelegate.state !== "edit_label") {
                    cutAction.trigger()
                    event.accepted = true
                }

                // copy
                if (model.isCopyable && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_C
                        && swipeDelegate.state !== "edit_name"
                        && swipeDelegate.state !== "edit_label") {
                    copyAction.trigger()
                    event.accepted = true
                }

                // paste
                if (model.canAddChildTreeItem && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_V
                        && swipeDelegate.state !== "edit_name"
                        && swipeDelegate.state !== "edit_label") {
                    pasteAction.trigger()
                    event.accepted = true
                }

                // add before
                if (model.canAddSiblingTreeItem && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && (event.modifiers & Qt.ShiftModifier)
                        && event.key === Qt.Key_N
                        && swipeDelegate.state !== "edit_name"
                        && swipeDelegate.state !== "edit_label") {
                    addBeforeAction.trigger()
                    event.accepted = true
                }

                // add after
                if (model.canAddSiblingTreeItem && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_N
                        && swipeDelegate.state !== "edit_name"
                        && swipeDelegate.state !== "edit_label") {
                    addAfterAction.trigger()
                    event.accepted = true
                }

                // add child
                if (model.canAddChildTreeItem && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_Space
                        && swipeDelegate.state !== "edit_name"
                        && swipeDelegate.state !== "edit_label") {
                    addChildAction.trigger()
                    event.accepted = true
                }

                // move up
                if (model.isMovable && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_Up
                        && swipeDelegate.state !== "edit_name"
                        && swipeDelegate.state !== "edit_label") {
                    moveUpAction.trigger()
                    event.accepted = true
                }

                // move down
                if (model.isMovable && !priv.renaming
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_Down
                        && swipeDelegate.state !== "edit_name"
                        && swipeDelegate.state !== "edit_label") {
                    moveDownAction.trigger()
                    event.accepted = true
                }

                // send to trash
                if (model.isTrashable && !priv.renaming
                        && event.key === Qt.Key_Delete
                        && swipeDelegate.state !== "edit_name"
                        && swipeDelegate.state !== "edit_label") {
                    sendToTrashAction.trigger()
                    event.accepted = true
                }

                if (event.key === Qt.Key_Escape) {
                    console.log("Escape key pressed")
                    event.accepted = true
                }
            }

            property bool focusOnBranchChecked: false

            contentItem: DropArea {
                id: dropArea

                keys: ["application/skribisto-overview-tree-item"]
                onEntered: {

                    console.log("entered")
                    console.log(drag.source.visualIndex,
                                draggableContent.visualIndex)
                    visualModel.items.move(drag.source.visualIndex,
                                           draggableContent.visualIndex)
                }

                onDropped: {
                    console.log("dropped")
                    if (drop.proposedAction === Qt.MoveAction) {
                        cancelDragTimer.stop()
                        listView.interactive = true
                        priv.dragging = false
                        console.log(moveSourceInt, draggableContent.visualIndex)
                        proxyModel.moveItem(moveSourceInt,
                                            draggableContent.visualIndex)
                    }
                }

                onExited: {
                    console.log('exited')
                }

                Rectangle {
                    id: draggableContent
                    property int visualIndex: model.index
                    property int sourceIndex: -2
                    property int projectId: model.projectId
                    property int treeItemId: model.treeItemId

                    property bool isCurrent: model.index === listView.currentIndex ? true : false

                    anchors {
                        horizontalCenter: parent.horizontalCenter
                        verticalCenter: parent.verticalCenter
                    }
                    width: parent.width

                    height: content.height

                    Drag.active: mouseDragHandler.active | touchDragHandler.active
                    Drag.source: draggableContent
                    Drag.hotSpot.x: width / 2
                    Drag.hotSpot.y: height / 2
                    Drag.keys: ["application/skribisto-overview-tree-item"]

                    Drag.supportedActions: Qt.MoveAction

                    opacity: mouseDragHandler.active | touchDragHandler.active ? 0.2 : 1.0

                    border.width: 2
                    border.color: mouseDragHandler.active
                                  | draggableContent.dragging ? SkrTheme.accent : "transparent"

                    Behavior on border.color {
                        enabled: SkrSettings.ePaperSettings.animationEnabled
                        ColorAnimation {
                            duration: 200
                        }
                    }

                    property bool dragging: false

                    Timer {
                        id: cancelDragTimer
                        repeat: false
                        interval: 3000
                        onTriggered: {
                            priv.dragging = false
                            draggableContent.dragging = false
                        }
                    }

                    /// without MouseArea, it breaks while dragging and scrolling:
                    MouseArea {
                        anchors.fill: parent
                        acceptedButtons: Qt.NoButton
                        onWheel: function(wheel) {
                            listView.interactive = false
                            listView.flick(0, wheel.angleDelta.y * 50)
                            wheel.accepted = true
                        }

                        enabled: listView.interactive === false
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
                        Item {
                            anchors.fill: parent
                            z: 1
                            HoverHandler {
                                id: hoverHandler
                            }
                        }

                        TapHandler {
                            id: tapHandler

                            onSingleTapped: function(eventPoint) {
                                priv.selecting = false

                                if (draggableContent.dragging) {
                                    eventPoint.accepted = false
                                    return
                                }
                                if (eventPoint.device.type === PointerDevice.Mouse) {
                                    listView.interactive = false
                                    Globals.touchUsed = false
                                }

                                if (eventPoint.device.type === PointerDevice.TouchScreen
                                        | eventPoint.device.type === PointerDevice.Stylus) {
                                    listView.interactive = true
                                    Globals.touchUsed = true
                                }

                                priv.currentTreeItemId = model.treeItemId
                                priv.currentProjectId = model.projectId
                                listView.currentIndex = model.index
                                swipeDelegate.forceActiveFocus()
                                eventPoint.accepted = true
                            }

                            onDoubleTapped: function(eventPoint){

                                if (draggableContent.dragging) {
                                    eventPoint.accepted = false
                                    return
                                }
                                if (eventPoint.device.type === PointerDevice.Mouse) {
                                    listView.interactive = false
                                    Globals.touchUsed = false
                                }

                                if (eventPoint.device.type === PointerDevice.TouchScreen
                                        | eventPoint.device.type === PointerDevice.Stylus) {
                                    listView.interactive = true
                                    Globals.touchUsed = true
                                }

                                //console.log("double tapped")
                                priv.currentTreeItemId = model.treeItemId
                                priv.currentProjectId = model.projectId
                                listView.currentIndex = model.index
                                openDocumentAction.trigger()
                                eventPoint.accepted = true
                            }

                            //                            onLongPressed: {
                            //                                // needed to activate the grab handler
                            //                                priv.currentTreeItemId = model.treeItemId
                            //                                priv.currentProjectId = model.projectId
                            //                                if (root.dragDropEnabled) {
                            //                                    enabled = false
                            //                                }
                            //                            }
                            onGrabChanged: function(point) {
                                point.accepted = false
                            }
                            grabPermissions: PointerHandler.TakeOverForbidden
                        }

                        TapHandler {
                            id: rightClickTapHandler
                            acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                            acceptedButtons: Qt.RightButton
                            onSingleTapped: function(eventPoint) {
                                listView.interactive = eventPoint.device.type
                                        === PointerDevice.Mouse

                                Globals.touchUsed = false
                                //console.log("right clicked")
                                if (menu.visible) {
                                    menu.close()
                                }




                                priv.currentTreeItemId = model.treeItemId
                                priv.currentProjectId = model.projectId
                                listView.currentIndex = model.index
                                priv.isOpenable = model.isOpenable
                                priv.canAddChildTreeItem = model.canAddChildTreeItem
                                priv.canAddSiblingTreeItem = model.canAddSiblingTreeItem
                                priv.isCopyable = model.isCopyable
                                priv.isMovable = model.isMovable
                                priv.isRenamable = model.isRenamable
                                priv.isTrashable = model.isTrashable




                                menu.popup(content,
                                           eventPoint.position.x,
                                           eventPoint.position.y)
                                eventPoint.accepted = true
                            }
                            grabPermissions: PointerHandler.TakeOverForbidden
                        }
                        TapHandler {
                            id: middleClickTapHandler
                            acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                            acceptedButtons: Qt.MiddleButton
                            onSingleTapped: function(eventPoint) {
                                Globals.touchUsed = false
                                listView.interactive = eventPoint.device.type
                                        === PointerDevice.Mouse
                                priv.currentTreeItemId = model.treeItemId
                                priv.currentProjectId = model.projectId
                                listView.currentIndex = model.index
                                swipeDelegate.forceActiveFocus()
                                openDocumentInAnotherViewAction.trigger()
                                eventPoint.accepted = true
                            }
                        }

                        /// without MouseArea, it breaks while dragging and scrolling:
                        MouseArea {
                            anchors.fill: parent
                            acceptedButtons: Qt.NoButton
                            onWheel: function(wheel) {
                                Globals.touchUsed = false
                                listView.interactive = false
                                listView.flick(0, wheel.angleDelta.y * 50)
                                wheel.accepted = true
                            }

                            enabled: listView.interactive === false
                        }

                        //---------------------------------------------------------
                        //------------------------------------------------
                        //---------------------------------------------------------
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

                                    SkrToolButton {
                                        id: treeItemIconIndicator
                                        //visible: model.projectIsBackup && model.treeItemId === -1
                                        enabled: true
                                        focusPolicy: Qt.NoFocus
                                        implicitHeight: 36 * SkrSettings.interfaceSettings.zoom
                                        implicitWidth: 36 * SkrSettings.interfaceSettings.zoom
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

                                            height: 36 * SkrSettings.interfaceSettings.zoom
                                            width: 36 * SkrSettings.interfaceSettings.zoom
                                            source: model.otherProperties ? getIconUrlFromPageType(
                                                                                model.type, model.projectId, model.treeItemId) : getIconUrlFromPageType(
                                                                                model.type, model.projectId, model.treeItemId)

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
                                            activeFocusOnTab: false
                                            font.bold: model.projectIsActive
                                                       && model.indent === -1 ? true : false
                                            text: model.indent
                                                  === 0 ? model.projectName : model.title
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
                                            Keys.onShortcutOverride: function(event)  {
                                                event.accepted = (event.key === Qt.Key_Escape)
                                            }
                                            Keys.onPressed: function(event) {
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
                                                    swipeDelegate.state = ""
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
                                            Keys.onShortcutOverride: function(event)  {
                                                event.accepted = (event.key === Qt.Key_Escape)
                                            }
                                            Keys.onPressed: function(event) {
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
                                                activeFocusOnTab: false
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
                                        id: coloredIndicator

                                        radius: 10
                                        Layout.fillHeight: true
                                        Layout.preferredWidth: moveHoverHandler.hovered ? 30 : 2
                                        Layout.alignment: Qt.AlignRight | Qt.AlignVCenter

                                        color: model.indent === 0 ? Material.color(
                                                                        Material.Indigo) : (model.indent === 1 ? Material.color(Material.LightBlue) : (model.indent === 2 ? Material.color(Material.LightGreen) : (model.indent === 3 ? Material.color(Material.Amber) : (model.indent === 4 ? Material.color(Material.DeepOrange) : Material.color(Material.Teal)))))

                                        //                                        MouseArea {
                                        //                                                anchors.fill: parent
                                        //                                                cursorShape: Qt.PointingHandCursor
                                        //                                                hoverEnabled: true
                                        //                                                acceptedButtons: Qt.NoButton

                                        ColumnLayout {
                                            anchors.fill: parent
                                            visible: moveHoverHandler.hovered | (Globals.touchUsed & draggableContent.isCurrent)

                                            Repeater {

                                                model: 8

                                                Rectangle {
                                                    Layout.preferredHeight: 1
                                                    Layout.preferredWidth: 15
                                                    Layout.alignment: Qt.AlignHCenter

                                                    color: SkrTheme.buttonBackground
                                                }
                                            }


                                        }
                                        //                                        }




                                        DragHandler {
                                            id: mouseDragHandler
                                            acceptedDevices: PointerDevice.Mouse
                                            target: draggableContent

                                            //xAxis.enabled: false
                                            //grabPermissions: PointerHandler.TakeOverForbidden
                                            onActiveChanged: {
                                                console.log("onActiveChanged",
                                                            active)
                                                if (active) {
                                                    listView.interactive = false
                                                    moveSourceInt = draggableContent.visualIndex
                                                    moveSourceTreeItemId
                                                            = draggableContent.treeItemId
                                                    moveSourceProjectId = draggableContent.projectId
                                                    priv.dragging = true
                                                    cancelDragTimer.stop()
                                                } else {
                                                    cancelDragTimer.stop()
                                                    priv.dragging = false
                                                    draggableContent.dragging = false
                                                    draggableContent.Drag.drop()

                                                    proxyModel.invalidate()
                                                }
                                            }
                                            enabled: false

                                            onCanceled: {
                                                console.log("onCanceled")
                                                cancelDragTimer.stop()
                                                priv.dragging = false
                                                draggableContent.dragging = false
                                            }

                                            grabPermissions: PointerHandler.CanTakeOverFromAnything
                                        }

                                        DragHandler {
                                            id: touchDragHandler
                                            acceptedDevices: PointerDevice.TouchScreen
                                                             | PointerDevice.Stylus
                                            target: draggableContent

                                            //xAxis.enabled: false
                                            //grabPermissions: PointerHandler.TakeOverForbidden
                                            onActiveChanged: {
                                                if (active) {
                                                    listView.interactive = false
                                                    moveSourceInt = draggableContent.visualIndex
                                                    moveSourceTreeItemId
                                                            = draggableContent.treeItemId
                                                    moveSourceProjectId = draggableContent.projectId
                                                    priv.dragging = true
                                                    cancelDragTimer.stop()
                                                } else {
                                                    listView.interactive = true
                                                    cancelDragTimer.stop()
                                                    priv.dragging = false
                                                    draggableContent.dragging = false
                                                    draggableContent.Drag.drop()
                                                    proxyModel.invalidate()
                                                }
                                            }
                                            enabled: false //draggableContent.dragging

                                            onCanceled: {
                                                cancelDragTimer.stop()
                                                priv.dragging = false
                                                draggableContent.dragging = false
                                            }
                                            grabPermissions: PointerHandler.CanTakeOverFromItems | PointerHandler.CanTakeOverFromAnything
                                        }

                                        TapHandler {
                                            acceptedDevices: PointerDevice.TouchScreen
                                                             | PointerDevice.Stylus

                                            onLongPressed: {

                                                // needed to activate the grab handler

                                                //                        if(draggableContent.dragging){
                                                //                            eventPoint.accepted = false
                                                //                            return
                                                //                        }
                                                draggableContent.dragging = true
                                                listView.interactive = false
                                                cancelDragTimer.start()
                                                priv.selecting = false
                                            }
                                        }
                                    }

                                    HoverHandler {
                                        id: moveHoverHandler


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
                                            orientation: Gradient.Vertical
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

                                        OutlineWritingZone {

                                            id: writingZone

                                            property string pageType: model.type

                                            clip: true
                                            projectId: model.projectId
                                            treeItemId: model.treeItemId
                                            spellCheckerKilled: true
                                            leftScrollItemVisible: false
                                            rightScrollItemVisible: Globals.touchUsed
                                            placeholderText: qsTr("Outline")

                                            textPointSize: SkrSettings.overviewTreeOutlineSettings.textPointSize
                                            textFontFamily: SkrSettings.overviewTreeOutlineSettings.textFontFamily
                                            textIndent: SkrSettings.overviewTreeOutlineSettings.textIndent
                                            textTopMargin: SkrSettings.overviewTreeOutlineSettings.textTopMargin

                                            stretch: true

                                            textAreaStyleBackgroundColor: SkrTheme.secondaryTextAreaBackground
                                            textAreaStyleForegroundColor: SkrTheme.secondaryTextAreaForeground
                                            paneStyleBackgroundColor: SkrTheme.listItemBackground
                                            textAreaStyleAccentColor: SkrTheme.accent
                                            name: "overviewOutline"

                                            //-----Zoom------------------------------------------------------------

                                            Shortcut {
                                                sequences: skrShortcutManager.shortcuts("plugin-overview-outline-text-zoom-in")
                                                context: Qt.WindowShortcut
                                                enabled: writingZone.activeFocus
                                                onActivated: {SkrSettings.overviewTreeOutlineSettings.textPointSize += 1}
                                            }

                                            Shortcut {
                                                sequences: skrShortcutManager.shortcuts("plugin-overview-outline-text-zoom-out")
                                                context: Qt.WindowShortcut
                                                enabled: writingZone.activeFocus
                                                onActivated: {SkrSettings.overviewTreeOutlineSettings.textPointSize -= 1}
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
                                        orientation: Gradient.Vertical
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
                                visible: SkrSettings.interfaceSettings.characterCountVisible
                                         || SkrSettings.interfaceSettings.wordCountVisible
                                //Layout.minimumWidth: 50
                                //Layout.maximumWidth: 100
                                Layout.fillHeight: true
                                Layout.fillWidth: false

                                ColumnLayout {
                                    id: characterCountLayout
                                    visible: SkrSettings.interfaceSettings.charCountVisible
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true

                                    SkrLabel {
                                        id: characterCountLabel
                                        visible: !model.hasChildren
                                        Layout.fillHeight: true
                                        Layout.fillWidth: true
                                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                                        text: model.charCountGoal > 0 ? qsTr("c: %1 / %2").arg(
                                                                           skrRootItem.toLocaleIntString(
                                                                               model.charCount)).arg(
                                                                           skrRootItem.toLocaleIntString(
                                                                               model.charCountGoal)) : qsTr("c: %1").arg(
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
                                        text: model.charCountGoal > 0 ? qsTr("all c: %1 / %2").arg(
                                                                           skrRootItem.toLocaleIntString(
                                                                               model.charCountWithChildren)).arg(
                                                                           skrRootItem.toLocaleIntString(
                                                                               model.charCountGoal)) : qsTr("all c: %1").arg(
                                                                           skrRootItem.toLocaleIntString(
                                                                               model.charCountWithChildren))
                                        verticalAlignment: Qt.AlignVCenter
                                    }
                                }

                                ColumnLayout {
                                    id: wordCountLayout
                                    visible: SkrSettings.interfaceSettings.wordCountVisible
                                    Layout.fillHeight: true
                                    Layout.fillWidth: true

                                    SkrLabel {
                                        id: wordCountLabel
                                        visible: !model.hasChildren
                                        Layout.fillHeight: true
                                        Layout.fillWidth: true
                                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                                        text: model.wordCountGoal > 0 ? qsTr("w: %1 / %2").arg(
                                                                           skrRootItem.toLocaleIntString(
                                                                               model.wordCount)).arg(
                                                                           skrRootItem.toLocaleIntString(
                                                                               model.wordCountGoal)) : qsTr("w: %1").arg(
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
                                        text: model.wordCountGoal > 0 ? qsTr("all w: %1 / %2").arg(
                                                                           skrRootItem.toLocaleIntString(
                                                                               model.wordCountWithChildren)).arg(
                                                                           skrRootItem.toLocaleIntString(
                                                                               model.wordCountGoal)) : qsTr("all w: %1").arg(
                                                                           skrRootItem.toLocaleIntString(
                                                                               model.wordCountWithChildren))
                                        verticalAlignment: Qt.AlignVCenter
                                    }
                                }
                            }

                            //-----------------------------------------------------------
                            //---------------Counts :---------------------------------------------
                            //-----------------------------------------------------------

                            RowLayout {
                                id: buttonsBox
                                Layout.preferredWidth: 40
                                visible: hoverHandler.hovered | draggableContent.isCurrent

                                Rectangle {
                                    Layout.preferredWidth: 1
                                    Layout.preferredHeight: content.height / 2
                                    Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                                    gradient: Gradient {
                                        orientation: Gradient.Vertical
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
                                            if (menu.visible) {
                                                menu.close()


                                            }


                                            swipeDelegate.forceActiveFocus()


                                            priv.currentTreeItemId = model.treeItemId
                                            priv.currentProjectId = model.projectId
                                            listView.currentIndex = model.index
                                            priv.isOpenable = model.isOpenable
                                            priv.canAddChildTreeItem = model.canAddChildTreeItem
                                            priv.canAddSiblingTreeItem = model.canAddSiblingTreeItem
                                            priv.isCopyable = model.isCopyable
                                            priv.isMovable = model.isMovable
                                            priv.isRenamable = model.isRenamable
                                            priv.isTrashable = model.isTrashable

                                            menu.popup(
                                                        menuButton,
                                                        menuButton.x,
                                                        menuButton.height)


                                        }


                                        //'visible: hoverHandler.hovered | draggableContent.isCurrent
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
                                        checked: swipeDelegate.focusOnBranchChecked

                                        onCheckedChanged: {
                                            priv.currentTreeItemId = model.treeItemId
                                            priv.currentProjectId = model.projectId

                                            listView.currentIndex = model.index
                                            swipeDelegate.forceActiveFocus()

                                            focusOnbranchAction.trigger()

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
                                        if(noteWritingZoneLoader.status === Loader.Ready){
                                            outlineBox.writingZone.writingZone.flickable.contentY = 1
                                            outlineBox.writingZone.writingZone.flickable.contentY = 0
                                        }



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
                            parent: Overlay.overlay
                        }
                        AnchorChanges {
                            target: draggableContent
                            anchors {
                                horizontalCenter: undefined
                                verticalCenter: undefined
                            }
                        }

                        PropertyChanges {
                            target: swipeDelegate
                            z: 2
                        }
                    },

                    State {
                        name: "unset_anchors"
                        AnchorChanges {
                            target: swipeDelegate
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
                        target: swipeDelegate
                        property: "ListView.delayRemove"
                        value: true
                    }
                    NumberAnimation {
                        target: swipeDelegate
                        property: "height"
                        to: 0
                        duration: 250
                        easing.type: Easing.InOutQuad
                    }
                    PropertyAction {
                        target: swipeDelegate
                        property: "ListView.delayRemove"
                        value: false
                    }
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

    listView.addDisplaced: Transition {
        enabled: SkrSettings.ePaperSettings.animationEnabled
        NumberAnimation {
            properties: "x,y"
            duration: 250
        }
    }

    listView.displaced: Transition {
        enabled: SkrSettings.ePaperSettings.animationEnabled
        NumberAnimation {
            properties: "x,y"
            duration: 250
        }
    }

    listView.moveDisplaced: Transition {
        enabled: SkrSettings.ePaperSettings.animationEnabled
        NumberAnimation {
            properties: "x,y"
            duration: 100
        }
    }

    //    Component {
    //        id: component_menu
    SkrMenu {
        id: menu

        property int treeItemId: priv.currentTreeItemId
        property int projectId: priv.currentProjectId
        property bool isOpenable: priv.isOpenable
        property bool canAddChildTreeItem: priv.canAddChildTreeItem
        property bool canAddSiblingTreeItem: priv.canAddSiblingTreeItem
        property bool isCopyable: priv.isCopyable
        property bool isMovable: priv.isMovable
        property bool isRenamable: priv.isRenamable
        property bool isTrashable: priv.isTrashable

        onOpened: {

        }

        onClosed: {
            //loader_menu.active = false
        }
        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.isOpenable
            height: currentTreeItemId !== -1
                    && menu.isOpenable ? undefined : 0
            action: openDocumentAction
        }
        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.isOpenable
            height: currentTreeItemId !== -1
                    && menu.isOpenable ? undefined : 0

            action: openDocumentInAnotherViewAction
        }

        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.isOpenable
            height: currentTreeItemId !== -1
                    && menu.isOpenable ? undefined : 0

            action: openPaperInNewWindowAction
        }

        MenuSeparator {
            visible: menu.canAddChildTreeItem
            height: menu.canAddChildTreeItem ? undefined : 0
        }

        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.canAddChildTreeItem
            height: currentTreeItemId !== -1
                    && menu.canAddChildTreeItem ? undefined : 0

            action: focusOnbranchAction
        }

        MenuSeparator {}

        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.isRenamable
            height: currentTreeItemId !== -1
                    && menu.isRenamable ? undefined : 0

            action: renameAction
        }

        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.isRenamable
            height: currentTreeItemId !== -1
                    && menu.isRenamable ? undefined : 0
            action: setLabelAction
        }

        SkrMenuItem {
            visible: currentTreeItemId !== -1
            height: currentTreeItemId !== -1 ? undefined : 0
            action: setGoalAction
        }

        MenuSeparator {}
        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.isMovable
            height: currentTreeItemId !== -1
                    && menu.isMovable ? undefined : 0
            action: cutAction
        }

        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.isCopyable
            height: currentTreeItemId !== -1
                    && menu.isCopyable ? undefined : 0
            action: copyAction
        }

        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.canAddChildTreeItem
            height: currentTreeItemId !== -1
                    && menu.canAddChildTreeItem ? undefined : 0
            action: pasteAction
        }

        MenuSeparator {}

        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.canAddSiblingTreeItem
            height: currentTreeItemId !== -1
                    && menu.canAddSiblingTreeItem ? undefined : 0
            action: addBeforeAction
        }

        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.canAddSiblingTreeItem
            height: currentTreeItemId !== -1
                    && menu.canAddSiblingTreeItem ? undefined : 0
            action: addAfterAction
        }
        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.canAddChildTreeItem
            height: currentTreeItemId !== -1
                    && menu.canAddChildTreeItem ? undefined : 0

            action: addChildAction
        }

        MenuSeparator {}
        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.isMovable
            height: currentTreeItemId !== -1
                    && menu.isMovable ? undefined : 0
            action: moveUpAction
        }

        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.isMovable
            height: currentTreeItemId !== -1
                    && menu.isMovable ? undefined : 0
            action: moveDownAction
        }
        MenuSeparator {}

        SkrMenuItem {
            visible: currentTreeItemId !== -1
                     && menu.isTrashable
            height: currentTreeItemId !== -1
                    && menu.isTrashable ? undefined : 0
            action: sendToTrashAction
        }
    }
    //    }
    //    Loader {
    //        id: loader_menu
    //        sourceComponent: component_menu
    //        active: false
    //    }

    //-------------------------------------------------------------------------------------
    //------Actions------------------------------------------------------------------------
    //-------------------------------------------------------------------------------------
    Action {
        id: openDocumentAction
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
        id: openDocumentInAnotherViewAction
        text: qsTr("Open in another view")
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

        id: setGoalAction
        text: qsTr("Set goal")
        //shortcut: "F2"
        icon {
            source: "qrc:///icons/backup/label.svg"
        }
        enabled: currentTreeItemId !== -1
        onTriggered: {

            itemWordGoalDialog.projectId = currentProjectId
            itemWordGoalDialog.treeItemId = currentTreeItemId
            itemWordGoalDialog.open()
            console.log("set goal", currentProjectId, currentTreeItemId)


        }
    }
    ItemWordGoalDialog{
        id: itemWordGoalDialog
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
        function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType, quantity) {
            newItemPopup.close()
            var result

            for(var i = 0; i < quantity ; i++){
                result = skrData.treeHub().addTreeItemAbove(projectId,
                                                            treeItemId,
                                                            pageType)
            }

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

        function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType, quantity) {
            newItemPopup.close()

            var result
            for(var i = 0; i < quantity ; i++){
                result = skrData.treeHub().addTreeItemBelow(projectId,
                                                            treeItemId,
                                                            pageType)
            }

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

        function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType, quantity) {
            newItemPopup.close()

            var result
            for(var i = 0; i < quantity ; i++){
                result = skrData.treeHub().addChildTreeItem(projectId,
                                                            treeItemId,
                                                            pageType)
            }

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
