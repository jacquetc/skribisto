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

AdvancedPanelForm {


    // --------------------------------------------
    // ---- advanced --------------------------------
    // --------------------------------------------

    devModeCheckBox.checked: SkrSettings.devSettings.devModeEnabled
    Binding {
        target: SkrSettings.devSettings
        property: "devModeEnabled"
        value: devModeCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    //--------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {
            advancedGroupBox.forceActiveFocus()
        }
    }
}
