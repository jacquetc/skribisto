import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

TextArea {
    id: root
    property bool styleElevation: false
    property string styleBackgroundColor: "#FFFFFF"
    property string styleForegroundColor: "#000000"
    property string styleAccentColor: "#000000"
    objectName: "SKRTextArea-" + Qt.formatDateTime(new Date(), "yyyyMMddhhmmsszzz")
    //color: styleForegroundColor

    //    Keys.priority: Keys.BeforeItem

    //    Keys.onReleased: {
    //        if (event.key === Qt.Key_Alt){
    //            console.log("alt not accepted")
    //            Globals.showMenuBarCalled()
    //            event.accepted = true
    //        }

    //    }
    Material.accent: styleAccentColor
    Material.foreground: styleForegroundColor

    background:
        Pane {
            Material.background: styleBackgroundColor
            Material.elevation: styleElevation ? 6 : 0
        }



}
