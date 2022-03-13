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

SettingsForm {



    centerTextCursorSwitch.checked: SkrSettings.behaviorSettings.centerTextCursor
    Binding {
        target: SkrSettings.behaviorSettings
        property: "centerTextCursor"
        value: centerTextCursorSwitch.checked
        restoreMode: Binding.RestoreBindingOrValue
    }


    // -------------------------------------------------------
    // ---- minimap scrollbar --------------------------------
    // --------------------------------------------------------



    showMinimapCheckBox.checked: SkrSettings.minimapSettings.visible
    Binding {
        target: SkrSettings.minimapSettings
        property: "visible"
        value: showMinimapCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    //dials :

    minimapDividerDial.onMoved: minimapDividerSpinBox.value = minimapDividerDial.value
    minimapDividerSpinBox.onValueModified: minimapDividerDial.value = minimapDividerSpinBox.value


    minimapDividerDial.value: SkrSettings.minimapSettings.divider
    minimapDividerSpinBox.value: SkrSettings.minimapSettings.divider
    Binding {
        delayed: true
        target: SkrSettings.minimapSettings
        property: "divider"
        value: minimapDividerDial.value
        restoreMode: Binding.RestoreBindingOrValue

    }


    //--------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {
            textGroupBox.forceActiveFocus()
        }
    }

}
