import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.3
import QtQml 2.12
import Qt.labs.settings 1.1

ApplicationWindow {

    id: rootWindow
    //visible: true
    minimumHeight: 500
    minimumWidth: 600

    onHeightChanged: Globals.height = height
    onWidthChanged: Globals.width = width

    x: settings.x
    y: settings.y
    height: settings.height
    width: settings.width

    visibility: settings.visibility
    Settings {
        id: settings
        category: "window"
        property int x: 0
        property int y: 0
        property int height: Screen.height
        property int width: Screen.width
        property int visibility: Window.Maximized
    }


    Action {

        id: fullscreenAction
        text: qsTr("Fullscreen")
        icon {
            name: "view-fullscreen"
            height: 50
            width: 50
        }

        shortcut: StandardKey.FullScreen
        checkable: true
        onCheckedChanged: {
            Globals.fullScreenCalled(fullscreenAction.checked)
        }
    }




    // style :
    //palette.window: "white"

    // Splash screen
    //    Window {
    //        id: splash
    //        color: "transparent"
    //        title: "Splash Window"
    //        modality: Qt.ApplicationModal
    //        flags: Qt.SplashScreen
    //        property int timeoutInterval: 1000
    //        signal timeout
    //        x: (Screen.width - splashImage.width) / 2
    //        y: (Screen.height - splashImage.height) / 2
    //        width: splashImage.width
    //        height: splashImage.height

    //        Image {
    //            id: splashImage
    //            source: "qrc:/pics/skribisto.svg"
    //        }
    //        Timer {
    //            interval: splash.timeoutInterval; running: true; repeat: false
    //            onTriggered: {
    //                splash.visible = false
    //                splash.timeout()
    //            }
    //        }
    //        Component.onCompleted: splash.visible = true
    //    }

    //    Loader {
    //        id: loader
    //        asynchronous: true
    //        sourceComponent: rootPage
    ////        onLoaded: splash.visible = false
    //    }

    //    Component{
    //        id: rootPage
    RootPage {
        //window: rootWindow
        anchors.fill: parent
    }



    Connections {
        target: Globals
        // @disable-check M16
        onFullScreenCalled: function (value) {
            console.log("fullscreen")
            if(value){
                visibility = Window.FullScreen
            }
            else {
                visibility = Window.AutomaticVisibility
            }

        }
    }

    onClosing: {
        console.log("quiting")

        settings.x = rootWindow.x
        settings.y = rootWindow.y
        settings.width = rootWindow.width
        settings.height = rootWindow.height
        settings.visibility = rootWindow.visibility

        plmData.projectHub().closeAllProjects()

        //close.accepted = false
    }
} //}

/*##^## Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
 ##^##*/

