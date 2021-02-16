pragma Singleton

import QtQuick 2.15
import eu.skribisto.themes 1.0
import Qt.labs.settings 1.1

QtObject {
    property string selectedThemeName: ""
    property string currentLightThemeName: skrThemes.currentLightTheme
    property string currentDarkThemeName: skrThemes.currentDarkTheme
    property string currentDistractionFreeThemeName: skrThemes.currentDistractionFreeTheme

    readonly property bool distractionFree: Globals.fullScreen
    property bool light: skrThemes.currentColorMode === SKRThemes.Light ? true : false

    onLightChanged: {
        determineCurrentColorMode()
    }

    onDistractionFreeChanged: {
        determineCurrentColorMode()
    }

    readonly property string mainTextAreaBackground: skrThemes.mainTextAreaBackground
    readonly property string mainTextAreaForeground: skrThemes.mainTextAreaForeground

    readonly property string secondaryTextAreaBackground:  skrThemes.secondaryTextAreaBackground
    readonly property string secondaryTextAreaForeground:  skrThemes.secondaryTextAreaForeground

    readonly property string pageBackground: skrThemes.pageBackground

    readonly property string buttonBackground: skrThemes.buttonBackground
    readonly property string buttonForeground:  skrThemes.buttonForeground
    readonly property string buttonIcon: skrThemes.buttonIcon
    readonly property string buttonIconDisabled: skrThemes.buttonIconDisabled

    readonly property string accent:  skrThemes.accent

    readonly property string spellcheck: skrThemes.spellcheck
    readonly property string toolBarBackground: skrThemes.toolBarBackground
    readonly property string pageToolBarBackground: skrThemes.pageToolBarBackground


    readonly property string divider: skrThemes.divider
    readonly property string menuBackground: skrThemes.menuBackground

    readonly  property string listItemBackground: skrThemes.listItemBackground

    property var skrThemes: SKRThemes {
        id: skrThemes
    }


    //------------------------------------------------------------------

    function determineCurrentColorMode(){


        // set theme color
        if(distractionFree){
            skrThemes.currentColorMode = SKRThemes.DistractionFree
        }
        else {
            if(light){
                skrThemes.currentColorMode = SKRThemes.Light
            }
            else{
                skrThemes.currentColorMode = SKRThemes.Dark
            }
        }

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


    onSelectedThemeNameChanged: {
        skrThemes.selectedTheme = selectedThemeName
    }

    function getColorPropertyValues(){
        var list = []

        list.push(skrThemes.mainTextAreaBackground)
        list.push(skrThemes.mainTextAreaForeground)
        list.push(skrThemes.secondaryTextAreaBackground)
        list.push(skrThemes.secondaryTextAreaForeground)
        list.push(skrThemes.pageBackground)
        list.push(skrThemes.buttonBackground)
        list.push(skrThemes.buttonForeground)
        list.push(skrThemes.buttonIcon)
        list.push(skrThemes.buttonIconDisabled)
        list.push(skrThemes.accent)
        list.push(skrThemes.spellcheck)
        list.push(skrThemes.toolBarBackground)
        list.push(skrThemes.pageToolBarBackground)
        list.push(skrThemes.divider)
        list.push(skrThemes.menuBackground)
        list.push(skrThemes.listItemBackground)

        return list
    }
    //-----------------------------------------------------------------
    function getColorPropertyExactNames(){
        var list = []

        list.push("mainTextAreaBackground")
        list.push("mainTextAreaForeground")
        list.push("secondaryTextAreaBackground")
        list.push("secondaryTextAreaForeground")
        list.push("pageBackground")
        list.push("buttonBackground")
        list.push("buttonForeground")
        list.push("buttonIcon")
        list.push("buttonIconDisabled")
        list.push("accent")
        list.push("spellcheck")
        list.push("toolBarBackground")
        list.push("pageToolBarBackground")
        list.push("divider")
        list.push("menuBackground")
        list.push("listItemBackground")

        return list
    }

    //-----------------------------------------------------------------

    function getPropertyHumanNames(){
        var list = []

        list.push(qsTr("Main text background"))
        list.push(qsTr("Main text foreground"))
        list.push(qsTr("Secondary text background"))
        list.push(qsTr("Secondary text foreground"))
        list.push(qsTr("Page background"))
        list.push(qsTr("Button background"))
        list.push(qsTr("Button foreground"))
        list.push(qsTr("Button icon"))
        list.push(qsTr("Button icon (disabled)"))
        list.push(qsTr("Accent"))
        list.push(qsTr("Spellcheck"))
        list.push(qsTr("ToolBar background"))
        list.push(qsTr("Page ToolBar background"))
        list.push(qsTr("Divider"))
        list.push(qsTr("Menu background"))
        list.push(qsTr("List item background"))

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
        case "mainTextAreaForeground":
            skrThemes.mainTextAreaForeground = color
            break;
        case "secondaryTextAreaBackground":
            skrThemes.secondaryTextAreaBackground = color
            break;
        case "secondaryTextAreaForeground":
            skrThemes.secondaryTextAreaForeground = color
            break;
        case "pageBackground":
            skrThemes.pageBackground = color
            break;
        case "buttonBackground":
            skrThemes.buttonBackground = color
            break;
        case "buttonForeground":
            skrThemes.buttonForeground = color
            break;
        case "buttonIcon":
            skrThemes.buttonIcon = color
            break;
        case "buttonIconDisabled":
            skrThemes.buttonIconDisabled = color
            break;
        case "accent":
            skrThemes.accent = color
            break;
        case "spellcheck":
            skrThemes.spellcheck = color
            break;
        case "toolBarBackground":
            skrThemes.toolBarBackground = color
            break;
        case "pageToolBarBackground":
            skrThemes.pageToolBarBackground = color
            break;
        case "divider":
            skrThemes.divider = color
            break;
        case "menuBackground":
            skrThemes.menuBackground = color
            break;
        case "listItemBackground":
            skrThemes.listItemBackground = color
            break;
        default:
            console.exception(propertyExactName, "doesn't exist")
        }

        if(skrThemes.isThemeEditable(currentThemeName))
            skrThemes.saveTheme(currentThemeName)

    }

    //-----------------------------------------------------------------


}
