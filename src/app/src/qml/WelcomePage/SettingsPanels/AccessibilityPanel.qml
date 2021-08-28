import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import Qt.labs.platform 1.1 as LabPlatform
import Qt.labs.settings 1.1
import eu.skribisto.projecthub 1.0
import eu.skribisto.spellchecker 1.0
import QtQuick.Controls.Material 2.15
import "../.."
import "../../Items"
import "../../Commons"

AccessibilityPanelForm {

    // --------------------------------------------
    // ---- accessibility --------------------------------
    // --------------------------------------------

    accessibilityCheckBox.checked: SkrSettings.accessibilitySettings.accessibilityEnabled
    Binding {
        target: SkrSettings.accessibilitySettings
        property: "accessibilityEnabled"
        value: accessibilityCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    accessibilityCheckBox.onCheckedChanged: {
        showMenuButtonCheckBox.checked = accessibilityCheckBox.checked
    }

    showMenuButtonCheckBox.checked: SkrSettings.accessibilitySettings.showMenuButton
    Binding {
        target: SkrSettings.accessibilitySettings
        property: "showMenuButton"
        value: showMenuButtonCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    //--------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {
            accessibilityGroupBox.forceActiveFocus()
        }
    }

}
