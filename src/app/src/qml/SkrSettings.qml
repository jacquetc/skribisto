pragma Singleton

import QtQuick
import QtQuick.Controls
import Qt.labs.settings 1.1

QtObject {

    property Settings interfaceSettings: Settings {
        category: "interface"
        property bool firstLaunch: true
    }

    property Settings backupSettings: Settings {
        category: "backup"
        property string paths: ""
        property bool backUpEveryCheckBoxChecked: false
        property int backUpEveryHours: 10
        property bool backUpOnceADay: false
    }

    property Settings saveSettings: Settings {
        category: "save"
        property string paths: ""
        property bool saveEveryCheckBoxChecked: false
        property int saveEveryMinutes: 10
    }
    property Settings accessibilitySettings: Settings {
        category: "accessibility"
        property bool accessibilityEnabled: false
        property bool showMenuButton: false
    }

    property Settings ePaperSettings: Settings {
        category: "ePaper"
        property bool textCursorUnblinking: false
        property bool animationEnabled: true
    }

    property Settings behaviorSettings: Settings {
        category: "behavior"
        property bool createEmptyProjectAtStart: false
        property bool centerTextCursor: false
    }

    property Settings spellCheckingSettings: Settings {
        category: "spellChecking"
        property bool spellCheckingActivation: true
        property string spellCheckingLangCode: "en_US"
    }

    property Settings textSettings: Settings {
        category: "text"
        property int textWidth: 500
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property string textFontFamily: skrRootItem.defaultFontFamily()
    }

    property Settings minimapSettings: Settings {
        category: "minimap"
        property int divider: 5
        property bool visible: true
    }

    property Settings outlinePadSettings: Settings {
        category: "outlinePad"
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property string textFontFamily: skrRootItem.defaultFontFamily()
    }

    property Settings relatedTextSettings: Settings {
        category: "relatedText"
        property int textWidth: -2 //unused but mandatory
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property string textFontFamily: skrRootItem.defaultFontFamily()
    }

    property Settings notePadSettings: Settings {
        category: "notePad"
        property int textWidth: -2 //unused but mandatory
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property string textFontFamily: skrRootItem.defaultFontFamily()
    }

    property Settings cardViewOutlineSettings: Settings {
        category: "cardViewOutline"
        property int textWidth: -2 //unused but mandatory
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 1
        property string textFontFamily: skrRootItem.defaultFontFamily()
    }

    property Settings cardViewSettings: Settings {
        category: "cardView"
        property double cardSizeMultiplier: 1.0
        property bool outlineBoxVisible: true
        property bool tagBoxVisible: true

    }


    property Settings overviewTreeOutlineSettings: Settings {
        category: "overviewTreeOutline"
        property int textWidth: -2 //unused but mandatory
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 1
        property string textFontFamily: skrRootItem.defaultFontFamily()
    }

    property Settings overviewTreeSettings: Settings {
        category: "overviewTree"
        property int treeItemDisplayMode: 1
        property int treeIndentation: 30
        property bool outlineBoxVisible: true
        property bool noteBoxVisible: true
        property bool tagBoxVisible: true
        property bool characterCountBoxVisible: true
        property bool wordCountBoxVisible: true
    }

    property Settings quickPrintSettings: Settings {
        category: "quickPrint"
        property string textFontFamily: skrRootItem.defaultFontFamily()
        property int textPointSize: Qt.application.font.pointSize
        property real textIndent: 2
        property real textTopMargin: 2
        property bool includeSynopsis: false
        property bool tagsEnabled: false
    }

    property Settings relationshipPanelSettings: Settings {
        category: "relationshipPanel"
        property string viewMode: "notes"
    }

    property Settings devSettings: Settings {
        category: "dev"
        property bool devModeEnabled: false
    }

    property int initialTextWidth: 0
}
