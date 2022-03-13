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

EPaperPanelForm {


    Component.onCompleted: {

    }

    // --------------------------------------------
    // ---- special E-Paper --------------------------------
    // --------------------------------------------

    setTextCursorUnblinkingCheckBox.checked: SkrSettings.ePaperSettings.textCursorUnblinking
    Binding {
        target: SkrSettings.ePaperSettings
        property: "textCursorUnblinking"
        value: setTextCursorUnblinkingCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }


    animationEnabledCheckBox.checked: SkrSettings.ePaperSettings.animationEnabled
    Binding {
        target: SkrSettings.ePaperSettings
        property: "animationEnabled"
        value: animationEnabledCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }


    //--------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {
            specialEPaperGroupBox.forceActiveFocus()
        }
    }
}
