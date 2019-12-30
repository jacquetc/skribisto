pragma Singleton
import QtQuick 2.12
import Qt.labs.settings 1.1

QtObject {

    property Settings interfaceSettings: Settings{
        category: "interface"
        property bool menuButtonsInStatusBar: false
        onMenuButtonsInStatusBarChanged: {
            console.log("menuButtonsInStatusBar", menuButtonsInStatusBar)
        }
    }



}
