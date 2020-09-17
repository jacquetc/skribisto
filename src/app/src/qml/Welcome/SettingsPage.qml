import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import eu.skribisto.projecthub 1.0
import ".."

SettingsPageForm {

    Component.onCompleted: {
        checkOnBackUpEveryCheckBox()
        checkOnSaveEveryCheckBox()
        backUpOnceADayIfNeeded()
    }


    //    // scrollview :
    //    Component.onCompleted: scroll.contentItem = contener
    //	function scrollChange(){
    //                if ((width > 600) && (height > 600)){
    //                        if(scroll.contentItem != itemNull) scroll.contentItem = itemNull
    //                        contener.width = width
    //                        contener.height = height
    //                } else if ((width < 600) && (height < 600)){
    //                        if(scroll.contentItem != contener) scroll.contentItem = contener
    //                        contener.width = 600
    //                        contener.height = 600
    //                } else if ((width < 600)){
    //                        if(scroll.contentItem != contener) scroll.contentItem = contener
    //                        contener.width = 600
    //                        contener.height = height
    //                } else {
    //                        if(scroll.contentItem != contener) scroll.contentItem = contener
    //                        contener.height = 600
    //                        contener.width = width
    //                }
    //        }

    //	onWidthChanged: scrollChange()
    //	onHeightChanged: scrollChange()




    menuButtonsInStatusBarSwitch.checked: SkrSettings.interfaceSettings.menuButtonsInStatusBar
    Binding {
        target: SkrSettings.interfaceSettings
        property: "menuButtonsInStatusBar"
        value: menuButtonsInStatusBarSwitch.checked
        restoreMode: Qt.Binding.RestoreBindingOrValue
    }


    // --------------------------------------------
    // ---- accessibility --------------------------------
    // --------------------------------------------

    disallowSwipeBetweenTabsCheckBox.checked: SkrSettings.accessibilitySettings.disallowSwipeBetweenTabsCheckBoxChecked
    Binding {
        target: SkrSettings.accessibilitySettings
        property: "disallowSwipeBetweenTabsCheckBoxChecked"
        value: disallowSwipeBetweenTabsCheckBox.checked
        restoreMode: Qt.Binding.RestoreBindingOrValue
    }

    showMenuBarCheckBox.checked: SkrSettings.accessibilitySettings.showMenuBar
    Binding {
        target: SkrSettings.accessibilitySettings
        property: "showMenuBar"
        value: showMenuBarCheckBox.checked
        restoreMode: Qt.Binding.RestoreBindingOrValue
    }

    // --------------------------------------------
    // ---- backup --------------------------------
    // --------------------------------------------


    backUpEveryCheckBox.checked: SkrSettings.backupSettings.backUpEveryCheckBoxChecked
    Binding {
        target: SkrSettings.backupSettings
        property: "backUpEveryCheckBoxChecked"
        value: backUpEveryCheckBox.checked
        restoreMode: Qt.Binding.RestoreBindingOrValue
    }

    backUpEveryCheckBox.onCheckedChanged:{
        checkOnBackUpEveryCheckBox()
    }

    function checkOnBackUpEveryCheckBox(){
        backupHoursDial.enabled = backUpEveryCheckBox.checked
        backupHoursSpinBox.enabled = backUpEveryCheckBox.checked

    }

    //backUp Once A Day CheckBox
    backUpOnceADayCheckBox.checked: SkrSettings.backupSettings.backUpOnceADay
    Binding {
        target: SkrSettings.backupSettings
        property: "backUpOnceADay"
        value: backUpOnceADayCheckBox.checked
        restoreMode: Qt.Binding.RestoreBindingOrValue
    }

    backUpOnceADayCheckBox.onCheckedChanged:{
        if(backUpOnceADayCheckBox.checked){
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
        restoreMode: Qt.Binding.RestoreBindingOrValue

    }

    // -------------------------
    // backup action


    Timer{
        id: backupTimer
        repeat: true
        running: backUpEveryCheckBox.checked
        interval: backupHoursSpinBox.value * 60 * 60 * 1000
        onTriggered: {
            backUpAction.trigger()
        }
    }



    // once a day :
    function backUpOnceADayIfNeeded(){
        if(!backUpOnceADayCheckBox.checked){
            return
        }
        var backupPaths = SkrSettings.backupSettings.paths
        var backupPathList = backupPaths.split(";")

        //no backup path set
        if (backupPaths === ""){
            //TODO: send notification, backup not configured

            return
        }

        var projectIdList = plmData.projectHub().getProjectIdList()
        var projectCount = plmData.projectHub().getProjectCount()


        // all projects :
        var i;
        for (i = 0; i < projectCount ; i++ ){
            var projectId = projectIdList[i]


            //no project path
            if (plmData.projectHub().getPath(projectId) === ""){
                //TODO: send notification, project not yet saved once

                break
            }

            // in all backup paths :
            var j;
            for (j = 0; j < backupPathList.length ; j++ ){
                var path = backupPathList[j]


                if (path === ""){
                    //TODO: send notification
                    continue
                }



                // check if wanted backup exists already at paths
                var isBackupThere = plmData.projectHub().doesBackupOfTheDayExistAtPath(projectId, path)

                if(isBackupThere){
                    break
                }

                // back up :

                var error = plmData.projectHub().backupAProject(projectId, "skrib", path)

                if (error.getErrorCode() === "E_PROJECT_path_is_readonly"){

                }

            }
        }
    }



    // --------------------------------------------
    // ---- save --------------------------------
    // --------------------------------------------


    saveEveryCheckBox.checked: SkrSettings.saveSettings.saveEveryCheckBoxChecked
    Binding {
        target: SkrSettings.saveSettings
        property: "saveEveryCheckBoxChecked"
        value: saveEveryCheckBox.checked
    }

    saveEveryCheckBox.onCheckedChanged:{
        checkOnSaveEveryCheckBox()
    }

    function checkOnSaveEveryCheckBox(){
        saveDial.enabled = saveEveryCheckBox.checked
        saveSpinBox.enabled = saveEveryCheckBox.checked

    }

    //dials :

    saveDial.onMoved: saveSpinBox.value = saveDial.value
    saveSpinBox.onValueModified: saveDial.value = saveSpinBox.value


    saveDial.value: SkrSettings.saveSettings.saveEveryMinutes
    saveSpinBox.value: SkrSettings.saveSettings.saveEveryMinutes
    Binding {
        delayed: true
        target: SkrSettings.saveSettings
        property: "saveEveryMinutes"
        value: saveDial.value
        restoreMode: Qt.Binding.RestoreBindingOrValue

    }


    // -------------------------
    // save action


    Timer{
        id: saveTimer
        repeat: true
        running: saveEveryCheckBox.checked
        interval: saveSpinBox.value * 60 * 1000
        onTriggered: {
            saveAction.trigger()
        }
    }


}
