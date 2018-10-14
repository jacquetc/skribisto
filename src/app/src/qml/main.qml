import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Window 2.3
import QtQml 2.2

ApplicationWindow {
    id: rootWindow
    visible: true
    minimumHeight: 600
    minimumWidth: 800

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
//            source: "qrc:/pics/plume-creator.svg"
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
        RootPage{
            //window: rootWindow
            anchors.fill: parent

        }
//   }



}
