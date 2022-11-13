import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import theme 1.0


Popup {

    //Material.background: SkrTheme.pageBackground
    //anchors.centerIn: Overlay.overlay

    readonly property int  windowWidth : Overlay.overlay === null ? 0 : Overlay.overlay.width
    readonly property int  windowHeight :  Overlay.overlay === null ? 0 : Overlay.overlay.height
    property bool enableBehavior: true


    modal: true
    background: Rectangle {
        color: SkrTheme.pageBackground
    }



}
