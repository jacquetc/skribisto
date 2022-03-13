import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQml.Models
import QtQml
import QtQuick.Controls.Material
import eu.skribisto.projecthub 1.0
import eu.skribisto.result 1.0
import "../Items"
import ".."

ListView {
    id: root

    signal openDocument(int openedProjectId, int openedTreeItemId, int projectId, int treeItemId)
    signal openDocumentInAnotherView(int projectId, int treeItemId)
    signal openDocumentInNewWindow(int projectId, int treeItemId)

    signal copyCalled(int projectId, int treeItemId)
    signal deleteDefinitivelyCalled(int projectId, int treeItemId)
    //signal sendToTrashCalled(int projectId, int treeItemId)
    signal escapeKeyPressed

    property int currentTreeItemId: -2
    property int currentProjectId: -2
    property int openedProjectId: -2
    property int openedTreeItemId: -2
    property bool hoveringChangingTheCurrentItemAllowed: true

    property alias visualModel: visualModel
    property var proxyModel

    DelegateModel {
        id: visualModel
        model: proxyModel
        delegate: dragDelegate
    }
    model: visualModel

    property int contextMenuItemIndex: -2
    onCurrentIndexChanged: {
        contextMenuItemIndex = root.currentIndex
    }

    // options :
    property bool treelikeIndentsVisible: false
    property bool checkButtonsVisible: false
    property bool openActionsEnabled: false
    property bool renameActionEnabled: false
    property bool sendToTrashActionEnabled: false
    property bool deleteActionEnabled: false
    property bool cutActionEnabled: false
    property bool copyActionEnabled: false
    property bool pasteActionEnabled: false
    property bool addSiblingTreeItemActionEnabled: false
    property bool addChildTreeItemActionEnabled: false
    property bool moveActionEnabled: false
    property bool elevationEnabled: false

    //tree-like onTreelikeIndents
    property int treeIndentOffset: 0
    property int treeIndentMultiplier: 10

    spacing: elevationEnabled ? 5 : 0
    leftMargin: elevationEnabled ? 5 : 0

    // checkButtons :
    function getCheckedTreeItemIdList() {
        return proxyModel.getCheckedIdsList()
    }

    function setCheckedTreeItemIdList(checkedTreeItemIdList) {
        proxyModel.setCheckedIdsList(checkedTreeItemIdList)
    }

    // TreeView item :
    delegate: Component {
        id: dragDelegate

        Item {
            id: delegateRoot

            Accessible.name: {

                var levelText
                if (treelikeIndentsVisible) {
                    levelText = qsTr("Level %1").arg(model.indent)
                }

                var titleText = titleLabel.text

                var checkedText
                if (checkButtonsVisible) {
                    checkedText = model.checkState === Qt.PartiallyChecked ? qsTr("partially checked") : model.checkState === Qt.Checked ? qsTr("checked") : model.checkState === Qt.Unchecked ? qsTr("unchecked") : ""
                }

                var labelText = ""
                if (labelLabel.text.length > 0) {
                    labelText = qsTr("label: %1").arg(labelLabel.text)
                }

                var hasChildrenText = ""
                if (model.hasChildren) {
                    hasChildrenText = qsTr("has children")
                }

                return levelText + " " + titleText + " " + checkedText + " "
                        + labelText + " " + hasChildrenText
            }
            Accessible.role: Accessible.ListItem
            Accessible.description: qsTr("navigation item")

            property int visualIndex: {
                return DelegateModel.itemsIndex
            }

            Binding {
                target: content
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

            height: content.height

            onActiveFocusChanged: {
                if (root.currentIndex === model.index && model.index !== -1
                        && activeFocus) {
                    root.currentTreeItemId = model.treeItemId
                }
            }

            //            drag.target: held ? content : undefined
            //            drag.axis: Drag.YAxis

            //            onPressAndHold: held = true
            //            onReleased: held = false
            //            Shortcut {
            //                sequence: "Ctrl+Up"
            //                onActivated: moveUpAction.trigger(delegateRoot)
            //            }
            //            Keys.onShortcutOverride: function(event)  {
            //                if (event.key === Qt.Key_Backspace) {
            //                    console.log("onShortcutOverride")
            //                    event.accepted = true
            //                }
            //            }
            //            Keys.onBackPressed: {
            //                console.log("eee")
            //            }
            function editName() {
                state = "edit_title"
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
                state = "edit_label"
                labelTextFieldForceActiveFocusTimer.start()
                labelTextField.selectAll()
            }

            Timer {
                id: labelTextFieldForceActiveFocusTimer
                repeat: false
                interval: 100
                onTriggered: {
                    labelTextField.forceActiveFocus()
                }
            }

            Keys.priority: Keys.AfterItem

            Keys.onShortcutOverride: function(event)  {
                if ((event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_N) {
                    event.accepted = true
                }
                if ((event.modifiers & Qt.ControlModifier)
                        && (event.modifiers & Qt.ShiftModifier)
                        && event.key === Qt.Key_N) {
                    event.accepted = true
                }
                if (copyActionEnabled && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_C) {
                    event.accepted = true
                }
                if (cutActionEnabled && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_X) {
                    event.accepted = true
                }
                if (pasteActionEnabled && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_V) {
                    event.accepted = true
                }
                if (renameActionEnabled && event.key === Qt.Key_Escape
                        && (delegateRoot.state == "edit_title"
                            || delegateRoot.state == "edit_label")) {
                    event.accepted = true
                }
                if (event.key === Qt.Key_Escape) {
                    event.accepted = true
                }
            }

            Keys.onPressed: function(event) {
                // avoid unwanted overshoot when taping right key
                if (event.key === Qt.Key_Right) {
                    event.accepted = true
                }
                if (model.isOpenable && openActionsEnabled
                        && event.key === Qt.Key_Return) {
                    console.log("Return key pressed")
                    openDocumentAction.trigger()
                    event.accepted = true
                }
                if (model.isOpenable && openActionsEnabled
                        && (event.modifiers & Qt.AltModifier)
                        && event.key === Qt.Key_Return) {
                    console.log("Alt Return key pressed")
                    openDocumentInAnotherViewAction.trigger()
                    event.accepted = true
                }

                // checked :
                if (checkButtonsVisible) {
                    if (event.key === Qt.Key_Space) {
                        console.log("Space pressed")
                        selectionCheckBox.toggle()
                        event.accepted = true
                    }
                }

                // rename
                if (model.isRenamable && renameActionEnabled
                        && event.key === Qt.Key_F2
                        && delegateRoot.state !== "edit_title"
                        && delegateRoot.state !== "edit_label") {
                    renameAction.trigger()
                    event.accepted = true
                }

                // cut
                if (model.isMovable && cutActionEnabled
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_X
                        && delegateRoot.state !== "edit_title"
                        && delegateRoot.state !== "edit_label") {
                    cutAction.trigger()
                    event.accepted = true
                }

                // copy
                if (copyActionEnabled && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_C
                        && delegateRoot.state !== "edit_title"
                        && delegateRoot.state !== "edit_label") {
                    copyAction.trigger()
                    event.accepted = true
                }

                // paste
                if (model.canAddChildTreeItem && addChildTreeItemActionEnabled
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_V
                        && delegateRoot.state !== "edit_title"
                        && delegateRoot.state !== "edit_label") {
                    pasteAction.trigger()
                    event.accepted = true
                }

                // add before
                if (model.canAddSiblingTreeItem
                        && addSiblingTreeItemActionEnabled
                        && (event.modifiers & Qt.ControlModifier)
                        && (event.modifiers & Qt.ShiftModifier)
                        && event.key === Qt.Key_N
                        && delegateRoot.state !== "edit_title"
                        && delegateRoot.state !== "edit_label") {
                    addBeforeAction.trigger()
                    event.accepted = true
                }

                // add after
                if (model.canAddSiblingTreeItem
                        && addSiblingTreeItemActionEnabled
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_N
                        && delegateRoot.state !== "edit_title"
                        && delegateRoot.state !== "edit_label") {
                    addAfterAction.trigger()
                    event.accepted = true
                }

                // add child
                if (model.canAddChildTreeItem && addChildTreeItemActionEnabled
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_Space
                        && delegateRoot.state !== "edit_title"
                        && delegateRoot.state !== "edit_label") {
                    addChildAction.trigger()
                    event.accepted = true
                }

                // move up
                if (model.isMovable && moveActionEnabled
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_Up
                        && delegateRoot.state !== "edit_title"
                        && delegateRoot.state !== "edit_label") {
                    moveUpAction.trigger()
                    event.accepted = true
                }

                // move down
                if (model.isMovable && moveActionEnabled
                        && (event.modifiers & Qt.ControlModifier)
                        && event.key === Qt.Key_Down
                        && delegateRoot.state !== "edit_title"
                        && delegateRoot.state !== "edit_label") {
                    moveDownAction.trigger()
                    event.accepted = true
                }

                // send to trash
                if (model.isTrashable && sendToTrashActionEnabled
                        && event.key === Qt.Key_Delete
                        && delegateRoot.state !== "edit_title"
                        && delegateRoot.state !== "edit_label") {
                    sendToTrashAction.trigger()
                    event.accepted = true
                }

                if (event.key === Qt.Key_Escape) {
                    console.log("Escape key pressed")
                    root.escapeKeyPressed()
                    event.accepted = true
                }
            }

            property bool editBugWorkaround: false

            SkrListItemPane {
                id: content
                property int visualIndex: 0
                property int sourceIndex: -2

                property bool isCurrent: model.index === root.currentIndex ? true : false

                anchors {
                    horizontalCenter: parent.horizontalCenter
                    verticalCenter: parent.verticalCenter
                }
                width: delegateRoot.width
                height: 40

                padding: 1

                elevation: root.elevationEnabled ? 4 : 0

                HoverHandler {
                    id: hoverHandler
                    //                    onHoveredChanged: {
                    //                        if (hoverHandler.hovered & hoveringChangingTheCurrentItemAllowed) {
                    //                            root.currentIndex = model.index
                    //                        }
                    //                    }
                }

                TapHandler {
                    id: tapHandler

                    onSingleTapped: function(eventPoint) {
                        root.currentIndex = model.index
                        delegateRoot.forceActiveFocus()
                        eventPoint.accepted = true
                    }

                    onDoubleTapped: function(eventPoint) {
                        console.log("double tapped")
                        root.currentIndex = model.index
                        openDocumentAction.trigger()
                        eventPoint.accepted = true
                    }

                    onGrabChanged: function(transition, point) {
                        point.accepted = false
                    }
                }

                TapHandler {
                    acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                    acceptedButtons: Qt.RightButton
                    onSingleTapped: function(eventPoint) {

                        root.currentIndex = model.index

                        if (menu.visible) {
                            menu.close()
                            return
                        }

                        menu.open()
                        eventPoint.accepted = true
                    }
                }
                TapHandler {
                    acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                    acceptedButtons: Qt.MiddleButton
                    onSingleTapped: function(eventPoint) {
                        root.currentIndex = model.index
                        openDocumentInAnotherViewAction.trigger()
                        eventPoint.accepted = true
                    }
                }

                Action {
                    id: openDocumentAction
                    //shortcut: "Return"
                    enabled: {
                        if (root.focus === true
                                && titleTextField.visible === false
                                && root.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: "Open document"
                    property string shortcutText: ""
                    onTriggered: {
                        //console.log("model.openedProjectId", openedProjectId)
                        //console.log("model.projectId", model.projectId)
                        root.openDocument(openedProjectId, openedTreeItemId,
                                          model.projectId, model.treeItemId)
                    }
                }

                Action {
                    id: openDocumentInAnotherViewAction
                    //shortcut: "Alt+Return"
                    property string shortcutText: ""
                    enabled: {
                        if (root.focus === true
                                && titleTextField.visible === false
                                && root.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: "Open document in a new tab"
                    onTriggered: {
                        console.log("model.projectId", model.projectId)
                        root.openDocumentInAnotherView(model.projectId,
                                                       model.treeItemId)
                    }
                }

                Action {
                    id: openDocumentInNewWindowAction
                    //shortcut: "Alt+Return"
                    enabled: {
                        if (root.enabled && titleTextField.visible === false
                                && root.currentIndex === model.index) {
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

                contentItem: ColumnLayout {
                    id: columnLayout3
                    anchors.fill: parent

                    RowLayout {
                        id: rowLayout
                        spacing: 2
                        Layout.fillHeight: true
                        Layout.fillWidth: true

                        SkrCheckBox {
                            id: selectionCheckBox
                            //Layout.fillHeight: true
                            //Layout.preferredWidth: 5
                            visible: checkButtonsVisible
                            tristate: true
                            focusPolicy: Qt.NoFocus

                            onPressed: function(event) {
                                root.currentIndex = model.index
                            }

                            onCheckStateChanged: {

                                if (root.currentIndex === model.index) {
                                    //console.log("model.hasChildren", model.hasChildren)
                                    if (checkState === Qt.PartiallyChecked
                                            && !proxyModel.hasChildren(
                                                model.projectId,
                                                model.treeItemId)) {
                                        model.checkState = Qt.Checked
                                    } else if (checkState === Qt.PartiallyChecked
                                               && proxyModel.hasChildren(
                                                   model.projectId,
                                                   model.treeItemId)) {
                                        model.checkState = Qt.PartiallyChecked
                                    } else if (checkState === Qt.Checked) {
                                        model.checkState = Qt.Checked
                                    } else if (checkState === Qt.Unchecked) {
                                        model.checkState = Qt.Unchecked
                                    }
                                }
                            }

                            Binding on checkState {
                                value: model.checkState
                                delayed: true
                                restoreMode: Binding.RestoreBindingOrValue
                            }
                        }

                        Rectangle {
                            id: trashedIndicator
                            color: "#b50003"
                            Layout.fillHeight: true
                            Layout.preferredWidth: 5
                            visible: model.trashed
                        }
                        Rectangle {
                            id: currentItemIndicator
                            color: "lightsteelblue"
                            Layout.fillHeight: true
                            Layout.preferredWidth: 5
                            visible: root.currentIndex === model.index
                        }
                        Rectangle {
                            id: openedItemIndicator
                            color: SkrTheme.accent
                            Layout.fillHeight: true
                            Layout.preferredWidth: 5
                            visible: model.projectId === openedProjectId
                                     && model.treeItemId === openedTreeItemId
                        }

                        SkrButton {
                            id: projectIsBackupIndicator
                            visible: model.projectIsBackup
                                     && model.treeItemId === -1
                            enabled: true
                            focusPolicy: Qt.NoFocus
                            implicitHeight: 32
                            implicitWidth: 32
                            Layout.maximumHeight: 30
                            padding: 0
                            rightPadding: 0
                            bottomPadding: 0
                            leftPadding: 2
                            topPadding: 0
                            flat: true
                            onDownChanged: down = false

                            icon {
                                source: "qrc:///icons/backup/tools-media-optical-burn-image.svg"
                                height: 32
                                width: 32
                            }

                            hoverEnabled: true
                            ToolTip.delay: 1000
                            ToolTip.timeout: 5000
                            ToolTip.visible: hovered
                            ToolTip.text: qsTr("This project is a backup")
                        }

                        Rectangle {
                            color: "transparent"
                            //border.width: 1
                            Layout.fillWidth: true
                            Layout.fillHeight: true

                            ColumnLayout {
                                id: columnLayout2
                                spacing: 1
                                anchors.fill: parent

                                SkrLabel {
                                    id: titleLabel
                                    activeFocusOnTab: false

                                    Layout.fillWidth: true
                                    Layout.topMargin: 2
                                    Layout.leftMargin: 4
                                    Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter

                                    text: model.indent === -1 ? model.projectName : model.title
                                    elide: Text.ElideRight
                                }

                                SkrTextField {
                                    id: labelTextField
                                    visible: false

                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    text: labelLabel.text
                                    maximumLength: 50

                                    placeholderText: qsTr("Enter label")

                                    onEditingFinished: {
                                        //if (!activeFocus) {
                                        //accepted()
                                        //}
                                        //console.log("editing label finished")
                                        model.label = text
                                        delegateRoot.state = ""
                                    }

                                    //Keys.priority: Keys.AfterItem
                                    Keys.onShortcutOverride: function(event){
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
                                            delegateRoot.state = ""
                                            event.accepted = true
                                        }
                                    }
                                }

                                SkrTextField {
                                    id: titleTextField
                                    visible: false

                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    text: titleLabel.text
                                    maximumLength: 50

                                    placeholderText: qsTr("Enter name")

                                    onEditingFinished: {
                                        //if (!activeFocus) {
                                        //accepted()
                                        //}
                                        console.log("editing finished")
                                        model.title = text
                                        delegateRoot.state = ""
                                    }

                                    //Keys.priority: Keys.AfterItem
                                    Keys.onShortcutOverride: function(event){
                                     event.accepted  = (event.key === Qt.Key_Escape)
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
                                            delegateRoot.state = ""
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
                                        horizontalAlignment: Qt.AlignRight
                                        Layout.fillWidth: true
                                    }
                                }
                            }
                            //                        MouseArea {
                            //                            anchors.fill: parent
                            //                        }
                        }

                        SkrToolButton {
                            id: menuButton
                            Layout.fillHeight: true
                            Layout.preferredWidth: 30

                            text: "..."
                            flat: true
                            focusPolicy: Qt.NoFocus

                            onClicked: {

                                root.currentIndex = model.index
                                delegateRoot.forceActiveFocus()

                                if (menu.visible) {
                                    menu.close()
                                    return
                                }

                                menu.open()
                            }

                            visible: hoverHandler.hovered | content.isCurrent
                        }

                        Rectangle {
                            Layout.fillHeight: true
                            Layout.preferredWidth: 2

                            color: model.indent === 0 ? Material.color(
                                                            Material.Indigo) : (model.indent === 1 ? Material.color(Material.LightBlue) : (model.indent === 2 ? Material.color(Material.LightGreen) : (model.indent === 3 ? Material.color(Material.Amber) : (model.indent === 4 ? Material.color(Material.DeepOrange) : Material.color(Material.Teal)))))
                        }
                    }
                    Rectangle {
                        id: separator
                        Layout.preferredHeight: 1
                        Layout.preferredWidth: content.width / 2
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignBottom
                        gradient: Gradient {
                            orientation: Qt.Horizontal
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
                }
            }
            //            DropArea {
            //                id: dropArea
            //                anchors {
            //                    fill: parent
            //                    margins: 10
            //                }
            //                property int sourceIndex: -1
            //                property int targetIndex: -1
            //                onEntered: {
            //                    sourceIndex = drag.source.DelegateModel.itemsIndex
            //                    targetIndex = dragArea.DelegateModel.itemsIndex
            //                    //                    var sourceIndex = drag.source.DelegateModel.itemsIndex
            //                    //                    var targetIndex = dragArea.DelegateModel.itemsIndex
            //                    visualModel.items.move(sourceIndex, targetIndex)

            //                    //                    var sourceModelIndex = drag.source.DelegateModel.modelIndex(
            //                    //                                sourceIndex)
            //                    //                    var targetModelIndex = dragArea.DelegateModel.modelIndex(
            //                    //                                targetIndex)

            //                    //                    console.log("targetIndex : ", sourceModelIndex.title)
            //                }

            //                onDropped: {
            //                    console.log("onDropped")
            //                }
            //            }
            states: [
                State {
                    name: "drag_active"
                    when: content.Drag.active

                    ParentChange {
                        target: content
                        parent: base
                    }
                    AnchorChanges {
                        target: content
                        anchors {
                            horizontalCenter: undefined
                            verticalCenter: undefined
                        }
                    }
                },
                State {
                    name: "edit_title"
                    PropertyChanges {
                        target: menuButton
                        visible: false
                    }
                    PropertyChanges {
                        target: titleLabel
                        visible: false
                    }
                    PropertyChanges {
                        target: content
                        height: 50
                    }
                    PropertyChanges {
                        target: labelLayout
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
                        target: content
                        height: 50
                    }
                    PropertyChanges {
                        target: labelLayout
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

            //            Shortcut {
            //                sequences: ["Ctrl+Shift+N"]
            //                onActivated: addBeforeAction.trigger()
            //                enabled: root.visible
            //            }
            SkrMenu {
                id: menu
                y: menuButton.height

                onOpened: {
                    hoveringChangingTheCurrentItemAllowed = false
                    // necessary to differenciate between all items
                    contextMenuItemIndex = model.index
                }

                onClosed: {
                    hoveringChangingTheCurrentItemAllowed = true
                }

                SkrMenuItem {
                    height: model.isOpenable
                            && openActionsEnabled ? undefined : 0
                    visible: model.isOpenable && openActionsEnabled
                    action: Action {
                        id: openTreeItemAction
                        text: qsTr("Open")
                        //shortcut: "Return"
                        icon {
                            source: "qrc:///icons/backup/document-edit.svg"
                        }
                        enabled: openActionsEnabled
                                 && contextMenuItemIndex === model.index
                                 && titleTextField.visible === false
                                 && root.enabled && model.treeItemId !== -1
                        onTriggered: {
                            console.log("open treeItem action",
                                        model.projectId, model.treeItemId)
                            openDocumentAction.trigger()
                        }
                    }
                }

                SkrMenuItem {
                    height: model.isOpenable
                            && openActionsEnabled ? undefined : 0
                    visible: model.isOpenable && openActionsEnabled
                    action: Action {
                        id: openTreeItemInAnotherViewAction
                        text: qsTr("Open in new tab")
                        //shortcut: "Alt+Return"
                        icon {
                            source: "qrc:///icons/backup/tab-new.svg"
                        }
                        enabled: openActionsEnabled
                                 && contextMenuItemIndex === model.index
                                 && titleTextField.visible === false
                                 && root.enabled && model.treeItemId !== -1
                        onTriggered: {
                            console.log("open treeItem in new tab action",
                                        model.projectId, model.treeItemId)
                            openDocumentInAnotherViewAction.trigger()
                        }
                    }
                }

                SkrMenuItem {
                    height: model.isOpenable
                            && openActionsEnabled ? undefined : 0
                    visible: model.isOpenable && openActionsEnabled
                    action: Action {
                        id: openTreeItemInNewWindowAction
                        text: qsTr("Open in new window")
                        //shortcut: "Alt+Return"
                        icon {
                            source: "qrc:///icons/backup/window-new.svg"
                        }
                        enabled: openActionsEnabled
                                 && contextMenuItemIndex === model.index
                                 && titleTextField.visible === false
                                 && root.enabled && model.treeItemId !== -1
                        onTriggered: {
                            console.log("open treeItem in new window action",
                                        model.projectId, model.treeItemId)
                            openDocumentInNewWindowAction.trigger()
                        }
                    }
                }

                MenuSeparator {
                    height: model.isRenamable
                            && renameActionEnabled ? undefined : 0
                    visible: model.isRenamable && renameActionEnabled
                }

                SkrMenuItem {
                    height: model.isRenamable
                            && renameActionEnabled ? undefined : 0
                    visible: model.isRenamable && renameActionEnabled
                    action: Action {
                        id: renameAction
                        text: qsTr("Rename")
                        //shortcut: "F2"
                        icon {
                            source: "qrc:///icons/backup/edit-rename.svg"
                        }
                        enabled: renameActionEnabled
                                 && contextMenuItemIndex === model.index
                                 && root.enabled
                        onTriggered: {
                            console.log("rename action", model.projectId,
                                        model.treeItemId)
                            delegateRoot.editName()
                        }
                    }
                }

                SkrMenuItem {
                    height: model.isRenamable
                            && renameActionEnabled ? undefined : 0
                    visible: model.isRenamable && renameActionEnabled
                    action: Action {
                        id: setLabelAction
                        text: qsTr("Set label")

                        icon {
                            source: "qrc:///icons/backup/label.svg"
                        }
                        enabled: renameActionEnabled
                                 && contextMenuItemIndex === model.index
                                 && root.enabled
                        onTriggered: {
                            console.log("sel label", model.projectId,
                                        model.treeItemId)
                            delegateRoot.editLabel()
                        }
                    }
                }

                MenuSeparator {
                    height: (model.isCopyable && copyActionEnabled)
                            || (model.isMovable && cutActionEnabled)
                            || (model.canAddChildTreeItem
                                && pasteActionEnabled) ? undefined : 0
                    visible: (model.isCopyable && copyActionEnabled)
                             || (model.isMovable && cutActionEnabled)
                             || (model.canAddChildTreeItem
                                 && pasteActionEnabled)
                }

                SkrMenuItem {
                    height: model.isMovable && cutActionEnabled ? undefined : 0
                    visible: model.isMovable && cutActionEnabled
                    action: Action {
                        id: cutAction
                        text: qsTr("Cut")
                        //shortcut: StandardKey.Cut
                        icon {
                            source: "qrc:///icons/backup/edit-cut.svg"
                        }
                        enabled: contextMenuItemIndex === model.index
                                 && root.enabled

                        onTriggered: {
                            console.log("cut action", model.projectId,
                                        model.treeItemId)
                            proxyModel.cut(model.projectId, model.treeItemId)
                        }
                    }
                }

                SkrMenuItem {
                    height: model.isCopyable
                            && copyActionEnabled ? undefined : 0
                    visible: model.isCopyable && copyActionEnabled
                    action: Action {
                        id: copyAction
                        text: qsTr("Copy")
                        //shortcut: StandardKey.Copy
                        icon {
                            source: "qrc:///icons/backup/edit-copy.svg"
                        }
                        enabled: copyActionEnabled
                                 && contextMenuItemIndex === model.index
                                 && root.enabled

                        onTriggered: {
                            console.log("copy action", model.projectId,
                                        model.treeItemId)
                            proxyModel.copy(model.projectId, model.treeItemId)
                        }
                    }
                }

                SkrMenuItem {
                    height: model.canAddChildTreeItem
                            && pasteActionEnabled ? undefined : 0
                    visible: model.canAddChildTreeItem && pasteActionEnabled
                    action: Action {
                        id: pasteAction
                        text: qsTr("Paste")
                        //shortcut: StandardKey.Copy
                        icon {
                            source: "qrc:///icons/backup/edit-paste.svg"
                        }
                        enabled: pasteActionEnabled
                                 && contextMenuItemIndex === model.index
                                 && root.enabled

                        onTriggered: {
                            console.log("paste action", model.projectId,
                                        model.treeItemId)
                            proxyModel.paste(model.projectId, model.treeItemId)
                        }
                    }
                }

                MenuSeparator {
                    height: (model.canAddSiblingTreeItem
                             || model.canAddChildTreeItem)
                            && (addChildTreeItemActionEnabled
                                || addSiblingTreeItemActionEnabled) ? undefined : 0
                    visible: (model.canAddSiblingTreeItem
                              || model.canAddChildTreeItem)
                             && (addChildTreeItemActionEnabled
                                 || addSiblingTreeItemActionEnabled)
                }

                SkrMenuItem {
                    height: model.canAddSiblingTreeItem
                            && addSiblingTreeItemActionEnabled ? undefined : 0
                    visible: model.canAddSiblingTreeItem
                             && addSiblingTreeItemActionEnabled
                    action: Action {
                        id: addBeforeAction
                        text: qsTr("Add before")
                        //shortcut: "Ctrl+Shift+N"
                        icon {
                            source: "qrc:///icons/backup/document-new.svg"
                        }
                        enabled: contextMenuItemIndex === model.index
                                 && root.enabled
                        onTriggered: {
                            console.log("add before action", model.projectId,
                                        model.treeItemId)

                            var result = proxyModel.addItemAbove(
                                        model.projectId, model.treeItemId, 0)

                            // edit it :
                            root.itemAtIndex(
                                        currentIndex).treeItemIdToEdit = result.getData(
                                        "treeItemId", -2)
                        }
                    }
                }

                SkrMenuItem {
                    height: model.canAddSiblingTreeItem
                            && addSiblingTreeItemActionEnabled ? undefined : 0
                    visible: model.canAddSiblingTreeItem
                             && addSiblingTreeItemActionEnabled
                    action: Action {
                        id: addAfterAction
                        text: qsTr("Add after")
                        //shortcut: "Ctrl+N"
                        icon {
                            source: "qrc:///icons/backup/document-new.svg"
                        }
                        enabled: contextMenuItemIndex === model.index
                                 && root.enabled
                        onTriggered: {
                            console.log("add after action", model.projectId,
                                        model.treeItemId)

                            var result = proxyModel.addItemBelow(
                                        model.projectId, model.treeItemId, 0)

                            // edit it :
                            root.itemAtIndex(
                                        currentIndex).treeItemIdToEdit = result.getData(
                                        "treeItemId", -2)
                        }
                    }
                }

                SkrMenuItem {
                    height: model.canAddChildTreeItem
                            && addChildTreeItemActionEnabled ? undefined : 0
                    visible: model.canAddChildTreeItem
                             && addChildTreeItemActionEnabled
                    action: Action {
                        id: addChildAction
                        text: qsTr("Add a sub-item")
                        //shortcut: "Ctrl+N"
                        icon {
                            source: "qrc:///icons/backup/document-new.svg"
                        }
                        enabled: contextMenuItemIndex === model.index
                                 && root.enabled
                        onTriggered: {
                            console.log("add child action", model.projectId,
                                        model.treeItemId)

                            var result = proxyModel.addChildItem(
                                        model.projectId, model.treeItemId, 0)

                            // edit it :
                            root.itemAtIndex(
                                        currentIndex).treeItemIdToEdit = result.getData(
                                        "treeItemId", -2)
                        }
                    }
                }

                MenuSeparator {
                    height: model.isMovable && moveActionEnabled ? undefined : 0
                    visible: model.isMovable && moveActionEnabled
                }

                SkrMenuItem {
                    height: model.isMovable && moveActionEnabled ? undefined : 0
                    visible: model.isMovable && moveActionEnabled
                    action: Action {
                        id: moveUpAction
                        text: qsTr("Move up")
                        //shortcut: "Ctrl+Up"
                        icon {
                            source: "qrc:///icons/backup/object-order-raise.svg"
                        }
                        enabled: root.enabled && currentIndex !== 0
                                 && root.enabled
                        onTriggered: {
                            console.log("move up action", currentProjectId,
                                        currentTreeItemId)

                            proxyModel.moveUp(currentProjectId,
                                              currentTreeItemId, currentIndex)
                        }
                    }
                }

                SkrMenuItem {
                    height: model.isMovable && moveActionEnabled ? undefined : 0
                    visible: model.isMovable && moveActionEnabled
                    action: Action {
                        id: moveDownAction
                        text: qsTr("Move down")
                        //shortcut: "Ctrl+Down"
                        icon {
                            source: "qrc:///icons/backup/object-order-lower.svg"
                        }
                        enabled: currentIndex !== visualModel.items.count - 1
                                 && root.enabled

                        onTriggered: {
                            console.log("move down action", currentProjectId,
                                        currentTreeItemId)

                            proxyModel.moveDown(currentProjectId,
                                                currentTreeItemId, currentIndex)
                        }
                    }
                }

                MenuSeparator {
                    height: model.isTrashable
                            && (sendToTrashActionEnabled
                                || deleteActionEnabled) ? undefined : 0
                    visible: model.isTrashable && (sendToTrashActionEnabled
                                                   || deleteActionEnabled)
                }

                SkrMenuItem {
                    height: model.isTrashable
                            && sendToTrashActionEnabled ? undefined : 0
                    visible: model.isTrashable && sendToTrashActionEnabled
                    action: Action {
                        text: qsTr("Send to trash")
                        //shortcut: "Del"
                        icon {
                            source: "qrc:///icons/backup/edit-delete.svg"
                        }
                        enabled: sendToTrashActionEnabled
                                 && contextMenuItemIndex === model.index
                                 && root.enabled && model.indent !== -1
                        onTriggered: {
                            console.log("sent to trash action",
                                        model.projectId, model.treeItemId)
                            //sendToTrashCalled(model.projectId, model.treeItemId)
                            proxyModel.trashItemWithChildren(model.projectId,
                                                             model.treeItemId)
                        }
                    }
                }

                SkrMenuItem {
                    height: model.isTrashable
                            && deleteActionEnabled ? undefined : 0
                    visible: model.isTrashable && deleteActionEnabled
                    action: Action {
                        text: qsTr("Delete definitively")
                        //shortcut: "Del"
                        icon {
                            source: "qrc:///icons/backup/edit-delete-shred.svg"
                        }
                        enabled: deleteActionEnabled
                                 && contextMenuItemIndex === model.index
                                 && root.enabled && model.indent !== -1
                        onTriggered: {
                            console.log("delete action", model.projectId,
                                        model.treeItemId)
                            deleteDefinitivelyCalled(model.projectId,
                                                     model.treeItemId)
                        }
                    }
                }
            }

            ListView.onRemove: SequentialAnimation {
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: true
                }
                NumberAnimation {
                    target: delegateRoot
                    property: "height"
                    to: 0
                    easing.type: Easing.InOutQuad
                }
                PropertyAction {
                    target: delegateRoot
                    property: "ListView.delayRemove"
                    value: false
                }
            }

            //----------------------------------------------------------
            ListView.onAdd: SequentialAnimation {
                PropertyAction {
                    target: delegateRoot
                    property: "height"
                    value: 0
                }
                NumberAnimation {
                    target: delegateRoot
                    property: "height"
                    to: delegateRoot.height
                    duration: 250
                    easing.type: Easing.InOutQuad
                }
            }

            // edit name  :
            property int treeItemIdToEdit: -2
            onTreeItemIdToEditChanged: {
                if (treeItemIdToEdit !== -2) {
                    editNameTimer.start()
                }
            }

            Timer {
                id: editNameTimer
                repeat: false
                interval: 250 //draggableContent.transitionAnimationDuration
                onTriggered: {
                    var index = proxyModel.findVisualIndex(model.projectId,
                                                           treeItemIdToEdit)
                    if (index !== -2) {
                        root.itemAtIndex(index).editName()
                    }
                    treeItemIdToEdit = -2
                }
            }
        }
    }

    remove: Transition {
        enabled: SkrSettings.ePaperSettings.animationEnabled

        SequentialAnimation {
            id: removeTreeItemAnimation
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
                to: root.width
                duration: 250
                easing.type: Easing.InBack
            }
            PropertyAction {
                property: "ListView.delayRemove"
                value: false
            }
        }
    }

    removeDisplaced: Transition {
        enabled: SkrSettings.ePaperSettings.animationEnabled
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
}
