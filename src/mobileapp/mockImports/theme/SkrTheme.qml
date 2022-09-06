pragma Singleton

import QtQuick

QtObject {
    property string selectedThemeName: "mock"
//    property alias selectedColorsMap: skrThemes.selectedColorsMap
    property string currentLightThemeName: "mock"
    property string currentDarkThemeName: "mock"
    property string currentDistractionFreeThemeName: "mock"

    readonly property bool distractionFree: false
    readonly property bool light: true

    property bool wasLight: true


//    function forceCurrentColorMode(colorMode){
//        skrThemes.currentColorMode = colorMode
//    }

//    function setLight() {
//        if(skrThemes.currentColorMode !== SKRThemes.DistractionFree){
//            wasLight = true
//            skrThemes.currentColorMode = SKRThemes.Light
//        }
//    }


//    function setDark() {
//        if(skrThemes.currentColorMode !== SKRThemes.DistractionFree){
//            wasLight = false
//            skrThemes.currentColorMode = SKRThemes.Dark
//        }
//    }

//    function setDistractionFree(value) {

//        if(value){
//            skrThemes.currentColorMode = SKRThemes.DistractionFree
//        }
//        else {
//            if(wasLight){
//                skrThemes.currentColorMode = SKRThemes.Light
//            }
//            else{
//                skrThemes.currentColorMode = SKRThemes.Dark
//            }
//        }
//    }

    readonly property string mainTextAreaBackground: "#FFFFFF"
    readonly property string mainTextAreaForeground: "#000000"

    readonly property string secondaryTextAreaBackground:  "#FFFFFF"
    readonly property string secondaryTextAreaForeground:  "#000000"

    readonly property string pageBackground: "#FAFAFA"

    readonly property string buttonBackground: "#D6D7D7"
    readonly property string buttonForeground:  "#000000"
    readonly property string buttonIcon: "#000000"
   readonly property string buttonIconDisabled: "#D6D7D7"

    readonly property string accent:  "#8BC34A"

    readonly property string spellCheckHighlight: "#800000"
    readonly property string findHighlight: "#ffff00"
    readonly property string otherHighlight_1: ""
    readonly property string otherHighlight_2: ""
    readonly property string otherHighlight_3: ""

    readonly property string minimapSpellCheckHighlight: "#800000"
    readonly property string minimapFindHighlight: "#ffff00"
    readonly property string minimapOtherHighlight_1: "#8bc34a"
    readonly property string minimapOtherHighlight_2: ""
    readonly property string minimapOtherHighlight_3: ""


    readonly property string toolBarBackground: "#00BCD4"
    readonly property string pageToolBarBackground: "#e6e6e6"


    readonly property string divider: "#00BCD4"
    readonly property string menuBackground: "#EDEDED"

    readonly  property string listItemBackground: "#f5f5f5"

//    property var skrThemes: SKRThemes {
//        id: skrThemes
//    }


//    //------------------------------------------------------------------

//    //------------------------------------------------------------------

//    function isThemeEditable(themeName){
//        return skrThemes.isThemeEditable(themeName)
//    }
//    //------------------------------------------------------------------

//    function getThemeList(){
//        return skrThemes.getThemeList()
//    }

//    //------------------------------------------------------------------


//    //-----------------------------------------------------------------

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

//    //-----------------------------------------------------------------
//    function duplicate(themeName, newThemeName){

//        return  skrThemes.duplicate(themeName, newThemeName)

//    }

//    //-----------------------------------------------------------------
//    function rename(themeName, newThemeName){

//        return  skrThemes.rename(themeName, newThemeName)

//    }
//    //-----------------------------------------------------------------
//    function remove(themeName){
//        if(!isThemeEditable(themeName)){
//            return
//        }
//        var isRemoved = skrThemes.remove(themeName)

//        if(isRemoved){
//            selectedThemeName = skrThemes.selectedTheme
//        }

//        return  isRemoved


//    }

//    //-----------------------------------------------------------------

    property var changedColorsMap: ({})

    onSelectedThemeNameChanged: {
        changedColorsMap = ({})
    }

//    function setColorProperty(propertyExactName, color){
//        changedColorsMap[propertyExactName] = color
//    }

//    function resetColorProperties(){
//        changedColorsMap = ({})
//        skrThemes.resetSelectedThemeColors()
//    }

//    function saveColorProperties(){

//        if(skrThemes.isThemeEditable(selectedThemeName))
//            skrThemes.saveTheme(selectedThemeName, changedColorsMap)
//    }

//    //-----------------------------------------------------------------


}
