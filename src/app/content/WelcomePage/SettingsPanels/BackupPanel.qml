import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts
import Qt.labs.platform 1.1 as LabPlatform
import Qt.labs.settings 1.1
import eu.skribisto.projecthub 1.0
import eu.skribisto.spellchecker 1.0
import QtQuick.Controls.Material

import SkrControls
import Skribisto

BackupPanelForm {

    Component.onCompleted: {

        checkOnBackUpEveryCheckBox()
        populateBackupPathListView()

        backUpOnceADayIfNeeded()
    }

    // --------------------------------------------
    // ---- backup --------------------------------
    // --------------------------------------------
    // backup paths :
    ListModel {
        id: backupPathListModel
    }
    backupPathListView.model: backupPathListModel

    function populateBackupPathListView() {

        var backupPaths = SkrSettings.backupSettings.paths
        var backupPathList = backupPaths.split(";")

        //no backup path set
        if (backupPaths === "") {

            //TODO: send notification, backup not configured
            return
        }

        var j
        for (j = 0; j < backupPathList.length; j++) {
            var path = backupPathList[j]
            if (path === "") {
                continue
            }

            backupPathListModel.append({
                                           "path": skrQMLTools.getURLFromLocalFile(
                                                       path),
                                           "localPath": path
                                       })
        }
    }

    function addBackupPath(pathUrl) {

        backupPathListModel.append({
                                       "path": pathUrl,
                                       "localPath": skrQMLTools.translateURLToLocalFile(
                                                        pathUrl)
                                   })
        saveBackupPathsToSettings()
    }

    function removeBackupPath(index) {

        backupPathListModel.remove(index)
        saveBackupPathsToSettings()
    }

    function saveBackupPathsToSettings() {

        var backupPathList = []
        var j
        for (j = 0; j < backupPathListModel.count; j++) {

            backupPathList.push(backupPathListModel.get(j).localPath)
        }

        var backupPaths = backupPathList.join(";")
        SkrSettings.backupSettings.paths = backupPaths
    }

    LabPlatform.FolderDialog {
        id: addBackupFolderDialog
        folder: LabPlatform.StandardPaths.writableLocation(
                    LabPlatform.StandardPaths.DocumentsLocation)
        onAccepted: {
            //                    var path = folderDialog.folder.toString()
            //                    path = path.replace(/^(file:\/{2})/,"");
            //                    model.path = path
            var path = addBackupFolderDialog.folder

            addBackupPath(path)
            backupPathListView.currentIndex = backupPathListView.count - 1
        }
        onRejected: {

        }
    }

    Action {
        id: addBackupPathAction
        text: qsTr("Add backup path")
        icon.source: "../../icons/3rdparty/backup/list-add.svg"
        onTriggered: {

            addBackupFolderDialog.open()
        }
    }
    addBackupPathButton.action: addBackupPathAction

    Action {
        id: removeBackupPathAction
        text: qsTr("Remove backup path")
        icon.source: "../../icons/3rdparty/backup/list-remove.svg"
        onTriggered: {

            removeBackupPath(backupPathListView.currentIndex)
        }
    }
    removeBackupPathButton.action: removeBackupPathAction

    backupPathListView.delegate: Component {
        id: itemDelegate

        Item {
            id: delegateRoot
            height: Material.delegateHeight

            anchors {
                left: Qt.isQtObject(parent) ? parent.left : undefined
                right: Qt.isQtObject(parent) ? parent.right : undefined
                leftMargin: 5
                rightMargin: 5
            }

            TapHandler {
                id: tapHandler
                onSingleTapped: function (eventPoint) {
                    backupPathListView.currentIndex = model.index
                    delegateRoot.forceActiveFocus()
                    eventPoint.accepted = true
                }
                onDoubleTapped: function (eventPoint) {

                    // path editor :
                    //backupPathListView.currentIndex = model.index
                    editName()

                    eventPoint.accepted = true
                }
            }

            function editName() {
                state = "edit_path"
                editPathTextField.forceActiveFocus()
                editPathTextField.selectAll()
            }

            LabPlatform.FolderDialog {
                id: backupFolderDialog

                onAccepted: {

                    model.path = backupFolderDialog.folder
                    model.localPath = skrQMLTools.translateURLToLocalFile(
                                backupFolderDialog.folder)
                }
                onRejected: {

                }
            }

            RowLayout {
                anchors.fill: parent

                SkrLabel {
                    id: pathLabel
                    activeFocusOnTab: false
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    text: model.localPath
                    horizontalAlignment: Qt.AlignLeft
                    verticalAlignment: Qt.AlignVCenter
                }
                SkrButton {
                    id: selectFolderButton
                    Layout.fillHeight: true
                    Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                    flat: true
                    icon.source: "../../icons/3rdparty/backup/document-open.svg"

                    onClicked: {
                        backupFolderDialog.open()
                        backupFolderDialog.currentFolder = model.path
                    }
                }

                SkrTextField {
                    id: editPathTextField
                    visible: false
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    text: model.localPath
                    horizontalAlignment: Qt.AlignLeft
                    verticalAlignment: Qt.AlignVCenter

                    placeholderText: qsTr("Enter a path to back up to")

                    onEditingFinished: {

                        //if (!activeFocus) {
                        //accepted()
                        //}
                        //console.log("editing finished")
                        model.path = text

                        saveBackupPathsToSettings()

                        delegateRoot.state = ""
                    }

                    //Keys.priority: Keys.AfterItem
                    Keys.onPressed: function (event) {
                        if (event.key === Qt.Key_Return) {
                            //console.log("Return key pressed path")
                            editingFinished()
                            event.accepted = true
                        }
                        if ((event.modifiers & Qt.CtrlModifier)
                                && event.key === Qt.Key_Return) {

                            //console.log("Ctrl Return key pressed title")
                            backupFolderDialog.open()
                            backupFolderDialog.currentFolder = model.path
                            event.accepted = true
                        }
                    }
                }
            }

            states: [
                State {
                    name: "edit_path"
                    PropertyChanges {
                        target: pathLabel
                        visible: false
                    }
                    PropertyChanges {
                        target: editPathTextField
                        visible: true
                    }
                }
            ]
        }
    }

    backupPathListView.highlight: Component {
        id: highlight
        Rectangle {

            radius: 5
            border.color: "lightsteelblue"
            border.width: 2
            visible: backupPathListView.activeFocus
            Behavior on y {
                enabled: SkrSettings.ePaperSettings.animationEnabled
                SpringAnimation {
                    spring: 3
                    damping: 0.2
                }
            }
        }
    }

    // backup every :
    backUpEveryCheckBox.checked: SkrSettings.backupSettings.backUpEveryCheckBoxChecked
    Binding {
        target: SkrSettings.backupSettings
        property: "backUpEveryCheckBoxChecked"
        value: backUpEveryCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    backUpEveryCheckBox.onCheckedChanged: {
        checkOnBackUpEveryCheckBox()
    }

    function checkOnBackUpEveryCheckBox() {
        backupHoursDial.enabled = backUpEveryCheckBox.checked
        backupHoursSpinBox.enabled = backUpEveryCheckBox.checked
    }

    //backUp Once A Day CheckBox
    backUpOnceADayCheckBox.checked: SkrSettings.backupSettings.backUpOnceADay
    Binding {
        target: SkrSettings.backupSettings
        property: "backUpOnceADay"
        value: backUpOnceADayCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    backUpOnceADayCheckBox.onCheckedChanged: {
        if (backUpOnceADayCheckBox.checked) {
            backUpOnceADayIfNeeded()
        }
    }

    //dials :
    backupHoursDial.onMoved: backupHoursSpinBox.value = backupHoursDial.value
    backupHoursSpinBox.onValueModified: backupHoursDial.value = backupHoursSpinBox.value

    backupHoursDial.value: SkrSettings.backupSettings.backUpEveryHours
    backupHoursSpinBox.value: SkrSettings.backupSettings.backUpEveryHours
    Binding {
        delayed: true
        target: SkrSettings.backupSettings
        property: "backUpEveryHours"
        value: backupHoursDial.value
        restoreMode: Binding.RestoreBindingOrValue
    }

    // -------------------------
    // backup action
    Timer {
        id: backupTimer
        repeat: true
        running: backUpEveryCheckBox.checked
        interval: backupHoursSpinBox.value * 60 * 60 * 1000
        onTriggered: {
            backUpAction.trigger()
        }
    }

    Connections {
        target: skrData.projectHub()
        function onProjectLoaded(projectId) {
            backUpOnceADayIfNeeded()
        }
    }

    // once a day :
    function backUpOnceADayIfNeeded() {
        if (!backUpOnceADayCheckBox.checked) {
            return
        }
        var backupPaths = SkrSettings.backupSettings.paths
        var backupPathList = backupPaths.split(";")

        //no backup path set
        if (backupPaths === "") {

            //TODO: send notification, backup not configured
            return
        }

        var projectIdList = skrData.projectHub().getProjectIdList()
        var projectCount = skrData.projectHub().getProjectCount()

        // all projects :
        var i
        for (i = 0; i < projectCount; i++) {
            var projectId = projectIdList[i]

            //no project path
            if (skrData.projectHub().getPath(projectId) === "") {

                //TODO: send notification, project not yet saved once
                break
            }

            // in all backup paths :
            var j
            for (j = 0; j < backupPathList.length; j++) {
                var path = backupPathList[j]

                if (path === "") {
                    //TODO: send notification
                    continue
                }

                // check if wanted backup exists already at paths
                var isBackupThere = skrData.projectHub(
                            ).doesBackupOfTheDayExistAtPath(
                            projectId, skrQMLTools.getURLFromLocalFile(path))

                if (isBackupThere) {
                    break
                }

                // back up :
                var result = skrData.projectHub().backupAProject(
                            projectId, "skrib",
                            skrQMLTools.getURLFromLocalFile(path))

                if (result.containsErrorCodeDetail("path_is_readonly")) {

                    //TODO: notification
                }
            }
        }
    }

    //--------------------------------------------------
    onActiveFocusChanged: {
        if (activeFocus) {
            backupGroupBox.forceActiveFocus()
        }
    }
}
