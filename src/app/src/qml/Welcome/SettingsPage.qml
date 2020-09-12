import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import ".."

SettingsPageForm {

    Component.onCompleted: {
        checkOnBackUpEveryCheckBox()
        checkOnSaveEveryCheckBox()
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
    }


    // --------------------------------------------
    // ---- accessibility --------------------------------
    // --------------------------------------------

    disallowSwipeBetweenTabsCheckBox.checked: SkrSettings.accessibilitySettings.disallowSwipeBetweenTabsCheckBoxChecked
    Binding {
        target: SkrSettings.accessibilitySettings
        property: "disallowSwipeBetweenTabsCheckBoxChecked"
        value: disallowSwipeBetweenTabsCheckBox.checked
    }

    showMenuBarCheckBox.checked: SkrSettings.accessibilitySettings.showMenuBar
    Binding {
        target: SkrSettings.accessibilitySettings
        property: "showMenuBar"
        value: showMenuBarCheckBox.checked
    }

    // --------------------------------------------
    // ---- backup --------------------------------
    // --------------------------------------------


    backUpEveryCheckBox.checked: SkrSettings.backupSettings.backUpEveryCheckBoxChecked
    Binding {
        target: SkrSettings.backupSettings
        property: "backUpEveryCheckBoxChecked"
        value: backUpEveryCheckBox.checked
    }

    backUpEveryCheckBox.onCheckedChanged:{
        checkOnBackUpEveryCheckBox()
    }

    function checkOnBackUpEveryCheckBox(){
        backupHoursDial.enabled = backUpEveryCheckBox.checked
    backupHoursSpinBox.enabled = backUpEveryCheckBox.checked

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

    }
}
