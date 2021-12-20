import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import ".."

Popup {
    id: root
    //Material.background: SkrTheme.pageBackground
    //anchors.centerIn: Overlay.overlay

    readonly property int  windowWidth : Overlay.overlay === null ? 0 : Overlay.overlay.width
    readonly property int  windowHeight :  Overlay.overlay === null ? 0 : Overlay.overlay.height
    property bool enableBehavior: true


    modal: true
    background: Rectangle {
        color: SkrTheme.pageBackground
    }

    onOpened: {
        // verify if popup appears outside the window

        var popupX = parent.mapToItem(Overlay.overlay, root.x, root.y).x
        var popupY = parent.mapToItem(Overlay.overlay, root.x, root.y).y

        if(popupX + root.width > windowWidth){
            //move left
            root.x = root.x - (popupX + root.width - windowWidth)
        }

        if(popupX < 0){
            //move right
            root.x = root.x + (0 - popupX)
        }

        if(popupY + root.height > windowHeight){
            //move up
            root.y = root.y - (popupY + root.height - windowHeight)
        }

        if(popupY < 0){
            //move down
            root.y = root.y + (0 - popupY)
        }
    }

    Behavior on x {
        enabled: enableBehavior && SkrSettings.ePaperSettings.animationEnabled
        SpringAnimation {
            spring: 5
            mass: 0.2
            damping: 0.2
        }
    }

    Behavior on y {
        enabled: enableBehavior  && SkrSettings.ePaperSettings.animationEnabled
        SpringAnimation {
            spring: 5
            mass: 0.2
            damping: 0.2
        }
    }

}
