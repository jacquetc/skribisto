pragma Singleton

import QtQuick
import eu.skribisto.themes 1.0
import Qt.labs.settings 1.1

QtObject {
    property alias selectedThemeName: skrThemes.selectedTheme
    property alias selectedColorsMap: skrThemes.selectedColorsMap
    property alias currentLightThemeName: skrThemes.currentLightTheme
    property alias currentDarkThemeName: skrThemes.currentDarkTheme
    property alias currentDistractionFreeThemeName: skrThemes.currentDistractionFreeTheme

    readonly property bool distractionFree: skrThemes.currentColorMode === SKRThemes.DistractionFree
    readonly property bool light: skrThemes.currentColorMode === SKRThemes.Light


    property bool wasLight: true


    function forceCurrentColorMode(colorMode){
        skrThemes.currentColorMode = colorMode
    }

    function setLight() {
        if(skrThemes.currentColorMode !== SKRThemes.DistractionFree){
            wasLight = true
            skrThemes.currentColorMode = SKRThemes.Light
        }
    }


    function setDark() {
        if(skrThemes.currentColorMode !== SKRThemes.DistractionFree){
            wasLight = false
            skrThemes.currentColorMode = SKRThemes.Dark
        }
    }

    function setDistractionFree(value) {

        if(value){
            skrThemes.currentColorMode = SKRThemes.DistractionFree
        }
        else {
            if(wasLight){
                skrThemes.currentColorMode = SKRThemes.Light
            }
            else{
                skrThemes.currentColorMode = SKRThemes.Dark
            }
        }
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

    readonly property string spellCheckHighlight: skrThemes.spellCheckHighlight
    readonly property string findHighlight: skrThemes.findHighlight
    readonly property string otherHighlight_1: skrThemes.otherHighlight_1
    readonly property string otherHighlight_2: skrThemes.otherHighlight_2
    readonly property string otherHighlight_3: skrThemes.otherHighlight_3

    readonly property string minimapSpellCheckHighlight: skrThemes.minimapSpellCheckHighlight
    readonly property string minimapFindHighlight: skrThemes.minimapFindHighlight
    readonly property string minimapOtherHighlight_1: skrThemes.minimapOtherHighlight_1
    readonly property string minimapOtherHighlight_2: skrThemes.minimapOtherHighlight_2
    readonly property string minimapOtherHighlight_3: skrThemes.minimapOtherHighlight_3


    readonly property string toolBarBackground: skrThemes.toolBarBackground
    readonly property string pageToolBarBackground: skrThemes.pageToolBarBackground


    readonly property string divider: skrThemes.divider
    readonly property string menuBackground: skrThemes.menuBackground

    readonly  property string listItemBackground: skrThemes.listItemBackground

    property var skrThemes: SKRThemes {
        id: skrThemes
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


    //-----------------------------------------------------------------

    function getPropertyHumanNames(){
        var map = ({})

        map["mainTextAreaBackground"] = qsTr("Main text background")
        map["mainTextAreaForeground"] = qsTr("Main text foreground")
        map["secondaryTextAreaBackground"] = qsTr("Secondary text background")
        map["secondaryTextAreaForeground"] = qsTr("Secondary text foreground")
        map["pageBackground"] = qsTr("Page background")
        map["buttonBackground"] = qsTr("Button background")
        map["buttonForeground"] = qsTr("Button foreground")
        map["buttonIcon"] = qsTr("Button icon")
        map["buttonIconDisabled"] = qsTr("Button icon (disabled)")
        map["accent"] = qsTr("Accent")
        map["spellCheckHighlight"] = qsTr("Spell check highlight")
        map["findHighlight"] = qsTr("Find highlight")
        map["otherHighlight_1"] = qsTr("Other highlight 1")
        map["otherHighlight_2"] = qsTr("Other highlight 2")
        map["otherHighlight_3"] = qsTr("Other highlight 2")
        map["minimapSpellCheckHighlight"] = qsTr("Minimap's spell check highlight")
        map["minimapFindHighlight"] = qsTr("Minimap's find highlight")
        map["minimapOtherHighlight_1"] = qsTr("Minimap's other highlight 1")
        map["minimapOtherHighlight_2"] = qsTr("Minimap's other highlight 2")
        map["minimapOtherHighlight_3"] = qsTr("Minimap's other highlight 3")
        map["toolBarBackground"] = qsTr("ToolBar background")
        map["pageToolBarBackground"] = qsTr("Page ToolBar background")
        map["divider"] = qsTr("Divider")
        map["menuBackground"] = qsTr("Menu background")
        map["listItemBackground"] = qsTr("List item background")

        return map

    }

    //-----------------------------------------------------------------
    function duplicate(themeName, newThemeName){

        return  skrThemes.duplicate(themeName, newThemeName)

    }

    //-----------------------------------------------------------------
    function rename(themeName, newThemeName){

        return  skrThemes.rename(themeName, newThemeName)

    }
    //-----------------------------------------------------------------
    function remove(themeName){
        if(!isThemeEditable(themeName)){
            return
        }
        var isRemoved = skrThemes.remove(themeName)

        if(isRemoved){
            selectedThemeName = skrThemes.selectedTheme
        }

        return  isRemoved


    }

    //-----------------------------------------------------------------

    property var changedColorsMap: ({})

    onSelectedThemeNameChanged: {
        changedColorsMap = ({})
    }

    function setColorProperty(propertyExactName, color){
        changedColorsMap[propertyExactName] = color
    }

    function resetColorProperties(){
        changedColorsMap = ({})
        skrThemes.resetSelectedThemeColors()
    }

    function saveColorProperties(){

        if(skrThemes.isThemeEditable(selectedThemeName))
            skrThemes.saveTheme(selectedThemeName, changedColorsMap)
    }

    //-----------------------------------------------------------------


}
