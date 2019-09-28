import QtQuick 2.11
import QtQuick.Layouts 1.3
import QtQuick.Controls 2.4

WriteLeftDockForm {

    property bool folded: state === "leftDockFolded" ? true : false
    onFoldedChanged:  folded ? fold(): unfold()

    function fold(){
        state = "leftDockFolded"

    }
    function unfold(){
        state = "leftDockUnfolded"

    }

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
            source: "qrc:/pics/skribisto.svg"
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

            PropertyAnimation { properties: "implicitWidth"; easing.type: Easing.InOutQuad;duration: 500  }


        }
    ]

    Component.onCompleted:{

    }
}
