import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts
import Qt.labs.platform 1.1 as LabPlatform
import Qt.labs.settings 1.1
import eu.skribisto.projecthub 1.0
import eu.skribisto.spellchecker 1.0
import QtQuick.Controls.Material
import "../.."
import "../../Items"
import "../../Commons"

SavePanelForm {

    Component.onCompleted: {
        checkOnSaveEveryCheckBox()
    }

    // ------------------------------------------------------
    // ---- save -------------------------------------------
    // -------------------------------------------------------


    saveEveryCheckBox.checked: SkrSettings.saveSettings.saveEveryCheckBoxChecked
    Binding {
        target: SkrSettings.saveSettings
        property: "saveEveryCheckBoxChecked"
        value: saveEveryCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
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
        restoreMode: Binding.RestoreBindingOrValue

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

    //--------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {
            saveGroupBox.forceActiveFocus()
        }
    }

}

