import QtQuick 2.4
import ".."

SettingsPageForm {

    menuButtonsInStatusBarSwitch.checked: SkrSettings.interfaceSettings.menuButtonsInStatusBar
    Binding {
        target: SkrSettings.interfaceSettings
        property: "menuButtonsInStatusBar"
        value: menuButtonsInStatusBarSwitch.checked
    }
}
