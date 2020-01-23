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

    property Settings welcomeSettings: Settings{
        category: "welcome"
        property bool createEmptyProjectAtStart: false

    }

    property Settings writeSettings: Settings{
        category: "write"
        property int textWidth: initialTextWidth
        property int textPointSize: Qt.application.font.pointSize
        property string textFontFamily: Qt.application.font.family
    }

    property Settings noteSettings: Settings{
        category: "note"
        property int textWidth: initialTextWidth
        property int textPointSize: Qt.application.font.pointSize
        property string textFontFamily: Qt.application.font.family
    }

    property int initialTextWidth: 0
    Component.onCompleted: {
        initialTextWidth = Globals.width / 3
    }
}
