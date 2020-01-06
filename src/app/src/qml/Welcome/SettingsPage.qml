import QtQuick 2.4
import ".."

SettingsPageForm {

    testSwitch.checked: SkrSettings.interfaceSettings.menuButtonsInStatusBar
    Binding {
        target: SkrSettings.interfaceSettings
        property: "menuButtonsInStatusBar"
        value: testSwitch.checked
    }
}
