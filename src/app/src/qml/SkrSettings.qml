pragma Singleton
import QtQuick 2.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1

QtObject {

    property Settings interfaceSettings: Settings{
        category: "interface"
        property bool menuButtonsInStatusBar: false
        property bool minimalistMenuTabs: false

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

    property Settings behaviorSettings: Settings{
        category: "behavior"
        property bool createEmptyProjectAtStart: false
        property bool centerTextCursor: false
    }

    property Settings spellCheckingSettings: Settings{
        category: "spellChecking"
        property bool spellCheckingActivation: true
        property string spellCheckingLangCode: "en_US"
    }

    property Settings textSettings: Settings{
        category: "text"
        property int textWidth: 500
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property string textFontFamily: skrRootItem.defaultFontFamily()
    }

    property Settings outlineSettings: Settings{
        category: "outline"
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property string textFontFamily: skrRootItem.defaultFontFamily()
    }


    property Settings noteSettings: Settings{
        category: "note"
        property int textWidth: initialTextWidth
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property string textFontFamily: skrRootItem.defaultFontFamily()
    }

    property Settings notePadSettings: Settings{
        category: "notePad"
        property int textWidth: -2 //unused but mandatory
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property string textFontFamily: skrRootItem.defaultFontFamily()
    }

    property Settings overviewTreeNoteSettings: Settings{
        category: "overviewTreeNote"
        property int textWidth: -2 //unused but mandatory
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property string textFontFamily: skrRootItem.defaultFontFamily()
    }

    property Settings overviewTreeSettings: Settings{
        category: "overviewTree"
        property int treeItemDisplayMode: 1
        property int treeIndentation: 30
        property bool synopsisBoxVisible: true
        property bool noteBoxVisible: true
        property bool tagBoxVisible: true
        property bool characterCountBoxVisible: true
        property bool wordCountBoxVisible: true
    }

    property Settings quickPrintSettings: Settings{
        category: "quickPrint"
        property string textFontFamily: skrRootItem.defaultFontFamily()
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property bool includeSynopsis: false
        property bool tagsEnabled: false
    }


    property int initialTextWidth: 0

}
