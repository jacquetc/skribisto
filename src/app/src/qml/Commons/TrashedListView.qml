import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import eu.skribisto.projecthub 1.0
import QtQuick.Controls.Material 2.15
import "../Items"
import ".."

TrashedListViewForm {
    id: root

    property var proxyModel
    property var model
    onModelChanged: {
        visualModel.model = model
    }
    signal openDocument(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openDocumentInNewTab(int projectId, int paperId)
    signal openDocumentInNewWindow(int projectId, int paperId)
    signal restoreDocument(int projectId, int paperId)




    property int currentProjectId: -2
    property int currentPaperId: -2
    property int currentIndex: listView.currentIndex
    property int openedProjectId: -2
    property int openedPaperId: -2
    property bool hoveringChangingTheCurrentItemAllowed: true
    listView.model: visualModel
    DelegateModel {
        id: visualModel

        delegate: dragDelegate
    }


    // scrollBar interactivity :

    listView.onContentHeightChanged: {
        //fix scrollbar visible at start
        if(scrollView.height === 0){
            scrollBarVerticalPolicy = ScrollBar.AlwaysOff
            return
        }

        if(listView.contentHeight > scrollView.height){
            scrollBarVerticalPolicy = ScrollBar.AlwaysOn
        }
        else {
            scrollBarVerticalPolicy = ScrollBar.AlwaysOff
        }
    }


    //-----------------------------------------------------------------------------


    //-----------------------------------------------------------------------------
    // project comboBox :



    trashProjectComboBox.model: ListModel {
        id: projectComboBoxModel
    }


    Connections {

        target: plmData.projectHub()
        function onProjectLoaded(projectId){


            var name =  plmData.projectHub().getProjectName(projectId)

            projectComboBoxModel.append({projectId: projectId, name: name})

            trashProjectComboBox.currentIndex = trashProjectComboBox.count -1
        }
    }

    Connections {

        target: plmData.projectHub()
        function onProjectClosed(projectId){

            populateProjectComboBoxModel()

        }
    }

    Component.onCompleted: {
        trashProjectComboBox.textRole = "name"
        trashProjectComboBox.valueRole = "projectId"
        populateProjectComboBoxModel()
    }
    trashProjectComboBox.displayText: qsTr("Trash: %1").arg(trashProjectComboBox.currentText)

    function populateProjectComboBoxModel(){

        projectComboBoxModel.clear()

        // populate

        var projectList = plmData.projectHub().getProjectIdList()

        var i;
        for(i = 0 ; i < projectList.length ; i++ ){
            var projectId = projectList[i]

            var name =  plmData.projectHub().getProjectName(projectId)

            projectComboBoxModel.append({projectId: projectId, name: name})
            console.log("projectList")

        }

        // select last :
        if (projectList.length > 0){
            trashProjectComboBox.currentIndex = projectList.length - 1;
            var _projectId = trashProjectComboBox.valueAt(projectList.length - 1)
            proxyModel.projectIdFilter = _projectId
            currentProjectId = _projectId
        }
    }
    trashProjectComboBox.onCurrentIndexChanged: {
        console.log("onCurrentIndexChanged")

        if(trashProjectComboBox.currentValue === undefined){

            var projectList = plmData.projectHub().getProjectIdList()
            trashProjectComboBox.currentIndex = projectList.length - 1;
            var _projectId = trashProjectComboBox.valueAt(projectList.length - 1)
            proxyModel.projectIdFilter = _projectId
            currentProjectId = _projectId
        }
        else {
            proxyModel.projectIdFilter = trashProjectComboBox.currentValue
            currentProjectId = trashProjectComboBox.currentValue

        }


    }


    //-----------------------------------------------------------------------------
    // go back button :

    signal goBack()

    goBackToolButton.action: Action {
        id: goBackAction
        text: "<"
        icon.name: "go-previous"
        onTriggered:{
            goBack()
        }
    }




    //----------------------------------------------------------------------------

    function prepareDeleteDefinitivelyDialog(projectId, paperId){

        var idList = [paperId]
        idList = idList.concat(proxyModel.getChildrenList(projectId, paperId, true, false))

        //get names
        var nameList = []
        var i
        for(i = 0 ; i < idList.length ; i++){
            var id = idList[i]

            nameList.push(proxyModel.getItemName(projectId, id))

        }

        deleteDefinitivelyDialog.projectId = projectId
        deleteDefinitivelyDialog.projectName = plmData.projectHub().getProjectName(projectId)
        deleteDefinitivelyDialog.paperIdList = idList
        deleteDefinitivelyDialog.paperNamesString = "\n- " + nameList.join("\n- ")
        deleteDefinitivelyDialog.open()

    }


    SimpleDialog {
        property int projectId: -2
        property string projectName: ""
        property var paperIdList: []
        property var paperNamesString: ""

        id: deleteDefinitivelyDialog
        title: "Warning"
        text: qsTr("Do you want to delete definitively the following documents from the \"%1\" project ?\n%2").arg(projectName).arg(paperNamesString)
        standardButtons: Dialog.Yes  |  Dialog.Cancel

        onRejected: {
            deleteDefinitivelyDialog.close()

        }

        onAccepted: {
            var i
            for(i = 0 ; i < paperIdList.length ; i++){
                var id = paperIdList[i]

                proxyModel.deleteDefinitively(projectId, id)
            }

        }
    }


    //----------------------------------------------------------------------------


    //----------------------------------------------------------------------------
    listMenuToolButton.icon.name: "overflow-menu"
    listMenuToolButton.onClicked: navigationMenu.open()

    SkrMenu {
        id: navigationMenu
        y: listMenuToolButton.height

        //        Action {
        //            text: qsTr("Rename")
        //        }

        //        MenuSeparator {}
        //        Action {
        //            text: qsTr("Remove")
        //        }
        //        MenuSeparator {}

        Action {
            text: qsTr("Empty the trash")
            enabled: navigationMenu.opened
            //shortcut: "Ctrl+Shift+Del"
            icon.name: "edit-delete-shred"
            onTriggered: {
                //TODO: fill that
            }
        }

    }

    //----------------------------------------------------------------------------
    // restore button :

    Action {
        id: restoreAction
        text: qsTr("Restore")
        //shortcut: ""
        enabled: listView.focus === true
        icon{
            name: "edit-undo"
            height: 100
            width: 100
        }
        onTriggered: {

            console.log('restore', currentProjectId, currentPaperId)
            restoreDocument(currentProjectId, currentPaperId)

        }
    }

    restoreToolButton.action: restoreAction

    //----------------------------------------------------------------------------

    // shortcuts



    Shortcut {
        enabled: listView.enabled
        sequences: ["Left", "Backspace"]
        onActivated: goBackAction.trigger()
        //enabled: listView.activeFocus
    }
    //-----------------------------------------------------------------------------
    listView.onCurrentIndexChanged: {
        contextMenuItemIndex = listView.currentIndex
    }

    property int contextMenuItemIndex: -2

    Binding {
        target: listView
        property: "currentIndex"
        value: proxyModel.forcedCurrentIndex
    }

    //----------------------------------------------------------------------------

    // focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            listView.forceActiveFocus()
        }
    }

    //----------------------------------------------------------------------------

    // TreeView item :
    Component {
        id: dragDelegate

        Item {
            id: delegateRoot



            Accessible.name: labelLabel.text.length === 0 ? titleLabel.text  +  ( model.hasChildren ? " " +qsTr("has child") :  "" ):
                                                            titleLabel.text + " " + qsTr("label:") + " " + labelLabel.text + ( model.hasChildren ? " " +qsTr("has child") :  "" )
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
            }

            height: content.height


            onActiveFocusChanged: {
                if(listView.currentIndex === model.index && model.index !== -1 && activeFocus){
                    root.currentPaperId = model.paperId
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
            //            Keys.onShortcutOverride: {
            //                if (event.key === Qt.Key_Backspace) {
            //                    console.log("onShortcutOverride")
            //                    event.accepted = true
            //                }
            //            }
            //            Keys.onBackPressed: {
            //                console.log("eee")
            //            }
            function editName() {
                state = "edit_name"
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
                state = "edit_label"
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
                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_N){
                    event.accepted = true
                }
                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_C){
                    event.accepted = true
                }
                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_X){
                    event.accepted = true
                }
                if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_V){
                    event.accepted = true
                }
                if( event.key === Qt.Key_Escape && (delegateRoot.state == "edit_name" || delegateRoot.state == "edit_label")){
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

                if (event.key === Qt.Key_F2 && delegateRoot.state !== "edit_name" && delegateRoot.state !== "edit_label"){
                    renameAction.trigger()
                    event.accepted = true
                }
                if (event.key === Qt.Key_Escape){
                    console.log("Escape key pressed")
                    goBackAction.trigger()
                    event.accepted = true
                }
            }


            Rectangle {
                id: content
                property int visualIndex: 0
                property int sourceIndex: -2

                property bool isCurrent: model.index === listView.currentIndex ? true : false

                anchors {
                    horizontalCenter: parent.horizontalCenter
                    verticalCenter: parent.verticalCenter
                }
                width: delegateRoot.width
                height: 50


                Behavior on color {
                    ColorAnimation {
                        duration: 100
                    }
                }

                color: "transparent"

                HoverHandler {
                    id: hoverHandler
                    //                    onHoveredChanged: {
                    //                        if (hoverHandler.hovered & hoveringChangingTheCurrentItemAllowed) {
                    //                            listView.currentIndex = model.index
                    //                        }
                    //                    }
                }

                TapHandler {
                    id: tapHandler



                    onSingleTapped: {
                        listView.currentIndex = model.index
                        delegateRoot.forceActiveFocus()
                        eventPoint.accepted = true
                    }

                    onDoubleTapped: {
                        console.log("double tapped")
                        listView.currentIndex = model.index
                        openDocumentAction.trigger()
                        eventPoint.accepted = true
                    }

                }

                TapHandler {
                    acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                    acceptedButtons: Qt.RightButton
                    onTapped: {


                        if(menu.visible){
                            menu.close()
                            return
                        }

                        listView.currentIndex = model.index
                        menu.open()
                        eventPoint.accepted = true
                    }
                }
                TapHandler {
                    acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                    acceptedButtons: Qt.MiddleButton
                    onTapped: {
                        listView.currentIndex = model.index
                        openDocumentInNewTabAction.trigger()
                        eventPoint.accepted = true

                    }
                }

                Action {
                    id: openDocumentAction
                    //shortcut: "Return"
                    enabled: {
                        if (listView.focus === true && titleTextField.visible === false
                                && listView.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: "Open document"
                    onTriggered: {
                        console.log("model.openedProjectId", openedProjectId)
                        console.log("model.projectId", model.projectId)
                        root.openDocument(openedProjectId, openedPaperId, model.projectId,
                                          model.paperId)
                    }
                }

                Action {
                    id: openDocumentInNewTabAction
                    //shortcut: "Alt+Return"
                    enabled: {
                        if (listView.focus === true && titleTextField.visible === false
                                && listView.currentIndex === model.index) {
                            return true
                        } else
                            return false
                    }

                    text: "Open document in a new tab"
                    onTriggered: {
                        console.log("model.projectId", model.projectId)
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

                ColumnLayout{
                    id: columnLayout3
                    anchors.fill: parent


                    RowLayout {
                        id: rowLayout
                        spacing: 2
                        Layout.fillHeight: true
                        Layout.fillWidth: true

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
                            visible: listView.currentIndex === model.index
                        }
                        Rectangle {
                            id: openedItemIndicator
                            color:  SkrTheme.accent
                            Layout.fillHeight: true
                            Layout.preferredWidth: 5
                            visible: model.projectId === openedProjectId && model.paperId === openedPaperId
                        }

                        SkrButton {
                            id: projectIsBackupIndicator
                            visible: model.projectIsBackup && model.paperId === -1
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
                                name: "tools-media-optical-burn-image"
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

                                    Layout.fillWidth: true
                                    Layout.topMargin: 2
                                    Layout.leftMargin: 4
                                    Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter

                                    text: model.indent === -1 ? model.projectName : model.name
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
                                        console.log("editing label finished")
                                        model.label = text
                                        delegateRoot.state = ""
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
                                        model.name = text
                                        delegateRoot.state = ""
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

                                SkrLabel {
                                    id: labelLabel

                                    //                                text: model.label
                                    text:  model.label === undefined ? "" : model.label
                                    Layout.bottomMargin: 2
                                    Layout.rightMargin: 4
                                    Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                                    elide: Text.ElideRight
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

                                listView.currentIndex = model.index
                                delegateRoot.forceActiveFocus()
                                menu.open()
                            }

                            visible: hoverHandler.hovered | content.isCurrent
                        }

                        Rectangle {
                            Layout.fillHeight: true
                            Layout.preferredWidth: 2

                            color: model.indent === 0 ? Material.color(Material.Indigo) :
                                                        (model.indent === 1 ? Material.color(Material.LightBlue) :
                                                                         (model.indent === 2 ? Material.color(Material.LightGreen) :
                                                                                               (model.indent === 3 ? Material.color(Material.Amber) :
                                                                                                                     (model.indent === 4 ? Material.color(Material.DeepOrange) :
                                                                                               Material.color(Material.Teal)
                                                                              ))))
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

            //                    //                    console.log("targetIndex : ", sourceModelIndex.name)
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
                    id: restoreMenuItem
                    action: restoreAction

                }



                Action {
                    id: openPaperAction
                    text: qsTr("Open")
                    //shortcut: "Return"
                    icon {
                        name: "document-edit"
                    }
                    enabled: contextMenuItemIndex === model.index && titleTextField.visible === false  && listView.enabled &&  model.paperId !== -1
                    onTriggered: {
                        console.log("from trashed: open paper action", model.projectId,
                                    model.paperId)
                        openDocumentAction.trigger()
                    }
                }

                Action {
                    id: openPaperInNewTabAction
                    text: qsTr("Open in new tab")
                    //shortcut: "Alt+Return"
                    icon {
                        name: "tab-new"
                    }
                    enabled: contextMenuItemIndex === model.index && titleTextField.visible === false  && listView.enabled &&  model.paperId !== -1
                    onTriggered: {
                        console.log("from trashed: open paper in new tab action", model.projectId,
                                    model.paperId)
                        openDocumentInNewTabAction.trigger()
                    }
                }

                Action {
                    id: openPaperInNewWindowAction
                    text: qsTr("Open in new window")
                    //shortcut: "Alt+Return"
                    icon {
                        name: "window-new"
                    }
                    enabled: contextMenuItemIndex === model.index && titleTextField.visible === false && listView.enabled &&  model.paperId !== -1
                    onTriggered: {
                        console.log("from trashed: open paper in new window action", model.projectId,
                                    model.paperId)
                        openDocumentInNewWindowAction.trigger()
                    }
                }


                MenuSeparator {}

                Action {
                    id: renameAction
                    text: qsTr("Rename")
                    //shortcut: "F2"
                    icon {
                        name: "edit-rename"
                    }
                    enabled: contextMenuItemIndex === model.index  && listView.enabled
                    onTriggered: {
                        console.log("from trashed: rename action", model.projectId,
                                    model.paperId)
                        delegateRoot.editName()
                    }
                }

                Action {
                    id: setLabelAction
                    text: qsTr("Set label")

                    icon {
                        name: "label"
                    }
                    enabled: contextMenuItemIndex === model.index  && listView.enabled && model.indent !== -1
                    onTriggered: {
                        console.log("from trashed: sel label", model.projectId,
                                    model.paperId)
                        delegateRoot.editLabel()
                    }
                }

                MenuSeparator {}

                Action {

                    text: qsTr("Copy")
                    //shortcut: StandardKey.Copy
                    icon {
                        name: "edit-copy"
                    }
                    enabled: contextMenuItemIndex === model.index  && listView.enabled && model.indent !== -1

                    onTriggered: {
                        console.log("from trashed: copy action", model.projectId,
                                    model.paperId)
                        proxyModel.copy(model.projectId, model.paperId)
                    }
                }

                MenuSeparator {}

                Action {
                    text: qsTr("Delete definitively")
                    //shortcut: "Del"
                    icon {
                        name: "edit-delete"
                    }
                    enabled: contextMenuItemIndex === model.index  && listView.enabled && model.indent !== -1
                    onTriggered: {
                        console.log("from trashed: delete action", model.projectId,
                                    model.paperId)

                        prepareDeleteDefinitivelyDialog(model.projectId, model.paperId)
                    }
                }
                MenuSeparator {}
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

            // move :
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
}
