pragma Singleton
import QtQuick 2.15
import Qt.labs.settings 1.1

QtObject {

    property Settings interfaceSettings: Settings{
        category: "interface"
        property bool menuButtonsInStatusBar: false
        onMenuButtonsInStatusBarChanged: {
            console.log("menuButtonsInStatusBar", menuButtonsInStatusBar)
        }

    }

    property Settings backupSettings: Settings{
        category: "backup"
        property string paths: ""
        property bool backUpEveryCheckBoxChecked: false
        property int backUpEveryHours: 10
        property bool backUpOnceADay: false

    }

    property Settings saveSettings: Settings{
        category: "save"
        property string paths: ""
        property bool saveEveryCheckBoxChecked: false
        property int saveEveryMinutes: 10

    }
    property Settings accessibilitySettings: Settings{
        category: "accessibility"
        property bool allowSwipeBetweenTabs: false
        property bool showMenuBar: false

    }

    property Settings ePaperSettings: Settings{
        category: "ePaper"
        property bool textCursorUnblinking: false

    }

    property Settings welcomeSettings: Settings{
        category: "welcome"
        property bool createEmptyProjectAtStart: false

    }

    property Settings writeSettings: Settings{
        category: "write"
        property int textWidth: initialTextWidth
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property string textFontFamily: Qt.application.font.family
    }

    property Settings noteSettings: Settings{
        category: "note"
        property int textWidth: initialTextWidth
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property string textFontFamily: Qt.application.font.family
    }

    property Settings notePadSettings: Settings{
        category: "notePad"
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property string textFontFamily: Qt.application.font.family
    }

    property int initialTextWidth: 0
    Component.onCompleted: {
        initialTextWidth = Globals.width / 3
    }
}
