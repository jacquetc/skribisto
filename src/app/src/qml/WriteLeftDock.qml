import QtQuick 2.11
import QtQuick.Layouts 1.3
import QtQuick.Controls 2.4

WriteLeftDockForm {

    Component{
        id: dockHeaderComp
        RowLayout {

            Text{
                text: headerText

            }
        }

    }


    Action{

        id: fullscreenAction
        text: qsTr("Fullscreen")
        icon {
            name: "welcome-icon"
            source: "qrc:/pics/plume-creator.svg"
            color: "transparent"
            height: 50
            width: 50

        }

        shortcut: "F5"
        checkable: true
        checked: true

        onTriggered: root_stack.currentIndex = 0
    }


    transitions: [
        Transition {

            //ParallelAnimation {

            PropertyAnimation { properties: "anchors.bottomMargin"; easing.type: Easing.InOutQuad;duration: 500  }
            //PropertyAnimation { properties: "visible"; easing.type: Easing.InOutQuad;duration: 1000  }
            //}

        }
    ]

    Component.onCompleted:{

    }
}
