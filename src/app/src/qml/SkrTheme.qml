pragma Singleton

import QtQuick 2.15
import eu.skribisto.themes 1.0
import Qt.labs.settings 1.1

QtObject {
    property string currentThemeName: ""

    readonly property bool distractionFree: Globals.fullScreen
    readonly property var mainTextAreaBackground: Globals.fullScreen ? skrThemes.distractionFree_mainTextAreaBackground : skrThemes.mainTextAreaBackground
    readonly property var mainTextAreaForeground: Globals.fullScreen ? skrThemes.distractionFree_mainTextAreaForeground : skrThemes.mainTextAreaForeground
    readonly property var secondaryTextAreaBackground:  Globals.fullScreen ? skrThemes.distractionFree_secondaryTextAreaBackground : skrThemes.secondaryTextAreaBackground
    readonly property var secondaryTextAreaForeground:  Globals.fullScreen ? skrThemes.distractionFree_secondaryTextAreaForeground : skrThemes.secondaryTextAreaForeground
    readonly property var pageBackground:  Globals.fullScreen ? skrThemes.distractionFree_pageBackground : skrThemes.pageBackground
    readonly property var buttonBackground:  Globals.fullScreen ? skrThemes.distractionFree_buttonBackground : skrThemes.buttonBackground
    readonly property var buttonForeground:  Globals.fullScreen ? skrThemes.distractionFree_buttonForeground : skrThemes.buttonForeground
    readonly property var buttonIcon:  Globals.fullScreen ?  skrThemes.distractionFree_buttonIcon : skrThemes.buttonIcon
    readonly property var accent:  Globals.fullScreen ? skrThemes.distractionFree_accent : skrThemes.accent
    readonly property var spellcheck:  Globals.fullScreen ? skrThemes.distractionFree_spellcheck : skrThemes.spellcheck
    readonly property var toolBarBackground:  Globals.fullScreen ? skrThemes.distractionFree_toolBarBackground : skrThemes.toolBarBackground
    readonly property var divider:  Globals.fullScreen ? skrThemes.distractionFree_divider : skrThemes.divider
    readonly property var menuBackground:  Globals.fullScreen ? skrThemes.distractionFree_menuBackground : skrThemes.menuBackground

    property var skrThemes: SKRThemes {
        id: skrThemes
    }

    property var settings: Settings{
        id: settings
        category: "theme"
        property string themeName: skrThemes.defaultTheme()
    }

    //------------------------------------------------------------------


    function determineCurrentTheme(){


        var themeName= settings.themeName

        // verify if it exists
        if(!skrThemes.doesThemeExist(themeName)){
            themeName = skrThemes.defaultTheme()
        }

        currentThemeName = themeName

        return currentThemeName
    }

    //------------------------------------------------------------------

    //------------------------------------------------------------------

    function isThemeEditable(themeName){
        return skrThemes.isThemeEditable(themeName)
    }
    //------------------------------------------------------------------

    function getThemeList(){
        return skrThemes.getThemeList()
    }

    //------------------------------------------------------------------

    Component.onCompleted: {
        determineCurrentTheme()
    }

    onCurrentThemeNameChanged: {
        skrThemes.currentTheme = currentThemeName
        settings.themeName = currentThemeName
    }

    function getColorPropertyValues(){
        var list = []

        list.push(skrThemes.mainTextAreaBackground)
        list.push(skrThemes.distractionFree_mainTextAreaBackground)
        list.push(skrThemes.mainTextAreaForeground)
        list.push(skrThemes.distractionFree_mainTextAreaForeground)
        list.push(skrThemes.secondaryTextAreaBackground)
        list.push(skrThemes.distractionFree_secondaryTextAreaBackground)
        list.push(skrThemes.secondaryTextAreaForeground)
        list.push(skrThemes.distractionFree_secondaryTextAreaForeground)
        list.push(skrThemes.pageBackground)
        list.push(skrThemes.distractionFree_pageBackground)
        list.push(skrThemes.buttonBackground)
        list.push(skrThemes.distractionFree_buttonBackground)
        list.push(skrThemes.buttonForeground)
        list.push(skrThemes.distractionFree_buttonForeground)
        list.push(skrThemes.buttonIcon)
        list.push(skrThemes.distractionFree_buttonIcon)
        list.push(skrThemes.accent)
        list.push(skrThemes.distractionFree_accent)
        list.push(skrThemes.spellcheck)
        list.push(skrThemes.distractionFree_spellcheck)
        list.push(skrThemes.toolBarBackground)
        list.push(skrThemes.distractionFree_toolBarBackground)
        list.push(skrThemes.divider)
        list.push(skrThemes.distractionFree_divider)
        list.push(skrThemes.menuBackground)
        list.push(skrThemes.distractionFree_menuBackground)

        return list
    }
    //-----------------------------------------------------------------
    function getColorPropertyExactNames(){
        var list = []

        list.push("mainTextAreaBackground")
        list.push("distractionFree_mainTextAreaBackground")
        list.push("mainTextAreaForeground")
        list.push("distractionFree_mainTextAreaForeground")
        list.push("secondaryTextAreaBackground")
        list.push("distractionFree_secondaryTextAreaBackground")
        list.push("secondaryTextAreaForeground")
        list.push("distractionFree_secondaryTextAreaForeground")
        list.push("pageBackground")
        list.push("distractionFree_pageBackground")
        list.push("buttonBackground")
        list.push("distractionFree_buttonBackground")
        list.push("buttonForeground")
        list.push("distractionFree_buttonForeground")
        list.push("buttonIcon")
        list.push("distractionFree_buttonIcon")
        list.push("accent")
        list.push("distractionFree_accent")
        list.push("spellcheck")
        list.push("distractionFree_spellcheck")
        list.push("toolBarBackground")
        list.push("distractionFree_toolBarBackground")
        list.push("divider")
        list.push("distractionFree_divider")
        list.push("menuBackground")
        list.push("distractionFree_menuBackground")

        return list
    }

    //-----------------------------------------------------------------

    function getPropertyHumanNames(){
        var list = []

        list.push(qsTr("Main text background"))
        list.push(qsTr("Main text background"))
        list.push(qsTr("Main text foreground"))
        list.push(qsTr("Main text foreground"))
        list.push(qsTr("Secondary text background"))
        list.push(qsTr("Secondary text background"))
        list.push(qsTr("Secondary text foreground"))
        list.push(qsTr("Secondary text foreground"))
        list.push(qsTr("Page background"))
        list.push(qsTr("Page background"))
        list.push(qsTr("Button background"))
        list.push(qsTr("Button background"))
        list.push(qsTr("Button foreground"))
        list.push(qsTr("Button foreground"))
        list.push(qsTr("Button icon"))
        list.push(qsTr("Button icon"))
        list.push(qsTr("Accent"))
        list.push(qsTr("Accent"))
        list.push(qsTr("Spellcheck"))
        list.push(qsTr("Spellcheck"))
        list.push(qsTr("ToolBar background"))
        list.push(qsTr("ToolBar background"))
        list.push(qsTr("Divider"))
        list.push(qsTr("Divider"))
        list.push(qsTr("Menu background"))
        list.push(qsTr("Menu background"))

        return list

    }

    //-----------------------------------------------------------------
    function duplicate(themeName, newThemeName){

        return  skrThemes.duplicate(themeName, newThemeName)

    }

    //-----------------------------------------------------------------
    function remove(themeName){
        if(!isThemeEditable(themeName)){
            return
        }
        var value = skrThemes.remove(themeName)

        if(value){
            currentThemeName = skrThemes.currentTheme
        }

        return  value


    }

    //-----------------------------------------------------------------
    function setColorProperty(propertyExactName, color){

        switch(propertyExactName) {
        case "mainTextAreaBackground":
            skrThemes.mainTextAreaBackground = color
            break;
        case "distractionFree_mainTextAreaBackground":
            skrThemes.distractionFree_mainTextAreaBackground = color
            break;
        case "mainTextAreaForeground":
            skrThemes.mainTextAreaForeground = color
            break;
        case "distractionFree_mainTextAreaForeground":
            skrThemes.distractionFree_mainTextAreaBackground = color
            break;
        case "secondaryTextAreaBackground":
            skrThemes.secondaryTextAreaBackground = color
            break;
        case "distractionFree_secondaryTextAreaBackground":
            skrThemes.distractionFree_secondaryTextAreaBackground = color
            break;
        case "secondaryTextAreaForeground":
            skrThemes.secondaryTextAreaForeground = color
            break;
        case "distractionFree_secondaryTextAreaForeground":
            skrThemes.distractionFree_secondaryTextAreaForeground = color
            break;
        case "pageBackground":
            skrThemes.pageBackground = color
            break;
        case "distractionFree_pageBackground":
            skrThemes.distractionFree_pageBackground = color
            break;
        case "buttonBackground":
            skrThemes.buttonBackground = color
            break;
        case "distractionFree_buttonBackground":
            skrThemes.distractionFree_buttonBackground = color
            break;
        case "buttonForeground":
            skrThemes.buttonForeground = color
            break;
        case "distractionFree_buttonForeground":
            skrThemes.distractionFree_buttonForeground = color
            break;
        case "buttonIcon":
            skrThemes.buttonIcon = color
            break;
        case "distractionFree_buttonIcon":
            skrThemes.distractionFree_buttonIcon = color
            break;
        case "accent":
            skrThemes.accent = color
            break;
        case "distractionFree_accent":
            skrThemes.distractionFree_accent = color
            break;
        case "spellcheck":
            skrThemes.spellcheck = color
            break;
        case "distractionFree_spellcheck":
            skrThemes.distractionFree_spellcheck = color
            break;
        case "toolBarBackground":
            skrThemes.toolBarBackground = color
            break;
        case "distractionFree_toolBarBackground":
            skrThemes.distractionFree_toolBarBackground = color
            break;
        case "divider":
            skrThemes.divider = color
            break;
        case "distractionFree_divider":
            skrThemes.distractionFree_divider = color
            break;
        case "menuBackground":
            skrThemes.menuBackground = color
            break;
        case "distractionFree_menuBackground":
            skrThemes.distractionFree_menuBackground = color
            break;
        default:
            console.exception(propertyExactName, "doesn't exist")
        }

        if(skrThemes.isThemeEditable(currentThemeName))
            skrThemes.saveTheme(currentThemeName)

    }

    //-----------------------------------------------------------------


}
